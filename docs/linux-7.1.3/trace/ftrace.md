## ftrace - 鍑芥暟璺熻釜鍣?


鐗堟潈鎵€鏈?2008 Red Hat 鍏徃銆?

:浣滆€?   Steven Rostedt <srostedt@redhat.com>
:璁稿彲璇?  The GNU Free Documentation License, Version 1.2
          (鍦?GPL v2 涓嬪弻閲嶈鍙?

:鍘熷瀹￠槄鑰?  Elias Oltmanns, Randy Dunlap, Andrew Morton,
		      John Kacur, and David Teigland.

- 缂栧啓閽堝: 2.6.28-rc2
- 鏇存柊閽堝: 3.10
- 鏇存柊閽堝: 4.13 - 鐗堟潈鎵€鏈?2017 VMware 鍏徃 Steven Rostedt
- 杞崲涓?rst 鏍煎紡 - Changbin Du <changbin.du@intel.com>

### 绠€浠?


ftrace 鏄竴涓唴閮ㄨ窡韪櫒锛屾棬鍦ㄥ府鍔╁紑鍙戣€呬笌绯荤粺璁捐鑰呬簡瑙ｅ唴鏍稿唴閮?
姝ｅ湪鍙戠敓浠€涔堛€傚畠鍙敤浜庤皟璇曟垨鍒嗘瀽鍙戠敓鍦ㄧ敤鎴风┖闂翠箣澶栫殑寤惰繜鍜屾€ц兘闂銆?

铏界劧 ftrace 閫氬父琚涓烘槸涓€涓嚱鏁拌窡韪櫒锛屼絾瀹冨疄闄呬笂鏄敱澶氫釜涓嶅悓
璺熻釜宸ュ叿缁勬垚鐨勬鏋躲€傚叾涓寘鎷欢杩熻窡韪紝鐢ㄤ簬妫€鏌ヤ粠涓柇绂佺敤鍒板惎鐢ㄤ箣闂淬€?
浠ュ強鎶㈠崰涔嬮棿銆佷粠浠诲姟琚敜閱掑埌璇ヤ换鍔＄湡姝ｈ璋冨害杩涙潵涔嬮棿鎵€鍙戠敓鐨勬儏鍐点€?

ftrace 鏈€甯歌鐨勭敤閫斾箣涓€鏄簨浠惰窡韪€傚唴鏍镐腑閬嶅竷鏁扮櫨涓潤鎬佷簨浠剁偣锛?
鍙互閫氳繃 tracefs 鏂囦欢绯荤粺鍚敤锛屼互鏌ョ湅鍐呮牳鏌愪簺閮ㄥ垎鍐呴儴姝ｅ湪鍙戠敓浠€涔堛€?

鏇村淇℃伅璇峰弬闃?events.rst銆?


### 瀹炵幇缁嗚妭


鏋舵瀯绉绘鑰呯瓑鐩稿叧缁嗚妭璇峰弬闃?Documentation/trace/ftrace-design.rst銆?


### 鏂囦欢绯荤粺


ftrace 浣跨敤 tracefs 鏂囦欢绯荤粺鏉ヤ繚瀛樻帶鍒舵枃浠朵互鍙婄敤浜庢樉绀鸿緭鍑虹殑鏂囦欢銆?

褰?tracefs 琚厤缃繘鍐呮牳锛堥€夋嫨浠绘剰 ftrace 閫夐」閮戒細濡傛锛夋椂锛屼細鍒涘缓
鐩綍 /sys/kernel/tracing銆傛寕杞芥柟寮忓涓嬶細

```

 tracefs       /sys/kernel/tracing       tracefs defaults        0       0

```

```

 mount -t tracefs nodev /sys/kernel/tracing

```
涓轰簡鏂逛究璁块棶璇ョ洰褰曪紝浣犲彲鑳芥兂瑕佸缓绔嬩竴涓蒋閾炬帴锛?

```

 ln -s /sys/kernel/tracing /tracing

```

  鍦?4.1 涔嬪墠锛屾墍鏈?ftrace 璺熻釜鎺у埗鏂囦欢閮戒綅浜?debugfs 鏂囦欢绯荤粺涓紝
  閫氬父浣嶄簬 /sys/kernel/debug/tracing銆備负浜嗗悜鍚庡吋瀹癸紝褰撴寕杞?debugfs
  鏂囦欢绯荤粺銆乼racefs 鏂囦欢绯荤粺浼氳嚜鍔ㄦ寕杞藉埌锛?

  /sys/kernel/debug/tracing

  浣嶄簬 tracefs 鏂囦欢绯荤粺涓殑鍏ㄩ儴鏂囦欢涔熶細鍑虹幇鍦ㄨ debugfs 鏂囦欢绯荤粺
  鐩綍涓€?

  浠讳綍琚€変腑鐨?ftrace 閫夐」涔熶細鍒涘缓 tracefs 鏂囦欢绯荤粺銆傛枃妗ｇ殑鍏朵綑閮ㄥ垎
  灏嗗亣璁句綘澶勪簬 ftrace 鐩綍涓紙cd /sys/kernel/tracing锛夛紝骞朵笖鍙細
  鍏虫敞璇ョ洰褰曞唴鐨勬枃浠讹紝鑰屼笉浼氱敤鍐楅暱鐨?"/sys/kernel/tracing" 璺緞鍚?
  鏉ュ垎鏁ｅ鍐呭鐨勬敞鎰忓姏銆?

灏辫繖鏍凤紒锛堝亣璁句綘宸茬粡灏?ftrace 閰嶇疆杩涗簡鍐呮牳锛?

鎸傝浇 tracefs 涔嬪悗锛屼綘灏嗗彲浠ヨ闂?ftrace 鐨勬帶鍒跺拰杈撳嚭鏂囦欢銆備互涓嬫槸鍏朵腑
涓€浜涘叧閿枃浠剁殑鍒楄〃锛?


 娉ㄦ剰锛氭墍鏈夋椂闂村€煎潎浠ュ井绉掍负鍗曚綅銆?

  current_tracer:

	璇ユ枃浠剁敤浜庤缃垨鏄剧ず褰撳墠宸查厤缃殑璺熻釜鍣ㄣ€傛洿鏀瑰綋鍓嶈窡韪櫒浼氬悓鏃?
	娓呴櫎鐜舰缂撳啿鍖虹殑鍐呭浠ュ強 "snapshot"锛堝揩鐓э級缂撳啿鍖恒€?

  available_tracers:

	璇ユ枃浠朵繚瀛樺凡缂栬瘧杩涘唴鏍哥殑鍚勭涓嶅悓绫诲瀷鐨勮窡韪櫒銆傝繖閲屽垪鍑虹殑
	璺熻釜鍣ㄥ彲浠ラ€氳繃灏嗗畠浠殑鍚嶅瓧 echo 鍒?current_tracer 鏉ラ厤缃€?

  tracing_on:

	璇ユ枃浠惰缃垨鏄剧ず鏄惁鍚敤浜嗗悜璺熻釜鐜舰缂撳啿鍖虹殑鍐欏叆銆傚悜璇ユ枃浠?
	echo 0 鍙鐢ㄨ窡韪櫒锛宔cho 1 鍙惎鐢ㄥ畠銆傛敞鎰忥紝杩欏彧浼氱鐢ㄥ悜鐜舰
	缂撳啿鍖虹殑鍐欏叆锛岃窡韪紑閿€浠嶆湁鍙兘鍦ㄧ户缁彂鐢熴€?

	鍐呮牳鍑芥暟 tracing_off() 鍙敤浜庡湪鍐呮牳鍐呴儴绂佺敤鍚戠幆褰㈢紦鍐插尯鐨勫啓鍏ワ紝
	杩欎細鎶婅鏂囦欢璁剧疆涓?"0"銆傜敤鎴风┖闂村彲浠ラ€氳繃鍚戣鏂囦欢 echo "1" 鏉?
	閲嶆柊鍚敤璺熻釜銆?

	娉ㄦ剰锛屽嚱鏁板拰浜嬩欢鐨?"traceoff" 瑙﹀彂鍣ㄤ篃浼氬皢璇ユ枃浠舵竻闆跺苟鍋滄璺熻釜銆?
	瀹冨悓鏍峰彲浠ョ敱鐢ㄦ埛绌洪棿浣跨敤璇ユ枃浠舵潵閲嶆柊鍚敤銆?

  trace:

	璇ユ枃浠朵互浜虹被鍙鐨勬牸寮忥紙濡備笅鎵€杩帮級淇濆瓨璺熻釜鐨勮緭鍑恒€備互 O_TRUNC
	鏍囧織鎵撳紑璇ユ枃浠惰繘琛屽啓鍏ヤ細娓呴櫎鐜舰缂撳啿鍖虹殑鍐呭銆傛敞鎰忥紝璇ユ枃浠?
	涓嶆槸涓€涓秷璐瑰瀷鏂囦欢銆傚鏋滆窡韪凡鍏抽棴锛堟病鏈夎窡韪櫒鍦ㄨ繍琛岋紝鎴?
	tracing_on 涓洪浂锛夛紝姣忔璇诲彇瀹冮兘浼氫骇鐢熺浉鍚岀殑杈撳嚭銆傚綋璺熻釜寮€鍚椂锛?
	鐢变簬瀹冭瘯鍥惧湪涓嶆秷璐圭殑鎯呭喌涓嬭鍙栨暣涓紦鍐插尯锛屽彲鑳戒細浜х敓涓嶄竴鑷寸殑
	缁撴灉銆?

  trace_pipe:

	鍏惰緭鍑轰笌 "trace" 鏂囦欢鐩稿悓锛屼絾璇ユ枃浠剁敤浜庨厤鍚堝疄鏃惰窡韪繘琛屾祦寮忚鍙栥€?
	浠庤鏂囦欢璇诲彇浼氶樆濉烇紝鐩村埌鑾峰彇鍒版柊鏁版嵁銆備笌 "trace" 鏂囦欢涓嶅悓锛岃鏂囦欢
	鏄竴涓秷璐瑰瀷鏂囦欢銆傝繖鎰忓懗鐫€浠庤鏂囦欢璇诲彇浼氬鑷撮『搴忚鍙栨樉绀烘洿鏂扮殑
	鏁版嵁銆備竴鏃︽暟鎹粠璇ユ枃浠惰鍑猴紝瀹冨氨琚秷璐规帀浜嗭紝椤哄簭璇诲彇灏嗕笉浼氬啀璇诲埌
	瀹冦€?trace" 鏂囦欢鏄潤鎬佺殑锛屽鏋滆窡韪櫒娌℃湁娣诲姞鏇村鏁版嵁锛屾瘡娆¤鍙栧畠
	閮戒細鏄剧ず鐩稿悓鐨勪俊鎭€?

  trace_options:

	璇ユ枃浠惰鐢ㄦ埛鍙互鎺у埗涓婅堪鏌愪釜杈撳嚭鏂囦欢涓墍鏄剧ず鐨勬暟鎹噺銆備篃鏈変竴浜?
	閫夐」鐢ㄤ簬淇敼璺熻釜鍣ㄦ垨浜嬩欢鐨勫伐浣滄柟寮忥紙鏍堝洖婧€佹椂闂存埑绛夛級銆?

  options:

	杩欐槸涓€涓洰褰曪紝鍏朵腑鍖呭惈姣忎釜鍙敤璺熻釜閫夐」锛堝悓鏍峰瓨鍦ㄤ簬 trace_options
	涓級瀵瑰簲鐨勪竴涓枃浠躲€備篃鍙互鍚戝搴旈€夐」鍚嶇殑鏂囦欢鍐欏叆 "1" 鎴?"0" 鏉?
	璁剧疆鎴栨竻闄よ閫夐」銆?

  tracing_max_latency:

	閮ㄥ垎璺熻釜鍣ㄤ細璁板綍鏈€澶у欢杩熴€備緥濡傦紝涓柇琚鐢ㄧ殑鏈€闀挎椂闂淬€傛渶澶ф椂闂?
	淇濆瓨鍦ㄨ鏂囦欢涓€傛渶澶ц窡韪褰曚篃浼氳淇濆瓨锛屽苟鐢?"trace" 鏄剧ず銆傚彧鏈夊綋
	寤惰繜澶т簬璇ユ枃浠朵腑鐨勫€兼椂锛屾墠浼氳褰曚竴鏉℃柊鐨勬渶澶ц窡韪紙浠ュ井绉掍负鍗曚綅锛夈€?

	鍚戣鏂囦欢 echo 涓€涓椂闂村€煎悗锛岄櫎闈炲欢杩熷ぇ浜庤鏂囦欢涓殑鏃堕棿锛屽惁鍒欎笉浼?
	璁板綍浠讳綍寤惰繜銆?

  tracing_thresh:

	褰撳欢杩熷ぇ浜庤鏂囦欢涓殑鏁板€兼椂锛岄儴鍒嗗欢杩熻窡韪櫒浼氳褰曚竴鏉¤窡韪€備粎褰?
	璇ユ枃浠朵腑鐨勬暟鍊煎ぇ浜?0 鏃舵墠鐢熸晥銆傦紙浠ュ井绉掍负鍗曚綅锛?

  buffer_percent:

	杩欐槸鐜舰缂撳啿鍖哄湪琚敜閱掍箣鍓嶉渶瑕佸～鍏呭灏戠殑姘翠綅绾裤€備篃灏辨槸璇达紝濡傛灉涓€涓?
	搴旂敤绋嬪簭鍦ㄤ竴涓?per_cpu 鐨?trace_pipe_raw 鏂囦欢涓婅皟鐢ㄩ樆濉炶鍙栫郴缁熻皟鐢紝
	瀹冧細涓€鐩撮樆濉烇紝鐩村埌 buffer_percent 鎸囧畾鐨勭粰瀹氭暟閲忕殑鏁版嵁杩涘叆鐜舰缂撳啿鍖猴紝
	鎵嶄細鍞ら啋璇诲彇鑰呫€傝繖涔?

```
	  0   - 琛ㄧず涓€鏃︾幆褰㈢紦鍐插尯涓湁浠讳綍鏁版嵁灏卞敜閱掋€?
	  50  - 琛ㄧず褰撳ぇ绾︿竴鍗婄殑鐜舰缂撳啿鍖哄瓙缂撳啿鍖哄～婊℃椂鍞ら啋銆?
	  100 - 琛ㄧず涓€鐩撮樆濉炵洿鍒扮幆褰㈢紦鍐插尯瀹屽叏濉弧锛屽嵆灏嗗紑濮嬭鐩栨棫鏁版嵁銆?
```

  buffer_size_kb:

	璇ユ枃浠惰缃垨鏄剧ず姣忎釜 CPU 缂撳啿鍖烘墍鎸佹湁鐨勫崈瀛楄妭鏁般€傞粯璁ゆ儏鍐典笅锛屾瘡涓?
	CPU 鐨勮窡韪紦鍐插尯澶у皬鐩稿悓銆傛樉绀虹殑鏁板€兼槸 CPU 缂撳啿鍖虹殑澶у皬锛岃€屼笉鏄墍鏈?
	缂撳啿鍖虹殑鎬诲ぇ灏忋€傝窡韪紦鍐插尯浠ラ〉锛堝唴鏍哥敤浜庡垎閰嶇殑鍐呭瓨鍧楋紝閫氬父涓?4 KB锛?
	涓哄崟浣嶅垎閰嶃€傚彲鑳戒細棰濆鍒嗛厤灏戣鍑犻〉浠ュ绾崇紦鍐插尯鐨勭鐞嗗厓鏁版嵁銆傚鏋滄渶鍚?
	鍒嗛厤鐨勯〉涓繕鏈夊浜庤姹傚瓧鑺傜殑绌洪棿锛岃椤电殑鍓╀綑閮ㄥ垎涔熶細琚娇鐢紝浣垮緱
	瀹為檯鍒嗛厤閲忓ぇ浜庢墍璇锋眰鎴栨樉绀虹殑銆?
	锛堟敞鎰忥紝鐢变簬缂撳啿鍖虹鐞嗗厓鏁版嵁鐨勫師鍥狅紝璇ュぇ灏忓彲鑳戒笉鏄〉澶у皬鐨勬暣鏁板€嶃€傦級

	鍗曚釜 CPU 鐨勭紦鍐插尯澶у皬鍙兘浼氫笉鍚岋紙瑙佷笅闈㈢殑 "per_cpu/cpu0/buffer_size_kb"锛夛紝
	濡傛灉涓嶅悓锛岃鏂囦欢灏嗘樉绀?"X"銆?

  buffer_total_size_kb:

	璇ユ枃浠舵樉绀烘墍鏈夎窡韪紦鍐插尯鍚堝苟鍚庣殑鎬诲ぇ灏忋€?

  buffer_subbuf_size_kb:

	璇ユ枃浠惰缃垨鏄剧ず瀛愮紦鍐插尯鐨勫ぇ灏忋€傜幆褰㈢紦鍐插尯琚垝鍒嗕负鑻ュ共涓浉鍚屽ぇ灏忕殑
	"瀛愮紦鍐插尯"銆備竴涓簨浠朵笉鑳藉ぇ浜庡瓙缂撳啿鍖虹殑澶у皬銆傞€氬父锛屽瓙缂撳啿鍖虹殑澶у皬
	绛変簬鏋舵瀯鐨勯〉澶у皬锛坸86 涓婁负 4K锛夈€傚瓙缂撳啿鍖哄紑澶磋繕鍖呭惈鍏冩暟鎹紝杩欏悓鏍?
	闄愬埗浜嗕簨浠剁殑澶у皬銆傝繖鎰忓懗鐫€褰撳瓙缂撳啿鍖轰负涓€涓〉澶у皬鏃讹紝娌℃湁浠讳綍浜嬩欢鑳?
	澶т簬椤靛ぇ灏忓噺鍘诲瓙缂撳啿鍖哄厓鏁版嵁銆?

	娉ㄦ剰锛宐uffer_subbuf_size_kb 鏄敤鎴锋寚瀹氬瓙缂撳啿鍖烘渶灏忓ぇ灏忕殑涓€绉嶆柟寮忋€?
	鐢变簬瀹炵幇缁嗚妭锛屽唴鏍稿彲鑳戒細鎶婂畠鍙樺ぇ锛屾垨鑰呭鏋滃唴鏍告棤娉曞鐞嗚璇锋眰锛屽垯鐩存帴
	浣挎搷浣滃け璐ャ€?

	鏇存敼瀛愮紦鍐插尯澶у皬鍙互璁╀簨浠跺ぇ浜庨〉澶у皬銆?

	娉ㄦ剰锛氭洿鏀瑰瓙缂撳啿鍖哄ぇ灏忔椂锛岃窡韪細鍋滄锛岀幆褰㈢紦鍐插尯鍜屽揩鐓х紦鍐插尯涓殑浠讳綍
	鏁版嵁閮戒細琚涪寮冦€?

  free_buffer:

	濡傛灉涓€涓繘绋嬫鍦ㄦ墽琛岃窡韪紝骞朵笖璇ヨ繘绋嬬殑鐜舰缂撳啿鍖哄簲鍦ㄥ叾缁撴潫鏃讹紙鍗充娇
	瀹冭淇″彿鏉€姝伙級琚敹缂?"閲婃斁"锛屽垯鍙互浣跨敤璇ユ枃浠舵潵瀹炵幇姝ょ洰鐨勩€傚湪璇ユ枃浠?
	鍏抽棴鏃讹紝鐜舰缂撳啿鍖轰細琚噸缃负鏈€灏忓ぇ灏忋€傝姝ｅ湪璺熻釜鐨勮繘绋嬪悓鏃舵墦寮€璇ユ枃浠讹紝
	褰撹杩涚▼閫€鍑烘椂锛屽叾瀵瑰簲姝ゆ枃浠剁殑鏂囦欢鎻忚堪绗︿細琚叧闂紝鍦ㄦ杩囩▼涓幆褰㈢紦鍐插尯
	浼氳"閲婃斁"銆?

	濡傛灉璁剧疆浜?disable_on_free 閫夐」锛屽畠涔熷彲鑳藉仠姝㈣窡韪€?

  tracing_cpumask:

	杩欐槸涓€涓帺鐮侊紝璁╃敤鎴峰彧鑳藉湪鐗瑰畾 CPU 涓婅繘琛岃窡韪€傛牸寮忎负琛ㄧず CPU 鐨?
	鍗佸叚杩涘埗瀛楃涓层€?

  set_ftrace_filter:

	褰撻厤缃簡鍔ㄦ€?ftrace 鏃讹紙瑙佷笅闈㈢殑 "dynamic ftrace" 涓€鑺傦級锛屼唬鐮佷細琚?
	鍔ㄦ€佷慨鏀癸紙浠ｇ爜鏂囨湰閲嶅啓锛変互绂佺敤瀵瑰嚱鏁版€ц兘鍒嗘瀽鍣紙mcount锛夌殑璋冪敤銆傝繖浣垮緱
	閰嶇疆璺熻釜甯︽潵鐨勬€ц兘寮€閿€鍑犱箮鍙互蹇界暐涓嶈銆傝繖杩樻湁涓€涓壇浣滅敤锛屽嵆鑳藉鍚敤
	鎴栫鐢ㄥ鐗瑰畾鍑芥暟鐨勮窡韪€傚悜璇ユ枃浠?echo 鍑芥暟鍚嶏紝灏嗘妸璺熻釜闄愬埗涓轰粎杩欎簺鍑芥暟銆?
	杩欎細褰卞搷 "function" 鍜?"function_graph" 璺熻釜鍣紝鍥犳涔熷奖鍝嶅嚱鏁版€ц兘鍒嗘瀽
	锛堣 "function_profile_enabled"锛夈€?

	鍙互鍐欏叆璇ユ枃浠剁殑鍑芥暟鍒楀湪 "available_filter_functions" 涓€?

	璇ユ帴鍙ｄ篃鍏佽浣跨敤鍛戒护銆傛洿澶氱粏鑺傝鍙傞槄 "Filter commands"锛堣繃婊ゅ懡浠わ級涓€鑺傘€?

	浣滀负涓€绉嶅姞閫熸墜娈碉紝鐢变簬澶勭悊瀛楃涓插彲鑳界浉褰撴槀璐碉紝骞朵笖闇€瑕佹鏌ユ墍鏈夋敞鍐屽埌
	璺熻釜涓殑鍑芥暟锛屽洜姝ゅ彲浠ユ敼涓哄悜璇ユ枃浠跺啓鍏ヤ竴涓储寮曘€傚啓鍏ヤ竴涓暟瀛楋紙浠?"1"
	寮€澶达級灏嗘敼涓洪€夋嫨 "available_filter_functions" 鏂囦欢涓搴旇浣嶇疆鐨勭浉鍚?
	鍑芥暟銆?

  set_ftrace_notrace:

	璇ユ枃浠剁殑浣滅敤涓?set_ftrace_filter 鐩稿弽銆傛坊鍔犲埌姝ゅ鐨勪换浣曞嚱鏁伴兘涓嶄細琚?
	璺熻釜銆傚鏋滀竴涓嚱鏁板悓鏃跺瓨鍦ㄤ簬 set_ftrace_filter 鍜?set_ftrace_notrace
	涓紝鍒欒鍑芥暟鍦╛涓峗浼氳璺熻釜銆?

  set_ftrace_pid:

	璁╁嚱鏁拌窡韪櫒鍙窡韪?PID 鍒楀湪璇ユ枃浠朵腑鐨勭嚎绋嬨€?

	濡傛灉璁剧疆浜?"function-fork" 閫夐」锛岄偅涔堝綋 PID 鍒楀湪璇ユ枃浠朵腑鐨勪换鍔?fork 鏃讹紝
	瀛愯繘绋嬬殑 PID 浼氳嚜鍔ㄦ坊鍔犲埌璇ユ枃浠朵腑锛屽瓙杩涚▼涔熷皢琚嚱鏁拌窡韪櫒璺熻釜銆傝閫夐」
	杩樹細瀵艰嚧閫€鍑虹殑浠诲姟鐨?PID 浠庤鏂囦欢涓绉婚櫎銆?

  set_ftrace_notrace_pid:

        璁╁嚱鏁拌窡韪櫒蹇界暐 PID 鍒楀湪璇ユ枃浠朵腑鐨勭嚎绋嬨€?

        濡傛灉璁剧疆浜?"function-fork" 閫夐」锛岄偅涔堝綋 PID 鍒楀湪璇ユ枃浠朵腑鐨勪换鍔?fork 鏃讹紝
	瀛愯繘绋嬬殑 PID 浼氳嚜鍔ㄦ坊鍔犲埌璇ユ枃浠朵腑锛屽瓙杩涚▼涔熶笉浼氳鍑芥暟璺熻釜鍣ㄨ窡韪€傝閫夐」
	鍚屾牱浼氬鑷撮€€鍑虹殑浠诲姟鐨?PID 浠庤鏂囦欢涓绉婚櫎銆?

        濡傛灉涓€涓?PID 鍚屾椂瀛樺湪浜庤鏂囦欢鍜?"set_ftrace_pid" 涓紝鍒欒鏂囦欢浼樺厛锛?
	璇ョ嚎绋嬩笉浼氳璺熻釜銆?

  set_event_pid:

	璁╀簨浠跺彧璺熻釜 PID 鍒楀湪璇ユ枃浠朵腑鐨勪换鍔°€傛敞鎰忥紝sched_switch 鍜?sched_wake_up
	涔熶細璺熻釜鍒楀湪璇ユ枃浠朵腑鐨勪簨浠躲€?

	瑕佽鍒楀湪璇ユ枃浠朵腑鐨勪换鍔＄殑瀛愯繘绋?PID 鍦?fork 鏃惰娣诲姞杩涙潵锛岃鍚敤
	"event-fork" 閫夐」銆傝閫夐」杩樹細瀵艰嚧浠诲姟鐨?PID 鍦ㄤ换鍔￠€€鍑烘椂浠庤鏂囦欢涓绉婚櫎銆?

  set_event_notrace_pid:

	璁╀簨浠朵笉璺熻釜 PID 鍒楀湪璇ユ枃浠朵腑鐨勪换鍔°€傛敞鎰忥紝sched_switch 鍜?sched_wakeup
	浼氳窡韪湭鍒楀湪璇ユ枃浠朵腑鐨勭嚎绋嬶紝鍗充娇鏌愪釜绾跨▼鐨?PID 鍦ㄨ鏂囦欢涓紝濡傛灉
	sched_switch 鎴?sched_wakeup 浜嬩欢鍚屾椂涔熻窡韪煇涓簲褰撹璺熻釜鐨勭嚎绋嬨€?

	瑕佽鍒楀湪璇ユ枃浠朵腑鐨勪换鍔＄殑瀛愯繘绋?PID 鍦?fork 鏃惰娣诲姞杩涙潵锛岃鍚敤
	"event-fork" 閫夐」銆傝閫夐」杩樹細瀵艰嚧浠诲姟鐨?PID 鍦ㄤ换鍔￠€€鍑烘椂浠庤鏂囦欢涓绉婚櫎銆?

  set_graph_function:

	鍒楀湪璇ユ枃浠朵腑鐨勫嚱鏁颁細浣垮嚱鏁板浘璺熻釜鍣ㄥ彧璺熻釜杩欎簺鍑芥暟浠ュ強瀹冧滑鎵€璋冪敤鐨勫嚱鏁般€?
	锛堟洿澶氱粏鑺傝 "dynamic ftrace" 涓€鑺傘€傦級娉ㄦ剰锛宻et_ftrace_filter 鍜?
	set_ftrace_notrace 浠嶇劧浼氬奖鍝嶅摢浜涘嚱鏁拌璺熻釜銆?

  set_graph_notrace:

	绫讳技浜?set_graph_function锛屼絾鍦ㄥ懡涓鍑芥暟鏃剁鐢ㄥ嚱鏁板浘璺熻釜锛岀洿鍒板畠閫€鍑?
	璇ュ嚱鏁颁负姝€傝繖鏍峰彲浠ュ拷鐣ュ鏌愪釜鐗瑰畾鍑芥暟鎵€璋冪敤鍑芥暟鐨勮窡韪€?

  available_filter_functions:

	璇ユ枃浠跺垪鍑?ftrace 宸插鐞嗗苟涓斿彲浠ヨ窡韪殑鍑芥暟銆傝繖浜涘氨鏄綘鍙互浼犻€掔粰
	"set_ftrace_filter"銆?set_ftrace_notrace"銆?set_graph_function" 鎴?
	"set_graph_notrace" 鐨勫嚱鏁板悕銆?
	锛堟洿澶氱粏鑺傝涓嬮潰鐨?"dynamic ftrace" 涓€鑺傘€傦級

  available_filter_functions_addrs:

	绫讳技浜?available_filter_functions锛屼絾涓烘瘡涓嚱鏁版樉绀哄湴鍧€銆傛樉绀虹殑鍦板潃鏄?
	琛ヤ竵绔欑偣鍦板潃锛屽彲鑳戒笌 /proc/kallsyms 涓殑鍦板潃涓嶅悓銆?

  syscall_user_buf_size:

	閮ㄥ垎绯荤粺璋冪敤璺熻釜浜嬩欢浼氳褰曟煇涓弬鏁版墍鎸囧悜鐨勭敤鎴风┖闂村湴鍧€涓殑鏁版嵁銆傛瘡涓?
	浜嬩欢鐨勬暟鎹噺鏄彈闄愮殑銆傝鏂囦欢淇濆瓨灏嗚璁板綍杩涚幆褰㈢紦鍐插尯浠ヤ繚瀛樿繖浜涙暟鎹殑鏈€澶?
	瀛楄妭鏁般€傚綋鍓嶆渶澶у€间负 165銆?

  dyn_ftrace_total_info:

	璇ユ枃浠剁敤浜庤皟璇曠洰鐨勩€傛樉绀哄凡琚浆鎹㈡垚 nop 骞朵笖鍙敤浜庤窡韪殑鍑芥暟鏁伴噺銆?

  enabled_functions:

	璇ユ枃浠舵洿澶氱敤浜庤皟璇?ftrace锛屼絾鍦ㄦ煡鐪嬫槸鍚︽湁浠讳綍鍑芥暟鎸傛帴浜嗗洖璋冩椂涔熷緢鏈夌敤銆?
	涓嶄粎璺熻釜鍩虹璁炬柦浼氱敤鍒?ftrace 鐨勫嚱鏁拌窡韪姛鑳斤紝鍏朵粬瀛愮郴缁熶篃鍙兘鐢ㄥ埌銆傝鏂囦欢
	鏄剧ず鎵€鏈夋寕鎺ヤ簡鍥炶皟鐨勫嚱鏁帮紝浠ュ強宸叉寕鎺ョ殑鍥炶皟鏁伴噺銆傛敞鎰忥紝涓€涓洖璋冧篃鍙兘璋冪敤
	澶氫釜鍑芥暟锛岃繖浜涗笉浼氳璁″叆姝よ鏁般€?

	濡傛灉娉ㄥ唽鐨勫洖璋冩槸浠?"save regs" 灞炴€э紙鍥犳寮€閿€鏇村ぇ锛夎璺熻釜鐨勫嚱鏁帮紝鍒欏湪涓?
	杩斿洖瀵勫瓨鍣ㄧ殑鍑芥暟鍚屼竴琛屼細鏄剧ず涓€涓?'R'銆?

	濡傛灉娉ㄥ唽鐨勫洖璋冩槸浠?"ip modify" 灞炴€э紙鍥犳 regs->ip 鍙互琚慨鏀癸級琚窡韪殑
	鍑芥暟锛屽垯鍦ㄥ悓涓€琛屼細鏄剧ず涓€涓?'I'銆?

	濡傛灉鎸傛帴浜嗕竴涓潪 ftrace 鐨勮功搴婏紙BPF锛夛紝鍒欎細鏄剧ず涓€涓?'D'銆傛敞鎰忥紝鏅€氱殑
	ftrace 韫﹀簥涔熷彲浠ユ寕鎺ワ紝浣嗕竴涓粰瀹氱殑鍑芥暟涓€娆″彧鑳芥寕鎺ヤ竴涓?鐩存帴"韫﹀簥銆?

	鏌愪簺鏋舵瀯鏃犳硶璋冪敤鐩存帴韫﹀簥锛岃€屾槸灏?ftrace ops 鍑芥暟鏀剧疆鍦ㄥ嚱鏁板叆鍙ｇ偣涔嬩笂銆?
	鍦ㄨ繖绉嶆儏鍐典笅浼氭樉绀轰竴涓?'O'銆?

	濡傛灉涓€涓嚱鏁拌繃鍘绘浘鎸傛帴浜?"ip modify" 鎴栫洿鎺ヨ皟鐢紝鍒欎細鏄剧ず涓€涓?'M'銆傝鏍囧織
	姘歌繙涓嶄細琚竻闄ゃ€傚畠鐢ㄤ簬浜嗚В鏌愪釜鍑芥暟鏄惁鏇捐 ftrace 鍩虹璁炬柦淇敼杩囷紝鍙敤浜?
	璋冭瘯銆?

	濡傛灉鏋舵瀯鏀寔锛屽畠杩樹細鏄剧ず璇ュ嚱鏁版鍦ㄧ洿鎺ヨ皟鐢ㄧ殑鍥炶皟銆傚鏋滆鏁板ぇ浜?1锛屽垯寰堝彲鑳?
	鏄?ftrace_ops_list_func()銆?

	濡傛灉涓€涓嚱鏁扮殑鍥炶皟璺宠浆鍒颁竴涓壒瀹氫簬璇ュ洖璋冭€岄潪鏍囧噯韫﹀簥鐨勮功搴婏紝鍒欎細鎵撳嵃鍏跺湴鍧€
	浠ュ強璇ヨ功搴婃墍璋冪敤鐨勫嚱鏁般€?

  touched_functions:

	璇ユ枃浠跺寘鍚浘閫氳繃 ftrace 鍩虹璁炬柦鎸傛帴杩囧嚱鏁板洖璋冪殑鎵€鏈夊嚱鏁般€傚畠鐨勬牸寮忎笌
	enabled_functions 鐩稿悓锛屼絾鏄剧ず鐨勬槸鎵€鏈夋浘缁忚璺熻釜杩囩殑鍑芥暟銆?

	瑕佹煡鐪嬩换浣曟浘琚?"ip modify" 鎴栫洿鎺ヨ功搴婁慨鏀硅繃鐨勫嚱鏁帮紝鍙互鎵ц浠ヤ笅鍛戒护锛?

	grep ' M ' /sys/kernel/tracing/touched_functions

  function_profile_enabled:

	璁剧疆璇ユ枃浠舵椂锛屽畠浼氬惎鐢ㄦ墍鏈夊嚱鏁扮殑 function 璺熻釜鍣紝濡傛灉宸查厤缃紝鍒欏惎鐢ㄥ嚱鏁板浘
	璺熻釜鍣ㄣ€傚畠浼氫繚瀛樿璋冪敤鍑芥暟鏁伴噺鐨勭洿鏂瑰浘锛屽鏋滈厤缃簡鍑芥暟鍥捐窡韪櫒锛屽畠杩樹細璁板綍
	杩欎簺鍑芥暟鎵€鑺辫垂鐨勬椂闂淬€傜洿鏂瑰浘鍐呭鍙互鏄剧ず鍦ㄤ互涓嬫枃浠朵腑锛?

	trace_stat/function<cpu>锛坒unction0銆乫unction1 绛夛級銆?

  trace_stat:

	涓€涓繚瀛樹笉鍚岃窡韪粺璁′俊鎭殑鐩綍銆?

  kprobe_events:

	鍚敤鍔ㄦ€佽窡韪偣銆傝鍙傞槄 kprobetrace.rst銆?

  kprobe_profile:

	鍔ㄦ€佽窡韪偣缁熻淇℃伅銆傝鍙傞槄 kprobetrace.rst銆?

  max_graph_depth:

	涓庡嚱鏁板浘璺熻釜鍣ㄩ厤鍚堜娇鐢ㄣ€傝繖鏄畠灏嗚窡韪繘鍏ュ嚱鏁扮殑娣卞害銆傚皢鍏惰缃负 1 灏嗗彧鏄剧ず
	浠庣敤鎴风┖闂磋皟鐢ㄧ殑绗竴涓唴鏍稿嚱鏁般€?

  printk_formats:

	璇ユ枃浠朵緵璇诲彇鍘熷鏍煎紡鏂囦欢鐨勫伐鍏蜂娇鐢ㄣ€傚鏋滅幆褰㈢紦鍐插尯涓殑涓€涓簨浠跺紩鐢ㄤ簡涓€涓?
	瀛楃涓诧紝鍒欏彧鎶婃寚鍚戣瀛楃涓茬殑鎸囬拡璁板綍杩涚紦鍐插尯锛岃€屼笉鏄瓧绗︿覆鏈韩銆傝繖瀵艰嚧宸ュ叿
	鏃犳硶鐭ラ亾閭ｄ釜瀛楃涓叉槸浠€涔堛€傝鏂囦欢鏄剧ず瀛楃涓插強鍏跺湴鍧€锛屼娇宸ュ叿鑳藉灏嗘寚閽堟槧灏?
	鍒板搴旂殑瀛楃涓层€?

  saved_cmdlines:

	闄ら潪浜嬩欢鐗瑰埆淇濆瓨浜嗕换鍔＄殑 comm锛屽惁鍒欏湪璺熻釜浜嬩欢涓彧璁板綍浠诲姟鐨?pid銆俧trace
	寤虹珛涓€涓?pid 鍒?comm 鐨勬槧灏勭紦瀛橈紝浠ュ皾璇曚负浜嬩欢鏄剧ず comm銆傚鏋滄煇涓?comm 鐨?pid
	鏈垪鍑猴紝鍒欒緭鍑轰腑浼氭樉绀?"<...>"銆?

	濡傛灉 "record-cmd" 閫夐」琚缃负 "0"锛屽垯鍦ㄨ褰曟湡闂翠笉浼氫繚瀛樹换鍔＄殑 comm銆傞粯璁?
	鎯呭喌涓嬪畠鏄惎鐢ㄧ殑銆?

  saved_cmdlines_size:

	榛樿鎯呭喌涓嬩繚瀛?128 涓?comm锛堣涓婇潰鐨?"saved_cmdlines"锛夈€傝澧炲姞鎴栧噺灏戣缂撳瓨
	鐨?comm 鏁伴噺锛屽悜璇ユ枃浠?echo 瑕佺紦瀛樼殑 comm 鏁伴噺銆?

  saved_tgids:

	濡傛灉璁剧疆浜?"record-tgid" 閫夐」锛屽垯姣忔璋冨害涓婁笅鏂囧垏鎹㈡椂锛屼换鍔＄殑绾跨▼缁?ID 浼氳
	淇濆瓨鍒颁竴涓槧灏?PID 鍒板叾 TGID 鐨勮〃涓€傞粯璁ゆ儏鍐典笅锛?record-tgid" 閫夐」鏄鐢ㄧ殑銆?

  snapshot:

	璇ユ枃浠舵樉绀?蹇収"缂撳啿鍖猴紝骞跺厑璁哥敤鎴峰褰撳墠姝ｅ湪杩愯鐨勮窡韪媿鎽勫揩鐓с€傛洿澶氱粏鑺?
	璇峰弬闃呬笅闈㈢殑 "Snapshot"锛堝揩鐓э級涓€鑺傘€?

  stack_max_size:

	褰撳惎鐢ㄦ爤璺熻釜鍣ㄦ椂锛岃鏂囦欢浼氭樉绀哄畠鎵€閬囧埌鐨勬渶澶ф爤澶у皬銆傝鍙傞槄涓嬮潰鐨?"Stack Trace"
	锛堟爤璺熻釜锛変竴鑺傘€?

  stack_trace:

	璇ユ枃浠舵樉绀哄惎鐢ㄦ爤璺熻釜鍣ㄦ椂鎵€閬囧埌鐨勬渶澶ф爤鐨勬爤鍥炴函銆傝鍙傞槄涓嬮潰鐨?"Stack Trace"
	锛堟爤璺熻釜锛変竴鑺傘€?

  stack_trace_filter:

	璇ユ枃浠剁被浼间簬 "set_ftrace_filter"锛屼絾瀹冮檺鍒舵爤璺熻釜鍣ㄦ墍妫€鏌ョ殑鍑芥暟銆?

  trace_clock:

	姣忓綋涓€涓簨浠惰璁板綍杩涚幆褰㈢紦鍐插尯鏃讹紝閮戒細娣诲姞涓€涓?鏃堕棿鎴?銆傝鏃堕棿鎴虫潵鑷煇涓?
	鎸囧畾鐨勬椂閽熴€傞粯璁ゆ儏鍐典笅锛宖trace 浣跨敤 "local" 鏃堕挓銆傝鏃堕挓闈炲父蹇苟涓斾弗鏍兼寜
	姣忎釜 CPU 鐙珛锛屼絾鍦ㄦ煇浜涚郴缁熶笂瀹冪浉瀵逛簬鍏朵粬 CPU 鍙兘涓嶆槸鍗曡皟鐨勩€傛崲鍙ヨ瘽璇达紝
	鏈湴鏃堕挓鍙兘涓庡叾浠?CPU 涓婄殑鏈湴鏃堕挓涓嶅悓姝ャ€?

	璺熻釜甯哥敤鐨勬椂閽燂細

```
	  # cat trace_clock
	  [local] global counter x86-tsc
```

	甯︽湁鏂规嫭鍙风殑鏃堕挓鏄鍦ㄧ敓鏁堢殑鏃堕挓銆?

	local:
		榛樿鏃堕挓锛屼絾鍙兘鍦ㄤ笉鍚?CPU 涔嬮棿涓嶅悓姝?

	global:
		璇ユ椂閽熶笌鎵€鏈?CPU 鍚屾锛屼絾鍙兘姣旀湰鍦版椂閽熺◢鎱€?

	counter:
		杩欐牴鏈笉鏄椂閽燂紝鑰屾槸涓€涓瓧闈笂鐨勫師瀛愯鏁板櫒銆傚畠閫愪釜閫掑锛屼絾涓庢墍鏈?
		CPU 鍚屾銆傚綋浣犻渶瑕佺‘鍒囩煡閬撲笉鍚?CPU 涓婁簨浠剁浉浜掍箣闂寸殑鍙戠敓椤哄簭鏃讹紝杩?
		寰堟湁鐢ㄣ€?

	uptime:
		璇ユ椂閽熶娇鐢?jiffies 璁℃暟鍣紝鏃堕棿鎴崇浉瀵逛簬绯荤粺鍚姩鍚庣殑鏃堕棿銆?

	perf:
		璇ユ椂閽熶娇 ftrace 浣跨敤涓?perf 鐩稿悓鐨勬椂閽熴€傛渶缁?perf 灏嗚兘澶熻鍙?ftrace
		缂撳啿鍖猴紝杩欏皢鏈夊姪浜庝氦缁囧悎骞舵暟鎹€?

	x86-tsc:
		鏋舵瀯鍙互瀹氫箟鑷繁鐨勬椂閽熴€備緥濡傦紝x86 鍦ㄨ繖閲屼娇鐢ㄨ嚜宸辩殑 TSC 鍛ㄦ湡鏃堕挓銆?

	ppc-tb:
		璇ユ椂閽熶娇鐢?powerpc 鐨?timebase 瀵勫瓨鍣ㄥ€笺€傚畠涓庢墍鏈?CPU 鍚屾锛屽苟涓斿鏋?
		宸茬煡 tb_offset锛岃繕鍙敤浜庡叧鑱旂鐞嗙▼搴?瀹㈡埛鏈轰箣闂寸殑浜嬩欢銆?

	mono:
		璇ユ椂閽熶娇鐢ㄥ揩閫熷崟璋冩椂閽燂紙CLOCK_MONOTONIC锛夛紝瀹冩槸鍗曡皟鐨勶紝骞跺彈 NTP 閫熺巼
		璋冩暣褰卞搷銆?

	mono_raw:
		璇ユ椂閽熸槸鍘熷鍗曡皟鏃堕挓锛圕LOCK_MONOTONIC_RAW锛夛紝瀹冩槸鍗曡皟鐨勶紝浣嗕笉鍙椾换浣?
		閫熺巼璋冩暣褰卞搷锛屽苟浠ヤ笌纭欢鏃堕挓婧愮浉鍚岀殑閫熺巼璧版椂銆?

	boot:
		璇ユ椂閽熸槸鍚姩鏃堕挓锛圕LOCK_BOOTTIME锛夛紝鍩轰簬蹇€熷崟璋冩椂閽燂紝浣嗕篃浼氳鍏ュ湪
		鎸傝捣鐘舵€佷腑鎵€鑺辫垂鐨勬椂闂淬€傜敱浜庢椂閽熻闂槸涓哄湪鎸傝捣璺緞涓殑璺熻釜浣跨敤鑰岃璁★紝
		濡傛灉鍦ㄥ揩閫?mono 鏃堕挓鏇存柊涔嬪墠銆佸湪鎸傝捣鏃堕棿琚鍏ヤ箣鍚庤闂鏃堕挓锛屽彲鑳戒細
		浜х敓涓€浜涘壇浣滅敤銆傚湪杩欑鎯呭喌涓嬶紝鏃堕挓鏇存柊鐪嬭捣鏉ヤ細姣旀甯告儏鍐典笅绋嶆棭鍙戠敓銆?
		姝ゅ鍦?32 浣嶇郴缁熶笂锛?4 浣嶅惎鍔ㄥ亸绉婚噺鍙兘浼氱湅鍒伴儴鍒嗘洿鏂般€傝繖浜涙晥搴斿緢缃曡锛?
		鍚庡鐞嗗簲褰撹兘澶熷鐞嗗畠浠€傛洿澶氫俊鎭鍙傞槄 ktime_get_boot_fast_ns() 鍑芥暟
		涓殑娉ㄩ噴銆?

	tai:
		璇ユ椂閽熸槸 tai 鏃堕挓锛圕LOCK_TAI锛夛紝娲剧敓鑷涓婃椂閽熸椂闂淬€備絾鏄紝璇ユ椂閽熶笉浼?
		缁忓巻鐢?NTP 鎻掑叆闂扮鎵€瀵艰嚧鐨勯棿鏂拰鍥炶烦銆傜敱浜庢椂閽熻闂槸涓鸿窡韪娇鐢ㄨ€岃璁★紝
		鍙兘浼氫骇鐢熷壇浣滅敤銆傚鏋滃唴閮?TAI 鍋忕Щ閲忚鏇存柊锛堜緥濡傜敱璁剧疆绯荤粺鏃堕棿鎴栦娇鐢?
		甯﹀亸绉婚噺鐨?adjtimex() 寮曡捣锛夛紝鏃堕挓璁块棶鍙兘浼氫骇鐢熼敊璇殑璇绘暟銆傝繖浜涙晥搴?
		寰堢綍瑙侊紝鍚庡鐞嗗簲褰撹兘澶熷鐞嗗畠浠€傛洿澶氫俊鎭鍙傞槄 ktime_get_tai_fast_ns()
		鍑芥暟涓殑娉ㄩ噴銆?

	瑕佽缃竴涓椂閽燂紝鍙渶灏嗘椂閽熷悕 echo 鍒拌鏂囦欢锛?

```
	  # echo global > trace_clock
```

	璁剧疆涓€涓椂閽熶細娓呴櫎鐜舰缂撳啿鍖虹殑鍐呭浠ュ強 "snapshot"锛堝揩鐓э級缂撳啿鍖恒€?

  trace_marker:

	璇ユ枃浠跺浜庡皢鐢ㄦ埛绌洪棿涓庡唴鏍镐腑鍙戠敓鐨勪簨浠跺悓姝ラ潪甯告湁鐢ㄣ€傚悜璇ユ枃浠跺啓鍏ュ瓧绗︿覆浼?
	琚啓鍏?ftrace 缂撳啿鍖恒€?

	鍦ㄥ簲鐢ㄧ▼搴忎腑锛屽湪搴旂敤绋嬪簭鍚姩鏃舵墦寮€璇ユ枃浠跺苟浠呭紩鐢ㄥ叾鏂囦欢鎻忚堪绗︽槸寰堟湁鐢ㄧ殑锛?

```
		void trace_write(const char *fmt, ...)
		{
			va_list ap;
			char buf[256];
			int n;

			if (trace_fd < 0)
				return;

			va_start(ap, fmt);
			n = vsnprintf(buf, 256, fmt, ap);
			va_end(ap);

			write(trace_fd, buf, n);
		}
```

	鍚姩锛?

```
		trace_fd = open("trace_marker", O_WRONLY);
```

	娉ㄦ剰锛氬啓鍏?trace_marker 鏂囦欢涔熷彲浠ヨЕ鍙戝啓鍏ュ埌
	/sys/kernel/tracing/events/ftrace/print/trigger 鐨勮Е鍙戝櫒銆傝鍙傞槄
	Documentation/trace/events.rst 涓殑 "Event triggers"锛堜簨浠惰Е鍙戝櫒锛変互鍙?
	Documentation/trace/histogram.rst锛堢 3 鑺傦級涓殑绀轰緥銆?

  trace_marker_raw:

	璇ユ枃浠剁被浼间簬涓婇潰鐨?trace_marker锛屼絾鐢ㄤ簬鍚戝叾鍐欏叆浜岃繘鍒舵暟鎹紝鍙互鐢ㄥ伐鍏蜂粠
	trace_pipe_raw 瑙ｆ瀽杩欎簺鏁版嵁銆?

  uprobe_events:

	鍦ㄧ▼搴忎腑娣诲姞鍔ㄦ€佽窡韪偣銆傝鍙傞槄 uprobetracer.rst銆?

  uprobe_profile:

	Uprobe 缁熻淇℃伅銆傝鍙傞槄 uprobetrace.txt銆?

  instances:

	杩欐槸涓€绉嶅垱寤哄涓窡韪紦鍐插尯鐨勬柟寮忥紝涓嶅悓鐨勪簨浠跺彲浠ヨ褰曞湪涓嶅悓鐨勭紦鍐插尯涓€?
	璇峰弬闃呬笅闈㈢殑 "Instances"锛堝疄渚嬶級涓€鑺傘€?

  events:

	璇ユ枃浠舵槸璺熻釜浜嬩欢鐩綍銆傚畠淇濆瓨宸茬紪璇戣繘鍐呮牳鐨勪簨浠惰窡韪偣锛堜篃绉颁负闈欐€佽窡韪偣锛夈€?
	瀹冩樉绀哄瓨鍦ㄥ摢浜涗簨浠惰窡韪偣锛屼互鍙婂畠浠浣曟寜绯荤粺鍒嗙粍銆傚湪涓嶅悓灞傜骇鏈?"enable"
	鏂囦欢锛屽悜瀹冧滑鍐欏叆 "1" 鍗冲彲鍚敤杩欎簺璺熻釜鐐广€?

	鏇村淇℃伅璇峰弬闃?events.rst銆?

  set_event:

	閫氳繃鍚戣鏂囦欢 echo 浜嬩欢鍚嶏紝灏嗗惎鐢ㄨ浜嬩欢銆?

	鏇村淇℃伅璇峰弬闃?events.rst銆?

  show_event_filters:

	甯︽湁杩囨护鍣ㄧ殑浜嬩欢鍒楄〃銆傚畠鏄剧ず绯荤粺/浜嬩欢瀵逛互鍙婃寕鎺ュ湪璇ヤ簨浠朵笂鐨勮繃婊ゅ櫒銆?

	鏇村淇℃伅璇峰弬闃?events.rst銆?

  show_event_triggers:

	甯︽湁瑙﹀彂鍣ㄧ殑浜嬩欢鍒楄〃銆傚畠鏄剧ず绯荤粺/浜嬩欢瀵逛互鍙婃寕鎺ュ湪璇ヤ簨浠朵笂鐨勮Е鍙戝櫒銆?

	鏇村淇℃伅璇峰弬闃?events.rst銆?

  available_events:

	鍙互杩涜璺熻釜鐨勫彲鐢ㄤ簨浠跺垪琛ㄣ€?

	鏇村淇℃伅璇峰弬闃?events.rst銆?

  timestamp_mode:

	鏌愪簺璺熻釜鍣ㄥ彲鑳戒細鏀瑰彉灏嗚窡韪簨浠惰褰曡繘浜嬩欢缂撳啿鍖烘椂鎵€浣跨敤鐨勬椂闂存埑妯″紡銆傚叿鏈?
	涓嶅悓妯″紡鐨勪簨浠跺彲浠ュ湪鍚屼竴缂撳啿鍖轰腑鍏卞瓨锛屼絾鍦ㄨ褰曟煇涓簨浠舵椂鐢熸晥鐨勬ā寮忓喅瀹氫簡
	璇ヤ簨浠朵娇鐢ㄥ摢绉嶆椂闂存埑妯″紡銆傞粯璁ょ殑鏃堕棿鎴虫ā寮忔槸 'delta'銆?

	璺熻釜甯哥敤鐨勬椂闂存埑妯″紡锛?

```
	  # cat timestamp_mode
	  [delta] absolute
```

	  甯︽湁鏂规嫭鍙风殑鏃堕棿鎴虫ā寮忔槸姝ｅ湪鐢熸晥鐨勬ā寮忋€?

	  delta: 榛樿鏃堕棿鎴虫ā寮?- 鏃堕棿鎴虫槸鐩稿浜庢瘡涓紦鍐插尯鏃堕棿鎴崇殑澧為噺銆?

	  absolute: 鏃堕棿鎴虫槸瀹屾暣鏃堕棿鎴筹紝鑰屼笉鏄浉瀵逛簬鍏朵粬鏌愪釜鍊肩殑澧為噺銆傚洜姝ゅ畠浼氬崰鐢?
	                鏇村绌洪棿锛屾晥鐜囦篃杈冧綆銆?

  hwlat_detector:

	纭欢寤惰繜鎺㈡祴鍣ㄧ殑鐩綍銆傝鍙傞槄涓嬮潰鐨?"Hardware Latency Detector"锛堢‖浠跺欢杩?
	鎺㈡祴鍣級涓€鑺傘€?

  per_cpu:

	璇ョ洰褰曞寘鍚?per_cpu 鐨勮窡韪俊鎭€?

  per_cpu/cpu0/buffer_size_kb:

	ftrace 缂撳啿鍖烘槸鎸?per_cpu 瀹氫箟鐨勩€備篃灏辨槸璇达紝姣忎釜 CPU 閮芥湁涓€涓嫭绔嬬殑缂撳啿鍖猴紝
	浠ヤ究鍐欏叆鍙互鍘熷瓙鏂瑰紡杩涜锛屽苟閬垮厤缂撳瓨鎶栧姩銆傝繖浜涚紦鍐插尯鍙兘鏈変笉鍚岀殑澶у皬銆傝鏂囦欢
	绫讳技浜?buffer_size_kb 鏂囦欢锛屼絾瀹冨彧鏄剧ず鎴栬缃壒瀹?CPU锛堟澶勪负 cpu0锛夌殑缂撳啿鍖哄ぇ灏忋€?

  per_cpu/cpu0/trace:

	璇ユ枃浠剁被浼间簬 "trace" 鏂囦欢锛屼絾瀹冨彧鏄剧ず鐗瑰畾浜庤 CPU 鐨勬暟鎹€傚鏋滃悜鍏跺啓鍏ワ紝瀹?
	鍙竻闄ょ壒瀹?CPU 鐨勭紦鍐插尯銆?

  per_cpu/cpu0/trace_pipe

	璇ユ枃浠剁被浼间簬 "trace_pipe" 鏂囦欢锛屽苟涓旀槸涓€涓秷璐瑰瀷璇诲彇锛屼絾瀹冨彧鏄剧ず锛堝苟娑堣垂锛?
	鐗瑰畾浜庤 CPU 鐨勬暟鎹€?

  per_cpu/cpu0/trace_pipe_raw

	瀵逛簬鑳藉瑙ｆ瀽 ftrace 鐜舰缂撳啿鍖轰簩杩涘埗鏍煎紡鐨勫伐鍏凤紝鍙互浣跨敤 trace_pipe_raw 鏂囦欢
	鐩存帴浠庣幆褰㈢紦鍐插尯鎻愬彇鏁版嵁銆傚€熷姪 splice() 绯荤粺璋冪敤锛岀紦鍐插尯鏁版嵁鍙互蹇€熶紶杈撳埌
	鏂囦欢鎴栫綉缁滐紝鐢辨湇鍔″櫒鏀堕泦杩欎簺鏁版嵁銆?

	涓?trace_pipe 涓€鏍凤紝杩欐槸涓€涓秷璐瑰瀷璇诲彇鍣紝澶氭璇诲彇鎬绘槸浼氫骇鐢熶笉鍚岀殑鏁版嵁銆?

  per_cpu/cpu0/snapshot:

	璇ユ枃浠剁被浼间簬涓?"snapshot" 鏂囦欢锛屼絾鍙細瀵瑰綋鍓?CPU 鎷嶆憚蹇収锛堝鏋滄敮鎸侊級銆傚畠鍙?
	鏄剧ず缁欏畾 CPU 鐨勫揩鐓у唴瀹癸紝濡傛灉鍚戝叾鍐欏叆锛屽垯鍙竻闄よ CPU 鐨勭紦鍐插尯銆?

  per_cpu/cpu0/snapshot_raw:

	绫讳技浜?trace_pipe_raw锛屼絾浼氫粠缁欏畾 CPU 鐨勫揩鐓х紦鍐插尯璇诲彇浜岃繘鍒舵牸寮忋€?

  per_cpu/cpu0/stats:

	璇ユ枃浠舵樉绀烘湁鍏崇幆褰㈢紦鍐插尯鐨勬煇浜涚粺璁′俊鎭細

	entries:
		缂撳啿鍖轰腑浠嶇劧瀛樺湪鐨勪簨浠舵暟閲忋€?

	overrun:
		鐢变簬缂撳啿鍖烘弧鑰岃瑕嗙洊鎵€涓㈠け鐨勪簨浠舵暟閲忋€?

	commit overrun:
		搴斿缁堜负闆躲€傚鏋滃湪宓屽浜嬩欢锛堢幆褰㈢紦鍐插尯鏄彲閲嶅叆鐨勶級涓彂鐢熶簡澶浜嬩欢锛?
		瀵艰嚧缂撳啿鍖哄～婊″苟寮€濮嬩涪寮冧簨浠讹紝璇ュ€间細琚缃€?

	bytes:
		瀹為檯璇诲彇鐨勫瓧鑺傛暟锛堟湭琚鐩栵級銆?

	oldest event ts:
		缂撳啿鍖轰腑鏈€鏃х殑鏃堕棿鎴?

	now ts:
		褰撳墠鏃堕棿鎴?

	dropped events:
		鐢变簬 overwrite 閫夐」鍏抽棴鑰屼涪澶辩殑浜嬩欢

	read events:
		宸茶鍙栫殑浜嬩欢鏁伴噺

```
### 璺熻釜鍣?


浠ヤ笅鏄綋鍓嶅彲浠ラ厤缃殑璺熻釜鍣ㄥ垪琛ㄣ€?

  "function"

	鐢ㄤ簬璺熻釜鎵€鏈夊唴鏍稿嚱鏁扮殑鍑芥暟璋冪敤璺熻釜鍣ㄣ€?

  "function_graph"

	绫讳技浜庡嚱鏁拌窡韪櫒锛屽尯鍒湪浜庡嚱鏁拌窡韪櫒鍦ㄥ嚱鏁板叆鍙ｅ鎺㈡祴锛岃€屽嚱鏁板浘璺熻釜鍣ㄥ湪
	鍑芥暟鐨勫叆鍙ｅ拰鍑哄彛閮借繘琛岃窡韪€傚畠杩樻彁渚涗簡缁樺埗绫讳技浜?C 浠ｇ爜婧愮爜鐨勫嚱鏁拌皟鐢?
	鍥剧殑鑳藉姏銆?

	娉ㄦ剰锛屽嚱鏁板浘鍦ㄥ唴閮ㄤ负姣忎釜瀹炰緥鍒嗗埆璁＄畻鍑芥暟寮€濮嬪拰杩斿洖鏃剁殑鏃堕棿銆傚鏋滄湁涓や釜
	瀹炰緥杩愯鍑芥暟鍥捐窡韪櫒骞惰窡韪浉鍚岀殑鍑芥暟锛岀敱浜庡悇鑷垎鍒鍙栨椂闂存埑鑰岄潪鍚屾椂璇诲彇锛?
	璁℃椂闀垮害鍙兘浼氭湁杞诲井鍋忓樊銆?

  "blk"

	鍧楄澶囪窡韪櫒銆俠lktrace 鐢ㄦ埛鎬佸簲鐢ㄧ▼搴忔墍浣跨敤鐨勮窡韪櫒銆?

  "hwlat"

	纭欢寤惰繜璺熻釜鍣紝鐢ㄤ簬妫€娴嬬‖浠舵槸鍚︿骇鐢熶换浣曞欢杩熴€傝鍙傞槄涓嬮潰鐨?"Hardware Latency
	Detector"锛堢‖浠跺欢杩熸帰娴嬪櫒锛変竴鑺傘€?

  "irqsoff"

	璺熻釜绂佺敤涓柇鐨勫尯鍩燂紝骞朵繚瀛樺叿鏈夋渶闀挎渶澶у欢杩熺殑璺熻釜銆傝鍙傞槄 tracing_max_latency銆?
	褰撹褰曞埌鏂扮殑鏈€澶у€兼椂锛屽畠浼氭浛鎹㈡棫鐨勮窡韪€傛渶濂介厤鍚?latency-format 閫夐」鍚敤鏃?
	鏌ョ湅姝よ窡韪紝閫夋嫨璇ヨ窡韪櫒鏃朵細鑷姩鍚敤璇ラ€夐」銆?

  "preemptoff"

	绫讳技浜?irqsoff锛屼絾璺熻釜骞惰褰曟姠鍗犺绂佺敤鐨勬椂闂撮暱搴︺€?

  "preemptirqsoff"

	绫讳技浜?irqsoff 鍜?preemptoff锛屼絾璺熻釜骞惰褰?irq 鍜?鎴栨姠鍗犺绂佺敤鐨勬渶闀挎椂闂淬€?

  "wakeup"

	璺熻釜骞惰褰曟渶楂樹紭鍏堢骇浠诲姟琚敜閱掑悗鍒板畠琚皟搴︽墍鑺辫垂鐨勬渶澶у欢杩熴€傛寜鐓ф櫘閫氬紑鍙戣€呯殑
	棰勬湡璺熻釜鎵€鏈変换鍔°€?

  "wakeup_rt"

	璺熻釜骞惰褰曚粎浠?RT 浠诲姟锛堝鍚屽綋鍓嶇殑 "wakeup" 閭ｆ牱锛夎鍞ら啋鎵€鑺辫垂鐨勬渶澶у欢杩熴€傝繖
	瀵瑰叧娉?RT 浠诲姟鍞ら啋鏃堕棿鐨勪汉寰堟湁鐢ㄣ€?

  "wakeup_dl"

	璺熻釜骞惰褰?SCHED_DEADLINE 浠诲姟琚敜閱掞紙濡傚悓 "wakeup" 鍜?"wakeup_rt" 閭ｆ牱锛夋墍
	鑺辫垂鐨勬渶澶у欢杩熴€?

  "mmiotrace"

	涓€绉嶇敤浜庤窡韪簩杩涘埗妯″潡鐨勭壒娈婅窡韪櫒銆傚畠浼氳窡韪竴涓ā鍧楀纭欢杩涜鐨勬墍鏈夎皟鐢紝
	浠ュ強瀹冧粠 I/O 璇诲啓鐨勬墍鏈夊唴瀹广€?

  "branch"

	璇ヨ窡韪櫒鍙互鍦ㄨ窡韪唴鏍镐腑鐨?likely/unlikely 璋冪敤鏃堕厤缃€傚畠浼氳窡韪懡涓竴涓?
	likely 鎴?unlikely 鍒嗘敮鐨勬椂鏈猴紝浠ュ強瀹冨璇ュ垎鏀娴嬬殑鏄惁姝ｇ‘銆?

  "nop"

	杩欐槸"浠€涔堥兘涓嶈窡韪?鐨勮窡韪櫒銆傝绉婚櫎鎵€鏈夎窡韪櫒锛屽彧闇€鍚?current_tracer echo
	"nop" 鍗冲彲銆?

### 閿欒鎯呭舰


  瀵逛簬澶у鏁?ftrace 鍛戒护锛屽け璐ユā寮忔槸鏄捐€屾槗瑙佺殑锛屽苟涓斾娇鐢ㄦ爣鍑嗚繑鍥炵爜杩涜鍙嶉銆?

  瀵逛簬鍏朵粬鏇村鏉傜殑鍛戒护锛屽彲閫氳繃 tracing/error_log 鏂囦欢鑾峰彇鎵╁睍閿欒淇℃伅銆傚浜庢敮鎸?
  瀹冪殑鍛戒护锛屽湪鍑洪敊鍚庤鍙?tracing/error_log 鏂囦欢浼氭樉绀烘湁鍏冲嚭閿欏師鍥犵殑鏇磋缁嗕俊鎭?
  锛堝鏋滄湁淇℃伅鍙敤锛夈€倀racing/error_log 鏂囦欢鏄竴涓惊鐜敊璇棩蹇楋紝鏄剧ず灏戦噺锛堝綋鍓嶄负
  8 鏉★級鏈€杩戠殑锛? 鏉★級澶辫触鍛戒护鐨?ftrace 閿欒銆?

  鎵╁睍閿欒淇℃伅鍙婄敤娉曢噰鐢ㄤ互涓嬪舰寮?

```

    # echo xxx > /sys/kernel/tracing/events/sched/sched_wakeup/trigger
    echo: write error: Invalid argument

    # cat /sys/kernel/tracing/error_log
    [ 5348.887237] location: error: Couldn't yyy: zzz
      Command: xxx
               ^
    [ 7517.023364] location: error: Bad rrr: sss
      Command: ppp qqq
                   ^

  瑕佹竻闄ら敊璇棩蹇楋紝鍚戝畠 echo 绌哄瓧绗︿覆锛?

```

    # echo > /sys/kernel/tracing/error_log

```
### 浣跨敤璺熻釜鍣ㄧ殑绀轰緥


浠ヤ笅鏄湪浠呬娇鐢?tracefs 鎺ュ彛锛堜笉浣跨敤浠讳綍鐢ㄦ埛鎬佸伐鍏凤級鎺у埗璺熻釜鍣ㄦ椂鐨勫吀鍨嬬ず渚嬨€?

### 杈撳嚭鏍煎紡锛?


```

  # tracer: function
  #
  # entries-in-buffer/entries-written: 140080/250280   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
              bash-1977  [000] .... 17284.993652: sys_close <-system_call_fastpath
              bash-1977  [000] .... 17284.993653: __close_fd <-sys_close
              bash-1977  [000] .... 17284.993653: _raw_spin_lock <-__close_fd
              sshd-1974  [003] .... 17284.993653: __srcu_read_unlock <-fsnotify
              bash-1977  [000] .... 17284.993654: add_preempt_count <-_raw_spin_lock
              bash-1977  [000] ...1 17284.993655: _raw_spin_unlock <-__close_fd
              bash-1977  [000] ...1 17284.993656: sub_preempt_count <-_raw_spin_unlock
              bash-1977  [000] .... 17284.993657: filp_close <-__close_fd
              bash-1977  [000] .... 17284.993657: dnotify_flush <-filp_close
              sshd-1974  [003] .... 17284.993658: sys_select <-system_call_fastpath
              ....

```

浼氭墦鍗颁竴涓ご閮紝鍏朵腑鍖呭惈鐢辫窡韪〃绀虹殑璺熻釜鍣ㄥ悕銆傚湪鏈緥涓窡韪櫒鏄?"function"銆傜劧鍚庡畠
鏄剧ず缂撳啿鍖轰腑鐨勪簨浠舵暟閲忎互鍙婂凡鍐欏叆鐨勬潯鐩€绘暟銆備袱鑰呯殑宸€煎氨鏄敱浜庣紦鍐插尯濉弧鑰屼涪澶辩殑
鏉＄洰鏁帮紙250280 - 140080 = 110200 涓簨浠朵涪澶憋級銆?

澶撮儴瑙ｉ噴浜嗕簨浠剁殑鍐呭銆備换鍔″悕 "bash"銆佷换鍔?PID "1977"銆佽繍琛屾墍鍦ㄧ殑 CPU "000"銆佸欢杩熸牸寮?
锛堝涓嬭В閲婏級銆?secs>.<usecs> 鏍煎紡鐨勬椂闂存埑銆佽璺熻釜鐨勫嚱鏁板悕 "sys_close" 浠ュ強璋冪敤璇ュ嚱鏁扮殑
鐖跺嚱鏁?"system_call_fastpath"銆傛椂闂存埑鏄嚱鏁拌杩涘叆鐨勬椂闂淬€?

### 寤惰繜璺熻釜鏍煎紡


褰撳惎鐢ㄤ簡 latency-format 閫夐」锛屾垨鑰呰缃簡鏌愪釜寤惰繜璺熻釜鍣ㄦ椂锛宼race 鏂囦欢浼氭彁渚涙洿澶氫俊鎭?
浠ヤ究鏌ョ湅

```

  # tracer: irqsoff
  #
  # irqsoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 259 us, #4/4, CPU#2 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: ps-6143 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: __lock_task_sighand
  #  => ended at:   _raw_spin_unlock_irqrestore
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
        ps-6143    2d...    0us!: trace_hardirqs_off <-__lock_task_sighand
        ps-6143    2d..1  259us+: trace_hardirqs_on <-_raw_spin_unlock_irqrestore
        ps-6143    2d..1  263us+: time_hardirqs_on <-_raw_spin_unlock_irqrestore
        ps-6143    2d..1  306us : <stack trace>
   => trace_hardirqs_on_caller
   => trace_hardirqs_on
   => _raw_spin_unlock_irqrestore
   => do_task_stat
   => proc_tgid_stat
   => proc_single_show
   => seq_read
   => vfs_read
   => sys_read
   => system_call_fastpath


```
杩欒〃鏄庡綋鍓嶈窡韪櫒鏄?"irqsoff"锛屾鍦ㄨ窡韪腑鏂绂佺敤鐨勬椂闂淬€傚畠缁欏嚭璺熻釜鐗堟湰锛堜粠涓嶆敼鍙橈級
浠ュ強鍏朵笂鎵ц鐨勫唴鏍哥増鏈紙3.8锛夈€傜劧鍚庡畠鏄剧ず鏈€澶у欢杩燂紙浠ュ井绉掍负鍗曚綅锛?59 us锛夈€傛樉绀虹殑
璺熻釜鏉＄洰鏁颁互鍙婃€绘暟锛堜袱鑰呴兘鏄洓锛?4/4锛夈€俈P銆並P銆丼P 鍜?HP 濮嬬粓涓洪浂锛屼繚鐣欎緵浠ュ悗浣跨敤銆?
#P 鏄湪绾?CPU 鐨勬暟閲忥紙#P:4锛夈€?

浠诲姟鏄欢杩熷彂鐢熸椂姝ｅ湪杩愯鐨勮繘绋嬨€傦紙ps pid锛?143锛夈€?

瀵艰嚧寤惰繜鐨勫紑濮嬪拰鍋滄锛堝垎鍒槸绂佺敤鍜屽惎鐢ㄤ腑鏂殑鍑芥暟锛夛細

  - __lock_task_sighand 鏄鐢ㄤ腑鏂殑浣嶇疆銆?
  - _raw_spin_unlock_irqrestore 鏄噸鏂板惎鐢ㄤ腑鏂殑浣嶇疆銆?

澶撮儴涔嬪悗鐨勫嚑琛屾槸璺熻釜鏈韩銆傚ご閮ㄨВ閲婁簡鍝釜鏄摢涓€?

  cmd: 璺熻釜涓繘绋嬬殑鍚嶅瓧銆?

  pid: 璇ヨ繘绋嬬殑 PID銆?

  CPU#: 璇ヨ繘绋嬭繍琛岀殑 CPU銆?

  irqs-off: 'd' 琛ㄧず涓柇琚鐢紝鍚﹀垯涓?'.'銆?

  need-resched:
 - 'B' 琛ㄧず TIF_NEED_RESCHED銆丳REEMPT_NEED_RESCHED 鍜?TIF_RESCHED_LAZY 閮藉凡璁剧疆锛?
 - 'N' 琛ㄧず TIF_NEED_RESCHED 鍜?PREEMPT_NEED_RESCHED 閮藉凡璁剧疆锛?
 - 'n' 浠?TIF_NEED_RESCHED 琚缃紝
 - 'p' 浠?PREEMPT_NEED_RESCHED 琚缃紝
 - 'L' 琛ㄧず PREEMPT_NEED_RESCHED 鍜?TIF_RESCHED_LAZY 閮藉凡璁剧疆锛?
 - 'b' 琛ㄧず TIF_NEED_RESCHED 鍜?TIF_RESCHED_LAZY 閮藉凡璁剧疆锛?
 - 'l' 浠?TIF_RESCHED_LAZY 琚缃?
 - '.' 鍚﹀垯銆?

  hardirq/softirq:
 - 'Z' - 鍦ㄧ‖涓柇鍐呴儴鍙戠敓浜?NMI
 - 'z' - NMI 姝ｅ湪杩愯
 - 'H' - 鍦ㄨ蒋涓柇鍐呴儴鍙戠敓浜嗙‖涓柇銆?
 - 'h' - 纭腑鏂鍦ㄨ繍琛?
 - 's' - 杞腑鏂鍦ㄨ繍琛?
 - '.' - 鏅€氫笂涓嬫枃銆?

  preempt-depth: preempt_disabled 鐨勫眰绾?

  浠ヤ笂鍐呭涓昏瀵瑰唴鏍稿紑鍙戣€呮湁鎰忎箟銆?

  time:
	褰撳惎鐢ㄤ簡 latency-format 閫夐」鏃讹紝trace 鏂囦欢杈撳嚭鍖呭惈鐩稿浜庤窡韪紑濮嬬殑鏃堕棿鎴炽€?
	杩欎笌绂佺敤 latency-format 鏃惰緭鍑虹粷瀵规椂闂存埑涓嶅悓銆?

  delay:
	杩欎粎涓轰簡鏇村ソ鍦板惛寮曚綘鐨勭溂鐞冦€傚畠闇€瑕佽淇涓哄彧鐩稿浜庡悓涓€ CPU銆傝繖浜涙爣璁扮敱褰撳墠
	杩欐潯璺熻釜涓庝笅涓€鏉¤窡韪箣闂寸殑宸€煎喅瀹氥€?

   - '$' - 澶т簬 1 绉?
   - '@' - 澶т簬 100 姣
   - '*' - 澶т簬 10 姣
   - '#' - 澶т簬 1000 寰
   - '!' - 澶т簬 100 寰
   - '+' - 澶т簬 10 寰
   - ' ' - 灏忎簬鎴栫瓑浜?10 寰銆?

  鍏朵綑閮ㄥ垎涓?'trace' 鏂囦欢鐩稿悓銆?

  娉ㄦ剰锛屽欢杩熻窡韪櫒閫氬父浠ヤ竴涓爤鍥炴函缁撴潫锛屼互渚胯交鏉炬壘鍒板欢杩熷彂鐢熺殑浣嶇疆銆?

### trace_options


trace_options 鏂囦欢锛堟垨 options 鐩綍锛夌敤浜庢帶鍒惰窡韪緭鍑轰腑鎵撳嵃浠€涔堬紝鎴栬€呮搷绾佃窡韪櫒銆?

```

  cat trace_options
	print-parent
	nosym-offset
	nosym-addr
	noverbose
	noraw
	nohex
	nobin
	noblock
	nofields
	trace_printk
	annotate
	nouserstacktrace
	nosym-userobj
	noprintk-msg-only
	context-info
	nolatency-format
	record-cmd
	norecord-tgid
	overwrite
	nodisable_on_free
	irq-info
	markers
	noevent-fork
	function-trace
	nofunction-fork
	nodisplay-graph
	nostacktrace
	nobranch

```
瑕佺鐢ㄥ叾涓竴涓€夐」锛屽悜鍏朵腑 echo 甯︽湁鍓嶇紑鐨勯€夐」

```

  echo noprint-parent > trace_options

```

```

  echo sym-offset > trace_options

```
浠ヤ笅鏄彲鐢ㄩ€夐」锛?

  print-parent
	鍦ㄥ嚱鏁拌窡韪腑锛屾樉绀鸿璋冪敤锛堢埗锛夊嚱鏁颁互鍙婃鍦ㄨ璺熻釜鐨勫嚱鏁般€?

```

	  print-parent:
	   bash-4000  [01]  1477.606694: simple_strtoul <-kstrtoul

	  noprint-parent:
	   bash-4000  [01]  1477.606694: simple_strtoul


  sym-offset
	涓嶄粎鏄剧ず鍑芥暟鍚嶏紝杩樻樉绀哄嚱鏁板唴鐨勫亸绉婚噺銆備緥濡傦紝浣犵湅鍒扮殑涓嶅啀鏄?"ktime_get"锛?
	鑰屾槸 "ktime_get+0xb/0x20"銆?

```

	  sym-offset:
	   bash-4000  [01]  1477.606694: simple_strtoul+0x6/0xa0

  sym-addr
	杩欒繕浼氭樉绀哄嚱鏁板湴鍧€浠ュ強鍑芥暟鍚嶃€?

```

	  sym-addr:
	   bash-4000  [01]  1477.606694: simple_strtoul <c0339346>

  verbose
	璇ラ€夐」澶勭悊

```
        latency-format 閫夐」鍚敤鏃剁殑 trace 鏂囦欢銆?

```

	    bash  4000 1 0 00000000 00010a95 [58127d26] 1720.415ms \
	    (+0.000ms): simple_strtoul (kstrtoul)

  raw
	璇ラ€夐」鏄剧ず鍘熷鏁板瓧銆傛閫夐」鏈€閫傚悎涓庤兘澶熸洿濂藉湴缈昏瘧鍘熷鏁板瓧鐨勭敤鎴锋€佸簲鐢ㄧ▼搴?
	閰嶅悎浣跨敤锛岃€屼笉鏄敱鍐呮牳鏉ュ畬鎴愩€?

  hex
	绫讳技浜?raw锛屼絾鏁板瓧閲囩敤鍗佸叚杩涘埗鏍煎紡銆?

  bin
	璇ラ€夐」浠ュ師濮嬩簩杩涘埗鏍煎紡鎵撳嵃杈撳嚭銆?

  block
	璁剧疆鍚庯紝杞璇诲彇 trace_pipe 鏃朵笉浼氶樆濉炪€?

  fields
	鎸夊叾绫诲瀷鎵€杩版墦鍗板瓧娈点€傝繖姣斾娇鐢?hex銆乥in 鎴?raw 鏇村ソ锛屽洜涓哄畠鑳芥洿濂藉湴瑙ｆ瀽
	浜嬩欢鐨勫唴瀹广€?

  trace_printk
	鍙互绂佹 trace_printk() 鍐欏叆缂撳啿鍖恒€?

  trace_printk_dest
	璁剧疆鍚庤 trace_printk() 鍙婄被浼肩殑鍐呴儴璺熻釜鍑芥暟鍐欏叆姝ゅ疄渚嬨€傛敞鎰忥紝鍙湁涓€涓窡韪?
	瀹炰緥鍙互璁剧疆姝ゆ爣蹇椼€傝缃鏍囧織浼氭竻闄や箣鍓嶈缃簡璇ユ爣蹇楃殑瀹炰緥鐨?trace_printk_dest
	鏍囧織銆傞粯璁ゆ儏鍐典笅锛岄《灞傝窡韪叿鏈夋璁剧疆锛屽鏋滃彟涓€涓疄渚嬭缃簡瀹冪劧鍚庡張娓呴櫎瀹冿紝
	椤跺眰璺熻釜浼氶噸鏂拌幏寰楄璁剧疆銆?

	姝ゆ爣蹇椾笉鑳借椤跺眰瀹炰緥娓呴櫎锛屽洜涓哄畠鏄粯璁ゅ疄渚嬨€傞《灞傚疄渚嬫竻闄ゆ鏍囧織鐨勫敮涓€鏂瑰紡锛?
	鏄敱鍙︿竴涓疄渚嬭缃畠銆?

  copy_trace_marker
	濡傛灉鏈夊簲鐢ㄧ▼搴忕‖缂栫爜鍐欏叆椤跺眰 trace_marker 鏂囦欢锛?sys/kernel/tracing/trace_marker
	鎴?trace_marker_raw锛夛紝鑰屽伐鍏峰笇鏈涘皢鍏惰浆鍒版煇涓疄渚嬶紝鍒欏彲浠ヤ娇鐢ㄦ閫夐」銆傚垱寤轰竴涓?
	瀹炰緥骞惰缃閫夐」锛屼箣鍚庢墍鏈夊椤跺眰 trace_marker 鏂囦欢鐨勫啓鍏ヤ篃閮戒細琚噸瀹氬悜鍒拌
	瀹炰緥銆?

	娉ㄦ剰锛岄粯璁ゆ儏鍐典笅椤跺眰瀹炰緥璁剧疆浜嗘閫夐」銆傚鏋滃畠琚鐢紝閭ｄ箞瀵?trace_marker 鎴?
	trace_marker_raw 鏂囦欢鐨勫啓鍏ュ皢涓嶄細琚啓鍏ラ《灞傛枃浠躲€傚鏋滄病鏈変换浣曞疄渚嬭缃閫夐」锛?
	鍒欏啓鍏ュ皢浠?ENODEV 閿欒鐮佸け璐ャ€?

  annotate
	褰?CPU 缂撳啿鍖哄凡婊★紝涓旀煇涓?CPU 缂撳啿鍖烘渶杩戞湁澶ч噺浜嬩欢锛堝洜鑰屾椂闂寸獥鍙ｈ緝鐭級锛岃€?
	鍙︿竴涓?CPU 鍙兘鍙湁灏戦噺浜嬩欢锛堜粠鑰屽彲浠ヤ繚鐣欒緝鏃х殑浜嬩欢锛夋椂锛屾儏鍐垫湁鏃朵細浠や汉鍥版儜銆?
	褰撴姤鍛婅窡韪椂锛屽畠鍏堟樉绀烘渶鏃х殑浜嬩欢锛屽苟涓斿彲鑳界湅璧锋潵濂藉儚鍙湁杩愯鏃堕棿鏈€闀跨殑閭ｄ釜
	CPU锛堟嫢鏈夋渶鏃т簨浠剁殑閭ｄ釜锛夊湪杩愯銆傚綋璁剧疆浜?annotate 閫夐」鏃讹紝瀹冧細鏄剧ず涓€涓柊鐨?
	CPU 缂撳啿鍖轰綍鏃跺紑濮嬶細

```
			  <idle>-0     [001] dNs4 21169.031481: wake_up_idle_cpu <-add_timer_on
			  <idle>-0     [001] dNs4 21169.031482: _raw_spin_unlock_irqrestore <-add_timer_on
			  <idle>-0     [001] .Ns4 21169.031484: sub_preempt_count <-_raw_spin_unlock_irqrestore
		##### CPU 2 buffer started ####
			  <idle>-0     [002] .N.1 21169.031484: rcu_idle_exit <-cpu_idle
			  <idle>-0     [001] .Ns3 21169.031484: _raw_spin_unlock <-clocksource_watchdog
			  <idle>-0     [001] .Ns3 21169.031485: sub_preempt_count <-_raw_spin_unlock
```

  userstacktrace
	璇ラ€夐」浼氭敼鍙樿窡韪€傚畠鍦ㄦ瘡娆¤窡韪簨浠朵箣鍚庤褰曞綋鍓嶇敤鎴风┖闂寸嚎绋嬬殑鏍堝洖婧€?

  sym-userobj
	褰撳惎鐢ㄤ簡鐢ㄦ埛鏍堝洖婧椂锛屾煡鎵捐鍦板潃灞炰簬鍝釜瀵硅薄锛屽苟鎵撳嵃鐩稿鍦板潃銆傚綋寮€鍚簡 ASLR
	鏃惰繖灏ゅ叾鏈夌敤锛屽惁鍒欏湪搴旂敤涓嶅啀杩愯鍚庯紝浣犳棤娉曞皢鍦板潃瑙ｆ瀽涓哄璞?鏂囦欢/琛屻€?

	鏌ユ壘鍦ㄤ綘璇诲彇 trace銆乼race_pipe 鏃舵墽琛屻€傜ず渚嬶細

```

		  a.out-1623  [000] 40874.465068: /root/a.out[+0x480] <-/root/a.out[+0
		  x494] <- /root/a.out[+0x4a8] <- /lib/libc-2.7.so[+0x1e1a6]


  printk-msg-only
	璁剧疆鍚庯紝trace_printk() 灏嗗彧鏄剧ず鏍煎紡鑰屼笉鏄剧ず鍏跺弬鏁帮紙濡傛灉浣跨敤浜?trace_bprintk()
	鎴?trace_bputs() 鏉ヤ繚瀛?trace_printk()锛夈€?

  context-info
	鍙樉绀轰簨浠舵暟鎹€傞殣钘?comm銆丳ID銆佹椂闂存埑銆丆PU 鍜屽叾浠栨湁鐢ㄦ暟鎹€?

  latency-format
	璇ラ€夐」浼氭敼鍙樿窡韪緭鍑恒€傚惎鐢ㄦ椂锛岃窡韪細鏄剧ず鏈夊叧寤惰繜鐨勯檮鍔犱俊鎭紝濡?"Latency trace
	format"锛堝欢杩熻窡韪牸寮忥級涓墍杩般€?

  pause-on-trace
	璁剧疆鍚庯紝涓鸿鍙栬€屾墦寮€ trace 鏂囦欢浼氭殏鍋滃悜鐜舰缂撳啿鍖虹殑鍐欏叆锛堝鍚?tracing_on 琚?
	璁剧疆涓?0锛夈€傝繖妯℃嫙浜?trace 鏂囦欢鏈€鍒濈殑琛屼负銆傚綋鏂囦欢鍏抽棴鏃讹紝璺熻釜浼氶噸鏂板惎鐢ㄣ€?

  hash-ptr
        璁剧疆鍚庯紝浜嬩欢 printk 鏍煎紡涓殑 "%p" 鏄剧ず鍝堝笇鍚庣殑鎸囬拡鍊艰€屼笉鏄湡瀹炲湴鍧€銆傚鏋滀綘
	鎯虫煡鏄庤窡韪棩蹇椾腑鍝釜鍝堝笇鍊煎搴斾簬鐪熷疄鍊硷紝杩欎細寰堟湁鐢ㄣ€?

  bitmask-list
        鍚敤鍚庯紝浣嶆帺鐮佷娇鐢?printk 鐨?"%*pbl" 鏍煎紡璇存槑绗︽樉绀轰负鍙鐨勮寖鍥村垪琛紙渚嬪
	0,2-5,7锛夈€傜鐢ㄦ椂锛堥粯璁わ級锛屼綅鎺╃爜浠ヤ紶缁熺殑鍗佸叚杩涘埗浣嶅浘琛ㄧず褰㈠紡鏄剧ず銆傚垪琛ㄦ牸寮?
	瀵逛簬璺熻釜 CPU 鎺╃爜鍜屽叾浠栧ぇ鍨嬩綅鎺╃爜鐗瑰埆鏈夌敤锛屽叾涓崟涓綅鐨勪綅缃瘮鍏跺崄鍏繘鍒剁紪鐮?
	鏇存湁鎰忎箟銆?

  record-cmd
	褰撳惎鐢ㄤ换浣曚簨浠舵垨璺熻釜鍣ㄦ椂锛屼細鍦?sched_switch 璺熻釜鐐逛腑鍚敤涓€涓挬瀛愶紝浠ュ～鍏呮槧灏勪簡
	pid 鍜?comm 鐨?comm 缂撳瓨銆備絾杩欏彲鑳戒細甯︽潵涓€浜涘紑閿€锛屽鏋滀綘鍙叧蹇?pid 鑰屼笉鍏冲績浠诲姟
	鍚嶏紝绂佺敤姝ら€夐」鍙互闄嶄綆璺熻釜鐨勫奖鍝嶃€傝鍙傞槄 "saved_cmdlines"銆?

  record-tgid
	褰撳惎鐢ㄤ换浣曚簨浠舵垨璺熻釜鍣ㄦ椂锛屼細鍦?sched_switch 璺熻釜鐐逛腑鍚敤涓€涓挬瀛愶紝浠ュ～鍏呮槧灏勪簡
	绾跨▼缁?ID锛圱GID锛夊埌 pid 鐨勭紦瀛樸€傝鍙傞槄 "saved_tgids"銆?

  overwrite
	璇ラ€夐」鎺у埗褰撹窡韪紦鍐插尯宸叉弧鏃跺彂鐢熺殑鎯呭喌銆傚鏋滀负 "1"锛堥粯璁わ級锛屾渶鏃х殑浜嬩欢浼氳
	涓㈠純骞惰鐩栥€傚鏋滀负 "0"锛屽垯涓㈠純鏈€鏂扮殑浜嬩欢銆?
	锛堣 per_cpu/cpu0/stats 涓殑 overrun 鍜?dropped锛?

  disable_on_free
	褰?free_buffer 鍏抽棴鏃讹紝璺熻釜浼氬仠姝紙tracing_on 琚缃负 0锛夈€?

  irq-info
	鏄剧ず涓柇銆佹姠鍗犺鏁般€乶eed resched 鏁版嵁銆傜鐢ㄦ椂锛岃窡韪湅璧锋潵濡備笅锛?

```

		# tracer: function
		#
		# entries-in-buffer/entries-written: 144405/9452052   #P:4
		#
		#           TASK-PID   CPU#      TIMESTAMP  FUNCTION
		#              | |       |          |         |
			  <idle>-0     [002]  23636.756054: ttwu_do_activate.constprop.89 <-try_to_wake_up
			  <idle>-0     [002]  23636.756054: activate_task <-ttwu_do_activate.constprop.89
			  <idle>-0     [002]  23636.756055: enqueue_task <-activate_task


  markers
	璁剧疆鍚庯紝trace_marker 鍙啓锛堜粎 root锛夈€傜鐢ㄦ椂锛屽 trace_marker 鐨勫啓鍏ュ皢浠?EINVAL
	閿欒銆?

  event-fork
	璁剧疆鍚庯紝鍒楀湪 set_event_pid 涓殑 PID 鐨勪换鍔″湪 fork 鏃讹紝鍏跺瓙杩涚▼鐨?PID 浼氳娣诲姞鍒?
	set_event_pid銆傚悓鏍凤紝褰撳垪鍦?set_event_pid 涓殑 PID 鐨勪换鍔￠€€鍑烘椂锛屽叾 PID 浼氫粠璇?
	鏂囦欢涓Щ闄ゃ€?

        杩欏悓鏍蜂細褰卞搷鍒楀湪 set_event_notrace_pid 涓殑 PID銆?

  function-trace
	濡傛灉鍚敤浜嗘閫夐」锛堥粯璁ゅ惎鐢級锛屽欢杩熻窡韪櫒灏嗗惎鐢ㄥ嚱鏁拌窡韪€傜鐢ㄦ椂锛屽欢杩熻窡韪櫒
	涓嶄細璺熻釜鍑芥暟銆傝繖鍦ㄨ繘琛屽欢杩熸祴璇曟椂闄嶄綆浜嗚窡韪櫒鐨勫紑閿€銆?

  function-fork
	璁剧疆鍚庯紝鍒楀湪 set_ftrace_pid 涓殑 PID 鐨勪换鍔″湪 fork 鏃讹紝鍏跺瓙杩涚▼鐨?PID 浼氳娣诲姞鍒?
	set_ftrace_pid銆傚悓鏍凤紝褰撳垪鍦?set_ftrace_pid 涓殑 PID 鐨勪换鍔￠€€鍑烘椂锛屽叾 PID 浼?
	浠庤鏂囦欢涓Щ闄ゃ€?

        杩欏悓鏍蜂細褰卞搷鍒楀湪 set_ftrace_notrace_pid 涓殑 PID銆?

  display-graph
	璁剧疆鍚庯紝寤惰繜璺熻釜鍣紙irqsoff銆亀akeup 绛夛級灏嗕娇鐢ㄥ嚱鏁板浘璺熻釜鑰屼笉鏄嚱鏁拌窡韪€?

  stacktrace
	璁剧疆鍚庯紝鍦ㄨ褰曚换浣曡窡韪簨浠跺悗浼氳褰曚竴鏉℃爤鍥炴函銆?

  branch
	鐢ㄨ窡韪櫒鍚敤鍒嗘敮璺熻釜銆傝繖浼氬惎鐢ㄥ垎鏀窡韪櫒浠ュ強褰撳墠璁剧疆鐨勮窡韪櫒銆傜敤 "nop" 璺熻釜鍣?
	鍚敤姝ら€夐」绛夊悓浜庝粎鍚敤 "branch" 璺熻釜鍣ㄣ€?

```

       file when the tracer is active. They always appear in the
       options directory.


浠ヤ笅鏄悇璺熻釜鍣ㄧ殑閫夐」锛?

鍑芥暟璺熻釜鍣ㄧ殑閫夐」锛?

  func_stack_trace
	璁剧疆鍚庯紝鍦ㄦ瘡娆¤褰曠殑鍑芥暟涔嬪悗閮戒細璁板綍涓€鏉℃爤鍥炴函銆傛敞鎰忥紒鍦ㄥ惎鐢ㄦ閫夐」涔嬪墠锛屽厛鐢?
	"set_ftrace_filter" 闄愬埗琚褰曠殑鍑芥暟锛屽惁鍒欑郴缁熸€ц兘浼氫弗閲嶄笅闄嶃€傝寰楀湪娓呴櫎鍑芥暟
	杩囨护鍣ㄤ箣鍓嶇鐢ㄦ閫夐」銆?

鍑芥暟鍥捐窡韪櫒鐨勯€夐」锛?

  鐢变簬鍑芥暟鍥捐窡韪櫒鐨勮緭鍑虹暐鏈変笉鍚岋紝瀹冩湁鑷繁鐨勪竴缁勯€夐」鏉ユ帶鍒舵樉绀哄唴瀹广€?

  funcgraph-overrun
	璁剧疆鍚庯紝鍦ㄦ瘡娆¤璺熻釜鐨勫嚱鏁颁箣鍚庝細鏄剧ず鍥炬爤鐨?婧㈠嚭"銆傛孩鍑烘槸鎸囪皟鐢ㄦ爤娣卞害澶т簬涓烘瘡涓?
	浠诲姟淇濈暀鐨勬繁搴︺€傛瘡涓换鍔￠兘鏈変竴涓浐瀹氬ぇ灏忕殑鍑芥暟鏁扮粍鐢ㄤ簬璋冪敤鍥捐窡韪€傚鏋滆皟鐢?
	娣卞害瓒呰繃璇ユ暟缁勶紝璇ュ嚱鏁板氨涓嶄細琚窡韪€傛孩鍑哄氨鏄敱浜庤秴鍑鸿鏁扮粍鑰岄敊杩囩殑鍑芥暟鏁伴噺銆?

  funcgraph-cpu
	璁剧疆鍚庯紝鏄剧ず鍙戠敓璺熻釜鐨?CPU 鐨?CPU 缂栧彿銆?

  funcgraph-overhead
	璁剧疆鍚庯紝濡傛灉鍑芥暟鑺辫垂鐨勬椂闂磋秴杩囦竴瀹氶噺锛屽垯浼氭樉绀轰竴涓欢杩熸爣璁般€傝涓婇潰澶撮儴鎻忚堪
	涓嬬殑 "delay"銆?

  funcgraph-proc
	涓庡叾浠栬窡韪櫒涓嶅悓锛岃繘绋嬬殑鍛戒护琛岄粯璁や笉鏄剧ず锛岃€屾槸浠呭湪涓婁笅鏂囧垏鎹㈡湡闂翠换鍔¤璺熻釜
	杩涘叆鍜岄€€鍑烘椂鎵嶆樉绀恒€傚惎鐢ㄦ閫夐」浼氳姣忎釜杩涚▼鐨勫懡浠ゆ樉绀哄湪姣忎釜琛屼笂銆?

  funcgraph-duration
	鍦ㄦ瘡涓嚱鏁扮粨鏉熸椂锛堣繑鍥炴椂锛夛紝鏄剧ず璇ュ嚱鏁颁腑鑺辫垂鐨勬椂闂撮暱搴︼紙浠ュ井绉掍负鍗曚綅锛夈€?

  funcgraph-abstime
	璁剧疆鍚庯紝姣忚閮戒細鏄剧ず鏃堕棿鎴炽€?

  funcgraph-irqs
	绂佺敤鍚庯紝鍙戠敓鍦ㄤ腑鏂唴閮ㄧ殑鍑芥暟涓嶄細琚窡韪€?

  funcgraph-tail
	璁剧疆鍚庯紝杩斿洖浜嬩欢浼氬寘鍚畠鎵€浠ｈ〃鐨勫嚱鏁般€傞粯璁ゆ儏鍐典笅鍏抽棴锛屽彧涓哄嚱鏁拌繑鍥炴樉绀轰竴涓?
	闂悎鑺辨嫭鍙?"}"銆?

  funcgraph-retval
	璁剧疆鍚庯紝姣忎釜琚窡韪嚱鏁扮殑杩斿洖鍊间細鎵撳嵃鍦ㄧ瓑鍙?"=" 涔嬪悗銆傞粯璁ゆ儏鍐典笅鍏抽棴銆?

  funcgraph-retval-hex
	璁剧疆鍚庯紝杩斿洖鍊煎皢濮嬬粓浠ュ崄鍏繘鍒舵牸寮忔墦鍗般€傚鏋滄湭璁剧疆璇ラ€夐」涓旇繑鍥炲€兼槸閿欒鐮侊紝鍒?
	浼氫互鏈夌鍙峰崄杩涘埗鏍煎紡鎵撳嵃锛涘惁鍒欎篃浼氫互鍗佸叚杩涘埗鏍煎紡鎵撳嵃銆傞粯璁ゆ儏鍐典笅璇ラ€夐」鍏抽棴銆?

  sleep-time
	杩愯鍑芥暟鍥捐窡韪櫒鏃讹紝灏嗕换鍔¤皟鍑猴紙schedule out锛夌殑鏃堕棿鍖呭惈杩涘叾鍑芥暟涓€傚惎鐢ㄦ椂锛?
	瀹冧細灏嗕换鍔¤璋冨嚭鐨勬椂闂磋鍏ュ嚱鏁拌皟鐢ㄧ殑涓€閮ㄥ垎銆?

  graph-time
	閰嶅悎鍑芥暟鍥捐窡韪櫒杩愯鍑芥暟鎬ц兘鍒嗘瀽鍣ㄦ椂锛屽皢璋冪敤宓屽鍑芥暟鐨勬椂闂村寘鍚湪鍐呫€傛湭璁剧疆
	鏃讹紝鎶ュ憡鐨勮鍑芥暟鏃堕棿鍙寘鍚鍑芥暟鑷韩鎵ц鐨勬椂闂达紝鑰屼笉鍖呭惈瀹冭皟鐢ㄧ殑鍑芥暟鐨勬椂闂淬€?

鍧楄澶囪窡韪櫒鐨勯€夐」锛?

  blk_classic
	鏄剧ず鏇寸簿绠€鐨勮緭鍑恒€?


### irqsoff


褰撲腑鏂绂佺敤鏃讹紝CPU 鏃犳硶瀵逛换浣曞叾浠栧閮ㄤ簨浠讹紙闄や簡 NMI 鍜?SMI锛夊仛鍑哄弽搴斻€傝繖浼氶樆姝?
瀹氭椂鍣ㄤ腑鏂Е鍙戯紝鎴栭樆姝㈤紶鏍囦腑鏂憡鐭ュ唴鏍告湁鏂扮殑榧犳爣浜嬩欢銆傜粨鏋滄槸鍙嶅簲鏃堕棿涓婄殑寤惰繜銆?

irqsoff 璺熻釜鍣ㄨ窡韪腑鏂绂佺敤鐨勬椂闂淬€傚綋杈惧埌涓€涓柊鐨勬渶澶у欢杩熸椂锛岃窡韪櫒浼氫繚瀛樺鑷磋
寤惰繜鐐圭殑璺熻釜锛岃繖鏍锋瘡褰撹揪鍒颁竴涓柊鐨勬渶澶у€硷紝鏃х殑宸蹭繚瀛樿窡韪氨浼氳涓㈠純锛屾柊鐨勮窡韪淇濆瓨銆?

瑕侀噸缃渶澶у€硷紝鍚?tracing_max_latency echo 0銆備互涓嬫槸

```

  # echo 0 > options/function-trace
  # echo irqsoff > current_tracer
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # ls -ltr
  [...]
  # echo 0 > tracing_on
  # cat trace
  # tracer: irqsoff
  #
  # irqsoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 16 us, #4/4, CPU#0 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: swapper/0-0 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: run_timer_softirq
  #  => ended at:   run_timer_softirq
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
    <idle>-0       0d.s2    0us+: _raw_spin_lock_irq <-run_timer_softirq
    <idle>-0       0dNs3   17us : _raw_spin_unlock_irq <-run_timer_softirq
    <idle>-0       0dNs3   17us+: trace_hardirqs_on <-run_timer_softirq
    <idle>-0       0dNs3   25us : <stack trace>
   => _raw_spin_unlock_irq
   => run_timer_softirq
   => __do_softirq
   => call_softirq
   => do_softirq
   => irq_exit
   => smp_apic_timer_interrupt
   => apic_timer_interrupt
   => rcu_idle_exit
   => cpu_idle
   => rest_init
   => start_kernel
   => x86_64_start_reservations
   => x86_64_start_kernel

```
杩欓噷鎴戜滑鐪嬪埌寤惰繜涓?16 寰锛堥潪甯稿ソ锛夈€俽un_timer_softirq 涓殑 _raw_spin_lock_irq 绂佺敤浜?
涓柇銆傛樉绀虹殑 16 涓庢樉绀虹殑鏃堕棿鎴?25us 涔嬮棿鐨勫樊寮傦紝鏄洜涓鸿褰曟渶澶у欢杩熺殑鏃堕棿涓庤褰曞叿鏈?
璇ュ欢杩熺殑鍑芥暟鐨勬椂闂翠箣闂达紝鏃堕挓琚鍔犱簡銆?

娉ㄦ剰涓婇潰鐨勭ず渚嬫湭璁剧疆 function-trace銆傚鏋滄垜浠缃?

```

 with echo 1 > options/function-trace

  # tracer: irqsoff
  #
  # irqsoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 71 us, #168/168, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: bash-2042 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: ata_scsi_queuecmd
  #  => ended at:   ata_scsi_queuecmd
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
      bash-2042    3d...    0us : _raw_spin_lock_irqsave <-ata_scsi_queuecmd
      bash-2042    3d...    0us : add_preempt_count <-_raw_spin_lock_irqsave
      bash-2042    3d..1    1us : ata_scsi_find_dev <-ata_scsi_queuecmd
      bash-2042    3d..1    1us : __ata_scsi_find_dev <-ata_scsi_find_dev
      bash-2042    3d..1    2us : ata_find_dev.part.14 <-__ata_scsi_find_dev
      bash-2042    3d..1    2us : ata_qc_new_init <-__ata_scsi_queuecmd
      bash-2042    3d..1    3us : ata_sg_init <-__ata_scsi_queuecmd
      bash-2042    3d..1    4us : ata_scsi_rw_xlat <-__ata_scsi_queuecmd
      bash-2042    3d..1    4us : ata_build_rw_tf <-ata_scsi_rw_xlat
  [...]
      bash-2042    3d..1   67us : delay_tsc <-__delay
      bash-2042    3d..1   67us : add_preempt_count <-delay_tsc
      bash-2042    3d..2   67us : sub_preempt_count <-delay_tsc
      bash-2042    3d..1   67us : add_preempt_count <-delay_tsc
      bash-2042    3d..2   68us : sub_preempt_count <-delay_tsc
      bash-2042    3d..1   68us+: ata_bmdma_start <-ata_bmdma_qc_issue
      bash-2042    3d..1   71us : _raw_spin_unlock_irqrestore <-ata_scsi_queuecmd
      bash-2042    3d..1   71us : _raw_spin_unlock_irqrestore <-ata_scsi_queuecmd
      bash-2042    3d..1   72us+: trace_hardirqs_on <-ata_scsi_queuecmd
      bash-2042    3d..1  120us : <stack trace>
   => _raw_spin_unlock_irqrestore
   => ata_scsi_queuecmd
   => scsi_dispatch_cmd
   => scsi_request_fn
   => __blk_run_queue_uncond
   => __blk_run_queue
   => blk_queue_bio
   => submit_bio_noacct
   => submit_bio
   => submit_bh
   => __ext3_get_inode_loc
   => ext3_iget
   => ext3_lookup
   => lookup_real
   => __lookup_hash
   => walk_component
   => lookup_last
   => path_lookupat
   => filename_lookup
   => user_path_at_empty
   => user_path_at
   => vfs_fstatat
   => vfs_stat
   => sys_newstat
   => system_call_fastpath


```
杩欓噷鎴戜滑璺熻釜浜嗕竴涓?71 寰鐨勫欢杩熴€備絾鎴戜滑涔熺湅鍒颁簡鍦ㄦ鏈熼棿琚皟鐢ㄧ殑鎵€鏈夊嚱鏁般€傛敞鎰忥紝閫氳繃
鍚敤鍑芥暟璺熻釜锛屾垜浠甫鏉ヤ簡棰濆鐨勫紑閿€銆傝繖涓紑閿€鍙兘浼氬欢闀垮欢杩熸椂闂淬€備絾灏界濡傛锛屾璺熻釜
鎻愪緵浜嗕竴浜涢潪甯告湁甯姪鐨勮皟璇曚俊鎭€?

濡傛灉鎴戜滑鍋忓ソ鍑芥暟鍥捐緭鍑鸿€岄潪鍑芥暟杈撳嚭锛屽彲浠ヨ缃?

```

 with echo 1 > options/display-graph

  # tracer: irqsoff
  #
  # irqsoff latency trace v1.1.5 on 4.20.0-rc6+
  # --------------------------------------------------------------------
  # latency: 3751 us, #274/274, CPU#0 | (M:desktop VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: bash-1507 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: free_debug_processing
  #  => ended at:   return_to_handler
  #
  #
  #                                       _-----=> irqs-off
  #                                      / _----=> need-resched
  #                                     | / _---=> hardirq/softirq
  #                                     || / _--=> preempt-depth
  #                                     ||| /
  #   REL TIME      CPU  TASK/PID       ||||     DURATION                  FUNCTION CALLS
  #      |          |     |    |        ||||      |   |                     |   |   |   |
          0 us |   0)   bash-1507    |  d... |   0.000 us    |  _raw_spin_lock_irqsave();
          0 us |   0)   bash-1507    |  d..1 |   0.378 us    |    do_raw_spin_trylock();
          1 us |   0)   bash-1507    |  d..2 |               |    set_track() {
          2 us |   0)   bash-1507    |  d..2 |               |      save_stack_trace() {
          2 us |   0)   bash-1507    |  d..2 |               |        __save_stack_trace() {
          3 us |   0)   bash-1507    |  d..2 |               |          __unwind_start() {
          3 us |   0)   bash-1507    |  d..2 |               |            get_stack_info() {
          3 us |   0)   bash-1507    |  d..2 |   0.351 us    |              in_task_stack();
          4 us |   0)   bash-1507    |  d..2 |   1.107 us    |            }
  [...]
       3750 us |   0)   bash-1507    |  d..1 |   0.516 us    |      do_raw_spin_unlock();
       3750 us |   0)   bash-1507    |  d..1 |   0.000 us    |  _raw_spin_unlock_irqrestore();
       3764 us |   0)   bash-1507    |  d..1 |   0.000 us    |  tracer_hardirqs_on();
      bash-1507    0d..1 3792us : <stack trace>
   => free_debug_processing
   => __slab_free
   => kmem_cache_free
   => vm_area_free
   => remove_vma
   => exit_mmap
   => mmput
   => begin_new_exec
   => load_elf_binary
   => search_binary_handler
   => __do_execve_file.isra.32
   => __x64_sys_execve
   => do_syscall_64
   => entry_SYSCALL_64_after_hwframe


```
### preemptoff


褰撴姠鍗犺绂佺敤鏃讹紝鎴戜滑鍙兘鑳芥帴鏀朵腑鏂紝浣嗕换鍔℃棤娉曡鎶㈠崰锛屾洿楂樹紭鍏堢骇鐨勪换鍔″繀椤荤瓑寰呮姠鍗?
琚噸鏂板惎鐢ㄥ悗鎵嶈兘鎶㈠崰杈冧綆浼樺厛绾х殑浠诲姟銆?

preemptoff 璺熻釜鍣ㄨ窡韪鐢ㄦ姠鍗犵殑浣嶇疆銆備笌 irqsoff 璺熻釜鍣ㄧ被浼硷紝瀹冭褰曟姠鍗犺绂佺敤鐨勬渶澶?
寤惰繜銆俻reemptoff 璺熻釜鍣ㄧ殑鎺у埗鏂瑰紡涓?irqsoff 璺熻釜鍣ㄩ潪甯哥浉浼笺€?

```

  # echo 0 > options/function-trace
  # echo preemptoff > current_tracer
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # ls -ltr
  [...]
  # echo 0 > tracing_on
  # cat trace
  # tracer: preemptoff
  #
  # preemptoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 46 us, #4/4, CPU#1 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: sshd-1991 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: do_IRQ
  #  => ended at:   do_IRQ
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
      sshd-1991    1d.h.    0us+: irq_enter <-do_IRQ
      sshd-1991    1d..1   46us : irq_exit <-do_IRQ
      sshd-1991    1d..1   47us+: trace_preempt_on <-do_IRQ
      sshd-1991    1d..1   52us : <stack trace>
   => sub_preempt_count
   => irq_exit
   => do_IRQ
   => ret_from_intr


```
杩欐湁涓€浜涙洿澶氱殑鍙樺寲銆傚綋杩涘叆涓柇鏃讹紙娉ㄦ剰 'h'锛夛紝鎶㈠崰琚鐢紝骞跺湪閫€鍑烘椂鍚敤銆備絾鎴戜滑涔?
鐪嬪埌鍦ㄨ繘鍏ユ姠鍗犵鐢ㄦ鍜岀寮€瀹冩椂涓柇宸茶绂佺敤锛?d'锛夈€傛垜浠笉鐭ラ亾鍦ㄦ鏈熼棿鎴栨鍚庝笉涔呬腑鏂?
鏄惁琚噸鏂板惎鐢ㄣ€?

```

  # tracer: preemptoff
  #
  # preemptoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 83 us, #241/241, CPU#1 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: bash-1994 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: wake_up_new_task
  #  => ended at:   task_rq_unlock
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
      bash-1994    1d..1    0us : _raw_spin_lock_irqsave <-wake_up_new_task
      bash-1994    1d..1    0us : select_task_rq_fair <-select_task_rq
      bash-1994    1d..1    1us : __rcu_read_lock <-select_task_rq_fair
      bash-1994    1d..1    1us : source_load <-select_task_rq_fair
      bash-1994    1d..1    1us : source_load <-select_task_rq_fair
  [...]
      bash-1994    1d..1   12us : irq_enter <-smp_apic_timer_interrupt
      bash-1994    1d..1   12us : rcu_irq_enter <-irq_enter
      bash-1994    1d..1   13us : add_preempt_count <-irq_enter
      bash-1994    1d.h1   13us : exit_idle <-smp_apic_timer_interrupt
      bash-1994    1d.h1   13us : hrtimer_interrupt <-smp_apic_timer_interrupt
      bash-1994    1d.h1   14us : _raw_spin_lock <-hrtimer_interrupt
      bash-1994    1d.h1   14us : add_preempt_count <-_raw_spin_lock
      bash-1994    1d.h2   14us : ktime_get_update_offsets <-hrtimer_interrupt
  [...]
      bash-1994    1d.h1   35us : lapic_next_event <-clockevents_program_event
      bash-1994    1d.h1   35us : irq_exit <-smp_apic_timer_interrupt
      bash-1994    1d.h1   36us : sub_preempt_count <-irq_exit
      bash-1994    1d..2   36us : do_softirq <-irq_exit
      bash-1994    1d..2   36us : __do_softirq <-call_softirq
      bash-1994    1d..2   36us : __local_bh_disable <-__do_softirq
      bash-1994    1d.s2   37us : add_preempt_count <-_raw_spin_lock_irq
      bash-1994    1d.s3   38us : _raw_spin_unlock <-run_timer_softirq
      bash-1994    1d.s3   39us : sub_preempt_count <-_raw_spin_unlock
      bash-1994    1d.s2   39us : call_timer_fn <-run_timer_softirq
  [...]
      bash-1994    1dNs2   81us : cpu_needs_another_gp <-rcu_process_callbacks
      bash-1994    1dNs2   82us : __local_bh_enable <-__do_softirq
      bash-1994    1dNs2   82us : sub_preempt_count <-__local_bh_enable
      bash-1994    1dN.2   82us : idle_cpu <-irq_exit
      bash-1994    1dN.2   83us : rcu_irq_exit <-irq_exit
      bash-1994    1dN.2   83us : sub_preempt_count <-irq_exit
      bash-1994    1.N.1   84us : _raw_spin_unlock_irqrestore <-task_rq_unlock
      bash-1994    1.N.1   84us+: trace_preempt_on <-task_rq_unlock
      bash-1994    1.N.1  104us : <stack trace>
   => sub_preempt_count
   => _raw_spin_unlock_irqrestore
   => task_rq_unlock
   => wake_up_new_task
   => do_fork
   => sys_clone
   => stub_clone


```
涓婇潰鏄缃簡 function-trace 鐨?preemptoff 璺熻釜绀轰緥銆傝繖閲屾垜浠湅鍒颁腑鏂苟闈炲湪鏁翠釜鏈熼棿閮?
琚鐢ㄣ€俰rq_enter 浠ｇ爜璁╂垜浠煡閬撴垜浠繘鍏ヤ簡涓€涓腑鏂?'h'銆傚湪姝や箣鍓嶏紝琚窡韪殑鍑芥暟浠嶇劧
鏄剧ず瀹冧笉鍦ㄤ腑鏂腑锛屼絾鎴戜滑浠庡嚱鏁版湰韬彲浠ョ湅鍑烘儏鍐靛苟闈炲姝ゃ€?

### preemptirqsoff


浜嗚В涓柇琚鐢ㄦ垨鎶㈠崰琚鐢ㄦ椂闂存渶闀跨殑浣嶇疆寰堟湁甯姪銆備絾鏈夋椂鎴戜滑鎯崇煡閬撴姠鍗犲拰/鎴栦腑鏂?
浣曟椂琚鐢ㄣ€?

```

    local_irq_disable();
    call_function_with_irqs_off();
    preempt_disable();
    call_function_with_irqs_and_preemption_off();
    local_irq_enable();
    call_function_with_preemption_off();
    preempt_enable();

```
irqsoff 璺熻釜鍣ㄤ細璁板綍 call_function_with_irqs_off() 鍜?
call_function_with_irqs_and_preemption_off() 鐨勬€婚暱搴︺€?

preemptoff 璺熻釜鍣ㄤ細璁板綍 call_function_with_irqs_and_preemption_off() 鍜?
call_function_with_preemption_off() 鐨勬€婚暱搴︺€?

浣嗕簩鑰呴兘涓嶄細璺熻釜涓柇鍜?鎴栨姠鍗犺绂佺敤鐨勬椂闂淬€傝繖涓€绘椂闂存槸鎴戜滑鏃犳硶璋冨害鐨勬椂闀裤€傝璁板綍
姝ゆ椂闀匡紝璇蜂娇鐢?preemptirqsoff 璺熻釜鍣ㄣ€?

鍚屾牱锛屼娇鐢ㄦ璺熻釜涓?irqsoff 鍜?preemptoff 璺熻釜鍣ㄩ潪甯哥浉浼笺€?

```

  # echo 0 > options/function-trace
  # echo preemptirqsoff > current_tracer
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # ls -ltr
  [...]
  # echo 0 > tracing_on
  # cat trace
  # tracer: preemptirqsoff
  #
  # preemptirqsoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 100 us, #4/4, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: ls-2230 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: ata_scsi_queuecmd
  #  => ended at:   ata_scsi_queuecmd
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
        ls-2230    3d...    0us+: _raw_spin_lock_irqsave <-ata_scsi_queuecmd
        ls-2230    3...1  100us : _raw_spin_unlock_irqrestore <-ata_scsi_queuecmd
        ls-2230    3...1  101us+: trace_preempt_on <-ata_scsi_queuecmd
        ls-2230    3...1  111us : <stack trace>
   => sub_preempt_count
   => _raw_spin_unlock_irqrestore
   => ata_scsi_queuecmd
   => scsi_dispatch_cmd
   => scsi_request_fn
   => __blk_run_queue_uncond
   => __blk_run_queue
   => blk_queue_bio
   => submit_bio_noacct
   => submit_bio
   => submit_bh
   => ext3_bread
   => ext3_dir_bread
   => htree_dirblock_to_tree
   => ext3_htree_fill_tree
   => ext3_readdir
   => vfs_readdir
   => sys_getdents
   => system_call_fastpath


```
trace_hardirqs_off_thunk 鍦?x86 涓婁粠涓柇琚鐢ㄦ椂鐢辨眹缂栦唬鐮佽皟鐢ㄣ€傛病鏈夊嚱鏁拌窡韪紝鎴戜滑
鏃犳硶鐭ラ亾鍦ㄦ姠鍗犵偣鍐呴儴涓柇鏄惁琚惎鐢ㄣ€傛垜浠‘瀹炵湅鍒板畠浠庡惎鐢ㄦ姠鍗犲紑濮嬨€?

```

  # tracer: preemptirqsoff
  #
  # preemptirqsoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 161 us, #339/339, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: ls-2269 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: schedule
  #  => ended at:   mutex_unlock
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
  kworker/-59      3...1    0us : __schedule <-schedule
  kworker/-59      3d..1    0us : rcu_preempt_qs <-rcu_note_context_switch
  kworker/-59      3d..1    1us : add_preempt_count <-_raw_spin_lock_irq
  kworker/-59      3d..2    1us : deactivate_task <-__schedule
  kworker/-59      3d..2    1us : dequeue_task <-deactivate_task
  kworker/-59      3d..2    2us : update_rq_clock <-dequeue_task
  kworker/-59      3d..2    2us : dequeue_task_fair <-dequeue_task
  kworker/-59      3d..2    2us : update_curr <-dequeue_task_fair
  kworker/-59      3d..2    2us : update_min_vruntime <-update_curr
  kworker/-59      3d..2    3us : cpuacct_charge <-update_curr
  kworker/-59      3d..2    3us : __rcu_read_lock <-cpuacct_charge
  kworker/-59      3d..2    3us : __rcu_read_unlock <-cpuacct_charge
  kworker/-59      3d..2    3us : update_cfs_rq_blocked_load <-dequeue_task_fair
  kworker/-59      3d..2    4us : clear_buddies <-dequeue_task_fair
  kworker/-59      3d..2    4us : account_entity_dequeue <-dequeue_task_fair
  kworker/-59      3d..2    4us : update_min_vruntime <-dequeue_task_fair
  kworker/-59      3d..2    4us : update_cfs_shares <-dequeue_task_fair
  kworker/-59      3d..2    5us : hrtick_update <-dequeue_task_fair
  kworker/-59      3d..2    5us : wq_worker_sleeping <-__schedule
  kworker/-59      3d..2    5us : kthread_data <-wq_worker_sleeping
  kworker/-59      3d..2    5us : put_prev_task_fair <-__schedule
  kworker/-59      3d..2    6us : pick_next_task_fair <-pick_next_task
  kworker/-59      3d..2    6us : clear_buddies <-pick_next_task_fair
  kworker/-59      3d..2    6us : set_next_entity <-pick_next_task_fair
  kworker/-59      3d..2    6us : update_stats_wait_end <-set_next_entity
        ls-2269    3d..2    7us : finish_task_switch <-__schedule
        ls-2269    3d..2    7us : _raw_spin_unlock_irq <-finish_task_switch
        ls-2269    3d..2    8us : do_IRQ <-ret_from_intr
        ls-2269    3d..2    8us : irq_enter <-do_IRQ
        ls-2269    3d..2    8us : rcu_irq_enter <-irq_enter
        ls-2269    3d..2    9us : add_preempt_count <-irq_enter
        ls-2269    3d.h2    9us : exit_idle <-do_IRQ
  [...]
        ls-2269    3d.h3   20us : sub_preempt_count <-_raw_spin_unlock
        ls-2269    3d.h2   20us : irq_exit <-do_IRQ
        ls-2269    3d.h2   21us : sub_preempt_count <-irq_exit
        ls-2269    3d..3   21us : do_softirq <-irq_exit
        ls-2269    3d..3   21us : __do_softirq <-call_softirq
        ls-2269    3d..3   21us+: __local_bh_disable <-__do_softirq
        ls-2269    3d.s4   29us : sub_preempt_count <-_local_bh_enable_ip
        ls-2269    3d.s5   29us : sub_preempt_count <-_local_bh_enable_ip
        ls-2269    3d.s5   31us : do_IRQ <-ret_from_intr
        ls-2269    3d.s5   31us : irq_enter <-do_IRQ
        ls-2269    3d.s5   31us : rcu_irq_enter <-irq_enter
  [...]
        ls-2269    3d.s5   31us : rcu_irq_enter <-irq_enter
        ls-2269    3d.s5   32us : add_preempt_count <-irq_enter
        ls-2269    3d.H5   32us : exit_idle <-do_IRQ
        ls-2269    3d.H5   32us : handle_irq <-do_IRQ
        ls-2269    3d.H5   32us : irq_to_desc <-handle_irq
        ls-2269    3d.H5   33us : handle_fasteoi_irq <-handle_irq
  [...]
        ls-2269    3d.s5  158us : _raw_spin_unlock_irqrestore <-rtl8139_poll
        ls-2269    3d.s3  158us : net_rps_action_and_irq_enable.isra.65 <-net_rx_action
        ls-2269    3d.s3  159us : __local_bh_enable <-__do_softirq
        ls-2269    3d.s3  159us : sub_preempt_count <-__local_bh_enable
        ls-2269    3d..3  159us : idle_cpu <-irq_exit
        ls-2269    3d..3  159us : rcu_irq_exit <-irq_exit
        ls-2269    3d..3  160us : sub_preempt_count <-irq_exit
        ls-2269    3d...  161us : __mutex_unlock_slowpath <-mutex_unlock
        ls-2269    3d...  162us+: trace_hardirqs_on <-mutex_unlock
        ls-2269    3d...  186us : <stack trace>
   => __mutex_unlock_slowpath
   => mutex_unlock
   => process_output
   => n_tty_write
   => tty_write
   => vfs_write
   => sys_write
   => system_call_fastpath


```
杩欐槸涓€娆℃湁瓒ｇ殑璺熻釜銆傚畠浠?kworker 杩愯骞惰皟搴﹀嚭鍘汇€乴s 鎺ョ寮€濮嬨€備絾涓€鏃?ls 閲婃斁浜?rq 閿?
骞跺惎鐢ㄤ簡涓柇锛堜絾鏈惎鐢ㄦ姠鍗狅級锛屼竴涓腑鏂氨琚Е鍙戜簡銆傚綋涓柇缁撴潫鏃讹紝瀹冨紑濮嬭繍琛岃蒋涓柇銆?
浣嗗湪杞腑鏂繍琛屾湡闂达紝鍙︿竴涓腑鏂瑙﹀彂浜嗐€傚綋涓柇鍦ㄨ蒋涓柇鍐呴儴杩愯鏃讹紝鏍囪涓?'H'銆?


### wakeup


浜轰滑鎰熷叴瓒ｇ殑涓€绉嶅父瑙佹儏鍐垫槸锛屼竴涓鍞ら啋鐨勪换鍔＄湡姝ｈ鍞ら啋鎵€鑺辫垂鐨勬椂闂淬€傚浜庨潪瀹炴椂浠诲姟锛?
杩欏彲鑳芥槸浠绘剰鐨勩€備絾鏃犺濡備綍璺熻釜瀹冮兘寰堟湁瓒ｃ€?

```

  # echo 0 > options/function-trace
  # echo wakeup > current_tracer
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # chrt -f 5 sleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: wakeup
  #
  # wakeup latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 15 us, #4/4, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: kworker/3:1H-312 (uid:0 nice:-20 policy:0 rt_prio:0)
  #    -----------------
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
    <idle>-0       3dNs7    0us :      0:120:R   + [003]   312:100:R kworker/3:1H
    <idle>-0       3dNs7    1us+: ttwu_do_activate.constprop.87 <-try_to_wake_up
    <idle>-0       3d..3   15us : __schedule <-schedule
    <idle>-0       3d..3   15us :      0:120:R ==> [003]   312:100:R kworker/3:1H


```
璺熻釜鍣ㄥ彧璺熻釜绯荤粺涓渶楂樹紭鍏堢骇鐨勪换鍔★紝浠ラ伩鍏嶈窡韪甯告儏鍐点€傝繖閲屾垜浠湅鍒?nice 浼樺厛绾т负
-20锛堥潪甯镐笉鍙嬪ソ锛夌殑 kworker锛屼粠瀹冭鍞ら啋鍒板畠杩愯锛屽彧鑺变簡 15 寰銆?

闈炲疄鏃朵换鍔℃病閭ｄ箞鏈夎叮銆傛洿鏈夎叮鐨勮窡韪槸鍙叧娉ㄥ疄鏃朵换鍔°€?

### wakeup_rt


鍦ㄥ疄鏃剁幆澧冧腑锛屼簡瑙ｈ鍞ら啋鐨勬渶楂樹紭鍏堢骇浠诲姟浠庤鍞ら啋鍒板畠鎵ц鎵€鑺辫垂鐨勫敜閱掓椂闂撮潪甯?
閲嶈銆傝繖涔熻绉颁负"璋冨害寤惰繜"銆傛垜寮鸿皟涓€鐐癸紝杩欐槸鍏充簬 RT 浠诲姟鐨勩€備簡瑙ｉ潪 RT 浠诲姟鐨勮皟搴?
寤惰繜涔熷緢閲嶈锛屼絾瀵逛簬闈?RT 浠诲姟锛屽钩鍧囪皟搴﹀欢杩熸洿鍚堥€傘€傚儚 LatencyTop 杩欐牱鐨勫伐鍏锋洿閫傚悎
姝ょ被娴嬮噺銆?

瀹炴椂鐜鍏虫敞鏈€鍧忔儏鍐靛欢杩熴€備篃灏辨槸鏌愪欢浜嬪彂鐢熸墍闇€鐨勬渶闀挎椂闂达紝鑰屼笉鏄钩鍧囨椂闂淬€傛垜浠彲浠?
鏈変竴涓潪甯稿揩鐨勮皟搴﹀櫒锛屽畠鍙兘鍙槸鍋跺皵鎵嶅嚭鐜颁竴娆″ぇ寤惰繜锛屼絾杩欏瀹炴椂浠诲姟鏉ヨ骞朵笉鍚堥€傘€?
wakeup_rt 璺熻釜鍣ㄥ氨鏄负璁板綍 RT 浠诲姟鐨勬渶鍧忔儏鍐靛敜閱掕€岃璁＄殑銆傞潪 RT 浠诲姟涓嶄細琚褰曪紝鍥犱负
璇ヨ窡韪櫒鍙褰曚竴涓渶鍧忔儏鍐碉紝璺熻釜涓嶅彲棰勬祴鐨勯潪 RT 浠诲姟浼氳鐩栨帀 RT 浠诲姟鐨勬渶鍧忔儏鍐靛欢杩?
锛堝彧闇€杩愯涓€娈垫椂闂寸殑鏅€?wakeup 璺熻釜鍣ㄥ氨鑳界湅鍒拌繖绉嶆晥鏋滐級銆?

鐢变簬璇ヨ窡韪櫒鍙鐞?RT 浠诲姟锛屾垜浠皢浠ヤ笌涔嬪墠璺熻釜鍣ㄧ暐鏈変笉鍚岀殑鏂瑰紡杩愯瀹冦€備笉鍐嶆墽琛?'ls'锛?
鑰屾槸鍦?'chrt' 涓嬭繍琛?'sleep 1'锛岃繖浼氭洿鏀逛换鍔＄殑浼樺厛绾с€?

```

  # echo 0 > options/function-trace
  # echo wakeup_rt > current_tracer
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # chrt -f 5 sleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: wakeup
  #
  # tracer: wakeup_rt
  #
  # wakeup_rt latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 5 us, #4/4, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: sleep-2389 (uid:0 nice:0 policy:1 rt_prio:5)
  #    -----------------
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
    <idle>-0       3d.h4    0us :      0:120:R   + [003]  2389: 94:R sleep
    <idle>-0       3d.h4    1us+: ttwu_do_activate.constprop.87 <-try_to_wake_up
    <idle>-0       3d..3    5us : __schedule <-schedule
    <idle>-0       3d..3    5us :      0:120:R ==> [003]  2389: 94:R sleep


```
鍦ㄤ竴涓┖闂茬郴缁熶笂杩愯锛屾垜浠湅鍒版墽琛屼换鍔″垏鎹㈠彧鑺变簡 5 寰銆傛敞鎰忥紝鐢变簬 schedule 涓殑
璺熻釜鐐逛綅浜庡疄闄?鍒囨崲"涔嬪墠锛屾垜浠湪琚褰曠殑浠诲姟鍗冲皢璋冨害杩涙潵鏃跺仠姝㈣窡韪€傚鏋滄垜浠湪涓€涓?
璋冨害鍣ㄦ湯灏炬坊鍔犱竴涓柊鐨勬爣璁帮紝杩欏彲鑳戒細鏀瑰彉銆?

娉ㄦ剰璁板綍鐨勪换鍔?'sleep' 鐨?PID 涓?2389锛屽畠鐨?rt_prio 涓?5銆傝浼樺厛绾ф槸鐢ㄦ埛绌洪棿浼樺厛绾э紝
鑰屼笉鏄唴鏍稿唴閮ㄤ紭鍏堢骇銆俻olicy 涓?1 琛ㄧず SCHED_FIFO锛? 琛ㄧず SCHED_RR銆?

娉ㄦ剰锛岃窡韪暟鎹樉绀虹殑鏄唴閮ㄤ紭鍏堢骇锛?9 - rtprio锛夈€?

```

  <idle>-0       3d..3    5us :      0:120:R ==> [003]  2389: 94:R sleep

```
0:120:R 琛ㄧず idle 浠?nice 浼樺厛绾?0锛?20 - 120锛夎繍琛岋紝骞跺浜庤繍琛屾€?'R'銆俿leep 浠诲姟浠?
2389: 94:R 琚皟搴﹁繘鏉ャ€備篃灏辨槸璇翠紭鍏堢骇鏄唴鏍?rtprio锛?9 - 5 = 94锛夛紝瀹冧篃澶勪簬杩愯鎬併€?

鐢?chrt -r 5 骞惰缃?function-trace 鍋氬悓鏍风殑浜嬨€?

```

  echo 1 > options/function-trace

  # tracer: wakeup_rt
  #
  # wakeup_rt latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 29 us, #85/85, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: sleep-2448 (uid:0 nice:0 policy:1 rt_prio:5)
  #    -----------------
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
    <idle>-0       3d.h4    1us+:      0:120:R   + [003]  2448: 94:R sleep
    <idle>-0       3d.h4    2us : ttwu_do_activate.constprop.87 <-try_to_wake_up
    <idle>-0       3d.h3    3us : check_preempt_curr <-ttwu_do_wakeup
    <idle>-0       3d.h3    3us : resched_curr <-check_preempt_curr
    <idle>-0       3dNh3    4us : task_woken_rt <-ttwu_do_wakeup
    <idle>-0       3dNh3    4us : _raw_spin_unlock <-try_to_wake_up
    <idle>-0       3dNh3    4us : sub_preempt_count <-_raw_spin_unlock
    <idle>-0       3dNh2    5us : ttwu_stat <-try_to_wake_up
    <idle>-0       3dNh2    5us : _raw_spin_unlock_irqrestore <-try_to_wake_up
    <idle>-0       3dNh2    6us : sub_preempt_count <-_raw_spin_unlock_irqrestore
    <idle>-0       3dNh1    6us : _raw_spin_lock <-__run_hrtimer
    <idle>-0       3dNh1    6us : add_preempt_count <-_raw_spin_lock
    <idle>-0       3dNh2    7us : _raw_spin_unlock <-hrtimer_interrupt
    <idle>-0       3dNh2    7us : sub_preempt_count <-_raw_spin_unlock
    <idle>-0       3dNh1    7us : tick_program_event <-hrtimer_interrupt
    <idle>-0       3dNh1    7us : clockevents_program_event <-tick_program_event
    <idle>-0       3dNh1    8us : ktime_get <-clockevents_program_event
    <idle>-0       3dNh1    8us : lapic_next_event <-clockevents_program_event
    <idle>-0       3dNh1    9us : irq_exit <-smp_apic_timer_interrupt
    <idle>-0       3dNh1    9us : sub_preempt_count <-irq_exit
    <idle>-0       3dN.2    9us : idle_cpu <-irq_exit
    <idle>-0       3dN.2    9us : rcu_irq_exit <-irq_exit
    <idle>-0       3dN.2   10us : rcu_eqs_enter_common.isra.45 <-rcu_irq_exit
    <idle>-0       3dN.2   10us : sub_preempt_count <-irq_exit
    <idle>-0       3.N.1   11us : rcu_idle_exit <-cpu_idle
    <idle>-0       3dN.1   11us : rcu_eqs_exit_common.isra.43 <-rcu_idle_exit
    <idle>-0       3.N.1   11us : tick_nohz_idle_exit <-cpu_idle
    <idle>-0       3dN.1   12us : menu_hrtimer_cancel <-tick_nohz_idle_exit
    <idle>-0       3dN.1   12us : ktime_get <-tick_nohz_idle_exit
    <idle>-0       3dN.1   12us : tick_do_update_jiffies64 <-tick_nohz_idle_exit
    <idle>-0       3dN.1   13us : cpu_load_update_nohz <-tick_nohz_idle_exit
    <idle>-0       3dN.1   13us : _raw_spin_lock <-cpu_load_update_nohz
    <idle>-0       3dN.1   13us : add_preempt_count <-_raw_spin_lock
    <idle>-0       3dN.2   13us : __cpu_load_update <-cpu_load_update_nohz
    <idle>-0       3dN.2   14us : sched_avg_update <-__cpu_load_update
    <idle>-0       3dN.2   14us : _raw_spin_unlock <-cpu_load_update_nohz
    <idle>-0       3dN.2   14us : sub_preempt_count <-_raw_spin_unlock
    <idle>-0       3dN.1   15us : calc_load_nohz_stop <-tick_nohz_idle_exit
    <idle>-0       3dN.1   15us : touch_softlockup_watchdog <-tick_nohz_idle_exit
    <idle>-0       3dN.1   15us : hrtimer_cancel <-tick_nohz_idle_exit
    <idle>-0       3dN.1   15us : hrtimer_try_to_cancel <-hrtimer_cancel
    <idle>-0       3dN.1   16us : lock_hrtimer_base.isra.18 <-hrtimer_try_to_cancel
    <idle>-0       3dN.1   16us : _raw_spin_lock_irqsave <-lock_hrtimer_base.isra.18
    <idle>-0       3dN.1   16us : add_preempt_count <-_raw_spin_lock_irqsave
    <idle>-0       3dN.2   17us : __remove_hrtimer <-remove_hrtimer.part.16
    <idle>-0       3dN.2   17us : hrtimer_force_reprogram <-__remove_hrtimer
    <idle>-0       3dN.2   17us : tick_program_event <-hrtimer_force_reprogram
    <idle>-0       3dN.2   18us : clockevents_program_event <-tick_program_event
    <idle>-0       3dN.2   18us : ktime_get <-clockevents_program_event
    <idle>-0       3dN.2   18us : lapic_next_event <-clockevents_program_event
    <idle>-0       3dN.2   19us : _raw_spin_unlock_irqrestore <-hrtimer_try_to_cancel
    <idle>-0       3dN.2   19us : sub_preempt_count <-_raw_spin_unlock_irqrestore
    <idle>-0       3dN.1   19us : hrtimer_forward <-tick_nohz_idle_exit
    <idle>-0       3dN.1   20us : ktime_add_safe <-hrtimer_forward
    <idle>-0       3dN.1   20us : ktime_add_safe <-hrtimer_forward
    <idle>-0       3dN.1   20us : hrtimer_start_range_ns <-hrtimer_start_expires.constprop.11
    <idle>-0       3dN.1   21us : __hrtimer_start_range_ns <-hrtimer_start_range_ns
    <idle>-0       3dN.1   21us : lock_hrtimer_base.isra.18 <-__hrtimer_start_range_ns
    <idle>-0       3dN.1   21us : _raw_spin_lock_irqsave <-lock_hrtimer_base.isra.18
    <idle>-0       3dN.1   21us : add_preempt_count <-_raw_spin_lock_irqsave
    <idle>-0       3dN.2   22us : ktime_add_safe <-__hrtimer_start_range_ns
    <idle>-0       3dN.2   22us : enqueue_hrtimer <-__hrtimer_start_range_ns
    <idle>-0       3dN.2   22us : tick_program_event <-__hrtimer_start_range_ns
    <idle>-0       3dN.2   23us : clockevents_program_event <-tick_program_event
    <idle>-0       3dN.2   23us : ktime_get <-clockevents_program_event
    <idle>-0       3dN.2   23us : lapic_next_event <-clockevents_program_event
    <idle>-0       3dN.2   24us : _raw_spin_unlock_irqrestore <-__hrtimer_start_range_ns
    <idle>-0       3dN.2   24us : sub_preempt_count <-_raw_spin_unlock_irqrestore
    <idle>-0       3dN.1   24us : account_idle_ticks <-tick_nohz_idle_exit
    <idle>-0       3dN.1   24us : account_idle_time <-account_idle_ticks
    <idle>-0       3.N.1   25us : sub_preempt_count <-cpu_idle
    <idle>-0       3.N..   25us : schedule <-cpu_idle
    <idle>-0       3.N..   25us : __schedule <-preempt_schedule
    <idle>-0       3.N..   26us : add_preempt_count <-__schedule
    <idle>-0       3.N.1   26us : rcu_note_context_switch <-__schedule
    <idle>-0       3.N.1   26us : rcu_sched_qs <-rcu_note_context_switch
    <idle>-0       3dN.1   27us : rcu_preempt_qs <-rcu_note_context_switch
    <idle>-0       3.N.1   27us : _raw_spin_lock_irq <-__schedule
    <idle>-0       3dN.1   27us : add_preempt_count <-_raw_spin_lock_irq
    <idle>-0       3dN.2   28us : put_prev_task_idle <-__schedule
    <idle>-0       3dN.2   28us : pick_next_task_stop <-pick_next_task
    <idle>-0       3dN.2   28us : pick_next_task_rt <-pick_next_task
    <idle>-0       3dN.2   29us : dequeue_pushable_task <-pick_next_task_rt
    <idle>-0       3d..3   29us : __schedule <-preempt_schedule
    <idle>-0       3d..3   30us :      0:120:R ==> [003]  2448: 94:R sleep


```
鍗充究鍚敤浜嗗嚱鏁拌窡韪紝杩欎篃涓嶆槸寰堝ぇ鐨勮窡韪紝鎵€浠ユ垜鎶婃暣涓窡韪兘鍖呭惈浜嗚繘鏉ャ€?

涓柇鍦ㄧ郴缁熺┖闂叉椂瑙﹀彂銆傚湪 task_woken_rt() 琚皟鐢ㄤ箣鍓嶇殑鏌愬锛孨EED_RESCHED 鏍囧織琚缃紝
杩欑敱绗竴娆″嚭鐜?'N' 鏍囧織鎸囩ず銆?

### 寤惰繜璺熻釜涓庝簨浠?


鐢变簬鍑芥暟璺熻釜浼氬甫鏉ュぇ寰楀鐨勫欢杩燂紝浣嗗鏋滀笉鐪嬪埌寤惰繜鏈熼棿鍙戠敓浜嗕粈涔堬紝灏卞緢闅剧煡閬撴槸浠€涔?
瀵艰嚧浜嗗畠銆傛湁涓€涓姌涓柟妗堬紝閭ｅ氨鏄惎鐢ㄤ簨浠躲€?

```

  # echo 0 > options/function-trace
  # echo wakeup_rt > current_tracer
  # echo 1 > events/enable
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # chrt -f 5 sleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: wakeup_rt
  #
  # wakeup_rt latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 6 us, #12/12, CPU#2 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: sleep-5882 (uid:0 nice:0 policy:1 rt_prio:5)
  #    -----------------
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
    <idle>-0       2d.h4    0us :      0:120:R   + [002]  5882: 94:R sleep
    <idle>-0       2d.h4    0us : ttwu_do_activate.constprop.87 <-try_to_wake_up
    <idle>-0       2d.h4    1us : sched_wakeup: comm=sleep pid=5882 prio=94 success=1 target_cpu=002
    <idle>-0       2dNh2    1us : hrtimer_expire_exit: hrtimer=ffff88007796feb8
    <idle>-0       2.N.2    2us : power_end: cpu_id=2
    <idle>-0       2.N.2    3us : cpu_idle: state=4294967295 cpu_id=2
    <idle>-0       2dN.3    4us : hrtimer_cancel: hrtimer=ffff88007d50d5e0
    <idle>-0       2dN.3    4us : hrtimer_start: hrtimer=ffff88007d50d5e0 function=tick_sched_timer expires=34311211000000 softexpires=34311211000000
    <idle>-0       2.N.2    5us : rcu_utilization: Start context switch
    <idle>-0       2.N.2    5us : rcu_utilization: End context switch
    <idle>-0       2d..3    6us : __schedule <-schedule
    <idle>-0       2d..3    6us :      0:120:R ==> [002]  5882: 94:R sleep


```
### 纭欢寤惰繜鎺㈡祴鍣?


纭欢寤惰繜鎺㈡祴鍣ㄩ€氳繃鍚敤 "hwlat" 璺熻釜鍣ㄦ潵杩愯銆?

娉ㄦ剰锛岃璺熻釜鍣ㄤ細褰卞搷绯荤粺鎬ц兘锛屽洜涓哄畠浼氬懆鏈熸€у湴璁╀竴涓?CPU 鍦ㄤ腑鏂鐢ㄧ殑鎯呭喌涓嬫寔缁繖绛夈€?

```

  # echo hwlat > current_tracer
  # sleep 100
  # cat trace
  # tracer: hwlat
  #
  # entries-in-buffer/entries-written: 13/13   #P:8
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
             <...>-1729  [001] d...   678.473449: #1     inner/outer(us):   11/12    ts:1581527483.343962693 count:6
             <...>-1729  [004] d...   689.556542: #2     inner/outer(us):   16/9     ts:1581527494.889008092 count:1
             <...>-1729  [005] d...   714.756290: #3     inner/outer(us):   16/16    ts:1581527519.678961629 count:5
             <...>-1729  [001] d...   718.788247: #4     inner/outer(us):    9/17    ts:1581527523.889012713 count:1
             <...>-1729  [002] d...   719.796341: #5     inner/outer(us):   13/9     ts:1581527524.912872606 count:1
             <...>-1729  [006] d...   844.787091: #6     inner/outer(us):    9/12    ts:1581527649.889048502 count:2
             <...>-1729  [003] d...   849.827033: #7     inner/outer(us):   18/9     ts:1581527654.889013793 count:1
             <...>-1729  [007] d...   853.859002: #8     inner/outer(us):    9/12    ts:1581527658.889065736 count:1
             <...>-1729  [001] d...   855.874978: #9     inner/outer(us):   9/11    ts:1581527660.861991877 count:1
             <...>-1729  [001] d...   863.938932: #10    inner/outer(us):    9/11    ts:1581527668.970010500 count:1 nmi-total:7 nmi-count:1
             <...>-1729  [007] d...   878.050780: #11    inner/outer(us):    9/12    ts:1581527683.385002600 count:1 nmi-total:5 nmi-count:1
             <...>-1729  [007] d...   886.114702: #12    inner/outer(us):    9/12    ts:1581527691.385001600 count:1


```
涓婇潰鐨勮緭鍑哄湪澶撮儴涓婂ぇ鑷寸浉鍚屻€傛墍鏈変簨浠堕兘浼氭湁涓柇绂佺敤鏍囧織 'd'銆傚湪 FUNCTION 鏍囬涓嬫柟鏄細

 #1
	杩欐槸璁板綍鐨勩€佸ぇ浜?tracing_threshold锛堣涓嬫枃锛夌殑浜嬩欢璁℃暟銆?

 inner/outer(us):   11/11

      杩欐樉绀轰袱涓暟瀛楋細"鍐呴儴寤惰繜"鍜?澶栭儴寤惰繜"銆傛祴璇曞湪涓€涓惊鐜腑杩愯锛屾鏌ヤ袱娆℃椂闂存埑銆?
      鍦ㄤ袱涓椂闂存埑涔嬮棿妫€娴嬪埌鐨勫欢杩熷氨鏄?鍐呴儴寤惰繜"锛岃€屽湪鍓嶄竴涓椂闂存埑鍜屽惊鐜腑涓嬩竴涓?
      鏃堕棿鎴充箣闂存娴嬪埌鐨勫欢杩熷氨鏄?澶栭儴寤惰繜"銆?

 ts:1581527483.343962693

      鍦ㄨ绐楀彛涓褰曠涓€涓欢杩熸椂鐨勭粷瀵规椂闂存埑銆?

 count:6

      鍦ㄨ绐楀彛鏈熼棿妫€娴嬪埌寤惰繜鐨勬鏁般€?

 nmi-total:7 nmi-count:1

      鍦ㄦ敮鎸佸畠鐨勬灦鏋勪笂锛屽鏋滄祴璇曟湡闂存潵浜?NMI锛屽垯 NMI 涓姳璐圭殑鏃堕棿浼氭姤鍛婂湪 "nmi-total"
      涓紙浠ュ井绉掍负鍗曚綅锛夈€?

      鎵€鏈夊叿鏈?NMI 鐨勬灦鏋勯兘浼氬湪娴嬭瘯鏈熼棿鏉ヤ簡 NMI 鏃舵樉绀?"nmi-count"銆?

hwlat 鏂囦欢锛?

  tracing_threshold
	璇ユ枃浠朵細鑷姩璁剧疆涓?"10"锛岃〃绀?10 寰銆傝繖鏄渶瑕佽妫€娴嬪埌鎵嶄細璁板綍璺熻釜鐨勫欢杩熼槇鍊笺€?

	娉ㄦ剰锛屽綋 hwlat 璺熻釜鍣ㄧ粨鏉燂紙鍚?"current_tracer" 鍐欏叆鍙︿竴涓窡韪櫒锛夋椂锛宼racing_threshold
	鐨勫師濮嬪€间細琚斁鍥炶鏂囦欢銆?

  hwlat_detector/width
	娴嬭瘯鍦ㄤ腑鏂鐢ㄧ姸鎬佷笅杩愯鐨勬椂闀裤€?

  hwlat_detector/window
	娴嬭瘯杩愯鐨勭獥鍙ｇ殑鏃堕暱銆備篃灏辨槸璇达紝娴嬭瘯浼氬湪姣忎釜 "window" 寰鍐呰繍琛?"width"
	寰銆?

  tracing_cpumask
	娴嬭瘯鍚姩鏃讹紝浼氬垱寤轰竴涓唴鏍哥嚎绋嬫潵杩愯娴嬭瘯銆傝绾跨▼浼氬湪姣忎釜鍛ㄦ湡锛堜竴涓?"window"锛?
	涔嬮棿浜?tracing_cpumask 涓垪鍑虹殑 CPU 涔嬮棿浜ゆ浛銆傝灏嗘祴璇曢檺鍒跺湪鐗瑰畾 CPU 涓婏紝璇?
	灏嗚鏂囦欢涓殑鎺╃爜璁剧疆涓烘祴璇曞簲褰撹繍琛岀殑閭ｄ簺 CPU銆?

### function


璇ヨ窡韪櫒灏辨槸鍑芥暟璺熻釜鍣ㄣ€傚彲浠ラ€氳繃璋冭瘯鏂囦欢绯荤粺鍚敤鍑芥暟璺熻釜鍣ㄣ€傜‘淇?ftrace_enabled 宸?
璁剧疆锛涘惁鍒欒璺熻釜鍣ㄥ氨鏄竴涓?nop銆傝鍙傞槄涓嬮潰鐨?"ftrace_enabled" 涓€鑺傘€?

```

  # sysctl kernel.ftrace_enabled=1
  # echo function > current_tracer
  # echo 1 > tracing_on
  # usleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: function
  #
  # entries-in-buffer/entries-written: 24799/24799   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
              bash-1994  [002] ....  3082.063030: mutex_unlock <-rb_simple_write
              bash-1994  [002] ....  3082.063031: __mutex_unlock_slowpath <-mutex_unlock
              bash-1994  [002] ....  3082.063031: __fsnotify_parent <-fsnotify_modify
              bash-1994  [002] ....  3082.063032: fsnotify <-fsnotify_modify
              bash-1994  [002] ....  3082.063032: __srcu_read_lock <-fsnotify
              bash-1994  [002] ....  3082.063032: add_preempt_count <-__srcu_read_lock
              bash-1994  [002] ...1  3082.063032: sub_preempt_count <-__srcu_read_lock
              bash-1994  [002] ....  3082.063033: __srcu_read_unlock <-fsnotify
  [...]


```
娉ㄦ剰锛氬嚱鏁拌窡韪櫒浣跨敤鐜舰缂撳啿鍖烘潵瀛樺偍涓婅堪鏉＄洰銆傛渶鏂扮殑鏁版嵁鍙兘浼氳鐩栨渶鏃х殑鏁版嵁銆傛湁鏃?
浣跨敤 echo 鏉ュ仠姝㈣窡韪苟涓嶅锛屽洜涓鸿窡韪彲鑳藉凡缁忚鐩栦簡浣犳兂璁板綍鐨勬暟鎹€傚洜姝わ紝鏈夋椂鏈€濂界洿鎺?
浠庣▼搴忎腑绂佺敤璺熻釜銆傝繖璁╀綘鑳藉鍦ㄥ懡涓綘鎰熷叴瓒ｇ殑閮ㄥ垎鏃跺仠姝㈣窡韪€傝浠?C 绋嬪簭鐩存帴绂佺敤璺熻釜锛?

```

	int trace_fd;
	[...]
	int main(int argc, char *argv[]) {
		[...]
		trace_fd = open(tracing_file("tracing_on"), O_WRONLY);
		[...]
		if (condition_hit()) {
			write(trace_fd, "0", 1);
		}
		[...]
	}

```
### 鍗曠嚎绋嬭窡韪?


閫氳繃鍚?set_ftrace_pid 鍐欏叆锛屼綘鍙互璺熻釜涓€涓?

```

  # cat set_ftrace_pid
  no pid
  # echo 3111 > set_ftrace_pid
  # cat set_ftrace_pid
  3111
  # echo function > current_tracer
  # cat trace | head
  # tracer: function
  #
  #           TASK-PID    CPU#    TIMESTAMP  FUNCTION
  #              | |       |          |         |
      yum-updatesd-3111  [003]  1637.254676: finish_task_switch <-thread_return
      yum-updatesd-3111  [003]  1637.254681: hrtimer_cancel <-schedule_hrtimeout_range
      yum-updatesd-3111  [003]  1637.254682: hrtimer_try_to_cancel <-hrtimer_cancel
      yum-updatesd-3111  [003]  1637.254683: lock_hrtimer_base <-hrtimer_try_to_cancel
      yum-updatesd-3111  [003]  1637.254685: fget_light <-do_sys_poll
      yum-updatesd-3111  [003]  1637.254686: pipe_poll <-do_sys_poll
  # echo > set_ftrace_pid
  # cat trace |head
  # tracer: function
  #
  #           TASK-PID    CPU#    TIMESTAMP  FUNCTION
  #              | |       |          |         |
  ##### CPU 3 buffer started ####
      yum-updatesd-3111  [003]  1701.957688: free_poll_entry <-poll_freewait
      yum-updatesd-3111  [003]  1701.957689: remove_wait_queue <-free_poll_entry
      yum-updatesd-3111  [003]  1701.957691: fput <-free_poll_entry
      yum-updatesd-3111  [003]  1701.957692: audit_syscall_exit <-sysret_audit
      yum-updatesd-3111  [003]  1701.957693: path_put <-audit_syscall_exit

```
濡傛灉浣犳兂鍦ㄦ墽琛屾椂璺熻釜涓€涓嚱鏁帮紝鍙互浣跨敤绫讳技杩欐牱涓€涓畝鍗曠▼搴忋€?

```

	#include <stdio.h>
	#include <stdlib.h>
	#include <sys/types.h>
	#include <sys/stat.h>
	#include <fcntl.h>
	#include <unistd.h>
	#include <string.h>

	#define _STR(x) #x
	#define STR(x) _STR(x)
	#define MAX_PATH 256

	const char *find_tracefs(void)
	{
	       static char tracefs[MAX_PATH+1];
	       static int tracefs_found;
	       char type[100];
	       FILE *fp;

	       if (tracefs_found)
		       return tracefs;

	       if ((fp = fopen("/proc/mounts","r")) == NULL) {
		       perror("/proc/mounts");
		       return NULL;
	       }

	       while (fscanf(fp, "%*s %"
		             STR(MAX_PATH)
		             "s %99s %*s %*d %*d\n",
		             tracefs, type) == 2) {
		       if (strcmp(type, "tracefs") == 0)
		               break;
	       }
	       fclose(fp);

	       if (strcmp(type, "tracefs") != 0) {
		       fprintf(stderr, "tracefs not mounted");
		       return NULL;
	       }

	       strcat(tracefs, "/tracing/");
	       tracefs_found = 1;

	       return tracefs;
	}

	const char *tracing_file(const char *file_name)
	{
	       static char trace_file[MAX_PATH+1];
	       snprintf(trace_file, MAX_PATH, "%s/%s", find_tracefs(), file_name);
	       return trace_file;
	}

	int main (int argc, char **argv)
	{
		if (argc < 1)
		        exit(-1);

		if (fork() > 0) {
		        int fd, ffd;
		        char line[64];
		        int s;

		        ffd = open(tracing_file("current_tracer"), O_WRONLY);
		        if (ffd < 0)
		                exit(-1);
		        write(ffd, "nop", 3);

		        fd = open(tracing_file("set_ftrace_pid"), O_WRONLY);
		        s = sprintf(line, "%d\n", getpid());
		        write(fd, line, s);

		        write(ffd, "function", 8);

		        close(fd);
		        close(ffd);

		        execvp(argv[1], argv+1);
		}

		return 0;
	}

```
鎴栬€呰繖涓畝鍗曠殑鑴氭湰锛?

```

  #!/bin/bash

  tracefs=`sed -ne 's/^tracefs \(.*\) tracefs.*/\1/p' /proc/mounts`
  echo 0 > $tracefs/tracing_on
  echo $$ > $tracefs/set_ftrace_pid
  echo function > $tracefs/current_tracer
  echo 1 > $tracefs/tracing_on
  exec "$@"


```
### 鍑芥暟鍥捐窡韪櫒


璇ヨ窡韪櫒涓庡嚱鏁拌窡韪櫒绫讳技锛屽尯鍒湪浜庡畠鍦ㄥ嚱鏁扮殑鍏ュ彛鍜屽嚭鍙ｅ閮借繘琛屾帰娴嬨€傝繖鏄€氳繃鍦ㄦ瘡涓?
task_struct 涓娇鐢ㄤ竴涓姩鎬佸垎閰嶇殑杩斿洖鍦板潃鏍堟潵瀹炵幇鐨勩€傚湪鍑芥暟鍏ュ彛澶勶紝璺熻釜鍣ㄤ細瑕嗙洊姣忎釜
琚窡韪嚱鏁扮殑杩斿洖鍦板潃锛屼互璁剧疆涓€涓嚜瀹氫箟鎺㈤拡銆傚洜姝ゅ師濮嬬殑杩斿洖鍦板潃琚瓨鍌ㄥ湪 task_struct
鐨勮繑鍥炲湴鍧€鏍堜笂銆?

鍦ㄥ嚱鏁颁袱绔兘杩涜鎺㈡祴浼氬甫鏉ョ壒娈婂姛鑳斤紝渚嬪锛?

- 娴嬮噺鍑芥暟鐨勬墽琛屾椂闂?
- 鎷ユ湁鍙潬鐨勮皟鐢ㄦ爤浠ョ粯鍒跺嚱鏁拌皟鐢ㄥ浘

璇ヨ窡韪櫒鍦ㄤ互涓嬪嚑绉嶆儏鍐典笅寰堟湁鐢細

- 浣犳兂鎵惧嚭鏌愪釜濂囨€殑鍐呮牳琛屼负鐨勫師鍥狅紝骞朵笖闇€瑕佽缁嗘煡鐪嬩换浣曞尯鍩燂紙鎴栫壒瀹氬尯鍩燂級鍐呴儴鍙戠敓浜?
  浠€涔堛€?

- 浣犳缁忓巻濂囨€殑寤惰繜锛屼絾寰堥毦鎵惧埌鍏舵牴婧愩€?

- 浣犳兂蹇€熸壘鍒版煇涓壒瀹氬嚱鏁版墍閲囧彇鐨勮矾寰?

- 浣犲彧鏄兂绐ユ帰涓€涓鍦ㄨ繍琛岀殑鍐呮牳鍐呴儴锛岀湅鐪嬮偅閲屽彂鐢熶簡浠€涔堛€?

```

  # tracer: function_graph
  #
  # CPU  DURATION                  FUNCTION CALLS
  # |     |   |                     |   |   |   |

   0)               |  sys_open() {
   0)               |    do_sys_open() {
   0)               |      getname() {
   0)               |        kmem_cache_alloc() {
   0)   1.382 us    |          __might_sleep();
   0)   2.478 us    |        }
   0)               |        strncpy_from_user() {
   0)               |          might_fault() {
   0)   1.389 us    |            __might_sleep();
   0)   2.553 us    |          }
   0)   3.807 us    |        }
   0)   7.876 us    |      }
   0)               |      alloc_fd() {
   0)   0.668 us    |        _spin_lock();
   0)   0.570 us    |        expand_files();
   0)   0.586 us    |        _spin_unlock();


```
鏈夊嚑涓垪鍙互鍔ㄦ€佸惎鐢?绂佺敤銆備綘鍙互鏍规嵁闇€瑕佷娇鐢ㄤ换鎰忛€夐」缁勫悎銆?

- 鍑芥暟鎵ц鎵€鍦ㄧ殑 CPU 缂栧彿榛樿鍚敤銆傛湁鏃舵渶濂藉彧璺熻釜涓€涓?CPU锛堣 tracing_cpumask 鏂囦欢锛夛紝
  鎴栬€呬綘鍙兘鍦?CPU 璺熻釜鍒囨崲鏃剁湅鍒颁贡搴忕殑鍑芥暟璋冪敤銆?

 - 闅愯棌锛歟cho nofuncgraph-cpu > trace_options
 - 鏄剧ず锛歟cho funcgraph-cpu > trace_options

- 鎸佺画鏃堕棿锛堝嚱鏁扮殑鎵ц鏃堕棿锛夋樉绀哄湪鍑芥暟鐨勯棴鍚堣姳鎷彿琛屼笂锛屾垨鑰呭湪鍙跺瓙鍑芥暟鐨勬儏鍐典笅鏄剧ず鍦ㄤ笌
  褰撳墠鍑芥暟鍚屼竴琛屼笂銆傞粯璁ゅ惎鐢ㄣ€?

 - 闅愯棌锛歟cho nofuncgraph-duration > trace_options
 - 鏄剧ず锛歟cho funcgraph-duration > trace_options

- overhead 瀛楁鍦ㄨ揪鍒版寔缁椂闂撮槇鍊兼椂浣嶄簬 duration 瀛楁涔嬪墠銆?

 - 闅愯棌锛歟cho nofuncgraph-overhead > trace_options
 - 鏄剧ず锛歟cho funcgraph-overhead > trace_options
 - 渚濊禆浜庯細funcgraph-duration

```

    3) # 1837.709 us |          } /* __switch_to */
    3)               |          finish_task_switch() {
    3)   0.313 us    |            _raw_spin_unlock_irq();
    3)   3.177 us    |          }
    3) # 1889.063 us |        } /* __schedule */
    3) ! 140.417 us  |      } /* __schedule */
    3) # 2034.948 us |    } /* schedule */
    3) * 33998.59 us |  } /* schedule_preempt_disabled */

    [...]

    1)   0.260 us    |              msecs_to_jiffies();
    1)   0.313 us    |              __rcu_read_unlock();
    1) + 61.770 us   |            }
    1) + 64.479 us   |          }
    1)   0.313 us    |          rcu_bh_qs();
    1)   0.313 us    |          __local_bh_enable();
    1) ! 217.240 us  |        }
    1)   0.365 us    |        idle_cpu();
    1)               |        rcu_irq_exit() {
    1)   0.417 us    |          rcu_eqs_enter_common.isra.47();
    1)   3.125 us    |        }
    1) ! 227.812 us  |      }
    1) ! 457.395 us  |    }
    1) @ 119760.2 us |  }

    [...]

    2)               |    handle_IPI() {
    1)   6.979 us    |                  }
    2)   0.417 us    |      scheduler_ipi();
    1)   9.791 us    |                }
    1) + 12.917 us   |              }
    2)   3.490 us    |    }
    1) + 15.729 us   |            }
    1) + 18.542 us   |          }
    2) $ 3594274 us  |  }

```

```

  + 琛ㄧず璇ュ嚱鏁拌秴杩?10 寰銆?
  ! 琛ㄧず璇ュ嚱鏁拌秴杩?100 寰銆?
  # 琛ㄧず璇ュ嚱鏁拌秴杩?1000 寰銆?
  * 琛ㄧず璇ュ嚱鏁拌秴杩?10 姣銆?
  @ 琛ㄧず璇ュ嚱鏁拌秴杩?100 姣銆?
  $ 琛ㄧず璇ュ嚱鏁拌秴杩?1 绉掋€?


```
- 浠诲姟/pid 瀛楁鏄剧ず鎵ц璇ュ嚱鏁扮殑绾跨▼鍛戒护琛屽拰 pid銆傞粯璁ょ鐢ㄣ€?

 - 闅愯棌锛歟cho nofuncgraph-proc > trace_options
 - 鏄剧ず锛歟cho funcgraph-proc > trace_options

```

    # tracer: function_graph
    #
    # CPU  TASK/PID        DURATION                  FUNCTION CALLS
    # |    |    |           |   |                     |   |   |   |
    0)    sh-4802     |               |                  d_free() {
    0)    sh-4802     |               |                    call_rcu() {
    0)    sh-4802     |               |                      __call_rcu() {
    0)    sh-4802     |   0.616 us    |                        rcu_process_gp_end();
    0)    sh-4802     |   0.586 us    |                        check_for_new_grace_period();
    0)    sh-4802     |   2.899 us    |                      }
    0)    sh-4802     |   4.040 us    |                    }
    0)    sh-4802     |   5.151 us    |                  }
    0)    sh-4802     | + 49.370 us   |                }

```
- 缁濆鏃堕棿瀛楁鏄郴缁熸椂閽熻嚜鍚姩浠ユ潵缁欏嚭鐨勭粷瀵规椂闂存埑銆傚湪鍑芥暟鐨勬瘡娆¤繘鍏?閫€鍑烘椂缁欏嚭姝ゆ椂闂寸殑
  蹇収銆?

 - 闅愯棌锛歟cho nofuncgraph-abstime > trace_options
 - 鏄剧ず锛歟cho funcgraph-abstime > trace_options

```

    #
    #      TIME       CPU  DURATION                  FUNCTION CALLS
    #       |         |     |   |                     |   |   |   |
    360.774522 |   1)   0.541 us    |                                          }
    360.774522 |   1)   4.663 us    |                                        }
    360.774523 |   1)   0.541 us    |                                        __wake_up_bit();
    360.774524 |   1)   6.796 us    |                                      }
    360.774524 |   1)   7.952 us    |                                    }
    360.774525 |   1)   9.063 us    |                                  }
    360.774525 |   1)   0.615 us    |                                  journal_mark_dirty();
    360.774527 |   1)   0.578 us    |                                  __brelse();
    360.774528 |   1)               |                                  reiserfs_prepare_for_journal() {
    360.774528 |   1)               |                                    unlock_buffer() {
    360.774529 |   1)               |                                      wake_up_bit() {
    360.774529 |   1)               |                                        bit_waitqueue() {
    360.774530 |   1)   0.594 us    |                                          __phys_addr();


```
鍑芥暟鍚嶆€绘槸鍦ㄥ嚱鏁扮殑闂悎鑺辨嫭鍙蜂箣鍚庢樉绀猴紝濡傛灉璇ュ嚱鏁板紑澶翠笉鍦ㄨ窡韪紦鍐插尯涓€?

瀵逛簬寮€澶村湪璺熻釜缂撳啿鍖轰腑鐨勫嚱鏁帮紝涔熷彲浠ュ惎鐢ㄩ棴鍚堣姳鎷彿涔嬪悗鏄剧ず鍑芥暟鍚嶏紝浠ヤ究鐢?grep 鏇?
瀹规槗鍦版悳绱㈠嚱鏁版寔缁椂闂淬€傞粯璁ょ鐢ㄣ€?

 - 闅愯棌锛歟cho nofuncgraph-tail > trace_options
 - 鏄剧ず锛歟cho funcgraph-tail > trace_options

```

    0)               |      putname() {
    0)               |        kmem_cache_free() {
    0)   0.518 us    |          __phys_addr();
    0)   1.757 us    |        }
    0)   2.861 us    |      }

  浣跨敤 funcgraph-tail 鐨勭ず渚嬶細

    0)               |      putname() {
    0)               |        kmem_cache_free() {
    0)   0.518 us    |          __phys_addr();
    0)   1.757 us    |        } /* kmem_cache_free() */
    0)   2.861 us    |      } /* putname() */

```
姣忎釜琚窡韪嚱鏁扮殑杩斿洖鍊煎彲浠ユ樉绀哄湪绛夊彿 "=" 涔嬪悗銆傚綋閬囧埌绯荤粺璋冪敤澶辫触鏃讹紝瀹冭兘闈炲父鏈夊府鍔╁湴
蹇€熷畾浣嶇涓€涓繑鍥為敊璇爜鐨勫嚱鏁般€?

 - 闅愯棌锛歟cho nofuncgraph-retval > trace_options
 - 鏄剧ず锛歟cho funcgraph-retval > trace_options

```

    1)               |    cgroup_migrate() {
    1)   0.651 us    |      cgroup_migrate_add_task(); /* = 0xffff93fcfd346c00 */
    1)               |      cgroup_migrate_execute() {
    1)               |        cpu_cgroup_can_attach() {
    1)               |          cgroup_taskset_first() {
    1)   0.732 us    |            cgroup_taskset_next(); /* = 0xffff93fc8fb20000 */
    1)   1.232 us    |          } /* cgroup_taskset_first = 0xffff93fc8fb20000 */
    1)   0.380 us    |          sched_rt_can_attach(); /* = 0x0 */
    1)   2.335 us    |        } /* cpu_cgroup_can_attach = -22 */
    1)   4.369 us    |      } /* cgroup_migrate_execute = -22 */
    1)   7.143 us    |    } /* cgroup_migrate = -22 */

```
涓婇潰鐨勭ず渚嬫樉绀哄嚱鏁?cpu_cgroup_can_attach 棣栧厛杩斿洖浜嗛敊璇爜 -22锛岀劧鍚庢垜浠彲浠ラ槄璇昏
鍑芥暟鐨勪唬鐮佹潵鎵惧埌鏍规湰鍘熷洜銆?

褰撴湭璁剧疆 funcgraph-retval-hex 閫夐」鏃讹紝杩斿洖鍊煎彲浠ヤ互鏅鸿兘鏂瑰紡鏄剧ず銆傚叿浣撴潵璇达紝濡傛灉瀹冩槸
閿欒鐮侊紝鍒欎細浠ユ湁绗﹀彿鍗佽繘鍒舵牸寮忔墦鍗帮紝鍚﹀垯浠ュ崄鍏繘鍒舵牸寮忔墦鍗般€?

 - 鏅鸿兘锛歟cho nofuncgraph-retval-hex > trace_options
 - 鍗佸叚杩涘埗锛歟cho funcgraph-retval-hex > trace_options

```

    1)               |      cgroup_migrate() {
    1)   0.651 us    |        cgroup_migrate_add_task(); /* = 0xffff93fcfd346c00 */
    1)               |        cgroup_migrate_execute() {
    1)               |          cpu_cgroup_can_attach() {
    1)               |            cgroup_taskset_first() {
    1)   0.732 us    |              cgroup_taskset_next(); /* = 0xffff93fc8fb20000 */
    1)   1.232 us    |            } /* cgroup_taskset_first = 0xffff93fc8fb20000 */
    1)   0.380 us    |            sched_rt_can_attach(); /* = 0x0 */
    1)   2.335 us    |          } /* cpu_cgroup_can_attach = 0xffffffea */
    1)   4.369 us    |        } /* cgroup_migrate_execute = 0xffffffea */
    1)   7.143 us    |      } /* cgroup_migrate = 0xffffffea */

```
鐩墠锛屼娇鐢?funcgraph-retval 閫夐」鏈変竴浜涢檺鍒讹紝杩欎簺闄愬埗灏嗗湪鏈潵琚秷闄わ細

- 鍗充娇鍑芥暟鐨勮繑鍥炵被鍨嬫槸 void锛屼粛鐒朵細鎵撳嵃涓€涓繑鍥炲€硷紝浣犲彲浠ョ洿鎺ュ拷鐣ュ畠銆?

- 鍗充娇杩斿洖鍊煎瓨鍌ㄥ湪澶氫釜瀵勫瓨鍣ㄤ腑锛屼篃鍙湁绗竴涓瘎瀛樺櫒涓殑鍊间細琚褰曞拰鎵撳嵃銆備妇渚嬫潵璇达紝
  鍦?x86 鏋舵瀯涓紝eax 鍜?edx 鐢ㄤ簬瀛樺偍涓€涓?64 浣嶇殑杩斿洖鍊硷紝浣?32 浣嶄繚瀛樺湪 eax 涓紝楂?
  32 浣嶄繚瀛樺湪 edx 涓€備絾鏄紝鍙湁淇濆瓨鍦?eax 涓殑鍊间細琚褰曞拰鎵撳嵃銆?

- 鍦ㄦ煇浜涜繃绋嬭皟鐢ㄦ爣鍑嗕腑锛屼緥濡?arm64 鐨?AAPCS64锛屽綋绫诲瀷灏忎簬涓€涓?GPR 鏃讹紝鐢辫皟鐢ㄨ€呰礋璐?
  鎵ц绐勫寲鎿嶄綔锛岄珮浣嶅彲鑳藉寘鍚?UNKNOWN 鍊笺€傚洜姝わ紝瀵逛簬姝ょ被鎯呭喌妫€鏌ヤ唬鐮佹槸鏄庢櫤鐨勩€備緥濡傦紝
  褰撳湪 64 浣?GPR 涓娇鐢?u8 鏃讹紝浣?[63:8] 鍙兘鍖呭惈浠绘剰鍊硷紝灏ゅ叾鏄湪杈冨ぇ绫诲瀷琚埅鏂椂
  锛堟棤璁烘槸鏄惧紡杩樻槸闅愬紡锛夈€備互涓嬫槸涓€浜涘叿浣撴渚嬫潵璇存槑杩欎竴鐐癸細

  **妗堜緥涓€**锛?

```

	u8 narrow_to_u8(u64 val)
	{
		// 闅愬紡鎴柇
		return val;
	}

  瀹冨彲鑳借缂栬瘧涓猴細

	narrow_to_u8:
		< ... ftrace 鎻掓々 ... >
		RET

  濡傛灉浣犲悜璇ュ嚱鏁颁紶鍏?0x123456789abcdef 骞舵兂灏嗗叾绐勫寲锛屽畠鍙兘琚褰曚负 0x123456789abcdef
  鑰屼笉鏄?0xef銆?

  **妗堜緥浜?*锛?

  鍑芥暟 error_if_not_4g_aligned 瀹氫箟濡備笅锛?

	int error_if_not_4g_aligned(u64 val)
	{
		if (val & GENMASK(31, 0))
			return -EINVAL;

		return 0;
	}

  瀹冨彲鑳借缂栬瘧涓猴細

	error_if_not_4g_aligned:
		CBNZ    w0, .Lnot_aligned
		RET			// 浣?[31:0] 涓洪浂锛屼綅
					// [63:32] 涓?UNKNOWN
	.Lnot_aligned:
		MOV    x0, #-EINVAL
		RET

  褰撲紶鍏?0x2_0000_0000 鏃讹紝杩斿洖鍊煎彲鑳借璁板綍涓?0x2_0000_0000 鑰屼笉鏄?0銆?

```
浣犲彲浠ヤ娇鐢?trace_printk() 鍦ㄧ壒瀹氬嚱鏁颁笂娣诲姞涓€浜涙敞閲娿€備緥濡傦紝濡傛灉浣犳兂鍦?__might_sleep()
鍑芥暟鍐呴儴娣诲姞娉ㄩ噴锛屽彧闇€鍖呭惈

```

	trace_printk("I'm a comment!\n")

```

```

   1)               |             __might_sleep() {
   1)               |                /* I'm a comment! */
   1)   1.449 us    |             }


```
浣犲彲鑳戒細鍦ㄨ璺熻釜鍣ㄧ殑浠ヤ笅 "dynamic ftrace"锛堝姩鎬?ftrace锛変竴鑺備腑鍙戠幇鍏朵粬鏈夌敤鍔熻兘锛屼緥濡?
鍙窡韪壒瀹氬嚱鏁版垨浠诲姟銆?

### 鍔ㄦ€?ftrace


濡傛灉璁剧疆浜?CONFIG_DYNAMIC_FTRACE锛屽湪鍑芥暟璺熻釜琚鐢ㄦ椂锛岀郴缁熻繍琛岀殑寮€閿€鍑犱箮涓洪浂銆傚叾
宸ヤ綔鍘熺悊鏄紝mcount 鍑芥暟璋冪敤锛堜綅浜庢瘡涓唴鏍稿嚱鏁扮殑寮€澶达紝鐢?gcc 鐨?-pg 寮€鍏崇敓鎴愶級涓€寮€濮?
鎸囧悜涓€涓畝鍗曠殑杩斿洖銆傦紙鍚敤 FTRACE 浼氬湪鍐呮牳缂栬瘧涓寘鍚?-pg 寮€鍏炽€傦級

鍦ㄧ紪璇戞椂锛屾瘡涓?C 鏂囦欢鐩爣閮戒細缁忚繃 recordmcount 绋嬪簭锛堜綅浜?scripts 鐩綍锛夈€傝绋嬪簭浼?
瑙ｆ瀽 C 鐩爣涓殑 ELF 澶达紝浠ユ壘鍒?.text 娈典腑鎵€鏈夎皟鐢?mcount 鐨勪綅缃€備粠 gcc 4.6 鐗堟湰寮€濮嬶紝
x86 澧炲姞浜?-mfentry锛屽畠璋冪敤 "__fentry__" 鑰屼笉鏄?"mcount"銆傚畠鍦ㄦ爤甯у垱寤轰箣鍓嶈皟鐢ㄣ€?

娉ㄦ剰锛屽苟闈炴墍鏈夋閮戒細琚窡韪€傚畠浠彲鑳戒細琚?notrace 闃绘锛屾垨浠ュ叾浠栨柟寮忚闃绘锛屽苟涓旀墍鏈?
鍐呰仈鍑芥暟閮戒笉浼氳璺熻釜銆傛煡鐪?"available_filter_functions" 鏂囦欢浠ヤ簡瑙ｅ摢浜涘嚱鏁板彲浠ヨ璺熻釜銆?

浼氬垱寤轰竴涓悕涓?"__mcount_loc" 鐨勬锛屽叾涓寘鍚 .text 娈典腑鎵€鏈?mcount/fentry 璋冪敤绔欑偣
鐨勫紩鐢ㄣ€俽ecordmcount 绋嬪簭灏嗚繖涓閲嶆柊閾炬帴鍥炲師濮嬬洰鏍囥€傚唴鏍哥殑鏈€缁堥摼鎺ラ樁娈典細灏嗘墍鏈?
杩欎簺寮曠敤娣诲姞鍒颁竴涓崟鐙殑琛ㄤ腑銆?

鍦ㄥ惎鍔ㄦ椂锛屽湪 SMP 鍒濆鍖栦箣鍓嶏紝鍔ㄦ€?ftrace 浠ｇ爜浼氭壂鎻忔琛ㄥ苟灏嗘墍鏈変綅缃洿鏂颁负 nop銆傚畠
杩樹細璁板綍杩欎簺浣嶇疆锛屽苟灏嗗畠浠坊鍔犲埌 available_filter_functions 鍒楄〃涓€傛ā鍧楀湪鍔犺浇鏃躲€?
鎵ц鍓嶈澶勭悊銆傚綋鍗歌浇妯″潡鏃讹紝瀹冧篃浼氬皢鍏跺嚱鏁颁粠 ftrace 鍑芥暟鍒楄〃涓Щ闄ゃ€傝繖鍦ㄦā鍧楀嵏杞戒唬鐮?
涓槸鑷姩瀹屾垚鐨勶紝妯″潡浣滆€呮棤闇€涓烘鎷呭績銆?

鍚敤璺熻釜鏃讹紝淇敼鍑芥暟璺熻釜鐐圭殑杩囩▼渚濊禆浜庢灦鏋勩€傛棫鏂规硶鏄娇鐢?kstop_machine 鏉ラ槻姝笌姝ｅ湪
鎵ц琚慨鏀逛唬鐮佺殑 CPU 鍙戠敓绔炰簤锛堣繖鍙兘瀵艰嚧 CPU 鍋氬嚭涓嶅笇鏈涚殑浜嬫儏锛岀壒鍒槸濡傛灉淇敼鍚庣殑浠ｇ爜
璺ㄨ秺浜嗙紦瀛橈紙鎴栭〉锛夎竟鐣岋級锛屽苟灏?nop 鎵撹ˉ涓佸洖璋冪敤銆備絾杩欎竴娆★紝瀹冧滑璋冪敤鐨勪笉鍐嶆槸 mcount
锛堥偅鍙槸涓€涓嚱鏁版々锛夈€傚畠浠幇鍦ㄨ皟鐢ㄨ繘鍏?ftrace 鍩虹璁炬柦銆?

淇敼鍑芥暟璺熻釜鐐圭殑鏂版柟娉曟槸锛氬湪瑕佷慨鏀圭殑浣嶇疆鏀剧疆涓€涓柇鐐癸紝鍚屾鎵€鏈?CPU锛屼慨鏀规柇鐐规湭瑕嗙洊鐨?
鎸囦护鍏朵綑閮ㄥ垎銆傚啀娆″悓姝ユ墍鏈?CPU锛岀劧鍚庣敤瀹屾垚鐨勭増鏈紙鎸囧悜 ftrace 璋冪敤绔欑偣锛夌Щ闄ゆ柇鐐广€?

鏌愪簺鏋舵瀯鐢氳嚦涓嶉渶瑕佹姌鑵惧悓姝ワ紝鍙互鐩存帴灏嗘柊浠ｇ爜瑕嗙洊鍦ㄦ棫浠ｇ爜涔嬩笂锛岃€屼笉浼氬嚭鐜板叾浠?CPU
鍚屾椂鎵ц瀹冪殑闂銆?

璁板綍琚窡韪嚱鏁扮殑涓€涓壒娈婂壇浣滅敤鏄紝鎴戜滑鐜板湪鍙互鏈夐€夋嫨鍦伴€夋嫨瑕佽窡韪摢浜涘嚱鏁帮紝浠ュ強甯屾湜
mcount 璋冪敤淇濇寔涓?nop 鐨勫摢浜涘嚱鏁般€?

浣跨敤涓や釜鏂囦欢锛屼竴涓敤浜庡惎鐢紝涓€涓敤浜庣鐢ㄦ寚瀹氬嚱鏁扮殑璺熻釜銆傚畠浠槸锛?

  set_ftrace_filter

鍜?

  set_ftrace_notrace

浣犲彲浠ユ坊鍔犲埌杩欎簺鏂囦欢涓殑鍙敤鍑芥暟鍒楄〃鍒楀湪锛?

   available_filter_functions

```

  # cat available_filter_functions
  put_prev_task_idle
  kmem_cache_create
  pick_next_task_rt
  cpus_read_lock
  pick_next_task_fair
  mutex_lock
  [...]

```

```

  # echo sys_nanosleep hrtimer_interrupt > set_ftrace_filter
  # echo function > current_tracer
  # echo 1 > tracing_on
  # usleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: function
  #
  # entries-in-buffer/entries-written: 5/5   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
            usleep-2665  [001] ....  4186.475355: sys_nanosleep <-system_call_fastpath
            <idle>-0     [001] d.h1  4186.475409: hrtimer_interrupt <-smp_apic_timer_interrupt
            usleep-2665  [001] d.h1  4186.475426: hrtimer_interrupt <-smp_apic_timer_interrupt
            <idle>-0     [003] d.h1  4186.475426: hrtimer_interrupt <-smp_apic_timer_interrupt
            <idle>-0     [002] d.h1  4186.475427: hrtimer_interrupt <-smp_apic_timer_interrupt

```
瑕佹煡鐪嬪摢浜涘嚱鏁版鍦ㄨ璺熻釜锛屼綘鍙互 cat 璇ユ枃浠讹細

```

  # cat set_ftrace_filter
  hrtimer_interrupt
  sys_nanosleep


```
涔熻杩欒繕涓嶅銆傝繃婊ゅ櫒杩樺厑璁?glob(7) 鍖归厤銆?

  `<match>*`
	鍖归厤浠?<match> 寮€澶寸殑鍑芥暟
  `*<match>`
	鍖归厤浠?<match> 缁撳熬鐨勫嚱鏁?
  `**<match>**`
	鍖归厤鍖呭惈 <match> 鐨勫嚱鏁?
  `<match1>*<match2>`
	鍖归厤浠?<match1> 寮€澶村苟浠?<match2> 缁撳熬鐨勫嚱鏁?

      鏈€濂戒娇鐢ㄥ紩鍙峰皢閫氶厤绗︽嫭璧锋潵锛屽惁鍒?shell 鍙兘浼氬皢鍙傛暟灞曞紑涓烘湰鍦扮洰褰曚腑鐨勬枃浠跺悕銆?

```

  # echo 'hrtimer_*' > set_ftrace_filter

```

```

  # tracer: function
  #
  # entries-in-buffer/entries-written: 897/897   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
            <idle>-0     [003] dN.1  4228.547803: hrtimer_cancel <-tick_nohz_idle_exit
            <idle>-0     [003] dN.1  4228.547804: hrtimer_try_to_cancel <-hrtimer_cancel
            <idle>-0     [003] dN.2  4228.547805: hrtimer_force_reprogram <-__remove_hrtimer
            <idle>-0     [003] dN.1  4228.547805: hrtimer_forward <-tick_nohz_idle_exit
            <idle>-0     [003] dN.1  4228.547805: hrtimer_start_range_ns <-hrtimer_start_expires.constprop.11
            <idle>-0     [003] d..1  4228.547858: hrtimer_get_next_event <-get_next_timer_interrupt
            <idle>-0     [003] d..1  4228.547859: hrtimer_start <-__tick_nohz_idle_enter
            <idle>-0     [003] d..2  4228.547860: hrtimer_force_reprogram <-__rem

```
娉ㄦ剰鎴戜滑涓㈠け浜?sys_nanosleep銆?

```

  # cat set_ftrace_filter
  hrtimer_run_queues
  hrtimer_run_pending
  hrtimer_setup
  hrtimer_cancel
  hrtimer_try_to_cancel
  hrtimer_forward
  hrtimer_start
  hrtimer_reprogram
  hrtimer_force_reprogram
  hrtimer_get_next_event
  hrtimer_interrupt
  hrtimer_nanosleep
  hrtimer_wakeup
  hrtimer_get_remaining
  hrtimer_get_res
  hrtimer_init_sleeper


```
杩欐槸鍥犱负 '>' 鍜?'>>' 鐨勮涓轰笌鍦?bash 涓畬鍏ㄤ竴鏍枫€傝閲嶅啓杩囨护鍣紝浣跨敤 '>'锛涜杩藉姞鍒?
杩囨护鍣紝浣跨敤 '>>'銆?

瑕佹竻闄よ繃婊ゅ櫒浠ヤ究璁板綍鎵€鏈夊嚱鏁?

```

 # echo > set_ftrace_filter
 # cat set_ftrace_filter
 #

```
鍐嶆锛岀幇鍦ㄦ垜浠兂杩藉姞銆?

```

  # echo sys_nanosleep > set_ftrace_filter
  # cat set_ftrace_filter
  sys_nanosleep
  # echo 'hrtimer_*' >> set_ftrace_filter
  # cat set_ftrace_filter
  hrtimer_run_queues
  hrtimer_run_pending
  hrtimer_setup
  hrtimer_cancel
  hrtimer_try_to_cancel
  hrtimer_forward
  hrtimer_start
  hrtimer_reprogram
  hrtimer_force_reprogram
  hrtimer_get_next_event
  hrtimer_interrupt
  sys_nanosleep
  hrtimer_nanosleep
  hrtimer_wakeup
  hrtimer_get_remaining
  hrtimer_get_res
  hrtimer_init_sleeper


```
set_ftrace_notrace 闃绘杩欎簺鍑芥暟琚窡韪€?

```

  # echo '*preempt*' '*lock*' > set_ftrace_notrace

```

```

  # tracer: function
  #
  # entries-in-buffer/entries-written: 39608/39608   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
              bash-1994  [000] ....  4342.324896: file_ra_state_init <-do_dentry_open
              bash-1994  [000] ....  4342.324897: open_check_o_direct <-do_last
              bash-1994  [000] ....  4342.324897: ima_file_check <-do_last
              bash-1994  [000] ....  4342.324898: process_measurement <-ima_file_check
              bash-1994  [000] ....  4342.324898: ima_get_action <-process_measurement
              bash-1994  [000] ....  4342.324898: ima_match_policy <-ima_get_action
              bash-1994  [000] ....  4342.324899: do_truncate <-do_last
              bash-1994  [000] ....  4342.324899: setattr_should_drop_suidgid <-do_truncate
              bash-1994  [000] ....  4342.324899: notify_change <-do_truncate
              bash-1994  [000] ....  4342.324900: current_fs_time <-notify_change
              bash-1994  [000] ....  4342.324900: current_kernel_time <-current_fs_time
              bash-1994  [000] ....  4342.324900: timespec_trunc <-current_fs_time

```
鎴戜滑鍙互鐪嬪埌涓嶅啀鏈?lock 鎴?preempt 璺熻釜銆?

### 閫氳繃绱㈠紩閫夋嫨鍑芥暟杩囨护鍣?


鐢变簬瀛楃涓插鐞嗕唬浠烽珮鏄傦紙鍦ㄥ皢浼犲叆鐨勫瓧绗︿覆涓庡嚱鏁板湴鍧€姣旇緝涔嬪墠锛岄渶瑕佸厛鏌ユ壘鍑芥暟鐨勫湴鍧€锛夛紝
涔熷彲浠ヤ娇鐢ㄤ竴涓储寮曟潵鍚敤鍑芥暟銆傝繖鍦ㄤ竴娆℃€ц缃暟鍗冧釜鐗瑰畾鍑芥暟鏃跺緢鏈夌敤銆傞€氳繃浼犲叆涓€涓暟瀛?
鍒楄〃锛屼笉浼氬彂鐢熶换浣曞瓧绗︿覆澶勭悊銆傜浉鍙嶏紝浼氶€夋嫨鍐呴儴鏁扮粍锛堝搴斾簬 "available_filter_functions"
鏂囦欢涓殑鍑芥暟锛変腑鐗瑰畾浣嶇疆澶勭殑鍑芥暟銆?

```

  # echo 1 > set_ftrace_filter

```
灏嗛€夋嫨 "available_filter_functions" 涓垪鍑虹殑绗竴涓嚱鏁?

```

  # head -1 available_filter_functions
  trace_initcall_finish_cb

  # cat set_ftrace_filter
  trace_initcall_finish_cb

  # head -50 available_filter_functions | tail -1
  x86_pmu_commit_txn

  # echo 1 50 > set_ftrace_filter
  # cat set_ftrace_filter
  trace_initcall_finish_cb
  x86_pmu_commit_txn

```
### 鍑芥暟鍥捐窡韪櫒鐨勫姩鎬?ftrace


铏界劧涓婇潰瑙ｉ噴鐨勫唴瀹瑰悓鏃舵秹鍙婂嚱鏁拌窡韪櫒鍜屽嚱鏁板浘璺熻釜鍣紝浣嗗嚱鏁板浘璺熻釜鍣ㄤ腑鍙湁涓€浜涚壒娈?
鍔熻兘鍙敤銆?

濡傛灉浣犲彧鎯宠窡韪竴涓嚱鏁板強鍏舵墍鏈夊瓙鍑芥暟锛?

```

 echo __do_fault > set_graph_function

```
灏嗕骇鐢熷涓?__do_fault() 鐨?灞曞紑"璺熻釜

```

   0)               |  __do_fault() {
   0)               |    filemap_fault() {
   0)               |      find_lock_page() {
   0)   0.804 us    |        find_get_page();
   0)               |        __might_sleep() {
   0)   1.329 us    |        }
   0)   3.904 us    |      }
   0)   4.979 us    |    }
   0)   0.653 us    |    _spin_lock();
   0)   0.578 us    |    page_add_file_rmap();
   0)   0.525 us    |    native_set_pte_at();
   0)   0.585 us    |    _spin_unlock();
   0)               |    unlock_page() {
   0)   0.541 us    |      page_waitqueue();
   0)   0.639 us    |      __wake_up_bit();
   0)   2.786 us    |    }
   0) + 14.237 us   |  }
   0)               |  __do_fault() {
   0)               |    filemap_fault() {
   0)               |      find_lock_page() {
   0)   0.698 us    |        find_get_page();
   0)               |        __might_sleep() {
   0)   1.412 us    |        }
   0)   3.950 us    |      }
   0)   5.098 us    |    }
   0)   0.631 us    |    _spin_lock();
   0)   0.571 us    |    page_add_file_rmap();
   0)   0.526 us    |    native_set_pte_at();
   0)   0.586 us    |    _spin_unlock();
   0)               |    unlock_page() {
   0)   0.533 us    |      page_waitqueue();
   0)   0.638 us    |      __wake_up_bit();
   0)   2.793 us    |    }
   0) + 14.012 us   |  }

```

```

 echo sys_open > set_graph_function
 echo sys_close >> set_graph_function

```
鐜板湪濡傛灉浣犳兂鍥炲埌璺熻釜鎵€鏈夊嚱鏁帮紝鍙互娓呴櫎

```

 echo > set_graph_function


```
### ftrace_enabled


娉ㄦ剰锛宲roc sysctl ftrace_enable 鏄嚱鏁拌窡韪櫒鐨勪竴涓€诲紑/鍏冲紑鍏炽€傞粯璁ゆ儏鍐典笅瀹冩槸鍚敤鐨?
锛堝綋鍐呮牳涓惎鐢ㄤ簡鍑芥暟璺熻釜鏃讹級銆傚鏋滃畠琚鐢紝鎵€鏈夊嚱鏁拌窡韪兘浼氳绂佺敤銆傝繖涓嶄粎鍖呮嫭 ftrace
鐨勫嚱鏁拌窡韪櫒锛屼篃鍖呮嫭浠讳綍鍏朵粬鐢ㄩ€旓紙perf銆乲probes銆佹爤璺熻釜銆佹€ц兘鍒嗘瀽绛夛級銆傚鏋滄敞鍐屼簡甯︽湁
FTRACE_OPS_FL_PERMANENT 鏍囧織璁剧疆鐨勫洖璋冿紝鍒欐棤娉曠鐢ㄥ畠銆?

璇疯皑鎱庣鐢ㄦ寮€鍏炽€?

```

  sysctl kernel.ftrace_enabled=0
  sysctl kernel.ftrace_enabled=1

 鎴?

  echo 0 > /proc/sys/kernel/ftrace_enabled
  echo 1 > /proc/sys/kernel/ftrace_enabled


```
### 杩囨护鍛戒护


set_ftrace_filter 鎺ュ彛鏀寔涓€浜涘懡浠ゃ€?

```

  <function>:<command>:<parameter>

```
鏀寔鐨勫懡浠ゅ涓嬶細

- mod:
  璇ュ懡浠ゆ寜妯″潡鍚敤鍑芥暟杩囨护銆傚弬鏁板畾涔夋ā鍧椼€備緥濡傦紝濡傛灉鍙兂瑕?ext3 妯″潡涓殑 write*
  鍑芥暟锛岃繍琛岋細

   echo 'write*:mod:ext3' > set_ftrace_filter

  璇ュ懡浠や互涓庡熀浜庡嚱鏁板悕杩囨护鐩稿悓鐨勬柟寮忎笌杩囨护鍣ㄤ氦浜掋€傚洜姝わ紝閫氳繃鍦ㄨ繃婊ゅ櫒鏂囦欢涓拷鍔狅紙>>锛?
  鏉ユ坊鍔犱笉鍚屾ā鍧椾腑鐨勬洿澶氬嚱鏁般€傞€氳繃鍔犲墠缂€鏉ョЩ闄ょ壒瀹氭ā鍧楃殑鍑芥暟

   echo '!writeback*:mod:ext3' >> set_ftrace_filter

  mod 鍛戒护鏀寔妯″潡 glob 鍖归厤銆傜鐢ㄩ櫎鐗瑰畾妯″潡澶栫殑鎵€鏈夊嚱鏁拌窡韪細

   echo '!*:mod:!ext3' >> set_ftrace_filter

  绂佺敤鎵€鏈夋ā鍧楃殑璺熻釜锛屼絾浠嶈窡韪唴鏍革細

   echo '!*:mod:*' >> set_ftrace_filter

  浠呭惎鐢ㄥ唴鏍歌繃婊わ細

   echo '*write*:mod:!*' >> set_ftrace_filter

  鍚敤妯″潡 glob 鍖归厤鐨勮繃婊わ細

   echo '*write*:mod:*snd*' >> set_ftrace_filter

```
- traceon/traceoff:
  杩欎簺鍛戒护鍦ㄥ懡涓寚瀹氬嚱鏁版椂鎵撳紑鍜屽叧闂窡韪€傚弬鏁板喅瀹氳窡韪郴缁熻鎵撳紑鍜屽叧闂殑娆℃暟銆傚鏋?
  鏈寚瀹氾紝鍒欐病鏈夐檺鍒躲€備緥濡傦紝瑕佸湪鍑虹幇 schedule bug 鏃剁鐢ㄨ窡韪?

   echo '__schedule_bug:traceoff:5' > set_ftrace_filter

  瑕佸湪姣忔鍛戒腑 __schedule_bug 鏃跺缁堢鐢ㄨ窡韪細

   echo '__schedule_bug:traceoff' > set_ftrace_filter

  鏃犺鏄惁杩藉姞鍒?set_ftrace_filter锛岃繖浜涘懡浠ら兘鏄疮绉殑銆傝绉婚櫎涓€涓懡浠わ紝鍦ㄥ叾鍓嶅姞 '!'
  骞跺幓鎺夊弬鏁帮細

   echo '!__schedule_bug:traceoff:0' > set_ftrace_filter

  涓婇潰绉婚櫎浜嗗甫鏈夎鏁板櫒鐨?__schedule_bug 鐨?traceoff 鍛戒护銆傝绉婚櫎涓嶅甫璁℃暟鍣ㄧ殑鍛戒护锛?

   echo '!__schedule_bug:traceoff' > set_ftrace_filter

```
- snapshot:
  鍦ㄥ懡涓鍑芥暟鏃朵細瑙﹀彂涓€娆″揩鐓с€?

   echo 'native_flush_tlb_others:snapshot' > set_ftrace_filter

  鍙揩鐓т竴娆★細

   echo 'native_flush_tlb_others:snapshot:1' > set_ftrace_filter

  瑕佺Щ闄や笂杩板懡浠わ細

   echo '!native_flush_tlb_others:snapshot' > set_ftrace_filter
   echo '!native_flush_tlb_others:snapshot:0' > set_ftrace_filter

```
- enable_event/disable_event:
  杩欎簺鍛戒护鍙互鍚敤鎴栫鐢ㄤ竴涓窡韪簨浠躲€傛敞鎰忥紝鐢变簬鍑芥暟璺熻釜鍥炶皟闈炲父鏁忔劅锛屽綋娉ㄥ唽杩欎簺鍛戒护
  鏃讹紝璺熻釜鐐逛細琚縺娲伙紝浣嗕互"杞?妯″紡绂佺敤銆備篃灏辨槸璇达紝璺熻釜鐐逛細琚皟鐢紝浣嗗彧鏄笉浼氳璺熻釜銆?
  鍙鏈変竴涓Е鍙戝畠鐨勫懡浠ゅ瓨鍦紝浜嬩欢璺熻釜鐐瑰氨淇濇寔姝ゆā寮忋€?

   echo 'try_to_wake_up:enable_event:sched:sched_switch:2' > \
   	 set_ftrace_filter

  鏍煎紡涓猴細

    <function>:enable_event:<system>:<event>[:count]
    <function>:disable_event:<system>:<event>[:count]

  瑕佺Щ闄や簨浠跺懡浠わ細

   echo '!try_to_wake_up:enable_event:sched:sched_switch:0' > \
   	 set_ftrace_filter
   echo '!schedule:disable_event:sched:sched_switch' > \
   	 set_ftrace_filter

```
- dump:
  鍛戒腑璇ュ嚱鏁版椂锛屽畠浼氬皢 ftrace 鐜舰缂撳啿鍖虹殑鍐呭杞偍鍒版帶鍒跺彴銆傚鏋滀綘闇€瑕佽皟璇曟煇浜涗笢瑗匡紝
  骞舵兂鍦ㄥ懡涓煇涓嚱鏁版椂杞偍璺熻釜锛岃繖浼氬緢鏈夌敤銆備篃璁稿畠鏄竴涓湪涓夐噸鏁呴殰鍙戠敓涔嬪墠琚皟鐢ㄣ€?
  涓斾笉鍏佽浣犺幏鍙栧父瑙勮浆鍌ㄧ殑鍑芥暟銆?

- cpudump:
  鍛戒腑璇ュ嚱鏁版椂锛屽畠浼氬皢褰撳墠 CPU 鐨?ftrace 鐜舰缂撳啿鍖哄唴瀹硅浆鍌ㄥ埌鎺у埗鍙般€備笌 "dump" 鍛戒护
  涓嶅悓锛屽畠鍙墦鍗版墽琛屼簡瑙﹀彂杞偍鐨勫嚱鏁扮殑閭ｄ釜 CPU 鐨勭幆褰㈢紦鍐插尯鍐呭銆?

- stacktrace:
  鍛戒腑璇ュ嚱鏁版椂锛屼細璁板綍涓€鏉℃爤鍥炴函銆?

### trace_pipe


trace_pipe 杈撳嚭涓?trace 鏂囦欢鐩稿悓鐨勫唴瀹癸紝浣嗗璺熻釜鐨勫奖鍝嶄笉鍚屻€傛瘡娆′粠 trace_pipe 璇诲彇
閮戒細琚秷璐广€傝繖鎰忓懗鐫€鍚庣画璇诲彇浼氫笉鍚屻€傝窡韪槸瀹炴椂鐨勩€?

```

  # echo function > current_tracer
  # cat trace_pipe > /tmp/trace.out &
  [1] 4153
  # echo 1 > tracing_on
  # usleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: function
  #
  # entries-in-buffer/entries-written: 0/0   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |

  #
  # cat /tmp/trace.out
             bash-1994  [000] ....  5281.568961: mutex_unlock <-rb_simple_write
             bash-1994  [000] ....  5281.568963: __mutex_unlock_slowpath <-mutex_unlock
             bash-1994  [000] ....  5281.568963: __fsnotify_parent <-fsnotify_modify
             bash-1994  [000] ....  5281.568964: fsnotify <-fsnotify_modify
             bash-1994  [000] ....  5281.568964: __srcu_read_lock <-fsnotify
             bash-1994  [000] ....  5281.568964: add_preempt_count <-__srcu_read_lock
             bash-1994  [000] ...1  5281.568965: sub_preempt_count <-__srcu_read_lock
             bash-1994  [000] ....  5281.568965: __srcu_read_unlock <-fsnotify
             bash-1994  [000] ....  5281.568967: sys_dup2 <-system_call_fastpath


```
娉ㄦ剰锛岃鍙?trace_pipe 鏂囦欢浼氶樆濉烇紝鐩村埌鏈夋洿澶氳緭鍏ュ姞鍏ャ€傝繖涓?trace 鏂囦欢鐩稿弽銆傚鏋滄湁
浠讳綍杩涚▼鎵撳紑浜?trace 鏂囦欢杩涜璇诲彇锛屽畠瀹為檯涓婁細绂佺敤璺熻釜骞堕樆姝㈡坊鍔犳柊鏉＄洰銆倀race_pipe 鏂囦欢
娌℃湁姝ら檺鍒躲€?

### 璺熻釜鏉＄洰


鍦ㄥ唴鏍镐腑璇婃柇闂鏃讹紝鏁版嵁杩囧鎴栬繃灏戦兘浼氫护浜哄洶鎵般€俠uffer_size_kb 鏂囦欢鐢ㄤ簬淇敼鍐呴儴璺熻釜
缂撳啿鍖虹殑澶у皬銆傚垪鍑虹殑鏁板瓧鏄瘡涓?CPU 鍙互璁板綍鐨勬潯鐩暟銆傝鐭ラ亾瀹屾暣澶у皬锛屽皢鍙兘鐨?CPU 鏁伴噺
涔樹互鏉＄洰鏁般€?

```

  # cat buffer_size_kb
  1408 (units kilobytes)

```
鎴栬€呯畝鍗曞湴璇诲彇 buffer_total_size_kb

```

  # cat buffer_total_size_kb
  5632

```
瑕佷慨鏀圭紦鍐插尯锛屽彧闇€ echo 涓€涓暟瀛楋紙浠?1024 瀛楄妭涓哄崟浣嶏級銆?

```

  # echo 10000 > buffer_size_kb
  # cat buffer_size_kb
  10000 (units kilobytes)

```
瀹冧細灏濊瘯灏藉彲鑳藉鍦板垎閰嶃€傚鏋滀綘鍒嗛厤杩囧锛屽彲鑳戒細瑙﹀彂鍐呭瓨涓嶈冻锛圤ut-Of-Memory锛夈€?

```

  # echo 1000000000000 > buffer_size_kb
  -bash: echo: write error: Cannot allocate memory
  # cat buffer_size_kb
  85

```
per_cpu 缂撳啿鍖轰篃鍙互鍗曠嫭鏇存敼锛?

```

  # echo 10000 > per_cpu/cpu0/buffer_size_kb
  # echo 100 > per_cpu/cpu1/buffer_size_kb

```
褰?per_cpu 缂撳啿鍖轰笉鐩稿悓鏃讹紝椤跺眰鐨?buffer_size_kb 鍙細鏄剧ず涓€涓?X

```

  # cat buffer_size_kb
  X

```
杩欏氨鏄?buffer_total_size_kb 鏈夌敤涔嬪锛?

```

  # cat buffer_total_size_kb
  12916

```
鍐欏叆椤跺眰鐨?buffer_size_kb 浼氬皢鎵€鏈夌紦鍐插尯閲嶇疆涓虹浉鍚屽ぇ灏忋€?

### 蹇収


CONFIG_TRACER_SNAPSHOT 涓烘墍鏈夐潪寤惰繜璺熻釜鍣ㄦ彁渚涗竴涓€氱敤鐨勫揩鐓у姛鑳姐€傦紙璁板綍鏈€澶у欢杩熺殑
寤惰繜璺熻釜鍣紝渚嬪 "irqsoff" 鎴?"wakeup"锛屼笉鑳戒娇鐢ㄦ鍔熻兘锛屽洜涓哄畠浠凡缁忓湪鍐呴儴浣跨敤浜?
蹇収鏈哄埗銆傦級

蹇収鍦ㄦ煇涓€鏃跺埢淇濈暀褰撳墠鐨勮窡韪紦鍐插尯锛岃€屼笉鍋滄璺熻釜銆俧trace 灏嗗綋鍓嶇紦鍐插尯涓庝竴涓鐢ㄧ紦鍐?
鍖轰氦鎹紝璺熻釜鍦ㄦ柊鐨勫綋鍓嶏紙=涔嬪墠鐨勫鐢級缂撳啿鍖轰腑缁х画銆?

"tracing" 鐩綍涓互涓嬩笌 tracefs 鐩稿叧鐨勬枃浠朵笌姝ゅ姛鑳芥湁鍏筹細

  snapshot:

	璇ユ枃浠剁敤浜庢媿鎽勫揩鐓у苟璇诲彇蹇収鐨勮緭鍑恒€傚悜璇ユ枃浠?echo 1 浠ュ垎閰嶄竴涓鐢ㄧ紦鍐插尯骞?
	鎷嶆憚蹇収锛堜氦鎹級锛岀劧鍚庝互涓?"trace" 鐩稿悓鐨勬牸寮忥紙鍦ㄤ笂闈?"鏂囦欢绯荤粺" 涓€鑺備腑鎻忚堪锛?
	浠庤鏂囦欢璇诲彇蹇収銆傚揩鐓х殑璇诲彇鍜岃窡韪彲浠ュ苟琛屾墽琛屻€傚綋澶囩敤缂撳啿鍖鸿鍒嗛厤鏃讹紝echo 0
	浼氶噴鏀惧畠锛宔cho 鍏朵粬锛堟鏁帮級鍊间細娓呴櫎蹇収鍐呭銆傛洿澶氱粏鑺傚涓嬭〃鎵€绀恒€?

	+--------------+------------+------------+------------+
	|鐘舵€乗杈撳叆     |     0      |     1      |    else    |
	+==============+============+============+============+
	|鏈垎閰?       |(涓嶆墽琛屼换浣曟搷浣?| 鍒嗛厤+浜ゆ崲 |(涓嶆墽琛屼换浣曟搷浣?|
	+--------------+------------+------------+------------+
	|宸插垎閰?       |    閲婃斁    |    浜ゆ崲    |    娓呴櫎    |
	+--------------+------------+------------+------------+

浠ヤ笅鏄娇鐢ㄥ揩鐓у姛鑳界殑绀轰緥銆?

```

  # echo 1 > events/sched/enable
  # echo 1 > snapshot
  # cat snapshot
  # tracer: nop
  #
  # entries-in-buffer/entries-written: 71/71   #P:8
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
            <idle>-0     [005] d...  2440.603828: sched_switch: prev_comm=swapper/5 prev_pid=0 prev_prio=120   prev_state=R ==> next_comm=snapshot-test-2 next_pid=2242 next_prio=120
             sleep-2242  [005] d...  2440.603846: sched_switch: prev_comm=snapshot-test-2 prev_pid=2242 prev_prio=120   prev_state=R ==> next_comm=kworker/5:1 next_pid=60 next_prio=120
  [...]
          <idle>-0     [002] d...  2440.707230: sched_switch: prev_comm=swapper/2 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=snapshot-test-2 next_pid=2229 next_prio=120

  # cat trace
  # tracer: nop
  #
  # entries-in-buffer/entries-written: 77/77   #P:8
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
            <idle>-0     [007] d...  2440.707395: sched_switch: prev_comm=swapper/7 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=snapshot-test-2 next_pid=2243 next_prio=120
   snapshot-test-2-2229  [002] d...  2440.707438: sched_switch: prev_comm=snapshot-test-2 prev_pid=2229 prev_prio=120 prev_state=S ==> next_comm=swapper/2 next_pid=0 next_prio=120
  [...]


```
濡傛灉浣犲皾璇曞湪褰撳墠鐨勮窡韪櫒鏄煇涓欢杩熻窡韪櫒鏃朵娇鐢ㄦ蹇収鍔熻兘锛屼綘浼氬緱鍒颁互涓嬬粨鏋溿€?

```

  # echo wakeup > current_tracer
  # echo 1 > snapshot
  bash: echo: write error: Device or resource busy
  # cat snapshot
  cat: snapshot: Device or resource busy


```
### 瀹炰緥


鍦?tracefs 鐨?tracing 鐩綍涓紝鏈変竴涓悕涓?"instances" 鐨勭洰褰曘€傚彲浠ヤ娇鐢?mkdir 鍦ㄨ鐩綍
鍐呭垱寤烘柊鐩綍锛屽苟浣跨敤 rmdir 绉婚櫎鐩綍銆傜敤 mkdir 鍦ㄦ鐩綍涓垱寤虹殑鐩綍鍦ㄥ垱寤哄悗宸茬粡鍖呭惈
鏂囦欢鍜屽瓙鐩綍銆?

```

  # mkdir instances/foo
  # ls instances/foo
  buffer_size_kb  buffer_total_size_kb  events  free_buffer  per_cpu
  set_event  snapshot  trace  trace_clock  trace_marker  trace_options
  trace_pipe  tracing_on

```
濡備綘鎵€瑙侊紝鏂扮洰褰曠湅璧锋潵涓?tracing 鐩綍鏈韩鐩镐技銆傚疄闄呬笂瀹冮潪甯哥浉浼硷紝鍙槸缂撳啿鍖轰笌浜嬩欢涓庝富
鐩綍鎴栧垱寤虹殑浠讳綍鍏朵粬瀹炰緥鏃犲叧銆?

鏂扮洰褰曚腑鐨勬枃浠朵笌 tracing 鐩綍涓悓鍚嶇殑鏂囦欢宸ヤ綔鏂瑰紡鐩稿悓锛屽彧鏄娇鐢ㄧ殑缂撳啿鍖烘槸涓€涓嫭绔嬬殑鏂?
缂撳啿鍖恒€傝繖浜涙枃浠跺奖鍝嶈缂撳啿鍖猴紝浣嗕笉浼氬奖鍝嶄富缂撳啿鍖猴紙trace_options 闄ゅ锛夈€傚綋鍓嶏紝
trace_options 瀵规墍鏈夊疄渚嬪拰椤跺眰缂撳啿鍖虹殑褰卞搷鐩稿悓锛屼絾杩欏湪鏈潵鐗堟湰涓彲鑳戒細鏀瑰彉銆備篃灏辨槸璇达紝
閫夐」鍙兘浼氬彉鎴愮壒瀹氫簬瀹冧滑鎵€鍦ㄧ殑瀹炰緥銆?

娉ㄦ剰锛岄偅閲屾病鏈変换浣曞嚱鏁拌窡韪櫒鏂囦欢锛屼篃娌℃湁 current_tracer 鍜?available_tracers銆傝繖鏄?
鍥犱负缂撳啿鍖虹洰鍓嶅彧鑳戒负瀹冧滑鍚敤浜嬩欢銆?

```

  # mkdir instances/foo
  # mkdir instances/bar
  # mkdir instances/zoot
  # echo 100000 > buffer_size_kb
  # echo 1000 > instances/foo/buffer_size_kb
  # echo 5000 > instances/bar/per_cpu/cpu1/buffer_size_kb
  # echo function > current_trace
  # echo 1 > instances/foo/events/sched/sched_wakeup/enable
  # echo 1 > instances/foo/events/sched/sched_wakeup_new/enable
  # echo 1 > instances/foo/events/sched/sched_switch/enable
  # echo 1 > instances/bar/events/irq/enable
  # echo 1 > instances/zoot/events/syscalls/enable
  # cat trace_pipe
  CPU:2 [LOST 11745 EVENTS]
              bash-2044  [002] .... 10594.481032: _raw_spin_lock_irqsave <-get_page_from_freelist
              bash-2044  [002] d... 10594.481032: add_preempt_count <-_raw_spin_lock_irqsave
              bash-2044  [002] d..1 10594.481032: __rmqueue <-get_page_from_freelist
              bash-2044  [002] d..1 10594.481033: _raw_spin_unlock <-get_page_from_freelist
              bash-2044  [002] d..1 10594.481033: sub_preempt_count <-_raw_spin_unlock
              bash-2044  [002] d... 10594.481033: get_pageblock_flags_group <-get_pageblock_migratetype
              bash-2044  [002] d... 10594.481034: __mod_zone_page_state <-get_page_from_freelist
              bash-2044  [002] d... 10594.481034: zone_statistics <-get_page_from_freelist
              bash-2044  [002] d... 10594.481034: __inc_zone_state <-zone_statistics
              bash-2044  [002] d... 10594.481034: __inc_zone_state <-zone_statistics
              bash-2044  [002] .... 10594.481035: arch_dup_task_struct <-copy_process
  [...]

  # cat instances/foo/trace_pipe
              bash-1998  [000] d..4   136.676759: sched_wakeup: comm=kworker/0:1 pid=59 prio=120 success=1 target_cpu=000
              bash-1998  [000] dN.4   136.676760: sched_wakeup: comm=bash pid=1998 prio=120 success=1 target_cpu=000
            <idle>-0     [003] d.h3   136.676906: sched_wakeup: comm=rcu_preempt pid=9 prio=120 success=1 target_cpu=003
            <idle>-0     [003] d..3   136.676909: sched_switch: prev_comm=swapper/3 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=rcu_preempt next_pid=9 next_prio=120
       rcu_preempt-9     [003] d..3   136.676916: sched_switch: prev_comm=rcu_preempt prev_pid=9 prev_prio=120 prev_state=S ==> next_comm=swapper/3 next_pid=0 next_prio=120
              bash-1998  [000] d..4   136.677014: sched_wakeup: comm=kworker/0:1 pid=59 prio=120 success=1 target_cpu=000
              bash-1998  [000] dN.4   136.677016: sched_wakeup: comm=bash pid=1998 prio=120 success=1 target_cpu=000
              bash-1998  [000] d..3   136.677018: sched_switch: prev_comm=bash prev_pid=1998 prev_prio=120 prev_state=R+ ==> next_comm=kworker/0:1 next_pid=59 prio=120
       kworker/0:1-59    [000] d..4   136.677022: sched_wakeup: comm=sshd pid=1995 prio=120 success=1 target_cpu=001
       kworker/0:1-59    [000] d..3   136.677025: sched_switch: prev_comm=kworker/0:1 prev_pid=59 prev_prio=120 prev_state=S ==> next_comm=bash next_pid=1998 prio=120
  [...]

  # cat instances/bar/trace_pipe
       migration/1-14    [001] d.h3   138.732674: softirq_raise: vec=3 [action=NET_RX]
            <idle>-0     [001] dNh3   138.732725: softirq_raise: vec=3 [action=NET_RX]
              bash-1998  [000] d.h1   138.733101: softirq_raise: vec=1 [action=TIMER]
              bash-1998  [000] d.h1   138.733102: softirq_raise: vec=9 [action=RCU]
              bash-1998  [000] ..s2   138.733105: softirq_entry: vec=1 [action=TIMER]
              bash-1998  [000] ..s2   138.733106: softirq_exit: vec=1 [action=TIMER]
              bash-1998  [000] ..s2   138.733106: softirq_entry: vec=9 [action=RCU]
              bash-1998  [000] ..s2   138.733109: softirq_exit: vec=9 [action=RCU]
              sshd-1995  [001] d.h1   138.733278: irq_handler_entry: irq=21 name=uhci_hcd:usb4
              sshd-1995  [001] d.h1   138.733280: irq_handler_exit: irq=21 ret=unhandled
              sshd-1995  [001] d.h1   138.733281: irq_handler_entry: irq=21 name=eth0
              sshd-1995  [001] d.h1   138.733283: irq_handler_exit: irq=21 ret=handled
  [...]

  # cat instances/zoot/trace
  # tracer: nop
  #
  # entries-in-buffer/entries-written: 18996/18996   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
              bash-1998  [000] d...   140.733501: sys_write -> 0x2
              bash-1998  [000] d...   140.733504: sys_dup2(oldfd: a, newfd: 1)
              bash-1998  [000] d...   140.733506: sys_dup2 -> 0x1
              bash-1998  [000] d...   140.733508: sys_close(fd: a)
              bash-1998  [000] d...   140.733510: sys_close -> 0x0
              bash-1998  [000] d...   140.733514: sys_rt_sigprocmask(how: 0, nset: 0, oset: 6e2768, sigsetsize: 8)
              bash-1998  [000] d...   140.733515: sys_rt_sigprocmask -> 0x0
              bash-1998  [000] d...   140.733516: sys_rt_sigaction(sig: 2, act: 7fff718846f0, oact: 7fff71884650, sigsetsize: 8)
              bash-1998  [000] d...   140.733516: sys_rt_sigaction -> 0x0

```
浣犲彲浠ョ湅鍒帮紝鏈€椤跺眰鐨勮窡韪紦鍐插尯鍙樉绀轰簡鍑芥暟璺熻釜銆俧oo 瀹炰緥鏄剧ず浜嗗敜閱掑拰浠诲姟鍒囨崲銆?

瑕佺Щ闄ゅ疄渚嬶紝鍙渶鍒犻櫎瀹冧滑鐨勭洰褰曪細

```

  # rmdir instances/foo
  # rmdir instances/bar
  # rmdir instances/zoot

```
娉ㄦ剰锛屽鏋滄湁杩涚▼鍦ㄦ煇涓疄渚嬬洰褰曚腑鎵撳紑浜嗚窡韪枃浠讹紝rmdir 灏嗕互 EBUSY 澶辫触銆?


### 鏍堣窡韪?


鐢变簬鍐呮牳鎷ユ湁鍥哄畾澶у皬鐨勬爤锛屽湪鍑芥暟涓婃氮璐规爤绌洪棿鏄緢閲嶈鐨勩€傚唴鏍稿紑鍙戣€呭繀椤绘敞鎰忎粬浠湪鏍堜笂
鍒嗛厤浜嗕粈涔堛€傚鏋滀粬浠垎閰嶈繃澶氾紝绯荤粺灏辨湁鏍堟孩鍑虹殑鍗遍櫓锛屽苟浼氬彂鐢熸崯鍧忥紝閫氬父瀵艰嚧绯荤粺鎭愭厡銆?

鏈変竴浜涘伐鍏蜂細妫€鏌ヨ繖涓€鐐癸紝閫氬父鏄€氳繃涓柇瀹氭湡妫€鏌ヤ娇鐢ㄦ儏鍐点€備絾濡傛灉浣犺兘鍦ㄦ瘡娆″嚱鏁拌皟鐢ㄦ椂
鎵ц妫€鏌ワ紝閭ｅ皢闈炲父鏈夌敤銆傜敱浜?ftrace 鎻愪緵浜嗗嚱鏁拌窡韪櫒锛屼娇寰楀湪姣忔鍑芥暟璋冪敤鏃舵鏌ユ爤澶у皬
鍙樺緱鏂逛究銆傝繖閫氳繃鏍堣窡韪櫒鍚敤銆?

CONFIG_STACK_TRACER 鍚敤 ftrace 鐨勬爤璺熻釜鍔熻兘銆傝鍚敤瀹冿紝鍚?/proc/sys/kernel/stack_tracer_enabled 鍐欏叆 '1'銆?

```

 # echo 1 > /proc/sys/kernel/stack_tracer_enabled

```
浣犱篃鍙互鍦ㄥ唴鏍稿懡浠よ涓婂惎鐢ㄥ畠锛屼互璺熻釜鍐呮牳鍦ㄥ惎鍔ㄦ湡闂寸殑鏍堝ぇ灏忥紝鏂规硶鏄悜鍐呮牳鍛戒护琛屽弬鏁?
娣诲姞 "stacktrace"銆?

杩愯鍑犲垎閽熷悗锛岃緭鍑哄涓嬶細

```

  # cat stack_max_size
  2928

  # cat stack_trace
          Depth    Size   Location    (18 entries)
          -----    ----   --------
    0)     2928     224   update_sd_lb_stats+0xbc/0x4ac
    1)     2704     160   find_busiest_group+0x31/0x1f1
    2)     2544     256   load_balance+0xd9/0x662
    3)     2288      80   idle_balance+0xbb/0x130
    4)     2208     128   __schedule+0x26e/0x5b9
    5)     2080      16   schedule+0x64/0x66
    6)     2064     128   schedule_timeout+0x34/0xe0
    7)     1936     112   wait_for_common+0x97/0xf1
    8)     1824      16   wait_for_completion+0x1d/0x1f
    9)     1808     128   flush_work+0xfe/0x119
   10)     1680      16   tty_flush_to_ldisc+0x1e/0x20
   11)     1664      48   input_available_p+0x1d/0x5c
   12)     1616      48   n_tty_poll+0x6d/0x134
   13)     1568      64   tty_poll+0x64/0x7f
   14)     1504     880   do_select+0x31e/0x511
   15)      624     400   core_sys_select+0x177/0x216
   16)      224      96   sys_select+0x91/0xb9
   17)      128     128   system_call_fastpath+0x16/0x1b

```
娉ㄦ剰锛屽鏋?gcc 浣跨敤浜?-mfentry锛屽嚱鏁颁細鍦ㄨ缃爤甯т箣鍓嶈璺熻釜銆傝繖鎰忓懗鐫€褰撲娇鐢?-mfentry 鏃讹紝
鍙跺瓙绾у嚱鏁颁笉浼氳鏍堣窡韪櫒娴嬭瘯銆?

鐩墠锛?mfentry 浠呯敱 x86 涓?gcc 4.6.0 鍙婁互涓婄増鏈娇鐢ㄣ€?

### 鏇村


鏇村缁嗚妭鍙互鍦ㄦ簮浠ｇ爜銆乲ernel/trace/*.c 鏂囦欢涓壘鍒般€?
