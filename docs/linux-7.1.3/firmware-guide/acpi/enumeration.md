
## 鍩轰簬 ACPI 鐨勮澶囨灇涓?


ACPI 5 寮曞叆浜嗕竴缁勬柊璧勬簮锛圲artTSerialBus銆両2cSerialBus銆丼piSerialBus銆丟pioIo 涓?GpioInt锛夛紝鍙敤浜庢灇涓句覆琛屾€荤嚎鎺у埗鍣ㄨ儗鍚庣殑浠庤澶囥€?

姝ゅ锛屾垜浠紑濮嬬湅鍒伴泦鎴愬湪 SoC/鑺墖缁勪腑鐨勫璁句粎鍑虹幇鍦?ACPI 鍛藉悕绌洪棿涓€傝繖浜涢€氬父鏄€氳繃鍐呭瓨鏄犲皠瀵勫瓨鍣ㄨ闂殑璁惧銆?

涓轰簡鏀寔杩欎竴鐐瑰苟灏藉彲鑳藉鐢ㄧ幇鏈夐┍鍔紝鎴戜滑鍐冲畾閲囧彇浠ヤ笅鍋氭硶锛?

  - 娌℃湁鎬荤嚎杩炴帴鍣ㄨ祫婧愮殑璁惧琛ㄧず涓?platform 璁惧銆?

  - 浣嶄簬鐪熷疄鎬荤嚎涔嬪悗銆佷笖瀛樺湪杩炴帴鍣ㄨ祫婧愮殑璁惧琛ㄧず涓?struct spi_device 鎴?struct i2c_client銆傛敞鎰忥紝鏍囧噯 UART 骞朵笉鏄€荤嚎锛屽洜姝や笉瀛樺湪 struct uart_device锛屼笉杩囧叾涓竴浜涘彲浠ョ敱 struct serdev_device 琛ㄧず銆?

鐢变簬 ACPI 涓?Device Tree 閮借〃绀轰竴妫佃澶囷紙鍙婂叾璧勬簮锛夋爲锛屾湰瀹炵幇灏藉彲鑳介伒寰?Device Tree 鐨勬柟寮忋€侫CPI 瀹炵幇鏋氫妇鎬荤嚎锛坧latform銆丼PI銆両2C锛屼互鍙婃煇浜涙儏鍐典笅鐨?UART锛夎儗鍚庣殑璁惧锛屽垱寤虹墿鐞嗚澶囷紝骞跺皢瀹冧滑缁戝畾鍒板叾鍦?ACPI 鍛藉悕绌洪棿涓殑 ACPI handle銆?

杩欐剰鍛崇潃褰?ACPI_HANDLE(dev) 杩斿洖闈?NULL 鏃讹紝璇ヨ澶囨槸浠?ACPI 鍛藉悕绌洪棿鏋氫妇鑰屾潵鐨勩€傛 handle 鍙敤浜庢彁鍙栧叾浠栬澶囩壒瀹氱殑閰嶇疆銆備笅闈㈡湁涓€涓ず渚嬨€?

## 骞冲彴鎬荤嚎鏀寔


鐢变簬鎴戜滑浣跨敤 platform 璁惧鏉ヨ〃绀烘湭杩炴帴鍒颁换浣曠墿鐞嗘€荤嚎鐨勮澶囷紝鎴戜滑鍙渶涓鸿璁惧瀹炵幇涓€涓?platform 椹卞姩骞舵坊鍔犲彈鏀寔鐨?ACPI ID銆傚鏋滃湪鍏朵粬鏌愪釜闈?ACPI 骞冲彴涓婁娇鐢ㄤ簡鐩稿悓鐨?IP 妯″潡锛岃椹卞姩涔熻鍙互寮€绠卞嵆鐢紝鎴栧彧闇€灏戦噺淇敼銆?

涓虹幇鏈夐┍鍔ㄦ坊鍔?ACPI 鏀寔搴斿綋鐩稿綋

```
	static const struct acpi_device_id mydrv_acpi_match[] = {
		/* ACPI IDs here */
		{ }
	};
	MODULE_DEVICE_TABLE(acpi, mydrv_acpi_match);

	static struct platform_driver my_driver = {
		...
		.driver = {
			.acpi_match_table = mydrv_acpi_match,
		},
	};
```

濡傛灉椹卞姩闇€瑕佹墽琛屾洿澶嶆潅鐨勫垵濮嬪寲锛堜緥濡傝幏鍙栧苟閰嶇疆 GPIO锛夛紝瀹冨彲浠ヨ幏鍙栧叾 ACPI handle 骞朵粠姝?ACPI 琛ㄤ腑鎻愬彇璇ヤ俊鎭€?

## ACPI 璁惧瀵硅薄


