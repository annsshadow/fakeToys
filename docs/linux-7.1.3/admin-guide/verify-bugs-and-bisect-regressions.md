
## 如何验证缺陷并进行回归的二分定位


本文介绍如何检查某Linux 内核问题是否出现在开发者当前维护的代码中——并进一步说明，若该问题属于回归（例如早期版本中并未出现），应如何定位导致该问题的变更

正文主要面向在普通硬件上运行主流 Linux 发行版内核、并希望向上Linux 开发者报告内核缺陷的用户。尽管如此，这些说明同样适用于已经熟悉自行构建内核的用户：它们有助于避免即使是经验丰富的开发者偶尔也会犯下的错误

..
   Note: if you see this note, you are reading the text's source file. You
   might want to switch to a rendered version: it makes it a lot easier to
   read and navigate this document -- especially when you want to look something
   up in the reference section, then jump back to where you left off.
..
   Find the latest rendered version of this text here:
   https://docs.kernel.org/admin-guide/verify-bugs-and-bisect-regressions.html

## 流程的核心（即“TL;DR”）


*[如果你是初次构建内核或对其进行二分定位，请忽略本节，直接前往下方'step-by-step guide <introguide_bissbs>'。本节使用与下文相同的命令，只是描述更为简略；不过这些步骤依然易于遵循，并且参考章节中的相关条目还提到了许多替代方案、陷阱以及其他注意事项，在你当前的情况下这些可能都至关重要。]*

**如果你想检查某个缺陷是否存在于开发者当前维护的代码*，只需执行 **准备工作（preparations* **1 段（segment 1*；在此过程中，把你日常使用的、最新的 Linux 内核视为“可用（working）”内核。下面的示例假设该内核为 6.0，因此将使用它的源码来准.config 文件

**如果你遇到的是一个回*，请至少执行**2 段（segment 2* 结束。随后你可以提交一份初步报告——也可以继续 **3 段（segment 3*，其中说明了如何执行一份完整的回归报告所需的二分定位。下面的示例假设 6.0.13 为“可用（working）”内核.1.5 为第一个“损坏（broken）”内核，因此将把 6.0 视为“良好（good）”版本并用于准备 .config 文件

```
    # * Remove any software that depends on externally maintained kernel modules
    #   or builds any automatically during bootup.
    # * Ensure Secure Boot permits booting self-compiled Linux kernels.
    # * If you are not already running the 'working' kernel, reboot into it.
    # * Install compilers and everything else needed for building Linux.
    # * Ensure to have 15 Gigabyte free space in your home directory.
    git clone -o mainline --no-checkout \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git ~/linux/
    cd ~/linux/
    git remote add -t master stable \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git
    git switch --detach v6.0
    # * Hint: if you used an existing clone, ensure no stale .config is around.
    make olddefconfig
    # * Ensure the former command picked the .config of the 'working' kernel.
    # * Connect external hardware (USB keys, tokens, ...), start a VM, bring up
    #   VPNs, mount network shares, and briefly try the feature that is broken.
    yes '' | make localmodconfig
    ./scripts/config --set-str CONFIG_LOCALVERSION '-local'
    ./scripts/config -e CONFIG_LOCALVERSION_AUTO
    # * Note, when short on storage space, check the guide for an alternative:
    ./scripts/config -d DEBUG_INFO_NONE -e KALLSYMS_ALL -e DEBUG_KERNEL \
      -e DEBUG_INFO -e DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT -e KALLSYMS
    # * Hint: at this point you might want to adjust the build configuration;
    #   you'll have to, if you are running Debian.
    make olddefconfig
    cp .config ~/kernel-config-working
```
- **1 段（Segment 1*：基于最新的 mainline 代码库构建一个内核

  这除了可以检查问题是否已经被修复之外，还能告诉后续需要知会哪些开发者；在回归的情况下，这一步还能排除问题是.config 变更而起的可能

```
    cd ~/linux/
    git switch --discard-changes --detach mainline/master

  b) Build, install, and boot a kernel::

    cp ~/kernel-config-working .config
    make olddefconfig
    make -j $(nproc --all)
    # * Make sure there is enough disk space to hold another kernel:
    df -h /boot/ /lib/modules/
    # * Note: on Arch Linux, its derivatives and a few other distributions
    #   the following commands will do nothing at all or only part of the
    #   job. See the step-by-step guide for further details.
    sudo make modules_install
    command -v installkernel && sudo make install
    # * Check how much space your self-built kernel actually needs, which
    #   enables you to make better estimates later:
    du -ch /boot/*$(make -s kernelrelease)* | tail -n 1
    du -sh /lib/modules/$(make -s kernelrelease)/
    # * Hint: the output of the following command will help you pick the
    #   right kernel from the boot menu:
    make -s kernelrelease | tee -a ~/kernels-built
    reboot
    # * Once booted, ensure you are running the kernel you just built by
    #   checking if the output of the next two commands matches:
    tail -n 1 ~/kernels-built
    uname -r
    cat /proc/sys/kernel/tainted

  c) Check if the problem occurs with this kernel as well.
```
- **2 段（Segment 2*：确保“良好（good）”内核同时也是“可用（working）”内核

  这在其他方面也验证了裁剪后的 .config 文件确实能正常工作，否则用它来做二分定位就是在浪费时间：

```
    cd ~/linux/
    git switch --discard-changes --detach v6.0

  b) Build, install, and boot a kernel as described earlier in *segment 1,
     section b* -- just feel free to skip the 'du' commands, as you have a rough
     estimate already.

  c) Ensure the feature that regressed with the 'broken' kernel actually works
     with this one.
```
- **3 段（Segment 3*：执行并验证二分定位

```
    git remote set-branches --add stable linux-6.1.y
    git fetch stable

  b) Initialize the bisection::

    cd ~/linux/
    git bisect start
    git bisect good v6.0
    git bisect bad v6.1.5

  c) Build, install, and boot a kernel as described earlier in *segment 1,
     section b*.

     In case building or booting the kernel fails for unrelated reasons, run
     ``git bisect skip``. In all other outcomes, check if the regressed feature
     works with the newly built kernel. If it does, tell Git by executing
     ``git bisect good``; if it does not, run ``git bisect bad`` instead.

     All three commands will make Git check out another commit; then re-execute
     this step (e.g. build, install, boot, and test a kernel to then tell Git
     the outcome). Do so again and again until Git shows which commit broke
     things. If you run short of disk space during this process, check the
     section 'Complementary tasks: cleanup during and after the process'
     below.

  d) Once your finished the bisection, put a few things away::

    cd ~/linux/
    git bisect log > ~/bisect-log
    cp .config ~/bisection-config-culprit
    git bisect reset

  e) Try to verify the bisection result::

    git switch --discard-changes --detach mainline/master
    git revert --no-edit cafec0cacaca0
    cp ~/kernel-config-working .config
    ./scripts/config --set-str CONFIG_LOCALVERSION '-local-cafec0cacaca0-reverted'

    This is optional, as some commits are impossible to revert. But if the
    second command worked flawlessly, build, install, and boot one more kernel
    kernel; just this time skip the first command copying the base .config file
    over, as that already has been taken care off.
```
- **辅助任务（Complementary tasks*：在流程进行期间及之后进行清理

  a) 为了避免在二分定位过程中耗尽磁盘空间，你可能需要删除一些之前构建的内核。你很可能希望将1 段和2 段期间构建的内核保留一段时间，但在实际二分定位过程中测试过的内核，你多半不再需要它们

```
       ls -ltr /lib/modules/*-local*

    To then for example erase a kernel that identifies itself as
    '6.0-rc1-local-gcafec0cacaca0', use this::

       sudo rm -rf /lib/modules/6.0-rc1-local-gcafec0cacaca0
       sudo kernel-install -v remove 6.0-rc1-local-gcafec0cacaca0
       # * Note, on some distributions kernel-install is missing
       #   or does only part of the job.

  b) If you performed a bisection and successfully validated the result, feel
     free to remove all kernels built during the actual bisection (Segment 3 c);
     the kernels you built earlier and later you might want to keep around for
     a week or two.
```
```
    git fetch mainline
    git switch --discard-changes --detach mainline/master
    git apply /tmp/foobars-proposed-fix-v1.patch
    cp ~/kernel-config-working .config
    ./scripts/config --set-str CONFIG_LOCALVERSION '-local-foobars-fix-v1'

  Build, install, and boot a kernel as described in *segment 1, section b* --
  but this time omit the first command copying the build configuration over,
  as that has been taken care of already.
```
## 关于如何验证缺陷并进行回归二分定位的分步指南


本指南介绍如何搭建你自己Linux 内核，以调查你打算报告的缺陷或回归。你想在多大程度上遵循这些说明，取决于你遇到的问题：

执行**1 段（segment 1* 结束，以 **验证你的内核问题是否出现Linux 内核开发者维护的代码*。如果是，你就可以准备报告该缺陷了——除非它在更早的内核版本中并未发生，那样你就至少应当继续 **2 段（segment 2* **检查该问题是否符合回归（regression）的定义**，回归会得到优先处理。根据结果，你就可以准备报告缺陷或提交一份初步的回归报告；与其提交后者，你也可以直接继续 **3 段（segment 3* **执行二分定位**，以获得一份开发者有义务处理的完整回归报告

 Preparations: 搭建一切以构建你自己的内核 <introprep_bissbs>.

 Segment 1: 用最新的代码库尝试复现问<introlatestcheck_bissbs>.

 Segment 2: 检查你构建的内核是否工作正<introworkingcheck_bissbs>.

 Segment 3: 执行二分定位并验证结<introbisect_bissbs>.

 Complementary tasks: 在遵循本指南期间及之后的清理工作 <introclosure_bissbs>.

 Optional tasks: 测试 revert、补丁或更新的版<introoptional_bissbs>.

