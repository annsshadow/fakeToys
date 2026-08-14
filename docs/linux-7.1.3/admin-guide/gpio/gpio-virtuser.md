
## 铏氭嫙 GPIO 娑堣垂鑰?

铏氭嫙 GPIO 娑堣垂鑰咃紙virtual GPIO consumer锛夋ā鍧楀厑璁哥敤鎴峰疄渚嬪寲铏氭嫙璁惧锛岃繖浜涜澶?浼氳姹?GPIO 骞堕€氳繃 debugfs 鎺у埗鍏惰涓恒€傝櫄鎷熸秷璐硅€呰澶囧彲浠ラ€氳繃璁惧鏍戞垨 configfs
杩涜瀹炰緥鍖栥€?
铏氭嫙娑堣垂鑰呬娇鐢ㄩ潰鍚戦┍鍔ㄧ▼搴忕殑 GPIO API锛屽苟鍏佽閫氳繃鐢ㄦ埛绌洪棿椹卞姩鐨勮嚜鍔ㄥ寲娴嬭瘯鏉ヨ鐩?瀹冦€侴PIO 浣跨敤 `gpiod_get_array()` 璇锋眰锛屽洜姝ゆ垜浠敮鎸佹瘡涓繛鎺?ID 瀵瑰簲澶氫釜 GPIO銆?
### 鍒涘缓 GPIO 娑堣垂鑰?

gpio-consumer 妯″潡娉ㄥ唽浜嗕竴涓悕涓?`'gpio-virtuser'` 鐨?configfs 瀛愮郴缁熴€傛湁鍏?configfs 鏂囦欢绯荤粺鐨勮缁嗕俊鎭紝璇峰弬闃?configfs 鏂囨。銆?
鐢ㄦ埛鍙互鍒涘缓 configfs 缁勫拰鏉＄洰鐨勫眰绾х粨鏋勶紝骞朵慨鏀规墍鏆撮湶灞炴€х殑鍊笺€傛秷璐硅€呬竴鏃﹀疄渚嬪寲锛?璇ュ眰绾х粨鏋勫皢琚浆鎹负閫傚綋鐨勮澶囧睘鎬с€傚叾閫氱敤缁撴瀯濡備笅锛?
**缁勶細** `/config/gpio-virtuser`

杩欐槸 gpio-consumer configfs 鏍戠殑椤跺眰鐩綍銆?
**缁勶細** `/config/gpio-consumer/example-name`

**灞炴€э細** `/config/gpio-consumer/example-name/live`

**灞炴€э細** `/config/gpio-consumer/example-name/dev_name`

杩欐槸涓€涓唬琛?GPIO 娑堣垂鑰呰澶囩殑鐩綍銆?
鍙鐨?`dev_name` 灞炴€ф毚闇茶璁惧鍦ㄧ郴缁熶腑鐨勫悕绉帮紝鍗冲畠浼氬嚭鐜板湪骞冲彴鎬荤嚎涓娿€傝繖瀵逛簬
鍦?`/sys/kernel/debug/gpio-virtuser/$dev_name` 涓嬪畾浣嶅叧鑱旂殑 debugfs 鐩綍寰堟湁鐢ㄣ€?
`'live'` 灞炴€х敤浜庡湪璁惧瀹屽叏閰嶇疆濂藉悗瑙﹀彂鍏跺疄闄呭垱寤恒€傚彲鎺ュ彈鐨勫€间负锛歚'1'` 鐢ㄤ簬鍚敤
铏氭嫙璁惧锛宍'0'` 鐢ㄤ簬绂佺敤骞舵媶闄ゅ畠銆?
### 鍒涘缓 GPIO 鏌ユ壘琛?

鐢ㄦ埛鍙互鍦ㄨ澶囩粍涓嬪垱寤哄涓?configfs 缁勶細

**缁勶細** `/config/gpio-consumer/example-name/con_id`

