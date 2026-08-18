## 鏁呴殰娉ㄥ叆鑳藉姏鍩虹璁炬柦


鍙﹁鍙傝 scsi_debug 鐨?"every_nth" 妯″潡閫夐」銆?

### 鍙敤鐨勬晠闅滄敞鍏ヨ兘鍔?

- failslab

  娉ㄥ叆 slab 鍒嗛厤澶辫触銆傦紙kmalloc()銆乲mem_cache_alloc()銆?..锛?
- fail_page_alloc

  娉ㄥ叆椤靛垎閰嶅け璐ャ€傦紙alloc_pages()銆乬et_free_pages()銆?..锛?
- fail_usercopy

  鍦ㄧ敤鎴峰唴瀛樿闂嚱鏁颁腑娉ㄥ叆澶辫触銆傦紙copy_from_user()銆乬et_user()銆?..锛?
- fail_futex

  娉ㄥ叆 futex 姝婚攣鍜?uaddr 閿欒銆?
- fail_sunrpc

  娉ㄥ叆鍐呮牳 RPC 瀹㈡埛绔拰鏈嶅姟鍣ㄧ澶辫触銆?
- fail_make_request

  鍦ㄩ€氳繃璁剧疆 /sys/block/<device>/make-it-fail 鎴?/sys/block/<device>/<partition>/make-it-fail 鎵€鍏佽鐨勮澶囦笂娉ㄥ叆纾佺洏 IO 閿欒銆傦紙submit_bio_noacct()锛?
- fail_mmc_request

  鍦ㄩ€氳繃璁剧疆 /sys/kernel/debug/mmc0/fail_mmc_request 涓嬬殑 debugfs 鏉＄洰鎵€鍏佽鐨勮澶囦笂娉ㄥ叆 MMC 鏁版嵁閿欒銆?
- fail_function

  閫氳繃璁剧疆鍦?/sys/kernel/debug/fail_function 涓嬬殑 debugfs 鏉＄洰锛屽鏍囪浜?ALLOW_ERROR_INJECTION() 瀹忕殑鐗瑰畾鍑芥暟娉ㄥ叆閿欒杩斿洖銆備笉鏀寔寮曞閫夐」銆?
- fail_skb_realloc

  灏?skb锛堝鎺ュ瓧缂撳啿鍖猴級閲嶆柊鍒嗛厤浜嬩欢娉ㄥ叆缃戠粶璺緞銆備富瑕佺洰鏍囨槸璇嗗埆骞堕槻姝㈢綉缁滃瓙绯荤粺涓笌鎸囬拡绠＄悊涓嶅綋鐩稿叧鐨勯棶棰樸€傞€氳繃鍦ㄥ叧閿偣寮哄埗 skb 閲嶆柊鍒嗛厤锛屾鐗规€у埗閫犱簡鐜版湁鎸囧悜 skb 澶撮儴鎸囬拡澶辨晥鐨勫満鏅€?
  褰撴晠闅滆娉ㄥ叆骞惰Е鍙戦噸鏂板垎閰嶆椂锛岀紦瀛樼殑鎸囧悜 skb 澶撮儴鍜屾暟鎹殑鎸囬拡涓嶅啀寮曠敤鏈夋晥鐨勫唴瀛樹綅缃€傝繖绉嶆晠鎰忕殑澶辨晥鏈夊姪浜庢毚闇查噸鏂板垎閰嶄簨浠跺悗鏈纭洿鏂版寚閽堢殑浠ｇ爜璺緞銆?
  閫氳繃鍒涘缓杩欎簺鍙楁帶鐨勬晠闅滃満鏅紝绯荤粺鍙互鎹曡幏浣跨敤闄堟棫鎸囬拡鐨勬儏鍐碉紝杩欏彲鑳藉鑷村唴瀛樻崯鍧忔垨绯荤粺涓嶇ǔ瀹氥€?
  瑕侀€夋嫨浣滅敤鐨勬帴鍙ｏ紝灏嗙綉缁滃悕绉板啓鍏?/sys/kernel/debug/fail_skb_realloc/devname銆傚鏋滄瀛楁鐣欑┖锛堝嵆榛樿鍊硷級锛宻kb 閲嶆柊鍒嗛厤灏嗚寮哄埗搴旂敤浜庢墍鏈夌綉缁滄帴鍙ｃ€?
  褰撳惎鐢?KASAN 鏃讹紝姝ゆ晠闅滄娴嬬殑鏈夋晥鎬т細澧炲己锛屽洜涓哄畠鏈夊姪浜庤瘑鍒棤鏁堝唴瀛樺紩鐢ㄥ拰閲婃斁鍚庝娇鐢紙UAF锛夐棶棰樸€?
