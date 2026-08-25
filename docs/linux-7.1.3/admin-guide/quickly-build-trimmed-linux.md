
## 如何快速构建一个精简Linux 内核


本指南讲解如何快速构建非常适合测试、同时也完全可以用于日常使用Linux 内核
## 过程精髓（即"太长不看"版）


*[如果你刚接触编译 Linux，请忽略这段 TL;DR，直接跳到下面的一节：那里有一份逐步指南它更详细，但仍然简洁、易于跟随；该指南及其所附的参考一节还提到了各种替代方案、陷阱和
补充方面，这些都可能与你相关。]*

如果你的系统使用Secure Boot 之类的技术，请先准备好允许启动自己编译的 Linux 内核安装编译器以及构Linux 所需的其他一切；确保在你home 目录下有 12 GB 的空闲空间现在运行以下命令以下载最新的 Linux 主线源代码：

```
    git clone --depth 1 -b master \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git ~/linux/
    cd ~/linux/
    # 提示：如果你想打补丁，请在此处进行。详见下文    # 提示：建议在此处为你的构建打上标签。详见下文    yes "" | make localmodconfig
    # 提示：此时你可能需要调整构建配置；如果你运行的Debian，就必须要调。详见下文    make -j $(nproc --all)
    # 注意：在许多常见发行版上下一条命令就足够了，但在 Arch Linux 及其衍生版和某些
    #   其他发行版上并非如此。详见下文    command -v installkernel && sudo make modules_install install
    reboot
```

要为以后的构建更新代码，请使用这些命令：

```
    cd ~/linux/
    git fetch --depth 1 origin
    # 注意：下一条命令会丢弃你对代码所做的任何修改    git checkout --force --detach origin/master
    # 提醒：如果你想（重新）打补丁，请在此处进行    # 提醒：你可能想在此处添加或修改一个构建标签    make olddefconfig
    make -j $(nproc --all)
    # 提醒：下一条命令在某些发行版上并不足够    command -v installkernel && sudo make modules_install install
    reboot
```

## 逐步指南


自己编译 Linux 内核原则上很简单。有各种不同的方式来做这件事。其中哪些真正可行、哪最好，取决于具体环境
本指南描述的方法非常适合那些想从源代码快速安Linux、而不想被复杂细节困扰的人；其目标
是覆盖在商品 PC 或服务器硬件上运行的主流 Linux 发行版上通常所需的一切
所描述的方法非常适合测试目的，例如尝试一个提议的修复，或检查某个问题在最新的代码库中否已经被修复。尽管如此，用这种方式构建的内核也完全可以用于日常使用，同时又易于保持更新
以下步骤描述了该过程的重要方面；后面一个全面的参考一节会更详细地解释其中每一项。它有时
也描述了替代方案、陷阱，以及可能在某个特定点发生的错误——以及如何让事情重新运转起来
..
   Note: if you see this note, you are reading the text's source file. You
   might want to switch to a rendered version, as it makes it a lot easier to
   quickly look something up in the reference section and afterwards jump back
   to where you left off. Find a the latest rendered version here:
   https://docs.kernel.org/admin-guide/quickly-build-trimmed-linux.html


 - 创建一个全新的备份，并把系统修复和恢复工具放在手边，以防万一出现意外情况
   [details<backup>]


 - 在使'Secure Boot' 或类似技术的平台上，准备好一切，确保系统以后会允许你自编译的
   内核启动。在商品 x86 系统上实现这一点最快最简单的方法是在 BIOS 设置工具中禁用此类技术；
   或者通过`mokutil --disable-validation` 发起的流程来移除它们的限制
   [details<secureboot>]


 - 安装构建 Linux 内核所需的所有软件。通常你需要：'bc'binutils'ld' 等）bison'   'flex'gcc'git'openssl'pahole'perl'，以'libelf' 'openssl' 的开   头文件。参考一节展示了如何在各种流行的 Linux 发行版上快速安装它们
   [details<buildrequires>]


 - 确保有足够的空闲空间用于构建和安Linux。对于后者，/lib/ 150 MB /boot/ 100 MB
   是一个稳妥的估计。对于存放源代码和构建产物，你的 home 目录12 GB 通常足够。如果你可用
   空间更少，务必查阅参考一节中关于调整内核构建配置的那一步：它提到一个能/home/ 下所需
   空间减少到约 4 GB 的技巧
   [details<diskspace>]


 - 获取你想要构建的 Linux 版本的源代码；然后切换到保存它们的目录，因为本指南中所有后续命   都打算从该目录执行
   *[Note: the following paragraphs describe how to retrieve the sources by
   partially cloning the Linux stable git repository. This is called a shallow
   clone. The reference section explains two alternatives:* :ref:`packaged
   archives<sources_archive>` **and** a full git clone<sources_full> *;
   prefer the latter, if downloading a lot of data does not bother you, as that
   will avoid some* :ref:`peculiar characteristics of shallow clones the
   reference section explains<sources_shallow>` **.]**