`'con_id'` 鐩綍浠ｈ〃鍗曚釜 GPIO 鏌ユ壘锛屽叾鍊兼槧灏勫埌 `gpiod_get()` 鍑芥暟鐨?`'con_id'`
鍙傛暟銆備緥濡傦細`con_id` == `'reset'` 鏄犲皠鍒?`reset-gpios` 璁惧灞炴€с€?
鐢ㄦ埛鍙互涓烘瘡涓煡鎵惧垎閰嶅涓?GPIO銆傛瘡涓?GPIO 鏄?`'con_id'` 缁勪笅鐨勪竴涓叿鏈夌敤鎴峰畾涔?鍚嶇О鐨勫瓙鐩綍銆?
**灞炴€э細** `/config/gpio-consumer/example-name/con_id/0/key`

**灞炴€э細** `/config/gpio-consumer/example-name/con_id/0/offset`

**灞炴€э細** `/config/gpio-consumer/example-name/con_id/0/drive`

**灞炴€э細** `/config/gpio-consumer/example-name/con_id/0/pull`

**灞炴€э細** `/config/gpio-consumer/example-name/con_id/0/active_low`

**灞炴€э細** `/config/gpio-consumer/example-name/con_id/0/transitory`

杩欐槸涓€涓弿杩?`con_id-gpios` 灞炴€т腑鍗曚釜 GPIO 鐨勭粍銆?
瀵逛簬浣跨敤 configfs 鍒涘缓鐨勮櫄鎷熸秷璐硅€咃紝鎴戜滑浣跨敤鏈哄櫒鏌ユ壘琛紝鍥犳鍙互灏嗚缁勮涓烘枃浠剁郴缁?涓?`'struct gpiod_lookup'` 涓崟涓潯鐩殑瀛楁涔嬮棿鐨勬槧灏勩€?
`'key'` 灞炴€т唬琛ㄨ GPIO 鎵€灞炵殑鑺墖鍚嶇О鎴?GPIO 绾胯矾鍚嶇О銆傝繖鍙栧喅浜?`'offset'` 灞炴€?鐨勫€硷細濡傛灉鍏跺€?>= 0锛屽垯 `'key'` 浠ｈ〃瑕佹煡鎵剧殑鑺墖鏍囩锛岃€?`'offset'` 浠ｈ〃璇ヨ姱鐗囦腑
绾胯矾鐨勫亸绉婚噺銆傚鏋?`'offset'` 涓?< 0锛屽垯 `'key'` 浠ｈ〃绾胯矾鐨勫悕绉般€?
鍏朵綑灞炴€ф槧灏勫埌 GPIO 鏌ユ壘缁撴瀯鐨?`'flags'` 瀛楁銆傚墠涓や釜鎺ュ彈瀛楃涓插€间綔涓哄弬鏁帮細

**`'drive'`锛?* `'push-pull'`銆乣'open-drain'`銆乣'open-source'`
**`'pull'`锛?* `'pull-up'`銆乣'pull-down'`銆乣'pull-disabled'`銆乣'as-is'`

`'active_low'` 鍜?`'transitory'` 鏄竷灏斿睘鎬с€?
### 婵€娲?GPIO 娑堣垂鑰?

閰嶇疆瀹屾垚鍚庯紝蹇呴』灏?`'live'` 灞炴€ц缃负 1 浠ュ疄渚嬪寲娑堣垂鑰呫€傚皢鍏惰鍥?0 鍙攢姣佽櫄鎷熻澶囥€?妯″潡灏嗗悓姝ョ瓑寰呮柊妯℃嫙鐨勮澶囪鎴愬姛鎺㈡祴锛屽鏋滄湭鍙戠敓锛屽啓鍏?`'live'` 灏嗗鑷撮敊璇€?
### 璁惧鏍?

铏氭嫙 GPIO 娑堣垂鑰呬篃鍙互鍦ㄨ澶囨爲涓畾涔夈€傚叾鍏煎瀛楃涓插繀椤讳负锛歚"gpio-virtuser"`锛?骞惰嚦灏戞湁涓€涓伒寰爣鍑嗗寲 GPIO 妯″紡鐨勫睘鎬с€?
涓€涓畾涔夎櫄鎷?GPIO 娑堣垂鑰呯殑璁惧鏍戜唬鐮佺ず渚嬶細


    gpio-virt-consumer {
        compatible = "gpio-virtuser";

        foo-gpios = <&gpio0 5 GPIO_ACTIVE_LOW>, <&gpio1 2 0>;
        bar-gpios = <&gpio0 6 0>;
    };

