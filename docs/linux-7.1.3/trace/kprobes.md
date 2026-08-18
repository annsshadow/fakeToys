## 鍐呮牳鎺㈤拡锛圞probes锛?

:Author: Jim Keniston <jkenisto@us.ibm.com>
:Author: Prasanna S Panchamukhi <prasanna.panchamukhi@gmail.com>
:Author: Masami Hiramatsu <mhiramat@kernel.org>


  1. 姒傚康锛欿probes 涓?Return Probes
  2. 鏀寔鐨勫鐞嗗櫒鏋舵瀯
  3. 閰嶇疆 Kprobes
  4. API 鍙傝€?  5. Kprobes 鐗规€т笌闄愬埗
  6. 鎺㈤拡寮€閿€
  7. TODO
  8. Kprobes 绀轰緥
  9. Kretprobes 绀轰緥
  10. 宸插簾寮冪壒鎬?  Appendix A: kprobes 鐨?debugfs 鎺ュ彛
  Appendix B: kprobes 鐨?sysctl 鎺ュ彛
  Appendix C: 鍙傝€冭祫鏂?
## 姒傚康锛欿probes 涓?Return Probes


Kprobes 浣夸綘鑳藉鍔ㄦ€佸湴鎻掑叆鍒颁换鎰忓唴鏍镐緥绋嬩腑锛屽苟浠ラ潪渚靛叆鏂瑰紡鏀堕泦璋冭瘯涓庢€ц兘淇℃伅銆?浣犲嚑涔庡彲浠ュ湪浠讳綍鍐呮牳浠ｇ爜鍦板潃 [^1^]_ 澶勮缃櫡闃憋紝骞舵寚瀹氫竴涓湪鍛戒腑鏂偣鏃惰璋冪敤鐨勫鐞嗕緥绋嬨€?
       kprobes_blacklist)

鐩墠鏈変袱绉嶇被鍨嬬殑鎺㈤拡锛歬probes 涓?kretprobes锛堜篃绉?return probes锛岃繑鍥炴帰閽堬級銆?kprobe 鍙互鎻掑叆鍒板唴鏍镐腑鍑犱箮浠绘剰涓€鏉℃寚浠や笂銆俽eturn probe 鍦ㄦ寚瀹氬嚱鏁拌繑鍥炴椂瑙﹀彂銆?
鍦ㄥ吀鍨嬫儏鍐典笅锛屽熀浜?Kprobes 鐨勬彃妗╄鎵撳寘涓轰竴涓唴鏍告ā鍧椼€?妯″潡鐨?init 鍑芥暟瀹夎锛?娉ㄥ唽"锛変竴涓垨澶氫釜鎺㈤拡锛宔xit 鍑芥暟娉ㄩ攢瀹冧滑銆?璇稿 register_kprobe() 涔嬬被鐨勬敞鍐屽嚱鏁版寚瀹氭帰閽堟彃鍏ョ殑浣嶇疆锛屼互鍙婃帰閽堝懡涓椂瑕佽皟鐢ㄧ殑澶勭悊渚嬬▼銆?
杩樻湁 `register_/unregister_*probes()` 鍑芥暟鐢ㄤ簬鎴愭壒娉ㄥ唽/娉ㄩ攢涓€缁?`*probes`銆?褰撲綘闇€瑕佷竴娆℃€ф敞閿€澶ч噺鎺㈤拡鏃讹紝杩欎簺鍑芥暟鍙互鍔犲揩娉ㄩ攢杩囩▼銆?
鎺ヤ笅鏉ョ殑鍥涗釜灏忚妭瑙ｉ噴浜嗕笉鍚岀被鍨嬫帰閽堢殑宸ヤ綔鏂瑰紡锛屼互鍙婅烦杞紭鍖栨槸濡備綍宸ヤ綔鐨勩€?瀹冧滑瑙ｉ噴浜嗕竴浜涗綘涓轰簡鏈€浣冲湴浣跨敤 Kprobes 鎵€闇€瑕佷簡瑙ｇ殑浜嬫儏鈥斺€斾緥濡傦紝pre_handler 涓?post_handler 鐨勫尯鍒紝浠ュ強濡備綍浣跨敤 kretprobe 鐨?maxactive 鍜?nmissed 瀛楁銆?浣嗗鏋滀綘鎬ヤ簬寮€濮嬩娇鐢?Kprobes锛屽彲浠ヨ烦鍒?kprobes_archs_supported銆?
### Kprobe 鏄浣曞伐浣滅殑锛?

褰撴敞鍐屼竴涓?kprobe 鏃讹紝Kprobes 浼氬鍒惰鎺㈡祴鐨勬寚浠わ紝骞剁敤涓€鏉℃柇鐐规寚浠わ紙渚嬪 i386 涓?x86_64 涓婄殑 int3锛夋浛鎹㈣鎺㈡祴鎸囦护鐨勯瀛楄妭锛堟垨澶氬瓧鑺傦級銆?
褰?CPU 鍛戒腑杩欐潯鏂偣鎸囦护鏃讹紝浼氬彂鐢熶竴娆￠櫡闃憋紝CPU 鐨勫瘎瀛樺櫒琚繚瀛橈紝鎺у埗鏉冮€氳繃 notifier_call_chain 鏈哄埗浼犻€掔粰 Kprobes銆?Kprobes 鎵ц涓庤 kprobe 鍏宠仈鐨?"pre_handler"锛屽苟鎶?kprobe 缁撴瀯浠ュ強淇濆瓨鐨勫瘎瀛樺櫒鍦板潃浼犵粰璇ュ鐞嗕緥绋嬨€?
鎺ョ潃锛孠probes 鍗曟鎵ц瀹冨鍒剁殑閭ｄ唤琚帰娴嬫寚浠ゃ€?锛堝師鍦板崟姝ユ墽琛屽疄闄呮寚浠ゆ湰搴旀洿绠€鍗曪紝浣嗛偅鏍?Kprobes 灏卞繀椤讳复鏃剁Щ闄ゆ柇鐐规寚浠ゃ€?杩欐牱浼氭墦寮€涓€涓緢灏忕殑鏃堕棿绐楀彛锛屾湡闂村彟涓€涓?CPU 鍙兘鐩存帴浠庢帰娴嬬偣涓€鎺犺€岃繃銆傦級

鍦ㄦ寚浠よ鍗曟鎵ц涔嬪悗锛孠probes 鎵ц涓庤 kprobe 鍏宠仈鐨?"post_handler"锛堝鏋滄湁鐨勮瘽锛夈€?闅忓悗鎵ц浠庢帰娴嬬偣涔嬪悗鐨勬寚浠ょ户缁€?
### 鏀瑰彉鎵ц璺緞


