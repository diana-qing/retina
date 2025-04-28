import argparse
import time
import subprocess
import os
from bcc import BPF
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import ctypes

class Data(ctypes.Structure):
    _fields_ = [
        ("pid", ctypes.c_uint),
        ("latency", ctypes.c_ulonglong),
    ]

def latency_hist(args):
    bpf_program = """
    #include <uapi/linux/ptrace.h>

    struct data_t {
        u32 pid;
        u64 latency;
    };

    BPF_HISTOGRAM(dist);
    BPF_HASH(start, u32);
    BPF_PERF_OUTPUT(latencies);

    int trace_func_entry(struct pt_regs *ctx)
    {
        u32 pid = bpf_get_current_pid_tgid();
        u64 ts = bpf_ktime_get_ns();
        start.update(&pid, &ts);
        return 0;
    }

    int trace_func_exit(struct pt_regs *ctx)
    {
        u32 pid = bpf_get_current_pid_tgid();

        u64 *tsp = start.lookup(&pid);
        if (tsp == 0) {
            return 0;   // missed start
        }

        u64 delta = bpf_ktime_get_ns() - *tsp;
        TIMING_UNIT

        struct data_t data = {};
        data.pid = pid;
        data.latency = delta;
        latencies.perf_submit(ctx, &data, sizeof(data));

        dist.increment(bpf_log2l(delta));
        start.delete(&pid);

        return 0;
    }
    """
    if args.microseconds:
        bpf_program = bpf_program.replace('TIMING_UNIT', 'delta /= 1000;')
        label = "usecs" 
    else:
        bpf_program = bpf_program.replace('TIMING_UNIT', '')
        label = "nsecs" 

    # p00 = subprocess.run("cargo clean", shell=True, capture_output=True, text=True)
    # compile_cmd = f"cargo build --bin {args.app} --features timing"
    # p0 = subprocess.run(compile_cmd, shell=True, capture_output=True, text=True)
    # path = f"/home/dianaq/Downloads/retina-fork/retina/target/debug/{args.app}"
    path = f"./target/debug/{args.app}"

    # get the mangled function name to pass into attach_uprobe() and attach_uretprobe()
    # TODO: what if different modules have funcs with the same name
    get_mangled_name_cmd = f"nm {path} | grep {args.function} | awk '{{print $3}}'"
    p1 = subprocess.run(get_mangled_name_cmd, shell=True, capture_output=True, text=True)
    mangled_name = p1.stdout.strip()

    if not mangled_name:
        print(f"{args.function} is never called.")
        return

    print('mangled_name:', mangled_name)
    print('address:', BPF.get_user_addresses(path, mangled_name))
    
    b = BPF(text=bpf_program)
    try:
        b.attach_uprobe(name=path, sym=mangled_name, fn_name="trace_func_entry", pid=-1)
        b.attach_uretprobe(name=path, sym=mangled_name, fn_name="trace_func_exit", pid=-1) 
    except Exception as e:
        print(f"Failed to attach uprobes: {e}")

    n_open_probes = b.num_open_uprobes()
    
    latencies = []
    def print_event(cpu, data, size):
        event = b["latencies"].event(data) 
        latencies.append(event.latency)
    b["latencies"].open_perf_buffer(print_event)

    ld_lib_path = "/home/dianaq/dpdk-21.08/lib/aarch64-linux-gnu"
    cmd = f"sudo env LD_LIBRARY_PATH={ld_lib_path} RUST_LOG=error {path} -c {args.config} --features timing"
    p2 = subprocess.Popen(cmd, shell=True) # stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL
    
    try:
       while p2.poll() is None:
           b.perf_buffer_poll(timeout=1000)
    except KeyboardInterrupt:
       p2.kill()
    
    dist = b.get_table("dist")
    num_func_calls = sum(count.value for count in dist.values())
    print(f"{args.function} was called {num_func_calls} times")

    print("Latency Histogram:")
    dist.print_log2_hist(label)

    print("\nLatency Stats:")
    print_latency_stats(latencies, label)

    if args.plot:
        plot_latency_hist(latencies, label, args.function, args.app)
    
    dist.clear()

def print_latency_stats(latencies, label):
    avg = np.mean(latencies)
    percentile25 = np.percentile(latencies, 25)
    median = np.median(latencies)
    percentile75 = np.percentile(latencies, 75)
    percentile95 = np.percentile(latencies, 95)
    print(f"Average: {avg} {label}")
    print(f"25th percentile: {percentile25} {label}")
    print(f"Median: {median} {label}")
    print(f"75th percentile: {percentile75} {label}")
    print(f"95th percentile: {percentile95} {label}\n")

def plot_latency_hist(latencies, label, func, app):
    plt.figure(figsize=(10, 6))
    plt.hist(latencies, bins=20)
    plt.xlabel(f"Latency ({label})")
    plt.ylabel("Count")
    plt.title(f"Distribution of latencies for {args.function}() when running {args.app}")
    plt.grid(True, ls="--")
    
    figs_dir = "./tests/perf/figs"
    os.makedirs(figs_dir, exist_ok=True)
    plt.savefig(os.path.join(figs_dir, f"{args.app}_{args.function}_latency.png"), dpi=300, bbox_inches='tight')

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("app")
    parser.add_argument("function")
    parser.add_argument("-c", "--config", default="./configs/offline.toml")
    parser.add_argument("-u", "--microseconds", action="store_true")
    parser.add_argument("-p", "--plot", action="store_true")
    args = parser.parse_args()
            
    latency_hist(args)
