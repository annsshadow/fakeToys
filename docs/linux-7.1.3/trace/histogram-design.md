## 鐩存柟鍥捐璁¤鏄?



:浣滆€? Tom Zanussi <zanussi@kernel.org>

鏈枃妗ｈ瘯鍥炬弿杩?ftrace 鐩存柟鍥炬槸濡備綍宸ヤ綔鐨勶紝浠ュ強鍚勪釜缁勬垚閮ㄥ垎濡備綍鏄犲皠鍒扮敤浜庡湪 trace_events_hist.c 鍜?tracing_map.c 涓疄鐜板畠浠殑鏁版嵁缁撴瀯銆?

   浠ヤ笅鎵€鏈?ftrace 鐩存柟鍥惧懡浠ょず渚嬮兘鍋囧畾褰撳墠宸ヤ綔鐩綍涓?

```
	# cd /sys/kernel/tracing

   Also, the histogram output displayed for those commands will be generally be truncated - only enough to make the point is displayed.

```

## 'hist_debug' 璺熻釜浜嬩欢鏂囦欢



濡傛灉鍐呮牳缂栬瘧鏃惰缃簡 CONFIG_HIST_TRIGGERS_DEBUG锛屽垯浼氬湪姣忎釜浜嬩欢鐨勫瓙鐩綍涓嚭鐜颁竴涓悕涓?'hist_debug' 鐨勪簨浠舵枃浠躲€傝鏂囦欢鍙殢鏃惰鍙栵紝骞跺皢鏄剧ず鏈枃妗ｆ墍杩扮殑閮ㄥ垎鐩存柟鍥捐Е鍙戝櫒鍐呴儴鐘舵€併€傚叿浣撶殑绀轰緥鍜岃緭鍑哄皢鍦ㄤ笅闈㈢殑娴嬭瘯鐢ㄤ緥涓弿杩般€?

## 鍩虹鐩存柟鍥?



棣栧厛鏄渶鍩虹鐨勭洿鏂瑰浘銆備笅闈㈠嚑涔庢槸浣犵敤鐩存柟鍥捐兘鍋氱殑鏈€绠€鍗曠殑浜嬫儏鈥斺€斿湪鍗曚釜浜嬩欢涓婄敤鍗曚釜閿垱寤轰竴涓洿鏂瑰浘锛?

```
  # echo 'hist:keys=pid' >> events/sched/sched_waking/trigger

  # cat events/sched/sched_waking/hist

  { pid:      18249 } hitcount:          1
  { pid:      13399 } hitcount:          1
  { pid:      17973 } hitcount:          1
  { pid:      12572 } hitcount:          1
  ...
  { pid:         10 } hitcount:        921
  { pid:      18255 } hitcount:       1444
  { pid:      25526 } hitcount:       2055
  { pid:       5257 } hitcount:       2055
  { pid:      27367 } hitcount:       2055
  { pid:       1728 } hitcount:       2161

  Totals:
    Hits: 21305
    Entries: 183
    Dropped: 0

```

杩欐浠ｇ爜鍦?sched_waking 浜嬩欢涓婂垱寤轰簡涓€涓互 pid 涓洪敭銆佷互鍗曚釜鍊?hitcount 涓哄€肩殑鐩存柟鍥俱€俬itcount 鍗充娇娌℃湁琚樉寮忔寚瀹氾紝涔熷缁堝瓨鍦ㄤ簬姣忎釜鐩存柟鍥句箣涓€?

hitcount 鍊兼槸涓€涓瘡涓《锛坆ucket锛夊搴旂殑鍊硷紝浼氬湪璇ラ敭姣忔鍛戒腑鏃惰嚜鍔ㄩ€掑锛屽湪鏈緥涓閿氨鏄?pid銆?

鍥犳鍦ㄨ繖涓洿鏂瑰浘涓紝姣忎釜 pid 閮芥湁涓€涓嫭绔嬬殑妗讹紝姣忎釜妗朵腑鍖呭惈涓€涓搴旂殑鍊硷紝鐢ㄤ簬缁熻璇?pid 璋冪敤 sched_waking 鐨勬鏁般€?

姣忎釜鐩存柟鍥鹃兘鐢变竴涓?hist_data 缁撴瀯浣擄紙struct hist_trigger_data锛夎〃绀恒€?

涓轰簡璺熻釜鐩存柟鍥句腑鐨勬瘡涓敭鍜屽€煎瓧娈碉紝hist_data 缁存姢浜嗕竴涓悕涓?fields[] 鐨勮繖绫诲瓧娈垫暟缁勩€俧ields[] 鏁扮粍鏄竴涓寘鍚瘡涓洿鏂瑰浘閿拰鍊硷紙杩樺寘鎷彉閲忥紝绋嶅悗璁ㄨ锛夋墍瀵瑰簲鐨?struct hist_field 琛ㄧず鐨勬暟缁勩€傛墍浠ュ浜庝笂闈㈢殑鐩存柟鍥撅紝鎴戜滑鏈変竴涓敭鍜屼竴涓€硷紱鍦ㄦ湰渚嬩腑锛岃繖涓€涓€兼槸 hitcount 鍊硷紝鎵€鏈夌洿鏂瑰浘閮芥嫢鏈夊畠锛屾棤璁哄畠浠槸鍚﹀畾涔変簡璇ュ€硷紝鑰屼笂闈㈢殑鐩存柟鍥惧苟娌℃湁瀹氫箟瀹冦€?

姣忎釜 struct hist_field 閮藉寘鍚竴涓寚鍚戜簨浠?trace_event_file 涓?ftrace_event_field 鐨勬寚閽堬紝浠ュ強涓庝箣鐩稿叧鐨勫悇绫讳俊鎭紝濡傚ぇ灏忋€佸亸绉汇€佺被鍨嬶紝杩樻湁涓€涓?hist field 鍑芥暟锛岀敤浜庝粠 ftrace 浜嬩欢缂撳啿鍖轰腑鍙栧嚭璇ュ瓧娈电殑鏁版嵁锛堝ぇ澶氭暟鎯呭喌涓嬪姝も€斺€旀湁浜?hist_field 姣斿 hitcount 骞朵笉鐩存帴鏄犲皠鍒拌窡韪紦鍐插尯涓殑浜嬩欢瀛楁锛屽湪杩欎簺鎯呭喌涓嬶紝鍏跺嚱鏁板疄鐜颁粠鍒鍙栧緱鍊硷級銆俧lags 瀛楁鎸囩ず璇ュ瓧娈靛睘浜庡摢绉嶇被鍨嬧€斺€旈敭銆佸€笺€佸彉閲忋€佸彉閲忓紩鐢ㄧ瓑锛岄粯璁ゆ槸鍊笺€?

闄や簡 fields[] 鏁扮粍涔嬪锛屽彟涓€涓噸瑕佺殑 hist_data 鏁版嵁缁撴瀯鏄负璇ョ洿鏂瑰浘鍒涘缓鐨?tracing_map 瀹炰緥锛屽畠淇濆瓨鍦?.map 鎴愬憳涓€倀racing_map 瀹炵幇浜嗙敤浜庡疄鐜扮洿鏂瑰浘鐨勫厤閿佸搱甯岃〃锛堝叧浜庡疄鐜?tracing_map 鐨勫簳灞傛暟鎹粨鏋勶紝璇峰弬闃?kernel/trace/tracing_map.h 涓殑澶ч噺璁ㄨ锛夈€傚氨鏈璁鸿€岃█锛宼racing_map 鍖呭惈鑻ュ共涓《锛屾瘡涓《瀵瑰簲涓€涓敱缁欏畾鐩存柟鍥鹃敭鍝堝笇寰楀埌鐨?tracing_map_elt 瀵硅薄銆?

涓嬮潰鏄竴寮犲浘锛屽叾绗竴閮ㄥ垎鎻忚堪浜嗕笂杩扮洿鏂瑰浘鐨?hist_data 浠ュ強鐩稿叧鐨勯敭鍜屽€煎瓧娈点€傛濡備綘鎵€鐪嬪埌鐨勶紝fields 鏁扮粍涓湁涓や釜瀛楁锛屼竴涓槸 hitcount 鐨?val 瀛楁锛屽彟涓€涓槸 pid 閿殑 key 瀛楁銆?

涓嬮潰鏄 tracing_map 鍦ㄦ煇涓繍琛屾椂鍒诲彲鑳藉憟鐜扮殑蹇収鍥俱€傚畠璇曞浘灞曠ず hist_data 瀛楁涓?tracing_map 涔嬮棿鐨勫叧绯伙細

```
  +------------------+
  | hist_data        |
  +------------------+     +----------------+
    | .fields[]      |---->| val = hitcount |----------------------------+
    +----------------+     +----------------+                            |
    | .map           |       | .size        |                            |
    +----------------+       +--------------+                            |
                             | .offset      |                            |
                             +--------------+                            |
                             | .fn()        |                            |
                             +--------------+                            |
                                   .                                     |
                                   .                                     |
                                   .                                     |
                           +----------------+ <--- n_vals                |
                           | key = pid      |----------------------------|--+
                           +----------------+                            |  |
                             | .size        |                            |  |
                             +--------------+                            |  |
                             | .offset      |                            |  |
                             +--------------+                            |  |
                             | .fn()        |                            |  |
                           +----------------+ <--- n_fields              |  |
                           | unused         |                            |  |
                           +----------------+                            |  |
                             |              |                            |  |
                             +--------------+                            |  |
                             |              |                            |  |
                             +--------------+                            |  |
                             |              |                            |  |
                             +--------------+                            |  |
                                            n_keys = n_fields - n_vals   |  |

```

hist_data 鐨?n_vals 鍜?n_fields 鍒掑畾浜?fields[] 鏁扮粍鐨勮寖鍥达紝骞舵妸閿拰鍊间粠浠ｇ爜鍏朵綑閮ㄥ垎涓垎绂诲嚭鏉ャ€?

涓嬮潰鏄竴涓繍琛屾椂鍒荤殑 tracing_map 閮ㄥ垎琛ㄧず鍥撅紝灞曠ず浜嗕粠 fields[] 鏁扮粍鐨勫悇涓儴鍒嗘寚鍚?tracing_map 瀵瑰簲閮ㄥ垎鐨勬寚閽堛€?

tracing_map 鐢变竴涓?tracing_map_entry 鏁扮粍鍜屼竴缁勯鍒嗛厤鐨?tracing_map_elt锛堜笅鍥剧畝鍐欎负 map_entry 鍜?map_elt锛夌粍鎴愩€俬ist_data.map 鏁扮粍涓殑 map_entry 鎬绘暟 = map->max_elts锛堝疄闄呬笂鏄?map->map_size锛屼絾鍏朵腑鍙湁 max_elts 涓浣跨敤銆傝繖鏄?map_insert() 绠楁硶鎵€闇€鐨勪竴涓睘鎬э級銆?

濡傛灉涓€涓?map_entry 鏈浣跨敤锛屽嵆杩樻病鏈夐敭鍝堝笇鍒板畠锛屽垯瀹冪殑 .key 鍊间负 0锛屽叾 .val 鎸囬拡涓?NULL銆備竴鏃︽煇涓?map_entry 琚崰鐢紝.key 鍊煎氨鍖呭惈璇ラ敭鐨勫搱甯屽€硷紝鑰?.val 鎴愬憳鎸囧悜涓€涓?map_elt锛屽叾涓寘鍚畬鏁寸殑閿互鍙?map_elt.fields[] 鏁扮粍涓瘡涓敭鎴栧€煎搴旂殑涓€涓潯鐩€俶ap_elt.fields[] 鏁扮粍涓湁涓€涓潯鐩搴斾簬鐩存柟鍥句腑鐨勬瘡涓?hist_field锛岃€屾瘡涓洿鏂瑰浘鍊兼墍瀵瑰簲鐨勩€佹寔缁仛鍚堢殑姹傚拰鍊煎氨淇濆瓨鍦ㄨ繖閲屻€?