鐢变簬 kprobes 鍙互鎺㈡祴杩愯涓殑鍐呮牳浠ｇ爜锛屽畠鑳藉鏀瑰彉瀵勫瓨鍣ㄩ泦鍚堬紝鍖呮嫭鎸囦护鎸囬拡銆?姝ゆ搷浣滈渶瑕佹瀬鍏跺皬蹇冿紝渚嬪淇濇寔鏍堝抚銆佹仮澶嶆墽琛岃矾寰勭瓑銆傜敱浜庡畠浣滅敤浜庤繍琛屼腑鐨勫唴鏍革紝骞朵笖闇€瑕佹繁鍏ョ殑璁＄畻鏈轰綋绯荤粨鏋勪笌骞跺彂璁＄畻鐭ヨ瘑锛屼綘寰堝鏄撴惉璧风煶澶寸牳鑷繁鐨勮剼銆?
濡傛灉浣犲湪 pre_handler 涓敼鍙樹簡鎸囦护鎸囬拡锛堝苟璁剧疆浜嗗叾浠栫浉鍏冲瘎瀛樺櫒锛夛紝浣犲繀椤昏繑鍥?!0锛屼互渚?kprobes 鍋滄鍗曟鎵ц骞剁洿鎺ヨ繑鍥炲埌缁欏畾鍦板潃銆?杩欎篃鎰忓懗鐫€涓嶅簲鍐嶈皟鐢?post_handler銆?
璇锋敞鎰忥紝鍦ㄦ煇浜涗娇鐢?TOC锛圱able of Contents锛岀洰褰曡〃锛夎繘琛屽嚱鏁拌皟鐢ㄧ殑浣撶郴缁撴瀯涓婏紝姝ゆ搷浣滃彲鑳芥洿鍥伴毦锛屽洜涓轰綘蹇呴』鍦ㄤ綘鐨勬ā鍧椾腑涓轰綘鐨勫嚱鏁板缓绔嬩竴涓柊鐨?TOC锛屽苟鍦ㄤ粠涓繑鍥炲悗鎭㈠鏃х殑 TOC銆?
### Return Probes


##### Return Probe 鏄浣曞伐浣滅殑锛?

褰撲綘璋冪敤 register_kretprobe() 鏃讹紝Kprobes 浼氬湪璇ュ嚱鏁扮殑鍏ュ彛澶勫缓绔嬩竴涓?kprobe銆?褰撹鎺㈡祴鐨勫嚱鏁拌璋冪敤涓斿懡涓鎺㈤拡鏃讹紝Kprobes 淇濆瓨涓€浠借繑鍥炲湴鍧€鐨勫壇鏈紝骞跺皢杩斿洖鍦板潃鏇挎崲涓?"trampoline"锛堣功搴婏級鐨勫湴鍧€銆?trampoline 鏄竴娈典换鎰忎唬鐮佲€斺€旈€氬父鍙槸涓€鏉?nop 鎸囦护銆?鍦ㄥ惎鍔ㄩ樁娈碉紝Kprobes 鍦?trampoline 澶勬敞鍐屼竴涓?kprobe銆?
褰撹鎺㈡祴鐨勫嚱鏁版墽琛屽叾杩斿洖鎸囦护鏃讹紝鎺у埗鏉冧紶閫掔粰 trampoline锛屽苟鍛戒腑璇ユ帰閽堛€?Kprobes 鐨?trampoline 澶勭悊渚嬬▼璋冪敤涓庤 kretprobe 鍏宠仈鐨勩€佺敤鎴锋寚瀹氱殑杩斿洖澶勭悊渚嬬▼锛岀劧鍚庡皢淇濆瓨鐨勬寚浠ゆ寚閽堣缃负淇濆瓨鐨勮繑鍥炲湴鍧€锛屼粠闄烽槺杩斿洖鏃跺氨鍦ㄩ偅閲屾仮澶嶆墽琛屻€?
鍦ㄨ鎺㈡祴鍑芥暟鎵ц鏈熼棿锛屽叾杩斿洖鍦板潃瀛樺偍鍦ㄤ竴涓?kretprobe_instance 绫诲瀷鐨勫璞′腑銆?鍦ㄨ皟鐢?register_kretprobe() 涔嬪墠锛岀敤鎴疯缃?kretprobe 缁撴瀯鐨?maxactive 瀛楁锛屼互鎸囧畾璇ユ寚瀹氬嚱鏁板彲浠ュ悓鏃惰鎺㈡祴鐨勫疄渚嬫暟閲忋€?register_kretprobe() 棰勫垎閰嶆寚瀹氭暟閲忕殑 kretprobe_instance 瀵硅薄銆?
渚嬪锛屽鏋滃嚱鏁版槸闈為€掑綊鐨勶紝骞朵笖鏄湪鎸佹湁鑷棆閿佺殑鎯呭喌涓嬭璋冪敤锛岄偅涔?maxactive = 1 灏辫冻澶熶簡銆?濡傛灉鍑芥暟鏄潪閫掑綊鐨勶紝骞朵笖姘歌繙涓嶄細鏀惧純 CPU锛堜緥濡傦紝閫氳繃淇″彿閲忔垨鎶㈠崰锛夛紝閭ｄ箞 NR_CPUS 灏辫冻澶熶簡銆?濡傛灉 maxactive <= 0锛屽垯琚缃负榛樿鍊硷細max(10, 2*NR_CPUS)銆?
濡傛灉浣犳妸 maxactive 璁剧疆寰楀お浣庯紝涔熶笉鏄粈涔堢伨闅撅紱浣犲彧鏄細閿欒繃涓€浜涙帰閽堛€?鍦?kretprobe 缁撴瀯涓紝nmissed 瀛楁鍦ㄦ敞鍐岃繑鍥炴帰閽堟椂琚涓洪浂锛屽苟涓旀瘡褰撹鎺㈡祴鍑芥暟琚繘鍏ヤ絾娌℃湁鍙敤鐨?kretprobe_instance 瀵硅薄鏉ュ缓绔嬭繑鍥炴帰閽堟椂閫掑銆?
##### Kretprobe 鍏ュ彛澶勭悊渚嬬▼锛坋ntry-handler锛?

Kretprobes 杩樻彁渚涗竴涓彲閫夌殑銆佺敤鎴锋寚瀹氱殑澶勭悊渚嬬▼锛屽畠鍦ㄥ嚱鏁板叆鍙ｅ杩愯銆?璇ュ鐞嗕緥绋嬮€氳繃璁剧疆 kretprobe 缁撴瀯鐨?entry_handler 瀛楁鏉ユ寚瀹氥€?姣忓綋 kretprobe 鏀剧疆鍦ㄥ嚱鏁板叆鍙ｅ鐨?kprobe 琚懡涓椂锛屽氨浼氳皟鐢ㄧ敤鎴峰畾涔夌殑 entry_handler锛堝鏋滄湁鐨勮瘽锛夈€?濡傛灉 entry_handler 杩斿洖 0锛堟垚鍔燂級锛屽垯淇濊瘉鍦ㄥ嚱鏁拌繑鍥炴椂浼氳皟鐢ㄧ浉搴旂殑杩斿洖澶勭悊渚嬬▼銆?濡傛灉 entry_handler 杩斿洖闈為浂閿欒锛屽垯 Kprobes 淇濇寔杩斿洖鍦板潃涓嶅彉锛屽苟涓旇鐗瑰畾鍑芥暟瀹炰緥鐨?kretprobe 涓嶅啀浜х敓杩涗竴姝ュ奖鍝嶃€?
澶氫釜鍏ュ彛涓庤繑鍥炲鐞嗕緥绋嬬殑璋冪敤閫氳繃涓庝箣鍏宠仈鐨勫敮涓€ kretprobe_instance 瀵硅薄杩涜鍖归厤銆?姝ゅ锛岀敤鎴疯繕鍙互鎸囧畾姣忎釜杩斿洖瀹炰緥鐨勭鏈夋暟鎹紝浣滀负姣忎釜 kretprobe_instance 瀵硅薄鐨勪竴閮ㄥ垎銆?杩欏湪鐩稿簲鐨勭敤鎴峰叆鍙ｄ笌杩斿洖澶勭悊渚嬬▼涔嬮棿鍏变韩绉佹湁鏁版嵁鏃剁壒鍒湁鐢ㄣ€?姣忎釜绉佹湁鏁版嵁瀵硅薄鐨勫ぇ灏忓彲浠ュ湪娉ㄥ唽 kretprobe 鏃堕€氳繃璁剧疆 kretprobe 缁撴瀯鐨?data_size 瀛楁鏉ユ寚瀹氥€?杩欎簺鏁版嵁鍙互閫氳繃姣忎釜 kretprobe_instance 瀵硅薄鐨?data 瀛楁璁块棶銆?
濡傛灉琚帰娴嬬殑鍑芥暟琚繘鍏ヤ絾娌℃湁鍙敤鐨?kretprobe_instance 瀵硅薄锛岄偅涔堥櫎浜嗛€掑 nmissed 璁℃暟澶栵紝鐢ㄦ埛 entry_handler 鐨勮皟鐢ㄤ篃浼氳璺宠繃銆?

