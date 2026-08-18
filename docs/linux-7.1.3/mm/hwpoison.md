## hwpoison


## 浠€涔堟槸 hwpoison锛?


鍗冲皢鎺ㄥ嚭鐨?Intel CPU 鏀寔浠庢煇浜涘唴瀛橀敊璇腑鎭㈠锛坄MCA recovery`锛夈€傝繖瑕佹眰鎿嶄綔绯荤粺灏嗘煇涓〉澹版槑涓衡€滃凡涓瘨锛坧oisoned锛夆€濓紝鏉€姝讳笌涔嬪叧鑱旂殑杩涚▼锛屽苟鍦ㄤ粖鍚庨伩鍏嶄娇鐢ㄨ椤点€?

璇ヨˉ涓侀泦鍦?VM锛堣櫄鎷熷唴瀛樼鐞嗭級涓疄鐜颁簡鎵€闇€鐨勫熀纭€璁炬柦銆?

```
	楂樺眰鏈哄櫒妫€鏌ュ鐞嗙▼搴忋€傚鐞嗙敱纭欢鎶ュ憡涓哄凡鎹熷潖鐨勯〉锛岄€氬父鏄敱浜?2 浣?ECC 鍐呭瓨鎴?
	缂撳瓨鏁呴殰鎵€鑷淬€?

	杩欎晶閲嶄簬鍦ㄥ悗鍙版娴嬩负宸叉崯鍧忕殑椤点€傚綋褰撳墠 CPU 璇曞浘娑堣垂鎹熷潖鏁版嵁鏃讹紝姝ｅ湪杩愯鐨?
	杩涚▼鍙互鐩存帴琚潃姝汇€傝繖鎰忓懗鐫€濡傛灉鐢变簬鏌愮鍘熷洜鏃犳硶澶勭悊璇ラ敊璇紝鍙互瀹夊叏鍦板拷鐣ュ畠锛?
	鍥犱负灏氭湭娑堣垂浠讳綍鎹熷潖鏁版嵁銆傚弽涔嬶紝褰撹繖绉嶆儏鍐靛彂鐢熸椂锛屼細鍐嶆鍙戠敓涓€娆℃満鍣ㄦ鏌ャ€?

	澶勭悊澶勪簬鍚勭鐘舵€佺殑椤电紦瀛橀〉銆傝繖閲屾鎵嬬殑閮ㄥ垎鍦ㄤ簬锛屾垜浠彲浠ュ紓姝ヤ簬鍏朵粬 VM 鐢ㄦ埛
	璁块棶浠绘剰椤碉紝鍥犱负鍐呭瓨鏁呴殰鍙兘闅忔椂闅忓湴鍦ㄤ换浣曞湴鏂瑰彂鐢燂紝鍙兘浼氳繚鍙嶅畠浠殑涓€浜?
	鍋囪銆傝繖灏辨槸涓轰粈涔堟浠ｇ爜蹇呴』鏋佷负灏忓績銆傞€氬父瀹冨皾璇曚娇鐢ㄦ甯哥殑鍔犻攣瑙勫垯锛屽嵆鑾峰彇
	鏍囧噯閿侊紝鍗充娇杩欐剰鍛崇潃閿欒澶勭悊鍙兘闇€瑕佽緝闀跨殑鏃堕棿銆?

	杩欓噷鐨勬煇浜涙搷浣滄晥鐜囪緝浣庝笖鍏锋湁闈炵嚎鎬х殑绠楁硶澶嶆潅搴︼紝鍥犱负鏁版嵁缁撴瀯灏氭湭閽堝杩欑
	鎯呭喌浼樺寲銆備粠 vma 鍒拌繘绋嬬殑鏄犲皠灏ゅ叾濡傛銆傜敱浜庤繖绉嶆儏鍐甸璁″緢灏戣锛屾垜浠笇鏈涘彲浠?
	鎺ュ彈杩欎竴鐐广€俙``
浠ｇ爜鐢?mm/memory-failure.c 涓殑楂樺眰澶勭悊绋嬪簭銆佷竴涓柊鐨勯〉涓瘨鏍囧織锛屼互鍙婂湪 VM 涓敤浜庡鐞嗕腑姣掗〉鐨勫悇绉嶆鏌ョ粍鎴愩€?

