use retina_core::{config::load_config, Runtime};
use retina_datatypes::{DnsTransaction, HttpTransaction, TlsHandshake};
use retina_filtergen::{filter, retina_main};

use clap::Parser;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    static ref file: Mutex<BufWriter<File>> =
        Mutex::new(BufWriter::new(File::create("test_not_protocol.jsonl").unwrap()));
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: PathBuf,
    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "FILE",
        default_value = "test_not_protocol.jsonl"
    )]
    outfile: PathBuf,
}

#[filter("!http")]
fn not_http_cb(tls: &TlsHandshake) { //SessionList or ConnRecord
    println!("cb1");
    if let Ok(serialized) = serde_json::to_string(&tls) {
        let mut wtr = file.lock().unwrap();
        wtr.write_all(serialized.as_bytes()).unwrap();
        wtr.write_all(b"\n").unwrap();
    }
}

// also test for !tcp, !ipv4, etc. - at least 1 packet-layer
// no HttpTransaction's should be logged
//#[filter("!http")]
//fn not_http_cb2(http: &HttpTransaction) {
//    println!("cb2");
//    if let Ok(serialized) = serde_json::to_string(&http) {
//        let mut wtr = file.lock().unwrap();
//        wtr.write_all(serialized.as_bytes()).unwrap();
//        wtr.write_all(b"\n").unwrap();
//    }
//}

//#[filter("!tls")]
//fn not_tls_cb(dns: &DnsTransaction) {
//    println!("cb3");
//    if let Ok(serialized) = serde_json::to_string(&dns) {
//        let mut wtr = file.lock().unwrap();
//        wtr.write_all(serialized.as_bytes()).unwrap();
//        wtr.write_all(b"\n").unwrap();
//    }
//}

#[retina_main(1)]
fn main() {
    let args = Args::parse();
    let config = load_config(&args.config);

    let mut runtime: Runtime<SubscribedWrapper> = Runtime::new(config, filter).unwrap();
    runtime.run();

    let mut wtr = file.lock().unwrap();
    wtr.flush().unwrap();
}
