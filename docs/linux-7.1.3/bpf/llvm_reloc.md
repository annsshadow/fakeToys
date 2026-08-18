
## BPF LLVM 閲嶅畾浣?

鏈枃妗ｆ弿杩?LLVM BPF 鍚庣鐨勯噸瀹氫綅绫诲瀷銆?
## 閲嶅畾浣嶈褰?

LLVM BPF 鍚庣浣跨敤浠ヤ笅 16 瀛楄妭璁板綍姣忎釜閲嶅畾浣?```

  typedef struct
  {
    Elf64_Addr    r_offset;  // Offset from the beginning of section.
    Elf64_Xword   r_info;    // Relocation type and symbol index.
  } Elf64_Rel;

```
```

  int g1 __attribute__((section("sec")));
  int g2 __attribute__((section("sec")));
  static volatile int l1 __attribute__((section("sec")));
  static volatile int l2 __attribute__((section("sec")));
  int test() {
    return g1 + g2 + l1 + l2;
  }

```
浣跨敤 `clang --target=bpf -O2 -c test.c` 缂栬瘧锛屼互涓嬫槸
```

       0:       18 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 r1 = 0 ll
                0000000000000000:  R_BPF_64_64  g1
       2:       61 11 00 00 00 00 00 00 r1 = *(u32 *)(r1 + 0)
       3:       18 02 00 00 00 00 00 00 00 00 00 00 00 00 00 00 r2 = 0 ll
                0000000000000018:  R_BPF_64_64  g2
       5:       61 20 00 00 00 00 00 00 r0 = *(u32 *)(r2 + 0)
       6:       0f 10 00 00 00 00 00 00 r0 += r1
       7:       18 01 00 00 08 00 00 00 00 00 00 00 00 00 00 00 r1 = 8 ll
                0000000000000038:  R_BPF_64_64  sec
       9:       61 11 00 00 00 00 00 00 r1 = *(u32 *)(r1 + 0)
      10:       0f 10 00 00 00 00 00 00 r0 += r1
      11:       18 01 00 00 0c 00 00 00 00 00 00 00 00 00 00 00 r1 = 12 ll
                0000000000000058:  R_BPF_64_64  sec
      13:       61 11 00 00 00 00 00 00 r1 = *(u32 *)(r1 + 0)
      14:       0f 10 00 00 00 00 00 00 r0 += r1
      15:       95 00 00 00 00 00 00 00 exit

```
涓婇潰鏈夊洓涓?`LD_imm64` 鎸囦护鐨勫洓涓噸瀹氫綅銆備互涓?`llvm-readelf -r test.o` 鏄剧ず浜嗚繖鍥涗釜鐨?浜岃繘鍒跺€?```

  Relocation section '.rel.text' at offset 0x190 contains 4 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name
  0000000000000000  0000000600000001 R_BPF_64_64            0000000000000000 g1
  0000000000000018  0000000700000001 R_BPF_64_64            0000000000000004 g2
  0000000000000038  0000000400000001 R_BPF_64_64            0000000000000000 sec
  0000000000000058  0000000400000001 R_BPF_64_64            0000000000000000 sec

```
姣忎釜閲嶅畾浣嶇敱 `Offset`锛? 瀛楄妭锛夊拰 `Info`锛? 瀛楄妭锛夎〃绀恒€備緥濡傦紝绗竴涓噸瀹氫綅瀵瑰簲浜庣涓€鏉?鎸囦护锛圤ffset 0x0锛夛紝鐩稿簲鐨?`Info` 鎸囩ず浜?`R_BPF_64_64`锛堢被鍨?1锛夌殑閲嶅畾浣嶇被鍨嬩互鍙婄鍙?琛ㄤ腑鐨勬潯鐩紙鏉＄洰 6锛夈€?```

  Symbol table '.symtab' contains 8 entries:
     Num:    Value          Size Type    Bind   Vis       Ndx Name
       0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT   UND
       1: 0000000000000000     0 FILE    LOCAL  DEFAULT   ABS test.c
       2: 0000000000000008     4 OBJECT  LOCAL  DEFAULT     4 l1
       3: 000000000000000c     4 OBJECT  LOCAL  DEFAULT     4 l2
       4: 0000000000000000     0 SECTION LOCAL  DEFAULT     4 sec
       5: 0000000000000000   128 FUNC    GLOBAL DEFAULT     2 test
       6: 0000000000000000     4 OBJECT  GLOBAL DEFAULT     4 g1
       7: 0000000000000004     4 OBJECT  GLOBAL DEFAULT     4 g2

```
绗?6 涓潯鐩槸鍊间负 0 鐨勫叏灞€鍙橀噺 `g1`銆?
绫讳技鍦帮紝绗簩涓噸瀹氫綅浣嶄簬 `.text` 鍋忕Щ `0x18`锛屾寚浠?3锛岀被鍨嬩负 `R_BPF_64_64`锛屽苟寮曠敤绗﹀彿
琛ㄤ腑鐨勬潯鐩?7銆傜浜屼釜閲嶅畾浣嶈В鏋愪负鍏ㄥ眬鍙橀噺 `g2`锛屽叾绗﹀彿鍊间负 4銆傝绗﹀彿鍊艰〃绀哄瓨鍌ㄥ叏灞€鍙橀噺
`g2` 鍒濆鍊肩殑 `.data` 鑺傝捣濮嬪鐨勫亸绉汇€?
绗笁鍜岀鍥涗釜閲嶅畾浣嶅紩鐢ㄩ潤鎬佸彉閲?`l1` 鍜?`l2`銆備粠涓婇潰鐨?`.rel.text` 鑺傜湅锛屼笉娓呮瀹冧滑鐪熸
寮曠敤鍝簺绗﹀彿锛屽洜涓哄畠浠兘寮曠敤绗﹀彿琛ㄦ潯鐩?4锛屽嵆绗﹀彿 `sec`锛屽畠鍏锋湁 `STT_SECTION` 绫诲瀷骞朵唬琛?涓€涓妭銆傚洜姝ゅ浜庨潤鎬佸彉閲忔垨鍑芥暟锛岃妭鍋忕Щ琚啓鍏ュ師濮?insn 缂撳啿鍖猴紝杩欑О涓?`A`锛坅ddend锛夈€?鏌ョ湅涓婇潰鐨?insn `7` 鍜?`11`锛屽畠浠叿鏈夎妭鍋忕Щ `8` 鍜?`12`銆備粠绗﹀彿琛ㄦ垜浠彲浠ユ壘鍒板畠浠搴斾簬
`l1` 鍜?`l2` 鐨勬潯鐩?`2` 鍜?`3`銆?
涓€鑸潵璇达紝瀵逛簬鍏ㄥ眬鍙橀噺鍜屽嚱鏁帮紝`A` 涓?0锛涘浜庨潤鎬佸彉閲?鍑芥暟锛宍A` 鏄妭鍋忕Щ鎴栧熀浜庤妭鍋忕Щ鐨?鏌愮璁＄畻缁撴灉銆傞潪鑺傚亸绉荤殑鎯呭喌鎸囩殑鏄嚱鏁拌皟鐢ㄣ€傛洿澶氱粏鑺傝涓嬫枃銆?
## 涓嶅悓鐨勯噸瀹氫綅绫诲瀷


鏀寔鍏閲嶅畾浣嶇被鍨嬨€備互涓嬫槸姒傝堪鍜?```

  Enum  ELF Reloc Type     Description      BitSize  Offset        Calculation
  0     R_BPF_NONE         None
  1     R_BPF_64_64        ld_imm64 insn    32       r_offset + 4  S + A
  2     R_BPF_64_ABS64     normal data      64       r_offset      S + A
  3     R_BPF_64_ABS32     normal data      32       r_offset      S + A
  4     R_BPF_64_NODYLD32  .BTF[.ext] data  32       r_offset      S + A
  10    R_BPF_64_32        call insn        32       r_offset + 4  (S + A) / 8 - 1

