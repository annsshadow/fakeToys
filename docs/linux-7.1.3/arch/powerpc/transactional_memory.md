## 浜嬪姟鍐呭瓨鏀寔

POWER 鍐呮牳鐩墠瀵硅鐗规€х殑鏀寔浠呴檺浜庢敮鎸佺敤鎴风▼搴忎娇鐢ㄥ畠銆傚唴鏍告湰韬洰鍓嶅苟鏈娇鐢ㄥ畠銆?
鏈枃妗ｆ棬鍦ㄦ€荤粨 Linux 濡備綍鏀寔璇ョ壒鎬э紝浠ュ強浣犲彲浠ヤ粠鑷繁鐨勭敤鎴风▼搴忎腑鏈熷緟鎬庢牱鐨勮涓恒€?
## 鍩烘湰姒傝堪

纭欢浜嬪姟鍐呭瓨锛圚ardware Transactional Memory锛夊湪 POWER8 澶勭悊鍣ㄤ笂鍙楁敮鎸侊紝鏄竴绉嶆敮鎸佷笉鍚屽舰寮?鍘熷瓙鍐呭瓨璁块棶鐨勭壒鎬с€傛彁渚涗簡鑻ュ共鏂版寚浠ゆ潵鐣屽畾浜嬪姟锛涗簨鍔′繚璇佽涔堜互鍘熷瓙鏂瑰紡瀹屾垚锛岃涔堝洖婊氬苟鎾ら攢
浠讳綍閮ㄥ垎鏇存敼銆?
```
  begin_move_money:
    tbegin
    beq   abort_handler

    ld    r4, SAVINGS_ACCT(r3)
    ld    r5, CURRENT_ACCT(r3)
    subi  r5, r5, 1
    addi  r4, r4, 1
    std   r4, SAVINGS_ACCT(r3)
    std   r5, CURRENT_ACCT(r3)

    tend

    b     continue

  abort_handler:
    ... test for odd failures ...

    /* Retry the transaction if it failed because it conflicted with
     * someone else: */
    b     begin_move_money

```
'tbegin' 鎸囦护琛ㄧず璧风偣锛?tend' 琛ㄧず缁堢偣銆傚湪杩欎袱涓偣涔嬮棿锛屽鐞嗗櫒澶勪簬鈥滀簨鍔♀€濓紙Transactional锛夌姸鎬侊紱
濡傛灉娌℃湁涓庣郴缁熶腑鍏朵粬浜嬪姟鎬ф垨闈炰簨鍔℃€ц闂殑鍐茬獊锛屼换浣曞唴瀛樺紩鐢ㄩ兘浼氫竴娆℃€у畬鎴愩€傚湪姝や緥涓紝濡傛灉
娌℃湁鍏朵粬澶勭悊鍣ㄨЕ纰拌繃 SAVINGS_ACCT(r3) 鎴?CURRENT_ACCT(r3)锛屼簨鍔″氨浼氬儚鏅€氱殑椤哄簭浠ｇ爜涓€鏍峰畬鎴愶紱
杩欐牱灏辨墽琛屼簡涓€娆′粠褰撳墠璐︽埛鍒板偍钃勮处鎴风殑鍘熷瓙杞处銆傚嵆浣夸娇鐢ㄧ殑鏄櫘閫氱殑 ld/std 鎸囦护锛堟敞鎰忔病鏈?lwarx/stwcx锛夛紝瑕佷箞 SAVINGS_ACCT(r3) 鍜?CURRENT_ACCT(r3) **閮?*琚洿鏂帮紝瑕佷箞閮?*涓?*琚洿鏂般€?
濡傛灉鍦ㄦ鏈熼棿鍙戠敓浜嗕笌浜嬪姟璁块棶浣嶇疆鐨勫啿绐侊紝浜嬪姟灏嗚 CPU 涓銆傚瘎瀛樺櫒鍜屽唴瀛樼姸鎬佷細鍥炴粴鍒?'tbegin'
鏃剁殑鐘舵€侊紝鎺у埗娴佸皢浠?'tbegin+4' 缁х画銆傜浜屾浼氳烦杞埌 abort_handler锛沘bort handler 鍙互妫€鏌ュけ璐?鍘熷洜骞堕噸璇曘€?
琚鏌ョ偣鍖栫殑瀵勫瓨鍣ㄥ寘鎷墍鏈?GPR銆丗PR銆乂R/VSR銆丩R銆丆CR/CR銆丆TR銆丗PCSR 浠ュ強涓€浜涘叾浠栫姸鎬?鏍囧織瀵勫瓨鍣紱
璇﹁ ISA銆?
## 浜嬪姟涓鐨勫師鍥?
- 涓庡叾浠栧鐞嗗櫒浣跨敤鐨勭紦瀛樿鍐茬獊
- 淇″彿
- 涓婁笅鏂囧垏鎹?- 鍏充簬浼氫腑姝簨鍔＄殑鎵€鏈夊唴瀹圭殑瀹屾暣鏂囨。锛岃鍙傝 ISA銆?
## 绯荤粺璋冪敤

