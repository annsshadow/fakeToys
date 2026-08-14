
## Netconsole


started by Ingo Molnar <mingo@redhat.com>, 2001.09.17

2.6 port and netpoll api by Matt Mackall <mpm@selenic.com>, Sep 9 2003

IPv6 support by Cong Wang <xiyou.wangcong@gmail.com>, Jan 1 2013

Extended console support by Tejun Heo <tj@kernel.org>, May 1 2015

Release prepend support by Breno Leitao <leitao@debian.org>, Jul 7 2023

Userdata append support by Matthew Wood <thepacketgeek@gmail.com>, Jan 22 2024

Sysdata append support by Breno Leitao <leitao@debian.org>, Jan 15 2025

## 绠€浠嬶細


璇ユā鍧楅€氳繃 UDP 璁板綍鍐呮牳 printk 娑堟伅锛屼究浜庡湪纾佺洏鏃ュ織澶辫触涓斾覆鍙ｆ帶鍒跺彴涓嶅疄鐢ㄧ殑鎯呭喌涓嬭繘琛岃皟璇曘€?
瀹冩棦鍙互浣滀负鍐呯疆鍔熻兘浣跨敤锛屼篃鍙互浣滀负妯″潡浣跨敤銆備綔涓哄唴缃姛鑳芥椂锛宯etconsole 鍦ㄧ綉鍗′箣鍚庣珛鍗冲垵濮嬪寲锛屽苟灏藉揩鍚敤鎸囧畾鐨勬帴鍙ｃ€傝櫧鐒惰繖鏃犳硶鎹曡幏鏃╂湡鐨勫唴鏍?panic锛屼絾瀹冪‘瀹炶兘鎹曡幏澶ч儴鍒嗙殑鍚姩杩囩▼銆?
## 鍙戦€佹柟涓庢帴鏀舵柟閰嶇疆锛?

瀹冩帴鍙椾竴涓瓧绗︿覆閰嶇疆鍙傛暟 "netconsole"锛屾牸寮忓涓嬶細
```
 netconsole=[+][r][src-port]@[src-ip]/[<dev>],[tgt-port]@<tgt-ip>/[tgt-macaddr]

   where
	+             if present, enable extended console support
	r             if present, prepend kernel version (release) to the message
	src-port      source for UDP packets (defaults to 6665)
	src-ip        source IP to use (interface address)
	dev           network interface name (eth0) or MAC address
	tgt-port      port for logging agent (6666)
	tgt-ip        IP address for logging agent
	tgt-macaddr   ethernet MAC address for logging agent (broadcast)
```
```
 linux netconsole=4444@10.0.0.1/eth1,9353@10.0.0.2/12:34:56:78:9a:bc
```
```
 insmod netconsole netconsole=@/,@10.0.0.2/
```
```
 insmod netconsole netconsole=@/,@fd00:1:2:3::1/
```
```
 linux netconsole=4444@10.0.0.1/22:33:44:55:66:77,9353@10.0.0.2/12:34:56:78:9a:bc
```
瀹冭繕鏀寔閫氳繃鐢ㄥ垎鍙峰垎闅斿涓唬鐞嗙殑鍙傛暟锛屾妸鏃ュ織鍙戦€佸埌澶氫釜杩滅▼浠ｇ悊锛?```
 modprobe netconsole netconsole="@/,@10.0.0.2/;@/eth1,6892@10.0.0.3/"
```
鍐呯疆鐨?netconsole 鍦?TCP 鍗忚鏍堝垵濮嬪寲鍚庣珛鍗冲惎鍔紝骞跺皾璇曞湪鎵€鎻愪緵鐨勫湴鍧€涓婂惎鐢ㄦ墍鎻愪緵鐨?dev銆?
杩滅▼涓绘満鏈夊嚑绉嶆帴鏀跺唴鏍告秷鎭殑鏂瑰紡锛屼緥濡傦細

1) syslogd

2) netcat

   鍦ㄤ娇鐢ㄥ熀浜?BSD 鐨?netcat 鐗堟湰锛堜緥濡?Fedora銆乷penSUSE 鍜?Ubuntu锛夌殑鍙戣鐗堜笂锛屽繀椤讳互涓嶅甫浠ヤ笅褰㈠紡鐨勬柟寮忔寚瀹氱洃鍚鍙?```
	nc -u -l -p <port>' / 'nc -u -l <port>

   or::

	netcat -u -l -p <port>' / 'netcat -u -l <port>
```
3) socat

