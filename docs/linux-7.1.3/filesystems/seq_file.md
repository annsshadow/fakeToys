
## seq_file 鎺ュ彛


	Copyright 2003 Jonathan Corbet <corbet@lwn.net>

	鏈枃浠舵渶鍒濇潵鑷?LWN.net 鐨勯┍鍔ㄧЩ妞嶏紙Driver Porting锛夌郴鍒楋紝浣嶄簬
	https://lwn.net/Articles/driver-porting/


璁惧椹卞姩锛堟垨鍏朵粬鍐呮牳缁勪欢锛夋湁璁稿鏂瑰紡鍚戠敤鎴锋垨绯荤粺绠＄悊鍛樻彁渚涗俊鎭€備竴绉嶆湁鐢ㄧ殑鎶€鏈槸鍦?debugfs銆?/proc 鎴栧叾浠栦綅缃垱寤鸿櫄鎷熸枃浠躲€傝櫄鎷熸枃浠跺彲浠ユ彁渚涗汉绫诲彲璇荤殑杈撳嚭锛屾棤闇€浠讳綍鐗规畩宸ュ叿绋嬪簭鍗冲彲鑾峰彇锛?瀹冧滑涔熻兘璁╄剼鏈紪鍐欒€呯殑宸ヤ綔鏇磋交鏉俱€傝櫄鎷熸枃浠剁殑浣跨敤閫愬勾澧為暱骞朵笉浠や汉鎰忓銆?
鐒惰€岋紝姝ｇ‘鍦板垱寤鸿繖浜涙枃浠朵竴鐩存湁鐐规鎵嬨€傝繑鍥炰竴涓瓧绗︿覆鐨勮櫄鎷熸枃浠跺苟涓嶉毦鍋氥€備絾濡傛灉杈撳嚭寰堥暱鈥斺€?瓒呰繃搴旂敤绋嬪簭鍗曟鎿嶄綔鍙兘璇诲彇鐨勯暱搴︹€斺€斾簨鎯呭氨鍙樺緱鏇村鏉備簡銆傚鐞嗗娆¤鍙栵紙鍜屽畾浣?lseek锛夐渶瑕?浠旂粏鍏虫敞璇诲彇鑰呭湪铏氭嫙鏂囦欢涓殑浣嶇疆鈥斺€旇繖涓綅缃緢鍙兘浣嶄簬鏌愪竴琛岃緭鍑虹殑涓棿銆備紶缁熶笂锛屽唴鏍镐腑鏈変笉灏?瀹炵幇鍦ㄨ繖鏂归潰鐘簡閿欍€?
鍐呮牳鐜板湪鍖呭惈涓€缁勶紙鐢?Alexander Viro 瀹炵幇锛夊嚱鏁帮紝鏃ㄥ湪璁╄櫄鎷熸枃浠剁殑鍒涘缓鑰呰交鏉惧湴鎶婁簨鎯呭仛瀵广€?
seq_file 鎺ュ彛閫氳繃 `<linux/seq_file.h>` 鎻愪緵銆俿eq_file 鏈変笁涓柟闈細

     - 涓€涓凯浠ｅ櫒锛坕terator锛夋帴鍙ｏ紝璁╄櫄鎷熸枃浠跺疄鐜拌兘澶熼€愭閬嶅巻瀹冩墍鍛堢幇鐨勫璞°€?
     - 涓€浜涚敤浜庢牸寮忓寲瀵硅薄浠ヤ究杈撳嚭鐨勫伐鍏峰嚱鏁帮紝鏃犻渶鎷呭績杈撳嚭缂撳啿鍖轰箣绫荤殑浜嬫儏銆?
     - 涓€缁勯鍒剁殑 file_operations锛屽疄鐜颁簡铏氭嫙鏂囦欢涓婄殑澶у鏁版搷浣溿€?