鍦ㄦ椿璺冧簨鍔″唴閮ㄥ彂璧风殑绯荤粺璋冪敤涓嶄細琚墽琛岋紝浜嬪姟灏嗚鍐呮牳浠ュけ璐ョ爜 TM_CAUSE_SYSCALL |
TM_CAUSE_PERSISTENT 鍒ゅ畾涓哄け璐ワ紙doomed锛夈€?
鍦ㄦ寕璧凤紙suspended锛夌殑浜嬪姟鍐呴儴鍙戣捣鐨勭郴缁熻皟鐢ㄤ細鍍忔甯镐竴鏍疯鎵ц锛屽唴鏍镐笉浼氭樉寮忓皢鍏跺垽瀹氫负澶辫触銆?鐒惰€岋紝鍐呮牳涓烘墽琛岃绯荤粺璋冪敤鎵€鍋氱殑浜嬫儏鍙兘瀵艰嚧浜嬪姟琚‖浠跺垽瀹氫负澶辫触銆傜郴缁熻皟鐢ㄥ湪鎸傝捣妯″紡涓嬫墽琛岋紝
鍥犳浠讳綍鍓綔鐢ㄩ兘鏄寔涔呯殑锛屼笌浜嬪姟鐨勬垚鍔熸垨澶辫触鏃犲叧銆傚唴鏍镐笉淇濊瘉鍝簺绯荤粺璋冪敤浼氬奖鍝嶄簨鍔＄殑鎴愬姛銆?
濡傛灉绯荤粺璋冪敤鏄€氳繃搴撳彂璧风殑锛屽湪渚濊禆绯荤粺璋冪敤鍦ㄦ椿璺冧簨鍔℃湡闂翠腑姝㈡椂蹇呴』灏忓績銆傚簱鍙兘浼氱紦瀛樺€硷紙杩?鍙兘璁╀汉璇互涓烘垚鍔燂級锛屾垨鑰呭湪杩涘叆鍐呮牳涔嬪墠鎵ц瀵艰嚧浜嬪姟澶辫触鐨勬搷浣滐紙杩欏彲鑳戒骇鐢熶笉鍚岀殑澶辫触鐮侊級銆?渚嬪 glibc 鐨?getpid() 鍜屾儼鎬х鍙疯В鏋愩€?
## 淇″彿

鍦ㄤ簨鍔℃湡闂撮€掗€佷俊鍙凤紙鍚屾鍜屽紓姝ワ級浼氭彁渚涚浜屼釜绾跨▼鐘舵€侊紙ucontext/mcontext锛夋潵琛ㄧず绗簩涓簨鍔℃€?瀵勫瓨鍣ㄧ姸鎬併€備俊鍙烽€掗€侀€氳繃 'treclaim' 鏉ユ崟鑾蜂袱绉嶅瘎瀛樺櫒鐘舵€侊紝鍥犳淇″彿浼氫腑姝簨鍔°€備紶缁欎俊鍙峰鐞嗙▼搴?鐨勫父瑙?ucontext_t 琛ㄧず琚鏌ョ偣鍖?鍘熷鐨勫瘎瀛樺櫒鐘舵€侊紱璇ヤ俊鍙风湅璧锋潵鍍忔槸鍦?'tbegin+4' 澶勫彂鐢熺殑銆?
濡傛灉 sighandler 鐨?ucontext 璁剧疆浜?uc_link锛屽垯宸茬粡閫掗€佷簡绗簩涓?ucontext銆備负浜嗘湭鏉ョ殑鍏煎鎬э紝搴?妫€鏌?MSR.TS 瀛楁浠ョ‘瀹氫簨鍔＄姸鎬佲€斺€斿鏋滄槸锛屽垯 uc->uc_link 涓殑绗簩涓?ucontext 琛ㄧず淇″彿鍙戠敓鏃?娲昏穬鐨勪簨鍔″瘎瀛樺櫒銆?
瀵逛簬 64 浣嶈繘绋嬶紝uc->uc_mcontext.regs->msr 鏄竴涓畬鏁寸殑 64 浣?MSR锛屽叾 TS 瀛楁鏄剧ず浜嗕簨鍔℃ā寮忋€?
瀵逛簬 32 浣嶈繘绋嬶紝mcontext 鐨?MSR 瀵勫瓨鍣ㄥ彧鏈?32 浣嶏紱楂?32 浣嶅瓨鍌ㄥ湪绗簩涓?ucontext 鐨?MSR 涓紝鍗?uc->uc_link->uc_mcontext.regs->msr銆傞珮瀛楀寘鍚簨鍔＄姸鎬?TS銆?
鐒惰€岋紝鍩烘湰鐨勪俊鍙峰鐞嗙▼搴忎笉闇€瑕佹劅鐭ヤ簨鍔★紝绠€鍗曞湴浠庡鐞嗙▼搴忚繑鍥炲氨鑳芥纭鐞嗭細

