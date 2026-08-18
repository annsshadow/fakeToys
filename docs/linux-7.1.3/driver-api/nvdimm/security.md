## NVDIMM 瀹夊叏鎬?

### 1. 寮曡█


闅忕潃 Intel Device Specific Methods锛圖SM锛岃澶囩壒瀹氭柟娉曪級v1.8 瑙勮寖 [^1^] 鐨勫紩鍏ワ紝
瀹夊叏鐩稿叧鐨?DSM 琚姞鍏ャ€傝瑙勮寖鏂板浜嗕互涓嬪畨鍏?DSM锛?get security state"锛堣幏鍙?瀹夊叏鐘舵€侊級銆?set passphrase"锛堣缃彛浠わ級銆?disable passphrase"锛堢鐢ㄥ彛浠わ級銆?"unlock unit"锛堣В閿佸崟鍏冿級銆?freeze lock"锛堝喕缁撻攣锛夈€?secure erase"锛堝畨鍏ㄦ摝闄わ級
浠ュ強 "overwrite"锛堣鍐欙級銆備负浜嗘敮鎸佽繖浜涘畨鍏ㄦ搷浣滐紝鍦?struct dimm 涓柊澧炰簡涓€涓?security_ops 鏁版嵁缁撴瀯锛屽苟鏆撮湶浜嗛€氱敤 API 浠ユ敮鎸佷笌鍘傚晢鏃犲叧鐨勬搷浣滅敤娉曘€?
### 2. Sysfs 鎺ュ彛


nvdimm 鐨?sysfs 鐩綍涓彁渚涗簡 "security" 杩欎竴 sysfs 灞炴€с€備緥濡傦細
/sys/devices/LNXSYSTM:00/LNXSYBUS:00/ACPI0012:00/ndbus0/nmem0/security

璇ュ睘鎬х殑 "show" 灞炴€т細鏄剧ず璇?DIMM 鐨勫畨鍏ㄧ姸鎬併€傚彲鐢ㄧ殑鐘舵€佹湁锛歞isabled锛堝凡绂佺敤锛夈€?unlocked锛堝凡瑙ｉ攣锛夈€乴ocked锛堝凡閿佸畾锛夈€乫rozen锛堝凡鍐荤粨锛夊拰 overwrite锛堣鍐欎腑锛夈€?濡傛灉涓嶆敮鎸佸畨鍏ㄧ壒鎬э紝璇?sysfs 灞炴€у皢涓嶅彲瑙併€?
瀵硅灞炴€ф墽琛屽啓鎿嶄綔鏃讹紝"store" 灞炴€т細鎺ュ彈鑻ュ共鍛戒护浠ユ敮鎸侀儴鍒嗗畨鍏ㄥ姛鑳斤細
update <old_keyid> <new_keyid> - 鍚敤鎴栨洿鏂板彛浠ゃ€?disable <keyid> - 绂佺敤宸插惎鐢ㄧ殑瀹夊叏鐗规€у苟绉婚櫎瀵嗛挜銆?freeze - 鍐荤粨瀹夊叏鐘舵€佺殑鍙樻洿銆?erase <keyid> - 鍒犻櫎鐜版湁鐢ㄦ埛鍔犲瘑瀵嗛挜銆?overwrite <keyid> - 鎿﹂櫎鏁翠釜 nvdimm銆?master_update <keyid> <new_keyid> - 鍚敤鎴栨洿鏂颁富鍙ｄ护銆?master_erase <keyid> - 鍒犻櫎鐜版湁鐢ㄦ埛鍔犲瘑瀵嗛挜銆?
### 3. 瀵嗛挜绠＄悊


