//! SSH transaction components.
//! 

use ssh_parser::{SshDisconnectReason};

/// A parsed SSH Protocol Version Exchange message.
#[derive(Clone, Debug, Serialize)]
pub struct SshVersionExchange {
    pub protoversion: String,
    pub softwareversion: String,
    pub comments: Option<String>, // comments are optional
}

/// A parsed SSH Key Exchange message.
#[derive(Clone, Debug, PartialEq)]
pub struct SshKeyExchange {
    pub ssh_msg_kexinit: Vec<u8>,
    pub cookie: Vec<u8>,
    pub kex_algs: Vec<String>,
    pub server_host_key_algs: Vec<String>,
    pub encryption_algs_client_to_server: Vec<String>,
    pub encryption_algs_server_to_client: Vec<String>,
    pub mac_algs_client_to_server: Vec<String>,
    pub mac_algs_server_to_client: Vec<String>,
    pub compression_algs_client_to_server: Vec<String>,
    pub compression_algs_server_to_client: Vec<String>,
    pub languages_client_to_server: Option<Vec<String>>,
    pub languages_server_to_client: Option<Vec<String>>,
    pub first_kex_packet_follows: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SshDHClient {
    pub ssh_msg_kexdh_init: Vec<u8>,
    pub e: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct SshDHServerResponse {
    pub ssh_msg_kexdh_reply: Vec<u8>,
    pub pubkey_and_certs: Vec<u8>,
    pub f: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct SshNewKeys {
    pub ssh_msg_newkeys: Vec<u8>,
}

pub struct SshServiceRequest {
    pub ssh_msg_service_request: Vec<u8>,
    pub service_name: String,
}

pub struct SshServiceAccept {
    pub ssh_msg_service_accept: Vec<u8>,
    pub service_name: String,
}

/// A parsed SSH Disconnection message.
pub struct SshDisconnect {
    pub ssh_msg_disconnect: Vec<u8>,
    pub reason_code: u32,
    pub description: String,
    pub language_tag: String,
}