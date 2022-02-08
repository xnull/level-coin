
use std::thread;
use std::thread::ThreadId;
use std::time::Duration;
use tokio::sync::watch;
use tokio::sync::mpsc;
use tokio::sync::watch::Receiver;
use tokio::task::JoinHandle;

use async_std::{
    prelude::*, // 1
    task, // 2
    net::{TcpListener, ToSocketAddrs}, // 3
};

#[derive(Debug, Copy, Clone)]
enum Govno {
    Paxnet {
        thread_id: ThreadId,
        msg: i32,
    }
}

#[tokio::main]
async fn main() {
    let thread_id: ThreadId = thread::current().id();
    println!("main thread: {:?}", thread_id);
    //mpsc().await


}

async fn mpsc() {
    let (mut tx, mut rx) = mpsc::channel(32);
    let mut curr_tx = tx.clone();

    let mut tasks = vec![];
    for i in 0..100 {
        let task = tokio::spawn(async move {
            let value = Govno::Paxnet {
                thread_id: thread::current().id(),
                msg: i,
            };
            //thread::sleep(Duration::from_secs(1));
            curr_tx.send(value).await;
        });

        tasks.push(task);

        curr_tx = tx.clone();
    }

    let manager = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            match message {
                Govno::Paxnet { thread_id, msg } => {
                    println!("GOT = thread: {:?}, msg: {}, curr thread: {:?}", thread_id, msg, thread::current().id());
                }
            }
        }
    });

    for async_task in tasks {
        async_task.await.unwrap();
    }

    manager.await.unwrap();
}


async fn yay() {
    println!("main thread: {:?}", thread::current().id());

    let task = tokio::spawn(async move {
        let thread_id: ThreadId = thread::current().id();
        println!("async task: {:?}", thread_id)
    });

    task.await.unwrap()
}