瀵嗛挜閫氳繃 DIMM id 涓庤礋杞界浉鍏宠仈銆備緥濡傦細
# cat /sys/devices/LNXSYSTM:00/LNXSYBUS:00/ACPI0012:00/ndbus0/nmem0/nfit/id
8089-a2-1740-00000133
璇?DIMM id 浼氫笌瀵嗛挜璐熻浇锛堝彛浠わ級涓€璧锋彁渚涚粰鍐呮牳銆?
瀹夊叏瀵嗛挜浠?姣忎釜 DIMM 涓€鎶婂瘑閽?鐨勬柟寮忕鐞嗐€傚瘑閽?鍙ｄ护"棰勬湡涓?32 瀛楄妭闀裤€傝繖绫讳技浜?ATA 瀹夊叏瑙勮寖 [^2^]銆傚湪 nvdimm 瑙ｉ攣鏈熼棿锛屽瘑閽ユ渶鍒濋€氳繃 request_key() 鍐呮牳 API
璋冪敤鑾峰彇銆傜敤鎴锋湁璐ｄ换纭繚鎵€鏈夊瘑閽ラ兘宸茬疆浜庡唴鏍哥敤鎴峰瘑閽ョ幆锛坲ser keyring锛変腑浠ヤ究
瑙ｉ攣銆?
鏍煎紡涓?enc32 鐨?nvdimm 鍔犲瘑瀵嗛挜锛坋ncrypted-key锛夌殑鎻忚堪鏍煎紡涓猴細
nvdimm:<bus-provider-specific-unique-id>

鍒涘缓 enc32 鏍煎紡鐨?encrypted-keys 璇峰弬瑙佹枃浠?`Documentation/security/keys/trusted-encrypted.rst`銆備娇鐢ㄤ富鍙俊瀵嗛挜锛坢aster
trusted key锛夐厤鍚?TPM 鏉ュ皝瑁咃紙sealing锛塭ncrypted-keys 鏄帹鑽愬仛娉曘€?
### 4. 瑙ｉ攣


褰撳唴鏍告灇涓?DIMM 鏃讹紝鍐呮牳浼氬皾璇曚粠鍐呮牳鐢ㄦ埛瀵嗛挜鐜腑妫€绱㈠瘑閽ャ€傝繖鏄В閿佷竴涓凡閿佸畾
DIMM 鐨勫敮涓€鏃舵満銆備竴鏃﹁В閿侊紝璇?DIMM 灏嗕繚鎸佽В閿佺姸鎬佺洿鍒伴噸鍚€傞€氬父鏌愪釜瀹炰綋锛堜緥濡?shell 鑴氭湰锛変細鍦?initramfs 闃舵灏嗘墍鏈夌浉鍏崇殑 encrypted-keys 娉ㄥ叆鍐呮牳鐢ㄦ埛瀵嗛挜鐜€?杩欎负瑙ｉ攣鍔熻兘鎻愪緵浜嗚闂墍鏈夌浉鍏冲瘑閽ワ紙鍏朵腑鍖呭惈瀵瑰簲 nvdimm 鐨勫彛浠わ級鐨勯€斿緞銆傚悓鏃?寤鸿鍦?libnvdimm 琚?modprobe 鍔犺浇涔嬪墠娉ㄥ叆瀵嗛挜銆?
### 5. 鏇存柊


杩涜鏇存柊鏃讹紝棰勬湡鐜版湁鐨勫瘑閽ヤ細浠庡唴鏍哥敤鎴峰瘑閽ョ幆涓Щ闄わ紝骞朵互涓嶅悓鐨勶紙鏃э級瀵嗛挜閲嶆柊
娉ㄥ叆銆傛棫瀵嗛挜鐨勬弿杩版槸浠€涔堟棤鍏崇揣瑕侊紝鍥犱负鏇存柊鎿嶄綔鎴戜滑鍙叧蹇?keyid銆傚悓鏃堕鏈熸柊瀵嗛挜
浠ユ湰鏂囨。鍓嶉潰鎻忚堪鐨勬牸寮忔敞鍏ュ叾鎻忚堪銆傚啓鍏?sysfs 灞炴€х殑鏇存柊鍛戒护鏍煎紡涓猴細
update <old keyid> <new keyid>

濡傛灉鐢变簬鍚敤瀹夊叏鐗规€ц€屼笉瀛樺湪鏃?keyid锛屽垯搴斾紶鍏?0銆?
### 6. 鍐荤粨锛團reeze锛?

