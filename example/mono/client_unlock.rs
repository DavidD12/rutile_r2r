use r2r::{example_interfaces::srv::AddTwoInts, QosProfile};
use rutile_r2r::mono::*;

#[derive(Default, Clone)]
pub struct MyData {
    pub i: i64,
}

impl MyData {
    pub fn new(i: i64) -> Self {
        Self { i }
    }
}

#[derive(Clone)]
pub struct Ros {
    pub client: Client<AddTwoInts::Service>,
}

fn client_timer_callback(data: MyData, ros: Ros) {
    let request = AddTwoInts::Request { a: data.i, b: 1 };
    println!("request: {:?}", request);
    match ros.client.call_blocking(request) {
        Ok(response) => println!("response: {:?}", response),
        Err(e) => eprintln!("error: {:?}", e),
    }
}

fn timer_callback(data: MyData) {
    println!("hello: {}", data.i);
}

fn main() -> Result<()> {
    let data = MyData::new(10);
    let mut node = Node::create("sync_core", "")?;
    let ros = Ros {
        client: node.create_client("/add", QosProfile::default())?,
    };

    node.create_wall_timer_2(
        std::time::Duration::from_secs(1),
        client_timer_callback,
        data.clone(),
        ros.clone(),
    )?;

    node.create_wall_timer_1(std::time::Duration::from_secs(1), timer_callback, data)?;

    node.spin(std::time::Duration::from_millis(10));
    Ok(())
}