鎴戜滑灏嗛€氳繃涓€涓瀬鍏剁畝鍗曠殑绀轰緥鏉ヤ簡瑙?seq_file 鎺ュ彛锛氫竴涓彲鍔犺浇妯″潡锛屽畠鍒涘缓涓€涓悕涓?/proc/sequence
鐨勬枃浠躲€傝鏂囦欢鍦ㄨ璇诲彇鏃讹紝绠€鍗曞湴浜х敓涓€缁勯€掑鐨勬暣鏁板€硷紝姣忚涓€涓€傝繖涓簭鍒椾細涓€鐩存寔缁紝鐩村埌鐢ㄦ埛
澶卞幓鑰愬績鍘绘壘鐐瑰埆鐨勪簨鎯呭仛銆傝鏂囦欢鏄彲瀹氫綅鐨勶紙seekable锛夛紝涔熷氨鏄鍙互鎵ц绫讳技濡備笅鐨勬搷浣?
```

    dd if=/proc/sequence of=out1 count=1
    dd if=/proc/sequence skip=1 of=out2 count=1

```
鐒跺悗鎷兼帴杈撳嚭鏂囦欢 out1 鍜?out2 灏辫兘寰楀埌姝ｇ‘鐨勭粨鏋溿€傛槸鐨勶紝杩欐槸涓€涓畬鍏ㄦ棤鐢ㄧ殑妯″潡锛屼絾閲嶇偣鏄湪
涓嶈糠澶变簬鍏朵粬缁嗚妭鐨勬儏鍐典笅灞曠ず璇ユ満鍒舵槸濡備綍宸ヤ綔鐨勩€傦紙鎯宠鏌ョ湅璇ユā鍧楀畬鏁存簮鐮佺殑浜哄彲浠ュ湪
https://lwn.net/Articles/22359/ 鎵惧埌銆傦級

## 宸插簾寮冪殑 create_proc_entry


璇锋敞鎰忥紝涓婃枃鏂囩珷浣跨敤鐨勬槸 create_proc_entry锛岃鍑芥暟宸插湪

```

    -	entry = create_proc_entry("sequence", 0, NULL);
    -	if (entry)
    -		entry->proc_fops = &ct_file_ops;
    +	entry = proc_create("sequence", 0, NULL, &ct_file_ops);

```
## 杩唬鍣ㄦ帴鍙?