- NVMe 鏁呴殰娉ㄥ叆

  鍦ㄩ€氳繃璁剧疆 /sys/kernel/debug/nvme*/fault_inject 涓嬬殑 debugfs 鏉＄洰鎵€鍏佽鐨勮澶囦笂锛屾敞鍏?NVMe 鐘舵€佺爜鍜岄噸璇曟爣蹇椼€傞粯璁ょ姸鎬佺爜涓?NVME_SC_INVALID_OPCODE锛屼笉閲嶈瘯銆傜姸鎬佺爜鍜岄噸璇曟爣蹇楀彲浠ラ€氳繃 debugfs 璁剧疆銆?
- Null 娴嬭瘯鍧楄澶囬┍鍔ㄦ晠闅滄敞鍏?
  閫氳繃璁剧疆 /sys/kernel/config/nullb/<disk>/timeout_inject 涓嬬殑閰嶇疆椤规敞鍏?IO 瓒呮椂锛岄€氳繃璁剧疆 /sys/kernel/config/nullb/<disk>/requeue_inject 涓嬬殑閰嶇疆椤规敞鍏ラ噸鏂版帓闃熻姹傦紝浠ュ強閫氳繃璁剧疆 /sys/kernel/config/nullb/<disk>/init_hctx_fault_inject 涓嬬殑閰嶇疆椤规敞鍏?init_hctx() 閿欒銆?
### 閰嶇疆鏁呴殰娉ㄥ叆鑳藉姏鐨勮涓?

##### debugfs 鏉＄洰


fault-inject-debugfs 鍐呮牳妯″潡鎻愪緵浜嗕竴浜?debugfs 鏉＄洰锛岀敤浜庤繍琛屾椂閰嶇疆鏁呴殰娉ㄥ叆鑳藉姏銆?
- /sys/kernel/debug/fail*/probability:

	娉ㄥ叆澶辫触鐨勫彲鑳芥€э紝浠ョ櫨鍒嗘瘮琛ㄧず銆?
	鏍煎紡锛?percent>

	娉ㄦ剰锛屾瘡鐧炬涓€娆″け璐ュ鏌愪簺娴嬭瘯鐢ㄤ緥鏉ヨ鏄浉褰撻珮鐨勯敊璇巼銆傚浜庢绫绘祴璇曠敤渚嬶紝鑰冭檻璁剧疆 probability=100 骞堕厤缃?/sys/kernel/debug/fail*/interval銆?
- /sys/kernel/debug/fail*/interval:

	鎸囧畾澶辫触涔嬮棿鐨勯棿闅旓紝閽堝閫氳繃浜嗘墍鏈夊叾浠栨祴璇曠殑 should_fail() 璋冪敤銆?
	娉ㄦ剰锛屽鏋滀綘閫氳繃 interval>1 鍚敤浜嗗畠锛屼綘寰堝彲鑳芥兂瑕佽缃?probability=100銆?
- /sys/kernel/debug/fail*/times:

	鎸囧畾澶辫触鏈€澶氬彲鑳藉彂鐢熺殑娆℃暟銆傚€?-1 琛ㄧず"鏃犻檺鍒?銆?
- /sys/kernel/debug/fail*/space:

	鎸囧畾涓€涓垵濮嬭祫婧?棰勭畻"锛屾瘡娆¤皟鐢?should_fail(,size) 鏃舵寜 "size" 閫掑噺銆傚湪 "space" 杈惧埌闆朵箣鍓嶏紝鏁呴殰娉ㄥ叆琚姂鍒躲€?
- /sys/kernel/debug/fail*/verbose

	鏍煎紡锛歿 0 | 1 | 2 }

	鎸囧畾娉ㄥ叆澶辫触鏃舵秷鎭殑璇︾粏绋嬪害銆?0' 琛ㄧず鏃犳秷鎭紱'1' 姣忔澶辫触鍙墦鍗颁竴琛屾棩蹇楋紱'2' 杩樹細鎵撳嵃璋冪敤鏍堣窡韪€斺€旀湁鍔╀簬璋冭瘯鏁呴殰娉ㄥ叆鏆撮湶鐨勯棶棰樸€?
