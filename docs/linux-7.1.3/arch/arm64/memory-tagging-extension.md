## AArch64 Linux 涓殑鍐呭瓨鏍囪鎵╁睍锛圡TE锛?

浣滆€咃細Vincenzo Frascino <vincenzo.frascino@arm.com>
         Catalin Marinas <catalin.marinas@arm.com>

鏃ユ湡锛?020-02-25

鏈枃妗ｆ弿杩颁簡鍦?AArch64 Linux 涓彁渚涘唴瀛樻爣璁版墿灞曪紙Memory Tagging Extension锛?鍔熻兘鐨勭浉鍏冲唴瀹广€?
## 绠€浠?

鍩轰簬 ARMv8.5 鐨勫鐞嗗櫒寮曞叆浜嗗唴瀛樻爣璁版墿灞曪紙MTE锛夌壒鎬с€侻TE 鏋勫缓鍦?ARMv8.0 鐨?铏氭嫙鍦板潃鏍囪 TBI锛圱op Byte Ignore锛屽拷鐣ユ渶楂樺瓧鑺傦級鐗规€т箣涓婏紝骞跺厑璁歌蒋浠惰闂?鐗╃悊鍦板潃绌洪棿涓瘡涓?16 瀛楄妭绮掑害锛坓ranule锛夌殑涓€涓?4 浣嶅垎閰嶆爣璁帮紙allocation tag锛夈€?杩欐牱鐨勫唴瀛樿寖鍥村繀椤讳互 Normal-Tagged 鍐呭瓨灞炴€ф槧灏勩€傞€昏緫鏍囪锛坙ogical tag锛夊彇鑷?鐢ㄤ簬鍐呭瓨璁块棶鐨勮櫄鎷熷湴鍧€鐨勭 59-56 浣嶃€傚惎鐢ㄤ簡 MTE 鐨?CPU 浼氬皢閫昏緫鏍囪涓庡垎閰嶆爣璁?杩涜姣旇緝锛屽苟鍙兘鍦ㄤ簩鑰呬笉鍖归厤鏃讹紙鍙栧喅浜庣郴缁熷瘎瀛樺櫒鐨勯厤缃級寮曞彂寮傚父銆?
## 鐢ㄦ埛绌洪棿鏀寔


褰撻€夋嫨浜?`CONFIG_ARM64_MTE` 涓旂‖浠舵敮鎸佸唴瀛樻爣璁版墿灞曟椂锛屽唴鏍搁€氳繃 `HWCAP2_MTE`
鍚戠敤鎴风┖闂撮€氬憡璇ョ壒鎬с€?
### PROT_MTE


涓轰簡璁块棶鍒嗛厤鏍囪锛岀敤鎴疯繘绋嬪繀椤讳娇鐢?`mmap()` 鍜?`mprotect()` 鐨勪竴涓柊鐨?`prot`
鏍囧織锛屽湪涓€娈靛湴鍧€鑼冨洿涓婂惎鐢ㄦ爣璁帮紙Tagged锛夊唴瀛樺睘鎬э細

`PROT_MTE` - 椤靛厑璁歌闂?MTE 鍒嗛厤鏍囪銆?
杩欎簺椤甸娆℃槧灏勫埌鐢ㄦ埛鍦板潃绌洪棿鏃讹紝鍒嗛厤鏍囪琚涓?0锛屽苟鍦ㄥ啓鏃跺鍒讹紙copy-on-write锛?鏃朵繚鐣欍€俙MAP_SHARED` 鍙楁敮鎸侊紝鍒嗛厤鏍囪鍙互鍦ㄨ繘绋嬩箣闂村叡浜€?
**娉ㄦ剰**锛歚PROT_MTE` 浠呭彈 `MAP_ANONYMOUS` 鍜屽熀浜?RAM 鐨勬枃浠舵槧灏勶紙`tmpfs`銆乣memfd`锛?鏀寔銆傚皢鍏朵紶缁欏叾浠栫被鍨嬬殑鏄犲皠浼氬鑷磋繖浜涚郴缁熻皟鐢ㄨ繑鍥?`-EINVAL`銆?
**娉ㄦ剰**锛歚PROT_MTE` 鏍囧織锛堝強鐩稿簲鐨勫唴瀛樼被鍨嬶級涓嶈兘琚?`mprotect()` 娓呴櫎銆?
**娉ㄦ剰**锛氫娇鐢?`MADV_DONTNEED` 鍜?`MADV_FREE` 鐨?`madvise()` 鍐呭瓨鑼冨洿锛屽湪璇ョ郴缁?璋冪敤涔嬪悗鐨勪换浣曟椂鍊欓兘鍙兘琚竻闄ゅ垎閰嶆爣璁帮紙璁句负 0锛夈€?
### 鏍囪妫€鏌ラ敊璇紙Tag Check Faults锛?