浣跨敤 seq_file 瀹炵幇铏氭嫙妯″潡鐨勬ā鍧楀繀椤诲疄鐜颁竴涓凯浠ｅ櫒瀵硅薄锛岃瀵硅薄鍏佽鍦ㄤ竴娆♀€滀細璇濃€濓紙澶ц嚧瀵瑰簲涓€娆?read() 绯荤粺璋冪敤锛夋湡闂撮€愭閬嶅巻鎰熷叴瓒ｇ殑鏁版嵁銆傚鏋滆凯浠ｅ櫒鑳藉绉诲姩鍒扮壒瀹氫綅缃€斺€斿氨鍍忓畠浠疄鐜扮殑鏂囦欢
閭ｆ牱锛屼笉杩囧彲浠ヨ嚜鐢卞湴鎶婁綅缃紪鍙锋槧灏勫埌浠绘剰鏂逛究鐨勫簭鍒椾綅缃€斺€旈偅涔堣凯浠ｅ櫒鍙渶瑕佸湪浼氳瘽鏈熼棿鏆傛椂瀛樺湪銆?濡傛灉杩唬鍣ㄦ棤娉曡交鏄撴壘鍒颁竴涓暟瀛椾綅缃紝浣嗗緢閫傚悎 first/next 鎺ュ彛锛屽垯璇ヨ凯浠ｅ櫒鍙互瀛樺偍鍦ㄧ鏈夋暟鎹尯涓紝
骞跺湪涓€涓細璇濆埌涓嬩竴涓細璇濅箣闂寸户缁€?
渚嬪锛屼竴涓粠琛ㄤ腑鏍煎紡鍖栭槻鐏瑙勫垯鐨?seq_file 瀹炵幇锛屽彲浠ユ彁渚涗竴涓畝鍗曠殑杩唬鍣紝鎶婁綅缃?N 瑙ｉ噴涓?閾句腑鐨勭 N 鏉¤鍒欍€備竴涓憟鐜版煇涓紙鍙兘鏄槗鍙樼殑锛夐摼琛ㄥ唴瀹圭殑 seq_file 瀹炵幇锛屽彲鑳戒細璁板綍涓€涓寚鍚?璇ラ摼琛ㄧ殑鎸囬拡锛屽墠鎻愭槸鍙互鍋氬埌杩欎竴鐐硅€屼笉浼氭湁褰撳墠浣嶇疆琚Щ闄ょ殑椋庨櫓銆?
鍥犳锛屽畾浣嶅彲浠ヤ互瀵规暟鎹敓鎴愯€呮渶鏈夋剰涔夌殑鏂瑰紡鏉ユ墽琛岋紝鑰屾暟鎹敓鎴愯€呮棤闇€鐭ラ亾浣嶇疆濡備綍杞崲涓鸿櫄鎷熸枃浠朵腑
鐨勫亸绉婚噺銆備竴涓槑鏄剧殑渚嬪鏄細浣嶇疆涓洪浂搴旇〃绀烘枃浠剁殑寮€濮嬨€?
/proc/sequence 鐨勮凯浠ｅ櫒鍙槸鎶婂皢瑕佽緭鍑虹殑涓嬩竴涓暟瀛楃殑璁℃暟浣滀负鍏朵綅缃€?
蹇呴』瀹炵幇鍥涗釜鍑芥暟鎵嶈兘浣胯凯浠ｅ櫒宸ヤ綔銆傜涓€涓悕涓?start()锛屽畠鍚姩涓€涓細璇濓紝骞朵互涓€涓綅缃綔涓哄弬鏁帮紝
杩斿洖涓€涓皢浠庤浣嶇疆寮€濮嬭鍙栫殑杩唬鍣ㄣ€備紶缁?start() 鐨?pos 鎬绘槸瑕佷箞涓洪浂锛岃涔堟槸鍓嶄竴涓細璇濅腑浣跨敤鐨?鏈€杩戜竴涓?pos銆?
瀵逛簬鎴戜滑鐨勭畝鍗曞簭鍒楃ず渚嬶紝

