use r2r::{QosProfile, example_interfaces::srv::AddTwoInts};
use rutile::*;

#[derive(Default)]
pub struct MyData {
    pub a: i64,
    pub b: i64,
    pub client: ClientSync<AddTwoInts::Service>,
}

impl Data for MyData {
    fn initialize(&mut self, node: &Node<Self>) -> Result<()> {
        //
        self.a = 0;
        self.b = 0;
        self.client =
            node.create_client_sync::<AddTwoInts::Service>("/add", QosProfile::default())?;

        Ok(())
    }
}

fn timer_callback(_: Arc<Mutex<r2r::Node>>, data_mutex: Arc<Mutex<MyData>>) -> Result<()> {
    let mut data = data_mutex.lock().unwrap();

    let request = AddTwoInts::Request {
        a: data.a,
        b: data.b,
    };
    println!("resquest: {:?}", request);
    // let response = data.client.call(request)?;
    // println!("response: {:?}", response);

    data.a += 1;
    data.b += 2;

    Ok(())
}

fn main() -> Result<()> {
    let mut node: Node<MyData> = Node::create("wall_timer_node", "")?;
    node.create_wall_timer(std::time::Duration::from_millis(1000), timer_callback)?;

    node.spin();

    Ok(())
}