褰撴煇鍦板潃鑼冨洿鍚敤浜?`PROT_MTE`锛屼笖璁块棶鏃堕€昏緫鏍囪涓庡垎閰嶆爣璁颁笉鍖归厤鏃讹紝鏈変笁绉嶅彲閰嶇疆鐨?琛屼负锛?
- **Ignore锛堝拷鐣ワ級** - 杩欐槸榛樿妯″紡銆侰PU锛堝拰鍐呮牳锛夊拷鐣ユ爣璁版鏌ラ敊璇€?
- **Synchronous锛堝悓姝ワ級** - 鍐呮牳鍚屾鍦板紩鍙戜竴涓?`SIGSEGV`锛屽叾涓?  `.si_code = SEGV_MTESERR` 涓?`.si_addr = <fault-address>`銆傚唴瀛樿闂笉浼氳鎵ц銆?  濡傛灉 `SIGSEGV` 琚嚭閿欑嚎绋嬪拷鐣ユ垨闃诲锛屾墍灞炶繘绋嬪皢琚粓姝㈠苟鐢熸垚 `coredump`銆?
- **Asynchronous锛堝紓姝ワ級** - 鍐呮牳鍦ㄥ嚭閿欑嚎绋嬩腑锛屽湪涓€涓垨澶氫釜鏍囪妫€鏌ラ敊璇箣鍚庡紓姝ュ湴
  寮曞彂涓€涓?`SIGSEGV`锛屽叾涓?`.si_code = SEGV_MTEAERR` 涓?`.si_addr = 0`锛堝嚭閿欏湴鍧€鏈煡锛夈€?
- **Asymmetric锛堥潪瀵圭О锛?* - 璇绘搷浣滄寜鍚屾妯″紡澶勭悊锛岃€屽啓鎿嶄綔鎸夊紓姝ユā寮忓鐞嗐€?
鐢ㄦ埛鍙互鎸夌嚎绋嬶紝浣跨敤 `prctl(PR_SET_TAGGED_ADDR_CTRL, flags, 0, 0, 0)` 绯荤粺璋冪敤
閫夋嫨涓婅堪妯″紡锛屽叾涓?`flags` 鍦?`PR_MTE_TCF_MASK` 浣嶅煙涓寘鍚互涓嬩换鎰忓€硷細

