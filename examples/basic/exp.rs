Filter: tls, Datatypes: ["TlsHandshake", "ConnRecord"], Callback: "tls_cb"
Filter: dns, Datatypes: ["DnsTransaction", "ConnRecord"], Callback: "dns_cb"
Expecting 2 subsctription(s)
Tree Pkt (pass)
,`- 0: ethernet
   |- 1: ipv4
   |  |- 2: tcp -- A: Actions { data: ActionData[PacketContinue], terminal_actions: ActionData[] }
   |  `- 3: udp -- A: Actions { data: ActionData[PacketContinue], terminal_actions: ActionData[] } x
   `- 4: ipv6 x
      |- 5: tcp -- A: Actions { data: ActionData[PacketContinue], terminal_actions: ActionData[] }
      `- 6: udp -- A: Actions { data: ActionData[PacketContinue], terminal_actions: ActionData[] } x

Tree Pkt
,`- 0: ethernet
   |- 1: ipv4
   |  |- 2: tcp -- A: Actions { data: ActionData[ProtoFilter, UpdatePDU], terminal_actions: ActionData[] }
   |  `- 3: udp -- A: Actions { data: ActionData[ProtoFilter, UpdatePDU], terminal_actions: ActionData[] } x
   `- 4: ipv6 x
      |- 5: tcp -- A: Actions { data: ActionData[ProtoFilter, UpdatePDU], terminal_actions: ActionData[] }
      `- 6: udp -- A: Actions { data: ActionData[ProtoFilter, UpdatePDU], terminal_actions: ActionData[] } x

Tree Proto
,`- 0: ethernet
   |- 1: tcp
   |  |- 2: dns -- A: Actions { data: ActionData[SessionTrack, UpdatePDU, ConnDeliver], terminal_actions: ActionData[UpdatePDU, ConnDeliver] }
   |  `- 3: tls -- A: Actions { data: ActionData[SessionTrack, UpdatePDU, ConnDeliver], terminal_actions: ActionData[UpdatePDU, ConnDeliver] } x
   `- 4: udp x
      `- 5: dns -- A: Actions { data: ActionData[SessionTrack, UpdatePDU, ConnDeliver], terminal_actions: ActionData[UpdatePDU, ConnDeliver] }

Tree S
,`- 0: ethernet

Tree C (D)
,`- 0: ethernet
   |- 1: tcp
   |  |- 2: dns D: ( dns_cb(DnsTransaction, ConnRecord), )
   |  `- 3: tls D: ( tls_cb(TlsHandshake, ConnRecord), ) x
   `- 4: udp x
      `- 5: dns D: ( dns_cb(DnsTransaction, ConnRecord), )

Tree Pkt (D)
,`- 0: ethernet

Datatypes {
  TlsHandshake,
  DnsTransaction,
  ConnRecord,
}

Parsers {
  tls,
  dns,
}

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use retina_core::{config::default_config, Runtime};
use retina_datatypes::{ConnRecord, DnsTransaction, TlsHandshake};
use retina_filtergen::{filter, retina_main};
fn tls_cb(tls: &TlsHandshake, conn_record: &ConnRecord) {
    {
        ::std::io::_print(
            format_args!("Tls SNI: {0}, conn. metrics: {1:?}\n", tls.sni(), conn_record),
        );
    };
}
fn dns_cb(dns: &DnsTransaction, conn_record: &ConnRecord) {
    {
        ::std::io::_print(
            format_args!(
                "DNS query domain: {0}, conn. metrics: {1:?}\n",
                dns.query_domain(),
                conn_record,
            ),
        );
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
    connrecord: ConnRecord,
}
impl Trackable for TrackedWrapper {
    type Subscribed = SubscribedWrapper;
    fn new(pdu: &retina_core::L4Pdu, core_id: retina_core::CoreId) -> Self {
        Self {
            sessions: ::alloc::vec::Vec::new(),
            mbufs: ::alloc::vec::Vec::new(),
            core_id,
            connrecord: ConnRecord::new(pdu),
        }
    }
    fn update(&mut self, pdu: &retina_core::L4Pdu, reassembled: bool) {
        self.connrecord.update(pdu, reassembled);
    }
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
        self.connrecord.clear();
    }
    fn sessions(&self) -> &Vec<retina_core::protocols::Session> {
        &self.sessions
    }
    fn track_session(&mut self, session: retina_core::protocols::Session) {
        self.sessions.push(session);
    }
    fn parsers() -> retina_core::protocols::stream::ParserRegistry {
        retina_core::protocols::stream::ParserRegistry::from_strings(
            <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new(["tls", "dns"])),
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
                } else if let Ok(udp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::udp::Udp,
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
                } else if let Ok(udp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::udp::Udp,
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
                if let Ok(tcp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::tcp::Tcp,
                >(ipv4) {
                    result
                        .push(
                            &Actions {
                                data: ActionData::from(544),
                                terminal_actions: ActionData::from(0),
                            },
                        );
                } else if let Ok(udp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::udp::Udp,
                >(ipv4) {
                    result
                        .push(
                            &Actions {
                                data: ActionData::from(544),
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
                                data: ActionData::from(544),
                                terminal_actions: ActionData::from(0),
                            },
                        );
                } else if let Ok(udp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::udp::Udp,
                >(ipv6) {
                    result
                        .push(
                            &Actions {
                                data: ActionData::from(544),
                                terminal_actions: ActionData::from(0),
                            },
                        );
                }
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
                result
                    .push(
                        &Actions {
                            data: ActionData::from(2816),
                            terminal_actions: ActionData::from(2560),
                        },
                    );
            } else if match conn.service() {
                retina_core::protocols::stream::ConnParser::Tls { .. } => true,
                _ => false,
            } {
                result
                    .push(
                        &Actions {
                            data: ActionData::from(2816),
                            terminal_actions: ActionData::from(2560),
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
                result
                    .push(
                        &Actions {
                            data: ActionData::from(2816),
                            terminal_actions: ActionData::from(2560),
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
    ) {
        if let Ok(tcp) = &retina_core::protocols::stream::ConnData::parse_to::<
            retina_core::protocols::stream::conn::TcpCData,
        >(conn) {
            if match conn.service() {
                retina_core::protocols::stream::ConnParser::Dns { .. } => true,
                _ => false,
            } {
                if let Some(s) = DnsTransaction::from_sessionlist(tracked.sessions()) {
                    dns_cb(s, &tracked.connrecord);
                }
            } else if match conn.service() {
                retina_core::protocols::stream::ConnParser::Tls { .. } => true,
                _ => false,
            } {
                if let Some(s) = TlsHandshake::from_sessionlist(tracked.sessions()) {
                    tls_cb(s, &tracked.connrecord);
                }
            }
        } else if let Ok(udp) = &retina_core::protocols::stream::ConnData::parse_to::<
            retina_core::protocols::stream::conn::UdpCData,
        >(conn) {
            if match conn.service() {
                retina_core::protocols::stream::ConnParser::Dns { .. } => true,
                _ => false,
            } {
                if let Some(s) = DnsTransaction::from_sessionlist(tracked.sessions()) {
                    dns_cb(s, &tracked.connrecord);
                }
            }
        }
    }
    retina_core::filter::FilterFactory::new(
        "((ipv4) and (tcp)) or ((ipv4) and (udp)) or ((ipv6) and (tcp)) or ((ipv6) and (udp))",
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
