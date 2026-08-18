
## libbpf 姒傝堪


libbpf 鏄竴涓熀浜?C 鐨勫簱锛屽寘鍚竴涓?BPF 鍔犺浇鍣紝瀹冩帴鏀剁紪璇戝ソ鐨?BPF
鐩爣鏂囦欢锛屽苟灏嗗叾鍑嗗骞跺姞杞藉埌 Linux 鍐呮牳涓€俵ibbpf 鎺ョ浜嗗姞杞姐€侀獙璇佷互鍙婂皢
BPF 绋嬪簭闄勫姞鍒板悇绉嶅唴鏍搁挬瀛愶紙hook锛夌殑閲嶆椿锛屼娇 BPF 搴旂敤寮€鍙戣€呭彧闇€涓撴敞浜?BPF 绋嬪簭鐨勬纭€т笌鎬ц兘銆?
浠ヤ笅鏄?libbpf 鏀寔鐨勯珮灞傜壒鎬э細

- 鎻愪緵楂樺眰鍜屼綆灞?API锛屼緵鐢ㄦ埛绌洪棿绋嬪簭涓?BPF 绋嬪簭杩涜浜や簰銆備綆灞?API 灏佽浜?  鍏ㄩ儴 bpf 绯荤粺璋冪敤鐨勫姛鑳斤紝鍦ㄧ敤鎴烽渶瑕佸鐢ㄦ埛绌洪棿涓?BPF 绋嬪簭涔嬮棿鐨勪氦浜?  杩涜鏇寸粏绮掑害鎺у埗鏃堕潪甯告湁鐢ㄣ€?- 涓?bpftool 鐢熸垚鐨?BPF 鐩爣楠ㄦ灦锛坰keleton锛夋彁渚涙暣浣撴敮鎸併€?  楠ㄦ灦鏂囦欢绠€鍖栦簡鐢ㄦ埛绌洪棿绋嬪簭璁块棶鍏ㄥ眬鍙橀噺浠ュ強涓?BPF 绋嬪簭鍗忎綔鐨勮繃绋嬨€?- 鎻愪緵 BPF 绔?API锛屽寘鎷?BPF 杈呭姪鍑芥暟瀹氫箟銆丅PF map 鏀寔浠ュ強
  tracing 杈呭姪鍑芥暟锛屼娇寮€鍙戣€呰兘澶熺畝鍖?BPF 浠ｇ爜鐨勭紪鍐欍€?- 鏀寔 BPF CO-RE 鏈哄埗锛屼娇 BPF 寮€鍙戣€呰兘澶熺紪鍐欏彲绉绘鐨?BPF 绋嬪簭锛?  杩欎簺绋嬪簭鍙互涓€娆＄紪璇戝苟鍦ㄤ笉鍚屽唴鏍哥増鏈笂杩愯銆?
鏈枃妗ｅ皢娣卞叆鎺㈣涓婅堪姒傚康锛屽府鍔╂偍鏇存繁鍏ュ湴鐞嗚В libbpf 鐨勮兘鍔涗笌浼樺娍锛?浠ュ強瀹冨浣曞府鍔╂偍楂樻晥鍦板紑鍙?BPF 搴旂敤銆?
## BPF 搴旂敤鐢熷懡鍛ㄦ湡涓?libbpf API


涓€涓?BPF 搴旂敤鐢变竴涓垨澶氫釜 BPF 绋嬪簭锛堝郊姝ゅ崗浣滄垨瀹屽叏鐙珛锛夈€丅PF map 浠ュ強
鍏ㄥ眬鍙橀噺缁勬垚銆傚叏灞€鍙橀噺鍦ㄦ墍鏈?BPF 绋嬪簭涔嬮棿鍏变韩锛屼娇瀹冧滑鑳藉鍥寸粫涓€缁?鍏叡鏁版嵁杩涜鍗忎綔銆俵ibbpf 鎻愪緵浜嗕竴绯诲垪 API锛岀敤鎴风┖闂寸▼搴忓彲浠ラ€氳繃瑙﹀彂
BPF 搴旂敤鐢熷懡鍛ㄦ湡鐨勪笉鍚岄樁娈垫潵鎿嶇旱杩欎簺 BPF 绋嬪簭銆?
浠ヤ笅灏忚妭绠€瑕佹杩颁簡 BPF 鐢熷懡鍛ㄦ湡鐨勫悇涓樁娈碉細