```
   socat udp-recv:<port> -
```
## 鍔ㄦ€侀噸閰嶇疆锛?

鍔ㄦ€佸彲閲嶉厤缃槸 netconsole 鐨勪竴涓湁鐢ㄨˉ鍏咃紝瀹冧娇杩滅▼鏃ュ織鐩爣鑳藉閫氳繃鍩轰簬 configfs 鐨勭敤鎴风┖闂存帴鍙ｅ湪杩愯鏃惰鍔ㄦ€佹坊鍔犮€佺Щ闄ゆ垨鍏跺弬鏁拌閲嶆柊閰嶇疆銆?
瑕佸寘鍚鐗规€э紝璇峰湪鏋勫缓 netconsole 妯″潡锛堟垨鍐呮牳锛屽鏋?netconsole 鏄唴缃殑锛夋椂閫夋嫨 CONFIG_NETCONSOLE_DYNAMIC銆?
浠ヤ笅鏄竴浜涚ず渚嬶紙鍏朵腑 configfs 鎸傝浇鍦?/sys/kernel/config 鎸傝浇鐐癸級銆?
```
 cd /sys/kernel/config/netconsole/
 mkdir target1
```
璇锋敞鎰忥紝鏂板垱寤虹殑鐩爣鍏锋湁榛樿鐨勫弬鏁板€硷紙濡備笂鎵€杩帮級锛屽苟涓旈粯璁ゆ槸绂佺敤鐨勨€斺€斿畠浠繀椤婚鍏堥€氳繃灏?"1" 鍐欏叆 "enabled" 灞炴€э紙閫氬父鍦ㄧ浉搴斿湴璁剧疆鍙傛暟涔嬪悗锛夋潵鍚敤锛屽涓嬫墍杩般€?
```
 rmdir /sys/kernel/config/netconsole/othertarget/
```
璇ユ帴鍙ｅ悜鐢ㄦ埛绌洪棿鏆撮湶浜?netconsole 鐩爣鐨勪互涓嬪弬鏁帮細

	=============== =================================       ============
	enabled		璇ョ洰鏍囧綋鍓嶆槸鍚﹀凡鍚敤锛?	锛堝彲璇诲啓锛?	extended	鏄惁鍚敤鎵╁睍妯″紡			锛堝彲璇诲啓锛?	release		鍦ㄦ秷鎭墠鍔犱笂鍐呮牳鐗堟湰锛坮elease锛?锛堝彲璇诲啓锛?	dev_name	鏈湴缃戠粶鎺ュ彛鍚嶇О			锛堝彲璇诲啓锛?	local_port	瑕佷娇鐢ㄧ殑婧?UDP 绔彛			锛堝彲璇诲啓锛?	remote_port	杩滅▼浠ｇ悊鐨?UDP 绔彛			锛堝彲璇诲啓锛?	local_ip	瑕佷娇鐢ㄧ殑婧?IP 鍦板潃			锛堝彲璇诲啓锛?	remote_ip	杩滅▼浠ｇ悊鐨?IP 鍦板潃			锛堝彲璇诲啓锛?	local_mac	鏈湴鎺ュ彛鐨?MAC 鍦板潃			锛堝彧璇伙級
	remote_mac	杩滅▼浠ｇ悊鐨?MAC 鍦板潃			锛堝彲璇诲啓锛?	transmit_errors	鏁版嵁鍖呭彂閫侀敊璇鏁?		锛堝彧璇伙級
	=============== =================================       ============

