## s390 SCSI 转储工具（zfcpdump

System z 机器（z900 或更高）提供硬件支持，用于在 SCSI 磁盘上创建系统转储。转储过程通过启动
一个转储工具来发起，该工具必须创建当前（可能已崩溃的）Linux 映像的转储。为了不把崩Linux
的内存被转储工具的数据覆盖，硬件在加载转储工具之前会保存一些内存以及启CPU 的寄存器集合之后存在一SCLP 硬件接口用于获取所保存的内存。当前保32 MB
zfcpdump 实现由一Linux 转储内核和一个用户空间转储工具组成，它们一起被加载32 MB 以下
的已保存内存区域中。zfcpdump 使用 zipl（包含在 s390-tools 包中）安装到 SCSI 磁盘上，以使设备可启动。Linux 系统的操作员随后可以通过启动装有 zfcpdump SCSI 磁盘来触SCSI 转储
用户空间转储工具通过 /proc/vmcore 接口访问崩溃系统的内存。该接口ELF core dump 格式导出
崩溃系统的内存和寄存器。为了访问由硬件保存的内存，SCLP 请求将在 /proc/vmcore 需要该数据创建。崩溃系统内存中未被硬件暂存（stash）的尾部部分可以直接从真实内存复制
要构建支持转储的内核，必须设置内核配置选项 CONFIG_CRASH_DUMP
要获得有效的 zfcpdump 内核配置，使“make zfcpdump_defconfig”
s390 zipl 工具在以下位置查zfcpdump 内核和可选的 initrd/initramfs
- kernel:  <zfcpdump directory>/zfcpdump.image
- ramdisk: <zfcpdump directory>/zfcpdump.rd

zfcpdump 目录s390-tools 包中定义
zfcpdump 的用户空间应用程序可以驻留在 intitramfs initrd 中。它也可以包含在内置的内initramfs 中。该应用程序/proc/vmcore zcore/mem 读取，并将系统转储写SCSI 磁盘
s390-tools 1.24.0 及更高版本构建一个外zfcpdump initramfs，其中带有一个将转储写入
SCSI 分区的用户空间应用程序
有关如何使用 zfcpdump 的更多信息，请参s390 “Using the Dump Tools手册，该书可IBM Knowledge Center 获取https://www.ibm.com/support/knowledgecenter/linuxonibm/liaaf/lnz_r_dt.html