- /sys/kernel/debug/fail*/task-filter:

	鏍煎紡锛歿 'Y' | 'N' }

	鍊?'N' 绂佺敤鎸夎繘绋嬭繃婊わ紙榛樿锛夈€備换浣曟鍊煎皢澶辫触闄愬埗涓轰粎鐢?/proc/<pid>/make-it-fail==1 鎸囩ず鐨勮繘绋嬨€?
- /sys/kernel/debug/fail*/require-start銆?  /sys/kernel/debug/fail*/require-end銆?  /sys/kernel/debug/fail*/reject-start銆?  /sys/kernel/debug/fail*/reject-end:

	鎸囧畾鍦ㄦ爤璺熻釜閬嶅巻鏈熼棿娴嬭瘯鐨勮櫄鎷熷湴鍧€鑼冨洿銆備粎褰撹閬嶅巻鏍堣窡韪腑鐨勬煇涓皟鐢ㄨ€呬綅浜庢墍闇€鑼冨洿鍐咃紝涓旀病鏈夎皟鐢ㄨ€呬綅浜庢嫆缁濊寖鍥村唴鏃讹紝鎵嶆敞鍏ュけ璐ャ€傞粯璁ゆ墍闇€鑼冨洿涓?[0,ULONG_MAX)锛堟暣涓櫄鎷熷湴鍧€绌洪棿锛夈€傞粯璁ゆ嫆缁濊寖鍥翠负 [0,0)銆?
- /sys/kernel/debug/fail*/stacktrace-depth:

	鎸囧畾鍦ㄦ悳绱?[require-start,require-end) 鎴?[reject-start,reject-end) 鑼冨洿鍐呰皟鐢ㄨ€呮椂閬嶅巻鐨勬渶澶ф爤璺熻釜娣卞害銆?
- /sys/kernel/debug/fail_page_alloc/ignore-gfp-highmem:

	鏍煎紡锛歿 'Y' | 'N' }

	榛樿鏄?'Y'锛屽皢鍏惰缃负 'N' 涔熶細鍚?highmem/鐢ㄦ埛鍒嗛厤锛坃_GFP_HIGHMEM 鍒嗛厤锛夋敞鍏ュけ璐ャ€?
- /sys/kernel/debug/failslab/cache-filter
	鏍煎紡锛歿 'Y' | 'N' }

        榛樿鏄?'N'锛屽皢鍏惰缃负 'Y' 灏嗗彧鍦ㄥ璞℃潵鑷煇浜涚壒瀹氱紦瀛樻椂鎵嶆敞鍏ュけ璐ャ€?
        閫氳繃鍚?/sys/kernel/slab/<cache>/failslab 鍐欏叆 '1' 鏉ラ€夋嫨缂撳瓨锛?
- /sys/kernel/debug/failslab/ignore-gfp-wait:
- /sys/kernel/debug/fail_page_alloc/ignore-gfp-wait:

	鏍煎紡锛歿 'Y' | 'N' }

	榛樿鏄?'Y'锛屽皢鍏惰缃负 'N' 涔熶細鍚戝彲浠ョ潯鐪犵殑鍒嗛厤锛坃_GFP_DIRECT_RECLAIM 鍒嗛厤锛夋敞鍏ュけ璐ャ€?
- /sys/kernel/debug/fail_page_alloc/min-order:

	鎸囧畾瑕佹敞鍏ュけ璐ョ殑鏈€灏忛〉鍒嗛厤闃躲€?
- /sys/kernel/debug/fail_futex/ignore-private:

	鏍煎紡锛歿 'Y' | 'N' }

	榛樿鏄?'N'锛屽皢鍏惰缃负 'Y' 灏嗗湪澶勭悊绉佹湁锛堝湴鍧€绌洪棿锛塮utex 鏃剁鐢ㄥけ璐ユ敞鍏ャ€?
- /sys/kernel/debug/fail_sunrpc/ignore-client-disconnect:

	鏍煎紡锛歿 'Y' | 'N' }

	榛樿鏄?'N'锛屽皢鍏惰缃负 'Y' 灏嗙鐢?RPC 瀹㈡埛绔笂鐨勬柇寮€杩炴帴娉ㄥ叆銆?
