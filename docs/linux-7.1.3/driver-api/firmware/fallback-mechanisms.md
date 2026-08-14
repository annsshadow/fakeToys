## 鍥為€€鏈哄埗


鍐呮牳鏀寔涓€绉嶅洖閫€鏈哄埗锛岀敤浜庡厠鏈嶅湪鏍规枃浠剁郴缁熶笂鐩存帴杩涜鏂囦欢绯荤粺鏌ユ壘澶辫触鐨勬儏鍐碉紝
鎴栧湪瀹為檯鍘熷洜瀵艰嚧鍥轰欢鏍规湰鏃犳硶瀹夎鍒版牴鏂囦欢绯荤粺涓婄殑鎯呭喌銆備笌鍥轰欢鍥為€€鏈哄埗鏀寔
鐩稿叧鐨勫唴鏍搁厤缃€夐」鏈夛細

  - CONFIG_FW_LOADER_USER_HELPER锛氬惎鐢ㄦ瀯寤哄浐浠跺洖閫€鏈哄埗銆傚浠婂ぇ澶氭暟鍙戣鐗?    閮藉惎鐢ㄤ簡姝ら€夐」銆傚鏋滃惎鐢ㄤ簡瀹冧絾绂佺敤浜?    CONFIG_FW_LOADER_USER_HELPER_FALLBACK锛屽垯浠呮湁鑷畾涔夊洖閫€鏈哄埗鍙敤锛?    涓斾粎閽堝 request_firmware_nowait() 璋冪敤銆?  - CONFIG_FW_LOADER_USER_HELPER_FALLBACK锛氬己鍒朵负姣忎釜璇锋眰鍦ㄦ墍鏈夊浐浠?API 璋冪敤
    涓婂惎鐢?kobject uevent 鍥為€€鏈哄埗锛宺equest_firmware_direct() 闄ゅ銆傚浠?    澶у鏁板彂琛岀増绂佺敤浜嗘閫夐」銆傝皟鐢?request_firmware_nowait() 鍏佽涓€绉嶆浛浠ｇ殑
    鍥為€€鏈哄埗锛氬鏋滃惎鐢ㄤ簡姝?kconfig 閫夐」锛屽苟涓旀偍浼犵粰 request_firmware_nowait()
    鐨勭浜屼釜鍙傛暟 uevent 璁句负 false锛屽垯鎮ㄦ槸鍦ㄥ憡鐭ュ唴鏍告偍鎷ユ湁鑷畾涔夌殑鍥為€€鏈哄埗锛?    灏嗙敱鎮ㄦ墜鍔ㄥ姞杞藉浐浠躲€傝瑙佷笅鏂囥€?
娉ㄦ剰锛岃繖鎰忓懗鐫€褰撻厤缃负锛?
CONFIG_FW_LOADER_USER_HELPER=y
CONFIG_FW_LOADER_USER_HELPER_FALLBACK=n

鏃讹紝鍗充娇瀵逛簬 uevent 璁句负 true 鐨?request_firmware_nowait()锛宬object uevent
鍥為€€鏈哄埗涔熸案杩滀笉浼氱敓鏁堛€?
## 鍥轰欢鍥為€€鏈哄埗鐨勫悎鐞嗘€?

鐩存帴鏂囦欢绯荤粺鏌ユ壘鍙兘鍥犲绉嶅師鍥犲け璐ャ€傚€煎緱鍒椾妇骞惰褰曡繖浜涘凡鐭ュ師鍥狅紝鍥犱负瀹冧滑
璇佹槑浜嗗洖閫€鏈哄埗鐨勫繀瑕佹€э細

