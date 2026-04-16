use r2r::{QosProfile, example_interfaces::srv::AddTwoInts};
use rutile_r2r::future_mono::*;

#[derive(Default, Clone)]
pub struct MyData {
    pub i: i64,
}

impl MyData {
    pub fn new(i: i64) -> Self {
        Self { i }
    }
}

async fn client_timer_callback(data: FMutex<MyData>, ros: Ros) {
    println!("-------------------------");
    let request = AddTwoInts::Request {
        a: data.lock().await.i,
        b: 1,
    };
    println!("request: {:?}", request);
    let response = match ros.client.call(request).await {
        Ok(response) => response,
        Err(e) => {
            eprintln!("error: {:?}", e);
            return;
        }
    };
    data.lock().await.i = response.sum
}

#[derive(Clone)]
pub struct Ros {
    pub client: Client<AddTwoInts::Service>,
}

async fn timer_callback(data: FMutex<MyData>) {
    println!("-------------------------");
    let i = { data.lock().await.i };
    println!("hello: {}", i);
}

fn main() -> Result<()> {
    let data = MyData::new(10);
    let data_mutex = FMutex::create(data);

    let mut node = Node::create("async_core", "")?;
    let ros = Ros {
        client: node.create_client("/add", QosProfile::default())?,
    };

    node.create_wall_timer_2(
        std::time::Duration::from_secs(1),
        client_timer_callback,
        data_mutex.clone(),
        ros.clone(),
    )?;

    node.create_wall_timer_1(
        std::time::Duration::from_secs(1),
        timer_callback,
        data_mutex.clone(),
    )?;

    node.spin(std::time::Duration::from_millis(10));

    Ok(())
}