### 璺宠浆浼樺寲鏄浣曞伐浣滅殑锛?

濡傛灉浣犵殑鍐呮牳浠?CONFIG_OPTPROBES=y 鏋勫缓锛堢洰鍓嶈鏍囧織鍦?x86/x86-64銆侀潪鎶㈠崰寮忓唴鏍镐笂鑷姩璁句负 'y'锛夛紝骞朵笖 "debug.kprobes_optimization" 鍐呮牳鍙傛暟琚涓?1锛堝弬瑙?sysctl(8)锛夛紝Kprobes 浼氬皾璇曞湪姣忎釜鎺㈡祴鐐逛娇鐢ㄨ烦杞寚浠ゅ彇浠ｆ柇鐐规寚浠わ紝浠ラ檷浣庢帰閽堝懡涓紑閿€銆?
##### 鍒濆鍖栦竴涓?Kprobe


褰撴敞鍐屼竴涓帰閽堟椂锛屽湪灏濊瘯姝や紭鍖栦箣鍓嶏紝Kprobes 浼氬湪鎸囧畾鍦板潃鎻掑叆涓€涓櫘閫氱殑銆佸熀浜庢柇鐐圭殑 kprobe銆?鍥犳锛屽嵆渚挎棤娉曚紭鍖栬繖涓壒瀹氱殑鎺㈡祴鐐癸紝閭ｉ噷浠嶄細鏈変竴涓帰閽堛€?
##### 瀹夊叏妫€鏌?

鍦ㄤ紭鍖栦竴涓帰閽堜箣鍓嶏紝Kprobes 鎵ц浠ヤ笅瀹夊叏妫€鏌ワ細

- Kprobes 楠岃瘉灏嗚璺宠浆鎸囦护鏇挎崲鐨勫尯鍩燂紙"浼樺寲鍖哄煙"锛夊畬鍏ㄤ綅浜庝竴涓嚱鏁板唴閮ㄣ€?  锛堣烦杞寚浠ゆ湁澶氬瓧鑺傦紝鍥犳鍙兘瑕嗙洊澶氭潯鎸囦护銆傦級

- Kprobes 鍒嗘瀽鏁翠釜鍑芥暟锛屽苟楠岃瘉娌℃湁璺宠浆杩涘叆浼樺寲鍖哄煙銆傚叿浣撹€岃█锛?
  - 鍑芥暟涓嶅寘鍚棿鎺ヨ烦杞紱
  - 鍑芥暟涓嶅寘鍚細瀵艰嚧寮傚父鐨勬寚浠わ紙鍥犱负寮傚父瑙﹀彂鐨勪慨澶嶄唬鐮佸彲鑳借烦鍥炰紭鍖栧尯鍩熲€斺€擪probes 浼氭鏌ュ紓甯歌〃鏉ラ獙璇佽繖涓€鐐癸級锛?  - 娌℃湁鍒颁紭鍖栧尯鍩熺殑杩戣烦杞紙鍒伴瀛楄妭鐨勮烦杞櫎澶栵級銆?
- 瀵逛簬浼樺寲鍖哄煙涓殑姣忔潯鎸囦护锛孠probes 楠岃瘉璇ユ寚浠ゅ彲浠?out of line锛堝紓鍦帮級鎵ц銆?
##### 鍑嗗缁曡缂撳啿鍖猴紙Detour Buffer锛?

鎺ヤ笅鏉ワ紝Kprobes 鍑嗗涓€涓?"detour"锛堢粫琛岋級缂撳啿鍖猴紝鍏朵腑鍖呭惈浠ヤ笅鎸囦护搴忓垪锛?
- 鍘嬪叆 CPU 瀵勫瓨鍣ㄧ殑浠ｇ爜锛堟ā鎷熸柇鐐归櫡闃憋級
- 瀵?trampoline 浠ｇ爜鐨勮皟鐢紝璇ヤ唬鐮佽皟鐢ㄧ敤鎴风殑鎺㈤拡澶勭悊渚嬬▼銆?- 鎭㈠瀵勫瓨鍣ㄧ殑浠ｇ爜
- 鏉ヨ嚜浼樺寲鍖哄煙鐨勬寚浠?- 璺冲洖鍘熷鎵ц璺緞鐨勬寚浠ゃ€?
##### 棰勪紭鍖?

鍦ㄥ噯澶囧ソ缁曡缂撳啿鍖轰箣鍚庯紝Kprobes 楠岃瘉涓嶅瓨鍦ㄤ互涓嬩换浣曚竴绉嶆儏鍐碉細

- 鎺㈤拡鏈変竴涓?post_handler銆?- 浼樺寲鍖哄煙涓殑鍏朵粬鎸囦护琚帰娴嬨€?- 鎺㈤拡琚鐢ㄣ€?
鍦ㄤ笂杩颁换浣曟儏鍐典笅锛孠probes 閮戒笉浼氬紑濮嬩紭鍖栬鎺㈤拡銆?鐢变簬杩欎簺閮芥槸涓存椂鎯呭喌锛屽鏋滄儏鍐靛彂鐢熷彉鍖栵紝Kprobes 浼氬皾璇曞啀娆″紑濮嬩紭鍖栧畠銆?
濡傛灉 kprobe 鍙互琚紭鍖栵紝Kprobes 灏嗚 kprobe 鎺掑叆涓€涓紭鍖栧垪琛紝骞惰涪鍔?kprobe-optimizer 宸ヤ綔闃熷垪鏉ヤ紭鍖栧畠銆?濡傛灉寰呬紭鍖栫殑鎺㈡祴鐐瑰湪琚紭鍖栦箣鍓嶈鍛戒腑锛孠probes 閫氳繃灏?CPU 鐨勬寚浠ゆ寚閽堣缃负缁曡缂撳啿鍖轰腑澶嶅埗鐨勪唬鐮侊紝灏嗘帶鍒舵潈杩旇繕缁欏師濮嬫寚浠よ矾寰勨€斺€斾粠鑰岃嚦灏戦伩鍏嶄簡鍗曟鎵ц銆?
##### 浼樺寲


