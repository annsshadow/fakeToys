
## Intel(R) Trace Hub (TH)锛堣嫳鐗瑰皵杩借釜涓灑锛?

### 姒傝堪


Intel(R) Trace Hub锛圱H锛岃嫳鐗瑰皵杩借釜涓灑锛夋槸涓€缁勭‖浠舵ā鍧楋紝鐢ㄤ簬缁忕敱澶氱绫诲瀷鐨?杩借釜杈撳嚭绔彛锛堥噰鐢?System Trace Protocol锛圡IPI STPv2锛夌紪鐮侊級浜х敓銆佸垏鎹㈠苟杈撳嚭
鏉ヨ嚜澶氫釜纭欢鍜岃蒋浠舵簮鐨勮拷韪暟鎹紝鏃ㄥ湪瀹炵幇鍏ㄧ郴缁熻皟璇曘€傛湁鍏宠纭欢鐨勬洿澶氫俊鎭紝
璇峰弬瑙?Intel(R) Trace Hub 寮€鍙戣€呮墜鍐?[^1^]銆?
瀹冪敱杩借釜婧愩€佽拷韪洰鏍囷紙杈撳嚭锛変互鍙婁竴涓垏鎹㈠櫒锛圙lobal Trace Hub锛孏TH锛屽叏灞€杩借釜涓灑锛?缁勬垚銆傝繖浜涜澶囨寕杞藉湪鍚勮嚜鐨勬€荤嚎锛?intel_th"锛変笂锛屽彲閫氳繃 sysfs 灞炴€ц鎺㈡祴涓庨厤缃€?
鐩墠鏀寔鐨?Intel TH 瀛愯澶囷紙妯″潡锛夊涓嬶細
  - Software Trace Hub锛圫TH锛岃蒋浠惰拷韪腑鏋級锛岃拷韪簮锛屾槸涓€涓?System Trace
    Module锛圫TM锛岀郴缁熻拷韪ā鍧楋級璁惧锛?  - Memory Storage Unit锛圡SU锛屽唴瀛樺瓨鍌ㄥ崟鍏冿級锛岃拷韪緭鍑猴紝鍏佽灏嗚拷韪腑鏋㈢殑
    杈撳嚭瀛樺偍鍦ㄧ郴缁熷唴瀛樹腑锛?  - Parallel Trace Interface output锛圥TI锛屽苟琛岃拷韪帴鍙ｈ緭鍑猴級锛岄€氳繃 PTI 绔彛
    灏嗚拷韪緭鍑哄埌澶栭儴璋冭瘯涓绘満锛?  - Global Trace Hub锛圙TH锛屽叏灞€杩借釜涓灑锛夛紝鍗充竴涓垏鎹㈠櫒锛屼篃鏄?Intel(R) Trace
    Hub 鏋舵瀯鐨勬牳蹇冪粍浠躲€?
杈撳嚭璁惧鐨勯€氱敤灞炴€у湪
Documentation/ABI/testing/sysfs-bus-intel_th-output-devices 涓湁璇存槑锛屽叾涓渶
鍊煎緱鍏虫敞鐨勬槸 "active"锛堟縺娲伙級锛岀敤浜庡惎鐢ㄦ垨绂佺敤鍚戣鐗瑰畾杈撳嚭璁惧鐨勮拷韪緭鍑恒€?
GTH 鍙€氳繃鍏?"masters"锛堜富璁惧锛夊睘鎬х粍灏嗕笉鍚岀殑 STP 涓昏澶囧鍚戜笉鍚岀殑杈撳嚭绔彛銆?鏇磋缁嗙殑 GTH 鎺ュ彛璇存槑瑙?Documentation/ABI/testing/sysfs-bus-intel_th-devices-gth銆?
STH 娉ㄥ唽涓€涓?stm class 璁惧锛屽苟缁忕敱瀹冨悜鐢ㄦ埛鎬佷笌鍐呮牳鎬佺殑杞欢杩借釜婧愭彁渚涙帴鍙ｃ€?鏇村淇℃伅璇峰弬瑙?Documentation/trace/stm.rst銆?
MSU 鍙閰嶇疆涓哄皢杩借釜鏁版嵁閲囬泦鍒扮郴缁熷唴瀛樼紦鍐插尯涓紝涔嬪悗鍙€氳繃鍏惰澶囪妭鐐逛互
read() 鎴?mmap() 鎺ュ彛璇诲彇锛屽苟瀵煎悜涓€涓?"software sink"锛堣蒋浠舵眹鑱氾級椹卞姩锛岀敱璇?椹卞姩娑堣垂鏁版嵁鍜?鎴栬繘涓€姝ヨ浆鍙戙€?
鎬讳綋鑰岃█锛孖ntel(R) Trace Hub 杩愯涓嶉渶瑕佷换浣曠壒娈婄殑鐢ㄦ埛鎬佽蒋浠讹紱涓€鍒囬兘鍙互閫氳繃
sysfs 灞炴€у拰璁惧鑺傜偣杩涜閰嶇疆銆佸惎鍔ㄤ笌閲囬泦銆?
[^1^] https://software.intel.com/sites/default/files/managed/d3/3c/intel-th-developer-manual.pdf

