
## 缁忓吀 BPF 涓?eBPF


eBPF 琚璁′负浠ヤ竴涓€瀵瑰簲鐨勬柟寮忚繘琛?JIT 缂栬瘧锛岃繖涔熶负
GCC/LLVM 缂栬瘧鍣ㄩ€氳繃
涓€涓?eBPF 鍚庣鐢熸垚鍑犱箮涓庡師鐢熺紪璇戜唬鐮佷竴鏍峰揩鐨勪紭鍖?eBPF 浠ｇ爜寮€杈熶簡鍙兘銆?

eBPF 鏍煎紡鐩稿浜庣粡鍏?BPF 鐨勪竴浜涙牳蹇冨彉鍖栵細

- 瀵勫瓨鍣ㄦ暟閲忎粠 2 涓鍔犲埌 10 涓細

  鏃ф牸寮忔湁涓や釜瀵勫瓨鍣?A 鍜?X锛屼互鍙婁竴涓殣钘忕殑甯ф寚閽堛€傛柊甯冨眬灏嗗叾鎵╁睍涓?10 涓唴閮ㄥ瘎瀛樺櫒鍜屼竴涓彧璇诲抚鎸囬拡銆傜敱浜?64 浣?CPU 閫氳繃瀵勫瓨鍣ㄥ悜鍑芥暟浼犻€掑弬鏁帮紝鍥犳 eBPF 绋嬪簭鍚戝唴鏍稿嚱鏁颁紶閫掔殑鍙傛暟涓暟琚檺鍒朵负 5 涓紝骞跺彟鏈変竴涓瘎瀛樺櫒鐢ㄤ簬鎺ユ敹鍐呮牳鍑芥暟鐨勮繑鍥炲€笺€傚湪鍘熺敓璋冪敤绾﹀畾涓紝x86_64 閫氳繃瀵勫瓨鍣ㄤ紶閫掑墠 6 涓弬鏁帮紝aarch64/sparcv9/mips64 鏈?7~8 涓瘎瀛樺櫒鐢ㄤ簬浼犲弬锛泋86_64 鏈?6 涓璋冪敤鑰呬繚瀛橈紙callee saved锛夊瘎瀛樺櫒锛岃€?aarch64/sparcv9/mips64 鏈?11 涓垨鏇村琚皟鐢ㄨ€呬繚瀛樺瘎瀛樺櫒銆?

  鍥犳锛屽湪 x86_64銆乤arch64 绛夋灦鏋勪笂锛屾墍鏈?eBPF 瀵勫瓨鍣ㄩ兘涓庣‖浠跺瘎瀛樺櫒涓€涓€瀵瑰簲锛宔BPF 璋冪敤绾﹀畾涔熺洿鎺ユ槧灏勫埌 64 浣嶆灦鏋勪笂鍐呮牳鎵€浣跨敤鐨?ABI銆?

  鍦?32 浣嶆灦鏋勪笂锛孞IT 鍙互瀵逛粎浣跨敤 32 浣嶇畻鏈繍绠楃殑绋嬪簭杩涜鏄犲皠锛岃€屾洿澶嶆潅鐨勭▼搴忓垯鍙兘琚氦鐢辫В閲婂櫒鎵ц銆?

  R0 - R5 are scratch registers and eBPF program needs spill/fill them if
  necessary across calls. Note that there is only one eBPF program (== one
  eBPF main routine) and it cannot call other eBPF functions, it can only
  call predefined in-kernel functions, though.

- 瀵勫瓨鍣ㄤ綅瀹戒粠 32 浣嶅鍔犲埌 64 浣嶏細

  Still, the semantics of the original 32-bit ALU operations are preserved
  via 32-bit subregisters. All eBPF registers are 64-bit with 32-bit lower
  subregisters that zero-extend into 64-bit if they are being written to.
  That behavior maps directly to x86_64 and arm64 subregister definition, but
  makes other JITs more difficult.

  32-bit architectures run 64-bit eBPF programs via interpreter.
  Their JITs may convert BPF programs that only use 32-bit subregisters into
  native instruction set and let the rest being interpreted.

  Operation is 64-bit, because on 64-bit architectures, pointers are also
  64-bit wide, and we want to pass 64-bit values in/out of kernel functions,
  so 32-bit eBPF registers would otherwise require to define register-pair
  ABI, thus, there won't be able to use a direct eBPF register to HW register
  mapping and JIT would need to do combine/split/move operations for every
  register in and out of the function, which is complex, bug prone and slow.
  Another reason is the use of atomic 64-bit counters.

