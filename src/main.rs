use rand::Rng;
use serde::Deserialize;
use std::fs;
use tokio::time::{Duration, sleep};

#[derive(Debug, Deserialize)]
struct Profile {
    cpu_cores: u32,
    cpu_breakdown: Option<String>,
    memory_gb: u32,
    npu_count: Option<u32>,
    npu_breakdown: Option<String>,
    npu_tops: Option<u32>,
    gpu_count: Option<u32>,
    gpu_breakdown: Option<String>,
    gpu_tops: Option<u32>,
    network_mbps: u32,
    wireless: bool,
}

#[tokio::main]
async fn main() {
    // Select profile, default to "default"
    let profile_name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "default".to_string());
    let profile_path = format!("settings/{}.toml", profile_name);
    let profile_content = fs::read_to_string(&profile_path)
        .unwrap_or_else(|_| panic!("Failed to read profile: {}", profile_path));
    let profile: Profile = toml::from_str(&profile_content)
        .unwrap_or_else(|_| panic!("Failed to parse profile: {}", profile_path));

    println!("Starting NanAI Joke Machine with profile: {}", profile_name);
    // Always enable start messages
    println!("ch0 [システム起動] NanAI Joke Machine 起動完了");
    println!(
        "プロファイル: {} - CPU: {} cores, Memory: {} GB, Network: {} Mbps",
        profile_name, profile.cpu_cores, profile.memory_gb, profile.network_mbps
    );

    let mut handles = vec![];

    // ch0 is disabled by default
    // if config.channels.ch0_enabled {
    //     let handle = tokio::spawn(async move {
    //         ch0_task().await;
    //     });
    //     handles.push(handle);
    // }

    // Always enable ch1 to ch7
    let handle = tokio::spawn(async move {
        ch1_task().await;
    });
    handles.push(handle);

    let handle = tokio::spawn(async move {
        ch2_task().await;
    });
    handles.push(handle);

    let handle = tokio::spawn(async move {
        ch3_task().await;
    });
    handles.push(handle);

    let handle = tokio::spawn(async move {
        ch4_task().await;
    });
    handles.push(handle);

    let handle = tokio::spawn(async move {
        ch5_task().await;
    });
    handles.push(handle);

    let handle = tokio::spawn(async move {
        ch6_task().await;
    });
    handles.push(handle);

    let handle = tokio::spawn(async move {
        ch7_task().await;
    });
    handles.push(handle);

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn ch1_task() {
    loop {
        sleep(Duration::from_secs(300)).await; // Every 5 minutes
        let intrusions = rand::random::<u32>() % 10;
        let viruses = rand::random::<u32>() % 5;
        println!(
            "ch1 [定時] [防御報告] {}件の侵入を阻止。{}件の侵入したウイルスを破壊。損害見受けられず。",
            intrusions, viruses
        );
    }
}

async fn ch2_task() {
    loop {
        sleep(Duration::from_secs(60)).await; // Every minute
        let cpu = rand::random::<f32>() * 100.0;
        let mem = rand::random::<f32>() * 100.0;
        let gpu = rand::random::<f32>() * 100.0;
        let npu = rand::random::<f32>() * 100.0;
        let disk = rand::random::<f32>() * 100.0;
        let net = rand::random::<f32>() * 100.0;
        println!(
            "ch2 [定時] [負荷率] cpu使用率 {:.1}% 平均 {:.1}% メモリ {:.1}% GPU {:.1}% NPU {:.1}% ディスクIO {:.1}% ネットワーク {:.1}%",
            cpu, cpu, mem, gpu, npu, disk, net
        );
    }
}

async fn ch3_task() {
    loop {
        sleep(Duration::from_secs(120)).await; // Every 2 minutes
        let actions = vec![
            "ch3 [注意] [走査開始] 敵システムを捜索中",
            "ch3 [注意] [走査中] 敵システムに脆弱性を発見",
            "ch3 [注意] [侵入開始] 敵システムに侵入開始",
            "ch3 [注意] [侵入中] 敵システムにPKV3ウイルスを注入",
            "ch3 [注意] [侵入中] 敵システムにPKV3ウイルスで有効打",
            "ch3 [注意] [侵入終了] 敵システムに侵入完了",
            "ch3 [注意] [破壊] 敵システムを打撃。ログから本体と断定。",
        ];
        let msg = actions[rand::rng().random_range(0..actions.len())];
        println!("{}", msg);
    }
}

async fn ch4_task() {
    loop {
        sleep(Duration::from_secs(600)).await; // Every 10 minutes
        println!("ch4 [ネットワーク管理] 端末の検証完了。正規端末を確認。");
    }
}

async fn ch5_task() {
    loop {
        sleep(Duration::from_secs(180)).await; // Every 3 minutes
        println!("ch5 [ファイルサーバー] ファイル圧縮処理完了。");
    }
}

async fn ch6_task() {
    loop {
        sleep(Duration::from_secs(240)).await; // Every 4 minutes
        println!("ch6 [バイナリリポジトリ] 新しいバイナリを追加。");
    }
}

async fn ch7_task() {
    loop {
        sleep(Duration::from_secs(7200)).await; // Every 2 hours
        println!("ch7 [未定義] 何もしない。");
    }
}
