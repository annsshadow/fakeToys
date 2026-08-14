
## WMI 椹卞姩寮€鍙戞寚鍗?


WMI 瀛愮郴缁熶负瀹炵幇 WMI 椹卞姩鎻愪緵浜嗕竴濂椾赴瀵岀殑椹卞姩 API锛岀浉鍏虫枃妗ｄ綅浜?Documentation/driver-api/wmi.rst銆傛湰鏂囨。灏嗕綔涓轰娇鐢ㄦ API 缂栧啓 WMI 椹卞姩鐨勫叆闂ㄦ寚鍗椼€傚畠鏄師濮?LWN 鏂囩珷 [^1^]_ 鐨勭画绡囷紝閭ｇ瘒鏂囩珷璁ㄨ鐨勬槸浣跨敤宸插簾寮冪殑鍩轰簬 GUID 鐨?WMI 鎺ュ彛鐨?WMI 椹卞姩銆?

### 鑾峰彇 WMI 璁惧淇℃伅


鍦ㄥ紑鍙?WMI 椹卞姩涔嬪墠锛屽繀椤诲厛鑾峰彇鐩稿叧 WMI 璁惧鐨勪俊鎭€傚彲浠ヤ娇鐢?`lswmi <https://pypi.org/project/lswmi>`_ 宸ュ叿閫氳繃浠ヤ笅鍛戒护鎻愬彇璇︾粏鐨?WMI 璁惧淇℃伅锛?

```

  lswmi -V

```
寰楀埌鐨勮緭鍑哄皢鍖呭惈缁欏畾鏈哄櫒涓婃墍鏈夊彲鐢?WMI 璁惧鐨勪俊鎭紝浠ュ強涓€浜涢澶栦俊鎭€?

涓轰簡杩涗竴姝ヤ簡瑙ｇ敤浜庝笌 WMI 璁惧閫氫俊鐨勬帴鍙ｏ紝鍙互浣跨敤 `bmfdec <https://github.com/pali/bmfdec>`_ 宸ュ叿鏉ヨВ鐮佺敤浜庢弿杩?WMI 璁惧鐨勪簩杩涘埗 MOF锛圡anaged Object Format锛屾墭绠″璞℃牸寮忥級淇℃伅銆?
`wmi-bmof` 椹卞姩灏嗘淇℃伅鏆撮湶缁欑敤鎴风┖闂达紝鍙傝
Documentation/wmi/devices/wmi-bmof.rst銆?

瑕佽幏鍙栬В鐮佸悗鐨勪簩杩涘埗 MOF 淇℃伅锛岃浣跨敤浠ヤ笅鍛戒护锛堥渶瑕?root 鏉冮檺锛夛細

```

  ./bmf2mof /sys/bus/wmi/devices/05901221-D566-11D1-B2F0-00A0C9062910[-X]/bmof

```
鏈夋椂锛屾煡鐪嬬敤浜庢弿杩?WMI 璁惧鐨勫弽姹囩紪 ACPI 琛紝鏈夊姪浜庣悊瑙?WMI 璁惧搴斿綋濡備綍宸ヤ綔銆備笌缁欏畾 WMI 璁惧鍏宠仈鐨?ACPI 鏂规硶鐨勮矾寰勶紝鍙互浣跨敤涓婇潰鎻愬埌鐨?`lswmi` 宸ュ叿鑾峰彇銆?

濡傛灉鎮ㄦ灏濊瘯灏嗘煇涓┍鍔ㄧЩ妞嶅埌 Linux锛屽苟涓旀槸鍦?Windows 绯荤粺涓婂伐浣滐紝閭ｄ箞 `WMIExplorer <https://github.com/vinaypamnani/wmie2>`_ 宸ュ叿浼氬緢鏈夌敤锛屽畠鍙互妫€鏌ュ彲鐢ㄧ殑 WMI 鏂规硶骞剁洿鎺ヨ皟鐢ㄥ畠浠€?

### 鍩烘湰 WMI 椹卞姩缁撴瀯


鍩烘湰鐨?WMI 椹卞姩鍥寸粫 struct wmi_driver 鏋勫缓锛岀劧鍚庨€氳繃涓€涓?struct wmi_device_id 琛ㄧ粦瀹氬埌鍖归厤鐨?WMI 璁惧锛?

