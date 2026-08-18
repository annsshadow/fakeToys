
## UAPI 妫€鏌ュ櫒锛圲API Checker锛?


UAPI 妫€鏌ュ櫒锛坄scripts/check-uapi.sh`锛夋槸涓€涓?shell 鑴氭湰锛岀敤浜庡湪 git 鏍戜腑妫€鏌?UAPI 澶存枃浠跺鐢ㄦ埛绌洪棿鍚戝悗鍏煎鎬с€?

## 閫夐」锛圤ptions锛?


鏈妭灏嗘弿杩板彲浠ョ敤鏉ヨ繍琛?`check-uapi.sh` 鐨勯€夐」銆?

```

    check-uapi.sh [-b BASE_REF] [-p PAST_REF] [-j N] [-l ERROR_LOG] [-i] [-q] [-v]

```
```

    -b BASE_REF    Base git reference to use for comparison. If unspecified or empty,
                   will use any dirty changes in tree to UAPI files. If there are no
                   dirty changes, HEAD will be used.
    -p PAST_REF    Compare BASE_REF to PAST_REF (e.g. -p v6.1). If unspecified or empty,
                   will use BASE_REF^1. Must be an ancestor of BASE_REF. Only headers
                   that exist on PAST_REF will be checked for compatibility.
    -j JOBS        Number of checks to run in parallel (default: number of CPU cores).
    -l ERROR_LOG   Write error log to file (default: no error log is generated).
    -i             Ignore ambiguous changes that may or may not break UAPI compatibility.
    -q             Quiet operation.
    -v             Verbose operation (print more information about each header being checked).

```
```

    ABIDIFF  Custom path to abidiff binary
    CC       C compiler (default is "gcc")
    ARCH     Target architecture of C compiler (default is host arch)

```
```

    0) Success
    1) ABI difference detected
    2) Prerequisite not met

```
## 绀轰緥锛圗xamples锛?


### 鍩烘湰鐢ㄦ硶锛圔asic Usage锛?


棣栧厛锛岃鎴戜滑灏濊瘯瀵逛竴涓?UAPI 澶存枃浠跺仛涓€涓槑鏄句細
```

    cat << 'EOF' | patch -l -p1
    --- a/include/uapi/linux/acct.h
    +++ b/include/uapi/linux/acct.h
    @@ -21,7 +21,9 @@
     #include <asm/param.h>
     #include <asm/byteorder.h>

    -/*
    +#define FOO
    +
    +/*
      *  comp_t is a 16-bit "floating" point number with a 3-bit base 8
      *  exponent and a 13-bit fraction.
      *  comp2_t is 24-bit with 5-bit base 2 exponent and 20 bit fraction
    diff --git a/include/uapi/linux/bpf.h b/include/uapi/linux/bpf.h
    EOF

```
```

    % ./scripts/check-uapi.sh
    Installing user-facing UAPI headers from dirty tree... OK
    Installing user-facing UAPI headers from HEAD... OK
    Checking changes to UAPI headers between HEAD and dirty tree...
    All 912 UAPI headers compatible with x86 appear to be backwards compatible

```
```

    cat << 'EOF' | patch -l -p1
    --- a/include/uapi/linux/bpf.h
    +++ b/include/uapi/linux/bpf.h
    @@ -74,7 +74,7 @@ struct bpf_insn {
            __u8    dst_reg:4;      /* dest register */
            __u8    src_reg:4;      /* source register */
            __s16   off;            /* signed offset */
    -       __s32   imm;            /* signed immediate constant */
    +       __u32   imm;            /* unsigned immediate constant */
     };

     /* Key of an a BPF_MAP_TYPE_LPM_TRIE entry */
    EOF

```
```

    % ./scripts/check-uapi.sh
    Installing user-facing UAPI headers from dirty tree... OK
    Installing user-facing UAPI headers from HEAD... OK
    Checking changes to UAPI headers between HEAD and dirty tree...
    ==== ABI differences detected in include/linux/bpf.h from HEAD -> dirty tree ====
        [C] 'struct bpf_insn' changed:
          type size hasn't changed
          1 data member change:
            type of '__s32 imm' changed:
              typedef name changed from __s32 to __u32 at int-ll64.h:27:1
              underlying type 'int' changed:
                type name changed from 'int' to 'unsigned int'
                type size hasn't changed
    ==================================================================================

    error - 1/912 UAPI headers compatible with x86 appear _not_ to be backwards compatible