璇ュ浘璇曞浘灞曠ず hist_data.fields[] 涓?map_elt.fields[] 涔嬮棿鐨勫叧绯伙紝鍥句腑鐢ㄨ繛绾跨粯鍒朵簡杩欑鍏宠仈锛?

```
  +-----------+		                                                 |  |
  | hist_data |		                                                 |  |
  +-----------+		                                                 |  |
    | .fields |		                                                 |  |
    +---------+     +-----------+		                         |  |
    | .map    |---->| map_entry |		                         |  |
    +---------+     +-----------+		                         |  |
                      | .key    |---> 0		                         |  |
                      +---------+		                         |  |
                      | .val    |---> NULL		                 |  |
                    +-----------+                                        |  |
                    | map_entry |                                        |  |
                    +-----------+                                        |  |
                      | .key    |---> pid = 999                          |  |
                      +---------+    +-----------+                       |  |
                      | .val    |--->| map_elt   |                       |  |
                      +---------+    +-----------+                       |  |
                           .           | .key    |---> full key *        |  |
                           .           +---------+    +---------------+  |  |
			   .           | .fields |--->| .sum (val)    |<-+  |
                    +-----------+      +---------+    | 2345          |  |  |
                    | map_entry |                     +---------------+  |  |
                    +-----------+                     | .offset (key) |<----+
                      | .key    |---> 0               | 0             |  |  |
                      +---------+                     +---------------+  |  |
                      | .val    |---> NULL                    .          |  |
                    +-----------+                             .          |  |
                    | map_entry |                             .          |  |
                    +-----------+                     +---------------+  |  |
                      | .key    |                     | .sum (val) or |  |  |
                      +---------+    +---------+      | .offset (key) |  |  |
                      | .val    |--->| map_elt |      +---------------+  |  |
                    +-----------+    +---------+      | .sum (val) or |  |  |
                    | map_entry |                     | .offset (key) |  |  |
                    +-----------+                     +---------------+  |  |
                      | .key    |---> pid = 4444                         |  |
                      +---------+    +-----------+                       |  |
                      | .val    |    | map_elt   |                       |  |
                      +---------+    +-----------+                       |  |
                                       | .key    |---> full key *        |  |
                                       +---------+    +---------------+  |  |
			               | .fields |--->| .sum (val)    |<-+  |
                                       +---------+    | 65523         |     |
                                                      +---------------+     |
                                                      | .offset (key) |<----+
                                                      | 0             |
                                                      +---------------+
                                                              .
                                                              .
                                                              .
                                                      +---------------+
                                                      | .sum (val) or |
                                                      | .offset (key) |
                                                      +---------------+
                                                      | .sum (val) or |
                                                      | .offset (key) |
                                                      +---------------+

```

```
  hist_data = struct hist_trigger_data
  hist_data.fields = struct hist_field
  fn = hist_field_fn_t
  map_entry = struct tracing_map_entry
  map_elt = struct tracing_map_elt
  map_elt.fields = struct tracing_map_field

```

姣忓綋鍙戠敓涓€涓柊浜嬩欢骞朵笖瀹冨叧鑱斾簡涓€涓?hist 瑙﹀彂鍣ㄦ椂锛屽氨浼氳皟鐢?event_hist_trigger()銆俥vent_hist_trigger() 棣栧厛澶勭悊閿細瀵逛簬閿腑鐨勬瘡涓瓙閿紙鍦ㄤ笂闈㈢殑渚嬪瓙涓紝鍙湁涓€涓搴斾簬 pid 鐨勫瓙閿級锛屼細浠?hist_data.fields[] 涓彇鍑鸿〃绀鸿瀛愰敭鐨?hist_field锛屽苟鍒╃敤涓庤瀛楁鍏宠仈鐨?hist field 鍑芥暟锛屼互鍙婂瓧娈电殑澶у皬鍜屽亸绉伙紝浠庡綋鍓嶈窡韪褰曚腑鍙栧嚭璇ュ瓙閿殑鏁版嵁銆?

娉ㄦ剰锛宧ist field 鍑芥暟鏇剧粡鏄?hist_field 缁撴瀯涓殑涓€涓嚱鏁版寚閽堛€傜敱浜庨拡瀵?Spectre 鐨勭紦瑙ｆ帾鏂斤紝瀹冭鏀规垚浜?fn_num锛屽苟涓斾娇鐢?hist_fn_call() 鏉ヨ皟鐢ㄥ搴斾簬 hist_field 缁撴瀯鐨?fn_num 鐨?hist field 鍑芥暟銆?

涓€鏃﹀彇鍥炲畬鏁寸殑閿紝灏辩敤瀹冨埌 tracing_map 涓煡鎵捐閿€傚鏋滄病鏈変笌璇ラ敭鍏宠仈鐨?tracing_map_elt锛屽氨浼氱敵璇蜂竴涓┖鐨勫苟鎻掑叆鍒版槧灏勪腑渚涙柊閿娇鐢ㄣ€傛棤璁哄摢绉嶆儏鍐碉紝閮戒細杩斿洖涓庤閿叧鑱旂殑 tracing_map_elt銆?

涓€鏃﹁幏寰椾簡 tracing_map_elt锛屽氨浼氳皟鐢?hist_trigger_elt_update()銆傞【鍚嶆€濅箟锛屽畠鏇存柊璇ュ厓绱狅紝杩欏熀鏈笂鎰忓懗鐫€鏇存柊璇ュ厓绱犵殑瀛楁銆傜洿鏂瑰浘涓殑姣忎釜閿拰鍊奸兘鍏宠仈鐫€涓€涓?tracing_map_field锛屽畠浠悇鑷搴斾簬鍒涘缓鐩存柟鍥炬椂鎵€鍒涘缓鐨勯敭鍜屽€?hist_field銆俬ist_trigger_elt_update() 閬嶅巻姣忎釜鍊?hist_field锛屽苟鍍忓鐞嗛敭閭ｆ牱锛屽埄鐢?hist_field 鐨勫嚱鏁般€佸ぇ灏忓拰鍋忕Щ浠庡綋鍓嶈窡韪褰曚腑鍙栧嚭璇ュ瓧娈电殑鍊笺€備竴鏃﹀彇鍒拌鍊硷紝瀹冨氨绠€鍗曞湴鎶婅繖涓€煎姞鍒拌瀛楁鎸佺画鏇存柊鐨?tracing_map_field.sum 鎴愬憳涓娿€傛湁浜?hist_field 鍑芥暟锛屾瘮濡?hitcount锛屽疄闄呬笂骞朵笉浠庤窡韪褰曚腑鍙栦换浣曚笢瑗匡紙hitcount 鍑芥暟鍙槸鎶婅鏁板櫒 sum 鍔?1锛夛紝浣嗘€濊矾鏄竴鏍风殑銆?

涓€鏃︽墍鏈夊€奸兘琚洿鏂帮紝hist_trigger_elt_update() 灏卞畬鎴愬苟杩斿洖銆傛敞鎰忥紝閿腑鐨勬瘡涓瓙閿篃鏈夊搴旂殑 tracing_map_field锛屼絾 hist_trigger_elt_update() 骞朵笉浼氭煡鐪嬫垨鏇存柊瀹冧滑鈥斺€斿畠浠彧鐢ㄤ簬鎺掑簭锛岃€岃繖鍙互绋嶅悗杩涜銆?

### 鍩虹鐩存柟鍥炬祴璇?



杩欐槸涓€涓€煎緱灏濊瘯鐨勫ソ渚嬪瓙銆傚畠浜х敓 3 涓€煎瓧娈靛拰 2 涓敭锛?

```
  # echo 'hist:keys=common_pid,call_site.sym:values=bytes_req,bytes_alloc,hitcount' >> events/kmem/kmalloc/trigger

```

瑕佹煡鐪嬭皟璇曟暟鎹紝鍙互 cat 涓€涓?kmem/kmalloc 鐨?'hist_debug' 鏂囦欢銆傚畠浼氭樉绀鸿鐩存柟鍥惧搴旂殑瑙﹀彂鍣ㄤ俊鎭紝浠ュ強涓庤鐩存柟鍥惧叧鑱旂殑 hist_data 鐨勫湴鍧€锛岃繖鍦ㄥ悗闈㈢殑渚嬪瓙涓細寰堟湁鐢ㄣ€傞殢鍚庡畠浼氭樉绀轰笌璇ョ洿鏂瑰浘鍏宠仈鐨勬€?hist_field 鏁伴噺锛屼互鍙婂叾涓湁澶氬皯瀵瑰簲浜庨敭銆佸灏戝搴斾簬鍊笺€?

鎺ョ潃瀹冧細鏄剧ず姣忎釜瀛楁鐨勮缁嗕俊鎭紝鍖呮嫭璇ュ瓧娈电殑 flags锛屼互鍙婃瘡涓瓧娈靛湪 hist_data 鐨?fields[] 鏁扮粍涓殑浣嶇疆锛岃繖浜涗俊鎭浜庨獙璇佸唴閮ㄧ姸鎬佹槸鍚︽纭潪甯告湁鐢紝骞朵笖鍚屾牱浼氬湪鍚庨潰鍙樺緱锛?

```
  # cat events/kmem/kmalloc/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=common_pid,call_site.sym:vals=hitcount,bytes_req,bytes_alloc:sort=hitcount:size=2048 [active]
  #

  hist_data: 000000005e48c9a5

  n_vals: 3
  n_keys: 2
  n_fields: 5

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        VAL: normal u64 value
      ftrace_event_field name: bytes_req
      type: size_t
      size: 8
      is_signed: 0

    hist_data->fields[2]:
      flags:
        VAL: normal u64 value
      ftrace_event_field name: bytes_alloc
      type: size_t
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[3]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: common_pid
      type: int
      size: 8
      is_signed: 1

    hist_data->fields[4]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: call_site
      type: unsigned long
      size: 8
      is_signed: 0

```

```
  # echo '!hist:keys=common_pid,call_site.sym:values=bytes_req,bytes_alloc,hitcount' >> events/kmem/kmalloc/trigger

```

## 鍙橀噺



鍙橀噺鍏佽涓€涓洿鏂瑰浘瑙﹀彂鍣ㄤ繚瀛樼殑鏁版嵁琚彟涓€涓洿鏂瑰浘瑙﹀彂鍣ㄨ幏鍙栥€備緥濡傦紝sched_waking 浜嬩欢涓婄殑瑙﹀彂鍣ㄥ彲浠ユ崟鑾锋煇涓壒瀹?pid 鐨勬椂闂存埑锛岀◢鍚庡垏鎹㈠埌璇?pid 鐨?sched_switch 浜嬩欢鍙互鑾峰彇璇ユ椂闂存埑骞剁敤瀹冩潵璁＄畻鏃堕棿宸細

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >>
          events/sched/sched_waking/trigger

  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0' >>
          events/sched/sched_switch/trigger

