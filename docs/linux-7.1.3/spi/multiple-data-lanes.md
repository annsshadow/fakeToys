## 具有多数据通道SPI 设备


一些专用的 SPI 控制器与外设支持多条数据通道，允许并行地一次性读取多个字这与八线 SPI 不同，后者是同时传输单个字的多个位
例如，支持并行闪存的控制器具备此特性，一些同步采ADC 也是如此，其中每通道都有自己的数据通道
### 描述接线


devicetree 中的 `spi-tx-bus-width` `spi-rx-bus-width` 属性用于描述控制器
之间连接了多少条数据通道，以及每条通道有多宽。数组中的项数表示通道数量每项的取值表示该通道的宽度（以位为单位）
例如，一个具有两4 位通道的双同步采样 ADC 可能如下所
```
    +--------------+    +----------+
    | SPI          |    | AD4630   |
    | Controller   |    | ADC      |
    |              |    |          |
    |          CS0 |--->| CS       |
    |          SCK |--->| SCK      |
    |          SDO |--->| SDI      |
    |              |    |          |
    |        SDIA0 |<---| SDOA0    |
    |        SDIA1 |<---| SDOA1    |
    |        SDIA2 |<---| SDOA2    |
    |        SDIA3 |<---| SDOA3    |
    |              |    |          |
    |        SDIB0 |<---| SDOB0    |
    |        SDIB1 |<---| SDOB1    |
    |        SDIB2 |<---| SDOB2    |
    |        SDIB3 |<---| SDOB3    |
    |              |    |          |
    +--------------+    +----------+

```
```

    spi {
        compatible = "my,spi-controller";

        ...

        adc@0 {
            compatible = "adi,ad4630";
            reg = <0>;
            ...
            spi-rx-bus-width = <4>, <4>; /* 2 lanes of 4 bits each */
            ...
        };
    };

```
在大多数情况下，通道会以对称方式接线（A A，B B，等等）。若非如此，需要额外的 `spi-rx-lane-map` `spi-tx-lane-map` 属性，以提供控制器通道物理通道导线之间的映射
下面是一个多通道 SPI 控制器将每条通道分别接到

```
    +--------------+    +----------+
    | SPI          |    | Thing 1  |
    | Controller   |    |          |
    |              |    |          |
    |          CS0 |--->| CS       |
    |         SDO0 |--->| SDI      |
    |         SDI0 |<---| SDO      |
    |        SCLK0 |--->| SCLK     |
    |              |    |          |
    |              |    +----------+
    |              |
    |              |    +----------+
    |              |    | Thing 2  |
    |              |    |          |
    |          CS1 |--->| CS       |
    |         SDO1 |--->| SDI      |
    |         SDI1 |<---| SDO      |
    |        SCLK1 |--->| SCLK     |
    |              |    |          |
    +--------------+    +----------+

```
```

    spi {
        compatible = "my,spi-controller";

        ...

        thing1@0 {
            compatible = "my,thing1";
            reg = <0>;
            ...
        };

        thing2@1 {
            compatible = "my,thing2";
            reg = <1>;
            ...
            spi-tx-lane-map = <1>; /* lane 0 is not used, lane 1 is used for tx wire */
            spi-rx-lane-map = <1>; /* lane 0 is not used, lane 1 is used for rx wire */
            ...
        };
    };


```
`spi-rx-bus-width` `spi-tx-bus-width` 的默认值为 `<1>`，因此即使使用了
`spi-rx-lane-map` `spi-tx-lane-map`，这些属性仍可省略
### 在外设驱动中的使

这类 SPI 控制器通常不支持任意使用多条通道，而是以几种预定义模式之一运行外设驱动应设`struct spi_transfer.multi_lane_mode <spi_transfer>` 字段以指示其在给定传输中想要使用的模式
该字段的可能取值具有以下语义：

- `SPI_MULTI_BUS_MODE_SINGLE`: 仅使用第一条通道，忽略其他通道。这意味着  运行方式与常SPI 外设相同。这是默认模式，因此无需显式设置
```

        tx_buf[0] = 0x88;

        struct spi_transfer xfer = {
            .tx_buf = tx_buf,
            .len = 1,
        };

        spi_sync_transfer(spi, &xfer, 1);

    Assuming the controller is sending the MSB first, the sequence of bits
    sent over the tx wire would be (right-most bit is sent first)::

        controller    > data bits >     peripheral
        ----------   ----------------   ----------
            SDO 0    0-0-0-1-0-0-0-1    SDI 0

```
- `SPI_MULTI_BUS_MODE_MIRROR`: 同时在所有通道上发送单个数据字。这仅对写入
  有意义，对读取无意义
```

        tx_buf[0] = 0x88;

        struct spi_transfer xfer = {
            .tx_buf = tx_buf,
            .len = 1,
            .multi_lane_mode = SPI_MULTI_BUS_MODE_MIRROR,
        };

        spi_sync_transfer(spi, &xfer, 1);

    The data is mirrored on each tx wire::

        controller    > data bits >     peripheral
        ----------   ----------------   ----------
            SDO 0    0-0-0-1-0-0-0-1    SDI 0
            SDO 1    0-0-0-1-0-0-0-1    SDI 1

```
- `SPI_MULTI_BUS_MODE_STRIPE`: 同时在每条通道上发送或接收两个不同的数据字  每条通道一个。这意味着缓冲区需要按容纳所有通道的数据来定大小。数据在缓冲  中交错排列，第一个字对应通道 0，第二个对应通道 1，依此类推。使用最后一  通道后，缓冲区中的下一个字再次对应通道 0。因此，缓冲区大小必须是通道数的
  整数倍。该模式既可用于读取也可用于写入
```

        struct spi_transfer xfer = {
            .rx_buf = rx_buf,
            .len = 2,
            .multi_lane_mode = SPI_MULTI_BUS_MODE_STRIPE,
        };

        spi_sync_transfer(spi, &xfer, 1);

    Each rx wire has a different data word sent simultaneously::

        controller    < data bits <     peripheral
        ----------   ----------------   ----------
            SDI 0    0-0-0-1-0-0-0-1    SDO 0
            SDI 1    1-0-0-0-1-0-0-0    SDO 1

    After the transfer, ``rx_buf[0] == 0x11`` (word from SDO 0) and
    ``rx_buf[1] == 0x88`` (word from SDO 1).


```
### SPI 控制器驱动支

为支持多条数据通道，SPI 控制器驱动需要将
`struct spi_controller.num_data_lanes <spi_controller>` 设置为大1 的值
然后，处SPI 传输的驱动部分需要检`struct spi_transfer.multi_lane_mode <spi_transfer>` 字段，并为每种支持的
模式实现相应行为，对不支持的模式返回错误
其余部分SPI 核心代码处理