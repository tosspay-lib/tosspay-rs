# on_donate

```rs
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
    loop {} // 코드가 계속 반복되고있지 않은 코드라면 이걸 넣어줘야해요
}
```