## TTY 椹卞姩涓?TTY 鎿嶄綔


## 鍒嗛厤

椹卞姩棣栧厛瑕佸仛鐨勬槸鍒嗛厤涓€涓?struct tty_driver銆傝繖閫氳繃 tty_alloc_driver()锛堟垨
__tty_alloc_driver()锛夊畬鎴愩€傛帴涓嬫潵锛屾柊鍒嗛厤鐨勭粨鏋勪綋琚～鍏ヤ俊鎭€傚叧浜庡叿浣撳簲濉叆
鍝簺鍐呭锛岃鍙傞槄鏈枃妗ｆ湯灏剧殑 `TTY 椹卞姩鍙傝€僠_銆?
鍒嗛厤渚嬬▼鎺ュ彈涓€涓┍鍔ㄦ渶澶氳兘澶勭悊鐨勮澶囨暟閲忎互鍙婁竴浜涙爣蹇椼€傛爣蹇楀嵆浠?`TTY_DRIVER_`
寮€澶淬€佸湪 `TTY 椹卞姩鏍囧織`_ 涓垪鍑哄苟鎻忚堪鐨勯偅浜涖€?
褰撻┍鍔ㄥ嵆灏嗚閲婃斁鏃讹紝浼氬鍏惰皟鐢?tty_driver_kref_put()銆傚畠浼氶€掑噺寮曠敤璁℃暟锛岃嫢鍑?鍒伴浂鍒欓噴鏀捐椹卞姩銆?
浣滀负鍙傝€冿紝鍒嗛厤涓庨噴鏀惧嚱鏁板湪涓嬮潰璇︾粏璇存槑锛?
   :identifiers: tty_alloc_driver
   :identifiers: __tty_alloc_driver tty_driver_kref_put

### TTY 椹卞姩鏍囧織

涓嬮潰缁欏嚭 tty_alloc_driver()锛堟垨 __tty_alloc_driver()锛夋墍鎺ュ彈鐨勬爣蹇楄鏄庯細

   :identifiers: tty_driver_flag

----

## 娉ㄥ唽

褰撲竴涓?struct tty_driver 琚垎閰嶅苟濉ソ鍐呭鍚庯紝鍙互浣跨敤 tty_register_driver()
杩涜娉ㄥ唽銆傚缓璁湪 tty_alloc_driver() 鐨?flags 涓紶鍏?`TTY_DRIVER_DYNAMIC_DEV`銆?鑻ヤ笉浼犲叆锛屽垯鍦?tty_register_driver() 鏈熼棿浼氬悓鏃舵敞鍐?*鎵€鏈?*璁惧锛屾绫婚┍鍔ㄥ彲
璺宠繃涓嬮潰鍏充簬娉ㄥ唽璁惧鐨勬钀姐€備笉杩?`娉ㄥ唽璁惧`_ 涓殑 struct tty_port 閮ㄥ垎浠嶇劧鐩稿叧銆?
   :identifiers: tty_register_driver tty_unregister_driver

### 娉ㄥ唽璁惧

姣忎釜 TTY 璁惧閮藉簲鐢变竴涓?struct tty_port 鏀拺銆傞€氬父锛孴TY 椹卞姩灏?tty_port 鍐呭祵鍒?璁惧鐨勭鏈夌粨鏋勪腑銆傚叧浜庡鐞?tty_port 鐨勬洿澶氱粏鑺傦紝鍙弬瑙?[tty_port](tty_port)銆傞┍鍔?杩樺缓璁娇鐢?tty_port_get() 鍜?tty_port_put() 杩涜 tty_port 鐨勫紩鐢ㄨ鏁般€傛渶鍚庝竴娆?put 搴斿綋閲婃斁璇?tty_port锛堝寘鎷澶囩殑绉佹湁缁撴瀯锛夈€?
闄ら潪鍦?tty_alloc_driver() 鐨?flags 涓紶鍏ヤ簡 `TTY_DRIVER_DYNAMIC_DEV`锛屽惁鍒?TTY
椹卞姩搴斿綋娉ㄥ唽绯荤粺涓彂鐜扮殑姣忎竴涓澶囷紙鍚庤€呬负鎺ㄨ崘鍋氭硶锛夈€傝繖閫氳繃 tty_register_device()
瀹屾垚锛涙垨鑰咃紝濡傛灉椹卞姩甯屾湜閫氳繃 struct attribute_group 鏆撮湶鏌愪簺淇℃伅锛屽垯浣跨敤
tty_register_device_attr()銆備簩鑰呴兘浼氭敞鍐岀 `index` 涓澶囷紝杩斿洖鍚庤璁惧鍗冲彲琚墦寮€銆?绋嶅悗 `鍏宠仈璁惧涓庣鍙_ 涓繕鎻忚堪浜嗘洿鎺ㄨ崘鐨?tty_port 鍙樹綋銆傜敱椹卞姩鑷绠＄悊绌洪棽绱㈠紩
骞堕€夋嫨姝ｇ‘鐨勯偅涓€涓€俆TY 灞傚彧浼氭嫆缁濇敞鍐屽浜庝紶鍏?tty_alloc_driver() 鏁伴噺鐨勮澶囥€?
褰撹澶囪鎵撳紑鏃讹紝TTY 灞傚垎閰?struct tty_struct 骞跺紑濮嬭皟鐢?:c`tty_driver.ops`
涓殑鎿嶄綔锛屽弬瑙?`TTY 鎿嶄綔鍙傝€僠_銆?
娉ㄥ唽渚嬬▼璇存槑濡備笅锛?
   :identifiers: tty_register_device tty_register_device_attr
        tty_unregister_device

----

### 鍏宠仈璁惧涓庣鍙?
濡傚墠鎵€杩帮紝姣忎釜 TTY 璁惧閮藉簲褰撳垎閰嶄竴涓?struct tty_port銆傛渶杩熷繀椤诲湪
:c`tty_driver.ops.install()` 鏃惰 TTY 灞傜煡鏅撳畠銆傛湁灏戦噺杈呭姪鍑芥暟鐢ㄤ簬**鍏宠仈**涓よ€呫€?鐞嗘兂鎯呭喌涓嬶紝椹卞姩鍦ㄦ敞鍐屾椂浣跨敤 tty_port_register_device() 鎴?tty_port_register_device_attr() 鏉ユ浛浠?tty_register_device() 鍜?tty_register_device_attr()銆傝繖鏍烽┍鍔ㄥ氨鏃犻渶鍏冲績鍚庣画鐨勫叧鑱斻€?
鑻ュ仛涓嶅埌锛岄┍鍔ㄤ粛鍙湪瀹為檯娉ㄥ唽**涔嬪墠**閫氳繃 tty_port_link_device() 灏?tty_port 鍏宠仈鍒?鏌愪釜鐗瑰畾绱㈠紩銆傚鏋滀粛涓嶅悎閫傦紝浣滀负鏈€鍚庣殑鎵嬫锛屽彲浠ュ湪 :c`tty_driver.ops.install`
閽╁瓙涓娇鐢?tty_port_install()銆傚悗鑰呬富瑕佺敤浜?PTY 绛夊唴瀛樹腑璁惧锛屽叾 tty_port 鏄寜闇€
鍒嗛厤鐨勩€?
鍏宠仈渚嬬▼鍦ㄦ璇存槑锛?
   :identifiers: tty_port_link_device tty_port_register_device
        tty_port_register_device_attr

----

## TTY 椹卞姩鍙傝€?
struct tty_driver 鐨勬墍鏈夋垚鍛樺湪姝よ鏄庛€傚繀闇€鐨勬垚鍛樺湪鏈熬娉ㄦ槑銆俿truct tty_operations
鍦ㄩ殢鍚庤鏄庛€?
   :identifiers: tty_driver

----

## TTY 鎿嶄綔鍙傝€?
褰?TTY 琚敞鍐屽悗锛岃繖浜涢┍鍔ㄩ挬瀛愬彲鐢?TTY 灞傝皟鐢細

   :identifiers: tty_operations