每个段落中的步骤说明了流程的重要方面，而一份详尽的参考章节为几乎所有步骤都提供了更多细节。参考章节有时还会列出替代方案、陷阱，以及在该特定步骤可能出现的问题——以及如何让事情重新回到正轨

关于如何报告 Linux 内核问题或回归的更多细节，请参阅 Documentation/admin-guide/reporting-issues.rst，它与本文档配合使用。其中特别解释了为什么即使你面对的是来自“stable/longterm”系列（例如 6.0.13）的内核问题，也需要用最新的“mainline”内核（例如 6.0.1-rc1 6.1-rc6 等版本）来验证缺陷

对于遇到回归的用户，该文档还解释了为什么在2 段之后提交一份初步报告是明智的，因为该回归及culprit 可能已经被知晓。关于究竟什么才算回归的更多细节，请参阅 Documentation/admin-guide/reporting-regressions.rst

如果你在遵循本指南时遇到任何问题，或者有好点子来改进它，请告知内核开发<submit_improvements_vbbr>


### Preparations: 搭建一切以构建你自己的内核


以下步骤为所有后续任务打下基础

Note: the instructions assume you are building and testing on the same machine; if you want to compile the kernel on another system, check Build kernels on a different machine <buildhost_bis> below.


- 创建一份全新的备份，并准备好系统修复与恢复工具，以防万一出现意外情况

  [details <backup_bisref>]


- 移除所有依赖外部开发的内核驱动、或会在启动时自动构建它们的软件。这包括但不限于 DKMS、openZFS、VirtualBox，以Nvidia 的图形驱动（包括GPL 许可的内核模块）

  [details <vanilla_bisref>]


- 在带有“Secure Boot”或类似机制的平台上，准备好一切，确保系统允许你自行编译的内核启动。在普x86 系统上，最快捷简便的方法是在 BIOS 设置工具中禁用此类机制；或者，通过`mokutil --disable-validation` 发起的流程来解除其限制

  [details <secureboot_bisref>]


- 确定贯穿本指南被视为“良好（good）”和“损坏（bad）”的内核版本

  - 你遵循本指南是想验证某个缺陷是否出现在主要开发者所关注的代码中？那么把你当前日常使用的最新内核版本视为“良好（good）”（例如 6.0.0.13 6.1-rc2）

  - 你遇到了回归，例如在切换到较新的内核版本后，某些功能损坏或表现变差？这种情况下取决于问题出现时的版本范围

    - 在从某个 stable/longterm 版本（例6.0.13）更新到更新mainline 系列（如 6.1-rc7 6.1），或基于它stable/longterm 版本（如 6.1.5）时发生了回归？那么把你可用内核所基于mainline 版本视为“良好（good）”版本（例如 6.0），并将第一个损坏的版本视为“损坏（bad）”版本（例如 6.1-rc7.1 6.1.5）。注意，此时仅仅是假6.0 没问题；这一假设将在2 段中检验

    - 在从一mainline 版本（例6.0）切换到更新的版本（6.1-rc1）或基于它的 stable/longterm 版本（如 6.1.5）时发生了回归？那么将最后一个可用版本（例如 6.0）视为“良好（good）”，将第一个损坏版本（例如 6.1-rc1 6.1.5）视为“损坏（bad）”

    - stable/longterm 系列内部更新时（例如6.0.13 6.0.15）发生了回归？那么将这些版本视为“良好（good）”和“损坏（bad）”（例如 6.0.13 6.0.15），因为你需要在该系列内部进行二分定位

  *注意，不要把“良好（good）”版本与“可用（working）”内核混淆；后一个术语在整篇指南中指的是最后一个一直正常工作着的内核

  [details <rangecheck_bisref>]


- 启动进入“可用（working）”内核，并简单试用一下明显已损坏的功能

  [details <bootworking_bisref>]


- 确保有足够的可用空间来构Linux。主目录15 GB 通常已经足够。如果你可用空间更少，请务必留意后续关于获取 Linux 源码和处理调试符号的步骤：两者都介绍了能减少空间占用的办法，应当能让你在4 GB 可用空间的情况下完成这些任务

  [details <diskspace_bisref>]


- 安装构建 Linux 内核所需的所有软件。通常你会需要：'bc'binutils'ld' 等）bison'flex'gcc'git'openssl'pahole'perl'，以'libelf' 'openssl' 的开发头文件。参考章节展示了如何在多种流行的 Linux 发行版上快速安装它们

  [details <buildrequires_bisref>]


- 获取 mainline Linux 源代码；然后进入存放这些代码的目录，因为本指南后续所有命令都应从该目录执行

  *注意，下面介绍的是通过完整mainline 克隆来获取源代码，截2024 年初其下载量约为 2.75 GB 参考小节介绍了两种替代方案 <sources_bisref> *：一种下载量不到 500 MB，另一种更适合网络不稳定的连接

  执行以下命令以获取一份全新的 mainline 代码库：
```

    git clone -o mainline --no-checkout \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git ~/linux/
    cd ~/linux/
    git remote add -t master stable \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git

  [:ref:`details <sources_bisref>`]

```

- 你之前确定的“良好（good）”或“损坏（bad）”版本中，是否有一个是 stable longterm 发行版（6.1.5）？那么，请下载它所属系列的源代
```

    git remote set-branches --add stable linux-6.1.y
    git fetch stable

```

- 开始准备内核构建配置（'.config' 文件）

  在此之前，请确认你仍在运行早前步骤要求你启动的“可用（working）”内核；如果不确定，可用 `uname -r` 查看当前kernelrelease 标识符

  之后，检出早前确定为“良好（good）”的版本对应的源代码。下面的示例命令假定该版本为 6.0；请注意，本命令及后续所Git 命令中的版本号都需要加上前缀
```

    git switch --discard-changes --detach v6.0

  Now create a build configuration file::

    make olddefconfig

  The kernel build scripts then will try to locate the build configuration file for the running kernel and then adjust it for the needs of the kernel sources you checked out. While doing so, it will print a few lines you need to check.

  Look out for a line starting with '# using defaults found in'. It should be followed by a path to a file in '/boot/' that contains the release identifier of your currently working kernel. If the line instead continues with something like 'arch/x86/configs/x86_64_defconfig', then the build infra failed to find the .config file for your running kernel -- in which case you have to put one there manually, as explained in the reference section.

  In case you can not find such a line, look for one containing '# configuration written to .config'. If that's the case you have a stale build configuration lying around. Unless you intend to use it, delete it; afterwards run 'make olddefconfig' again and check if it now picked up the right config file as base.

  [:ref:`details <oldconfig_bisref>`]

```

- 禁用那些对你的配置而言明显多余的任意内核模块。这一步是可选的，但对于二分定位尤其明智，因为它能极大地加快构建过程——除非上一步取得的 .config 文件已经针对你和你的硬件需求做了定制，那样的话你应跳过此步

  为准备精简，请连接你偶尔使用的外部硬件（USB 密钥、令牌等），快速启动一VM，并启用 VPN。如果你在开始遵循本指南后重启过，请确保已经尝试使用那个导致
```

     yes '' | make localmodconfig

  There is a catch to this, as the 'apparently' in initial sentence of this step and the preparation instructions already hinted at:

  “localmodconfig”目标很容易禁用那些仅偶尔使用的功能对应的内核模块——例如自启动以来尚未连接的外部外设的模块、尚未使用的虚拟化软件、VPN 隧道，以及其他一些东西。这是因为某些任务依赖的内核模块只有在你首次执行这类任务时，Linux 才会加载

  localmodconfig 的这一缺点并不值得你忧心，但应当记在心里：如果本指南构建的内核出现某种异常行为，这很可能就是原因。你可以用参考小节中列出的技巧来降低或几乎消除这一风险；但如果是仅为快速测试而构建内核，只要它能启动并让你正常测试出问题的功能，通常不值得在此上花费太多精力

  [:ref:`details <localmodconfig_bisref>`]

```

- 确保你将构建的所有内核都能通过一种特
```

    ./scripts/config --set-str CONFIG_LOCALVERSION '-local'
    ./scripts/config -e CONFIG_LOCALVERSION_AUTO

  [:ref:`details <tagging_bisref>`]

```

- 决定如何处理调试符号

  就本文档而言，通常明智的做法是启用它们，因为你很有可能会需要从一个“panic”、“Oops”
```

    ./scripts/config -d DEBUG_INFO_NONE -e KALLSYMS_ALL -e DEBUG_KERNEL \
      -e DEBUG_INFO -e DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT -e KALLSYMS

  But if you are extremely short on storage space, you might want to disable debug symbols instead::

    ./scripts/config -d DEBUG_INFO -d DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT \
      -d DEBUG_INFO_DWARF4 -d DEBUG_INFO_DWARF5 -e CONFIG_DEBUG_INFO_NONE

  [:ref:`details <debugsymbols_bisref>`]

```