"enabled" 灞炴€ц繕鐢ㄤ簬鎺у埗鑳藉惁鏇存柊鐩爣鐨勫弬鏁扳€斺€斾綘鍙兘淇敼宸茬鐢ㄧ洰鏍囷紙鍗?"enabled" 涓?0锛夌殑鍙傛暟銆?
```
 cat enabled				# check if enabled is 1
 echo 0 > enabled			# disable the target (if required)
 echo eth2 > dev_name			# set local interface
 echo 10.0.0.4 > remote_ip		# update some parameter
 echo cb:a9:87:65:43:21 > remote_mac	# update more parameters
 echo 1 > enabled			# enable target again
```
浣犱篃鍙互鍔ㄦ€佸湴鏇存柊鏈湴鎺ュ彛銆傚鏋滀綘鎯宠浣跨敤鏂拌繎鍚姩锛堜笖鍦?netconsole 鍔犺浇/鍒濆鍖栨椂鍙兘杩樹笉瀛樺湪锛夌殑鎺ュ彛锛岃繖灏ゅ叾鏈夌敤銆?
鍦ㄥ紩瀵兼椂锛堟垨妯″潡鍔犺浇鏃讹級閫氳繃 `netconsole=` 鍙傛暟瀹氫箟鐨勭洰鏍囦細琚祴浜堝悕绉?`cmdline<index>`銆備緥濡傦紝鍙傛暟涓殑绗竴涓洰鏍囪鍛藉悕涓?`cmdline0`銆備綘鍙互閫氳繃鍒涘缓鍚屽悕 configfs 鐩綍鏉ユ帶鍒跺拰淇敼杩欎簺鐩爣銆?
```
 netconsole=4444@10.0.0.1/eth1,9353@10.0.0.2/12:34:56:78:9a:bc;4444@10.0.0.1/eth1,9353@10.0.0.3/12:34:56:78:9a:bc
```
```
 mkdir cmdline0
 cat cmdline0/remote_ip
 10.0.0.2

 mkdir cmdline1
 cat cmdline1/remote_ip
 10.0.0.3
```
### 杩藉姞鐢ㄦ埛鏁版嵁


鍦ㄥ惎鐢ㄤ簡 netconsole 鍔ㄦ€侀厤缃殑鎯呭喌涓嬶紝鍙互灏嗚嚜瀹氫箟鐢ㄦ埛鏁版嵁杩藉姞鍒版秷鎭殑鏈熬銆傜敤鎴锋暟鎹潯鐩彲浠ュ湪涓嶆洿鏀圭洰鏍?"enabled" 灞炴€х殑鎯呭喌涓嬭淇敼銆?
浣嶄簬 `userdata` 涓嬬殑鐩綍锛堥敭锛夐暱搴﹂檺鍒朵负 53 涓瓧绗︼紝骞朵笖
```
 cd /sys/kernel/config/netconsole && mkdir cmdline0
 cd cmdline0
 mkdir userdata/foo
 echo bar > userdata/foo/value
 mkdir userdata/qux
 echo baz > userdata/qux/value
```
```
 echo "This is a message" > /dev/kmsg
```
```
 12,607,22085407756,-;This is a message
  foo=bar
  qux=baz
```
```
 cd /sys/kernel/config/netconsole/cmdline0/userdata
 for f in `ls userdata`; do echo $f=$(cat userdata/$f/value); done
```
濡傛灉鍒涘缓浜?`userdata` 鏉＄洰浣嗘病鏈夊悜 `value` 鏂囦欢鍐欏叆鏁版嵁锛?```
 cd /sys/kernel/config/netconsole && mkdir cmdline0
 cd cmdline0
 mkdir userdata/foo
 echo bar > userdata/foo/value
 mkdir userdata/qux
```
```
 echo "This is a message" > /dev/kmsg
 12,607,22085407756,-;This is a message
  foo=bar
```
```
 rmdir /sys/kernel/config/netconsole/cmdline0/userdata/qux
```
   鍚戠敤鎴锋暟鎹€煎啓鍏ュ瓧绗︿覆鏃讹紝杈撳叆浼氭寜琛屾媶鍒?```
     mkdir userdata/testing
     printf "val1\nval2" > userdata/testing/value
     # userdata store value is called twice, first with "val1\n" then "val2"
     # so "val2" is stored, being the last value stored
     cat userdata/testing/value
     val2

   寤鸿涓嶈鍐欏叆甯︽湁鎹㈣绗︾殑鐢ㄦ埛鏁版嵁鍊笺€?```
### 鍦?userdata 涓嚜鍔ㄥ～鍏呬换鍔″悕


