## FPGA Region


### 姒傝堪


鏈枃妗ｆ棬鍦ㄧ畝瑕佹杩?FPGA region API 鐨勭敤娉曘€傚叧浜?region 鏇村叿姒傚康鎬х殑浠嬬粛鍙互鍦?Device Tree
缁戝畾鏂囨。 [#f1]_ 涓壘鍒般€?
灏辨湰 API 鏂囨。鑰岃█锛屾垜浠笉濡ㄨ涓€涓?region 灏?FPGA Manager 涓庝竴搴э紙鎴栧搴э級妗ユ帴鍏宠仈鍒?FPGA 鐨?涓€涓彲閲嶇紪绋嬪尯鍩熸垨鏁翠釜 FPGA銆傝 API 鎻愪緵浜嗘敞鍐?region 浠ュ強瀵?region 杩涜缂栫▼鐨勬柟娉曘€?
鐩墠鍦?fpga-region.c 涔嬩笂銆佸唴鏍镐腑鍞竴鐨勫眰鏄?[#f1]_ 涓弿杩扮殑 Device Tree 鏀寔
锛坥f-fpga-region.c锛夈€侱T 鏀寔灞備娇鐢?region 瀵?FPGA 杩涜缂栫▼锛岀劧鍚庝娇鐢?DT 澶勭悊鏋氫妇銆傞€氱敤鐨?region 浠ｇ爜鏃ㄥ湪琚叾浠栧湪缂栫▼鍚庢湁鍏跺畠鏋氫妇鏂瑰紡鐨勬柟妗堟墍浣跨敤銆?
涓€涓?fpga-region 鍙互閰嶇疆涓轰簡瑙ｄ互涓嬪唴瀹癸細

 - 浣跨敤鍝釜 FPGA manager 杩涜缂栫▼

 - 鍦ㄧ紪绋嬪墠绂佺敤銆佺紪绋嬪悗鍚敤鐨勬ˉ鎺?
缂栫▼ FPGA 闀滃儚鎵€闇€鐨勯澶栦俊鎭€氳繃 struct fpga_image_info 浼犲叆锛屽寘鎷細

 - 鎸囧悜闀滃儚鐨勬寚閽堬紝鍙互鏄垎鏁?鑱氶泦缂撳啿鍖恒€佽繛缁紦鍐插尯锛屾垨鍥轰欢鏂囦欢鍚?
 - 鎸囩ず鍏蜂綋鐗规€х殑鏍囧織锛屼緥濡傞暅鍍忔槸鍚︾敤浜庨儴鍒嗛噸閰嶇疆

### 濡備綍娣诲姞涓€涓柊鐨?FPGA region


浣跨敤绀轰緥鍙 [#f2]_ 鐨?probe 鍑芥暟銆?
### 娣诲姞鏂扮殑 FPGA region 鐨?API


- struct fpga_region - FPGA region 缁撴瀯浣?- struct fpga_region_info - __fpga_region_register_full() 鐨勫弬鏁扮粨鏋勪綋
- __fpga_region_register_full() - 浣跨敤 fpga_region_info 缁撴瀯浣撳垱寤哄苟娉ㄥ唽涓€涓?FPGA region锛?  浠ユ彁渚涙渶澶у寲鐨勯€夐」鐏垫椿鎬?- __fpga_region_register() - 浣跨敤鏍囧噯鍙傛暟鍒涘缓骞舵敞鍐屼竴涓?FPGA region
- fpga_region_unregister() - 娉ㄩ攢涓€涓?FPGA region

杈呭姪瀹?`fpga_region_register()` 涓?`fpga_region_register_full()` 浼氳嚜鍔ㄥ皢娉ㄥ唽璇?FPGA region
鐨勬ā鍧楄涓烘嫢鏈夎€呫€?
FPGA region 鐨?probe 鍑芥暟闇€瑕佽幏鍙栧瀹冨皢鐢ㄤ簬缂栫▼鐨?FPGA Manager 鐨勫紩鐢ㄣ€傝繖閫氬父浼氬湪 region 鐨?probe 鍑芥暟鏈熼棿鍙戠敓銆?
- fpga_mgr_get() - 鑾峰彇瀵?FPGA manager 鐨勫紩鐢紝澧炲姞寮曠敤璁℃暟
- of_fpga_mgr_get() - 鑾峰彇瀵?FPGA manager 鐨勫紩鐢紝澧炲姞寮曠敤璁℃暟锛岀粰瀹氫竴涓澶囪妭鐐?- fpga_mgr_put() - 閲婃斁涓€涓?FPGA manager

FPGA region 闇€瑕佹寚瀹氬湪缂栫▼ FPGA 鏃惰鎺у埗鍝簺妗ユ帴銆俽egion 椹卞姩鍙互鍦?probe 鏈熼棿鏋勫缓涓€涓ˉ鎺ュ垪琛?锛?c`fpga_region->bridge_list`锛夛紝涔熷彲浠ユ湁涓€涓嚱鏁扮敤浜庡湪缂栫▼鍓嶇珛鍗冲垱寤鸿缂栫▼鐨勬ˉ鎺ュ垪琛?锛?c`fpga_region->get_bridges`锛夈€侳PGA bridge 妗嗘灦鎻愪緵浠ヤ笅 API 鏉ュ鐞嗘瀯寤烘垨鎷嗛櫎璇ュ垪琛ㄣ€?
- fpga_bridge_get_to_list() - 鑾峰彇瀵?FPGA bridge 鐨勫紩鐢紝灏嗗叾鍔犲叆鍒楄〃
- of_fpga_bridge_get_to_list() - 鑾峰彇瀵?FPGA bridge 鐨勫紩鐢紝灏嗗叾鍔犲叆鍒楄〃锛岀粰瀹氫竴涓澶囪妭鐐?- fpga_bridges_put() - 缁欏畾涓€涓ˉ鎺ュ垪琛紝閲婃斁瀹冧滑

   :functions: fpga_region

   :functions: fpga_region_info

   :functions: __fpga_region_register_full

   :functions: __fpga_region_register

   :functions: fpga_region_unregister

   :functions: fpga_mgr_get

   :functions: of_fpga_mgr_get

   :functions: fpga_mgr_put

   :functions: fpga_bridge_get_to_list

   :functions: of_fpga_bridge_get_to_list

   :functions: fpga_bridges_put