- /sys/kernel/debug/fail_sunrpc/ignore-server-disconnect:

	鏍煎紡锛歿 'Y' | 'N' }

	榛樿鏄?'N'锛屽皢鍏惰缃负 'Y' 灏嗙鐢?RPC 鏈嶅姟鍣ㄧ涓婄殑鏂紑杩炴帴娉ㄥ叆銆?
- /sys/kernel/debug/fail_sunrpc/ignore-cache-wait:

	鏍煎紡锛歿 'Y' | 'N' }

	榛樿鏄?'N'锛屽皢鍏惰缃负 'Y' 灏嗙鐢?RPC 鏈嶅姟鍣ㄧ涓婄殑缂撳瓨绛夊緟娉ㄥ叆銆?
- /sys/kernel/debug/fail_function/inject:

	鏍煎紡锛歿 'function-name' | '!function-name' | '' }

	閫氳繃鍚嶇О鎸囧畾閿欒娉ㄥ叆鐨勭洰鏍囧嚱鏁般€傚鏋滃嚱鏁板悕甯︽湁 '!' 鍓嶇紑锛屽垯灏嗕粠娉ㄥ叆鍒楄〃涓Щ闄ょ粰瀹氬嚱鏁般€傚鏋滄湭鎸囧畾浠讳綍鍐呭锛?'锛夛紝鍒欐竻绌烘敞鍏ュ垪琛ㄣ€?
- /sys/kernel/debug/fail_function/injectable:

	锛堝彧璇伙級鏄剧ず鍙敞鍏ラ敊璇殑鍑芥暟浠ュ強鍙互鎸囧畾鐨勯敊璇€肩被鍨嬨€傞敊璇被鍨嬪皢鏄互涓嬩箣涓€锛? - NULL:	retval 蹇呴』涓?0銆? - ERRNO: retval 蹇呴』涓?-1 鍒?-MAX_ERRNO锛?4096锛夈€? - ERR_NULL: retval 蹇呴』涓?0 鎴?-1 鍒?-MAX_ERRNO锛?4096锛夈€?
- /sys/kernel/debug/fail_function/<function-name>/retval:

	鎸囧畾瑕佹敞鍏ュ埌缁欏畾鍑芥暟鐨?閿欒"杩斿洖鍊笺€傚綋鐢ㄦ埛鎸囧畾涓€涓柊鐨勬敞鍏ユ潯鐩椂浼氬垱寤烘鏂囦欢銆傛敞鎰忔鏂囦欢鍙帴鍙楁棤绗﹀彿鍊笺€傚洜姝わ紝濡傛灉浣犳兂浣跨敤璐熺殑 errno锛屼綘鏈€濂戒娇鐢?'printf' 鑰屼笉鏄?'echo'锛屼緥濡傦細
	$ printf %#x -12 > retval

- /sys/kernel/debug/fail_skb_realloc/devname:

        鎸囧畾瑕佸己鍒惰繘琛?SKB 閲嶆柊鍒嗛厤鐨勭綉缁滄帴鍙ｃ€傚鏋滅暀绌猴紝SKB 閲嶆柊鍒嗛厤灏嗗簲鐢ㄤ簬鎵€鏈夌綉缁滄帴鍙ｃ€?
```
          # 鍦?eth0 涓婂己鍒惰繘琛?skb 閲嶆柊鍒嗛厤
          echo "eth0" > /sys/kernel/debug/fail_skb_realloc/devname

          # 娓呴櫎閫夋嫨骞跺湪鎵€鏈夋帴鍙ｄ笂寮哄埗杩涜 skb 閲嶆柊鍒嗛厤
          echo "" > /sys/kernel/debug/fail_skb_realloc/devname
```

##### 寮曞閫夐」


涓轰簡鍦?debugfs 涓嶅彲鐢ㄦ椂锛堟棭鏈熷惎鍔ㄦ湡闂达級娉ㄥ叆鏁呴殰锛?
```
	failslab=
	fail_page_alloc=
	fail_usercopy=
	fail_make_request=
	fail_futex=
	fail_skb_realloc=
	mmc_core.fail_request=<interval>,<probability>,<space>,<times>
```

##### proc 鏉＄洰