```

	static void *ct_seq_start(struct seq_file *s, loff_t *pos)
	{
	        loff_t *spos = kmalloc(sizeof(loff_t), GFP_KERNEL);
	        if (! spos)
	                return NULL;
	        *spos = *pos;
	        return spos;
	}

```
杩欎釜杩唬鍣ㄧ殑鏁翠釜鏁版嵁缁撴瀯灏辨槸涓€涓繚瀛樺綋鍓嶄綅缃殑鍗曚竴 loff_t 鍊笺€傚簭鍒楄凯浠ｅ櫒娌℃湁涓婇檺锛屼絾澶у鏁板叾浠?seq_file 瀹炵幇骞堕潪濡傛锛涘湪澶у鏁版儏鍐典笅锛宻tart() 鍑芥暟搴旇妫€鏌モ€滆秴杩囨枃浠舵湯灏锯€濈殑鎯呭喌锛屽苟鍦ㄥ繀瑕佹椂
杩斿洖 NULL銆?
瀵逛簬鏇村鏉傜殑搴旂敤锛宻eq_file 缁撴瀯鐨?private 瀛楁鍙敤浜庡湪浼氳瘽涔嬮棿淇濆瓨鐘舵€併€俿tart() 鍑芥暟杩樺彲浠ヨ繑鍥?涓€涓壒娈婂€?SEQ_START_TOKEN锛涘鏋滀綘甯屾湜鎸囩ず浣犵殑 show() 鍑芥暟锛堜笅鏂囨弿杩帮級鍦ㄨ緭鍑洪《閮ㄦ墦鍗颁竴涓ご閮紝
鍙互浣跨敤瀹冦€備笉杩?SEQ_START_TOKEN 鍙簲鍦ㄥ亸绉婚噺涓洪浂鏃朵娇鐢ㄣ€係EQ_START_TOKEN 瀵规牳蹇?seq_file
浠ｇ爜娌℃湁鐗规畩鍚箟銆傚畠浣滀负涓€绉嶄究鍒╂彁渚涳紝鐢ㄤ簬 start() 鍑芥暟涓?next() 鍜?show() 鍑芥暟涔嬮棿鐨勯€氫俊銆?
鎺ヤ笅鏉ヨ瀹炵幇鐨勫嚱鏁帮紝浠や汉鎯婅鍦帮紝鍙仛 next()锛涘畠鐨勫伐浣滄槸鎶婅凯浠ｅ櫒鍚戝墠绉诲姩鍒板簭鍒椾腑鐨勪笅涓€涓綅缃€?绀轰緥妯″潡鍙互绠€鍗曞湴灏嗕綅缃姞涓€锛涙洿鏈夌敤鐨勬ā鍧椾細鍋氬繀瑕佺殑宸ヤ綔鏉ラ亶鍘嗘煇鏁版嵁缁撴瀯銆俷ext() 鍑芥暟杩斿洖涓€涓?鏂拌凯浠ｅ櫒锛屽鏋滃簭鍒楃粨鏉熷垯杩斿洖

```

	static void *ct_seq_next(struct seq_file *s, void *v, loff_t *pos)
	{
	        loff_t *spos = v;
	        *pos = ++*spos;
	        return spos;
	}

```
next() 鍑芥暟搴旇鎶?`*pos` 璁剧疆涓轰竴涓?start() 鍙互鐢ㄦ潵鍦ㄥ簭鍒椾腑鎵惧埌鏂颁綅缃殑鍊笺€傚綋杩唬鍣ㄨ瀛樺偍鍦?绉佹湁鏁版嵁鍖轰腑銆佽€屼笉鏄湪姣忔 start() 鏃堕噸鏂板垵濮嬪寲鏃讹紝浠呬粎鎶?`*pos` 璁剧疆涓轰换鎰忛潪闆跺€硷紙闆舵€绘槸鍛婅瘔
start() 瑕侀噸鍚簭鍒楋級浼间箮灏辫冻澶熶簡銆備絾鐢变簬鍘嗗彶闂锛岃繖骞朵笉鍏呭垎銆?
鍘嗗彶涓婏紝璁稿 next() 鍑芥暟**娌℃湁**鍦ㄦ枃浠舵湯灏炬洿鏂?`*pos`銆傚鏋滆鍊奸殢鍚庤 start() 鐢ㄦ潵鍒濆鍖栬凯浠ｅ櫒锛?灏卞彲鑳藉鑷磋竟鐣屾儏鍐碉紝鍗冲簭鍒椾腑鐨勬渶鍚庝竴涓潯鐩湪鏂囦欢涓鎶ュ憡涓ゆ銆備负浜嗛樆姝㈣繖涓?bug 姝荤伆澶嶇噧锛屾牳蹇?seq_file 浠ｇ爜鐜板湪浼氬湪 next() 鍑芥暟涓嶆敼鍙?`*pos` 鐨勫€兼椂浜х敓涓€涓鍛娿€傚洜姝わ紝next() 鍑芥暟**蹇呴』**
鏀瑰彉 `*pos` 鐨勫€硷紝骞朵笖褰撶劧蹇呴』鎶婂畠璁剧疆涓轰竴涓潪闆跺€笺€?
stop() 鍑芥暟鍏抽棴涓€涓細璇濓紱瀹冪殑宸ヤ綔褰撶劧鏄竻鐞嗐€傚鏋滀负杩唬鍣ㄥ垎閰嶄簡鍔ㄦ€佸唴瀛橈紝stop() 灏辨槸閲婃斁瀹冪殑
鍦版柟锛涘鏋?start() 鑾峰彇浜嗕竴涓攣锛宻top() 蹇呴』閲婃斁閭ｄ釜閿併€傚湪 stop() 涔嬪墠鏈€鍚庝竴娆?next() 璋冪敤鎵€
璁剧疆鐨?`*pos` 鍊间細琚浣忥紝骞剁敤浜庝笅涓€浼氳瘽鐨勭涓€娆?start() 璋冪敤锛岄櫎闈炲璇ユ枃浠惰皟鐢ㄤ簡 lseek()锛涘湪
閭ｇ鎯呭喌涓?
```

	static void ct_seq_stop(struct seq_file *s, void *v)
	{
	        kfree(v);
	}

```
鏈€鍚庯紝show() 鍑芥暟搴旇鏍煎紡鍖栧綋鍓嶆寚鍚戠殑瀵硅薄