鎰熺煡浜嬪姟鐨勪俊鍙峰鐞嗙▼搴忓彲浠ヤ粠绗簩涓?ucontext 璇诲彇浜嬪姟鎬у瘎瀛樺櫒鐘舵€併€傝繖瀵逛簬宕╂簝澶勭悊绋嬪簭纭畾渚嬪
瀵艰嚧 SIGSEGV 鐨勬寚浠ゅ湴鍧€鏄繀瑕佺殑銆?
```
    void crash_handler(int sig, siginfo_t *si, void *uc)
    {
      ucontext_t *ucp = uc;
      ucontext_t *transactional_ucp = ucp->uc_link;

      if (ucp.link) {
        u64 msr = ucp->uc_mcontext.regs->msr;
        /* May have transactional ucontext! */
  #ifndef __powerpc64__
        msr |= ((u64)transactional_ucp->uc_mcontext.regs->msr) << 32;
  #endif
        if (MSR_TM_ACTIVE(msr)) {
           /* Yes, we crashed during a transaction.  Oops. */
   fprintf(stderr, "Transaction to be restarted at 0x%llx, but "
                           "crashy instruction was at 0x%llx\n",
                           ucp->uc_mcontext.regs->nip,
                           transactional_ucp->uc_mcontext.regs->nip);
        }
      }

      fix_the_problem(ucp->dar);
    }

```
褰撳浜庢椿璺冧簨鍔′腑骞舵敹鍒颁俊鍙锋椂锛屾垜浠渶瑕佸皬蹇冨鐞嗘爤銆傛湁鍙兘鍦?tbegin 涔嬪悗鏍堝凡缁忓悜涓婂洖閫€浜嗐€傝繖閲?鏄庢樉鐨勬儏鍐垫槸 tbegin 鍦ㄤ竴涓嚱鏁板唴閮ㄨ璋冪敤锛屽苟鍦?tend 涔嬪墠杩斿洖銆傚湪杩欑鎯呭喌涓嬶紝鏍堟槸琚鏌ョ偣鍖?浜嬪姟鍐呭瓨鐘舵€佺殑涓€閮ㄥ垎銆傚鏋滄垜浠互闈炰簨鍔℃柟寮忔垨鍦ㄦ寕璧风姸鎬佷笅鍐欒鐩栧畠锛屽氨浼氭湁楹荤儲锛屽洜涓哄鏋滄垜浠?閬囧埌 tm abort锛岀▼搴忚鏁板櫒鍜屾爤鎸囬拡浼氬洖鍒?tbegin 澶勶紝浣嗘垜浠湪鍐呭瓨涓殑鏍堝皢涓嶅啀鏈夋晥銆?
涓轰簡閬垮厤杩欎竴鐐癸紝褰撳湪娲昏穬浜嬪姟涓帴鏀朵俊鍙锋椂锛屾垜浠渶瑕佷娇鐢ㄦ潵鑷鏌ョ偣鍖栫姸鎬佺殑鏍堟寚閽堬紝鑰屼笉鏄帹娴?鐘舵€併€傝繖纭繚淇″彿涓婁笅鏂囷紙浠?tm 鎸傝捣鏂瑰紡鍐欏叆锛変細琚啓鍦ㄥ洖婊氭墍闇€鏍堢殑涓嬫柟銆傜敱浜?treclaim 浼氫腑姝?浜嬪姟锛屽洜姝ゅ湪 tbegin 鍜屼俊鍙蜂箣闂村啓鍏ョ殑浠讳綍鍐呭瓨鏃犺濡備綍閮戒細琚洖婊氥€?
瀵逛簬鍦ㄩ潪 TM 鎴栨寕璧锋ā寮忎笅鎺ユ敹鐨勪俊鍙凤紝鎴戜滑浣跨敤姝ｅ父/闈炴鏌ョ偣鍖栫殑鏍堟寚閽堛€?
鍦?sighandler 鍐呴儴鍙戣捣骞跺湪浠?sighandler 杩斿洖鍒板唴鏍告椂鎸傝捣鐨勪簨鍔★紝灏嗚鍥炴敹骞朵涪寮冦€?
## 鍐呮牳浣跨敤鐨勫け璐ュ師鍥犵爜

