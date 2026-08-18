## Cgroup 鍐荤粨鍣紙Freezer锛?

cgroup 鍐荤粨鍣ㄥ鎵瑰鐞嗕綔涓氱鐞嗙郴缁熷緢鏈夌敤锛岃繖绫荤郴缁熶細鎸夌郴缁熺鐞嗗憳鐨勬剰鎰?鍚姩鍜屽仠姝竴缁勪换鍔★紝浠ヨ皟搴︽満鍣ㄧ殑璧勬簮銆傝繖绉嶇▼搴忓父鐢ㄤ簬 HPC 闆嗙兢锛屼互
鏁翠綋璋冨害瀵归泦缇ょ殑璁块棶銆俢group 鍐荤粨鍣ㄤ娇鐢?cgroup 鏉ユ弿杩拌鐢辨壒澶勭悊浣滀笟绠＄悊
绯荤粺鍚姩/鍋滄鐨勪换鍔￠泦鍚堛€傚畠杩樻彁渚涗簡鍚姩鍜屽仠姝㈢粍鎴愯浣滀笟鐨勪换鍔＄殑鎵嬫銆?
cgroup 鍐荤粨鍣ㄥ妫€鏌ョ偣锛坈heckpointing锛夋鍦ㄨ繍琛岀殑浠诲姟缁勪篃寰堟湁鐢ㄣ€傚喕缁撳櫒
鍏佽妫€鏌ョ偣浠ｇ爜閫氳繃灏濊瘯灏?cgroup 涓殑浠诲姟寮哄埗杩涘叆闈欐锛坬uiescent锛夌姸鎬佹潵鑾峰彇
浠诲姟鐨勪竴鑷撮暅鍍忋€備竴鏃︿换鍔￠潤姝紝鍙︿竴涓换鍔″氨鍙互閬嶅巻 /proc 鎴栬皟鐢ㄥ唴鏍告帴鍙ｆ潵
鏀堕泦鏈夊叧杩欎簺闈欐浠诲姟鐨勪俊鎭€傚鏋滃彂鐢熷彲鎭㈠鐨勯敊璇紝琚鏌ョ偣鐨勪换鍔″彲浠ュ湪
涔嬪悗閲嶆柊鍚姩銆傝繖涔熷厑璁搁€氳繃灏嗘敹闆嗗埌鐨勪俊鎭鍒跺埌鍙︿竴涓妭鐐瑰苟鍦ㄩ偅閲岄噸鍚换鍔★紝
鍦ㄩ泦缇や腑鐨勮妭鐐逛箣闂磋縼绉昏妫€鏌ョ偣鐨勪换鍔°€?
鍦ㄧ敤鎴风┖闂翠腑锛孲IGSTOP 涓?SIGCONT 鐨勫簭鍒楀苟涓嶆€绘槸瓒充互鍋滄鍜屾仮澶嶄换鍔°€傝繖涓や釜
淇″彿閮藉彲浠ヤ粠鎴戜滑甯屾湜鍐荤粨鐨勪换鍔″唴閮ㄨ瀵熷埌銆傝櫧鐒?SIGSTOP 涓嶈兘琚崟鑾枫€侀樆濉炴垨
蹇界暐锛屼絾瀹冨彲浠ヨ绛夊緟鎴?ptrace 鐨勭埗浠诲姟鐪嬪埌銆係IGCONT 灏ゅ叾涓嶅悎閫傦紝鍥犱负瀹冨彲浠ヨ
浠诲姟鎹曡幏銆備换浣曡璁＄敤鏉ョ洃瑙?SIGSTOP 鍜?SIGCONT 鐨勭▼搴忛兘鍙兘鍥犲皾璇曚娇鐢?SIGSTOP 鍜?SIGCONT 鏉ュ仠姝㈠拰鎭㈠浠诲姟鑰岃鐮村潖銆傛垜浠彲浠?```

	$ echo $$
	16644
	$ bash
	$ echo $$
	16690

	From a second, unrelated bash shell:
	$ kill -SIGSTOP 16690
	$ kill -SIGCONT 16690

	<at this point 16690 exits and causes 16644 to exit too>

```
杩欏彂鐢熸槸鍥犱负 bash 鍙互瑙傚療鍒拌繖涓や釜淇″彿骞堕€夋嫨濡備綍鍝嶅簲瀹冧滑銆?
鍙︿竴涓崟鑾峰苟鍝嶅簲杩欎簺淇″彿鐨勭▼搴忕ず渚嬫槸 gdb銆備簨瀹炰笂锛屼换浣曡璁′娇鐢?ptrace 鐨?绋嬪簭閮藉彲鑳藉湪浣跨敤杩欑鍋滄鍜屾仮澶嶄换鍔＄殑鏂规硶鏃堕亣鍒伴棶棰樸€?
鐩稿弽锛宑group 鍐荤粨鍣ㄤ娇鐢ㄥ唴鏍稿喕缁撳櫒浠ｇ爜锛岄槻姝㈠喕缁?瑙ｅ喕鍛ㄦ湡瀵硅鍐荤粨鐨勪换鍔?鍙銆傝繖浣垮緱涓婇潰鐨?bash 绀轰緥鍜?gdb 鑳藉濡傞鏈熻埇杩愯銆?
cgroup 鍐荤粨鍣ㄦ槸鍒嗗眰鐨勩€傚喕缁撲竴涓?cgroup 浼氬喕缁撳睘浜庤 cgroup 鍙婂叾鎵€鏈夊悗浠?cgroup 鐨勬墍鏈変换鍔°€傛瘡涓?cgroup 閮芥湁鑷繁鐨勭姸鎬侊紙self-state锛岃嚜韬姸鎬侊級浠ュ強
浠庣埗绾х户鎵跨殑鐘舵€侊紙parent-state锛岀埗鐘舵€侊級銆傚綋涓斾粎褰撲袱涓姸鎬侀兘涓?THAWED 鏃讹紝
璇?cgroup 鎵嶆槸 THAWED銆?
cgroup 鍐荤粨鍣ㄥ垱寤轰互涓?cgroupfs 鏂囦欢銆?
- freezer.state锛氬彲璇诲啓銆?
  璇诲彇鏃讹紝杩斿洖 cgroup 鐨勬湁鏁堢姸鎬佲€斺€斺€淭HAWED鈥濄€佲€淔REEZING鈥濇垨鈥淔ROZEN鈥濄€?  杩欐槸鑷韩鐘舵€佷笌鐖剁姸鎬佺殑缁撳悎銆傚鏋滀换涓€涓鍦ㄥ喕缁擄紝鍒欒 cgroup 姝ｅ湪鍐荤粨
  锛團REEZING 鎴?FROZEN锛夈€?
  FREEZING 鐨?cgroup 鍦ㄥ睘浜庤 cgroup 鍙婂叾鎵€鏈夊悗浠ｇ殑浠诲姟閮藉彉涓哄喕缁撴椂锛?  杞崲涓?FROZEN 鐘舵€併€傛敞鎰忥紝鍦ㄥ皢涓€涓柊浠诲姟娣诲姞鍒拌 cgroup 鎴栧叾鏌愪釜鍚庝唬
  cgroup 涔嬪悗锛岀洿鍒版柊浠诲姟琚喕缁撲箣鍓嶏紝cgroup 浼氫粠 FROZEN 鍥為€€鍒?FREEZING銆?
  鍐欏叆鏃讹紝璁剧疆 cgroup 鐨勮嚜韬姸鎬併€傚厑璁镐袱涓€尖€斺€斺€淔ROZEN鈥濆拰鈥淭HAWED鈥濄€?  濡傛灉鍐欏叆 FROZEN锛屽垯璇?cgroup锛堝鏋滃皻鏈湪鍐荤粨涓級杩炲悓鍏舵墍鏈夊悗浠?cgroup
  涓€璧疯繘鍏?FREEZING 鐘舵€併€?
  濡傛灉鍐欏叆 THAWED锛屽垯 cgroup 鐨勮嚜韬姸鎬佹敼涓?THAWED銆傛敞鎰忥紝濡傛灉鐖剁姸鎬佷粛鍦?  鍐荤粨涓紝鏈夋晥鐘舵€佸彲鑳戒笉浼氭敼鍙樹负 THAWED銆傚鏋滄煇涓?cgroup 鐨勬湁鏁堢姸鎬佸彉涓?  THAWED锛屽垯鎵€鏈夊洜鍏跺喕缁撶殑鍚庝唬涔熶細绂诲紑鍐荤粨鐘舵€併€?
