
## Linux I2C 鐨?Sysfs


## 姒傝堪


鐢变簬瀛樺湪 I2C MUX锛圛2C 澶氳矾澶嶇敤鍣級锛孖2C 鎷撴墤鍙兘鍗佸垎澶嶆潅銆侺inux
鍐呮牳灏?MUX 閫氶亾鎶借薄涓洪€昏緫 I2C 鎬荤嚎缂栧彿銆傜劧鑰岋紝浠?I2C 鎬荤嚎鐗╃悊缂栧彿鍜?MUX
鎷撴墤鏄犲皠鍒伴€昏緫 I2C 鎬荤嚎缂栧彿涔嬮棿瀛樺湪鐭ヨ瘑楦挎矡銆傛湰鏂囨。鏃ㄥ湪濉ˉ杩欎竴楦挎矡锛屼娇
璇昏€咃紙渚嬪纭欢宸ョ▼甯堝拰鏂扮殑杞欢寮€鍙戜汉鍛橈級鑳藉閫氳繃浜嗚В鐗╃悊 I2C
鎷撴墤骞跺湪 Linux shell 涓祻瑙?I2C sysfs锛屾帉鎻″唴鏍镐腑閫昏緫 I2C 鎬荤嚎鐨勬蹇点€傝繖涓€鐭ヨ瘑
瀵逛簬浣跨敤 `i2c-tools` 杩涜寮€鍙戝拰璋冭瘯鍗佸垎鏈夌敤涓斿繀涓嶅彲灏戙€?
### 鐩爣璇昏€?

闇€瑕佷娇鐢?Linux shell 涓庤繍琛?Linux 鐨勭郴缁熶笂鐨?I2C 瀛愮郴缁熻繘琛屼氦浜掔殑浜哄憳銆?
### 鍓嶆彁鏉′欢


1. 浜嗚В Linux shell 鏂囦欢绯荤粺鍛戒护鍜屾搷浣滅殑涓€鑸煡璇嗐€?
2. 浜嗚В I2C銆両2C MUX 鍜?I2C 鎷撴墤鐨勪竴鑸煡璇嗐€?
## I2C Sysfs 鐨勪綅缃?

