
## Linux 纭欢鏃堕棿鎴冲紩鎿庯紙HTE, Hardware Timestamping Engine锛?

:Author: Dipen Patel

### 绠€浠?

鏌愪簺璁惧鍐呯疆鏈夌‖浠舵椂闂存埑寮曟搸锛屽彲浠ュ疄鏃剁洃瑙嗕竴缁勭郴缁熶俊鍙枫€佺嚎璺€佹€荤嚎绛夌殑鐘舵€?鍙樺寲锛涗竴鏃︽娴嬪埌鍙樺寲锛屽畠浠彲浠ヨ嚜鍔ㄥ瓨鍌ㄥ彂鐢熷彉鍖栨椂鍒荤殑鏃堕棿鎴炽€備笌浣跨敤杞欢鏂瑰紡
锛堝嵆 ktime 鍙婂叾鍚岀被锛夌浉姣旓紝姝ょ被鍔熻兘鏈夊姪浜庤幏寰楁洿鍑嗙‘鐨勬椂闂存埑銆?
鏈枃妗ｆ弿杩颁簡渚涚‖浠舵椂闂存埑寮曟搸鐨勬彁渚涙柟锛坧rovider锛変笌娑堣垂鏂癸紙consumer锛夐┍鍔ㄤ娇鐢?鐨?API锛岃繖浜涢┍鍔ㄥ笇鏈涗娇鐢ㄧ‖浠舵椂闂存埑寮曟搸锛圚TE锛夋鏋躲€傛秷璐规柟涓庢彁渚涙柟閮藉繀椤诲寘鍚?`#include <linux/hte.h>`銆?
### 鎻愪緵缁欐彁渚涙柟鐨?HTE 妗嗘灦 API


   :functions: devm_hte_register_chip hte_push_ts_ns

### 鎻愪緵缁欐秷璐规柟鐨?HTE 妗嗘灦 API


   :functions: hte_init_line_attr hte_ts_get hte_ts_put devm_hte_request_ts_ns hte_request_ts_ns hte_enable_ts hte_disable_ts of_hte_req_count hte_get_clk_src_info

### HTE 妗嗘灦鍏叡缁撴瀯浣?

### 鍏充簬 HTE 鏃堕棿鎴虫暟鎹殑鏇村璇存槑


`struct hte_ts_data` 鐢ㄤ簬鍦ㄦ秷璐规柟涓庢彁渚涙柟涔嬮棿浼犻€掓椂闂存埑璇︾粏淇℃伅銆傚畠浠?u64
琛ㄨ揪绾崇绾х殑鏃堕棿鎴虫暟鎹€備笅闈㈡槸 GPIO 绾胯矾鍏稿瀷鏃堕棿鎴虫暟鎹敓鍛藉懆鏈熺殑涓€涓ず渚嬶細

```

 - 鐩戣 GPIO 绾胯矾鍙樺寲銆? - 妫€娴?GPIO 绾胯矾涓婄殑鐘舵€佸彉鍖栥€? - 灏嗘椂闂存埑杞崲涓虹撼绉掋€? - 濡傛灉鎻愪緵鏂瑰叿澶囪纭欢鑳藉姏锛屽垯灏?GPIO 鍘熷鐢靛钩瀛樺叆 raw_level 鍙橀噺銆? - 灏嗚 hte_ts_data 瀵硅薄鎺ㄩ€佺粰 HTE 瀛愮郴缁熴€? - HTE 瀛愮郴缁熼€掑 seq 璁℃暟鍣紝骞惰皟鐢ㄦ秷璐规柟鎻愪緵鐨勫洖璋冦€?   鏍规嵁鍥炶皟鐨勮繑鍥炲€硷紝HTE 鏍稿績鍦ㄧ嚎绋嬩笂涓嬫枃涓皟鐢ㄦ绾у洖璋冦€?
```

### HTE 瀛愮郴缁?debugfs 灞炴€?

HTE 瀛愮郴缁熷湪 `/sys/kernel/debug/hte/` 鍒涘缓 debugfs 灞炴€с€傚畠杩樺湪
`/sys/kernel/debug/hte/<provider>/<label or line id>/` 鍒涘缓涓庣嚎璺?淇″彿鐩稿叧鐨?debugfs 灞炴€с€傛敞鎰忚繖浜涘睘鎬ч兘鏄彧璇荤殑銆?
`ts_requested`
		浠庣粰瀹氭彁渚涙柟璇锋眰鐨勫疄浣撴€绘暟锛屽叾涓疄浣撶敱鎻愪緵鏂瑰畾涔夛紝鍙兘浠ｈ〃
		绾胯矾銆丟PIO銆佽姱鐗囦俊鍙枫€佹€荤嚎绛夆€︹€?                璇ュ睘鎬т綅浜?`/sys/kernel/debug/hte/<provider>/`銆?
`total_ts`
		鎻愪緵鏂规敮鎸佺殑瀹炰綋鎬绘暟銆?                璇ュ睘鎬т綅浜?`/sys/kernel/debug/hte/<provider>/`銆?
`dropped_timestamps`
		缁欏畾绾胯矾涓婅涓㈠純鐨勬椂闂存埑銆?                璇ュ睘鎬т綅浜?`/sys/kernel/debug/hte/<provider>/<label or line id>/`銆?