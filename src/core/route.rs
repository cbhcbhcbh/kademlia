use crate::core::kbucket::KBucket;

#[derive(Debug)]
pub enum RouteTableNode {
    Leaf(KBucket),
    Branch {
        zero: Box<RouteTableNode>,
        one: Box<RouteTableNode>,
    }
}

impl RouteTableNode {
    pub fn new() -> Self {
        Self::Leaf(Default::default())
    }
}