Kprobe-optimizer 涓嶄細绔嬪嵆鎻掑叆璺宠浆鎸囦护锛?鐩稿弽锛屼负浜嗗畨鍏ㄨ捣瑙侊紝瀹冮鍏堣皟鐢?synchronize_rcu()锛屽洜涓?CPU 鏈夊彲鑳藉湪鎵ц浼樺寲鍖哄煙鐨勪腑閫旇涓柇 [^3^]_銆?濡備綘鎵€鐭ワ紝synchronize_rcu() 鑳藉纭繚璋冪敤 synchronize_rcu() 鏃跺浜庢椿鍔ㄧ姸鎬佺殑鎵€鏈変腑鏂兘宸插畬鎴愶紝浣嗗彧鏈夊湪 CONFIG_PREEMPT=n 鏃舵墠鎴愮珛銆?鍥犳锛岃繖涓増鏈殑 kprobe 浼樺寲浠呮敮鎸?CONFIG_PREEMPT=n 鐨勫唴鏍?[^4^]_銆?
涔嬪悗锛孠probe-optimizer 璋冪敤 stop_machine()锛屼娇鐢?text_poke_smp() 灏嗕紭鍖栧尯鍩熸浛鎹负涓€鏉℃寚鍚戠粫琛岀紦鍐插尯鐨勮烦杞寚浠ゃ€?
##### 鍙栨秷浼樺寲


褰撲竴涓浼樺寲鐨?kprobe 琚敞閿€銆佺鐢ㄦ垨琚彟涓€涓?kprobe 闃诲鏃讹紝瀹冨皢琚彇娑堜紭鍖栥€?濡傛灉杩欑鎯呭喌鍙戠敓鍦ㄤ紭鍖栧畬鎴愪箣鍓嶏紝鍒欒 kprobe 鍙槸浠庝紭鍖栧垪琛ㄤ腑鍑洪槦銆?濡傛灉浼樺寲宸茬粡瀹屾垚锛屽垯浣跨敤 text_poke_smp() 灏嗚烦杞浛鎹负鍘熷浠ｇ爜锛堥櫎浜嗛瀛楄妭涓殑 int3 鏂偣锛夈€?
   the optimizer replaces the 2nd instruction with the jump **address**
   while the interrupt handler is running. When the interrupt
   returns to original address, there is no valid instruction,
   and it causes an unexpected result.

   stop-machine method that ksplice uses for supporting a CONFIG_PREEMPT=y
   kernel.

鏋佸椤荤煡锛?璺宠浆浼樺寲鏀瑰彉浜?kprobe 鐨?pre_handler 琛屼负銆?鍦ㄦ病鏈変紭鍖栫殑鎯呭喌涓嬶紝pre_handler 鍙互閫氳繃鏀瑰彉 regs->ip 骞惰繑鍥?1 鏉ユ敼鍙樺唴鏍哥殑鎵ц璺緞銆?鐒惰€岋紝褰撴帰閽堣浼樺寲鏃讹紝璇ヤ慨鏀逛細琚拷鐣ャ€?鍥犳锛屽鏋滀綘鎯宠璋冩暣鍐呮牳鐨勬墽琛岃矾寰勶紝浣犻渶瑕佷娇鐢ㄤ互涓嬩换涓€鎶€鏈潵鎶戝埗浼樺寲锛?
- 涓?kprobe 鐨?post_handler 鎸囧畾涓€涓┖鍑芥暟銆?
鎴?
- 鎵ц 'sysctl -w debug.kprobes_optimization=n'


### 榛戝悕鍗曪紙Blacklist锛?

Kprobes 鍙互鎺㈡祴闄よ嚜韬箣澶栫殑澶ч儴鍒嗗唴鏍搞€?杩欐剰鍛崇潃鏈変竴浜涘嚱鏁版槸 kprobes 鏃犳硶鎺㈡祴鐨勩€傛帰娴嬶紙闄烽槺锛夋绫诲嚱鏁板彲鑳藉鑷撮€掑綊闄烽槺锛堜緥濡傚弻閲嶆晠闅滐級锛屾垨鑰呭祵濂楃殑鎺㈤拡澶勭悊渚嬬▼鍙兘姘歌繙涓嶈璋冪敤銆?Kprobes 灏嗘绫诲嚱鏁颁綔涓洪粦鍚嶅崟绠＄悊銆?濡傛灉浣犳兂鎶婁竴涓嚱鏁板姞鍏ラ粦鍚嶅崟锛屼綘鍙渶瑕侊紙1锛夊寘鍚?linux/kprobes.h锛屽苟涓旓紙2锛変娇鐢?NOKPROBE_SYMBOL() 瀹忔潵鎸囧畾涓€涓榛戝悕鍗曠殑鍑芥暟銆?Kprobes 浼氬皢缁欏畾鐨勬帰閽堝湴鍧€涓庨粦鍚嶅崟姣斿锛屽鏋滅粰瀹氬湴鍧€鍦ㄩ粦鍚嶅崟涓紝鍒欐嫆缁濇敞鍐屽畠銆?

## 鏀寔鐨勫鐞嗗櫒鏋舵瀯


Kprobes 涓?return probes 鍦ㄤ互涓嬩綋绯荤粨鏋勪笂瀹炵幇锛?
- i386锛堟敮鎸佽烦杞紭鍖栵級
- x86_64锛圓MD-64, EM64T锛夛紙鏀寔璺宠浆浼樺寲锛?- ppc64
- sparc64锛堝皻鏈疄鐜?Return probes銆傦級
- arm
- ppc
- mips
- s390
- parisc
- loongarch
- riscv

## 閰嶇疆 Kprobes


浣跨敤 make menuconfig/xconfig/oldconfig 閰嶇疆鍐呮牳鏃讹紝纭繚 CONFIG_KPROBES 琚涓?"y"锛屽湪 "General architecture-dependent options" 涓嬫煡鎵?"Kprobes"銆?
涓轰簡鑳藉鍔犺浇涓庡嵏杞藉熀浜?Kprobes 鐨勬彃妗╂ā鍧楋紝纭繚 "Loadable module support"锛圕ONFIG_MODULES锛変笌 "Module unloading"锛圕ONFIG_MODULE_UNLOAD锛夎璁句负 "y"銆?
鍚屾椂锛岀‘淇?CONFIG_KALLSYMS锛岀敋鑷?CONFIG_KALLSYMS_ALL 涔熻璁句负 "y"锛屽洜涓哄唴鏍稿唴鐨?kprobe 鍦板潃瑙ｆ瀽浠ｇ爜浣跨敤浜?kallsyms_lookup_name()銆?
濡傛灉浣犻渶瑕佸湪鍑芥暟涓棿鎻掑叆涓€涓帰閽堬紝浣犲彲鑳戒細鍙戠幇 "Compile the kernel with debug info"锛圕ONFIG_DEBUG_INFO锛夊緢鏈夌敤锛岃繖鏍蜂綘灏卞彲浠ヤ娇鐢?"objdump -d -l vmlinux" 鏉ユ煡鐪嬫簮鐮佸埌鐩爣浠ｇ爜鐨勬槧灏勩€?
## API 鍙傝€?

Kprobes API 涓烘瘡绉嶇被鍨嬬殑鎺㈤拡鍚勫寘鍚竴涓?"register"锛堟敞鍐岋級鍑芥暟涓庝竴涓?"unregister"锛堟敞閿€锛夊嚱鏁般€?璇?API 杩樺寘鍚?"register_*probes" 涓?"unregister_*probes" 鍑芥暟锛岀敤浜庯紙鍙嶏級娉ㄥ唽鎺㈤拡鏁扮粍銆?浠ヤ笅鏄杩欎簺鍑芥暟浠ュ強浣犲皢缂栧啓鐨勫叧鑱旀帰閽堝鐞嗕緥绋嬬殑绠€鏄庛€佽糠浣犳墜鍐屽紡瑙勮寖銆?鏈夊叧绀轰緥锛岃鍙傝 samples/kprobes/ 瀛愮洰褰曚腑鐨勬枃浠躲€?
### register_kprobe


