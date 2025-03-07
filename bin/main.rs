use rutile::*;

#[derive(Default)]
pub struct MyData {}

impl Data for MyData {
    fn initialize(&mut self, node: &Node<Self>) -> Result<()> {
        let p = node.get_parameter_with_default::<i64>("period", 100 as i64)?;
        let p = node.get_parameter::<i64>("period")?;

        Ok(())
    }
}

fn main() -> Result<()> {
    let mut node: Node<MyData> = Node::create("rutile_node", "")?;

    node.spin();

    Ok(())
}
