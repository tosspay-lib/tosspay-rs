# on_donate

```rs
use tosspay::TossPay;
#[tokio::main]
async fn main() {
    let toss = TossPay::new("objective".to_string() /* 여기 자신의 toss-id 써주세여*/);
    let code = toss.on_payment(|data| {
        if data.amount == 1 {
            println!("{:?}", data);
            Ok(()) // 성공했다면 Ok(())를 반환해주세요
        } else {
            println!("nou");
            Err(())  // 실패했다면 Ok(())를 반환해주세요
        }
    });
    println!("{}", code);
    loop {} // 코드가 계속 반복되고있지 않은 코드라면 이걸 넣어줘야해요
}
```