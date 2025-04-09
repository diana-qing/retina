import argparse
import time
import subprocess
import statistics

def run_application(args):
    build_cmd = f'cargo build --release --bin {args.application}'
    executable = f'./target/release/{args.application}'
    cmd = f'sudo env LD_LIBRARY_PATH=$LD_LIBRARY_PATH RUST_LOG=error {executable} --config {args.config}'
    
    p1 = subprocess.run(build_cmd, shell=True, capture_output=True, text=True)
    
    elapsed_times = []
    for i in range(int(args.num_runs)):
        start = time.perf_counter()
        p2 = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        end = time.perf_counter()
        elapsed = end - start
        elapsed_times.append(elapsed)
    
    avg_exec_time = statistics.mean(elapsed_times)

    print(f"Executing: {cmd}") 
    print(f"Average execution time across {args.num_runs} runs: {avg_exec_time:.4f} secs\n")

if __name__ == '__main__':
    parser = argparse.ArgumentParser()

    parser.add_argument('application')
    parser.add_argument('-c', '--config', default='./configs/offline.toml')
    parser.add_argument('-n', '--num_runs', default=5)

    args = parser.parse_args()

    run_application(args)
