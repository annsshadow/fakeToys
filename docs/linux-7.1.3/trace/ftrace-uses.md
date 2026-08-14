## 浣跨敤 ftrace 閽╂寕鍒板嚱鏁?
涓?4.14 鎾板啓

## 绠€浠?
ftrace 鍩虹璁炬柦鏈€鍒濊鍒涘缓鐢ㄦ潵灏嗗洖璋冮檮鍔犲埌鍑芥暟鐨勫紑澶达紝浠ヨ褰曞拰杩借釜鍐呮牳鐨勬祦绋嬨€備絾瀵瑰嚱鏁板紑澶寸殑鍥炶皟鍙互鏈夊叾浠栫敤渚嬨€傛棤璁烘槸鐢ㄤ簬鍐呮牳鐑ˉ涓侊紙live kernel patching锛夛紝杩樻槸鐢ㄤ簬瀹夊叏鐩戞帶銆傛湰鏂囨。鎻忚堪濡備綍浣跨敤 ftrace 瀹炵幇浣犺嚜宸辩殑鍑芥暟鍥炶皟銆?
## ftrace 涓婁笅鏂?
  鍚戝唴鏍镐腑鍑犱箮浠讳綍鍑芥暟娣诲姞鍥炶皟鐨勮兘鍔涗即闅忕潃椋庨櫓銆傚洖璋冨彲浠ヤ粠浠讳綍涓婁笅鏂囷紙鏅€氥€乻oftirq銆乮rq 鍜?NMI锛夎皟鐢ㄣ€傚洖璋冧篃鍙互鍦ㄥ嵆灏嗚繘鍏ョ┖闂层€丆PU 涓婄嚎鍜屼笅绾挎湡闂达紝鎴栧嵆灏嗚繘鍏ョ敤鎴风┖闂存椂璋冪敤銆傝繖瑕佹眰瀵瑰洖璋冨唴閮ㄥ彲浠ュ仛浠€涔堟牸澶栧皬蹇冦€傚洖璋冨彲浠ュ湪 RCU 鐨勪繚鎶よ寖鍥翠箣澶栬璋冪敤銆?
鏈夎緟鍔╁嚱鏁板彲浠ュ府鍔╅槻姝㈤€掑綊锛屽苟纭繚 RCU 姝ｅ湪鐩戣锛坵atching锛夈€傝繖浜涘皢鍦ㄤ笅闈㈣В閲娿€?
## ftrace_ops 缁撴瀯

瑕佹敞鍐屼竴涓嚱鏁板洖璋冿紝闇€瑕佷竴涓?ftrace_ops銆傛缁撴瀯鐢ㄤ簬鍛婅瘔 ftrace 鍝釜鍑芥暟搴斾綔涓哄洖璋冭璋冪敤锛屼互鍙婅鍥炶皟灏嗘墽琛屽摢浜涗繚鎶や粠鑰屼笉闇€瑕?ftrace 鏉ュ鐞嗐€?
鍦ㄥ悜 ftrace 娉ㄥ唽 ftrace_ops 鏃讹紝鍙渶瑕佽缃竴涓瓧娈碉細

 struct ftrace_ops ops = {
       .func			= my_callback_func,
       .flags			= MY_FTRACE_FLAGS
       .private			= any_private_data_structure,
 };

