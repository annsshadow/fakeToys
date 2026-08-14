## Seccomp BPF锛圫ECure COMPuting with filters锛屽甫杩囨护鍣ㄧ殑瀹夊叏璁＄畻锛?

## 绠€浠?

澶ч噺绯荤粺璋冪敤瀵规瘡涓敤鎴锋€佽繘绋嬮兘鏄紑鏀剧殑锛屼絾鍏朵腑璁稿鍦ㄨ繘绋嬬殑鏁翠釜鐢熷懡鍛ㄦ湡涓兘涓嶄細琚娇鐢ㄣ€傞殢鐫€绯荤粺璋冪敤鐨勬紨杩涗笌鎴愮啛锛宐ug 琚彂鐜板苟琚秷闄ゃ€傛煇浜涚敤鎴锋€佸簲鐢ㄥ彈鐩婁簬鎷ユ湁涓€缁勬洿灏戝彲鐢ㄧ郴缁熻皟鐢ㄧ殑闆嗗悎銆傜敱姝ゅ緱鍒扮殑闆嗗悎鍑忓皬浜嗘毚闇茬粰搴旂敤绋嬪簭鐨勫唴鏍告€绘敾鍑婚潰銆傜郴缁熻皟鐢ㄨ繃婊ゆ鏄负杩欑被搴旂敤绋嬪簭鑰岃銆?
Seccomp 杩囨护鎻愪緵浜嗕竴绉嶆満鍒讹紝浣胯繘绋嬭兘澶熶负浼犲叆鐨勭郴缁熻皟鐢ㄦ寚瀹氳繃婊ゅ櫒銆傝杩囨护鍣ㄤ互 Berkeley Packet Filter锛圔PF锛夌▼搴忕殑褰㈠紡琛ㄨ揪锛屼笌濂楁帴瀛楄繃婊ゅ櫒绫讳技锛屽尯鍒湪浜庢墍鎿嶄綔鐨勬暟鎹笌姝ｅ湪杩涜鐨勭郴缁熻皟鐢ㄧ浉鍏筹細绯荤粺璋冪敤鍙峰拰绯荤粺璋冪敤鍙傛暟銆傝繖浣垮緱鑳藉浠ュ瘜浜庤〃杈惧姏鐨勬柟寮忚繃婊ょ郴缁熻皟鐢紝浣跨敤涓€绉嶆棭宸插鐢ㄦ埛鎬佸紑鏀俱€佷笖鏁版嵁闆嗙洿瑙傜殑杩囨护绋嬪簭璇█銆?
姝ゅ锛孊PF 浣垮緱 seccomp 鐨勪娇鐢ㄨ€呬笉浼氭拨涓烘鏌ユ椂闂?浣跨敤鏃堕棿锛圱OCTOU锛夋敾鍑荤殑鍙楀鑰咃紝杩欑被鏀诲嚮鍦ㄧ郴缁熻皟鐢ㄦ嫤鎴鏋朵腑寰堝父瑙併€侭PF 绋嬪簭涓嶈兘瑙ｅ紩鐢ㄦ寚閽堬紝杩欏氨灏嗘墍鏈夎繃婊ゅ櫒闄愬埗涓哄彧鑳界洿鎺ュ绯荤粺璋冪敤鍙傛暟姹傚€笺€?
## 瀹冨苟闈炰粈涔?

绯荤粺璋冪敤杩囨护骞堕潪娌欑銆傚畠鎻愪緵浜嗕竴绉嶅畾涔夋竻鏅扮殑鏈哄埗锛岀敤浜庢渶灏忓寲鏆撮湶鐨勫唴鏍告敾鍑婚潰銆傚畠鏄緵娌欑寮€鍙戣€呬娇鐢ㄧ殑宸ュ叿銆傞櫎姝や箣澶栵紝閽堝閫昏緫琛屼负涓庝俊鎭祦鐨勭瓥鐣ュ簲褰撶粨鍚堝叾浠栫郴缁熷姞鍥烘妧鏈€佷互鍙婏紙鍙兘鐨勮瘽锛変綘鎵€閫夋嫨鐨?LSM 鏉ョ鐞嗐€傚瘜浜庤〃杈惧姏銆佸姩鎬佺殑杩囨护鍣ㄦ部姝よ矾寰勬彁渚涗簡鏇村閫夐」锛堜緥濡傞伩鍏嶇梾鎬佽妯★紝鎴栭€夋嫨鍏佽 socketcall() 涓摢浜涘璺鐢ㄧ郴缁熻皟鐢級锛岃繖浜涘彲鑳借閿欒鍦扮悊瑙ｄ负鏇村畬鏁寸殑娌欑瑙ｅ喅鏂规銆?
## 鐢ㄦ硶


