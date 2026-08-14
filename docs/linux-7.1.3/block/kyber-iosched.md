## Kyber I/O 调度器可调参数


Kyber 调度器仅有的两个可调参数是读与同步写的目标延迟。Kyber 会节流请求以满足这些目标延迟。

### read_lat_nsec


读的目标延迟（以纳秒为单位）。

### write_lat_nsec


同步写的目标延迟（以纳秒为单位）。
