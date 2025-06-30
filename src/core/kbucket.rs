use bit_vec::BitVec;

use std::{collections::VecDeque, net::IpAddr};

use crate::core::node::NodeId;

use super::node::Node;

#[derive(Debug, Default)]
pub struct KBucket {
    prefix_bits: BitVec,
    queue: VecDeque<Node>,
}

impl KBucket {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_node(&mut self, node_id: NodeId, ip_addr: IpAddr, port: u16) {
        
    }
}