- freezer.self_freezing锛氬彧璇汇€?
  鏄剧ず鑷韩鐘舵€併€傚鏋滆嚜韬姸鎬佷负 THAWED 鍒欎负 0锛屽惁鍒欎负 1銆?  褰撲笖浠呭綋瀵?freezer.state 鐨勬渶鍚庝竴娆″啓鍏ユ槸 鈥淔ROZEN鈥?鏃讹紝璇ュ€间负 1銆?
- freezer.parent_freezing锛氬彧璇汇€?
  鏄剧ず鐖剁姸鎬併€傚鏋滆 cgroup 鐨勭鍏堥兘娌℃湁琚喕缁撳垯涓?0锛屽惁鍒欎负 1銆?
鏍?cgroup 鏄笉鍙喕缁撶殑锛屼笂杩版帴鍙ｆ枃浠朵笉瀛樺湪銆?
```

   # mkdir /sys/fs/cgroup/freezer
   # mount -t cgroup -ofreezer freezer /sys/fs/cgroup/freezer
   # mkdir /sys/fs/cgroup/freezer/0
   # echo $some_pid > /sys/fs/cgroup/freezer/0/tasks

```
```

   # cat /sys/fs/cgroup/freezer/0/freezer.state
   THAWED

```
```

   # echo FROZEN > /sys/fs/cgroup/freezer/0/freezer.state
   # cat /sys/fs/cgroup/freezer/0/freezer.state
   FREEZING
   # cat /sys/fs/cgroup/freezer/0/freezer.state
   FROZEN

```
```

   # echo THAWED > /sys/fs/cgroup/freezer/0/freezer.state
   # cat /sys/fs/cgroup/freezer/0/freezer.state
   THAWED

```
杩欐槸涓€涓熀鏈満鍒讹紝鍦ㄧ畝鍗曞満鏅笅搴旇鑳戒负鐢ㄦ埛绌洪棿浠诲姟鍋氭纭殑浜嬨€?
璇ュ喕缁撳櫒瀹炵幇鍙楀埌缂洪櫡鐨勫奖鍝嶏紙鍙傝鎻愪氦 76f969e8948d8锛堚€渃group: cgroup v2 freezer鈥濓級锛夛紝
寤鸿浣跨敤 cgroup v2 鍐荤粨鍣ㄣ€?