```

	static int ct_seq_show(struct seq_file *s, void *v)
	{
	        loff_t *spos = v;
	        seq_printf(s, "%lld\n", (long long)*spos);
	        return 0;
	}

```
濡傛灉涓€鍒囨甯革紝show() 鍑芥暟搴旇杩斿洖闆躲€備互甯歌鏂瑰紡杩斿洖涓€涓礋鐨勯敊璇爜琛ㄧず鍑轰簡鐐归棶棰橈紱瀹冧細琚紶鍥?鐢ㄦ埛绌洪棿銆傝繖涓嚱鏁颁篃鍙互杩斿洖 SEQ_SKIP锛岃繖浼氬鑷磋烦杩囧綋鍓嶆潯鐩紱濡傛灉 show() 鍑芥暟鍦ㄨ繑鍥?SEQ_SKIP
涔嬪墠宸茬粡浜х敓浜嗚緭鍑猴紝閭ｄ箞閭ｉ儴鍒嗚緭鍑轰細琚涪寮冦€?
鎴戜滑绋嶅悗浼氱湅 seq_printf()銆備絾棣栧厛锛岄€氳繃鍒涘缓涓€涓?seq_operations 缁撴瀯鏉ュ畬鎴?seq_file 杩唬鍣ㄧ殑
瀹氫箟

```

	static const struct seq_operations ct_seq_ops = {
	        .start = ct_seq_start,
	        .next  = ct_seq_next,
	        .stop  = ct_seq_stop,
	        .show  = ct_seq_show
	};

```
绋嶅悗鎴戜滑灏嗛渶瑕佽繖涓粨鏋勬潵鎶婃垜浠殑杩唬鍣ㄤ笌 /proc 鏂囦欢缁戝畾璧锋潵銆?
鍊煎緱涓€鎻愮殑鏄紝鐢?start() 杩斿洖骞惰鍏朵粬鍑芥暟鎿嶄綔鐨勮凯浠ｅ櫒鍊硷紝瀵逛簬 seq_file 浠ｇ爜鏉ヨ琚涓哄畬鍏ㄤ笉閫忔槑
锛坥paque锛夈€傚洜姝ゅ畠鍙互鏄换浣曟湁鍔╀簬閫愭閬嶅巻寰呰緭鍑烘暟鎹殑涓滆タ銆傝鏁板櫒鍙兘鏈夌敤锛屼絾瀹冧篃鍙互鏄竴涓?鐩存帴鎸囧悜鏁扮粍鎴栭摼琛ㄧ殑鎸囬拡銆傚彧瑕佺▼搴忓憳鎰忚瘑鍒板湪涓ゆ璋冪敤杩唬鍣ㄥ嚱鏁颁箣闂村彲鑳藉彂鐢熶换浣曚簨鎯咃紝鎬庝箞閮借銆?涓嶈繃锛宻eq_file 浠ｇ爜锛堟寜璁捐锛変笉浼氬湪 start() 鍜?stop() 鐨勮皟鐢ㄤ箣闂翠紤鐪狅紝鍥犳鍦ㄨ繖娈垫椂闂村唴鎸佹湁閿?鏄悎鐞嗙殑銆俿eq_file 浠ｇ爜鍦ㄨ凯浠ｅ櫒澶勪簬娲诲姩鐘舵€佹椂涔熶細閬垮厤鑾峰彇浠讳綍鍏朵粬閿併€?
鐢?start() 鎴?next() 杩斿洖鐨勮凯浠ｅ櫒鍊间繚璇佷細琚紶閫掔粰鍚庣画鐨?next() 鎴?stop() 璋冪敤銆傝繖浣垮緱璇稿鎵€
鑾峰彇鐨勯攣绛夎祫婧愯兘澶熻鍙潬鍦伴噴鏀俱€備絾鏄?*娌℃湁**淇濊瘉璇ヨ凯浠ｅ櫒浼氳浼犻€掔粰 show()锛屽敖绠″湪瀹炶返涓畠閫氬父
浼氳浼犻€掋€?

