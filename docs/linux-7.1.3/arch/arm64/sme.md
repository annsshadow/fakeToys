## Scalable Matrix Extension support for AArch64 Linux

AArch64 Linux 瀵瑰彲浼哥缉鐭╅樀鎵╁睍锛圫ME锛夌殑鏀寔


This document outlines briefly the interface provided to userspace by Linux in
order to support use of the ARM Scalable Matrix Extension (SME).

鏈枃妗ｇ畝瑕佹杩颁簡 Linux 涓烘敮鎸佷娇鐢?ARM 鍙几缂╃煩闃垫墿灞曪紙SME锛夎€屾彁渚涚粰鐢ㄦ埛绌洪棿鐨勬帴鍙ｃ€?
This is an outline of the most important features and issues only and not
intended to be exhaustive.  It should be read in conjunction with the SVE
documentation in sve.rst which provides details on the Streaming SVE mode
included in SME.

鏈枃妗ｄ粎姒傝堪鏈€閲嶈鐨勭壒鎬у拰闂锛屽苟闈為潰闈勘鍒般€傚畠搴斾笌 sve.rst 涓殑 SVE 鏂囨。涓€骞堕槄璇伙紝
鍚庤€呮彁渚涗簡鍏充簬 SME 涓墍鍖呭惈娴佸紡 SVE锛圫treaming SVE锛夋ā寮忕殑缁嗚妭銆?
This document does not aim to describe the SME architecture or programmer's
model.  To aid understanding, a minimal description of relevant programmer's
model features for SME is included in Appendix A.

鏈枃妗ｆ棤鎰忔弿杩?SME 鏋舵瀯鎴栫紪绋嬫ā鍨嬨€備负渚夸簬鐞嗚В锛岄檮褰?A 涓寘鍚簡 SME 鐩稿叧缂栫▼妯″瀷鐗规€?鐨勭畝瑕佽鏄庛€?

### 1.  General

### 1. 姒傝堪


- PSTATE.SM, PSTATE.ZA, the streaming mode vector length, the ZA and (when
  present) ZTn register state and TPIDR2_EL0 are tracked per thread.

- PSTATE.SM銆丳STATE.ZA銆佹祦寮忔ā寮忓悜閲忛暱搴︺€乑A 浠ュ強锛堝湪瀛樺湪鏃讹級ZTn 瀵勫瓨鍣ㄧ姸鎬佸拰
  TPIDR2_EL0 鍧囨寜绾跨▼杩涜璺熻釜銆?
- The presence of SME is reported to userspace via HWCAP2_SME in the aux vector
  AT_HWCAP2 entry.  Presence of this flag implies the presence of the SME
  instructions and registers, and the Linux-specific system interfaces
  described in this document.  SME is reported in /proc/cpuinfo as "sme".

- SME 鐨勫瓨鍦ㄩ€氳繃杈呭姪鍚戦噺 AT_HWCAP2 鏉＄洰涓殑 HWCAP2_SME 鎶ュ憡缁欑敤鎴风┖闂淬€傝鏍囧織鐨?  瀛樺湪鎰忓懗鐫€ SME 鎸囦护鍜屽瘎瀛樺櫒浠ュ強鏈枃妗ｆ墍鎻忚堪鐨?Linux 鐗瑰畾绯荤粺鎺ュ彛鐨勫瓨鍦ㄣ€係ME 鍦?  /proc/cpuinfo 涓互 "sme" 褰㈠紡鎶ュ憡銆?
- The presence of SME2 is reported to userspace via HWCAP2_SME2 in the
  aux vector AT_HWCAP2 entry.  Presence of this flag implies the presence of
  the SME2 instructions and ZT0, and the Linux-specific system interfaces
  described in this document.  SME2 is reported in /proc/cpuinfo as "sme2".

- SME2 鐨勫瓨鍦ㄩ€氳繃杈呭姪鍚戦噺 AT_HWCAP2 鏉＄洰涓殑 HWCAP2_SME2 鎶ュ憡缁欑敤鎴风┖闂淬€傝鏍囧織鐨?  瀛樺湪鎰忓懗鐫€ SME2 鎸囦护鍜?ZT0 浠ュ強鏈枃妗ｆ墍鎻忚堪鐨?Linux 鐗瑰畾绯荤粺鎺ュ彛鐨勫瓨鍦ㄣ€係ME2 鍦?  /proc/cpuinfo 涓互 "sme2" 褰㈠紡鎶ュ憡銆?
- Support for the execution of SME instructions in userspace can also be
  detected by reading the CPU ID register ID_AA64PFR1_EL1 using an MRS
  instruction, and checking that the value of the SME field is nonzero. [^3^]

- 鐢ㄦ埛绌洪棿涔熷彲閫氳繃 MRS 鎸囦护璇诲彇 CPU ID 瀵勫瓨鍣?ID_AA64PFR1_EL1锛屽苟妫€鏌?SME 瀛楁鐨?  鍊兼槸鍚﹂潪闆讹紝鏉ユ娴嬪鐢ㄦ埛绌洪棿涓墽琛?SME 鎸囦护鐨勬敮鎸併€俒^3^]

  It does not guarantee the presence of the system interfaces described in the
  following sections: software that needs to verify that those interfaces are
  present must check for HWCAP2_SME instead.

  杩欏苟涓嶄繚璇佷互涓嬪皬鑺傛墍鎻忚堪鐨勭郴缁熸帴鍙ｇ殑瀛樺湪锛氶渶瑕佺‘璁よ繖浜涙帴鍙ｅ瓨鍦ㄧ殑杞欢蹇呴』杞€?  妫€鏌?HWCAP2_SME銆?
- There are a number of optional SME features, presence of these is reported
  through AT_HWCAP2 through:

- 瀛樺湪鑻ュ共鍙€夌殑 SME 鐗规€э紝瀹冧滑鐨勫瓨鍦ㄩ€氳繃 AT_HWCAP2 鎶ュ憡濡備笅锛?
	HWCAP2_SME_I16I64
	HWCAP2_SME_F64F64
	HWCAP2_SME_I8I32
	HWCAP2_SME_F16F32
	HWCAP2_SME_B16F32
	HWCAP2_SME_F32F32
	HWCAP2_SME_FA64
        HWCAP2_SME2

  This list may be extended over time as the SME architecture evolves.

  姝ゅ垪琛ㄥ彲鑳介殢鐫€ SME 鏋舵瀯鐨勬紨杩涜€屾墿灞曘€?
  These extensions are also reported via the CPU ID register ID_AA64SMFR0_EL1,
  which userspace can read using an MRS instruction.  See elf_hwcaps.txt and
  cpu-feature-registers.txt for details.

  杩欎簺鎵╁睍涔熼€氳繃 CPU ID 瀵勫瓨鍣?ID_AA64SMFR0_EL1 鎶ュ憡锛岀敤鎴风┖闂村彲浣跨敤 MRS 鎸囦护璇诲彇
  璇ュ瘎瀛樺櫒銆傝瑙?elf_hwcaps.txt 鍜?cpu-feature-registers.txt銆?