```
	#include <linux/kprobes.h>
	int register_kprobe(struct kprobe *kp);
```

鍦ㄥ湴鍧€ kp->addr 澶勮缃竴涓柇鐐广€傚綋鏂偣琚懡涓椂锛孠probes 璋冪敤 kp->pre_handler銆?鍦ㄨ鎺㈡祴鎸囦护琚崟姝ユ墽琛屼箣鍚庯紝Kprobe 璋冪敤 kp->post_handler銆?浠讳綍鎴栧叏閮ㄥ鐞嗕緥绋嬮兘鍙互涓?NULL銆傚鏋滆缃簡 kp->flags 涓?KPROBE_FLAG_DISABLED锛屽垯璇?kp 灏嗚娉ㄥ唽浣嗗浜庣鐢ㄧ姸鎬侊紝鍥犳鍦ㄨ皟鐢?enable_kprobe(kp) 涔嬪墠涓嶄細鍛戒腑瀹冪殑澶勭悊渚嬬▼銆?

   1. 闅忕潃 "symbol_name" 瀛楁琚紩鍏ュ埌 struct kprobe 涓紝鎺㈡祴鐐瑰湴鍧€瑙ｆ瀽鐜板湪鐢卞唴鏍歌礋璐ｃ€?```
	kp.symbol_name = "symbol_name";
```

      锛堣濡傚嚱鏁版弿杩扮涔嬬被鐨?64 浣?powerpc 缁嗚妭浼氳閫忔槑鍦板鐞嗭級

   2. 濡傛灉浣犵煡閬撳畨瑁呮帰娴嬬偣鐨勭鍙峰唴鍋忕Щ锛屽彲浣跨敤 struct kprobe 鐨?"offset" 瀛楁銆傝瀛楁鐢ㄤ簬璁＄畻鎺㈡祴鐐广€?
   3. 鍙兘鎸囧畾 kprobe 鐨?"symbol_name" 鎴?"addr" 浜岃€呬箣涓€銆傚鏋滀袱鑰呴兘鎸囧畾锛宬probe 娉ㄥ唽灏嗕互 -EINVAL 澶辫触銆?
   4. 瀵逛簬 CISC 浣撶郴缁撴瀯锛堝 i386 涓?x86_64锛夛紝kprobes 浠ｇ爜涓嶄細楠岃瘉 kprobe.addr 鏄惁浣嶄簬鎸囦护杈圭晫涓娿€?      浣跨敤 "offset" 鏃惰璋ㄦ厧銆?
```
register_kprobe() 鍦ㄦ垚鍔熸椂杩斿洖 0锛屽惁鍒欒繑鍥炰竴涓礋鐨?errno銆?```

```
	#include <linux/kprobes.h>
	#include <linux/ptrace.h>
	int pre_handler(struct kprobe *p, struct pt_regs *regs);
```

p 鎸囧悜涓庢柇鐐瑰叧鑱旂殑 kprobe锛宺egs 鎸囧悜淇濆瓨鏂偣鍛戒腑鏃跺瘎瀛樺櫒鐨勭粨鏋勩€?闄ら潪浣犳槸涓€涓?Kprobes 鏋佸锛屽惁鍒欏湪杩欓噷杩斿洖 0銆?
```
	#include <linux/kprobes.h>
	#include <linux/ptrace.h>
	void post_handler(struct kprobe *p, struct pt_regs *regs,
			  unsigned long flags);
```

p 涓?regs 鐨勬弿杩颁笌 pre_handler 鐩稿悓銆俧lags 浼间箮鎬绘槸涓洪浂銆?
### register_kretprobe


```
	#include <linux/kprobes.h>
	int register_kretprobe(struct kretprobe *rp);
```

涓哄湴鍧€涓?rp->kp.addr 鐨勫嚱鏁板缓绔嬩竴涓繑鍥炴帰閽堛€?褰撹鍑芥暟杩斿洖鏃讹紝Kprobes 璋冪敤 rp->handler銆?鍦ㄨ皟鐢?register_kretprobe() 涔嬪墠锛屼綘蹇呴』閫傚綋鍦拌缃?rp->maxactive锛涜瑙?"Return Probe 鏄浣曞伐浣滅殑锛?銆?
register_kretprobe() 鍦ㄦ垚鍔熸椂杩斿洖 0锛屽惁鍒欒繑鍥炰竴涓礋鐨?errno銆?

```
	#include <linux/kprobes.h>
	#include <linux/ptrace.h>
	int kretprobe_handler(struct kretprobe_instance *ri,
			      struct pt_regs *regs);
```

regs 鐨勬弿杩板悓 kprobe.pre_handler銆俽i 鎸囧悜 kretprobe_instance 瀵硅薄锛屽叾涓互涓嬪瓧娈靛彲鑳戒护浜烘劅鍏磋叮锛?
- ret_addr锛氳繑鍥炲湴鍧€
- rp锛氭寚鍚戝搴旂殑 kretprobe 瀵硅薄
- task锛氭寚鍚戝搴旂殑 task struct
- data锛氭寚鍚戞瘡涓繑鍥炲疄渚嬬殑绉佹湁鏁版嵁锛涜瑙?"Kretprobe entry-handler"銆?
regs_return_value(regs) 瀹忔彁渚涗簡涓€涓畝鍗曠殑鎶借薄锛岀敤浜庢寜鐓т綋绯荤粨鏋?ABI 鐨勫畾涔夛紝浠庨€傚綋鐨勫瘎瀛樺櫒涓彁鍙栬繑鍥炲€笺€?
璇ュ鐞嗕緥绋嬬殑杩斿洖鍊肩洰鍓嶈蹇界暐銆?
### unregister_*probe


```
	#include <linux/kprobes.h>
	void unregister_kprobe(struct kprobe *kp);
	void unregister_kretprobe(struct kretprobe *rp);
```

绉婚櫎鎸囧畾鐨勬帰閽堛€傚湪鎺㈤拡娉ㄥ唽涔嬪悗鐨勪换浣曟椂鍒婚兘鍙互璋冪敤娉ㄩ攢鍑芥暟銆?

   If the functions find an incorrect probe (ex. an unregistered probe),
   they clear the addr field of the probe.

### register_*probes


```
	#include <linux/kprobes.h>
	int register_kprobes(struct kprobe **kps, int num);
	int register_kretprobes(struct kretprobe **rps, int num);
```

娉ㄥ唽鎸囧畾鏁扮粍涓殑 num 涓帰閽堛€傚鏋滃湪娉ㄥ唽杩囩▼涓彂鐢熶换浣曢敊璇紝鍦ㄨ閿欒鎺㈤拡涔嬪墠鐨勬墍鏈夋暟缁勬帰閽堥兘浼氬湪 register_*probes 鍑芥暟杩斿洖涔嬪墠琚畨鍏ㄥ湴娉ㄩ攢銆?
- kps/rps锛氫竴涓寚鍚?`*probe` 鏁版嵁缁撴瀯鐨勬寚閽堟暟缁?- num锛氭暟缁勬潯鐩殑鏁伴噺銆?

   You have to allocate(or define) an array of pointers and set all
   of the array entries before using these functions.

### unregister_*probes


```
	#include <linux/kprobes.h>
	void unregister_kprobes(struct kprobe **kps, int num);
	void unregister_kretprobes(struct kretprobe **rps, int num);
```