```

灏辩洿鏂瑰浘鏁版嵁缁撴瀯鑰岃█锛屽彉閲忚瀹炵幇涓哄彟涓€绉嶇被鍨嬬殑 hist_field锛屽浜庣粰瀹氱殑 hist 瑙﹀彂鍣紝瀹冧滑琚坊鍔犲埌鎵€鏈?val 瀛楁涔嬪悗鐨?hist_data.fields[] 鏁扮粍涓€備负浜嗘妸瀹冧滑涓庡凡鏈夌殑閿拰鍊煎瓧娈靛尯鍒嗗紑锛岀粰瀹冧滑璧嬩簣浜嗕竴绉嶆柊鐨勬爣蹇楃被鍨?HIST_FIELD_FL_VAR锛堢畝鍐欎负 FL_VAR锛夛紝骞朵笖瀹冧滑杩樺埄鐢ㄤ簡 struct hist_field 涓竴涓柊鐨?.var.idx 瀛楁鎴愬憳锛岃鎴愬憳灏嗗彉閲忔槧灏勫埌涓€涓笓闂ㄦ柊澧炵殑銆佺敤浜庡瓨鍌ㄥ拰鑾峰彇鍙橀噺鍊肩殑 map_elt.vars[] 鏁扮粍鐨勬煇涓储寮曘€備笅闈㈢殑鍥惧睍绀轰簡杩欎簺鏂板厓绱狅紝骞舵柊澧炰簡涓€涓搴斾簬涓婇潰 sched_waking 瑙﹀彂鍣ㄤ腑 ts0 鍙橀噺鐨勬柊鍙橀噺鏉＄洰 ts0銆?

### sched_waking 鐩存柟鍥?



  +------------------+
  | hist_data        |<-------------------------------------------------------+
  +------------------+   +-------------------+                                |
    | .fields[]      |-->| val = hitcount    |                                |
    +----------------+   +-------------------+                                |
    | .map           |     | .size           |                                |
    +----------------+     +-----------------+                                |
                           | .offset         |                                |
                           +-----------------+                                |
                           | .fn()           |                                |
                           +-----------------+                                |
                           | .flags          |                                |
                           +-----------------+                                |
                           | .var.idx        |                                |
                         +-------------------+                                |
                         | var = ts0         |                                |
                         +-------------------+                                |
                           | .size           |                                |
                           +-----------------+                                |
                           | .offset         |                                |
                           +-----------------+                                |
                           | .fn()           |                                |
                           +-----------------+                                |
                           | .flags & FL_VAR |                                |
                           +-----------------+                                |
                           | .var.idx        |----------------------------+-+ |
                           +-----------------+                            | | |
			            .                                     | | |
				    .                                     | | |
                                    .                                     | | |
                         +-------------------+ <--- n_vals                | | |
                         | key = pid         |                            | | |
                         +-------------------+                            | | |
                           | .size           |                            | | |
                           +-----------------+                            | | |
                           | .offset         |                            | | |
                           +-----------------+                            | | |
                           | .fn()           |                            | | |
                           +-----------------+                            | | |
                           | .flags & FL_KEY |                            | | |
                           +-----------------+                            | | |
                           | .var.idx        |                            | | |
                         +-------------------+ <--- n_fields              | | |
                         | unused            |                            | | |
                         +-------------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                                             n_keys = n_fields - n_vals   | | |
                                                                          | | |

杩欎笌鍩虹鎯呭舰闈炲父鐩镐技銆傚湪涓婂浘涓紝鎴戜滑鍙互鐪嬪埌 struct hist_field 缁撴瀯鏂板浜嗕竴涓?.flags 鎴愬憳锛屽苟涓?hist_data.fields 涓柊澧炰簡涓€涓〃绀?ts0 鍙橀噺鐨勬潯鐩€傚浜庝竴涓櫘閫氱殑 val hist_field锛?flags 鍙槸 0锛堜慨楗扮鏍囧織闄ゅ锛夛紝浣嗗鏋滆鍊艰瀹氫箟涓哄彉閲忥紝鍒?.flags 浼氬寘鍚竴缁?FL_VAR 浣嶃€?

濡備綘鎵€瑙侊紝ts0 鏉＄洰鐨?.var.idx 鎴愬憳鍖呭惈浜嗘寚鍚戜繚瀛樺彉閲忓€肩殑 tracing_map_elt 鐨?.vars[] 鏁扮粍鐨勭储寮曘€傛瘡褰撹缃垨璇诲彇璇ュ彉閲忕殑鍊兼椂閮戒細鐢ㄥ埌杩欎釜 idx銆傚垎閰嶇粰缁欏畾鍙橀噺鐨?map_elt.vars 绱㈠紩锛岀敱 create_tracing_map_fields() 鍦ㄨ皟鐢?tracing_map_add_var() 涔嬪悗璧嬪€煎苟淇濆瓨鍦?.var.idx 涓€?

涓嬮潰鏄竴涓繍琛屾椂鍒荤殑鐩存柟鍥捐〃绀哄浘锛屽畠濉厖浜嗘槧灏勶紝骞朵笌涓婇潰鐨?hist_data 鍜?hist_field 鏁版嵁缁撴瀯鐩稿搴斻€?

璇ュ浘璇曞浘灞曠ず hist_data.fields[] 涓?map_elt.fields[] 浠ュ強 map_elt.vars[] 涔嬮棿鐨勫叧绯伙紝鍥句腑鍦ㄤ袱涓浘涔嬮棿缁樺埗浜嗚繛绾裤€傚浜庢瘡涓?map_elt锛屼綘鍙互鐪嬪埌 .fields[] 鎴愬憳鎸囧悜鏌愪釜閿垨鍊肩殑 .sum 鎴?.offset锛岃€?.vars[] 鎴愬憳鎸囧悜鏌愪釜鍙橀噺鐨勫€笺€備袱鍥句箣闂寸殑绠ご灞曠ず浜嗚繖浜?tracing_map 鎴愬憳涓庣浉搴斿瓧娈靛畾涔変箣闂寸殑鍏宠仈锛?

```
  +-----------+		                                                  | | |
  | hist_data |		                                                  | | |
  +-----------+		                                                  | | |
    | .fields |		                                                  | | |
    +---------+     +-----------+		                          | | |
    | .map    |---->| map_entry |		                          | | |
    +---------+     +-----------+		                          | | |
                      | .key    |---> 0		                          | | |
                      +---------+		                          | | |
                      | .val    |---> NULL		                  | | |
                    +-----------+                                         | | |
                    | map_entry |                                         | | |
                    +-----------+                                         | | |
                      | .key    |---> pid = 999                           | | |
                      +---------+    +-----------+                        | | |
                      | .val    |--->| map_elt   |                        | | |
                      +---------+    +-----------+                        | | |
                           .           | .key    |---> full key *         | | |
                           .           +---------+    +---------------+   | | |
			   .           | .fields |--->| .sum (val)    |   | | |
                           .           +---------+    | 2345          |   | | |
                           .        +--| .vars   |    +---------------+   | | |
                           .        |  +---------+    | .offset (key) |   | | |
                           .        |                 | 0             |   | | |
                           .        |                 +---------------+   | | |
                           .        |                         .           | | |
                           .        |                         .           | | |
                           .        |                         .           | | |
                           .        |                 +---------------+   | | |
                           .        |                 | .sum (val) or |   | | |
                           .        |                 | .offset (key) |   | | |
                           .        |                 +---------------+   | | |
                           .        |                 | .sum (val) or |   | | |
                           .        |                 | .offset (key) |   | | |
                           .        |                 +---------------+   | | |
                           .        |                                     | | |
                           .        +---------------->+---------------+   | | |
			   .                          | ts0           |<--+ | |
                           .                          | 113345679876  |   | | |
                           .                          +---------------+   | | |
                           .                          | unused        |   | | |
                           .                          |               |   | | |
                           .                          +---------------+   | | |
                           .                                  .           | | |
                           .                                  .           | | |
                           .                                  .           | | |
                           .                          +---------------+   | | |
                           .                          | unused        |   | | |
                           .                          |               |   | | |
                           .                          +---------------+   | | |
                           .                          | unused        |   | | |
                           .                          |               |   | | |
                           .                          +---------------+   | | |
                           .                                              | | |
                    +-----------+                                         | | |
                    | map_entry |                                         | | |
                    +-----------+                                         | | |
                      | .key    |---> pid = 4444                          | | |
                      +---------+    +-----------+                        | | |
                      | .val    |--->| map_elt   |                        | | |
                      +---------+    +-----------+                        | | |
                           .           | .key    |---> full key *         | | |
                           .           +---------+    +---------------+   | | |
			   .           | .fields |--->| .sum (val)    |   | | |
                                       +---------+    | 2345          |   | | |
                                    +--| .vars   |    +---------------+   | | |
                                    |  +---------+    | .offset (key) |   | | |
                                    |                 | 0             |   | | |
                                    |                 +---------------+   | | |
                                    |                         .           | | |
                                    |                         .           | | |
                                    |                         .           | | |
                                    |                 +---------------+   | | |
                                    |                 | .sum (val) or |   | | |
                                    |                 | .offset (key) |   | | |
                                    |                 +---------------+   | | |
                                    |                 | .sum (val) or |   | | |
                                    |                 | .offset (key) |   | | |
                                    |                 +---------------+   | | |
                                    |                                     | | |
                                    |                 +---------------+   | | |
			            +---------------->| ts0           |<--+ | |
                                                      | 213499240729  |     | |
                                                      +---------------+     | |
                                                      | unused        |     | |
                                                      |               |     | |
                                                      +---------------+     | |
                                                              .             | |
                                                              .             | |
                                                              .             | |
                                                      +---------------+     | |
                                                      | unused        |     | |
                                                      |               |     | |
                                                      +---------------+     | |
                                                      | unused        |     | |
                                                      |               |     | |
                                                      +---------------+     | |