鏂板浜嗕竴绉?seccomp 妯″紡锛屽苟浣跨敤涓庝弗鏍?seccomp 鐩稿悓鐨?prctl(2) 璋冪敤鏉ュ惎鐢ㄣ€傚鏋滄灦鏋勫叿澶?`CONFIG_HAVE_ARCH_SECCOMP_FILTER`锛屽垯鍙寜濡備笅鏂瑰紡娣诲姞杩囨护鍣細

`PR_SET_SECCOMP`锛?	鐜板湪鎺ュ彈涓€涓澶栫殑鍙傛暟锛岀敤浜庨€氳繃 BPF 绋嬪簭鎸囧畾涓€涓柊杩囨护鍣ㄣ€傝 BPF 绋嬪簭灏嗗湪鍙嶆槧绯荤粺璋冪敤鍙枫€佸弬鏁板強鍏朵粬鍏冩暟鎹殑 struct seccomp_data 涓婃墽琛屻€傞殢鍚?BPF 绋嬪簭蹇呴』杩斿洖鏌愪釜鍙帴鍙楃殑鍊硷紝浠ュ憡鐭ュ唴鏍稿簲褰撻噰鍙栧摢涓姩浣溿€?
```
		prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, prog);

	The 'prog' argument is a pointer to a struct sock_fprog which
	will contain the filter program.  If the program is invalid, the
	call will return -1 and set errno to ``EINVAL``.

	If ``fork``/``clone`` and ``execve`` are allowed by @prog, any child
	processes will be constrained to the same filters and system
	call ABI as the parent.

	Prior to use, the task must call ``prctl(PR_SET_NO_NEW_PRIVS, 1)`` or
	run with ``CAP_SYS_ADMIN`` privileges in its namespace.  If these are not
	true, ``-EACCES`` will be returned.  This requirement ensures that filter
	programs cannot be applied to child processes with greater privileges
	than the task that installed them.

	Additionally, if ``prctl(2)`` is allowed by the attached filter,
	additional filters may be layered on which will increase evaluation
	time, but allow for further decreasing the attack surface during
	execution of a process.
```

涓婅堪璋冪敤鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖闈為浂鍊笺€?
## 杩斿洖鍊?

seccomp 杩囨护鍣ㄥ彲浠ヨ繑鍥炰笅鍒椾换鎰忓€笺€傚鏋滃瓨鍦ㄥ涓繃婊ゅ櫒锛屽缁欏畾绯荤粺璋冪敤姹傚€煎緱鍒扮殑杩斿洖鍊煎皢濮嬬粓閲囩敤浼樺厛绾ф渶楂樼殑鍊笺€傦紙渚嬪锛宍SECCOMP_RET_KILL_PROCESS` 鎬绘槸浼樺厛銆傦級

鎸夊叾浼樺厛绾ч『搴忓涓嬶細

`SECCOMP_RET_KILL_PROCESS`锛?	瀵艰嚧鏁翠釜杩涚▼绔嬪嵆閫€鍑猴紝涓斾笉鎵ц璇ョ郴缁熻皟鐢ㄣ€備换鍔＄殑閫€鍑虹姸鎬侊紙`status & 0x7f`锛夊皢鏄?`SIGSYS`锛岃€岄潪 `SIGKILL`銆?
`SECCOMP_RET_KILL_THREAD`锛?	瀵艰嚧璇ヤ换鍔＄珛鍗抽€€鍑猴紝涓斾笉鎵ц璇ョ郴缁熻皟鐢ㄣ€備换鍔＄殑閫€鍑虹姸鎬侊紙`status & 0x7f`锛夊皢鏄?`SIGSYS`锛岃€岄潪 `SIGKILL`銆?
`SECCOMP_RET_TRAP`锛?	瀵艰嚧鍐呮牳鍚戣Е鍙戣璋冪敤鐨勪换鍔″彂閫?`SIGSYS` 淇″彿锛屼笖涓嶆墽琛岃绯荤粺璋冪敤銆俙siginfo->si_call_addr` 灏嗘樉绀虹郴缁熻皟鐢ㄦ寚浠ょ殑鍦板潃锛岃€?`siginfo->si_syscall` 涓?`siginfo->si_arch` 灏嗘寚绀哄皾璇曚簡鍝釜绯荤粺璋冪敤銆傜▼搴忚鏁板櫒灏嗚〃鐜板緱濡傚悓绯荤粺璋冪敤宸茬粡鍙戠敓锛堝嵆瀹冧笉浼氭寚鍚戠郴缁熻皟鐢ㄦ寚浠わ級銆傝繑鍥炲€煎瘎瀛樺櫒灏嗗寘鍚竴涓笌鏋舵瀯鐩稿叧鐨勫€尖€斺€旇嫢鎭㈠鎵ц锛岃灏嗗叾璁句负鍚堢悊鐨勫€笺€傦紙涔嬫墍浠ヤ笌鏋舵瀯鐩稿叧锛屾槸鍥犱负鐢?`-ENOSYS` 鏇挎崲瀹冨彲鑳戒細瑕嗙洊涓€浜涙湁鐢ㄤ俊鎭€傦級

	杩斿洖鍊肩殑 `SECCOMP_RET_DATA` 閮ㄥ垎灏嗕綔涓?`si_errno` 浼犻€掋€?
	鐢?seccomp 瑙﹀彂鐨?`SIGSYS` 鍏?si_code 涓?`SYS_SECCOMP`銆?
