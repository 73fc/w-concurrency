use anyhow::Result;
use rand::Rng;
use std::{thread, time::Duration};
use wconcurrency::Metrics;

const N: usize = 2;
const M: usize = 4;
fn main() -> Result<()> {
    let metrics = Metrics::new();

    println!("{:?}", metrics.snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_woker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{:?}", metrics);
    }
    #[allow(unused)]
    Ok(())
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.woker.{}", idx))?;
        }
        #[allow(unused)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_woker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..1000)));
            let page = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unused)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
