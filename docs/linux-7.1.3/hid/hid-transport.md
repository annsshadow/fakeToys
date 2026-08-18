## HID I/O 浼犺緭椹卞姩

HID 瀛愮郴缁熺嫭绔嬩簬搴曞眰鐨勪紶杈撻┍鍔ㄣ€傛渶鍒濅粎鏀寔 USB锛屼絾鍏朵粬瑙勮寖涔熼噰绾充簡 HID 璁捐骞舵彁渚涗簡鏂扮殑浼犺緭椹卞姩銆傚唴鏍歌嚦灏戝寘鍚 USB銆丅luetooth銆両2C 浠ュ強鐢ㄦ埛绌洪棿 I/O 椹卞姩鐨勬敮鎸併€?
## 1) HID 鎬荤嚎

HID 瀛愮郴缁熻璁捐涓轰竴涓€荤嚎銆備换浣?I/O 瀛愮郴缁熼兘鍙互鎻愪緵 HID 璁惧骞跺悜 HID 鎬荤嚎娉ㄥ唽銆傞殢鍚?HID core 鍦ㄥ叾涓婂姞杞介€氱敤鐨勮澶囬┍鍔ㄣ€備紶杈撻┍鍔ㄨ礋璐ｅ師濮嬫暟鎹殑浼犺緭浠ュ強璁惧鐨勫缓绔嬩笌绠＄悊銆侶ID core 璐熻矗鎶ュ憡瑙ｆ瀽銆佹姤鍛婅В閲婁互鍙婄敤鎴风┖闂?API銆傝澶囩殑鐗瑰畾缁嗚妭鍜屾€紓琛屼负锛坬uirks锛夌敱鍚勫眰鏍规嵁鍏蜂綋鎯呭喌澶勭悊銆?
```

 +-----------+  +-----------+            +-----------+  +-----------+
 | Device #1 |  | Device #i |            | Device #j |  | Device #k |
 +-----------+  +-----------+            +-----------+  +-----------+
          \      //                              \      //
        +------------+                          +------------+
        | I/O Driver |                          | I/O Driver |
        +------------+                          +------------+
              ||                                      ||
     +------------------+                    +------------------+
     | Transport Driver |                    | Transport Driver |
     +------------------+                    +------------------+
                       \___                ___/
                           \              /
                          +----------------+
                          |    HID Core    |
                          +----------------+
                           /  |        |  \
                          /   |        |   \
             ____________/    |        |    \_________________
            /                 |        |                      \
           /                  |        |                       \
 +----------------+  +-----------+  +------------------+  +------------------+
 | Generic Driver |  | MT Driver |  | Custom Driver #1 |  | Custom Driver #2 |
 +----------------+  +-----------+  +------------------+  +------------------+

```
绀轰緥椹卞姩锛?
  - I/O锛歎SB銆両2C銆丅luetooth-l2cap
  - Transport锛歎SB-HID銆両2C-HID銆丅T-HIDP

鏈浘涓?"HID Core" 浠ヤ笅鐨勯儴鍒嗗仛浜嗙畝鍖栵紝鍥犱负閭ｄ簺閮ㄥ垎浠呬笌 HID 璁惧椹卞姩鐩稿叧銆備紶杈撻┍鍔ㄦ棤闇€浜嗚В杩欎簺缁嗚妭銆?
### 1.1) 璁惧寤虹珛