涓€鑸潵璇达紝鍦ㄤ娇鐢?ACPI 浣滀负骞冲彴鍥轰欢涓?OS 涔嬮棿鎺ュ彛鐨勭郴缁熶腑鏈変袱绫昏澶囷細涓€绫绘槸鏃犻渶骞冲彴鍥轰欢鍗忓姪銆侀€氳繃涓哄叾鎵€鍦ㄧ壒瀹氭€荤嚎瀹氫箟鐨勫崗璁紙渚嬪 PCI 涓殑閰嶇疆绌洪棿锛夊嵆鍙鍘熺敓鍙戠幇骞舵灇涓剧殑璁惧锛涘彟涓€绫绘槸闇€瑕佺敱骞冲彴鍥轰欢鎻忚堪鎵嶈兘琚彂鐜扮殑璁惧銆備笉杩囷紝瀵逛簬骞冲彴鍥轰欢宸茬煡鐨勪换浣曡澶囷紝鏃犺瀹冨睘浜庡摢涓€绫伙紝鍦?ACPI 鍛藉悕绌洪棿涓兘鍙兘瀛樺湪涓€涓搴旂殑 ACPI 璁惧瀵硅薄锛屾鏃?Linux 鍐呮牳浼氬熀浜庡畠涓鸿璁惧鍒涘缓涓€涓?struct acpi_device 瀵硅薄銆?

閭ｄ簺 struct acpi_device 瀵硅薄浠庝笉鐢ㄤ簬涓哄師鐢熷彲鍙戠幇鐨勮澶囩粦瀹氶┍鍔紝鍥犱负瀹冧滑鐢卞叾浠栫被鍨嬬殑璁惧瀵硅薄锛堜緥濡?PCI 璁惧鐨?struct pci_dev锛夎〃绀猴紝骞剁敱璁惧椹卞姩缁戝畾锛堢浉搴旂殑 struct acpi_device 瀵硅薄鍒欑敤浣滃叧浜庤璁惧閰嶇疆鐨勯澶栦俊鎭潵婧愶級銆傛澶栵紝ACPI 璁惧鏋氫妇鏍稿績浠ｇ爜涓虹粷澶у鏁板€熷姪骞冲彴鍥轰欢鍙戠幇骞舵灇涓剧殑璁惧鍒涘缓 struct platform_device 瀵硅薄锛岃€岃繖浜?platform 璁惧瀵硅薄鍙互鐢?platform 椹卞姩缁戝畾锛屼笌鍘熺敓鍙灇涓捐澶囩殑鎯呭喌鐩存帴绫绘瘮銆傚洜姝わ紝灏嗛┍鍔ㄧ粦瀹氬埌 struct acpi_device 瀵硅薄鍦ㄩ€昏緫涓婃槸涓嶄竴鑷寸殑锛屽洜鑰岄€氬父鏄棤鏁堢殑锛屽寘鎷负鍊熷姪骞冲彴鍥轰欢鍙戠幇鐨勮澶囩紪鍐欑殑椹卞姩涔熸槸濡傛銆?

鍘嗗彶涓婏紝鏇句负涓€浜涘€熷姪骞冲彴鍥轰欢鏋氫妇鐨勮澶囧疄鐜拌繃鐩存帴缁戝畾鍒?struct acpi_device 瀵硅薄鐨?ACPI 椹卞姩锛屼絾涓嶅缓璁换浣曟柊椹卞姩杩欐牱鍋氥€傚涓婃墍杩帮紝杩欎簺璁惧鍘熷垯涓婇兘浼氬垱寤?platform 璁惧瀵硅薄锛堟澶勬棤鍏崇殑灏戞暟渚嬪闄ゅ锛夛紝鍥犳鍗充娇鐩稿簲鐨?ACPI 璁惧瀵硅薄鏄繖绉嶆儏鍐典笅鍞竴鐨勮澶囬厤缃俊鎭潵婧愶紝涔熷簲浣跨敤 platform 椹卞姩鏉ュ鐞嗗畠浠€?

瀵逛簬姣忎釜鎷ユ湁瀵瑰簲 struct acpi_device 瀵硅薄鐨勮澶囷紝鎸囧悜瀹冪殑鎸囬拡鐢?ACPI_COMPANION() 瀹忚繑鍥烇紝鍥犳鎬绘槸鍙互閫氳繃杩欑鏂瑰紡鑾峰彇鍒板瓨鍌ㄥ湪 ACPI 璁惧瀵硅薄涓殑璁惧閰嶇疆淇℃伅銆傜浉搴斿湴锛宻truct acpi_device 鍙涓哄唴鏍镐笌 ACPI 鍛藉悕绌洪棿涔嬮棿鎺ュ彛鐨勪竴閮ㄥ垎锛岃€屽叾浠栫被鍨嬬殑璁惧瀵硅薄锛堜緥濡?struct pci_dev 鎴?struct platform_device锛夊垯鐢ㄤ簬涓庣郴缁熷叾浣欓儴鍒嗕氦浜掋€?

## DMA 鏀寔


閫氳繃 ACPI 鏋氫妇鐨?DMA 鎺у埗鍣ㄥ簲鍦ㄧ郴缁熶腑娉ㄥ唽锛屼互鎻愪緵瀵瑰叾璧勬簮鐨勯€氱敤璁块棶銆備緥濡傦紝甯屾湜浠庡睘璁惧鑳介€氳繃閫氱敤 API 璋冪敤 dma_request_chan() 璁块棶鐨勯┍鍔紝蹇呴』鍦?probe 鍑芥暟鏈熬鍍忎笅闈㈣繖鏍锋敞鍐岃嚜宸憋細

```
	err = devm_acpi_dma_controller_register(dev, xlate_func, dw);
	/* Handle the error if it's not a case of !CONFIG_ACPI */
```