- **Open 闃舵**锛氬湪姝ら樁娈碉紝libbpf 瑙ｆ瀽 BPF
  鐩爣鏂囦欢骞跺彂鐜?BPF map銆丅PF 绋嬪簭浠ュ強鍏ㄥ眬鍙橀噺銆傚湪 BPF 搴旂敤琚墦寮€鍚庯紝
  鐢ㄦ埛绌洪棿搴旂敤鍙互鍦ㄦ墍鏈夊疄浣撹鍒涘缓鍜屽姞杞戒箣鍓嶈繘琛岄澶栫殑璋冩暣
  锛堜緥濡傦紝蹇呰鏃惰缃?BPF 绋嬪簭绫诲瀷锛涗负鍏ㄥ眬鍙橀噺棰勮鍒濆鍊肩瓑锛夈€?
- **Load 闃舵**锛氬湪鍔犺浇闃舵锛宭ibbpf 鍒涘缓 BPF
  map銆佽В鏋愬悇绉嶉噸瀹氫綅锛屽苟灏?BPF 绋嬪簭楠岃瘉骞跺姞杞藉埌鍐呮牳涓€傛鏃讹紝libbpf
  浼氭牎楠?BPF 搴旂敤鐨勫悇涓儴鍒嗗苟灏嗗叾鍔犺浇鍒板唴鏍革紝浣嗚繕娌℃湁浠讳綍 BPF 绋嬪簭
  琚墽琛屻€傚湪鍔犺浇闃舵涔嬪悗锛屽彲浠ヨ缃?BPF map 鐨勫垵濮嬬姸鎬侊紝鑰屾棤闇€鎷呭績涓?  BPF 绋嬪簭浠ｇ爜鎵ц浜х敓绔炰簤銆?
- **Attachment 闃舵**锛氬湪姝ら樁娈碉紝libbpf
  灏?BPF 绋嬪簭闄勫姞鍒板悇绉?BPF 閽╁瓙鐐癸紙渚嬪 tracepoint銆乲probe銆?  cgroup 閽╁瓙銆佺綉缁滄暟鎹寘澶勭悊娴佹按绾跨瓑锛夈€傚湪姝ら樁娈碉紝BPF 绋嬪簭鎵ц
  璇稿澶勭悊鏁版嵁鍖呮垨鏇存柊 BPF map 涓庡叏灞€鍙橀噺绛夋湁鐢ㄥ伐浣滐紝杩欎簺鍐呭
  鍙互浠庣敤鎴风┖闂磋鍙栥€?
- **Tear down 闃舵**锛氬湪鎷嗛櫎闃舵锛?  libbpf 灏?BPF 绋嬪簭浠庡唴鏍镐笂鍒嗙骞跺嵏杞藉畠浠€侭PF map 琚攢姣侊紝
  BPF 搴旂敤浣跨敤鐨勬墍鏈夎祫婧愯閲婃斁銆?
## BPF 鐩爣楠ㄦ灦鏂囦欢