- 检查你是否想要或需要调整其他一些内核配置选项

  - 你使用的Debian 吗？那么，你可能希望通过执行参考小节中介绍的额外调整来避免已知问题

    [details <configmods_distros_bisref>].

  - 如果你想影响配置的其他方面，现在就用你喜欢的工具去做。注意，要使'menuconfig' 'nconfig' 这样make 目标，你需要安ncurses 的开发文件；对于 'xconfig'，你同样需Qt5 Qt6 的头文件

    [details <configmods_individual_bisref>].

- 在最新调整之后重新处.config，并将其保存在安全的
```

     make olddefconfig
     cp .config ~/kernel-config-working

  [:ref:`details <saveconfig_bisref>`]

```

### 1 段：尝试用最新的代码库复现问


以下步骤用于确认问题是否出现在开发者当前维护的代码中。如果你遇到的是回归问题，它还能确认问题不是由某.config 变更引起的，否则报告该问题就是在浪费时间[details <introlatestcheck_bisref>]


- 检出最新的 Linux 代码库

  - 你的“良好（good）”和“损坏（bad）”版本是否来自同一stable longterm 系列？那么请查看 `kernel.org 首页 <https://kernel.org/>`_：如果它列出了该系列中一个不带“[EOL]”标签的发行版，就检出该系列
```

      cd ~/linux/
      git switch --discard-changes --detach stable/linux-6.1.y

    Your series is unsupported, if is not listed or carrying a 'end of life' tag. In that case you might want to check if a successor series (say linux-6.2.y) or mainline (see next point) fix the bug.

  * In all other cases, run::

      cd ~/linux/
      git switch --discard-changes --detach mainline/master

  [:ref:`details <checkoutmaster_bisref>`]

```

- 使用你准备的配置文件构建第一个内核的镜像与模块：
```

    cp ~/kernel-config-working .config
    make olddefconfig
    make -j $(nproc --all)

  If you want your kernel packaged up as deb, rpm, or tar file, see the reference section for alternatives, which obviously will require other steps to install as well.

  [:ref:`details <build_bisref>`]

```

- 安装你新构建的内核
```

    df -h /boot/ /lib/modules/

  For now assume 150 MByte in /boot/ and 200 in /lib/modules/ will suffice; how much your kernels actually require will be determined later during this guide.

  Now install the kernel's modules and its image, which will be stored in parallel to the your Linux distribution's kernels::

    sudo make modules_install
    command -v installkernel && sudo make install

  The second command ideally will take care of three steps required at this point: copying the kernel's image to /boot/, generating an initramfs, and adding an entry for both to the boot loader's configuration.

  Sadly some distributions (among them Arch Linux, its derivatives, and many immutable Linux distributions) will perform none or only some of those tasks. You therefore want to check if all of them were taken care of and manually perform those that were not. The reference section provides further details on that; your distribution's documentation might help, too.

  Once you figured out the steps needed at this point, consider writing them down: if you will build more kernels as described in segment 2 and 3, you will have to perform those again after executing ``command -v installkernel [...]``.

  [:ref:`details <install_bisref>`]

```

- 如果你打算继续遵循本指南，请检查需要多少存储空
```

    du -ch /boot/*$(make -s kernelrelease)* | tail -n 1
    du -sh /lib/modules/$(make -s kernelrelease)/

  Write down or remember those two values for later: they enable you to prevent running out of disk space accidentally during a bisection.

  [:ref:`details <storagespace_bisref>`]

```

```

    make -s kernelrelease | tee -a ~/kernels-built

  Remember the identifier momentarily, as it will help you pick the right kernel from the boot menu upon restarting.

```

- 重启进入你新构建的内核。为确保你启动的确实是你刚构建的那个，你可能想验证这些命令的输出
```

    tail -n 1 ~/kernels-built
    uname -r

```

```

    cat /proc/sys/kernel/tainted

  If that command does not return '0', check the reference section, as the cause for this might interfere with your testing.

  [:ref:`details <tainted_bisref>`]

```

- 验证你构建的新内核是否出现了该缺陷。如果没有，请查阅参考小节中的说明，以确保你的测试过程中没有出岔子

  [details <recheckbroken_bisref>]


- 你刚构建的是 stable longterm 内核吗？并且你能否用它复现该回归？那么你也应当测试最新的 mainline 代码库，因为结果决定了该缺陷必须提交给哪些开发者
```

    cd ~/linux/
    git switch --discard-changes --detach mainline/master

  Now use the checked out code to build and install another kernel using the commands the earlier steps already described in more detail::

    cp ~/kernel-config-working .config
    make olddefconfig
    make -j $(nproc --all)
    # * Check if the free space suffices holding another kernel:
    df -h /boot/ /lib/modules/
    sudo make modules_install
    command -v installkernel && sudo make install
    make -s kernelrelease | tee -a ~/kernels-built
    reboot

  Confirm you booted the kernel you intended to start and check its tainted status::

    tail -n 1 ~/kernels-built
    uname -r
    cat /proc/sys/kernel/tainted

  Now verify if this kernel is showing the problem. If it does, then you need to report the bug to the primary developers; if it does not, report it to the stable team. See Documentation/admin-guide/reporting-issues.rst for details.

  [:ref:`details <recheckstablebroken_bisref>`]

```

你遵循本指南是为了验证某个问题是否存在于 Linux 内核开发者当前维护的代码中吗？那么到此你就完成了。如果你之后想删除刚构建的内核，请参阅“补充任务：遵循本指南期间及之后的清理工<introclosure_bissbs>”

如果你遇到的是回归问题，请继续并至少执行下一段


### 2 段：检查你构建的内核是否工作正


如果是回归问题，你现在需要确保早前创建的精简配置文件按预期工作；否则用它进行二分定位就是在浪费时间[details <introworkingcheck_bisref>]


- 构建你自己的“可用（working）”内核变体，并检查那个发生回归的功能在它上面是否按预期工作

  首先检出早前确定为
```

    cd ~/linux/
    git switch --discard-changes --detach v6.0

  Now use the checked out code to configure, build, and install another kernel using the commands the previous subsection explained in more detail::

    cp ~/kernel-config-working .config
    make olddefconfig
    make -j $(nproc --all)
    # * Check if the free space suffices holding another kernel:
    df -h /boot/ /lib/modules/
    sudo make modules_install
    command -v installkernel && sudo make install
    make -s kernelrelease | tee -a ~/kernels-built
    reboot

  When the system booted, you may want to verify once again that the kernel you started is the one you just built::

    tail -n 1 ~/kernels-built
    uname -r

  Now check if this kernel works as expected; if not, consult the reference section for further instructions.

  [:ref:`details <recheckworking_bisref>`]

```

### 3 段：执行二分定位并验证结


在完成了所有准备工作和预防性构建之后，你现在可以开始二分定位了。这会让你构建相当多的内核——通常约为 15 个，如果你是在更新到较新系列（如6.0.13 6.1.5）时遇到的回归。但不用担心，由于早前创建的精简构建配置，这个过程比许多人想象的要快得多：在普x86 机器上，平均来说编译每个内核通常只需10 15 分钟


- 开始二分定位，并告Git 早前确定的版
```

    cd ~/linux/
    git bisect start
    git bisect good v6.0
    git bisect bad v6.1.5

  [:ref:`details <bisectstart_bisref>`]

```

- 现在Git 检出的代码，借助
```

    cp ~/kernel-config-working .config
    make olddefconfig
    make -j $(nproc --all)
    # * Check if the free space suffices holding another kernel:
    df -h /boot/ /lib/modules/
    sudo make modules_install
    command -v installkernel && sudo make install
    make -s kernelrelease | tee -a ~/kernels-built
    reboot

  If compilation fails for some reason, run ``git bisect skip`` and restart executing the stack of commands from the beginning.

  In case you skipped the 'test latest codebase' step in the guide, check its description as for why the 'df [...]' and 'make -s kernelrelease [...]' commands are here.

  Important note: the latter command from this point on will print release identifiers that might look odd or wrong to you -- which they are not, as it's totally normal to see release identifiers like '6.0-rc1-local-gcafec0cacaca0' if you bisect between versions 6.1 and 6.2 for example.

  [:ref:`details <bisectbuild_bisref>`]

```

- 现在检查那个发生回归的功能在你刚构建的内核中是否工作正常

   你可能还是想先确认你启动的内核正是你构建的那
```

    cd ~/linux/
    tail -n 1 ~/kernels-built
    uname -r

  Now verify if the feature that regressed works at this kernel bisection point.
  If it does, run this::

    git bisect good

  If it does not, run this::

    git bisect bad

  Be sure about what you tell Git, as getting this wrong just once will send the rest of the bisection totally off course.

  While the bisection is ongoing, Git will use the information you provided to find and check out another bisection point for you to test. While doing so, it will print something like 'Bisecting: 675 revisions left to test after this (roughly 10 steps)' to indicate how many further changes it expects to be tested. Now build and install another kernel using the instructions from the previous step; afterwards follow the instructions in this step again.

  Repeat this again and again until you finish the bisection -- that's the case when Git after tagging a change as 'good' or 'bad' prints something like 'cafecaca0c0dacafecaca0c0dacafecaca0c0da is the first bad commit'; right afterwards it will show some details about the culprit including the patch description of the change. The latter might fill your terminal screen, so you might need to scroll up to see the message mentioning the culprit; alternatively, run ``git bisect log > ~/bisection-log``.

  [:ref:`details <bisecttest_bisref>`]

```