骞跺湪闇€瑕佹椂瀹炵幇鑷畾涔夌殑 xlate 鍑芥暟锛堥€氬父 acpi_dma_simple_xlate() 宸茶冻澶燂級锛岃鍑芥暟灏?struct acpi_dma_spec 鎻愪緵鐨?FixedDMA 璧勬簮杞崲涓虹浉搴旂殑 DMA 閫氶亾銆傜浉鍏充唬鐮佺墖鏂涓嬶細

```
	#ifdef CONFIG_ACPI
	struct filter_args {
		/* Provide necessary information for the filter_func */
		...
	};

	static bool filter_func(struct dma_chan *chan, void *param)
	{
		/* Choose the proper channel */
		...
	}

	static struct dma_chan *xlate_func(struct acpi_dma_spec *dma_spec,
			struct acpi_dma *adma)
	{
		dma_cap_mask_t cap;
		struct filter_args args;

		/* Prepare arguments for filter_func */
		...
		return dma_request_channel(cap, filter_func, &args);
	}
	#else
	static struct dma_chan *xlate_func(struct acpi_dma_spec *dma_spec,
			struct acpi_dma *adma)
	{
		return NULL;
	}
	#endif
```

dma_request_chan() 浼氫负姣忎釜宸叉敞鍐岀殑 DMA 鎺у埗鍣ㄨ皟鐢?xlate_func()銆傚湪 xlate 鍑芥暟涓紝蹇呴』鏍规嵁 struct acpi_dma_spec 涓殑淇℃伅浠ュ強 struct acpi_dma 鎻愪緵鐨勬帶鍒跺櫒灞炴€ф潵閫夋嫨鍚堥€傜殑閫氶亾銆?

瀹㈡埛绔繀椤讳娇鐢ㄥ搴斾簬鐗瑰畾 FixedDMA 璧勬簮鐨勫瓧绗︿覆鍙傛暟璋冪敤 dma_request_chan()銆傞粯璁ゆ儏鍐典笅 "tx" 琛ㄧず FixedDMA 璧勬簮鏁扮粍鐨勭涓€椤癸紝"rx" 琛ㄧず绗簩椤广€備笅琛ㄦ紨绀轰簡涓€涓?

```
	Device (I2C0)
	{
		...
		Method (_CRS, 0, NotSerialized)
		{
			Name (DBUF, ResourceTemplate ()
			{
				FixedDMA (0x0018, 0x0004, Width32bit, _Y48)
				FixedDMA (0x0019, 0x0005, Width32bit, )
			})
		...
		}
	}
```

鍥犳锛屽湪鏈緥涓姹傜嚎涓?0x0018 鐨?FixedDMA 鏄?"tx"锛屼笅涓€涓槸 "rx"銆?

鍦ㄥ仴澹殑瀹炵幇涓紝瀹㈡埛绔笉宸ч渶瑕佺洿鎺ヨ皟鐢?acpi_dma_request_slave_chan_by_index()锛屼粠鑰屾寜绱㈠紩閫夋嫨鐗瑰畾鐨?FixedDMA 璧勬簮銆?

## 鍛藉悕涓柇


閫氳繃 ACPI 鏋氫妇鐨勯┍鍔ㄥ彲浠ュ湪 ACPI 琛ㄤ腑涓轰腑鏂懡鍚嶏紝杩欎簺鍚嶇О鍙敤浜庡湪椹卞姩涓幏鍙?IRQ 鍙枫€備腑鏂悕绉板彲浠ュ湪 _DSD 涓互 'interrupt-names' 鍒楀嚭銆傝繖浜涘悕绉板簲鍒椾负涓€涓瓧绗︿覆鏁扮粍锛屽畠浠皢鏄犲皠鍒?ACPI 琛ㄤ腑涓庡叾绱㈠紩瀵瑰簲鐨?Interrupt() 璧勬簮銆?

```
    Device (DEV0) {
        ...
        Name (_CRS, ResourceTemplate() {
            ...
            Interrupt (ResourceConsumer, Level, ActiveHigh, Exclusive) {
                0x20,
                0x24
            }
        })

        Name (_DSD, Package () {
            ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
            Package () {
                Package () { "interrupt-names", Package () { "default", "alert" } },
            }
        ...
        })
    }
```

涓柇鍚嶇О 'default' 灏嗗搴?Interrupt() 璧勬簮涓殑 0x20锛?alert' 瀵瑰簲 0x24銆傛敞鎰忥紝浠呮槧灏?Interrupt() 璧勬簮锛岃€屼笉鏄犲皠 GpioInt() 鎴栫被浼艰祫婧愩€?

椹卞姩鍙互璋冪敤鍑芥暟 fwnode_irq_get_byname()锛屼互 fwnode 涓庝腑鏂悕绉颁綔涓哄弬鏁帮紝鏉ヨ幏鍙栫浉搴旂殑 IRQ 鍙枫€?

## SPI 涓茶鎬荤嚎鏀寔


浣嶄簬 SPI 鎬荤嚎涔嬪悗鐨勪粠璁惧闄勬湁 SpiSerialBus 璧勬簮銆係PI 鏍稿績浼氳嚜鍔ㄦ彁鍙栧畠锛屽苟涓斾竴鏃︽€荤嚎椹卞姩璋冪敤 spi_register_master()锛屼粠璁惧灏变細琚灇涓俱€?