閫氬父锛孡inux Sysfs 鏂囦欢绯荤粺鎸傝浇鍦?`/sys` 鐩綍涓嬶紝鍥犳鍙互鍦?`/sys/bus/i2c/devices` 涓嬫壘鍒?I2C Sysfs锛屼綘鍙互鐩存帴 `cd` 鍒拌鐩綍銆?璇ョ洰褰曚笅鏈変竴绯诲垪绗﹀彿閾炬帴銆備互 `i2c-` 寮€澶寸殑閾炬帴鏄?I2C 鎬荤嚎锛屽彲鑳芥槸鐗╃悊鐨勶紝
涔熷彲鑳芥槸閫昏緫鐨勩€傚叾浠栦互鏁板瓧寮€澶村苟浠ユ暟瀛楃粨灏剧殑閾炬帴鏄?I2C 璁惧锛屽叾涓涓€涓暟瀛楁槸
I2C 鎬荤嚎缂栧彿锛岀浜屼釜鏁板瓧鏄?I2C 鍦板潃銆?
```

  blueline:/sys/bus/i2c/devices $ ls
  0-0008  0-0061  1-0028  3-0043  4-0036  4-0041  i2c-1  i2c-3
  0-000c  0-0066  2-0049  4-000b  4-0040  i2c-0   i2c-2  i2c-4

```
`i2c-2` 鏄紪鍙蜂负 2 鐨?I2C 鎬荤嚎锛宍2-0049` 鏄€荤嚎 2 涓婂湴鍧€涓?0x49銆佸凡缁戝畾鍐呮牳椹卞姩鐨?I2C 璁惧銆?
## 鏈


棣栧厛锛屾垜浠畾涔変竴浜涙湳璇紝浠ラ伩鍏嶅悗缁珷鑺備腑鐨勬贩娣嗐€?
### 锛堢墿鐞嗭級I2C 鎬荤嚎鎺у埗鍣?

杩愯 Linux 鍐呮牳鐨勭‖浠剁郴缁熷彲鑳芥嫢鏈夊涓墿鐞?I2C 鎬荤嚎鎺у埗鍣ㄣ€傝繖浜涙帶鍒跺櫒鏄‖浠朵笖
鐗╃悊鐨勶紝绯荤粺鍙兘鍦ㄥ唴瀛樼┖闂翠腑瀹氫箟澶氫釜瀵勫瓨鍣ㄦ潵鎿嶄綔杩欎簺鎺у埗鍣ㄣ€侺inux 鍐呮牳鍦ㄦ簮鐮?鐩綍 `drivers/i2c/busses` 涓嬫湁 I2C 鎬荤嚎椹卞姩锛岀敤浜庡皢鍐呮牳 I2C API 杞崲涓洪拡瀵逛笉鍚?绯荤粺鐨勫瘎瀛樺櫒鎿嶄綔銆傛鏈骞朵笉灞€闄愪簬 Linux 鍐呮牳銆?
### I2C 鎬荤嚎鐗╃悊缂栧彿


瀵逛簬姣忎釜鐗╃悊 I2C 鎬荤嚎鎺у埗鍣紝绯荤粺鍘傚晢鍙兘浼氫负姣忎釜鎺у埗鍣ㄥ垎閰嶄竴涓墿鐞嗙紪鍙枫€?渚嬪锛屽叿鏈夋渶浣庡瘎瀛樺櫒鍦板潃鐨勭涓€涓?I2C 鎬荤嚎鎺у埗鍣ㄥ彲鑳借绉颁负 `I2C-0`銆?
### 閫昏緫 I2C 鎬荤嚎


浣犲湪 Linux I2C Sysfs 涓湅鍒扮殑姣忎釜 I2C 鎬荤嚎缂栧彿閮芥槸涓€涓鍒嗛厤浜嗙紪鍙风殑閫昏緫 I2C
鎬荤嚎銆傝繖绫讳技浜庤蒋浠朵唬鐮侀€氬父缂栧啓鍦ㄨ櫄鎷熷唴瀛樼┖闂翠箣涓婏紝鑰岄潪鐗╃悊鍐呭瓨绌洪棿銆?
姣忎釜閫昏緫 I2C 鎬荤嚎鍙兘鏄煇涓墿鐞?I2C 鎬荤嚎鎺у埗鍣ㄧ殑鎶借薄锛屼篃鍙兘鏄煇涓?I2C MUX
涔嬪悗鐨勯€氶亾鐨勬娊璞°€傚鏋滃畠鏄?MUX 閫氶亾鐨勬娊璞★紝閭ｄ箞姣忓綋鎴戜滑閫氳繃姝ょ被閫昏緫鎬荤嚎璁块棶
I2C 璁惧鏃讹紝鍐呮牳浼氫綔涓烘娊璞＄殑涓€閮ㄥ垎涓轰綘鎶?I2C MUX 鍒囨崲鍒扮浉搴旂殑閫氶亾銆?
### 鐗╃悊 I2C 鎬荤嚎


濡傛灉閫昏緫 I2C 鎬荤嚎鏄煇涓墿鐞?I2C 鎬荤嚎鎺у埗鍣ㄧ殑鐩存帴鎶借薄锛屾垜浠О涔嬩负鐗╃悊 I2C 鎬荤嚎銆?
### 娉ㄦ剰浜嬮」


瀵逛簬鍙簡瑙ｇ數璺澘鐗╃悊 I2C 璁捐鐨勪汉鏉ヨ锛岃繖閮ㄥ垎鍙兘浠や汉鍥版儜銆傚疄闄呬笂锛屽湪璁惧鏍?婧愭枃浠讹紙DTS锛夌殑 `aliases` 娈典笅锛屽彲浠ュ皢 I2C 鎬荤嚎鐗╃悊缂栧彿閲嶅懡鍚嶄负閫昏緫 I2C 鎬荤嚎
绾у埆涓婄殑涓嶅悓缂栧彿銆傜浉鍏?DTS 鏂囦欢绀轰緥鍙傝
`arch/arm/boot/dts/nuvoton-npcm730-gsj.dts`銆?
鏈€浣冲疄璺碉細**锛堥拡瀵瑰唴鏍歌蒋浠跺紑鍙戜汉鍛橈級** 鏈€濂借 I2C 鎬荤嚎鐗╃悊缂栧彿涓庡叾瀵瑰簲鐨勯€昏緫 I2C
鎬荤嚎缂栧彿淇濇寔涓€鑷达紝鑰屼笉鏄噸鍛藉悕鎴栨槧灏勫畠浠紝杩欐牱鍙互鍑忓皯缁欏叾浠栫敤鎴风殑鍥版儜銆傝繖浜?鐗╃悊 I2C 鎬荤嚎鍙互浣滀负 I2C MUX 鎵囧嚭鐨勮壇濂借捣鐐广€傚湪鍚庣画绀轰緥涓紝鎴戜滑灏嗗亣璁剧墿鐞?I2C
鎬荤嚎鐨勭紪鍙蜂笌鍏?I2C 鎬荤嚎鐗╃悊缂栧彿鐩稿悓銆?
## 閬嶅巻閫昏緫 I2C 鎬荤嚎


浠ヤ笅鍐呭灏嗕娇鐢ㄤ竴涓洿澶嶆潅鐨?I2C 鎷撴墤浣滀负绀轰緥銆備笅闈㈡槸璇?I2C 鎷撴墤鐨勭畝瑕佸浘绀恒€傚鏋?浣犵涓€鐪兼病鐪嬫噦锛屼笉蹇呮媴蹇冿紝缁х画闃呰鏈枃妗ｏ紝璇诲畬鍚庡啀鍥炵湅鍗冲彲銆?
```

  i2c-7 (physical I2C bus controller 7)
  `-- 7-0071 (4-channel I2C MUX at 0x71)
      |-- i2c-60 (channel-0)
      |-- i2c-73 (channel-1)
      |   |-- 73-0040 (I2C sensor device with hwmon directory)
      |   |-- 73-0070 (I2C MUX at 0x70, exists in DTS, but failed to probe)
      |   `-- 73-0072 (8-channel I2C MUX at 0x72)
      |       |-- i2c-78 (channel-0)
      |       |-- ... (channel-1...6, i2c-79...i2c-84)
      |       `-- i2c-85 (channel-7)
      |-- i2c-86 (channel-2)
      `-- i2c-203 (channel-3)