I/O 椹卞姩閫氬父鍚戜紶杈撻┍鍔ㄦ彁渚涚儹鎻掓嫈妫€娴嬫垨璁惧鏋氫妇 API銆備紶杈撻┍鍔ㄥ埄鐢ㄨ繖浜涙潵瀵绘壘鍚堥€傜殑 HID 璁惧銆傚畠浠垎閰?HID 璁惧瀵硅薄骞跺悜 HID core 娉ㄥ唽銆備紶杈撻┍鍔ㄦ棤闇€鍚?HID core 娉ㄥ唽鑷韩銆侶ID core 姘歌繙涓嶇煡閬撴湁鍝簺浼犺緭椹卞姩鍙敤锛屼篃涓嶅叧蹇冭繖涓€鐐癸紝瀹冨彧鍏冲績璁惧銆?
浼犺緭椹卞姩浼氫负姣忎釜璁惧闄勫姞涓€涓父椹荤殑 "struct hid_ll_driver" 瀵硅薄銆備竴鏃﹁澶囧悜 HID core 娉ㄥ唽锛孒ID core 灏变細閫氳繃璇ョ粨鏋勪綋鎻愪緵鐨勫洖璋冧笌璁惧閫氫俊銆?
浼犺緭椹卞姩璐熻矗妫€娴嬭澶囨晠闅滀笌鎷旈櫎銆傚彧瑕佽澶囦粛澶勪簬娉ㄥ唽鐘舵€侊紝鏃犺鏄惁鏈夎澶囨晠闅滐紝HID core 閮戒細缁х画鎿嶄綔璇ヨ澶囥€備竴鏃︿紶杈撻┍鍔ㄦ娴嬪埌鎷旈櫎鎴栨晠闅滀簨浠讹紝灏卞繀椤讳粠 HID core 娉ㄩ攢璇ヨ澶囷紝姝ゅ悗 HID core 灏嗗仠姝娇鐢ㄦ墍鎻愪緵鐨勫洖璋冦€?
### 1.2) 浼犺緭椹卞姩瑕佹眰

鏈枃妗ｄ腑鐨?"asynchronous"锛堝紓姝ワ級涓?"synchronous"锛堝悓姝ワ級鎻忚堪鐨勬槸涓庣‘璁わ紙acknowledgement锛夌浉鍏崇殑浼犺緭琛屼负銆傚紓姝ラ€氶亾涓嶅緱鎵ц浠讳綍鍚屾鎿嶄綔锛屼緥濡傜瓑寰呯‘璁ゆ垨鏍￠獙銆傞€氬父锛屽湪寮傛閫氶亾涓婅繍琛岀殑 HID 璋冪敤蹇呴』鑳藉鍦?atomic-context锛堝師瀛愪笂涓嬫枃锛変腑鑹ソ宸ヤ綔銆?鍙︿竴鏂归潰锛屽悓姝ラ€氶亾鍙互鐢变紶杈撻┍鍔ㄤ互浠绘剰鏂瑰紡瀹炵幇銆傚畠浠彲鑳戒笌寮傛閫氶亾鐩稿悓锛屼絾涔熷彲浠ヤ互闃诲鏂瑰紡鎻愪緵纭鎶ュ憡銆佸け璐ヨ嚜鍔ㄩ噸浼犵瓑銆傚鏋滃紓姝ラ€氶亾涓婇渶瑕佹绫诲姛鑳斤紝浼犺緭椹卞姩蹇呴』閫氳繃鍏惰嚜韬殑 worker 绾跨▼鏉ュ疄鐜般€?
HID core 瑕佹眰浼犺緭椹卞姩閬靛惊鐗瑰畾鐨勮璁°€備紶杈撻┍鍔ㄥ繀椤讳负姣忎釜 HID 璁惧鎻愪緵涓や釜鍙屽悜 I/O 閫氶亾銆傝繖浜涢€氶亾鍦ㄧ‖浠朵笂鏈韩鏈繀鏄弻鍚戠殑銆備紶杈撻┍鍔ㄤ篃鍙兘鍙彁渚?4 涓崟鍚戦€氶亾锛屾垨鑰呭皢鍏ㄩ儴鍥涗釜閫氶亾澶氳矾澶嶇敤鍒板崟涓€鐗╃悊閫氶亾涓娿€備絾鍦ㄦ湰鏂囨。涓紝鎴戜滑灏嗗畠浠弿杩颁负涓や釜鍙屽悜閫氶亾锛屽洜涓哄畠浠叿鏈夎嫢骞插叡鍚岀壒鎬с€?
 - 涓柇閫氶亾锛坕ntr锛夛細intr 閫氶亾鐢ㄤ簬寮傛鏁版嵁鎶ュ憡銆傛湰閫氶亾涓婁笉鍙戦€佺鐞嗗懡浠ゆ垨鏁版嵁纭銆備换浣曟湭缁忚姹傜殑浼犲叆鎴栦紶鍑烘暟鎹姤鍛婇兘蹇呴』閫氳繃鏈€氶亾鍙戦€侊紝涓旇繙绔笉浼氬彂閫佺‘璁ゃ€傝澶囬€氬父鍦ㄦ湰閫氶亾涓婂彂閫佸叾杈撳叆浜嬩欢銆傞櫎闈為渶瑕侀珮鍚炲悙閲忥紝鍚﹀垯浼犲嚭浜嬩欢涓€鑸笉閫氳繃 intr 鍙戦€併€? - 鎺у埗閫氶亾锛坈trl锛夛細ctrl 閫氶亾鐢ㄤ簬鍚屾璇锋眰涓庤澶囩鐞嗐€傛湭缁忚姹傜殑鏁版嵁杈撳叆浜嬩欢涓嶅緱鍦ㄦ湰閫氶亾鍙戦€侊紝閫氬父浼氳蹇界暐銆傜浉鍙嶏紝璁惧鍙湪鏈€氶亾涓婂彂閫佺鐞嗕簨浠舵垨鏄涓绘満璇锋眰鐨勫簲绛斻€?   control 閫氶亾鐢ㄤ簬瀵硅澶囪繘琛岀洿鎺ョ殑闃诲鏌ヨ锛屼笌 intr 閫氶亾涓婄殑浠讳綍浜嬩欢鏃犲叧銆?   浼犲嚭鎶ュ憡閫氬父閫氳繃鍚屾鐨?SET_REPORT 璇锋眰鍦?ctrl 閫氶亾涓婂彂閫併€?