### 鎬荤嚎涓庡瓙璁惧


绯荤粺涓瘡涓?Intel TH 璁惧閮戒細鍒涘缓涓€鏉″睘浜庤嚜韬殑鎬荤嚎锛屽苟鍒嗛厤涓€涓?id 缂栧彿锛?璇ョ紪鍙峰弽鏄?TH 璁惧琚灇涓剧殑椤哄簭銆傛墍鏈?TH 瀛愯澶囷紙intel_th 鎬荤嚎涓婄殑璁惧锛夐兘浠?璇?id 寮€澶达細0-gth銆?-msc0銆?-msc1銆?-pti銆?-sth锛屽叾鍚庤窡闅忚澶囧悕绉颁互鍙婁竴涓?鍙€夌殑绱㈠紩銆?
杈撳嚭璁惧鍦?/dev/intel_thN 澶勪篃浼氳幏寰椾竴涓澶囪妭鐐癸紝鍏朵腑 N 涓?Intel TH 璁惧鐨?id銆備緥濡傦紝MSU 鐨勫唴瀛樼紦鍐插尯鍦ㄥ垎閰嶅悗鍙€氳繃 /dev/intel_th0/msc{0,1} 璁块棶銆?
### 蹇€熺ず渚?

```

	$ cat /sys/bus/intel_th/devices/0-msc0/port
	0

```
```

	$ echo 0 > /sys/bus/intel_th/devices/0-gth/masters/33

```
# 鍦ㄧ涓€涓唴瀛樼紦鍐插尯涓婂垎閰嶄竴涓?2 绐楀彛鐨?multiblock 缂撳啿鍖?```

	$ echo multi > /sys/bus/intel_th/devices/0-msc0/mode
	$ echo 64,64 > /sys/bus/intel_th/devices/0-msc0/nr_pages

```
```

	$ echo 1 > /sys/bus/intel_th/devices/0-msc0/wrap

```
```

	$ echo 1 > /sys/bus/intel_th/devices/0-msc0/active

```
# .. 鍚戜富璁惧 33 鍙戦€佹暟鎹紝鏇村缁嗚妭瑙?stm.txt ..
# .. 绛夊緟杩借釜鏁版嵁鍫嗙Н ..
```

	$ echo 0 > /sys/bus/intel_th/devices/0-msc0/active

```
```

	$ cat /dev/intel_th0/msc0 > my_stp_trace

