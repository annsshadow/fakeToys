## 鍐呮牳椹卞姩 lp3944


  - National Semiconductor LP3944 Fun-light 鑺墖

    Prefix: 'lp3944'

    Addresses scanned: 鏃狅紙瑙佷笅鏂囪鏄庨儴鍒嗭級

    Datasheet:

	Publicly available at the National Semiconductor website
	http://www.national.com/pf/LP/LP3944.html

Authors:
	Antonio Ospite <ospite@studenti.unina.it>


### 鎻忚堪

LP3944 鏄竴涓緟鍔╄姱鐗囷紝鍙┍鍔ㄥ杈?8 涓?LED锛屽叿鏈変袱绉嶅彲缂栫▼ DIM 妯″紡锛涘畠鐢氳嚦鍙互鐢ㄤ綔 gpio 鎵╁睍鍣紝浣嗘湰椹卞姩鍋囧畾瀹冭鐢ㄤ綔 LED 鎺у埗鍣ㄣ€?

DIM 妯″紡鐢ㄤ簬涓?LED 璁剧疆 _闂儊_ 妯″紡锛岃妯″紡閫氳繃鎻愪緵涓や釜鍙傛暟鏉ユ寚瀹氾細

  - period锛堝懆鏈燂級锛?
	浠?0s 鍒?1.6s
  - duty cycle锛堝崰绌烘瘮锛夛細
	LED 鐐逛寒鏃堕棿鍗犲懆鏈熺殑鐧惧垎姣旓紝浠?0 鍒?100

灏?LED 璁剧疆涓?DIM0 鎴?DIM1 妯″紡浼氫娇鍏舵寜鐓ц妯″紡闂儊銆傝瑙佹暟鎹墜鍐屻€?

LP3944 鍙湪 Motorola A910 鏅鸿兘鎵嬫満涓壘鍒帮紝瀹冮┍鍔?rgb LED銆佺浉鏈洪棯鍏夌伅浠ュ強 lcd 鐨勭數婧愩€?


### 璇存槑

璇ヨ姱鐗囦富瑕佺敤浜庡祵鍏ュ紡鐜锛屽洜姝ゆ湰椹卞姩鏈熸湜瀹冮€氳繃 i2c_board_info 鏈哄埗娉ㄥ唽銆?

瑕佸湪閫傞厤鍣?0 鐨勫湴鍧€ 0x60 澶勬敞鍐岃鑺墖锛岃璁剧疆骞冲彴鏁版嵁
```

	static struct i2c_board_info a910_i2c_board_info[] __initdata = {
		{
			I2C_BOARD_INFO("lp3944", 0x60),
			.platform_data = &a910_lp3944_leds,
		},
	};

```
```

	i2c_register_board_info(0, a910_i2c_board_info,
			ARRAY_SIZE(a910_i2c_board_info));

```