- 在以下操作之前，先将 Git 的二分日志与当前.config 文件保存在安全的地方
```

    cd ~/linux/
    git bisect log > ~/bisection-log
    cp .config ~/bisection-config-culprit
    git bisect reset

  [:ref:`details <bisectlog_bisref>`]

```

- 尝试在最mainline 之上回退罪魁祸首，看是否能修复你的回归

  This is optional, as it might be impossible or hard to realize. The former is the case, if the bisection determined a merge commit as the culprit; the latter happens if other changes depend on the culprit. But if the revert succeeds, it is worth building another kernel, as it validates the result of a bisection, which can easily deroute; it furthermore will let kernel developers know, if they can resolve the regression with a quick revert.

  Begin by checking out the latest codebase depending on the range you bisected:

  - Did you face a regression within a stable/longterm series (say between 6.0.13 and 6.0.15) that does not happen in mainline? Then check out the
```

      git fetch stable
      git switch --discard-changes --detach linux-6.0.y

  * In all other cases check out latest mainline::

      git fetch mainline
      git switch --discard-changes --detach mainline/master

    If you bisected a regression within a stable/longterm series that also happens in mainline, there is one more thing to do: look up the mainline commit-id. To do so, use a command like ``git show abcdcafecabcd`` to view the patch description of the culprit. There will be a line near the top which looks like 'commit cafec0cacaca0 upstream.' or 'Upstream commit cafec0cacaca0'; use that commit-id in the next command and not the one the bisection blamed.

  Now try reverting the culprit by specifying its commit id::

    git revert --no-edit cafec0cacaca0

  If that fails, give up trying and move on to the next step; if it works, adjust the tag to facilitate the identification and prevent accidentally overwriting another kernel::

    cp ~/kernel-config-working .config
    ./scripts/config --set-str CONFIG_LOCALVERSION '-local-cafec0cacaca0-reverted'

  Build a kernel using the familiar command sequence, just without copying the the base .config over::

    make olddefconfig &&
    make -j $(nproc --all)
    # * Check if the free space suffices holding another kernel:
    df -h /boot/ /lib/modules/
    sudo make modules_install
    command -v installkernel && sudo make install
    make -s kernelrelease | tee -a ~/kernels-built
    reboot

  Now check one last time if the feature that made you perform a bisection works with that kernel: if everything went well, it should not show the regression.

  [:ref:`details <revert_bisref>`]

```

### 补充任务：二分定位期间及之后的清理工


在遵循本指南期间及之后，你可能想要或需要删除一些已安装的内核：否则启动菜单会变得混乱，或者空间可能耗尽


- 要删除某个已安装的内核，先查它的“kernelrelease”标识符。本指南将它们保存在“~/kernels-built”中，但也可以借助以下
```

    ls -ltr /lib/modules/*-local*

  You in most situations want to remove the oldest kernels built during the actual bisection (e.g. segment 3 of this guide). The two ones you created beforehand (e.g. to test the latest codebase and the version considered 'good') might become handy to verify something later -- thus better keep them around, unless you are really short on storage space.

  To remove the modules of a kernel with the kernelrelease identifier '*6.0-rc1-local-gcafec0cacaca0*', start by removing the directory holding its modules::

    sudo rm -rf /lib/modules/6.0-rc1-local-gcafec0cacaca0

  Afterwards try the following command::

    sudo kernel-install -v remove 6.0-rc1-local-gcafec0cacaca0

  On quite a few distributions this will delete all other kernel files installed while also removing the kernel's entry from the boot menu. But on some distributions kernel-install does not exist or leaves boot-loader entries or kernel image and related files behind; in that case remove them as described in the reference section.

  [:ref:`details <makeroom_bisref>`]

```

- 一旦完成二分定位，不要立即删除你搭建的任何东西，因为你可能还需要用到其中一些。哪些可以安全删除，取决于二分定位的结果

  - 你最初能否用最新代码库复现该回归，并且在二分定位后通过回退最新代码库之上的罪魁祸首修复了问题？那么你可能想把那两个内核保留一段时间，但安全地删除所有发行标识符中包含local”的其他内核

  - 二分定位是否终结于一个合并提交，或因其他原因显得可疑？那么你可能想尽可能多地保留内核几天：你很可能会被要求重新检查某些东西

  - 在其他情况下，最好将以下内核保留一段时间：从最新代码库构建的那个、用被视为“良好（good）”的版本创建的那一个，以及你在实际二分过程中编译的最后三四个

  [details <finishingtouch_bisref>]


### 可选：测试回退、补丁或更高版本


在报告缺陷期间或之后，你可能想要、也可能会被要求去测试回退、调试补丁、提议的修复，或其他版本。这种情况下，请遵循以下说明

- 更新你的 Git 克隆并检出最新代码

  - 如果你想测试 mainline，请在检出前先获取其最新变
```

      git fetch mainline
      git switch --discard-changes --detach mainline/master

  * In case you want to test a stable or longterm kernel, first add the branch holding the series you are interested in (6.2 in the example), unless you already did so earlier::

      git remote set-branches --add stable linux-6.2.y

    Then fetch the latest changes and check out the latest version from the series::

      git fetch stable
      git switch --discard-changes --detach stable/linux-6.2.y

```

```

    cp ~/kernel-config-working .config

```

- 你的下一步取决于你想做什么：

  - 如果你只是想测试最新代码库，直接进入下一步即可，你已经准备就绪

  - 如果你想测试回退是否能修复某个问题，请回退一个或多个
```

      git revert --no-edit cafec0cacaca0

    Now give that kernel a special tag to facilitates its identification and prevent accidentally overwriting another kernel::

      ./scripts/config --set-str CONFIG_LOCALVERSION '-local-cafec0cacaca0-reverted'

  * In case you want to test a patch, store the patch in a file like '/tmp/foobars-proposed-fix-v1.patch' and apply it like this::

      git apply /tmp/foobars-proposed-fix-v1.patch

    In case of multiple patches, repeat this step with the others.

    Now give that kernel a special tag to facilitates its identification and prevent accidentally overwriting another kernel::

    ./scripts/config --set-str CONFIG_LOCALVERSION '-local-foobars-fix-v1'

```

- 使用熟悉的命令构建内核，只是不要复制内核
```

    make olddefconfig &&
    make -j $(nproc --all)
    # * Check if the free space suffices holding another kernel:
    df -h /boot/ /lib/modules/
    sudo make modules_install
    command -v installkernel && sudo make install
    make -s kernelrelease | tee -a ~/kernels-built
    reboot

```

- 现在确认你启动的是新构建的内核并检查它

[details <introoptional_bisref>]


### 结语


你已到达分步指南的结尾

你在遵循分步指南时是否遇到了参考小节未能解决的麻烦？是否发现了错误？或者是否有改进指南的想法？

如果有上述任何情况，请通过发送简短说明或补丁Thorsten Leemhuis <linux@leemhuis.info>，并最好抄送公开Linux 文档邮件列表 <linux-doc@vger.kernel.org>，让开发者知晓。这样的反馈对进一步改进本文至关重要，也符合所有人的利益，因为它能让更多人掌握此处描述的任务


## 分步指南的参考小


本节包含对上述分步指南中几乎每一项内容的补充信息

### 构建自有内核的准备工


  **本节中的步骤为所有后续测试奠定基础*
  [... <introprep_bissbs>]

本指南所有后续小节中的步骤都依赖于此处描述的内容

[back to step-by-step guide <introprep_bissbs>].


#### 为紧急情况做准备


  **创建一份全新备份，并将系统修复与恢复工具放在手边*
  [... <backup_bissbs>]

请记住，你面对的是计算机，它们有时会发生意想不到的状况——尤其是当你摆弄像操作系统内核这样关键的部分时。而这正是你在此过程中要做的事。因此，即便不太可能发生，也最好为出现差错做好准备

[back to step-by-step guide <backup_bissbs>]


#### 移除任何与外部维护的内核模块相关的东


  *移除所有依赖外部开发的内核驱动、或会自动构建这类驱动的程序 [...<vanilla_bissbs>]

外部开发的内核模块很容易在二分定位过程中引发麻烦

但本指南包含这一步还有一个更重要的原因：大多数内核开发者不会理会那些使用了此类模块的内核所出现的回归报告。这是因为这类内核不再被视为“vanilla（原版）”，正如 Documentation/admin-guide/reporting-issues.rst 中更详细地解释的那样

[back to step-by-step guide <vanilla_bissbs>]


#### 应对 Secure Boot 之类的机


  *在启用了“Secure Boot”或类似机制的平台（commodity x86）上，请确保系统稍后允许你自行编译的内核启动，并做好一切准备 [... <secureboot_bissbs>]

许多现代系统只允许某些特定的操作系统启动；这正是它们默认拒绝启动自行编译的内核的原因

理想情况下，你应通过证书让你的平台信任你自行构建的内核，从而解决这个问题。具体做法此处不展开，因为那需要多个步骤，会使本文偏离主题Documentation/admin-guide/module-signing.rst' 以及各种网页 already 更详细地说明了所需的一切

临时禁用 Secure Boot 之类的机制，是让你自己的 Linux 启动的另一种办法。在普x86 系统上，可以BIOS 设置中完成；所需步骤因机器而异，因此此处无法详述

