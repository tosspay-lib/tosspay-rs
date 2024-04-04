use std::{sync::mpsc, thread};

#[derive(Clone)]
pub struct TossPay {
    toss_id: String,
}
impl TossPay {
    pub fn new(toss_id: String) -> Self {
        TossPay { toss_id }
    }
    pub async fn trace_all(&self) -> mpsc::Receiver<TossPayData> {
        let fake_self = self.clone();
        let (sender, receiver) = mpsc::channel();
        let mut old_datas = Vec::new();
        tokio::spawn(async move {
            loop {
                let json = fake_self.request().await;
                let datas = json.success.data;
                if old_datas.clone() != datas.clone() {
                    let send_data = datas.clone();
                    send_data
                        .iter()
                        .filter(|e| old_datas.iter().find(|x| x == e).is_none())
                        .for_each(|x| {
                            sender.send(x.to_owned()).unwrap();
                        });
                    // let send_data = datas.clone().iter().filter(|e| old_datas.clone().contains(e));
                }
                old_datas = datas.clone();
                thread::sleep(std::time::Duration::from_secs(3));
            }
        });
        receiver
    }

    async fn request(&self) -> TossPayJson {
        let toss_response = reqwest::Client::new()
            .get(format!("https://api-public.toss.im/api-public/v3/cashtag/transfer-feed/received/list?inputWord={}", self.toss_id));
        let json = serde_json::from_str::<TossPayJson>(
            &toss_response
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .replace("null", "\"\""),
        )
        .unwrap();
        json
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct TossPayJson {
    pub resultType: String,
    pub success: TossPaySuccess,
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct TossPaySuccess {
    pub nextCursor: String,
    pub data: Vec<TossPayData>,
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct TossPayData {
    senderDisplayName: String,
    amount: usize,
    msg: String,
}