.flags 鍜?.private 閮芥槸鍙€夌殑銆傚彧鏈?.func 鏄繀闇€鐨勩€?
```

    register_ftrace_function(&ops);

```
```

    unregister_ftrace_function(&ops);

```
```

    #include <linux/ftrace.h>

```
娉ㄥ唽鐨勫洖璋冨皢鍦?register_ftrace_function() 琚皟鐢ㄤ箣鍚庛€佽繑鍥炰箣鍓嶇殑鏌愪釜鏃跺埢寮€濮嬭璋冪敤銆傚洖璋冨紑濮嬭璋冪敤鐨勭‘鍒囨椂闂村彇鍐充簬鏋舵瀯鍜屾湇鍔＄殑璋冨害銆傚鏋滃洖璋冨繀椤诲湪绮剧‘鏃跺埢寮€濮嬶紝鍒欏畠鑷繁蹇呴』澶勭悊浠讳綍鍚屾銆?
unregister_ftrace_function() 灏嗕繚璇佸湪 unregister_ftrace_function() 杩斿洖涔嬪悗锛屽嚱鏁颁笉鍐嶈皟鐢ㄨ鍥炶皟銆傛敞鎰忥紝涓轰簡鎵ц杩欎竴淇濊瘉锛寀nregister_ftrace_function() 鍙兘闇€瑕佷竴浜涙椂闂存潵瀹屾垚銆?
## 鍥炶皟鍑芥暟

鍥炶皟鍑芥暟鐨勫師鍨嬪涓嬶紙鑷?v4.14 璧凤級锛?
   void callback_func(unsigned long ip, unsigned long parent_ip,
                      struct ftrace_ops **op, struct pt_regs **regs);

@ip
	 杩欐槸姝ｅ湪琚拷韪殑鍑芥暟鐨勬寚浠ゆ寚閽堛€?      	 锛坒entry 鎴?mcount 鍦ㄥ嚱鏁扮殑浣嶇疆锛?
@parent_ip
	 杩欐槸璋冪敤浜嗚杩借釜鍑芥暟鐨勫嚱鏁扮殑鎸囦护鎸囬拡
	锛堝嚱鏁拌皟鐢ㄥ彂鐢熺殑浣嶇疆锛夈€?
@op
	 杩欐槸鎸囧悜鐢ㄤ簬娉ㄥ唽璇ュ洖璋冪殑 ftrace_ops 鐨勬寚閽堛€?	 杩欏彲鐢ㄤ簬閫氳繃 private 鎸囬拡鍚戝洖璋冧紶閫掓暟鎹€?
@regs
	 濡傛灉鍦?ftrace_ops 缁撴瀯涓缃簡 FTRACE_OPS_FL_SAVE_REGS 鎴?	 FTRACE_OPS_FL_SAVE_REGS_IF_SUPPORTED 鏍囧織锛岄偅涔堣繖灏嗘寚鍚?	 pt_regs 缁撴瀯锛屽氨鍍忓湪 ftrace 鎵€杩借釜鐨勫嚱鏁板紑澶存斁缃簡涓€涓?	 鏂偣涓€鏍枫€傚惁鍒欏畠瑕佷箞鍖呭惈鍨冨溇鏁版嵁锛岃涔堜负 NULL銆?
## 淇濇姢浣犵殑鍥炶皟

鐢变簬鍑芥暟鍙互浠庝换浣曞湴鏂硅皟鐢紝骞朵笖鍥炶皟璋冪敤鐨勫嚱鏁颁篃鍙兘琚拷韪€佸苟璋冪敤鍚屼竴涓洖璋冿紝鍥犳蹇呴』浣跨敤閫掑綊淇濇姢銆傚湪杩欐柟闈㈡湁涓や釜杈呭姪鍑芥暟鍙互鎻愪緵甯姪銆傚鏋滀綘杩欐牱寮€濮嬩綘鐨勪唬鐮侊細

 int bit;

	bit = ftrace_test_recursion_trylock(ip, parent_ip);
	if (bit < 0)
		return;

骞朵互杩欐牱缁撴潫锛?
	ftrace_test_recursion_unlock(bit);