```
	Device (EEP0)
	{
		Name (_ADR, 1)
		Name (_CID, Package () {
			"ATML0025",
			"AT25",
		})
		...
		Method (_CRS, 0, NotSerialized)
		{
			SPISerialBus(1, PolarityLow, FourWireMode, 8,
				ControllerInitiated, 1000000, ClockPolarityLow,
				ClockPhaseFirst, "\\_SB.PCI0.SPI1",)
		}
		...
```

SPI 璁惧椹卞姩鍙渶浠ョ被浼间簬 platform 璁惧椹卞姩鐨勬柟寮忔坊鍔?ACPI ID銆備笅闈㈡槸涓€涓垜浠坊鍔?ACPI 鏀寔鐨勭ず渚?

```
	static const struct acpi_device_id at25_acpi_match[] = {
		{ "AT25", 0 },
		{ }
	};
	MODULE_DEVICE_TABLE(acpi, at25_acpi_match);

	static struct spi_driver at25_driver = {
		.driver = {
			...
			.acpi_match_table = at25_acpi_match,
		},
	};
```

娉ㄦ剰锛岃椹卞姩瀹為檯涓婇渶瑕佹洿澶氫俊鎭紝渚嬪椤靛ぇ灏忕瓑

```
	Device (EEP0)
	{
		...
		Name (_DSD, Package ()
		{
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package ()
			{
				Package () { "size", 1024 },
				Package () { "pagesize", 32 },
				Package () { "address-width", 16 },
			}
		})
	}
```

鐒跺悗 at25 SPI 椹卞姩鍙互閫氳繃璋冪敤璁惧灞炴€ф帴鍙ｈ幏鍙栨閰嶇疆

```
	err = device_property_read_u32(dev, "size", &size);
	if (err)
		...error handling...

	err = device_property_read_u32(dev, "pagesize", &page_size);
	if (err)
		...error handling...

	err = device_property_read_u32(dev, "address-width", &addr_width);
	if (err)
		...error handling...
```

## I2C 涓茶鎬荤嚎鏀寔


浣嶄簬 I2C 鎬荤嚎鎺у埗鍣ㄤ箣鍚庣殑浠庤澶囧彧闇€鍍?platform 涓?SPI 椹卞姩閭ｆ牱娣诲姞 ACPI ID銆備竴鏃﹂€傞厤鍣ㄦ敞鍐岋紝I2C 鏍稿績浼氳嚜鍔ㄦ灇涓炬帶鍒跺櫒璁惧鑳屽悗鐨勪换浣曚粠璁惧銆?

涓嬮潰鏄皢 ACPI 鏀寔娣诲姞鍒扮幇鏈?mpu3050 鐨勭ず渚?

```
	static const struct acpi_device_id mpu3050_acpi_match[] = {
		{ "MPU3050", 0 },
		{ }
	};
	MODULE_DEVICE_TABLE(acpi, mpu3050_acpi_match);

	static struct i2c_driver mpu3050_i2c_driver = {
		.driver	= {
			.name	= "mpu3050",
			.pm	= &mpu3050_pm,
			.of_match_table = mpu3050_of_match,
			.acpi_match_table = mpu3050_acpi_match,
		},
		.probe		= mpu3050_probe,
		.remove		= mpu3050_remove,
		.id_table	= mpu3050_ids,
	};
	module_i2c_driver(mpu3050_i2c_driver);
```

## 瀵?PWM 璁惧鐨勫紩鐢?


鏈夋椂涓€涓澶囧彲浠ユ槸鏌愪釜 PWM 閫氶亾鐨勬秷璐硅€呫€傛樉鐒?OS 甯屾湜鐭ラ亾鏄摢涓€涓€備负浜嗘彁渚涜繖绉嶆槧灏勶紝鐗规畩灞炴€у凡琚?

```
    Device (DEV)
    {
        Name (_DSD, Package ()
        {
            ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
            Package () {
                Package () { "compatible", Package () { "pwm-leds" } },
                Package () { "label", "alarm-led" },
                Package () { "pwms",
                    Package () {
                        "\\_SB.PCI0.PWM",  // <PWM device reference>
                        0,                 // <PWM index>
                        600000000,         // <PWM period>
                        0,                 // <PWM flags>
                    }
                }
            }
        })
        ...
    }
```

鍦ㄤ笂杩扮ず渚嬩腑锛屽熀浜?PWM 鐨?LED 椹卞姩寮曠敤浜?\_SB.PCI0.PWM 璁惧鐨?PWM 閫氶亾 0锛屽垵濮嬪懆鏈熻缃负 600 ms锛堟敞鎰忚鍊间互绾崇缁欏嚭锛夈€?

## GPIO 鏀寔


