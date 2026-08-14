
## 濡備綍浣跨敤 radiotap 澶?

### 鎸囧悜 radiotap 澶村寘鍚枃浠?

Radiotap 澶存槸鍙橀暱涓斿彲鎵╁睍鐨勶紝浣犲彲浠ヤ粠浠ヤ笅浣嶇疆鑾峰彇澶ч儴鍒?```

    ./include/net/ieee80211_radiotap.h

```
鏈枃妗ｇ粰鍑轰竴涓瑙堬紝骞跺氨涓€浜涜竟鐣屾儏鍐电粰鍑鸿绀恒€?

### 澶寸殑缁撴瀯


寮€澶存湁涓€涓浐瀹氶儴鍒嗭紝鍏朵腑鍖呭惈涓€涓?u32 浣嶅浘锛岀敤浜庡畾涔変笌姣忎釜浣嶅叧鑱旂殑鍙兘鍙傛暟鏄惁瀛樺湪銆?鍥犳锛屽鏋?ieee80211_radiotap_header 鐨?it_present 鎴愬憳鐨?b0 琚疆浣嶏紝灏辨剰鍛崇潃鍙傛暟鍖轰腑
瀛樺湪鍙傛暟绱㈠紩 0锛圛EEE80211_RADIOTAP_TSFT锛夌殑澶淬€?
```

   < 8-byte ieee80211_radiotap_header >
   [ <possible argument bitmap extensions ... > ]
   [ <argument> ... ]

```
鐩墠鍙畾涔変簡 13 涓彲鑳界殑鍙傛暟绱㈠紩锛屼絾濡傛灉鎴戜滑鐢ㄥ畬浜?u32 it_present 鎴愬憳涓殑绌洪棿锛岃瀹?b31 缃綅琛ㄧず鍚庨潰杩樻湁涓€涓?u32 浣嶅浘锛堝涓婃墍绀衡€滃彲鑳界殑鍙傛暟浣嶅浘鎵╁睍...鈥濓級锛屽苟涓旀瘡娆″弬鏁?鐨勮捣鐐瑰悜鍓嶇Щ鍔?4 瀛楄妭銆?
杩樿娉ㄦ剰锛宨t_len 鎴愬憳 __le16 琚涓?ieee80211_radiotap_header 鍙婂叾鍚庝换浣曞弬鏁版墍瑕嗙洊鐨?鎬诲瓧鑺傛暟銆?

### 瀵瑰弬鏁扮殑瑕佹眰


鍦ㄥご鐨勫浐瀹氶儴鍒嗕箣鍚庯紝鏄偅浜涘湪 ieee80211_radiotap_header 鐨?it_present 鎴愬憳涓搴斾綅琚?缃綅鐨勬瘡涓弬鏁扮储寮曠殑鍙傛暟锛屼緷娆¤窡闅忋€?
 - 鎵€鏈夊弬鏁伴兘浠ュ皬绔紙little-endian锛夋柟寮忓瓨鍌紒

 - 缁欏畾鍙傛暟绱㈠紩鐨勫弬鏁拌浇鑽峰叿鏈夊浐瀹氬ぇ灏忋€傚洜姝?IEEE80211_RADIOTAP_TSFT 瀛樺湪鎬绘槸琛ㄧず瀛樺湪涓€涓?   8 瀛楄妭鍙傛暟銆傚弬瑙?./include/net/ieee80211_radiotap.h 涓殑娉ㄩ噴锛屼簡瑙ｆ墍鏈夊弬鏁板ぇ灏忕殑
   娓呮櫚缁嗗垎銆?
 - 鍙傛暟蹇呴』浣跨敤濉厖锛坧adding锛夊榻愬埌鍙傛暟澶у皬鎵€鍦ㄨ竟鐣屻€傚洜姝や竴涓?u16 鍙傛暟鑻ュ皻鏈浜?u16
   杈圭晫涓婏紝鍒欏繀椤诲紑濮嬩簬涓嬩竴涓?u16 杈圭晫锛泆32 蹇呴』寮€濮嬩簬涓嬩竴涓?u32 杈圭晫锛屼緷姝ょ被鎺ㄣ€?
 - 鈥滃榻愨€濇槸鐩稿浜?ieee80211_radiotap_header 鐨勮捣鐐癸紙鍗?radiotap 澶寸殑绗竴涓瓧鑺傦級鑰岃█鐨勩€?   璇ョ涓€涓瓧鑺傜殑缁濆瀵归綈鏂瑰紡骞舵湭瀹氫箟銆傚洜姝ゅ嵆浣挎暣涓?radiotap 澶村紑濮嬩簬渚嬪鍦板潃 0x00000003锛?   鍑轰簬瀵归綈鐩殑锛岃 radiotap 澶寸殑绗竴涓瓧鑺備粛琚涓?0銆?
 - 涓婇潰涓€鐐硅鏄庯紝鍦ㄥ浐瀹氱殑 radiotap 澶存垨鍙傛暟鍖轰腑锛屽瀛楄妭瀹炰綋鍙兘娌℃湁浠讳綍缁濆瀵归綈锛岃繖鎰忓懗鐫€
   浣犲湪灏濊瘯璁块棶杩欎簺澶氬瓧鑺傚疄浣撴椂蹇呴』閲囧彇鐗规畩鐨勮閬挎帾鏂姐€傛煇浜涙灦鏋勶紙濡?Blackfin锛夋棤娉曞鐞?   瀵规寚鍚戝鏁板湴鍧€鐨?u16 鎸囬拡鐨勮В寮曠敤銆傜浉鍙嶏紝浣犲繀椤讳娇鐢ㄥ唴鏍?API get_unaligned() 鏉ヨВ寮曠敤璇?   鎸囬拡锛屽湪闇€瑕佽繖鏍峰仛鐨勬灦鏋勪笂瀹冧細閫愬瓧鑺傚湴杩涜銆?
 - 缁欏畾鍙傛暟绱㈠紩鐨勫弬鏁板彲浠ユ槸澶氱绫诲瀷缁勫悎鍦ㄤ竴璧枫€備緥濡?IEEE80211_RADIOTAP_CHANNEL 鐨勫弬鏁拌浇鑽?   鐢变袱涓?u16 缁勬垚锛屾€婚暱搴︿负 4銆傚綋杩欑鎯呭喌鍙戠敓鏃讹紝搴旂敤鐨勬槸澶勭悊 u16 鐨勫～鍏呰鍒欙紝鑰屼笉鏄鐞?   4 瀛楄妭鍗曚竴瀹炰綋鐨勮鍒欍€?

