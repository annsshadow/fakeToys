
## Checkpatch


Checkpatch (scripts/checkpatch.pl) 是一个 perl 脚本，用于检查补丁中的琐碎风格违规，并可选择性地修正它们。
Checkpatch 也可以针对文件上下文运行，并且
无需内核源码树即可运行。

Checkpatch 并非永远正确。你的判断优先于 checkpatch 给出的信息。
如果你的代码带着这些违规看起来更好，那么很可能最好
保持原样。


## 选项


本节将描述 checkpatch 运行时的各个选项。

```

  ./scripts/checkpatch.pl [OPTION]... [FILE]...

```
可用选项：

 - -q,  --quiet

   启用安静模式。

 - -v,  --verbose
   启用详细模式。会输出额外的详细测试说明，
   以提供该特定信息为何被显示的原因。

 - --no-tree

   在没有内核源码树的情况下运行 checkpatch。

 - --no-signoff

   禁用 'Signed-off-by' 行检查。sign-off 是补丁说明末尾的一行简单文本，
   用于证明你编写了它，或
   你拥有将其作为开源补丁传递的权利。

```

	 Signed-off-by: Random J Developer <random@developer.example.org>

   Setting this flag effectively stops a message for a missing signed-off-by
   line in a patch context.

 - --patch

   Treat FILE as a patch.  This is the default option and need not be
   explicitly specified.

 - --emacs

   Set output to emacs compile window format.  This allows emacs users to jump
   from the error in the compile window directly to the offending line in the
   patch.

 - --terse

   Output only one line per report.

 - --showfile

   Show the diffed file position instead of the input file position.

 - -g,  --git

   Treat FILE as a single commit or a git revision range.

   Single commit with:

   - <rev>
   - <rev>^
   - <rev>~n

   Multiple commits with:

   - <rev1>..<rev2>
   - <rev1>...<rev2>
   - <rev>-<count>

 - -f,  --file

   Treat FILE as a regular source file.  This option must be used when running
   checkpatch on source files in the kernel.

 - --subjective,  --strict

   Enable stricter tests in checkpatch.  By default the tests emitted as CHECK
   do not activate by default.  Use this flag to activate the CHECK tests.

 - --list-types

   Every message emitted by checkpatch has an associated TYPE.  Add this flag
   to display all the types in checkpatch.

   Note that when this flag is active, checkpatch does not read the input FILE,
   and no message is emitted.  Only a list of types in checkpatch is output.

 - --types TYPE(,TYPE2...)

   Only display messages with the given types.

   Example::

     ./scripts/checkpatch.pl mypatch.patch --types EMAIL_SUBJECT,BRACES

 - --ignore TYPE(,TYPE2...)

   Checkpatch will not emit messages for the specified types.

   Example::

     ./scripts/checkpatch.pl mypatch.patch --ignore EMAIL_SUBJECT,BRACES

 - --show-types

   By default checkpatch doesn't display the type associated with the messages.
   Set this flag to show the message type in the output.

 - --max-line-length=n

   Set the max line length (default 100).  If a line exceeds the specified
   length, a LONG_LINE message is emitted.


   The message level is different for patch and file contexts.  For patches,
   a WARNING is emitted.  While a milder CHECK is emitted for files.  So for
   file contexts, the --strict flag must also be enabled.

 - --min-conf-desc-length=n

   Set the Kconfig entry minimum description length, if shorter, warn.

 - --tab-size=n

   Set the number of spaces for tab (default 8).

 - --root=PATH

   PATH to the kernel tree root.

   This option must be specified when invoking checkpatch from outside
   the kernel root.

 - --no-summary

   Suppress the per file summary.

 - --mailback

   Only produce a report in case of Warnings or Errors.  Milder Checks are
   excluded from this.

 - --summary-file

   Include the filename in summary.

 - --debug KEY=[0|1]

   Turn on/off debugging of KEY, where KEY is one of 'values', 'possible',
   'type', and 'attr' (default is all off).

 - --fix

   This is an EXPERIMENTAL feature.  If correctable errors exist, a file
   <inputfile>.EXPERIMENTAL-checkpatch-fixes is created which has the
   automatically fixable errors corrected.

 - --fix-inplace

   EXPERIMENTAL - Similar to --fix but input file is overwritten with fixes.

   DO NOT USE this flag unless you are absolutely sure and you have a backup
   in place.

 - --ignore-perl-version

   Override checking of perl version.  Runtime errors may be encountered after
   enabling this flag if the perl version does not meet the minimum specified.

 - --codespell

   Use the codespell dictionary for checking spelling errors.

 - --codespellfile

   Use the specified codespell file.
   Default is '/usr/share/codespell/dictionary.txt'.

 - --typedefsfile

   Read additional types from this file.

 - --color[=WHEN]

   Use colors 'always', 'never', or only when output is a terminal ('auto').
   Default is 'auto'.

 - --kconfig-prefix=WORD

   Use WORD as a prefix for Kconfig symbols (default is `CONFIG_`).

 - -h, --help, --version

   Display the help text.

```
## 信息级别