## 鏍煎紡鍖栬緭鍑?

seq_file 浠ｇ爜绠＄悊杩唬鍣ㄦ墍鍒涘缓杈撳嚭涓殑浣嶇疆锛屽苟灏嗗叾閫佸叆鐢ㄦ埛鐨勭紦鍐插尯銆備絾涓轰簡璁╁畠宸ヤ綔锛岃杈撳嚭蹇呴』
琚紶閫掔粰 seq_file 浠ｇ爜銆傚凡缁忓畾涔変簡涓€浜涘伐鍏峰嚱鏁版潵浣胯繖椤逛换鍔″彉寰楀鏄撱€?
澶у鏁颁唬鐮佸皢鐩存帴浣跨敤 seq_printf()锛屽畠鐨勫伐浣滄柟寮忎笌 printk() 闈炲父鐩镐技锛屼絾闇€瑕?seq_file 鎸囬拡浣滀负
鍙傛暟銆?
```

	seq_putc(struct seq_file *m, char c);
	seq_puts(struct seq_file *m, const char *s);
	seq_escape(struct seq_file *m, const char *s, const char *esc);

```
鍓嶄袱涓垎鍒緭鍑哄崟涓瓧绗﹀拰瀛楃涓诧紝姝ｅ浜轰滑鎵€鏈熸湜鐨勯偅鏍枫€俿eq_escape() 绫讳技浜?seq_puts()锛屼笉鍚?涔嬪鍦ㄤ簬 s 涓换浣曞睘浜庡瓧绗︿覆 esc 鐨勫瓧绗﹀湪杈撳嚭涓皢浠ュ叓杩涘埗褰㈠紡琛ㄧず銆?
```

	int seq_path(struct seq_file *m, const struct path *path,
		     const char *esc);
	int seq_path_root(struct seq_file *m, const struct path *path,
			  const struct path *root, const char *esc)

```
杩欓噷锛宲ath 鎸囩ず鎰熷叴瓒ｇ殑鏂囦欢锛宔sc 鏄竴缁勫簲鍦ㄨ緭鍑轰腑杞箟鐨勫瓧绗︺€傝皟鐢?seq_path() 灏嗚緭鍑虹浉瀵逛簬
褰撳墠杩涚▼鏂囦欢绯荤粺鏍圭殑璺緞銆傚鏋滈渶瑕佷笉鍚岀殑鏍癸紝鍙互涓?seq_path_root() 涓€璧蜂娇鐢ㄣ€傚鏋滄渶缁堝彂鐜版棤娉?浠?root 鍒拌揪 path锛宻eq_path_root() 杩斿洖 SEQ_SKIP銆?
```

	bool seq_has_overflowed(struct seq_file *m);

```
濡傛灉杩斿洖 true锛屽垯閬垮厤杩涗竴姝ヨ皟鐢?seq_<output>銆?
seq_has_overflowed 杩斿洖 true 鎰忓懗鐫€ seq_file 缂撳啿鍖哄皢琚涪寮冿紝骞朵笖 seq_show 鍑芥暟灏嗗皾璇曞垎閰嶄竴涓?鏇村ぇ鐨勭紦鍐插尯骞堕噸璇曟墦鍗般€?

