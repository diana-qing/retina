use retina_core::{config::load_config, Runtime};
use retina_datatypes::{ConnRecord, HttpTransaction, SessionList};
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

// also test for !tcp, !ipv4, etc. - at least 1 packet-layer
// #[filter("!tcp")]
// fn not_tcp_cb(conn_record: &ConnRecord) {
//    println!("protocol: {}", conn_record.five_tuple.proto);
//    if let Ok(serialized) = serde_json::to_string(&conn_record) {
//        let mut wtr = file.lock().unwrap();
//        wtr.write_all(serialized.as_bytes()).unwrap();
//        wtr.write_all(b"\n").unwrap();
//    }
// }

#[filter("!tls")]
fn not_tls_cb(session_list: &SessionList) { //SessionList or ConnRecord
    println!("cb2");
    if let Ok(serialized) = serde_json::to_string(&session_list) {
        let mut wtr = file.lock().unwrap();
        wtr.write_all(serialized.as_bytes()).unwrap();
        wtr.write_all(b"\n").unwrap();
    }
}

// no HttpTransaction's should be logged
//#[filter("!http")]
//fn not_http_cb(http: &HttpTransaction) {
//    println!("cb3");
//    if let Ok(serialized) = serde_json::to_string(&http) {
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