涓€娆℃€хЩ闄ゆ寚瀹氭暟缁勪腑鐨?num 涓帰閽堛€?

   If the functions find some incorrect probes (ex. unregistered
   probes) in the specified array, they clear the addr field of those
   incorrect probes. However, other probes in the array are
   unregistered correctly.

### disable_*probe


```
	#include <linux/kprobes.h>
	int disable_kprobe(struct kprobe *kp);
	int disable_kretprobe(struct kretprobe *rp);
```

涓存椂绂佺敤鎸囧畾鐨?`*probe`銆備綘鍙互閫氳繃 enable_*probe() 鍐嶆鍚敤瀹冦€備綘蹇呴』鎸囧畾宸茶娉ㄥ唽鐨勬帰閽堛€?
### enable_*probe


```
	#include <linux/kprobes.h>
	int enable_kprobe(struct kprobe *kp);
	int enable_kretprobe(struct kretprobe *rp);
```

鍚敤琚?disable_**probe() 绂佺敤鎺夌殑 `**probe`銆備綘蹇呴』鎸囧畾宸茶娉ㄥ唽鐨勬帰閽堛€?
## Kprobes 鐗规€т笌闄愬埗


Kprobes 鍏佽鍦ㄥ悓涓€鍦板潃涓婃湁澶氫釜鎺㈤拡銆?姝ゅ锛屽甫鏈?post_handler 鐨勬帰娴嬬偣鏃犳硶琚紭鍖栥€?鍥犳锛屽鏋滀綘鍦ㄤ竴涓凡浼樺寲鐨勬帰娴嬬偣涓婂畨瑁呬竴涓甫鏈?post_handler 鐨?kprobe锛岃鎺㈡祴鐐瑰皢琚嚜鍔ㄥ彇娑堜紭鍖栥€?
涓€鑸€岃█锛屼綘鍙互鍦ㄥ唴鏍镐腑鐨勪换浣曚綅缃畨瑁呮帰閽堛€?鐗瑰埆鏄紝浣犲彲浠ユ帰娴嬩腑鏂鐞嗕緥绋嬨€傛湰鑺傝璁哄凡鐭ョ殑渚嬪鎯呭喌銆?
濡傛灉浣犺瘯鍥惧湪瀹炵幇浜?Kprobes 鐨勪唬鐮侊紙涓昏鏄?kernel/kprobes.c 涓?`arch/*/kernel/kprobes.c`锛屼絾涔熷寘鎷?do_page_fault 涓?notifier_call_chain 涔嬬被鐨勫嚱鏁帮級涓畨瑁呮帰閽堬紝register_*probe 鍑芥暟灏嗚繑鍥?-EINVAL銆?
濡傛灉浣犲湪涓€涓彲鍐呰仈鐨勫嚱鏁颁腑瀹夎鎺㈤拡锛孠probes 涓嶄細璇曞浘杩借釜璇ュ嚱鏁版墍鏈夌殑鍐呰仈瀹炰緥骞跺湪閭ｉ噷瀹夎鎺㈤拡銆?gcc 鍙兘鍦ㄦ湭琚姹傜殑鎯呭喌涓嬪唴鑱斾竴涓嚱鏁帮紝鎵€浠ュ鏋滀綘娌℃湁鐪嬪埌棰勬湡鐨勬帰閽堝懡涓紝璇疯浣忚繖涓€鐐广€?
鎺㈤拡澶勭悊渚嬬▼鍙互淇敼琚帰娴嬪嚱鏁扮殑鐜鈥斺€斾緥濡傦紝閫氳繃淇敼鍐呮牳鏁版嵁缁撴瀯锛屾垨鑰呬慨鏀?pt_regs 缁撴瀯鐨勫唴瀹癸紙杩欎簺鍐呭鍦ㄤ粠鏂偣杩斿洖鏃朵細琚仮澶嶅埌瀵勫瓨鍣級銆?鍥犳锛孠probes 鍙互琚敤鏉ワ紝渚嬪锛屽畨瑁呬竴涓?bug 淇锛屾垨鑰呮敞鍏ユ晠闅滅敤浜庢祴璇曘€?褰撶劧锛孠probes 鏃犳硶鍖哄垎钃勬剰娉ㄥ叆鐨勬晠闅滀笌鎰忓鏁呴殰銆傝鍕块厭鍚庢帰閽堛€?
Kprobes 涓嶄細璇曞浘闃绘鎺㈤拡澶勭悊渚嬬▼浜掔浉韪╄笍鈥斺€斾緥濡傦紝鎺㈡祴 printk() 鐒跺悗浠庢帰閽堝鐞嗕緥绋嬩腑璋冪敤 printk()銆?濡傛灉涓€涓帰閽堝鐞嗕緥绋嬪懡涓簡涓€涓帰閽堬紝閭ｄ箞鍦ㄨ瀹炰緥涓浜屼釜鎺㈤拡鐨勫鐞嗕緥绋嬪皢涓嶄細杩愯锛屽苟涓旂浜屼釜鎺㈤拡鐨?kprobe.nmissed 鎴愬憳灏嗚閫掑銆?
鑷?Linux v2.6.15-rc1 璧凤紝澶氫釜澶勭悊渚嬬▼锛堟垨鍚屼竴澶勭悊渚嬬▼鐨勫涓疄渚嬶級鍙互鍦ㄤ笉鍚岀殑 CPU 涓婂苟鍙戣繍琛屻€?
Kprobes 涓嶄娇鐢ㄤ簰鏂ヤ綋锛屼篃涓嶅垎閰嶅唴瀛橈紝闄ら潪鍦ㄦ敞鍐屼笌娉ㄩ攢鏈熼棿銆?
鎺㈤拡澶勭悊渚嬬▼鍦ㄦ姠鍗犵鐢ㄦ垨涓柇绂佺敤鐨勭姸鎬佷笅杩愯锛岃繖鍙栧喅浜庝綋绯荤粨鏋勪笌浼樺寲鐘舵€併€傦紙渚嬪锛屽湪 x86/x86-64 涓婏紝kretprobe 澶勭悊渚嬬▼涓庝紭鍖栧悗鐨?kprobe 澶勭悊渚嬬▼鏄湪涓柇鏈鐢ㄧ殑鎯呭喌涓嬭繍琛岀殑銆傦級
鏃犺濡備綍锛屼綘鐨勫鐞嗕緥绋嬩笉搴旀斁寮?CPU锛堜緥濡傦紝璇曞浘鑾峰彇淇″彿閲忥紝鎴栫瓑寰?I/O锛夈€?
鐢变簬杩斿洖鎺㈤拡鏄€氳繃灏嗚繑鍥炲湴鍧€鏇挎崲涓?trampoline 鐨勫湴鍧€鏉ュ疄鐜扮殑锛屾爤鍥炴函涓庡 __builtin_return_address() 鐨勮皟鐢紝閫氬父浼氫负琚?kretprobe 鎺㈡祴鐨勫嚱鏁扮粰鍑?trampoline 鐨勫湴鍧€锛岃€屼笉鏄湡瀹炵殑杩斿洖鍦板潃銆?锛堟嵁鎴戜滑鎵€鐭ワ紝__builtin_return_address() 浠呯敤浜庢彃妗╀笌閿欒鎶ュ憡銆傦級

