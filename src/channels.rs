use rand::Rng;
use tokio::time::{Duration, sleep};

pub async fn ch1_task() {
    loop {
        sleep(Duration::from_secs(300)).await; // Every 5 minutes
        let intrusions = rand::rng().random_range(0..10);
        let viruses = rand::rng().random_range(0..5);
        println!(
            "ch1 [定時] [防御報告] {}件の侵入を阻止。{}件の侵入したウイルスを破壊。損害見受けられず。",
            intrusions, viruses
        );
    }
}

pub async fn ch2_task() {
    loop {
        sleep(Duration::from_secs(60)).await; // Every minute
        let cpu = rand::rng().random::<f32>() * 100.0;
        let mem = rand::rng().random::<f32>() * 100.0;
        let gpu = rand::rng().random::<f32>() * 100.0;
        let npu = rand::rng().random::<f32>() * 100.0;
        let disk = rand::rng().random::<f32>() * 100.0;
        let net = rand::rng().random::<f32>() * 100.0;
        println!(
            "ch2 [定時] [負荷率] cpu使用率 {:.1}% 平均 {:.1}% メモリ {:.1}% GPU {:.1}% NPU {:.1}% ディスクIO {:.1}% ネットワーク {:.1}%",
            cpu, cpu, mem, gpu, npu, disk, net
        );
    }
}

pub async fn ch3_task() {
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

pub async fn ch4_task() {
    loop {
        sleep(Duration::from_secs(600)).await; // Every 10 minutes
        println!("ch4 [ネットワーク管理] 端末の検証完了。正規端末を確認。");
    }
}

pub async fn ch5_task() {
    loop {
        sleep(Duration::from_secs(180)).await; // Every 3 minutes
        println!("ch5 [ファイルサーバー] ファイル圧縮処理完了。");
    }
}

pub async fn ch6_task() {
    loop {
        sleep(Duration::from_secs(240)).await; // Every 4 minutes
        println!("ch6 [バイナリリポジトリ] 新しいバイナリを追加。");
    }
}

pub async fn ch7_task() {
    loop {
        sleep(Duration::from_secs(7200)).await; // Every 2 hours
        println!("ch7 [未定義] 何もしない。");
    }
}