- 涓庡惎鍔ㄨ繃绋嬩腑璁块棶鏍规枃浠剁郴缁熷彂鐢熺珵浜夈€?
- 浠庢寕璧锋仮澶嶆椂鍙戠敓绔炰簤銆傝繖鐢卞浐浠剁紦瀛樿В鍐筹紝浣嗗浐浠剁紦瀛樹粎鍦ㄦ偍浣跨敤 uevent 鏃?  鍙楁敮鎸侊紝涓?request_firmware_into_buf() 涓嶆敮鎸佸畠銆?
- 鍥轰欢鏃犳硶閫氳繃甯歌鎵嬫璁块棶锛?
        - 鏃犳硶瀹夎鍒版牴鏂囦欢绯荤粺涓?        - 鍥轰欢鎻愪緵浣跨敤鏈湴淇℃伅鏀堕泦鐨勩€佷负璁惧閲忚韩瀹氬埗鐨勯潪甯哥嫭鐗圭殑璁惧鐗瑰畾
          鏁版嵁銆備緥濡傜Щ鍔ㄨ澶?WiFi 鑺墖缁勭殑鏍″噯鏁版嵁銆傝鏍″噯鏁版嵁骞堕潪鎵€鏈?          璁惧閫氱敤锛岃€屾槸閽堝姣忓彴璁惧瀹氬埗銆傛绫讳俊鎭彲鑳藉畨瑁呭湪闄ゆ牴鏂囦欢绯荤粺
          鎵€鍦ㄥ垎鍖轰箣澶栫殑鍗曠嫭闂瓨鍒嗗尯涓娿€?
## 鍥為€€鏈哄埗鐨勭被鍨?

瀹為檯鏈変袱绉嶅洖閫€鏈哄埗鍙敤锛屽畠浠叡鐢ㄥ悓涓€涓?sysfs 鎺ュ彛浣滀负鍔犺浇璁炬柦锛?
- Kobject uevent 鍥為€€鏈哄埗
- 鑷畾涔夊洖閫€鏈哄埗

棣栧厛鏉ヨ褰曞叡鐢ㄧ殑 sysfs 鍔犺浇璁炬柦銆?
## 鍥轰欢 sysfs 鍔犺浇璁炬柦


涓轰簡甯姪璁惧椹卞姩浣跨敤鍥為€€鏈哄埗涓婁紶鍥轰欢锛屽浐浠跺熀纭€璁炬柦浼氬垱寤轰竴涓?sysfs 鎺ュ彛锛?浠ヤ究鐢ㄦ埛绌洪棿鍦ㄥ浐浠跺氨缁椂鍔犺浇骞堕€氱煡銆傝 sysfs 鐩綍閫氳繃 fw_create_instance()
鍒涘缓銆傛璋冪敤鍒涘缓涓€涓互鎵€璇锋眰鍥轰欢鍛藉悕鐨勬柊 struct device锛屽苟閫氳繃灏嗗彂鍑鸿姹傜殑
璁惧鍏宠仈涓鸿璁惧鐨勭埗璁惧锛屽皢鍏跺缓绔嬪埌璁惧灞傜骇涓€傝 sysfs 鐩綍鐨勬枃浠跺睘鎬х敱
鏂拌澶囩殑绫伙紙firmware_class锛夊拰缁勶紙fw_dev_attr_groups锛夊畾涔夊拰鎺у埗銆傝繖鍏跺疄
灏辨槸鏈€鍒?firmware_class 妯″潡鍚嶇О鐨勭敱鏉ワ紝鍥犱负鏈€鍒濆敮涓€鍙敤鐨勫浐浠跺姞杞芥満鍒跺氨鏄?鎴戜滑鐜板湪鐢ㄤ綔鍥為€€鏈哄埗鐨勬満鍒讹紝瀹冩敞鍐屼簡涓€涓?struct class firmware_class銆傜敱浜?鎵€鏆撮湶鐨勫睘鎬ф槸妯″潡鍚嶇О鐨勪竴閮ㄥ垎锛屾ā鍧楀悕 firmware_class 灏嗘潵涓嶈兘琚噸鍛藉悕锛屼互
纭繚涓庢棫鐢ㄦ埛绌洪棿鐨勫悜鍚庡吋瀹规€с€?
瑕佷娇鐢?sysfs 鎺ュ彛鍔犺浇鍥轰欢锛屾垜浠毚闇蹭竴涓?loading 鎸囩ず绗︼紝浠ュ強涓€涓枃浠?鐢ㄦ潵涓婁紶鍥轰欢鍒帮細

  - /sys/$DEVPATH/loading
  - /sys/$DEVPATH/data

