use std::sync::{Arc, Mutex};

use r2r::{example_interfaces::srv::AddTwoInts, QosProfile};
use rutile_r2r::multi::*;

#[derive(Clone)]
pub struct Data {
    pub count: Arc<Mutex<i64>>,
    pub client: Client<AddTwoInts::Service>,
}

fn timer_callback(data: Data) {
    let mut count = data.count.lock().unwrap_or_else(|e| e.into_inner());
    let request = AddTwoInts::Request {
        a: *count,
        b: *count * 10,
    };
    *count += 1;
    drop(count);

    println!("request: {:?}", request);
    match data.client.call_blocking(request) {
        Ok(response) => println!("response: {:?}", response),
        Err(e) => eprintln!("error: {:?}", e),
    }
}

fn main() -> Result<()> {
    let mut node = Node::create("minimal_client", "")?;
    let data = Data {
        count: Arc::new(Mutex::new(0)),
        client: node.create_client("add_two_ints", QosProfile::default())?,
    };

    node.create_wall_timer_1(std::time::Duration::from_secs(1), timer_callback, data)?;
    node.spin(std::time::Duration::from_millis(10));
    Ok(())
}
