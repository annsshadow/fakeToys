


## Coccinelle


Coccinelle 是一个用于模式匹配和文本转换的工具，在内核开发中有许多用途，包括应用复杂的、树
范围的补丁，以及检测有问题的编程模式。

### 获取 Coccinelle


内核中包含的语义补丁（semantic patch）使用了由 Coccinelle 1.0.0-rc11 及更高版本提供的特性
和选项。使用更早的版本会失败，因为 Coccinelle 文件和 coccicheck 所使用的选项名称已经更新。

Coccinelle 可通过许多发行版的包管理器获取，例如：

 - Debian
 - Fedora
 - Ubuntu
 - OpenSUSE
 - Arch Linux
 - Gentoo
 - NetBSD
 - FreeBSD

一些发行版打的包已经过时，建议使用从 Coccinelle 主页发布的最新版本：

https://coccinelle.gitlabpages.inria.fr/website

或从 Github 获取：

https://github.com/coccinelle/coccinelle

```

        ./autogen
        ./configure
        make

```

```

        sudo make install

```
从源码构建的更详细安装说明可以在以下位置找到：

https://github.com/coccinelle/coccinelle/blob/master/install.txt

### 补充文档


关于补充文档，请参阅 wiki：

https://bottest.wiki.kernel.org/coccicheck.html

wiki 文档始终指向该脚本的 linux-next 版本。

关于语义补丁语言（SmPL，Semantic Patch Language）语法文档，请参阅：

https://coccinelle.gitlabpages.inria.fr/website/docs/main_grammar.html

### 在 Linux 内核上使用 Coccinelle


顶层 Makefile 中定义了一个 Coccinelle 专用的目标。该目标名为 `coccicheck`，它会调用 `scripts`
目录中的 `coccicheck` 前端。

定义了四种基本模式：`patch`、`report`、`context` 和 `org`。要使用的模式通过 `MODE=<mode>`
设置 MODE 变量来指定。

- `patch` 在可能的情况下提出一个修复。

- `report` 生成如下格式的报告：
  file:line:column-column: message（文件:行:列-列: 消息）

- `context` 以类似 diff 的风格高亮感兴趣的行及其上下文。感兴趣的行用 `-` 标示。

- `org` 生成 Emacs 的 Org mode 格式的报告。

请注意，并非所有语义补丁都实现了所有模式。为了便于使用 Coccinelle，默认模式是 “report”。

另有两种模式提供了这些模式的常见组合。

- `chain` 按上述顺序尝试前面的模式，直到其中一个成功。

- `rep+ctxt` 依次运行 report 模式和 context 模式。它应与 C 选项（下文描述）一起使用，该选项以
  文件为单位检查代码。

#### 示例


```

		make coccicheck MODE=report

```

```

		make coccicheck MODE=patch


```
coccicheck 目标会把 `scripts/coccinelle` 子目录中可用的每个语义补丁应用到整个 Linux 内核。

对于每个语义补丁，都会提出一条提交信息。它描述了该语义补丁所检查的问题，并包含对 Coccinelle
的引用。

与任何静态代码分析器一样，Coccinelle 会产生误报（false positive）。因此，报告必须仔细检查，
补丁也必须经过审查。

```

   make coccicheck MODE=report V=1

```
默认情况下，coccicheck 会把调试日志打印到 stdout，并把 stderr 重定向到 /dev/null。这可能使
coccicheck 的输出难以阅读和理解。调试和错误消息也可以改为写入一个调试文件，通过

```

    make coccicheck MODE=report DEBUG_FILE="cocci.log"

```
Coccinelle 不能覆盖一个调试文件。与其反复删除日志，不如

```

    make coccicheck MODE=report DEBUG_FILE="cocci-$(date -Iseconds).log"

```
### Coccinelle 并行化


默认情况下，coccicheck 会尽量以并行方式运行。要改变这一点，可以使用

```

   make coccicheck MODE=report J=4

```
从 Coccinelle 1.0.2 起，Coccinelle 使用 Ocaml parmap 进行并行化；如果检测到对此的支持，你将
受益于 parmap 并行化。