### 绀轰緥锛氭湁鏁堢殑 radiotap 澶?

```

	0x00, 0x00, // <-- radiotap version + pad byte
	0x0b, 0x00, // <- radiotap header length
	0x04, 0x0c, 0x00, 0x00, // <-- bitmap
	0x6c, // <-- rate (in 500kHz units)
	0x0c, //<-- tx power
	0x01 //<-- antenna


```
### 浣跨敤 Radiotap 瑙ｆ瀽鍣?

濡傛灉浣犻渶瑕佽В鏋愪竴涓?radiotap 缁撴瀯锛屽彲浠ヤ娇鐢ㄤ綅浜?net/wireless/radiotap.c 涓殑 radiotap
瑙ｆ瀽鍣ㄥぇ骞呯畝鍖栧伐浣滐紝瀹冩嫢鏈?```

    #include <net/cfg80211.h>

    /* buf points to the start of the radiotap header part */

    int MyFunction(u8 * buf, int buflen)
    {
	    int pkt_rate_100kHz = 0, antenna = 0, pwr = 0;
	    struct ieee80211_radiotap_iterator iterator;
	    int ret = ieee80211_radiotap_iterator_init(&iterator, buf, buflen);

	    while (!ret) {

		    ret = ieee80211_radiotap_iterator_next(&iterator);

		    if (ret)
			    continue;

		    /* see if this argument is something we can use */

		    switch (iterator.this_arg_index) {
		    /*
		    * You must take care when dereferencing iterator.this_arg
		    * for multibyte types... the pointer is not aligned.  Use
		    * get_unaligned((type *)iterator.this_arg) to dereference
		    * iterator.this_arg for type "type" safely on all arches.
		    */
		    case IEEE80211_RADIOTAP_RATE:
			    /* radiotap "rate" u8 is in
			    * 500kbps units, eg, 0x02=1Mbps
			    */
			    pkt_rate_100kHz = (*iterator.this_arg) * 5;
			    break;

		    case IEEE80211_RADIOTAP_ANTENNA:
			    /* radiotap uses 0 for 1st ant */
			    antenna = *iterator.this_arg);
			    break;

		    case IEEE80211_RADIOTAP_DBM_TX_POWER:
			    pwr = *iterator.this_arg;
			    break;

		    default:
			    break;
		    }
	    }  /* while more rt headers */

	    if (ret != -ENOENT)
		    return TXRX_DROP;

	    /* discard the radiotap header part */
	    buf += iterator.max_length;
	    buflen -= iterator.max_length;

	    ...

    }

```
Andy Green <andy@warmcat.com>
