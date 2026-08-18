
## GPIO 鑱氬悎鍣?
GPIO 鑱氬悎鍣ㄦ彁渚涗簡涓€绉嶅皢 GPIO 鑱氬悎锛屽苟浠ヤ竴涓柊鐨?gpio_chip 鏆撮湶鍑烘潵鐨勬満鍒躲€傚畠鏀寔
浠ヤ笅浣跨敤鍦烘櫙銆?
### 浣跨敤 Sysfs 鑱氬悎 GPIO

GPIO 鎺у埗鍣ㄩ€氳繃 /dev/gpiochip* 瀛楃璁惧瀵煎嚭鍒扮敤鎴风┖闂淬€傚杩欎簺璁惧鐨勮闂帶鍒剁敱
鏍囧噯 UNIX 鏂囦欢绯荤粺鏉冮檺鎻愪緵锛岄噰鐢?鍏ㄦ湁鎴栧叏鏃?鐨勬柟寮忥細瑕佷箞鏌愪釜 GPIO 鎺у埗鍣ㄥ鏌愪釜
鐢ㄦ埛鍙闂紝瑕佷箞涓嶅彲璁块棶銆?
GPIO 鑱氬悎鍣ㄩ€氳繃灏嗕竴缁勪竴涓垨澶氫釜 GPIO 鑱氬悎涓轰竴涓柊鐨?gpio_chip锛屾潵涓鸿繖缁?GPIO
鎻愪緵璁块棶鎺у埗锛岃 gpio_chip 鍙互浣跨敤鏍囧噯 UNIX 鏂囦欢鎷ユ湁鑰呭拰鏉冮檺鍒嗛厤缁欐煇涓粍鎴栫敤鎴枫€?姝ゅ锛岃繖涔熺畝鍖栧苟鍔犲浐浜嗗皢 GPIO 瀵煎嚭鍒拌櫄鎷熸満鐨勮繃绋嬶紝鍥犱负铏氭嫙鏈哄彧闇€鑾峰彇鏁翠釜 GPIO
鎺у埗鍣紝鑰屼笉鍐嶉渶瑕佸叧蹇冭鑾峰彇鍝簺 GPIO銆佷笉鑾峰彇鍝簺 GPIO锛屼粠鑰屽噺灏忎簡鏀诲嚮闈€?
鑱氬悎鐨?GPIO 鎺у埗鍣ㄩ€氳繃鍐欏叆 sysfs 涓殑鍙啓灞炴€ф枃浠舵潵瀹炰緥鍖栧拰閿€姣併€?
    /sys/bus/platform/drivers/gpio-aggregator/

	"new_device" ...
		鐢ㄦ埛绌洪棿鍙互閫氳繃鍚?"new_device" 鏂囦欢鍐欏叆涓€涓弿杩拌鑱氬悎鐨?		GPIO 鐨勫瓧绗︿覆锛屾潵璇锋眰鍐呮牳瀹炰緥鍖栦竴涓仛鍚堢殑 GPIO 鎺у埗鍣紝
		鏍煎紡濡備笅锛?
		.. code-block:: none

		    [<gpioA>] [<gpiochipB> <offsets>] ...

		鍏朵腑锛?
		    "<gpioA>" ...
			    鏄竴涓?GPIO 绾挎潯鍚嶇О锛?
		    "<gpiochipB>" ...
			    鏄竴涓?GPIO 鑺墖鏍囩锛屽苟涓?
		    "<offsets>" ...
			    鏄竴涓互閫楀彿鍒嗛殧鐨?GPIO 鍋忕Щ鍜?鎴栫敱鐭í绾胯〃绀虹殑
			    GPIO 鍋忕Щ鑼冨洿鍒楄〃銆?
		Example: 閫氳繃鑱氬悎 "e6052000.gpio" 鐨?GPIO 绾挎潯 19 浠ュ強
		"e6050000.gpio" 鐨?GPIO 绾挎潯 20-21锛屽疄渚嬪寲涓€涓柊鐨?GPIO
		鑱氬悎鍣紝鎴愪负涓€涓柊鐨?gpio_chip锛?
		.. code-block:: sh

		    $ echo 'e6052000.gpio 19 e6050000.gpio 20-21' > new_device

	"delete_device" ...
		鐢ㄦ埛绌洪棿鍙互閫氳繃灏嗗叾璁惧鍚嶇О鍐欏叆 "delete_device" 鏂囦欢锛?		鏉ヨ姹傚唴鏍稿湪浣跨敤鍚庨攢姣佷竴涓仛鍚堢殑 GPIO 鎺у埗鍣ㄣ€?
		Example: 閿€姣佸厛鍓嶅垱寤虹殑鑱氬悎 GPIO 鎺у埗鍣紝鍋囧畾鍏跺悕涓?		"gpio-aggregator.0"锛?
		.. code-block:: sh

		    $ echo gpio-aggregator.0 > delete_device


### 浣跨敤 Configfs 鑱氬悎 GPIO