杩欎簺鍦?<asm/reg.h> 涓畾涔夛紝鐢ㄤ簬鍖哄垎鍐呮牳涓浜嬪姟鐨勪笉鍚屽師鍥狅細

 ====================== ================================
 TM_CAUSE_RESCHED       绾跨▼琚噸鏂拌皟搴︺€? TM_CAUSE_TLBI          杞欢 TLB 澶辨晥銆? TM_CAUSE_FAC_UNAV      FP/VEC/VSX 涓嶅彲鐢ㄩ櫡闃便€? TM_CAUSE_SYSCALL       鏉ヨ嚜娲昏穬浜嬪姟鐨勭郴缁熻皟鐢ㄣ€? TM_CAUSE_SIGNAL        宸查€掗€佷俊鍙枫€? TM_CAUSE_MISC          褰撳墠鏈娇鐢ㄣ€? TM_CAUSE_ALIGNMENT     瀵归綈閿欒銆? TM_CAUSE_EMULATE       瑙﹀強鍐呭瓨鐨勬ā鎷熴€? ====================== ================================

杩欎簺鍙互鐢辩敤鎴风▼搴忕殑 abort handler 浣滀负 TEXASR[0:7] 鏉ユ鏌ャ€傚鏋滅 7 浣嶇疆浣嶏紝琛ㄧず閿欒琚涓?鎸佷箙鐨勩€備緥濡?TM_CAUSE_ALIGNMENT 鏄寔涔呯殑锛岃€?TM_CAUSE_RESCHED 涓嶆槸銆?
## GDB

GDB 鍜?ptrace 鐩墠涓嶈兘鎰熺煡 TM銆傚鏋滃湪涓€涓簨鍔℃湡闂村仠涓嬫潵锛岀湅璧锋潵灏卞儚浜嬪姟鍒氬垰寮€濮嬶紙鍛堢幇鐨勬槸
琚鏌ョ偣鍖栫殑鐘舵€侊級銆傜劧鍚庝簨鍔℃棤娉曠户缁紝骞朵細璧板け璐ュ鐞嗙▼搴忚矾绾裤€傛澶栵紝浜嬪姟鎬х殑绗簩涓瘎瀛樺櫒鐘舵€?灏嗕笉鍙闂€侴DB 鐩墠鍙互鐢ㄤ簬浣跨敤 TM 鐨勭▼搴忥紝浣嗗湪浜嬪姟鍐呴儴鐨勯儴鍒嗗垯涓嶈銆?
## POWER9

POWER9 涓婄殑 TM 鍦ㄥ瓨鍌ㄥ畬鏁村瘎瀛樺櫒鐘舵€佹柟闈㈠瓨鍦ㄩ棶棰樸€傛