```
     git clone --no-checkout --depth 1 -b master \
       https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git ~/linux/
     cd ~/linux/

   If you want to access recent mainline releases and pre-releases, deepen you
   clone's history to the oldest mainline version you are interested in::

     git fetch --shallow-exclude=v6.0 origin

   In case you want to access a stable/longterm release (say v6.1.5), simply add
   the branch holding that series; afterwards fetch the history at least up to
   the mainline version that started the series (v6.1)::

     git remote set-branches --add origin linux-6.1.y
     git fetch --shallow-exclude=v6.0 origin

   Now checkout the code you are interested in. If you just performed the
   initial clone, you will be able to check out a fresh mainline codebase, which
   is ideal for checking whether developers already fixed an issue::

      git checkout --detach origin/master

   If you deepened your clone, you instead of ``origin/master`` can specify the
   version you deepened to (``v6.0`` above); later releases like ``v6.1`` and
   pre-release like ``v6.2-rc1`` will work, too. Stable or longterm versions
   like ``v6.1.5`` work just the same, if you added the appropriate
   stable/longterm branch as described.

   [:ref:`details<sources>`]
```

 - 如果你想应用一个内核补丁，现在就做。通常一条这样的命令
```
     patch -p1 < ../proposed-fix.patch

   If the ``-p1`` is actually needed, depends on how the patch was created; in
   case it does not apply thus try without it.

   If you cloned the sources with git and anything goes sideways, run ``git
   reset --hard`` to undo any changes to the sources.

   [:ref:`details<patching>`]
```

 - 如果你为内核打了补丁，或者已经安装了一个相同版本的内核
```
     echo "-proposed_fix" > localversion

   Running ``uname -r`` under your kernel later will then print something like
   '6.1-rc4-proposed_fix'.

   [:ref:`details<tagging>`]
```

.. _configuration_sbs:

* 基于现有配置为你的内核创建构建配置
  如果你自己已经准备好了这样一'.config' 文件，把它复制到 ~/linux/ 并运  ``make olddefconfig``
  如果你的发行版或别人已经把正在运行的内核针对你或你的硬件需求裁剪过：那make 目标
  'olddefconfig' 会尝试以该内核的 .config 作为基础
  这个 make 目标对其他人也适用——但你经常可以通过改用这条命令来节省大量时间：

```
     yes "" | make localmodconfig
```

  它会尝试以你的发行版内核为基础，但随后会为你配置中明显多余的功能禁用模块。这将极大地
  减少编译时间，尤其是当你运行的是一个来自商Linux 发行版的通用内核时
  这里有一个坑localmodconfig' 很可能禁用你自启Linux 以来没有使用过的内核功能——比  当前未连接的外部设备的驱动，或你尚未使用过的虚拟化软件。你可以用参考一节概述的技巧来减少
  甚至几乎消除这种风险；但在仅为快速测试目的构建内核时，这些功能缺失通常无伤大雅。不过在使用
  用这make 目标构建的内核时，你应当把这一点记在心里，因为它可能是你偶尔才用的东西停止
  工作原因
   [:ref:`details<configuration>`]

```
 - 检查你是否想或必须调整一些内核配置选项
  - 考虑如何处理调试符号。如果你以后可能需要解码一个例如在 'panic'Oops'warning'     'BUG' 中找到的堆栈跟踪，就启用它们；反之，如果你存储空间紧张或更喜欢更小的内核二进制文件，
    就禁用它们。关于如何做这两者的细节，请参阅参考一节。如果两者都不适用，简单地不去管它多半
    也无妨。[details<configmods_debugsymbols>]

  - 你运行的Debian 吗？那么请执行参考一节中解释的额外调整，以避免已知问题    [details<configmods_distros>]
  - 如果你想影响配置的其他方面，现在就通过 'menuconfig' 'xconfig' 之类make 目标来做    [details<configmods_individual>]```

 - 编译内核
```
     make -j $(nproc --all)

   If you want your kernel packaged up as deb, rpm, or tar file, see the
   reference section for alternatives.

   [:ref:`details<build>`]
```

 - 安装内核
```
     command -v installkernel && sudo make modules_install install

   Often all left for you to do afterwards is a ``reboot``, as many commodity
   Linux distributions will then create an initramfs (also known as initrd) and
   an entry for your kernel in your bootloader's configuration; but on some
   distributions you have to take care of these two steps manually for reasons
   the reference section explains.

   On a few distributions like Arch Linux and its derivatives the above command
   does nothing at all; in that case you have to manually install your kernel,
   as outlined in the reference section.

   If you are running an immutable Linux distribution, check its documentation
   and the web to find out how to install your own kernel there.

   [:ref:`details<install>`]
```

 - 以后要构建另一个内核，你需要类似的步骤，但有时命令略有不同
