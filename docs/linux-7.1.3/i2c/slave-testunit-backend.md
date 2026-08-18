
## Linux I2C 浠庢満 testunit 鍚庣


by Wolfram Sang <wsa@sang-engineering.com> in 2020

姝ゅ悗绔彲鐢ㄤ簬瑙﹀彂閽堝 I2C 鎬荤嚎涓绘帶鍒跺櫒鐨勬祴璇曠敤渚嬶紝杩欎簺娴嬭瘯闇€瑕佸叿鏈夌壒瀹?鑳藉姏锛堜笖閫氬父涓嶆槗鑾峰緱锛夌殑杩滅▼璁惧銆備緥瀛愬寘鎷涓绘帶鍒跺櫒娴嬭瘯鍜?SMBus Host
Notify 娴嬭瘯銆傚浜庢煇浜涙祴璇曪紝I2C 浠庢満鎺у埗鍣ㄥ繀椤昏兘澶熷湪涓绘ā寮忎笌浠庢ā寮忎箣闂?鍒囨崲锛屽洜涓哄畠涔熼渶瑕佸彂閫佹暟鎹€?
璇锋敞鎰忥紝杩欐槸涓€涓敤浜庢祴璇曞拰璋冭瘯鐨勮澶囷紝涓嶅簲鍦ㄧ敓浜ф瀯寤轰腑鍚敤銆傝櫧鐒舵垜浠?鍋氫簡涓€瀹氱殑鐗堟湰绠＄悊骞跺敖鍔涗繚鎸佸悜鍚庡吋瀹癸紝浣?*涓嶄繚璇?*绋冲畾鐨?ABI锛?
```

  # echo "slave-testunit 0x1030" > /sys/bus/i2c/devices/i2c-0/new_device

```
鎴栬€呬娇鐢ㄥ浐浠惰妭鐐广€備笅闈㈡槸涓€涓?devicetree 绀轰緥锛堟敞鎰忚繖鍙槸涓€涓?```

  &i2c0	{
        ...

	testunit@30 {
		compatible = "slave-testunit";
		reg = <(0x30 | I2C_OWN_SLAVE_ADDRESS)>;
	};
  };

```
涔嬪悗锛岃璁惧浼氬紑濮嬬洃鍚€傝鍙栧皢杩斿洖鍗曚釜瀛楄妭銆傚叾鍊间负 0 琛ㄧず testunit
绌洪棽锛屽惁鍒欎负褰撳墠姝ｅ湪杩愯鐨勫懡浠ょ殑缂栧彿銆?
鍐欏叆鏃讹紝璇ヨ澶囩敱 4 涓?8 浣嶅瘎瀛樺櫒缁勬垚锛岄櫎鏌愪簺鈥滈儴鍒嗏€濆懡浠ゅ锛岃鍚姩涓€涓?娴嬭瘯鐢ㄤ緥蹇呴』鍐欏叆鎵€鏈夊瘎瀛樺櫒锛屼篃灏辨槸璇翠綘閫氬父浼氬悜璁惧鍐欏叆 4 涓瓧鑺傘€?瀵勫瓨鍣ㄥ涓嬶細

  :header: "Offset", "Name", "Description"

  0x00, CMD, 瑕佽Е鍙戠殑娴嬭瘯
  0x01, DATAL, 璇ユ祴璇曠殑閰嶇疆瀛楄妭 1
  0x02, DATAH, 璇ユ祴璇曠殑閰嶇疆瀛楄妭 2
  0x03, DELAY, 鍚姩娴嬭瘯鍓嶇殑寤惰繜锛屽崟浣嶄负 n * 10ms

```

  # i2cset -y <bus_num> <testunit_address> <CMD> <DATAL> <DATAH> <DELAY> i

```
DELAY 鏄竴涓€氱敤鍙傛暟锛屼細寤惰繜 CMD 涓祴璇曠殑鎵ц鐨勬墽琛屻€傚綋鏌愪釜鍛戒护姝ｅ湪杩愯
锛堝寘鎷欢杩熸湡闂达級鏃讹紝鏂扮殑鍛戒护涓嶄細琚簲绛斻€備綘闇€瑕佺瓑寰呮棫鍛戒护瀹屾垚銆?
鍛戒护鍦ㄤ笅涓€鑺備腑鎻忚堪銆傛棤鏁堢殑鍛戒护浼氬鑷翠紶杈撲笉琚簲绛斻€?
### 鍛戒护


#### 0x00 NOOP