BPF 楠ㄦ灦鏄?libbpf API 鎿嶄綔 BPF 鐩爣鐨勫彟涓€绉嶆帴鍙ｃ€傞鏋朵唬鐮佹娊璞′簡閫氱敤鐨?libbpf API锛屼粠鑰屽ぇ骞呯畝鍖栦簡浠庣敤鎴风┖闂存搷绾?BPF 绋嬪簭鐨勪唬鐮併€傞鏋朵唬鐮佸寘鍚?BPF 鐩爣鏂囦欢鐨勫瓧鑺傜爜琛ㄧず锛岀畝鍖栦簡鍒嗗彂 BPF 浠ｇ爜鐨勮繃绋嬨€傜敱浜庡唴宓屼簡 BPF
瀛楄妭鐮侊紝鎮ㄧ殑搴旂敤浜岃繘鍒舵枃浠舵棤闇€鍐嶉儴缃查澶栫殑鏂囦欢銆?
鎮ㄥ彲浠ラ€氳繃灏?BPF 鐩爣鏂囦欢浼犵粰 bpftool 鏉ョ敓鎴愯鐩爣鏂囦欢瀵瑰簲鐨勯鏋跺ご鏂囦欢
`(.skel.h)`銆傜敓鎴愮殑 BPF 楠ㄦ灦鎻愪緵浜嗕互涓嬩笌 BPF 鐢熷懡鍛ㄦ湡鐩稿搴旂殑鑷畾涔夊嚱鏁帮紝
姣忎釜鍑芥暟閮戒互鍏蜂綋鐨勭洰鏍囧悕浣滀负鍓嶇紑锛?
- `<name>__open()` 鈥?鍒涘缓骞舵墦寮€ BPF 搴旂敤锛坄<name>` 浠ｈ〃
  鍏蜂綋鐨?bpf 鐩爣鍚嶏級
- `<name>__load()` 鈥?瀹炰緥鍖栥€佸姞杞藉苟楠岃瘉 BPF 搴旂敤鍚勯儴鍒?- `<name>__attach()` 鈥?闄勫姞鎵€鏈夊彲鑷姩闄勫姞鐨?BPF 绋嬪簭锛堣繖鏄?  鍙€夌殑锛屾偍涔熷彲浠ラ€氳繃鐩存帴浣跨敤 libbpf API 鑾峰緱鏇村鎺у埗锛?- `<name>__destroy()` 鈥?鍒嗙鎵€鏈?BPF 绋嬪簭骞?  閲婃斁鎵€鏈夊凡浣跨敤鐨勮祫婧?
浣跨敤楠ㄦ灦浠ｇ爜鏄搷浣?bpf 绋嬪簭鐨勬帹鑽愭柟寮忋€傝娉ㄦ剰锛孊PF 楠ㄦ灦鎻愪緵浜嗗搴曞眰
BPF 鐩爣鐨勮闂紝鍥犳鍗充究浣跨敤浜?BPF 楠ㄦ灦锛屽嚒鏄兘鐢ㄩ€氱敤 libbpf API 瀹屾垚鐨?鎿嶄綔浠嶇劧閮藉彲浠ュ畬鎴愩€傚畠鏄竴涓檮鍔犵殑渚垮埄鐗规€э紝娌℃湁绯荤粺璋冪敤锛屼篃娌℃湁
绻佺悙鐨勪唬鐮併€?
### 浣跨敤楠ㄦ灦鏂囦欢鐨勫叾浠栦紭鍔?

- BPF 楠ㄦ灦涓虹敤鎴风┖闂寸▼搴忔彁渚涗簡鎿嶄綔 BPF 鍏ㄥ眬鍙橀噺鐨勬帴鍙ｃ€傞鏋朵唬鐮佸皢鍏ㄥ眬鍙橀噺
  浠ョ粨鏋勪綋鐨勫舰寮忓唴瀛樻槧灏勫埌鐢ㄦ埛绌洪棿銆傝缁撴瀯浣撴帴鍙ｅ厑璁哥敤鎴风┖闂寸▼搴忓湪 BPF
  鍔犺浇闃舵涔嬪墠鍒濆鍖?BPF 绋嬪簭锛屽苟鍦ㄤ箣鍚庝粠鐢ㄦ埛绌洪棿鑾峰彇鍜屾洿鏂版暟鎹€?
- `skel.h` 鏂囦欢鍒楀嚭浜嗗彲鐢ㄧ殑 map銆佺▼搴忕瓑鍐呭锛屽弽鏄犱簡鐩爣鏂囦欢鐨勭粨鏋勩€侭PF 楠ㄦ灦
  灏嗘墍鏈夌殑 BPF map 鍜?BPF 绋嬪簭浣滀负缁撴瀯浣撳瓧娈电洿鎺ユ毚闇层€傝繖灏辨秷闄や簡浣跨敤
  `bpf_object_find_map_by_name()` 鍜?  `bpf_object_find_program_by_name()` API 杩涜鍩轰簬瀛楃涓叉煡鎵剧殑闇€瑕侊紝
  浠庤€屽噺灏戜簡鍥?BPF 婧愪唬鐮佷笌鐢ㄦ埛绌洪棿浠ｇ爜澶辨鑰屼骇鐢熺殑閿欒銆?
- 鍐呭祵鐨勭洰鏍囨枃浠跺瓧鑺傜爜琛ㄧず纭繚浜嗛鏋朵笌 BPF 鐩爣鏂囦欢濮嬬粓淇濇寔鍚屾銆?
## BPF 杈呭姪鍑芥暟


libbpf 鎻愪緵 BPF 绔?API锛孊PF 绋嬪簭鍙互浣跨敤瀹冧滑涓庣郴缁熻繘琛屼氦浜掋€侭PF 杈呭姪鍑芥暟
鐨勫畾涔変娇寮€鍙戣€呰兘澶熷儚浣跨敤鍏朵粬鏅€?C 鍑芥暟涓€鏍峰湪 BPF 浠ｇ爜涓娇鐢ㄥ畠浠€備緥濡傦紝
鏈変竴浜涜緟鍔╁嚱鏁板彲鐢ㄤ簬鎵撳嵃璋冭瘯淇℃伅銆佽幏鍙栫郴缁熷惎鍔ㄤ互鏉ョ殑鏃堕棿銆佷笌 BPF map
浜や簰銆佹搷绾电綉缁滄暟鎹寘绛夈€?
鏈夊叧杩欎簺杈呭姪鍑芥暟鐨勪綔鐢ㄣ€佹帴鏀剁殑鍙傛暟浠ュ強杩斿洖鍊肩殑瀹屾暣璇存槑锛岃鍙傞槄
`bpf-helpers
<https://man7.org/linux/man-pages/man7/bpf-helpers.7.html>`_ 鎵嬪唽椤点€?
## BPF CO-RE锛堜竴娆＄紪璇?鈥?鍒板杩愯锛?

