Filter: !tls, Datatypes: ["TlsHandshake"], Callback: "not_tls_cb"
Expecting 1 subsctription(s)
Datatypes {
  TlsHandshake,
}

Parsers {
  tls,
}

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use retina_core::{config::load_config, Runtime};
use retina_datatypes::{ConnRecord, TlsHandshake, SessionList};
use retina_filtergen::{filter, retina_main};
use clap::Parser;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Mutex;
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct file {
    __private_field: (),
}
#[doc(hidden)]
#[allow(non_upper_case_globals)]
static file: file = file { __private_field: () };
impl ::lazy_static::__Deref for file {
    type Target = Mutex<BufWriter<File>>;
    fn deref(&self) -> &Mutex<BufWriter<File>> {
        #[inline(always)]
        fn __static_ref_initialize() -> Mutex<BufWriter<File>> {
            Mutex::new(BufWriter::new(File::create("test_not_protocol.jsonl").unwrap()))
        }
        #[inline(always)]
        fn __stability() -> &'static Mutex<BufWriter<File>> {
            static LAZY: ::lazy_static::lazy::Lazy<Mutex<BufWriter<File>>> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for file {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
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
impl clap::Parser for Args {}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
#[allow(deprecated)]
impl clap::CommandFactory for Args {
    fn into_app<'b>() -> clap::Command<'b> {
        let __clap_app = clap::Command::new("test_not_protocol");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn into_app_for_update<'b>() -> clap::Command<'b> {
        let __clap_app = clap::Command::new("test_not_protocol");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
impl clap::FromArgMatches for Args {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = Args {
            config: __clap_arg_matches
                .get_one::<::std::ffi::OsString>("config")
                .map(|s| ::std::ops::Deref::deref(s))
                .ok_or_else(|| clap::Error::raw(
                    clap::ErrorKind::MissingRequiredArgument,
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The following required argument was not provided: {0}",
                                "config",
                            ),
                        );
                        res
                    }),
                ))
                .and_then(|s| ::std::result::Result::Ok::<
                    _,
                    clap::Error,
                >(::std::convert::From::from(s)))?,
            outfile: __clap_arg_matches
                .get_one::<::std::ffi::OsString>("outfile")
                .map(|s| ::std::ops::Deref::deref(s))
                .ok_or_else(|| clap::Error::raw(
                    clap::ErrorKind::MissingRequiredArgument,
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The following required argument was not provided: {0}",
                                "outfile",
                            ),
                        );
                        res
                    }),
                ))
                .and_then(|s| ::std::result::Result::Ok::<
                    _,
                    clap::Error,
                >(::std::convert::From::from(s)))?,
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if __clap_arg_matches.contains_id("config") {
            #[allow(non_snake_case)]
            let config = &mut self.config;
            *config = __clap_arg_matches
                .get_one::<::std::ffi::OsString>("config")
                .map(|s| ::std::ops::Deref::deref(s))
                .ok_or_else(|| clap::Error::raw(
                    clap::ErrorKind::MissingRequiredArgument,
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The following required argument was not provided: {0}",
                                "config",
                            ),
                        );
                        res
                    }),
                ))
                .and_then(|s| ::std::result::Result::Ok::<
                    _,
                    clap::Error,
                >(::std::convert::From::from(s)))?;
        }
        if __clap_arg_matches.contains_id("outfile") {
            #[allow(non_snake_case)]
            let outfile = &mut self.outfile;
            *outfile = __clap_arg_matches
                .get_one::<::std::ffi::OsString>("outfile")
                .map(|s| ::std::ops::Deref::deref(s))
                .ok_or_else(|| clap::Error::raw(
                    clap::ErrorKind::MissingRequiredArgument,
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The following required argument was not provided: {0}",
                                "outfile",
                            ),
                        );
                        res
                    }),
                ))
                .and_then(|s| ::std::result::Result::Ok::<
                    _,
                    clap::Error,
                >(::std::convert::From::from(s)))?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
