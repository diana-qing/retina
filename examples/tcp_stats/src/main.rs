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
    let callback = |conn: Connection| {
        let num_gaps_orig_resp = conn.orig.gaps.len() + conn.resp.gaps.len();
        num_gaps.fetch_add(num_gaps_orig_resp, Ordering::Relaxed);
        num_conn.fetch_add(1, Ordering::Relaxed);
    };

    let mut runtime = Runtime::new(config, filter, callback)?;
    runtime.run();

    let numerator = num_gaps.load(Ordering::SeqCst);
    let denom = num_conn.load(Ordering::SeqCst);
    println!("Total number of TCP connections: {:?}", num_conn);
    println!("Number of gaps across all TCP connections: {:?}", num_gaps);
    println!("Percent of TCP connections with gaps: {:.2}", (numerator as f64 / denom as f64) * 100.0);
    Ok(())
}