在主x86 Linux 发行版上，还有第三种通用的办法：为你Linux 环境禁用所Secure Boot 限制。你可以通过运行 `mokutil --disable-validation` 来启动这一过程；它会提示你创建一个一次性密码，记下来是安全的。现在重启；BIOS 完成所有自检后，引导加载程序 Shim 会显示一个蓝色方框，提示“Press any key to perform MOK management”。在倒计时结束前按下任意键，即可打开一个菜单。选择“Change Secure Boot state”。Shim 的“MokManager”会要求你输入之前设定的一次性密码中的三个随机字符。输入后，确认你确实想要禁用该验证。之后，允许 MokManager 重启机器

[back to step-by-step guide <secureboot_bissbs>]


#### 启动最后一个工作正常的内核


  *启动进入最后一个工作正常的内核，并简要重新确认那个发生回归的功能是否真的正常工作 [...<bootworking_bissbs>]

这能让后续涉及创建和精简配置的步骤做出正确的事情

[back to step-by-step guide <bootworking_bissbs>]


#### 空间需


  **确保有足够的空闲空间用于构建 Linux*
  [... <diskspace_bissbs>]

上述数字只是粗略估计，并预留了较大的余量以确保安全，因此你实际需要的往往更少

If you have space constraints, be sure to hay attention to the :ref:`关于调试符号的步<debugsymbols_bissbs>` and its :ref:` accompanying 参考小<debugsymbols_bisref>`, as disabling then will reduce the consumed disk space by quite a few gigabytes.

[back to step-by-step guide <diskspace_bissbs>]


#### 二分范围


  *确定贯穿本指南、被视为“良好（good）”与“损坏（bad）”的内核版本 [...<rangecheck_bissbs>]

确定待检查的提交范围通常很直接，除非回归发生在从一stable 系列的发行版切换到较新系列的发行版时（如6.0.13 6.1.5）。这种情况下 Git 需要一些引导，因为没有一条直系的继承线

这是因为随着 6.0 的发布，mainline 推进到了 6.1，stable 系列 6.0.y 则分叉到了一旁。因此从理论上讲，你6.1.5 上遇到的问题可能只在 6.0.13 中正常，因为它是由进入某6.0.y 发行版的提交修复的，但从未进mainline 6.1.y 系列。所幸，由于 stable/longterm 维护者维护代码的方式，这种情况通常不会发生。因此，6.0 假设为“良好（good）”内核是相当安全的。不过这个假设无论如何都会被检验，因为该内核将在本指南的第 2 段中被构建和测试；如果你尝试6.0.13 6.1.15 之间进行二分，Git 也会强制你这样做

[back to step-by-step guide <rangecheck_bissbs>]


#### 安装构建所需的依


  **安装构建 Linux 内核所需的全部软件*
  [...<buildrequires_bissbs>]

内核相当独立，但除了编译器之类的工具外，有时你还需要几个库才能构建它。如何安装所需的一切，取决于你Linux 发行版以及你即将构建的内核的配置

以下是一些主流发行版上你通常需要的示例

```

    sudo pacman --needed -S bc binutils bison flex gcc git kmod libelf openssl \
      pahole perl zlib ncurses qt6-base

```

```

    sudo apt install bc binutils bison dwarves flex gcc git kmod libelf-dev \
      libssl-dev make openssl pahole perl-base pkg-config zlib1g-dev \
      libncurses-dev qt6-base-dev g++

```

```

    sudo dnf install binutils \
      /usr/bin/{bc,bison,flex,gcc,git,openssl,make,perl,pahole,rpmbuild} \
      /usr/include/{libelf.h,openssl/pkcs7.h,zlib.h,ncurses.h,qt6/QtGui/QAction}

```

```

    sudo zypper install bc binutils bison dwarves flex gcc git \
      kernel-install-tools libelf-devel make modutils openssl openssl-devel \
      perl-base zlib-devel rpm-build ncurses-devel qt6-base-devel

```

这些命令会安装一些通常（但并非总是）需要的软件包。例如，你可能想跳过安装 ncurses 的开发头文件，因为只有在以后想用 'menuconfig' 'nconfig' 这些 make 目标来调整内核构建配置时才需要它们；同样，如果你不打算用 'xconfig' 调整 .config，也可以省略 Qt6 的头文件

此外，对于本指南未涵盖的任务——例如从内核tools/ 目录构建工具时——你可能还需要额外的库及其开发头文件

[back to step-by-step guide <buildrequires_bissbs>]


#### 使用 Git 下载源代


  **获取 Linux mainline 源代码*
  [...<sources_bissbs>]

分步指南介绍了如何通过 Linus mainline 仓库的完Git 克隆来下Linux 源代码。关于这一点没什么更多可说的——但还有两种替代的获取方式，可能对你更合适：

- 如果你的网络连接不稳定，可以考虑使用 'Git bundle'<sources_bundle_bisref>

- 如果下载完整的仓库耗时太久或需要过多存储空间，可以考虑 :ref:`使用 'shallow clone'（浅克隆sources_shallow_bisref>`


###### 使用 bundle 下载 Linux mainline 源代


使用以下命令通过
```

    wget -c \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/clone.bundle
    git clone --no-checkout clone.bundle ~/linux/
    cd ~/linux/
    git remote remove origin
    git remote add mainline \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git
    git fetch mainline
    git remote add -t master stable \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git

```
如果 'wget' 命令失败，只需重新执行它，它会从断点处继续

[back to step-by-step guide <sources_bissbs>]
[back to section intro <sources_bisref>]


#### 使用浅克隆下Linux mainline 源代


```

    git clone -o mainline --no-checkout --depth 1 -b master \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git ~/linux/
    cd ~/linux/
    git remote add -t master stable \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git

```
现在将克隆的历史加深到你“良好（good）”版本对mainline 发行版的上上个版本。如果后者是 6.0 6.0.13，那5.19 是上一个版本.18 是上上个版本——因此将历史加深
```

    git fetch --shallow-exclude=v5.18 mainline

```
之后，按照分步指南中的说明，stable Git 仓库添加为远程，并添加所有需要的 stable 分支

注意，浅克隆有几个特殊之处：

- 对于二分定位，历史需要比看起来必要的程度再加深几mainline 版本，如前所述。这是因为否Git 将无法回退或描述某个范围内的多数提交（6.1..6.2），因为它们在内部基于更早的内核发行版（6.0-rc2 5.19-rc3）

- 本文档在大多数地方使用带`--shallow-exclude=` `git fetch` 来指定你关心的最早版本（准确地说：是它的 git 标签）。你也可以改`--shallow-since=` 参数，指定一个绝对日期（`'2023-07-15'`）或相对日期（如 `'12 months'`）来定义想要下载的历史深度。在mainline 进行二分时，请确保将历史至少加深到你“良好（good）”内核所基于mainline 发行版发布前 7 个月

- 警告：在加深克隆时，你可能会遇到类似“fatal: error in object: unshallow cafecaca0c0dacafecaca0c0dacafecaca0c0da”的错误。这种情况下请运`git repack -d` 并重试

[back to step-by-step guide <sources_bissbs>]
[back to section intro <sources_bisref>]


#### 开始定义内核的构建配置


  **开始准备内核构建配置（'.config' 文件）*
  [... <oldconfig_bissbs>]

*注意，这是本指南中创建或修改构建产物的多个步骤中的第一步。本指南中的命令为了简单起见，将这些产物直接存放在源代码树中。如果你更愿意把构建产物单独存放，可以创建一个类似“~/linux-builddir/”的目录，并在本指南所make 调用中加入参`O=~/linux-builddir/`。你还需要让其他命令也指向该目录——其中包``./scripts/config [...]`` 命令，它们需`--file ~/linux-builddir/.config`` 才能找到正确的构建配置

按照上述建议创建 .config 文件时，有两件事很容易出错：

- 如果构建目录中已存在 .config 文件（如“~/linux/.config”），oldconfig 目标会使用它。如果你正是这个意图（见下一步），那完全没问题；但在所有其他情况下，你都应删除它。例如，如果你在遵循本指南时走得更远，后来因遇到问题回到这里从头重新配置，这一点就很重要

- 有时 olddefconfig 无法定位你正在运行内核的 .config 文件，从而会使用默认值，正如指南中简要提到的那样。这种情况下，请检查你的发行版是否在某处提供了该配置文件；如果有，就手动把它放到正确的位置（如“~/linux/.config”）。在某些发行版上
```

    zcat /proc/config.gz > .config

  Once you put it there, run ``make olddefconfig`` again to adjust it to the needs of the kernel about to be built.