瑕佷笂浼犲浐浠讹紝鎮ㄥ皢 1 鍐欏叆 loading 鏂囦欢锛屼互鎸囩ず鎮ㄦ鍦ㄥ姞杞藉浐浠躲€傜劧鍚庡皢鍥轰欢鍐欏叆
data 鏂囦欢锛屽苟閫氳繃灏?0 鍐欏叆 loading 鏂囦欢鏉ラ€氱煡鍐呮牳鍥轰欢宸插氨缁€?
鐢ㄤ簬甯姪閫氳繃 sysfs 鍔犺浇鍥轰欢鐨勫浐浠惰澶囷紝浠呭湪鐩存帴鍥轰欢鍔犺浇澶辫触銆佷笖涓烘偍鐨勫浐浠?璇锋眰鍚敤浜嗗洖閫€鏈哄埗鏃舵墠浼氬垱寤猴紝杩欑敱 `firmware_fallback_sysfs` 璁剧疆銆傞渶瑕?閲嶇敵鐨勬槸锛屽鏋滅洿鎺ユ枃浠剁郴缁熸煡鎵炬垚鍔燂紝鍒欎笉浼氬垱寤轰换浣曡澶囥€?
```

        echo 1 > /sys/$DEVPATH/loading

```
浼氱珛鍗虫竻闄や换浣曞厛鍓嶇殑閮ㄥ垎鍔犺浇锛屽苟浣垮浐浠?API 杩斿洖閿欒銆傚湪鍔犺浇鍥轰欢鏃讹紝
firmware_class 浠?PAGE_SIZE 涓哄閲忓闀夸竴涓敤浜庡浐浠舵暟鎹殑缂撳啿鍖猴紝浠ュ绾?浼犲叆鐨勯暅鍍忋€?
firmware_data_read() 鍜?firmware_loading_show() 浠呬负 test_firmware 椹卞姩
鎻愪緵鐢ㄤ簬娴嬭瘯锛屽畠浠笉浼氬湪姝ｅ父浣跨敤涓璋冪敤锛屼篃涓嶆湡鏈涜鐢ㄦ埛绌洪棿甯歌浣跨敤銆?
### firmware_fallback_sysfs

   :functions: firmware_fallback_sysfs

## 鍥轰欢 kobject uevent 鍥為€€鏈哄埗