- `PR_MTE_TCF_NONE`  - **蹇界暐**鏍囪妫€鏌ラ敊璇?                         锛堣嫢涓庡叾浠栭€夐」缁勫悎鍒欒蹇界暐锛?- `PR_MTE_TCF_SYNC`  - **鍚屾**鏍囪妫€鏌ラ敊璇ā寮?- `PR_MTE_TCF_ASYNC` - **寮傛**鏍囪妫€鏌ラ敊璇ā寮?
濡傛灉鏈寚瀹氫换浣曟ā寮忥紝鏍囪妫€鏌ラ敊璇皢琚拷鐣ャ€傚鏋滃彧鎸囧畾浜嗗崟涓€妯″紡锛岀▼搴忓皢鍦ㄨ妯″紡涓?杩愯銆傚鏋滄寚瀹氫簡澶氫釜妯″紡锛屽垯鎸変笅鏂団€滄瘡 CPU 鍋忓ソ鐨勬爣璁版鏌ユā寮忊€濅竴鑺傛墍杩伴€夋嫨妯″紡銆?
褰撳墠鐨勬爣璁版鏌ラ敊璇厤缃彲浠ヤ娇鐢?`prctl(PR_GET_TAGGED_ADDR_CTRL, 0, 0, 0, 0)` 绯荤粺
璋冪敤璇诲彇銆傚鏋滆姹備簡澶氫釜妯″紡锛屽垯鍏ㄩ儴閮戒細琚姤鍛娿€?
鏍囪妫€鏌ヤ篃鍙互閫氳繃璁剧疆 `PSTATE.TCO` 浣嶏紙浣跨敤 `MSR TCO, #1`锛夊鏌愪釜鐢ㄦ埛绾跨▼绂佺敤銆?
**娉ㄦ剰**锛氫俊鍙峰鐞嗙▼搴忓缁堜互 `PSTATE.TCO = 0` 琚皟鐢紝涓庤涓柇鐨勪笂涓嬫枃鏃犲叧銆?`PSTATE.TCO` 浼氬湪 `sigreturn()` 鏃舵仮澶嶃€?
**娉ㄦ剰**锛氱敤鎴峰簲鐢ㄧ▼搴忔病鏈夊彲鐢ㄧ殑**鍖归厤鍏ㄩ儴锛坢atch-all锛?*閫昏緫鏍囪銆?
**娉ㄦ剰**锛氬唴鏍稿鐢ㄦ埛鍦板潃绌洪棿锛堜緥濡?`read()` 绯荤粺璋冪敤锛夌殑璁块棶锛屽湪鐢ㄦ埛绾跨▼鐨勬爣璁版鏌?妯″紡涓?`PR_MTE_TCF_NONE` 鎴?`PR_MTE_TCF_ASYNC` 鏃朵笉琚鏌ャ€傚鏋滄爣璁版鏌ユā寮忎负
`PR_MTE_TCF_SYNC`锛屽唴鏍镐細灏芥渶澶у姫鍔涙鏌ュ叾瀵圭敤鎴峰湴鍧€鐨勮闂紝浣嗘棤娉曞缁堜繚璇併€備笉璁?鐢ㄦ埛閰嶇疆濡備綍锛屽唴鏍稿鐢ㄦ埛鍦板潃鐨勮闂缁堜互鏈夋晥鐨?`PSTATE.TCO` 鍊?0 鎵ц銆?
### 鍦?``IRG``銆乣`ADDG`` 鍜?``SUBG`` 鎸囦护涓帓闄ゆ爣璁?