```

注意，olddefconfig 目标会把任何未定义的构建选项设置为默认值。如果你更愿意手动设置这些配置选项，请改用 `make oldconfig`。这样，对于每个未定义的配置选项，都会询问你如何继续；如果不确定如何回答，直接按“enter”应用默认值即可。不过请注意，对于二分定位，你通常应使用默认值，否则可能会启用某个新功能，引发看起来像回归的问题（例如由于安全限制）

有时，把一个为某个内核（如 6.1）准备的配置文件用于更旧mainline 发行版时，会发生奇怪的事情——尤其是当后者旧得多时（5.15）。这也是指南中上一步让你启动一切正常的内核的原因之一。因此，如果你手动添.config 文件，务必确保它来自工作正常的内核，而不是来自出现回归的那个

如果你想为另一台机器构建内核，请找到它的内核构建配置；通常 `ls /boot/config-$(uname -r)` 会打印出它的名称。将该文件复制到构建机器上，并保存为 ~/linux/.config；之后运`make olddefconfig` 进行调整

[back to step-by-step guide <oldconfig_bissbs>]


#### 精简内核的构建配


  **禁用那些对你的配置而言明显多余的任意内核模块*
  [... <localmodconfig_bissbs>]

正如分步指南中已经简要说明的：使localmodconfig 时，很容易出现你自行构建的内核缺少某些模块的情况，这些模块对应的任务你在使用make 目标前至少执行过一次。这是因为某些任务依赖的内核模块只有在你首次执行该任务时才会自动加载。所以，如果你自启动内核以来从未执行过该任务，这些模块就不会被加载——在 localmodconfig 看来它们显得多余，于是便被禁用，从而减少需要编译的代码量

你可以通过执行那些通常会自动加载额外内核模块的典型任务来尽量避免这一问题：启动一VM、建VPN 连接、回环挂CD/DVD ISO、挂载网络共享（CIFS、NFS 等），并连接所有外部设备（2FA 密钥、头戴设备、网络摄像头等）以及你平时不使用的文件系统的存储设备（btrfs、ext4、FAT、NTFS、XFS 等）。但很难想到所有可能需要的全部东西——即便内核开发者在这一步也常常忘掉这或那

不要被这种风险困扰，尤其是仅为测试目的而编译内核时：所有通常关键的东西都会在那里。而且如果你忘了某些重要内容，以后可以手动开启缺失的功能，并快速重新运行命令，编译并安装一个具备你所需一切的内核

但如果你打算定期构建并使用自行编译的内核，可以通过记录你的系统在几周内加载了哪些模块来降低风险。你可以`modprobed-db <https://github.com/graysky2/modprobed-db>`_ 将这一过程自动化。之后使`LSMOD=<path>` 
```

  yes '' | make LSMOD='${HOME}'/.config/modprobed.db localmodconfig

```
如果你复制了一份合适的 .config 作为基础（见上一步），该参数也允许你为另一台机器构建精简内核。只需在那台系统上运行 `lsmod > lsmod_foo-machine`，并将生成的文件复制到你的构建主机的主目录。然后运行以下命令，而不
```

  yes '' | make LSMOD=~/lsmod_foo-machine localmodconfig

```

[back to step-by-step guide <localmodconfig_bissbs>]


#### 为即将构建的内核打上标签


  *确保你将构建的所有内核都能通过一种特殊标签和唯一的版本标识符被清晰识别 [... <tagging_bissbs>]

这能让你将自己发行版的内核与本过程中创建的内核区分开来，因为后者的文件或目录名称中会包含local”；它还有助于在启动菜单中挑选正确的条目，并避免混淆你构建的内核——因为在二分定位期间，它们的版本号看起来会有些混乱

[back to step-by-step guide <tagging_bissbs>]


#### 决定是否启用调试符号


  **决定如何处理调试符号* [... <debugsymbols_bissbs>]

当你的内核在后续运行中抛出“panic”、“Oops”、“warning”或“BUG”时，拥有调试符号可能很重要，因为那样你就能找到问题中代码确切发生的位置。但收集和嵌入所需的调试信息需要时间，并且会消耗相当多的空间：2022 年底，用 localmodconfig 精简的典x86 内核，启用调试符号时构建产物约为 5 GB，而禁用时不到 1 GB。生成的内核镜像与模块也会更大，从而增/boot/ 的存储需求和加载时间

因此，如果你想要一个较小的内核，并且以后不太可能去解码栈回溯，就可能需要禁用调试符号以避免这些弊端。如果后来发现确实需要它们，只需按上述方式启用并重新构建内核即可

另一方面，如果你之后很可能需要解码栈回溯，那么在这个过程中就一定要启用它们。Documentation/admin-guide/reporting-issues.rst 中的“Decode failure messages（解码失败信息）”一节对此过程有更详细的说明

[back to step-by-step guide <debugsymbols_bissbs>]


#### 调整构建配置


  *检查你是否想要或需要调整其他一些内核配置选项

根据你的需要，此时你可能想要或必须调整一些内核配置选项


###### 发行版特定的调整


  **Are you running** [... <configmods_bissbs>]

以下小节有助于你避免在本指南中提到的几个普通发行版上构建时出现的已知问题

**Debian:**

- 删除对已失效证书文件的引用，否则它会导致你的构建
```

   ./scripts/config --set-str SYSTEM_TRUSTED_KEYS ''

  Alternatively, download the needed certificate and make that configuration option point to it, as `Debian 手册中有更详细的说明 <https://debian-handbook.info/browse/stable/sect.kernel-compilation.html>`_ -- or generate your own, as explained in Documentation/admin-guide/module-signing.rst.

```

[back to step-by-step guide <configmods_bissbs>]


###### 个人化调


  *如果你想影响配置的其他方面，现在就去做 [... <configmods_bissbs>]

此时你可以使`make menuconfig` `make nconfig` 这样的命令，通过基于文本的用户界面来启用或禁用某些功能；若要使用图形化配置工具，则运`make xconfig`。两者都需要其所依赖工具包（分别ncurses 以及 Qt5 Qt6）的开发库；如果缺少所需内容，会出现错误消息提示你

[back to step-by-step guide <configmods_bissbs>]


#### .config 文件妥善收好


  **在最新更改之后重新处.config，并将其保存在安全的地方*
  [... <saveconfig_bissbs>]

把你准备好的 .config 放在一旁，因为在本指南后续每次开始构建另一个内核之前，你都想要把它复制回构建目录。这是因为在不同版本之间来回切换可能会以奇怪的方式改动 .config 文件；这些改动偶尔会引发副作用，可能扰乱测试，或在有些情况下使你二分定位的结果变得毫无意义

[back to step-by-step guide <saveconfig_bissbs>]


### 尝试用最新代码库复现问题


  *确认该回归不是由某些 .config 变更引起的，并检查它在最新代码库中是否依然存在 [... <introlatestcheck_bissbs>]

对某些读者来说，此时检查最新代码库可能显得没有必要，尤其是如果你已经用发行版提供的内核做过，或者遇到的stable/longterm 系列内部的回归。但在以下这些理由下，我们强烈建议这样做

- 你会在真正开始二分定位之前，就遇到由你的环境引起的任何问题。这将让你很容易区分“这很可能是我环境里的某个问题”与“这次变更需要在二分过程中跳过，因为该阶段的源代码含有一个不相关的问题，导致构建或启动失败”

- 这些步骤能排除你的问题是否由“可用（working）”内核与“损坏（broken）”内核之间构建配置的某些变更引起。例如，当你的发行版在新内核中启用了某个额外的安全特性，而旧内核中该特性被禁用或尚不支持时，就可能出现这种情况。该安全特性可能会妨碍你做的某些事情——这种情况下，从 Linux 内核上游开发者的角度看，你的问题并不构成回归，正Documentation/admin-guide/reporting-regressions.rst 中更详细解释的那样。因此，如果你去二分它，就是在浪费时间

- 如果你回归的成因在最mainline 代码库中已经被修复，那么你的二分定位就白做了。这一点对于你stable/longterm 发行版中遇到的回归同样成立，因为它们往往是由被回移植（backport）的 mainline 变更中的问题引起的——这种情况下，问题必须先mainline 中修复。也许它已经在那里被修复，并且修复正在被回移植的过程中

- 此外，对stable/longterm 系列内部的回归，至关重要的是弄清该问题是否特定于该系列，还是mainline 内核中也会出现，因为报告需要发送给不同的人

  - 特定于某stable/longterm 系列的回归由 stable 团队负责；mainline Linux 开发者可能会在意，也可能不会

  - mainline 中也出现的回归，则是由常规的 Linux 开发者与维护者负责处理；stable 团队不关心，也不需要参与报告，只需要在修复就绪时被告知去回移植它

  如果你把报告发错了对象，它可能会被忽略——即便得到回复，开发者也很可能会让你先判断属于上述哪种情况，再进行深入查看

[back to step-by-step guide <introlatestcheck_bissbs>]


#### 检出最新的 Linux 代码


  **检出最新的 Linux 代码库*
  [... <checkoutmaster_bissbs>]

如果你以后想再次检查是否有一个更新的代码库能修复该问题，请记得再次运行前面提到的那条 `git fetch --shallow-exclude [...]` 命令，以更新你的本地 Git 仓库

[back to step-by-step guide <checkoutmaster_bissbs>]


#### 构建你的内核


  *使用你准备好的配置文件，构建第一个内核的镜像与模块 [... <build_bissbs>]

在这个阶段很多事情都可能出错，但下面的说明能帮你自助解决。另一个小节介绍了如何直接将内核打包成 deb、rpm tar 文件

###### 处理构建错误


当构建错误发生时，它可能是由你机器环境的某些方面引起的，这种情况通常能快速修复；但有时问题出在代码中，只能由开发者修复。仔细查看失败信息，再结合在网上做一些调研，通常能告诉你属于哪一种情况。要进行这样的调查，请重新启动构
```

  make V=1

```
`V=1` 会启用详细输出，这可能是查看真实错误所必需的。为了让错误更容易被发现，这条命令还省略了早前用于让系统所CPU 核心都参与该任务``-j $(nproc --all)``——但这种并行性在出错时也会带来一些混乱

几秒钟后，构建过程应该会再次遇到该错误。现在试着找出描述该问题最关键的那一行。然后在网上搜索该行中最重要、最不通用的一段（比如 4 8 个单词）；避免或去掉任何看起来与特定系统相关的内容，比如你的用户名或`/home/username/linux/` 这样的本地路径名。先用你常用的搜索引擎搜这个字符串，然后再通过 `lore.kernel.org/all/ <https://lore.kernel.org/all/>`_ 搜索 Linux 内核邮件列表

