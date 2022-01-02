# Benchmark Report

> Benchmarked on [GitHub Action: Ubuntu 20.04, CPU 2 Core, RAM 7GB](https://docs.github.com/en/actions/using-github-hosted-runners/about-github-hosted-runners#supported-runners-and-hardware-resources)

## Index

- pool size = 4
- pool size = 8
- pool size = 16

## Result

> PostgreSQL Total Query Time Benchmark

### pool size = 4

| workers = 4 | workers = 16 | workers = 64 |
| ----------- | ------------ | ------------ |
| ![p04_w04]  | ![p04_w16]   | ![p04_w64]   |

### pool size = 8

| workers = 4 | workers = 16 | workers = 64 |
| ----------- | ------------ | ------------ |
| ![p08_w04]  | ![p08_w16]   | ![p08_w64]   |

### pool size = 16

| workers = 4 | workers = 16 | workers = 64 |
| ----------- | ------------ | ------------ |
| ![p16_w04]  | ![p16_w16]   | ![p16_w64]   |

[p04_w04]: /postgres/results/benchmark(p04_w04).svg
[p04_w16]: /postgres/results/benchmark(p04_w16).svg
[p04_w64]: /postgres/results/benchmark(p04_w64).svg
[p08_w04]: /postgres/results/benchmark(p08_w04).svg
[p08_w16]: /postgres/results/benchmark(p08_w16).svg
[p08_w64]: /postgres/results/benchmark(p08_w64).svg
[p16_w04]: /postgres/results/benchmark(p16_w04).svg
[p16_w16]: /postgres/results/benchmark(p16_w16).svg
[p16_w64]: /postgres/results/benchmark(p16_w64).svg