鐢变簬涓?sysfs 鎺ュ彛鍒涘缓浜嗕竴涓澶囦互杈呭姪鍔犺浇鍥轰欢浣滀负鍥為€€鏈哄埗锛岀敤鎴风┖闂村彲浠?渚濋潬 kobject uevent 鑾风煡璇ヨ澶囩殑娣诲姞銆傚皢璁惧鍔犲叆璁惧灞傜骇鎰忓懗鐫€鍥轰欢鍔犺浇鐨?鍥為€€鏈哄埗宸茶鍚姩銆傚疄鐜扮粏鑺傝鍙傞槄 fw_load_sysfs_fallback()锛岀壒鍒槸鍏充簬
dev_set_uevent_suppress() 鍜?kobject_uevent() 鐨勪娇鐢ㄣ€?
鍐呮牳鐨?kobject uevent 鏈哄埗瀹炵幇浜?lib/kobject_uevent.c锛屽畠鍚戠敤鎴风┖闂村彂鍑?uevent銆備綔涓哄 kobject uevent 鐨勮ˉ鍏咃紝Linux 鍙戣鐗堜篃鍙互鍚敤
CONFIG_UEVENT_HELPER_PATH锛屽畠鍒╃敤鏍稿績鍐呮牳鐨?usermode helper锛圲MH锛夊姛鑳芥潵
璋冪敤涓€涓敤鎴风┖闂磋緟鍔╃▼搴忓鐞?kobject uevent銆備笉杩囧湪瀹炶返涓紝娌℃湁浠讳綍鏍囧噯
鍙戣鐗堟浘缁忎娇鐢ㄨ繃 CONFIG_UEVENT_HELPER_PATH銆傚鏋滃惎鐢ㄤ簡
CONFIG_UEVENT_HELPER_PATH锛屽垯姣忔鍐呮牳涓?kobject_uevent_env() 琚皟鐢ㄤ互瑙﹀彂
kobject uevent 鏃讹紝閮戒細璋冪敤姝や簩杩涘埗绋嬪簭銆?
鐢ㄦ埛绌洪棿鏇炬敮鎸佷笉鍚岀殑瀹炵幇鏉ュ埄鐢ㄦ鍥為€€鏈哄埗銆傚綋鍥轰欢鍔犺浇鍙兘浣跨敤 sysfs 鏈哄埗鏃讹紝
鐢ㄦ埛绌洪棿缁勪欢 鈥渉otplug鈥?鎻愪緵浜嗙洃鎺?kobject 浜嬩欢鐨勫姛鑳姐€傚巻鍙蹭笂瀹冨悗鏉ヨ systemd
鐨?udev 鍙栦唬锛屼笉杩囪嚜 2014 骞?8 鏈堢殑 v217 璧凤紝udev 鍥轰欢鍔犺浇鏀寔宸蹭粠 udev 涓?绉婚櫎锛坰ystemd commit be2ea723b1d0锛屸€渦dev: remove userspace firmware loading
support鈥濓級銆傝繖鎰忓懗鐫€濡備粖澶у鏁?Linux 鍙戣鐗堝苟鏈娇鐢ㄦ垨鍒╃敤 kobject uevent
鎻愪緵鐨勫浐浠跺洖閫€鏈哄埗銆傜敱浜庡浠婂ぇ澶氭暟鍙戣鐗堢鐢ㄤ簡
CONFIG_FW_LOADER_USER_HELPER_FALLBACK锛岃繖涓€鎯呭喌灏や负涓ラ噸銆?
鏈夊叧 kobject 浜嬩欢鍙橀噺璁剧疆鐨勭粏鑺傦紝璇峰弬闃?do_firmware_uevent()銆傜洰鍓嶉殢
鈥渒object add鈥濅簨浠朵紶閫掔粰鐢ㄦ埛绌洪棿鐨勫彉閲忔湁锛?
- FIRMWARE=鍥轰欢鍚嶇О
- TIMEOUT=瓒呮椂鍊?- ASYNC=璇?API 璇锋眰鏄惁涓哄紓姝?
榛樿鎯呭喌涓?DEVPATH 鐢卞唴鏍稿唴閮?kobject 鍩虹璁炬柦璁剧疆銆?```

        # $DEVPATH 鍜?$FIRMWARE 鍧囧凡鐢辩幆澧冩彁渚涖€?        MY_FW_DIR=/lib/firmware/
        echo 1 > /sys/$DEVPATH/loading
        cat $MY_FW_DIR/$FIRMWARE > /sys/$DEVPATH/data
        echo 0 > /sys/$DEVPATH/loading

```
## 鍥轰欢鑷畾涔夊洖閫€鏈哄埗


request_firmware_nowait() 鐨勮皟鐢ㄨ€呮湁鍙︿竴绉嶅彲鐢ㄧ殑閫夋嫨锛氫緷璧?sysfs 鍥為€€鏈哄埗锛?浣嗚姹備笉瑕佸悜鐢ㄦ埛绌洪棿鍙戝嚭 kobject uevent銆傝繖鑳屽悗鐨勫師濮嬮€昏緫鏄紝闄?udev 涔嬪鐨?宸ュ叿鍙兘闇€瑕佸埌闈炰紶缁熻矾寰勨€斺€斿嵆 鈥淒irect filesystem lookup鈥濓紙鐩存帴鏂囦欢绯荤粺鏌ユ壘锛?涓€鑺傛墍璁板綍鍒楄〃涔嬪鐨勮矾寰勨€斺€斿幓鏌ユ壘鍥轰欢銆傛閫夐」瀵瑰叾浠栦换浣?API 璋冪敤閮戒笉鍙敤锛?鍥犱负瀹冧滑鎬绘槸琚己鍒跺彂鍑?uevent銆?
鐢变簬 uevent 鍙湁鍦ㄥ洖閫€鏈哄埗鍦ㄥ唴鏍镐腑鍚敤鏃舵墠鏈夋剰涔夛紝浼间箮鍦ㄤ竴浜涙病鏈夊湪鍏跺唴鏍镐腑
鍚敤鍥為€€鏈哄埗鐨勫唴鏍镐笂鍚敤 uevent 浼氬緢濂囨€€傞仐鎲剧殑鏄紝鎴戜滑杩樹緷璧栧彲鐢?request_firmware_nowait() 绂佺敤鐨?uevent 鏍囧織鏉ヤ负鍥轰欢璇锋眰璁剧疆鍥轰欢缂撳瓨銆傚涓?鎵€杩帮紝鍥轰欢缂撳瓨浠呭湪璇?API 璋冪敤鐨?uevent 鍚敤鏃舵墠琚缃€傚敖绠¤繖浼氫负
request_firmware_nowait() 璋冪敤绂佺敤鍥轰欢缂撳瓨锛屼絾姝?API 鐨勪娇鐢ㄨ€呬笉搴斿皢鍏剁敤浜?绂佺敤缂撳瓨锛屽洜涓洪偅骞堕潪璇ユ爣蹇楃殑鍘熷鐢ㄩ€斻€備笉璁剧疆 uevent 鏍囧織鎰忓懗鐫€鎮ㄥ笇鏈涢€夋嫨鍔犲叆
鍥轰欢鍥為€€鏈哄埗锛屼絾鎮ㄥ笇鏈涙姂鍒?kobject uevent锛屽洜涓烘偍鎷ユ湁鑷畾涔夌殑瑙ｅ喅鏂规锛屽畠灏?浠ユ煇绉嶆柟寮忕洃鎺ф偍鐨勮澶囪鍔犲叆璁惧灞傜骇锛屽苟閫氳繃鑷畾涔夎矾寰勪负鎮ㄥ姞杞藉浐浠躲€?
## 鍥轰欢鍥為€€瓒呮椂