```
      cd ~/linux/

   In case you want to build a version from a stable/longterm series you have
   not used yet (say 6.2.y), tell git to track it::

      git remote set-branches --add origin linux-6.2.y

   Now fetch the latest upstream changes; you again need to specify the earliest
   version you care about, as git otherwise might retrieve the entire commit
   history::

     git fetch --shallow-exclude=v6.0 origin

   Now switch to the version you are interested in -- but be aware the command
   used here will discard any modifications you performed, as they would
   conflict with the sources you want to checkout::

     git checkout --force --detach origin/master

   At this point you might want to patch the sources again or set/modify a build
   tag, as explained earlier. Afterwards adjust the build configuration to the
   new codebase using olddefconfig, which will now adjust the configuration file
   you prepared earlier using localmodconfig  (~/linux/.config) for your next
   kernel::

     # reminder: if you want to apply patches, do it at this point
     # reminder: you might want to update your build tag at this point
     make olddefconfig

   Now build your kernel::

     make -j $(nproc --all)

   Afterwards install the kernel as outlined above::

     command -v installkernel && sudo make modules_install install

   [:ref:`details<another>`]
```

 - 你的内核以后很容易移除，因为它的各个部分只存放在两个地方，并且可以通过内核的发行名清晰
   识别。只要确保不要删除你正在运行的内核，因为那可能使你的系统无法启动
   首先删除保存你内核模块的目录，它命名为：

```
     sudo rm -rf /lib/modules/6.0.1-foobar
```

  现在试一下下面这条命令，它在一些发行版上会删除安装的所有其他内核文件，同时bootloader
  配置中移除该内核的条目：

```
     command -v kernel-install && sudo kernel-install -v remove 6.0.1-foobar
```

  如果那条命令没有任何输出或失败，请参阅参考一节；如果/boot/ 中仍然有任何名为
  '*6.0.1-foobar*' 的文件，也这么做
   [:ref:`details<uninstall>`]


按照逐步指南操作时遇到了参考一节也没能解决的麻烦吗？你发现了错误吗？或者你对如何改进本指南
有想法吗
如果以上任意情况适用，请通过Thorsten Leemhuis <linux@leemhuis.info> 发送简短说明或补丁最好同时抄送公开Linux 文档邮件列表 <linux-doc@vger.kernel.org>，让开发者知道。这样的反馈
对进一步改进本文至关重要，这符合每个人的利益，因为它能让更多人掌握此处描述的任务
## 逐步指南参考一

本节保存了上述指南中每一步的附加信息

### 为紧急情况做准备


   **Create a fresh backup and put system repair and restore tools at hand**
   [... <backup_sbs>]

记住，你正在与计算机打交道，计算机有时会发生意外——尤其是当你摆弄像操作系统内核这样关键的部分时而这正是你在此过程中要做的事情。因此，最好为出现意外做好准备，即使它本不应该发生
[back to step-by-step guide <backup_sbs>]


### 应对 Secure Boot 之类的技

   *On platforms with 'Secure Boot' or similar techniques, prepare everything to
   ensure the system will permit your self-compiled kernel to boot later.*
   [... <secureboot_sbs>]

许多现代系统只允许某些操作系统启动；因此默认情况下它们会拒绝启动自编译的内核
最理想的做法是借助证书和签名让你的平台信任你自构建的内核。如何做到这一点这里不描述，因为这需多个步骤，会使本文偏离其目的太远Documentation/admin-guide/module-signing.rst' 以及多个网页
已经对此做了更详细的说明
临时禁用 Secure Boot 之类的方案是让你自己Linux 启动的另一种方式。在商品 x86 系统上，可以BIOS 设置工具中做到这一点；具体步骤这里不描述，因为它们在不同机器之间差异很大
在主x86 Linux 发行版上，还有第三种选择且是通用的：为你Linux 环境禁用所Secure Boot
限制。你可以通过运行 `mokutil --disable-validation` 来发起此流程；它会提示你创建一个一次性密码，
把它写下来是安全的。现在重启；在你BIOS 完成所有自检之后，bootloader Shim 会立即显示一蓝色方框，上面有一条消Press any key to perform MOK management"。在倒计时结束前按某个键。这打开一个菜单，在其中选择"Change Secure Boot state"。Shim "MokManager" 现在会要求你输入之前
一次性密码中随机选出的三个字符。一旦提供，确认你确实想要禁用校验。之后，允许 MokManager 重启机器
[back to step-by-step guide <secureboot_sbs>]


### 安装构建需

   **Install all software required to build a Linux kernel.**
   [...<buildrequires_sbs>]

