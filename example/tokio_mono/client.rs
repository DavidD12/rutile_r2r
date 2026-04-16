use r2r::{QosProfile, example_interfaces::srv::AddTwoInts};
use rutile_r2r::tokio_mono::*;

pub struct Data {
    pub count: i64,
    pub client: Client<AddTwoInts::Service>,
}

async fn timer_callback(data_mutex: TMutex<Data>) {
    let mut data = data_mutex.lock().await;

    let request = AddTwoInts::Request {
        a: data.count,
        b: data.count * 10,
    };
    let response = match data.client.call(request).await {
        Ok(response) => response,
        Err(e) => {
            eprintln!("error: {:?}", e);
            return;
        }
    };
    println!("{:?}", response);
    data.count += response.sum;
}

fn main() -> Result<()> {
    let mut node = Node::create("minimal_client", "")?;
    let data = Data {
        count: 0,
        client: node.create_client("add_two_ints", QosProfile::default())?,
    };
    let data_mutex = TMutex::create(data);

    node.create_wall_timer_1(
        std::time::Duration::from_secs(1),
        timer_callback,
        data_mutex,
    )?;

    node.spin(std::time::Duration::from_millis(10));
    Ok(())
}