- Debuggers should restrict themselves to interacting with the target via the
  NT_ARM_SVE, NT_ARM_SSVE, NT_ARM_ZA and NT_ARM_ZT regsets.  The recommended
  way of detecting support for these regsets is to connect to a target process
  first and then attempt a

- 璋冭瘯鍣ㄥ簲浠呴檺浜庨€氳繃 NT_ARM_SVE銆丯T_ARM_SSVE銆丯T_ARM_ZA 鍜?NT_ARM_ZT regset 涓庣洰鏍?  浜や簰銆傛娴嬪杩欎簺 regset 鏀寔鎯呭喌鐨勬帹鑽愭柟寮忔槸鍏堣繛鎺ュ埌涓€涓洰鏍囪繘绋嬶紝鐒跺悗灏濊瘯涓€涓?
	ptrace(PTRACE_GETREGSET, pid, NT_ARM_<regset>, &iov).

- Whenever ZA register values are exchanged in memory between userspace and
  the kernel, the register value is encoded in memory as a series of horizontal
  vectors from 0 to VL/8-1 stored in the same endianness invariant format as is
  used for SVE vectors.

- 姣忓綋 ZA 瀵勫瓨鍣ㄥ€煎湪鐢ㄦ埛绌洪棿涓庡唴鏍镐箣闂撮€氳繃鍐呭瓨浜ゆ崲鏃讹紝瀵勫瓨鍣ㄥ€间互涓€绯诲垪姘村钩鍚戦噺鐨?  褰㈠紡缂栫爜鍦ㄥ唴瀛樹腑锛屼粠 0 鍒?VL/8-1锛岄噰鐢ㄤ笌 SVE 鍚戦噺鐩稿悓鐨勫瓧鑺傚簭鏃犲叧鏍煎紡瀛樺偍銆?
- On thread creation PSTATE.ZA and TPIDR2_EL0 are preserved unless CLONE_VM
  is specified, in which case PSTATE.ZA is set to 0 and TPIDR2_EL0 is set to 0.

- 鍦ㄧ嚎绋嬪垱寤烘椂锛孭STATE.ZA 鍜?TPIDR2_EL0 浼氳淇濈暀锛岄櫎闈炴寚瀹氫簡 CLONE_VM锛屽湪閭ｇ鎯呭喌涓?  PSTATE.ZA 琚涓?0锛孴PIDR2_EL0 琚涓?0銆?
### 2.  Vector lengths

### 2. 鍚戦噺闀垮害


SME defines a second vector length similar to the SVE vector length which
controls the size of the streaming mode SVE vectors and the ZA matrix array.
The ZA matrix is square with each side having as many bytes as a streaming
mode SVE vector.

SME 瀹氫箟浜嗙浜屼釜鍚戦噺闀垮害锛岀被浼间簬 SVE 鍚戦噺闀垮害锛屽畠鎺у埗娴佸紡妯″紡 SVE 鍚戦噺鍜?ZA 鐭╅樀
鏁扮粍鐨勫ぇ灏忋€俍A 鐭╅樀鏄柟褰㈢殑锛屾瘡杈圭殑闀垮害绛変簬涓€涓祦寮忔ā寮?SVE 鍚戦噺鐨勫瓧鑺傛暟銆?

### 3.  System call behaviour

### 3. 绯荤粺璋冪敤琛屼负


- On syscall PSTATE.ZA is preserved, if PSTATE.ZA==1 then the contents of the
  ZA matrix and ZTn (if present) are preserved.

- 鍦ㄧ郴缁熻皟鐢ㄦ椂 PSTATE.ZA 琚繚鐣欙紝濡傛灉 PSTATE.ZA==1锛屽垯 ZA 鐭╅樀鍜?ZTn锛堣嫢瀛樺湪锛夌殑
  鍐呭琚繚鐣欍€?
- On syscall PSTATE.SM will be cleared and the SVE registers will be handled
  as per the standard SVE ABI.

- 鍦ㄧ郴缁熻皟鐢ㄦ椂 PSTATE.SM 浼氳娓呴櫎锛孲VE 瀵勫瓨鍣ㄥ皢鎸夋爣鍑?SVE ABI 澶勭悊銆?
- None of the SVE registers, ZA or ZTn are used to pass arguments to
  or receive results from any syscall.

- SVE 瀵勫瓨鍣ㄣ€乑A 鎴?ZTn 閮戒笉鐢ㄤ簬鍚戜换浣曠郴缁熻皟鐢ㄤ紶閫掑弬鏁版垨浠庝换浣曠郴缁熻皟鐢ㄦ帴鏀剁粨鏋溿€?
- On process creation (eg, clone()) the newly created process will have
  PSTATE.SM cleared.

- 鍦ㄨ繘绋嬪垱寤烘椂锛堜緥濡?clone()锛夛紝鏂板垱寤虹殑杩涚▼鍏?PSTATE.SM 浼氳娓呴櫎銆?
- All other SME state of a thread, including the currently configured vector
  length, the state of the PR_SME_VL_INHERIT flag, and the deferred vector
  length (if any), is preserved across all syscalls, subject to the specific
  exceptions for execve() described in section 6.

- 绾跨▼鐨勬墍鏈夊叾浠?SME 鐘舵€侊紝鍖呮嫭褰撳墠閰嶇疆鐨勫悜閲忛暱搴︺€丳R_SME_VL_INHERIT 鏍囧織鐨勭姸鎬侊紝
  浠ュ強寤惰繜鍚戦噺闀垮害锛堣嫢鏈夛級锛屽湪鎵€鏈夌郴缁熻皟鐢ㄤ箣闂撮兘浼氳淇濈暀锛屼絾椤婚伒寰 6 鑺備腑閽堝
  execve() 鎻忚堪鐨勭壒瀹氫緥澶栥€?

### 4.  Signal handling

### 4. 淇″彿澶勭悊


- Signal handlers are invoked with PSTATE.SM=0, PSTATE.ZA=0, and TPIDR2_EL0=0.

