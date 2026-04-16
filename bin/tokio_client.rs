use r2r::{QosProfile, example_interfaces::srv::AddTwoInts};
use rutile_r2r::tokio::*;

pub struct Data {
    pub count: i64,
    pub client: Client<AddTwoInts::Service>,
}

// Lock All
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

pub async fn timer_callback_light(data_mutex: TMutex<Data>) {
    // 1) short critical section
    let (client, request) = {
        let mut data = data_mutex.lock().await;
        let request = AddTwoInts::Request {
            a: data.count,
            b: data.count * 10,
        };
        data.count += 1;
        (data.client.clone(), request)
    };
    // 2) async call without holding the lock
    let response = client.call(request).await;
    println!("{:?}", response);
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut node = Node::create("minimal_client", "")?;
    //
    let data = Data {
        count: 0,
        client: node.create_client("add_two_ints", QosProfile::default())?,
    };
    let data_mutex = TMutex::create(data);
    //
    node.create_wall_timer_1(
        std::time::Duration::from_secs(1),
        timer_callback,
        data_mutex,
    )?;
    //
    node.spin(std::time::Duration::from_millis(10));
    //
    Ok(())
}