## 璁╀竴鍒囪繍杞捣鏉?

鍒扮洰鍓嶄负姝紝鎴戜滑鏈変竴缁勪笉閿欑殑鍑芥暟锛屽畠浠彲浠ュ湪 seq_file 绯荤粺涓骇鐢熻緭鍑猴紝浣嗘垜浠繕娌℃湁鎶婂畠浠彉鎴?鐢ㄦ埛鍙鐨勬枃浠躲€傚湪鍐呮牳涓垱寤轰竴涓枃浠跺綋鐒堕渶瑕佸垱寤轰竴缁?file_operations 鏉ュ疄鐜拌鏂囦欢涓婄殑鎿嶄綔銆?seq_file 鎺ュ彛鎻愪緵浜嗕竴缁勯鍒舵搷浣滐紝瀹屾垚浜嗗ぇ閮ㄥ垎宸ヤ綔銆備笉杩囷紝铏氭嫙鏂囦欢鐨勪綔鑰呬粛鐒跺繀椤诲疄鐜?open()
鏂规硶鏉ユ妸涓€鍒囬兘鎸傛帴璧锋潵銆俹pen 鍑芥暟閫氬父寰堢畝鍗?
```

	static int ct_open(struct inode *inode, struct file *file)
	{
		return seq_open(file, &ct_seq_ops);
	}

```
杩欓噷锛屽 seq_open() 鐨勮皟鐢ㄦ帴鍙楁垜浠箣鍓嶅垱寤虹殑 seq_operations 缁撴瀯锛屽苟璁剧疆涓洪亶鍘嗚櫄鎷熸枃浠躲€?
鍦ㄦ垚鍔熸墦寮€鏃讹紝seq_open() 鎶?struct seq_file 鎸囬拡瀛樺偍鍦?file->private_data 涓€傚鏋滀綘鏈夋煇涓簲鐢紝
鍏朵腑鍚屼竴涓凯浠ｅ櫒鍙敤浜庡涓枃浠讹紝浣犲彲浠ユ妸浠绘剰鎸囬拡瀛樺偍鍦?seq_file 缁撴瀯鐨?private 瀛楁涓紱璇ュ€奸殢鍚?鍙杩唬鍣ㄥ嚱鏁板彇鍥炪€?
杩樻湁涓€涓?seq_open() 鐨勫寘瑁呭嚱鏁板彨 seq_open_private()銆傚畠 kmalloc 涓€鍧楀～闆剁殑鍐呭瓨锛屽苟鎶婃寚鍚戝畠鐨?鎸囬拡瀛樺偍鍦?seq_file 缁撴瀯鐨?private 瀛楁涓紝鎴愬姛鏃惰繑鍥?0銆傝

```

	static int ct_open(struct inode *inode, struct file *file)
	{
		return seq_open_private(file, &ct_seq_ops,
					sizeof(struct mystruct));
	}

```
杩樻湁涓€涓彉浣撳嚱鏁?__seq_open_private()锛屽畠鍔熻兘涓婂畬鍏ㄧ浉鍚岋紝鍙槸濡傛灉鎴愬姛锛屽畠浼氳繑鍥炴寚鍚戞墍鍒嗛厤鍐呭瓨鐨?鎸囬拡

