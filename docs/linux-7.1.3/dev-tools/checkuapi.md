
## UAPI 检查器（UAPI Checker）


UAPI 检查器（`scripts/check-uapi.sh`）是一个 shell 脚本，用于在 git 树中检查 UAPI 头文件对用户空间向后兼容性。

## 选项（Options）


本节将描述可以用来运行 `check-uapi.sh` 的选项。

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
## 示例（Examples）


### 基本用法（Basic Usage）


首先，让我们尝试对一个 UAPI 头文件做一个明显会
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
在这种情况下，脚本报告这个类型变更，是因为它可能会破坏传入负数值的用户空间程序。现在，假设你知道没有任何用户空间程序可能会用到 `imm` 中的负值，因此在那里改成无符号类型应该不会有任何影响。你可以给脚本传 `-i` 标志来忽略这些变更
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
由于我们是在重排一个已有的结构体成员，这里没有歧义，
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
它没抓到任何破坏性变更，因为默认情况下它只比较 `HEAD` 与 `HEAD^1`。破坏性的变更提交在 `HEAD~2`。如果我们希望搜索范围回溯得更远，就得用 `-p` 选项传入一个不同的过去引用。在这种情况下，让我们给脚本传 `-p HEAD~2`，这样它就检查 `HEAD~2` 到 `HEAD` 之间的 UAPI 变更
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
另一种做法是，我们也可以用 `-b HEAD~` 运行。这会把基准引用设为 `HEAD~`，于是脚本会比较它与 `HEAD~^1`。

### 架构特定的头文件（Architecture-specific Headers）


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
这是对一个 arm64 专属 UAPI 头文件的改动。在本例中，我正从一台带 x86 编译器的 x86 机器运行脚本，因此默认情况下
```

    % ./scripts/check-uapi.sh
    Installing user-facing UAPI headers from dirty tree... OK
    Installing user-facing UAPI headers from HEAD... OK
    No changes to UAPI headers were applied between HEAD and dirty tree

```
用 x86 编译器，我们无法检查 `arch/arm64` 中的头文件，所以脚本根本不会尝试。

如果我们想检查这个头文件，就得用 arm64 编译器并
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
我们可以看到，在 `ARCH` 和 `CC` 为该文件正确设置后，ABI 变更被正确地报告了。另外注意脚本所检查的 UAPI 头文件总数发生了变化。这是因为为 arm64 平台安装的头部数量与 x86 不同。

### 跨依赖破坏（Cross-Dependency Breakages）


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
这里，我们在改动 `types.h` 中的一个 `typedef`。这并不破坏 `types.h` 中的 UAPI，但树中的其他 UAPI 可能因
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
注意脚本注意到失败的头文件并未改变，因此它假定是它所包含的某个头文件导致了破坏。确实，我们看到 `eventpoll.h` 用到了 `linux/types.h`。

### UAPI 头文件移除（UAPI Header Removals）


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
这个脚本把一个 UAPI 头文件从安装列表中移除。让我们运行
```

    % ./scripts/check-uapi.sh
    Installing user-facing UAPI headers from dirty tree... OK
    Installing user-facing UAPI headers from HEAD... OK
    Checking changes to UAPI headers between HEAD and dirty tree...
    ==== UAPI header include/asm/termios.h was removed between HEAD and dirty tree ====

    error - 1/912 UAPI headers compatible with x86 appear _not_ to be backwards compatible

```
移除一个 UAPI 头文件被视为一种破坏性变更，脚本会将其标记为如此。

### 检查历史 UAPI 兼容性（Checking Historic UAPI Compatibility）


你可以用 `-b` 和 `-p` 选项来检查 git 树中不同的片段。例如，要检查标签之间所有被更改的 UAPI 头文件
```

    % ./scripts/check-uapi.sh -b v6.1 -p v6.0
    Installing user-facing UAPI headers from v6.1... OK
    Installing user-facing UAPI headers from v6.0... OK
    Checking changes to UAPI headers between v6.0 and v6.1...

    --- snip ---
    error - 37/907 UAPI headers compatible with x86 appear _not_ to be backwards compatible

```
注意：在 v5.3 之前，脚本所需的一个头文件不存在，因此脚本无法检查那之前的变更。

你会注意到脚本检测到了许多不向后兼容的 UAPI 变更。鉴于内核 UAPI 本应永远保持稳定，这是一个令人警觉的结果。这把我们带到了下一节：注意事项（caveats）。

## 注意事项（Caveats）


UAPI 检查器对作者的意图不做任何假设，因此某些类型的变更可能会被标记，即便它们是有意破坏 UAPI 的。

### 为重构或弃用而移除（Removals For Refactoring or Deprecation）


```

    % ./scripts/check-uapi.sh -b ba47652ba655
    Installing user-facing UAPI headers from ba47652ba655... OK
    Installing user-facing UAPI headers from ba47652ba655^1... OK
    Checking changes to UAPI headers between ba47652ba655^1 and ba47652ba655...
    ==== UAPI header include/linux/meye.h was removed between ba47652ba655^1 and ba47652ba655 ====

    error - 1/910 UAPI headers compatible with x86 appear _not_ to be backwards compatible

```
脚本总会标记移除（即便它们是有意的）。

### 结构体扩展（Struct Expansions）


取决于结构体在内核空间中的处理方式，一个扩展结构体的变更可能是非破坏性的。

如果一个结构体被用作 ioctl 的参数，那么内核驱动必须能处理任意大小的 ioctl 命令。除此之外，在从用户复制数据时你需要小心。例如说
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
不过，这次变更是有可能被安全地完成的。

如果一个用户空间程序是用版本 1 构建的，它会认为 `sizeof(struct foo)` 是 8。这个尺寸会被编码进发往内核的 ioctl 值中。如果内核是用版本 2 构建的，它会认为 `sizeof(struct foo)` 是 16。

内核可以用 `_IOC_SIZE` 宏来获取用户传入的 ioctl 码中编码的尺寸，然后
```

    int handle_ioctl(unsigned long cmd, unsigned long arg)
    {
        switch _IOC_NR(cmd) {
        0x01: {
            struct foo my_cmd;  /* size 16 in the kernel */

            ret = copy_struct_from_user(&my_cmd, arg, sizeof(struct foo), _IOC_SIZE(cmd));
            ...

```
`copy_struct_from_user` 会在内核中把结构体清零，然后只复制从用户传入的字节（使新成员保持为零）。如果用户传入了更大的结构体，多余的成员会被忽略。

如果你知道内核代码中已经考虑了这种情况，你可以给脚本传 `-i`，这样像这样的结构体扩展就会被忽略。

### Flex 数组迁移（Flex Array Migration）


虽然脚本会处理向已有 flex 数组的扩展，但它仍会标记从 1 元素的伪 flex 数组到真 flex 数组的初始迁移
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
目前，没有办法过滤这类变更，因此请注意这种可能的误报（false positive）。

### 总结（Summary）


虽然许多类型的误报会被脚本过滤掉，仍有可能出现脚本标记了一个并未破坏 UAPI 的变更的情况。也有可能一个**确实**破坏用户空间的变更未被此脚本标记。虽然脚本已在大量内核历史上运行过，仍可能存在未被涵盖的边界情况。

此脚本的意图是作为维护者或自动化工具的一个快速检查，而非补丁兼容性的最终权威。最好记住：运用你的最佳判断（理想情况下再加上用户空间的一个单元测试）来确保你的 UAPI 变更是向后兼容的！
