use tokio::runtime::Runtime;
use flume::{Sender, Receiver};
use std::time::Duration;
use tokio::signal;

async fn send_values(tx: Sender<i32>, n: i32) {
    for i in 1..=n {
        tx.send(i).unwrap();
        let _ = tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Sent: {}", i);
    }
}

async fn receive_values(rx: Receiver<i32>) {
    for value in rx {
        println!("Received: {}", value);
        let _ = tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

fn main() {
    let rt = Runtime::new().unwrap();

    let (tx, rx) = flume::bounded(10); // Creates channel with buffer

    let n = 100;

    rt.spawn(async move {
        send_values(tx.clone(), n).await;
    });

    rt.spawn(async move {
        receive_values(rx).await;
    });

    rt.block_on(async {
        match signal::ctrl_c().await {
            Ok(_) => {
                println!("Ctrl+C received, shutting down...");
            }
            Err(err) => {
                eprintln!("Error listening for Ctrl+C: {}", err);
            }
        }
    });

    println!("Program finished");
}

// Запуск через cargo run в связи с появлением библиотек