- 淇″彿澶勭悊鍑芥暟琚皟鐢ㄦ椂 PSTATE.SM=0銆丳STATE.ZA=0锛屼笖 TPIDR2_EL0=0銆?
- A new signal frame record TPIDR2_MAGIC is added formatted as a struct
  tpidr2_context to allow access to TPIDR2_EL0 from signal handlers.

- 鏂板浜嗕竴涓俊鍙峰抚璁板綍 TPIDR2_MAGIC锛屽叾鏍煎紡涓?struct tpidr2_context锛屼互渚夸粠淇″彿澶勭悊
  鍑芥暟璁块棶 TPIDR2_EL0銆?
- A new signal frame record za_context encodes the ZA register contents on
  signal delivery. [^1^]

- 涓€涓柊鐨勪俊鍙峰抚璁板綍 za_context 鍦ㄤ俊鍙烽€掍氦鏃剁紪鐮?ZA 瀵勫瓨鍣ㄥ唴瀹广€俒^1^]

- The signal frame record for ZA always contains basic metadata, in particular
  the thread's vector length (in za_context.vl).

- ZA 鐨勪俊鍙峰抚璁板綍濮嬬粓鍖呭惈鍩烘湰鍏冩暟鎹紝鐗瑰埆鏄嚎绋嬬殑鍚戦噺闀垮害锛堝湪 za_context.vl 涓級銆?
- The ZA matrix may or may not be included in the record, depending on
  the value of PSTATE.ZA.  The registers are present if and only if:
  za_context.head.size >= ZA_SIG_CONTEXT_SIZE(sve_vq_from_vl(za_context.vl))
  in which case PSTATE.ZA == 1.

- ZA 鐭╅樀鏄惁鍖呭惈鍦ㄨ褰曚腑锛屽彇鍐充簬 PSTATE.ZA 鐨勫€笺€傚綋涓斾粎褰?  za_context.head.size >= ZA_SIG_CONTEXT_SIZE(sve_vq_from_vl(za_context.vl))
  鏃跺瘎瀛樺櫒鎵嶅瓨鍦紝姝ゆ椂 PSTATE.ZA == 1銆?
- If matrix data is present, the remainder of the record has a vl-dependent
  size and layout.  Macros ZA_SIG_* are defined [^1^] to facilitate access to
  them.

- 濡傛灉瀛樺湪鐭╅樀鏁版嵁锛岃褰曠殑鍏朵綑閮ㄥ垎鍏锋湁渚濊禆浜?vl 鐨勫ぇ灏忓拰甯冨眬銆傚畯 ZA_SIG_* 宸茶瀹氫箟
  [^1^] 浠ヤ究浜庤闂畠浠€?
- The matrix is stored as a series of horizontal vectors in the same format as
  is used for SVE vectors.

- 鐭╅樀浠ヤ竴绯诲垪姘村钩鍚戦噺鐨勫舰寮忓瓨鍌紝閲囩敤涓?SVE 鍚戦噺鐩稿悓鐨勬牸寮忋€?
- If the ZA context is too big to fit in sigcontext.__reserved[], then extra
  space is allocated on the stack, an extra_context record is written in
  __reserved[] referencing this space.  za_context is then written in the
  extra space.  Refer to [^1^] for further details about this mechanism.

- 濡傛灉 ZA 涓婁笅鏂囧お澶ц€屾棤娉曟斁鍏?sigcontext.__reserved[]锛屽垯鍦ㄦ爤涓婂垎閰嶉澶栫┖闂达紝骞跺湪
  __reserved[] 涓啓鍏ヤ竴涓?extra_context 璁板綍鏉ュ紩鐢ㄨ绌洪棿銆傞殢鍚?za_context 琚啓鍏?  璇ラ澶栫┖闂淬€傛湁鍏虫鏈哄埗鐨勬洿澶氱粏鑺傝鍙傞槄 [^1^]銆?
- If ZTn is supported and PSTATE.ZA==1 then a signal frame record for ZTn will
  be generated.

- 濡傛灉鏀寔 ZTn 涓?PSTATE.ZA==1锛屽垯浼氱敓鎴?ZTn 鐨勪俊鍙峰抚璁板綍銆?
- The signal record for ZTn has magic ZT_MAGIC (0x5a544e01) and consists of a
  standard signal frame header followed by a struct zt_context specifying
  the number of ZTn registers supported by the system, then zt_context.nregs
  blocks of 64 bytes of data per register.

- ZTn 鐨勪俊鍙疯褰曞叿鏈夐瓟鏁?ZT_MAGIC锛?x5a544e01锛夛紝鐢变竴涓爣鍑嗕俊鍙峰抚澶达紝鍚庤窡涓€涓?  struct zt_context锛堟寚瀹氱郴缁熸敮鎸佺殑 ZTn 瀵勫瓨鍣ㄦ暟閲忥級锛岀劧鍚庢槸姣忎釜瀵勫瓨鍣?64 瀛楄妭鏁版嵁
  鐨?zt_context.nregs 涓潡缁勬垚銆?

### 5.  Signal return

### 5. 淇″彿杩斿洖


When returning from a signal handler:

浠庝俊鍙峰鐞嗗嚱鏁拌繑鍥炴椂锛?
- If there is no za_context record in the signal frame, or if the record is
  present but contains no register data as described in the previous section,
  then ZA is disabled.

- 濡傛灉淇″彿甯т腑娌℃湁 za_context 璁板綍锛屾垨鑰呰璁板綍瀛樺湪浣嗕笉鍖呭惈涓婁竴鑺傛墍杩扮殑瀵勫瓨鍣ㄦ暟鎹紝
  鍒?ZA 琚鐢ㄣ€?
- If za_context is present in the signal frame and contains matrix data then
  PSTATE.ZA is set to 1 and ZA is populated with the specified data.

- 濡傛灉淇″彿甯т腑瀛樺湪 za_context 涓斿寘鍚煩闃垫暟鎹紝鍒?PSTATE.ZA 琚涓?1锛屽苟涓?ZA 琚～鍏?  鎸囧畾鐨勬暟鎹€?
- The vector length cannot be changed via signal return.  If za_context.vl in
  the signal frame does not match the current vector length, the signal return
  attempt is treated as illegal, resulting in a forced SIGSEGV.

- 鍚戦噺闀垮害涓嶈兘閫氳繃淇″彿杩斿洖鏉ユ敼鍙樸€傚鏋滀俊鍙峰抚涓殑 za_context.vl 涓庡綋鍓嶅悜閲忛暱搴︿笉鍖归厤锛?  鍒欎俊鍙疯繑鍥炲皾璇曡瑙嗕负闈炴硶锛屽鑷村己鍒朵骇鐢?SIGSEGV銆?
- If ZTn is not supported or PSTATE.ZA==0 then it is illegal to have a
  signal frame record for ZTn, resulting in a forced SIGSEGV.

