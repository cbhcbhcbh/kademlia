use std::net::IpAddr;

use crate::core::{
    kbucket::{self, KBucket},
    node::{self, NodeId, get_node_id_bit},
};

#[derive(Debug)]
pub enum RouteTableEnrty {
    Leaf(KBucket),
    Branch {
        zero: Box<RouteTableEnrty>,
        one: Box<RouteTableEnrty>,
    },
}

impl RouteTableEnrty {
    pub fn new() -> Self {
        Self::Leaf(Default::default())
    }

    pub fn add_node(&mut self, bit: usize, node_id: NodeId, addr: IpAddr, port: u16) {
        match self {
            RouteTableEnrty::Leaf(kbucket) => match kbucket.add_node(node_id, addr, port) {
                super::kbucket::KBucketAddResult::Added => (),
                super::kbucket::KBucketAddResult::Replaced(zero, one) => {
                    *self = RouteTableEnrty::Branch {
                        zero: Box::new(RouteTableEnrty::Leaf(zero)),
                        one: Box::new(RouteTableEnrty::Leaf(one)),
                    };
                }
            },
            RouteTableEnrty::Branch { zero, one } => {
                if get_node_id_bit(&node_id, bit) {
                    one.add_node(bit + 1, node_id, addr, port);
                } else {
                    zero.add_node(bit + 1, node_id, addr, port);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{
        K_REPLICATIONS,
        core::{kbucket::test::rand_ip_addr_and_port, node::Node, route},
    };

    #[test]
    fn test_route_table_add_node() {
        for _ in 0..100 {
            let mut route_table = RouteTableEnrty::new();
            (0..K_REPLICATIONS).for_each(|i| {
                let (addr, port) = rand_ip_addr_and_port();
                let node = Node::from_random_node_id(addr, port);
                route_table.add_node(0, node.node_id(), addr, port);
                assert!(matches!(route_table, RouteTableEnrty::Leaf(_)));
                match &route_table {
                    RouteTableEnrty::Leaf(b) => assert_eq!(b.len(), i + 1),
                    RouteTableEnrty::Branch { .. } => unreachable!(),
                }
            });

            let (addr, port) = rand_ip_addr_and_port();
            let node = Node::from_random_node_id(addr, port);
            route_table.add_node(0, node.node_id(), addr, port);
            match &route_table {
                RouteTableEnrty::Leaf(_) => unreachable!(),
                RouteTableEnrty::Branch { zero, one } => {
                    match &**zero {
                        RouteTableEnrty::Leaf(b) => {
                            for node in b.queue().iter() {
                                assert!(!node.get_node_id_bit(0));
                            }
                        }
                        RouteTableEnrty::Branch { .. } => unreachable!(),
                    }
                    match &**one {
                        RouteTableEnrty::Leaf(b) => {
                            for node in b.queue().iter() {
                                assert!(node.get_node_id_bit(0));
                            }
                        }
                        RouteTableEnrty::Branch { .. } => unreachable!(),
                    }
                }
            }
        }
    }
}
