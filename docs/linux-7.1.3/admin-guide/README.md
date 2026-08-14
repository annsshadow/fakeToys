
## Linux 内核 6.x 版本 <http://kernel.org/>


这些是 Linux 6 版本的发行说明。请仔细阅读，因为它们说明了这一切的来龙去脉，
解释了如何安装内核，以及出错时该怎么办。

### 什么是 Linux？


  Linux 是 Unix 操作系统的克隆版本，由 Linus Torvalds 在来自网络各地的、松散
  协作的黑客团队的协助下从零开始编写而成。它的目标是符合 POSIX 与单一 UNIX
  规范（Single UNIX Specification）。

  它具备现代功能完备的 Unix 所应具备的所有特性，包括真正的多任务、虚拟内存、
  共享库、按需加载、共享的写时复制可执行文件、完善的内存管理，以及包含 IPv4
  与 IPv6 的多栈网络。

  它依据 GNU 通用公共许可证第 2 版（GPL v2）发行——更多细节请参阅随附的
  COPYING 文件。

### 它运行在哪些硬件上？


  尽管 Linux 最初是为 32 位 x86 PC（386 或更高）开发的，但如今它也能运行在
  （至少）Compaq Alpha AXP、Sun SPARC 与 UltraSPARC、Motorola 68000、PowerPC、
  PowerPC64、ARM、Hitachi SuperH、Cell、IBM S/390、MIPS、HP PA-RISC、Intel
  IA-64、DEC VAX、AMD x86-64、Xtensa 以及 ARC 架构上。

  Linux 很容易移植到大多数通用的 32 位或 64 位架构上，只要它们拥有分页内存
  管理单元（PMMU）以及 GNU C 编译器（gcc，即 GNU 编译器集合 GCC 的一部分）的
  移植版本。Linux 也被移植到了若干没有 PMMU 的架构上，不过那样功能显然会受到
  一定限制。
  Linux 还被移植到了它自身之上。现在你可以把内核作为用户空间应用程序来运行——
  这被称为 UserMode Linux（UML）。

### 文档


 - 无论是互联网上的电子形式，还是书籍中，都有大量文档可用，既有 Linux 专属的，
   也有关于一般 UNIX 问题的。我建议在任意 Linux FTP 站点上的文档子目录中查找
   LDP（Linux 文档计划，Linux Documentation Project）的书籍。本 README 并非
   关于该系统的文档：还有更好的资料来源。

 - Documentation/ 子目录下有各种 README 文件：例如，它们通常包含某些特定驱动
   的内核相关安装说明。请阅读 Documentation/process/changes.rst <changes> 文件，
   因为它包含了升级内核可能引发的问题的相关信息。

### 安装内核源代码


 - 如果你安装的是完整源代码，请把内核压缩包放在一个你拥有权限的目录中（例如
   你的主目录），并
```

     xz -cd linux-6.x.tar.xz | tar xvf -

   Replace "X" with the version number of the latest kernel.

   Do NOT use the /usr/src/linux area! This area has a (usually
   incomplete) set of kernel headers that are used by the library header
   files.  They should match the library, and not get messed up by
   whatever the kernel-du-jour happens to be.

 - You can also upgrade between 6.x releases by patching.  Patches are
   distributed in the xz format.  To install by patching, get all the
   newer patch files, enter the top level directory of the kernel source
   (linux-6.x) and execute::

     xz -cd ../patch-6.x.xz | patch -p1

   Replace "x" for all versions bigger than the version "x" of your current
   source tree, **in_order**, and you should be ok.  You may want to remove
   the backup files (some-file-name~ or some-file-name.orig), and make sure
   that there are no failed patches (some-file-name# or some-file-name.rej).
   If there are, either you or I have made a mistake.

   Unlike patches for the 6.x kernels, patches for the 6.x.y kernels
   (also known as the -stable kernels) are not incremental but instead apply
   directly to the base 6.x kernel.  For example, if your base kernel is 6.0
   and you want to apply the 6.0.3 patch, you must not first apply the 6.0.1
   and 6.0.2 patches. Similarly, if you are running kernel version 6.0.2 and
   want to jump to 6.0.3, you must first reverse the 6.0.2 patch (that is,
   patch -R) **before** applying the 6.0.3 patch. You can read more on this in
   :ref:`Documentation/process/applying-patches.rst <applying_patches>`.

   Alternatively, the script patch-kernel can be used to automate this
   process.  It determines the current kernel version and applies any
   patches found::

     linux/scripts/patch-kernel linux

   The first argument in the command above is the location of the
   kernel source.  Patches are applied from the current directory, but
   an alternative directory can be specified as the second argument.

 - Make sure you have no stale .o files and dependencies lying around::

     cd linux
     make mrproper

   You should now have the sources correctly installed.

```
### 软件要求


  编译并运行 6.x 内核需要各种软件包的最新版本。关于所需的最低版本号以及如何
  获取这些软件包的更新，请参阅 Documentation/process/changes.rst <changes>。
  请注意，使用这些软件包过于陈旧的版本可能导致难以追踪的间接错误，因此不要以为
  在构建或运行过程中出现明显问题时，你才需要去更新软件包。