`SECCOMP_RET_ERRNO`锛?	瀵艰嚧杩斿洖鍊肩殑浣?16 浣嶄綔涓?errno 浼犻€掔粰鐢ㄦ埛鎬侊紝涓斾笉鎵ц璇ョ郴缁熻皟鐢ㄣ€?
`SECCOMP_RET_USER_NOTIF`锛?	瀵艰嚧鍦ㄧ敤鎴锋€侀€氱煡 fd 涓婂彂閫佷竴鏉?`struct seccomp_notif` 娑堟伅锛堣嫢宸查檮鍔狅級锛屽惁鍒欏彂閫?`-ENOSYS`銆傚叧浜庡浣曞鐞嗙敤鎴烽€氱煡锛屽弬瑙佷笅鏂囪璁恒€?
`SECCOMP_RET_TRACE`锛?	褰撹繑鍥炴鍊兼椂锛屼細瀵艰嚧鍐呮牳鍦ㄦ墽琛岀郴缁熻皟鐢ㄤ箣鍓嶅皾璇曢€氱煡涓€涓熀浜?`ptrace()` 鐨勮窡韪櫒銆傝嫢涓嶅瓨鍦ㄨ窡韪櫒锛屽垯鍚戠敤鎴锋€佽繑鍥?`-ENOSYS`锛屼笖涓嶆墽琛岃绯荤粺璋冪敤銆?
	濡傛灉璺熻釜鍣ㄤ娇鐢?`ptrace(PTRACE_SETOPTIONS)` 璇锋眰浜?`PTRACE_O_TRACESECCOMP`锛屽畠灏变細鏀跺埌閫氱煡銆傝窡韪櫒灏嗘敹鍒?`PTRACE_EVENT_SECCOMP` 閫氱煡锛屼笖 BPF 绋嬪簭杩斿洖鍊肩殑 `SECCOMP_RET_DATA` 閮ㄥ垎鍙€氳繃 `PTRACE_GETEVENTMSG` 渚涜窡韪櫒鑾峰彇銆?
	璺熻釜鍣ㄥ彲浠ラ€氳繃灏嗙郴缁熻皟鐢ㄥ彿鏀逛负 -1 鏉ヨ烦杩囪绯荤粺璋冪敤銆傛垨鑰咃紝璺熻釜鍣ㄥ彲浠ラ€氳繃灏嗙郴缁熻皟鐢ㄦ敼涓轰竴涓湁鏁堢殑绯荤粺璋冪敤鍙锋潵鏀瑰彉鎵€璇锋眰鐨勭郴缁熻皟鐢ㄣ€傝嫢璺熻釜鍣ㄨ姹傝烦杩囪绯荤粺璋冪敤锛屽垯绯荤粺璋冪敤灏嗚〃鐜板緱濡傚悓杩斿洖璺熻釜鍣ㄦ斁鍏ヨ繑鍥炲€煎瘎瀛樺櫒涓殑鍊笺€?
	鍦ㄩ€氱煡璺熻釜鍣ㄤ箣鍚庯紝涓嶄細鍐嶈繍琛?seccomp 妫€鏌ャ€傦紙杩欐剰鍛崇潃鍩轰簬 seccomp 鐨勬矙绠卞湪鍏佽浣跨敤 ptrace 鏃跺繀椤绘瀬涓鸿皑鎱庯紝鍗充究鏄鍏朵粬宸叉矙绠卞寲鐨勮繘绋嬩篃鏄姝わ紱ptrace 璺熻釜鍣ㄥ彲鍒╃敤姝ゆ満鍒堕€冮€搞€傦級

