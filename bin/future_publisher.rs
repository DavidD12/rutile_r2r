use r2r::QosProfile;
use rutile::future::*;

#[derive(Default)]
pub struct Data {
    pub count: isize,
    pub publisher: Publisher<r2r::std_msgs::msg::String>,
}

async fn timer_callback(data_mutex: FutureMutex<Data>) {
    let mut data = data_mutex.lock().await;
    //
    let message = r2r::std_msgs::msg::String {
        data: format!("Hello, world! {}", data.count),
    };
    println!("Publishing {:?}", message);
    data.count += 1;
    //
    let _ = data.publisher.publish(&message);
}

fn main() -> Result<()> {
    let mut node = Node::create("minimal_publisher", "")?;
    //
    let data = Data {
        count: 0,
        publisher: node.create_publisher("topic", QosProfile::default())?,
    };
    //
    let data_mutex = FutureMutex::create(data);
    //
    node.create_wall_timer_1(
        std::time::Duration::from_millis(500),
        timer_callback,
        data_mutex,
    )?;
    //
    node.spin();
    //
    Ok(())
}
