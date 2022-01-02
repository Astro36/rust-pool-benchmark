use qp_postgres::tokio_postgres::NoTls;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let mut file = File::create("qp-postgres-v0.1-result.txt").unwrap();
    let iters = 10;
    let worker_iters = 10;
    for pool_size in [4, 8, 16] {
        for workers in [4, 16, 64] {
            let config = "postgresql://postgres:postgres@localhost".parse().unwrap();
            let pool = qp_postgres::connect(config, NoTls, pool_size);

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
                                let client = pool.acquire().await.unwrap();
                                let row = client.query_one("SELECT 1", &[]).await.unwrap();
                                let _int: i32 = row.get(0);
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
                "qp-postgres-v0.1 (pool={}, worker={}): {:?} (Q1={:?}, Q3={:?})",
                pool_size, workers, median, q1, q3,
            );
            writeln!(
                file,
                "qp-postgres-v0.1,{},{},{},{},{}",
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