- /proc/<pid>/fail-nth銆?  /proc/self/task/<tid>/fail-nth:

	鍚戞鏂囦欢鍐欏叆鏁存暟 N 浼氫娇璇ヤ换鍔′腑鐨勭 N 娆¤皟鐢ㄥけ璐ャ€備粠姝ゆ枃浠惰鍙栦細杩斿洖涓€涓暣鏁板€笺€傚€?'0' 琛ㄧず鐢ㄥ厛鍓嶅姝ゆ枃浠剁殑鍐欏叆鎵€璁剧疆鐨勬晠闅滃凡琚敞鍏ャ€傛鏁存暟 N 琛ㄧず鏁呴殰灏氭湭琚敞鍏ャ€傛敞鎰忔鏂囦欢鍚敤鎵€鏈夌被鍨嬬殑鏁呴殰锛坰lab銆乫utex 绛夛級銆傛璁剧疆浼樺厛浜庢墍鏈夊叾浠栭€氱敤鐨?debugfs 璁剧疆锛堝 probability銆乮nterval銆乼imes 绛夛級銆備絾姣忚兘鍔涜缃紙渚嬪 fail_futex/ignore-private锛変紭鍏堜簬瀹冦€?
	姝ょ壒鎬ф棬鍦ㄧ敤浜庡崟涓郴缁熻皟鐢ㄧ殑鏁呴殰绯荤粺鎬ф祴璇曘€傚弬瑙佷笅闈㈢殑渚嬪瓙銆?

### 鍙敞鍏ラ敊璇殑鍑芥暟


鏈儴鍒嗛潰鍚戣€冭檻鍚?ALLOW_ERROR_INJECTION() 瀹忔坊鍔犲嚱鏁扮殑鍐呮牳寮€鍙戣€呫€?

##### 鍙敞鍏ラ敊璇嚱鏁扮殑瑕佹眰


鐢变簬鍑芥暟绾ч敊璇敞鍏ヤ細寮鸿鏀瑰彉浠ｇ爜璺緞骞惰繑鍥為敊璇紝鍗充娇杈撳叆鍜屾潯浠堕兘姝ｇ‘锛屽鏋滃厑璁稿涓嶅彲娉ㄥ叆閿欒鐨勫嚱鏁拌繘琛岄敊璇敞鍏ワ紝鍙兘瀵艰嚧鎰忓鐨勫唴鏍稿穿婧冦€傚洜姝わ紝浣狅紙鍜屽闃呰€咃級蹇呴』纭繚锛?
- 鍑芥暟鍦ㄥけ璐ユ椂浼氳繑鍥為敊璇爜锛屽苟涓旇皟鐢ㄨ€呭繀椤绘纭鏌ュ畠锛堥渶瑕佽兘澶熶粠涓仮澶嶏級銆?
- 鍑芥暟鍦ㄧ涓€娆￠敊璇繑鍥炰箣鍓嶄笉浼氭墽琛屼换浣曞彲鑳芥敼鍙樹换浣曠姸鎬佺殑浠ｇ爜銆傝鐘舵€佸寘鎷叏灞€鎴栧眬閮紝鎴栬緭鍏ュ彉閲忋€備緥濡傦紝娓呴櫎杈撳嚭鍦板潃瀛樺偍锛堜緥濡?`*ret = NULL`锛夈€侀€掑/閫掑噺璁℃暟鍣ㄣ€佽缃爣蹇椼€佹姠鍗?涓柇绂佺敤鎴栬幏鍙栭攣锛堝鏋滆繖浜涘湪杩斿洖閿欒涔嬪墠琚仮澶嶏紝鍒欏彲浠ワ級銆?
绗竴涓姹傚緢閲嶈锛屽畠浼氬鑷撮噴鏀撅紙閲婃斁瀵硅薄锛夊嚱鏁伴€氬父姣斿垎閰嶅嚱鏁版洿闅炬敞鍏ラ敊璇€傚鏋滄绫婚噴鏀惧嚱鏁扮殑閿欒娌℃湁琚纭鐞嗭紝寰堝鏄撳鑷村唴瀛樻硠婕忥紙璋冪敤鑰呬細璇互涓哄璞″凡琚噴鏀炬垨宸叉崯鍧忥級銆?
绗簩涓姹傛槸閽堝璋冪敤鑰呯殑锛屽畠鏈熸湜鍑芥暟鎬绘槸鍋氫竴浜涗簨鎯呫€傚洜姝わ紝濡傛灉鍑芥暟鐨勯敊璇敞鍏ヨ烦杩囦簡鏁翠釜鍑芥暟锛岃繖绉嶆湡鏈涘氨琚繚鑳屼簡锛屽苟瀵艰嚧鎰忓閿欒銆?

