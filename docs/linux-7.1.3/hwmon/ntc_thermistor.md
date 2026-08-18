## 鍐呮牳椹卞姩 ntc_thermistor


鏉ヨ嚜 Murata 鐨勫彈鏀寔鐑晱鐢甸樆锛?
- Murata NTC 鐑晱鐢甸樆 NCP15WB473銆丯CP18WB473銆丯CP21WB473銆丯CP03WB473銆?  NCP15WL333銆丯CP03WF104銆丯CP15XH103

  Prefixes: 'ncp15wb473', 'ncp18wb473', 'ncp21wb473', 'ncp03wb473',
  'ncp15wl333', 'ncp03wf104', 'ncp15xh103'

  Datasheet: 鍙湪 Murata 鍏紑鑾峰彇

鏉ヨ嚜 EPCOS 鐨勫彈鏀寔鐑晱鐢甸樆锛?
- EPCOS NTC 鐑晱鐢甸樆 B57330V2103

  Prefixes: b57330v2103

  Datasheet: 鍙湪 EPCOS 鍏紑鑾峰彇

鍏朵粬 NTC 鐑晱鐢甸樆鍙渶閫氳繃娣诲姞琛ュ伩琛ㄥ嵆鍙敮鎸侊紱渚嬪锛孨CP15WL333 鐨勬敮鎸佹槸閫氳繃
ncpXXwl333 琛ㄦ坊鍔犵殑銆?
Authors:

	MyungJoo Ham <myungjoo.ham@samsung.com>

### 鎻忚堪


NTC锛堣礋娓╁害绯绘暟锛夌儹鏁忕數闃绘槸涓€绉嶇畝鍗曠殑鐑晱鐢甸樆锛岃姹傜敤鎴锋彁渚涚數闃诲€煎苟鏌ユ壘鐩稿簲鐨勮ˉ鍋胯〃
浠ヨ幏寰楁俯搴﹁緭鍏ャ€?
NTC 椹卞姩鎻愪緵甯︽湁绾挎€ц繎浼煎嚱鏁扮殑鏌ユ壘琛紝浠ュ強鍥涚鐢佃矾妯″瀷锛屽苟鍙€変笉浣跨敤鍏朵腑浠讳綍涓€绉嶆ā鍨嬨€?
```

   $	resistor
   [TH]	the thermistor

```
The four circuit models provided are:

```

     [pullup_uV]
	 |    |
	[TH]  $ (pullup_ohm)
	 |    |
	 +----+-----------------------[read_uV]
	 |
	 $ (pulldown_ohm)
	 |
	-+- (ground)

```
```

     [pullup_uV]
	 |
	[TH]
	 |
	 +----------------------------[read_uV]
	 |
	 $ (pulldown_ohm)
	 |
	-+- (ground)

```
```

     [pullup_uV]
	 |
	 $ (pullup_ohm)
	 |
	 +----+-----------------------[read_uV]
	 |    |
	[TH]  $ (pulldown_ohm)
	 |    |
	-+----+- (ground)

```
```

     [pullup_uV]
	 |
	 $ (pullup_ohm)
	 |
	 +----------------------------[read_uV]
	 |
	[TH]
	 |
	-+- (ground)

```
When one of the four circuit models is used, read_uV, pullup_uV, pullup_ohm,
pulldown_ohm, and connect should be provided. When none of the four models
are suitable or the user can get the resistance directly, the user should
provide read_ohm and _not_ provide the others.

### Sysfs 鎺ュ彛


=============== == =============================================================
name		   蹇呭～鐨勫叏灞€灞炴€э紝鍗崇儹鏁忕數闃荤殑鍚嶇О銆?=============== == =============================================================
temp1_type	RO 濮嬬粓涓?4锛堢儹鏁忕數闃伙級

temp1_input	RO 娴嬮噺娓╁害骞舵彁渚涙祴寰楃殑鍊笺€?		   锛堣鍙栨鏂囦欢浼氬惎鍔ㄨ鍙栬繃绋嬨€傦級
=============== == =============================================================

娉ㄦ剰姣忎釜 NTC 鐑晱鐢甸樆鍙湁涓€涓儹鏁忕數闃伙紱鍥犳鍙瓨鍦?temp1銆?