当启用 parmap 时，coccicheck 会使用 `--chunksize 1` 参数来启用动态负载均衡。这确保我们一个一个
地持续向线程分发工作，从而避免大部分工作只由少数几个线程完成的情况。通过动态负载均衡，如果某个
线程提前完成，我们会持续向它分发更多工作。

当 parmap 启用时，如果 Coccinelle 中发生了错误，该错误值会被传播回来，并且 `make coccicheck`
命令的返回值会捕获这个返回值。

### 使用单个语义补丁运行 Coccinelle


可选的 make 变量 COCCI 可用于检查单个语义补丁。在这种情况下，该变量必须用要应用的语义补丁的
名字初始化。

```

	make coccicheck COCCI=<my_SP.cocci> MODE=patch

```

```

	make coccicheck COCCI=<my_SP.cocci> MODE=report


```
### 控制 Coccinelle 处理哪些文件


默认会检查整个内核源代码树。

要将 Coccinelle 应用到特定目录，可以使用 `M=`。

```

    make coccicheck M=drivers/net/wireless/

```
要以文件为单位（而非目录为单位）应用 Coccinelle，makefile 使用 C 变量来选择要处理的文件。该
变量可用于为整个内核、特定目录或单个文件运行脚本。

例如，要检查 drivers/bluetooth/bfusb.c，向 C 变量传入值 1 以检查 make 认为相关的文件

```

    make C=1 CHECK=scripts/coccicheck drivers/bluetooth/bfusb.o

```
向 C 变量传入值 2 以检查文件而不管其是否

```

    make C=2 CHECK=scripts/coccicheck drivers/bluetooth/bfusb.o

```
在这些以文件为单位工作的模式下，不会显示关于语义补丁的信息，也不会提出提交信息。

这默认运行 scripts/coccinelle 中的每个语义补丁。COCCI 变量也可额外用于仅应用单个语义补丁，
如上一节所示。

默认模式是 “report”。你可以用上文解释的 MODE 变量选择另一种模式。

### 调试 Coccinelle SmPL 补丁


使用 coccicheck 最好，因为它在 spatch 命令行中提供了与我们编译内核时所使用选项相匹配的
包含选项。你可以通过使用 V=1 来了解这些选项是什么；然后你就可以加上调试选项手动运行
Coccinelle。

调试针对 SmPL 补丁运行 Coccinelle 的一个更简单的方法，是让 coccicheck 把 stderr 重定向到
一个调试文件。如示例所述，默认 stderr 被重定向到 /dev/null；如果你想捕获 stderr，可以

```

    rm -f cocci.err
    make coccicheck COCCI=scripts/coccinelle/free/kfree.cocci MODE=report DEBUG_FILE=cocci.err
    cat cocci.err

```
你可以使用 SPFLAGS 添加调试标志；例如，在调试时你可能想向 SPFLAGS 同时添加 `--profile
--show-trying`。例如

```

    rm -f err.log
    export COCCI=scripts/coccinelle/misc/irqf_oneshot.cocci
    make coccicheck DEBUG_FILE="err.log" MODE=report SPFLAGS="--profile --show-trying" M=./drivers/mfd

```
err.log 现在将包含性能分析（profiling）信息，而 stdout 将随着 Coccinelle 推进工作提供一些
进度信息。

注意：

DEBUG_FILE 支持仅在 coccinelle >= 1.0.2 时可用。

目前，DEBUG_FILE 支持仅适用于检查文件夹，而不适用于单个文件。这是因为检查单个文件需要调用
spatch 两次，导致 DEBUG_FILE 两次都被设置为相同的值，从而产生错误。

### .cocciconfig 支持


Coccinelle 支持读取 .cocciconfig 以获取每次生成 spatch 时都应使用的默认 Coccinelle 选项。
.cocciconfig 中变量的优先顺序如下：

- 首先处理当前用户的主目录
- 接下来处理调用 spatch 所在目录
- 如果使用，最后处理通过 `--dir` 选项提供的目录

`make coccicheck` 也支持使用 M= 目标。如果你没有提供任何 M= 目标，则假定你想以整个内核为目标。

