import argparse
import time
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
        dist.atomic_increment(bpf_log2l(delta)); // nanoseconds by default
        return 0;
    }
    """

    path = f'./target/release/{args.application}'
    symbol = args.function

    b = BPF(text=bpf_program)
    b.attach_uprobe(name=path, sym_re=symbol, fn_name="trace_func_entry")
    b.attach_uretprobe(name=path, sym_re=symbol, fn_name="trace_func_return")
    num_func_calls = b.num_open_uprobes()

    print(f"Tracing latency of {symbol}...")

    while (1):
        try:
            time.sleep(1)
        except KeyboardInterrupt:
            print(f"{symbol} was called {num_func_calls} times")
        
            print("Latency Histogram:")
            dist = b.get_table("dist")
            label = "nsecs" 
            dist.print_log2_hist(label)
            exit()

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('-f', '--function')

    args = parser.parse_args()
            
    latency_hist(args)