```
渚嬪锛宍R_BPF_64_64` 閲嶅畾浣嶇被鍨嬬敤浜?`ld_imm64` 鎸囦护銆傚疄闄呭緟閲嶅畾浣嶇殑鏁版嵁锛? 鎴栬妭鍋忕Щ锛夊瓨鍌?鍦?`r_offset + 4`锛岃/鍐欐暟鎹綅瀹戒负 32锛? 瀛楄妭锛夈€傝閲嶅畾浣嶅彲浠ョ敤绗﹀彿鍊煎姞涓婇殣寮忓姞鏁版潵瑙ｆ瀽銆?娉ㄦ剰 `BitSize` 涓?32锛岃繖鎰忓懗鐫€鑺傚亸绉诲繀椤诲皬浜庢垨绛変簬 `UINT32_MAX`锛岃繖鐢?LLVM BPF 鍚庣寮哄埗
鎵ц銆?
鍦ㄥ彟涓€绉嶆儏鍐典笅锛宍R_BPF_64_ABS64` 閲嶅畾浣嶇被鍨嬬敤浜庢櫘閫氱殑 64 浣嶆暟鎹€傚疄闄呭緟閲嶅畾浣嶇殑鏁版嵁瀛樺偍
鍦?`r_offset`锛岃/鍐欐暟鎹綅瀹戒负 64锛? 瀛楄妭锛夈€傝閲嶅畾浣嶅彲浠ョ敤绗﹀彿鍊煎姞涓婇殣寮忓姞鏁版潵瑙ｆ瀽銆?
`R_BPF_64_ABS32` 鍜?`R_BPF_64_NODYLD32` 绫诲瀷閮界敤浜?32 浣嶆暟鎹€備絾 `R_BPF_64_NODYLD32`
鐗规寚 `.BTF` 鍜?`.BTF.ext` 鑺備腑鐨勯噸瀹氫綅銆傚浜庡儚 bcc 杩欐牱娑夊強 llvm `ExecutionEngine
RuntimeDyld` 鐨勬儏鍐碉紝`R_BPF_64_NODYLD32` 绫诲瀷鐨勯噸瀹氫綅涓嶅簲瑙ｆ瀽涓哄疄闄呯殑鍑芥暟/鍙橀噺鍦板潃銆傚惁鍒欙紝
`.BTF` 鍜?`.BTF.ext` 灏嗗彉寰楀 bcc 鍜屽唴鏍镐笉鍙敤銆?
绫诲瀷 `R_BPF_64_32` 鐢ㄤ簬 call 鎸囦护銆俢all 鐩爣鐨勮妭鍋忕Щ瀛樺偍鍦?`r_offset + 4`锛?2 浣嶏級锛屽苟
璁＄畻涓?`(S + A) / 8 - 1`銆?
## 绀轰緥


绫诲瀷 `R_BPF_64_64` 鍜?`R_BPF_64_32` 鐢ㄤ簬瑙ｆ瀽 `ld_imm64`
```

  __attribute__((noinline)) __attribute__((section("sec1")))
  int gfunc(int a, int b) {
    return a * b;
  }
  static __attribute__((noinline)) __attribute__((section("sec1")))
  int lfunc(int a, int b) {
    return a + b;
  }
  int global __attribute__((section("sec2")));
  int test(int a, int b) {
    return gfunc(a, b) +  lfunc(a, b) + global;
  }