- 濡傛灉涓嶆敮鎸?ZTn 鎴?PSTATE.ZA==0锛屽垯鎷ユ湁 ZTn 鐨勪俊鍙峰抚璁板綍鏄潪娉曠殑锛屼細瀵艰嚧寮哄埗浜х敓
  SIGSEGV銆?

### 6.  prctl extensions

### 6.  prctl 鎵╁睍


Some new prctl() calls are added to allow programs to manage the SME vector
length:

鏂板浜嗕竴浜?prctl() 璋冪敤鏉ュ厑璁哥▼搴忕鐞?SME 鍚戦噺闀垮害锛?
prctl(PR_SME_SET_VL, unsigned long arg)

    Sets the vector length of the calling thread and related flags, where
    arg == vl | flags.  Other threads of the calling process are unaffected.

    vl 鏄皟鐢ㄧ嚎绋嬬殑鍚戦噺闀垮害鍙婄浉鍏虫爣蹇楋紝鍏朵腑 arg == vl | flags銆傝皟鐢ㄨ繘绋嬬殑
    鍏朵粬绾跨▼涓嶅彈褰卞搷銆?
    vl is the desired vector length, where sve_vl_valid(vl) must be true.

    vl 鏄湡鏈涚殑鍚戦噺闀垮害锛屽叾涓?sve_vl_valid(vl) 蹇呴』涓虹湡銆?
    flags:

    PR_SME_VL_INHERIT

        Inherit the current vector length across execve().  Otherwise, the
        vector length is reset to the system default at execve().  (See
        Section 9.)

        鍦?execve() 鏈熼棿缁ф壙褰撳墠鍚戦噺闀垮害銆傚惁鍒欙紝鍚戦噺闀垮害鍦?execve() 鏃惰閲嶇疆涓?        绯荤粺榛樿鍊笺€傦紙鍙傝绗?9 鑺傘€傦級

    PR_SME_SET_VL_ONEXEC

        Defer the requested vector length change until the next execve()
        performed by this thread.

        灏嗘墍璇锋眰鐨勫悜閲忛暱搴﹀彉鏇存帹杩熷埌鏈嚎绋嬫墽琛岀殑涓嬩竴娆?execve()銆?
        The effect is equivalent to implicit execution of the following
        call immediately after the next execve() (if any) by the thread:

        鍏舵晥鏋滅瓑鍚屼簬鍦ㄦ湰娆★紙鑻ユ湁锛塭xecve() 涔嬪悗鐢辫绾跨▼闅愬紡鎵ц浠ヤ笅璋冪敤锛?
        prctl(PR_SME_SET_VL, arg & ~PR_SME_SET_VL_ONEXEC)

        This allows launching of a new program with a different vector
        length, while avoiding runtime side effects in the caller.

        杩欏厑璁镐互涓嶅悓鐨勫悜閲忛暱搴﹀惎鍔ㄤ竴涓柊绋嬪簭锛屽悓鏃堕伩鍏嶅璋冪敤鑰呬骇鐢熻繍琛屾椂鍓綔鐢ㄣ€?
        Without PR_SME_SET_VL_ONEXEC, the requested change takes effect
        immediately.

        鑻ユ病鏈?PR_SME_SET_VL_ONEXEC锛屾墍璇锋眰鐨勫彉鏇翠細绔嬪嵆鐢熸晥銆?

    Return value: a nonnegative on success, or a negative value on error:
        EINVAL: SME not supported, invalid vector length requested, or
            invalid flags.


    杩斿洖鍊硷細鎴愬姛鏃朵负闈炶礋鏁帮紝鍑洪敊鏃朵负璐熷€硷細
        EINVAL锛氫笉鏀寔 SME銆佽姹備簡鏃犳晥鐨勫悜閲忛暱搴︼紝鎴栨棤鏁堢殑鏍囧織銆?

    On success:

    鎴愬姛鏃讹細

    - Either the calling thread's vector length or the deferred vector length
      to be applied at the next execve() by the thread (dependent on whether
      PR_SME_SET_VL_ONEXEC is present in arg), is set to the largest value
      supported by the system that is less than or equal to vl.  If vl ==
      SVE_VL_MAX, the value set will be the largest value supported by the
      system.

    - 璋冪敤绾跨▼鐨勫悜閲忛暱搴︼紝鎴栧皢鍦ㄤ笅涓€娆?execve() 鏃剁敱璇ョ嚎绋嬪簲鐢ㄧ殑寤惰繜鍚戦噺闀垮害锛堝彇鍐充簬
      arg 涓槸鍚﹀惈鏈?PR_SME_SET_VL_ONEXEC锛夛紝琚涓虹郴缁熸敮鎸佺殑灏忎簬鎴栫瓑浜?vl 鐨勬渶澶у€笺€?      濡傛灉 vl == SVE_VL_MAX锛屾墍璁剧殑鍊间负绯荤粺鏀寔鐨勬渶澶у€笺€?
    - Any previously outstanding deferred vector length change in the calling
      thread is cancelled.

    - 璋冪敤绾跨▼涓换浣曞厛鍓嶆湭鍐崇殑寤惰繜鍚戦噺闀垮害鍙樻洿琚彇娑堛€?
    - The returned value describes the resulting configuration, encoded as for
      PR_SME_GET_VL.  The vector length reported in this value is the new
      current vector length for this thread if PR_SME_SET_VL_ONEXEC was not
      present in arg; otherwise, the reported vector length is the deferred
      vector length that will be applied at the next execve() by the calling
      thread.

    - 杩斿洖鍊兼弿杩颁簡缁撴灉閰嶇疆锛屾寜 PR_SME_GET_VL 鐨勬柟寮忕紪鐮併€傚鏋?arg 涓笉鍚?      PR_SME_SET_VL_ONEXEC锛屾鍊间腑鎶ュ憡鐨勫悜閲忛暱搴︽槸鏈嚎绋嬫柊鐨勫綋鍓嶅悜閲忛暱搴︼紱鍚﹀垯锛?      鎶ュ憡鐨勫悜閲忛暱搴︽槸灏嗗湪璋冪敤绾跨▼鐨勪笅涓€娆?execve() 鏃跺簲鐢ㄧ殑寤惰繜鍚戦噺闀垮害銆?
    - Changing the vector length causes all of ZA, ZTn, P0..P15, FFR and all
      bits of Z0..Z31 except for Z0 bits [127:0] .. Z31 bits [127:0] to become
      unspecified, including both streaming and non-streaming SVE state.
      Calling PR_SME_SET_VL with vl equal to the thread's current vector
      length, or calling PR_SME_SET_VL with the PR_SME_SET_VL_ONEXEC flag,
      does not constitute a change to the vector length for this purpose.

    - 鏀瑰彉鍚戦噺闀垮害浼氫娇 ZA銆乑Tn銆丳0..P15銆丗FR 浠ュ強 Z0..Z31 涓櫎 Z0 姣旂壒 [127:0] ..
      Z31 姣旂壒 [127:0] 涔嬪鐨勬墍鏈夋瘮鐗瑰彉涓烘湭鎸囧畾鐘舵€侊紝鍖呮嫭娴佸紡鍜岄潪娴佸紡 SVE 鐘舵€併€?      浠ョ瓑浜庣嚎绋嬪綋鍓嶅悜閲忛暱搴︾殑 vl 璋冪敤 PR_SME_SET_VL锛屾垨浠?PR_SME_SET_VL_ONEXEC
      鏍囧織璋冪敤 PR_SME_SET_VL锛屽湪姝ゆ剰涔変笂涓嶆瀯鎴愬鍚戦噺闀垮害鐨勬洿鏀广€?
    - Changing the vector length causes PSTATE.ZA to be cleared.
      Calling PR_SME_SET_VL with vl equal to the thread's current vector
      length, or calling PR_SME_SET_VL with the PR_SME_SET_VL_ONEXEC flag,
      does not constitute a change to the vector length for this purpose.

    - 鏀瑰彉鍚戦噺闀垮害浼氬鑷?PSTATE.ZA 琚竻闄ゃ€備互绛変簬绾跨▼褰撳墠鍚戦噺闀垮害鐨?vl 璋冪敤
      PR_SME_SET_VL锛屾垨浠?PR_SME_SET_VL_ONEXEC 鏍囧織璋冪敤 PR_SME_SET_VL锛屽湪姝ゆ剰涔変笂
      涓嶆瀯鎴愬鍚戦噺闀垮害鐨勬洿鏀广€?