```
### 鍖哄垎鐗╃悊 I2C 鎬荤嚎涓庨€昏緫 I2C 鎬荤嚎


鍖哄垎鐗╃悊 I2C 鎬荤嚎鍜岄€昏緫 I2C 鎬荤嚎鐨勪竴涓畝鍗曟柟娉曪紝鏄娇鐢?`ls -l` 鎴?`readlink`
鍛戒护璇诲彇 I2C 鎬荤嚎鐩綍涓嬪悕涓?`device` 鐨勭鍙烽摼鎺ャ€?
鍙︿竴涓彲妫€鏌ョ殑绗﹀彿閾炬帴鏄?`mux_device`銆傝閾炬帴鍙瓨鍦ㄤ簬浠庡彟涓€鏉?I2C 鎬荤嚎鎵囧嚭鐨?閫昏緫 I2C 鎬荤嚎鐩綍涓€傝鍙栨閾炬帴杩樿兘鍛婅瘔浣犳槸鍝釜 I2C MUX 璁惧鍒涘缓浜嗚繖涓€昏緫 I2C
鎬荤嚎銆?
濡傛灉绗﹀彿閾炬帴鎸囧悜浠?`.i2c` 缁撳熬鐨勭洰褰曪紝閭ｄ箞瀹冨簲璇ユ槸涓€涓墿鐞?I2C 鎬荤嚎锛岀洿鎺ユ娊璞′簡
鏌愪釜鐗╃悊 I2C 鎬荤嚎鎺у埗鍣ㄣ€備緥濡傦細
```

  $ readlink /sys/bus/i2c/devices/i2c-7/device
  ../../f0087000.i2c
  $ ls /sys/bus/i2c/devices/i2c-7/mux_device
  ls: /sys/bus/i2c/devices/i2c-7/mux_device: No such file or directory

