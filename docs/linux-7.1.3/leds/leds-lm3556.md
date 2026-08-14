
## lm3556 鍐呮牳椹卞姩


- Texas Instrument锛?  1.5 A 鍚屾鍗囧帇 LED 闂厜鐏┍鍔紝甯﹂珮杈圭數娴佹簮
- 鏁版嵁鎵嬪唽锛歨ttp://www.national.com/ds/LM/LM3556.pdf

浣滆€咃細
      - Daniel Jeong

	鑱旂郴鏂瑰紡锛欴aniel Jeong(daniel.jeong-at-ti.com, gshark.jeong-at-gmail.com)

### 鎻忚堪

LM3556 鏈?3 绉嶅姛鑳斤細闂厜锛團lash锛夈€佹墜鐢碉紙Torch锛夊拰鎸囩ず锛圛ndicator锛夈€?
##### 闂厜妯″紡


鍦ㄩ棯鍏夋ā寮忎笅锛孡ED 鐢垫祦婧愶紙LED锛夋彁渚?16 涓洰鏍囩數娴佺瓑绾э紝浠?93.75 mA 鍒?1500 mA銆傞棯鍏夌數娴侀€氳繃 CURRENT CONTROL REGISTER锛?x09锛夎皟鑺傘€傞棯鍏夋ā寮忕敱 ENABLE REGISTER锛?x0A锛夋縺娲伙紝鎴栭€氳繃鎷夐珮 STROBE 寮曡剼婵€娲汇€?
LM3556 闂厜鍙€氳繃 /sys/class/leds/flash/brightness 鏂囦欢鎺у埗

- 鑻?STROBE 寮曡剼宸蹭娇鑳斤紝浠ヤ笅绀轰緥浠呮帶鍒朵寒搴︼紝
  ON/OFF 鐢?STROBE 寮曡剼鎺у埗銆?
闂厜绀轰緥锛?
```

	#echo 0 > /sys/class/leds/flash/brightness

```
```

	#echo 1 > /sys/class/leds/flash/brightness

```
...
```

	#echo 16 > /sys/class/leds/flash/brightness

```
##### 鎵嬬數妯″紡


鍦ㄦ墜鐢垫ā寮忎笅锛岀數娴佹簮锛圠ED锛夐€氳繃 CURRENT CONTROL REGISTER锛?x09锛夌紪绋嬨€傛墜鐢垫ā寮忕敱 ENABLE REGISTER锛?x0A锛夋垨纭欢 TORCH 杈撳叆婵€娲汇€?
LM3556 鎵嬬數鍙€氳繃 /sys/class/leds/torch/brightness 鏂囦欢鎺у埗銆?- 鑻?TORCH 寮曡剼宸蹭娇鑳斤紝浠ヤ笅绀轰緥浠呮帶鍒朵寒搴︼紝
  ON/OFF 鐢?TORCH 寮曡剼鎺у埗銆?
鎵嬬數绀轰緥锛?
```

	#echo 0 > /sys/class/leds/torch/brightness

```
```

	#echo 1 > /sys/class/leds/torch/brightness

```
...
```

	#echo 8 > /sys/class/leds/torch/brightness

```
##### 鎸囩ず妯″紡


鎸囩ず妯″紡鍙€氳繃 /sys/class/leds/indicator/pattern 鏂囦欢璁剧疆锛宨ndicator_pattern 鏁扮粍涓瀹氫箟浜?4 绉嶆ā寮忋€?
鏍规嵁 N-blank銆佽剦鍐叉椂闂村拰 N 鍛ㄦ湡鐨勫彇鍊硷紝浼氱敓鎴愪笉鍚岀殑妯″紡銆傚鏋滀綘鎯充负鑷湁璁惧瀹氫箟鏂版ā寮忥紝璇蜂娇鐢ㄨ嚜宸辩殑鍊间慨鏀?indicator_pattern 鏁扮粍鍜?INDIC_PATTERN_SIZE銆?
鏈夊叧 N-blank銆佽剦鍐叉椂闂村拰 N 鍛ㄦ湡鐨勬洿澶氱粏鑺傦紝璇峰弬闃呮暟鎹墜鍐屻€?
鎸囩ず妯″紡绀轰緥锛?
```

	#echo 0 > /sys/class/leds/indicator/pattern

```
...
```

	#echo 3 > /sys/class/leds/indicator/pattern

```
鎸囩ず浜害鍙€氳繃 sys/class/leds/indicator/brightness 鏂囦欢鎺у埗銆?
绀轰緥锛?
```

	#echo 0 > /sys/class/leds/indicator/brightness

```
```

	#echo 1 > /sys/class/leds/indicator/brightness

```
...
```

	#echo 8 > /sys/class/leds/indicator/brightness

```
### 娉ㄦ剰浜嬮」

椹卞姩鏈熸湜閫氳繃 i2c_board_info 鏈哄埗娉ㄥ唽銆傝鍦ㄧ壒瀹氶€傞厤鍣ㄤ笂浠ュ湴鍧€ 0x63 娉ㄥ唽璇ヨ姱鐗囷紝璇锋牴鎹?include/linux/platform_data/leds-lm3556.h 璁剧疆骞冲彴鏁版嵁锛岃缃?i2c 鏉夸俊鎭?
```

	static struct i2c_board_info board_i2c_ch4[] __initdata = {
		{
			 I2C_BOARD_INFO(LM3556_NAME, 0x63),
			 .platform_data = &lm3556_pdata,
		 },
	};

```
骞跺湪骞冲彴 init 鍑芥暟涓敞鍐屽畠

```

	board_register_i2c_bus(4, 400,
				board_i2c_ch4, ARRAY_SIZE(board_i2c_ch4));

```