```

瀵逛簬姣忎釜宸蹭娇鐢ㄧ殑鏄犲皠鏉＄洰锛岄兘鏈変竴涓?map_elt 鎸囧悜涓€涓寘鍚笌璇ョ洿鏂瑰浘鏉＄洰鍏宠仈鐨勫彉閲忓綋鍓嶅€肩殑 .vars 鏁扮粍銆傛墍浠ュ湪涓婇潰锛屼笌 pid 999 鍏宠仈鐨勬椂闂存埑鏄?113345679876锛岃€屽湪 pid 4444 鐨勫悓涓€涓?.var.idx 涓殑鏃堕棿鎴冲彉閲忔槸 213499240729銆?

### sched_switch 鐩存柟鍥?



涓婇潰 sched_waking 鐩存柟鍥炬墍閰嶅鐨?sched_switch 鐩存柟鍥惧涓嬫墍绀恒€俿ched_switch 鐩存柟鍥炬渶閲嶈鐨勬柟闈㈠湪浜庡畠寮曠敤浜嗕笂闈?sched_waking 鐩存柟鍥句腑鐨勪竴涓彉閲忋€?

杩欎釜鐩存柟鍥惧浘涓庡埌鐩墠涓烘灞曠ず鐨勫叾浠栧浘闈炲父鐩镐技锛屼絾瀹冨鍔犱簡鍙橀噺寮曠敤銆備綘鍙互鐪嬪埌鏅€氱殑 hitcount 鍜岄敭瀛楁锛屽鍔犱竴涓敤涓?sched_waking ts0 鍙橀噺鐩稿悓鏂瑰紡瀹炵幇鐨勬柊鐨?wakeup_lat 鍙橀噺锛屼絾闄ゆ涔嬪杩樻湁涓€涓甫鏈夋柊 FL_VAR_REF锛圚IST_FIELD_FL_VAR_REF 鐨勭畝鍐欙級鏍囧織鐨勬潯鐩€?

涓庢柊鐨勫彉閲忓紩鐢ㄥ瓧娈电浉鍏宠仈鐨勮繕鏈夊嚑涓柊鐨?hist_field 鎴愬憳锛歷ar.hist_data 鍜?var_ref_idx銆傚浜庝竴涓彉閲忓紩鐢紝var.hist_data 涓?var.idx 閰嶅悎浣跨敤锛屼簩鑰呭叡鍚屽敮涓€鏍囪瘑鏌愪釜鐗瑰畾鐩存柟鍥句笂鐨勪竴涓壒瀹氬彉閲忋€倂ar_ref_idx 鍙槸鐢ㄤ簬缂撳瓨姣忎釜鍙橀噺鍊肩殑 var_ref_vals[] 鏁扮粍鐨勭储寮曪紝姣忓綋鏌愪釜 hist 瑙﹀彂鍣ㄨ鏇存柊鏃堕兘浼氱紦瀛樸€傝繖浜涚粨鏋滃€奸殢鍚庤鍏朵粬浠ｇ爜锛堜緥濡備娇鐢?var_ref_idx 鍊兼潵璧嬪弬鐨?trace action 浠ｇ爜锛夋渶缁堣闂€?

涓嬮潰鐨勫浘鎻忚堪浜?sched_switch 鐨勬儏褰細

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0' >>
          events/sched/sched_switch/trigger

                                                                            | |
  +------------------+                                                      | |
  | hist_data        |                                                      | |
  +------------------+   +-----------------------+                          | |
    | .fields[]      |-->| val = hitcount        |                          | |
    +----------------+   +-----------------------+                          | |
    | .map           |     | .size               |                          | |
    +----------------+     +---------------------+                          | |
 +--| .var_refs[]    |     | .offset             |                          | |
 |  +----------------+     +---------------------+                          | |
 |                         | .fn()               |                          | |
 |   var_ref_vals[]        +---------------------+                          | |
 |  +-------------+        | .flags              |                          | |
 |  | $ts0        |<---+   +---------------------+                          | |
 |  +-------------+    |   | .var.idx            |                          | |
 |  |             |    |   +---------------------+                          | |
 |  +-------------+    |   | .var.hist_data      |                          | |
 |  |             |    |   +---------------------+                          | |
 |  +-------------+    |   | .var_ref_idx        |                          | |
 |  |             |    | +-----------------------+                          | |
 |  +-------------+    | | var = wakeup_lat      |                          | |
 |         .           | +-----------------------+                          | |
 |         .           |   | .size               |                          | |
 |         .           |   +---------------------+                          | |
 |  +-------------+    |   | .offset             |                          | |
 |  |             |    |   +---------------------+                          | |
 |  +-------------+    |   | .fn()               |                          | |
 |  |             |    |   +---------------------+                          | |
 |  +-------------+    |   | .flags & FL_VAR     |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .var.idx            |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .var.hist_data      |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .var_ref_idx        |                          | |
 |                     |   +---------------------+                          | |
 |                     |             .                                      | |
 |                     |             .                                      | |
 |                     |             .                                      | |
 |                     | +-----------------------+ <--- n_vals              | |
 |                     | | key = pid             |                          | |
 |                     | +-----------------------+                          | |
 |                     |   | .size               |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .offset             |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .fn()               |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .flags              |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .var.idx            |                          | |
 |                     | +-----------------------+ <--- n_fields            | |
 |                     | | unused                |                          | |
 |                     | +-----------------------+                          | |
 |                     |   |                     |                          | |
 |                     |   +---------------------+                          | |
 |                     |   |                     |                          | |
 |                     |   +---------------------+                          | |
 |                     |   |                     |                          | |
 |                     |   +---------------------+                          | |
 |                     |   |                     |                          | |
 |                     |   +---------------------+                          | |
 |                     |   |                     |                          | |
 |                     |   +---------------------+                          | |
 |                     |                         n_keys = n_fields - n_vals | |
 |                     |                                                    | |
 |                     |						    | |
 |                     | +-----------------------+                          | |
 +---------------------->| var_ref = $ts0        |                          | |
                       | +-----------------------+                          | |
                       |   | .size               |                          | |
                       |   +---------------------+                          | |
                       |   | .offset             |                          | |
                       |   +---------------------+                          | |
                       |   | .fn()               |                          | |
                       |   +---------------------+                          | |
                       |   | .flags & FL_VAR_REF |                          | |
                       |   +---------------------+                          | |
                       |   | .var.idx            |--------------------------+ |
                       |   +---------------------+                            |
                       |   | .var.hist_data      |----------------------------+
                       |   +---------------------+
                       +---| .var_ref_idx        |
                           +---------------------+

```

```
  hist_data = struct hist_trigger_data
  hist_data.fields = struct hist_field
  fn = hist_field_fn_t
  FL_KEY = HIST_FIELD_FL_KEY
  FL_VAR = HIST_FIELD_FL_VAR
  FL_VAR_REF = HIST_FIELD_FL_VAR_REF

```

褰撲竴涓洿鏂瑰浘瑙﹀彂鍣ㄤ娇鐢ㄤ簡涓€涓彉閲忔椂锛屽氨浼氬垱寤轰竴涓甫鏈?HIST_FIELD_FL_VAR_REF 鏍囧織鐨勬柊 hist_field銆傚浜庝竴涓?VAR_REF 瀛楁锛屽叾 var.idx 鍜?var.hist_data 鍙栦笌鎵€寮曠敤鍙橀噺鐩稿悓鐨勫€硷紝鍚屾椂涔熷寘鎷墍寮曠敤鍙橀噺鐨?size銆乼ype 鍜?is_signed 鍊笺€俈AR_REF 瀛楁鐨?.name 琚缃负瀹冩墍寮曠敤鍙橀噺鐨勫悕瀛椼€傚鏋滃彉閲忓紩鐢ㄦ槸浣跨敤鏄惧紡鐨?system.event.$var_ref 璁板彿鍒涘缓鐨勶紝閭ｄ箞璇?hist_field 鐨?system 鍜?event_name 鍙橀噺涔熶細琚缃€?

鍥犳锛屼负浜嗗鐞?sched_switch 鐩存柟鍥剧殑涓€涓簨浠讹紝鍥犱负鎴戜滑寮曠敤浜嗗彟涓€涓洿鏂瑰浘涓婄殑涓€涓彉閲忥紝鎵€浠ラ渶瑕佸厛瑙ｆ瀽鎵€鏈夌殑鍙橀噺寮曠敤銆傝繖鏄€氳繃浠?event_hist_trigger() 鍙戣捣鐨?resolve_var_refs() 璋冪敤瀹屾垚鐨勩€傚畠鐨勪綔鐢ㄦ槸鍙栧嚭琛ㄧず sched_switch 鐩存柟鍥剧殑 hist_data 涓殑 var_refs[] 鏁扮粍銆傚浜庡叾涓殑姣忎竴涓紝閮戒細鍒╃敤鎵€寮曠敤鍙橀噺鐨?var.hist_data 浠ュ強褰撳墠閿紝鍒伴偅涓洿鏂瑰浘涓煡鎵惧搴旂殑 tracing_map_elt銆備竴鏃︽壘鍒帮紝灏辩敤鎵€寮曠敤鍙橀噺鐨?var.idx锛岄€氳繃 tracing_map_read_var(elt, var.idx) 鏌ユ壘璇ュ彉閲忕殑鍊硷紝浠庤€屽緱鍒拌鍏冪礌瀵瑰簲鐨勫彉閲忓€硷紝鍦ㄤ笂闈㈣繖涓緥瀛愪腑灏辨槸 ts0銆傛敞鎰忥紝琛ㄧず鍙橀噺鍙婂叾寮曠敤鐨勪袱涓?hist_field 鎷ユ湁鐩稿悓鐨?var.idx锛屾墍浠ヨ繖涓繃绋嬫槸鐩存帴鐨勩€?

### 鍙橀噺涓庡彉閲忓紩鐢ㄦ祴璇?



杩欎釜渚嬪瓙鍦?sched_waking 浜嬩欢涓婂垱寤轰竴涓彉閲?ts0锛屽苟鍦?sched_switch 瑙﹀彂鍣ㄤ腑鍔犱互浣跨敤銆俿ched_switch 瑙﹀彂鍣ㄨ繕锛?

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0' >> events/sched/sched_switch/trigger

```

瑙傚療 sched_waking 鐨?'hist_debug' 杈撳嚭锛岄櫎浜嗘櫘閫氱殑閿拰鍊?hist_field 涔嬪锛屽湪 val fields 鑺備腑鎴戜滑鍙互鐪嬪埌涓€涓甫鏈?HIST_FIELD_FL_VAR 鏍囧織鐨勫瓧娈碉紝杩欒〃鏄庤瀛楁琛ㄧず涓€涓彉閲忋€傛敞鎰忥紝闄や簡鍖呭惈鍦?var.name 瀛楁涓殑鍙橀噺鍚嶄箣澶栵紝瀹冭繕鍖呭惈 var.idx锛屽嵆鎸囧悜淇濆瓨璇ュ彉閲忓疄闄呬綅缃殑 tracing_map_elt.vars[] 鏁扮粍鐨勭储寮曘€傝繕瑕佹敞鎰忥紝杈撳嚭鏄剧ず鍙橀噺浣嶄簬锛?

```
  # cat events/sched/sched_waking/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=pid:vals=hitcount:ts0=common_timestamp.usecs:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 000000009536f554

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1

```

缁х画鐪?sched_switch 瑙﹀彂鍣ㄧ殑 hist_debug 杈撳嚭锛岄櫎浜嗛偅涓湭琚娇鐢ㄧ殑 wakeup_lat 鍙橀噺涔嬪锛屾垜浠繕鐪嬪埌涓€涓柊鐨勩€佹樉绀哄彉閲忓紩鐢ㄧ殑鑺傘€傚彉閲忓紩鐢ㄤ箣鎵€浠ユ樉绀哄湪涓€涓嫭绔嬬殑鑺備腑锛屾槸鍥犱负闄や簡鍦ㄩ€昏緫涓婁笌鍙橀噺鍜屽€肩浉鍒嗙涔嬪锛屽畠浠疄闄呬笂浣嶄簬涓€涓嫭绔嬬殑 hist_data 鏁扮粍 var_refs[] 涓€?

鍦ㄨ繖涓緥瀛愪腑锛宻ched_switch 瑙﹀彂鍣ㄥ紩鐢ㄤ簡 sched_waking 瑙﹀彂鍣ㄤ笂鐨勪竴涓彉閲?$ts0銆傝瀵熷叾缁嗚妭锛屾垜浠彲浠ョ湅鍒版墍寮曠敤鍙橀噺鐨?var.hist_data 鍊间笌鍓嶉潰鏄剧ず鐨?sched_waking 瑙﹀彂鍣ㄧ浉鍖归厤锛岃€?var.idx 鍊间笌鍓嶉潰鏄剧ず鐨勮鍙橀噺鐨?var.idx 鍊肩浉鍖归厤銆傚悓鏃舵樉绀虹殑杩樻湁璇ュ彉閲忓紩鐢ㄧ殑 var_ref_idx 鍊硷紝鍙橀噺鐨勫€煎氨鏄紦瀛樺湪杩欓噷锛屼緵锛?

```
  # cat events/sched/sched_switch/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=next_pid:vals=hitcount:wakeup_lat=common_timestamp.usecs-$ts0:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 00000000f4ee8006

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 0
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: next_pid
      type: pid_t
      size: 8
      is_signed: 1

  variable reference fields:

    hist_data->var_refs[0]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 000000009536f554
      var_ref_idx (into hist_data->var_refs[]): 0
      type: u64
      size: 8
      is_signed: 0

```

```
  # echo '!hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0' >> events/sched/sched_switch/trigger

  # echo '!hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

## 鍔ㄤ綔涓庡鐞嗗櫒锛圓ctions and Handlers锛?



鍦ㄥ墠闈緥瀛愮殑鍩虹涓婏紝鎴戜滑鐜板湪瑕佸閭ｄ釜 wakeup_lat 鍙橀噺鍋氱偣浜嬫儏锛屽嵆鎶婂畠鍜屽彟涓€涓瓧娈典綔涓轰竴涓悎鎴愪簨浠跺彂閫佸嚭鍘汇€?