閭ｄ箞涓棿鍖呭惈鐨勪唬鐮佸皢鍙畨鍏ㄤ娇鐢紝鍗充娇瀹冩渶缁堣皟鐢ㄤ簡鍥炶皟姝ｅ湪杩借釜鐨勫嚱鏁般€傛敞鎰忥紝鎴愬姛鏃?ftrace_test_recursion_trylock() 灏嗙鐢ㄦ姠鍗狅紝鑰?ftrace_test_recursion_unlock() 灏嗗啀娆″惎鐢紙濡傛灉涔嬪墠宸插惎鐢級銆傛寚浠ゆ寚閽堬紙ip锛夊強鍏剁埗鎸囬拡锛坧arent_ip锛夎浼犻€掔粰 ftrace_test_recursion_trylock() 浠ヨ褰曢€掑綊鍙戠敓鐨勪綅缃紙濡傛灉璁剧疆浜?CONFIG_FTRACE_RECORD_RECURSION锛夈€?
鎴栬€咃紝濡傛灉鍦?ftrace_ops 涓婅缃簡 FTRACE_OPS_FL_RECURSION 鏍囧織锛堝涓嬫墍杩帮級锛岄偅涔堝皢浣跨敤涓€涓緟鍔?trampoline 鏉ヤ负鍥炶皟娴嬭瘯閫掑綊锛屾棤闇€杩涜閫掑綊娴嬭瘯銆備絾杩欎唬浠锋槸鏉ヨ嚜棰濆鍑芥暟璋冪敤鐨勭暐寰洿澶氬紑閿€銆?
濡傛灉浣犵殑鍥炶皟璁块棶浠讳綍闇€瑕?RCU 淇濇姢鐨勬暟鎹垨涓寸晫鍖猴紝鏈€濂界‘淇?RCU 姝ｅ湪鈥滅洃瑙嗏€濓紝鍚﹀垯璇ユ暟鎹垨涓寸晫鍖哄皢涓嶄細鎸夐鏈熷彈鍒颁繚鎶ゃ€傚湪杩欑鎯呭喌涓嬫坊鍔狅細

	if (!rcu_is_watching())
		return;

鎴栬€咃紝濡傛灉鍦?ftrace_ops 涓婅缃簡 FTRACE_OPS_FL_RCU 鏍囧織锛堝涓嬫墍杩帮級锛岄偅涔堝皢浣跨敤涓€涓緟鍔?trampoline 鏉ヤ负鍥炶皟娴嬭瘯 rcu_is_watching锛屾棤闇€杩涜鍏朵粬娴嬭瘯銆備絾杩欎唬浠锋槸鏉ヨ嚜棰濆鍑芥暟璋冪敤鐨勭暐寰洿澶氬紑閿€銆?
## ftrace 鏍囧織

ftrace_ops 鏍囧織閮藉湪 include/linux/ftrace.h 涓畾涔夊拰璁板綍銆傚叾涓竴浜涙爣蹇楃敤浜?ftrace 鐨勫唴閮ㄥ熀纭€璁炬柦锛屼絾鐢ㄦ埛搴斿綋浜嗚В鐨勬爣蹇楀涓嬶細

FTRACE_OPS_FL_SAVE_REGS
	濡傛灉鍥炶皟闇€瑕佽鍙栨垨淇敼浼犻€掔粰鍥炶皟鐨?pt_regs锛屽垯蹇呴』璁剧疆姝ゆ爣蹇椼€傚湪涓嶆敮鎸佸皢 pt_regs 浼犻€掔粰鍥炶皟鐨勬灦鏋勪笂锛屾敞鍐屽甫鏈夋鏍囧織鐨?ftrace_ops 灏嗗け璐ャ€?
FTRACE_OPS_FL_SAVE_REGS_IF_SUPPORTED
	绫讳技浜?SAVE_REGS锛屼絾鍦ㄤ笉鏀寔浼犻€?regs 鐨勬灦鏋勪笂娉ㄥ唽 ftrace_ops 涓嶄細鍥犺缃簡姝ゆ爣蹇楄€屽け璐ャ€備絾鍥炶皟蹇呴』妫€鏌?regs 鏄惁涓?NULL 浠ョ‘瀹氳鏋舵瀯鏄惁鏀寔銆?
