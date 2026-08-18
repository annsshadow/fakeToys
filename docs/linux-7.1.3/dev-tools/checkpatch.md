
## Checkpatch


Checkpatch (scripts/checkpatch.pl) 鏄竴涓?perl 鑴氭湰锛岀敤浜庢鏌ヨˉ涓佷腑鐨勭悙纰庨鏍艰繚瑙勶紝骞跺彲閫夋嫨鎬у湴淇瀹冧滑銆?
Checkpatch 涔熷彲浠ラ拡瀵规枃浠朵笂涓嬫枃杩愯锛屽苟涓?
鏃犻渶鍐呮牳婧愮爜鏍戝嵆鍙繍琛屻€?

Checkpatch 骞堕潪姘歌繙姝ｇ‘銆備綘鐨勫垽鏂紭鍏堜簬 checkpatch 缁欏嚭鐨勪俊鎭€?
濡傛灉浣犵殑浠ｇ爜甯︾潃杩欎簺杩濊鐪嬭捣鏉ユ洿濂斤紝閭ｄ箞寰堝彲鑳芥渶濂?
淇濇寔鍘熸牱銆?


## 閫夐」


鏈妭灏嗘弿杩?checkpatch 杩愯鏃剁殑鍚勪釜閫夐」銆?

```

  ./scripts/checkpatch.pl [OPTION]... [FILE]...

```
鍙敤閫夐」锛?

 - -q,  --quiet

   鍚敤瀹夐潤妯″紡銆?

 - -v,  --verbose
   鍚敤璇︾粏妯″紡銆備細杈撳嚭棰濆鐨勮缁嗘祴璇曡鏄庯紝
   浠ユ彁渚涜鐗瑰畾淇℃伅涓轰綍琚樉绀虹殑鍘熷洜銆?

 - --no-tree

   鍦ㄦ病鏈夊唴鏍告簮鐮佹爲鐨勬儏鍐典笅杩愯 checkpatch銆?

 - --no-signoff

   绂佺敤 'Signed-off-by' 琛屾鏌ャ€俿ign-off 鏄ˉ涓佽鏄庢湯灏剧殑涓€琛岀畝鍗曟枃鏈紝
   鐢ㄤ簬璇佹槑浣犵紪鍐欎簡瀹冿紝鎴?
   浣犳嫢鏈夊皢鍏朵綔涓哄紑婧愯ˉ涓佷紶閫掔殑鏉冨埄銆?

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
## 淇℃伅绾у埆


checkpatch 涓殑淇℃伅鍒嗕负涓変釜绾у埆銆備俊鎭殑绾у埆琛ㄧず閿欒鐨勪弗閲嶇▼搴︺€?
瀹冧滑鍒嗗埆鏄細

 - ERROR

   杩欐槸鏈€涓ユ牸鐨勭骇鍒€侲RROR 绫诲瀷鐨勪俊鎭繀椤昏涓ヨ們瀵瑰緟锛?
   鍥犱负瀹冧滑琛ㄧず闈炲父鍙兘鍑洪敊鐨勪簨椤广€?

 - WARNING

   杩欐槸娆′竴绾х殑涓ユ牸绾у埆銆俉ARNING 绫诲瀷鐨勪俊鎭渶瑕佹洿浠旂粏鐨勫鏌ワ紝
   浣嗗畠姣?ERROR 娓╁拰銆?

 - CHECK

   杩欐槸鏈€娓╁拰鐨勭骇鍒€傝繖浜涙槸鍙兘闇€瑕佺◢鍔犳枱閰岀殑浜嬮」銆?

## 绫诲瀷鎻忚堪


鏈妭鍖呭惈瀵?checkpatch 涓墍鏈変俊鎭被鍨嬬殑鎻忚堪銆?