涓嬮潰鐨?onmatch() 鍔ㄤ綔鍩烘湰鎰忔€濇槸锛氭瘡褰撴垜浠湁涓€涓?sched_switch 浜嬩欢锛屽鏋滃瓨鍦ㄤ竴涓尮閰嶇殑 sched_waking 浜嬩欢锛堝湪鏈緥涓紝鍗?sched_waking 鐩存柟鍥句腑瀛樺湪涓€涓?pid 涓庢湰 sched_switch 浜嬩欢鐨?next_pid 瀛楁鐩稿尮閰嶏級锛屾垜浠氨鍙栧嚭 wakeup_latency() trace 鍔ㄤ綔涓寚瀹氱殑鍙橀噺锛屽苟鐢ㄥ畠浠悜璺熻釜娴佷腑鐢熸垚涓€涓柊鐨?wakeup_latency 浜嬩欢銆?

娉ㄦ剰锛屽儚 wakeup_latency()锛堝畠涔熷彲浠ョ瓑浠峰湴鍐欐垚 trace(wakeup_latency,$wakeup_lat,next_pid)锛夎繖鏍风殑 trace 澶勭悊鍣紝鍏跺疄鐜拌姹備紶缁?trace 澶勭悊鍣ㄧ殑鍙傛暟蹇呴』鏄彉閲忋€傚湪鏈緥涓紝$wakeup_lat 鏄剧劧鏄竴涓彉閲忥紝浣?next_pid 涓嶆槸锛屽洜涓哄畠鍙槸 sched_switch 璺熻釜浜嬩欢涓竴涓瓧娈电殑鍚嶅瓧銆傜敱浜庡嚑涔庢瘡涓?trace() 鍜?save() 鍔ㄤ綔閮戒細杩欐牱鍋氾紝鎵€浠ュ疄鐜颁簡涓€涓壒娈婃嵎寰勶紝鍏佽鍦ㄨ繖浜涙儏鍐典笅鐩存帴浣跨敤瀛楁鍚嶃€傚叾宸ヤ綔鏂瑰紡鏄細鍦ㄥ簳灞備細涓烘墍鎸囧悕鐨勫瓧娈靛垱寤轰竴涓复鏃跺彉閲忥紝杩欎釜鍙橀噺鎵嶆槸瀹為檯浼犵粰 trace 澶勭悊鍣ㄧ殑涓滆タ銆傚湪浠ｇ爜鍜屾枃妗ｄ腑锛岃繖绫诲彉閲忚绉颁负鈥滃瓧娈靛彉閲忥紙field variable锛夆€濄€?

鍏朵粬璺熻釜浜嬩欢鐩存柟鍥句笂鐨勫瓧娈典篃鍙互琚娇鐢ㄣ€傚湪閭ｇ鎯呭喌涓嬶紝鎴戜滑蹇呴』鐢熸垚涓€涓柊鐨勭洿鏂瑰浘浠ュ強涓€涓懡鍚嶄笉澶伆褰撶殑 'synthetic_field'锛堣繖閲岀殑 synthetic 涓庡悎鎴愪簨浠舵鏃犲叧绯伙級锛屽苟鎶婇偅涓壒娈婄殑鐩存柟鍥惧瓧娈靛綋浣滃彉閲忔潵浣跨敤銆?

涓嬮潰鐨勫浘浠ヤ笂涓嬫枃鐨勬柟寮忥紝鍊熷姪浣跨敤 onmatch() 澶勭悊鍣ㄥ拰 trace() 鍔ㄤ綔鐨?sched_switch 鐩存柟鍥撅紝灞曠ず浜嗕笂杩版柊澧炵殑鍏冪礌銆?

```
  # echo 'wakeup_latency u64 lat; pid_t pid' >> synthetic_events

```

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >>
          events/sched/sched_waking/trigger

```

鏈€鍚庯紝鎴戜滑鍦?sched_switch 浜嬩欢涓婂垱寤轰竴涓?hist 瑙﹀彂鍣紝鐢ㄦ潵鐢熸垚 wakeup_latency() trace 浜嬩欢銆傚湪鏈緥涓紝鎴戜滑鎶?next_pid 浼犲叆 wakeup_latency 鍚堟垚浜嬩欢鐨勮皟鐢紝璇ワ細

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0: \
          onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid)' >>
	  /sys/kernel/tracing/events/sched/sched_switch/trigger

```

sched_switch 浜嬩欢鐨勫浘涓庡墠闈㈢殑渚嬪瓙绫讳技锛屼絾瀹冨睍绀轰簡 hist_data 鏂板鐨?field_vars[] 鏁扮粍锛屽苟灞曠ず浜?field_vars 涓庝负瀹炵幇瀛楁鍙橀噺鑰屽垱寤虹殑鍙橀噺鍙婂紩鐢ㄤ箣闂寸殑鍏宠仈銆傚叿浣撶粏鑺傚皢鍦ㄤ笅闈㈣璁猴細

```
    +------------------+
    | hist_data        |
    +------------------+   +-----------------------+
      | .fields[]      |-->| val = hitcount        |
      +----------------+   +-----------------------+
      | .map           |     | .size               |
      +----------------+     +---------------------+
  +---| .field_vars[]  |     | .offset             |
  |   +----------------+     +---------------------+
  |+--| .var_refs[]    |     | .offset             |
  ||  +----------------+     +---------------------+
  ||                         | .fn()               |
  ||   var_ref_vals[]        +---------------------+
  ||  +-------------+        | .flags              |
  ||  | $ts0        |<---+   +---------------------+
  ||  +-------------+    |   | .var.idx            |
  ||  | $next_pid   |<-+ |   +---------------------+
  ||  +-------------+  | |   | .var.hist_data      |
  ||+>| $wakeup_lat |  | |   +---------------------+
  ||| +-------------+  | |   | .var_ref_idx        |
  ||| |             |  | | +-----------------------+
  ||| +-------------+  | | | var = wakeup_lat      |
  |||        .         | | +-----------------------+
  |||        .         | |   | .size               |
  |||        .         | |   +---------------------+
  ||| +-------------+  | |   | .offset             |
  ||| |             |  | |   +---------------------+
  ||| +-------------+  | |   | .fn()               |
  ||| |             |  | |   +---------------------+
  ||| +-------------+  | |   | .flags & FL_VAR     |
  |||                  | |   +---------------------+
  |||                  | |   | .var.idx            |
  |||                  | |   +---------------------+
  |||                  | |   | .var.hist_data      |
  |||                  | |   +---------------------+
  |||                  | |   | .var_ref_idx        |
  |||                  | |   +---------------------+
  |||                  | |              .
  |||                  | |              .
  |||                  | |              .
  |||                  | |              .
  ||| +--------------+ | |              .
  +-->| field_var    | | |              .
   || +--------------+ | |              .
   ||   | var        | | |              .
   ||   +------------+ | |              .
   ||   | val        | | |              .
   || +--------------+ | |              .
   || | field_var    | | |              .
   || +--------------+ | |              .
   ||   | var        | | |              .
   ||   +------------+ | |              .
   ||   | val        | | |              .
   ||   +------------+ | |              .
   ||         .        | |              .
   ||         .        | |              .
   ||         .        | | +-----------------------+ <--- n_vals
   || +--------------+ | | | key = pid             |
   || | field_var    | | | +-----------------------+
   || +--------------+ | |   | .size               |
   ||   | var        |--+|   +---------------------+
   ||   +------------+ |||   | .offset             |
   ||   | val        |-+||   +---------------------+
   ||   +------------+ |||   | .fn()               |
   ||                  |||   +---------------------+
   ||                  |||   | .flags              |
   ||                  |||   +---------------------+
   ||                  |||   | .var.idx            |
   ||                  |||   +---------------------+ <--- n_fields
   ||                  |||
   ||                  |||                           n_keys = n_fields - n_vals
   ||                  ||| +-----------------------+
   ||                  |+->| var = next_pid        |
   ||                  | | +-----------------------+
   ||                  | |   | .size               |
   ||                  | |   +---------------------+
   ||                  | |   | .offset             |
   ||                  | |   +---------------------+
   ||                  | |   | .flags & FL_VAR     |
   ||                  | |   +---------------------+
   ||                  | |   | .var.idx            |
   ||                  | |   +---------------------+
   ||                  | |   | .var.hist_data      |
   ||                  | |   +-----------------------+
   ||                  +-->| val for next_pid      |
   ||                  | | +-----------------------+
   ||                  | |   | .size               |
   ||                  | |   +---------------------+
   ||                  | |   | .offset             |
   ||                  | |   +---------------------+
   ||                  | |   | .fn()               |
   ||                  | |   +---------------------+
   ||                  | |   | .flags              |
   ||                  | |   +---------------------+
   ||                  | |   |                     |
   ||                  | |   +---------------------+
   ||                  | |
   ||                  | |
   ||                  | | +-----------------------+
   +|------------------|-|>| var_ref = $ts0        |
    |                  | | +-----------------------+
    |                  | |   | .size               |
    |                  | |   +---------------------+
    |                  | |   | .offset             |
    |                  | |   +---------------------+
    |                  | |   | .fn()               |
    |                  | |   +---------------------+
    |                  | |   | .flags & FL_VAR_REF |
    |                  | |   +---------------------+
    |                  | +---| .var_ref_idx        |
    |                  |   +-----------------------+
    |                  |   | var_ref = $next_pid   |
    |                  |   +-----------------------+
    |                  |     | .size               |
    |                  |     +---------------------+
    |                  |     | .offset             |
    |                  |     +---------------------+
    |                  |     | .fn()               |
    |                  |     +---------------------+
    |                  |     | .flags & FL_VAR_REF |
    |                  |     +---------------------+
    |                  +-----| .var_ref_idx        |
    |                      +-----------------------+
    |                      | var_ref = $wakeup_lat |
    |                      +-----------------------+
    |                        | .size               |
    |                        +---------------------+
    |                        | .offset             |
    |                        +---------------------+
    |                        | .fn()               |
    |                        +---------------------+
    |                        | .flags & FL_VAR_REF |
    |                        +---------------------+
    +------------------------| .var_ref_idx        |
                             +---------------------+

```

濡備綘鎵€瑙侊紝瀵逛簬涓€涓瓧娈靛彉閲忥紝浼氬垱寤轰袱涓?hist_field锛氫竴涓〃绀哄彉閲忥紙鍦ㄦ湰渚嬩腑鏄?next_pid锛夛紝鍙︿竴涓敤浜庡儚鏅€?val 瀛楁閭ｆ牱浠庤窡韪祦涓湡姝ｅ彇寰楄瀛楁鐨勫€笺€傚畠浠槸鐙珛浜庢櫘閫氬彉閲忓垱寤鸿繃绋嬭€屽垱寤虹殑锛屽苟淇濆瓨鍦?hist_data->field_vars[] 鏁扮粍涓€傚叧浜庡畠浠浣曡浣跨敤锛岃瑙佷笅鏂囥€傛澶栵紝杩樹細鍒涘缓涓€涓紩鐢?hist_field锛屽畠鏄紩鐢ㄥ瓧娈靛彉閲忥紙濡?trace() 鍔ㄤ綔涓殑 $next_pid 鍙橀噺锛夋墍蹇呴渶鐨勩€?

娉ㄦ剰锛?wakeup_lat 涔熸槸涓€涓彉閲忓紩鐢紝寮曠敤琛ㄨ揪寮?common_timestamp-$ts0 鐨勫€硷紝鍥犳涔熼渶瑕佸垱寤轰竴涓〃绀鸿寮曠敤鐨?hist field 鏉＄洰銆?