BPF 绋嬪簭鍦ㄥ唴鏍哥┖闂磋繍琛岋紝鑳藉璁块棶鍐呮牳鍐呭瓨鍜屾暟鎹粨鏋勩€侭PF 搴旂敤闈复鐨勪竴涓?灞€闄愭槸缂轰箯璺ㄤ笉鍚屽唴鏍哥増鏈拰閰嶇疆鐨勫彲绉绘鎬с€俙BCC
<https://github.com/iovisor/bcc/>`_ 鏄?BPF 鍙Щ妞嶆€х殑瑙ｅ喅鏂规涔嬩竴銆?鐒惰€岋紝瀹冨甫鏉ヤ簡杩愯鏃跺紑閿€锛屽苟涓旂敱浜庡皢缂栬瘧鍣ㄥ祵鍏ュ簲鐢ㄨ€屼骇鐢熶簡杈冨ぇ鐨勪簩杩涘埗浣撶Н銆?
libbpf 閫氳繃鏀寔 BPF CO-RE 姒傚康鏉ユ彁鍗?BPF 绋嬪簭鐨勫彲绉绘鎬с€?BPF CO-RE 灏?BTF 绫诲瀷淇℃伅銆乴ibbpf 浠ュ強缂栬瘧鍣ㄧ粨鍚堣捣鏉ワ紝鐢熸垚涓€涓彲鍦ㄥ涓?鍐呮牳鐗堟湰鍜岄厤缃笂杩愯鐨勫崟涓€鍙墽琛屼簩杩涘埗鏂囦欢銆?
涓轰簡浣?BPF 绋嬪簭鍙Щ妞嶏紝libbpf 渚濊禆浜庤繍琛屼腑鍐呮牳鐨?BTF 绫诲瀷淇℃伅銆傚唴鏍镐篃閫氳繃
`sysfs` 鍦?`/sys/kernel/btf/vmlinux` 鏆撮湶杩欎竴鑷弿杩扮殑鏉冨▉ BTF
淇℃伅銆?
鎮ㄥ彲浠ヤ娇鐢ㄤ互涓嬪懡浠や负杩愯涓殑鍐呮牳鐢熸垚 BTF 淇℃伅锛?
```

  $ bpftool btf dump file /sys/kernel/btf/vmlinux format c > vmlinux.h

```
璇ュ懡浠や細鐢熸垚涓€涓?`vmlinux.h` 澶存枃浠讹紝鍏朵腑鍖呭惈杩愯涓殑鍐呮牳浣跨敤鐨勬墍鏈夊唴鏍哥被鍨?锛圼BTF types <../btf>](BTF types <../btf>)锛夈€傚湪鎮ㄧ殑 BPF 绋嬪簭鍖呭惈
`vmlinux.h` 鍚庯紝灏变笉鍐嶄緷璧栫郴缁熻寖鍥村唴鐨勫唴鏍稿ご鏂囦欢銆?
libbpf 閫氳繃鏌ョ湅 BPF 绋嬪簭璁板綍涓嬫潵鐨?BTF 绫诲瀷涓庨噸瀹氫綅淇℃伅锛屽苟灏嗗叾涓庤繍琛屼腑
鍐呮牳鎻愪緵鐨?BTF 淇℃伅锛坴mlinux锛夎繘琛屽尮閰嶆潵瀹炵幇 BPF 绋嬪簭鐨勫彲绉绘鎬с€傛帴鐫€锛?libbpf 瑙ｆ瀽骞跺尮閰嶆墍鏈夌被鍨嬩笌瀛楁锛屾洿鏂板繀瑕佺殑鍋忕Щ閲忎互鍙婂叾浠栧彲閲嶅畾浣嶆暟鎹紝
浠ョ‘淇?BPF 绋嬪簭鐨勯€昏緫鍦ㄧ壒瀹氬涓绘満鍐呮牳涓婃纭繍琛屻€傚洜姝わ紝BPF CO-RE 姒傚康
娑堥櫎浜嗕笌 BPF 寮€鍙戠浉鍏崇殑寮€閿€锛屼娇寮€鍙戣€呮棤闇€淇敼鍗冲彲缂栧啓鍙Щ妞嶇殑 BPF 搴旂敤锛?涔熸棤闇€鍦ㄧ洰鏍囨満鍣ㄤ笂杩涜杩愯鏃舵簮浠ｇ爜缂栬瘧銆?
浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曚娇鐢?BPF CO-RE 鍜?libbf 璇诲彇鍐呮牳
`task_struct` 鐨?parent 瀛楁銆備互鍙噸瀹氫綅鏂瑰紡璇诲彇瀛楁鐨勫熀鏈緟鍔╁嚱鏁版槸
`bpf_core_read(dst, sz, src)`锛屽畠浼氫粠 `src` 寮曠敤鐨勫瓧娈佃鍙?`sz` 涓瓧鑺傚埌 `dst` 鎸囧悜鐨勫唴瀛樹腑銆?
   :emphasize-lines: 6

    //...
    struct task_struct **task = (void **)bpf_get_current_task();
    struct task_struct *parent_task;
    int err;

    err = bpf_core_read(&parent_task, sizeof(void *), &task->parent);
    if (err) {
      /** handle error **/
    }

    /** parent_task 鍖呭惈浜?task->parent 鎸囬拡鐨勫€?**/