freeze 鎿嶄綔涓嶉渶瑕佷换浣曞瘑閽ャ€傚畨鍏ㄩ厤缃彲鐢卞叿鏈?root 鏉冮檺鐨勭敤鎴峰喕缁撱€?
### 7. 绂佺敤锛圖isable锛?

瀹夊叏绂佺敤鐨勫懡浠ゆ牸寮忎负锛?disable <keyid>

涓€涓粦瀹氬埌璇?nvdimm銆佸甫鏈夊綋鍓嶅彛浠よ礋杞界殑瀵嗛挜搴斿綋瀛樺湪浜庡唴鏍哥敤鎴峰瘑閽ョ幆涓€?
### 8. 瀹夊叏鎿﹂櫎锛圫ecure Erase锛?

鎵ц瀹夊叏鎿﹂櫎鐨勫懡浠ゆ牸寮忎负锛?erase <keyid>

涓€涓粦瀹氬埌璇?nvdimm銆佸甫鏈夊綋鍓嶅彛浠よ礋杞界殑瀵嗛挜搴斿綋瀛樺湪浜庡唴鏍哥敤鎴峰瘑閽ョ幆涓€?
### 9. 瑕嗗啓锛圤verwrite锛?

鎵ц瑕嗗啓鐨勫懡浠ゆ牸寮忎负锛?overwrite <keyid>

濡傛灉鏈惎鐢ㄥ畨鍏ㄧ壒鎬э紝瑕嗗啓鍙互鍦ㄦ病鏈夊瘑閽ョ殑鎯呭喌涓嬭繘琛屻€傚彲浼犲叆瀵嗛挜搴忓垪鍙?0 鏉ヨ〃绀?鏃犲瘑閽ャ€?
鍙互杞 sysfs 灞炴€?"security" 浠ョ瓑寰呰鍐欏畬鎴愩€傛牴鎹?nvdimm 澶у皬涓嶅悓锛岃鍐欏彲鑳?鎸佺画鏁板崄鍒嗛挓鎴栨洿涔呫€?
涓€涓粦瀹氬埌璇?nvdimm銆佸甫鏈夊綋鍓嶇敤鎴峰彛浠ょ殑 encrypted-key 搴斿綋琚敞鍏ワ紝骞堕€氳繃 sysfs
浼犲叆鍏?keyid銆?
### 10. 涓绘洿鏂帮紙Master Update锛?

鎵ц涓绘洿鏂扮殑鍛戒护鏍煎紡涓猴細
update <old keyid> <new keyid>

涓绘洿鏂扮殑杩愯鏈哄埗涓?update 鐩稿悓锛屽彧鏄紶鍏ュ唴鏍哥殑鏄富鍙ｄ护瀵嗛挜銆備富鍙ｄ护瀵嗛挜鍙槸
鍙︿竴涓?encrypted-key銆?
璇ュ懡浠や粎鍦ㄥ畨鍏ㄧ壒鎬ц绂佺敤鏃跺彲鐢ㄣ€?
### 11. 涓绘摝闄わ紙Master Erase锛?

鎵ц涓绘摝闄ょ殑鍛戒护鏍煎紡涓猴細
master_erase <current keyid>

璇ュ懡浠ょ殑杩愯鏈哄埗涓?erase 鐩稿悓锛屽彧鏄紶鍏ュ唴鏍哥殑鏄富鍙ｄ护瀵嗛挜銆備富鍙ｄ护瀵嗛挜鍙槸鍙︿竴涓?encrypted-key銆?
璇ュ懡浠や粎鍦ㄤ富瀹夊叏鐗规€у凡鍚敤鏃跺彲鐢紝杩欑敱鎵╁睍瀹夊叏鐘舵€佹寚绀恒€?
[^1^]: https://pmem.io/documents/NVDIMM_DSM_Interface-V1.8.pdf

[^2^]: http://www.t13.org/documents/UploadedDocuments/docs2006/e05179r4-ACS-SecurityClarifications.pdf