checkpatch 中的信息分为三个级别。信息的级别表示错误的严重程度。
它们分别是：

 - ERROR

   这是最严格的级别。ERROR 类型的信息必须被严肃对待，
   因为它们表示非常可能出错的事项。

 - WARNING

   这是次一级的严格级别。WARNING 类型的信息需要更仔细的审查，
   但它比 ERROR 温和。

 - CHECK

   这是最温和的级别。这些是可能需要稍加斟酌的事项。

## 类型描述


本节包含对 checkpatch 中所有信息类型的描述。



### 分配风格


  **ALLOC_ARRAY_ARGS**
    kcalloc 或 kmalloc_array 的第一个参数应为元素个数。
    sizeof() 作为第一个参数通常是错误的。


    See: https://www.kernel.org/doc/html/latest/core-api/memory-allocation.html

  **ALLOC_SIZEOF_STRUCT**
    这种分配风格不佳。通常对于使用 sizeof() 获取内存大小的
    分配函数族而言。
```

      p = alloc(sizeof(struct foo), ...)

    should be::

      p = alloc(sizeof(*p), ...)

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#allocating-memory

  **ALLOC_WITH_MULTIPLY**
    Prefer kmalloc_array/kcalloc over kmalloc/kzalloc with a
    sizeof multiply.

    See: https://www.kernel.org/doc/html/latest/core-api/memory-allocation.html


```
### API 使用


  **ARCH_DEFINES**
    应尽可能避免架构特定的 define，
    无论何处都应如此。

  **ARCH_INCLUDE_LINUX**
    每当包含 asm/file.h 且 linux/file.h 存在时，
    若 linux/file.h 包含了 asm/file.h，可以进行转换。
    但这并非总是如此（见 signal.h）。
    此信息类型仅针对来自 arch/ 的包含发出。

  **AVOID_BUG**
    BUG() 或 BUG_ON() 应被完全避免。
    改用 WARN() 和 WARN_ON()，并尽可能优雅地
    处理“不可能”的错误情况。

    See: https://www.kernel.org/doc/html/latest/process/deprecated.html#bug-and-bug-on

  **CONSIDER_KSTRTO**
    simple_strtol()、simple_strtoll()、simple_strtoul() 和
    simple_strtoull() 函数会显式忽略溢出，这可能
    给调用者带来意外结果。相应的 kstrtol()、
    kstrtoll()、kstrtoul() 和 kstrtoull() 函数通常
    是正确的替代。

    See: https://www.kernel.org/doc/html/latest/process/deprecated.html#simple-strtol-simple-strtoll-simple-strtoul-simple-strtoull

  **CONSTANT_CONVERSION**