prctl(PR_SME_GET_VL)

    Gets the vector length of the calling thread.

    鑾峰彇璋冪敤绾跨▼鐨勫悜閲忛暱搴︺€?
    The following flag may be OR-ed into the result:

    浠ヤ笅鏍囧織鍙 OR 杩涚粨鏋滀腑锛?
        PR_SME_VL_INHERIT

        Vector length will be inherited across execve().

        鍚戦噺闀垮害灏嗗湪 execve() 鏈熼棿琚户鎵裤€?
    There is no way to determine whether there is an outstanding deferred
    vector length change (which would only normally be the case between a
    fork() or vfork() and the corresponding execve() in typical use).

    鏃犳硶鍒ゆ柇鏄惁瀛樺湪鏈喅鐨勫欢杩熷悜閲忛暱搴﹀彉鏇达紙閫氬父鍙細鍦ㄥ吀鍨嬬殑 fork() 鎴?vfork() 涓?    鐩稿簲鐨?execve() 涔嬮棿鍑虹幇锛夈€?
    To extract the vector length from the result, bitwise and it with
    PR_SME_VL_LEN_MASK.

    瑕佷粠缁撴灉涓彁鍙栧悜閲忛暱搴︼紝瀵瑰叾鎸変綅涓?PR_SME_VL_LEN_MASK銆?
    Return value: a nonnegative value on success, or a negative value on error:
        EINVAL: SME not supported.

    杩斿洖鍊硷細鎴愬姛鏃朵负闈炶礋鍊硷紝鍑洪敊鏃朵负璐熷€硷細
        EINVAL锛氫笉鏀寔 SME銆?

### 7.  ptrace extensions

### 7.  ptrace 鎵╁睍


- A new regset NT_ARM_SSVE is defined for access to streaming mode SVE
  state via PTRACE_GETREGSET and  PTRACE_SETREGSET, this is documented in
  sve.rst.

- 瀹氫箟浜嗕竴涓柊鐨?regset NT_ARM_SSVE锛岀敤浜庨€氳繃 PTRACE_GETREGSET 鍜?PTRACE_SETREGSET
  璁块棶娴佸紡妯″紡 SVE 鐘舵€侊紝杩欏湪 sve.rst 涓湁璁拌浇銆?
- A new regset NT_ARM_ZA is defined for ZA state for access to ZA state via
  PTRACE_GETREGSET and PTRACE_SETREGSET.

- 瀹氫箟浜嗕竴涓柊鐨?regset NT_ARM_ZA锛岀敤浜庨€氳繃 PTRACE_GETREGSET 鍜?PTRACE_SETREGSET
  璁块棶 ZA 鐘舵€併€?
  Refer to [^2^] for definitions.

  瀹氫箟璇峰弬闃?[^2^]銆?
The regset data starts with struct user_za_header, containing:

regset 鏁版嵁浠?struct user_za_header 寮€澶达紝鍏朵腑鍖呭惈锛?
    size

        Size of the complete regset, in bytes.
        This depends on vl and possibly on other things in the future.

        regset 鐨勫畬鏁村ぇ灏忥紝浠ュ瓧鑺備负鍗曚綅銆?        杩欏彇鍐充簬 vl锛屽皢鏉ヤ篃鍙兘鍙栧喅浜庡叾浠栧洜绱犮€?
        If a call to PTRACE_GETREGSET requests less data than the value of
        size, the caller can allocate a larger buffer and retry in order to
        read the complete regset.

        濡傛灉瀵?PTRACE_GETREGSET 鐨勮皟鐢ㄨ姹傜殑鏁版嵁灏戜簬 size 鐨勫€硷紝璋冪敤鑰呭彲浠ュ垎閰?        鏇村ぇ鐨勭紦鍐插尯骞堕噸璇曪紝浠ヨ鍙栧畬鏁寸殑 regset銆?
    max_size

        Maximum size in bytes that the regset can grow to for the target
        thread.  The regset won't grow bigger than this even if the target
        thread changes its vector length etc.

        regset 鑳戒负鐩爣绾跨▼澧為暱鍒扮殑鏈€澶у瓧鑺傛暟銆傚嵆浣跨洰鏍囩嚎绋嬫敼鍙樺叾鍚戦噺闀垮害绛夛紝
        regset 涔熶笉浼氬闀垮埌瓒呰繃姝ゅ€笺€?
    vl

        Target thread's current streaming vector length, in bytes.

        鐩爣绾跨▼褰撳墠鐨勬祦寮忓悜閲忛暱搴︼紝浠ュ瓧鑺備负鍗曚綅銆?
    max_vl

        Maximum possible streaming vector length for the target thread.

        鐩爣绾跨▼鍙兘鐨勬渶澶ф祦寮忓悜閲忛暱搴︺€?
    flags

        Zero or more of the following flags, which have the same
        meaning and behaviour as the corresponding PR_SET_VL_* flags:

        浠ヤ笅涓€涓垨澶氫釜鏍囧織锛屽叾鍚箟鍜岃涓轰笌瀵瑰簲鐨?PR_SET_VL_* 鏍囧織鐩稿悓锛?
            SME_PT_VL_INHERIT

            SME_PT_VL_ONEXEC (SETREGSET only).