鍦ㄤ唬鐮佺墖娈典腑锛屾垜浠鍏堜娇鐢?`bpf_get_current_task()` 鑾峰彇鎸囧悜褰撳墠
`task_struct` 鐨勬寚閽堛€傜劧鍚庝娇鐢?`bpf_core_read()` 灏?task struct 鐨?parent 瀛楁璇诲叆 `parent_task` 鍙橀噺銆俙bpf_core_read()` 寰堝儚
`bpf_probe_read_kernel()` BPF 杈呭姪鍑芥暟锛屼笉鍚屼箣澶勫湪浜庡畠浼氳褰曞叧浜庤瀛楁
鐨勪俊鎭紝浠ヤ究鍦ㄧ洰鏍囧唴鏍镐笂杩涜閲嶅畾浣嶃€備篃灏辨槸璇达紝濡傛灉 `parent` 瀛楁鐢变簬
鍏跺墠闈㈡柊澧炰簡鏌愪釜瀛楁鑰屽亸绉诲埌浜?`struct task_struct` 鍐呯殑涓嶅悓鍋忕Щ浣嶇疆锛?libbpf 浼氳嚜鍔ㄥ皢瀹為檯鍋忕Щ閲忚皟鏁村埌姝ｇ‘鐨勫€笺€?
## 寮€濮嬩娇鐢?libbpf


璇锋煡鐪?`libbpf-bootstrap <https://github.com/libbpf/libbpf-bootstrap>`_
浠ｇ爜浠撳簱锛屽叾涓寘鍚娇鐢?libbpf 鏋勫缓鍚勭被 BPF 搴旂敤鐨勭畝鍗曠ず渚嬨€?
鍙﹁鍙傞槄 `libbpf API 鏂囨。
<https://libbpf.readthedocs.io/en/latest/api.html>`_銆?
## libbpf 涓?Rust