```

      __constant_cpu_to_be[x]
      __constant_cpu_to_le[x]
      __constant_be[x]_to_cpu
      __constant_le[x]_to_cpu
      __constant_htons
      __constant_ntohs

    Using any of these outside of include/uapi/ is not preferred as using the
    function without __constant_ is identical when the argument is a
    constant.

    In big endian systems, the macros like __constant_cpu_to_be32(x) and
    cpu_to_be32(x) expand to the same expression::

      #define __constant_cpu_to_be32(x) ((__force __be32)(__u32)(x))
      #define __cpu_to_be32(x)          ((__force __be32)(__u32)(x))

    In little endian systems, the macros __constant_cpu_to_be32(x) and
    cpu_to_be32(x) expand to __constant_swab32 and __swab32.  __swab32
    has a __builtin_constant_p check::

      #define __swab32(x)				\
        (__builtin_constant_p((__u32)(x)) ?	\
        ___constant_swab32(x) :			\
        __fswab32(x))

    So ultimately they have a special case for constants.
    Similar is the case with all of the macros in the list.  Thus
    using the __constant_... forms are unnecessarily verbose and
    not preferred outside of include/uapi.

    See: https://lore.kernel.org/lkml/1400106425.12666.6.camel@joe-AO725/

  **DEPRECATED_API**
    Usage of a deprecated RCU API is detected.  It is recommended to replace
    old flavourful RCU APIs by their new vanilla-RCU counterparts.

    The full list of available RCU APIs can be viewed from the kernel docs.

    See: https://www.kernel.org/doc/html/latest/RCU/whatisRCU.html#full-list-of-rcu-apis

  **DEVICE_ATTR_FUNCTIONS**
    The function names used in DEVICE_ATTR is unusual.
    Typically, the store and show functions are used with <attr>_store and
    <attr>_show, where <attr> is a named attribute variable of the device.

    Consider the following examples::

      static DEVICE_ATTR(type, 0444, type_show, NULL);
      static DEVICE_ATTR(power, 0644, power_show, power_store);

    The function names should preferably follow the above pattern.

    See: https://www.kernel.org/doc/html/latest/driver-api/driver-model/device.html#attributes

  **DEVICE_ATTR_RO**
    The DEVICE_ATTR_RO(name) helper macro can be used instead of
    DEVICE_ATTR(name, 0444, name_show, NULL);

    Note that the macro automatically appends _show to the named
    attribute variable of the device for the show method.

    See: https://www.kernel.org/doc/html/latest/driver-api/driver-model/device.html#attributes

  **DEVICE_ATTR_RW**
    The DEVICE_ATTR_RW(name) helper macro can be used instead of
    DEVICE_ATTR(name, 0644, name_show, name_store);

    Note that the macro automatically appends _show and _store to the
    named attribute variable of the device for the show and store methods.

    See: https://www.kernel.org/doc/html/latest/driver-api/driver-model/device.html#attributes

  **DEVICE_ATTR_WO**
    The DEVICE_AATR_WO(name) helper macro can be used instead of
    DEVICE_ATTR(name, 0200, NULL, name_store);

    Note that the macro automatically appends _store to the
    named attribute variable of the device for the store method.

    See: https://www.kernel.org/doc/html/latest/driver-api/driver-model/device.html#attributes

  **DUPLICATED_SYSCTL_CONST**
    Commit d91bff3011cf ("proc/sysctl: add shared variables for range
    check") added some shared const variables to be used instead of a local
    copy in each source file.

    Consider replacing the sysctl range checking value with the shared
    one in include/linux/sysctl.h.  The following conversion scheme may
    be used::

      &zero     ->  SYSCTL_ZERO
      &one      ->  SYSCTL_ONE
      &int_max  ->  SYSCTL_INT_MAX

    See:

      1. https://lore.kernel.org/lkml/20190430180111.10688-1-mcroce@redhat.com/
      2. https://lore.kernel.org/lkml/20190531131422.14970-1-mcroce@redhat.com/

  **ENOSYS**
    ENOSYS means that a nonexistent system call was called.
    Earlier, it was wrongly used for things like invalid operations on
    otherwise valid syscalls.  This should be avoided in new code.

    See: https://lore.kernel.org/lkml/5eb299021dec23c1a48fa7d9f2c8b794e967766d.1408730669.git.luto@amacapital.net/

  **ENOTSUPP**
    ENOTSUPP is not a standard error code and should be avoided in new patches.
    EOPNOTSUPP should be used instead.

    See: https://lore.kernel.org/netdev/20200510182252.GA411829@lunn.ch/

  **EXPORT_SYMBOL**
    EXPORT_SYMBOL should immediately follow the symbol to be exported.

  **IN_ATOMIC**
    in_atomic() is not for driver use so any such use is reported as an ERROR.
    Also in_atomic() is often used to determine if sleeping is permitted,
    but it is not reliable in this use model.  Therefore its use is
    strongly discouraged.

    However, in_atomic() is ok for core kernel use.

    See: https://lore.kernel.org/lkml/20080320201723.b87b3732.akpm@linux-foundation.org/

  **LOCKDEP**
    The lockdep_no_validate class was added as a temporary measure to
    prevent warnings on conversion of device->sem to device->mutex.
    It should not be used for any other purpose.

    See: https://lore.kernel.org/lkml/1268959062.9440.467.camel@laptop/

  **MALFORMED_INCLUDE**
    The #include statement has a malformed path.  This has happened
    because the author has included a double slash "//" in the pathname
    accidentally.

  **USE_LOCKDEP**
    lockdep_assert_held() annotations should be preferred over
    assertions based on spin_is_locked()

    See: https://www.kernel.org/doc/html/latest/locking/lockdep-design.html#annotations

  **UAPI_INCLUDE**
    No #include statements in include/uapi should use a uapi/ path.

  **USLEEP_RANGE**
    usleep_range() should be preferred over udelay(). The proper way of
    using usleep_range() is mentioned in the kernel docs.


```
### 注释


  **BLOCK_COMMENT_STYLE**
    注释风格不正确。多行注释的首选风格是