##### 鍙敞鍏ラ敊璇嚱鏁扮殑绫诲瀷


姣忎釜鍙敞鍏ラ敊璇殑鍑芥暟閮戒細鐢?ALLOW_ERROR_INJECTION() 瀹忔寚瀹氶敊璇被鍨嬨€傚鏋滀綘娣诲姞涓€涓柊鐨勫彲娉ㄥ叆閿欒鍑芥暟锛屽繀椤讳粩缁嗛€夋嫨瀹冦€傚鏋滈€夋嫨浜嗛敊璇殑閿欒绫诲瀷锛屽唴鏍稿彲鑳戒細宕╂簝锛屽洜涓哄畠鍙兘鏃犳硶澶勭悊璇ラ敊璇€傚湪 include/asm-generic/error-injection.h 涓畾涔変簡 4 绉嶉敊璇被鍨?
EI_ETYPE_NULL
  姝ゅ嚱鏁板湪澶辫触鏃朵細杩斿洖 `NULL`銆備緥濡傝繑鍥炲凡鍒嗛厤瀵硅薄鐨勫湴鍧€銆?
EI_ETYPE_ERRNO
  姝ゅ嚱鏁板湪澶辫触鏃朵細杩斿洖 `-errno` 閿欒鐮併€備緥濡傚綋杈撳叆閿欒鏃惰繑鍥?-EINVAL銆傝繖灏嗗寘鎷偅浜涢€氳繃 ERR_PTR() 瀹忚繑鍥炵紪鐮佷簡 `-errno` 鐨勫湴鍧€鐨勫嚱鏁般€?
EI_ETYPE_ERRNO_NULL
  姝ゅ嚱鏁板湪澶辫触鏃朵細杩斿洖 `-errno` 鎴?`NULL`銆傚鏋滄鍑芥暟鐨勮皟鐢ㄨ€呬娇鐢?IS_ERR_OR_NULL() 瀹忔鏌ヨ繑鍥炲€硷紝鍒欐绫诲瀷鏄悎閫傜殑銆?
EI_ETYPE_TRUE
  姝ゅ嚱鏁板湪澶辫触鏃朵細杩斿洖 `true`锛堥潪闆剁殑姝ｅ€硷級銆?
濡傛灉浣犳寚瀹氫簡閿欒鐨勭被鍨嬶紝渚嬪涓鸿繑鍥炲凡鍒嗛厤瀵硅薄鐨勫嚱鏁版寚瀹?EI_TYPE_ERRNO锛屽彲鑳戒細瀵艰嚧闂锛屽洜涓鸿繑鍥炲€间笉鏄璞″湴鍧€锛岃皟鐢ㄨ€呮棤娉曡闂鍦板潃銆?

### 濡備綍娣诲姞鏂扮殑鏁呴殰娉ㄥ叆鑳藉姏


- #include <linux/fault-inject.h>

- 瀹氫箟鏁呴殰灞炴€?
  DECLARE_FAULT_ATTR(name);

  鏈夊叧 struct fault_attr 鐨勫畾涔夛紝璇峰弬闃?fault-inject.h 涓殑缁嗚妭銆?
- 鎻愪緵閰嶇疆鏁呴殰灞炴€х殑鏂规硶

- 寮曞閫夐」

  濡傛灉浣犻渶瑕佷粠鍚姩鏃跺氨鍚敤鏁呴殰娉ㄥ叆鑳藉姏锛屽彲浠ユ彁渚涘紩瀵奸€夐」鏉ラ厤缃畠銆備负姝ゆ湁涓€涓緟鍔╁嚱鏁帮細

	setup_fault_attr(attr, str);

- debugfs 鏉＄洰

  failslab銆乫ail_page_alloc銆乫ail_usercopy 鍜?fail_make_request 浣跨敤杩欑鏂瑰紡銆傝緟鍔╁嚱鏁帮細

	fault_create_debugfs_attr(name, parent, attr);