```
    commit 4bb3c7a0208fc13ca70598efd109901a7cd45ae7
    Author: Paul Mackerras <paulus@ozlabs.org>
    Date:   Wed Mar 21 21:32:01 2018 +1100
    KVM: PPC: Book3S HV: Work around transactional memory bugs in POWER9

```
涓轰簡搴斿杩欎竴鐐癸紝涓嶅悓鐨?POWER9 鑺墖浠ヤ笉鍚屾柟寮忓惎鐢?TM銆?
鍦?POWER9N DD2.01 鍙婃洿浣庣増鏈笂锛孴M 琚鐢ㄣ€傚嵆 HWCAP2[PPC_FEATURE2_HTM] 鏈缃€?
鍦?POWER9N DD2.1 涓婏紝TM 鐢卞浐浠堕厤缃负鍦?tm 鎸傝捣鍙戠敓鏃舵€绘槸涓浜嬪姟銆傚洜姝?tsuspend 浼氬鑷翠簨鍔?琚腑姝㈠苟鍥炴粴銆傚唴鏍稿紓甯镐篃浼氬鑷翠簨鍔¤涓骞跺洖婊氾紝骞朵笖寮傚父涓嶄細鍙戠敓銆傚鏋滅敤鎴风┖闂存瀯閫犱簡涓€涓惎鐢?TM 鎸傝捣鐨?sigcontext锛岃 sigcontext 灏嗚鍐呮牳鎷掔粷銆傛妯″紡閫氳繃鍚戠敤鎴疯缃?HWCAP2[PPC_FEATURE2_HTM_NO_SUSPEND] 鏉ラ€氬憡銆傚湪姝ゆā寮忎笅 HWCAP2[PPC_FEATURE2_HTM] 鏈缃€?
鍦?POWER9N DD2.2 鍙婃洿楂樼増鏈笂锛孠VM 鍜?POWERVM 涓哄鎴锋満妯℃嫙 TM锛堝鎻愪氦 4bb3c7a0208f 鎵€杩帮級锛屽洜姝?TM 涓哄鎴锋満鍚敤锛屽嵆 HWCAP2[PPC_FEATURE2_HTM] 涓哄鎴锋満鐢ㄦ埛绌洪棿璁剧疆銆傚ぇ閲忎娇鐢?TM 鎸傝捣锛坱suspend
鎴栧唴鏍告寕璧凤級鐨勫鎴锋満浼氬鑷撮櫡鍏ョ鐞嗙▼搴忥紙hypervisor锛夛紝鍥犳浼氶伃鍙楁€ц兘涓嬮檷銆備富鏈虹敤鎴风┖闂寸殑 TM
琚鐢紝鍗?HWCAP2[PPC_FEATURE2_HTM] 鏈缃€傦紙灏界濡傛灉灏嗘潵鎴戜滑灏嗘ā鎷熷甫鍏ヤ富鏈虹敤鎴风┖闂翠笂涓嬫枃
鍒囨崲锛屾垜浠彲鑳戒細鍦ㄦ煇涓椂鍊欏惎鐢ㄥ畠锛夈€?
POWER9C DD1.2 鍙婃洿楂樼増鏈粎閫氳繃 POWERVM 鎻愪緵锛屽洜姝?Linux 鍙綔涓哄鎴锋満杩愯銆傚湪杩欎簺绯荤粺涓?TM 鍍?POWER9N DD2.2 涓€鏍疯妯℃嫙銆?
浠?POWER8 鍒?POWER9 鐨勫鎴锋満杩佺Щ鍦?POWER9N DD2.2 鍜?POWER9C DD1.2 涓婂彲浠ュ伐浣溿€傜敱浜庤緝鏃╃殑
POWER9 澶勭悊鍣ㄤ笉鏀寔 TM 妯℃嫙锛岄偅閲屼笉鏀寔浠?POWER8 鍒?POWER9 鐨勮縼绉汇€?
## 鍐呮牳瀹炵幇

### h/rfid mtmsrd 鎬櫀

濡?ISA 涓墍瀹氫箟锛宺fid 鏈変竴涓湪鏃╂湡寮傚父澶勭悊涓湁鐢ㄧ殑鎬櫀銆傚綋澶勪簬鐢ㄦ埛绌洪棿浜嬪姟涓苟閫氳繃鏌愪釜寮傚父
杩涘叆鍐呮牳鏃讹紝MSR 鏈€缁堜細鏄?TM=0 涓?TS=01锛堝嵆 TM 鍏抽棴浣?TM 鎸傝捣锛夈€傞€氬父鍐呮牳浼氬笇鏈涙敼鍙?MSR 涓殑
浣嶏紝骞朵細鎵ц涓€涓?rfid 鏉ュ仛鍒拌繖涓€鐐广€傚湪杩欑鎯呭喌涓嬶紝rfid 鍙兘浣?SRR0 涓?TM=0 涓?TS=00锛堝嵆 TM
鍏抽棴涓旈潪浜嬪姟锛夛紝鑰岀粨鏋?MSR 灏嗕繚鐣欎箣鍓嶇殑 TM=0 鍜?TS=01锛堝嵆淇濇寔鎸傝捣锛夈€傝繖鏄灦鏋勪腑鐨勪竴涓€櫀锛屽洜涓?杩欓€氬父鏄粠 TS=01 鍒?TS=00锛堝嵆鎸傝捣 -> 闈炰簨鍔★級鐨勮浆绉伙紝鑰岃繖鏄竴娆￠潪娉曡浆绉汇€?
璇ユ€櫀鍦ㄦ灦鏋勪腑 rfid 鐨勫畾涔夐噷鐢ㄤ互涓嬭鎻忚堪锛?
  if (MSR 29:31 卢 = 0b010 | SRR1 29:31 卢 = 0b000) then
     MSR 29:31 <- SRR1 29:31

hrfid 鍜?mtmsrd 鏈夌浉鍚岀殑鎬櫀銆?
Linux 鍐呮牳鍦ㄥ叾鏃╂湡寮傚父澶勭悊涓娇鐢ㄤ簡杩欎釜鎬櫀銆?