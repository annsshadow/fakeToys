## 缁熶竴鍙墿灞曞浐浠舵帴鍙ｏ紙UEFI锛?

UEFI锛屽嵆缁熶竴鍙墿灞曞浐浠舵帴鍙ｏ紙Unified Extensible Firmware Interface锛夛紝鏄竴浠借鑼冿紝瑙勫畾浜嗗吋瀹瑰浐浠舵帴鍙ｇ殑琛屼负銆傚畠鐢?UEFI 璁哄潧缁存姢 - http://www.uefi.org/銆?
UEFI 鏄叾鍓嶈韩 'EFI' 鐨勬紨杩涳紝鍥犳鍦ㄦ湰鏂囨。鍙婄浉鍏崇殑婧愪唬鐮佷腑锛孍FI 鍜?UEFI 杩欎袱涓湳璇湪涓€瀹氱▼搴︿笂鍙互浜掓崲浣跨敤銆傞€氬父锛屼换浣曟柊鍐呭閮戒娇鐢?'UEFI'锛岃€?'EFI' 鎸囦唬閬楃暀锛坙egacy锛変唬鐮佹垨瑙勮寖銆?
## Linux 涓殑 UEFI 鏀寔


鍦ㄥ甫鏈夌鍚?UEFI 瑙勮寖鐨勫浐浠剁殑骞冲彴涓婂惎鍔紝浣垮唴鏍歌兘澶熸敮鎸侀澶栫殑鐗规€э細

- UEFI 杩愯鏃舵湇鍔★紙Runtime Services锛?- 閫氳繃 UEFI 閰嶇疆琛ㄧ殑鏍囧噯鍖栨帴鍙ｆ绱㈠悇绉嶉厤缃俊鎭€傦紙ACPI銆丼MBIOS 绛夛級

瑕佸疄闄呭惎鐢?[U]EFI 鏀寔锛岃鍚敤锛?
- CONFIG_EFI=y
- CONFIG_EFIVAR_FS=y 鎴?m

璇ュ疄鐜颁緷璧栦簬鍦ㄦ墎骞宠澶囨爲锛團lattened Device Tree锛孎DT锛変腑鎺ユ敹鍏充簬 UEFI 鐜鐨勪俊鎭€斺€斿洜姝や粎鍦?CONFIG_OF 涓嬪彲鐢ㄣ€?
## UEFI stub


"stub" 鏄竴椤瑰姛鑳斤紝瀹冨皢 Image/zImage 鎵╁睍涓轰竴涓湁鏁堢殑 UEFI PE/COFF 鍙墽琛屾枃浠讹紝鍖呭惈涓€涓姞杞藉櫒搴旂敤绋嬪簭锛屼娇寰楀彲浠ョ洿鎺ヤ粠 UEFI shell銆佸惎鍔ㄨ彍鍗曪紝鎴栧儚 Gummiboot 鎴?rEFInd 杩欐牱鐨勮交閲忕骇寮曞鍔犺浇绋嬪簭鍔犺浇鍐呮牳銆?
甯︽湁 stub 鏀寔鏋勫缓鐨勫唴鏍搁暅鍍忎粛鐒舵槸涓€涓湁鏁堢殑鍐呮牳闀滃儚锛屽彲鐢ㄤ簬鍦ㄩ潪 UEFI 鐜涓惎鍔ㄣ€?
## ARM 涓婄殑 UEFI 鍐呮牳鏀寔


ARM 鏋舵瀯锛坅rm 鍜?arm64锛変笂鐨?UEFI 鍐呮牳鏀寔浠呭湪閫氳繃 stub 鍚姩鏃跺彲鐢ㄣ€?
鍦?UEFI 妯″紡涓嬪惎鍔ㄦ椂锛宻tub 浼氫粠鎻愪緵鐨?DT 涓垹闄や换浣曞唴瀛樿妭鐐广€傜浉鍙嶏紝鍐呮牳璇诲彇 UEFI 鍐呭瓨鏄犲皠锛坢emory map锛夈€?
stub 浼氱敤浠ヤ笅鍙傛暟濉厖 FDT 鐨?/chosen 鑺傜偣锛堝唴鏍镐篃浼氭壂鎻忚繖浜涘弬鏁帮級锛?
==========================  ======   ===========================================
鍚嶇О                       绫诲瀷     鎻忚堪
==========================  ======   ===========================================
linux,uefi-system-table     64-bit   UEFI 绯荤粺琛紙System Table锛夌殑鐗╃悊鍦板潃銆?
linux,uefi-mmap-start       64-bit   UEFI 鍐呭瓨鏄犲皠鐨勭墿鐞嗗湴鍧€锛?                                     鐢?UEFI GetMemoryMap() 璋冪敤濉厖銆?
linux,uefi-mmap-size        32-bit   涓婁竴椤规墍鎸囩殑 UEFI 鍐呭瓨鏄犲皠鐨勫ぇ灏忥紙瀛楄妭锛夈€?
linux,uefi-mmap-desc-size   32-bit   UEFI 鍐呭瓨鏄犲皠涓瘡涓潯鐩殑澶у皬锛堝瓧鑺傦級銆?
linux,uefi-mmap-desc-ver    32-bit   mmap 鎻忚堪绗︽牸寮忕殑鐗堟湰銆?
kaslr-seed                  64-bit   鐢ㄤ簬闅忔満鍖栧唴鏍搁暅鍍忓熀鍧€浣嶇疆鐨勭喌銆?
bootargs                    String   鍐呮牳鍛戒护琛?==========================  ======   ===========================================