鍥轰欢鍥為€€鏈哄埗鏈変竴涓秴鏃躲€傚鏋滃湪瓒呮椂鍊间箣鍓嶅浐浠舵湭琚姞杞藉埌 sysfs 鎺ュ彛涓婏紝鍒欎細鍚?椹卞姩鍙戦€佷竴涓敊璇€傞粯璁ゆ儏鍐典笅锛屽鏋?uevent 鏄彲鍙栫殑锛岃秴鏃惰涓?60 绉掞紝鍚﹀垯浣跨敤
MAX_JIFFY_OFFSET锛堝敖鍙兘澶х殑瓒呮椂锛夈€傚湪闈?uevent 鎯呭喌涓嬩娇鐢?MAX_JIFFY_OFFSET
鐨勯€昏緫鏄紝鑷畾涔夎В鍐虫柟妗堝皢鏈夊畠闇€瑕佺殑鏃堕棿鏉ュ姞杞藉浐浠躲€?
鎮ㄥ彲浠ラ€氳繃灏嗘湡鏈涚殑瓒呮椂鍐欏叆浠ヤ笅鏂囦欢鏉ヨ嚜瀹氫箟鍥轰欢瓒呮椂锛?
- /sys/class/firmware/timeout

濡傛灉鎮ㄥ啓鍏?0锛屾剰鍛崇潃灏嗕娇鐢?MAX_JIFFY_OFFSET銆傝秴鏃剁殑鏁版嵁绫诲瀷鏄?int銆?
## EFI 鍐呭祵鍥轰欢鍥為€€鏈哄埗


鍦ㄦ煇浜涜澶囦笂锛岀郴缁熺殑 EFI 浠ｇ爜/ROM 鍙兘鍖呭惈绯荤粺閮ㄥ垎闆嗘垚澶栬璁惧鐨勫浐浠跺壇鏈紝
鑰岃澶栬鐨?Linux 璁惧椹卞姩闇€瑕佽闂鍥轰欢銆?
闇€瑕佹绫诲浐浠剁殑璁惧椹卞姩鍙互浣跨敤 firmware_request_platform() 鍑芥暟锛屾敞鎰忚繖鏄?涓€涓笌鍏朵粬鍥為€€鏈哄埗鍒嗙鐨勫洖閫€鏈哄埗锛屼笖涓嶄娇鐢?sysfs 鎺ュ彛銆?
闇€瑕佹鍥轰欢鐨勮澶囬┍鍔ㄥ彲浠ヤ娇鐢?efi_embedded_fw_desc 缁撴瀯浣撴潵鎻忚堪鍏舵墍闇€鍥轰欢锛?
   :functions: efi_embedded_fw_desc