- 鏉′欢璺宠浆鐨?jt/jf 鐩爣琚浛鎹负 jt/椤哄簭鎵ц锛坒all-through锛夛細

  灏界鍘熷璁捐涓湁璇稿 ``if (cond) jump_true; else jump_false;`` 杩欐牱鐨勭粨鏋勶紝浣嗗畠浠琚浛鎹负绫讳技 `if (cond) jump_true; /** else fall-through **/` 鐨勬浛浠ｇ粨鏋勩€?

- 寮曞叆浜?bpf_call 鎸囦护浠ュ強闆跺紑閿€鐨勫瘎瀛樺櫒浼犲弬绾﹀畾锛岀敤浜庝笌鍏朵粬鍐呮牳鍑芥暟涔嬮棿鐨勮皟鐢細

  鍦ㄨ繘琛屽唴鏍稿嚱鏁拌皟鐢ㄤ箣鍓嶏紝eBPF 绋嬪簭闇€瑕佹寜鐓ц皟鐢ㄧ害瀹氬皢鍑芥暟鍙傛暟鏀惧叆 R1 鑷?R5 瀵勫瓨鍣紝闅忓悗瑙ｉ噴鍣ㄤ細浠庤繖浜涘瘎瀛樺櫒涓彇鍑哄弬鏁板苟浼犻€掔粰鍐呮牳鍑芥暟銆傚鏋?R1~R5 瀵勫瓨鍣ㄨ鏄犲皠鍒扮粰瀹氭灦鏋勪笂鐢ㄤ簬浼犲弬鐨?CPU 瀵勫瓨鍣紝鍒?JIT 缂栬瘧鍣ㄦ棤闇€棰濆鍙戝嚭鏁版嵁绉诲姩鎸囦护銆傚嚱鏁板弬鏁板皢浣嶄簬姝ｇ‘鐨勫瘎瀛樺櫒涓紝BPF_CALL 鎸囦护涔熶細琚?JIT 缂栬瘧涓哄崟鏉?'call' 纭欢鎸囦护銆傞€夋嫨杩欑璋冪敤绾﹀畾鏄负浜嗗湪涓嶆崯澶辨€ц兘鐨勫墠鎻愪笅瑕嗙洊甯歌鐨勮皟鐢ㄥ満鏅€?

  鍦ㄥ唴鏍稿嚱鏁拌皟鐢ㄤ箣鍚庯紝R1~R5 浼氳閲嶇疆涓轰笉鍙鐘舵€侊紝鑰?R0 涓繚瀛樼潃鍑芥暟鐨勮繑鍥炲€笺€傜敱浜?R6~R9 鏄璋冪敤鑰呬繚瀛橈紙callee saved锛夊瘎瀛樺櫒锛屽叾鐘舵€佷細鍦ㄨ皟鐢ㄨ繃绋嬩腑寰椾互淇濈暀銆?