ACPI 5 寮曞叆浜嗕袱涓柊璧勬簮鏉ユ弿杩?GPIO 杩炴帴锛欸pioIo 涓?GpioInt銆傝繖浜涜祫婧愬彲鐢ㄤ簬灏嗚澶囦娇鐢ㄧ殑 GPIO 缂栧彿浼犻€掔粰椹卞姩銆侫CPI 5.1 閫氳繃 _DSD锛圖evice Specific Data锛岃澶囩壒瀹氭暟鎹級瀵规杩涜浜嗘墿灞曪紝闄ゅ叾浠栧姛鑳藉锛岃繕浣垮緱鍙互涓?GPIO 鍛藉悕銆?

```
	Device (DEV)
	{
		Method (_CRS, 0, NotSerialized)
		{
			Name (SBUF, ResourceTemplate()
			{
				// Used to power on/off the device
				GpioIo (Exclusive, PullNone, 0, 0, IoRestrictionOutputOnly,
					"\\_SB.PCI0.GPI0", 0, ResourceConsumer) { 85 }

				// Interrupt for the device
				GpioInt (Edge, ActiveHigh, ExclusiveAndWake, PullNone, 0,
					 "\\_SB.PCI0.GPI0", 0, ResourceConsumer) { 88 }
			}

			Return (SBUF)
		}

		// ACPI 5.1 _DSD used for naming the GPIOs
		Name (_DSD, Package ()
		{
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package ()
			{
				Package () { "power-gpios", Package () { ^DEV, 0, 0, 0 } },
				Package () { "irq-gpios", Package () { ^DEV, 1, 0, 0 } },
			}
		})
		...
	}
```

杩欎簺 GPIO 缂栧彿鏄浉瀵逛簬鎺у埗鍣ㄧ殑锛岃矾寰?"\\_SB.PCI0.GPI0" 鎸囧畾浜嗘帶鍒跺櫒鎵€鍦ㄧ殑璺緞銆備负浜嗗湪 Linux 涓娇鐢ㄨ繖浜?GPIO锛屾垜浠渶瑕佸皢瀹冧滑杞崲涓虹浉搴旂殑 Linux GPIO 鎻忚堪绗︺€?

瀵规鏈変竴涓爣鍑嗙殑 GPIO API锛屽叾鏂囨。浣嶄簬 Documentation/admin-guide/gpio/銆?

鍦ㄤ笂杩扮ず渚嬩腑锛屾垜浠彲浠ラ€氳繃浠ヤ笅鏂瑰紡鑾峰彇鐩稿簲鐨勪袱涓?GPIO 鎻忚堪绗︼細

```
	#include <linux/gpio/consumer.h>
	...

	struct gpio_desc *irq_desc, *power_desc;

	irq_desc = gpiod_get(dev, "irq");
	if (IS_ERR(irq_desc))
		/* handle error */

	power_desc = gpiod_get(dev, "power");
	if (IS_ERR(power_desc))
		/* handle error */

	/* Now we can use the GPIO descriptors */
```

杩欎簺鍑芥暟杩樻湁 devm_* 鐗堟湰锛屼細鍦ㄨ澶囬噴鏀炬椂涓€骞堕噴鏀炬弿杩扮銆?

鏈夊叧涓?GPIO 鐩稿叧鐨?_DSD 缁戝畾锛岃瑙?Documentation/firmware-guide/acpi/gpio-properties.rst銆?

## RS-485 鏀寔


ACPI _DSD锛圖evice Specific Data锛夊彲鐢ㄤ簬鎻忚堪 UART 鐨?RS-485 鑳藉姏銆?

```
	Device (DEV)
	{
		...

		// ACPI 5.1 _DSD used for RS-485 capabilities
		Name (_DSD, Package ()
		{
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package ()
			{
				Package () {"rs485-rts-active-low", Zero},
				Package () {"rs485-rx-active-high", Zero},
				Package () {"rs485-rx-during-tx", Zero},
			}
		})
		...
```

## MFD 璁惧


MFD 璁惧灏嗗叾瀛愯澶囨敞鍐屼负 platform 璁惧銆傚浜庡瓙璁惧锛岄渶瑕佷竴涓?ACPI handle锛屼緵鍏跺紩鐢ㄤ笌鑷韩鐩稿叧鐨?ACPI 鍛藉悕绌洪棿閮ㄥ垎銆傚湪 Linux MFD 瀛愮郴缁熶腑鎴戜滑鎻愪緵涓ょ鏂瑰紡锛?

  - 瀛愯澶囧叡浜埗璁惧鐨?ACPI handle銆?
  - MFD cell 鍙互鎸囧畾璇ヨ澶囩殑 ACPI id銆?

瀵逛簬绗竴绉嶆儏鍐碉紝MFD 椹卞姩鏃犻渶鍋氫换浣曚簨銆傜敓鎴愮殑瀛?platform 璁惧鍏?ACPI_COMPANION() 灏嗚璁剧疆涓烘寚鍚戠埗璁惧銆?

濡傛灉 ACPI 鍛藉悕绌洪棿涓湁涓€涓垜浠彲浠ラ€氳繃 ACPI id 鎴?ACPI

```
	static struct mfd_cell_acpi_match my_subdevice_cell_acpi_match = {
		.pnpid = "XYZ0001",
		.adr = 0,
	};

	static struct mfd_cell my_subdevice_cell = {
		.name = "my_subdevice",
		/* set the resources relative to the parent */
		.acpi_match = &my_subdevice_cell_acpi_match,
	};
```

