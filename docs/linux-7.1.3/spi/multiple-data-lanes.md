## 鍏锋湁澶氭暟鎹€氶亾鐨?SPI 璁惧


涓€浜涗笓鐢ㄧ殑 SPI 鎺у埗鍣ㄤ笌澶栬鏀寔澶氭潯鏁版嵁閫氶亾锛屽厑璁稿苟琛屽湴涓€娆℃€ц鍙栧涓瓧銆?杩欎笌鍙?鍥?鍏嚎 SPI 涓嶅悓锛屽悗鑰呮槸鍚屾椂浼犺緭鍗曚釜瀛楃殑澶氫釜浣嶃€?
渚嬪锛屾敮鎸佸苟琛岄棯瀛樼殑鎺у埗鍣ㄥ叿澶囨鐗规€э紝涓€浜涘悓姝ラ噰鏍?ADC 涔熸槸濡傛锛屽叾涓瘡涓?閫氶亾閮芥湁鑷繁鐨勬暟鎹€氶亾銆?
### 鎻忚堪鎺ョ嚎


devicetree 涓殑 `spi-tx-bus-width` 涓?`spi-rx-bus-width` 灞炴€х敤浜庢弿杩版帶鍒跺櫒
涔嬮棿杩炴帴浜嗗灏戞潯鏁版嵁閫氶亾锛屼互鍙婃瘡鏉￠€氶亾鏈夊瀹姐€傛暟缁勪腑鐨勯」鏁拌〃绀洪€氶亾鏁伴噺锛?姣忛」鐨勫彇鍊艰〃绀鸿閫氶亾鐨勫搴︼紙浠ヤ綅涓哄崟浣嶏級銆?
渚嬪锛屼竴涓叿鏈変袱鏉?4 浣嶉€氶亾鐨勫弻鍚屾閲囨牱 ADC 鍙兘濡備笅鎵€绀?
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
鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝閫氶亾浼氫互瀵圭О鏂瑰紡鎺ョ嚎锛圓 鎺?A锛孊 鎺?B锛岀瓑绛夛級銆傝嫢闈炲姝わ紝鍒?闇€瑕侀澶栫殑 `spi-rx-lane-map` 涓?`spi-tx-lane-map` 灞炴€э紝浠ユ彁渚涙帶鍒跺櫒閫氶亾涓?鐗╃悊閫氶亾瀵肩嚎涔嬮棿鐨勬槧灏勩€?
涓嬮潰鏄竴涓閫氶亾 SPI 鎺у埗鍣ㄥ皢姣忔潯閫氶亾鍒嗗埆鎺ュ埌

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
`spi-rx-bus-width` 涓?`spi-tx-bus-width` 鐨勯粯璁ゅ€间负 `<1>`锛屽洜姝ゅ嵆浣夸娇鐢ㄤ簡
`spi-rx-lane-map` 涓?`spi-tx-lane-map`锛岃繖浜涘睘鎬т粛鍙渷鐣ャ€?
### 鍦ㄥ璁鹃┍鍔ㄤ腑鐨勪娇鐢?

杩欑被 SPI 鎺у埗鍣ㄩ€氬父涓嶆敮鎸佷换鎰忎娇鐢ㄥ鏉￠€氶亾锛岃€屾槸浠ュ嚑绉嶉瀹氫箟妯″紡涔嬩竴杩愯銆?澶栬椹卞姩搴旇缃?`struct spi_transfer.multi_lane_mode <spi_transfer>` 瀛楁锛?浠ユ寚绀哄叾鍦ㄧ粰瀹氫紶杈撲腑鎯宠浣跨敤鐨勬ā寮忋€?
璇ュ瓧娈电殑鍙兘鍙栧€煎叿鏈変互涓嬭涔夛細

- `SPI_MULTI_BUS_MODE_SINGLE`: 浠呬娇鐢ㄧ涓€鏉￠€氶亾锛屽拷鐣ュ叾浠栭€氶亾銆傝繖鎰忓懗鐫€鍏?  杩愯鏂瑰紡涓庡父瑙?SPI 澶栬鐩稿悓銆傝繖鏄粯璁ゆā寮忥紝鍥犳鏃犻渶鏄惧紡璁剧疆銆?
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
- `SPI_MULTI_BUS_MODE_MIRROR`: 鍚屾椂鍦ㄦ墍鏈夐€氶亾涓婂彂閫佸崟涓暟鎹瓧銆傝繖浠呭鍐欏叆
  鏈夋剰涔夛紝瀵硅鍙栨棤鎰忎箟銆?
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
- `SPI_MULTI_BUS_MODE_STRIPE`: 鍚屾椂鍦ㄦ瘡鏉￠€氶亾涓婂彂閫佹垨鎺ユ敹涓や釜涓嶅悓鐨勬暟鎹瓧锛?  姣忔潯閫氶亾涓€涓€傝繖鎰忓懗鐫€缂撳啿鍖洪渶瑕佹寜瀹圭撼鎵€鏈夐€氶亾鐨勬暟鎹潵瀹氬ぇ灏忋€傛暟鎹湪缂撳啿鍖?  涓氦閿欐帓鍒楋紝绗竴涓瓧瀵瑰簲閫氶亾 0锛岀浜屼釜瀵瑰簲閫氶亾 1锛屼緷姝ょ被鎺ㄣ€備娇鐢ㄦ渶鍚庝竴鏉?  閫氶亾鍚庯紝缂撳啿鍖轰腑鐨勪笅涓€涓瓧鍐嶆瀵瑰簲閫氶亾 0銆傚洜姝わ紝缂撳啿鍖哄ぇ灏忓繀椤绘槸閫氶亾鏁扮殑
  鏁存暟鍊嶃€傝妯″紡鏃㈠彲鐢ㄤ簬璇诲彇涔熷彲鐢ㄤ簬鍐欏叆銆?
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
### SPI 鎺у埗鍣ㄩ┍鍔ㄦ敮鎸?

涓烘敮鎸佸鏉℃暟鎹€氶亾锛孲PI 鎺у埗鍣ㄩ┍鍔ㄩ渶瑕佸皢
`struct spi_controller.num_data_lanes <spi_controller>` 璁剧疆涓哄ぇ浜?1 鐨勫€笺€?
鐒跺悗锛屽鐞?SPI 浼犺緭鐨勯┍鍔ㄩ儴鍒嗛渶瑕佹鏌?`struct spi_transfer.multi_lane_mode <spi_transfer>` 瀛楁锛屽苟涓烘瘡绉嶆敮鎸佺殑
妯″紡瀹炵幇鐩稿簲琛屼负锛屽涓嶆敮鎸佺殑妯″紡杩斿洖閿欒銆?
鍏朵綑閮ㄥ垎鐢?SPI 鏍稿績浠ｇ爜澶勭悊銆?