```
鍦ㄦ渚嬩腑锛宍i2c-7` 鏄竴鏉＄墿鐞?I2C 鎬荤嚎锛屽洜姝ゅ叾鐩綍涓嬫病鏈?`mux_device` 绗﹀彿閾炬帴銆?濡傛灉鍐呮牳杞欢寮€鍙戜汉鍛橀伒寰笉閲嶅懡鍚嶇墿鐞?I2C 鎬荤嚎鐨勬儻渚嬶紝杩欎篃鎰忓懗鐫€瀹冨搴旂郴缁熶腑缂栧彿涓?7 鐨勭墿鐞?I2C 鎬荤嚎鎺у埗鍣ㄣ€?
鍙︿竴鏂归潰锛屽鏋滅鍙烽摼鎺ユ寚鍚戝彟涓€鏉?I2C 鎬荤嚎锛屽垯褰撳墠鐩綍鎵€琛ㄧず鐨?I2C 鎬荤嚎蹇呭畾鏄竴鏉?閫昏緫鎬荤嚎銆傝閾炬帴鎸囧悜鐨?I2C 鎬荤嚎鏄埗鎬荤嚎锛屽彲鑳芥槸鐗╃悊 I2C 鎬荤嚎锛屼篃鍙兘鏄€昏緫 I2C
鎬荤嚎銆傚湪杩欑鎯呭喌涓嬶紝褰撳墠鐩綍鎵€琛ㄧず鐨?I2C 鎬荤嚎鎶借薄鐨勬槸鐖舵€荤嚎涓嬬殑鏌愪釜 I2C MUX 閫氶亾銆?
```

  $ readlink /sys/bus/i2c/devices/i2c-73/device
  ../../i2c-7
  $ readlink /sys/bus/i2c/devices/i2c-73/mux_device
  ../7-0071

```
`i2c-73` 鏄敱 `i2c-7` 涓嬫煇涓?I2C MUX 鎵囧嚭鐨勯€昏緫鎬荤嚎锛岃 MUX 鐨?I2C 鍦板潃涓?0x71銆傛瘡褰撴垜浠闂€荤嚎 73 涓婄殑鏌愪釜 I2C 璁惧鏃讹紝鍐呮牳鎬讳細浣滀负鎶借薄鐨勪竴閮ㄥ垎锛屾妸鍦板潃涓?0x71 鐨?I2C MUX 涓轰綘鍒囨崲鍒扮浉搴旈€氶亾銆?
### 鏌ユ壘閫昏緫 I2C 鎬荤嚎缂栧彿


