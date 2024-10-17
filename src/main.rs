use reqwest::Client;
use tokio::time::{sleep, Duration};
const NACOS_ADDR: &str = "127.0.0.1";
const NACOS_PORT: &str = "8848";

const SERVER_NAME: &str = "nacos-test-service";

const SERVER_ADDR: &str = "127.0.0.1";
const SERVER_PORT: i32 = 5800;

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
            "http://".to_owned()
                + NACOS_ADDR
                + ":"
                + NACOS_PORT
                + "/nacos/v1/ns/instance?serviceName="
                + SERVER_NAME
                + "&ip="
                + SERVER_ADDR
                + "&port="
                + &SERVER_PORT.to_string(),
        )
        .send()
        .await;
}

async fn service_beat() {
    loop {
        let client = Client::new();
        let _ = client
            .put(
                "http://".to_owned()
                    + NACOS_ADDR
                    + ":"
                    + NACOS_PORT
                    + "/nacos/v1/ns/instance/beat?serviceName="
                    + SERVER_NAME
                    + "&ip="
                    + SERVER_ADDR
                    + "&port="
                    + &SERVER_PORT.to_string(),
            )
            .send()
            .await;
        sleep(Duration::from_secs(5)).await;
    }
}