### 鎺у埗铏氭嫙 GPIO 娑堣垂鑰?

婵€娲诲悗锛岃澶囧皢瀵煎嚭 debugfs 灞炴€э紝鐢ㄤ簬鎺у埗 GPIO 鏁扮粍浠ュ強姣忎釜鍗曠嫭璇锋眰鐨?GPIO 绾胯矾銆?鎴戜滑鏉ヨ€冭檻浠ヤ笅璁惧灞炴€э細`foo-gpios = <&gpio0 0 0>, <&gpio0 4 0>;`銆?
灏嗗垱寤轰互涓?debugfs 灞炴€х粍锛?
**缁勶細** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo/`

杩欐槸鍖呭惈鏁翠釜 GPIO 鏁扮粍灞炴€х殑缁勩€?
**灞炴€э細** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo/values`

**灞炴€э細** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo/values_atomic`

杩欎袱涓睘鎬ч兘鍏佽璇诲彇鍜岃缃?GPIO 鍊兼暟缁勩€傜敤鎴峰繀椤讳互瀛楃涓插舰寮忎紶鍏ユ暟缁勬墍鍖呭惈鏁伴噺鐨?鍊硷紝瀛楃涓茬敱浠ｈ〃闈炴縺娲诲拰婵€娲?GPIO 鐘舵€佺殑闆跺拰涓€缁勬垚銆傚湪鏈緥涓細`echo 11 > values`銆?
`values_atomic` 灞炴€х殑宸ヤ綔鏂瑰紡涓?`values` 鐩稿悓锛屼絾鍐呮牳灏嗗湪涓柇涓婁笅鏂囦腑鎵ц GPIO
椹卞姩鍥炶皟銆?
**缁勶細** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/`

杩欐槸涓€涓唬琛ㄥ崟涓?GPIO 鐨勭粍锛宍$index` 鏄叾鍦ㄦ暟缁勪腑鐨勫亸绉婚噺銆?
**灞炴€э細** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/consumer`

鍏佽璁剧疆鍜岃鍙?GPIO 绾胯矾鐨勬秷璐硅€呮爣绛俱€?
**灞炴€э細** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/debounce`

鍏佽璁剧疆鍜岃鍙?GPIO 绾胯矾鐨勫幓鎶栧懆鏈熴€?
**灞炴€э細** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/direction`

**灞炴€э細** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/direction_atomic`

杩欎袱涓睘鎬у厑璁歌缃?GPIO 绾胯矾鐨勬柟鍚戙€傚畠浠帴鍙?`"input"` 鍜?`"output"` 浣滀负鍊笺€?鍘熷瓙鍙樹綋鍦ㄤ腑鏂笂涓嬫枃涓墽琛岄┍鍔ㄥ洖璋冦€?
**灞炴€э細** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/interrupts`

濡傛灉璇ョ嚎璺互杈撳叆妯″紡璇锋眰锛屽悜璇ュ睘鎬у啓鍏?`1` 灏嗕娇妯″潡鐩戝惉璇?GPIO 涓婄殑杈规部涓柇銆?鍐欏叆 `0` 灏嗙鐢ㄧ洃瑙嗐€傝鍙栬灞炴€ц繑鍥炲綋鍓嶅凡娉ㄥ唽鐨勪腑鏂暟锛堜袱涓竟娌匡級銆?
**灞炴€э細** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/value`

**灞炴€э細** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/value_atomic`

杩欎袱涓睘鎬ч兘鍏佽璇诲彇鍜岃缃悇涓凡璇锋眰 GPIO 绾胯矾鐨勫€笺€傚畠浠帴鍙椾互涓嬪€硷細`1` 鍜?`0`銆?