璁惧涓?HID core 涔嬮棿鐨勯€氫俊涓昏閫氳繃 HID 鎶ュ憡瀹屾垚銆傛姤鍛婂彲浠ユ槸浠ヤ笅涓夌绫诲瀷涔嬩竴锛?
 - INPUT 鎶ュ憡锛圛NPUT Report锛夛細INPUT 鎶ュ憡鎻愪緵浠庤澶囧埌涓绘満鐨勬暟鎹€傝繖浜涙暟鎹彲鑳藉寘鍚寜閿簨浠躲€佽酱浜嬩欢銆佺數姹犵姸鎬佺瓑銆傝繖浜涙暟鎹敱璁惧鐢熸垚锛屽苟鍙湪鏈夋垨娌℃湁鏄惧紡璇锋眰鐨勬儏鍐典笅鍙戦€佺粰涓绘満銆傝澶囧彲浠ラ€夋嫨鎸佺画鍙戦€佹暟鎹紝鎴栦粎鍦ㄧ姸鎬佹敼鍙樻椂鍙戦€併€? - OUTPUT 鎶ュ憡锛圤UTPUT Report锛夛細OUTPUT 鎶ュ憡鐢ㄤ簬鏀瑰彉璁惧鐘舵€併€傚畠浠粠涓绘満鍙戝線璁惧锛屽彲鑳藉寘鍚?LED 璇锋眰銆侀渿鍔ㄨ姹傜瓑銆侽UTPUT 鎶ュ憡姘歌繙涓嶄細浠庤澶囧彂寰€涓绘満锛屼絾涓绘満鍙互鑾峰彇瀹冧滑鐨勫綋鍓嶇姸鎬併€?   涓绘満鍙互閫夋嫨鎸佺画鍙戦€?OUTPUT 鎶ュ憡锛屾垨浠呭湪鐘舵€佹敼鍙樻椂鍙戦€併€? - FEATURE 鎶ュ憡锛團EATURE Report锛夛細FEATURE 鎶ュ憡鐢ㄤ簬鐗瑰畾鐨勯潤鎬佽澶囩壒鎬э紝浠庝笉鑷彂涓婃姤銆備富鏈哄彲浠ヨ鍙栧拰/鎴栧啓鍏ュ畠浠互璁块棶璇稿鐢垫睜鐘舵€佹垨璁惧璁剧疆涔嬬被鐨勬暟鎹€?   FEATURE 鎶ュ憡缁濅笉浼氬湪鏃犺姹傜殑鎯呭喌涓嬪彂閫併€備富鏈哄繀椤绘樉寮忓湴璁剧疆鎴栬幏鍙?FEATURE 鎶ュ憡銆傝繖涔熸剰鍛崇潃 FEATURE 鎶ュ憡姘歌繙涓嶄細鍦?intr 閫氶亾涓婂彂閫侊紝鍥犱负璇ラ€氶亾鏄紓姝ョ殑銆?