褰撹皟鐢?hist_trigger_elt_update() 鏉ヨ幏鍙栨櫘閫氱殑閿拰鍊煎瓧娈垫椂锛屽畠杩樹細璋冪敤 update_field_vars()锛屽悗鑰呬細閬嶅巻涓鸿鐩存柟鍥惧垱寤虹殑姣忎釜 field_var锛堝彲浠?hist_data->field_vars 鑾峰緱锛夛紝璋冪敤 val->fn() 浠庡綋鍓嶈窡韪褰曚腑鑾峰彇鏁版嵁锛岀劧鍚庝娇鐢ㄨ鍙橀噺鐨?var.idx 鎶婂彉閲忚缃埌鐩稿簲 tracing_map_elt 鐨?elt->vars[var.idx] 澶勩€?

涓€鏃︽墍鏈夊彉閲忛兘宸叉洿鏂帮紝灏卞彲浠ヤ粠 event_hist_trigger() 璋冪敤 resolve_var_refs()锛屾鏃朵笉浠呮垜浠殑 $ts0 鍜?$next_pid 寮曠敤鍙互琚В鏋愶紝$wakeup_lat 寮曠敤涔熷彲浠ャ€傝嚦姝わ紝trace() 鍔ㄤ綔鍙渶璁块棶鍦?var_ref_vals[] 鏁扮粍涓眹闆嗙殑鍊硷紝骞剁敓鎴愯 trace 浜嬩欢銆?

瀵逛簬涓?save() 鍔ㄤ綔鍏宠仈鐨勫瓧娈靛彉閲忥紝鍙戠敓鐨勬槸鍚屾牱鐨勮繃绋嬨€?

```
  hist_data = struct hist_trigger_data
  hist_data.fields = struct hist_field
  field_var = struct field_var
  fn = hist_field_fn_t
  FL_KEY = HIST_FIELD_FL_KEY
  FL_VAR = HIST_FIELD_FL_VAR
  FL_VAR_REF = HIST_FIELD_FL_VAR_REF

```

### trace() 鍔ㄤ綔鐨勫瓧娈靛彉閲忔祴璇?



杩欎釜渚嬪瓙鍦ㄥ墠涓€涓祴璇曚緥瀛愮殑鍩虹涓婏紝鏈€缁堢敤涓婁簡 wakeup_lat 鍙橀噺锛屾澶栬繕鍒涘缓浜嗕竴瀵瑰瓧娈靛彉閲忥紝鐒跺悗閫氳繃 onmatch() 澶勭悊鍣ㄦ妸瀹冧滑鍏ㄩ儴浼犵粰 wakeup_latency() trace 鍔ㄤ綔銆?

```
  # echo 'wakeup_latency u64 lat; pid_t pid; char comm[16]' >> synthetic_events

```

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

鏈€鍚庯紝鍍忓墠闈㈢殑娴嬭瘯渚嬪瓙涓€鏍凤紝鎴戜滑鍒╃敤鏉ヨ嚜 sched_waking 瑙﹀彂鍣ㄧ殑 $ts0 寮曠敤锛屾妸鍞ら啋寤惰繜璁＄畻骞惰祴缁?wakeup_lat 鍙橀噺锛岀劧鍚庢渶缁堟妸瀹冨拰 sched_switch 浜嬩欢鐨勪竴瀵瑰瓧娈?next_pid 涓?next_comm 涓€璧凤紝鐢ㄦ潵鐢熸垚涓€涓?wakeup_latency trace 浜嬩欢銆俷ext_pid 鍜?next_comm 浜嬩欢瀛楁锛?

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,next_comm)' >> /sys/kernel/tracing/events/sched/sched_switch/trigger

```

sched_waking 鐨?hist_debug 杈撳嚭鏄剧ず鐨勬暟鎹笌锛?

```
  # cat events/sched/sched_waking/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=pid:vals=hitcount:ts0=common_timestamp.usecs:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 00000000d60ff61f

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1

```

sched_switch 鐨?hist_debug 杈撳嚭鏄剧ず浜嗕笌鍓嶉潰娴嬭瘯渚嬪瓙鐩稿悓鐨勯敭鍜屽€煎瓧娈碘€斺€旀敞鎰?wakeup_lat 浠嶅湪 val fields 鑺備腑锛屼絾鏂扮殑瀛楁鍙橀噺骞朵笉鍦ㄩ偅閲屸€斺€斿敖绠″瓧娈靛彉閲忎篃鏄彉閲忥紝瀹冧滑琚崟鐙繚瀛樺湪 hist_data 鐨?field_vars[] 鏁扮粍涓€傝櫧鐒跺瓧娈靛彉閲忓拰鏅€氬彉閲忎綅浜庝笉鍚岀殑鍦版柟锛屼絾浣犲彲浠ョ湅鍒拌繖浜涘彉閲忓湪 tracing_map_elt.vars[] 涓殑瀹為檯浣嶇疆纭疄鍍忛鏈熺殑閭ｆ牱鍏锋湁閫掑鐨勭储寮曪細wakeup_lat 鍗犵敤浜?var.idx = 0 鐨勬Ы浣嶏紝鑰?next_pid 鍜?next_comm 鐨勫瓧娈靛彉閲忕殑鍊煎垎鍒槸 var.idx = 1 鍜?var.idx = 2銆傝繕瑕佹敞鎰忥紝杩欎簺鍊间笌鍙橀噺寮曠敤瀛楁鑺備腑瀵瑰簲閭ｄ簺鍙橀噺鐨勫紩鐢ㄦ墍鏄剧ず鐨勫€肩浉鍚屻€傜敱浜庡瓨鍦ㄤ袱涓Е鍙戝櫒锛屽洜姝や篃灏辨湁涓や釜 hist_data 鍦板潃锛屽湪杩涜鍖归厤鏃朵篃闇€瑕佹妸杩欎簺鍦板潃鑰冭檻杩涙潵鈥斺€斾綘鍙互鐪嬪埌绗竴涓彉閲忓紩鐢ㄧ殑鏄墠涓€涓?hist 瑙﹀彂鍣紙鍙傝涓庤瑙﹀彂鍣ㄥ叧鑱旂殑 hist_data 鍦板潃锛変笂鐨?0 鍙?var.idx锛岃€岀浜屼釜鍙橀噺寮曠敤鐨勬槸 sched_switch hist 瑙﹀彂鍣ㄤ笂鐨?0 鍙?var.idx锛屽叾浣欐墍鏈夊彉閲忓紩鐢ㄤ篃鏄姝ゃ€?

鏈€鍚庯紝鍔ㄤ綔璺熻釜鍙橀噺鑺傚彧鏄剧ず浜嗙郴缁燂細

```
  # cat events/sched/sched_switch/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=next_pid:vals=hitcount:wakeup_lat=common_timestamp.usecs-$ts0:sort=hitcount:size=2048:clock=global:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,next_comm) [active]
  #

  hist_data: 0000000008f551b7

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 0
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: next_pid
      type: pid_t
      size: 8
      is_signed: 1

  variable reference fields:

    hist_data->var_refs[0]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 00000000d60ff61f
      var_ref_idx (into hist_data->var_refs[]): 0
      type: u64
      size: 8
      is_signed: 0

    hist_data->var_refs[1]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 0000000008f551b7
      var_ref_idx (into hist_data->var_refs[]): 1
      type: u64
      size: 0
      is_signed: 0

    hist_data->var_refs[2]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: next_pid
      var.idx (into tracing_map_elt.vars[]): 1
      var.hist_data: 0000000008f551b7
      var_ref_idx (into hist_data->var_refs[]): 2
      type: pid_t
      size: 4
      is_signed: 0

    hist_data->var_refs[3]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: next_comm
      var.idx (into tracing_map_elt.vars[]): 2
      var.hist_data: 0000000008f551b7
      var_ref_idx (into hist_data->var_refs[]): 3
      type: char[16]
      size: 256
      is_signed: 0

  field variables:

    hist_data->field_vars[0]:

      field_vars[0].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: next_pid
      var.idx (into tracing_map_elt.vars[]): 1

      field_vars[0].val:
      ftrace_event_field name: next_pid
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->field_vars[1]:

      field_vars[1].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: next_comm
      var.idx (into tracing_map_elt.vars[]): 2

      field_vars[1].val:
      ftrace_event_field name: next_comm
      type: char[16]
      size: 256
      is_signed: 0

  action tracking variables (for onmax()/onchange()/onmatch()):

    hist_data->actions[0].match_data.event_system: sched
    hist_data->actions[0].match_data.event: sched_waking

```

```
  # echo '!hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,next_comm)' >> /sys/kernel/tracing/events/sched/sched_switch/trigger

  # echo '!hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

  # echo '!wakeup_latency u64 lat; pid_t pid; char comm[16]' >> synthetic_events

```

### action_data 涓?trace() 鍔ㄤ綔



濡備笂鎵€杩帮紝褰?trace() 鍔ㄤ綔鐢熸垚涓€涓悎鎴愪簨浠舵椂锛屽悎鎴愪簨浠剁殑鎵€鏈夊弬鏁拌涔堝凡缁忔槸鍙橀噺锛岃涔堣杞崲鎴愪簡鍙橀噺锛堥€氳繃瀛楁鍙橀噺锛夛紝鏈€缁堟墍鏈夎繖浜涘彉閲忓€奸兘閫氳繃寮曠敤鏀堕泦鍒?var_ref_vals[] 鏁扮粍涓€?

涓嶈繃锛寁ar_ref_vals[] 鏁扮粍涓殑鍊煎苟涓嶄竴瀹氭寜鐓у悎鎴愪簨浠跺弬鏁扮殑鐩稿悓椤哄簭鎺掑垪銆備负浜嗚В鍐宠繖涓棶棰橈紝struct action_data 鍖呭惈浜嗗彟涓€涓暟缁?var_ref_idx[]锛岀敤浜庡皢 trace 鍔ㄤ綔鐨勫弬鏁版槧灏勫埌 var_ref_vals[] 鐨勫€笺€備笅闈㈡槸涓€涓細

```
  +------------------+     wakeup_latency()
  | action_data      |       event params               var_ref_vals[]
  +------------------+    +-----------------+        +-----------------+
    | .var_ref_idx[] |--->| $wakeup_lat idx |---+    |                 |
    +----------------+    +-----------------+   |    +-----------------+
    | .synth_event   |    | $next_pid idx   |---|-+  | $wakeup_lat val |
    +----------------+    +-----------------+   | |  +-----------------+
                                   .            | +->| $next_pid val   |
                                   .            |    +-----------------+
                                   .            |           .
                          +-----------------+   |           .
			  |                 |   |           .
			  +-----------------+   |    +-----------------+
                                                +--->| $wakeup_lat val |
                                                     +-----------------+

```

鍩烘湰涓婏紝杩欏湪鍚堟垚浜嬩欢鎺㈡祴锛坧robe锛変腑鏈€缁堟槸杩欐牱琚娇鐢ㄧ殑锛?

```
  for each field i in .synth_event
    val_idx = .var_ref_idx[i]
    val = var_ref_vals[val_idx]

```

### action_data 涓?onXXX() 澶勭悊鍣?



闄や簡 onmatch() 涔嬪鐨?hist 瑙﹀彂鍣?onXXX() 鍔ㄤ綔锛屾瘮濡?onmax() 鍜?onchange()锛屼篃浼氬埄鐢ㄥ苟鍦ㄥ唴閮ㄥ垱寤洪殣钘忕殑鍙橀噺銆傝繖浜涗俊鎭繚瀛樺湪 action_data.track_data 缁撴瀯浣撲腑锛屽苟涓斾篃浼氬儚涓嬮潰渚嬪瓙涓弿杩扮殑閭ｆ牱锛屾樉绀哄湪 hist_debug 杈撳嚭涓€?

閫氬父锛宱nmax() 鎴?onchange() 澶勭悊鍣ㄤ細涓庯細

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0: \
          onmax($wakeup_lat).save(next_comm,prev_pid,prev_prio,prev_comm)' >>
          /sys/kernel/tracing/events/sched/sched_switch/trigger

```

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0: \
          onmax($wakeup_lat).snapshot()' >>
          /sys/kernel/tracing/events/sched/sched_switch/trigger