```
### 涓绘満璋冭瘯鍣ㄦā寮?

鍙互閰嶇疆杩借釜涓灑锛屽苟浠庝竴涓€氳繃鏌愭潯纭欢璋冭瘯鎺ュ彛杩炴帴鐨勮繙绋嬭皟璇曚富鏈烘潵鎺у埗鍏?杩借釜閲囬泦锛涜鎺ュ彛闅忓悗鏃㈢敤浜庢帶鍒?Intel Trace Hub锛屼篃鐢ㄤ簬灏嗗畠鐨勮拷韪暟鎹紶杈撳埌
璋冭瘯涓绘満銆?
闇€瑕佸憡鐭ラ┍鍔ㄦ鍦ㄥ仛杩欐牱鐨勫畨鎺掞紝浠ヤ究瀹冧笉鍘昏Е纰颁换浣曢噰闆?绔彛閰嶇疆锛屽苟閬垮厤涓?璋冭瘯涓绘満鐨勯厤缃闂浉鍐茬獊銆傚湪姝ゆā寮忎笅锛岄┍鍔ㄥ敮涓€鎵ц鐨勬椿鍔ㄥ氨鏄皢杞欢杩借釜
鏀堕泦鍒?Software Trace Hub锛堜竴涓?stm class 璁惧锛夈€傜敤鎴蜂粛椤昏礋璐ｅ缓绔嬫帴鏀剁
瑙ｇ爜鍣ㄨ兘澶熻瘑鍒殑銆佸悎閫傜殑 master/channel锛堜富璁惧/閫氶亾锛夋槧灏勩€?
瑕佸惎鐢ㄤ富鏈烘ā寮忥紝璇峰皢 'intel_th' 鍐呮牳妯″潡鐨?'host_mode' 鍙傛暟璁句负 'y'銆俰ntel_th
鎬荤嚎涓婂皢涓嶄細鍑虹幇浠讳綍铏氭嫙杈撳嚭璁惧銆傚悓鏃讹紝'gth' 璁惧鐨勮拷韪厤缃笌閲囬泦鎺у埗灞炴€х粍
涔熶笉浼氳鏆撮湶銆?sth' 璁惧灏嗙収甯稿伐浣溿€?
### 杞欢姹囪仛锛圫oftware Sinks锛?

Memory Storage Unit锛圡SU锛夐┍鍔ㄦ彁渚涗簡涓€涓唴鏍告€?API锛屼緵鍏朵粬椹卞姩娉ㄥ唽涓鸿拷韪暟鎹?鐨勮蒋浠舵眹鑱氥€傛绫婚┍鍔ㄥ彲杩涗竴姝ラ€氳繃鍏朵粬璁惧锛堝 USB 璁惧鎺у埗鍣ㄦ垨缃戝崱锛夊鍑烘暟鎹€?
```
 - 閫氱煡杞欢姹囪仛鏌愪釜鐗瑰畾绐楀彛宸插啓婊★紝骞?閿佸畾"璇ョ獥鍙ｏ紙鍗充护鍏朵笉鍐嶅彲鐢ㄤ簬杩借釜
   閲囬泦锛夛紱鍙戠敓杩欑鎯呭喌鏃讹紝MSU 椹卞姩浼氳嚜鍔ㄥ垏鎹㈠埌缂撳啿鍖轰腑鐨勪笅涓€涓獥鍙ｏ紙濡傛灉瀹?   鏈閿佸畾锛夛紝鍚﹀垯灏嗗仠姝㈣拷韪噰闆嗭紱
 - 璺熻釜绐楀彛鐨?閿佸畾"鐘舵€侊紝骞朵负杞欢姹囪仛椹卞姩鎻愪緵涓€绉嶆柟寮忥紝浠ヤ究鍦ㄦ煇涓獥鍙ｈ
   瑙ｉ攣銆佸彲鍐嶆鐢ㄤ簬閲囬泦杩借釜鏁版嵁鏃堕€氱煡 MSU 椹卞姩銆?
```
绀轰緥姹囪仛椹卞姩 msu-sink 婕旂ず浜嗚蒋浠舵眹鑱氱殑瀹炵幇銆備粠鍔熻兘涓婅锛屽畠鍙槸鍦ㄧ獥鍙ｄ竴鍐欐弧
灏辫В閿侊紝浣?MSU 浠ュ惊鐜紦鍐插尯妯″紡鎸佺画杩愯銆備笌 "multi"锛堝绐楀彛锛夋ā寮忎笉鍚岋紝瀹冧細
濉弧缂撳啿鍖轰腑鐨勬墍鏈夌獥鍙ｏ紝鑰岄潪浠呯涓€涓€傚彲閫氳繃鍚?"mode" 鏂囦欢鍐欏叆 "sink" 鏉ュ惎鐢?锛堝墠鎻愭槸 msu-sink.ko 宸插姞杞斤級銆?