```

  static const struct wmi_device_id foo_id_table[] = {
         /* Only use uppercase letters! */
         { "936DA01F-9ABD-4D9D-80C7-02AF85C822A8", NULL },
         { }
  };
  MODULE_DEVICE_TABLE(wmi, foo_id_table);

  static struct wmi_driver foo_driver = {
        .driver = {
                .name = "foo",
                .probe_type = PROBE_PREFER_ASYNCHRONOUS,        /* recommended */
                .pm = pm_sleep_ptr(&foo_dev_pm_ops),            /* optional */
        },
        .id_table = foo_id_table,
        .probe = foo_probe,
        .remove = foo_remove,         /* optional, devres is preferred */
        .shutdown = foo_shutdown,     /* optional, called during shutdown */
        .notify_new = foo_notify,     /* optional, for event handling */
        .min_event_size = X,          /* optional, simplifies event payload size verification */
        .no_singleton = true,         /* required for new WMI drivers */
  };
  module_wmi_driver(foo_driver);

```
褰?WMI 椹卞姩缁戝畾鍒颁竴涓尮閰嶇殑 WMI 璁惧鏃讹紝浼氳皟鐢?probe() 鍥炶皟銆傞€氬父搴旇鍦ㄨ繖涓嚱鏁颁腑
鍒嗛厤椹卞姩鐗瑰畾鐨勬暟鎹粨鏋勫苟鍒濆鍖栧埌鍏朵粬鍐呮牳瀛愮郴缁熺殑鎺ュ彛銆?

褰?WMI 椹卞姩浠庢煇涓?WMI 璁惧瑙ｇ粦鏃讹紝浼氳皟鐢?remove() 鍥炶皟銆備负浜嗘敞閿€鍒板叾浠栧唴鏍稿瓙绯荤粺鐨勬帴鍙?
骞堕噴鏀捐祫婧愶紝搴旇浣跨敤 devres銆傝繖鍙互绠€鍖?probe 鏈熼棿鐨勯敊璇鐞嗭紝骞朵笖閫氬父鍙互瀹屽叏鐪佺暐璇ュ洖璋冿紝
璇﹁ Documentation/driver-api/driver-model/devres.rst銆?

shutdown() 鍥炶皟鍦ㄥ叧鏈恒€侀噸鍚垨 kexec 鏈熼棿琚皟鐢ㄣ€傚畠鐨勫敮涓€鐩殑鏄鐢?WMI 璁惧锛屽苟灏嗗叾缃簬涓€涓?
宸茬煡鐨勭姸鎬侊紝浠ヤ究 WMI 椹卞姩鍦ㄩ噸鍚垨 kexec 涔嬪悗鑳藉閲嶆柊鎺ョ銆傚ぇ澶氭暟 WMI 椹卞姩涓嶉渶瑕佺壒娈婄殑鍏虫満澶勭悊锛?
鍥犳鍙互鐪佺暐璇ュ洖璋冦€?

璇锋敞鎰忥紝鏂扮殑 WMI 椹卞姩蹇呴』鑳藉琚娆″疄渚嬪寲锛屽苟涓旂姝娇鐢ㄤ换浣曞凡搴熷純鐨勫熀浜?GUID 鎴栧熀浜?ACPI 鐨?
WMI 鍑芥暟銆傝繖鎰忓懗鐫€ WMI 椹卞姩搴旇涓虹粰瀹氭満鍣ㄤ笂瀛樺湪澶氫釜鍖归厤 WMI 璁惧鐨勫満鏅仛濂藉噯澶囥€?

鍥犳锛學MI 椹卞姩搴旇浣跨敤 Documentation/driver-api/driver-model/design-patterns.rst 涓弿杩扮殑
鐘舵€佸鍣紙state container锛夎璁℃ā寮忋€?

             鍦ㄥ悓涓€璁惧涓婂悓鏃跺鐞?WMI 浜嬩欢蹇呯劧浼氬鑷?WMI 璁惧鐘舵€佹崯鍧忥紝骞跺彲鑳藉紩鍙戝紓甯歌涓恒€?

### WMI 鏂规硶椹卞姩


WMI 椹卞姩鍙互浣跨敤 wmidev_invoke_method() 璋冪敤 WMI 璁惧鏂规硶銆傚浜庢瘡娆?WMI 鏂规硶璋冪敤锛學MI 椹卞姩
闇€瑕佹彁渚涘疄渚嬪彿鍜屾柟娉?ID锛屼互鍙婂寘鍚柟娉曞弬鏁扮殑缂撳啿鍖猴紝杩樺彲閫夊湴鎻愪緵涓€涓敤浜庡瓨鏀剧粨鏋滅殑缂撳啿鍖恒€?
褰撹皟鐢ㄤ笉杩斿洖浠讳綍鍊肩殑 WMI 鏂规硶鏃讹紝搴旇鏀圭敤 wmidev_invoke_procedure()銆?

