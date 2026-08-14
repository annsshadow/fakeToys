
## CoreSight - Perf


    :Author:   Carsten Haitzler <carsten.haitzler@arm.com>
    :Date:     June 29th, 2022

Perf 鑳藉鍦ㄦ湰鍦拌闂?CoreSight 璺熻釜鏁版嵁骞跺皢鍏跺瓨鍌ㄥ埌杈撳嚭鐨?perf 鏁版嵁鏂囦欢涓€傝繖浜涙暟鎹殢鍚庡彲浠ヨ瑙ｇ爜锛屼互缁欏嚭琚窡韪殑鎸囦护锛岀敤浜庤皟璇曟垨鎬ц兘鍒嗘瀽銆備綘
```

   perf record -e cs_etm//u testbinary

```
杩欎細杩愯鏌愪釜娴嬭瘯浜岃繘鍒舵枃浠讹紙testbinary锛夌洿鍒板畠閫€鍑猴紝骞惰褰曚竴涓?perf.data 璺熻釜鏂囦欢銆傚鏋?CoreSight 宸ヤ綔姝ｅ父锛岃鏂囦欢浼氬寘鍚?AUX 娈点€備綘鍙互杩欐牱杞偍璇ユ枃浠剁殑鍐呭锛?```

   perf report --stdio --dump -i perf.data

```
```

   0x1e78 [0x30]: PERF_RECORD_AUXTRACE size: 0x11dd0  offset: 0  ref: 0x1b614fc1061b0ad1  idx: 0  tid: 531230  cpu: -1

   . ... CoreSight ETM Trace data: size 73168 bytes
           Idx:0; ID:10;   I_ASYNC : Alignment Synchronisation.
             Idx:12; ID:10;  I_TRACE_INFO : Trace Info.; INFO=0x0 { CC.0 }
             Idx:17; ID:10;  I_ADDR_L_64IS0 : Address, Long, 64 bit, IS0.; Addr=0x0000000000000000;
             Idx:26; ID:10;  I_TRACE_ON : Trace On.
             Idx:27; ID:10;  I_ADDR_CTXT_L_64IS0 : Address & Context, Long, 64 bit, IS0.; Addr=0x0000FFFFB6069140; Ctxt: AArch64,EL0, NS;
             Idx:38; ID:10;  I_ATOM_F6 : Atom format 6.; EEEEEEEEEEEEEEEEEEEEEEEE
             Idx:39; ID:10;  I_ATOM_F6 : Atom format 6.; EEEEEEEEEEEEEEEEEEEEEEEE
             Idx:40; ID:10;  I_ATOM_F6 : Atom format 6.; EEEEEEEEEEEEEEEEEEEEEEEE
             Idx:41; ID:10;  I_ATOM_F6 : Atom format 6.; EEEEEEEEEEEEN
             ...

```
濡傛灉浣犵湅鍒颁笂杩板唴瀹癸紝璇存槑浣犵殑绯荤粺姝ｅ湪姝ｇ‘鍦拌窡韪?CoreSight 鏁版嵁銆?```

   make CORESIGHT=1

```
鏋勫缓瀹冿紙鎸囧甫 CoreSight 鏀寔鐨?perf锛夐渶瑕?OpenCSD銆備綘鍙互瀹夎鍙戣鐗堟彁渚涚殑鏀寔鍖咃紝濡?libopencsd 鍜?libopencsd-dev锛屾垨鑰呬笅杞芥簮鐮佽嚜琛屾瀯寤恒€侽penCSD 涓婃父浠ｇ爜浣嶄簬锛?
  https://github.com/Linaro/OpenCSD

鏈夊叧鏋勫缓甯?CoreSight 鏀寔鐨?perf 浠ュ強鏇磋灏界敤娉曠殑瀹屾暣淇℃伅锛岃鍙傝锛?
  https://github.com/Linaro/OpenCSD/blob/master/HOWTO.md


### 鍐呮牳 CoreSight 鏀寔


浣犺繕搴旇鍦ㄤ綘鐨勫唴鏍搁厤缃腑鍚敤 CoreSight 鏀寔銆?```

   CONFIG_CORESIGHT=y

```
浣犲彲鑳借繕鎯宠鍚敤鍏朵粬鍚勭 CoreSight 閫夐」
```

   CONFIG_CORESIGHT_LINKS_AND_SINKS=y
   CONFIG_CORESIGHT_LINK_AND_SINK_TMC=y
   CONFIG_CORESIGHT_CATU=y
   CONFIG_CORESIGHT_SINK_TPIU=y
   CONFIG_CORESIGHT_SINK_ETBV10=y
   CONFIG_CORESIGHT_SOURCE_ETM4X=y
   CONFIG_CORESIGHT_CTI=y
   CONFIG_CORESIGHT_CTI_INTEGRATION_REGS=y

```
鏇村淇℃伅璇峰弬闃呭唴鏍搁厤缃府鍔┿€?
### 浣跨敤 AUX 鏆傚仠涓庢仮澶嶈繘琛岀粏绮掑害璺熻釜


