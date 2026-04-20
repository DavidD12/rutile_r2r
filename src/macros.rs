#[macro_export]
macro_rules! create_wall_timer {
    ($node:expr, $period:expr, $callback:expr $(,)?) => {
        ($node).create_wall_timer_0($period, $callback)
    };
    ($node:expr, $period:expr, $callback:expr, $data:expr $(,)?) => {
        ($node).create_wall_timer_1($period, $callback, $data)
    };
    ($node:expr, $period:expr, $callback:expr, $data_1:expr, $data_2:expr $(,)?) => {
        ($node).create_wall_timer_2($period, $callback, $data_1, $data_2)
    };
    ($node:expr, $period:expr, $callback:expr, $data_1:expr, $data_2:expr, $data_3:expr $(,)?) => {
        ($node).create_wall_timer_3($period, $callback, $data_1, $data_2, $data_3)
    };
    ($node:expr, $period:expr, $callback:expr, $data_1:expr, $data_2:expr, $data_3:expr, $data_4:expr $(,)?) => {
        ($node).create_wall_timer_4($period, $callback, $data_1, $data_2, $data_3, $data_4)
    };
    ($node:expr, $period:expr, $callback:expr, $data_1:expr, $data_2:expr, $data_3:expr, $data_4:expr, $data_5:expr $(,)?) => {
        ($node).create_wall_timer_5(
            $period, $callback, $data_1, $data_2, $data_3, $data_4, $data_5,
        )
    };
    ($($tt:tt)*) => {
        compile_error!(
            "Invalid `create_wall_timer!` invocation. Expected one of:\n\
             - create_wall_timer!(node, period, callback)\n\
             - create_wall_timer!(node, period, callback, data)\n\
             - create_wall_timer!(node, period, callback, data1, data2)\n\
             - create_wall_timer!(node, period, callback, data1, data2, data3)\n\
             - create_wall_timer!(node, period, callback, data1, data2, data3, data4)\n\
             - create_wall_timer!(node, period, callback, data1, data2, data3, data4, data5)"
        )
    };
}

#[macro_export]
macro_rules! create_subscription {
    ($node:expr, $topic:expr, $qos_profile:expr, $callback:expr $(,)?) => {
        ($node).create_subscription_0($topic, $qos_profile, $callback)
    };
    ($node:expr, $topic:expr, $qos_profile:expr, $callback:expr, $data:expr $(,)?) => {
        ($node).create_subscription_1($topic, $qos_profile, $callback, $data)
    };
    ($node:expr, $topic:expr, $qos_profile:expr, $callback:expr, $data_1:expr, $data_2:expr $(,)?) => {
        ($node).create_subscription_2($topic, $qos_profile, $callback, $data_1, $data_2)
    };
    ($node:expr, $topic:expr, $qos_profile:expr, $callback:expr, $data_1:expr, $data_2:expr, $data_3:expr $(,)?) => {
        ($node).create_subscription_3(
            $topic,
            $qos_profile,
            $callback,
            $data_1,
            $data_2,
            $data_3,
        )
    };
    ($node:expr, $topic:expr, $qos_profile:expr, $callback:expr, $data_1:expr, $data_2:expr, $data_3:expr, $data_4:expr $(,)?) => {
        ($node).create_subscription_4(
            $topic,
            $qos_profile,
            $callback,
            $data_1,
            $data_2,
            $data_3,
            $data_4,
        )
    };
    ($node:expr, $topic:expr, $qos_profile:expr, $callback:expr, $data_1:expr, $data_2:expr, $data_3:expr, $data_4:expr, $data_5:expr $(,)?) => {
        ($node).create_subscription_5(
            $topic,
            $qos_profile,
            $callback,
            $data_1,
            $data_2,
            $data_3,
            $data_4,
            $data_5,
        )
    };
    ($($tt:tt)*) => {
        compile_error!(
            "Invalid `create_subscription!` invocation. Expected one of:\n\
             - create_subscription!(node, topic, qos_profile, callback)\n\
             - create_subscription!(node, topic, qos_profile, callback, data)\n\
             - create_subscription!(node, topic, qos_profile, callback, data1, data2)\n\
             - create_subscription!(node, topic, qos_profile, callback, data1, data2, data3)\n\
             - create_subscription!(node, topic, qos_profile, callback, data1, data2, data3, data4)\n\
             - create_subscription!(node, topic, qos_profile, callback, data1, data2, data3, data4, data5)"
        )
    };
}

#[macro_export]
macro_rules! create_service {
    ($node:expr, $service_ty:ty, $service_name:expr, $qos_profile:expr, $callback:expr $(,)?) => {
        ($node).create_service_typed_0::<$service_ty, _, _>(
            $service_name,
            $qos_profile,
            $callback,
        )
    };
    ($node:expr, $service_ty:ty, $service_name:expr, $qos_profile:expr, $callback:expr, $data:expr $(,)?) => {
        ($node).create_service_typed_1::<$service_ty, _, _, _>(
            $service_name,
            $qos_profile,
            $callback,
            $data,
        )
    };
    ($node:expr, $service_ty:ty, $service_name:expr, $qos_profile:expr, $callback:expr, $data_1:expr, $data_2:expr $(,)?) => {
        ($node).create_service_typed_2::<$service_ty, _, _, _, _>(
            $service_name,
            $qos_profile,
            $callback,
            $data_1,
            $data_2,
        )
    };
    ($node:expr, $service_ty:ty, $service_name:expr, $qos_profile:expr, $callback:expr, $data_1:expr, $data_2:expr, $data_3:expr $(,)?) => {
        ($node).create_service_typed_3::<$service_ty, _, _, _, _, _>(
            $service_name,
            $qos_profile,
            $callback,
            $data_1,
            $data_2,
            $data_3,
        )
    };
    ($node:expr, $service_ty:ty, $service_name:expr, $qos_profile:expr, $callback:expr, $data_1:expr, $data_2:expr, $data_3:expr, $data_4:expr $(,)?) => {
        ($node).create_service_typed_4::<$service_ty, _, _, _, _, _, _>(
            $service_name,
            $qos_profile,
            $callback,
            $data_1,
            $data_2,
            $data_3,
            $data_4,
        )
    };
    ($node:expr, $service_ty:ty, $service_name:expr, $qos_profile:expr, $callback:expr, $data_1:expr, $data_2:expr, $data_3:expr, $data_4:expr, $data_5:expr $(,)?) => {
        ($node).create_service_typed_5::<$service_ty, _, _, _, _, _, _, _>(
            $service_name,
            $qos_profile,
            $callback,
            $data_1,
            $data_2,
            $data_3,
            $data_4,
            $data_5,
        )
    };
    ($($tt:tt)*) => {
        compile_error!(
            "Invalid `create_service!` invocation. Expected one of:\n\
             - create_service!(node, ServiceType, service_name, qos_profile, callback)\n\
             - create_service!(node, ServiceType, service_name, qos_profile, callback, data)\n\
             - create_service!(node, ServiceType, service_name, qos_profile, callback, data1, data2)\n\
             - create_service!(node, ServiceType, service_name, qos_profile, callback, data1, data2, data3)\n\
             - create_service!(node, ServiceType, service_name, qos_profile, callback, data1, data2, data3, data4)\n\
             - create_service!(node, ServiceType, service_name, qos_profile, callback, data1, data2, data3, data4, data5)"
        )
    };
}