INPUT 鍜?OUTPUT 鎶ュ憡鍙互浣滀负绾暟鎹姤鍛婂湪 intr 閫氶亾涓婂彂閫併€傚 INPUT 鎶ュ憡鑰岃█杩欐槸甯歌鐨勮繍琛屾ā寮忋€備絾瀵?OUTPUT 鎶ュ憡鑰岃█寰堝皯杩欐牱鍋氾紝鍥犱负 OUTPUT 鎶ュ憡閫氬父鐩稿綋绋€灏戙€備笉杩囪澶囧彲鑷敱鍦板ぇ閲忎娇鐢ㄥ紓姝?OUTPUT 鎶ュ憡锛堜緥濡傦紝瀹氬埗鐨?HID 闊抽鎵０鍣ㄥ氨澶ч噺浣跨敤璇ユ満鍒讹級銆?
涓嶈繃锛岀函鎶ュ憡涓嶅緱鍦?ctrl 閫氶亾涓婂彂閫併€傜浉鍙嶏紝ctrl 閫氶亾鎻愪緵鍚屾鐨?GET/SET_REPORT 璇锋眰銆傜函鎶ュ憡鍙厑璁稿湪 intr 閫氶亾涓婂彂閫侊紝骞朵笖鏄€氶亾涓婂敮涓€鐨勬暟鎹紶杈撴柟寮忋€?
 - GET_REPORT锛欸ET_REPORT 璇锋眰浠ユ姤鍛?ID 浣滀负杞借嵎锛岀敱涓绘満鍙戝線璁惧銆傝澶囧繀椤讳互閽堝鎵€璇锋眰鎶ュ憡 ID 鐨勬暟鎹姤鍛婁綔涓哄悓姝ョ‘璁わ紝鍦?ctrl 閫氶亾涓婂簲绛斻€傛瘡涓澶囧彧鑳芥湁涓€涓?GET_REPORT 璇锋眰澶勪簬鎸傝捣鐘舵€併€傜敱浜庨儴鍒嗕紶杈撻┍鍔ㄤ笉鍏佽鍚屾椂鍙戣捣澶氫釜 GET_REPORT 璇锋眰锛孒ID core 寮哄埗瀹炴柦浜嗚繖涓€闄愬埗銆?   娉ㄦ剰锛屼綔涓?GET_REPORT 璇锋眰搴旂瓟鑰岃鍙戦€佺殑鏁版嵁鎶ュ憡锛屼笉浼氳褰撲綔閫氱敤璁惧浜嬩欢澶勭悊銆備篃灏辨槸璇达紝濡傛灉璁惧涓嶈繍琛屽湪鎸佺画鏁版嵁涓婃姤妯″紡锛屽 GET_REPORT 鐨勫簲绛斾笉浼氭浛浠?intr 閫氶亾涓婄姸鎬佹敼鍙樻椂鐨勫師濮嬫暟鎹姤鍛娿€?   GET_REPORT 浠呯敱瀹氬埗鐨?HID 璁惧椹卞姩鐢ㄤ簬鏌ヨ璁惧鐘舵€併€傞€氬父 HID core 浼氱紦瀛樹换鎰忚澶囩姸鎬侊紝鍥犳闄や簡鍦ㄨ澶囧垵濮嬪寲鏈熼棿涓鸿幏鍙栧綋鍓嶇姸鎬佸锛岄伒寰?HID 瑙勮寖鐨勮澶囧苟涓嶉渶瑕佹璇锋眰銆?   GET_REPORT 璇锋眰鍙拡瀵逛笁绉嶆姤鍛婄被鍨嬩腑鐨勪换鎰忎竴绉嶅彂閫侊紝骞跺簲杩斿洖璁惧鐨勫綋鍓嶆姤鍛婄姸鎬併€備絾鏄紝鑻ヨ鑼冧笉鍏佽锛屽簳灞備紶杈撻┍鍔ㄥ彲鑳戒細闃绘浠?OUTPUT 鎶ュ憡浣滀负杞借嵎銆? - SET_REPORT锛歋ET_REPORT 璇锋眰浠ユ姤鍛?ID 鍔犳暟鎹綔涓鸿浇鑽枫€傚畠鐢变富鏈哄彂寰€璁惧锛岃澶囧繀椤绘牴鎹墍缁欐暟鎹洿鏂板叾褰撳墠鎶ュ憡鐘舵€併€傚彲浣跨敤涓夌鎶ュ憡绫诲瀷涓殑浠绘剰涓€绉嶃€備絾鏄紝鑻ヨ鑼冧笉鍏佽锛屽簳灞備紶杈撻┍鍔ㄥ彲鑳戒細闃绘浠?INPUT 鎶ュ憡浣滀负杞借嵎銆?   璁惧蹇呴』浠ュ悓姝ョ‘璁ゅ簲绛斻€備絾鏄紝HID core 骞朵笉瑕佹眰浼犺緭椹卞姩灏嗚纭杞彂缁?HID core銆?   涓?GET_REPORT 鐩稿悓锛屽悓涓€鏃跺埢鍙兘鏈変竴涓?SET_REPORT 澶勪簬鎸傝捣鐘舵€併€傜敱浜庨儴鍒嗕紶杈撻┍鍔ㄤ笉鏀寔澶氫釜鍚屾 SET_REPORT 璇锋眰锛孒ID core 寮哄埗瀹炴柦浜嗚繖涓€闄愬埗銆?