```

    u64 f1() { return (*_f2)(1); }
    u64 f2(u64 a) { return f3(a + 1, a); }
    u64 f3(u64 a, u64 b) { return a - b; }

  GCC can compile f1, f3 into x86_64::

    f1:
	movl $1, %edi
	movq _f2(%rip), %rax
	jmp  *%rax
    f3:
	movq %rdi, %rax
	subq %rsi, %rax
	ret

  Function f2 in eBPF may look like::

    f2:
	bpf_mov R2, R1
	bpf_add R1, 1
	bpf_call f3
	bpf_exit

  If f2 is JITed and the pointer stored to ``_f2``. The calls f1 -> f2 -> f3 and
  returns will be seamless. Without JIT, __bpf_prog_run() interpreter needs to
  be used to call into f2.

  For practical reasons all eBPF programs have only one argument 'ctx' which is
  already placed into R1 (e.g. on __bpf_prog_run() startup) and the programs
  can call kernel functions with up to 5 arguments. Calls with 6 or more arguments
  are currently not supported, but these restrictions can be lifted if necessary
  in the future.

  On 64-bit architectures all register map to HW registers one to one. For
  example, x86_64 JIT compiler can map them as ...

  ::

    R0 - rax
    R1 - rdi
    R2 - rsi
    R3 - rdx
    R4 - rcx
    R5 - r8
    R6 - rbx
    R7 - r13
    R8 - r14
    R9 - r15
    R10 - rbp

  ... since x86_64 ABI mandates rdi, rsi, rdx, rcx, r8, r9 for argument passing
  and rbx, r12 - r15 are callee saved.

  Then the following eBPF pseudo-program::

    bpf_mov R6, R1 /* save ctx */
    bpf_mov R2, 2
    bpf_mov R3, 3
    bpf_mov R4, 4
    bpf_mov R5, 5
    bpf_call foo
    bpf_mov R7, R0 /* save foo() return value */
    bpf_mov R1, R6 /* restore ctx for next call */
    bpf_mov R2, 6
    bpf_mov R3, 7
    bpf_mov R4, 8
    bpf_mov R5, 9
    bpf_call bar
    bpf_add R0, R7
    bpf_exit

  After JIT to x86_64 may look like::

    push %rbp
    mov %rsp,%rbp
    sub $0x228,%rsp
    mov %rbx,-0x228(%rbp)
    mov %r13,-0x220(%rbp)
    mov %rdi,%rbx
    mov $0x2,%esi
    mov $0x3,%edx
    mov $0x4,%ecx
    mov $0x5,%r8d
    callq foo
    mov %rax,%r13
    mov %rbx,%rdi
    mov $0x6,%esi
    mov $0x7,%edx
    mov $0x8,%ecx
    mov $0x9,%r8d
    callq bar
    add %r13,%rax
    mov -0x228(%rbp),%rbx
    mov -0x220(%rbp),%r13
    leaveq
    retq

  Which is in this example equivalent in C to::

    u64 bpf_filter(u64 ctx)
    {
	return foo(ctx, 2, 3, 4, 5) + bar(ctx, 6, 7, 8, 9);
    }

  In-kernel functions foo() and bar() with prototype: u64 (*)(u64 arg1, u64
  arg2, u64 arg3, u64 arg4, u64 arg5); will receive arguments in proper
  registers and place their return value into ``%rax`` which is R0 in eBPF.
  Prologue and epilogue are emitted by JIT and are implicit in the
  interpreter. R0-R5 are scratch registers, so eBPF program needs to preserve
  them across the calls as defined by calling convention.

  For example the following program is invalid::

    bpf_mov R1, 1
    bpf_call foo
    bpf_mov R0, R1
    bpf_exit

  After the call the registers R1-R5 contain junk values and cannot be read.
  An in-kernel verifier.rst is used to validate eBPF programs.

```
鍚屾牱鍦ㄦ柊鐨勮璁′腑锛宔BPF 琚檺鍒朵负 4096 鏉℃寚浠わ紝杩欐剰鍛崇潃浠讳綍
绋嬪簭閮戒細蹇€熺粓姝紝骞朵笖鍙細璋冪敤鍥哄畾鏁伴噺鐨勫唴鏍?
鍑芥暟銆傚師濮?BPF 涓?eBPF 閮芥槸鍙屾搷浣滄暟鎸囦护锛?
杩欐湁鍔╀簬鍦?JIT 鏈熼棿瀹炵幇 eBPF 鎸囦护涓?x86 鎸囦护涔嬮棿鐨勪竴涓€瀵瑰簲鏄犲皠銆?

