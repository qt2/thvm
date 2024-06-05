# thvm

Experimental register-based VM written in Rust.

Currently very limited instructions are available, which are prepared for benchmarks.

## Benchmark

| Runtime            | Time (ns/iter) |
| ------------------ | -------------: |
| native             |        3.01976 |
| vm                 |        7.53506 |
| vm (stack-based)   |       28.19452 |
| python (reference) |       44.97422 |