`SECCOMP_RET_LOG`锛?	瀵艰嚧绯荤粺璋冪敤鍦ㄨ璁板綍涔嬪悗鎵ц銆傚簲鐢ㄥ紑鍙戣€呭簲浣跨敤瀹冩潵浜嗚В鍏跺簲鐢ㄧ▼搴忛渶瑕佸摢浜涚郴缁熻皟鐢紝鑰屾棤闇€鍙嶅缁忓巻澶氭娴嬭瘯涓庡紑鍙戝懆鏈熸潵鏋勫缓璇ュ垪琛ㄣ€?
	浠呭綋 actions_logged sysctl 瀛楃涓蹭腑鍖呭惈 "log" 鏃讹紝璇ュ姩浣滄墠浼氳璁板綍銆?
`SECCOMP_RET_ALLOW`锛?	瀵艰嚧绯荤粺璋冪敤琚墽琛屻€?
濡傛灉瀛樺湪澶氫釜杩囨护鍣紝瀵圭粰瀹氱郴缁熻皟鐢ㄦ眰鍊煎緱鍒扮殑杩斿洖鍊煎皢濮嬬粓閲囩敤浼樺厛绾ф渶楂樼殑鍊笺€?
浼樺厛绾т粎鐢?`SECCOMP_RET_ACTION` 鎺╃爜鍐冲畾銆傚綋澶氫釜杩囨护鍣ㄨ繑鍥炵浉鍚屼紭鍏堢骇鐨勫€兼椂锛屽彧浼氳繑鍥炴渶杩戝畨瑁呯殑杩囨护鍣ㄦ墍鎻愪緵鐨?`SECCOMP_RET_DATA`銆?
## 闄烽槺


浣跨敤杩囩▼涓渶闇€閬垮厤鐨勯櫡闃辨槸锛氫粎渚濇嵁绯荤粺璋冪敤鍙疯繘琛岃繃婊よ€屼笉妫€鏌ユ灦鏋勫€笺€備负浠€涔堬紵鍦ㄤ换浣曟敮鎸佸绉嶇郴缁熻皟鐢ㄨ皟鐢ㄧ害瀹氱殑鏋舵瀯涓婏紝绯荤粺璋冪敤鍙峰彲鑳介殢鍏蜂綋璋冪敤鏂瑰紡鑰屼笉鍚屻€傚鏋滀笉鍚岃皟鐢ㄧ害瀹氫腑鐨勭紪鍙峰彂鐢熼噸鍙狅紝杩囨护鍣ㄤ腑鐨勬鏌ュ氨鍙兘琚互鐢ㄣ€傚姟蹇呮鏌?arch 鍊硷紒

## 绀轰緥


`samples/seccomp/` 鐩綍涓棦鍖呭惈涓€涓?x86 鐗瑰畾鐨勭ず渚嬶紝涔熷寘鍚竴涓洿閫氱敤鐨勩€佺敤浜?BPF 绋嬪簭鐢熸垚鐨勯珮绾у畯鎺ュ彛绀轰緥銆?
## 鐢ㄦ埛鎬侀€氱煡


