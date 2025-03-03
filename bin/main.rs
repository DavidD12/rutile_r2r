use rutile::*;

#[derive(Default)]
pub struct MyData {}

impl Data for MyData {
    fn initialize(&mut self, _: &Node<Self>) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut node: Node<MyData> = Node::create("rutile_node", "")?;

    node.spin();

    Ok(())
}