- The effects of changing the vector length and/or flags are equivalent to
  those documented for PR_SME_SET_VL.

- 鏀瑰彉鍚戦噺闀垮害鍜?鎴栨爣蹇楃殑鏁堟灉绛夊悓浜?PR_SME_SET_VL 涓杞界殑鏁堟灉銆?
  The caller must make a further GETREGSET call if it needs to know what VL is
  actually set by SETREGSET, unless is it known in advance that the requested
  VL is supported.

  濡傛灉璋冪敤鑰呴渶瑕佺煡閬?SETREGSET 瀹為檯璁剧疆鐨?VL锛屽垯蹇呴』杩涜杩涗竴姝ョ殑 GETREGSET 璋冪敤锛?  闄ら潪浜嬪厛宸茬煡鎵€璇锋眰鐨?VL 鍙楁敮鎸併€?
- The size and layout of the payload depends on the header fields.  The
  ZA_PT_ZA*() macros are provided to facilitate access to the data.

- 璐熻浇鐨勫ぇ灏忓拰甯冨眬鍙栧喅浜庡ご閮ㄥ瓧娈点€傛彁渚涗簡 ZA_PT_ZA*() 瀹忎互渚夸簬璁块棶鏁版嵁銆?
- In either case, for SETREGSET it is permissible to omit the payload, in which
  case the vector length and flags are changed and PSTATE.ZA is set to 0
  (along with any consequences of those changes).  If a payload is provided
  then PSTATE.ZA will be set to 1.

- 鏃犺鍝鎯呭喌锛屽浜?SETREGSET锛屽彲浠ョ渷鐣ヨ礋杞斤紝姝ゆ椂鍚戦噺闀垮害鍜屾爣蹇椾細琚敼鍙樹笖
  PSTATE.ZA 琚涓?0锛堜互鍙婅繖浜涘彉鏇村甫鏉ョ殑浠讳綍鍚庢灉锛夈€傚鏋滄彁渚涗簡璐熻浇锛屽垯 PSTATE.ZA
  灏嗚璁句负 1銆?
- For SETREGSET, if the requested VL is not supported, the effect will be the
  same as if the payload were omitted, except that an EIO error is reported.
  No attempt is made to translate the payload data to the correct layout
  for the vector length actually set.  It is up to the caller to translate the
  payload layout for the actual VL and retry.

- 瀵逛簬 SETREGSET锛屽鏋滄墍璇锋眰鐨?VL 涓嶅彈鏀寔锛屽叾鏁堟灉涓庣渷鐣ヨ礋杞界浉鍚岋紝鍙槸浼氭姤鍛婁竴涓?  EIO 閿欒銆備笉浼氬皾璇曞皢璐熻浇鏁版嵁杞崲涓哄疄闄呰缃殑鍚戦噺闀垮害鎵€瀵瑰簲鐨勬纭竷灞€銆傜敱璋冪敤鑰?  璐熻矗涓哄疄闄呯殑 VL 杞崲璐熻浇甯冨眬骞堕噸璇曘€?
- The effect of writing a partial, incomplete payload is unspecified.

- 鍐欏叆涓嶅畬鏁寸殑銆侀儴鍒嗚礋杞界殑鏁堟灉鏄湭鎸囧畾鐨勩€?
- A new regset NT_ARM_ZT is defined for access to ZTn state via
  PTRACE_GETREGSET and PTRACE_SETREGSET.

- 瀹氫箟浜嗕竴涓柊鐨?regset NT_ARM_ZT锛岀敤浜庨€氳繃 PTRACE_GETREGSET 鍜?PTRACE_SETREGSET
  璁块棶 ZTn 鐘舵€併€?
- The NT_ARM_ZT regset consists of a single 512 bit register.

- NT_ARM_ZT regset 鐢变竴涓崟鐙殑 512 浣嶅瘎瀛樺櫒缁勬垚銆?
- When PSTATE.ZA==0 reads of NT_ARM_ZT will report all bits of ZTn as 0.

- 褰?PSTATE.ZA==0 鏃讹紝瀵?NT_ARM_ZT 鐨勮鍙栦細灏?ZTn 鐨勬墍鏈夋瘮鐗规姤鍛婁负 0銆?
- Writes to NT_ARM_ZT will set PSTATE.ZA to 1.

- 瀵?NT_ARM_ZT 鐨勫啓鍏ヤ細灏?PSTATE.ZA 璁句负 1銆?
- If any register data is provided along with SME_PT_VL_ONEXEC then the
  registers data will be interpreted with the current vector length, not
  the vector length configured for use on exec.

- 濡傛灉鎻愪緵浜嗕换浣曞瘎瀛樺櫒鏁版嵁骞跺悓鏃跺甫鏈?SME_PT_VL_ONEXEC锛屽垯瀵勫瓨鍣ㄦ暟鎹皢浣跨敤褰撳墠鍚戦噺
  闀垮害鏉ヨВ閲婏紝鑰岄潪涓?exec 閰嶇疆浣跨敤鐨勫悜閲忛暱搴︺€?

### 8.  ELF coredump extensions

### 8.  ELF coredump 鎵╁睍


- NT_ARM_SSVE notes will be added to each coredump for
  each thread of the dumped process.  The contents will be equivalent to the
  data that would have been read if a PTRACE_GETREGSET of the corresponding
  type were executed for each thread when the coredump was generated.

- 灏嗕负琚?dump 杩涚▼鐨勬瘡涓€涓嚎绋嬶紝鍦ㄦ瘡涓?coredump 涓坊鍔?NT_ARM_SSVE 澶囨敞銆傚叾鍐呭绛夊悓浜?  鍦ㄧ敓鎴?coredump 鏃讹紝濡傛灉瀵规瘡涓嚎绋嬫墽琛屼簡鐩稿簲绫诲瀷鐨?PTRACE_GETREGSET 鎵€浼氳鍒扮殑鏁版嵁銆?
- A NT_ARM_ZA note will be added to each coredump for each thread of the
  dumped process.  The contents will be equivalent to the data that would have
  been read if a PTRACE_GETREGSET of NT_ARM_ZA were executed for each thread
  when the coredump was generated.