FTRACE_OPS_FL_RECURSION
	榛樿鎯呭喌涓嬶紝鏈熸湜鍥炶皟鑳藉澶勭悊閫掑綊銆備絾濡傛灉鍥炶皟涓嶅お鎷呭績寮€閿€锛岄偅涔堣缃浣嶅皢閫氳繃璋冪敤涓€涓緟鍔╁嚱鏁版潵涓哄洖璋冩坊鍔犻€掑綊淇濇姢锛岃杈呭姪鍑芥暟鎵ц閫掑綊淇濇姢锛屽苟涓斾粎鍦ㄤ笉閫掑綊鏃舵墠璋冪敤鍥炶皟銆?
	娉ㄦ剰锛屽鏋滄湭璁剧疆姝ゆ爣蹇楋紝涓斿彂鐢熶簡閫掑綊锛屽彲鑳藉鑷寸郴缁熷穿婧冿紝骞跺彲鑳介€氳繃涓夐噸閿欒锛坱riple fault锛夐噸鍚€?
	娉ㄦ剰锛屽鏋滆缃簡姝ゆ爣蹇楋紝閭ｄ箞鍥炶皟灏嗗缁堝湪绂佺敤鎶㈠崰鐨勬儏鍐典笅琚皟鐢ㄣ€傚鏋滄湭璁剧疆锛屽垯鍥炶皟鏈夊彲鑳斤紙浣嗕笉淇濊瘉锛夊湪鍙姠鍗犱笂涓嬫枃涓璋冪敤銆?
FTRACE_OPS_FL_IPMODIFY
	闇€瑕佽缃?FTRACE_OPS_FL_SAVE_REGS銆傚鏋滃洖璋冭鈥滃姭鎸佲€濊杩借釜鐨勫嚱鏁帮紙鐢ㄥ彟涓€涓嚱鏁颁唬鏇胯杩借釜鐨勫嚱鏁拌皟鐢級锛屽垯闇€瑕佽缃鏍囧織銆傝繖灏辨槸 live kernel patches锛堝唴鏍哥儹琛ヤ竵锛夋墍鐢ㄧ殑銆傛病鏈夋鏍囧織锛宲t_regs->ip 鏃犳硶琚慨鏀广€?
	娉ㄦ剰锛屼换浣曠粰瀹氬嚱鏁颁竴娆″彧鑳芥敞鍐屼竴涓缃簡 FTRACE_OPS_FL_IPMODIFY 鐨?ftrace_ops銆?
FTRACE_OPS_FL_RCU
	濡傛灉璁剧疆浜嗘鏍囧織锛岄偅涔堝洖璋冨皢鍙 RCU 姝ｅ湪鈥滅洃瑙嗏€濈殑鍑芥暟璋冪敤銆傚鏋滃洖璋冨嚱鏁版墽琛屼换浣?rcu_read_lock() 鎿嶄綔锛屽垯闇€瑕佹鏍囧織銆?
	RCU 鍦ㄧ郴缁熻繘鍏ョ┖闂层€丆PU 琚彇涓嬪拰閲嶆柊涓婄嚎锛屼互鍙婁粠鍐呮牳杩涘叆鐢ㄦ埛绌洪棿鍐嶅洖鍒板唴鏍哥┖闂存椂鍋滄鐩戣銆傚湪杩欎簺杞崲鏈熼棿锛屽洖璋冨彲鑳借鎵ц锛岃€?RCU 鍚屾涓嶄細淇濇姢瀹冦€?
FTRACE_OPS_FL_PERMANENT
        濡傛灉鍦ㄤ换浣?ftrace ops 涓婅缃簡姝ゆ爣蹇楋紝閭ｄ箞閫氳繃鍚?proc sysctl ftrace_enabled 鍐欏叆 0 鏃犳硶绂佺敤杩借釜銆傚悓鏍峰湴锛屽鏋?ftrace_enabled 涓?0锛屽垯鏃犳硶娉ㄥ唽璁剧疆浜嗚鏍囧織鐨勫洖璋冦€?
        Livepatch 浣跨敤瀹冧互閬垮厤涓㈠け鍑芥暟閲嶅畾鍚戯紝浠庤€岀郴缁熶繚鎸佸彈淇濇姢銆?