鏈妭灏嗘弿杩板浣曞熀浜庣墿鐞嗙‖浠?I2C 鎷撴墤鐨勭煡璇嗭紝鎵惧嚭琛ㄧず鐗瑰畾 I2C MUX 閫氶亾鐨勯€昏緫 I2C
鎬荤嚎缂栧彿銆?
鍦ㄦ绀轰緥涓紝鎴戜滑鏈変竴涓郴缁燂紝鍏剁墿鐞?I2C 鎬荤嚎 7 鍦?DTS 涓湭琚噸鍛藉悕銆傝鎬荤嚎涓婃湁涓€涓?鍦板潃涓?0x71 鐨?4 閫氶亾 MUX銆傚湪 0x71 杩欎釜 MUX 鐨勯€氶亾 1 涔嬪悗锛岃繕鏈変竴涓湴鍧€涓?0x72 鐨?8 閫氶亾 MUX銆傝鎴戜滑娴忚 Sysfs锛屾壘鍑?0x72 MUX 鐨勯€氶亾 3 鐨勯€昏緫 I2C 鎬荤嚎缂栧彿銆?
```

  ~$ cd /sys/bus/i2c/devices/i2c-7
  /sys/bus/i2c/devices/i2c-7$ ls
  7-0071         i2c-60         name           subsystem
  delete_device  i2c-73         new_device     uevent
  device         i2c-86         of_node
  i2c-203        i2c-dev        power

```
```

  /sys/bus/i2c/devices/i2c-7$ cd 7-0071/
  /sys/bus/i2c/devices/i2c-7/7-0071$ ls -l
  channel-0   channel-3   modalias    power
  channel-1   driver      name        subsystem
  channel-2   idle_state  of_node     uevent

```
```

  /sys/bus/i2c/devices/i2c-7/7-0071$ readlink channel-1
  ../i2c-73

```
鎴戜滑鍙戠幇 `i2c-7` 涓?0x71 MUX 鐨勯€氶亾 1 琚垎閰嶄簡閫昏緫 I2C 鎬荤嚎缂栧彿 73銆?```

  # cd to i2c-73 under I2C Sysfs root
  /sys/bus/i2c/devices/i2c-7/7-0071$ cd /sys/bus/i2c/devices/i2c-73
  /sys/bus/i2c/devices/i2c-73$

  # cd the channel symbolic link
  /sys/bus/i2c/devices/i2c-7/7-0071$ cd channel-1
  /sys/bus/i2c/devices/i2c-7/7-0071/channel-1$

  # cd the link content
  /sys/bus/i2c/devices/i2c-7/7-0071$ cd ../i2c-73
  /sys/bus/i2c/devices/i2c-7/i2c-73$

```
鏃犺鍝鏂瑰紡锛屾渶缁堥兘浼氳繘鍏?`i2c-73` 鐨勭洰褰曘€傜被浼煎湴锛屾垜浠幇鍦ㄥ彲浠ユ壘鍑?0x72 MUX
鍙婂叾瀵瑰簲鐨勯€昏緫 I2C 鎬荤嚎缂栧彿锛?```

  /sys/bus/i2c/devices/i2c-73$ ls
  73-0040        device         i2c-83         new_device
  73-004e        i2c-78         i2c-84         of_node
  73-0050        i2c-79         i2c-85         power
  73-0070        i2c-80         i2c-dev        subsystem
  73-0072        i2c-81         mux_device     uevent
  delete_device  i2c-82         name
  /sys/bus/i2c/devices/i2c-73$ cd 73-0072
  /sys/bus/i2c/devices/i2c-73/73-0072$ ls
  channel-0   channel-4   driver      of_node
  channel-1   channel-5   idle_state  power
  channel-2   channel-6   modalias    subsystem
  channel-3   channel-7   name        uevent
  /sys/bus/i2c/devices/i2c-73/73-0072$ readlink channel-3
  ../i2c-81

```
鍦ㄨ繖閲岋紝鎴戜滑寰楃煡 0x72 MUX 鐨勯€氶亾 3 鐨勯€昏緫 I2C 鎬荤嚎缂栧彿鏄?81銆備箣鍚庢垜浠彲浠ョ敤杩欎釜
缂栧彿鍒囨崲鍒板畠鑷繁鐨?I2C Sysfs 鐩綍锛屾垨鍙戝嚭 `i2c-tools` 鍛戒护銆?
鎻愮ず锛氫竴鏃︿綘鐞嗚В浜嗗甫 MUX 鐨?I2C 鎷撴墤锛屽鏋滀綘鐨勭郴缁熶笂鍙敤锛宍I2C Tools
<https://i2c.wiki.kernel.org/index.php/I2C_Tools>`_ 涓殑鍛戒护
`i2cdetect -l
<https://manpages.debian.org/unstable/i2c-tools/i2cdetect.8.en.html>`_
鍙互璁╀綘杞绘澗浜嗚В I2C 鎷撴墤姒傝銆備緥濡傦細
```

  $ i2cdetect -l | grep -e '\-73' -e _7 | sort -V
  i2c-7   i2c             npcm_i2c_7                              I2C adapter
  i2c-73  i2c             i2c-7-mux (chan_id 1)                   I2C adapter
  i2c-78  i2c             i2c-73-mux (chan_id 0)                  I2C adapter
  i2c-79  i2c             i2c-73-mux (chan_id 1)                  I2C adapter
  i2c-80  i2c             i2c-73-mux (chan_id 2)                  I2C adapter
  i2c-81  i2c             i2c-73-mux (chan_id 3)                  I2C adapter
  i2c-82  i2c             i2c-73-mux (chan_id 4)                  I2C adapter
  i2c-83  i2c             i2c-73-mux (chan_id 5)                  I2C adapter
  i2c-84  i2c             i2c-73-mux (chan_id 6)                  I2C adapter
  i2c-85  i2c             i2c-73-mux (chan_id 7)                  I2C adapter

```
### 鍥哄畾鐨勯€昏緫 I2C 鎬荤嚎缂栧彿