内核相当独立，但除了编译器之类的工具之外，有时你还需要几个库来构建它。如何安装所需的一切取决于
你的 Linux 发行版以及你将要构建的内核的配置
以下是一些主流发行版上你通常需要的例子
```
     sudo apt install bc binutils bison dwarves flex gcc git make openssl \
       pahole perl-base libssl-dev libelf-dev

 * Fedora and derivatives::

     sudo dnf install binutils /usr/include/{libelf.h,openssl/pkcs7.h} \
       /usr/bin/{bc,bison,flex,gcc,git,openssl,make,perl,pahole}

 * openSUSE and derivatives::

     sudo zypper install bc binutils bison dwarves flex gcc git make perl-base \
       openssl openssl-devel libelf-dev
```

如果你想知道为什么这些列表包openssl 及其开发头文件：它们是 Secure Boot 支持所需要的，许发行版在x86 机器的内核配置中启用了它
有时你也需要诸bzip2、gzip、lz4、lzma、lzo、xz zstd 等压缩格式的工具
如果你执行本指南未涵盖的任务，可能需要额外的库及其开发头文件。例如，tools/ 目录构建内核工具
时需zlib；用 'menuconfig' 'xconfig' 之类make 目标调整构建配置将需ncurses Qt5
的开发头文件
[back to step-by-step guide <buildrequires_sbs>]


### 空间需

   **Ensure to have enough free space for building and installing Linux.**
   [... <diskspace_sbs>]

提到的数字是在留足余量以保证安全前提下的粗略估计，所以通常你需要得更少
如果你空间受限，记得在到达关于配置调整的 <configmods> 一节时阅读参考一节，因为确保禁用调试符号
会把消耗的磁盘空间减少好几 GB
[back to step-by-step guide <diskspace_sbs>]


### 下载源代

  **Retrieve the sources of the Linux version you intend to build.**
  [...<sources_sbs>]

逐步指南概述了如何使用浅克隆（shallow clone）来获取 Linux 的源代码。关于这种方法还有更多可说的
<sources_shallow>，并且还有两种值得一提的替代方式：打包归<sources_archive> 和完git 克隆
<sources_full>。以是否使用适当的预发布版本而非最新的主线代码更明<sources_snapshot>"如何获取更新鲜的主线代码<sources_fresher>"这两个方面也需要阐述
注意，为简单起见，本指南中使用的命令把构建产物存放在源代码树中。如果你倾向于把它们分开，只需所make 调用中加上类`O=~/linux-builddir/` 的内容；同时调整所有添加文件或修改任何生成文件
（如你的 '.config'）的命令中的路径
[back to step-by-step guide <sources_sbs>]


#### 浅克隆值得注意的特

逐步指南使用了浅克隆，因为它是本文档目标受众大多数情况下的最佳方案。这种方式有几个方面值得一提：

 - 本文档在大多数地方使`git fetch` 配合 `--shallow-exclude=` 来指定你关心的最早版本（更准   地说：它git 标签）。你也可以改用参`--shallow-since=` 来指定一个绝对的（比`'2023-07-15'`   或相对的（`'12 months'`）日期，以定义你想要下载的历史深度。作为第二种替代，你也可以显式指   某个深度，使用类`--depth=1` 的参数，除非你为 stable/longterm 内核添加了分支
 - 运行 `git fetch` 时，记得始终像逐步指南中那样指定最早的版本、你关心的时刻，或显式的深度。否   你将面临下载几乎整个 git 历史的风险，这会消耗相当多的时间和带宽，同时也会给服务器带来压力
   注意，你不一定要始终使用相同的版本或日期。但当你随着时间的推移改变它时，git 会把历史加深   压扁到指定的点。这让你能够获取你最初以为不需要的版本——或者它会丢弃较旧版本的源代码，例如当你
   想释放一些磁盘空间时。后者在使用`--shallow-since=` `--depth=` 时会自动发生
 - 警告：当加深你的克隆时，你可能会遇到类似
   'fatal: error in object: unshallow cafecaca0c0dacafecaca0c0dacafecaca0c0da' 的错误   在这种情况下运行 `git repack -d` 然后再试一次``

 - 如果你想回退某个版本的改动（比如 Linux 6.3）或进行二分（v6.2..v6.3），最好让 `git fetch` 获取
   早至三个版本之前（比6.0）的对象：`git describe` 之后就能像在完整 git 克隆中一样描述大多数提交
[back to step-by-step guide <sources_sbs>] [back to section intro <sources>]


#### 使用打包归档下载源代