**Group:** `/config/gpio-aggregator`

    杩欐槸 gpio-aggregator configfs 鏍戠殑鏍圭洰褰曘€?
**Group:** `/config/gpio-aggregator/<example-name>`

    璇ョ洰褰曡〃绀轰竴涓?GPIO 鑱氬悎鍣ㄨ澶囥€備綘鍙互涓?`<example-name>` 鎸囧畾浠绘剰鍚嶇О
    锛堜緥濡?`agg0`锛夛紝浣嗕互 `_sysfs` 鍓嶇紑寮€澶寸殑鍚嶇О闄ゅ锛岃繖浜涘悕绉颁繚鐣欑粰閫氳繃
    Sysfs 鍒涘缓鐨勮澶囩殑鑷姩鐢熸垚鐨?configfs 鏉＄洰浣跨敤銆?
**Attribute:** `/config/gpio-aggregator/<example-name>/live`

    `live` 灞炴€у厑璁稿湪璁惧瀹屽叏閰嶇疆濂藉悗瑙﹀彂鍏跺疄闄呭垱寤恒€傚彲鎺ュ彈鐨勫€间负锛?
    - `1`, `yes`, `true` : 鍚敤铏氭嫙璁惧
    - `0`, `no`, `false` : 绂佺敤铏氭嫙璁惧

**Attribute:** `/config/gpio-aggregator/<example-name>/dev_name`

    鍙鐨?`dev_name` 灞炴€ф毚闇茶璁惧鐨勫悕绉帮紝瀹冨皢鍑虹幇鍦ㄥ钩鍙颁笂锛堜緥濡?    `gpio-aggregator.0`锛夈€傝繖瀵逛簬璇嗗埆鏂板垱寤虹殑鑱氬悎鍣ㄦ墍瀵瑰簲鐨勫瓧绗﹁澶囧緢鏈夌敤銆?    濡傛灉瀹冩槸 `gpio-aggregator.0`锛岄偅涔?    `/sys/devices/platform/gpio-aggregator.0/gpiochipX` 璺緞浼氬憡璇変綘璇?GPIO
    璁惧鐨?id 鏄?`X`銆?
褰撲綘鎯宠瀹炰緥鍖?`Y+1`锛圷 >= 0锛夋潯绾挎椂锛屽繀椤讳负姣忔潯鎯宠瀹炰緥鍖栫殑铏氭嫙绾垮垱寤哄瓙鐩綍锛?鍚嶇О蹇呴』涓ユ牸涓?`line0`銆乣line1`銆?..銆乣lineY`銆傚湪閫氳繃灏?`live` 璁句负 1 鏉ユ縺娲?璁惧涔嬪墠锛岄厤缃ソ鎵€鏈夌嚎銆?
**Group:** `/config/gpio-aggregator/<example-name>/<lineY>/`

    璇ョ洰褰曡〃绀鸿鍖呭惈杩涜仛鍚堝櫒鐨勪竴鏉?GPIO 绾裤€?
**Attribute:** `/config/gpio-aggregator/<example-name>/<lineY>/key`

**Attribute:** `/config/gpio-aggregator/<example-name>/<lineY>/offset`

    鍒涘缓 `<lineY>` 鐩綍鍚庣殑榛樿鍊间负锛?
    - `key` : <empty>
    - `offset` : -1

    `key` 蹇呴』濮嬬粓鏄惧紡閰嶇疆锛岃€?`offset` 鍒欒鎯呭喌鑰屽畾銆傛瘡涓?`<lineY>` 鏈変袱绉?    閰嶇疆妯″紡锛?
    (a). 鎸?GPIO 绾挎潯鍚嶇О鏌ユ壘锛?
         - 灏?`key` 璁剧疆涓虹嚎鏉″悕绉般€?         - 纭繚 `offset` 淇濇寔涓?-1锛堥粯璁ゅ€硷級銆?
    (b). 鎸?GPIO 鑺墖鍚嶇О浠ュ強璇ヨ姱鐗囧唴鐨勭嚎鏉″亸绉绘煡鎵撅細

         - 灏?`key` 璁剧疆涓鸿姱鐗囧悕绉般€?         - 灏?`offset` 璁剧疆涓虹嚎鏉″亸绉伙紙0 <= `offset` < 65535锛夈€?
**Attribute:** `/config/gpio-aggregator/<example-name>/<lineY>/name`

    `name` 灞炴€т负 lineY 璁剧疆涓€涓嚜瀹氫箟鍚嶇О銆傚鏋滅暀绌猴紝璇ョ嚎灏嗕繚鎸佹棤鍚嶃€?