```

      /*
       * This is the preferred style
       * for multi line comments.
       */

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#commenting

  **C99_COMMENTS**
    C99 style single line comments (//) should not be used.
    Prefer the block comment style instead.

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#commenting

  **DATA_RACE**
    Applications of data_race() should have a comment so as to document the
    reasoning behind why it was deemed safe.

    See: https://lore.kernel.org/lkml/20200401101714.44781-1-elver@google.com/

  **FSF_MAILING_ADDRESS**
    Kernel maintainers reject new instances of the GPL boilerplate paragraph
    directing people to write to the FSF for a copy of the GPL, since the
    FSF has moved in the past and may do so again.
    So do not write paragraphs about writing to the Free Software Foundation's
    mailing address.

    See: https://lore.kernel.org/lkml/20131006222342.GT19510@leaf/

  **UNCOMMENTED_RGMII_MODE**
    Historically, the RGMII PHY modes specified in Device Trees have been
    used inconsistently, often referring to the usage of delays on the PHY
    side rather than describing the board.

    PHY modes "rgmii", "rgmii-rxid" and "rgmii-txid" modes require the clock
    signal to be delayed on the PCB; this unusual configuration should be
    described in a comment. If they are not (meaning that the delay is realized
    internally in the MAC or PHY), "rgmii-id" is the correct PHY mode.

```
### 提交信息


  **BAD_SIGN_OFF**
    signed-off-by 行不符合社区指定的
    标准。

    See: https://www.kernel.org/doc/html/latest/process/submitting-patches.html#developer-s-certificate-of-origin-1-1

  **BAD_STABLE_ADDRESS_STYLE**
    用于 stable 的邮箱格式不正确。
```

      1. stable@vger.kernel.org
      2. stable@kernel.org

    For adding version info, the following comment style should be used::

      stable@vger.kernel.org # version info

  **COMMIT_COMMENT_SYMBOL**
    Commit log lines starting with a '#' are ignored by git as
    comments.  To solve this problem addition of a single space
    infront of the log line is enough.

  **COMMIT_MESSAGE**
    The patch is missing a commit description.  A brief
    description of the changes made by the patch should be added.

    See: https://www.kernel.org/doc/html/latest/process/submitting-patches.html#describe-your-changes

  **EMAIL_SUBJECT**
    Naming the tool that found the issue is not very useful in the
    subject line.  A good subject line summarizes the change that
    the patch brings.

    See: https://www.kernel.org/doc/html/latest/process/submitting-patches.html#describe-your-changes

  **FROM_SIGN_OFF_MISMATCH**
    The author's email does not match with that in the Signed-off-by:
    line(s). This can be sometimes caused due to an improperly configured
    email client.

    This message is emitted due to any of the following reasons::

      - The email names do not match.
      - The email addresses do not match.
      - The email subaddresses do not match.
      - The email comments do not match.

  **MISSING_SIGN_OFF**
    The patch is missing a Signed-off-by line.  A signed-off-by
    line should be added according to Developer's certificate of
    Origin.

    See: https://www.kernel.org/doc/html/latest/process/submitting-patches.html#sign-your-work-the-developer-s-certificate-of-origin

  **NO_AUTHOR_SIGN_OFF**
    The author of the patch has not signed off the patch.  It is
    required that a simple sign off line should be present at the
    end of explanation of the patch to denote that the author has
    written it or otherwise has the rights to pass it on as an open
    source patch.

    See: https://www.kernel.org/doc/html/latest/process/submitting-patches.html#sign-your-work-the-developer-s-certificate-of-origin

  **DIFF_IN_COMMIT_MSG**
    Avoid having diff content in commit message.
    This causes problems when one tries to apply a file containing both
    the changelog and the diff because patch(1) tries to apply the diff
    which it found in the changelog.

    See: https://lore.kernel.org/lkml/20150611134006.9df79a893e3636019ad2759e@linux-foundation.org/

  **GERRIT_CHANGE_ID**
    To be picked up by gerrit, the footer of the commit message might
    have a Change-Id like::

      Change-Id: Ic8aaa0728a43936cd4c6e1ed590e01ba8f0fbf5b
      Signed-off-by: A. U. Thor <author@example.com>

    The Change-Id line must be removed before submitting.

  **GIT_COMMIT_ID**
    The proper way to reference a commit id is:
    commit <12+ chars of sha1> ("<title line>")

    An example may be::

      Commit e21d2170f36602ae2708 ("video: remove unnecessary
      platform_set_drvdata()") removed the unnecessary
      platform_set_drvdata(), but left the variable "dev" unused,
      delete it.

    See: https://www.kernel.org/doc/html/latest/process/submitting-patches.html#describe-your-changes

  **BAD_FIXES_TAG**
    The Fixes: tag is malformed or does not follow the community conventions.
    This can occur if the tag have been split into multiple lines (e.g., when
    pasted in an email program with word wrapping enabled).

    See: https://www.kernel.org/doc/html/latest/process/submitting-patches.html#describe-your-changes

  **BAD_COMMIT_SEPARATOR**
    The commit separator is a single line with 3 dashes.
    The regex match is '^---$'
    Lines that start with 3 dashes and have more content on the same line
    may confuse tools that apply patches.

```
### 比较风格


  **ASSIGN_IN_IF**
    不要在 if 条件中使用赋值。