```
浣跨敤 `clang --target=bpf -O2 -c test.c` 缂栬瘧锛屾垜浠皢寰楀埌
```

  Disassembly of section .text:

  0000000000000000 <test>:
         0:       bf 26 00 00 00 00 00 00 r6 = r2
         1:       bf 17 00 00 00 00 00 00 r7 = r1
         2:       85 10 00 00 ff ff ff ff call -1
                  0000000000000010:  R_BPF_64_32  gfunc
         3:       bf 08 00 00 00 00 00 00 r8 = r0
         4:       bf 71 00 00 00 00 00 00 r1 = r7
         5:       bf 62 00 00 00 00 00 00 r2 = r6
         6:       85 10 00 00 02 00 00 00 call 2
                  0000000000000030:  R_BPF_64_32  sec1
         7:       0f 80 00 00 00 00 00 00 r0 += r8
         8:       18 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 r1 = 0 ll
                  0000000000000040:  R_BPF_64_64  global
        10:       61 11 00 00 00 00 00 00 r1 = *(u32 *)(r1 + 0)
        11:       0f 10 00 00 00 00 00 00 r0 += r1
        12:       95 00 00 00 00 00 00 00 exit

  Disassembly of section sec1:

  0000000000000000 <gfunc>:
         0:       bf 20 00 00 00 00 00 00 r0 = r2
         1:       2f 10 00 00 00 00 00 00 r0 *= r1
         2:       95 00 00 00 00 00 00 00 exit

  0000000000000018 <lfunc>:
         3:       bf 20 00 00 00 00 00 00 r0 = r2
         4:       0f 10 00 00 00 00 00 00 r0 += r1
         5:       95 00 00 00 00 00 00 00 exit