impl clap::Args for Args {
    fn augment_args<'b>(__clap_app: clap::Command<'b>) -> clap::Command<'b> {
        {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("config")
                        .takes_value(true)
                        .value_name("CONFIG")
                        .required(true && clap::ArgAction::StoreValue.takes_values())
                        .value_parser(clap::builder::ValueParser::os_string())
                        .action(clap::ArgAction::StoreValue);
                    let arg = arg.short('c').long("config").value_name("FILE");
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("outfile")
                        .takes_value(true)
                        .value_name("OUTFILE")
                        .required(false && clap::ArgAction::StoreValue.takes_values())
                        .value_parser(clap::builder::ValueParser::os_string())
                        .action(clap::ArgAction::StoreValue);
                    let arg = arg
                        .short('o')
                        .long("outfile")
                        .value_name("FILE")
                        .default_value("test_not_protocol.jsonl");
                    arg
                });
            __clap_app
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command<'b>) -> clap::Command<'b> {
        {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("config")
                        .takes_value(true)
                        .value_name("CONFIG")
                        .required(false && clap::ArgAction::StoreValue.takes_values())
                        .value_parser(clap::builder::ValueParser::os_string())
                        .action(clap::ArgAction::StoreValue);
                    let arg = arg.short('c').long("config").value_name("FILE");
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("outfile")
                        .takes_value(true)
                        .value_name("OUTFILE")
                        .required(false && clap::ArgAction::StoreValue.takes_values())
                        .value_parser(clap::builder::ValueParser::os_string())
                        .action(clap::ArgAction::StoreValue);
                    let arg = arg
                        .short('o')
                        .long("outfile")
                        .value_name("FILE")
                        .default_value("test_not_protocol.jsonl");
                    arg
                });
            __clap_app
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Args {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Args",
            "config",
            &self.config,
            "outfile",
            &&self.outfile,
        )
    }
}
fn not_tls_cb(tls: &TlsHandshake) {
    {
        ::std::io::_print(format_args!("cb3\n"));
    };
    if let Ok(serialized) = serde_json::to_string(&tls) {
        let mut wtr = file.lock().unwrap();
        wtr.write_all(serialized.as_bytes()).unwrap();
        wtr.write_all(b"\n").unwrap();
    }
}
use retina_core::filter::actions::*;
use retina_core::subscription::{Trackable, Subscribable};
use retina_datatypes::{FromSession, Tracked, FromMbuf, StaticData, PacketList};
pub struct SubscribedWrapper;
impl Subscribable for SubscribedWrapper {
    type Tracked = TrackedWrapper;
}
pub struct TrackedWrapper {
    sessions: Vec<retina_core::protocols::Session>,
    mbufs: Vec<retina_core::Mbuf>,
    core_id: retina_core::CoreId,
}
impl Trackable for TrackedWrapper {
    type Subscribed = SubscribedWrapper;
    fn new(pdu: &retina_core::L4Pdu, core_id: retina_core::CoreId) -> Self {
        Self {
            sessions: ::alloc::vec::Vec::new(),
            mbufs: ::alloc::vec::Vec::new(),
            core_id,
        }
    }
    fn update(&mut self, pdu: &retina_core::L4Pdu, reassembled: bool) {}
    fn core_id(&self) -> &retina_core::CoreId {
        &self.core_id
    }
    fn buffer_packet(
        &mut self,
        pdu: &retina_core::L4Pdu,
        actions: &Actions,
        reassembled: bool,
    ) {
        if !reassembled && actions.data.intersects(ActionData::PacketCache) {
            self.mbufs.push(retina_core::Mbuf::new_ref(&pdu.mbuf));
        }
        if actions.data.intersects(ActionData::PacketTrack) {}
    }
    fn packets(&self) -> &Vec<retina_core::Mbuf> {
        &self.mbufs
    }
    fn drain_cached_packets(&mut self) {
        self.mbufs = ::alloc::vec::Vec::new();
    }
    fn drain_tracked_packets(&mut self) {}
    fn clear(&mut self) {
        self.drain_tracked_packets();
        self.drain_cached_packets();
        self.sessions = ::alloc::vec::Vec::new();
    }
    fn sessions(&self) -> &Vec<retina_core::protocols::Session> {
        &self.sessions
    }
    fn track_session(&mut self, session: retina_core::protocols::Session) {
        self.sessions.push(session);
    }
    fn parsers() -> retina_core::protocols::stream::ParserRegistry {
        retina_core::protocols::stream::ParserRegistry::from_strings(
            <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new(["tls"])),
        )
    }
}
pub fn filter() -> retina_core::filter::FilterFactory<TrackedWrapper> {
    fn packet_continue(
        mbuf: &retina_core::Mbuf,
        core_id: &retina_core::CoreId,
    ) -> Actions {
        let mut result = retina_core::filter::Actions::new();
        if let Ok(ethernet) = &retina_core::protocols::packet::Packet::parse_to::<
            retina_core::protocols::packet::ethernet::Ethernet,
        >(mbuf) {
            if let Ok(ipv4) = &retina_core::protocols::packet::Packet::parse_to::<
                retina_core::protocols::packet::ipv4::Ipv4,
            >(ethernet) {
                result
                    .push(
                        &Actions {
                            data: ActionData::from(1),
                            terminal_actions: ActionData::from(0),
                        },
                    );
            } else if let Ok(ipv6) = &retina_core::protocols::packet::Packet::parse_to::<
                retina_core::protocols::packet::ipv6::Ipv6,
            >(ethernet) {
                result
                    .push(
                        &Actions {
                            data: ActionData::from(1),
                            terminal_actions: ActionData::from(0),
                        },
                    );
            }
        }
        result
    }
    fn packet_filter(mbuf: &retina_core::Mbuf, tracked: &TrackedWrapper) -> Actions {
        let mut result = retina_core::filter::Actions::new();
        if let Ok(ethernet) = &retina_core::protocols::packet::Packet::parse_to::<
            retina_core::protocols::packet::ethernet::Ethernet,
        >(mbuf) {
            if let Ok(ipv4) = &retina_core::protocols::packet::Packet::parse_to::<
                retina_core::protocols::packet::ipv4::Ipv4,
            >(ethernet) {
                if let Ok(tcp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::tcp::Tcp,
                >(ipv4) {
                    result
                        .push(
                            &Actions {
                                data: ActionData::from(32),
                                terminal_actions: ActionData::from(0),
                            },
                        );
                } else if let Ok(udp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::udp::Udp,
                >(ipv4) {
                    result
                        .push(
                            &Actions {
                                data: ActionData::from(32),
                                terminal_actions: ActionData::from(0),
                            },
                        );
                }
                result
                    .push(
                        &Actions {
                            data: ActionData::from(144),
                            terminal_actions: ActionData::from(144),
                        },
                    );
            } else if let Ok(ipv6) = &retina_core::protocols::packet::Packet::parse_to::<
                retina_core::protocols::packet::ipv6::Ipv6,
            >(ethernet) {
                if let Ok(tcp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::tcp::Tcp,
                >(ipv6) {
                    result
                        .push(
                            &Actions {
                                data: ActionData::from(32),
                                terminal_actions: ActionData::from(0),
                            },
                        );
                } else if let Ok(udp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::udp::Udp,
                >(ipv6) {
                    result
                        .push(
                            &Actions {
                                data: ActionData::from(32),
                                terminal_actions: ActionData::from(0),
                            },
                        );
                }
                result
                    .push(
                        &Actions {
                            data: ActionData::from(144),
                            terminal_actions: ActionData::from(144),
                        },
                    );
            }
        }
        result
    }
    fn protocol_filter(
        conn: &retina_core::protocols::ConnData,
        tracked: &TrackedWrapper,
    ) -> Actions {
        let mut result = retina_core::filter::Actions::new();
        if let Ok(tcp) = &retina_core::protocols::stream::ConnData::parse_to::<
            retina_core::protocols::stream::conn::TcpCData,
        >(conn) {
            if match conn.service() {
                retina_core::protocols::stream::ConnParser::Dns { .. } => true,
                _ => false,
            } {
                {
                    ::std::io::_print(
                        format_args!("conn.service(): {0:?}\n", conn.service()),
                    );
                };
                result
                    .push(
                        &Actions {
                            data: ActionData::from(128),
                            terminal_actions: ActionData::from(0),
                        },
                    );
            } else if match conn.service() {
                retina_core::protocols::stream::ConnParser::Http { .. } => true,
                _ => false,
            } {
                result
                    .push(
                        &Actions {
                            data: ActionData::from(128),
                            terminal_actions: ActionData::from(0),
                        },
                    );
            } else if match conn.service() {
                retina_core::protocols::stream::ConnParser::Ssh { .. } => true,
                _ => false,
            } {
                result
                    .push(
                        &Actions {
                            data: ActionData::from(128),
                            terminal_actions: ActionData::from(0),
                        },
                    );
            }
        } else if let Ok(udp) = &retina_core::protocols::stream::ConnData::parse_to::<
            retina_core::protocols::stream::conn::UdpCData,
        >(conn) {
            if match conn.service() {
                retina_core::protocols::stream::ConnParser::Dns { .. } => true,
                _ => false,
            } {
                {
                    ::std::io::_print(
                        format_args!("conn.service(): {0:?}\n", conn.service()),
                    );
                };
                result
                    .push(
                        &Actions {
                            data: ActionData::from(128),
                            terminal_actions: ActionData::from(0),
                        },
                    );
            } else if match conn.service() {
                retina_core::protocols::stream::ConnParser::Quic { .. } => true,
                _ => false,
            } {
                result
                    .push(
                        &Actions {
                            data: ActionData::from(128),
                            terminal_actions: ActionData::from(0),
                        },
                    );
            }
        }
        result
    }
    fn session_filter(
        session: &retina_core::protocols::Session,
        conn: &retina_core::protocols::ConnData,
        tracked: &TrackedWrapper,
    ) -> Actions {
        let mut result = retina_core::filter::Actions::new();
        if let Ok(ipv4) = &retina_core::protocols::stream::ConnData::parse_to::<
            retina_core::protocols::stream::conn::Ipv4CData,
        >(conn) {
            if let Some(s) = TlsHandshake::from_session(session) {
                not_tls_cb(s);
            }
        } else if let Ok(ipv6) = &retina_core::protocols::stream::ConnData::parse_to::<
            retina_core::protocols::stream::conn::Ipv6CData,
        >(conn) {
            if let Some(s) = TlsHandshake::from_session(session) {
                not_tls_cb(s);
            }
        }
        result
    }
    fn packet_deliver(
        mbuf: &retina_core::Mbuf,
        conn: &retina_core::protocols::ConnData,
        tracked: &TrackedWrapper,
    ) {}
    fn connection_deliver(
        conn: &retina_core::protocols::ConnData,
        tracked: &TrackedWrapper,
    ) {}
    retina_core::filter::FilterFactory::new(
        "((ipv4)) or ((ipv6))",
        packet_continue,
        packet_filter,
        protocol_filter,
        session_filter,
        packet_deliver,
        connection_deliver,
    )
}
fn main() {
    let args = Args::parse();
    let config = load_config(&args.config);
    let mut runtime: Runtime<SubscribedWrapper> = Runtime::new(config, filter).unwrap();
    runtime.run();
    let mut wtr = file.lock().unwrap();
    wtr.flush().unwrap();
}