鍦?netconsole configfs 灞傜骇涓紝鏈変竴涓悕涓?`taskname_enabled` 鐨勬枃浠讹紝浣嶄簬 `userdata` 鐩綍涓嬨€傝鏂囦欢鐢ㄤ簬鍚敤鎴栫鐢ㄨ嚜鍔ㄤ换鍔″悕濉厖鐗规€с€傝鐗规€т細鑷姩濉厖褰撳墠姝ｅ湪璐熻矗鍙戦€佹秷鎭殑 CPU 涓婅璋冨害鐨勪换鍔＄殑鍚嶇О銆?
```
  echo 1 > /sys/kernel/config/netconsole/target1/userdata/taskname_enabled
```
褰撳惎鐢ㄨ閫夐」鍚庯紝netconsole 娑堟伅浼氬湪 userdata 瀛楁涓寘鍚竴琛岄澶栧唴瀹癸紝鏍煎紡涓?`taskname=<浠诲姟鍚?`銆傝繖浣垮緱 netconsole 娑堟伅鐨勬帴鏀舵柟鑳藉杞绘澗鎵惧嚭鐢熸垚璇ユ秷鎭椂褰撳墠琚皟搴︾殑搴旂敤绋嬪簭锛屼粠鑰屼负鍐呮牳娑堟伅鎻愪緵棰濆鐨勪笂涓嬫枃骞舵湁鍔╀簬瀵瑰叾鍒嗙被銆?
```
  echo "This is a message" > /dev/kmsg
  12,607,22085407756,-;This is a message
   taskname=echo
```
鍦ㄦ绀轰緥涓紝璇ユ秷鎭槸鍦?"echo" 浣滀负褰撳墠琚皟搴﹁繘绋嬫椂鐢熸垚鐨勩€?
### 鍦?userdata 涓嚜鍔ㄥ～鍏呭唴鏍哥増鏈紙release锛?

鍦?netconsole configfs 灞傜骇涓紝鏈変竴涓悕涓?`release_enabled` 鐨勬枃浠讹紝浣嶄簬 `userdata` 鐩綍涓嬨€傝鏂囦欢鎺у埗鍐呮牳鐗堟湰锛坮elease锛夎嚜鍔ㄥ～鍏呯壒鎬э紝瀹冧細灏嗗唴鏍哥増鏈俊鎭拷鍔犲埌鎵€鍙戦€佹瘡鏉℃秷鎭殑 userdata 瀛楀吀涓€?
```
  echo 1 > /sys/kernel/config/netconsole/target1/userdata/release_enabled
```
```
  echo "This is a message" > /dev/kmsg
  12,607,22085407756,-;This is a message
   release=6.14.0-rc6-01219-g3c027fbd941d
```
   璇ョ壒鎬ф彁渚涚殑鏁版嵁涓?"release prepend" 鐗规€х浉鍚屻€備笉杩囷紝鍦ㄨ繖绉嶆儏鍐典笅锛岀増鏈俊鎭槸琚拷鍔犲埌 userdata 瀛楀吀涓紝鑰屼笉鏄寘鍚湪娑堟伅澶撮噷銆?
### 鍦?userdata 涓嚜鍔ㄥ～鍏?CPU 缂栧彿


鍦?netconsole configfs 灞傜骇涓紝鏈変竴涓悕涓?`cpu_nr` 鐨勬枃浠讹紝浣嶄簬 `userdata` 鐩綍涓嬨€傝鏂囦欢鐢ㄤ簬鍚敤鎴栫鐢?CPU 缂栧彿鑷姩濉厖鐗规€с€傝鐗规€т細鑷姩濉厖姝ｅ湪鍙戦€佹秷鎭殑 CPU 鐨勭紪鍙枫€?
```
  echo 1 > /sys/kernel/config/netconsole/target1/userdata/cpu_nr
```
褰撳惎鐢ㄨ閫夐」鍚庯紝netconsole 娑堟伅浼氬湪 userdata 瀛楁涓寘鍚竴琛岄澶栧唴瀹癸紝鏍煎紡涓?`cpu=<cpu_number>`銆傝繖浣垮緱 netconsole 娑堟伅鐨勬帴鏀舵柟鑳藉杞绘澗鍖哄垎鍜岃В澶嶇敤鏉ヨ嚜涓嶅悓 CPU 鐨勬秷鎭紝鍦ㄥ鐞嗗苟琛屾棩蹇楄緭鍑烘椂灏ゅ叾鏈夌敤銆?
```
  echo "This is a message" > /dev/kmsg
  12,607,22085407756,-;This is a message
   cpu=42
```
鍦ㄦ绀轰緥涓紝璇ユ秷鎭敱 CPU 42 鍙戦€併€?
   濡傛灉鐢ㄦ埛宸插湪 userdata 瀛楀吀涓缃簡涓€涓啿绐佺殑 `cpu` 閿紝涓や釜閿兘浼氳鎶ュ憡锛屽叾涓唴鏍稿～鍏呯殑鏉＄洰鍑虹幇鍦ㄥ叾鍚?```
     # User-defined CPU entry
     mkdir -p /sys/kernel/config/netconsole/target1/userdata/cpu
     echo "1" > /sys/kernel/config/netconsole/target1/userdata/cpu/value

   Output might look like::

     12,607,22085407756,-;This is a message
      cpu=1
      cpu=42    # kernel-populated value
```
### 鍦?userdata 涓嚜鍔ㄥ～鍏呮秷鎭?ID