閰嶇疆瀹屾垚鍚庯紝蹇呴』灏?`'live'` 灞炴€ц涓?1 鎵嶈兘瀹炰緥鍖栬仛鍚堝櫒璁惧銆傚彲浠ュ皢鍏惰鍥?0 鏉?閿€姣佽铏氭嫙璁惧銆傛ā鍧椾細鍚屾绛夊緟鏂扮殑鑱氬悎鍣ㄨ澶囪鎴愬姛鎺㈡祴锛屽鏋滄湭鍙戠敓锛屽啓鍏?`'live'`
灏嗗鑷撮敊璇€傝繖涓庝娇鐢?sysfs `new_device` 鎺ュ彛鍒涘缓鏃剁殑鎯呭喌涓嶅悓銆?
   瀵逛簬閫氳繃 Sysfs 鍒涘缓鐨勮仛鍚堝櫒锛宑onfigfs 鏉＄洰浼氳嚜鍔ㄧ敓鎴愶紝骞惰〃鐜颁负
   `/config/gpio-aggregator/_sysfs.<N>/`銆備綘鏃犳硶浣跨敤 mkdir(2)/rmdir(2) 娣诲姞鎴?   鍒犻櫎绾挎潯鐩綍銆傝淇敼绾挎潯锛屽繀椤讳娇鐢?"delete_device" 鎺ュ彛鎷嗛櫎鐜版湁璁惧骞朵粠澶?   閲嶆柊閰嶇疆銆備絾鏄紝褰?`live` 鐢辨墜宸ヨ涓?0 鏃讹紙鍗冲畠涓嶇瓑寰呭欢鍚庣殑鎺㈡祴锛夛紝浣犱粛鐒?   鍙互鐢?`live` 灞炴€у垏鎹㈣仛鍚堝櫒锛屽苟璋冩暣姣忔潯绾跨殑 `key`銆乣offset` 鍜?`name`
   灞炴€с€?
#### 绀轰緥閰嶇疆鍛戒护



    # 涓鸿仛鍚堝櫒璁惧鍒涘缓鐩綍
    $ mkdir /sys/kernel/config/gpio-aggregator/agg0

    # 閰嶇疆姣忔潯绾?    $ mkdir /sys/kernel/config/gpio-aggregator/agg0/line0
    $ echo gpiochip0 > /sys/kernel/config/gpio-aggregator/agg0/line0/key
    $ echo 6         > /sys/kernel/config/gpio-aggregator/agg0/line0/offset
    $ echo test0     > /sys/kernel/config/gpio-aggregator/agg0/line0/name
    $ mkdir /sys/kernel/config/gpio-aggregator/agg0/line1
    $ echo gpiochip0 > /sys/kernel/config/gpio-aggregator/agg0/line1/key
    $ echo 7         > /sys/kernel/config/gpio-aggregator/agg0/line1/offset
    $ echo test1     > /sys/kernel/config/gpio-aggregator/agg0/line1/name

    # 婵€娲昏仛鍚堝櫒璁惧
    $ echo 1         > /sys/kernel/config/gpio-aggregator/agg0/live


### 閫氱敤 GPIO 椹卞姩

GPIO 鑱氬悎鍣ㄤ篃鍙互鐢ㄤ綔 DT 涓弿杩扮殑銆佺敱绠€鍗?GPIO 鎿嶄綔鐨勮澶囩殑閫氱敤椹卞姩锛岃€屾棤闇€
涓撶敤鐨勫唴鏍稿唴椹卞姩銆傝繖鍦ㄥ伐涓氭帶鍒朵腑寰堟湁鐢紝骞朵笖涓庝緥濡?spidev 骞舵棤涓嶅悓锛屽悗鑰呭厑璁?鐢ㄦ埛浠庣敤鎴风┖闂翠笌 SPI 璁惧閫氫俊銆?
灏嗕竴涓澶囩粦瀹氬埌 GPIO 鑱氬悎鍣紝鍙互閫氳繃淇敼 gpio-aggregator 椹卞姩锛屾垨鑰呴€氳繃鍐欏叆
Sysfs 涓殑 "driver_override" 鏂囦欢鏉ュ畬鎴愩€?
Example: 濡傛灉 "door" 鏄竴涓湪 DT 涓弿杩扮殑銆佺敱 GPIO 鎿嶄綔鐨勮澶囷紝浣跨敤鍏惰嚜宸辩殑
```

	door {
		compatible = "myvendor,mydoor";

		gpios = <&gpio2 19 GPIO_ACTIVE_HIGH>,
			<&gpio2 20 GPIO_ACTIVE_LOW>;
		gpio-line-names = "open", "lock";
	};

```
瀹冨彲浠ラ€氳繃浠ヤ笅涓ょ鏂瑰紡缁戝畾鍒?GPIO 鑱氬悎鍣細

1. 灏嗗叾 compatible 鍊兼坊鍔犲埌 `gpio_aggregator_dt_ids[]`锛?2. 浣跨敤 "driver_override" 鎵嬪姩缁戝畾锛?

    $ echo gpio-aggregator > /sys/bus/platform/devices/door/driver_override
    $ echo door > /sys/bus/platform/drivers/gpio-aggregator/bind

涔嬪悗锛屼細鍒涘缓涓€涓柊鐨?gpiochip "door"锛?

    $ gpioinfo door
    gpiochip12 - 2 lines:
	    line   0:       "open"       unused   input  active-high
	    line   1:       "lock"       unused   input  active-high