### 鍒嗛厤椋庢牸


  **ALLOC_ARRAY_ARGS**
    kcalloc 鎴?kmalloc_array 鐨勭涓€涓弬鏁板簲涓哄厓绱犱釜鏁般€?
    sizeof() 浣滀负绗竴涓弬鏁伴€氬父鏄敊璇殑銆?


    See: https://www.kernel.org/doc/html/latest/core-api/memory-allocation.html

  **ALLOC_SIZEOF_STRUCT**
    杩欑鍒嗛厤椋庢牸涓嶄匠銆傞€氬父瀵逛簬浣跨敤 sizeof() 鑾峰彇鍐呭瓨澶у皬鐨?
    鍒嗛厤鍑芥暟鏃忚€岃█銆?
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
### API 浣跨敤


  **ARCH_DEFINES**
    搴斿敖鍙兘閬垮厤鏋舵瀯鐗瑰畾鐨?define锛?
    鏃犺浣曞閮藉簲濡傛銆?

  **ARCH_INCLUDE_LINUX**
    姣忓綋鍖呭惈 asm/file.h 涓?linux/file.h 瀛樺湪鏃讹紝
    鑻?linux/file.h 鍖呭惈浜?asm/file.h锛屽彲浠ヨ繘琛岃浆鎹€?
    浣嗚繖骞堕潪鎬绘槸濡傛锛堣 signal.h锛夈€?
    姝や俊鎭被鍨嬩粎閽堝鏉ヨ嚜 arch/ 鐨勫寘鍚彂鍑恒€?

  **AVOID_BUG**
    BUG() 鎴?BUG_ON() 搴旇瀹屽叏閬垮厤銆?
    鏀圭敤 WARN() 鍜?WARN_ON()锛屽苟灏藉彲鑳戒紭闆呭湴
    澶勭悊鈥滀笉鍙兘鈥濈殑閿欒鎯呭喌銆?

    See: https://www.kernel.org/doc/html/latest/process/deprecated.html#bug-and-bug-on

  **CONSIDER_KSTRTO**
    simple_strtol()銆乻imple_strtoll()銆乻imple_strtoul() 鍜?
    simple_strtoull() 鍑芥暟浼氭樉寮忓拷鐣ユ孩鍑猴紝杩欏彲鑳?
    缁欒皟鐢ㄨ€呭甫鏉ユ剰澶栫粨鏋溿€傜浉搴旂殑 kstrtol()銆?
    kstrtoll()銆乲strtoul() 鍜?kstrtoull() 鍑芥暟閫氬父
    鏄纭殑鏇夸唬銆?

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
### 娉ㄩ噴


  **BLOCK_COMMENT_STYLE**
    娉ㄩ噴椋庢牸涓嶆纭€傚琛屾敞閲婄殑棣栭€夐鏍兼槸
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
### 鎻愪氦淇℃伅


  **BAD_SIGN_OFF**
    signed-off-by 琛屼笉绗﹀悎绀惧尯鎸囧畾鐨?
    鏍囧噯銆?

    See: https://www.kernel.org/doc/html/latest/process/submitting-patches.html#developer-s-certificate-of-origin-1-1

  **BAD_STABLE_ADDRESS_STYLE**
    鐢ㄤ簬 stable 鐨勯偖绠辨牸寮忎笉姝ｇ‘銆?
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
### 姣旇緝椋庢牸


  **ASSIGN_IN_IF**
    涓嶈鍦?if 鏉′欢涓娇鐢ㄨ祴鍊笺€?
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
### 缂╄繘涓庢崲琛?


  **CODE_INDENT**
    浠ｇ爜缂╄繘搴斾娇鐢?tab 鑰岄潪绌烘牸銆?
    闄ゆ敞閲娿€佹枃妗ｅ拰 Kconfig 涔嬪锛?
    绌烘牸浠庝笉琚敤浜庣缉杩涖€?

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#indentation

  **DEEP_INDENTATION**
    浣跨敤 6 涓垨鏇村 tab 鐨勭缉杩涢€氬父琛ㄦ槑
    浠ｇ爜缂╄繘杩囨繁銆?

    寤鸿閲嶆瀯 if/else/for/do/while/switch 璇彞涓?
    杩囧害鐨勭缉杩涖€?

    See: https://lore.kernel.org/lkml/1328311239.21255.24.camel@joe2Laptop/

  **SWITCH_CASE_INDENT_LEVEL**
    switch 搴斾笌 case 澶勪簬鐩稿悓鐨勭缉杩涚骇鍒€?
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
### 瀹忋€佸睘鎬т笌绗﹀彿


  **ARRAY_SIZE**
    ARRAY_SIZE(foo) 瀹忓簲浼樺厛浜?
    sizeof(foo)/sizeof(foo[^0^])锛岀敤浜庤幏鍙栨暟缁勪腑
    鐨勫厓绱犱釜鏁般€?

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
### 鍑芥暟涓庡彉閲?


  **CAMELCASE**
    閬垮厤浣跨敤椹煎嘲鍛藉悕锛圕amelCase锛夋爣璇嗙銆?

    See: https://www.kernel.org/doc/html/latest/process/coding-style.html#naming

  **CONST_CONST**
    浣跨敤 `const <type> const *` 閫氬父搴斿啓涓?
    `const <type> * const`銆?

  **CONST_STRUCT**
    浣跨敤 const 閫氬父鏄釜濂戒富鎰忋€侰heckpatch 浼氳鍙?
    涓€涓父鐢ㄧ粨鏋勪綋鍒楄〃锛岃繖浜涚粨鏋勪綋鎬绘槸鎴?
    鍑犱箮鎬绘槸甯搁噺銆?

    鐜版湁鐨勭粨鏋勪綋鍒楄〃鍙粠
    `scripts/const_structs.checkpatch` 鏌ョ湅銆?

    See: https://lore.kernel.org/lkml/alpine.DEB.2.10.1608281509480.3321@hadrien/

    宓屽叆寮忓嚱鏁板悕涓嶅お閫傚悎浣跨敤锛屽洜涓洪噸鏋勫彲鑳藉鑷?
    鍑芥暟閲嶅懡鍚嶃€備紭鍏堜娇鐢?
    "%s"銆乢_func__ 鑰岄潪
    宓屽叆寮忓嚱鏁板悕銆?

    娉ㄦ剰锛岃繖鍦?-f锛?-file锛塩heckpatch 閫夐」涓嬩笉璧蜂綔鐢紝
    鍥犱负瀹冧緷璧栬ˉ涓佷笂涓嬫枃鎻愪緵鍑芥暟鍚嶃€?

  **FUNCTION_ARGUMENTS**
    姝よ鍛婂洜浠ヤ笅浠讳竴鍘熷洜鍙戝嚭锛?

       1. 鍑芥暟澹版槑涓殑鍙傛暟娌℃湁鎸夊涓嬫柟寮忎功鍐欙細
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
### 鏉冮檺


  **DEVICE_ATTR_PERMS**
    DEVICE_ATTR 涓娇鐢ㄧ殑鏉冮檺涓嶅父瑙併€?
    閫氬父鍙娇鐢ㄤ笁绉嶆潈闄?鈥斺€?0644锛圧W锛夈€?444锛圧O锛?
    鍜?0200锛圵O锛夈€?

    See: https://www.kernel.org/doc/html/latest/filesystems/sysfs.html#attributes

  **EXECUTE_PERMISSIONS**
    婧愭枃浠舵病鏈夌悊鐢遍渶瑕佸彲鎵ц銆傚彲鎵ц浣?
    鍙互瀹夊叏鍦扮Щ闄ゃ€?

  **EXPORTED_WORLD_WRITABLE**
    瀵煎嚭鍏ㄥ眬鍙啓鐨?sysfs/debugfs 鏂囦欢閫氬父鏄欢鍧忎簨銆?
    闅忔剰杩欐牱鍋氬彲鑳藉紩鍏ヤ弗閲嶇殑瀹夊叏婕忔礊銆?
    杩囧幓锛屾煇浜?debugfs 婕忔礊鐪嬩技鍏佽浠讳綍鏈湴鐢ㄦ埛
    鍚戣澶囧瘎瀛樺櫒鍐欏叆浠绘剰鍊?鈥斺€?杩欑鎯呭喌
    鍑犱箮涓嶄細甯︽潵浠€涔堝ソ澶勩€?

    See: https://lore.kernel.org/linux-arm-kernel/cover.1296818921.git.segoon@openwall.com/

  **NON_OCTAL_PERMISSIONS**
    鏉冮檺浣嶅簲浣跨敤 4 浣嶅叓杩涘埗鏉冮檺锛堝 0700 鎴?0444锛夈€?
    閬垮厤浣跨敤鍗佽繘鍒剁瓑浠讳綍鍏朵粬杩涘埗銆?

  **SYMBOLIC_PERMS**
    鍏繘鍒跺舰寮忕殑鏉冮檺浣嶆瘮鍏剁鍙峰舰寮忔洿鏄撹銆佹洿鏄撶悊瑙ｏ紝
    鍥犱负璁稿鍛戒护琛屽伐鍏烽兘浣跨敤杩欑琛ㄧず娉曘€傜粡楠屼赴瀵岀殑鍐呮牳寮€鍙戣€?
    鏁板崄骞存潵涓€鐩翠娇鐢ㄨ繖浜涗紶缁熺殑 Unix 鏉冮檺浣嶏紝鍥犳浠栦滑鍙戠幇
    鍏繘鍒惰〃绀烘硶姣旂鍙峰畯鏇村鏄撶悊瑙ｃ€備緥濡傦紝
    S_IWUSR|S_IRUGO 姣?0644 鏇撮毦璇伙紝鑰?0644 鍙嶈€?
    妯＄硦浜嗗紑鍙戣€呯殑鎰忓浘鑰岄潪婢勬竻瀹冦€?


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
### 鍏朵粬


  **CONFIG_DESCRIPTION**
    Kconfig 绗﹀彿搴旀湁涓€涓畬鏁存弿杩板畠鐨勫府鍔╂枃鏈€?


  **CORRUPTED_PATCH**
    琛ヤ竵浼间箮宸叉崯鍧忔垨琛岃鎹㈣銆?
    璇峰湪鍙戦€佺粰缁存姢鑰呬箣鍓嶉噸鏂扮敓鎴愯ˉ涓佹枃浠躲€?

  **CVS_KEYWORD**
    鐢变簬 linux 宸茶縼绉诲埌 git锛孋VS 鏍囪涓嶅啀浣跨敤銆?
    鍥犳涓嶅簲娣诲姞 CVS 椋庢牸鐨勫叧閿瓧锛?Id$銆?$Revision$銆?$Log$锛夈€?


  **DEFAULT_NO_BREAK**
    switch 鐨?default case 鏈夋椂浼氳鍐欐垚 "default:;"銆傝繖鍙兘瀵艰嚧
    鍦?default 涔嬩笅鏂板鐨?case 鍑虹幇缂洪櫡銆?

    搴斿湪绌虹殑 default 璇彞涔嬪悗娣诲姞 "break;"锛屼互閬垮厤
    涓嶆湡鏈涚殑 fallthrough銆?

  **DOS_LINE_ENDINGS**
    瀵逛簬 DOS 鏍煎紡鐨勮ˉ涓侊紝琛屽熬浼氭湁澶氫綑鐨?^M 绗﹀彿銆?
    搴斿皢鍏剁Щ闄ゃ€?

  **DT_SCHEMA_BINDING_PATCH**
    DT 缁戝畾宸茶縼绉诲埌鍩轰簬 json-schema 鐨勬牸寮忥紝鑰岄潪
    鑷敱鏍煎紡鏂囨湰銆?

    See: https://www.kernel.org/doc/html/latest/devicetree/bindings/writing-schema.html

  **DT_SPLIT_BINDING_PATCH**
    璁惧鏍戠粦瀹氬簲褰撴槸瀹冧滑鑷繁鐨勭嫭绔嬭ˉ涓併€傝繖鏄洜涓?
    缁戝畾鍦ㄩ€昏緫涓婄嫭绔嬩簬椹卞姩瀹炵幇锛屽畠浠湁涓嶅悓鐨勭淮鎶よ€?