`SECCOMP_RET_USER_NOTIF` 杩斿洖鐮佷娇 seccomp 杩囨护鍣ㄨ兘澶熷皢鐗瑰畾鐨勭郴缁熻皟鐢ㄤ紶閫掔粰鐢ㄦ埛鎬佸鐞嗐€傝繖瀵逛簬瀹瑰櫒绠＄悊鍣ㄧ瓑搴旂敤鍙兘寰堟湁鐢紝瀹冧滑甯屾湜鎷︽埅鐗瑰畾鐨勭郴缁熻皟鐢紙`mount()`銆乣finit_module()` 绛夛級骞舵敼鍙樺叾琛屼负銆?
瑕佽幏鍙栭€氱煡 fd锛屽彲瀵?`seccomp()` 绯荤粺璋冪敤浣跨敤 `SECCOMP_FILTER_FLAG_NEW_LISTENER` 鍙傛暟锛?
    fd = seccomp(SECCOMP_SET_MODE_FILTER, SECCOMP_FILTER_FLAG_NEW_LISTENER, &prog);

璇ヨ皟鐢ㄦ垚鍔熸椂浼氳繑鍥炰竴涓拡瀵硅杩囨护鍣ㄧ殑 listener fd锛岄殢鍚庡彲閫氳繃 `SCM_RIGHTS` 鎴栫被浼兼満鍒朵紶閫掋€傛敞鎰忥紝filter fd 瀵瑰簲浜庣壒瀹氱殑杩囨护鍣紝鑰岄潪鐗瑰畾鐨勪换鍔°€傚洜姝わ紝濡傛灉璇ヤ换鍔￠殢鍚?fork锛屼袱涓换鍔＄殑 notifications 閮戒細鍑虹幇鍦ㄥ悓涓€涓?filter fd 涓娿€傚 filter fd 鐨勮鍐欎篃鏄悓姝ョ殑锛屽洜姝や竴涓?filter fd 鍙互瀹夊叏鍦版嫢鏈夊涓鍙栬€呫€?
seccomp 閫氱煡 fd 鐨勬帴鍙ｇ敱涓や釜缁撴瀯浣撶粍鎴愶細

    struct seccomp_notif_sizes {
        __u16 seccomp_notif;
        __u16 seccomp_notif_resp;
        __u16 seccomp_data;
    };

    struct seccomp_notif {
        __u64 id;
        __u32 pid;
        __u32 flags;
        struct seccomp_data data;
    };

    struct seccomp_notif_resp {
        __u64 id;
        __s64 val;
        __s32 error;
        __u32 flags;
    };

`struct seccomp_notif_sizes` 缁撴瀯浣撳彲鐢ㄤ簬纭畾 seccomp 閫氱煡涓墍鐢ㄥ悇绉嶇粨鏋勪綋鐨勫ぇ灏忋€俙struct seccomp_data` 鐨勫ぇ灏忔湭鏉ュ彲鑳戒細鏀瑰彉锛屽洜姝や唬鐮佸簲浣跨敤锛?
    struct seccomp_notif_sizes sizes;
    seccomp(SECCOMP_GET_NOTIF_SIZES, 0, &sizes);