淇濈暀渚涘皢鏉ヤ娇鐢ㄣ€?
#### 0x01 READ_BYTES


  :header-rows: 1

  - - CMD
    - DATAL
    - DATAH
    - DELAY

  - - 0x01
    - 瑕佽鍙栨暟鎹殑鍦板潃锛堜綆 7 浣嶏紝鏈€楂樹綅褰撳墠鏈敤锛?    - 瑕佽鍙栫殑瀛楄妭鏁?    - n * 10ms

杩橀渶瑕佷富妯″紡銆傝繖瀵逛簬娴嬭瘯浣犵殑鎬荤嚎涓绘帶鍒跺櫒鏄惁姝ｇ‘鍦板鐞嗗涓绘帶鍒跺櫒寰堟湁鐢ㄣ€?浣犲彲浠ヨЕ鍙?testunit 浠庢€荤嚎涓婄殑鍙︿竴涓澶囪鍙栧瓧鑺傘€傚鏋滆娴嬬殑鎬荤嚎涓绘帶鍒跺櫒
鍚屾椂涔熸兂璁块棶鎬荤嚎锛屾€荤嚎灏嗗浜庡繖鐘舵€併€傝鍙?128 瀛楄妭鐨勭ず渚嬶細
```

  # i2cset -y 0 0x30 1 0x50 0x80 5 i

```
#### 0x02 SMBUS_HOST_NOTIFY


  :header-rows: 1

  - - CMD
    - DATAL
    - DATAH
    - DELAY

  - - 0x02
    - 瑕佸彂閫佺殑鐘舵€佸瓧浣庡瓧鑺?    - 瑕佸彂閫佺殑鐘舵€佸瓧楂樺瓧鑺?    - n * 10ms

杩橀渶瑕佷富妯″紡銆傝娴嬭瘯灏嗗悜涓绘満鍙戦€佷竴鏉?SMBUS_HOST_NOTIFY 娑堟伅銆傝娉ㄦ剰锛岀姸鎬?瀛楃洰鍓嶅湪 Linux 鍐呮牳涓蹇界暐銆?```

  # i2cset -y 0 0x30 2 0x42 0x64 1 i

```
濡傛灉涓绘満鎺у埗鍣ㄦ敮鎸?HostNotify锛岃繖鏉¤皟璇曠骇鍒殑娑堟伅浼?```

  Detected HostNotify from address 0x30

```
#### 0x03 SMBUS_BLOCK_PROC_CALL


  :header-rows: 1

  - - CMD
    - DATAL
    - DATAH
    - DELAY

  - - 0x03
    - 0x01锛堝嵆杩樹細鍐嶅啓鍏ヤ竴涓瓧鑺傦級
    - 瑕佸洖閫佺殑瀛楄妭鏁?    - 鐪佺暐锛岄儴鍒嗗懡浠わ紒

閮ㄥ垎鍛戒护銆傝娴嬭瘯浼氭寜鐓?SMBus 瑙勮寖鐨勫畾涔夊搷搴斾竴娆″潡澶勭悊璋冪敤锛坆lock process
call锛夈€傚啓鍏ョ殑閭ｄ竴涓暟鎹瓧鑺傛寚瀹氫簡鍦ㄩ殢鍚庣殑璇讳紶杈撲腑灏嗗洖閫佸灏戝瓧鑺傘€傝娉ㄦ剰锛?鍦ㄦ璇讳紶杈撲腑锛宼estunit 浼氬厛鏀剧疆鍚庣画瀛楄妭鐨勯暱搴﹀墠缂€銆傚洜姝わ紝濡傛灉浣犵殑涓绘満
鎬荤嚎椹卞姩鍍忓ぇ澶氭暟椹卞姩閭ｆ牱妯℃嫙 SMBus 璋冪敤锛屽畠灏遍渶瑕佹敮鎸?i2c_msg 鐨?I2C_M_RECV_LEN 鏍囧織銆傝繖鏄竴涓緢濂界殑娴嬭瘯鐢ㄤ緥銆傝繑鍥炵殑鏁版嵁鍏堟槸闀垮害锛岀劧鍚庢槸
涓€涓粠 length-1 鍒?0 鐨勫瓧鑺傛暟缁勩€備笅闈㈡槸涓€涓娇鐢?i2ctransfer 妯℃嫙
i2c_smbus_block_process_call() 鐨勭ず渚嬶紙浣犻渶瑕?i2c-tools v4.2 鎴?```

  # i2ctransfer -y 0 w3@0x30 3 1 0x10 r?
  0x10 0x0f 0x0e 0x0d 0x0c 0x0b 0x0a 0x09 0x08 0x07 0x06 0x05 0x04 0x03 0x02 0x01 0x00

```
#### 0x04 GET_VERSION_WITH_REP_START


  :header-rows: 1

  - - CMD
    - DATAL
    - DATAH
    - DELAY

  - - 0x04
    - 褰撳墠鏈敤
    - 褰撳墠鏈敤
    - 鐪佺暐锛岄儴鍒嗗懡浠わ紒

