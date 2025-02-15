use std::{net::IpAddr, sync::Arc};

use aquatic_common::access_list::AccessListArcSwap;

pub use aquatic_common::ValidUntil;
use aquatic_ws_protocol::{InfoHash, PeerId};

#[derive(Copy, Clone, Debug)]
pub enum IpVersion {
    V4,
    V6,
}

impl IpVersion {
    pub fn canonical_from_ip(ip: IpAddr) -> IpVersion {
        match ip {
            IpAddr::V4(_) => Self::V4,
            IpAddr::V6(addr) => match addr.octets() {
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, _, _, _, _] => Self::V4,
                _ => Self::V6,
            },
        }
    }
}

#[derive(Default, Clone)]
pub struct State {
    pub access_list: Arc<AccessListArcSwap>,
}

#[derive(Copy, Clone, Debug)]
pub struct PendingScrapeId(pub usize);

#[derive(Copy, Clone, Debug)]
pub struct ConsumerId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConnectionId(pub usize);

#[derive(Clone, Copy, Debug)]
pub struct ConnectionMeta {
    /// Index of socket worker responsible for this connection. Required for
    /// sending back response through correct channel to correct worker.
    pub out_message_consumer_id: ConsumerId,
    pub connection_id: ConnectionId,
    pub ip_version: IpVersion,
    pub pending_scrape_id: Option<PendingScrapeId>,
}

#[derive(Clone, Copy, Debug)]
pub enum SwarmControlMessage {
    ConnectionClosed {
        info_hash: InfoHash,
        peer_id: PeerId,
        ip_version: IpVersion,
    },
}