```

	static int ct_open(struct inode *inode, struct file *file)
	{
		struct mystruct *p =
			__seq_open_private(file, &ct_seq_ops, sizeof(*p));

		if (!p)
			return -ENOMEM;

		p->foo = bar; /* 鍒濆鍖栨垜鐨勪笢瑗?*/
			...
		p->baz = true;

		return 0;
	}

```
鏈変竴涓搴旂殑 close 鍑芥暟 seq_release_private() 鍙敤锛屽畠浼氶噴鏀惧湪瀵瑰簲 open 涓垎閰嶇殑鍐呭瓨銆?
鍏朵粬鎰熷叴瓒ｇ殑鎿嶄綔鈥斺€攔ead()銆乴lseek() 鍜?release()鈥斺€斿叏閮ㄧ敱 seq_file 浠ｇ爜鏈韩瀹炵幇銆傚洜姝や竴涓櫄鎷?鏂囦欢鐨?
```

	static const struct file_operations ct_file_ops = {
	        .owner   = THIS_MODULE,
	        .open    = ct_open,
	        .read    = seq_read,
	        .llseek  = seq_lseek,
	        .release = seq_release
	};

```
杩樻湁涓€涓?seq_release_private()锛屽畠鍦ㄩ噴鏀剧粨鏋勪箣鍓嶆妸 seq_file private 瀛楁鐨勫唴瀹逛紶缁?kfree()銆?
鏈€鍚庝竴姝ユ槸鍒涘缓 /proc 鏂囦欢鏈韩銆傚湪绀轰緥涓?
```

	static int ct_init(void)
	{
	        struct proc_dir_entry *entry;

	        proc_create("sequence", 0, NULL, &ct_file_ops);
	        return 0;
	}

	module_init(ct_init);

```
鑰岃繖鍩烘湰涓婂氨鏄叏閮ㄤ簡銆?

## seq_list


濡傛灉浣犵殑鏂囦欢瑕侀亶鍘嗕竴涓摼琛紝浣犲彲鑳戒細鐢ㄥ埌杩欎簺

```

	struct list_head *seq_list_start(struct list_head *head,
	       		 		 loff_t pos);
	struct list_head *seq_list_start_head(struct list_head *head,
			 		      loff_t pos);
	struct list_head *seq_list_next(void *v, struct list_head *head,
					loff_t *ppos);

```
杩欎簺杈呭姪鍑芥暟浼氭妸 pos 瑙ｉ噴涓洪摼琛ㄤ腑鐨勪竴涓綅缃紝骞剁浉搴斿湴杩涜杩唬銆備綘鐨?start() 鍜?next() 鍑芥暟鍙渶
瑕佺敤涓€涓寚鍚戠浉搴?list_head 缁撴瀯鐨勬寚閽堟潵璋冪敤 `seq_list_*` 杈呭姪鍑芥暟銆?

## 鏋佺畝鐗堟湰


瀵逛簬鏋佸叾绠€鍗曠殑铏氭嫙鏂囦欢锛屾湁涓€涓洿绠€鍗曠殑鎺ュ彛銆備竴涓ā鍧楀彲浠ュ彧瀹氫箟 show() 鍑芥暟锛屽畠搴旇鍒涘缓铏氭嫙鏂囦欢
灏嗗寘鍚殑鎵€鏈夎緭鍑恒€傝鏂囦欢鐨?open() 鏂规硶闅忓悗

```

	int single_open(struct file *file,
	                int (*show)(struct seq_file *m, void *p),
	                void *data);

```
褰撹緭鍑烘椂鍒诲埌鏉ユ椂锛宻how() 鍑芥暟浼氳璋冪敤涓€娆°€備紶缁?single_open() 鐨?data 鍊煎彲浠ュ湪 seq_file 缁撴瀯鐨?private 瀛楁涓壘鍒般€備娇鐢?single_open() 鏃讹紝绋嬪簭鍛樺簲璇ュ湪 file_operations 缁撴瀯涓娇鐢?single_release()
鑰岄潪 seq_release()锛屼互閬垮厤鍐呭瓨娉勬紡銆?