濡傛灉鍦?DTS 涓湭鎸囧畾锛屽綋 I2C MUX 椹卞姩琚簲鐢ㄤ笖 MUX 璁惧鎴愬姛 probe 鏃讹紝鍐呮牳灏嗗熀浜?褰撳墠鏈€澶х殑閫昏緫鎬荤嚎缂栧彿锛岄€掑鍦颁负 MUX 閫氶亾鍒嗛厤閫昏緫鎬荤嚎缂栧彿銆備緥濡傦紝濡傛灉绯荤粺涓?`i2c-15` 鏄渶楂樼殑閫昏緫鎬荤嚎缂栧彿锛屼笖涓€涓?4 閫氶亾 MUX 琚垚鍔熷簲鐢紝閭ｄ箞 MUX 閫氶亾 0 灏?鑾峰緱 `i2c-16`锛屼竴鐩村埌 MUX 閫氶亾 3 鑾峰緱 `i2c-19`銆?
鍐呮牳杞欢寮€鍙戜汉鍛樿兘澶熷湪 DTS 涓皢鎵囧嚭鐨?MUX 閫氶亾鍥哄畾鍒伴潤鎬佺殑閫昏緫 I2C 鎬荤嚎缂栧彿銆傛湰鏂囨。
涓嶄細娣卞叆璁茶В濡備綍鍦?DTS 涓疄鐜拌繖涓€鐐癸紝浣嗘垜浠彲浠ュ湪浠ヤ笅绀轰緥涓湅鍒帮細
`arch/arm/boot/dts/aspeed-bmc-facebook-wedge400.dts`

鍦ㄤ笂杩扮ず渚嬩腑锛岀墿鐞?I2C 鎬荤嚎 2 涓婃湁涓€涓湴鍧€涓?0x70 鐨?8 閫氶亾 I2C MUX銆傝 MUX 鐨?閫氶亾 2 鍦?DTS 涓瀹氫箟涓?`imux18`锛屽苟閫氳繃 `aliases` 娈典腑鐨?`i2c18 = &imux18;` 鍥哄畾鍒伴€昏緫 I2C 鎬荤嚎缂栧彿 18銆?
鏇磋繘涓€姝ワ紝鍙互璁捐涓€濂椾究浜庝汉绫昏蹇嗘垨閫氳繃绠楁湳璁＄畻寰楀嚭鐨勯€昏緫 I2C 鎬荤嚎缂栧彿鏂规銆備緥濡傦紝
鎴戜滑鍙互灏嗘€荤嚎 3 涓?MUX 鐨勬墖鍑洪€氶亾鍥哄畾涓轰粠 30 寮€濮嬨€備簬鏄?30 灏嗘槸鎬荤嚎 3 涓?MUX 閫氶亾 0
鐨勯€昏緫鎬荤嚎缂栧彿锛岃€?37 灏嗘槸鎬荤嚎 3 涓?MUX 閫氶亾 7 鐨勯€昏緫鎬荤嚎缂栧彿銆?
## I2C 璁惧


鍦ㄤ箣鍓嶇殑绔犺妭涓紝鎴戜滑涓昏浠嬬粛鐨勬槸 I2C 鎬荤嚎銆傛湰鑺傝鎴戜滑鐪嬬湅浠庨摼鎺ュ悕涓?`${bus}-${addr}`
鏍煎紡鐨?I2C 璁惧鐩綍涓彲浠ヤ簡瑙ｅ埌浠€涔堛€傚悕绉颁腑鐨?`${bus}` 閮ㄥ垎鏄€昏緫 I2C 鎬荤嚎鐨勫崄杩涘埗
缂栧彿锛岃€?`${addr}` 閮ㄥ垎鏄瘡涓澶?I2C 鍦板潃鐨勫崄鍏繘鍒剁紪鍙枫€?
### I2C 璁惧鐩綍鍐呭


