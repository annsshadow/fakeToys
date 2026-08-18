
## 濡備綍璁剧疆 Technisat/B2C2 Flexcop 璁惧


   鏈枃妗ｅ凡杩囨椂銆?
Author: Uwe Bugla <uwe.bugla@gmx.de> August 2009

### 纭浣犳嫢鏈夌殑璁惧


閲嶈鎻愮ず锛氳椹卞姩涓嶆敮鎸?Technisat USB 2 璁惧锛?
棣栧厛鐢ㄥ彂琛岀増鑷甫鐨勫唴鏍稿惎鍔ㄤ綘鐨?Linux 涓绘満锛?

	lspci -vvv 锛堝 PCI 璁惧锛夋垨 lsusb -vvv 锛堝 USB 璁惧锛変細鏄剧ず渚嬪锛?	02:0b.0 Network controller: Techsan Electronics Co Ltd B2C2 FlexCopII DVB chip /
	Technisat SkyStar2 DVB card (rev 02)

	dmesg | grep frontend 鍙兘鏄剧ず渚嬪锛?	DVB: registering frontend 0 (Conexant CX24123/CX24109)...

### 鍐呮牳缂栬瘧锛?

濡傛灉 Flexcop / Technisat 鏄綘涓绘満涓敮涓€鐨?DVB / TV / Radio 璁惧锛岃鍘绘帀涓嶅繀瑕佺殑妯″潡骞堕€変腑浠ヤ笅椤癸細

`Multimedia support` => `Customise analog and hybrid tuner modules to build`

鍦ㄦ鐩綍涓彇娑堝嬀閫夊叾涓墍鏈夊凡婵€娲荤殑椹卞姩锛堥櫎鐢ㄤ簬绗笁浠?ATSC 鐨?`Simple tuner support` 澶?鈥斺€?瑙佹儏褰?9锛夈€?
鐒跺悗璇锋縺娲伙細

- 涓绘ā鍧楅儴鍒嗭細

  `Multimedia support` => `DVB/ATSC adapters` => `Technisat/B2C2 FlexcopII(b) and FlexCopIII adapters`

  #) => `Technisat/B2C2 Air/Sky/Cable2PC PCI` 锛圥CI 鍗★級鎴?  #) => `Technisat/B2C2 Air/Sky/Cable2PC USB` 锛圲SB 1.1 閫傞厤鍣級
     浠ュ強鐢ㄤ簬鏁呴殰鎺掗櫎锛?  #) => `Enable debug for the B2C2 FlexCop drivers`

- 鍓嶇 / 璋冭皭鍣?/ 瑙ｈ皟鍣ㄦā鍧楅儴鍒嗭細

  `Multimedia support` => `DVB/ATSC adapters`
   => `Customise the frontend modules to build` `Customise DVB frontends` =>

  - SkyStar DVB-S 淇鐗?2.3锛?
    #) => `Zarlink VP310/MT312/ZL10313 based`
    #) => `Generic I2C PLL based tuners`

  - SkyStar DVB-S 淇鐗?2.6锛?
    #) => `ST STV0299 based`
    #) => `Generic I2C PLL based tuners`

  - SkyStar DVB-S 淇鐗?2.7锛?
    #) => `Samsung S5H1420 based`
    #) => `Integrant ITD1000 Zero IF tuner for DVB-S/DSS`
    #) => `ISL6421 SEC controller`

  - SkyStar DVB-S 淇鐗?2.8锛?
    #) => `Conexant CX24123 based`
    #) => `Conexant CX24113/CX24128 tuner for DVB-S/DSS`
    #) => `ISL6421 SEC controller`

  - AirStar DVB-T 鍗★細

    #) => `Zarlink MT352 based`
    #) => `Generic I2C PLL based tuners`

  - CableStar DVB-C 鍗★細

    #) => `ST STV0297 based`
    #) => `Generic I2C PLL based tuners`

  - AirStar ATSC 鍗＄涓€浠ｏ細

    #) => `Broadcom BCM3510`

  - AirStar ATSC 鍗＄浜屼唬锛?
    #) => `NxtWave Communications NXT2002/NXT2004 based`
    #) => `Generic I2C PLL based tuners`

  - AirStar ATSC 鍗＄涓変唬锛?
    #) => `LG Electronics LGDT3302/LGDT3303 based`
    #) `Multimedia support` => `Customise analog and hybrid tuner modules to build` => `Simple tuner support`
