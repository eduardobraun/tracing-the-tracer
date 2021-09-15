use tracing::{error, info};
use tracing_subscriber;

#[tracing::instrument]
fn fn_a () {
    info!("hello from fn_a");
    for i in (0..3).rev() {
        let res = sub_one(i, "some secret");
        info!(num=i, res, "{}-1={}", i, res);
    }
}

#[tracing::instrument(
    skip(secret),
)]
fn sub_one (value: u8, secret: &str) -> u8 {
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