大多数时候，这样能找到解释问题所在的内容；而且往往其中一条结果还会为你的的问题提供解决方案。如果找不到与你的问题匹配的内容，就换个角度再试，比如修改搜索词，或改用错误信息中的另一行

归根结底，你遇到的大多数问题很可能已经被别人遇到并报告过了。这其中包括那些成因不在你的系统、而在代码中的问题。如果你遇到的是后一类，那么也很可能能为你的为题找到解决方案（如补丁）或变通办法

###### 将内核打


分步指南使用默认make 目标（在 x86 上为 'bzImage' 'modules'）来构建内核的镜像与模块，随后由指南中的后续步骤安装。你也可以改用以下目标之一，直接构建所有内容并直接打包

- `make -j $(nproc --all) bindeb-pkg` 以生deb 

- `make -j $(nproc --all) binrpm-pkg` 以生rpm 

- `make -j $(nproc --all) tarbz2-pkg` 以生bz2 压缩tar 

这里只是为此目的提供的部make 目标，其他目标请`make help`。你也可以在运行 `make -j $(nproc --all)` 之后再使用这些目标，因为它们会拾取已经构建好的所有内容

如果你使用这些目标来生成 deb rpm 包，请忽略分步指南中关于安装和卸载内核的说明；改为使用对应格式的包管理工具（dpkg rpm），或构建在它们之上的包管理工具（apt、aptitude、dnf/yum、zypper 等）来安装和卸载这些包。请注意，用这两make 目标生成的包旨在适用于使用这些格式的各种发行版，因此有时它们的行为会与你发行版的内核包有所不同

[back to step-by-step guide <build_bissbs>]


#### 将内核安装到


  **Install the kernel you just built.** [... <install_bissbs>]

在分步指南中执行命令之后你需要做什么，取决于你的发行版上是否存`/sbin/installkernel` 可执行文件，以及它的实现方式

如果找到installkernel，内核的构建系统会把内核镜像的实际安装工作委托给这个可执行文件，它会执行以下部分或全部任务：

- 在几乎所Linux 发行版上，installkernel 都会把你的内核镜像存/boot/，通常名为boot/vmlinuz-<kernelrelease_id>”；通常它还会在旁边放一个“System.map-<kernelrelease_id>”

- 在大多数发行版上，installkernel 随后会生成一个“initramfs”（有时也叫“initrd”），通常存储为boot/initramfs-<kernelrelease_id>.img”或boot/initrd-<kernelrelease_id>”。普通发行版依赖这个文件来启动，因此务必先执make 目标“modules_install”，否则你发行版initramfs 生成器将无法找到打包进镜像所需的模块

- 在某些发行版上，installkernel 还会为你的内核在引导加载程序的配置中添加一个条目

如果你的发行版缺installkernel 脚本，或只处理了其中一部分任务，你就必须自己完成部分或全部任务。详情请查阅发行版的文档。如果拿不准，可以安
```

   sudo install -m 0600 $(make -s image_name) /boot/vmlinuz-$(make -s kernelrelease)
   sudo install -m 0600 System.map /boot/System.map-$(make -s kernelrelease)

```
现在使用你的发行版为此提供的工具生成 initramfs。之后将你的内核添加到引导加载程序配置中，并重启

[back to step-by-step guide <install_bissbs>]


#### 每个内核的存储需


  *检查内核、其模块以及 initramfs 等其他相关文件消耗了多少存储空间 [... <storagespace_bissbs>]

二分定位过程中构建的内核会在 /boot/ /lib/modules/ 下占用相当多的空间，尤其是当你启用了调试符号时。这使得在二分过程中很容易把卷填满——以至于连早先还能正常工作的内核都可能无法启动。为避免这种情况，你需要知道每个已安装内核通常需要多大空间