濡傛灉鎮ㄤ娇鐢?Rust 鏋勫缓 BPF 搴旂敤锛屽缓璁娇鐢?`Libbpf-rs <https://github.com/libbpf/libbpf-rs>`_ 搴擄紝鑰屼笉鏄洿鎺ヤ娇鐢?bindgen 鐢熸垚鐨?libbpf 缁戝畾銆侺ibbpf-rs 浠ョ鍚?Rust 涔犳儻鐨勬帴鍙ｅ皝瑁呬簡 libbpf
鍔熻兘锛屽苟鎻愪緵 libbpf-cargo 鎻掍欢鏉ュ鐞?BPF 浠ｇ爜缂栬瘧鍜岄鏋剁敓鎴愩€備娇鐢?Libbpf-rs
浼氫娇 BPF 搴旂敤鐨勭敤鎴风┖闂撮儴鍒嗘洿鏄撲簬鏋勫缓銆傝娉ㄦ剰锛孊PF 绋嬪簭鏈韩浠嶅繀椤荤敤
绾?C 缂栧啓銆?
## libbpf 鏃ュ織璁板綍


榛樿鎯呭喌涓嬶紝libbpf 灏嗕俊鎭拰璀﹀憡娑堟伅璁板綍鍒?stderr銆傝繖浜涙秷鎭殑璇︾粏绋嬪害鍙互
閫氳繃璁剧疆鐜鍙橀噺 LIBBPF_LOG_LEVEL 涓?warn銆乮nfo 鎴?debug 鏉ユ帶鍒躲€傚彲浠ヤ娇鐢?`libbpf_set_print()` 璁剧疆鑷畾涔夌殑鏃ュ織鍥炶皟銆?
## 鍏朵粬鏂囨。


- `Program types and ELF Sections <https://libbpf.readthedocs.io/en/latest/program_types.html>`_
- `API naming convention <https://libbpf.readthedocs.io/en/latest/libbpf_naming_convention.html>`_
- `Building libbpf <https://libbpf.readthedocs.io/en/latest/libbpf_build.html>`_
- `API documentation Convention <https://libbpf.readthedocs.io/en/latest/libbpf_naming_convention.html#api-documentation-convention>`_