### 内核的构建目录


  编译内核时，默认所有输出文件都会与内核源代码存放在一起。
  使用 `make O=output/dir` 选项可以让你为输出文件（包括 .config）指定一个
  备用位置。
```

     kernel source code: /usr/src/linux-6.x
     build directory:    /home/name/build/kernel

   To configure and build the kernel, use::

     cd /usr/src/linux-6.x
     make O=/home/name/build/kernel menuconfig
     make O=/home/name/build/kernel
     sudo make O=/home/name/build/kernel modules_install install

   Please note: If the ``O=output/dir`` option is used, then it must be
   used for all invocations of make.

```
### 配置内核


  即使你只是升级一个小版本，也不要跳过这一步。每个发行版都会加入新的配置选项，
  如果配置文件没有按预期设置好，就会出现各种奇怪的问题。如果你想以最少的工作量
  把现有配置带到新版本，可以使用 `make oldconfig`，它只会就新出现的问题向你
  提问。
```

     "make config"      Plain text interface.

     "make menuconfig"  Text based color menus, radiolists & dialogs.

     "make nconfig"     Enhanced text based color menus.

     "make xconfig"     Qt based configuration tool.

     "make gconfig"     GTK based configuration tool.

     "make oldconfig"   Default all questions based on the contents of
                        your existing ./.config file and asking about
                        new config symbols.

     "make olddefconfig"
                        Like above, but sets new symbols to their default
                        values without prompting.

     "make defconfig"   Create a ./.config file by using the default
                        symbol values from either arch/$ARCH/configs/defconfig
                        or arch/$ARCH/configs/${PLATFORM}_defconfig,
                        depending on the architecture.

     "make ${PLATFORM}_defconfig"
                        Create a ./.config file by using the default
                        symbol values from
                        arch/$ARCH/configs/${PLATFORM}_defconfig.
                        Use "make help" to get a list of all available
                        platforms of your architecture.

     "make allyesconfig"
                        Create a ./.config file by setting symbol
                        values to 'y' as much as possible.

     "make allmodconfig"
                        Create a ./.config file by setting symbol
                        values to 'm' as much as possible.

     "make allnoconfig" Create a ./.config file by setting symbol
                        values to 'n' as much as possible.

     "make randconfig"  Create a ./.config file by setting symbol
                        values to random values.

     "make localmodconfig" Create a config based on current config and
                           loaded modules (lsmod). Disables any module
                           option that is not needed for the loaded modules.

                           To create a localmodconfig for another machine,
                           store the lsmod of that machine into a file
                           and pass it in as a LSMOD parameter.

                           Also, you can preserve modules in certain folders
                           or kconfig files by specifying their paths in
                           parameter LMC_KEEP.

                   target$ lsmod > /tmp/mylsmod
                   target$ scp /tmp/mylsmod host:/tmp

                   host$ make LSMOD=/tmp/mylsmod \
                           LMC_KEEP="drivers/usb:drivers/gpu:fs" \
                           localmodconfig

                           The above also works when cross compiling.

     "make localyesconfig" Similar to localmodconfig, except it will convert
                           all module options to built in (=y) options. You can
                           also preserve modules by LMC_KEEP.

     "make kvm_guest.config"   Enable additional options for kvm guest kernel
                               support.

     "make xen.config"   Enable additional options for xen dom0 guest kernel
                         support.

     "make tinyconfig"  Configure the tiniest possible kernel.

   You can find more information on using the Linux kernel config tools
   in Documentation/kbuild/kconfig.rst.

 - NOTES on ``make config``:

    - Having unnecessary drivers will make the kernel bigger, and can
      under some circumstances lead to problems: probing for a
      nonexistent controller card may confuse your other controllers.

    - A kernel with math-emulation compiled in will still use the
      coprocessor if one is present: the math emulation will just
      never get used in that case.  The kernel will be slightly larger,
      but will work on different machines regardless of whether they
      have a math coprocessor or not.

    - The "kernel hacking" configuration details usually result in a
      bigger or slower kernel (or both), and can even make the kernel
      less stable by configuring some routines to actively try to
      break bad code to find kernel problems (kmalloc()).  Thus you
      should probably answer 'n' to the questions for "development",
      "experimental", or "debugging" features.

```
### 编译内核


 - 请确保你至少拥有可用的 gcc 8.1。更多信息，请参阅
   Documentation/process/changes.rst <changes>。

 - 执行 `make` 以创建压缩的内核镜像。如果你安装了 lilo，或者你的发行版带有内核
   安装程序能够识别的安装脚本，也可以执行 `make install`。大多数流行的发行版都
   会带有可被识别的安装脚本。你可能需要先检查你的发行版设置。

   要完成实际的安装，你必须拥有 root 权限，但正常的构建过程都不需要它。不要滥用
   root 之名。

 - 如果你把内核的任何部分配置为 `modules`，你还必须执行 `make modules_install`。

 - 详细的（verbose）内核编译/构建输出：

   通常，内核构建系统运行在相当安静的模式下（但并非完全静默）。不过，有时你或其
   他内核开发人员需要确切地看到编译、链接或其他命令的执行方式。为此，请使用
   “verbose” 构建模式。这是通过传入