濡傛灉涓€涓嚱鏁扮殑璋冪敤娆℃暟涓庤繑鍥炴鏁颁笉鍖归厤锛屽湪璇ュ嚱鏁颁笂娉ㄥ唽杩斿洖鎺㈤拡鍙兘浼氫骇鐢熶笉鑹粨鏋溿€?鍦ㄨ繖绉嶆儏鍐典笅锛屼細鎵撳嵃涓€琛岋細
kretprobe BUG!: Processing kretprobe d000000000041aa8 @ c00000000004f48c
鍑€熻繖浜涗俊鎭紝浜轰滑灏辫兘灏嗗紩璧烽棶棰樼殑 kretprobe 鐨勭‘鍒囧疄渚嬪叧鑱旇捣鏉ャ€?鎴戜滑宸茬粡瑕嗙洊浜?do_exit() 鐨勬儏鍐点€俤o_execve() 涓?do_fork() 涓嶆槸闂銆?鎴戜滑灏氫笉娓呮鍏朵粬鍙兘浜х敓姝ら棶棰樼殑鍏蜂綋鎯呭喌銆?
濡傛灉鍦ㄤ竴涓嚱鏁拌繘鍏ユ垨閫€鍑烘椂锛孋PU 杩愯鍦ㄥ綋鍓嶄换鍔′互澶栫殑鏍堜笂锛屽湪璇ュ嚱鏁颁笂娉ㄥ唽杩斿洖鎺㈤拡鍙兘浼氫骇鐢熶笉鑹粨鏋溿€?鍑轰簬杩欎釜鍘熷洜锛孠probes 涓嶆敮鎸佸湪 x86_64 鐗堟湰鐨?__switch_to() 涓婁娇鐢ㄨ繑鍥炴帰閽堬紙鎴?kprobes锛夛紱娉ㄥ唽鍑芥暟杩斿洖 -EINVAL銆?
鍦?x86/x86-64 涓婏紝鐢变簬 Kprobes 鐨勮烦杞紭鍖栦細澶ц寖鍥村湴淇敼鎸囦护锛屼紭鍖栧瓨鍦ㄤ竴浜涢檺鍒躲€?涓轰簡瑙ｉ噴瀹冿紝鎴戜滑寮曞叆涓€浜涙湳璇€傛兂璞′竴涓敱涓ゆ潯 2 瀛楄妭鎸囦护涓庝竴鏉?3 瀛楄妭鎸囦护缁勬垚鐨勪笁鎸囦护搴忓垪銆?
```
		IA
		|
	[-2][-1][0][1][2][3][4][5][6][7]
		[ins1][ins2][  ins3 ]
		[<-     DCR       ->]
		[<- JTPR ->]

	ins1: 1st Instruction
	ins2: 2nd Instruction
	ins3: 3rd Instruction
	IA:  Insertion Address
	JTPR: Jump Target Prohibition Region
	DCR: Detoured Code Region
```

DCR 涓殑鎸囦护琚鍒跺埌 kprobe 鐨?out-of-line 缂撳啿鍖轰腑锛屽洜涓?DCR 涓殑瀛楄妭琚竴鏉?5 瀛楄妭璺宠浆鎸囦护鏇挎崲銆傚洜姝ゅ瓨鍦ㄨ嫢骞查檺鍒躲€?
a) DCR 涓殑鎸囦护蹇呴』鍙噸瀹氫綅銆?b) DCR 涓殑鎸囦护涓嶅緱鍖呭惈璋冪敤鎸囦护銆?c) JTPR 涓嶅緱浣滀负浠讳綍璺宠浆鎴栬皟鐢ㄦ寚浠ょ殑鐩爣銆?d) DCR 涓嶅緱璺ㄨ秺鍑芥暟涔嬮棿鐨勮竟鐣屻€?
鏃犺濡備綍锛岃繖浜涢檺鍒堕兘鐢卞唴鏍稿唴鐨勬寚浠よВ鐮佸櫒妫€鏌ワ紝鎵€浠ヤ綘鏃犻渶涓烘鎷呭績銆?
## 鎺㈤拡寮€閿€


鍦?2005 骞翠娇鐢ㄧ殑涓€娆惧吀鍨?CPU 涓婏紝涓€娆?kprobe 鍛戒腑闇€瑕?0.5 鍒?1.0 寰鏉ュ鐞嗐€?鍏蜂綋鑰岃█锛屼竴涓噸澶嶅懡涓悓涓€鎺㈡祴鐐广€佹瘡娆¤Е鍙戜竴涓畝鍗曞鐞嗕緥绋嬬殑鍩哄噯娴嬭瘯鎶ュ憡姣忕 1-2 鐧句竾娆″懡涓紝鍏蜂綋鍙栧喅浜庝綋绯荤粨鏋勩€?杩斿洖鎺㈤拡鍛戒腑閫氬父姣?kprobe 鍛戒腑澶氳姳 50-75% 鐨勬椂闂淬€?褰撳湪涓€涓嚱鏁颁笂璁剧疆浜嗚繑鍥炴帰閽堟椂锛屽湪璇ュ嚱鏁板叆鍙ｅ鍐嶆坊鍔犱竴涓?kprobe 鍩烘湰涓婁笉浼氬鍔犲紑閿€銆?
```
  k = kprobe; r = return probe; kr = kprobe + return probe
  on same function

  i386: Intel Pentium M, 1495 MHz, 2957.31 bogomips
  k = 0.57 usec; r = 0.92; kr = 0.99

  x86_64: AMD Opteron 246, 1994 MHz, 3971.48 bogomips
  k = 0.49 usec; r = 0.80; kr = 0.82

  ppc64: POWER5 (gr), 1656 MHz (SMT disabled, 1 virtual CPU per physical CPU)
  k = 0.77 usec; r = 1.26; kr = 1.45
```

### 浼樺寲鍚庣殑鎺㈤拡寮€閿€


閫氬父锛屼竴娆′紭鍖栧悗鐨?kprobe 鍛戒腑闇€瑕?0.07 鍒?0.1 寰鏉ュ鐞嗐€?```
  k = unoptimized kprobe, b = boosted (single-step skipped), o = optimized kprobe,
  r = unoptimized kretprobe, rb = boosted kretprobe, ro = optimized kretprobe.

  i386: Intel(R) Xeon(R) E5410, 2.33GHz, 4656.90 bogomips
  k = 0.80 usec; b = 0.33; o = 0.05; r = 1.10; rb = 0.61; ro = 0.33

  x86-64: Intel(R) Xeon(R) E5410, 2.33GHz, 4656.90 bogomips
  k = 0.99 usec; b = 0.43; o = 0.06; r = 1.24; rb = 0.68; ro = 0.30
```

## TODO