- 妯″潡鍙傛暟

  濡傛灉鏁呴殰娉ㄥ叆鑳藉姏鐨勮寖鍥翠粎闄愪簬鍗曚釜鍐呮牳妯″潡锛屾渶濂芥彁渚涙ā鍧楀弬鏁版潵閰嶇疆鏁呴殰灞炴€с€?
- 娣诲姞鎻掑叆澶辫触鐨勯挬瀛?
  褰?should_fail() 杩斿洖 true 鏃讹紝瀹㈡埛绔唬鐮佸簲娉ㄥ叆涓€涓け璐ワ細

	should_fail(attr, size);


### 搴旂敤绀轰緥


```
    #!/bin/bash

    FAILTYPE=failslab
    echo Y > /sys/kernel/debug/$FAILTYPE/task-filter
    echo 10 > /sys/kernel/debug/$FAILTYPE/probability
    echo 100 > /sys/kernel/debug/$FAILTYPE/interval
    echo -1 > /sys/kernel/debug/$FAILTYPE/times
    echo 0 > /sys/kernel/debug/$FAILTYPE/space
    echo 2 > /sys/kernel/debug/$FAILTYPE/verbose
    echo Y > /sys/kernel/debug/$FAILTYPE/ignore-gfp-wait

    faulty_system()
    {
	bash -c "echo 1 > /proc/self/make-it-fail && exec $*"
    }

    if [ $# -eq 0 ]
    then
	echo "Usage: $0 modulename [ modulename ... ]"
	exit 1
    fi

    for m in $*
    do
	echo inserting $m...
	faulty_system modprobe $m

	echo removing $m...
	faulty_system modprobe -r $m
    done
```

------------------------------------------------------------------------------

```
    #!/bin/bash

    FAILTYPE=fail_page_alloc
    module=$1

    if [ -z $module ]
    then
	echo "Usage: $0 <modulename>"
	exit 1
    fi

    modprobe $module

    if [ ! -d /sys/module/$module/sections ]
    then
	echo Module $module is not loaded
	exit 1
    fi

    cat /sys/module/$module/sections/.text > /sys/kernel/debug/$FAILTYPE/require-start
    cat /sys/module/$module/sections/.data > /sys/kernel/debug/$FAILTYPE/require-end

    echo N > /sys/kernel/debug/$FAILTYPE/task-filter
    echo 10 > /sys/kernel/debug/$FAILTYPE/probability
    echo 100 > /sys/kernel/debug/$FAILTYPE/interval
    echo -1 > /sys/kernel/debug/$FAILTYPE/times
    echo 0 > /sys/kernel/debug/$FAILTYPE/space
    echo 2 > /sys/kernel/debug/$FAILTYPE/verbose
    echo Y > /sys/kernel/debug/$FAILTYPE/ignore-gfp-wait
    echo Y > /sys/kernel/debug/$FAILTYPE/ignore-gfp-highmem
    echo 10 > /sys/kernel/debug/$FAILTYPE/stacktrace-depth

    trap "echo 0 > /sys/kernel/debug/$FAILTYPE/probability" SIGINT SIGTERM EXIT

    echo "Injecting errors into the module $module... (interrupt to stop)"
    sleep 1000000
```

------------------------------------------------------------------------------

```
    #!/bin/bash

    rm -f testfile.img
    dd if=/dev/zero of=testfile.img bs=1M seek=1000 count=1
    DEVICE=$(losetup --show -f testfile.img)
    mkfs.btrfs -f $DEVICE
    mkdir -p tmpmnt

    FAILTYPE=fail_function
    FAILFUNC=open_ctree
    echo $FAILFUNC > /sys/kernel/debug/$FAILTYPE/inject
    printf %#x -12 > /sys/kernel/debug/$FAILTYPE/$FAILFUNC/retval
    echo N > /sys/kernel/debug/$FAILTYPE/task-filter
    echo 100 > /sys/kernel/debug/$FAILTYPE/probability
    echo 0 > /sys/kernel/debug/$FAILTYPE/interval
    echo -1 > /sys/kernel/debug/$FAILTYPE/times
    echo 0 > /sys/kernel/debug/$FAILTYPE/space
    echo 1 > /sys/kernel/debug/$FAILTYPE/verbose

    mount -t btrfs $DEVICE tmpmnt
    if [ $? -ne 0 ]
    then
	echo "SUCCESS!"
    else
	echo "FAILED!"
	umount tmpmnt
    fi

    echo > /sys/kernel/debug/$FAILTYPE/inject

    rmdir tmpmnt
    losetup -d $DEVICE
    rm testfile.img
```