```
绗竴涓噸瀹氫綅瀵瑰簲浜?`gfunc(a, b)`锛屽叾涓?`gfunc` 鐨勫€间负 0锛屽洜姝?`call` 鎸囦护鍋忕Щ涓?`(0 + 0)/8 - 1 = -1`銆傜浜屼釜閲嶅畾浣嶅搴斾簬 `lfunc(a, b)`锛屽叾涓?`lfunc` 鐨勮妭鍋忕Щ涓?`0x18`锛?鍥犳 `call` 鎸囦护鍋忕Щ涓?`(0 + 0x18)/8 - 1 = 2`銆傜涓変釜閲嶅畾浣嶅搴斾簬 `global` 鐨?ld_imm64锛?鍏惰妭鍋忕Щ涓?`0`銆?```

  int global() { return 0; }
  struct t { void *g; } gbl = { global };

```
浣跨敤 `clang --target=bpf -O2 -g -c test.c` 缂栬瘧锛屾垜浠皢閫氳繃鍛戒护鍦?`.data` 鑺備腑鐪嬪埌濡備笅
閲嶅畾浣?```

  Relocation section '.rel.data' at offset 0x458 contains 1 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name
  0000000000000000  0000000700000002 R_BPF_64_ABS64         0000000000000000 global

```
璇ラ噸瀹氫綅琛ㄧず `.data` 鑺傜殑鍓?8 瀛楄妭搴斿～鍏呬负 `global` 鍙橀噺鐨勫湴鍧€銆?
閫氳繃 `llvm-readelf` 杈撳嚭锛屾垜浠彲浠ョ湅鍒?dwarf 鑺傛湁涓€鍫?```

  Relocation section '.rel.debug_info' at offset 0x468 contains 13 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name
  0000000000000006  0000000300000003 R_BPF_64_ABS32         0000000000000000 .debug_abbrev
  000000000000000c  0000000400000003 R_BPF_64_ABS32         0000000000000000 .debug_str
  0000000000000012  0000000400000003 R_BPF_64_ABS32         0000000000000000 .debug_str
  0000000000000016  0000000600000003 R_BPF_64_ABS32         0000000000000000 .debug_line
  000000000000001a  0000000400000003 R_BPF_64_ABS32         0000000000000000 .debug_str
  000000000000001e  0000000200000002 R_BPF_64_ABS64         0000000000000000 .text
  000000000000002b  0000000400000003 R_BPF_64_ABS32         0000000000000000 .debug_str
  0000000000000037  0000000800000002 R_BPF_64_ABS64         0000000000000000 gbl
  0000000000000040  0000000400000003 R_BPF_64_ABS32         0000000000000000 .debug_str
  ......

```
```

  Relocation section '.rel.BTF' at offset 0x538 contains 1 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name
  0000000000000084  0000000800000004 R_BPF_64_NODYLD32      0000000000000000 gbl

  Relocation section '.rel.BTF.ext' at offset 0x548 contains 2 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name
  000000000000002c  0000000200000004 R_BPF_64_NODYLD32      0000000000000000 .text
  0000000000000040  0000000200000004 R_BPF_64_NODYLD32      0000000000000000 .text