```

      if ((foo = bar(...)) < BAZ) {

    should be written as::

      foo = bar(...);
      if (foo < BAZ) {

  **BOOL_COMPARISON**
    Comparisons of A to true and false are better written
    as A and !A.

    See: https://lore.kernel.org/lkml/1365563834.27174.12.camel@joe-AO722/

  **COMPARISON_TO_NULL**
    Comparisons to NULL in the form (foo == NULL) or (foo != NULL)
    are better written as (!foo) and (foo).

  **CONSTANT_COMPARISON**
    Comparisons with a constant or upper case identifier on the left
    side of the test should be avoided.


```
### 缩进与换行


  **CODE_INDENT**
    代码缩进应使用 tab 而非空格。
    除注释、文档和 Kconfig 之外，
    空格从不被用于缩进。

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#indentation

  **DEEP_INDENTATION**
    使用 6 个或更多 tab 的缩进通常表明
    代码缩进过深。

    建议重构 if/else/for/do/while/switch 语句中
    过度的缩进。

    See: https://lore.kernel.org/lkml/1328311239.21255.24.camel@joe2Laptop/

  **SWITCH_CASE_INDENT_LEVEL**
    switch 应与 case 处于相同的缩进级别。
```

      switch (suffix) {
      case 'G':
      case 'g':
              mem <<= 30;
              break;
      case 'M':
      case 'm':
              mem <<= 20;
              break;
      case 'K':
      case 'k':
              mem <<= 10;
              fallthrough;
      default:
              break;
      }

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#indentation

  **LONG_LINE**
    The line has exceeded the specified maximum length.
    To use a different maximum line length, the --max-line-length=n option
    may be added while invoking checkpatch.

    Earlier, the default line length was 80 columns.  Commit bdc48fa11e46
    ("checkpatch/coding-style: deprecate 80-column warning") increased the
    limit to 100 columns.  This is not a hard limit either and it's
    preferable to stay within 80 columns whenever possible.

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#breaking-long-lines-and-strings

  **LONG_LINE_STRING**
    A string starts before but extends beyond the maximum line length.
    To use a different maximum line length, the --max-line-length=n option
    may be added while invoking checkpatch.

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#breaking-long-lines-and-strings

  **LONG_LINE_COMMENT**
    A comment starts before but extends beyond the maximum line length.
    To use a different maximum line length, the --max-line-length=n option
    may be added while invoking checkpatch.

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#breaking-long-lines-and-strings

  **SPLIT_STRING**
    Quoted strings that appear as messages in userspace and can be
    grepped, should not be split across multiple lines.

    See: https://lore.kernel.org/lkml/20120203052727.GA15035@leaf/

  **MULTILINE_DEREFERENCE**
    A single dereferencing identifier spanned on multiple lines like::

      struct_identifier->member[index].
      member = <foo>;

    is generally hard to follow. It can easily lead to typos and so makes
    the code vulnerable to bugs.

    If fixing the multiple line dereferencing leads to an 80 column
    violation, then either rewrite the code in a more simple way or if the
    starting part of the dereferencing identifier is the same and used at
    multiple places then store it in a temporary variable, and use that
    temporary variable only at all the places. For example, if there are
    two dereferencing identifiers::

      member1->member2->member3.foo1;
      member1->member2->member3.foo2;

    then store the member1->member2->member3 part in a temporary variable.
    It not only helps to avoid the 80 column violation but also reduces
    the program size by removing the unnecessary dereferences.

    But if none of the above methods work then ignore the 80 column
    violation because it is much easier to read a dereferencing identifier
    on a single line.

  **TRAILING_STATEMENTS**
    Trailing statements (for example after any conditional) should be
    on the next line.
    Statements, such as::

      if (x == y) break;

    should be::

      if (x == y)
              break;


```
### 宏、属性与符号


  **ARRAY_SIZE**
    ARRAY_SIZE(foo) 宏应优先于
    sizeof(foo)/sizeof(foo[^0^])，用于获取数组中
    的元素个数。