璋冪敤瑙ｉ噴鍣ㄥ嚱鏁扮殑杈撳叆涓婁笅鏂囨寚閽堟槸閫氱敤鐨勶紝
鍏跺唴瀹圭敱鍏蜂綋鐨勭敤渚嬪畾涔夈€傚浜?seccomp锛屽瘎瀛樺櫒 R1 鎸囧悜
seccomp_data锛涘浜庤浆鎹㈠悗鐨?BPF 杩囨护鍣紝R1 鎸囧悜 skb銆?

```

  op:16, jt:8, jf:8, k:32    ==>    op:8, dst_reg:4, src_reg:4, off:16, imm:32

```
鍒扮洰鍓嶄负姝㈠凡瀹炵幇 87 鏉?eBPF 鎸囦护銆? 浣?'op' 鎿嶄綔鐮佸瓧娈?
涓烘柊鎸囦护鐣欐湁绌洪棿銆傚叾涓竴浜涘彲鑳戒娇鐢?16/24/32 瀛楄妭鐨勭紪鐮併€傛柊
鎸囦护蹇呴』鏄?8 瀛楄妭鐨勬暣鏁板€嶏紝浠ヤ繚鎸佸悜鍚庡吋瀹广€?

eBPF 鏄竴涓€氱敤鐨?RISC 鎸囦护闆嗐€傚苟闈炴瘡涓瘎瀛樺櫒鍜?
姣忔潯鎸囦护閮戒細鍦ㄤ粠鍘熷 BPF 鍒?eBPF 鐨勮浆鎹㈣繃绋嬩腑琚敤鍒般€?
渚嬪锛宻ocket 杩囨护鍣ㄤ笉浼氫娇鐢?`exclusive add` 鎸囦护锛屼絾
tracing 杩囨护鍣ㄥ彲鑳戒細浣跨敤瀹冩潵缁存姢浜嬩欢璁℃暟鍣ㄧ瓑銆備緥濡傦紝瀵勫瓨鍣?R9
涔熶笉浼氳 socket 杩囨护鍣ㄤ娇鐢紝浣嗘洿澶嶆潅鐨勮繃婊ゅ櫒鍙兘浼?
鐢ㄥ敖瀵勫瓨鍣紝浠庤€屼笉寰椾笉鍊熷姪鏍堜笂鐨勬孩鍑?鍥炲～锛坰pill/fill锛夈€?

eBPF 鍙敤浣滈€氱敤姹囩紪鍣紝鐢ㄤ簬鏈€鍚庣殑鎬ц兘
浼樺寲锛宻ocket 杩囨护鍣ㄥ拰 seccomp 灏嗗叾鐢ㄤ綔姹囩紪鍣ㄣ€倀racing
杩囨护鍣ㄥ彲鑳藉皢鍏剁敤浣滄眹缂栧櫒锛屼互渚夸粠鍐呮牳鐢熸垚浠ｇ爜銆傚湪鍐呮牳涓娇鐢ㄦ椂
鍙兘涓嶅彈瀹夊叏鍥犵礌鐨勯檺鍒讹紝鍥犱负鐢熸垚鐨?eBPF 浠ｇ爜
鍙兘鍙槸鍦ㄤ紭鍖栧唴閮ㄤ唬鐮佽矾寰勶紝鑰屼笉浼氭毚闇茬粰鐢ㄦ埛绌洪棿銆?
eBPF 鐨勫畨鍏ㄦ€у彲鏉ヨ嚜 verifier.rst銆傚湪涓婅堪杩欑被鐢ㄤ緥涓紝
瀹冨彲浠ヨ褰撲綔瀹夊叏鐨勬寚浠ら泦浣跨敤銆?