```

    OPTIONS="--dir $srcroot $COCCIINCLUDE"

```
这里，$srcroot 指的是目标的源代码目录：当使用 M= 时它指向外部模块的源代码目录，否则指向内核
源代码目录。第三条规则确保 spatch 从目标目录读取 .cocciconfig，从而允许外部模块拥有自己的
.cocciconfig 文件。

如果不使用内核的 coccicheck 目标，请保持上述 .cocciconfig 读取的优先顺序逻辑。如果使用内核的
coccicheck 目标，可通过 SPFLAGS 覆盖内核 .coccicheck 的任何设置。

我们在针对 Linux 使用 Coccinelle 时，通过我们自己的 Linux .cocciconfig 提供了一组合理的
Linux 默认选项，以提示 Coccinelle 可以使用 git 进行 `git grep` 查询（通过 coccigrep）。目前
200 秒的超时应该足够了。

Coccinelle 在读取 .cocciconfig 时拾取的选项不会作为运行在你系统上的 spatch 进程的参数出现。
要确认实际使用了哪些选项，可以

```

      spatch --print-options-only

```
你可以通过使用 SPFLAGS 覆盖为你自己偏好的索引选项。请注意，当存在冲突选项时，Coccinelle 会优先
采用最后传入的选项。使用 .cocciconfig 也可以使用 idutils，不过鉴于 Coccinelle 遵循的优先顺序，
由于内核现在带有自己的 .cocciconfig，如果需要使用 idutils，你将必须使用 SPFLAGS。更多关于如何
使用 idutils 的细节，请参阅下文 “Additional flags（附加标志）” 一节。

### 附加标志


可以通过 SPFLAGS 变量向 spatch 传递附加标志。这可以工作，因为 Coccinelle 会遵循最后传入的标志

```

    make SPFLAGS=--use-glimpse coccicheck

```
Coccinelle 也支持 idutils，但需要 coccinelle >= 1.0.6。当没有指定 ID 文件时，Coccinelle 假定
你的 ID 数据库文件位于内核顶层的 .id-utils.index 文件中。Coccinelle

```

    mkid -i C --output .id-utils.index

```
如果你有另一个数据库文件名，也可以直接通过如下方式使用符号链接

```

    make SPFLAGS=--use-idutils coccicheck

```
或者你也可以显式指定数据库文件名

```

    make SPFLAGS="--use-idutils /full-path/to/ID" coccicheck

```
参见 `spatch --help` 以了解更多关于 spatch 选项的信息。

请注意，`--use-glimpse` 和 `--use-idutils` 选项需要外部工具来为代码建立索引。因此它们默认都不
激活。然而，通过使用这些工具之一为代码建立索引，并根据所使用的 cocci 文件，spatch 可以更快地
处理整个代码库。

### SmPL 补丁专有选项


SmPL 补丁可以对自己传给 Coccinelle 的选项有要求。SmPL 补丁专有选项可以通过如下方式提供

```

	// Options: --no-includes --include-headers

```
### SmPL 补丁的 Coccinelle 版本要求


随着 Coccinelle 特性不断增加，一些更高级的 SmPL 补丁可能需要更新版本的 Coccinelle。如果一个
SmPL 补丁要求最低版本的 Coccinelle，可以如下指定

```

	// Requires: 1.0.5

```
### 提出新的语义补丁


内核开发者可以提出并提交新的语义补丁。为了清晰起见，它们应当组织在 `scripts/coccinelle/` 的
子目录中。


### ``report`` 模式的详细说明


```

  file:line:column-column: message

```
#### 示例


```

	make coccicheck MODE=report COCCI=scripts/coccinelle/api/err_cast.cocci

```

```

   <smpl>
   @r depends on !context && !patch && (org || report)@
   expression x;
   position p;
   @@

     ERR_PTR@p(PTR_ERR(x))

   @script:python depends on report@
   p << r.p;
   x << r.x;
   @@

   msg="ERR_CAST can be used with %s" % (x)
   coccilib.report.print_report(p[0], msg)
   </smpl>

```
这段 SmPL 摘录在标准输出上生成如下条目