浣撶郴缁撴瀯鍏佽閫氳繃 `GCR_EL1.Exclude` 瀵勫瓨鍣ㄤ綅鍩熸帓闄ゆ煇浜涜闅忔満鐢熸垚鐨勬爣璁般€傞粯璁ゆ儏鍐典笅锛?Linux 鎺掗櫎闄?0 浠ュ鐨勬墍鏈夋爣璁般€傜敤鎴风嚎绋嬪彲浠ヤ娇鐢?``prctl(PR_SET_TAGGED_ADDR_CTRL,
flags, 0, 0, 0)`` 绯荤粺璋冪敤鍦ㄩ殢鏈虹敓鎴愮殑闆嗗悎閲屽惎鐢ㄧ壒瀹氭爣璁帮紝鍏朵腑 `flags`` 鍦?`PR_MTE_TAG_MASK` 浣嶅煙涓寘鍚爣璁颁綅鍥俱€?
**娉ㄦ剰**锛氱‖浠朵娇鐢ㄧ殑鏄帓闄ゆ帺鐮侊紝鑰?`prctl()` 鎺ュ彛鎻愪緵鐨勬槸鍖呭惈鎺╃爜銆傚寘鍚帺鐮佷负 `0`
锛堟帓闄ゆ帺鐮?`0xffff`锛変細瀵艰嚧 CPU 濮嬬粓鐢熸垚鏍囪 `0`銆?
### 姣?CPU 鍋忓ソ鐨勬爣璁版鏌ユā寮?

鍦ㄦ煇浜?CPU 涓婏紝MTE 鍦ㄦ洿涓ユ牸鏍囪妫€鏌ユā寮忎笅鐨勬€ц兘涓庤緝瀹芥澗鏍囪妫€鏌ユā寮忎笅鐨勬€ц兘鐩歌繎銆?褰撹姹備簡杈冨鏉剧殑妫€鏌ユā寮忔椂锛屽湪杩欎簺 CPU 涓婂惎鐢ㄦ洿涓ユ牸鐨勬鏌ユ槸鍊煎緱鐨勶紝浠ヤ究鍦ㄤ笉甯︽潵
鎬ц兘涓嬮檷鐨勫墠鎻愪笅鑾峰緱鏇翠弗鏍兼鏌ョ殑閿欒妫€娴嬩紭鍔裤€備负鏀寔杩欑鍦烘櫙锛岀壒鏉冪敤鎴峰彲浠ュ皢鏇翠弗鏍?鐨勬爣璁版鏌ユā寮忛厤缃负璇?CPU 鍋忓ソ鐨勬爣璁版鏌ユā寮忋€?
姣忎釜 CPU 鍋忓ソ鐨勬爣璁版鏌ユā寮忕敱 `/sys/devices/system/cpu/cpu<N>/mte_tcf_preferred`
鎺у埗锛岀壒鏉冪敤鎴峰彲浠ュ悜鍏跺啓鍏ュ€?`async`銆乣sync` 鎴?`asymm`銆傛瘡涓?CPU 榛樿鐨勫亸濂芥ā寮忎负
`async`銆?
涓轰簡鍏佽绋嬪簭鍙兘鍦?CPU 鍋忓ソ鐨勬爣璁版鏌ユā寮忎笅杩愯锛岀敤鎴风▼搴忓彲浠ュ湪 ``prctl(PR_SET_TAGGED_ADDR_CTRL,
flags, 0, 0, 0)`` 绯荤粺璋冪敤鐨?`flags` 鍙傛暟涓缃涓爣璁版鏌ラ敊璇ā寮忎綅銆傚鏋滃悓鏃惰姹備簡
鍚屾鍜屽紓姝ユā寮忥紝閭ｄ箞鍐呮牳涔熷彲鑳介€夋嫨闈炲绉版ā寮忋€傚鏋?CPU 鍋忓ソ鐨勬爣璁版鏌ユā寮忓浜庝换鍔?鎵€鎻愪緵鐨勬爣璁版鏌ユā寮忛泦鍚堜腑锛屽垯閫夋嫨璇ユā寮忋€傚惁鍒欙紝鍐呮牳灏嗕粠浠诲姟鐨勬ā寮忛泦涓寜涓嬭堪鍋忓ソ
椤哄簭閫夋嫨涓€绉嶆ā寮忥細

 1. 寮傛锛圓synchronous锛? 2. 闈炲绉帮紙Asymmetric锛? 3. 鍚屾锛圫ynchronous锛?
娉ㄦ剰锛岀敤鎴风┖闂存棤娉曞湪璇锋眰澶氱妯″紡鐨勫悓鏃剁鐢ㄩ潪瀵圭О妯″紡銆?
### 鍒濆杩涚▼鐘舵€?

鍦?`execve()` 鏃讹紝鏂拌繘绋嬪叿鏈変互涓嬮厤缃細

- `PR_TAGGED_ADDR_ENABLE` 璁句负 0锛堢鐢級
- 鏈€夋嫨浠讳綍鏍囪妫€鏌ユā寮忥紙鏍囪妫€鏌ラ敊璇蹇界暐锛?- `PR_MTE_TAG_MASK` 璁句负 0锛堟墍鏈夋爣璁伴兘琚帓闄わ級
- `PSTATE.TCO` 璁句负 0
- 鍒濆鍐呭瓨鏄犲皠鍧囨湭璁剧疆 `PROT_MTE`