## 杩囨护瑕佽拷韪殑鍑芥暟

濡傛灉鍥炶皟鍙粠鐗瑰畾鍑芥暟璋冪敤锛屽垯蹇呴』璁剧疆杩囨护鍣ㄣ€傝繃婊ゅ櫒鎸夊悕绉版坊鍔狅紝濡傛灉宸茬煡涔熷彲鎸?ip 娣诲姞銆?
   int ftrace_set_filter(struct ftrace_ops **ops, unsigned char **buf,
                         int len, int reset);

@ops
	 鐢ㄤ簬璁剧疆杩囨护鍣ㄧ殑 ops

@buf
	 鎸佹湁鍑芥暟杩囨护鏂囨湰鐨勫瓧绗︿覆銆?@len
	 瀛楃涓茬殑闀垮害銆?
@reset
	 闈為浂琛ㄧず鍦ㄥ簲鐢ㄦ杩囨护鍣ㄤ箣鍓嶉噸缃墍鏈夎繃婊ゅ櫒銆?
杩囨护鍣ㄨ〃绀哄湪鍚敤杩借釜鏃跺簲鍚敤鍝簺鍑芥暟銆傚鏋?@buf 涓?NULL 涓旇缃簡 reset锛屽垯鎵€鏈夊嚱鏁伴兘灏嗚鍚敤浠ヤ緵杩借釜銆?
@buf 涔熷彲浠ユ槸 glob 琛ㄨ揪寮忥紝浠ュ惎鐢ㄦ墍鏈夊尮閰嶇壒瀹氭ā寮忕殑鍑芥暟銆?
璇峰弬闃?Documentation/trace/ftrace.rst 涓殑 Filter Commands锛堣繃婊ゅ懡浠わ級銆?
瑕佷粎杩借釜 schedule 鍑芥暟锛?
   ret = ftrace_set_filter(&ops, "schedule", strlen("schedule"), 0);

瑕佹坊鍔犳洿澶氬嚱鏁帮紝澶氭璋冪敤 ftrace_set_filter()锛屽皢 @reset 鍙傛暟璁句负 0銆傝绉婚櫎褰撳墠鐨勮繃婊ゅ櫒闆嗗苟鐢?@buf 瀹氫箟鐨勬柊鍑芥暟鏇挎崲瀹冿紝灏?@reset 璁句负闈為浂銆?
瑕佺Щ闄ゆ墍鏈夎杩囨护鐨勫嚱鏁板苟杩借釜鎵€鏈夊嚱鏁帮細

   ret = ftrace_set_filter(&ops, NULL, 0, 1);

鏈夋椂澶氫釜鍑芥暟鍏锋湁鐩稿悓鐨勫悕绉般€傝鍦ㄨ繖绉嶆儏鍐典笅杩借釜鐗瑰畾鍑芥暟锛屽彲浠ヤ娇鐢?ftrace_set_filter_ip()銆?
   ret = ftrace_set_filter_ip(&ops, ip, 0, 0);