刚接触编Linux 的人常常以为通过 https://kernel.org 的首页下载归档是获取 Linux 源代码的最佳方法在某些情况下确实如此，如果你确定只构建一个特定内核版本且不改动任何代码的话。问题是：你可能确信如此，但在实践中这常常被证明是一个错误的假设
这是因为当报告或调试问题时，开发者常常会要求尝试另一个版本。他们也可能建议`git revert` 临时
撤销某个提交，或可能提供各种补丁来尝试。有时报告者也会被要求使用 `git bisect` 来找出导致问题的
改动。这些事情都依赖 git，或者有git 会容易和快捷得多
浅克隆也不会增加任何显著开销。例如，当你使用 `git clone --depth=1` 来创建一个最新主线代码库的浅克隆
时，git 只会比通过 kernel.org 首页下载最新的主线预发布版（即 'rc'）多取一点点数据
因此浅克隆通常是更好的选择。尽管如此，如果你还是想使用打包的源代码归档，请通过 kernel.org 下载一个；
之后把其内容解压到某个目录并切换到解压时创建的子目录。逐步指南的其余部分都照常工作，除了依git
的部分——但这主要涉及连续构建其他版本的那一节
[back to step-by-step guide <sources_sbs>] [back to section intro <sources>]


#### 使用完整 git 克隆下载源代

如果你不在意下载和存储大量数据（截至 2023 年初4.4 GB），那就执行完整 git 克隆，而不是浅克隆这样你会避免上述的特殊之处，并拥有所有：

```
    curl -L \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/clone.bundle \
      -o linux-stable.git.bundle
    git clone linux-stable.git.bundle ~/linux/
    rm linux-stable.git.bundle
    cd ~/linux/
    git remote set-url origin \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git
    git fetch origin
    git checkout --detach origin/master
```

[back to step-by-step guide <sources_sbs>] [back to section intro <sources>]


#### 恰当的预发布版本（RC）与最新主

当使git 克隆源代码并检origin/master 时，你常常获取的代码库处于最新版本与下一个发布或预发版本之间。这几乎总是你给主线一个机会时想要的代码：v6.1-rc5 这样的预发布版并不特殊，因为它们
在发布前不会获得任何显著的额外测试
有一个例外：你可能想在其后继者的第一个预发布版本（v6.2-rc1）发布之前，坚持使用最新的主线发布（比v6.1）。这是因为在此期间编译错误和其他问题更有可能发生，因为此时主线处于它合并窗口"
（merge window）：一个通常为期两周的阶段，其间为下一个发布版本所做的绝大部分改动会被合并
[back to step-by-step guide <sources_sbs>] [back to section intro <sources>]


#### 避免主线滞后


对浅克隆和完整克隆的解释都是Linux stable git 仓库获取代码。这对本文档的读者来说更简单，因为允许轻松访问主线stable/longterm 发布版本。这种方式只有一个缺点：

合并到主线仓库的改动只每隔几小时同步Linux stable 仓库master 分支。这种滞后在大多数时不值得担心；但如果你真的需要最新代码，只需
```
    git remote add mainline \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git
    git fetch mainline
    git checkout --detach mainline/master
```

在浅克隆上这样做时，记得用前面描述的某个参数调用 `git fetch` 来限制深度
[back to step-by-step guide <sources_sbs>] [back to section intro <sources>]


### 给源代码打补丁（可选）


  **In case you want to apply a kernel patch, do so now.**
  [...<patching_sbs>]

这正是你可能想为内核打补丁的地方——例如，当某个开发者提出了一个修复，并请你检查它是否有帮助时逐步指南已经解释了这里所有关键的内容
[back to step-by-step guide <patching_sbs>]


### 为此内核构建打标签（可选，通常明智

  *If you patched your kernel or already have that kernel version installed,
  better tag your kernel by extending its release name:*
  [...<tagging_sbs>]

为你的内核打标签有助于避免以后混淆，尤其是当你为内核打了补丁时。添加一个独立的标签还将确保内核映像及其模块与任何现有内核并行安装
有多种方式添加这样的标签。逐步指南通过在你构建目录中创建一'localversion' 文件来实现其中一种，
内核构建脚本会自动从中获取该标签。你以后可以修改该文件以在后续构建中使用不同的标签，或简单地删除
该文件以丢弃标签
[back to step-by-step guide <tagging_sbs>]


### 为你的内核定义构建配

  *Create the build configuration for your kernel based on an existing
  configuration.* [... <configuration_sbs>]

这一步有几个方面需要更仔细的解释：


#### 使用另一个配置文件作为基础时的陷阱


make 目标localmodconfig olddefconfig 有一些共同的陷阱，你应当了解
 - 这些目标会复用你构建目录中已有的内核构建配置（例'~/linux/.config'），如果存在的话。因   如果你想从零开始，就需要删除它
 - make 目标会尝试自动找到你正在运行的内核的配置，但可能选得不好。一行像
   '# using defaults found in /boot/config-6.0.7-250.fc36.x86_64'    'using config: /boot/config-6.0.7-250.fc36.x86_64' 会告诉你它们选了哪个文件。如果那不是想要   那个，只需在使用这make 目标之前把它存为 '~/linux/.config'
 - 如果你试图把一个为某个内核（比v6.0）准备的配置文件用在一个更老的代际（比v5.15）上，可能会
   发生意想不到的事情。在这种情况下，你可能想使用一个你的发行版在使用那个或稍老的内核版本时所用过   配置作为基础

#### 影响配置


