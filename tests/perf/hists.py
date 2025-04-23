import argparse
import time
import subprocess
import os
from bcc import BPF
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import ctypes

def latency_hist(args):
    bpf_program = """
    #include <uapi/linux/ptrace.h>

    BPF_HISTOGRAM(dist);
    BPF_HASH(start, u32);

    int trace_func_entry(struct pt_regs *ctx)
    {
        u32 pid = bpf_get_current_pid_tgid();
        u64 ts = bpf_ktime_get_ns();
        start.update(&pid, &ts);
        return 0;
    }

    int trace_func_return(struct pt_regs *ctx)
    {
        u32 pid = bpf_get_current_pid_tgid();

        u64 *tsp = start.lookup(&pid);
        if (tsp == 0) {
            return 0;   // missed start
        }

        u64 delta = bpf_ktime_get_ns() - *tsp;
        TIMING_UNIT

        start.delete(&pid);
        dist.increment(bpf_log2l(delta));
        return 0;
    }
    """
    if args.microseconds:
        bpf_program = bpf_program.replace('TIMING_UNIT', 'delta /= 1000;')
        label = "usecs" 
    else:
        bpf_program = bpf_program.replace('TIMING_UNIT', '')
        label = "nsecs" 

    # path = f"/home/dianaq/Downloads/retina-fork/retina/target/release/{args.app}"
    path = f"./target/release/{args.app}"

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
        b.attach_uretprobe(name=path, sym=mangled_name, fn_name="trace_func_return", pid=-1) 
    except Exception as e:
        print(f"Failed to attach uprobes: {e}")

    n_open_probes = b.num_open_uprobes()
    
    ld_lib_path = "/home/dianaq/dpdk-21.08/lib/aarch64-linux-gnu"
    cmd = f"sudo env LD_LIBRARY_PATH={ld_lib_path} RUST_LOG=error {path} -c {args.config}"
    p2 = subprocess.Popen(cmd, shell=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    
    try:
       while p2.poll() is None:
           time.sleep(1)
    except KeyboardInterrupt:
       p2.kill()
    
    dist = b.get_table("dist")
    num_func_calls = sum(count.value for count in dist.values())
    print(f"{args.function} was called {num_func_calls} times")

    if args.plot:
        bins = []
        counts = []
        for k, v in dist.items():
            bin = k.value
            count = v.value
            bins.append(1 << bin)  # Convert back from log2 to actual value
            # bins.append(bin) 
            counts.append(count)
        
        print('bins:', bins)
        print('counts:', counts)

        plt.figure(figsize=(10, 6))
        plt.bar(bins, counts, align="center")
        # plt.xscale('log', basex=2)
        plt.xlabel(f"Latency ({label})")
        plt.ylabel("Count")
        plt.title(f"Distribution of {args.function} latency when running {args.app}")
        plt.grid(True, ls="--")
        
        figs_dir = "./tests/perf/figs"
        os.makedirs(figs_dir, exist_ok=True)
        plt.savefig(os.path.join(figs_dir, f"{args.app}_{args.function}_latency.png"), dpi=300, bbox_inches='tight')
    
    print("Latency Histogram:")
    dist.print_log2_hist(label)
    dist.clear()

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("app")
    parser.add_argument("function")
    parser.add_argument("-c", "--config", default="./configs/offline.toml")
    parser.add_argument("-u", "--microseconds", action="store_true")
    parser.add_argument("-p", "--plot", action="store_true")
    args = parser.parse_args()
            
    latency_hist(args)
