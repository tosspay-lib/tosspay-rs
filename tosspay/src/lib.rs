#[macro_use]
extern crate lazy_static;

use std::{
    sync::{mpsc, Mutex},
    thread,
};

use rand::Rng;
lazy_static! {
    static ref IS_RUNNING: Mutex<bool> = Mutex::new(false);
    static ref RES: Mutex<TossPayJson> = Mutex::new(TossPayJson {
        resultType: String::new(),
        success: TossPaySuccess {
            nextCursor: String::new(),
            data: Vec::new(),
        },
    });
    static ref IDS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}
#[derive(Clone)]
pub struct TossPay {
    pub toss_id: String,
}
impl TossPay {
    pub fn new(toss_id: String) -> Self {
        if *IS_RUNNING.lock().unwrap() {
            panic!("Can't create multiple tosspay!");
        }

        *IS_RUNNING.lock().unwrap() = true;
        let toss_id_clone = toss_id.clone();

        tokio::spawn(async move {
            loop {
                let toss_response = reqwest::Client::new()
                    .get(format!("https://api-public.toss.im/api-public/v3/cashtag/transfer-feed/received/list?inputWord={}", toss_id_clone));
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
                *RES.lock().unwrap() = json;
                thread::sleep(std::time::Duration::from_secs(2));
            }
        });

        TossPay { toss_id }
    }

    pub fn on_donate(&self, f: fn(TossPayData)) {
        let mut old_datas = Vec::new();
        tokio::spawn(async move {
            loop {
                let json = RES.lock().unwrap().clone();
                let datas = json.success.data;
                if old_datas.clone() != datas.clone() {
                    let send_data = datas.clone();
                    send_data
                        .iter()
                        .filter(|e| old_datas.iter().find(|x| x == e).is_none())
                        .for_each(|x| f(x.to_owned()));
                    // let send_data = datas.clone().iter().filter(|e| old_datas.clone().contains(e));
                }
                old_datas = datas.clone();
                // let rng = rand
                thread::sleep(std::time::Duration::from_secs(2));
            }
        });
    }
    pub fn on_payment(&self, f: fn(TossPayData) -> Result<(), ()>) -> String {
        let code = gen_code();
        let mut fake_code = code.clone();
        IDS.lock().unwrap().push(code.clone());
        let mut old_datas = Vec::new();
        thread::spawn(move || 'scan_loop: loop {
            let json = RES.lock().unwrap().clone();
            let datas = json.success.data;
            if old_datas.clone() != datas.clone() {
                let send_data = datas.clone();
                send_data
                    .iter()
                    .filter(|e| old_datas.iter().find(|x| x == e).is_none())
                    .for_each(|x| {
                        if x.senderDisplayName == fake_code {
                            match f(x.to_owned()) {
                                Ok(_) => {
                                    fake_code = "".to_string();
                                }
                                Err(_) => {}
                            }
                        }
                    });
                if fake_code == "".to_string() {
                    break 'scan_loop;
                }
            }
            old_datas = datas.clone();
            thread::sleep(std::time::Duration::from_secs(2));
        });
        return code.clone();
    }

    pub async fn trace_all(&self) -> mpsc::Receiver<TossPayData> {
        let (sender, receiver) = mpsc::channel();
        let mut old_datas = Vec::new();
        tokio::spawn(async move {
            loop {
                let json = RES.lock().unwrap().clone();
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
                // let rng = rand
                thread::sleep(std::time::Duration::from_secs(2));
            }
        });
        receiver
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
    pub senderDisplayName: String,
    pub amount: usize,
    pub msg: String,
}

fn gen_code() -> String {
    let mut rng = rand::thread_rng();
    let code = format!(
        "{}{}{}{}{}",
        rng.gen_range(0..10),
        rng.gen_range(0..10),
        rng.gen_range(0..10),
        rng.gen_range(0..10),
        rng.gen_range(0..10)
    );
    if IDS.lock().unwrap().contains(&code) {
        return gen_code();
    }
    code
}
