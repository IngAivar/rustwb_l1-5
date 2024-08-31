use flume::{Sender, Receiver};
use std::time::Duration;
use tokio::signal;

#[tokio::main]
async fn main() {
    // Создаем канал для передачи данных между задачами
    let (tx, rx) = flume::bounded(10); // Создаем ограниченный канал с буфером на 10 элементов

    // Запускаем задачу, которая будет отправлять значения в канал
    tokio::spawn(async move {
        send_values(tx.clone(), 100).await;
    });

    // Запускаем задачу, которая будет получать значения из канала
    tokio::spawn(async move {
        receive_values(rx).await;
    });

    // Ожидаем сигнала Ctrl+C для завершения программы
    signal::ctrl_c().await.unwrap();
    println!("Ctrl+C received, shutting down...");
    println!("Program finished");
}

async fn send_values(tx: Sender<i32>, n: i32) {
    // Отправляем значения в канал с задержкой
    for i in 1..=n {
        tx.send(i).unwrap();
        println!("Sent: {}", i);
        let _ = tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn receive_values(rx: Receiver<i32>) {
    // Получаем значения из канала и выводим их
    for value in rx {
        println!("Received: {}", value);
        let _ = tokio::time::sleep(Duration::from_secs(1)).await;
    }
}