EFI 鍐呭祵鍥轰欢浠ｇ爜鐨勫伐浣滄柟寮忔槸鎵弿鎵€鏈?EFI_BOOT_SERVICES_CODE 鍐呭瓨娈碉紝瀵绘壘
鍖归厤鍓嶇紑鐨?8 瀛楄妭搴忓垪锛涘鏋滄壘鍒颁簡鍓嶇紑锛屽垯瀵?length 瀛楄妭鍋?sha256锛岃嫢鍖归厤鍒?澶嶅埗 length 瀛楄妭骞跺皢鍏跺姞鍏ュ凡鎵惧埌鍥轰欢鍒楄〃銆?
涓洪伩鍏嶅湪鎵€鏈夌郴缁熶笂閮借繘琛岃繖绉嶄唬浠疯緝楂樼殑鎵弿锛屼娇鐢ㄤ簡 dmi 鍖归厤銆傞┍鍔ㄥ簲褰撳鍑轰竴涓?dmi_system_id 鏁扮粍锛屽叾涓瘡涓潯鐩殑 driver_data 鎸囧悜涓€涓?efi_embedded_fw_desc銆?
瑕佸悜 efi-embedded-fw 浠ｇ爜娉ㄥ唽姝ゆ暟缁勶紝椹卞姩闇€瑕侊細

1. 濮嬬粓鍐呭缓鍒板唴鏍镐腑锛屾垨灏?dmi_system_id 鏁扮粍瀛樻斁鍦ㄤ竴涓缁堣鍐呭缓鐨勭嫭绔?   鐩爣鏂囦欢涓€?
2. 鍦?include/linux/efi_embedded_fw.h 涓坊鍔犲璇?dmi_system_id 鏁扮粍鐨?   extern 澹版槑銆?
3. 灏?dmi_system_id 鏁扮粍娣诲姞鍒?drivers/firmware/efi/embedded-firmware.c 涓?   鐨?embedded_fw_table锛屽苟鐢?#ifdef 娴嬭瘯璇ラ┍鍔ㄦ槸鍚︽琚唴寤烘潵鍖呰９銆?
4. 鍦ㄥ叾 Kconfig 鏉＄洰涓坊鍔?鈥渟elect EFI_EMBEDDED_FIRMWARE if EFI_STUB鈥濄€?
firmware_request_platform() 鍑芥暟灏嗗缁堥鍏堝皾璇曚互鎸囧畾鍚嶇О鐩存帴浠庣鐩樺姞杞藉浐浠讹紝
鍥犳閫氳繃灏嗘枃浠舵斁鍦?/lib/firmware 涓嬶紝EFI 鍐呭祵鍥轰欢鎬绘槸鍙互琚鐩栥€?
娉ㄦ剰锛?
1. 鎵弿 EFI 鍐呭祵鍥轰欢鐨勪唬鐮佽繍琛屼簬 start_kernel() 鎺ヨ繎鏈熬澶勶紝鎭板湪璋冪敤
   rest_init() 涔嬪墠銆傚浜庝娇鐢?subsys_initcall() 娉ㄥ唽鑷韩鐨勬櫘閫氶┍鍔ㄥ拰瀛愮郴缁?   鑰岃█杩欐棤鍏崇揣瑕併€傝繖鎰忓懗鐫€杩愯寰楁洿鏃╃殑浠ｇ爜鏃犳硶浣跨敤 EFI 鍐呭祵鍥轰欢銆?
2. 鐩墠 EFI 鍐呭祵鍥轰欢浠ｇ爜鍋囧畾鍥轰欢鎬绘槸璧峰浜庝竴涓?8 瀛楄妭鏁存暟鍊嶇殑鍋忕Щ锛屽鏋滄偍鐨?   鎯呭喌骞堕潪濡傛锛岃鎻愪氦琛ヤ竵鏉ヤ慨澶嶃€?
3. 鐩墠 EFI 鍐呭祵鍥轰欢浠ｇ爜浠呭湪 x86 涓婂伐浣滐紝鍥犱负鍏朵粬鏋舵瀯鍦?EFI 鍐呭祵鍥轰欢浠ｇ爜
   鏈夋満浼氭壂鎻忎箣鍓嶅氨閲婃斁浜?EFI_BOOT_SERVICES_CODE銆?
