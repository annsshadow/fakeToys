## 璋冭妭鍣ㄦ満鍣ㄩ┍鍔ㄦ帴鍙ｏ紙Regulator Machine Driver Interface锛?

璋冭妭鍣ㄦ満鍣ㄩ┍鍔ㄦ帴鍙ｇ敤浜庢澘绾?鏈哄櫒鐗瑰畾鐨勫垵濮嬪寲浠ｇ爜鏉ラ厤缃皟鑺傚櫒锛坮egulator锛夊瓙绯荤粺銆?
```

  Regulator-1 -+-> Regulator-2 --> [Consumer A @ 1.8 - 2.0V]
               |
               +-> [Consumer B @ 3.3V]

```

娑堣垂鑰?A 鍜?B 鐨勯┍鍔ㄥ繀椤绘槧灏勫埌姝ｇ‘鐨勮皟鑺傚櫒锛屼互渚挎帶鍒跺畠浠殑鐢垫簮銆傝繖绉嶆槧灏勫彲浠ュ湪鏈哄櫒鍒濆鍖栦唬鐮佷腑閫氳繃涓烘瘡涓秷璐硅€呭垱寤?struct regulator_consumer_supply 鏉ュ疄鐜?
```

  struct regulator_consumer_supply {
	const char *dev_name;	/* consumer dev_name() */
	const char *supply;	/* consumer supply - e.g. "vcc" */
  };

```

```

  static struct regulator_consumer_supply regulator1_consumers[] = {
	REGULATOR_SUPPLY("Vcc", "consumer B"),
  };

  static struct regulator_consumer_supply regulator2_consumers[] = {
	REGULATOR_SUPPLY("Vcc", "consumer A"),
  };

```

杩欏皢 Regulator-1 鏄犲皠鍒?Consumer B 鐨?'Vcc' 渚涚數锛屽苟灏?Regulator-2 鏄犲皠鍒?Consumer A 鐨?'Vcc' 渚涚數銆?
鐜板湪鍙互閫氳繃涓烘瘡涓皟鑺傚櫒鐢垫簮鍩熷畾涔?struct regulator_init_data 鏉ユ敞鍐岀害鏉熴€傝缁撴瀯杩樺皢娑堣垂鑰呮槧灏勮繘鍘?
```

  static struct regulator_init_data regulator1_data = {
	.constraints = {
		.name = "Regulator-1",
		.min_uV = 3300000,
		.max_uV = 3300000,
		.valid_modes_mask = REGULATOR_MODE_NORMAL,
	},
	.num_consumer_supplies = ARRAY_SIZE(regulator1_consumers),
	.consumer_supplies = regulator1_consumers,
  };

```

name 瀛楁搴旇缃负瀵规澘绾ч厤缃叾浠栬皟鑺傚櫒鐨勪緵鐢点€佷互鍙婄敤浜庢棩蹇楀拰鍏朵粬璇婃柇杈撳嚭鏃舵湁鐢ㄦ弿杩版€х殑鍐呭銆傞€氬父鍘熺悊鍥句腑鐢ㄤ簬渚涚數杞紙supply rail锛夌殑鍚嶇О鏄竴涓笉閿欑殑閫夋嫨銆傚鏋滄病鏈夋彁渚涘悕绉帮紝瀛愮郴缁熶細鑷姩閫夋嫨涓€涓€?
Regulator-1 涓?Regulator-2 渚涚數銆傝繖绉嶅叧绯诲繀椤诲悜鏍稿績娉ㄥ唽锛屼互渚垮綋 Consumer A 鍚敤鍏朵緵鐢碉紙Regulator-2锛夋椂锛孯egulator-1 涔熶細闅忎箣鍚敤銆備緵鐢佃皟鑺傚櫒鐢?supply_regulator 瀛楁璁剧疆

```

  static struct regulator_init_data regulator2_data = {
	.supply_regulator = "Regulator-1",
	.constraints = {
		.min_uV = 1800000,
		.max_uV = 2000000,
		.valid_ops_mask = REGULATOR_CHANGE_VOLTAGE,
		.valid_modes_mask = REGULATOR_MODE_NORMAL,
	},
	.num_consumer_supplies = ARRAY_SIZE(regulator2_consumers),
	.consumer_supplies = regulator2_consumers,
  };

```

```

  static struct platform_device regulator_devices[] = {
	{
		.name = "regulator",
		.id = DCDC_1,
		.dev = {
			.platform_data = &regulator1_data,
		},
	},
	{
		.name = "regulator",
		.id = DCDC_2,
		.dev = {
			.platform_data = &regulator2_data,
		},
	},
  };
  /* register regulator 1 device */
  platform_device_register(&regulator_devices[0]);

  /* register regulator 2 device */
  platform_device_register(&regulator_devices[1]);

```