make 目标 olddefconfig 以及使用 localmodconfig 时的 `yes "" |` 会把任何未定义的构建选项设为它们默认值。这其中包括会禁用许多在你的基础内核发布之后才引入的内核功能
如果你想手动设置这些配置选项，请使用 `oldconfig` 而非 `olddefconfig`，或者在使用 localmodconfig 省略 `yes "" |`。然后对于每个未定义的配置选项，你都会被问到如何进行。如果你不确定该如何回答，只需
'enter' 应用默认值

#### 使用 localmodconfig 时的大坑


正如逐步指南中已经简要解释的：使localmodconfig 时，很容易发生你自构建的内核缺少你在使用这个 make
目标之前没有执行过的任务所需的模块。这是因为那些任务需要的内核模块通常会在你第一次执行该任务时自加载；如果你在使localmodconfig 之前至少执行过一次该任务，后者就会假定这些模块是多余的而禁用它们
你可以通过执行那些常常会自加载额外内核模块的典型任务来尽量避免这一点：启动一个虚拟机、建VPN 连接回环挂载一CD/DVD ISO、挂载网络共享（CIFS、NFS……），以及连接所有外部设备（2FA 密钥、头戴式
耳机、网络摄像头……）以及你平时不使用的文件系统（btrfs、ext4、FAT、NTFS、XFS……）的存储设备。但很难
想到一切可能需要的——即便是内核开发者在这个点上也常常忘记这或那
不要让这种风险困扰你，尤其是在仅为测试目的编译内核时：所有通常关键的东西都会在那里。而且如果你忘某些重要的东西，以后可以打开缺失的功能，并快速运行命令来编译和安装一个更好的内核
但如果你打算定期构建和使用自构建的内核，你可能想通过记录你的系统在几周过程中加载了哪些模块来降低风险你可以用 `modprobed-db <https://github.com/graysky2/modprobed-db>`_ 把它自动化。之后使`LSMOD=<path>` 来：

```
    yes "" | make LSMOD="${HOME}"/.config/modprobed.db localmodconfig
```

#### localmodconfig 进行远程构建


如果你想localmodconfig 为另一台机器构建内核，在它上面运行 `lsmod > lsmod_foo-machine` 并把该文传输到你的构建主机。现在像这样把构建脚本指向该文件：``yes "" | make LSMOD=~/lsmod_foo-machine
localmodconfig``。注意，在这种情况下你可能也想从另一台机器复制一份基础内核配置过来，并把它作为 .config
放在你的构建目录中
[back to step-by-step guide <configuration_sbs>]


### 调整构建配置


   *Check if you might want to or have to adjust some kernel configuration
   options:*

根据你的需求，在这一点上你可能想或必须调整一些内核配置选项

#### 调试符号


   **Evaluate how you want to handle debug symbols.**
   [...<configmods_sbs>]

大多数用户不需要关心这个，通常保持原样就好；但如果你可能需要解码一个堆栈跟踪，或想减少空间占用，你
应当更仔细地看一下
当你的内核以后运行时抛出 'panic'Oops'warning' 'BUG' 时，拥有可用的调试符号可能很重要因为那时你将能够找到问题在代码中发生的确切位置。但收集和嵌入所需的调试信息需要时间并消耗相当多空间：在 2022 年末，用 localmodconfig 配置的典x86 内核的构建产物在开启调试符号时消耗约 5 GB 空间而在禁用时不1 GB。由此产生的内核映像和模块也更大，从而增加了加载时间
因此，如果你想要一个小的内核，并且不太可能解码堆栈跟踪
```
    ./scripts/config --file .config -d DEBUG_INFO \
      -d DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT -d DEBUG_INFO_DWARF4 \
      -d DEBUG_INFO_DWARF5 -e CONFIG_DEBUG_INFO_NONE
    make olddefconfig
```

另一方面，如果你以后很可能需要解码堆栈跟踪（Documentation/admin-guide/tainted-kernels.rst 中的
"Decode failure messages" 所解释的），你绝对想启用它们：

```
    ./scripts/config --file .config -d DEBUG_INFO_NONE -e DEBUG_KERNEL
      -e DEBUG_INFO -e DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT -e KALLSYMS -e KALLSYMS_ALL
    make olddefconfig
```

注意，许多主流发行版在其内核配置中启用了调试符号——因此像 localmodconfig olddefconfig 这样make
目标常常会采用该设置
[back to step-by-step guide <configmods_sbs>]


#### 发行版特定的调整


   **Are you running** [... <configmods_sbs>]

以下几节帮助你避免在本指南若干商品发行版上已知会发生的构建问题

**Debian:**

 - 移除对一个证书文件的陈旧引用，它会让你的构建