鍏朵粬 ctrl 閫氶亾璇锋眰鐢?USB-HID 鏀寔锛屼絾鍦ㄥぇ澶氭暟鍏朵粬浼犺緭灞傝鑼冧腑涓嶅彲鐢紙鎴栧凡琚純鐢級锛?
 - GET/SET_IDLE锛氫粎鐢?USB-HID 鍜?I2C-HID 浣跨敤銆? - GET/SET_PROTOCOL锛欻ID core 涓嶄娇鐢ㄣ€? - RESET锛氱敱 I2C-HID 浣跨敤锛屾湭鍦?HID core 涓寕鎺ャ€? - SET_POWER锛氱敱 I2C-HID 浣跨敤锛屾湭鍦?HID core 涓寕鎺ャ€?
## 2) HID API

### 2.1) 鍒濆鍖?
浼犺緭椹卞姩閫氬父浣跨敤浠ヤ笅娴佺▼鏉ユ敞鍐屼竴涓柊璁惧

```

	struct hid_device *hid;
	int ret;

	hid = hid_allocate_device();
	if (IS_ERR(hid)) {
		ret = PTR_ERR(hid);
		goto err_<...>;
	}

	strscpy(hid->name, <device-name-src>, sizeof(hid->name));
	strscpy(hid->phys, <device-phys-src>, sizeof(hid->phys));
	strscpy(hid->uniq, <device-uniq-src>, sizeof(hid->uniq));

	hid->ll_driver = &custom_ll_driver;
	hid->bus = <device-bus>;
	hid->vendor = <device-vendor>;
	hid->product = <device-product>;
	hid->version = <device-version>;
	hid->country = <device-country>;
	hid->dev.parent = <pointer-to-parent-device>;
	hid->driver_data = <transport-driver-data-field>;

	ret = hid_add_device(hid);
	if (ret)
		goto err_<...>;

```

涓€鏃﹁繘鍏?hid_add_device()锛孒ID core 灏卞彲鑳戒娇鐢?"custom_ll_driver" 涓彁渚涚殑鍥炶皟銆傛敞鎰忥紝鑻ュ簳灞備紶杈撻┍鍔ㄤ笉鏀寔锛屽垯鍍?"country" 杩欐牱鐨勫瓧娈靛彲琚拷鐣ャ€?
```

	hid_destroy_device(hid);

```

涓€鏃?hid_destroy_device() 杩斿洖锛孒ID core 灏嗕笉鍐嶄娇鐢ㄤ换浣曢┍鍔ㄥ洖璋冦€?
### 2.2) hid_ll_driver 鎿嶄綔