```

      #define ARRAY_SIZE(x) (sizeof(x) / sizeof((x)[0]))

  **AVOID_EXTERNS**
    Function prototypes don't need to be declared extern in .h
    files.  It's assumed by the compiler and is unnecessary.

  **AVOID_L_PREFIX**
    Local symbol names that are prefixed with `.L` should be avoided,
    as this has special meaning for the assembler; a symbol entry will
    not be emitted into the symbol table.  This can prevent `objtool`
    from generating correct unwind info.

    Symbols with STB_LOCAL binding may still be used, and `.L` prefixed
    local symbol names are still generally usable within a function,
    but `.L` prefixed local symbol names should not be used to denote
    the beginning or end of code regions via
    `SYM_CODE_START_LOCAL`/`SYM_CODE_END`

  **BIT_MACRO**
    Defines like: 1 << <digit> could be BIT(digit).
    The BIT() macro is defined via include/linux/bits.h::

      #define BIT(nr)         (1UL << (nr))

  **CONST_READ_MOSTLY**
    When a variable is tagged with the __read_mostly annotation, it is a
    signal to the compiler that accesses to the variable will be mostly
    reads and rarely(but NOT never) a write.

    const __read_mostly does not make any sense as const data is already
    read-only.  The __read_mostly annotation thus should be removed.

  **DATE_TIME**
    It is generally desirable that building the same source code with
    the same set of tools is reproducible, i.e. the output is always
    exactly the same.

    The kernel does *not* use the ``__DATE__`` and ``__TIME__`` macros,
    and enables warnings if they are used as they can lead to
    non-deterministic builds.

    See: https://www.kernel.org/doc/html/latest/kbuild/reproducible-builds.html#timestamps

  **DEFINE_ARCH_HAS**
    The ARCH_HAS_xyz and ARCH_HAVE_xyz patterns are wrong.

    For big conceptual features use Kconfig symbols instead.  And for
    smaller things where we have compatibility fallback functions but
    want architectures able to override them with optimized ones, we
    should either use weak functions (appropriate for some cases), or
    the symbol that protects them should be the same symbol we use.

    See: https://lore.kernel.org/lkml/CA+55aFycQ9XJvEOsiM3txHL5bjUc8CeKWJNR_H+MiicaddB42Q@mail.gmail.com/

  **DO_WHILE_MACRO_WITH_TRAILING_SEMICOLON**
    do {} while(0) macros should not have a trailing semicolon.

  **INIT_ATTRIBUTE**
    Const init definitions should use __initconst instead of
    __initdata.

    Similarly init definitions without const require a separate
    use of const.

  **INLINE_LOCATION**
    The inline keyword should sit between storage class and type.

    For example, the following segment::

      inline static int example_function(void)
      {
              ...
      }

    should be::

      static inline int example_function(void)
      {
              ...
      }

  **MISPLACED_INIT**
    It is possible to use section markers on variables in a way
    which gcc doesn't understand (or at least not the way the
    developer intended)::

      static struct __initdata samsung_pll_clock exynos4_plls[nr_plls] = {

    does not put exynos4_plls in the .initdata section. The __initdata
    marker can be virtually anywhere on the line, except right after
    "struct". The preferred location is before the "=" sign if there is
    one, or before the trailing ";" otherwise.

    See: https://lore.kernel.org/lkml/1377655732.3619.19.camel@joe-AO722/

  **MULTISTATEMENT_MACRO_USE_DO_WHILE**
    Macros with multiple statements should be enclosed in a
    do - while block.  Same should also be the case for macros
    starting with `if` to avoid logic defects::

      #define macrofun(a, b, c)                 \
        do {                                    \
                if (a == 5)                     \
                        do_this(b, c);          \
        } while (0)

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#macros-enums-and-rtl

  **PREFER_FALLTHROUGH**
    Use the `fallthrough;` pseudo keyword instead of
    `/* fallthrough */` like comments.

  **TRAILING_SEMICOLON**
    Macro definition should not end with a semicolon. The macro
    invocation style should be consistent with function calls.
    This can prevent any unexpected code paths::

      #define MAC do_something;

    If this macro is used within a if else statement, like::

      if (some_condition)
              MAC;

      else
              do_something;

    Then there would be a compilation error, because when the macro is
    expanded there are two trailing semicolons, so the else branch gets
    orphaned.

    See: https://lore.kernel.org/lkml/1399671106.2912.21.camel@joe-AO725/

  **MACRO_ARG_UNUSED**
    If function-like macros do not utilize a parameter, it might result
    in a build warning. We advocate for utilizing static inline functions
    to replace such macros.
    For example, for a macro such as the one below::

      #define test(a) do { } while (0)

    there would be a warning like below::

      WARNING: Argument 'a' is not used in function-like macro.

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#macros-enums-and-rtl

  **SINGLE_STATEMENT_DO_WHILE_MACRO**
    For the multi-statement macros, it is necessary to use the do-while
    loop to avoid unpredictable code paths. The do-while loop helps to
    group the multiple statements into a single one so that a
    function-like macro can be used as a function only.

    But for the single statement macros, it is unnecessary to use the
    do-while loop. Although the code is syntactically correct but using
    the do-while loop is redundant. So remove the do-while loop for single
    statement macros.

  **WEAK_DECLARATION**
    Using weak declarations like __attribute__((weak)) or __weak
    can have unintended link defects.  Avoid using them.


```
### 函数与变量


  **CAMELCASE**
    避免使用驼峰命名（CamelCase）标识符。

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#naming

  **CONST_CONST**
    使用 `const <type> const *` 通常应写为
    `const <type> * const`。

  **CONST_STRUCT**
    使用 const 通常是个好主意。Checkpatch 会读取
    一个常用结构体列表，这些结构体总是或
    几乎总是常量。

    现有的结构体列表可从
    `scripts/const_structs.checkpatch` 查看。

    See: https://lore.kernel.org/lkml/alpine.DEB.2.10.1608281509480.3321@hadrien/

    嵌入式函数名不太适合使用，因为重构可能导致
    函数重命名。优先使用
    "%s"、__func__ 而非
    嵌入式函数名。

    注意，这在 -f（--file）checkpatch 选项下不起作用，
    因为它依赖补丁上下文提供函数名。

  **FUNCTION_ARGUMENTS**
    此警告因以下任一原因发出：

       1. 函数声明中的参数没有按如下方式书写：
