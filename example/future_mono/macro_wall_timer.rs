use rutile_r2r::future_mono::*;

async fn timer_0() {
    println!("future_mono wall timer arity 0");
}

async fn timer_2(a: i32, b: i32) {
    println!("future_mono wall timer arity 2: {} {}", a, b);
}

async fn timer_5(a: i32, b: i32, c: i32, d: i32, e: i32) {
    println!("future_mono wall timer arity 5: {} {} {} {} {}", a, b, c, d, e);
}

fn main() -> Result<()> {
    let mut node = Node::create("macro_wall_timer_future_mono", "")?;

    rutile_r2r::create_wall_timer!(node, std::time::Duration::from_millis(700), timer_0)?;
    rutile_r2r::create_wall_timer!(
        node,
        std::time::Duration::from_millis(900),
        timer_2,
        10,
        20,
    )?;
    rutile_r2r::create_wall_timer!(
        node,
        std::time::Duration::from_millis(1100),
        timer_5,
        1,
        2,
        3,
        4,
        5,
    )?;

    node.spin(std::time::Duration::from_millis(10));
    Ok(())
}