Arm CoreSight 鍙兘浜х敓澶ч噺鐨勭‖浠惰窡韪暟鎹紝杩欎細甯︽潵璁板綍寮€閿€锛屽苟鍦ㄦ煡鐪嬫€ц兘鍒嗘瀽缁撴灉鏃跺垎鏁ｇ敤鎴锋敞鎰忓姏銆備负浜嗙紦瑙ｈ繃澶氳窡韪暟鎹殑闂锛孭erf 鎻愪緵浜?AUX 鏆傚仠锛坧ause锛夊拰鎭㈠锛坮esume锛夊姛鑳斤紝浠ュ疄鐜扮粏绮掑害璺熻釜銆?
AUX 鏆傚仠鍜屾仮澶嶅彲浠ョ敱鍏宠仈鐨勪簨浠惰Е鍙戙€傝繖浜涗簨浠跺彲浠ユ槸 ftrace 璺熻釜鐐癸紙鍖呮嫭闈欐€佸拰鍔ㄦ€佽窡韪偣锛夋垨 PMU 浜嬩欢锛堜緥濡?CPU PMU 鍛ㄦ湡浜嬩欢锛夈€備负浜嗗垱寤轰竴涓甫鏈?AUX 鏆傚仠/鎭㈠鐨?perf 浼氳瘽锛屽紩鍏ヤ簡涓変釜閰嶇疆椤癸細

- "aux-action=start-paused"锛氫负 cs_etm PMU 浜嬩欢鎸囧畾锛屼娇鍏朵互鏆傚仠鐘舵€佸惎鍔ㄣ€?- "aux-action=pause"锛氱敤姝ら厤缃」鎸囧畾涓€涓叧鑱斾簨浠朵互鏆傚仠 AUX 璺熻釜銆?- "aux-action=resume"锛氱敤姝ら厤缃」鎸囧畾涓€涓叧鑱斾簨浠朵互鎭㈠ AUX 璺熻釜銆?```

  perf record -e cs_etm/aux-action=start-paused/k,syscalls:sys_enter_openat/aux-action=resume/,syscalls:sys_exit_openat/aux-action=pause/ ls

```
```

  perf record -a -e cs_etm/aux-action=start-paused/k \
        -e cycles/aux-action=pause,period=10000000/ \
        -e cycles/aux-action=resume,period=1050000/ -- sleep 1

```
### Perf 娴嬭瘯 - 楠岃瘉鍐呮牳涓庣敤鎴风┖闂?perf CoreSight 鏄惁宸ヤ綔


褰撲綘杩愯 perf test 鏃讹紝瀹冧細杩涜澶ч噺鑷祴璇曘€傚叾涓竴浜涙祴璇曚細瑕嗙洊 CoreSight锛堜粎鍦ㄥ惎鐢ㄤ笖浣嶄簬 ARM64 涓婃椂锛夈€傞€氬父浣犱細鍦ㄥ唴鏍告爲鐨?tools/perf 鐩綍涓嬭繍琛?perf test銆備竴浜涙祴璇曚細妫€鏌ユ煇浜涘唴閮?perf 鏀寔锛屼緥濡傦細

   Check Arm CoreSight trace data recording and synthesized samples
   妫€鏌?Arm CoreSight 璺熻釜鏁版嵁璁板綍涓庡悎鎴愰噰鏍?   Check Arm SPE trace data recording and synthesized samples
   妫€鏌?Arm SPE 璺熻釜鏁版嵁璁板綍涓庡悎鎴愰噰鏍?
鍙︿竴浜涙祴璇曚細瀹為檯浣跨敤 perf record 浠ュ強 tests/shell/coresight 涓殑涓€浜涙祴璇曚簩杩涘埗鏂囦欢锛屽苟鏀堕泦璺熻釜浠ョ‘淇濊揪鍒版渶浣庣殑鍔熻兘姘村钩銆傚惎鍔ㄨ繖浜涙祴璇曠殑鑴氭湰浣嶄簬鍚屼竴鐩綍涓€傚畠浠湅璧锋潵閮藉儚锛?
   CoreSight / ASM 绾惊鐜?   CoreSight / Memcpy 16k 10 绾跨▼
   CoreSight / 绾跨▼寰幆 10 绾跨▼ - 妫€鏌?TID
   etc.

