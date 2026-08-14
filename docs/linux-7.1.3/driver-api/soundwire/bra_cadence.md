### Cadence IP BRA 支持


#### 格式要求


Cadence IP 依赖 PDI0 用于 TX，PDI1 用于 RX。数据需要按以下约定格式化：

  (1) 所有数据存储在 32 位 PDI FIFO 的位 15..0 中。

  (2) 包的开始是 BIT(31)。

  (3) 包的结束是 BIT(30)。

  (4) 包 ID 存储在位 19..16 中。该包 ID 由软件确定，通常是一个滚动计数器。

  (5) 应根据需要插入填充，使 Header CRC、Header response、Footer CRC、Footer response
      始终位于 Byte0。写操作时由软件插入填充，读操作时软件应丢弃硬件添加的填充。

#### 示例格式


下表表示提供给 PDI0 的一个写命令后跟一个读命令的序列。
```

	+---+---+--------+---------------+---------------+
	+ 1 | 0 | ID = 0 |  WR HDR[1]    |  WR HDR[0]    |
	+   |   |        |  WR HDR[3]    |  WR HDR[2]    |
	+   |   |        |  WR HDR[5]    |  WR HDR[4]    |
	+   |   |        |  pad          |  WR HDR CRC   |
	+   |   |        |  WR Data[1]   |  WR Data[0]   |
	+   |   |        |  WR Data[3]   |  WR Data[2]   |
	+   |   |        |  WR Data[n-2] |  WR Data[n-3] |
	+   |   |        |  pad          |  WR Data[n-1] |
	+ 0 | 1 |        |  pad          |  WR Data CRC  |
	+---+---+--------+---------------+---------------+
	+ 1 | 0 | ID = 1 |  RD HDR[1]    |  RD HDR[0]    |
	+   |   |        |  RD HDR[3]    |  RD HDR[2]    |
	+   |   |        |  RD HDR[5]    |  RD HDR[4]    |
	+ 0 | 1 |        |  pad          |  RD HDR CRC   |
	+---+---+--------+---------------+---------------+


```
下表表示在同一写命令后跟读命令时，在 PDI1 上接收到的数据。
```

	+---+---+--------+---------------+---------------+
	+ 1 | 0 | ID = 0 |  pad          |  WR Hdr Rsp   |
	+ 0 | 1 |        |  pad          |  WR Ftr Rsp   |
	+---+---+--------+---------------+---------------+
	+ 1 | 0 | ID = 0 |  pad          |  Rd Hdr Rsp   |
	+   |   |        |  RD Data[1]   |  RD Data[0]   |
	+   |   |        |  RD Data[3]   |  RD Data[2]   |
	+   |   |        |  RD HDR[n-2]  |  RD Data[n-3] |
	+   |   |        |  pad          |  RD Data[n-1] |
	+   |   |        |  pad          |  RD Data CRC  |
	+ 0 | 1 |        |  pad          |  RD Ftr Rsp   |
	+---+---+--------+---------------+---------------+


```
