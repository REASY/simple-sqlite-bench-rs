# Results

## Env

* OS: Ubuntu 25.04, 6.14.0-15-generic
* CPU: AMD Ryzen 9 9950X 16-Core Processor
* Disk: NVMe SSD, Samsung SSD 970 EVO Plus 2TB

## 10 Million rows, Synchronous flag is Normal, Journal mode is WAL

| Batch size | Unique tag values | Average throughput, rows/s | Total time, s | DB size, Mbytes |
|------------|-------------------|----------------------------|---------------|-----------------|
| 1000       | 20000             | 28987.41                   | 344.98        | 993             |
| 10000      | 20000             | 60299.17                   | 165.84        | 993             |
| 1000       | 10000             | 28788.16                   | 347.36        | 996             |
| 10000      | 10000             | 58226.09                   | 171.74        | 996             |

## 10 Million rows, Synchronous flag is Off, Journal mode is WAL

| Batch size | Unique tag values | Average throughput, rows/s | Total time, s | DB size, Mbytes |
|------------|-------------------|----------------------------|---------------|-----------------|
| 1000       | 20000             | 142733.26                  | 70.06         | 993             |
| 10000      | 20000             | 127382.47                  | 78.50         | 993             |
| 1000       | 10000             | 141192.05                  | 70.83         | 996             |
| 10000      | 10000             | 125123.74                  | 79.92         | 996             |

## 10 Million rows, Synchronous flag is Off, Journal mode is Off

| Batch size | Unique tag values | Average throughput, rows/s | Total time, s | DB size, Mbytes |
|------------|-------------------|----------------------------|---------------|-----------------|
| 1000       | 20000             | 419535.44                  | 23.84         | 993             |
| 10000      | 20000             | 411174.74                  | 24.32         | 993             |
| 1000       | 10000             | 425278.60                  | 23.51         | 996             |
| 10000      | 10000             | 413135.34                  | 24.21         | 996             |