鍦?`fork()` 鏃讹紝鏂拌繘绋嬬户鎵跨埗杩涚▼鐨勯厤缃拰鍐呭瓨鏄犲皠灞炴€э紝浣嗕娇鐢?`MADV_WIPEONFORK` 鐨?`madvise()` 鑼冨洿闄ゅ鈥斺€旇繖浜涜寖鍥寸殑鏁版嵁鍜屾爣璁颁細琚竻闄わ紙璁句负 0锛夈€?
### ``ptrace()`` 鎺ュ彛


`PTRACE_PEEKMTETAGS` 鍜?`PTRACE_POKEMTETAGS` 鍏佽杩借釜鑰咃紙tracer锛変粠琚拷韪€咃紙tracee锛?鐨勫湴鍧€绌洪棿璇诲彇鏍囪鎴栧悜鍏惰缃爣璁般€俙ptrace()` 绯荤粺璋冪敤浠?``ptrace(request, pid, addr,
data)`` 褰㈠紡璋冪敤锛屽叾涓細

- `request` - `PTRACE_PEEKMTETAGS` 鎴?`PTRACE_POKEMTETAGS` 涔嬩竴銆?- `pid` - 琚拷韪€呯殑 PID銆?- `addr` - 琚拷韪€呭湴鍧€绌洪棿涓殑鍦板潃銆?- `data` - 鎸囧悜涓€涓?`struct iovec` 鐨勬寚閽堬紝鍏朵腑 `iov_base` 鎸囧悜杩借釜鑰呭湴鍧€绌洪棿涓?  闀垮害涓?`iov_len` 鐨勭紦鍐插尯銆?
杩借釜鑰呯殑 `iov_base` 缂撳啿鍖轰腑鐨勬爣璁拌〃绀轰负姣忓瓧鑺備竴涓?4 浣嶆爣璁帮紝瀵瑰簲浜庤杩借釜鑰呭湴鍧€绌洪棿
涓殑涓€涓?16 瀛楄妭 MTE 鏍囪绮掑害銆?
**娉ㄦ剰**锛氬鏋?`addr` 鏈榻愬埌 16 瀛楄妭绮掑害锛屽唴鏍稿皢浣跨敤鐩稿簲鐨勫榻愬湴鍧€銆?
`ptrace()` 杩斿洖鍊硷細

- 0 - 鏍囪宸茶澶嶅埗锛岃拷韪€呯殑 `iov_len` 琚洿鏂颁负浼犺緭鐨勬爣璁版暟閲忋€傚鏋滆杩借釜鑰呮垨杩借釜鑰?  鐨勫湴鍧€绌洪棿涓殑璇锋眰鍦板潃鑼冨洿鏃犳硶璁块棶鎴栦笉鍏锋湁鏈夋晥鏍囪锛岃鍊煎彲鑳藉皬浜庤姹傜殑 `iov_len`銆?- `-EPERM` - 鏃犳硶杩借釜鎸囧畾鐨勮繘绋嬨€?- `-EIO` - 鏃犳硶璁块棶琚拷韪€呯殑鍦板潃鑼冨洿锛堜緥濡傛棤鏁堝湴鍧€锛夛紝鏈鍒朵换浣曟爣璁般€俙iov_len`
  鏈洿鏂般€?- `-EFAULT` - 璁块棶杩借釜鑰呭唴瀛橈紙`struct iovec` 鎴?`iov_base` 缂撳啿鍖猴級鏃跺嚭閿欙紝鏈鍒?  浠讳綍鏍囪銆俙iov_len` 鏈洿鏂般€?- `-EOPNOTSUPP` - 琚拷韪€呯殑鍦板潃娌℃湁鏈夋晥鏍囪锛堜粠鏈互 `PROT_MTE` 鏍囧織鏄犲皠锛夈€俙iov_len`
  鏈洿鏂般€?
