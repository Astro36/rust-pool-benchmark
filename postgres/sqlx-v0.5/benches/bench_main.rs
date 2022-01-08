use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Row;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let mut file = File::create("sqlx-v0.5-result.txt").unwrap();
    let iters = 10;
    let worker_iters = 10;
    for pool_size in [4, 8, 16] {
        for workers in [4, 16, 64] {
            let pool = PgPoolOptions::new()
                .min_connections(pool_size)
                .max_connections(pool_size)
                .connect("postgresql://postgres:postgres@localhost")
                .await
                .unwrap();

            // reserve resources.
            let mut v = Vec::with_capacity(pool_size as usize);
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
                                let mut conn = pool.acquire().await.unwrap();
                                let _int = sqlx::query("SELECT 1")
                                    .try_map(|row: PgRow| row.try_get::<i32, _>(0))
                                    .fetch_one(&mut conn)
                                    .await
                                    .unwrap();
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
                "sqlx-v0.5 (pool={}, worker={}): {:?} (Q1={:?}, Q3={:?})",
                pool_size, workers, median, q1, q3,
            );
            writeln!(
                file,
                "sqlx-v0.5,{},{},{},{},{}",
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