```
鍦ㄨ繖绉嶆儏鍐典笅锛岃剼鏈姤鍛婅繖涓被鍨嬪彉鏇达紝鏄洜涓哄畠鍙兘浼氱牬鍧忎紶鍏ヨ礋鏁板€肩殑鐢ㄦ埛绌洪棿绋嬪簭銆傜幇鍦紝鍋囪浣犵煡閬撴病鏈変换浣曠敤鎴风┖闂寸▼搴忓彲鑳戒細鐢ㄥ埌 `imm` 涓殑璐熷€硷紝鍥犳鍦ㄩ偅閲屾敼鎴愭棤绗﹀彿绫诲瀷搴旇涓嶄細鏈変换浣曞奖鍝嶃€備綘鍙互缁欒剼鏈紶 `-i` 鏍囧織鏉ュ拷鐣ヨ繖浜涘彉鏇?
```

    % ./scripts/check-uapi.sh -i
    Installing user-facing UAPI headers from dirty tree... OK
    Installing user-facing UAPI headers from HEAD... OK
    Checking changes to UAPI headers between HEAD and dirty tree...
    All 912 UAPI headers compatible with x86 appear to be backwards compatible

```
```

    cat << 'EOF' | patch -l -p1
    --- a/include/uapi/linux/bpf.h
    +++ b/include/uapi/linux/bpf.h
    @@ -71,8 +71,8 @@ enum {

     struct bpf_insn {
            __u8    code;           /* opcode */
    -       __u8    dst_reg:4;      /* dest register */
            __u8    src_reg:4;      /* source register */
    +       __u8    dst_reg:4;      /* dest register */
            __s16   off;            /* signed offset */
            __s32   imm;            /* signed immediate constant */
     };
    EOF

```
鐢变簬鎴戜滑鏄湪閲嶆帓涓€涓凡鏈夌殑缁撴瀯浣撴垚鍛橈紝杩欓噷娌℃湁姝т箟锛?
```

    % ./scripts/check-uapi.sh -i
    Installing user-facing UAPI headers from dirty tree... OK
    Installing user-facing UAPI headers from HEAD... OK
    Checking changes to UAPI headers between HEAD and dirty tree...
    ==== ABI differences detected in include/linux/bpf.h from HEAD -> dirty tree ====
        [C] 'struct bpf_insn' changed:
          type size hasn't changed
          2 data member changes:
            '__u8 dst_reg' offset changed from 8 to 12 (in bits) (by +4 bits)
            '__u8 src_reg' offset changed from 12 to 8 (in bits) (by -4 bits)
    ==================================================================================

    error - 1/912 UAPI headers compatible with x86 appear _not_ to be backwards compatible

```
```

    % git commit -m 'Breaking UAPI change' include/uapi/linux/bpf.h
    [detached HEAD f758e574663a] Breaking UAPI change
     1 file changed, 1 insertion(+), 1 deletion(-)
    % git commit -m 'Innocuous UAPI change' include/uapi/linux/acct.h
    [detached HEAD 2e87df769081] Innocuous UAPI change
     1 file changed, 3 insertions(+), 1 deletion(-)

```
```

    % ./scripts/check-uapi.sh
    Installing user-facing UAPI headers from HEAD... OK
    Installing user-facing UAPI headers from HEAD^1... OK
    Checking changes to UAPI headers between HEAD^1 and HEAD...
    All 912 UAPI headers compatible with x86 appear to be backwards compatible

```
瀹冩病鎶撳埌浠讳綍鐮村潖鎬у彉鏇达紝鍥犱负榛樿鎯呭喌涓嬪畠鍙瘮杈?`HEAD` 涓?`HEAD^1`銆傜牬鍧忔€х殑鍙樻洿鎻愪氦鍦?`HEAD~2`銆傚鏋滄垜浠笇鏈涙悳绱㈣寖鍥村洖婧緱鏇磋繙锛屽氨寰楃敤 `-p` 閫夐」浼犲叆涓€涓笉鍚岀殑杩囧幓寮曠敤銆傚湪杩欑鎯呭喌涓嬶紝璁╂垜浠粰鑴氭湰浼?`-p HEAD~2`锛岃繖鏍峰畠灏辨鏌?`HEAD~2` 鍒?`HEAD` 涔嬮棿鐨?UAPI 鍙樻洿
```

    % ./scripts/check-uapi.sh -p HEAD~2
    Installing user-facing UAPI headers from HEAD... OK
    Installing user-facing UAPI headers from HEAD~2... OK
    Checking changes to UAPI headers between HEAD~2 and HEAD...
    ==== ABI differences detected in include/linux/bpf.h from HEAD~2 -> HEAD ====
        [C] 'struct bpf_insn' changed:
          type size hasn't changed
          2 data member changes:
            '__u8 dst_reg' offset changed from 8 to 12 (in bits) (by +4 bits)
            '__u8 src_reg' offset changed from 12 to 8 (in bits) (by -4 bits)
    ==============================================================================

    error - 1/912 UAPI headers compatible with x86 appear _not_ to be backwards compatible

```
鍙︿竴绉嶅仛娉曟槸锛屾垜浠篃鍙互鐢?`-b HEAD~` 杩愯銆傝繖浼氭妸鍩哄噯寮曠敤璁句负 `HEAD~`锛屼簬鏄剼鏈細姣旇緝瀹冧笌 `HEAD~^1`銆?

### 鏋舵瀯鐗瑰畾鐨勫ご鏂囦欢锛圓rchitecture-specific Headers锛?


```

    cat << 'EOF' | patch -l -p1
    --- a/arch/arm64/include/uapi/asm/sigcontext.h
    +++ b/arch/arm64/include/uapi/asm/sigcontext.h
    @@ -70,6 +70,7 @@ struct sigcontext {
     struct _aarch64_ctx {
            __u32 magic;
            __u32 size;
    +       __u32 new_var;
     };

     #define FPSIMD_MAGIC   0x46508001
    EOF

```
杩欐槸瀵逛竴涓?arm64 涓撳睘 UAPI 澶存枃浠剁殑鏀瑰姩銆傚湪鏈緥涓紝鎴戞浠庝竴鍙板甫 x86 缂栬瘧鍣ㄧ殑 x86 鏈哄櫒杩愯鑴氭湰锛屽洜姝ら粯璁ゆ儏鍐典笅
```

    % ./scripts/check-uapi.sh
    Installing user-facing UAPI headers from dirty tree... OK
    Installing user-facing UAPI headers from HEAD... OK
    No changes to UAPI headers were applied between HEAD and dirty tree

```
鐢?x86 缂栬瘧鍣紝鎴戜滑鏃犳硶妫€鏌?`arch/arm64` 涓殑澶存枃浠讹紝鎵€浠ヨ剼鏈牴鏈笉浼氬皾璇曘€?

濡傛灉鎴戜滑鎯虫鏌ヨ繖涓ご鏂囦欢锛屽氨寰楃敤 arm64 缂栬瘧鍣ㄥ苟
```

    % CC=aarch64-linux-gnu-gcc ARCH=arm64 ./scripts/check-uapi.sh
    Installing user-facing UAPI headers from dirty tree... OK
    Installing user-facing UAPI headers from HEAD... OK
    Checking changes to UAPI headers between HEAD and dirty tree...
    ==== ABI differences detected in include/asm/sigcontext.h from HEAD -> dirty tree ====
        [C] 'struct _aarch64_ctx' changed:
          type size changed from 64 to 96 (in bits)
          1 data member insertion:
            '__u32 new_var', at offset 64 (in bits) at sigcontext.h:73:1
        -- snip --
        [C] 'struct zt_context' changed:
          type size changed from 128 to 160 (in bits)
          2 data member changes (1 filtered):
            '__u16 nregs' offset changed from 64 to 96 (in bits) (by +32 bits)
            '__u16 __reserved[3]' offset changed from 80 to 112 (in bits) (by +32 bits)
    =======================================================================================

    error - 1/884 UAPI headers compatible with arm64 appear _not_ to be backwards compatible

```
鎴戜滑鍙互鐪嬪埌锛屽湪 `ARCH` 鍜?`CC` 涓鸿鏂囦欢姝ｇ‘璁剧疆鍚庯紝ABI 鍙樻洿琚纭湴鎶ュ憡浜嗐€傚彟澶栨敞鎰忚剼鏈墍妫€鏌ョ殑 UAPI 澶存枃浠舵€绘暟鍙戠敓浜嗗彉鍖栥€傝繖鏄洜涓轰负 arm64 骞冲彴瀹夎鐨勫ご閮ㄦ暟閲忎笌 x86 涓嶅悓銆?

### 璺ㄤ緷璧栫牬鍧忥紙Cross-Dependency Breakages锛?


```

    cat << 'EOF' | patch -l -p1
    --- a/include/uapi/linux/types.h
    +++ b/include/uapi/linux/types.h
    @@ -52,7 +52,7 @@ typedef __u32 __bitwise __wsum;
     #define __aligned_be64 __be64 __attribute__((aligned(8)))
     #define __aligned_le64 __le64 __attribute__((aligned(8)))

    -typedef unsigned __bitwise __poll_t;
    +typedef unsigned short __bitwise __poll_t;

     #endif /*  __ASSEMBLY__ */
     #endif /* _UAPI_LINUX_TYPES_H */
    EOF

```
杩欓噷锛屾垜浠湪鏀瑰姩 `types.h` 涓殑涓€涓?`typedef`銆傝繖骞朵笉鐮村潖 `types.h` 涓殑 UAPI锛屼絾鏍戜腑鐨勫叾浠?UAPI 鍙兘鍥?
```

    % ./scripts/check-uapi.sh
    Installing user-facing UAPI headers from dirty tree... OK
    Installing user-facing UAPI headers from HEAD... OK
    Checking changes to UAPI headers between HEAD and dirty tree...
    ==== ABI differences detected in include/linux/eventpoll.h from HEAD -> dirty tree ====
        [C] 'struct epoll_event' changed:
          type size changed from 96 to 80 (in bits)
          2 data member changes:
            type of '__poll_t events' changed:
              underlying type 'unsigned int' changed:
                type name changed from 'unsigned int' to 'unsigned short int'
                type size changed from 32 to 16 (in bits)
            '__u64 data' offset changed from 32 to 16 (in bits) (by -16 bits)
    ========================================================================================
    include/linux/eventpoll.h did not change between HEAD and dirty tree...
    It's possible a change to one of the headers it includes caused this error:
    #include <linux/fcntl.h>
    #include <linux/types.h>

```
娉ㄦ剰鑴氭湰娉ㄦ剰鍒板け璐ョ殑澶存枃浠跺苟鏈敼鍙橈紝鍥犳瀹冨亣瀹氭槸瀹冩墍鍖呭惈鐨勬煇涓ご鏂囦欢瀵艰嚧浜嗙牬鍧忋€傜‘瀹烇紝鎴戜滑鐪嬪埌 `eventpoll.h` 鐢ㄥ埌浜?`linux/types.h`銆?

### UAPI 澶存枃浠剁Щ闄わ紙UAPI Header Removals锛?


```

    cat << 'EOF' | patch -l -p1
    diff --git a/include/uapi/asm-generic/Kbuild b/include/uapi/asm-generic/Kbuild
    index ebb180aac74e..a9c88b0a8b3b 100644
    --- a/include/uapi/asm-generic/Kbuild
    +++ b/include/uapi/asm-generic/Kbuild
    @@ -31,6 +31,6 @@ mandatory-y += stat.h
     mandatory-y += statfs.h
     mandatory-y += swab.h
     mandatory-y += termbits.h
    -mandatory-y += termios.h
    +#mandatory-y += termios.h
     mandatory-y += types.h
     mandatory-y += unistd.h
    EOF

```
杩欎釜鑴氭湰鎶婁竴涓?UAPI 澶存枃浠朵粠瀹夎鍒楄〃涓Щ闄ゃ€傝鎴戜滑杩愯
```

    % ./scripts/check-uapi.sh
    Installing user-facing UAPI headers from dirty tree... OK
    Installing user-facing UAPI headers from HEAD... OK
    Checking changes to UAPI headers between HEAD and dirty tree...
    ==== UAPI header include/asm/termios.h was removed between HEAD and dirty tree ====

    error - 1/912 UAPI headers compatible with x86 appear _not_ to be backwards compatible

```
绉婚櫎涓€涓?UAPI 澶存枃浠惰瑙嗕负涓€绉嶇牬鍧忔€у彉鏇达紝鑴氭湰浼氬皢鍏舵爣璁颁负濡傛銆?

### 妫€鏌ュ巻鍙?UAPI 鍏煎鎬э紙Checking Historic UAPI Compatibility锛?


浣犲彲浠ョ敤 `-b` 鍜?`-p` 閫夐」鏉ユ鏌?git 鏍戜腑涓嶅悓鐨勭墖娈点€備緥濡傦紝瑕佹鏌ユ爣绛句箣闂存墍鏈夎鏇存敼鐨?UAPI 澶存枃浠?
```

    % ./scripts/check-uapi.sh -b v6.1 -p v6.0
    Installing user-facing UAPI headers from v6.1... OK
    Installing user-facing UAPI headers from v6.0... OK
    Checking changes to UAPI headers between v6.0 and v6.1...

    --- snip ---
    error - 37/907 UAPI headers compatible with x86 appear _not_ to be backwards compatible

```
娉ㄦ剰锛氬湪 v5.3 涔嬪墠锛岃剼鏈墍闇€鐨勪竴涓ご鏂囦欢涓嶅瓨鍦紝鍥犳鑴氭湰鏃犳硶妫€鏌ラ偅涔嬪墠鐨勫彉鏇淬€?

浣犱細娉ㄦ剰鍒拌剼鏈娴嬪埌浜嗚澶氫笉鍚戝悗鍏煎鐨?UAPI 鍙樻洿銆傞壌浜庡唴鏍?UAPI 鏈簲姘歌繙淇濇寔绋冲畾锛岃繖鏄竴涓护浜鸿瑙夌殑缁撴灉銆傝繖鎶婃垜浠甫鍒颁簡涓嬩竴鑺傦細娉ㄦ剰浜嬮」锛坈aveats锛夈€?

## 娉ㄦ剰浜嬮」锛圕aveats锛?


UAPI 妫€鏌ュ櫒瀵逛綔鑰呯殑鎰忓浘涓嶅仛浠讳綍鍋囪锛屽洜姝ゆ煇浜涚被鍨嬬殑鍙樻洿鍙兘浼氳鏍囪锛屽嵆渚垮畠浠槸鏈夋剰鐮村潖 UAPI 鐨勩€?

### 涓洪噸鏋勬垨寮冪敤鑰岀Щ闄わ紙Removals For Refactoring or Deprecation锛?


```

    % ./scripts/check-uapi.sh -b ba47652ba655
    Installing user-facing UAPI headers from ba47652ba655... OK
    Installing user-facing UAPI headers from ba47652ba655^1... OK
    Checking changes to UAPI headers between ba47652ba655^1 and ba47652ba655...
    ==== UAPI header include/linux/meye.h was removed between ba47652ba655^1 and ba47652ba655 ====

    error - 1/910 UAPI headers compatible with x86 appear _not_ to be backwards compatible

```
鑴氭湰鎬讳細鏍囪绉婚櫎锛堝嵆渚垮畠浠槸鏈夋剰鐨勶級銆?

### 缁撴瀯浣撴墿灞曪紙Struct Expansions锛?


鍙栧喅浜庣粨鏋勪綋鍦ㄥ唴鏍哥┖闂翠腑鐨勫鐞嗘柟寮忥紝涓€涓墿灞曠粨鏋勪綋鐨勫彉鏇村彲鑳芥槸闈炵牬鍧忔€х殑銆?

濡傛灉涓€涓粨鏋勪綋琚敤浣?ioctl 鐨勫弬鏁帮紝閭ｄ箞鍐呮牳椹卞姩蹇呴』鑳藉鐞嗕换鎰忓ぇ灏忕殑 ioctl 鍛戒护銆傞櫎姝や箣澶栵紝鍦ㄤ粠鐢ㄦ埛澶嶅埗鏁版嵁鏃朵綘闇€瑕佸皬蹇冦€備緥濡傝
```

    struct foo {
        __u64 a; /* added in version 1 */
    +   __u32 b; /* added in version 2 */
    +   __u32 c; /* added in version 2 */
    }

```
```

    [C] 'struct foo' changed:
      type size changed from 64 to 128 (in bits)
      2 data member insertions:
        '__u32 b', at offset 64 (in bits)
        '__u32 c', at offset 96 (in bits)

```
涓嶈繃锛岃繖娆″彉鏇存槸鏈夊彲鑳借瀹夊叏鍦板畬鎴愮殑銆?

濡傛灉涓€涓敤鎴风┖闂寸▼搴忔槸鐢ㄧ増鏈?1 鏋勫缓鐨勶紝瀹冧細璁や负 `sizeof(struct foo)` 鏄?8銆傝繖涓昂瀵镐細琚紪鐮佽繘鍙戝線鍐呮牳鐨?ioctl 鍊间腑銆傚鏋滃唴鏍告槸鐢ㄧ増鏈?2 鏋勫缓鐨勶紝瀹冧細璁や负 `sizeof(struct foo)` 鏄?16銆?

鍐呮牳鍙互鐢?`_IOC_SIZE` 瀹忔潵鑾峰彇鐢ㄦ埛浼犲叆鐨?ioctl 鐮佷腑缂栫爜鐨勫昂瀵革紝鐒跺悗
```

    int handle_ioctl(unsigned long cmd, unsigned long arg)
    {
        switch _IOC_NR(cmd) {
        0x01: {
            struct foo my_cmd;  /* size 16 in the kernel */

            ret = copy_struct_from_user(&my_cmd, arg, sizeof(struct foo), _IOC_SIZE(cmd));
            ...

```
`copy_struct_from_user` 浼氬湪鍐呮牳涓妸缁撴瀯浣撴竻闆讹紝鐒跺悗鍙鍒朵粠鐢ㄦ埛浼犲叆鐨勫瓧鑺傦紙浣挎柊鎴愬憳淇濇寔涓洪浂锛夈€傚鏋滅敤鎴蜂紶鍏ヤ簡鏇村ぇ鐨勭粨鏋勪綋锛屽浣欑殑鎴愬憳浼氳蹇界暐銆?

濡傛灉浣犵煡閬撳唴鏍镐唬鐮佷腑宸茬粡鑰冭檻浜嗚繖绉嶆儏鍐碉紝浣犲彲浠ョ粰鑴氭湰浼?`-i`锛岃繖鏍峰儚杩欐牱鐨勭粨鏋勪綋鎵╁睍灏变細琚拷鐣ャ€?

### Flex 鏁扮粍杩佺Щ锛團lex Array Migration锛?


铏界劧鑴氭湰浼氬鐞嗗悜宸叉湁 flex 鏁扮粍鐨勬墿灞曪紝浣嗗畠浠嶄細鏍囪浠?1 鍏冪礌鐨勪吉 flex 鏁扮粍鍒扮湡 flex 鏁扮粍鐨勫垵濮嬭縼绉?
```

    struct foo {
          __u32 x;
    -     __u32 flex[1]; /* fake flex */
    +     __u32 flex[];  /* real flex */
    };

```
```

    [C] 'struct foo' changed:
      type size changed from 64 to 32 (in bits)
      1 data member change:
        type of '__u32 flex[1]' changed:
          type name changed from '__u32[1]' to '__u32[]'
          array type size changed from 32 to 'unknown'
          array type subrange 1 changed length from 1 to 'unknown'

```
鐩墠锛屾病鏈夊姙娉曡繃婊よ繖绫诲彉鏇达紝鍥犳璇锋敞鎰忚繖绉嶅彲鑳界殑璇姤锛坒alse positive锛夈€?

### 鎬荤粨锛圫ummary锛?


铏界劧璁稿绫诲瀷鐨勮鎶ヤ細琚剼鏈繃婊ゆ帀锛屼粛鏈夊彲鑳藉嚭鐜拌剼鏈爣璁颁簡涓€涓苟鏈牬鍧?UAPI 鐨勫彉鏇寸殑鎯呭喌銆備篃鏈夊彲鑳戒竴涓?*纭疄**鐮村潖鐢ㄦ埛绌洪棿鐨勫彉鏇存湭琚鑴氭湰鏍囪銆傝櫧鐒惰剼鏈凡鍦ㄥぇ閲忓唴鏍稿巻鍙蹭笂杩愯杩囷紝浠嶅彲鑳藉瓨鍦ㄦ湭琚兜鐩栫殑杈圭晫鎯呭喌銆?

姝よ剼鏈殑鎰忓浘鏄綔涓虹淮鎶よ€呮垨鑷姩鍖栧伐鍏风殑涓€涓揩閫熸鏌ワ紝鑰岄潪琛ヤ竵鍏煎鎬х殑鏈€缁堟潈濞併€傛渶濂借浣忥細杩愮敤浣犵殑鏈€浣冲垽鏂紙鐞嗘兂鎯呭喌涓嬪啀鍔犱笂鐢ㄦ埛绌洪棿鐨勪竴涓崟鍏冩祴璇曪級鏉ョ‘淇濅綘鐨?UAPI 鍙樻洿鏄悜鍚庡吋瀹圭殑锛?