鍦ㄦ瘡涓?I2C 璁惧鐩綍鍐呴儴锛屾湁涓€涓悕涓?`name` 鐨勬枃浠躲€傝鏂囦欢璇存槑鍐呮牳椹卞姩鐢ㄦ潵
鍖归厤璇ヨ澶囩殑璁惧鍚嶇О鏄粈涔堬細
```

  /sys/bus/i2c/devices/i2c-73$ cat 73-0040/name
  ina230
  /sys/bus/i2c/devices/i2c-73$ cat 73-0070/name
  pca9546
  /sys/bus/i2c/devices/i2c-73$ cat 73-0072/name
  pca9547

```
鏈変竴涓悕涓?`driver` 鐨勭鍙烽摼鎺ワ紝鐢ㄤ簬璇存槑浣跨敤浜嗗摢涓?Linux 鍐呮牳椹卞姩锛?```

  /sys/bus/i2c/devices/i2c-73$ readlink -f 73-0040/driver
  /sys/bus/i2c/drivers/ina2xx
  /sys/bus/i2c/devices/i2c-73$ readlink -f 73-0072/driver
  /sys/bus/i2c/drivers/pca954x

```
浣嗗鏋?`driver` 閾炬帴涓€寮€濮嬪氨涓嶅瓨鍦紝鍒欏彲鑳芥剰鍛崇潃鍐呮牳椹卞姩鐢变簬
浠ヤ笅鍘熷洜鏈兘鎴愬姛 probe 璇ヨ澶囷細
```

  /sys/bus/i2c/devices/i2c-73$ ls 73-0070/driver
  ls: 73-0070/driver: No such file or directory
  /sys/bus/i2c/devices/i2c-73$ dmesg | grep 73-0070
  pca954x 73-0070: probe failed
  pca954x 73-0070: probe failed

```
鏍规嵁 I2C 璁惧鐨勪笉鍚屼互鍙婄敤浜?probe 璇ヨ澶囩殑鍐呮牳椹卞姩鐨勪笉鍚岋紝璁惧鐩綍涓殑鍐呭涔熷彲鑳?涓嶅悓銆?
### I2C MUX 璁惧


铏界劧浣犲彲鑳藉湪鍓嶉潰鐨勭珷鑺傚凡缁忔湁鎵€浜嗚В锛孖2C MUX 璁惧鍦ㄥ叾璁惧鐩綍涓細鍖呭惈鍚嶄负
`channel-*` 鐨勭鍙烽摼鎺ャ€?```

  /sys/bus/i2c/devices/i2c-73$ ls -l 73-0072/channel-*
  lrwxrwxrwx ... 73-0072/channel-0 -> ../i2c-78
  lrwxrwxrwx ... 73-0072/channel-1 -> ../i2c-79
  lrwxrwxrwx ... 73-0072/channel-2 -> ../i2c-80
  lrwxrwxrwx ... 73-0072/channel-3 -> ../i2c-81
  lrwxrwxrwx ... 73-0072/channel-4 -> ../i2c-82
  lrwxrwxrwx ... 73-0072/channel-5 -> ../i2c-83
  lrwxrwxrwx ... 73-0072/channel-6 -> ../i2c-84
  lrwxrwxrwx ... 73-0072/channel-7 -> ../i2c-85

```
### I2C 浼犳劅鍣ㄨ澶?/ Hwmon


I2C 浼犳劅鍣ㄨ澶囦篃寰堝父瑙併€傚鏋滃畠浠鏌愪釜鍐呮牳 hwmon锛堢‖浠剁洃鎺э級椹卞姩鎴愬姛缁戝畾锛屼綘灏?鍦?I2C 璁惧鐩綍涓湅鍒颁竴涓悕涓?`hwmon` 鐨勭洰褰曘€傜户缁繁鍏ワ紝浣犱細鎵惧埌 Hwmon
sysfs 鎺ュ彛锛?```

  /sys/bus/i2c/devices/i2c-73/73-0040/hwmon/hwmon17$ ls
  curr1_input        in0_lcrit_alarm    name               subsystem
  device             in1_crit           power              uevent
  in0_crit           in1_crit_alarm     power1_crit        update_interval
  in0_crit_alarm     in1_input          power1_crit_alarm
  in0_input          in1_lcrit          power1_input
  in0_lcrit          in1_lcrit_alarm    shunt_resistor

```
鍏充簬 Hwmon Sysfs 鐨勬洿澶氫俊鎭紝璇峰弬闃呰鏂囨。锛?
../hwmon/sysfs-interface.rst

### 鍦?I2C Sysfs 涓疄渚嬪寲 I2C 璁惧


璇峰弬闃?instantiating-devices.rst 涓殑鈥滄柟娉?4锛氫粠鐢ㄦ埛绌洪棿瀹炰緥鍖栤€濅竴鑺傘€?