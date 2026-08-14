
## HMAT - 异构内存属性表


异构内存属性表包含缓存等信息
内存邻近域的属性以及带宽和延迟详细信息。
出于本文档的目的，我们将仅讨论 SSLIB 条目。

## SLBI

系统位置延迟和带宽信息记录延迟和
邻近域的带宽信息。

Linux 使用该表来配置交错权重和内存层。

```

               Structure Type : 0001 [SLLBI]
                    Data Type : 00         <- Latency
 Target Proximity Domain List : 00000000
 Target Proximity Domain List : 00000001
                        Entry : 0080       <- DRAM LTC
                        Entry : 0100       <- CXL LTC

               Structure Type : 0001 [SLLBI]
                    Data Type : 03         <- Bandwidth
 Target Proximity Domain List : 00000000
 Target Proximity Domain List : 00000001
                        Entry : 1200       <- DRAM BW
                        Entry : 0200       <- CXL BW

```