鏉ョ‘瀹氳鍒嗛厤鐨勫悇绉嶇粨鏋勪綋鐨勫ぇ灏忋€傜ず渚嬪弬瑙?samples/seccomp/user-trap.c銆?
鐢ㄦ埛鍙€氳繃鍦?seccomp 閫氱煡 fd 涓婅皟鐢?`ioctl(SECCOMP_IOCTL_NOTIF_RECV)`锛堟垨 `poll()`锛夋潵璇诲彇骞舵帴鏀朵竴涓?`struct seccomp_notif`锛屽畠鍖呭惈浜斾釜鎴愬憳锛氱粨鏋勪綋鐨勮緭鍏ラ暱搴︺€佹瘡涓繃婊ゅ櫒鍞竴鐨?`id`銆佽Е鍙戣璇锋眰鐨勪换鍔＄殑 `pid`锛堣嫢璇ヤ换鍔″浜庣洃鍚€?pid 鍛藉悕绌洪棿涓嶅彲瑙佺殑 pid ns 涓紝鍒欏彲鑳戒负 0锛夈€傝閫氱煡杩樺寘鍚紶閫掔粰 seccomp 鐨?`data`锛屼互鍙婁竴涓繃婊ゅ櫒鏍囧織銆傚湪璋冪敤 ioctl 涔嬪墠锛屽簲灏嗚缁撴瀯浣撴竻闆躲€?
鐒跺悗鐢ㄦ埛鎬佸彲鍩轰簬杩欎簺淇℃伅鍋氬嚭鍐冲畾锛屽苟閫氳繃 `ioctl(SECCOMP_IOCTL_NOTIF_SEND)` 鍙戦€佷竴涓搷搴旓紝鎸囩ず搴旇繑鍥炵粰鐢ㄦ埛鎬佺殑鍐呭銆俙struct seccomp_notif_resp` 鐨?`id` 鎴愬憳搴斾笌 `struct seccomp_notif` 涓殑 `id` 鐩稿悓銆?
鐢ㄦ埛鎬佽繕鍙互閫氳繃 `ioctl(SECCOMP_IOCTL_NOTIF_ADDFD)` 鍚戦€氱煡杩涚▼娣诲姞鏂囦欢鎻忚堪绗︺€俙struct seccomp_notif_addfd` 鐨?`id` 鎴愬憳搴斾笌 `struct seccomp_notif` 涓殑 `id` 鐩稿悓銆俙newfd_flags` 鏍囧織鍙敤浜庡湪閫氱煡杩涚▼鐨勬枃浠舵弿杩扮涓婅缃濡?O_CLOEXEC 涔嬬被鐨勬爣蹇椼€傚鏋滅洃绠¤€咃紙supervisor锛夊笇鏈涗互鐗瑰畾缂栧彿娉ㄥ叆鏂囦欢鎻忚堪绗︼紝鍙互浣跨敤 `SECCOMP_ADDFD_FLAG_SETFD` 鏍囧織锛屽苟灏?`newfd` 鎴愬憳璁句负瑕佷娇鐢ㄧ殑鐗瑰畾缂栧彿銆傚鏋滆鏂囦欢鎻忚堪绗﹀凡鍦ㄩ€氱煡杩涚▼涓墦寮€锛屽垯浼氳鏇挎崲銆傜洃绠¤€呬篃鍙互娣诲姞涓€涓?FD锛屽苟閫氳繃浣跨敤 `SECCOMP_ADDFD_FLAG_SEND` 鏍囧織鍘熷瓙鍦颁綔鍑哄搷搴旓紝姝ゆ椂杩斿洖鍊煎皢鏄娉ㄥ叆鐨勬枃浠舵弿杩扮缂栧彿銆?
閫氱煡杩涚▼鍙兘琚姠鍗狅紝瀵艰嚧閫氱煡琚腑姝€傚綋璇曞浘浠ｈ〃閫氱煡杩涚▼鎵ц鑰楁椂杈冮暱銆佷笖閫氬父鍙噸璇曠殑鎿嶄綔锛堜緥濡傛寕杞芥枃浠剁郴缁燂級鏃讹紝杩欏彲鑳藉甫鏉ラ棶棰樸€備綔涓烘浛浠ｏ紝鍦ㄨ繃婊ゅ櫒瀹夎鏃讹紝鍙互璁剧疆 `SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV` 鏍囧織銆傝鏍囧織鐨勪綔鐢ㄦ槸锛氬綋鐩戠鑰呮敹鍒扮敤鎴烽€氱煡鏃讹紝閫氱煡杩涚▼灏嗗拷鐣ラ潪鑷村懡淇″彿锛岀洿鍒板搷搴旇鍙戦€併€傚湪閫氱煡琚敤鎴锋€佹帴鏀朵箣鍓嶅彂閫佺殑淇″彿鍒欑収甯稿鐞嗐€?
鍊煎緱娉ㄦ剰鐨勬槸锛宍struct seccomp_data` 鍖呭惈绯荤粺璋冪敤瀵勫瓨鍣ㄥ弬鏁扮殑鍊硷紝浣嗕笉鍖呭惈鎸囧悜鍐呭瓨鐨勬寚閽堛€備换鍔＄殑鍏у瓨鍙€氳繃 `ptrace()` 鎴?`/proc/pid/mem` 渚涙嫢鏈夌浉搴旀潈闄愮殑璺熻釜鍣ㄨ闂€備絾鏄紝搴旀敞鎰忛伩鍏嶆湰鏂囨。鍓嶈堪鎻愬埌鐨?TOCTOU锛氬湪鍋氬嚭浠讳綍绛栫暐鍐冲畾涔嬪墠锛屼粠琚窡韪€呭唴瀛樹腑璇诲彇鐨勬墍鏈夊弬鏁伴兘搴斿厛璇诲叆璺熻釜鍣ㄧ殑鍐呭瓨銆傝繖浣垮緱瀵圭郴缁熻皟鐢ㄥ弬鏁拌兘澶熷仛鍑哄師瀛愭€у喅瀹氥€?
## Sysctl 鍙傛暟