閮ㄥ垎鍛戒护銆傚彂閫佹鍛戒护鍚庯紝testunit 浼氫互涓€涓熀浜?UTS_RELEASE銆佷互 NUL 缁撳熬鐨?鐗堟湰瀛楃涓叉潵鍥炲簲璇绘秷鎭€傜涓€涓瓧绗﹀缁堟槸 'v'锛岀増鏈瓧绗︿覆闀垮害鏈€澶т负 128
瀛楄妭銆備笉杩囷紝瀹冧粎鍦ㄨ娑堟伅閫氳繃 repeated start 涓庡啓娑堟伅鐩歌繛鏃舵墠浼氬洖搴斻€傚鏋?浣犵殑鎺у埗鍣ㄩ┍鍔ㄥ鐞?```

  # i2ctransfer -y 0 w3@0x30 4 0 0 r128
  0x76 0x36 0x2e 0x31 0x31 0x2e 0x30 0x2d 0x72 0x63 0x31 0x2d 0x30 0x30 0x30 0x30 ...

```
```

  # i2ctransfer -y -b 0 w3@0x30 4 0 0 r128
  v6.11.0-rc1-00009-gd37a1b4d3fd0

```
涓ゆ潯娑堟伅涔嬮棿鐨?STOP/START 缁勫悎**涓?*浼氱敓鏁堬紝鍥犱负瀹冧滑涓嶇瓑鍚屼簬涓€涓?REPEATED START銆備緥濡傦紝杩欏彧浼氳繑鍥?```

  # i2cset -y 0 0x30 4 0 0 i; i2cget -y 0 0x30
  0x00

```
#### 0x05 SMBUS_ALERT_REQUEST


  :header-rows: 1

  - - CMD
    - DATAL
    - DATAH
    - DELAY

  - - 0x05
    - 鍝嶅簲鍊硷紙楂?7 浣嶈В閲婁负 I2C 鍦板潃锛?    - 褰撳墠鏈敤
    - n * 10ms

璇ユ祴璇曢€氳繃 SMBAlert 寮曡剼寮曞彂涓€涓腑鏂紝涓绘満鎺у埗鍣ㄥ繀椤诲鐞嗗畠銆傝寮曡剼蹇呴』
浣滀负 GPIO 杩炴帴鍒?testunit銆傚 GPIO 鐨勮闂笉鍏佽鐫＄湢銆傜洰鍓嶏紝杩欏彧鑳戒娇鐢?鍥轰欢鑺傜偣鏉ユ弿杩般€傚洜姝わ紝瀵逛簬 devicetree锛屼綘浼氬湪 testunit 涓坊鍔犵被浼煎涓嬬殑
```

  gpios = <&gpio1 24 GPIO_ACTIVE_LOW>;

```
浠ヤ笅鍛戒护浼氬湪 1 绉掑悗瑙﹀彂涓€涓搷搴斿€间负 0xc9 鐨勫憡璀?```

  # i2cset -y 0 0x30 5 0xc9 0x00 100 i

```
濡傛灉涓绘満鎺у埗鍣ㄦ敮鎸?SMBusAlert锛岃繖鏉¤皟璇曠骇鍒殑娑堟伅浼?```

  smbus_alert 0-000c: SMBALERT# from dev 0x64, flag 1

```
杩欐潯娑堟伅鍙兘鍑虹幇涓嶆涓€娆★紝鍥犱负 testunit 鏄蒋浠惰€岄潪纭欢锛屽洜姝ゅ彲鑳芥棤娉曞揩閫?鍝嶅簲涓绘満鐨勫洖澶?```

  # cat /proc/interrupts | grep smbus_alert
   93:          1  gpio-rcar  26 Edge      smbus_alert

```
濡傛灉涓绘満鍦?1 绉掑唴娌℃湁鍝嶅簲鍛婅锛屾祴璇曞皢琚腑姝紝testunit 浼氭姤鍛婁竴涓敊璇€?
瀵逛簬姝ゆ祴璇曪紝testunit 浼氱煭鏆傚湴鏀惧純鍏惰鍒嗛厤鐨勫湴鍧€锛屽苟鍦?SMBus Alert
Response Address锛?x0c锛変笂鐩戝惉銆備箣鍚庡畠浼氶噸鏂板垎閰嶅叾鍘熷鍦板潃銆?