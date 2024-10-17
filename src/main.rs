use reqwest::Client;
use tokio::time::{sleep, Duration};

const SERVER_NAME: &str = "users-service";
const USERNAME: &str = "admin";
const PASSWORD: &str = "admin";

const SERVER_ADDR: &str = "127.0.0.1";
const IP_ADDR: &str = "127.0.0.1";
const PORT: i32 = 5800;

#[tokio::main]
async fn main() {
    // 使用 tokio::spawn 并发执行两个异步任务
    let register_handle = tokio::spawn(async {
        service_register().await;
    });

    let beat_handle = tokio::spawn(async {
        service_beat().await;
    });

    // 等待两个任务完成
    let _ = tokio::try_join!(register_handle, beat_handle);
}

async fn service_register() {
    let client = reqwest::Client::new();
    let _ = client
        .post(
            "http://127.0.0.1:8848/nacos/v1/ns/instance?serviceName=".to_owned()
                + SERVER_NAME
                + "&ip="
                + IP_ADDR
                + "&port="
                + &PORT.to_string(),
        )
        .send()
        .await;
}

async fn service_beat() {
    loop {
        let client = Client::new();
        let _ = client.put("http://127.0.0.1:8848/nacos/v1/ns/instance/beat?serviceName=users-service&ip=127.0.0.1&port=8091").send().await;
        sleep(Duration::from_secs(5)).await;
    }
}