灏界 ip 蹇呴』鏄嚱鏁板唴璋冪敤 fentry 鎴?mcount 鐨勫湴鍧€鎵€鍦ㄤ綅缃€傛鍑芥暟鐢?perf 鍜?kprobes 浣跨敤锛屽畠浠粠鐢ㄦ埛锛堥€氬父浣跨敤鍐呮牳鐨勮皟璇曚俊鎭級鑾峰彇 ip 鍦板潃銆?
濡傛灉浣跨敤 glob 璁剧疆杩囨护鍣紝鍑芥暟鍙互琚坊鍔犲埌涓€涓€渘otrace鈥濆垪琛紝璇ュ垪琛ㄥ皢闃绘杩欎簺鍑芥暟璋冪敤鍥炶皟銆?鈥渘otrace鈥濆垪琛ㄤ紭鍏堜簬鈥渇ilter鈥濆垪琛ㄣ€傚鏋滀袱涓垪琛ㄩ兘闈炵┖涓斿寘鍚浉鍚岀殑鍑芥暟锛屽垯浠讳綍鍑芥暟閮戒笉浼氳皟鐢ㄥ洖璋冦€?
绌虹殑鈥渘otrace鈥濆垪琛ㄨ〃绀哄厑璁歌繃婊ゅ櫒瀹氫箟鐨勬墍鏈夊嚱鏁拌杩借釜銆?
   int ftrace_set_notrace(struct ftrace_ops **ops, unsigned char **buf,
                          int len, int reset);

杩欐帴鍙椾笌 ftrace_set_filter() 鐩稿悓鐨勫弬鏁帮紝浣嗕細灏嗗畠鎵惧埌鐨勫嚱鏁版坊鍔犲埌涓嶈杩借釜鐨勫垪琛ㄤ腑銆傝繖鏄笌杩囨护鍣ㄥ垪琛ㄥ垎寮€鐨勫垪琛紝骞朵笖姝ゅ嚱鏁颁笉浼氫慨鏀硅繃婊ゅ櫒鍒楄〃銆?
闈為浂鐨?@reset 灏嗗湪鎶婂尮閰?@buf 鐨勫嚱鏁版坊鍔犲埌鍏朵腑涔嬪墠娓呴櫎鈥渘otrace鈥濆垪琛ㄣ€?
娓呴櫎鈥渘otrace鈥濆垪琛ㄤ笌娓呴櫎杩囨护鍣ㄥ垪琛ㄧ浉鍚?
  ret = ftrace_set_notrace(&ops, NULL, 0, 1);

杩囨护鍣ㄥ拰 notrace 鍒楄〃鍙互闅忔椂鏇存敼銆傚鏋滃彧搴旀湁涓€缁勫嚱鏁拌皟鐢ㄥ洖璋冿紝鏈€濂藉湪娉ㄥ唽鍥炶皟涔嬪墠璁剧疆杩囨护鍣ㄣ€備絾鏇存敼涔熷彲鑳藉湪鍥炶皟娉ㄥ唽涔嬪悗鍙戠敓銆?
濡傛灉杩囨护鍣ㄥ凡灏变綅锛屼笖 @reset 闈為浂锛屼笖 @buf 鍖呭惈鍖归厤鍑芥暟鐨?glob锛屽垯鍒囨崲灏嗗湪 ftrace_set_filter() 璋冪敤鏈熼棿鍙戠敓銆備换浣曟椂鍒婚兘涓嶄細鏈夋墍鏈夊嚱鏁伴兘璋冪敤鍥炶皟銆?
   ftrace_set_filter(&ops, "schedule", strlen("schedule"), 1);

   register_ftrace_function(&ops);

   msleep(10);

   ftrace_set_filter(&ops, "try_to_wake_up", strlen("try_to_wake_up"), 1);

涓庝互涓嬩笉鍚岋細

   ftrace_set_filter(&ops, "schedule", strlen("schedule"), 1);

   register_ftrace_function(&ops);

   msleep(10);

   ftrace_set_filter(&ops, NULL, 0, 1);

   ftrace_set_filter(&ops, "try_to_wake_up", strlen("try_to_wake_up"), 0);

鍥犱负鍚庤€呭湪閲嶇疆鏃堕棿鍜屾柊杩囨护鍣ㄨ缃椂闂翠箣闂翠細鏈変竴涓煭鏆傜殑鏃堕棿娈碉紝鎵€鏈夊嚱鏁伴兘浼氳皟鐢ㄥ洖璋冦€?