```
    ./scripts/config --file .config --set-str SYSTEM_TRUSTED_KEYS ''
```

   或者，下载所需证书并让该配置选项指向它，正如 `the Debian handbook explains in more detail
   <https://debian-handbook.info/browse/stable/sect.kernel-compilation.html>`_ 所详述——或者生   你自己的，如 Documentation/admin-guide/module-signing.rst 中所述
[back to step-by-step guide <configmods_sbs>]


#### 单独调整


   *If you want to influence the other aspects of the configuration, do so
   now* [... <configmods_sbs>]

在这一点上你可以使用像 `make menuconfig` 这样的命令，通过基于文本的界面来启用或禁用某些功能；要使图形化配置，请改make 目标 `xconfig` `gconfig`。它们都需要其所基于工具包（ncurses、Qt5、Gtk2的开发库；如果缺失所需之物，一条错误消息会告诉你
[back to step-by-step guide <configmods_sbs>]


### 构建你的内核


  **Build the image and the modules of your kernel** [... <build_sbs>]

这个阶段可能出错的地方很多，但下面的说明会帮助你自助。另一个小节讲解如何直接把你的内核打包debrpm tar 文件

#### 处理构建错误


当发生构建错误时，它可能是由你机器设置的某些方面引起的，通常可以快速修复；但其他时候问题出在代中，只能由开发者修复。仔细检查失败消息，再加上一些互联网上的研究，通常会告诉你这两者属于哪一种。要
进行这样的调查，重新启动构建
```
    make V=1
```

`V=1` 会激活详细输出，这可能是看清实际错误所必需的。为了让它更容易被注意到，这条命令也省略了之用于利用系统中每CPU 核心``-j $(nproc --all)`` ——但这种并行化在出错时也会带来一些杂乱
几秒钟后，构建过程应该会再次遇到错误。现在试着找出描述问题最关键的那一行。然后在互联网上搜索该行
中最重要、最不通用的一段（比如 4 8 个词）；避免或去掉任何看起来远程系统特定的东西，比如你的用户或像 `/home/username/linux/` 这样的本地路径名。首先用该字符串尝试你常用的互联网搜索引擎，之后通过
`lore.kernel.org/all/ <https://lore.kernel.org/all/>`_ 搜索 Linux 内核邮件列表
这大多数时候会找到能解释问题所在的内容；很常见的是其中一个命中也会为你的提供一个解决方案。如果你
没有找到与你问题匹配的内容，换一个角度，通过修改搜索词或使用错误消息中的另一行再试一次
归根结底，你将要遇到的大多数麻烦很可能已经被别人遇到并报告过了。这包括原因不在你的系统、而在于代码的
问题。如果你遇到了其中之一，你也可能为你的找到解决方案（比如一个补丁）或变通方法

#### 打包你的内核


逐步指南使用默认make 目标（x86 上的 'bzImage' 'modules'）来构建内核的映像和模块，指南后面的
步骤再安装它们。你也可以改用以下目标之一，直接构建一切并直接打包
 - `make -j $(nproc --all) bindeb-pkg` 生成 deb 
 - `make -j $(nproc --all) binrpm-pkg` 生成 rpm 
 - `make -j $(nproc --all) tarbz2-pkg` 生成 bz2 压缩tar 
这只是为此目的可用的 make 目标的一个选择，其他的请参`make help`。你也可以在运行
`make -j $(nproc --all)` 之后使用这些目标，因为它们会接管所有已经构建好的东西
如果你使用这些目标来生成 deb rpm 包，请忽略逐步指南中关于安装和移除内核的说明；而是使用该格式的
包工具（dpkg rpm）或构建于其上的包管理工具（apt、aptitude、dnf/yum、zypper……）来安装和移除
包。注意，用这两个 make 目标生成的包被设计为可在使用这些格式的各种发行版上工作，因此它们有时表现与你的发行版的内核包有所不同
[back to step-by-step guide <build_sbs>]


### 安装你的内核


  **Now install your kernel** [... <install_sbs>]

执行逐步指南中的命令之后需要做什么，取决于是否存在以`installkernel` 可执行文件如何实现。许多商Linux 发行版在 `/sbin/` 中附带了这样一个内核安装器，它完成所需的一切，因此除了重启你无事可做。但有些
发行版包含的 installkernel 只完成部分工作——少数发行版则完全缺少它，把所有工作留给你
如果找到`installkernel`，内核的构建系统会把内核映像及相关文件的实际安装委托给这个可执行文件。在
几乎所Linux 发行版上，它会把映像存为 '/boot/vmlinuz-<你的内核发行'，并在旁边放一'System.map-<你的内核发行'。因此，你的内核会与其他已存在的内核并行安装，除非你已经有一个发行名
完全相同的内核
许多发行版上installkernel 之后会生成一'initramfs'（通常也称'initrd'），商品发行版依赖它启动；因此务必保持逐步指南中使用的两个 make 目标的顺序，因为如果你在内核模块之前安装内核映像，事就会乱套。通常 installkernel 之后也会把你的内核添加到 bootloader 配置中。如果你的发行版installkernel
不处理它们，你就得自己负责这两项任务中的一项或两项
少数发行版如 Arch Linux 及其衍生版完全没installkernel 可执行文件。在这些发行版上只需用内核的
```
     sudo make modules_install
     sudo install -m 0600 $(make -s image_name) /boot/vmlinuz-$(make -s kernelrelease)
     sudo install -m 0600 System.map /boot/System.map-$(make -s kernelrelease)
```