**娉ㄦ剰**锛氫笂杩拌姹傛病鏈夌灛鏃堕敊璇紝鍥犳鐢ㄦ埛绋嬪簭鍦ㄧ郴缁熻皟鐢ㄨ繑鍥為潪闆跺€兼椂涓嶅簲瀵瑰叾閲嶈瘯銆?
`PTRACE_GETREGSET` 鍜?`PTRACE_SETREGSET`锛岄厤鍚?``addr ==
`NT_ARM_TAGGED_ADDR_CTRL`锛屽厑璁?`ptrace()` 鎸夌収
Documentation/arch/arm64/tagged-address-abi.rst 鍙婁笂鏂囩殑 `prctl()` 閫夐」鎵€杩帮紝璁块棶
杩涚▼鐨勬爣璁板湴鍧€ ABI 鎺у埗鍜?MTE 閰嶇疆銆傜浉搴旂殑 `regset` 涓?1 涓?8 瀛楄妭鐨勫厓绱?锛坄sizeof(long))`锛夈€?
### Core dump 鏀寔


浠?`PROT_MTE` 鏄犲皠鐨勭敤鎴峰唴瀛樼殑鍒嗛厤鏍囪锛屼細浣滀负棰濆鐨?`PT_AARCH64_MEMTAG_MTE` 娈?杞偍鍒?core 鏂囦欢涓€傛绫绘鐨勭▼搴忓ご瀹氫箟濡備笅锛?
:`p_type`: `PT_AARCH64_MEMTAG_MTE`
:`p_flags`: 0
:`p_offset`: 娈靛湪鏂囦欢涓殑鍋忕Щ閲?:`p_vaddr`: 娈电殑铏氭嫙鍦板潃锛屼笌鐩稿簲鐨?`PT_LOAD` 娈电浉鍚?:`p_paddr`: 0
:`p_filesz`: 娈靛湪鏂囦欢涓殑澶у皬锛岃绠椾负 `p_mem_sz / 32`
  锛堜袱涓?4 浣嶆爣璁拌鐩?32 瀛楄妭鍐呭瓨锛?:`p_memsz`: 娈靛湪鍐呭瓨涓殑澶у皬锛屼笌鐩稿簲鐨?`PT_LOAD` 娈电浉鍚?:`p_align`: 0

鏍囪浠ヤ袱涓?4 浣嶆爣璁板瓨浜庝竴涓瓧鑺傜殑鏂瑰紡锛屽瓨鏀惧湪 core 鏂囦欢涓?`p_offset` 澶勩€傛爣璁扮矑搴︿负
16 瀛楄妭锛屼竴涓?4K 椤靛湪 core 鏂囦欢涓渶瑕?128 瀛楄妭銆?
## 姝ｇ‘鐢ㄦ硶绀轰緥


**MTE 绀轰緥浠ｇ爜**


    /*
     - 闇€浠?-march=armv8.5-a+memtag 缂栬瘧
     */
    #include <errno.h>
    #include <stdint.h>
    #include <stdio.h>
    #include <stdlib.h>
    #include <unistd.h>
    #include <sys/auxv.h>
    #include <sys/mman.h>
    #include <sys/prctl.h>

    /*
     - From arch/arm64/include/uapi/asm/hwcap.h
     */
    #define HWCAP2_MTE              (1 << 18)

    /*
     - From arch/arm64/include/uapi/asm/mman.h
     */
    #define PROT_MTE                 0x20

    /*
     - From include/uapi/linux/prctl.h
     */
    #define PR_SET_TAGGED_ADDR_CTRL 55
    #define PR_GET_TAGGED_ADDR_CTRL 56
    # define PR_TAGGED_ADDR_ENABLE  (1UL << 0)
    # define PR_MTE_TCF_SHIFT       1
    # define PR_MTE_TCF_NONE        (0UL << PR_MTE_TCF_SHIFT)
    # define PR_MTE_TCF_SYNC        (1UL << PR_MTE_TCF_SHIFT)
    # define PR_MTE_TCF_ASYNC       (2UL << PR_MTE_TCF_SHIFT)
    # define PR_MTE_TCF_MASK        (3UL << PR_MTE_TCF_SHIFT)
    # define PR_MTE_TAG_SHIFT       3
    # define PR_MTE_TAG_MASK        (0xffffUL << PR_MTE_TAG_SHIFT)

    /*
     - 鍚戠粰瀹氭寚閽堟彃鍏ヤ竴涓殢鏈虹殑閫昏緫鏍囪銆?     */
    #define insert_random_tag(ptr) ({                       \
            uint64_t __val;                                 \
            asm("irg %0, %1" : "=r" (__val) : "r" (ptr));   \
            __val;                                          \
    })

    /*
     - 鍦ㄧ洰鏍囧湴鍧€涓婅缃垎閰嶆爣璁般€?     */
    #define set_tag(tagged_addr) do {                                      \
            asm volatile("stg %0, [%0]" : : "r" (tagged_addr) : "memory"); \
    } while (0)

    int main()
    {
            unsigned char *a;
            unsigned long page_sz = sysconf(_SC_PAGESIZE);
            unsigned long hwcap2 = getauxval(AT_HWCAP2);

            /** 妫€鏌ユ槸鍚﹀瓨鍦?MTE **/
            if (!(hwcap2 & HWCAP2_MTE))
                    return EXIT_FAILURE;

            /*
             - 鍚敤鏍囪鍦板潃 ABI銆佸悓姝ユ垨寮傛锛堝熀浜庢瘡 CPU 鍋忓ソ锛夌殑 MTE
             - 鏍囪妫€鏌ラ敊璇紝骞跺厑璁搁殢鏈虹敓鎴愰泦鍚堜腑闄?0 澶栫殑鎵€鏈?             - 鏍囪銆?             */
            if (prctl(PR_SET_TAGGED_ADDR_CTRL,
                      PR_TAGGED_ADDR_ENABLE | PR_MTE_TCF_SYNC | PR_MTE_TCF_ASYNC |
                      (0xfffe << PR_MTE_TAG_SHIFT),
                      0, 0, 0)) {
                    perror("prctl() failed");
                    return EXIT_FAILURE;
            }

            a = mmap(0, page_sz, PROT_READ | PROT_WRITE,
                     MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
            if (a == MAP_FAILED) {
                    perror("mmap() failed");
                    return EXIT_FAILURE;
            }

            /*
             - 鍦ㄤ笂闈㈢殑鍖垮悕 mmap 涓婂惎鐢?MTE銆傝鏍囧織涔熷彲浠ョ洿鎺ヤ紶缁?             - mmap() 浠庤€岃烦杩囪繖涓€姝ャ€?             */
            if (mprotect(a, page_sz, PROT_READ | PROT_WRITE | PROT_MTE)) {
                    perror("mprotect() failed");
                    return EXIT_FAILURE;
            }

            /** 浠ラ粯璁ゆ爣璁?(0) 璁块棶 **/
            a[^0^] = 1;
            a[^1^] = 2;

            printf("a[^0^] = %hhu a[^1^] = %hhu\n", a[^0^], a[^1^]);

            /** 璁剧疆閫昏緫涓庡垎閰嶆爣璁?**/
            a = (unsigned char *)insert_random_tag(a);
            set_tag(a);

            printf("%p\n", a);

            /** 浠ラ潪闆舵爣璁拌闂?**/
            a[^0^] = 3;
            printf("a[^0^] = %hhu a[^1^] = %hhu\n", a[^0^], a[^1^]);

            /*
             - 濡傛灉 MTE 琚纭惎鐢紝涓嬩竴鏉℃寚浠ゅ皢浜х敓涓€涓?             - 寮傚父銆?             */
            printf("Expecting SIGSEGV...\n");
            a[^16^] = 0xdd;

            /** 鍦?PR_MTE_TCF_SYNC 妯″紡涓嬩笉搴旀墦鍗拌繖琛?**/
            printf("...haven't got one\n");

            return EXIT_FAILURE;
    }