Seccomp 鐨?sysctl 鏂囦欢浣嶄簬 `/proc/sys/kernel/seccomp/` 鐩綍涓€備笅闈㈡弿杩拌鐩綍涓殑姣忎釜鏂囦欢锛?
`actions_avail`锛?	浠ュ瓧绗︿覆褰㈠紡缁欏嚭鐨勩€佸彧璇讳笖鏈夊簭鐨?seccomp 杩斿洖鍊煎垪琛紙鍙傝涓婃枃 `SECCOMP_RET_*` 瀹忥級銆傚叾浠庡乏鍒板彸鐨勬帓鍒楅『搴忎负浠庢渶涓嶅鏉剧殑杩斿洖鍊煎埌鏈€瀹芥澗鐨勮繑鍥炲€笺€?
	璇ュ垪琛ㄨ〃绀哄唴鏍告敮鎸佺殑 seccomp 杩斿洖鍊奸泦鍚堛€傜敤鎴锋€佺▼搴忓彲浣跨敤璇ュ垪琛ㄦ潵鍒ゆ柇锛氱▼搴忔瀯寤烘椂 `seccomp.h` 涓殑鍔ㄤ綔锛屼笌褰撳墠杩愯鍐呮牳瀹為檯鏀寔鐨勫姩浣滈泦鍚堟槸鍚︿笉鍚屻€?
`actions_logged`锛?	涓€涓彲璇诲啓鐨勩€佹湁搴忕殑 seccomp 杩斿洖鍊煎垪琛紙鍙傝涓婃枃 `SECCOMP_RET_*` 瀹忥級锛岃〃绀哄厑璁歌璁板綍鐨勮繑鍥炲€笺€傚啓鍏ヨ鏂囦欢鏃舵棤闇€鏈夊簭锛屼絾璇诲彇鏃跺皢浠ヤ笌 actions_avail sysctl 鐩稿悓鐨勬柟寮忔帓搴忋€?
	`actions_logged` sysctl 涓嶆帴鍙?`allow` 瀛楃涓诧紝鍥犱负鏃犳硶璁板綍 `SECCOMP_RET_ALLOW` 鍔ㄤ綔銆傚皾璇曞悜璇?sysctl 鍐欏叆 `allow` 浼氬鑷磋繑鍥?EINVAL銆?
## 娣诲姞鏋舵瀯鏀寔


鏉冨▉瑕佹眰鍙傝 `arch/Kconfig`銆備竴鑸潵璇达紝濡傛灉鏌愪釜鏋舵瀯鍚屾椂鏀寔 ptrace_event 涓?seccomp锛屽畠灏辫兘浠ュ皯閲忎慨琛ユ敮鎸?seccomp 杩囨护鍣細鍗?`SIGSYS` 鏀寔涓?seccomp 杩斿洖鍊兼鏌ャ€傜劧鍚庡畠鍙渶鍦ㄥ叾鏋舵瀯鐗瑰畾鐨?Kconfig 涓坊鍔?`CONFIG_HAVE_ARCH_SECCOMP_FILTER`銆?
## 娉ㄦ剰浜嬮」


