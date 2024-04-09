use std::{thread::sleep, time::Duration};
use tosspay::TossPay;
#[tokio::main]
async fn main() {
    let toss = TossPay::new("wntjd0612".to_string());
    toss.on_donate(|data| {
        if data.amount == 1 {
            println!("{:?}", data);
        } else {
            println!("nou");
        }
    });
    loop {
        sleep(Duration::new(1, 0));
    }
}
