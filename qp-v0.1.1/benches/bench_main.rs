use qp::async_trait;
use qp::pool::Pool;
use qp::resource::Factory;
use std::convert::Infallible;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

pub struct IntFactory;

#[async_trait]
impl Factory for IntFactory {
    type Output = i32;
    type Error = Infallible;

    async fn try_create(&self) -> Result<Self::Output, Self::Error> {
        Ok(0)
    }

    async fn validate(&self, resource: &Self::Output) -> bool {
        resource >= &0
    }
}

#[tokio::main]
async fn main() {
    let mut file = File::create("qp-v0.1.1-result.txt").unwrap();
    let iters = 1_000;
    let worker_iters = 10;
    for pool_size in [4, 8, 16] {
        for workers in [4, 16, 64, 256] {
            let pool = Pool::new(IntFactory, pool_size);

            // reserve resources.
            let mut v = Vec::with_capacity(pool_size);
            for _ in 0..pool_size {
                v.push(pool.acquire().await.unwrap());
            }
            drop(v);

            let mut elapsed = Vec::with_capacity(iters);
            for _ in 0..iters {
                let handles = (0..workers)
                    .map(|_| {
                        let pool = pool.clone();
                        tokio::spawn(async move {
                            for _ in 0..worker_iters {
                                let mut int = pool.acquire().await.unwrap();
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
                "qp-v0.1.1 (pool={}, worker={}): {:?} (Q1={:?}, Q3={:?})",
                pool_size, workers, median, q1, q3,
            );
            writeln!(
                file,
                "qp-v0.1.1,{},{},{},{},{}",
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
