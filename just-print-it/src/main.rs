use tracing::{error, info};
use tracing_subscriber;

fn fn_a () {
    info!("hello from fn_a");
    for i in (0..3).rev() {
        let res = sub_one(i);
        info!(num=i, res, "{}-1={}", i, res);
    }
}

fn sub_one (value: u8) -> u8 {
    info!("hello from sub_one");
    match value.checked_sub(1) {
        Some(i) => i,
        None => {
            error!("subtraction failed!");
            0
        },
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    info!("hello from main");
    fn_a();
}
