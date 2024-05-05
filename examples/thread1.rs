use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    id: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit!");
        42
    });

    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join err: {:?}", e))?;

    println!("secret is {}", secret);
    Ok(())
}

fn producer(id: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(id, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 10 == 0 {
            break;
        }
    }
    println!("producer{}, is drop", id);
    Ok(())
}

impl Msg {
    fn new(id: usize, value: usize) -> Self {
        Self { id, value }
    }
}