注意，多数情况下本指南中使用的模式boot/**$(make -s kernelrelease)**”会匹配启动内核所需的所有文件——但路径和命名方案都不是强制性的。因此在某些发行版上，你需要到不同的位置去查找

[back to step-by-step guide <storagespace_bissbs>]


#### 检查你新构建的内核是否认为自己“tainted（被污染）


  **检查内核是否将自己标记为“tainted（被污染）”*
  [... <tainted_bissbs>]

当发生某些可能导致后续看似完全无关的错误的事情时，Linux 会将自己标记tainted（被污染）。这就是为什么开发者可能会忽略或草率回应来自被污染内核的报告——当然，除非内核正是在所报告缺陷发生的那一刻设置了该标志

因此，你应参Documentation/admin-guide/tainted-kernels.rst 中的说明，去查明内核为何被污染；这样做也符合你自己的利益，否则你的测试可能有问题

[back to step-by-step guide <tainted_bissbs>]


#### 检查基于最mainline 代码库构建的内核


  **验证你构建的新内核是否出现了该缺陷*
  [... <recheckbroken_bissbs>]

你的缺陷或回归没有在你用最新代码库构建的内核上出现，可能有几个原因。以下是最常见的：

- 该缺陷当时已经被修复

- 你怀疑是回归的问题，其实是由你的内核提供商所做的构建配置变更引起的

- 你的问题可能是一个竞态条件，在你的内核上不会显现；精简后的构建配置、不同的调试符号设置、所使用的编译器，以及其他各种因素都可能导致这种情况

- 如果你是stable/longterm 内核遇到的该回归，那么它可能是特定于该系列的问题；本指南的下一步会对此进行检查

[back to step-by-step guide <recheckbroken_bissbs>]


#### 检查基于最stable/longterm 代码库构建的内核


  *你是否正面临某个 stable/longterm 发行版内部的回归，却未能用你刚用最mainline 源代码构建的内核复现它？那么请检查该特定系列的最新代码库是否已经修复了这个问题 [... <recheckstablebroken_bissbs>]

如果这个内核也没有出现该回归，那么大概率就不需要进行二分定位了

[back to step-by-step guide <recheckstablebroken_bissbs>]


### 确保“良好（good）”版本确实工作良


  **检查你构建的内核是否工作正常*
  [... <introworkingcheck_bissbs>]

本节将重新确立一个已知可工作的基础。跳过它也许很诱人，但通常是个坏主意，因为它做了一件重要的事：

它能确保你早前准备的 .config 文件确实按预期工作。这也符合你自己的利益，因为精简配置并非万无一失——在怀疑构建配置可能出了问题之前，你可能会白白构建和测试十个或更多内核

仅这一点就足以成为在此花费时间的理由，但这并非唯一的理由

本指南的许多读者通常运行的是打了补丁的内核，或使用了附加模块，或两者兼有。因此这些内核不被视为“vanilla（原版）”——这样一来，那个发生回归的东西可能从一开始在“良好（good）”版本的 vanilla 构建中就从未正常工作过

对于那些注意到不同系列的 stable/longterm 内核之间出现回归（如 6.0.13..6.1.5）的人，还有第三个理由：它能确保你在过程中早些时候假设为“良好（good）”的内核版本（如 6.0）确实在正常工作

[back to step-by-step guide <introworkingcheck_bissbs>]


#### 构建你自己的“良好（good）”内核版


  *构建你自己的可用（working）内核变体，并检查那个发生回归的功能在它上面是否按预期工作 [... <recheckworking_bisref>]

如果随着新内核损坏的那个功能，在你第一个自行构建的内核上也不工作，请在继续之前找出并解决原因。出现这种情况的原因有很多。以下是一些排查思路

- 检taint 状态以`dmesg` 的输出，也许是某个不相关的问题出错了

- 也许 localmodconfig 做了些奇怪的事，禁用了测试该功能所需的模块？那你可能需要基于最后一个工作正常的内核.config 重新创建一个配置文件，并跳过精简；在 .config 中手动禁用某些功能同样可能奏效，并能减少构建时间

- 也许这根本不是内核回归，而是由某些偶然因素、损坏的 initramfs（也initrd）、新的固件文件，或更新后的用户态软件引起的

- 也许那是你发行版内核中添加的某个功能，而当时的 vanilla Linux 从未支持过？

注意，如果你发现并修复了 .config 文件的问题，你会想用它从最新代码库再构建一个内核，因为你早前对 mainline 以及某个受影stable/longterm 系列的最新版本所做的测试，很可能都是有问题的

[back to step-by-step guide <recheckworking_bisref>]


### 执行二分定位并验证结


  *在完成了所有准备工作和预防性构建之后，你现在可以开始二分定位了 [... <introbisect_bissbs>]

本段中的步骤执行并验证二分定位

[back to step-by-step guide <introbisect_bissbs>].


#### 开始二分定


  *开始二分定位，并告Git 早前确定的“良好（good）”与“损坏（bad）”版本 [... <bisectstart_bissbs>]

这将启动二分定位过程；最后一条命令会Git 检出位于“良好（good）”与“损坏（bad）”变更之间大约中点处的某个提交供你测试

[back to step-by-step guide <bisectstart_bissbs>]


#### 从二分点构建内核


  *使用你早前用过的相同命令，从 Git 检出的代码构建、安装并启动一个内核 [... <bisectbuild_bissbs>]

这里有两件事值得注意

- 偶尔，构建内核会失败，或者由于某
```

    git bisect skip

  Git 随后会检出附近另一个提交，运气好的话它应该能更好地工作。之后重新执行这一步

```
- 二分过程中可能会出现那些看起来有点奇怪的版本标识符，这是因为 Linux 内核的各个子系统会在其前一个版本（6.1）完成之前，就为新的 mainline 发行版（6.2）准备变更。因此它们会基于稍早一些的点（6.1-rc1 甚至 6.0）进行开发——然后在 6.1 发布后，未经变基或压缩就合并6.2。这就导致了二分过程中会出现那些看起来有点奇怪的版本标识符

[back to step-by-step guide <bisectbuild_bissbs>]


#### 二分检查点


  **检查那个发生回归的功能在你刚构建的内核中是否工作正常*
  [... <bisecttest_bisref>]

确保你告Git 的内容准确无误：只要错一次，就会让后续的二分定位完全偏离正轨，因此那之后的所有测试都将白费

[back to step-by-step guide <bisecttest_bisref>]


#### 收好二分日志


  **Git 的二分日志与当前.config 文件保存在安全的地方*
  [... <bisectlog_bisref>]

如上所述：只要把某一个内核错误地标记为“good”或“bad”，就会让二分定位的最终结果变得毫无用处。这种情况下，你通常不得不从头重新开始二分定位。而日志可以防止这种情况，因为它可能让别人指出二分大概是在哪里跑偏了——这样一来，你也许只需构建几个内核，而不是十个或更多，就能解决问题

.config 文件收好，是因为在你报告回归之后，开发者很有可能会向你要它

[back to step-by-step guide <bisectlog_bisref>]


#### 尝试回退罪魁祸首


  *尝试在最新代码库之上回退罪魁祸首，看是否能修复你的回归 [... <revert_bissbs>]

这是一个可选步骤，但只要有可能你就应当尝试：当你提出二分定位结果时，开发者很有可能会要求你执行这一步。既然你已经进入状态，此时再构建一个内核应该不成问题，不妨一试

分步指南已经涵盖了所有相关内容，只有一件略显少见的情况除外：你是否用某stable/longterm 系列对一个同样出现在 mainline 中的回归做了二分定位，但 Git 无法mainline revert 该提交？那么尝试在受影响stable/longterm 系列revert culprit——如果成功，就改为测试该内核版本

[back to step-by-step guide <revert_bissbs>]

### 在遵循本指南期间及之后的清理步骤


  *During and after following this guide you might want or need to remove some
  of the kernels you installed.* [... <introclosure_bissbs>]

本节中的步骤描述了清理流程

[back to step-by-step guide <introclosure_bissbs>].

#### 在二分定位过程中的清


  *To remove one of the kernels you installed, look up its 'kernelrelease'
  identifier.* [... <makeroom_bissbs>]

你在此过程中安装的内核以后很容易删除，因为它的各个部分只存储在两个位置，且标识清晰。因此，当你手动安装内核（从而绕过了发行版的打包系统）时，无需担心会把机器搞乱：你的内核的各个部分以后都相对容易删除

这两个位置之一/lib/modules/ 下的一个目录，其中保存了每个已安装内核的模块。该目录以内核的 release 标识符命名；因此，要删除你构建的某个内核的所有模块，只需删除它在 /lib/modules/ 中的模块目录即可

另一个位置是 /boot/，安装内核时通常会在其中放置两到五个文件。它们的文件名中通常都包release 名称，但具体文件数量和确切名称在一定程度上取决于你发行版的 installkernel 可执行文件及initramfs 生成器。在某些发行版上，分步指南中提到`kernel-install remove...` 命令会替你删除所有这些文件，同时还会从你bootloader 配置中移除该内核的菜单项。在其他发行版上，这两项任务需要你自己完成。以下命令应当能以交互方式删除某个具有该 release 名称的内核的三个主要文件

```
  rm -i /boot/{System.map,vmlinuz,initr}-6.0-rc1-local-gcafec0cacaca0

```
之后，检/boot/ 中是否还有其他文件名包含 '6.0-rc1-local-gcafec0cacaca0' 的文件，并考虑也将其删除。现在从你的 bootloader 配置中移除该内核的启动项；具体步骤在不同Linux 发行版之间差异很大

注意，手动删除内核的文件或目录时要小心像 '*' 这样的通配符：你可能本想删6.0 6.0.1，却不小心删除了 6.0.13 内核的文件

[back to step-by-step guide <makeroom_bissbs>]

#### 在二分定位之后的清理


  *Once you have finished the bisection, do not immediately remove anything
  you set up, as you might need a few things again.*
  [... <finishingtouch_bissbs>]

当你确实存储空间紧张时，按分步指南所述删除内核可能释放不了你期望的那么多空间。这种情况下，现在也可以考虑一并运`rm -rf ~/linux/*`。这会删除构建产物和 Linux 源码，但会保Git 仓库（~/linux/.git/）——因此一条简单的 `git reset --hard` 就能把源码恢复回来

此时连仓库一并删除可能并不明智：开发者很有可能会要求你再构建一个内核来执行额外的测试——例如测试一个调试补丁或提议的修复。关于如何执行这些操作的细节，可以在 :ref:`Optional tasks: test reverts, patches, or later versions <introoptional_bissbs>` 一节中找到

你之所以想~/kernel-config-working 文件保留几周，也是出于这些额外的测试

[back to step-by-step guide <finishingtouch_bissbs>]

### 测试 revert、补丁或更新的版


  *While or after reporting a bug, you might want or potentially will be asked
  to test reverts, patches, proposed fixes, or other versions.*
  [... <introoptional_bissbs>]

本节中使用的所有命令都应当相当直白，因此除了有一点之外没有太多可补充的：按照说明设置内核 tag 时，确保它不要比示例中用的那个长太多，因为如kernelrelease 标识符超63 个字符就会出问题

[back to step-by-step guide <introoptional_bissbs>].

## 附加信息

### 在另一台机器上构建内核


要在另一台系统上编译内核，只需对分步指南的说明稍作改动

- 在你之后想要安装并测试内核的那台机器上开始遵循本指南

- 在执':ref:`Boot into the working kernel and briefly use the apparently broken feature <bootworking_bissbs>`' 之后，使`lsmod > ~/test-machine-lsmod` 将已加载模块的列表保存到一个文件。然后找到正在运行的内核的构建配置（关于在哪里可以找到它，请参阅 ':ref:`Start defining the build configuration for your kernel <oldconfig_bisref>`'），并将其保存为 '~/test-machine-config-working'。将这两个文件传输到你的构建主机的家目录

- 在构建主机上继续遵循本指南（例如':ref:`Ensure to have enough free space for building [...] <diskspace_bisref>`' 开始）

- 当你到达 ':ref:`Start preparing a kernel build configuration[...] <oldconfig_bissbs>`' 时：在第一次运`make olddefconfig` 之前，执行以下命令，将你的配置基于来

```
    cp ~/test-machine-config-working ~/linux/.config

```
- 在接下来 ':ref:`disable any apparently superfluous kernel

```
    yes '' | make localmodconfig LSMOD=~/lsmod_foo-machine localmodconfig

```
- 继续遵循本指南，但忽略那些说明每次都要如何编译、安装并重启进入某个内核的指示。改为构

```
    cp ~/kernel-config-working .config
    make olddefconfig &&
    make -j $(nproc --all) targz-pkg

  This will generate a gzipped tar file whose name is printed in the last
  line shown; for example, a kernel with the kernelrelease identifier
  '6.0.0-rc1-local-g928a87efa423' built for x86 machines usually will
  be stored as '~/linux/linux-6.0.0-rc1-local-g928a87efa423-x86.tar.gz'.

  Copy that file to your test machine's home directory.

```
- 切换到测试机器，检查是否有足够空间容纳另一

```
    sudo tar -xvzf ~/linux-6.0.0-rc1-local-g928a87efa423-x86.tar.gz -C /

  Afterwards :ref:`generate the initramfs and add the kernel to your boot
  loader's configuration <install_bisref>`; on some distributions the following
  command will take care of both these tasks::

    sudo /sbin/installkernel 6.0.0-rc1-local-g928a87efa423 /boot/vmlinuz-6.0.0-rc1-local-g928a87efa423

  Now reboot and ensure you started the intended kernel.

```
这种方式在为另一种架构构建时也同样有效：只需安装交叉编译器，并在每次调用 make 时加上适当的参数（例如 `make ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- [...]`）

### 额外的阅读材


- The `man page for 'git bisect' <https://git-scm.com/docs/git-bisect>`_ and
  `fighting regressions with 'git bisect' <https://git-scm.com/docs/git-bisect-lk2009.html>`_
  in the Git documentation.
- `Working with git bisect <https://nathanchance.dev/posts/working-with-git-bisect/>`_
  from kernel developer Nathan Chancellor.
- `Using Git bisect to figure out when brokenness was introduced <http://webchick.net/node/99>`_.
- `Fully automated bisecting with 'git bisect run' <https://lwn.net/Articles/317154>`_.

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
   https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/Documentation/admin-guide/verify-bugs-and-bisect-regressions.rst
..
   Note: Only the content of this RST file as found in the Linux kernel sources
   is available under CC-BY-4.0, as versions of this text that were processed
   (for example by the kernel's build system) might contain content taken from
   files which use a more restrictive license.