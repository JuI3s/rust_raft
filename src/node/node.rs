use crate::node::interface::*;
use crate::node::overlay_node::*;
pub struct Node<T: IOverlayNode> {
    role: Role,
    overlay_node: Box<T>,
}

impl Node<OverlayNode> {
    fn new() -> Self {
        Node {
            role: Role::Follower,
            overlay_node: Box::new(OverlayNode::new()),
        }
    }
}

impl INode for Node<OverlayNode> {
    fn convert_to_candidate(&mut self) {}
}