vDSO 鍙兘瀵艰嚧鏌愪簺绯荤粺璋冪敤瀹屽叏鍦ㄧ敤鎴锋€佽繍琛岋紝褰撲綘鍦ㄤ笉鍚屾満鍣ㄤ笂杩愯绋嬪簭銆佽€岃繖浜涚▼搴忓洖閫€鍒扮湡瀹炵郴缁熻皟鐢ㄦ椂锛屼細閫犳垚鎰忓銆備负浜嗗湪 x86 涓婂敖閲忓噺灏戣繖绫绘剰澶栵紝鍔″繀鍦ㄦ祴璇曟椂灏?`/sys/devices/system/clocksource/clocksource0/current_clocksource` 璁句负绫讳技 `acpi_pm` 鐨勫€笺€?
鍦?x86-64 涓婏紝vsyscall 妯℃嫙榛樿鏄惎鐢ㄧ殑銆傦紙vsyscall 鏄?vDSO 璋冪敤鐨勬棫寮忓彉浣撱€傦級鐩墠锛岃妯℃嫙鐨?vsyscall 浼氶伒瀹?seccomp锛屼絾鏈変竴浜涙€紓涔嬪锛?
- `SECCOMP_RET_TRAP` 鐨勮繑鍥炲€间細灏?`si_call_addr` 璁句负鎸囧悜缁欏畾璋冪敤鐨?vsyscall 鍏ュ彛锛岃€岄潪 'syscall' 鎸囦护涔嬪悗鐨勫湴鍧€銆備换浣曞笇鏈涢噸鍚璋冪敤鐨勪唬鐮侀兘搴旀剰璇嗗埌锛?a) 涓€鏉?ret 鎸囦护宸茶妯℃嫙锛?b) 灏濊瘯鎭㈠绯荤粺璋冪敤浼氬啀娆¤Е鍙戞爣鍑嗙殑 vsyscall 妯℃嫙瀹夊叏妫€鏌伙紝浣垮緱鎭㈠绯荤粺璋冪敤鍩烘湰娌℃湁鎰忎箟銆?
- `SECCOMP_RET_TRACE` 鐨勮繑鍥炲€间細鍍忓線甯镐竴鏍峰悜璺熻釜鍣ㄥ彂淇″彿锛屼絾鏃犳硶浣跨敤 orig_rax 瀵勫瓨鍣ㄥ皢绯荤粺璋冪敤鏀逛负鍙︿竴涓郴缁熻皟鐢ㄣ€傚彧鑳藉皢鍏舵敼涓?-1 浠ヨ烦杩囧綋鍓嶈妯℃嫙鐨勮皟鐢ㄣ€備换浣曞叾浠栨敼鍔ㄩ兘鍙兘缁堟杩涚▼銆傝窡韪櫒鐪嬪埌鐨?rip 鍊煎皢鏄郴缁熻皟鐢ㄥ叆鍙ｅ湴鍧€锛涜繖涓庢甯歌涓轰笉鍚屻€傝窡韪櫒缁濅笉鍙慨鏀?rip 鎴?rsp銆傦紙涓嶈渚濊禆鍏朵粬鏀瑰姩鏉ョ粓姝㈣繘绋嬨€傚畠浠垨璁歌兘鐢熸晥銆備緥濡傦紝鍦ㄦ煇浜涘唴鏍镐笂锛岄€夋嫨涓€涓粎鍦ㄦ湭鏉ュ唴鏍镐腑瀛樺湪鐨勭郴缁熻皟鐢ㄤ細琚纭ā鎷燂紙閫氳繃杩斿洖 `-ENOSYS`锛夈€傦級

瑕佹娴嬭繖绉嶅彜鎬涓猴紝璇锋鏌?``addr & ~0x0C00 == 0xFFFFFFFFFF600000`銆傦紙瀵逛簬 `SECCOMP_RET_TRACE`锛屼娇鐢?rip锛涘浜?`SECCOMP_RET_TRAP`锛屼娇鐢?`siginfo->si_call_addr`銆傦級涓嶈妫€鏌ヤ换浣曞叾浠栨潯浠讹細鏈潵鍐呮牳鍙兘浼氭敼杩?vsyscall 妯℃嫙锛岃€屽綋鍓嶅唴鏍稿湪 vsyscall=native 妯″紡涓嬭涓轰篃浼氫笉鍚岋紝浣嗗湪杩欎簺鎯呭喌涓?`0xF...F600{0,4,8,C}00` 澶勭殑鎸囦护涓嶄細鏄郴缁熻皟鐢ㄣ€?
娉ㄦ剰锛岀幇浠ｇ郴缁熷嚑涔庝笉鍙兘浣跨敤 vsyscall鈥斺€斿畠浠槸閬楃暀鐗规€э紝涓旀瘮鏍囧噯绯荤粺璋冪敤鎱㈠緱澶氥€傛柊浠ｇ爜浼氫娇鐢?vDSO锛岃€岀敱 vDSO 鍙戣捣鐨勭郴缁熻皟鐢ㄤ笌姝ｅ父鐨勭郴缁熻皟鐢ㄦ棤娉曞尯鍒嗐€?