鐒跺悗锛孉CPI id "XYZ0001" 琚敤浜庣洿鎺ュ湪 MFD 璁惧涓嬫煡鎵句竴涓?ACPI 璁惧锛岃嫢鎵惧埌锛屽垯璇?ACPI companion 璁惧琚粦瀹氬埌鐢熸垚鐨勫瓙 platform 璁惧銆?

## Device Tree 鍛藉悕绌洪棿閾炬帴璁惧 ID


Device Tree 鍗忚浣跨敤鍩轰簬 "compatible" 灞炴€х殑璁惧鏍囪瘑锛岃灞炴€х殑鍊兼槸涓€涓瓧绗︿覆鎴栦竴缁勫瓧绗︿覆锛岃椹卞姩涓庨┍鍔ㄦ牳蹇冭瘑鍒负璁惧鏍囪瘑绗︺€傛墍鏈夎繖浜涘瓧绗︿覆鐨勯泦鍚堝彲琚涓轰竴涓澶囨爣璇嗗懡鍚嶇┖闂达紝绫讳技浜?ACPI/PNP 璁惧 ID 鍛藉悕绌洪棿銆傚洜姝わ紝鍘熷垯涓婁笉搴旀湁蹇呰涓哄湪 Device Tree锛圖T锛夊懡鍚嶇┖闂翠腑宸叉湁鏍囪瘑瀛楃涓茬殑璁惧鍒嗛厤涓€涓柊鐨勶紙涓斿彲璇存槸鍐椾綑鐨勶級ACPI/PNP 璁惧 ID锛屽挨鍏舵槸褰撹 ID 浠呯敤浜庤〃鏄庢煇涓粰瀹氳澶囦笌鍙︿竴涓澶囧吋瀹癸紙鍚庤€呭ぇ姒傚湪鍐呮牳涓凡鏈夊尮閰嶇殑椹卞姩锛夋椂銆?

鍦?ACPI 涓紝鍚嶄负 _CID锛圕ompatible ID锛屽吋瀹?ID锛夌殑璁惧鏍囪瘑瀵硅薄鐢ㄤ簬鍒楀嚭缁欏畾璁惧鎵€鍏煎璁惧鐨?ID锛屼絾杩欎簺 ID 蹇呴』灞炰簬 ACPI 瑙勮寖瑙勫畾鐨勬煇涓懡鍚嶇┖闂达紙璇﹁ ACPI 6.0 绗?6.1.2 鑺傦級锛岃€?DT 鍛藉悕绌洪棿骞堕潪鍏朵腑涔嬩竴銆傛澶栵紝瑙勮寖寮哄埗瑕佹眰鎵€鏈夎〃绀鸿澶囩殑 ACPI 瀵硅薄閮藉繀椤诲瓨鍦?_HID 鎴?_ADR 鏍囪瘑瀵硅薄锛圓CPI 6.0 绗?6.1 鑺傦級銆傚浜庝笉鍙灇涓剧殑鎬荤嚎绫诲瀷锛岃瀵硅薄蹇呴』鏄?_HID锛屼笖鍏跺€间篃蹇呴』鏄鑼冭瀹氱殑鏌愪釜鍛藉悕绌洪棿涓殑璁惧 ID銆?

鐗规畩鐨?DT 鍛藉悕绌洪棿閾炬帴璁惧 ID锛孭RP0001锛屾彁渚涗簡涓€绉嶅湪 ACPI 涓娇鐢ㄧ幇鏈?DT 鍏煎璁惧鏍囪瘑銆佸悓鏃跺張鑳芥弧瓒充笂杩版簮鑷?ACPI 瑙勮寖涔嬭姹傜殑鏂规硶銆傚叿浣撴潵璇达紝濡傛灉 _HID 杩斿洖 PRP0001锛孉CPI 瀛愮郴缁熷皢鍦ㄨ澶囧璞＄殑 _DSD 涓煡鎵?"compatible" 灞炴€э紝骞朵娇鐢ㄨ灞炴€х殑鍊兼寜鐓у師濮?DT 璁惧鏍囪瘑绠楁硶鏉ヨ瘑鍒浉搴旇澶囥€傚鏋?"compatible" 灞炴€т笉瀛樺湪鎴栧叾鍊兼棤鏁堬紝璇ヨ澶囧皢涓嶄細琚?ACPI 瀛愮郴缁熸灇涓俱€傚惁鍒欙紝瀹冨皢鑷姩浣滀负 platform 璁惧琚灇涓撅紙闄ら潪璇ヨ澶囦笌鍏剁埗璁惧涔嬮棿瀛樺湪 I2C 鎴?SPI 閾炬帴锛屾鏃?ACPI 鏍稿績浼氬皢璁惧鏋氫妇鐣欑粰鐖惰澶囩殑椹卞姩锛夛紝骞朵笖 "compatible" 灞炴€у€间腑鐨勬爣璇嗗瓧绗︿覆灏嗕笌 _CID 鍒楀嚭鐨勮澶?ID锛堝鏋滃瓨鍦級涓€璧风敤浜庝负璇ヨ澶囨煡鎵鹃┍鍔ㄣ€?

