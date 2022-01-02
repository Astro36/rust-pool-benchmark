use r2d2::Pool;
use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Instant;

fn main() {
    let mut file = File::create("r2d2-postgres-v0.18-result.txt").unwrap();
    let iters = 10;
    let worker_iters = 10;
    for pool_size in [4, 8, 16] {
        for workers in [4, 16, 64] {
            let config = "postgresql://postgres:postgres@localhost".parse().unwrap();
            let manager = PostgresConnectionManager::new(config, NoTls);
            let pool = Pool::builder().max_size(pool_size).build(manager).unwrap();

            // reserve resources.
            let mut v = Vec::with_capacity(pool_size as usize);
            for _ in 0..pool_size {
                v.push(pool.get().unwrap());
            }
            drop(v);

            let mut elapsed = Vec::with_capacity(iters);
            for _ in 0..iters {
                let handles = (0..workers)
                    .map(|_| {
                        let pool = pool.clone();
                        thread::spawn(move || {
                            thread::park();
                            for _ in 0..worker_iters {
                                let mut client = pool.get().unwrap();
                                let row = client.query_one("SELECT 1", &[]).unwrap();
                                let _int: i32 = row.get(0);
                            }
                        })
                    })
                    .collect::<Vec<_>>();
                let start = Instant::now();
                for handle in handles {
                    handle.thread().unpark();
                    handle.join().unwrap();
                }
                elapsed.push(start.elapsed());
            }
            elapsed.sort();
            let median = elapsed[iters / 2];
            let q1 = elapsed[iters / 4];
            let q3 = elapsed[iters * 3 / 4];
            println!(
                "r2d2-postgres-v0.18 (pool={}, worker={}): {:?} (Q1={:?}, Q3={:?})",
                pool_size, workers, median, q1, q3,
            );
            writeln!(
                file,
                "r2d2-postgres-v0.18,{},{},{},{},{}",
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
