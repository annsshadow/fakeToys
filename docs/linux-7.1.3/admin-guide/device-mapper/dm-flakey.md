## dm-flakey


璇?target 涓?linear target 鐩稿悓锛屽彧鏄畠浼氬懆鏈熸€у湴琛ㄧ幇鍑轰笉鍙潬鐨勮涓恒€傚畠宸茶璇佹槑鍦ㄦā鎷?鏁呴殰璁惧鐢ㄤ簬娴嬭瘯鏃跺緢鏈夌敤銆?
浠庤〃琚姞杞界殑鏃跺埢璧凤紝璁惧鍙敤 <up interval> 绉掞紝鐒跺悗琛ㄧ幇鍑轰笉鍙潬琛屼负 <down interval> 绉掞紝
鐒跺悗杩欎釜寰幆閲嶅銆?
鍙﹀锛屼篃鑰冭檻灏嗗叾涓?dm-delay target 缁撳悎浣跨敤锛屽悗鑰呭彲浠ュ欢杩熻鍐欏拰/鎴栧皢鍏跺彂閫佸埌涓嶅悓鐨?搴曞眰璁惧銆?
### 琛ㄥ弬鏁?

```

  <dev path> <offset> <up interval> <down interval> \
    [<num_features> [<feature arguments>]]

```
蹇呴€夊弬鏁帮細

    <dev path>锛?        搴曞眰鍧楄澶囩殑瀹屾暣璺緞鍚嶏紝鎴栬€?鈥渕ajor:minor鈥?璁惧鍙枫€?    <offset>锛?        璁惧鍐呯殑璧峰鎵囧尯銆?    <up interval>锛?        璁惧鍙敤鐨勭鏁般€?    <down interval>锛?        璁惧杩斿洖閿欒鐨勭鏁般€?
鍙€夌壒鎬у弬鏁帮細

  濡傛灉涓嶅瓨鍦ㄤ换浣曠壒鎬у弬鏁帮紝鍦ㄤ笉鍙潬鏈熼棿锛屾墍鏈?I/O 閮借繑鍥為敊璇€?
  error_reads锛?	鎵€鏈夎 I/O 閮戒互鎶ラ敊澶辫触銆?	鍐?I/O 琚纭鐞嗐€?
  drop_writes锛?	鎵€鏈夊啓 I/O 琚潤榛樺拷鐣ャ€?	璇?I/O 琚纭鐞嗐€?
  error_writes锛?	鎵€鏈夊啓 I/O 閮戒互鎶ラ敊澶辫触銆?	璇?I/O 琚纭鐞嗐€?
  corrupt_bio_byte <Nth_byte> <direction> <value> <flags>锛?	鍦?<down interval> 鏈熼棿锛屽皢姣忎釜鍖归厤 bio 鐨勬暟鎹殑绗?<Nth_byte> 瀛楄妭鏇挎崲涓?<value>銆?
    <Nth_byte>锛?	瑕佹浛鎹㈢殑瀛楄妭鍋忕Щ銆?	璁℃暟浠?1 寮€濮嬶紝浠ユ浛鎹㈢涓€涓瓧鑺傘€?    <direction>锛?	'r' 琛ㄧず鎹熷潖璇伙紝'w' 琛ㄧず鎹熷潖鍐欍€?	'w' 涓?drop_writes 涓嶅吋瀹广€?    <value>锛?	瑕佸啓鍏ョ殑鍊硷紙0-255锛夈€?    <flags>锛?	浠呭綋 bio->bi_opf 璁剧疆浜嗘墍鏈夐€夊畾鐨勬爣蹇楁椂鎵嶆墽琛屾浛鎹€?
  random_read_corrupt <probability>
	鍦?<down interval> 鏈熼棿锛屽皢璇?bio 涓殑闅忔満瀛楄妭鏇挎崲涓洪殢鏈哄€笺€俻robability 鏄竴涓粙浜?	0 鍒?1000000000 涔嬮棿鐨勬暣鏁帮紝琛ㄧず 0% 鍒?100% 鐨勬崯鍧忔鐜囥€?
  random_write_corrupt <probability>
	鍦?<down interval> 鏈熼棿锛屽皢鍐?bio 涓殑闅忔満瀛楄妭鏇挎崲涓洪殢鏈哄€笺€俻robability 鏄竴涓粙浜?	0 鍒?1000000000 涔嬮棿鐨勬暣鏁帮紝琛ㄧず 0% 鍒?100% 鐨勬崯鍧忔鐜囥€?
绀轰緥锛?
```

  corrupt_bio_byte 32 r 1 0

```
```

  corrupt_bio_byte 224 w 0 32

```