鍙敤鐨?HID 鍥炶皟濡備笅锛?
```

      int (*start) (struct hid_device *hdev)

   Called from HID device drivers once they want to use the device. Transport
   drivers can choose to setup their device in this callback. However, normally
   devices are already set up before transport drivers register them to HID core
   so this is mostly only used by USB-HID.

   ::

      void (*stop) (struct hid_device *hdev)

   Called from HID device drivers once they are done with a device. Transport
   drivers can free any buffers and deinitialize the device. But note that
   ->start() might be called again if another HID device driver is loaded on the
   device.

   Transport drivers are free to ignore it and deinitialize devices after they
   destroyed them via hid_destroy_device().

   ::

      int (*open) (struct hid_device *hdev)

   Called from HID device drivers once they are interested in data reports.
   Usually, while user-space didn't open any input API/etc., device drivers are
   not interested in device data and transport drivers can put devices asleep.
   However, once ->open() is called, transport drivers must be ready for I/O.
   ->open() calls are nested for each client that opens the HID device.

   ::

      void (*close) (struct hid_device *hdev)

   Called from HID device drivers after ->open() was called but they are no
   longer interested in device reports. (Usually if user-space closed any input
   devices of the driver).

   Transport drivers can put devices asleep and terminate any I/O of all
   ->open() calls have been followed by a ->close() call. However, ->start() may
   be called again if the device driver is interested in input reports again.

   ::

      int (*parse) (struct hid_device *hdev)

   Called once during device setup after ->start() has been called. Transport
   drivers must read the HID report-descriptor from the device and tell HID core
   about it via hid_parse_report().

   ::

      int (*power) (struct hid_device *hdev, int level)

   Called by HID core to give PM hints to transport drivers. Usually this is
   analogical to the ->open() and ->close() hints and redundant.

   ::

      void (*request) (struct hid_device *hdev, struct hid_report *report,
		       int reqtype)

   Send a HID request on the ctrl channel. "report" contains the report that
   should be sent and "reqtype" the request type. Request-type can be
   HID_REQ_SET_REPORT or HID_REQ_GET_REPORT.

   This callback is optional. If not provided, HID core will assemble a raw
   report following the HID specs and send it via the ->raw_request() callback.
   The transport driver is free to implement this asynchronously.

   ::

      int (*wait) (struct hid_device *hdev)

   Used by HID core before calling ->request() again. A transport driver can use
   it to wait for any pending requests to complete if only one request is
   allowed at a time.

   ::

      int (*raw_request) (struct hid_device *hdev, unsigned char reportnum,
                          __u8 *buf, size_t count, unsigned char rtype,
                          int reqtype)

   Same as ->request() but provides the report as raw buffer. This request shall
   be synchronous. A transport driver must not use ->wait() to complete such
   requests. This request is mandatory and hid core will reject the device if
   it is missing.

   ::

      int (*output_report) (struct hid_device *hdev, __u8 *buf, size_t len)

   Send raw output report via intr channel. Used by some HID device drivers
   which require high throughput for outgoing requests on the intr channel. This
   must not cause SET_REPORT calls! This must be implemented as asynchronous
   output report on the intr channel!

   ::

      int (*idle) (struct hid_device *hdev, int report, int idle, int reqtype)

   Perform SET/GET_IDLE request. Only used by USB-HID, do not implement!

```

### 2.3) 鏁版嵁璺緞

浼犺緭椹卞姩璐熻矗浠?I/O 璁惧璇诲彇鏁版嵁銆傚畠浠繀椤昏嚜琛屽鐞嗕换浣曚笌 I/O 鐩稿叧鐨勭姸鎬佽窡韪€侶ID core 涓嶅疄鐜板崗璁彙鎵嬫垨鍏朵粬绠＄悊鍛戒护锛岃€岃繖绫诲懡浠ゅ彲鑳芥槸缁欏畾 HID 浼犺緭瑙勮寖鎵€瑕佹眰鐨勩€?
浠庤澶囪鍙栧埌鐨勬瘡涓師濮嬫暟鎹寘閮藉繀椤婚€氳繃 hid_input_report() 閫佸叆 HID core銆備綘蹇呴』鎸囧畾閫氶亾绫诲瀷锛坕ntr 鎴?ctrl锛変互鍙婃姤鍛婄被鍨嬶紙input/output/feature锛夈€傛甯告儏鍐典笅锛岄€氳繃璇?API 鎻愪緵鐨勫彧鏈?input 鎶ュ憡銆?
缁忕敱 ->request() 鍙戝嚭鐨?GET_REPORT 璇锋眰鐨勫簲绛斾篃蹇呴』閫氳繃璇?API 鎻愪緵銆傝€?->raw_request() 鐨勫簲绛旀槸鍚屾鐨勶紝蹇呴』鐢变紶杈撻┍鍔ㄦ嫤鎴紝涓嶅緱浼犻€掔粰 hid_input_report()銆?瀵?SET_REPORT 璇锋眰鐨勭‘璁わ紝HID core 骞朵笉鍏冲績銆?
----------------------------------------------------

Written 2013, David Herrmann <dh.herrmann@gmail.com>