```

    /home/user/linux/crypto/ctr.c:188:9-16: ERR_CAST can be used with alg
    /home/user/linux/crypto/authenc.c:619:9-16: ERR_CAST can be used with auth
    /home/user/linux/crypto/xts.c:227:9-16: ERR_CAST can be used with alg


```
### ``patch`` 模式的详细说明


当 `patch` 模式可用时，它会为每个识别出的问题提出一个修复。

#### 示例


```

	make coccicheck MODE=patch COCCI=scripts/coccinelle/api/err_cast.cocci

```

```

    <smpl>
    @ depends on !context && patch && !org && !report @
    expression x;
    @@

    - ERR_PTR(PTR_ERR(x))
    + ERR_CAST(x)
    </smpl>

```
这段 SmPL 摘录在标准输出上生成补丁块（patch hunk），如下所示

```

    diff -u -p a/crypto/ctr.c b/crypto/ctr.c
    --- a/crypto/ctr.c 2010-05-26 10:49:38.000000000 +0200
    +++ b/crypto/ctr.c 2010-06-03 23:44:49.000000000 +0200
    @@ -185,7 +185,7 @@ static struct crypto_instance *crypto_ct
 	alg = crypto_attr_alg(tb[1], CRYPTO_ALG_TYPE_CIPHER,
 				  CRYPTO_ALG_TYPE_MASK);
 	if (IS_ERR(alg))
    -		return ERR_PTR(PTR_ERR(alg));
    +		return ERR_CAST(alg);

 	/* Block size must be >= 4 bytes. */
 	err = -EINVAL;

```
### ``context`` 模式的详细说明


`context` 以类似 diff 的风格高亮感兴趣的行及其上下文。

      **注意**：生成的类似 diff 的输出并不是一个可应用的补丁。`context` 模式的意图是
      高亮重要的行（用减号 `-` 标注），并给出周围的一些上下文行。这个输出可以和
      Emacs 的 diff 模式一起用来审查代码。

#### 示例


```

	make coccicheck MODE=context COCCI=scripts/coccinelle/api/err_cast.cocci

```

```

    <smpl>
    @ depends on context && !patch && !org && !report@
    expression x;
    @@

    * ERR_PTR(PTR_ERR(x))
    </smpl>

```
这段 SmPL 摘录在标准输出上生成 diff 块（diff hunk），如下所示

```

    diff -u -p /home/user/linux/crypto/ctr.c /tmp/nothing
    --- /home/user/linux/crypto/ctr.c	2010-05-26 10:49:38.000000000 +0200
    +++ /tmp/nothing
    @@ -185,7 +185,6 @@ static struct crypto_instance *crypto_ct
 	alg = crypto_attr_alg(tb[1], CRYPTO_ALG_TYPE_CIPHER,
 				  CRYPTO_ALG_TYPE_MASK);
 	if (IS_ERR(alg))
    -		return ERR_PTR(PTR_ERR(alg));

 	/* Block size must be >= 4 bytes. */
 	err = -EINVAL;

```
### ``org`` 模式的详细说明


`org` 生成 Emacs 的 Org mode 格式的报告。

#### 示例


```

	make coccicheck MODE=org COCCI=scripts/coccinelle/api/err_cast.cocci

```

```

    <smpl>
    @r depends on !context && !patch && (org || report)@
    expression x;
    position p;
    @@

      ERR_PTR@p(PTR_ERR(x))

    @script:python depends on org@
    p << r.p;
    x << r.x;
    @@

    msg="ERR_CAST can be used with %s" % (x)
    msg_safe=msg.replace("[","@(").replace("]",")")
    coccilib.org.print_todo(p[0], msg_safe)
    </smpl>

```
这段 SmPL 摘录在标准输出上生成 Org 条目，如下所示

```

    * TODO [[view:/home/user/linux/crypto/ctr.c::face=ovl-face1::linb=188::colb=9::cole=16][ERR_CAST can be used with alg]]
    * TODO [[view:/home/user/linux/crypto/authenc.c::face=ovl-face1::linb=619::colb=9::cole=16][ERR_CAST can be used with auth]]
    * TODO [[view:/home/user/linux/crypto/xts.c::face=ovl-face1::linb=227::colb=9::cole=16][ERR_CAST can be used with alg]]

```