```

     make V=1 all

   To have the build system also tell the reason for the rebuild of each
   target, use ``V=2``.  The default is ``V=0``.

 - Keep a backup kernel handy in case something goes wrong.  This is
   especially true for the development releases, since each new release
   contains new code which has not been debugged.  Make sure you keep a
   backup of the modules corresponding to that kernel, as well.  If you
   are installing a new kernel with the same version number as your
   working kernel, make a backup of your modules directory before you
   do a ``make modules_install``.

   Alternatively, before compiling, use the kernel config option
   "LOCALVERSION" to append a unique suffix to the regular kernel version.
   LOCALVERSION can be set in the "General Setup" menu.

 - In order to boot your new kernel, you'll need to copy the kernel
   image (e.g. .../linux/arch/x86/boot/bzImage after compilation)
   to the place where your regular bootable kernel is found.

 - Booting a kernel directly from a storage device without the assistance
   of a bootloader such as LILO or GRUB, is no longer supported in BIOS
   (non-EFI systems). On UEFI/EFI systems, however, you can use EFISTUB
   which allows the motherboard to boot directly to the kernel.
   On modern workstations and desktops, it's generally recommended to use a
   bootloader as difficulties can arise with multiple kernels and secure boot.
   For more details on EFISTUB,
   see "Documentation/admin-guide/efi-stub.rst".

 - It's important to note that as of 2016 LILO (LInux LOader) is no longer in
   active development, though as it was extremely popular, it often comes up
   in documentation. Popular alternatives include GRUB2, rEFInd, Syslinux,
   systemd-boot, or EFISTUB. For various reasons, it's not recommended to use
   software that's no longer in active development.

 - Chances are your distribution includes an install script and running
   ``make install`` will be all that's needed. Should that not be the case
   you'll have to identify your bootloader and reference its documentation or
   configure your EFI.

```
### 传统 LILO 说明


 - 如果你使用 LILO，内核镜像在 /etc/lilo.conf 文件中指定。内核镜像文件通常是
   /vmlinuz、/boot/vmlinuz、/bzImage 或 /boot/bzImage。要使用新内核，请保存旧
   镜像的副本，并将新镜像复制到旧镜像之上。然后，你必须重新运行 LILO 来更新加载
   映射！否则，你将无法引导新内核镜像。

 - 重新安装 LILO 通常只需运行 /sbin/lilo。你可能希望编辑 /etc/lilo.conf，为旧的
   内核镜像（比如 /vmlinux.old）指定一个条目，以防新内核无法工作。更多信息请参阅
   LILO 文档。

 - 重新安装 LILO 之后，你就大功告成了。关闭系统，重启，享受吧！

 - 如果你需要更改内核镜像中的默认根设备、视频模式等，请酌情使用引导加载程序的
   启动选项。无需为了更改这些参数而重新编译内核。

 - 用新内核重启并享受吧。


### 如果出了问题


如果你遇到了看似由内核缺陷导致的问题，请依照
'Documentation/admin-guide/reporting-issues.rst' 中的说明操作。

理解内核缺陷报告的提示见 'Documentation/admin-guide/bug-hunting.rst'。更多关于
使用 gdb 调试内核的内容，见 'Documentation/process/debugging/gdb-kernel-debugging.rst'
与 'Documentation/process/debugging/kgdb.rst'。