绫讳技鍦帮紝濡傛灉 PRP0001 鍑虹幇鍦?_CID 杩斿洖鐨勮澶?ID 鍒楄〃涓紝鍒?"compatible" 灞炴€у€硷紙濡傛灉瀛樺湪涓旀湁鏁堬級鍒楀嚭鐨勬爣璇嗗瓧绗︿覆灏嗚鐢ㄤ簬鏌ユ壘鍖归厤璇ヨ澶囩殑椹卞姩锛屼絾鍦ㄨ繖绉嶆儏鍐典笅锛屽畠浠浉瀵逛簬 _HID 涓?_CID 鍒楀嚭鐨勫叾浠栬澶?ID 鐨勪紭鍏堢骇锛屽彇鍐充簬 PRP0001 鍦?_CID 杩斿洖鍖呬腑鐨勪綅缃€傚叿浣撴潵璇达紝_HID 杩斿洖鐨勮澶?ID 浠ュ強鍦?_CID 杩斿洖鍖呬腑浣嶄簬 PRP0001 涔嬪墠鐨勮澶?ID 灏嗛鍏堣妫€鏌ャ€傚悓鏍峰湪杩欑鎯呭喌涓嬶紝璁惧灏嗚鏋氫妇鍒扮殑鎬荤嚎绫诲瀷鍙栧喅浜?_HID 杩斿洖鐨勮澶?ID銆?

渚嬪锛屼笅闈㈢殑 ACPI 绀轰緥鍙敤浜庢灇涓句竴涓?lm75 绫诲瀷鐨?I2C 娓╁害浼犳劅鍣紝骞朵娇鐢?Device Tree

```
	Device (TMP0)
	{
		Name (_HID, "PRP0001")
		Name (_DSD, Package () {
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package () {
				Package () { "compatible", "ti,tmp75" },
			}
		})
		Method (_CRS, 0, Serialized)
		{
			Name (SBUF, ResourceTemplate ()
			{
				I2cSerialBusV2 (0x48, ControllerInitiated,
					400000, AddressingMode7Bit,
					"\\_SB.PCI0.I2C1", 0x00,
					ResourceConsumer, , Exclusive,)
			})
			Return (SBUF)
		}
	}
```

瀹氫箟 _HID 杩斿洖 PRP0001銆佷笖 _DSD 涓病鏈?"compatible" 灞炴€ф垨 _CID 鐨勮澶囧璞℃槸鍚堟硶鐨勶紝鍙瀹冧滑鐨勬煇涓鍏堟彁渚涗簡涓€涓甫鏈夋湁鏁?"compatible" 灞炴€х殑 _DSD銆傝繖鏍风殑璁惧瀵硅薄闅忓悗琚畝鍗曞湴瑙嗕负棰濆鐨勩€屽潡銆嶏紝鍚戝鍚堢鍏堣澶囬┍鍔ㄦ彁渚涘垎灞傞厤缃俊鎭€?

涓嶈繃锛孭RP0001 鍙兘浠庤澶囧璞＄殑 _HID 鎴?_CID 杩斿洖锛屽墠鎻愭槸涓庡畠鍏宠仈鐨?_DSD锛堟棤璁烘槸璁惧瀵硅薄鑷韩鐨?_DSD锛岃繕鏄笂杩般€屽鍚堣澶囥€嶆儏鍐典笅鍏剁鍏堢殑 _DSD锛夎繑鍥炵殑鎵€鏈夊睘鎬ч兘鍙互鍦?ACPI 鐜涓娇鐢ㄣ€傚惁鍒欙紝_DSD 鏈韩琚涓烘棤鏁堬紝鍥犺€屽叾杩斿洖鐨?"compatible" 灞炴€т篃灏辨鏃犳剰涔夈€?

鏇村淇℃伅璇峰弬闃?Documentation/firmware-guide/acpi/DSD-properties-rules.rst銆?

## PCI 灞傜骇琛ㄧず


鏈夋椂锛屽湪宸茬煡 PCI 璁惧浣嶄簬 PCI 鎬荤嚎涓婄殑浣嶇疆鏃舵灇涓惧畠浼氬緢鏈夌敤銆備緥濡傦紝鏌愪簺绯荤粺灏?PCI 璁惧锛堜互澶綉銆乄i-Fi銆佷覆鍙ｇ瓑锛夌洿鎺ョ剨鎺ュ湪涓绘澘涓婂浐瀹氫綅缃€傚湪杩欑鎯呭喌涓嬶紝鍙互鏍规嵁杩欎簺 PCI 璁惧鍦?PCI 鎬荤嚎鎷撴墤涓殑浣嶇疆鏉ュ紩鐢ㄥ畠浠€?

瑕佽瘑鍒竴涓?PCI 璁惧锛岄渶瑕佸畬鏁寸殑灞傜骇鎻忚堪锛屼粠鑺墖缁勬牴绔彛涓€鐩村埌鏈€缁堣澶囷紝缁忚繃鏉夸笂鎵€鏈夌殑涓棿妗?浜ゆ崲鏈恒€?

渚嬪锛屽亣璁炬垜浠湁涓€涓郴缁燂紝鍏朵富鏉夸笂鐒婃帴浜嗕竴涓?PCIe 涓插彛鈥斺€擡xar XR17V3521銆傝 UART 鑺墖杩樺寘鍚?16 涓?GPIO锛屾垜浠笇鏈涗负杩欎簺寮曡剼娣诲姞灞炴€?`gpio-line-names` [^1^]_銆?