```

## CO-RE 閲嶅畾浣?

浠庣洰鏍囨枃浠剁殑瑙掑害鏉ョ湅锛孋O-RE 鏈哄埗鏄綔涓轰竴缁?CO-RE 鐗瑰畾鐨勯噸瀹氫綅璁板綍瀹炵幇鐨勩€傝繖浜涢噸瀹氫綅璁板綍
涓?ELF 閲嶅畾浣嶆棤鍏筹紝骞剁紪鐮佸湪 .BTF.ext 鑺備腑銆傛湁鍏?.BTF.ext 缁撴瀯鐨勬洿澶氫俊鎭紝璇峰弬瑙?Documentation/bpf/btf.rst <BTF_Ext_Section>銆?
CO-RE 閲嶅畾浣嶅簲鐢ㄤ簬 BPF 鎸囦护锛屼互鍦ㄥ姞杞芥椂鐢ㄤ笌鐩爣鍐呮牳鐩稿叧鐨勪俊鎭洿鏂版寚浠ょ殑绔嬪嵆鏁版垨鍋忕Щ瀛楁銆?
瑕佹墦琛ヤ竵鐨勫瓧娈垫牴鎹寚浠ょ被閫夋嫨锛?
- 瀵逛簬 BPF_ALU銆丅PF_ALU64銆丅PF_LD锛宍immediate` 瀛楁琚ˉ涓侊紱
- 瀵逛簬 BPF_LDX銆丅PF_STX銆丅PF_ST锛宍offset` 瀛楁琚ˉ涓侊紱
- BPF_JMP銆丅PF_JMP32 鎸囦护**涓嶅簲**琚ˉ涓併€?
## 閲嶅畾浣嶇绫?

鏈夊嚑绉?CO-RE 閲嶅畾浣嶏紝鍙垎涓轰笁缁勶細

- 鍩轰簬瀛楁 - 鐢ㄤ笌瀛楁鐩稿叧鐨勪俊鎭ˉ涓佹寚浠わ紝渚嬪灏?BPF_LDX 鎸囦护鐨?offset 瀛楁鏇存敼涓哄弽鏄?  鐩爣鍐呮牳涓壒瀹氱粨鏋勪綋瀛楁鐨勫亸绉汇€?
- 鍩轰簬绫诲瀷 - 鐢ㄤ笌绫诲瀷鐩稿叧鐨勪俊鎭ˉ涓佹寚浠わ紝渚嬪灏?BPF_ALU move 鎸囦护鐨?immediate 瀛楁鏇存敼涓?  0 鎴?1锛屼互鍙嶆槧鐩爣鍐呮牳涓槸鍚﹀瓨鍦ㄧ壒瀹氱被鍨嬨€?
- 鍩轰簬鏋氫妇 - 鐢ㄤ笌鏋氫妇鐩稿叧鐨勪俊鎭ˉ涓佹寚浠わ紝渚嬪灏?BPF_LD_IMM64 鎸囦护鐨?immediate 瀛楁鏇存敼涓?  鍙嶆槧鐩爣鍐呮牳涓壒瀹氭灇涓惧瓧闈㈤噺鐨勫€笺€?
閲嶅畾浣嶇绫荤殑瀹屾暣鍒楄〃鐢变互涓?enum 琛ㄧず锛?
enum bpf_core_relo_kind {
	BPF_CORE_FIELD_BYTE_OFFSET = 0,  /** field byte offset **/
	BPF_CORE_FIELD_BYTE_SIZE   = 1,  /** field size in bytes **/
	BPF_CORE_FIELD_EXISTS      = 2,  /** field existence in target kernel **/
	BPF_CORE_FIELD_SIGNED      = 3,  /** field signedness (0 - unsigned, 1 - signed) **/
	BPF_CORE_FIELD_LSHIFT_U64  = 4,  /** bitfield-specific left bitshift **/
	BPF_CORE_FIELD_RSHIFT_U64  = 5,  /** bitfield-specific right bitshift **/
	BPF_CORE_TYPE_ID_LOCAL     = 6,  /** type ID in local BPF object **/
	BPF_CORE_TYPE_ID_TARGET    = 7,  /** type ID in target kernel **/
	BPF_CORE_TYPE_EXISTS       = 8,  /** type existence in target kernel **/
	BPF_CORE_TYPE_SIZE         = 9,  /** type size in bytes **/
	BPF_CORE_ENUMVAL_EXISTS    = 10, /** enum value existence in target kernel **/
	BPF_CORE_ENUMVAL_VALUE     = 11, /** enum value integer value **/
	BPF_CORE_TYPE_MATCHES      = 12, /** type match in target kernel **/
 };

娉ㄦ剰锛?
- `BPF_CORE_FIELD_LSHIFT_U64` 鍜?`BPF_CORE_FIELD_RSHIFT_U64` 搴旇鐢ㄤ簬浣跨敤浠ヤ笅绠楁硶璇诲彇
  浣嶅煙鍊硷細

  .. code-block:: c

     // To read bitfield `f` from `struct s`
     is_signed = relo(s->f, BPF_CORE_FIELD_SIGNED)
     off = relo(s->f, BPF_CORE_FIELD_BYTE_OFFSET)
     sz  = relo(s->f, BPF_CORE_FIELD_BYTE_SIZE)
     l   = relo(s->f, BPF_CORE_FIELD_LSHIFT_U64)
     r   = relo(s->f, BPF_CORE_FIELD_RSHIFT_U64)
     // define `v` as signed or unsigned integer of size `sz`
     v = **({s|u}<sz> **)((void *)s + off)
     v <<= l
     v >>= r

- `BPF_CORE_TYPE_MATCHES` 鏌ヨ鍖归厤鍏崇郴锛屽畾涔夊涓嬶細

  - 瀵逛簬鏁存暟锛氱被鍨嬪拰绗﹀彿閮藉尮閰嶅垯绫诲瀷鍖归厤锛?  - 瀵逛簬鏁扮粍鍜屾寚閽堬細鐩爣绫诲瀷琚€掑綊鍖归厤锛?  - 瀵逛簬缁撴瀯浣撳拰鑱斿悎浣擄細

    - 灞€閮ㄦ垚鍛橀渶瑕佷互鐩稿悓鍚嶇О瀛樺湪浜庣洰鏍囦腑锛?
    - 瀵逛簬姣忎釜鎴愬憳锛屾垜浠€掑綊妫€鏌ュ尮閰嶏紝闄ら潪瀹冨凡缁忓湪鎸囬拡涔嬪悗锛屽湪杩欑鎯呭喌涓嬫垜浠彧妫€鏌ュ尮閰?      鐨勫悕绉板拰鍏煎鐨?kind锛?
  - 瀵逛簬鏋氫妇锛?
    - 灞€閮ㄥ彉浣撳繀椤绘寜绗﹀彿鍚嶇О锛堣€岄潪鏁板€硷級鍦ㄧ洰鏍囦腑鏈夊尮閰嶏紱

    - 澶у皬蹇呴』鍖归厤锛堜絾 enum 鍙互鍖归厤 enum64锛屽弽涔嬩害鐒讹級锛?
  - 瀵逛簬鍑芥暟鎸囬拡锛?
    - 灞€閮ㄧ被鍨嬩腑鍙傛暟鐨勬暟閲忓拰浣嶇疆蹇呴』鍖归厤鐩爣锛?    - 瀵逛簬姣忎釜鍙傛暟鍜岃繑鍥炲€硷紝鎴戜滑閫掑綊妫€鏌ュ尮閰嶃€?
## CO-RE 閲嶅畾浣嶈褰?

閲嶅畾浣嶈褰曠紪鐮佷负浠ヤ笅缁撴瀯锛?
struct bpf_core_relo {
	__u32 insn_off;
	__u32 type_id;
	__u32 access_str_off;
	enum bpf_core_relo_kind kind;
};

- `insn_off` - 涓庢閲嶅畾浣嶅叧鑱旂殑浠ｇ爜鑺傚唴鐨勬寚浠ゅ亸绉伙紙浠ュ瓧鑺備负鍗曚綅锛夛紱

- `type_id` - 鍙噸瀹氫綅绫诲瀷鎴栧瓧娈电殑"鏍?锛堝寘鍚級瀹炰綋鐨?BTF 绫诲瀷 ID锛?
- `access_str_off` - 瀵瑰簲 .BTF 瀛楃涓茶妭鍐呯殑鍋忕Щ銆傚瓧绗︿覆鐨勮В閲婂彇鍐充簬鍏蜂綋鐨勯噸瀹氫綅绉嶇被锛?
  - 瀵逛簬鍩轰簬瀛楁鐨勯噸瀹氫綅锛屽瓧绗︿覆浣跨敤瀛楁鍜屾暟缁勭储寮曠殑搴忓垪锛堜互鍐掑彿锛?锛夊垎闅旓級鏉ョ紪鐮佽璁块棶
    鐨勫瓧娈点€傚畠鍦ㄦ蹇典笂闈炲父鎺ヨ繎 LLVM 鐨?`getelementptr <GEP_>`_ 鎸囦护鐢ㄤ簬鏍囪瘑瀛楁鍋忕Щ鐨勫弬鏁般€?    渚嬪锛岃€冭檻浠ヤ笅 C 浠ｇ爜锛?
    .. code-block:: c

       struct sample {
           int a;
           int b;
           struct { int c[^10^]; };
       } __attribute__((preserve_access_index));
       struct sample *s;

    - 瀵?`s[^0^].a` 鐨勮闂細琚紪鐮佷负 `0:0`锛?
      - `0`锛歚s` 鐨勭涓€涓厓绱狅紙濡傚悓 `s` 鏄竴涓暟缁勶級锛?      - `0`锛歚struct sample` 涓瓧娈?`a` 鐨勭储寮曘€?
    - 瀵?`s->a` 鐨勮闂篃浼氳缂栫爜涓?`0:0`銆?    - 瀵?`s->b` 鐨勮闂細琚紪鐮佷负 `0:1`锛?
      - `0`锛歚s` 鐨勭涓€涓厓绱狅紱
      - `1`锛歚struct sample` 涓瓧娈?`b` 鐨勭储寮曘€?
    - 瀵?`s[^1^].c[^5^]` 鐨勮闂細琚紪鐮佷负 `1:2:0:5`锛?
      - `1`锛歚s` 鐨勭浜屼釜鍏冪礌锛?      - `2`锛歚struct sample` 涓尶鍚嶇粨鏋勪綋瀛楁鐨勭储寮曪紱
      - `0`锛氬尶鍚嶇粨鏋勪綋涓瓧娈?`c` 鐨勭储寮曪紱
      - `5`锛氳闂暟缁勫厓绱?#5銆?
  - 瀵逛簬鍩轰簬绫诲瀷鐨勯噸瀹氫綅锛屽瓧绗︿覆搴斾负 "0"锛?
  - 瀵逛簬鍩轰簬鏋氫妇鍊肩殑閲嶅畾浣嶏紝瀛楃涓插寘鍚叾鏋氫妇绫诲瀷鍐呮灇涓惧€肩殑绱㈠紩锛?
- `kind` - `enum bpf_core_relo_kind` 涔嬩竴銆?

## CO-RE 閲嶅畾浣嶇ず渚?

瀵逛簬浠ヤ笅 C 浠ｇ爜锛?
struct foo {
   int a;
   int b;
   unsigned c:15;
 } __attribute__((preserve_access_index));

 enum bar { U, V };

浣跨敤浠ヤ笅 BTF 瀹氫箟锛?
...
[^2^] STRUCT 'foo' size=8 vlen=2
        'a' type_id=3 bits_offset=0
        'b' type_id=3 bits_offset=32
        'c' type_id=4 bits_offset=64 bitfield_size=15
[^3^] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED
[^4^] INT 'unsigned int' size=4 bits_offset=0 nr_bits=32 encoding=(none)
...
[^16^] ENUM 'bar' encoding=UNSIGNED size=4 vlen=2
        'U' val=0
        'V' val=1

褰撲娇鐢?`__attribute__((preserve_access_index))` 鏃讹紝瀛楁鍋忕Щ閲嶅畾浣嶄細鑷姩鐢熸垚锛屼緥濡傦細

  void alpha(struct foo **s, volatile unsigned long **g) {
    *g = s->a;
    s->a = 1;
  }

  00 <alpha>:
    0:  r3 = **(s32 **)(r1 + 0x0)
           00:  CO-RE <byte_off> [^2^] struct foo::a (0:0)
    1:  **(u64 **)(r2 + 0x0) = r3
    2:  **(u32 **)(r1 + 0x0) = 0x1
           10:  CO-RE <byte_off> [^2^] struct foo::a (0:0)
    3:  exit


鎵€鏈夐噸瀹氫綅绉嶇被閮藉彲浠ラ€氳繃鍐呯疆鍑芥暟璇锋眰銆備緥濡傚熀浜庡瓧娈电殑閲嶅畾浣嶏細

  void bravo(struct foo **s, volatile unsigned long **g) {
    **g = __builtin_preserve_field_info(s->b, 0 /** field byte offset */);
    **g = __builtin_preserve_field_info(s->b, 1 /** field byte size */);
    **g = __builtin_preserve_field_info(s->b, 2 /** field existence */);
    **g = __builtin_preserve_field_info(s->b, 3 /** field signedness */);
    **g = __builtin_preserve_field_info(s->c, 4 /** bitfield left shift */);
    **g = __builtin_preserve_field_info(s->c, 5 /** bitfield right shift */);
  }

  20 <bravo>:
     4:     r1 = 0x4
            20:  CO-RE <byte_off> [^2^] struct foo::b (0:1)
     5:     **(u64 **)(r2 + 0x0) = r1
     6:     r1 = 0x4
            30:  CO-RE <byte_sz> [^2^] struct foo::b (0:1)
     7:     **(u64 **)(r2 + 0x0) = r1
     8:     r1 = 0x1
            40:  CO-RE <field_exists> [^2^] struct foo::b (0:1)
     9:     **(u64 **)(r2 + 0x0) = r1
    10:     r1 = 0x1
            50:  CO-RE <signed> [^2^] struct foo::b (0:1)
    11:     **(u64 **)(r2 + 0x0) = r1
    12:     r1 = 0x31
            60:  CO-RE <lshift_u64> [^2^] struct foo::c (0:2)
    13:     **(u64 **)(r2 + 0x0) = r1
    14:     r1 = 0x31
            70:  CO-RE <rshift_u64> [^2^] struct foo::c (0:2)
    15:     **(u64 **)(r2 + 0x0) = r1
    16:     exit


鍩轰簬绫诲瀷鐨勯噸瀹氫綅锛?
  void charlie(struct foo **s, volatile unsigned long **g) {
    **g = __builtin_preserve_type_info(**s, 0 /** type existence **/);
    **g = __builtin_preserve_type_info(**s, 1 /** type size **/);
    **g = __builtin_preserve_type_info(**s, 2 /** type matches **/);
    **g = __builtin_btf_type_id(**s, 0 /** type id in this object file **/);
    **g = __builtin_btf_type_id(**s, 1 /** type id in target kernel **/);
  }

  88 <charlie>:
    17:     r1 = 0x1
            88:  CO-RE <type_exists> [^2^] struct foo
    18:     **(u64 **)(r2 + 0x0) = r1
    19:     r1 = 0xc
            98:  CO-RE <type_size> [^2^] struct foo
    20:     **(u64 **)(r2 + 0x0) = r1
    21:     r1 = 0x1
            a8:  CO-RE <type_matches> [^2^] struct foo
    22:     **(u64 **)(r2 + 0x0) = r1
    23:     r1 = 0x2 ll
            b8:  CO-RE <local_type_id> [^2^] struct foo
    25:     **(u64 **)(r2 + 0x0) = r1
    26:     r1 = 0x2 ll
            d0:  CO-RE <target_type_id> [^2^] struct foo
    28:     **(u64 **)(r2 + 0x0) = r1
    29:     exit

鍩轰簬鏋氫妇鐨勯噸瀹氫綅锛?
  void delta(struct foo **s, volatile unsigned long **g) {
    **g = __builtin_preserve_enum_value(**(enum bar **)U, 0 /** enum literal existence */);
    **g = __builtin_preserve_enum_value(**(enum bar **)V, 1 /** enum literal value */);
  }

  f0 <delta>:
    30:     r1 = 0x1 ll
            f0:  CO-RE <enumval_exists> [^16^] enum bar::U = 0
    32:     **(u64 **)(r2 + 0x0) = r1
    33:     r1 = 0x1 ll
            108:  CO-RE <enumval_value> [^16^] enum bar::V = 1
    35:     **(u64 **)(r2 + 0x0) = r1
    36:     exit
