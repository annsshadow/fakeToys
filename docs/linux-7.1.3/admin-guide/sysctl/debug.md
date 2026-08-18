## /proc/sys/debug/


杩欎簺鏂囦欢鍑虹幇鍦?`/proc/sys/debug/`锛屽叿浣撳彇鍐充簬鍐呮牳閰嶇疆锛?


## exception-trace


璇ユ爣蹇楁帶鍒跺唴鏍告槸鍚﹀皢鏈夊叧鏈鐞嗕俊鍙凤紙濡傛閿欒锛夌殑淇℃伅鎵撳嵃鍒板唴鏍告棩蹇楋紙`dmesg`锛夈€?

- `0`锛氫笉璺熻釜鏈鐞嗙殑淇″彿銆?
- `1`锛氭墦鍗版湁鍏虫湭澶勭悊淇″彿鐨勪俊鎭€?

榛樿鍊煎湪澶у鏁版灦鏋勶紙濡?x86銆丮IPS銆丷ISC-V锛変笂涓?`1`锛屼絾鍦?**arm64** 涓婁负 `0`銆?

瀹為檯鎵撳嵃鐨勪俊鎭拰鎻愪緵鐨勪笂涓嬫枃鍥?CPU 鏋舵瀯鑰屽紓锛屽樊寮傛樉钁椼€備緥濡傦細

- 鍦?**x86** 涓婏紝閫氬父鎵撳嵃鎸囦护鎸囬拡锛圛P锛夈€侀敊璇爜浠ュ強瀵艰嚧椤甸敊璇殑鍦板潃銆?
- 鍦?**PowerPC** 涓婏紝鍙兘鎵撳嵃涓嬩竴鎸囦护鎸囬拡锛圢IP锛夈€侀摼鎺ュ瘎瀛樺櫒锛圠R锛変互鍙婂叾瀹冪浉鍏冲瘎瀛樺櫒銆?

鍚敤鍚庯紝璇ョ壒鎬ч€氬父浼氳闄愰€燂紝浠ラ槻姝㈠湪鍐呮牳宕╂簝寰幆涓唴鏍告棩蹇楄娣规病銆?

## kprobes-optimization


璇ユ爣蹇楀惎鐢ㄦ垨绂佺敤鏌愪簺鏋舵瀯锛堝 x86锛変笂 Kprobes 鐨勪紭鍖栥€?

- `0`锛氬叧闂?Kprobes 浼樺寲銆?
- `1`锛氬紑鍚?Kprobes 浼樺寲锛堥粯璁わ級銆?

鏈夊叧 Kprobes 鍙婂叾浼樺寲鐨勬洿澶氱粏鑺傦紝璇峰弬闃?Documentation/trace/kprobes.rst銆?

Copyright (c) 2026, Shubham Chakraborty <chakrabortyshubham66@gmail.com>

鏈夊叧涓€鑸俊鎭拰娉曞緥澹版槑锛岃鍙傞槄 Documentation/admin-guide/sysctl/index.rst銆?
