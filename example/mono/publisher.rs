use std::sync::{Arc, Mutex};

use r2r::QosProfile;
use rutile_r2r::mono::*;

#[derive(Clone)]
pub struct Data {
    pub count: Arc<Mutex<isize>>,
    pub publisher: Publisher<r2r::std_msgs::msg::String>,
}

fn timer_callback(data: Data) {
    let mut count = data.count.lock().unwrap_or_else(|e| e.into_inner());
    let message = r2r::std_msgs::msg::String {
        data: format!("Hello, world! {}", *count),
    };
    println!("Publishing {:?}", message);
    *count += 1;
    data.publisher.publish(&message);
}

fn main() -> Result<()> {
    let mut node = Node::create("minimal_publisher", "")?;
    let data = Data {
        count: Arc::new(Mutex::new(0)),
        publisher: node.create_publisher("topic", QosProfile::default())?,
    };

    node.create_wall_timer(std::time::Duration::from_millis(500), timer_callback, data)?;
    node.spin(std::time::Duration::from_millis(10));
    Ok(())
}
