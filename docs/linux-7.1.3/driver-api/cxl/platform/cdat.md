## 一致性设备属性表（CDAT）


CDAT 提供诸如 CXL 加速器、交换机或端点等设备的功能与性能属性。其表格式类似于
ACPI 表。CDAT 数据可由 BIOS 在启动时解析，也可在运行时枚举（例如设备热插拔之后）。

术语：
DPA — 设备物理地址（Device Physical Address），由 CXL 设备用来表示该设备所支持的地址。

DSMADHandle — 与由 DSMAS 表定义的 DPA 范围相关联的设备唯一句柄。


## 设备作用域内存亲和性结构（DSMAS）


DSMAS 包含诸如 DSMADHandle、DPA 基地址（DPA Base）和 DPA 长度（DPA Length）等信息。

该表由 Linux 与设备作用域延迟与带宽信息结构（DSLBIS）结合使用，以确定 CXL 设备
自身的性能属性。

```

 Structure Type : 00 [DSMAS]
       Reserved : 00
         Length : 0018              <- 24d, size of structure
    DSMADHandle : 01
          Flags : 00
       Reserved : 0000
       DPA Base : 0000000040000000  <- 1GiB base
     DPA Length : 0000000080000000  <- 2GiB size


```
## 设备作用域延迟与带宽信息结构（DSLBIS）


该表由 Linux 与 DSMAS 结合使用，以确定 CXL 设备的性能属性。DSLBIS 包含基于
DSMADHandle 匹配的延迟与带宽信息。

```

   Structure Type : 01 [DSLBIS]
         Reserved : 00
           Length : 18                     <- 24d, size of structure
           Handle : 0001                   <- DSMAS handle
            Flags : 00                     <- Matches flag field for HMAT SLLBIS
        Data Type : 00                     <- Latency
 Entry Basee Unit : 0000000000001000       <- Entry Base Unit field in HMAT SSLBIS
            Entry : 010000000000           <- First byte used here, CXL LTC
         Reserved : 0000

   Structure Type : 01 [DSLBIS]
         Reserved : 00
           Length : 18                     <- 24d, size of structure
           Handle : 0001                   <- DSMAS handle
            Flags : 00                     <- Matches flag field for HMAT SLLBIS
        Data Type : 03                     <- Bandwidth
 Entry Basee Unit : 0000000000001000       <- Entry Base Unit field in HMAT SSLBIS
            Entry : 020000000000           <- First byte used here, CXL BW
         Reserved : 0000


```
## 交换机作用域延迟与带宽信息结构（SSLBIS）


SSLBIS 包含有关交换机延迟和带宽的信息。

该表由 Linux 用于计算从设备到根端口的 CXL 路径的性能坐标，其中交换机是路径的一部分。

```

  Structure Type : 05 [SSLBIS]
        Reserved : 00
          Length : 20                           <- 32d, length of record, including SSLB entries
       Data Type : 00                           <- Latency
        Reserved : 000000
 Entry Base Unit : 00000000000000001000         <- Matches Entry Base Unit in HMAT SSLBIS

                                                <- SSLB Entry 0
       Port X ID : 0100                         <- First port, 0100h represents an upstream port
       Port Y ID : 0000                         <- Second port, downstream port 0
         Latency : 0100                         <- Port latency
        Reserved : 0000
                                                <- SSLB Entry 1
       Port X ID : 0100
       Port Y ID : 0001
         Latency : 0100
        Reserved : 0000


  Structure Type : 05 [SSLBIS]
        Reserved : 00
          Length : 18                           <- 24d, length of record, including SSLB entry
       Data Type : 03                           <- Bandwidth
        Reserved : 000000
 Entry Base Unit : 00000000000000001000         <- Matches Entry Base Unit in HMAT SSLBIS

                                                <- SSLB Entry 0
       Port X ID : 0100                         <- First port, 0100h represents an upstream port
       Port Y ID : FFFF                         <- Second port, FFFFh indicates any port
       Bandwidth : 1200                         <- Port bandwidth
        Reserved : 0000

```
CXL 驱动结合使用 CDAT、HMAT、SRAT 以及其他数据，为 CXL 设备生成“整条路径性能”数据。
