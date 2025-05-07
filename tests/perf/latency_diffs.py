import argparse
import time
import subprocess
import os
from bcc import BPF
from collections import defaultdict
from itertools import combinations
import numpy as np
import pandas as pd
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import ctypes
import pprint

# key: (run_num, func name), val: list of latencies
EBPF_LATENCIES = {}
TIMER_LATENCIES = {}

# key: func name, val: float representing difference in avg latencies 
AVG_LATENCY_DIFFS = {}

# key: (func1, func2), val: list of (func1 avg latency - func2 avg latency) across all runs
EBPF_DIFFS_BTWN_FUNCS = {}
RDTSC_DIFFS_BTWN_FUNCS = {}

class Data(ctypes.Structure):
    _fields_ = [
        ("pid", ctypes.c_uint),
        ("latency", ctypes.c_ulonglong),
    ]

def latency_hist(args):
    setup_code = """
    #include <uapi/linux/ptrace.h>

    struct key_t {
        u32 pid;
        u64 func_id;
    };

    struct data_t {
        u32 pid;
        u64 func_id;
        u64 latency;
    };

    struct hist_key_t {
        u64 func_id;
        u64 slot;
    };

    BPF_HISTOGRAM(dist, struct hist_key_t);
    BPF_HASH(start, struct key_t, u64);
    BPF_PERF_OUTPUT(latencies);
    """

    entry_exit_code = """
    int trace_func_{id}_entry(struct pt_regs *ctx)
    {{
        struct key_t key = {{}};
        key.pid = bpf_get_current_pid_tgid();
        key.func_id = {id};

        u64 ts = bpf_ktime_get_ns();

        start.update(&key, &ts);
        return 0;
    }}

    int trace_func_{id}_exit(struct pt_regs *ctx)
    {{
        struct key_t key = {{}};
        key.pid = bpf_get_current_pid_tgid();
        key.func_id = {id};

        u64 *tsp = start.lookup(&key);
        if (tsp == 0) return 0;  

        u64 delta = bpf_ktime_get_ns() - *tsp;
        TIMING_UNIT

        struct data_t data = {{}};
        data.pid = key.pid;
        data.func_id = key.func_id;
        data.latency = delta;
        latencies.perf_submit(ctx, &data, sizeof(data));

        struct hist_key_t hkey = {{}};
        hkey.func_id = key.func_id;
        hkey.slot = bpf_log2l(delta);
        dist.increment(hkey);

        start.delete(&key);
        return 0;
    }}
    """
    probing_code = ""
    FUNC_ID_MAPPINGS = {}
    for i, func in enumerate(args.function):
        FUNC_ID_MAPPINGS[i + 1] = func
        probing_code += entry_exit_code.format(id=i+1)
    bpf_program = setup_code + probing_code

    if args.microseconds:
        bpf_program = bpf_program.replace('TIMING_UNIT', 'delta /= 1000;')
        label = "usecs" 
    else:
        bpf_program = bpf_program.replace('TIMING_UNIT', '')
        label = "nsecs" 

    # path = f"/home/dianaq/Downloads/retina-fork/retina/target/debug/{args.app}"
    path = f"./target/debug/{args.app}"

    funcs = []
    # get the mangled function name to pass into attach_uprobe() and attach_uretprobe()
    # TODO: what if different modules have funcs with the same name
    for func in args.function:
        get_mangled_name_cmd = f"nm {path} | grep {func} | awk '{{print $3}}'"
        p1 = subprocess.run(get_mangled_name_cmd, shell=True, capture_output=True, text=True)
        mangled_name = p1.stdout.strip()

        if not mangled_name:
            print(f"{func} is never called.")
        
        # print('mangled_name:', mangled_name)
        # print('address:', BPF.get_user_addresses(path, mangled_name))
        funcs.append(mangled_name)

    # no functions to profile 
    if not funcs: 
        return
    
    # print(f"Functions to profile: {funcs}")
    b = BPF(text=bpf_program)
    for func_id, func_mangled_name in enumerate(funcs):
        try:
            entry_func = f"trace_func_{func_id + 1}_entry"
            exit_func = f"trace_func_{func_id + 1}_exit"
            b.attach_uprobe(name=path, sym=func_mangled_name, fn_name=entry_func, pid=-1)
            b.attach_uretprobe(name=path, sym=func_mangled_name, fn_name=exit_func, pid=-1) 
        except Exception as e:
            print(f"Failed to attach uprobes: {e}")

    n_open_probes = b.num_open_uprobes()
    print('n_open_probes:', n_open_probes)
    
    for i in range(args.num_runs):
        latencies = defaultdict(list)
        def print_event(cpu, data, size):
            event = b["latencies"].event(data) 
            latencies[event.func_id].append(event.latency)
        b["latencies"].open_perf_buffer(print_event)

        ld_lib_path = "/home/dianaq/dpdk-21.08/lib/aarch64-linux-gnu"
        cmd = f"sudo env LD_LIBRARY_PATH={ld_lib_path} RUST_LOG=error {path} -c {args.config}"
        p2 = subprocess.Popen(cmd, shell=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

        try:
            while p2.poll() is None:
                b.perf_buffer_poll(timeout=1000)
        except KeyboardInterrupt:
            p2.kill()
        
        dist = b.get_table("dist")
        hists = {}
        for k, v in dist.items():
            func_id = k.func_id 
            if func_id not in hists:
                hists[func_id] = {}
            hists[func_id][k.slot] = hists[func_id].get(k.slot, 0) + v.value

        print(f"Run {i+1}:")
        for func_id, slots in hists.items():
            func_name = FUNC_ID_MAPPINGS[func_id]
            func_hist = {slot: count for slot, count in sorted(slots.items())}
            num_func_calls = sum(func_hist.values())
            print(f"{func_name} was called {num_func_calls} times")
            # print(f"\nLatency Histogram for {func_name}():")
            # dist.print_log2_hist(func_hist, "usecs")

        # compute stats for eBPF
        for func_id, latencies_lst in latencies.items():
            func_name = FUNC_ID_MAPPINGS[func_id]
            EBPF_LATENCIES[(i, func_name)] = ebpf_latency_stats(func_name, latencies_lst, label)
        
        # compute stats for Retina's Timer API
        for func_id, func_name in FUNC_ID_MAPPINGS.items():
            TIMER_LATENCIES[(i, func_name)] = timer_latency_stats(func_name, label)

            # compute difference in avg eBPF and Timer API latency
            # latency diff (in microseconds) = eBPF latency - timer API latency
            latency_diff = EBPF_LATENCIES[(i, func_name)][0] - TIMER_LATENCIES[(i, func_name)][0]
            if func_name not in AVG_LATENCY_DIFFS:
                AVG_LATENCY_DIFFS[func_name] = [latency_diff]
            else: 
                AVG_LATENCY_DIFFS[func_name].append(latency_diff)
        
        # clear dist before the next run
        dist.clear()

    for func_name in AVG_LATENCY_DIFFS.keys():
        visualize_diffs(args.app, func_name)
    
    for func_name in AVG_LATENCY_DIFFS.keys():
        print(f"Latency diffs for {func_name}: {AVG_LATENCY_DIFFS[func_name]}")
    
    # differences across functions
    for run_num in range(args.num_runs):
        for comb in combinations(FUNC_ID_MAPPINGS.values(), 2):
            func1 = comb[0]
            func2 = comb[1]
            ebpf_func_diff = EBPF_LATENCIES[(run_num, func1)][0] - EBPF_LATENCIES[(run_num, func2)][0]
            if (func1, func2) not in EBPF_DIFFS_BTWN_FUNCS:
                EBPF_DIFFS_BTWN_FUNCS[(func1, func2)] = [ebpf_func_diff]
            else:
                EBPF_DIFFS_BTWN_FUNCS[(func1, func2)].append(ebpf_func_diff)
            
            rdtsc_func_diff = TIMER_LATENCIES[(run_num, func1)][0] - TIMER_LATENCIES[(run_num, func2)][0]
            if (func1, func2) not in RDTSC_DIFFS_BTWN_FUNCS:
                RDTSC_DIFFS_BTWN_FUNCS[(func1, func2)] = [rdtsc_func_diff]
            else:
                RDTSC_DIFFS_BTWN_FUNCS[(func1, func2)].append(rdtsc_func_diff)
    
    for comb in combinations(FUNC_ID_MAPPINGS.values(), 2):
        func1 = comb[0]
        func2 = comb[1]
        visualize_func_diffs(args.app, func1, func2)

    # print('EBPF_DIFFS_BTWN_FUNCS:', EBPF_DIFFS_BTWN_FUNCS)
    # print('RDTSC_DIFFS_BTWN_FUNCS:', RDTSC_DIFFS_BTWN_FUNCS)

def ebpf_latency_stats(func_name, latencies, label):
    # microseconds
    avg = np.mean(latencies)
    p25 = np.percentile(latencies, 25)
    median = np.median(latencies)
    p75 = np.percentile(latencies, 75)
    p95 = np.percentile(latencies, 95)

    # print(f"\nEBPF Latency Stats for {func_name}():")
    # print(f"Average: {avg:.3f} {label}")
    # print(f"25th percentile: {p25} {label}")
    # print(f"Median: {median} {label}")
    # print(f"75th percentile: {p75} {label}")
    # print(f"95th percentile: {p95} {label}")
    return avg, p25, median, p75, p95

def timer_latency_stats(func_name, label):
    stats = ['avg', 'p25', 'p50', 'p75', 'p95']
    cycle_hist = pd.read_csv('./cycle_hist.csv')

    if func_name == 'process_packet':
        name = 'process'
    elif func_name == 'filter_first_packet':
        name = 'packet_filter'
    elif func_name == 'handle_parse':
        name = 'applayer_parse'

    filtered_df = cycle_hist[(cycle_hist['name'] == name)]
    # measured in CPU cycles
    index = filtered_df.index[0]
    avg_cc = filtered_df.at[index, 'avg']
    p25_cc = filtered_df.at[index, 'p25']
    median_cc = filtered_df.at[index, 'p50']
    p75_cc = filtered_df.at[index, 'p75']
    p95_cc = filtered_df.at[index, 'p95']

    # convert from CPU cycles to microseconds 
    clock_freq = 24000000.0
    multiplier = 1000000
    avg = (avg_cc / clock_freq) * multiplier
    p25 = (p25_cc / clock_freq) * multiplier
    median = (median_cc / clock_freq) * multiplier
    p75 = (p75_cc / clock_freq) * multiplier
    p95 = (p95_cc / clock_freq) * multiplier

    # print(f"\nLatency Stats (rdtsc) for {func_name}():")
    # print(f'Average: {avg_cc} CPU cycles ({avg:.3f} {label})')
    # print(f'25th percentile: {p25_cc} CPU cycles ({p25:.3f} {label})')
    # print(f'Median: {median_cc} CPU cycles ({median:.3f} {label})')
    # print(f'75th percentile: {p75_cc} CPU cycles ({p75:.3f} {label})')
    # print(f'95th percentile: {p95_cc} CPU cycles ({p95:.3f} {label})')
    # print()
    return avg, p25, median, p75, p95

def visualize_diffs(app, func_name):
    plt.hist(AVG_LATENCY_DIFFS[func_name], bins=20)
    plt.grid(True, ls="--")
    plt.xlabel('difference (ebpf - rdtsc) in avg latency (usecs)')
    plt.ylabel('count')

    figs_dir = "./tests/perf/figs/diffs"
    os.makedirs(figs_dir, exist_ok=True)
    plt.savefig(os.path.join(figs_dir, f"{app}_{func_name}.png"), dpi=300, bbox_inches='tight')
    plt.clf()

def visualize_func_diffs(app, func1, func2):
    plt.hist(EBPF_DIFFS_BTWN_FUNCS[(func1, func2)], bins=20, label='eBPF', alpha=0.7)
    plt.hist(RDTSC_DIFFS_BTWN_FUNCS[(func1, func2)], bins=20, label='rdtsc', alpha=0.7)
    plt.grid(True, ls="--")
    plt.legend(loc='upper right')
    plt.xlabel(f"difference in avg latency between {func1} and {func2} (usecs)")

    figs_dir = "./tests/perf/figs/diffs/funcs"
    os.makedirs(figs_dir, exist_ok=True)
    plt.savefig(os.path.join(figs_dir, f"{app}_{func1}_{func2}.png"), dpi=300, bbox_inches='tight')
    plt.clf()

def comma_sep_list(value):
    return value.split(',')

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("app")
    parser.add_argument("-f", "--function", type=comma_sep_list)
    parser.add_argument("-c", "--config", default="./configs/offline.toml")
    parser.add_argument("-n", "--num_runs", type=int, default=1)
    parser.add_argument("-u", "--microseconds", action="store_true")
    parser.add_argument("-m", "--mode", default="debug")
    # parser.add_argument("-p", "--plot", action="store_true")
    args = parser.parse_args()
            
    latency_hist(args)