------------------------------------------------------------------------------

```
    # 灏?skbuff_head_cache 鏍囪涓烘晠闅?    echo 1 > /sys/kernel/slab/skbuff_head_cache/failslab
    # 寮€鍚紦瀛樿繃婊わ紙榛樿鍏抽棴锛?    echo 1 > /sys/kernel/debug/failslab/cache-filter
    # 寮€鍚晠闅滄敞鍏?    echo 1 > /sys/kernel/debug/failslab/times
    echo 1 > /sys/kernel/debug/failslab/probability
```

### 鐢ㄤ簬杩愯甯?failslab 鎴?fail_page_alloc 鍛戒护鐨勫伐鍏?
涓轰簡浣夸笂杩颁换鍔℃洿瀹规槗瀹屾垚锛屾垜浠彲浠ヤ娇鐢?tools/testing/fault-injection/failcmd.sh銆傝杩愯鍛戒护 "./tools/testing/fault-injection/failcmd.sh --help" 鑾峰彇鏇村淇℃伅骞跺弬瑙佷互涓嬬ず渚嬨€?
绀轰緥锛?
杩愯鍛戒护 "make -C tools/testing/selftests/ run_tests" 骞舵敞鍏?slab

```
	# ./tools/testing/fault-injection/failcmd.sh \
		-- make -C tools/testing/selftests/ run_tests
```

涓庝笂杩扮浉鍚岋紝浣嗘寚瀹氭渶澶?100 娆″け璐ヨ€屼笉鏄竴娆?
```
	# ./tools/testing/fault-injection/failcmd.sh --times=100 \
		-- make -C tools/testing/selftests/ run_tests
```

涓庝笂杩扮浉鍚岋紝浣嗘敞鍏ラ〉鍒嗛厤澶辫触鑰屼笉鏄?slab

```
	# env FAILCMD_TYPE=fail_page_alloc \
		./tools/testing/fault-injection/failcmd.sh --times=100 \
		-- make -C tools/testing/selftests/ run_tests
```

### 浣跨敤 fail-nth 杩涜绯荤粺鎬ф晠闅?
浠ヤ笅浠ｇ爜绯荤粺鎬у湴瀵圭 0銆?銆?鈥︹€︽鏁呴殰杩涜娉ㄥ叆

```
  #include <sys/types.h>
  #include <sys/stat.h>
  #include <sys/socket.h>
  #include <sys/syscall.h>
  #include <fcntl.h>
  #include <unistd.h>
  #include <string.h>
  #include <stdlib.h>
  #include <stdio.h>
  #include <errno.h>

  int main()
  {
	int i, err, res, fail_nth, fds[2];
	char buf[128];

	system("echo N > /sys/kernel/debug/failslab/ignore-gfp-wait");
	sprintf(buf, "/proc/self/task/%ld/fail-nth", syscall(SYS_gettid));
	fail_nth = open(buf, O_RDWR);
	for (i = 1;; i++) {
		sprintf(buf, "%d", i);
		write(fail_nth, buf, strlen(buf));
		res = socketpair(AF_LOCAL, SOCK_STREAM, 0, fds);
		err = errno;
		pread(fail_nth, buf, sizeof(buf), 0);
		if (res == 0) {
			close(fds[0]);
			close(fds[1]);
		}
		printf("%d-th fault %c: res=%d/%d\n", i, atoi(buf) ? 'N' : 'Y',
			res, err);
		if (atoi(buf))
			break;
	}
	return 0;
  }
```

```
	1-th fault Y: res=-1/23
	2-th fault Y: res=-1/23
	3-th fault Y: res=-1/12
	4-th fault Y: res=-1/12
	5-th fault Y: res=-1/23
	6-th fault Y: res=-1/23
	7-th fault Y: res=-1/23
	8-th fault Y: res=-1/12
	9-th fault Y: res=-1/12
	10-th fault Y: res=-1/12
	11-th fault Y: res=-1/12
	12-th fault Y: res=-1/12
	13-th fault Y: res=-1/12
	14-th fault Y: res=-1/12
	15-th fault Y: res=-1/12
	16-th fault N: res=0/12
```
