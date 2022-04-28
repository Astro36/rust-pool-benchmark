use async_trait::async_trait;
use deadpool::managed::{Manager, Pool, RecycleResult};
use std::convert::Infallible;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

pub struct IntManager;

#[async_trait]
impl Manager<i32, Infallible> for IntManager {

    async fn create(&self) -> Result<i32, Infallible> {
        Ok(0)
    }

    async fn recycle(&self, _: &mut i32) -> RecycleResult<Infallible> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut file = File::create("deadpool-v0.7-result.txt").unwrap();
    let iters = 1_000;
    let worker_iters = 10;
    for pool_size in [4, 8, 16] {
        for workers in [4, 16, 64, 256] {
            let pool = Pool::new(IntManager, pool_size);

            // reserve resources.
            let mut v = Vec::with_capacity(pool_size);
            for _ in 0..pool_size {
                v.push(pool.get().await.unwrap());
            }
            drop(v);

            let mut elapsed = Vec::with_capacity(iters);
            for _ in 0..iters {
                let handles = (0..workers)
                    .map(|_| {
                        let pool = pool.clone();
                        tokio::spawn(async move {
                            for _ in 0..worker_iters {
                                let mut int = pool.get().await.unwrap();
                                *int += 1;
                            }
                        })
                    })
                    .collect::<Vec<_>>();
                let start = Instant::now();
                for handle in handles {
                    handle.await.unwrap();
                }
                elapsed.push(start.elapsed());
            }
            elapsed.sort();
            let median = elapsed[iters / 2];
            let q1 = elapsed[iters / 4];
            let q3 = elapsed[iters * 3 / 4];
            println!(
                "deadpool-v0.7 (pool={}, worker={}): {:?} (Q1={:?}, Q3={:?})",
                pool_size, workers, median, q1, q3,
            );
            writeln!(
                file,
                "deadpool-v0.7,{},{},{},{},{}",
                pool_size,
                workers,
                q1.as_nanos(),
                median.as_nanos(),
                q3.as_nanos()
            )
            .unwrap();
        }
    }
}