涓庡師濮?BPF 涓€鏍凤紝eBPF 杩愯鍦ㄥ彈鎺х幆澧冧腑锛?
鍏锋湁纭畾鎬э紝鍐呮牳鍙互杞绘槗璇佹槑鍏跺畨鍏ㄦ€с€傜▼搴忕殑瀹夊叏鎬?
鍙互閫氳繃涓ゆ纭畾锛氱涓€姝ヨ繘琛屾繁搴︿紭鍏堟悳绱紝浠ョ姝?
寰幆鍜屽叾浠?CFG 鏍￠獙锛涚浜屾浠庣涓€鏉℃寚浠ゅ紑濮嬶紝
閬嶅巻鎵€鏈夊彲鑳借矾寰勩€傚畠浼氭ā鎷熸瘡鏉℃寚浠ょ殑鎵ц骞惰瀵?
瀵勫瓨鍣ㄤ笌鏍堢殑鐘舵€佸彉鍖栥€?

## 鎿嶄綔鐮佺紪鐮?


eBPF 澶嶇敤浜嗙粡鍏?BPF 鐨勫ぇ閮ㄥ垎鎿嶄綔鐮佺紪鐮侊紝浠ョ畝鍖栦粠缁忓吀 BPF
鍒?eBPF 鐨勮浆鎹€?

瀵逛簬绠楁湳鍜岃烦杞寚浠わ紝8 浣?'code' 瀛楁琚垝鍒嗕负涓変釜
```

  +----------------+--------+--------------------+
  |   4 bits       |  1 bit |   3 bits           |
  | operation code | source | instruction class  |
  +----------------+--------+--------------------+
  (MSB)                                      (LSB)

```
涓変釜鏈€浣庢湁鏁堜綅锛圠SB锛夊瓨鍌ㄦ寚浠ょ被鍒紝绫诲埆涔嬩竴鏄細

  ===================     ===============
  Classic BPF classes     eBPF classes
  ===================     ===============
  BPF_LD    0x00          BPF_LD    0x00
  BPF_LDX   0x01          BPF_LDX   0x01
  BPF_ST    0x02          BPF_ST    0x02
  BPF_STX   0x03          BPF_STX   0x03
  BPF_ALU   0x04          BPF_ALU   0x04
  BPF_JMP   0x05          BPF_JMP   0x05
  BPF_RET   0x06          BPF_JMP32 0x06
  BPF_MISC  0x07          BPF_ALU64 0x07
  ===================     ===============

绗?4 浣嶅婧愭搷浣滄暟杩涜缂栫爜鈥︹€?

```

	BPF_K     0x00
	BPF_X     0x08

 * in classic BPF, this means::

	BPF_SRC(code) == BPF_X - use register X as source operand
	BPF_SRC(code) == BPF_K - use 32-bit immediate as source operand

 * in eBPF, this means::

	BPF_SRC(code) == BPF_X - use 'src_reg' register as source operand
	BPF_SRC(code) == BPF_K - use 32-bit immediate as source operand

```
鈥︹€﹁€屽洓涓渶楂樻湁鏁堜綅锛圡SB锛夊瓨鍌ㄦ搷浣滅爜銆?

```

  BPF_ADD   0x00
  BPF_SUB   0x10
  BPF_MUL   0x20
  BPF_DIV   0x30
  BPF_OR    0x40
  BPF_AND   0x50
  BPF_LSH   0x60
  BPF_RSH   0x70
  BPF_NEG   0x80
  BPF_MOD   0x90
  BPF_XOR   0xa0
  BPF_MOV   0xb0  /* eBPF only: mov reg to reg */
  BPF_ARSH  0xc0  /* eBPF only: sign extending shift right */
  BPF_END   0xd0  /* eBPF only: endianness conversion */

```
```

  BPF_JA    0x00  /* BPF_JMP only */
  BPF_JEQ   0x10
  BPF_JGT   0x20
  BPF_JGE   0x30
  BPF_JSET  0x40
  BPF_JNE   0x50  /* eBPF only: jump != */
  BPF_JSGT  0x60  /* eBPF only: signed '>' */
  BPF_JSGE  0x70  /* eBPF only: signed '>=' */
  BPF_CALL  0x80  /* eBPF BPF_JMP only: function call */
  BPF_EXIT  0x90  /* eBPF BPF_JMP only: function return */
  BPF_JLT   0xa0  /* eBPF only: unsigned '<' */
  BPF_JLE   0xb0  /* eBPF only: unsigned '<=' */
  BPF_JSLT  0xc0  /* eBPF only: signed '<' */
  BPF_JSLE  0xd0  /* eBPF only: signed '<=' */

```
鍥犳 BPF_ADD | BPF_X | BPF_ALU 鍦ㄧ粡鍏?BPF 鍜?eBPF 涓兘琛ㄧず 32 浣嶅姞娉曪紝
鍗?A += X銆?
鍦?eBPF 涓畠琛ㄧず dst_reg = (u32) dst_reg + (u32) src_reg锛涚被浼煎湴锛?
BPF_XOR | BPF_K | BPF_ALU 鍦ㄧ粡鍏?BPF 涓〃绀?A ^= imm32锛屽湪 eBPF 涓浉搴斿湴
琛ㄧず src_reg = (u32) src_reg ^ (u32) imm32銆?

