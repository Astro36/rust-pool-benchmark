# Benchmark Report

> Benchmarked on [GitHub Action: Ubuntu 20.04, CPU 2 Core, RAM 7GB](https://docs.github.com/en/actions/using-github-hosted-runners/about-github-hosted-runners#supported-runners-and-hardware-resources)

## Index

- pool size = 4
- pool size = 8
- pool size = 16

## Result

> Resource Acquisition Time Benchmark

### pool size = 4

| workers = 4 | workers = 16 |
| ----------- | ----------- |
| ![p04_w004] | ![p04_w016] |

| workers = 64 | workers = 256 |
| ----------- | ----------- |
| ![p04_w064] | ![p04_w256] |

### pool size = 8

| workers = 4 | workers = 16 |
| ----------- | ------------ |
| ![p08_w004] | ![p08_w016]  |

| workers = 64 | workers = 256 |
| ------------ | ------------- |
| ![p08_w064]  | ![p08_w256]   |

### pool size = 16

| workers = 4 | workers = 16 |
| ----------- | ------------ |
| ![p16_w004] | ![p16_w016]  |

| workers = 64 | workers = 256 |
| ------------ | ------------- |
| ![p16_w064]  | ![p16_w256]   |

[p04_w004]: /results/benchmark(p04_w004).svg
[p04_w016]: /results/benchmark(p04_w016).svg
[p04_w064]: /results/benchmark(p04_w064).svg
[p04_w256]: /results/benchmark(p04_w256).svg
[p08_w004]: /results/benchmark(p08_w004).svg
[p08_w016]: /results/benchmark(p08_w016).svg
[p08_w064]: /results/benchmark(p08_w064).svg
[p08_w256]: /results/benchmark(p08_w256).svg
[p16_w004]: /results/benchmark(p16_w004).svg
[p16_w016]: /results/benchmark(p16_w016).svg
[p16_w064]: /results/benchmark(p16_w064).svg
[p16_w256]: /results/benchmark(p16_w256).svg