```

### save() 鍔ㄤ綔鐨勫瓧娈靛彉閲忔祴璇?



鍦ㄨ繖涓緥瀛愪腑锛屾垜浠笉鐢熸垚鍚堟垚浜嬩欢锛岃€屾槸浣跨敤 save() 鍔ㄤ綔锛屽湪 onmax() 澶勭悊鍣ㄦ娴嬪埌鍛戒腑涓€涓柊鐨勬渶澶у欢杩熸椂锛屼繚瀛樺瓧娈靛€笺€傚拰鍓嶉潰鐨勪緥瀛愪竴鏍凤紝琚繚瀛樼殑鍊间篃鏄瓧娈靛€硷紝浣嗗湪杩欑鎯呭喌涓嬶紝瀹冧滑淇濆瓨鍦ㄤ竴涓悕涓?save_vars[] 鐨勭嫭绔?hist_data 鏁扮粍涓€?

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

涓嶈繃鍦ㄦ湰渚嬩腑锛屾垜浠缃?sched_switch 瑙﹀彂鍣紝浠ヤ究姣忓綋鍛戒腑涓€涓柊鐨勬渶澶у欢杩熸椂锛屽氨淇濆瓨涓€浜?sched_switch 瀛楁鍊笺€傚浜?onmax() 澶勭悊鍣ㄥ拰 save() 鍔ㄤ綔锛岄兘浼氬垱寤哄彉閲忥紝锛?

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmax($wakeup_lat).save(next_comm,prev_pid,prev_prio,prev_comm)' >> events/sched/sched_switch/trigger

```

sched_waking 鐨?hist_debug 杈撳嚭鏄剧ず鐨勬暟鎹笌锛?

```
  # cat events/sched/sched_waking/hist_debug

  #
  # trigger info: hist:keys=pid:vals=hitcount:ts0=common_timestamp.usecs:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 00000000e6290f48

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1

```

sched_switch 瑙﹀彂鍣ㄧ殑杈撳嚭鏄剧ず浜嗕笌涔嬪墠鐩稿悓鐨?val 鍜?key 鍊硷紝浣嗕篃鏄剧ず浜嗗嚑涓柊鐨勮妭銆?

棣栧厛锛屽姩浣滆窡韪彉閲忚妭鐜板湪鏄剧ず浜?actions[].track_data 淇℃伅锛屾弿杩颁簡鐢ㄤ簬璺熻釜锛堝湪鏈緥涓級杩愯鏈€澶у€肩殑鐗规畩璺熻釜鍙橀噺鍜屽紩鐢ㄣ€俛ctions[].track_data.var_ref 鎴愬憳鍖呭惈瀵硅璺熻釜鍙橀噺鐨勫紩鐢紝鍦ㄦ湰渚嬩腑鏄?$wakeup_lat 鍙橀噺銆備负浜嗘墽琛?onmax() 澶勭悊鍣ㄥ嚱鏁帮紝杩橀渶瑕佷竴涓彉閲忥紝閫氳繃姣忓綋鍛戒腑鏂版渶澶у€兼椂灏辫鏇存柊鏉ヨ窡韪綋鍓嶆渶澶у€笺€傚湪鏈緥涓紝鎴戜滑鍙互鐪嬪埌涓€涓嚜鍔ㄧ敓鎴愮殑鍚嶄负 '__max' 鐨勫彉閲忓凡缁忚鍒涘缓锛屽苟鍙浜?actions[].track_data.track_var 鍙橀噺涓€?

鏈€鍚庯紝鍦ㄦ柊鐨勨€渟ave action variables鈥濊妭涓紝鎴戜滑鍙互鐪嬪埌 save() 鍑芥暟鐨?4 涓弬鏁板鑷村垱寤轰簡 4 涓瓧娈靛彉閲忥紝鐢ㄤ簬鍦ㄥ懡涓渶澶у€兼椂淇濆瓨鎵€鎸囧悕瀛楁鐨勫€笺€傝繖浜涘彉閲忎繚瀛樺湪鑴辩浜?hist_data 鐨勪竴涓嫭绔?save_vars[] 鏁扮粍涓紝鍥犳鏄剧ず鍦ㄥ彟涓€涓細

```
  # cat events/sched/sched_switch/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=next_pid:vals=hitcount:wakeup_lat=common_timestamp.usecs-$ts0:sort=hitcount:size=2048:clock=global:onmax($wakeup_lat).save(next_comm,prev_pid,prev_prio,prev_comm) [active]
  #

  hist_data: 0000000057bcd28d

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 0
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: next_pid
      type: pid_t
      size: 8
      is_signed: 1

  variable reference fields:

    hist_data->var_refs[0]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 00000000e6290f48
      var_ref_idx (into hist_data->var_refs[]): 0
      type: u64
      size: 8
      is_signed: 0

    hist_data->var_refs[1]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 0000000057bcd28d
      var_ref_idx (into hist_data->var_refs[]): 1
      type: u64
      size: 0
      is_signed: 0

  action tracking variables (for onmax()/onchange()/onmatch()):

    hist_data->actions[0].track_data.var_ref:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 0000000057bcd28d
      var_ref_idx (into hist_data->var_refs[]): 1
      type: u64
      size: 0
      is_signed: 0

    hist_data->actions[0].track_data.track_var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: __max
      var.idx (into tracing_map_elt.vars[]): 1
      type: u64
      size: 8
      is_signed: 0

  save action variables (save() params):

    hist_data->save_vars[0]:

      save_vars[0].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: next_comm
      var.idx (into tracing_map_elt.vars[]): 2

      save_vars[0].val:
      ftrace_event_field name: next_comm
      type: char[16]
      size: 256
      is_signed: 0

    hist_data->save_vars[1]:

      save_vars[1].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: prev_pid
      var.idx (into tracing_map_elt.vars[]): 3

      save_vars[1].val:
      ftrace_event_field name: prev_pid
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->save_vars[2]:

      save_vars[2].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: prev_prio
      var.idx (into tracing_map_elt.vars[]): 4

      save_vars[2].val:
      ftrace_event_field name: prev_prio
      type: int
      size: 4
      is_signed: 1

    hist_data->save_vars[3]:

      save_vars[3].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: prev_comm
      var.idx (into tracing_map_elt.vars[]): 5

      save_vars[3].val:
      ftrace_event_field name: prev_comm
      type: char[16]
      size: 256
      is_signed: 0

```

```
  # echo '!hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmax($wakeup_lat).save(next_comm,prev_pid,prev_prio,prev_comm)' >> events/sched/sched_switch/trigger

  # echo '!hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

## 鍑犱釜鐗规畩鎯呭喌



灏界涓婇潰娑电洊浜嗙洿鏂瑰浘鍐呴儴鏈哄埗鐨勫熀纭€锛屼絾杩樻湁鍑犱釜鐗规畩鎯呭喌鍊煎緱璁ㄨ锛屽洜涓哄畠浠線寰€浼氬甫鏉ユ洿澶氱殑鍥版儜銆傚畠浠垎鍒槸鍏朵粬鐩存柟鍥句笂鐨勫瓧娈靛彉閲忥紝浠ュ強鍒悕锛坅lias锛夛紝涓よ€呴兘灏嗗湪涓嬮潰閫氳繃绀轰緥娴嬭瘯銆佷娇鐢?hist_debug 鏂囦欢鍔犱互璇存槑銆?

### 鍏朵粬鐩存柟鍥句笂鐨勫瓧娈靛彉閲忔祴璇?



杩欎釜渚嬪瓙涓庡墠闈㈢殑渚嬪瓙绫讳技锛屼絾鍦ㄦ湰渚嬩腑锛宻ched_switch 瑙﹀彂鍣ㄥ紩鐢ㄤ簡鍙︿竴涓簨浠讹紙鍗?sched_waking 浜嬩欢锛変笂鐨勪竴涓?hist 瑙﹀彂鍣ㄥ瓧娈点€備负浜嗗疄鐜拌繖涓€鐐癸紝浼氫负閭ｄ釜鍏朵粬浜嬩欢鍒涘缓涓€涓瓧娈靛彉閲忥紝浣嗙敱浜庣幇鏈夌殑鐩存柟鍥炬棤娉曡浣跨敤锛堝洜涓虹幇鏈夌洿鏂瑰浘鏄笉鍙彉鐨勶級锛屾墍浠ヤ細鍒涘缓骞朵娇鐢ㄤ竴涓甫鏈夊尮閰嶅彉閲忕殑鏂扮洿鏂瑰浘锛屾垜浠皢鍦ㄤ笅闈㈡樉绀虹殑 hist_debug 杈撳嚭涓湅鍒拌繖涓€鐐广€?

棣栧厛锛屾垜浠垱寤?wakeup_latency 鍚堟垚浜嬩欢銆傛敞鎰忥細

```
  # echo 'wakeup_latency u64 lat; pid_t pid; int prio' >> synthetic_events

```

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

杩欓噷鎴戜滑鍦?sched_switch 涓婅缃竴涓?hist 瑙﹀彂鍣紝浣跨敤鍛藉悕 sched_waking 浜嬩欢鐨?onmatch 澶勭悊鍣ㄦ潵鍙戦€佷竴涓?wakeup_latency 浜嬩欢銆傛敞鎰忥紝浼犵粰 wakeup_latency() 鐨勭涓変釜鍙傛暟鏄?prio锛屽畠鏄竴涓渶瑕佷负鍏跺垱寤哄瓧娈靛彉閲忕殑瀛楁鍚嶃€傜劧鑰岋紝sched_switch 浜嬩欢涓婂苟娌℃湁浠讳綍 prio 瀛楁锛屾墍浠ヤ技涔庝笉鍙兘涓哄畠鍒涘缓瀛楁鍙橀噺銆備笌涔嬪尮閰嶇殑 sched_waking 浜嬩欢纭疄鏈変竴涓?prio 瀛楁锛屽洜姝ゅ簲璇ュ彲浠ュ埄鐢ㄥ畠鏉ヨ揪鍒拌繖涓洰鐨勩€傞棶棰樺湪浜庯紝鐩墠杩樹笉鍙兘鍦ㄧ幇鏈夌洿鏂瑰浘涓婂畾涔変竴涓柊鐨勫彉閲忥紝鍥犳鏃犳硶鍚戠幇鏈夌殑 sched_waking 鐩存柟鍥炬坊鍔犳柊鐨?prio 瀛楁鍙橀噺銆備笉杩囷紝鍙互涓哄悓涓€涓簨浠跺垱寤轰竴涓澶栫殑銆佲€樺尮閰嶁€欑殑 sched_waking 鐩存柟鍥撅紙鍗冲畠浣跨敤鐩稿悓鐨勯敭鍜岃繃婊ゅ櫒锛夛紝骞跺湪鍏朵笂瀹氫箟鏂扮殑 prio 瀛楁鍙橀噺銆?

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,prio)' >> events/sched/sched_switch/trigger

```

涓嬮潰鏄?sched_waking hist 瑙﹀彂鍣ㄧ殑 hist_debug 淇℃伅杈撳嚭銆傛敞鎰忚緭鍑轰腑鏄剧ず浜嗕袱涓洿鏂瑰浘锛氱涓€涓槸鎴戜滑鍦ㄥ墠闈緥瀛愪腑瑙佽繃鐨勬櫘閫?sched_waking 鐩存柟鍥撅紝绗簩涓槸鎴戜滑涓轰簡鎻愪緵 prio 瀛楁鍙橀噺鑰屽垱寤虹殑閭ｄ釜鐗规畩鐩存柟鍥俱€?