- 灏嗕负琚?dump 杩涚▼鐨勬瘡涓€涓嚎绋嬶紝鍦ㄦ瘡涓?coredump 涓坊鍔?NT_ARM_ZA 澶囨敞銆傚叾鍐呭绛夊悓浜?  鍦ㄧ敓鎴?coredump 鏃讹紝濡傛灉瀵规瘡涓嚎绋嬫墽琛屼簡 NT_ARM_ZA 鐨?PTRACE_GETREGSET 鎵€浼氳鍒扮殑
  鏁版嵁銆?
- A NT_ARM_ZT note will be added to each coredump for each thread of the
  dumped process.  The contents will be equivalent to the data that would have
  been read if a PTRACE_GETREGSET of NT_ARM_ZT were executed for each thread
  when the coredump was generated.

- 灏嗕负琚?dump 杩涚▼鐨勬瘡涓€涓嚎绋嬶紝鍦ㄦ瘡涓?coredump 涓坊鍔?NT_ARM_ZT 澶囨敞銆傚叾鍐呭绛夊悓浜?  鍦ㄧ敓鎴?coredump 鏃讹紝濡傛灉瀵规瘡涓嚎绋嬫墽琛屼簡 NT_ARM_ZT 鐨?PTRACE_GETREGSET 鎵€浼氳鍒扮殑
  鏁版嵁銆?
- The NT_ARM_TLS note will be extended to two registers, the second register
  will contain TPIDR2_EL0 on systems that support SME and will be read as
  zero with writes ignored otherwise.

- NT_ARM_TLS 澶囨敞灏嗘墿灞曞埌涓や釜瀵勫瓨鍣紝绗簩涓瘎瀛樺櫒鍦ㄦ敮鎸?SME 鐨勭郴缁熶笂灏嗗寘鍚?  TPIDR2_EL0锛屽惁鍒欏皢琚浣滈浂涓斿啓鍏ヨ蹇界暐銆?
### 9.  System runtime configuration

### 9. 绯荤粺杩愯鏃堕厤缃?

- To mitigate the ABI impact of expansion of the signal frame, a policy
  mechanism is provided for administrators, distro maintainers and developers
  to set the default vector length for userspace processes:

- 涓哄噺杞讳俊鍙峰抚鎵╁睍甯︽潵鐨?ABI 褰卞搷锛屾彁渚涗簡涓€绉嶇瓥鐣ユ満鍒讹紝渚涚鐞嗗憳銆佸彂琛岀増缁存姢鑰呭拰
  寮€鍙戣€呬负鐢ㄦ埛绌洪棿杩涚▼璁剧疆榛樿鍚戦噺闀垮害锛?
/proc/sys/abi/sme_default_vector_length

    Writing the text representation of an integer to this file sets the system
    default vector length to the specified value rounded to a supported value
    using the same rules as for setting vector length via PR_SME_SET_VL.

    鍚戞鏂囦欢鍐欏叆涓€涓暣鏁扮殑鏂囨湰琛ㄧず锛屼細灏嗙郴缁熼粯璁ゅ悜閲忛暱搴﹁涓烘寚瀹氱殑鍊硷紙鎸変笌閫氳繃
    PR_SME_SET_VL 璁剧疆鍚戦噺闀垮害鐩稿悓鐨勮鍒欏彇鏁村埌鍙楁敮鎸佺殑鍊硷級銆?
    The result can be determined by reopening the file and reading its
    contents.

    鍙€氳繃閲嶆柊鎵撳紑鏂囦欢骞惰鍙栧叾鍐呭鏉ョ‘瀹氱粨鏋溿€?
    At boot, the default vector length is initially set to 32 or the maximum
    supported vector length, whichever is smaller and supported.  This
    determines the initial vector length of the init process (PID 1).

    鍦ㄥ惎鍔ㄦ椂锛岄粯璁ゅ悜閲忛暱搴︽渶鍒濊璁句负 32 鎴栨渶澶у彈鏀寔鐨勫悜閲忛暱搴︼紝鍙栧叾涓緝灏忎笖鍙楁敮鎸佽€呫€?    杩欏喅瀹氫簡 init 杩涚▼锛圥ID 1锛夌殑鍒濆鍚戦噺闀垮害銆?
    Reading this file returns the current system default vector length.

    璇诲彇姝ゆ枃浠惰繑鍥炲綋鍓嶇郴缁熼粯璁ゅ悜閲忛暱搴︺€?
- At every execve() call, the new vector length of the new process is set to
  the system default vector length, unless

- 鍦ㄦ瘡娆?execve() 璋冪敤鏃讹紝鏂拌繘绋嬬殑鏂板悜閲忛暱搴﹁璁句负绯荤粺榛樿鍚戦噺闀垮害锛岄櫎闈?
    - PR_SME_VL_INHERIT (or equivalently SME_PT_VL_INHERIT) is set for the
      calling thread, or

    - 涓鸿皟鐢ㄧ嚎绋嬭缃簡 PR_SME_VL_INHERIT锛堟垨绛変环鐨?SME_PT_VL_INHERIT锛夛紝鎴?
    - a deferred vector length change is pending, established via the
      PR_SME_SET_VL_ONEXEC flag (or SME_PT_VL_ONEXEC).

    - 瀛樺湪涓€涓€氳繃 PR_SME_SET_VL_ONEXEC 鏍囧織锛堟垨 SME_PT_VL_ONEXEC锛夊缓绔嬬殑寰呭畾
      寤惰繜鍚戦噺闀垮害鍙樻洿銆?
- Modifying the system default vector length does not affect the vector length
  of any existing process or thread that does not make an execve() call.

- 淇敼绯荤粺榛樿鍚戦噺闀垮害涓嶄細褰卞搷浠讳綍涓嶈繘琛?execve() 璋冪敤鐨勫凡鏈夎繘绋嬫垨绾跨▼鐨勫悜閲忛暱搴︺€?

## Appendix A.  SME programmer's model (informative)

## 闄勫綍 A.  SME 缂栫▼妯″瀷锛堜粎渚涘弬鑰冿級


This section provides a minimal description of the additions made by SME to the
ARMv8-A programmer's model that are relevant to this document.

鏈妭绠€瑕佹弿杩颁簡 SME 瀵逛笌鏈枃妗ｇ浉鍏崇殑 ARMv8-A 缂栫▼妯″瀷鎵€鍋氱殑琛ュ厖銆?
Note: This section is for information only and not intended to be complete or
to replace any architectural specification.

