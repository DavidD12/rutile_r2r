use super::*;

pub trait Data: Default + 'static {
    fn initialize(&mut self, node: &Node<Self>) -> Result<()>;
}
