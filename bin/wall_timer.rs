use rutile::*;

#[derive(Default)]
pub struct MyData {
    count: i32,
}

impl Data for MyData {
    fn initialize(&mut self, _: &Node<Self>) -> Result<()> {
        Ok(())
    }
}

fn timer_callback(_: Arc<Mutex<r2r::Node>>, data_mutex: Arc<Mutex<MyData>>) -> Result<()> {
    let mut data = data_mutex.lock().unwrap();

    data.count += 1;
    println!("count = {}", data.count);

    Ok(())
}

fn main() -> Result<()> {
    let mut node = Node::create("wall_timer_node", "")?;

    node.create_wall_timer(std::time::Duration::from_millis(1000), timer_callback)?;

    node.spin();

    Ok(())
}