鍦?netconsole configfs 灞傜骇涓紝鏈変竴涓悕涓?`msgid_enabled` 鐨勬枃浠讹紝浣嶄簬 `userdata` 鐩綍涓嬨€傝鏂囦欢鎺у埗娑堟伅 ID 鑷姩濉厖鐗规€э紝瀹冧細涓哄彂閫佸埌缁欏畾鐩爣鐨勬瘡鏉℃秷鎭垎閰嶄竴涓暟鍊?ID锛屽苟灏嗚 ID 杩藉姞鍒版墍鍙戦€佹瘡鏉℃秷鎭殑 userdata 瀛楀吀涓€?
娑堟伅 ID 浣跨敤姣忎釜鐩爣涓€涓殑 32 浣嶈鏁板櫒鐢熸垚锛屾瘡鍚戣鐩爣鍙戦€佷竴鏉℃秷鎭氨閫掑涓€娆°€傝娉ㄦ剰锛岃璁℃暟鍣ㄥ湪杈惧埌 uint32_t 鏈€澶у€煎悗浼氬洖缁曪紝鍥犳娑堟伅 ID 鍦ㄩ暱鏃堕棿鑼冨洿鍐呭苟闈炲叏灞€鍞竴銆備笉杩囷紝鐩爣浠嶇劧鍙互鍒╃敤瀹冿紝閫氳繃璇嗗埆 ID 搴忓垪涓殑闂撮殭鏉ユ娴嬫秷鎭槸鍚﹀湪鍒拌揪鐩爣涔嬪墠琚涪寮冦€?
鍖哄垎娑堟伅 ID 涓庢秷鎭殑 <sequnum> 瀛楁寰堥噸瑕併€傛煇浜涘唴鏍告秷鎭彲鑳芥案杩滀笉浼氬埌杈?netconsole锛堜緥濡傜敱浜?printk 闄愰€燂級銆傚洜姝わ紝<sequnum> 涓殑闂撮殭涓嶈兘鍗曠嫭鐢ㄦ潵鎸囩ず娑堟伅鍦ㄤ紶杈撹繃绋嬩腑琚涪寮冿紝鍥犱负瀹冨彲鑳戒粠鏈€氳繃 netconsole 鍙戦€佽繃銆傚彟涓€鏂归潰锛屾秷鎭?ID 鍙垎閰嶇粰瀹為檯閫氳繃 netconsole 浼犺緭鐨勬秷鎭€?
```
  echo "This is message #1" > /dev/kmsg
  echo "This is message #2" > /dev/kmsg
  13,434,54928466,-;This is message #1
   msgid=1
  13,435,54934019,-;This is message #2
   msgid=2
```
## 鎵╁睍鎺у埗鍙帮細