```
	07:00.0 Serial controller: Exar Corp. XR17V3521 Dual PCIe UART (rev 03)


	00:00.0 Host bridge: Intel Corp... Host Bridge (rev 0d)
	...
	00:13.0 PCI bridge: Intel Corp... PCI Express Port A #1 (rev fd)
	00:13.1 PCI bridge: Intel Corp... PCI Express Port A #2 (rev fd)
	00:13.2 PCI bridge: Intel Corp... PCI Express Port A #3 (rev fd)
	00:14.0 PCI bridge: Intel Corp... PCI Express Port B #1 (rev fd)
	00:14.1 PCI bridge: Intel Corp... PCI Express Port B #2 (rev fd)
	...
	05:00.0 PCI bridge: Pericom Semiconductor Device 2404 (rev 05)
	06:01.0 PCI bridge: Pericom Semiconductor Device 2404 (rev 05)
	06:02.0 PCI bridge: Pericom Semiconductor Device 2404 (rev 05)
	06:03.0 PCI bridge: Pericom Semiconductor Device 2404 (rev 05)
	07:00.0 Serial controller: Exar Corp. XR17V3521 Dual PCIe UART (rev 03) <-- Exar
	...


	-[0000:00]-+-00.0
	           ...
	           +-13.0-[01]----00.0
	           +-13.1-[02]----00.0
	           +-13.2-[03]--
	           +-14.0-[04]----00.0
	           +-14.1-[05-09]----00.0-[06-09]--+-01.0-[07]----00.0 <-- Exar
	           |                               +-02.0-[08]----00.0
	           |                               \-03.0-[09]--
	           ...
	           \-1f.1
```


瑕佹弿杩拌繖涓?Exar 璁惧鍦?PCI 鎬荤嚎涓婄殑浣嶇疆锛屾垜浠繀椤讳粠 ACPI 鍚嶇О寮€濮?

```
	Bus: 0 - Device: 14 - Function: 1
```


瑕佹壘鍒拌繖浜涗俊鎭紝鏈夊繀瑕佸弽姹囩紪 BIOS ACPI 琛紝

```
	mkdir ~/tables/
	cd ~/tables/
	acpidump > acpidump
	acpixtract -a acpidump
	iasl -e ssdt?.* -d dsdt.dat
```


鐜板湪锛屽湪 dsdt.dsl 涓紝鎴戜滑蹇呴』鎼滅储鍦板潃涓?0x14锛堣澶囷級鍜?0x01锛堝姛鑳斤級鐩稿叧鐨勮澶囥€傚湪杩欑鎯呭喌涓嬫垜浠彲浠ユ壘鍒颁互涓嬪唴瀹?

```
	Scope (_SB.PCI0)
	{
	... other definitions follow ...
		Device (RP02)
		{
			Method (_ADR, 0, NotSerialized)  // _ADR: Address
			{
				If ((RPA2 != Zero))
				{
					Return (RPA2) /* \RPA2 */
				}
				Else
				{
					Return (0x00140001)
				}
			}
	... other definitions follow ...
```


鑰?_ADR 鏂规硶 [^3^]_ 鎭板ソ杩斿洖鎴戜滑姝ｅ湪瀵绘壘鐨勮澶?鍔熻兘缁勫悎銆傚€熷姪杩欎簺淇℃伅骞跺垎鏋愪笂闈㈢殑 `lspci` 杈撳嚭锛堣澶囧垪琛ㄤ笌璁惧鏍戜袱鑰咃級锛屾垜浠彲浠ヤ负 Exar PCIe UART 缂栧啓濡備笅 ACPI 鎻忚堪锛屽悓鏃跺姞鍏ュ叾 GPIO 绾垮垪琛?

```
	Scope (_SB.PCI0.RP02)
	{
		Device (BRG1) //Bridge
		{
			Name (_ADR, 0x0000)

			Device (BRG2) //Bridge
		{
				Name (_ADR, 0x00010000)

				Device (EXAR)
			{
					Name (_ADR, 0x0000)

					Name (_DSD, Package ()
				{
						ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
						Package ()
						{
							Package ()
							{
								"gpio-line-names",
								Package ()
								{
									"mode_232",
									"mode_422",
									"mode_485",
									"misc_1",
									"misc_2",
									"misc_3",
									"",
									"",
									"aux_1",
									"aux_2",
									"aux_3",
								}
							}
						}
					}
				})
				}
			}
		}
	}
```


浣嶇疆 "_SB.PCI0.RP02" 鏄€氳繃涓婅堪瀵?dsdt.dsl 琛ㄧ殑璋冩煡寰楀埌鐨勶紝鑰岃澶囧悕 "BRG1"銆?BRG2" 涓?"EXAR" 鏄€氳繃鍒嗘瀽 Exar UART 鍦?PCI 鎬荤嚎鎷撴墤涓殑浣嶇疆鍒涘缓鐨勩€?

## 鍙傝€冭祫鏂?



```
    https://uefi.org/sites/default/files/resources/ACPI_6_3_May16.pdf锛屽紩鐢ㄦ棩鏈?2020-11-18
```