```

           void foo
           (int bar, int baz)

         This should be corrected to::

           void foo(int bar, int baz)

      2. Some arguments for the function definition do not
         have an identifier name.  Example::

           void foo(int)

         All arguments should have identifier names.

  **FUNCTION_WITHOUT_ARGS**
    Function declarations without arguments like::

      int foo()

    should be::

      int foo(void)

  **GLOBAL_INITIALISERS**
    Global variables should not be initialized explicitly to
    0 (or NULL, false, etc.).  Your compiler (or rather your
    loader, which is responsible for zeroing out the relevant
    sections) automatically does it for you.

  **INITIALISED_STATIC**
    Static variables should not be initialized explicitly to zero.
    Your compiler (or rather your loader) automatically does
    it for you.

  **MULTIPLE_ASSIGNMENTS**
    Multiple assignments on a single line makes the code unnecessarily
    complicated. So on a single line assign value to a single variable
    only, this makes the code more readable and helps avoid typos.

  **RETURN_PARENTHESES**
    return is not a function and as such doesn't need parentheses::

      return (bar);

    can simply be::

      return bar;

  **UNINITIALIZED_PTR_WITH_FREE**
    Pointers with __free attribute should be declared at the place of use
    and initialized (see include/linux/cleanup.h). In this case
    declarations at the top of the function rule can be relaxed. Not doing
    so may lead to undefined behavior as the memory assigned (garbage,
    in case not initialized) to the pointer is freed automatically when
    the pointer goes out of scope.

    Also see: https://lore.kernel.org/lkml/58fd478f408a34b578ee8d949c5c4b4da4d4f41d.camel@HansenPartnership.com/

    Example::

      type var __free(free_func);
      ... // var not used, but, in future someone might add a return here
      var = malloc(var_size);
      ...

    should be initialized as::

      ...
      type var __free(free_func) = malloc(var_size);
      ...


```
### 权限


  **DEVICE_ATTR_PERMS**
    DEVICE_ATTR 中使用的权限不常见。
    通常只使用三种权限 —— 0644（RW）、0444（RO）
    和 0200（WO）。

    See: https://www.kernel.org/doc/html/latest/filesystems/sysfs.html#attributes

  **EXECUTE_PERMISSIONS**
    源文件没有理由需要可执行。可执行位
    可以安全地移除。

  **EXPORTED_WORLD_WRITABLE**
    导出全局可写的 sysfs/debugfs 文件通常是件坏事。
    随意这样做可能引入严重的安全漏洞。
    过去，某些 debugfs 漏洞看似允许任何本地用户
    向设备寄存器写入任意值 —— 这种情况
    几乎不会带来什么好处。

    See: https://lore.kernel.org/linux-arm-kernel/cover.1296818921.git.segoon@openwall.com/

  **NON_OCTAL_PERMISSIONS**
    权限位应使用 4 位八进制权限（如 0700 或 0444）。
    避免使用十进制等任何其他进制。

  **SYMBOLIC_PERMS**
    八进制形式的权限位比其符号形式更易读、更易理解，
    因为许多命令行工具都使用这种表示法。经验丰富的内核开发者
    数十年来一直使用这些传统的 Unix 权限位，因此他们发现
    八进制表示法比符号宏更容易理解。例如，
    S_IWUSR|S_IRUGO 比 0644 更难读，而 0644 反而
    模糊了开发者的意图而非澄清它。


    See: https://lore.kernel.org/lkml/CA+55aFw5v23T-zvDZp-MmD_EYxF8WbafwwB59934FV7g21uMGQ@mail.gmail.com/


### Spacing and Brackets


  **ASSIGNMENT_CONTINUATIONS**
    Assignment operators should not be written at the start of a
    line but should follow the operand at the previous line.

  **BRACES**
    The placement of braces is stylistically incorrect.
    The preferred way is to put the opening brace last on the line,