缁忓吀 BPF 浣跨敤 BPF_MISC 绫绘潵琛ㄧず A = X 鍜?X = A 鐨勪紶閫併€?
eBPF 鍒欐敼鐢?BPF_MOV | BPF_X | BPF_ALU 浠ｇ爜銆傜敱浜?eBPF 涓病鏈?
BPF_MISC 鎿嶄綔锛岀被鍒?7 琚敤浣?BPF_ALU64锛岃〃绀?
涓?BPF_ALU 瀹屽叏鐩稿悓鐨勬搷浣滐紝浣嗘搷浣滄暟涓?64 浣嶅
鑰岄潪 32 浣嶃€傚洜姝?BPF_ADD | BPF_X | BPF_ALU64 琛ㄧず 64 浣嶅姞娉曪紝鍗筹細
dst_reg = dst_reg + src_reg

缁忓吀 BPF 鑰楄垂鏁翠釜 BPF_RET 绫绘潵琛ㄧず鍗曚竴鐨?`ret`
鎿嶄綔銆傜粡鍏?BPF_RET | BPF_K 琛ㄧず灏?imm32 澶嶅埗鍒拌繑鍥炲瘎瀛樺櫒
骞舵墽琛屽嚱鏁伴€€鍑恒€俥BPF 鐨勫缓妯′笌 CPU 鐩稿尮閰嶏紝鍥犳 BPF_JMP | BPF_EXIT
鍦?eBPF 涓粎琛ㄧず鍑芥暟閫€鍑恒€俥BPF 绋嬪簭闇€瑕佸厛灏嗚繑鍥炲€?
瀛樺叆瀵勫瓨鍣?R0锛屽啀鎵ц BPF_EXIT銆俥BPF 涓殑绫诲埆 6 琚敤浣?
BPF_JMP32锛岃〃绀轰笌 BPF_JMP 瀹屽叏鐩稿悓鐨勬搷浣滐紝浣嗘瘮杈冩搷浣滄暟
涓?32 浣嶅銆?

```

  +--------+--------+-------------------+
  | 3 bits | 2 bits |   3 bits          |
  |  mode  |  size  | instruction class |
  +--------+--------+-------------------+
  (MSB)                             (LSB)

```
澶у皬淇グ绗︽槸涓嬪垪涔嬩竴鈥︹€?


```

  BPF_W   0x00    /* word */
  BPF_H   0x08    /* half word */
  BPF_B   0x10    /* byte */
  BPF_DW  0x18    /* eBPF only, double word */

```
```

 B  - 1 byte
 H  - 2 byte
 W  - 4 byte
 DW - 8 byte (eBPF only)

```
```

  BPF_IMM     0x00  /* used for 32-bit mov in classic BPF and 64-bit in eBPF */
  BPF_ABS     0x20
  BPF_IND     0x40
  BPF_MEM     0x60
  BPF_LEN     0x80  /* classic BPF only, reserved in eBPF */
  BPF_MSH     0xa0  /* classic BPF only, reserved in eBPF */
  BPF_ATOMIC  0xc0  /* eBPF only, atomic operations */

```