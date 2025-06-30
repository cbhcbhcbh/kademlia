use std::net::IpAddr;

use sha1::{Digest, Sha1};

use crate::{core::node, K_REPLICATIONS};

pub type NodeId = [u8; K_REPLICATIONS];

#[derive(Debug)]
pub struct Node {
    node_id: NodeId,
    ip_addr: IpAddr,
    port: u16,
}

impl Node {
    pub fn new(node_id: NodeId, ip_addr: IpAddr, port: u16) -> Self {
        Node {
            node_id,
            ip_addr,
            port,
        }
    }

    pub fn from_random_node_id(ip_addr: IpAddr, port: u16) -> Self {
        let mut node_id = [0; K_REPLICATIONS];
        rand::fill(&mut node_id);
        Node::new(node_id, ip_addr, port)
    }

    pub fn from_ip_addr_sha1(ip_addr: IpAddr, port: u16) -> Self {
        let mut sha1 = Sha1::new();
        match ip_addr {
            IpAddr::V4(a) => sha1.update(a.octets()),
            IpAddr::V6(a) => sha1.update(a.octets()),
        }
        sha1.update(port.to_le_bytes());
        let hash = sha1.finalize();
        let node_id = core::array::from_fn(|i| hash[i]);
        Node::new(node_id, ip_addr, port)
    }
}
