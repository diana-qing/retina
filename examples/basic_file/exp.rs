Tree Pkt (pass)
,`- 0: ethernet
   |- 1: ipv4
   |  `- 2: tcp -- A: Actions { data: ActionData[PacketContinue], terminal_actions: ActionData[] }
   `- 3: ipv6 x
      `- 4: tcp -- A: Actions { data: ActionData[PacketContinue], terminal_actions: ActionData[] }

Tree Pkt
,`- 0: ethernet
   |- 1: ipv4 -- A: Actions { data: ActionData[ProtoFilter], terminal_actions: ActionData[] }
   `- 2: ipv6 -- A: Actions { data: ActionData[ProtoFilter], terminal_actions: ActionData[] } x

Tree Proto
,`- 0: ethernet
   `- 1: tls -- A: Actions { data: ActionData[SessionFilter], terminal_actions: ActionData[] }

Tree S
,`- 0: ethernet
   `- 1: tls
      |- 2: tls.sni matches ^.*\.com$ D: ( tls_cb(TlsHandshake, FilterStr), )
      |- 3: tls.sni matches ^.*\.net$ D: ( tls_cb(TlsHandshake, FilterStr), )
      `- 4: tls.sni matches ^.*\.edu$ D: ( tls_cb(TlsHandshake, FilterStr), )

Tree C (D)
,`- 0: ethernet

Tree Pkt (D)
,`- 0: ethernet

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
use retina_core::{config::default_config, Runtime};
use retina_datatypes::{FilterStr, TlsHandshake};
use retina_filtergen::subscription;
fn tls_cb(tls: &TlsHandshake, filter_str: &FilterStr) {
    {
        ::std::io::_print(format_args!("Matched filter {0}: {1:?}\n", filter_str, tls));
    };
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
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct RE0 {
    __private_field: (),
}
#[doc(hidden)]
#[allow(non_upper_case_globals)]
static RE0: RE0 = RE0 { __private_field: () };
impl ::lazy_static::__Deref for RE0 {
    type Target = regex::Regex;
    fn deref(&self) -> &regex::Regex {
        #[inline(always)]
        fn __static_ref_initialize() -> regex::Regex {
            regex::Regex::new("^.*\\.com$").unwrap()
        }
        #[inline(always)]
        fn __stability() -> &'static regex::Regex {
            static LAZY: ::lazy_static::lazy::Lazy<regex::Regex> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for RE0 {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct RE1 {
    __private_field: (),
}
#[doc(hidden)]
#[allow(non_upper_case_globals)]
static RE1: RE1 = RE1 { __private_field: () };
impl ::lazy_static::__Deref for RE1 {
    type Target = regex::Regex;
    fn deref(&self) -> &regex::Regex {
        #[inline(always)]
        fn __static_ref_initialize() -> regex::Regex {
            regex::Regex::new("^.*\\.net$").unwrap()
        }
        #[inline(always)]
        fn __stability() -> &'static regex::Regex {
            static LAZY: ::lazy_static::lazy::Lazy<regex::Regex> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for RE1 {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct RE2 {
    __private_field: (),
}
#[doc(hidden)]
#[allow(non_upper_case_globals)]
static RE2: RE2 = RE2 { __private_field: () };
impl ::lazy_static::__Deref for RE2 {
    type Target = regex::Regex;
    fn deref(&self) -> &regex::Regex {
        #[inline(always)]
        fn __static_ref_initialize() -> regex::Regex {
            regex::Regex::new("^.*\\.edu$").unwrap()
        }
        #[inline(always)]
        fn __stability() -> &'static regex::Regex {
            static LAZY: ::lazy_static::lazy::Lazy<regex::Regex> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for RE2 {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
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
                if let Ok(tcp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::tcp::Tcp,
                >(ipv4) {
                    result
                        .push(
                            &Actions {
                                data: ActionData::from(1),
                                terminal_actions: ActionData::from(0),
                            },
                        );
                }
            } else if let Ok(ipv6) = &retina_core::protocols::packet::Packet::parse_to::<
                retina_core::protocols::packet::ipv6::Ipv6,
            >(ethernet) {
                if let Ok(tcp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::tcp::Tcp,
                >(ipv6) {
                    result
                        .push(
                            &Actions {
                                data: ActionData::from(1),
                                terminal_actions: ActionData::from(0),
                            },
                        );
                }
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
                result
                    .push(
                        &Actions {
                            data: ActionData::from(32),
                            terminal_actions: ActionData::from(0),
                        },
                    );
            } else if let Ok(ipv6) = &retina_core::protocols::packet::Packet::parse_to::<
                retina_core::protocols::packet::ipv6::Ipv6,
            >(ethernet) {
                result
                    .push(
                        &Actions {
                            data: ActionData::from(32),
                            terminal_actions: ActionData::from(0),
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
        if match conn.service() {
            retina_core::protocols::stream::ConnParser::Tls { .. } => true,
            _ => false,
        } {
            result
                .push(
                    &Actions {
                        data: ActionData::from(64),
                        terminal_actions: ActionData::from(0),
                    },
                );
        }
        result
    }
    fn session_filter(
        session: &retina_core::protocols::Session,
        conn: &retina_core::protocols::ConnData,
        tracked: &TrackedWrapper,
    ) -> Actions {
        let mut result = retina_core::filter::Actions::new();
        if let retina_core::protocols::stream::SessionData::Tls(tls) = &session.data {
            if RE0.is_match(&tls.sni()[..]) {
                if let Some(s) = TlsHandshake::from_session(session) {
                    tls_cb(s, &"tls.sni ~ '^.*\\.com$'");
                }
            }
            if RE1.is_match(&tls.sni()[..]) {
                if let Some(s) = TlsHandshake::from_session(session) {
                    tls_cb(s, &"tls.sni ~ '^.*\\.net$'");
                }
            }
            if RE2.is_match(&tls.sni()[..]) {
                if let Some(s) = TlsHandshake::from_session(session) {
                    tls_cb(s, &"tls.sni ~ '^.*\\.edu$'");
                }
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
        "((ipv4) and (tcp)) or ((ipv6) and (tcp))",
        packet_continue,
        packet_filter,
        protocol_filter,
        session_filter,
        packet_deliver,
        connection_deliver,
    )
}
fn main() {
    let config = default_config();
    let mut runtime: Runtime<SubscribedWrapper> = Runtime::new(config, filter).unwrap();
    runtime.run();
}