```

      if (x is true) {
              we do y
      }

    This applies for all non-functional blocks.
    However, there is one special case, namely functions: they have the
    opening brace at the beginning of the next line, thus::

      int function(int x)
      {
              body of function
      }

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#placing-braces-and-spaces

  **BRACKET_SPACE**
    Whitespace before opening bracket '[' is prohibited.
    There are some exceptions:

    1. With a type on the left::

        int [] a;

    2. At the beginning of a line for slice initialisers::

        [0...10] = 5,

    3. Inside a curly brace::

        = { [0...10] = 5 }

  **CONCATENATED_STRING**
    Concatenated elements should have a space in between.
    Example::

      printk(KERN_INFO"bar");

    should be::

      printk(KERN_INFO "bar");

  **ELSE_AFTER_BRACE**
    `else {` should follow the closing block `}` on the same line.

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#placing-braces-and-spaces

  **LINE_SPACING**
    Vertical space is wasted given the limited number of lines an
    editor window can display when multiple blank lines are used.

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#spaces

  **OPEN_BRACE**
    The opening brace should be following the function definitions on the
    next line.  For any non-functional block it should be on the same line
    as the last construct.

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#placing-braces-and-spaces

  **POINTER_LOCATION**
    When using pointer data or a function that returns a pointer type,
    the preferred use of * is adjacent to the data name or function name
    and not adjacent to the type name.
    Examples::

      char *linux_banner;
      unsigned long long memparse(char *ptr, char **retptr);
      char *match_strdup(substring_t *s);

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#spaces

  **SPACING**
    Whitespace style used in the kernel sources is described in kernel docs.

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#spaces

  **TRAILING_WHITESPACE**
    Trailing whitespace should always be removed.
    Some editors highlight the trailing whitespace and cause visual
    distractions when editing files.

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#spaces

  **UNNECESSARY_PARENTHESES**
    Parentheses are not required in the following cases:

      1. Function pointer uses::

          (foo->bar)();

        could be::

          foo->bar();

      2. Comparisons in if::

          if ((foo->bar) && (foo->baz))
          if ((foo == bar))

        could be::

          if (foo->bar && foo->baz)
          if (foo == bar)

      3. addressof/dereference single Lvalues::

          &(foo->bar)
          *(foo->bar)

        could be::

          &foo->bar
          *foo->bar

  **WHILE_AFTER_BRACE**
    while should follow the closing bracket on the same line::

      do {
              ...
      } while(something);

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#placing-braces-and-spaces


```
### 其他


  **CONFIG_DESCRIPTION**
    Kconfig 符号应有一个完整描述它的帮助文本。


  **CORRUPTED_PATCH**
    补丁似乎已损坏或行被换行。
    请在发送给维护者之前重新生成补丁文件。

  **CVS_KEYWORD**
    由于 linux 已迁移到 git，CVS 标记不再使用。
    因此不应添加 CVS 风格的关键字（$Id$、 $Revision$、 $Log$）。


  **DEFAULT_NO_BREAK**
    switch 的 default case 有时会被写成 "default:;"。这可能导致
    在 default 之下新增的 case 出现缺陷。

    应在空的 default 语句之后添加 "break;"，以避免
    不期望的 fallthrough。

  **DOS_LINE_ENDINGS**
    对于 DOS 格式的补丁，行尾会有多余的 ^M 符号。
    应将其移除。

  **DT_SCHEMA_BINDING_PATCH**
    DT 绑定已迁移到基于 json-schema 的格式，而非
    自由格式文本。

    See: https://www.kernel.org/doc/html/latest/devicetree/bindings/writing-schema.html

  **DT_SPLIT_BINDING_PATCH**
    设备树绑定应当是它们自己的独立补丁。这是因为
    绑定在逻辑上独立于驱动实现，它们有不同的维护者
（即使通常经由同一棵树合入），并且这样能让用
    git-filter-branch 创建的纯 DT 树拥有更清晰的
    历史记录。

    See: https://www.kernel.org/doc/html/latest/devicetree/bindings/submitting-patches.html#i-for-patch-submitters

  **EMBEDDED_FILENAME**
    在文件内嵌入完整文件名路径并没有特别大的用处，
    因为路径经常被移动从而变得不正确。

  **FILE_PATH_CHANGES**
    每当添加、移动或删除文件时，MAINTAINERS 文件中的
    模式可能不同步或过期。

    因此在这些情况下可能需要更新 MAINTAINERS。

  **MEMSET**
    memset 的使用似乎不正确。这可能是由
    参数顺序错误导致。请重新检查用法。

  **NOT_UNIFIED_DIFF**
    补丁文件似乎不是 unified-diff 格式。请
    在发送给维护者之前重新生成补丁文件。

  **PLACEHOLDER_USE**
    检测遗留在封面信或提交头/日志中、未处理的占位符文本。
```

      *** SUBJECT HERE ***
      *** BLURB HERE ***

    These typically come from autogenerated templates. Replace them with a proper
    subject and description before sending.

  **PRINTF_0XDECIMAL**
    Prefixing 0x with decimal output is defective and should be corrected.

  **SPDX_LICENSE_TAG**
    The source file is missing or has an improper SPDX identifier tag.
    The Linux kernel requires the precise SPDX identifier in all source files,
    and it is thoroughly documented in the kernel docs.

    See: https://www.kernel.org/doc/html/latest/process/license-rules.html

  **TYPO_SPELLING**
    Some words may have been misspelled.  Consider reviewing them.

```