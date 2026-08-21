## Linux NTFS 文件系统驱动


   - 概述（Overview   - 工具支持（Utilities support   - 支持的挂载选项（Supported mount options

## 概述（Overview

NTFS 是一Linux 内核文件系统驱动，为 NTFS 卷提供完整的读写支持。它面向高性能、现代内核基础设施（iomap、folio）而设计，并注重长期稳定的维护

## 工具支持（Utilities support

名为 ntfsprogs-plus NTFS 工具项目提供mkfs.ntfs、fsck.ntfs 以及其他相关工具（例ntfsinfo、ntfsclone 等），用于创建、检查和维护 NTFS 卷。这些工具既可用于配xfstests 进行文件系统测试，也可用于恢复损坏的 NTFS 设备
该项目位于：

  https://github.com/ntfsprogs-plus/ntfsprogs-plus


## 支持的挂载选项（Supported mount options

NTFS 驱动支持以下挂载选项
======================= ====================================================
iocharset=name          用于转换用户可见文件名所使用的编码与 16                         Unicode 字符之间字符集
nls=name                已废弃选项。仍受支持，但今后请使用
                        iocharset=name銆。
uid=
gid=
umask=                  提供默认的所有者、组和访问模式掩码。这些选项                        行为mount(8) 中所述一致。默认情况下，文目录
                        root 所有，她拥有读写权限，以及对目录的
                        浏览权限。其他任何人都没有任何访问权限。即默认情况                        所有文件的模式rw-------，目录为 rwx------                        这是默认 fmask=0177 dmask=0077 的结果                        使用 umask 0 将赋予所有人全部权限，即所有文                        和目录的模式都为 rwxrwxrwx
fmask=
dmask=                  与同时作用于文件和目录的 umask 不同，fmask                         作用于文件，dmask 仅作用于目录
showmeta=<BOOL>
show_sys_files=<BOOL>   如果指定show_sys_files，则在目录列表中显示
                        系统文件。否则默认行为是隐藏系统文件                        请注意，即使指定show_sys_files$MFT" 也会                        glibc 中的缺陷/特性而不可见。此外，无论是否指定
                        show_sys_files，所有文件都可通过名称访问，即你始                        可以执行 "ls -l \$UpCase" 来专门显示包Unicode
                        大写表的系统文件
case_sensitive=<BOOL>   如果指定case_sensitive，则将所有文件名视为
                        大小写敏感，并在 POSIX 命名空间中创建文件名
                        （默认行为）。注意，Linux NTFS 驱动永不会创                        短文件名，并在重命名/删除相应长文件名时将其移除                        注意，如果短文件名存在，文件仍可通过短文件名访问
nocase=<BOOL>           如果指定nocase，则以不区分大小写的方式处理
                        文件名
disable_sparse=<BOOL>   如果指定disable_sparse，则在此卷上（仅在本
                        次挂载期间）禁用在文件内部创建稀疏区域（即空洞）                        默认情况下启用稀疏区域的创建，这与传Unix 文件
                        系统的行为一致
errors=opt              指定 NTFS 在严重错误时的行为：panic、以只读模式
                        重新挂载分区，或不做任何处理继续运行（默认行为）
mft_zone_multiplier=    设置卷的 MFT 区域倍数（该设置不会跨挂载保持，
                        可以在不同挂载之间更改，但不能在重新挂载时更改）                        允许取1 4 为默认值。MFT 区域倍数决定                        为卷上的 MFT 保留多少空间。如果其他所有空间都用尽                        MFT 区域将动态缩小，因此这不会影响可用空间的大小                        然而，它可能通过影响 MFT 的碎片化而对性能产生影响                        一般情况下使用默认值。如果你有大量小文件，则使用
                        较大的值。各取值含义如下：

                        =====   =================================
                        取     MFT 区域大小（占卷大小的百分比）
                        =====   =================================
                          1             12.5%
                          2             25%
                          3             37.5%
                          4             50%
                        =====   =================================

                        注意此选项对只读挂载无意义
preallocated_size=      设置预分配大小，以优化小 chunk 大小下的
                        runlist 合并开销（默认为 64KB）
acl=<BOOL>              启用 POSIX ACL 支持。指定后，存储在扩展属性中                        POSIX ACL 将被强制执行。默认关闭。需要启用内                        配置 NTFS_FS_POSIX_ACL
sys_immutable=<BOOL>    NTFS 系统文件（例$MFTLogFileBitmap                        $UpCase 等）对用户发起的修改不可变，以提供额                        安全性。默认关闭
nohidden=<BOOL>         隐藏带有 Windows "hidden" 属性的文件和目录                        默认显示隐藏项
hide_dot_files=<BOOL>   隐藏以圆点（"."）开头的名称。默认显示点文件                        启用后，"." 开头创建的文件和目录将从目                        列表中隐藏
windows_names=<BOOL>    拒绝创建/重命名包Windows 上不允许的字符或
                        保留设备名的文件（例CON、NUL、AUX、COM1                        LPT1 等）。默认关闭discard=<BOOL>          在文件删截断时释放的簇上发出块设discard                        以通知底层存储======================= ====================================================