鐩墠鐨勪富瑕佺洰鏍囨槸 KVM 瀹㈡埛鏈猴紝浣嗗畠涔熼€傜敤浜庡悇绫诲簲鐢ㄧ▼搴忋€侹VM 鏀寔闇€瑕佽緝鏂扮殑 qemu-kvm 鐗堟湰銆?

涓轰簡 KVM 鐨勪娇鐢紝闇€瑕佷竴绉嶆柊鐨勪俊鍙风被鍨嬶紝浠ヤ究 KVM 鑳藉灏嗘満鍣ㄦ鏌ヤ互姝ｇ‘鐨勫湴鍧€娉ㄥ叆鍒板鎴锋満涓€傝繖鍦ㄧ悊璁轰笂涔熷厑璁稿叾浠栧簲鐢ㄧ▼搴忓鐞嗗唴瀛樻晠闅溿€傞璁″ぇ澶氭暟搴旂敤绋嬪簭涓嶄細杩欎箞鍋氾紝浣嗘煇浜涢潪甯镐笓鐢ㄧ殑搴旂敤绋嬪簭鍙兘浼氥€?

## 鏁呴殰鎭㈠妯″紡


鍐呭瓨鏁呴殰鎭㈠鍙互澶勪簬涓ょ锛堝疄闄呬笂鏄笁绉嶏級妯″紡锛?

vm.memory_failure_recovery sysctl 璁句负闆讹細
	鎵€鏈夊唴瀛樻晠闅滈兘浼氬鑷?panic銆備笉瑕佸皾璇曟仮澶嶃€?

early kill
	锛堝彲鍦ㄥ叏灞€鍜屾瘡涓繘绋嬬骇鍒帶鍒讹級
	涓€鏃︽娴嬪埌閿欒灏卞悜搴旂敤绋嬪簭鍙戦€?SIGBUS
	杩欏厑璁歌兘澶熶互娓╁拰鏂瑰紡澶勭悊鍐呭瓨閿欒鐨勫簲鐢ㄧ▼搴?
	锛堜緥濡備涪寮冨彈褰卞搷瀵硅薄锛?
	杩欐槸 KVM qemu 浣跨敤鐨勬ā寮忋€?

late kill
	褰撳簲鐢ㄧ▼搴忛亣鍒版崯鍧忕殑椤垫椂鍙戦€?SIGBUS銆?
	杩欏浜庢病鏈夊唴瀛橀敊璇劅鐭ョ殑搴旂敤绋嬪簭鏈€鍚堥€傦紝涔熸槸榛樿妯″紡
	娉ㄦ剰鏌愪簺椤靛缁堟寜 late kill 鏂瑰紡澶勭悊銆?

## 鐢ㄦ埛鎺у埗


vm.memory_failure_recovery
	瑙?sysctl.txt

vm.memory_failure_early_kill
	鍦ㄥ叏灞€鍚敤 early kill 妯″紡

PR_MCE_KILL
	璁剧疆 early/late kill 妯″紡 / 鎭㈠涓虹郴缁熼粯璁?

	arg1: PR_MCE_KILL_CLEAR:
		鎭㈠涓虹郴缁熼粯璁?
	arg1: PR_MCE_KILL_SET:
		arg2 瀹氫箟绾跨▼鐗瑰畾鐨勬ā寮?

		PR_MCE_KILL_EARLY:
			Early kill
		PR_MCE_KILL_LATE:
			Late kill
		PR_MCE_KILL_DEFAULT
			浣跨敤绯荤粺鍏ㄥ眬榛樿

	娉ㄦ剰锛屽鏋滀綘甯屾湜鏈変竴涓笓鐢ㄧ嚎绋嬩唬琛ㄨ繘绋嬪鐞?
	SIGBUS(BUS_MCEERR_AO)锛屼綘搴旇鍦ㄦ寚瀹氱嚎绋嬩笂璋冪敤
	prctl(PR_MCE_KILL_EARLY)銆傚惁鍒欙紝SIGBUS 浼氳鍙戦€佺粰涓荤嚎绋嬨€?

PR_MCE_KILL_GET
	杩斿洖褰撳墠妯″紡

## 娴嬭瘯


- madvise(MADV_HWPOISON, ....)锛堜互 root 韬唤锛? 鍦ㄨ繘绋嬩腑姣掑寲涓€涓〉浠ョ敤浜庢祴璇?