锛堝嵆浣块€氬父缁忕敱鍚屼竴妫垫爲鍚堝叆锛夛紝骞朵笖杩欐牱鑳借鐢?
    git-filter-branch 鍒涘缓鐨勭函 DT 鏍戞嫢鏈夋洿娓呮櫚鐨?
    鍘嗗彶璁板綍銆?

    See: https://www.kernel.org/doc/html/latest/devicetree/bindings/submitting-patches.html#i-for-patch-submitters

  **EMBEDDED_FILENAME**
    鍦ㄦ枃浠跺唴宓屽叆瀹屾暣鏂囦欢鍚嶈矾寰勫苟娌℃湁鐗瑰埆澶х殑鐢ㄥ锛?
    鍥犱负璺緞缁忓父琚Щ鍔ㄤ粠鑰屽彉寰椾笉姝ｇ‘銆?

  **FILE_PATH_CHANGES**
    姣忓綋娣诲姞銆佺Щ鍔ㄦ垨鍒犻櫎鏂囦欢鏃讹紝MAINTAINERS 鏂囦欢涓殑
    妯″紡鍙兘涓嶅悓姝ユ垨杩囨湡銆?

    鍥犳鍦ㄨ繖浜涙儏鍐典笅鍙兘闇€瑕佹洿鏂?MAINTAINERS銆?

  **MEMSET**
    memset 鐨勪娇鐢ㄤ技涔庝笉姝ｇ‘銆傝繖鍙兘鏄敱
    鍙傛暟椤哄簭閿欒瀵艰嚧銆傝閲嶆柊妫€鏌ョ敤娉曘€?

  **NOT_UNIFIED_DIFF**
    琛ヤ竵鏂囦欢浼间箮涓嶆槸 unified-diff 鏍煎紡銆傝
    鍦ㄥ彂閫佺粰缁存姢鑰呬箣鍓嶉噸鏂扮敓鎴愯ˉ涓佹枃浠躲€?

  **PLACEHOLDER_USE**
    妫€娴嬮仐鐣欏湪灏侀潰淇℃垨鎻愪氦澶?鏃ュ織涓€佹湭澶勭悊鐨勫崰浣嶇鏂囨湰銆?
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