涓婅堪缂撳啿鍖虹殑甯冨眬鏄澶囩壒瀹氱殑锛岀敱涓庣粰瀹?WMI 璁惧鍏宠仈鐨勪簩杩涘埗 MOF 鏁版嵁鎻忚堪銆傝浜岃繘鍒?MOF 鏁版嵁
杩樹娇鐢?`WmiMethodId` 闄愬畾绗︽弿杩扮粰瀹?WMI 鏂规硶鐨勬柟娉?ID銆傛毚闇?WMI 鏂规硶鐨?WMI 璁惧閫氬父鍙毚闇插崟涓?
瀹炰緥锛堝疄渚嬪彿 0锛夛紝浣嗙悊璁轰笂涔熷彲浠ユ毚闇插涓疄渚嬨€傚湪杩欑鎯呭喌涓嬶紝鍙互浣跨敤 wmidev_instance_count()
鑾峰彇瀹炰緥鐨勬暟閲忋€?

鏈夊叧 WMI 鏂规硶椹卞姩鐨勭ず渚嬶紝璇峰弬闃?drivers/platform/x86/intel/wmi/thunderbolt.c銆?

### WMI 鏁版嵁鍧楅┍鍔?


WMI 椹卞姩鍙互浣跨敤 wmidev_query_block() 鏌ヨ WMI 鏁版嵁鍧楋紝杩斿洖缂撳啿鍖虹殑甯冨眬鍚屾牱鏄澶囩壒瀹氱殑锛?
骞剁敱浜岃繘鍒?MOF 鏁版嵁鎻忚堪銆備竴浜?WMI 鏁版嵁鍧椾篃鏄彲鍐欑殑锛屽彲浠ヤ娇鐢?wmidev_set_block() 璁剧疆銆傛暟鎹潡
瀹炰緥鐨勬暟閲忓悓鏍峰彲浠ヤ娇鐢?wmidev_instance_count() 鑾峰彇銆?

鏈夊叧 WMI 鏁版嵁鍧楅┍鍔ㄧ殑绀轰緥锛岃鍙傞槄 drivers/platform/x86/intel/wmi/sbl-fw-update.c銆?

### WMI 浜嬩欢椹卞姩


WMI 椹卞姩鍙互閫氳繃 struct wmi_driver 鍐呴儴鐨?notify_new() 鍥炶皟鎺ユ敹 WMI 浜嬩欢銆傞殢鍚?WMI 瀛愮郴缁熶細
璐熻矗鐩稿簲鍦拌缃 WMI 浜嬩欢銆傝娉ㄦ剰锛屼紶閫掔粰姝ゅ洖璋冪殑缂撳啿鍖哄竷灞€鏄澶囩壒瀹氱殑锛屽苟涓旂紦鍐插尯鐨勯噴鏀?
鐢?WMI 瀛愮郴缁熻嚜韬畬鎴愶紝鑰屼笉鏄敱椹卞姩瀹屾垚銆?

WMI 椹卞姩鏍稿績浼氱‘淇?notify_new() 鍥炶皟鍙湪 probe() 鍥炶皟琚皟鐢ㄤ箣鍚庢墠浼氳璋冪敤锛屽苟涓斿湪璋冪敤椹卞姩
鐨?remove() 鎴?shutdown() 鍥炶皟鐨勫墠鍚庝笉浼氭敹鍒颁换浣曚簨浠躲€?

涓嶈繃锛學MI 椹卞姩寮€鍙戣€呭簲璇ユ剰璇嗗埌锛屽涓?WMI 浜嬩欢鍙兘浼氳骞跺彂鎺ユ敹锛屽洜姝や换浣曞繀瑕佺殑鍔犻攣閮介渶瑕佺敱
WMI 椹卞姩鑷韩鎻愪緵銆?

WMI 椹卞姩杩樺彲浠ラ€氳繃濉厖 struct wmi_driver 涓殑 `min_event_size` 瀛楁锛屾寚绀?WMI 椹卞姩鏍稿績鑷姩
鎷掔粷鍖呭惈杩囧皬浜嬩欢璐熻浇鐨?WMI 浜嬩欢銆傚洜姝わ紝灏嗚瀛楁璁句负 0 灏嗕娇 WMI 椹卞姩鑳藉鎺ユ敹涓嶅甫浠讳綍浜嬩欢璐熻浇鐨?
WMI 浜嬩欢銆?