瑙傚療涓嬮潰鐨勭浜屼釜鐩存柟鍥撅紝鎴戜滑鐪嬪埌涓€涓悕涓?synthetic_prio 鐨勫彉閲忋€傝繖灏辨槸涓?prio 瀛楁鍒涘缓鐨勫瓧娈靛彉閲忥細

```
  # cat events/sched/sched_waking/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=pid:vals=hitcount:ts0=common_timestamp.usecs:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 00000000349570e4

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1


  # event histogram
  #
  # trigger info: hist:keys=pid:vals=hitcount:synthetic_prio=prio:sort=hitcount:size=2048 [active]
  #

  hist_data: 000000006920cf38

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      ftrace_event_field name: prio
      var.name: synthetic_prio
      var.idx (into tracing_map_elt.vars[]): 0
      type: int
      size: 4
      is_signed: 1

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1

```

瑙傚療涓嬮潰鐨?sched_switch 鐩存柟鍥撅紝鎴戜滑鍙互鐪嬪埌瀵?sched_waking 涓?synthetic_prio 鍙橀噺鐨勫紩鐢紝鑰岃瀵熷叾鍏宠仈鐨?hist_data 鍦板潃锛屾垜浠湅鍒板畠鐨勭‘涓庨偅涓柊鐩存柟鍥剧浉鍏宠仈銆傝繕瑕佹敞鎰忥紝鍏朵粬寮曠敤鍒嗗埆鎸囧悜涓€涓櫘閫氬彉閲?wakeup_lat锛屼互鍙婁竴涓櫘閫氬瓧娈靛彉閲?next_pid锛岋細

```
  # cat events/sched/sched_switch/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=next_pid:vals=hitcount:wakeup_lat=common_timestamp.usecs-$ts0:sort=hitcount:size=2048:clock=global:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,prio) [active]
  #

  hist_data: 00000000a73b67df

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 0
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: next_pid
      type: pid_t
      size: 8
      is_signed: 1

  variable reference fields:

    hist_data->var_refs[0]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 00000000349570e4
      var_ref_idx (into hist_data->var_refs[]): 0
      type: u64
      size: 8
      is_signed: 0

    hist_data->var_refs[1]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 00000000a73b67df
      var_ref_idx (into hist_data->var_refs[]): 1
      type: u64
      size: 0
      is_signed: 0

    hist_data->var_refs[2]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: next_pid
      var.idx (into tracing_map_elt.vars[]): 1
      var.hist_data: 00000000a73b67df
      var_ref_idx (into hist_data->var_refs[]): 2
      type: pid_t
      size: 4
      is_signed: 0

    hist_data->var_refs[3]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: synthetic_prio
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 000000006920cf38
      var_ref_idx (into hist_data->var_refs[]): 3
      type: int
      size: 4
      is_signed: 1

  field variables:

    hist_data->field_vars[0]:

      field_vars[0].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: next_pid
      var.idx (into tracing_map_elt.vars[]): 1

      field_vars[0].val:
      ftrace_event_field name: next_pid
      type: pid_t
      size: 4
      is_signed: 1

  action tracking variables (for onmax()/onchange()/onmatch()):

    hist_data->actions[0].match_data.event_system: sched
    hist_data->actions[0].match_data.event: sched_waking

```

```
  # echo '!hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,prio)' >> events/sched/sched_switch/trigger

  # echo '!hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

  # echo '!wakeup_latency u64 lat; pid_t pid; int prio' >> synthetic_events

```

### 鍒悕娴嬭瘯



杩欎釜渚嬪瓙涓庡墠闈㈢殑渚嬪瓙闈炲父鐩镐技锛屼絾婕旂ず浜嗗埆鍚嶏紙alias锛夋爣蹇椼€?

```
  # echo 'wakeup_latency u64 lat; pid_t pid; char comm[16]' >> synthetic_events

```

鎺ヤ笅鏉ワ紝鎴戜滑鍒涘缓涓€涓被浼间簬鍓嶉潰渚嬪瓙鐨?sched_waking 瑙﹀彂鍣紝锛?

```
  # echo 'hist:keys=pid:waking_pid=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

瀵逛簬 sched_switch 瑙﹀彂鍣紝鎴戜滑涓嶇洿鎺ュ湪 wakeup_latency 鍚堟垚浜嬩欢鐨勮皟鐢ㄤ腑浣跨敤 $waking_pid锛岃€屾槸鍒涘缓涓€涓悕涓?$woken_pid 鐨?$waking_pid 鍒悕锛屽苟鍦ㄥ悎鎴愪簨浠朵腑浣跨敤瀹冿細

```
  # echo 'hist:keys=next_pid:woken_pid=$waking_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,$woken_pid,next_comm)' >> events/sched/sched_switch/trigger

```

瑙傚療 sched_waking 鐨?hist_debug 杈撳嚭锛岄櫎浜嗭細

```
  # cat events/sched/sched_waking/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=pid:vals=hitcount:waking_pid=pid,ts0=common_timestamp.usecs:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 00000000a250528c

  n_vals: 3
  n_keys: 1
  n_fields: 4

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      ftrace_event_field name: pid
      var.name: waking_pid
      var.idx (into tracing_map_elt.vars[]): 0
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: ts0
      var.idx (into tracing_map_elt.vars[]): 1
      type: u64
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[3]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1

```

sched_switch 鐨?hist_debug 杈撳嚭鏄剧ず锛屼竴涓悕涓?woken_pid 鐨勫彉閲忓凡缁忚鍒涘缓锛屽苟涓旇繕璁剧疆浜?HIST_FIELD_FL_ALIAS 鏍囧織銆傚畠涔熻缃簡 HIST_FIELD_FL_VAR 鏍囧織锛岃繖灏辨槸瀹冨嚭鐜板湪 val field 鑺備腑鐨勫師鍥犮€?

灏界鏈夎繖涓疄鐜扮粏鑺傦紝涓€涓埆鍚嶅彉閲忓疄闄呬笂鏇村儚涓€涓彉閲忓紩鐢紱浜嬪疄涓婏紝瀹冨彲浠ヨ瑙嗕负鈥滃紩鐢ㄧ殑寮曠敤鈥濄€傝瀹炵幇浼氫粠琚紩鐢ㄧ殑鍙橀噺寮曠敤澶嶅埗 var_ref->fn()锛屽湪鏈緥涓槸 waking_pid 鐨?fn()锛屽嵆 hist_field_var_ref()锛屽苟鎶婅鍑芥暟浣滀负鍒悕鐨?fn()銆俬ist_field_var_ref() 杩欎釜 fn() 闇€瑕佸畠鎵€鐢ㄥ彉閲忓紩鐢ㄧ殑 var_ref_idx锛屽洜姝?waking_pid 鐨?var_ref_idx 涔熻澶嶅埗鍒颁簡鍒悕涓€傛渶缁堢粨鏋滄槸锛氬綋鍙栧洖鍒悕鐨勫€兼椂锛屽畠鏈€缁堟墍鍋氱殑涓庡師濮嬪紩鐢ㄤ細鍋氱殑瀹屽叏鐩稿悓锛屽嵆浠?var_ref_vals[] 鏁扮粍涓彇鍥炵浉鍚岀殑鍊笺€備綘鍙互鍦ㄨ緭鍑轰腑楠岃瘉杩欎竴鐐癸細娉ㄦ剰鍒悕鐨?var_ref_idx锛堝湪鏈緥涓槸 woken_pid锛変笌鍙橀噺寮曠敤瀛楁鑺備腑閭ｄ釜寮曠敤 waking_pid 鐨?var_ref_idx 鐩稿悓銆?

姝ゅ锛屼竴鏃﹀畠鍙栧緱璇ュ€硷紝鐢变簬瀹冩湰韬篃鏄竴涓彉閲忥紝瀹冨氨浼氭妸璇ュ€间繚瀛樿繘鑷繁鐨?var.idx銆傛墍浠?woken_pid 鍒悕鐨?var.idx 鏄?0锛屽綋瀹冪殑 fn() 琚皟鐢ㄦ潵鏇存柊鑷韩鏃讹紝瀹冧細鐢ㄦ潵鑷?var_ref_idx 0 鐨勫€煎～鍏呰妲戒綅銆備綘杩樹細娉ㄦ剰鍒帮紝鍦ㄥ彉閲忓紩鐢ㄨ妭涓湁涓€涓?woken_pid 鐨?var_ref銆傞偅鏄 woken_pid 鍒悕鍙橀噺鐨勫紩鐢紝浣犲彲浠ョ湅鍒板畠浠庝笌 woken_pid 鍒悕鐩稿悓鐨?var.idx锛堝嵆 0锛夊彇鍥炲€硷紝杩涜€屽啀鎶婂畠鑷繁淇濆瓨鍦ㄥ叾 var_ref_idx 妲戒綅 3 涓紝鑰岃繖涓綅缃笂鐨勫€兼渶缁堝氨鏄璧嬬粰锛?

```
  # cat events/sched/sched_switch/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=next_pid:vals=hitcount:woken_pid=$waking_pid,wakeup_lat=common_timestamp.usecs-$ts0:sort=hitcount:size=2048:clock=global:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,$woken_pid,next_comm) [active]
  #

  hist_data: 0000000055d65ed0

  n_vals: 3
  n_keys: 1
  n_fields: 4

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
        HIST_FIELD_FL_ALIAS
      var.name: woken_pid
      var.idx (into tracing_map_elt.vars[]): 0
      var_ref_idx (into hist_data->var_refs[]): 0
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 1
      type: u64
      size: 0
      is_signed: 0

  key fields:

    hist_data->fields[3]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: next_pid
      type: pid_t
      size: 8
      is_signed: 1

  variable reference fields:

    hist_data->var_refs[0]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: waking_pid
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 00000000a250528c
      var_ref_idx (into hist_data->var_refs[]): 0
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->var_refs[1]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: ts0
      var.idx (into tracing_map_elt.vars[]): 1
      var.hist_data: 00000000a250528c
      var_ref_idx (into hist_data->var_refs[]): 1
      type: u64
      size: 8
      is_signed: 0

    hist_data->var_refs[2]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 1
      var.hist_data: 0000000055d65ed0
      var_ref_idx (into hist_data->var_refs[]): 2
      type: u64
      size: 0
      is_signed: 0

    hist_data->var_refs[3]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: woken_pid
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 0000000055d65ed0
      var_ref_idx (into hist_data->var_refs[]): 3
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->var_refs[4]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: next_comm
      var.idx (into tracing_map_elt.vars[]): 2
      var.hist_data: 0000000055d65ed0
      var_ref_idx (into hist_data->var_refs[]): 4
      type: char[16]
      size: 256
      is_signed: 0

  field variables:

    hist_data->field_vars[0]:

      field_vars[0].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: next_comm
      var.idx (into tracing_map_elt.vars[]): 2

      field_vars[0].val:
      ftrace_event_field name: next_comm
      type: char[16]
      size: 256
      is_signed: 0

  action tracking variables (for onmax()/onchange()/onmatch()):

    hist_data->actions[0].match_data.event_system: sched
    hist_data->actions[0].match_data.event: sched_waking

```

```
  # echo '!hist:keys=next_pid:woken_pid=$waking_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,$woken_pid,next_comm)' >> events/sched/sched_switch/trigger

  # echo '!hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

  # echo '!wakeup_latency u64 lat; pid_t pid; char comm[16]' >> synthetic_events

```