濡傛灉宸ュ叿浜岃繘鍒舵枃浠朵笉瀛樺湪浜?tests/shell/coresight\*/ 涓紝杩欎簺 perf record 娴嬭瘯灏嗕笉浼氳繍琛岋紝鑰屾槸琚烦杩囥€傚鏋滀綘鐨勭‖浠朵笉鏀寔 CoreSight锛岄偅涔堣涔堜笉鏋勫缓甯?CoreSight 鏀寔鐨?perf锛岃涔堢Щ闄よ繖浜涗簩杩涘埗鏂囦欢锛屼互鍏嶈繖浜涙祴璇曞け璐ワ紝璁╁畠浠敼涓鸿璺宠繃銆?
杩欎簺娴嬭瘯浼氬湪褰撳墠宸ヤ綔鐩綍锛堜緥濡?tools/perf锛変腑璁板綍鍘嗗彶缁撴灉锛屽苟浠?stats-\*.csv 杩欐牱鐨勫悕绉板懡鍚嶏紝渚嬪锛?
   stats-asm_pure_loop-out.csv
   stats-memcpy_thread-16k_10.csv
   ...

杩欎簺缁熻鏂囦欢璁板綍 perf 鏁版嵁杈撳嚭涓?AUX 鏁版嵁娈电殑鏌愪簺鏂归潰锛岀粺璁℃煇浜涚壒瀹氱紪鐮佺殑鏁伴噺锛堜竴绉嶄互闈炲父绠€鍗曠殑鏂瑰紡纭鍏舵槸鍚︽甯稿伐浣滅殑鏂规硶锛夈€侰oreSight 鐨勪竴涓棶棰樻槸锛屽綋闇€瑕佽褰曠殑寰呰褰曟暟鎹噺瓒冲澶ф椂锛屽叾涓竴閮ㄥ垎鍙兘浼氱敱浜庡鐞嗗櫒鏈兘鍙婃椂鍞ら啋浠ヤ粠缂撳啿鍖鸿鍑烘墍鏈夋暟鎹瓑鍘熷洜鑰屼涪澶便€備綘浼氭敞鎰忓埌姣忔杩愯 perf test 鏀堕泦鍒扮殑鏁版嵁閲忓彲鑳芥湁寰堝ぇ宸紓銆傚鏋滀綘鎯宠瀵熷畠闅忔椂闂村浣曞彉鍖栵紝鍙渶澶氭杩愯 perf test锛屾墍鏈夎繖浜?csv 鏂囦欢閮戒細涓嶆柇杩藉姞鏇村鏁版嵁锛屼緵浣犱箣鍚庢鏌ャ€佺粯鍥炬垨浠ュ叾浠栨柟寮忎娇鐢紝鏉ュ垽鏂儏鍐靛彉濂借繕鏄彉鍧忋€?
杩欐剰鍛崇潃鏈夋椂杩欎簺娴嬭瘯浼氬け璐ワ紝鍥犱负瀹冧滑娌℃湁鎹曡幏鍒版墍闇€鐨勫叏閮ㄦ暟鎹€傝繖鍏充箮闅忔椂闂磋窡韪墍浜х敓鐨勬暟鎹川閲忓拰鏁伴噺锛屼互鍙婅瀵熷 Linux 鍐呮牳鐨勬洿鏀逛綍鏃舵敼鍠勪簡璺熻釜璐ㄩ噺銆?
璇锋敞鎰忥紝鍏朵腑涓€浜涙祴璇曡繍琛屾椂闂寸浉褰撻暱锛岀壒鍒槸鍦ㄥ鐞?perf 鏁版嵁鏂囦欢骞惰浆鍌ㄥ唴瀹逛互妫€鏌ュ叾鍐呴儴鏃躲€?
浣犲彲浠ラ€氳繃鍦ㄨ繍琛?perf 涔嬪墠璁剧疆 PERF_TEST_CORESIGHT_STATDIR 鐜鍙橀噺鏉ユ敼鍙樿繖浜?csv 鏃ュ織鐨勫瓨鍌ㄤ綅缃?```

   export PERF_TEST_CORESIGHT_STATDIR=/var/tmp
   perf test

```
瀹冧滑杩樹細灏嗙敓鎴愮殑 perf 杈撳嚭鏁版嵁瀛樺偍鍦ㄥ綋鍓?```

   perf-asm_pure_loop-out.data
   perf-memcpy_thread-16k_10.data
   ...

```
浣犲彲浠ラ€氳繃璁剧疆
```

   PERF_TEST_CORESIGHT_DATADIR=/var/tmp
   perf test

```
鏉ユ敼鍙?perf 鏁版嵁鏂囦欢鐨勫瓨鍌ㄤ綅缃€傚鏋滀綘甯屾湜灏嗘祴璇曡緭鍑轰繚瀛樺湪褰撳墠宸ヤ綔鐩綍涔嬪浠ヨ繘琛岄暱鏈熷瓨鍌ㄥ拰妫€鏌ワ紝鍙互璁剧疆涓婅堪鐜鍙橀噺銆?