娉ㄦ剰锛氭湰鑺備粎渚涘弬鑰冿紝鏃犳剰瀹屾暣锛屼篃涓嶅彇浠ｄ换浣曟灦鏋勮鑼冦€?
### A.1.  Registers

### A.1.  瀵勫瓨鍣?

In A64 state, SME adds the following:

鍦?A64 鐘舵€佷笅锛孲ME 澧炲姞浜嗕互涓嬪唴瀹癸細

- A new mode, streaming mode, in which a subset of the normal FPSIMD and SVE
  features are available.  When supported EL0 software may enter and leave
  streaming mode at any time.

- 涓€绉嶆柊妯″紡锛屽嵆娴佸紡妯″紡锛坰treaming mode锛夛紝鍦ㄨ妯″紡涓嬪彲鐢ㄦ甯?FPSIMD 鍜?SVE 鐗规€х殑
  涓€涓瓙闆嗐€傚綋鍙楁敮鎸佹椂锛孍L0 杞欢鍙互闅忔椂杩涘叆鍜岀寮€娴佸紡妯″紡銆?
  For best system performance it is strongly encouraged for software to enable
  streaming mode only when it is actively being used.

  涓轰簡鑾峰緱鏈€浣崇郴缁熸€ц兘锛屽己鐑堝缓璁蒋浠朵粎鍦ㄤ富鍔ㄤ娇鐢ㄦ祦寮忔ā寮忔椂鎵嶅惎鐢ㄥ畠銆?
- A new vector length controlling the size of ZA and the Z registers when in
  streaming mode, separately to the vector length used for SVE when not in
  streaming mode.  There is no requirement that either the currently selected
  vector length or the set of vector lengths supported for the two modes in
  a given system have any relationship.  The streaming mode vector length
  is referred to as SVL.

- 涓€涓柊鐨勫悜閲忛暱搴︼紝鐢ㄤ簬鎺у埗澶勪簬娴佸紡妯″紡鏃?ZA 鍜?Z 瀵勫瓨鍣ㄧ殑澶у皬锛屼笌涓嶅湪娴佸紡妯″紡鏃?  鐢ㄤ簬 SVE 鐨勫悜閲忛暱搴︾浉浜掔嫭绔嬨€傚浜庢煇涓粰瀹氱郴缁燂紝褰撳墠閫夋嫨鐨勫悜閲忛暱搴︼紝鎴栦袱绉嶆ā寮忔墍
  鏀寔鐨勫悜閲忛暱搴﹂泦鍚堬紝閮戒笉瑕佹眰鏈変换浣曞叧绯汇€傛祦寮忔ā寮忕殑鍚戦噺闀垮害琚О涓?SVL銆?
- A new ZA matrix register.  This is a square matrix of SVLxSVL bits.  Most
  operations on ZA require that streaming mode be enabled but ZA can be
  enabled without streaming mode in order to load, save and retain data.

- 涓€涓柊鐨?ZA 鐭╅樀瀵勫瓨鍣ㄣ€傝繖鏄竴涓?SVLxSVL 姣旂壒鐨勬柟褰㈢煩闃点€傚 ZA 鐨勫ぇ澶氭暟鎿嶄綔瑕佹眰
  鍚敤娴佸紡妯″紡锛屼絾 ZA 鍙互鍦ㄤ笉鍚敤娴佸紡妯″紡鐨勬儏鍐典笅琚惎鐢紝浠ヤ究鍔犺浇銆佷繚瀛樺拰淇濈暀鏁版嵁銆?
  For best system performance it is strongly encouraged for software to enable
  ZA only when it is actively being used.

  涓轰簡鑾峰緱鏈€浣崇郴缁熸€ц兘锛屽己鐑堝缓璁蒋浠朵粎鍦ㄤ富鍔ㄤ娇鐢?ZA 鏃舵墠鍚敤瀹冦€?
- A new ZT0 register is introduced when SME2 is present. This is a 512 bit
  register which is accessible when PSTATE.ZA is set, as ZA itself is.

- 褰?SME2 瀛樺湪鏃跺紩鍏ヤ竴涓柊鐨?ZT0 瀵勫瓨鍣ㄣ€傝繖鏄竴涓?512 浣嶅瘎瀛樺櫒锛屽湪 PSTATE.ZA 琚?  璁剧疆鏃跺彲璁块棶锛屾濡?ZA 鏈韩涓€鏍枫€?
- Two new 1 bit fields in PSTATE which may be controlled via the SMSTART and
  SMSTOP instructions or by access to the SVCR system register:

- PSTATE 涓袱涓柊鐨?1 姣旂壒瀛楁锛屽彲閫氳繃 SMSTART 鍜?SMSTOP 鎸囦护锛屾垨閫氳繃瀵?SVCR 绯荤粺
  瀵勫瓨鍣ㄧ殑璁块棶鏉ユ帶鍒讹細

  - PSTATE.ZA, if this is 1 then the ZA matrix is accessible and has valid
    data while if it is 0 then ZA can not be accessed.  When PSTATE.ZA is
    changed from 0 to 1 all bits in ZA are cleared.

  - PSTATE.ZA锛屽鏋滀负 1锛屽垯 ZA 鐭╅樀鍙闂笖鍚湁鏈夋晥鏁版嵁锛涘鏋滀负 0锛屽垯 ZA 涓嶅彲璁块棶銆?    褰?PSTATE.ZA 浠?0 鍙樹负 1 鏃讹紝ZA 涓殑鎵€鏈夋瘮鐗硅娓呴櫎銆?
  - PSTATE.SM, if this is 1 then the PE is in streaming mode.  When the value
    of PSTATE.SM is changed then it is implementation defined if the subset
    of the floating point register bits valid in both modes may be retained.
    Any other bits will be cleared.

  - PSTATE.SM锛屽鏋滀负 1锛屽垯 PE 澶勪簬娴佸紡妯″紡銆傚綋 PSTATE.SM 鐨勫€兼敼鍙樻椂锛屼袱绉嶆ā寮忛兘
    鏈夋晥鐨勬诞鐐瑰瘎瀛樺櫒姣旂壒瀛愰泦鏄惁鍙繚鐣欐槸鐢卞叿浣撳疄鐜板畾涔夌殑銆備换浣曞叾浠栨瘮鐗归兘灏嗚娓呴櫎銆?

## References

## 鍙傝€?

[^1^] arch/arm64/include/uapi/asm/sigcontext.h
    AArch64 Linux signal ABI definitions

[^2^] arch/arm64/include/uapi/asm/ptrace.h
    AArch64 Linux ptrace ABI definitions

[^3^] Documentation/arch/arm64/cpu-feature-registers.rst