- 閫氳繃 debugfs `/sys/kernel/debug/hwpoison/` 鐨?hwpoison-inject 妯″潡

  corrupt-pfn
	鍚戝洖鏄惧埌璇ユ枃浠剁殑 PFN 澶勬敞鍏?hwpoison 鏁呴殰銆傝繖浼氬仛涓€浜?
	鏃╂湡杩囨护锛屼互閬垮厤鍦ㄦ祴璇曞浠朵腑鎹熷潖闈為鏈熺殑椤点€?

  unpoison-pfn
	瀵瑰洖鏄惧埌璇ユ枃浠剁殑 PFN 澶勭殑椤佃繘琛岃蒋浠惰В姣掋€傝繖鏍?
	璇ラ〉鍙互鍐嶆琚娇鐢ㄣ€傝繖浠呭 Linux 娉ㄥ叆鐨勬晠闅滄湁鏁堬紝
	瀵圭湡瀹炵殑鍐呭瓨鏁呴殰鏃犳晥銆備竴鏃﹀彂鐢熶换浣曠‖浠跺唴瀛樻晠闅滐紝
	璇ョ壒鎬у皢琚鐢ㄣ€?

  娉ㄦ剰杩欎簺娉ㄥ叆鎺ュ彛骞朵笉绋冲畾锛屽彲鑳戒細鍦ㄤ笉鍚屽唴鏍哥増鏈箣闂村彂鐢熷彉鍖?

  corrupt-filter-dev-major, corrupt-filter-dev-minor
	浠呭鐞嗕笌鐢卞潡璁惧涓?娆¤澶囧彿瀹氫箟
	鐨勬枃浠剁郴缁熺浉鍏宠仈鐨勯〉銆?1U 涓洪€氶厤鍊笺€傝繖搴斾粎鐢ㄤ簬
	浜哄伐娉ㄥ叆鐨勬祴璇曘€?

  corrupt-filter-memcg
	灏嗘敞鍏ラ檺鍒朵负灞炰簬 memgroup 鐨勯〉銆傜敱 memcg 鐨?inode
	鍙锋寚瀹氥€?

```

		mkdir /sys/fs/cgroup/mem/hwpoison

	        usemem -m 100 -s 1000 &
		echo `jobs -p` > /sys/fs/cgroup/mem/hwpoison/tasks

		memcg_ino=$(ls -id /sys/fs/cgroup/mem/hwpoison | cut -f1 -d' ')
		echo $memcg_ino > /debug/hwpoison/corrupt-filter-memcg

		page-types -p `pidof init`   --hwpoison  # shall do nothing
		page-types -p `pidof usemem` --hwpoison  # poison its pages

  corrupt-filter-flags-mask, corrupt-filter-flags-value
	褰撴寚瀹氭椂锛屼粎褰?((page_flags & mask) == value)
	鏃舵墠姣掑寲椤点€傝繖鍏佽瀵瑰绉嶇被鍨嬬殑椤佃繘琛屽帇鍔涙祴璇曘€?
	page_flags 涓?/proc/kpageflags 涓浉鍚屻€傛爣蹇椾綅瀹氫箟浜?
	include/linux/kernel-page-flags.h锛屽苟鍦?
	Documentation/admin-guide/mm/pagemap.rst 涓湁鏂囨。璇存槑銆?

```
- 鏋舵瀯鐗瑰畾鐨?MCE 娉ㄥ叆鍣?

  x86 鏈?mce-inject銆乵ce-test

  mce-test 涓竴浜涘彲绉绘鐨?hwpoison 娴嬭瘯绋嬪簭锛岃涓嬫枃銆?

```
## 鍙傝€冭祫鏂?


http://halobates.de/mce-lc09-2.pdf
	LinuxCon 09 涓婄殑姒傝堪婕旇

git://git.kernel.org/pub/scm/utils/cpu/mce/mce-test.git
	娴嬭瘯濂椾欢锛堝彲绉绘鐨?hwpoison 涓撶敤娴嬭瘯浣嶄簬 tsrc锛?

git://git.kernel.org/pub/scm/utils/cpu/mce/mce-inject.git
	x86 鐗瑰畾鐨勬敞鍏ュ櫒


## 灞€闄愭€?

- 骞堕潪鎵€鏈夐〉绫诲瀷閮藉彈鏀寔锛屼篃姘歌繙涓嶄細鍏ㄩ儴鏀寔銆傚ぇ澶氭暟鍐呮牳鍐呴儴
  瀵硅薄鏃犳硶鎭㈠锛岀洰鍓嶄粎鏀寔 LRU 椤点€?

---
Andi Kleen, Oct 2009