a. SystemTap (http://sourceware.org/systemtap)锛氫负鍩轰簬鎺㈤拡鐨勬彃妗╂彁渚涚畝鍖栫殑缂栫▼鎺ュ彛銆傝瘯涓€璇曘€?b. sparc64 鐨勫唴鏍歌繑鍥炴帰閽堛€?c. 瀵瑰叾浠栦綋绯荤粨鏋勭殑鏀寔銆?d. 鐢ㄦ埛绌洪棿鎺㈤拡銆?e. 鐩戣鐐规帰閽堬紙鍦ㄦ暟鎹紩鐢ㄦ椂瑙﹀彂锛夈€?
## Kprobes 绀轰緥


鍙傝 samples/kprobes/kprobe_example.c

## Kretprobes 绀轰緥


鍙傝 samples/kprobes/kretprobe_example.c

## 宸插簾寮冪壒鎬?

Jprobes 鐜板湪鏄竴涓凡搴熷純鐨勭壒鎬с€備緷璧栧畠鐨勪汉搴斿綋杩佺Щ鍒板叾浠栬拷韪壒鎬э紝鎴栬€呬娇鐢ㄦ洿鏃х殑鍐呮牳銆傝鑰冭檻灏嗕綘鐨勫伐鍏疯縼绉诲埌浠ヤ笅閫夐」涔嬩竴锛?
- 浣跨敤 trace-event 鏉ヨ拷韪甫鏈夊弬鏁扮殑鐩爣鍑芥暟銆?
  trace-event 鏄竴涓綆寮€閿€锛堝叧闂椂鍑犱箮涓嶅彲瑙佸紑閿€锛夌殑闈欐€佸畾涔変簨浠舵帴鍙ｃ€?  浣犲彲浠ュ畾涔夋柊浜嬩欢锛屽苟閫氳繃 ftrace 鎴栦换浣曞叾浠栬拷韪伐鍏疯拷韪畠銆?
  鍙傝浠ヤ笅缃戝潃锛?
    - https://lwn.net/Articles/379903/
    - https://lwn.net/Articles/381064/
    - https://lwn.net/Articles/383362/

- 灏?ftrace 鍔ㄦ€佷簨浠讹紙kprobe event锛変笌 perf-probe 涓€璧蜂娇鐢ㄣ€?
  濡傛灉浣犱互璋冭瘯淇℃伅鏋勫缓鍐呮牳锛圕ONFIG_DEBUG_INFO=y锛夛紝浣犲彲浠ラ€氳繃 perf-probe 鎵惧埌鍝釜瀵勫瓨鍣?鏍堣鍒嗛厤缁欏摢涓眬閮ㄥ彉閲忔垨鍙傛暟锛屽苟寤虹珛鏂颁簨浠舵潵杩借釜瀹冦€?
  鍙傝浠ヤ笅鏂囨。锛?
  - Documentation/trace/kprobetrace.rst
  - Documentation/trace/events.rst
  - tools/perf/Documentation/perf-probe.txt


## kprobes 鐨?debugfs 鎺ュ彛


闅忕潃杈冩柊鐨勫唴鏍革紙> 2.6.20锛夛紝宸叉敞鍐?kprobes 鐨勫垪琛ㄥ湪 /sys/kernel/debug/kprobes/ 鐩綍涓嬪彲瑙侊紙鍋囧畾 debugfs 鎸傝浇鍦?//sys/kernel/debug锛夈€?
```
	c015d71a  k  vfs_read+0x0
	c03dedc5  r  tcp_v4_rcv+0x0
```

绗竴鍒楁彁渚涙帰閽堟彃鍏ョ殑鍐呮牳鍦板潃銆?绗簩鍒楁爣璇嗘帰閽堢殑绫诲瀷锛坘 - kprobe 涓?r - kretprobe锛夛紝绗笁鍒楁寚瀹氭帰閽堢殑 symbol+offset銆?濡傛灉琚帰娴嬬殑鍑芥暟灞炰簬鏌愪釜妯″潡锛屼篃浼氭寚瀹氭ā鍧楀悕銆傚悗缁垪鏄剧ず鎺㈤拡鐘舵€併€傚鏋滄帰閽堜綅浜庝竴涓笉鍐嶆湁鏁堢殑铏氭嫙鍦板潃涓婏紙妯″潡 init 娈点€佸搴斾簬宸茶鍗歌浇妯″潡鐨勬ā鍧楄櫄鎷熷湴鍧€锛夛紝杩欐牱鐨勬帰閽堣鏍囪涓?[GONE]銆傚鏋滄帰閽堣涓存椂绂佺敤锛岃繖鏍风殑鎺㈤拡琚爣璁颁负 [DISABLED]銆傚鏋滄帰閽堣浼樺寲锛屽畠琚爣璁颁负 [OPTIMIZED]銆傚鏋滄帰閽堟槸鍩轰簬 ftrace 鐨勶紝瀹冭鏍囪涓?[FTRACE]銆?
/sys/kernel/debug/kprobes/enabled锛氬己鍒跺紑鍚?鍏抽棴 kprobes銆?
鎻愪緵涓€涓棆閽紝鐢ㄤ簬鍏ㄥ眬鍦般€佸己鍒舵€у湴寮€鍚垨鍏抽棴宸叉敞鍐岀殑 kprobes銆?榛樿鎯呭喌涓嬶紝鎵€鏈?kprobes 閮芥槸鍚敤鐨勩€傞€氳繃鍚戣鏂囦欢鍥炴樉 "0"锛屾墍鏈夊凡娉ㄥ唽鐨勬帰閽堝皢琚В闄ゆ瑁咃紝鐩村埌鍚戣鏂囦欢鍥炴樉 "1" 涓烘銆?璇锋敞鎰忥紝杩欎釜鏃嬮挳鍙槸瑙ｉ櫎姝﹁鍜屾瑁呮墍鏈?kprobes锛屽苟涓嶆敼鍙樻瘡涓帰閽堢殑绂佺敤鐘舵€併€傝繖鎰忓懗鐫€锛屽鏋滀綘閫氳繃杩欎釜鏃嬮挳寮€鍚墍鏈?kprobes锛岃绂佺敤鐨?kprobes锛堟爣璁颁负 [DISABLED]锛変笉浼氳鍚敤銆?

## kprobes 鐨?sysctl 鎺ュ彛


/proc/sys/debug/kprobes-optimization锛氬紑鍚?鍏抽棴 kprobes 浼樺寲銆?
褰?CONFIG_OPTPROBES=y 鏃讹紝浼氬嚭鐜拌繖涓?sysctl 鎺ュ彛锛屽畠鎻愪緵涓€涓棆閽紝鐢ㄤ簬鍏ㄥ眬鍦般€佸己鍒舵€у湴寮€鍚垨鍏抽棴璺宠浆浼樺寲锛堝弬瑙?kprobes_jump_optimization 灏忚妭锛夈€?榛樿鎯呭喌涓嬶紝璺宠浆浼樺寲鏄厑璁哥殑锛圤N锛夈€傚鏋滀綘鍚戣鏂囦欢鍥炴樉 "0"锛屾垨鑰呴€氳繃 sysctl 灏?"debug.kprobes_optimization" 璁句负 0锛屾墍鏈夊凡浼樺寲鐨勬帰閽堝皢琚彇娑堜紭鍖栵紝骞朵笖姝ゅ悗娉ㄥ唽鐨勪换浣曟柊鎺㈤拡灏嗕笉浼氳浼樺寲銆?
璇锋敞鎰忥紝杩欎釜鏃嬮挳**鏀瑰彉**浼樺寲鐘舵€併€傝繖鎰忓懗鐫€宸蹭紭鍖栫殑鎺㈤拡锛堟爣璁颁负 [OPTIMIZED]锛夊皢琚彇娑堜紭鍖栵紙[OPTIMIZED] 鏍囩灏嗚绉婚櫎锛夈€傚鏋滆鏃嬮挳琚墦寮€锛屽畠浠皢琚啀娆′紭鍖栥€?
## 鍙傝€冭祫鏂?

鏈夊叧 Kprobes 鐨勬洿澶氫俊鎭紝璇峰弬鑰冧互涓?URL锛?
- https://lwn.net/Articles/132196/
- https://www.kernel.org/doc/ols/2006/ols2006v2-pages-109-124.pdf
