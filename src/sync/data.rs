use super::*;
use crate::Result;

pub trait Data: Default + 'static {
    fn initialize(&mut self, node: &Node<Self>) -> Result<()>;
}
