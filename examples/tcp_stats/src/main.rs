use retina_core::config::load_config;
use retina_core::subscription::Connection;
use retina_core::Runtime;
use retina_filtergen::filter;

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: PathBuf,
}

#[filter("tcp")]
fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let config = load_config(&args.config);

    let num_gaps = AtomicUsize::new(0);
    let num_conn = AtomicUsize::new(0);
    let num_remaining = AtomicUsize::new(0);
    let callback = |conn: Connection| {
        num_conn.fetch_add(1, Ordering::Relaxed);

        let num_gaps_orig_resp = conn.orig.gaps.len() + conn.resp.gaps.len();
        num_gaps.fetch_add(num_gaps_orig_resp, Ordering::Relaxed);
        
        let num_remaining_gaps = conn.orig.content_gaps() + conn.resp.content_gaps();
        num_remaining.fetch_add(num_remaining_gaps as usize, Ordering::Relaxed);
    };

    let mut runtime = Runtime::new(config, filter, callback)?;
    runtime.run();

    let load_num_gaps = num_gaps.load(Ordering::SeqCst);
    let load_num_conn = num_conn.load(Ordering::SeqCst);
    println!("Number of TCP connections: {:?}", num_conn);
    println!("Number of observed gaps across all TCP connections: {:?}", num_gaps);
    println!("Percent of TCP connections with gaps: {:.2}", (load_num_gaps as f64 / load_num_conn as f64) * 100.0);

    println!("Number of remaining gaps at the end of all TCP connections: {:?}", num_remaining);
    Ok(())
}