如果你的发行版借助 initramfs 启动，现在用你的发行版为此过程提供的工具为你的内核生成一个。之后把你的
内核添加到你bootloader 配置中并重启
[back to step-by-step guide <install_sbs>]


### 以后再来一

  *To later build another kernel you need similar, but sometimes slightly
  different commands* [... <another_sbs>]

构建后续内核的过程类似，但在某些点上略有不同。例如你不想对后续的内核构建使用 'localmodconfig'，因你已经创建了一个你想从现在起使用的精简配置。因此改为只使用 `oldconfig` `olddefconfig` 来把你的
构建配置调整到你要构建的内核版本的需求
如果你用 git 创建了浅克隆，请记住 :ref:`以更详细方式解释该设置的那个小节 <sources>`：你需要使用略不同`git fetch` 命令，并且在切换到另一个系列时需要添加一个额外的远程分支
[back to step-by-step guide <another_sbs>]


### 以后卸载内核


  *All parts of your installed kernel are identifiable by its release name and
  thus easy to remove later.* [... <uninstall_sbs>]

不要担心手动安装内核从而绕过了你发行版的打包系统会把你的机器彻底弄乱：你内核的所有部分以后都很容移除，因为文件只存放在两个地方，并且通常可以通过内核的发行名识别
这两个地方之一/lib/modules/ 中的一个目录，它保存每个已安装内核的模块。这个目录以内核的发行名命名因此，要移除其中一个内核的所有模块，只需移除它在 /lib/modules/ 中的模块目录
另一个地方是 /boot/，在那里安装一个内核时通常会放置一到五个文件。它们通常都包含发行名在文件名中，有多少文件及其名称在一定程度上取决于你发行版的 installkernel 可执行文件（见上<install>）及initramfs 生成器。在某些发行版上，逐步指南中提到的 `kernel-install` 命令会为你移除所有这些文件——同也移除它们在内核 bootloader 配置中的条目。在其他发行版上，你得自己负责这些步骤。以下命令应当交互式移除一个内核的两个主要文件
```
    rm -i /boot/{System.map,vmlinuz}-6.0.1-foobar
```

现在移除对应initramfs，它通常名为类似 `/boot/initramfs-6.0.1-foobar.img` `/boot/initrd.img-6.0.1-foobar`。之后检/boot/ 中文件名包含 '6.0.1-foobar' 的其他文件并一并删除现在从你bootloader 配置中移除该内核
注意，手动删除内核的文件或目录时，对'*' 这样的通配符要非常小心：当你只想删6.0 6.0.1 时，
你可能会意外删除 6.0.11 内核的文件
[back to step-by-step guide <uninstall_sbs>]


## FAQ


### 为什么这how-to"在我的系统上不工作？


如一开始所述，本指旨在覆盖在商PC 或服务器硬件上运行的主流 Linux 发行版上构建内核通常所需的一尽管如此，所概述的方法在很多其他设置上也应当能工作。但试图在一份指南中覆盖每个可能的用例会违背其目的，
因为没有这样的聚焦，你将需要几十条或几百条类似"如果你有 <某机器或发行，你在此处必须做
<这个和那 <instead|additionally>"这样的构造。其中每一条都会让文本更长、更复杂、更难跟随
话虽如此：这当然是一个权衡。因此，如果你认为一个额外的用例值得描述，请按上<submit_improvements_qbtl>
所述把它建议给本文档的维护者

..
   end-of-content
..
   This document is maintained by Thorsten Leemhuis <linux@leemhuis.info>. If
   you spot a typo or small mistake, feel free to let him know directly and
   he'll fix it. You are free to do the same in a mostly informal way if you
   want to contribute changes to the text -- but for copyright reasons please CC
   linux-doc@vger.kernel.org and 'sign-off' your contribution as
   Documentation/process/submitting-patches.rst explains in the section 'Sign
   your work - the Developer's Certificate of Origin'.
..
   This text is available under GPL-2.0+ or CC-BY-4.0, as stated at the top
   of the file. If you want to distribute this text under CC-BY-4.0 only,
   please use 'The Linux kernel development community' for author attribution
   and link this as source:
   https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/Documentation/admin-guide/quickly-build-trimmed-linux.rst
..
   Note: Only the content of this RST file as found in the Linux kernel sources
   is available under CC-BY-4.0, as versions of this text that were processed
   (for example by the kernel's build system) might contain content taken from
   files which use a more restrictive license.