鏈夊叧 WMI 浜嬩欢椹卞姩鐨勭ず渚嬶紝璇峰弬闃?drivers/platform/x86/xiaomi-wmi.c銆?

### 涓?WMI 椹卞姩鏍稿績浜ゆ崲鏁版嵁


WMI 椹卞姩鍙互浣跨敤 struct wmi_buffer 涓?WMI 椹卞姩鏍稿績浜ゆ崲鏁版嵁銆傝繖浜涚紦鍐插尯鐨勫唴閮ㄧ粨鏋勬槸璁惧鐗瑰畾鐨勶紝
鍙湁 WMI 椹卞姩鎵嶇煡閬撱€傚洜姝わ紝WMI 椹卞姩鑷韩璐熻矗瑙ｆ瀽鍜屾牎楠屼粠鍏?WMI 璁惧鎺ユ敹鍒扮殑鏁版嵁銆?

涓婅堪缂撳啿鍖虹殑缁撴瀯鐢辩浉鍏?WMI 璁惧鐨?MOF 鏁版嵁鎻忚堪銆傚綋杩欐牱鐨勭紦鍐插尯鍖呭惈澶氫釜鏁版嵁椤规椂锛岄€氬父瀹氫箟涓€涓?
C 缁撴瀯骞跺湪瑙ｆ瀽鏃朵娇鐢ㄥ畠鏄悎鐞嗙殑銆傜敱浜?WMI 椹卞姩鏍稿績淇濊瘉浠?WMI 璁惧鎺ユ敹鍒扮殑鎵€鏈夌紦鍐插尯閮芥寜 8 瀛楄妭
杈圭晫瀵归綈锛學MI 椹卞姩鍙互绠€鍗曞湴杩涜 WMI 缂撳啿鍖烘暟鎹笌杩欎釜 C 缁撴瀯涔嬮棿鐨勮浆鎹紙cast锛夈€?

涓嶈繃锛岃繖鍙湁鍦ㄧ紦鍐插尯鐨勫昂瀵歌楠岃瘉涓鸿冻浠ュ绾虫暣涓?C 缁撴瀯涔嬪悗鎵嶅簲杩涜銆俉MI 椹卞姩搴旇鎷掔粷杩囧皬鐨勭紦鍐插尯锛?
鍥犱负瀹冧滑閫氬父鏄?WMI 璁惧鐢ㄦ潵鍙戝嚭鍐呴儴閿欒淇″彿鐨勩€備絾杩囧ぇ鐨勭紦鍐插尯搴旇琚帴鍙楋紝浠ユā鎷?Windows WMI
瀹炵幇鐨勮銆?

鍦ㄤ负瑙ｆ瀽 WMI 缂撳啿鍖哄畾涔?C 缁撴瀯鏃讹紝搴旇灏婇噸鏁版嵁椤圭殑瀵归綈鏂瑰紡銆傝繖瀵逛簬 64 浣嶆暣鏁板挨涓洪噸瑕侊紝鍥犱负
瀹冧滑鍦?64 浣嶏紙8 瀛楄妭瀵归綈锛夊拰 32 浣嶏紙4 瀛楄妭瀵归綈锛夋灦鏋勪笂鍏锋湁涓嶅悓鐨勫榻愭柟寮忋€傚洜姝わ紝鎵嬪姩鎸囧畾姝ょ被
鏁版嵁椤圭殑瀵归綈鏂瑰紡锛屾垨鍦ㄩ€傚綋鏃跺皢鏁翠釜缁撴瀯鏍囪涓?packed 鏄釜濂戒富鎰忋€傛暣鏁版暟鎹」涓€鑸槸鏃犵鍙峰皬绔暣鏁帮紝
搴旇浣跨敤 `__le64` 绛夌被鍨嬫樉寮忔爣璁般€傝В鏋?WMI 瀛楃涓叉暟鎹」鏃跺簲浣跨敤 struct wmi_string锛屽洜涓?WMI 瀛楃涓?
鐨勫竷灞€涓?C 瀛楃涓蹭笉鍚屻€?

鏈夊叧 WMI 鏁版嵁椤逛簩杩涘埗鏍煎紡鐨勬洿澶氫俊鎭紝璇峰弬闃?Documentation/wmi/acpi-interface.rst銆?

