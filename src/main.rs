use tosspay::TossPay;
#[tokio::main]
async fn main() {
    let toss = TossPay::new("wntjd0612".to_string());
    let trace = toss.trace_all().await;
    loop {
        let recv = trace.recv();
        if recv.is_ok() {
            let data = recv.unwrap();
            println!("{:?}", data);
        }
    }
}
