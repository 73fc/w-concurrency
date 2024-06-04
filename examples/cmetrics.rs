use anyhow::{Ok, Result};
use rand::Rng;
use std::{thread, time::Duration};
use wconcurrency::CMetrics;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = CMetrics::new();

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_woker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
    #[allow(unreachable_code)]
    Ok(())
}

fn task_worker(idx: usize, metrics: CMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}

fn request_woker(metrics: CMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metrics.inc(format!("reg.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });

    Ok(())
}