濡傛灉閰嶇疆琛屽墠缂€浜?'+'锛屾垨鑰?"extended" 閰嶇疆鏂囦欢琚涓?1锛屽垯鍚敤鎵╁睍鎺у埗鍙版敮鎸併€備竴涓紩瀵?```
 linux netconsole=+4444@10.0.0.1/eth1,9353@10.0.0.2/12:34:56:78:9a:bc
```
鏃ュ織娑堟伅浼氫互鎵╁睍鍏冩暟鎹ご鐨勫舰寮忎紶杈?```
 <level>,<sequnum>,<timestamp>,<contflag>;<message text>
```
濡傛灉鍚敤浜?'r'锛坮elease锛夌壒鎬э紝鍒欎細鍦ㄦ秷鎭腑鍖呭惈鍐呮牳鐗堟湰鍙?```
 6.4.0,6,444,501151268,-;netconsole: network logging started
```
<message text> 涓殑涓嶅彲鎵撳嵃瀛楃浣跨敤 "\xff" 璁版硶杩涜杞箟銆傚鏋滄秷鎭寘鍚彲閫夌殑瀛楀吀锛屽垯浣跨敤鍘熸牱鐨勬崲琛岀浣滀负鍒嗛殧绗︺€?
濡傛灉涓€鏉℃秷鎭棤娉曟斁鍏ヤ竴瀹氭暟閲忕殑瀛楄妭锛堝綋鍓嶄负 1000锛変腑锛宯etconsole 浼氬皢鍏舵媶鍒嗕负澶氫釜鍒嗙墖銆傝繖浜?```
 ncfrag=<byte-offset>/<total-bytes>
```
渚嬪锛屽亣璁惧垎鍧楀ぇ灏忓皬寰楀锛屾秷鎭?"the first
```
 6,416,1758426,-,ncfrag=0/31;the first chunk,
 6,416,1758426,-,ncfrag=16/31; the 2nd chunk.
```
## 鏉傞」璇存槑锛?

   榛樿鐩爣鐨勪互澶綉璁剧疆浣跨敤骞挎挱浠ュお缃戝湴鍧€鏉ュ彂閫佹暟鎹寘锛岃繖浼氬鑷村悓涓€浠ュお缃戞涓婂叾浠栫郴缁熺殑璐熻浇澧炲姞銆?

   鏌愪簺 LAN 浜ゆ崲鏈哄彲鑳借閰嶇疆涓烘姂鍒朵互澶綉骞挎挱锛屽洜姝ゅ缓璁€氳繃浼犵粰 netconsole 鐨勯厤缃弬鏁版樉寮忔寚瀹氳繙绋嬩唬鐞嗙殑 MAC 鍦板潃銆?

```
	ping -c 1 10.0.0.2 ; /sbin/arp -n | grep 10.0.0.2
```
   濡傛灉杩滅▼鏃ュ織浠ｇ悊涓庡彂閫佹柟浣嶄簬涓嶅悓鐨?LAN 瀛愮綉锛屽缓璁皾璇曞皢榛樿缃戝叧鐨?MAC 鍦板潃锛堜綘鍙互浣跨敤 /sbin/route -n 鏌ュ埌锛夋寚瀹氫负杩滅▼ MAC 鍦板潃銆?

   缃戠粶璁惧锛堜笂杩颁緥瀛愪腑鐨?eth1锛夊彲浠ヨ繍琛屼换浣曠被鍨嬬殑鍏朵粬缃戠粶娴侀噺锛宯etconsole 涓嶄細閫犳垚骞叉壈銆傚鏋滃唴鏍告秷鎭噺寰堝ぇ锛宯etconsole 鍙兘浼氬鑷村叾浠栨祦閲忓嚭鐜拌交寰欢杩燂紝浣嗕笉搴斾骇鐢熷叾浠栧奖鍝嶃€?

   濡傛灉浣犲彂鐜拌繙绋嬫棩蹇椾唬鐞嗘病鏈夋帴鏀舵垨鎵撳嵃鍑哄彂閫佹柟鐨勬墍鏈夋秷鎭紝寰堝彲鑳芥槸鍥犱负浣犲皢鍙戦€佹柟涓婄殑 "console_loglevel" 鍙傛暟璁剧疆寰楀彧鍙戦€侀珮
```
	dmesg -n 8

   or by specifying "debug" on the kernel command line at boot, to send
   all kernel messages to the console. A specific value for this parameter
   can also be set using the "loglevel" kernel boot option. See the
   dmesg(8) man page and Documentation/admin-guide/kernel-parameters.rst
   for details.
```
Netconsole 琚璁′负灏藉彲鑳藉嵆鏃讹紝浠ヤ究鑳藉璁板綍鍗充娇鏄渶鍏抽敭鐨勫唴鏍?bug銆傚畠涔熷彲浠ュ湪 IRQ 涓婁笅鏂囦腑宸ヤ綔锛屽苟涓斿湪鍙戦€佹暟鎹寘鏃朵笉鍚敤涓柇銆傜敱浜庤繖浜涚嫭鐗圭殑闇€姹傦紝閰嶇疆鏃犳硶鏇村姞鑷姩鍖栵紝骞朵笖涓€浜涘熀鏈檺鍒跺皢闀挎湡瀛樺湪锛氫粎鏀寔 IP 缃戠粶銆乁DP 鏁版嵁鍖呭拰浠ュお缃戣澶囥€?