### 涓€娆℃€у鐞嗗涓?WMI 璁惧


鍥轰欢鍘傚晢浣跨敤澶氫釜 WMI 璁惧鏉ユ帶鍒跺崟涓墿鐞嗚澶囩殑涓嶅悓鏂归潰鐨勬儏鍐靛緢澶氥€傝繖鍙兘浣?WMI 椹卞姩鐨勫紑鍙戝彉寰楀鏉傦紝
鍥犱负杩欎簺椹卞姩鍙兘闇€瑕佺浉浜掗€氫俊锛屼互鍚戠敤鎴风┖闂村憟鐜扮粺涓€鐨勬帴鍙ｃ€?

鍏朵腑涓€绉嶆儏鍐垫秹鍙婁竴涓?WMI 浜嬩欢璁惧锛屽畠闇€瑕佸湪鏀跺埌 WMI 浜嬩欢鏃朵笌涓€涓?WMI 鏁版嵁鍧楄澶囨垨 WMI 鏂规硶璁惧
閫氫俊銆傚湪杩欑鎯呭喌涓嬶紝搴旇寮€鍙戜袱涓?WMI 椹卞姩锛屼竴涓敤浜?WMI 浜嬩欢璁惧锛屽彟涓€涓敤浜庡彟涓€涓?WMI 璁惧銆?

WMI 浜嬩欢璁惧椹卞姩鍙湁涓€涓洰鐨勶細鎺ユ敹 WMI 浜嬩欢銆佹牎楠屼换浣曢檮鍔犵殑浜嬩欢鏁版嵁骞惰皟鐢ㄤ竴涓€氱煡閾撅紙notifier chain锛夈€?
鍙︿竴涓?WMI 椹卞姩鍦ㄦ帰娴嬫湡闂村皢鑷繁鍔犲叆杩欎釜閫氱煡閾撅紝浠庤€屾瘡娆℃敹鍒?WMI 浜嬩欢鏃堕兘浼氬緱鍒伴€氱煡銆傝繖涓?WMI 椹卞姩
闅忓悗鍙互杩涗竴姝ュ鐞嗚浜嬩欢锛屼緥濡傞€氳繃浣跨敤涓€涓緭鍏ヨ澶囥€?

瀵逛簬鍏朵粬 WMI 璁惧鐨勭粍鍚堬紝涔熷彲浠ヤ娇鐢ㄧ被浼肩殑鏈哄埗銆?

### 闇€瑕侀伩鍏嶇殑浜嬮」


寮€鍙?WMI 椹卞姩鏃讹紝鏈夊嚑浠朵簨搴旇閬垮厤锛?

- 浣跨敤宸插簾寮冪殑鍩轰簬 GUID 鐨?WMI 鎺ュ彛锛屽畠浣跨敤 GUID 鑰屼笉鏄?WMI 璁惧缁撴瀯浣?
- 浣跨敤宸插簾寮冪殑鍩轰簬 ACPI 鐨?WMI 鎺ュ彛锛屽畠浣跨敤 ACPI 瀵硅薄鑰屼笉鏄櫘閫氱紦鍐插尯
- 鍦ㄤ笌 WMI 璁惧閫氫俊鏃剁粫杩?WMI 瀛愮郴缁?
- 鏃犳硶琚娆″疄渚嬪寲鐨?WMI 椹卞姩

璁稿杈冩棫鐨?WMI 椹卞姩杩濆弽浜嗘鍒楄〃涓殑涓€鏉℃垨澶氭潯銆傚師鍥犳槸 WMI 瀛愮郴缁熷湪杩囧幓浜屽崄骞翠腑鍙戠敓浜嗘樉钁楁紨杩涳紝
鍥犳杈冩棫鐨?WMI 椹卞姩涓瓨鍦ㄥぇ閲忓巻鍙查仐鐣欑殑绯熺矔銆?

鏂扮殑 WMI 椹卞姩杩橀渶瑕佺鍚?Documentation/process/coding-style.rst 涓瀹氱殑 Linux 鍐呮牳浠ｇ爜椋庢牸銆?
checkpatch 宸ュ叿鍙互鎹曡幏璁稿甯歌鐨勪唬鐮侀鏍艰繚瑙勶紝鎮ㄥ彲浠ヤ娇鐢ㄤ互涓嬪懡浠よ皟鐢ㄥ畠锛?

```

  ./scripts/checkpatch.pl --strict <path to driver file>

```
## 鍙傝€冩枃妗?


