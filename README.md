# Results

## Env

* OS: Ubuntu 25.04, 6.14.0-15-generic
* CPU: AMD Ryzen 9 9950X 16-Core Processor
* Disk: NVMe SSD, Samsung SSD 970 EVO Plus 2TB
* rustc: 1.87.0 (17067e9ac 2025-05-09)

## Result

| Batch size | Unique tag values | Synchronous flag | Journal mode | Schema type          | Average throughput (records/s) | Total time (s) | DB size (MiB) | User time (s) | System time (s) |
|------------|-------------------|------------------|--------------|----------------------|--------------------------------|----------------|---------------|---------------|-----------------|
| 1000       | 10000             | Normal           | wal          | SingleTable          | 28 861.65                      | 346.48         | 995.32        | 23.38         | 85.21           |
| 1000       | 10000             | Normal           | off          | SingleTable          | 75 362.21                      | 132.69         | 995.64        | 11.88         | 31.58           |
| 1000       | 10000             | Off              | wal          | SingleTable          | 140 144.09                     | 71.36          | 995.35        | 18.16         | 53.04           |
| 1000       | 10000             | Off              | off          | SingleTable          | 420 036.20                     | 23.81          | 995.36        | 9.70          | 14.13           |
| 1000       | 10000             | Normal           | wal          | MappingAndDataTables | 39 253.96                      | 254.75         | 292.54        | 23.37         | 51.96           |
| 1000       | 10000             | Normal           | off          | MappingAndDataTables | 84 796.80                      | 117.93         | 292.55        | 16.16         | 20.54           |
| 1000       | 10000             | Off              | wal          | MappingAndDataTables | 218 519.27                     | 45.76          | 292.54        | 16.70         | 29.04           |
| 1000       | 10000             | Off              | off          | MappingAndDataTables | 458 750.11                     | 21.80          | 292.54        | 12.78         | 9.05            |
| 1000       | 10000             | Normal           | wal          | Fts5Table            | 171 046.89                     | 58.46          | 1 076.31      | 37.48         | 3.54            |
| 1000       | 10000             | Normal           | off          | Fts5Table            | 90 917.51                      | 109.99         | 1 077.95      | 38.97         | 2.17            |
| 1000       | 10000             | Off              | wal          | Fts5Table            | 252 739.21                     | 39.57          | 1 076.05      | 37.05         | 2.61            |
| 1000       | 10000             | Off              | off          | Fts5Table            | 266 694.04                     | 37.50          | 1 077.70      | 36.31         | 1.28            |
| 1000       | 20000             | Normal           | wal          | SingleTable          | 28 795.79                      | 347.27         | 992.44        | 23.79         | 86.64           |
| 1000       | 20000             | Normal           | off          | SingleTable          | 75 895.19                      | 131.76         | 992.37        | 11.50         | 32.57           |
| 1000       | 20000             | Off              | wal          | SingleTable          | 143 098.47                     | 69.88          | 992.34        | 17.80         | 51.95           |
| 1000       | 20000             | Off              | off          | SingleTable          | 428 771.83                     | 23.32          | 992.27        | 9.35          | 14.03           |
| 1000       | 20000             | Normal           | wal          | MappingAndDataTables | 53 747.39                      | 186.06         | 293.11        | 20.23         | 37.71           |
| 1000       | 20000             | Normal           | off          | MappingAndDataTables | 90 203.50                      | 110.86         | 293.10        | 16.94         | 16.28           |
| 1000       | 20000             | Off              | wal          | MappingAndDataTables | 260 612.07                     | 38.37          | 293.10        | 15.89         | 22.36           |
| 1000       | 20000             | Off              | off          | MappingAndDataTables | 499 308.00                     | 20.03          | 293.10        | 12.82         | 7.30            |
| 1000       | 20000             | Normal           | wal          | Fts5Table            | 163 331.19                     | 61.23          | 1 106.43      | 38.73         | 3.73            |
| 1000       | 20000             | Normal           | off          | Fts5Table            | 89 852.41                      | 111.29         | 1 107.38      | 39.99         | 2.25            |
| 1000       | 20000             | Off              | wal          | Fts5Table            | 245 159.83                     | 40.79          | 1 107.27      | 38.09         | 2.80            |
| 1000       | 20000             | Off              | off          | Fts5Table            | 255 466.26                     | 39.14          | 1 105.91      | 37.74         | 1.38            |
| 10000      | 10000             | Normal           | wal          | SingleTable          | 58 417.15                      | 171.18         | 995.43        | 19.05         | 77.54           |
| 10000      | 10000             | Normal           | off          | SingleTable          | 146 807.83                     | 68.12          | 995.40        | 10.53         | 24.77           |
| 10000      | 10000             | Off              | wal          | SingleTable          | 126 210.24                     | 79.23          | 995.02        | 17.77         | 61.48           |
| 10000      | 10000             | Off              | off          | SingleTable          | 412 007.82                     | 24.27          | 995.39        | 10.29         | 13.92           |
| 10000      | 10000             | Normal           | wal          | MappingAndDataTables | 81 632.54                      | 122.50         | 292.55        | 18.20         | 48.51           |
| 10000      | 10000             | Normal           | off          | MappingAndDataTables | 180 956.20                     | 55.26          | 292.55        | 13.33         | 17.44           |
| 10000      | 10000             | Off              | wal          | MappingAndDataTables | 184 818.32                     | 54.11          | 292.53        | 17.45         | 36.63           |
| 10000      | 10000             | Off              | off          | MappingAndDataTables | 445 454.66                     | 22.45          | 292.54        | 12.98         | 9.49            |
| 10000      | 10000             | Normal           | wal          | Fts5Table            | 205 811.06                     | 48.59          | 1 077.44      | 35.77         | 3.76            |
| 10000      | 10000             | Normal           | off          | Fts5Table            | 220 950.56                     | 45.26          | 1 078.05      | 35.11         | 1.55            |
| 10000      | 10000             | Off              | wal          | Fts5Table            | 258 948.23                     | 38.62          | 1 078.02      | 35.42         | 3.19            |
| 10000      | 10000             | Off              | off          | Fts5Table            | 277 570.96                     | 36.03          | 1 077.51      | 34.82         | 1.24            |
| 10000      | 20000             | Normal           | wal          | SingleTable          | 59 768.13                      | 167.31         | 992.29        | 18.24         | 76.05           |
| 10000      | 20000             | Normal           | off          | SingleTable          | 149 614.30                     | 66.84          | 992.36        | 10.52         | 25.16           |
| 10000      | 20000             | Off              | wal          | SingleTable          | 128 281.28                     | 77.95          | 992.44        | 18.02         | 59.86           |
| 10000      | 20000             | Off              | off          | SingleTable          | 414 021.99                     | 24.15          | 992.06        | 10.25         | 13.81           |
| 10000      | 20000             | Normal           | wal          | MappingAndDataTables | 101 356.31                     | 98.66          | 293.11        | 16.92         | 37.32           |
| 10000      | 20000             | Normal           | off          | MappingAndDataTables | 213 010.67                     | 46.95          | 293.11        | 12.96         | 14.10           |
| 10000      | 20000             | Off              | wal          | MappingAndDataTables | 225 387.20                     | 44.37          | 293.11        | 16.11         | 28.19           |
| 10000      | 20000             | Off              | off          | MappingAndDataTables | 484 606.05                     | 20.64          | 293.10        | 12.82         | 7.83            |
| 10000      | 20000             | Normal           | wal          | Fts5Table            | 196 461.05                     | 50.90          | 1 107.07      | 36.89         | 3.97            |
| 10000      | 20000             | Normal           | off          | Fts5Table            | 215 136.79                     | 46.48          | 1 107.25      | 36.06         | 1.64            |
| 10000      | 20000             | Off              | wal          | Fts5Table            | 251 142.90                     | 39.82          | 1 108.68      | 36.49         | 3.36            |
| 10000      | 20000             | Off              | off          | Fts5Table            | 266 837.96                     | 37.48          | 1 107.73      | 36.25         | 1.30            |