4. 褰撳墠瀵?EFI_BOOT_SERVICES_CODE 鐨勬毚鍔涙壂鎻忔槸涓€绉嶄复鏃剁殑鏆村姏鏂规銆傛浘鏈夎璁?   浣跨敤 UEFI Platform Initialization锛圥I锛夎鑼冪殑 Firmware Volume 鍗忚銆傝繖涓€
   鏂规宸茶鎷掔粷锛屽洜涓?FV 鍗忚渚濊禆 PI 瑙勮寖鐨?**internal** 鎺ュ彛锛屽苟涓旓細
   1. PI 瑙勮寖鏍规湰鏈畾涔夊璁惧浐浠?   2. PI 瑙勮寖鐨勫唴閮ㄦ帴鍙ｄ笉淇濊瘉浠讳綍鍚戝悗鍏煎鎬с€侳V 涓殑浠讳綍瀹炵幇缁嗚妭閮藉彲鑳?   鍙戠敓鍙樻洿锛屽苟鍙兘鍥犵郴缁熻€屽紓銆傛敮鎸?FV 鍗忚灏嗗崄鍒嗗洶闅撅紝鍥犱负瀹冨埢鎰忓叿鏈?   妯＄硦鎬с€?
### 妫€鏌ュ苟鎻愬彇鍐呭祵鍥轰欢鐨勭ず渚?

瑕佹鏌ワ紙渚嬪锛塖ilead 瑙︽懜灞忔帶鍒跺櫒鐨勫唴宓屽浐浠讹紝璇锋墽琛屼互涓嬫搷浣滐細

1. 鍦ㄥ唴鏍稿懡浠よ涓姞鍏?efi=debug 鍚姩绯荤粺

2. 灏?/sys/kernel/debug/efi/boot_services_code? 澶嶅埗鍒版偍鐨勪富鐩綍

3. 鍦ㄥ崄鍏繘鍒剁紪杈戝櫒涓墦寮€ boot_services_code? 鏂囦欢锛屾悳绱?Silead 鍥轰欢鐨?   榄旀湳鍓嶇紑锛欶0 00 00 00 02 00 00 00锛岃繖浼氱粰鍑哄浐浠跺湪 boot_services_code? 鏂囦欢
   涓殑璧峰鍦板潃銆?
4. 璇ュ浐浠舵湁鐗瑰畾妯″紡锛屽畠浠ヤ竴涓?8 瀛楄妭鐨勯〉鍦板潃寮€澶达紝绗竴椤甸€氬父涓?F0 00 00 00
   02 00 00 00锛屽悗璺?32 浣嶅瓧鍦板潃 + 32 浣嶅€肩殑閰嶅銆傚瓧鍦板潃姣忓閫掑 4 瀛楄妭
   锛? 涓瓧锛夛紝鐩村埌涓€椤靛畬鎴愩€備竴椤靛畬鏁村悗璺熼殢涓€涓柊鐨勯〉鍦板潃锛屽啀璺熸洿澶氬瓧 + 鍊?   閰嶅銆傝繖褰㈡垚涓€绉嶉潪甯哥嫭鐗圭殑妯″紡銆傚悜涓嬫粴鍔ㄧ洿鍒版妯″紡鍋滄锛岃繖缁欏嚭鍥轰欢鍦?   boot_services_code? 鏂囦欢涓殑缁撴潫鍦板潃銆?
5. 鈥渄d if=boot_services_code? of=firmware bs=1 skip=<begin-addr> count=<len>鈥?   灏嗕负鎮ㄦ彁鍙栧浐浠躲€傚湪鍗佸叚杩涘埗缂栬緫鍣ㄤ腑妫€鏌ュ浐浠舵枃浠讹紝浠ョ‘淇濇偍缁欏嚭鐨?dd 鍙傛暟
   姝ｇ‘銆?
6. 灏嗗叾浠ユ湡鏈涚殑鍚嶇О澶嶅埗鍒?/lib/firmware 涓嬭繘琛屾祴璇曘€?
7. 濡傛灉鎻愬彇鐨勫浐浠跺彲鐢紝鎮ㄥ彲浠ヤ娇鐢ㄦ壘鍒扮殑淇℃伅濉厖涓€涓?efi_embedded_fw_desc
   缁撴瀯浣撴潵鎻忚堪瀹冿紝杩愯 鈥渟ha256sum firmware鈥?浠ヨ幏鍙栬濉叆 sha256 瀛楁鐨?   sha256 鏍￠獙鍜屻€?