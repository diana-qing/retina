import argparse
import time
import subprocess
import os
from bcc import BPF

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

        start.delete(&pid);
        dist.increment(bpf_log2l(delta)); // nanoseconds by default
        return 0;
    }
    """
    # path = f"/home/dianaq/Downloads/retina-fork/retina/target/release/{args.app}"
    path = f"./target/release/{args.app}"
    symbol = args.function

    print(list(BPF.get_user_addresses(path, symbol)))
    
    b = BPF(text=bpf_program)
    b.attach_uprobe(name=path, sym_re=symbol, fn_name="trace_func_entry", pid=-1)
    b.attach_uretprobe(name=path, sym_re=symbol, fn_name="trace_func_return", pid=-1) 
    n_open_probes = b.num_open_uprobes()
    
    ld_lib_path = "/home/dianaq/dpdk-21.08/lib/aarch64-linux-gnu"
    cmd = f"sudo env LD_LIBRARY_PATH={ld_lib_path} RUST_LOG=error {path} -c {args.config}"
    p = subprocess.Popen(cmd, shell=True)
    
    try:
       while p.poll() is None:
           time.sleep(1)
    except KeyboardInterrupt:
       p.kill()
    
    dist = b.get_table("dist")
    num_func_calls = sum(count.value for count in dist.values())
    print(f"{symbol} was called {num_func_calls} times")
        
    print("Latency Histogram:")
    label = "nsecs" 
    dist.print_log2_hist(label)
    dist.clear()

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("app")
    parser.add_argument("-f", "--function")
    parser.add_argument("-c", "--config", default="./configs/offline.toml")
    args = parser.parse_args()
            
    latency_hist(args)
