## Ioctl 编号

本页是内核 ioctl 编号分配登记表，列出从用户空间可见的各 ioctl 命令（按标识字母/数字与序列号归类），并注明其所在头文件与用途。它为驱动开发者在新增 ioctl 时选择唯一编号、避免冲突提供参考，也是 strace 等工具解码 ioctl 编号的依据。


1999 年 10 月 19 日

Michael Elizabeth Chastain
<mec@shout.net>

如果你正在为内核新增 ioctl，应当使用 <linux/ioctl.h> 中定义的 _IO
宏：

    ====== ===========================
    宏     参数
    ====== ===========================
    _IO    无
    _IOW   写（从用户空间读取）
    _IOR   读（写入用户空间）
    _IOWR  写并读
    ====== ===========================

“写”和“读”是从用户的角度而言，正如系统调用“write”和“read”。
例如，SET_FOO ioctl 应为 _IOW，尽管内核实际上会从用户空间读取数据；
GET_FOO ioctl 应为 _IOR，尽管内核实际上会向用户空间写入数据。

宏的第一个参数是下表中用于标识的字母或数字。由于驱动数量众多，
许多驱动与其他驱动共享同一个字母的一部分。

如果你正在为新设备编写驱动并需要一个字母，请选取一个有足够扩展
空间的未使用区块：32 到 256 个 ioctl 命令通常足够。你可以通过修改
此文件并经由 :doc:`常规的补丁提交流程
</process/submitting-patches>` 提交补丁来登记该区块。

第二个参数是一个序列号，用于区分不同的 ioctl。第三个参数（不适用于
_IO）是进出内核的数据类型（例如 'int' 或 'struct foo'）。

   不要使用 sizeof(arg) 作为第三个参数，因为这会导致你的 ioctl 误以为
   它传入了一个 size_t 类型的参数。

有些设备使用其主设备号作为标识；只要它是唯一的，这样做就可以。
有些设备则不规则，完全不遵循任何约定。

遵循此约定有如下好处：

(1) 保持 ioctl 全局唯一有助于错误检查：
    如果某个程序在错误的设备上调用 ioctl，它会得到一个错误，
    而不是某些意外行为。

(2) 'strace' 的构建过程会自动找到用这些宏定义的 ioctl 编号。

(3) 当编号唯一时，'strace' 能够将编号解码回有用的名称。

(4) 当使用此约定来定义 ioctl 编号时，寻找 ioctl 的人可以更轻松地
    用 grep 查找它们。

(5) 遵循约定时，驱动代码可以使用通用代码在用户空间与内核空间之间
    复制参数。

本表列出了从用户空间可见的 ioctl，不包括来自 drivers/staging/ 的。
====  =====  ========================================================= ================================================================
Code  Seq#    Include File                                             Comments
      (hex)
====  =====  ========================================================= ================================================================
0x00  00-1F  linux/fs.h                                                冲突！
0x00  00-1F  scsi/scsi_ioctl.h                                         冲突！
0x00  00-1F  linux/fb.h                                                冲突！
0x00  00-1F  linux/wavefront.h                                         冲突！
0x02  all    linux/fd.h
0x03  all    linux/hdreg.h
0x04  D2-DC  linux/umsdos_fs.h                                         自 2.6.11 起已废弃，但不要复用这些。
0x06  all    linux/lp.h
0x07  9F-D0  linux/vmw_vmci_defs.h, uapi/linux/vm_sockets.h
0x09  all    linux/raid/md_u.h
0x10  00-0F  drivers/char/s390/vmcp.h
0x10  10-1F  arch/s390/include/uapi/sclp_ctl.h
0x10  20-2F  arch/s390/include/uapi/asm/hypfs.h
0x12  all    linux/fs.h                                                BLK* ioctl 命令
             linux/blkpg.h
             linux/blkzoned.h
             linux/blk-crypto.h
0x15  all    linux/fs.h                                                FS_IOC_* ioctl 命令
0x1b  all                                                              InfiniBand 子系统
                                                                       <http://infiniband.sourceforge.net/>
0x20  all    drivers/cdrom/cm206.h
0x22  all    scsi/sg.h
0x3E  00-0F  linux/counter.h                                           <mailto:linux-iio@vger.kernel.org>
'!'   00-1F  uapi/linux/seccomp.h
'#'   00-3F                                                            IEEE 1394 子系统
                                                                       整个子系统的区块
'$'   00-0F  linux/perf_counter.h, linux/perf_event.h
'%'   00-0F  include/uapi/linux/stm.h                                  系统跟踪模块（STM）子系统
                                                                       <mailto:alexander.shishkin@linux.intel.com>
'&'   00-07  drivers/firewire/nosy-user.h
'*'   00-1F  uapi/linux/user_events.h                                  用户事件子系统
                                                                       <mailto:linux-trace-kernel@vger.kernel.org>
'1'   00-1F  linux/timepps.h                                           来自 Ulrich Windl 的 PPS 工具包
                                                                       <ftp://ftp.de.kernel.org/pub/linux/daemons/ntp/PPS/>
'2'   01-04  linux/i2o.h
'3'   00-0F  drivers/s390/char/raw3270.h                               冲突！
'3'   00-1F  linux/suspend_ioctls.h,                                   冲突！
             kernel/power/user.c
'8'   all                                                              SNP8023 高级网卡
                                                                       <mailto:mcr@solidum.com>
';'   64-7F  linux/vfio.h
';'   80-FF  linux/iommufd.h
'='   00-3f  uapi/linux/ptp_clock.h                                    <mailto:richardcochran@gmail.com>
'@'   00-0F  linux/radeonfb.h                                          冲突！
'@'   00-0F  drivers/video/aty/aty128fb.c                              冲突！
'A'   00-1F  linux/apm_bios.h                                          冲突！
'A'   00-0F  linux/agpgart.h,                                          冲突！
             drivers/char/agp/compat_ioctl.h
'A'   00-7F  sound/asound.h                                            冲突！
'B'   00-1F  linux/cciss_ioctl.h                                       冲突！
'B'   00-0F  include/linux/pmu.h                                       冲突！
'B'   C0-FF  高级 bbus                                             <mailto:maassen@uni-freiburg.de>
'B'   00-0F  xen/xenbus_dev.h                                          冲突！
'C'   all    linux/soundcard.h                                         冲突！
'C'   01-2F  linux/capi.h                                              冲突！
'C'   F0-FF  drivers/net/wan/cosa.h                                    冲突！
'D'   all    arch/s390/include/asm/dasd.h
'D'   40-5F  drivers/scsi/dpt/dtpi_ioctl.h                             自 2022 起已废弃
'D'   05     drivers/scsi/pmcraid.h
'E'   all    linux/input.h                                             冲突！
'E'   00-0F  xen/evtchn.h                                              冲突！
'F'   all    linux/fb.h                                                冲突！
'F'   01-02  drivers/scsi/pmcraid.h                                    冲突！
'F'   20     drivers/video/fsl-diu-fb.h                                冲突！
'F'   20     linux/ivtvfb.h                                            冲突！
'F'   20     linux/matroxfb.h                                          冲突！
'F'   20     drivers/video/aty/atyfb_base.c                            冲突！
'F'   00-0F  video/da8xx-fb.h                                          冲突！
'F'   80-8F  linux/arcfb.h                                             冲突！
'F'   DD     video/sstfb.h                                             冲突！
'G'   00-3F  drivers/misc/sgi-gru/grulib.h                             冲突！
'G'   00-0F  xen/gntalloc.h, xen/gntdev.h                              冲突！
'H'   00-7F  linux/hiddev.h                                            冲突！
'H'   00-0F  linux/hidraw.h                                            冲突！
'H'   01     linux/mei.h                                               冲突！
'H'   02     linux/mei.h                                               冲突！
'H'   03     linux/mei.h                                               冲突！
'H'   00-0F  sound/asound.h                                            冲突！
'H'   20-40  sound/asound_fm.h                                         冲突！
'H'   80-8F  sound/sfnt_info.h                                         冲突！
'H'   10-8F  sound/emu10k1.h                                           冲突！
'H'   10-1F  sound/sb16_csp.h                                          冲突！
'H'   10-1F  sound/hda_hwdep.h                                         冲突！
'H'   40-4F  sound/hdspm.h                                             冲突！
'H'   40-4F  sound/hdsp.h                                              冲突！
'H'   90     sound/usb/usx2y/usb_stream.h
'H'   00-0F  uapi/misc/habanalabs.h                                    冲突！
'H'   A0     uapi/linux/usb/cdc-wdm.h
'H'   C0-F0  net/bluetooth/hci.h                                       冲突！
'H'   C0-DF  net/bluetooth/hidp/hidp.h                                 冲突！
'H'   C0-DF  net/bluetooth/cmtp/cmtp.h                                 冲突！
'H'   C0-DF  net/bluetooth/bnep/bnep.h                                 冲突！
'H'   F1     linux/hid-roccat.h                                        <mailto:erazor_de@users.sourceforge.net>
'H'   F8-FA  sound/firewire.h
'I'   all    linux/isdn.h                                              冲突！
'I'   00-0F  drivers/isdn/divert/isdn_divert.h                         冲突！
'I'   40-4F  linux/mISDNif.h                                           冲突！
'K'   all    linux/kd.h
'L'   00-1F  linux/loop.h                                              冲突！
'L'   10-1F  drivers/scsi/mpt3sas/mpt3sas_ctl.h                        冲突！
'L'   E0-FF  linux/ppdd.h                                              加密磁盘设备驱动
                                                                       <http://linux01.gwdg.de/~alatham/ppdd.html>
'M'   all    linux/soundcard.h                                         冲突！
'M'   01-16  mtd/mtd-abi.h                                             冲突！
      and    drivers/mtd/mtdchar.c
'M'   01-03  drivers/scsi/megaraid/megaraid_sas.h
'M'   00-0F  drivers/video/fsl-diu-fb.h                                冲突！
'N'   00-1F  drivers/usb/scanner.h
'N'   40-7F  drivers/block/nvme.c
'N'   80-8F  uapi/linux/ntsync.h                                       NT 同步原语
                                                                       <mailto:wine-devel@winehq.org>
'O'   00-06  mtd/ubi-user.h                                            UBI
'P'   all    linux/soundcard.h                                         冲突！
'P'   60-6F  sound/sscape_ioctl.h                                      冲突！
'P'   00-0F  drivers/usb/class/usblp.c                                 冲突！
'P'   01-09  drivers/misc/pci_endpoint_test.c                          冲突！
'P'   00-0F  xen/privcmd.h                                             冲突！
'P'   00-05  linux/tps6594_pfsm.h                                      冲突！
'Q'   all    linux/soundcard.h
'R'   00-1F  linux/random.h                                            冲突！
'R'   01     linux/rfkill.h                                            冲突！
'R'   20-2F  linux/trace_mmap.h
'R'   C0-DF  net/bluetooth/rfcomm.h
'R'   E0     uapi/linux/fsl_mc.h
'S'   all    linux/cdrom.h                                             冲突！
'S'   80-81  scsi/scsi_ioctl.h                                         冲突！
'S'   82-FF  scsi/scsi.h                                               冲突！
'S'   00-7F  sound/asequencer.h                                        冲突！
'T'   all    linux/soundcard.h                                         冲突！
'T'   00-AF  sound/asound.h                                            冲突！
'T'   all    arch/x86/include/asm/ioctls.h                             冲突！
'T'   C0-DF  linux/if_tun.h                                            冲突！
'U'   all    sound/asound.h                                            冲突！
'U'   00-CF  linux/uinput.h                                            冲突！
'U'   00-EF  linux/usbdevice_fs.h
'U'   C0-CF  drivers/bluetooth/hci_uart.h
'V'   all    linux/vt.h                                                冲突！
'V'   all    linux/videodev2.h                                         冲突！
'V'   C0     linux/ivtvfb.h                                            冲突！
'V'   C0     linux/ivtv.h                                              冲突！
'V'   C0     media/si4713.h                                            冲突！
'W'   00-1F  linux/watchdog.h                                          冲突！
'W'   00-1F  linux/wanrouter.h                                         冲突！ (pre 3.9)
'W'   00-3F  sound/asound.h                                            冲突！
'W'   40-5F  drivers/pci/switch/switchtec.c
'W'   60-61  linux/watch_queue.h
'X'   all    fs/xfs/xfs_fs.h,                                          冲突！
             fs/xfs/linux-2.6/xfs_ioctl32.h,
             include/linux/falloc.h,
             linux/fs.h,
'X'   all    fs/ocfs2/ocfs_fs.h                                        冲突！
'Z'   14-15  drivers/message/fusion/mptctl.h
'['   00-3F  linux/usb/tmc.h                                           USB 测试与测量设备
                                                                       <mailto:gregkh@linuxfoundation.org>
'a'   all    linux/atm*.h, linux/sonet.h                               Linux 上的 ATM
                                                                       <http://lrcwww.epfl.ch/>
'b'   00-FF                                                            冲突！ bit3 vme 主机桥
                                                                       <mailto:natalia@nikhefk.nikhef.nl>
'b'   00-0F  linux/dma-buf.h                                           冲突！
'c'   00-7F  linux/comstats.h                                          冲突！
'c'   00-7F  linux/coda.h                                              冲突！
'c'   00-1F  linux/chio.h                                              冲突！
'c'   80-9F  arch/s390/include/asm/chsc.h                              冲突！
'c'   A0-AF  arch/x86/include/asm/msr.h 冲突！
'd'   00-FF  linux/char/drm/drm.h                                      冲突！
'd'   02-40  pcmcia/ds.h                                               冲突！
'd'   F0-FF  linux/digi1.h
'e'   all    linux/digi1.h                                             冲突！
'f'   00-1F  linux/ext2_fs.h                                           冲突！
'f'   00-1F  linux/ext3_fs.h                                           冲突！
'f'   00-0F  fs/jfs/jfs_dinode.h                                       冲突！
'f'   00-0F  fs/ext4/ext4.h                                            冲突！
'f'   00-0F  linux/fs.h                                                冲突！
'f'   00-0F  fs/ocfs2/ocfs2_fs.h                                       冲突！
'f'   13-27  linux/fscrypt.h
'f'   81-8F  linux/fsverity.h
'g'   00-0F  linux/usb/gadgetfs.h
'g'   20-2F  linux/usb/g_printer.h
'h'   00-7F                                                            冲突！ Charon 文件系统
                                                                       <mailto:zapman@interlan.net>
'h'   00-1F  linux/hpet.h                                              冲突！
'h'   80-8F  fs/hfsplus/ioctl.c
'i'   00-3F  linux/i2o-dev.h                                           冲突！
'i'   0B-1F  linux/ipmi.h                                              冲突！
'i'   80-8F  linux/i8k.h
'i'   90-9F  `linux/iio/*.h`                                           IIO
'j'   00-3F  linux/joystick.h
'k'   00-0F  linux/spi/spidev.h                                        冲突！
'k'   00-05  video/kyro.h                                              冲突！
'k'   10-17  linux/hsi/hsi_char.h                                      HSI 字符设备
'l'   00-3F  linux/tcfs_fs.h                                           透明加密文件系统
                                                                       <http://web.archive.org/web/%2A/http://mikonos.dia.unisa.it/tcfs>
'l'   40-7F  linux/udf_fs_i.h                                          开发中：
                                                                       <https://github.com/pali/udftools>
'm'   00-09  linux/mmtimer.h                                           冲突！
'm'   all    linux/mtio.h                                              冲突！
'm'   all    linux/soundcard.h                                         冲突！
'm'   all    linux/synclink.h                                          冲突！
'm'   00-19  drivers/message/fusion/mptctl.h                           冲突！
'm'   00     drivers/scsi/megaraid/megaraid_ioctl.h                    冲突！
'n'   00-7F  linux/ncp_fs.h and fs/ncpfs/ioctl.c
'n'   80-8F  uapi/linux/nilfs2_api.h                                   NILFS2
'n'   E0-FF  linux/matroxfb.h                                          matroxfb
'o'   00-1F  fs/ocfs2/ocfs2_fs.h                                       OCFS2
'o'   00-03  mtd/ubi-user.h                                            冲突！ (OCFS2 and UBI overlaps)
'o'   40-41  mtd/ubi-user.h                                            UBI
'o'   01-A1  `linux/dvb/*.h`                                           DVB
'p'   00-0F  linux/phantom.h                                           冲突！ (OpenHaptics needs this)
'p'   00-1F  linux/rtc.h                                               冲突！
'p'   40-7F  linux/nvram.h
'p'   80-9F  linux/ppdev.h                                             user-space parport
                                                                       <mailto:tim@cyberelk.net>
'p'   A1-A5  linux/pps.h                                               LinuxPPS
'p'   B1-B3  linux/pps_gen.h                                           LinuxPPS
                                                                       <mailto:giometti@linux.it>
'q'   00-1F  linux/serio.h
'q'   80-FF  linux/telephony.h                                         Internet PhoneJACK, Internet LineJACK
             linux/ixjuser.h                                           <http://web.archive.org/web/%2A/http://www.quicknet.net>
'r'   00-1F  linux/msdos_fs.h and fs/fat/dir.c
's'   all    linux/cdk.h
't'   00-7F  linux/ppp-ioctl.h
't'   80-8F  linux/isdn_ppp.h
't'   90-91  linux/toshiba.h                                           toshiba and toshiba_acpi SMM
'u'   00-1F  linux/smb_fs.h                                            gone
'u'   00-2F  linux/ublk_cmd.h                                          冲突！
'u'   20-3F  linux/uvcvideo.h                                          USB 视频类主机驱动
'u'   40-4f  linux/udmabuf.h                                           用户空间 dma-buf 杂项设备
'v'   00-1F  linux/ext2_fs.h                                           冲突！
'v'   00-1F  linux/fs.h                                                冲突！
'v'   00-0F  linux/sonypi.h                                            冲突！
'v'   00-0F  media/v4l2-subdev.h                                       冲突！
'v'   20-27  arch/powerpc/include/uapi/asm/vas-api.h                   VAS API
'v'   C0-FF  linux/meye.h                                              冲突！
'w'   all                                                              CERN SCI 驱动
'y'   00-1F                                                            基于数据包的用户态通信
                                                                       <mailto:zapman@interlan.net>
'z'   00-3F                                                            CAN 总线卡 冲突！
                                                                       <mailto:hdstich@connectu.ulm.circular.de>
'z'   40-7F                                                            CAN 总线卡 冲突！
                                                                       <mailto:oe@port.de>
'z'   10-4F  drivers/s390/crypto/zcrypt_api.h                          冲突！
'|'   00-7F  linux/media.h
'|'   80-9F  samples/                                                  任意示例与样例驱动
0x80  00-1F  linux/fb.h
0x81  00-1F  linux/vduse.h
0x89  00-06  arch/x86/include/asm/sockios.h
0x89  0B-DF  linux/sockios.h
0x89  E0-EF  linux/sockios.h                                           SIOCPROTOPRIVATE range
0x89  F0-FF  linux/sockios.h                                           SIOCDEVPRIVATE range
0x8A  00-1F  linux/eventpoll.h
0x8B  all    linux/wireless.h
0x8C  00-3F                                                            WiNRADiO 驱动
                                                                       <http://www.winradio.com.au/>
0x90  00     drivers/cdrom/sbpcd.h
0x92  00-0F  drivers/usb/mon/mon_bin.c
0x93  60-7F  linux/auto_fs.h
0x94  all    fs/btrfs/ioctl.h                                          Btrfs 文件系统
             and linux/fs.h                                            部分已提升到 vfs/generic
0x97  00-7F  fs/ceph/ioctl.h                                           Ceph 文件系统
0x99  00-0F                                                            537-Addinboard 驱动
                                                                       <mailto:buk@buks.ipn.de>
0x9A  00-0F  include/uapi/fwctl/fwctl.h
0xA0  all    linux/sdp/sdp.h                                           工业设备项目
                                                                       <mailto:kenji@bitgate.com>
0xA1  0      linux/vtpm_proxy.h                                        TPM 模拟器代理驱动
0xA2  all    uapi/linux/acrn.h                                         ACRN 虚拟机监控器
0xA3  80-8F                                                            端口 ACL 开发中：
                                                                       <mailto:tlewis@mindspring.com>
0xA3  90-9F  linux/dtlk.h
0xA4  00-1F  uapi/linux/tee.h                                          通用 TEE 子系统
0xA4  00-1F  uapi/asm/sgx.h                                            <mailto:linux-sgx@vger.kernel.org>
0xA5  01-05  linux/surface_aggregator/cdev.h                           Microsoft Surface 平台系统聚合器
                                                                       <mailto:luzmaximilian@gmail.com>
0xA5  20-2F  linux/surface_aggregator/dtx.h                            Microsoft Surface DTX 驱动
                                                                       <mailto:luzmaximilian@gmail.com>
0xAA  00-3F  linux/uapi/linux/userfaultfd.h
0xAB  00-1F  linux/nbd.h
0xAC  00-1F  linux/raw.h
0xAD  00                                                               Netfilter device 开发中：
                                                                       <mailto:rusty@rustcorp.com.au>
0xAE  00-1F  linux/kvm.h                                               基于内核的虚拟机（KVM）
                                                                       <mailto:kvm@vger.kernel.org>
0xAE  40-FF  linux/kvm.h                                               基于内核的虚拟机（KVM）
                                                                       <mailto:kvm@vger.kernel.org>
0xAE  20-3F  linux/nitro_enclaves.h                                    Nitro Enclaves
0xAF  00-1F  linux/fsl_hypervisor.h                                    Freescale 虚拟机监控器
0xB0  all                                                              RATIO 设备 开发中：
                                                                       <mailto:vgo@ratio.de>
0xB1  00-1F                                                            PPPoX
                                                                       <mailto:mostrows@styx.uwaterloo.ca>
0xB2  00     arch/powerpc/include/uapi/asm/papr-vpd.h                  powerpc/pseries VPD API
                                                                       <mailto:linuxppc-dev@lists.ozlabs.org>
0xB2  01-02  arch/powerpc/include/uapi/asm/papr-sysparm.h              powerpc/pseries system parameter API
                                                                       <mailto:linuxppc-dev@lists.ozlabs.org>
0xB2  03-05  arch/powerpc/include/uapi/asm/papr-indices.h              powerpc/pseries indices API
                                                                       <mailto:linuxppc-dev@lists.ozlabs.org>
0xB2  06-07  arch/powerpc/include/uapi/asm/papr-platform-dump.h        powerpc/pseries Platform Dump API
                                                                       <mailto:linuxppc-dev@lists.ozlabs.org>
0xB2  08     arch/powerpc/include/uapi/asm/papr-physical-attestation.h powerpc/pseries Physical Attestation API
                                                                       <mailto:linuxppc-dev@lists.ozlabs.org>
0xB2  09     arch/powerpc/include/uapi/asm/papr-hvpipe.h               powerpc/pseries HVPIPE API
                                                                       <mailto:linuxppc-dev@lists.ozlabs.org>
0xB3  00     linux/mmc/ioctl.h
0xB4  00-0F  linux/gpio.h                                              <mailto:linux-gpio@vger.kernel.org>
0xB5  00-0F  uapi/linux/rpmsg.h                                        <mailto:linux-remoteproc@vger.kernel.org>
0xB6  all    linux/fpga-dfl.h
0xB7  all    uapi/linux/remoteproc_cdev.h                              <mailto:linux-remoteproc@vger.kernel.org>
0xB7  all    uapi/linux/nsfs.h                                         <mailto:Andrei Vagin <avagin@openvz.org>>
0xB8  01-02  uapi/misc/mrvl_cn10k_dpi.h                                Marvell CN10K DPI driver
0xB8  all    uapi/linux/mshv.h                                         Microsoft Hyper-V /dev/mshv driver
                                                                       <mailto:linux-hyperv@vger.kernel.org>
0xBA  00-0F  uapi/linux/liveupdate.h                                   Pasha Tatashin
                                                                       <mailto:pasha.tatashin@soleen.com>
0xC0  00-0F  linux/usb/iowarrior.h
0xCA  00-0F  uapi/misc/cxl.h                                           自 6.15 起已废弃
0xCA  10-2F  uapi/misc/ocxl.h
0xCA  80-BF  uapi/scsi/cxlflash_ioctl.h                                自 6.15 起已废弃
0xCB  00-1F                                                            CBM 串行 IEC 总线 开发中：
                                                                       <mailto:michael.klein@puffin.lb.shuttle.de>
0xCC  00-0F  drivers/misc/ibmvmc.h                                     pseries VMC 驱动
0xCD  01     linux/reiserfs_fs.h                                       自 6.13 起已废弃
0xCE  01-02  uapi/linux/cxl_mem.h                                      Compute Express Link Memory Devices
0xCF  02     fs/smb/client/cifs_ioctl.h
0xDD  00-3F                                                            ZFCP 设备驱动，见 drivers/s390/scsi/
                                                                       <mailto:aherrman@de.ibm.com>
0xE5  00-3F  linux/fuse.h
0xEC  00-01  drivers/platform/chrome/cros_ec_dev.h                     ChromeOS EC 驱动
0xEE  00-09  uapi/linux/pfrut.h                                        平台固件运行时更新与遥测
0xF3  00-3F  drivers/usb/misc/sisusbvga/sisusb.h                       sisfb（开发中）
                                                                       <mailto:thomas@winischhofer.net>
0xF6  all                                                              LTTng Linux 跟踪工具集下一代
                                                                       <mailto:mathieu.desnoyers@efficios.com>
0xF8  all    arch/x86/include/uapi/asm/amd_hsmp.h                      AMD HSMP EPYC 系统管理接口驱动
                                                                       <mailto:nchatrad@amd.com>
0xF9  00-0F  uapi/misc/amd-apml.h                                      AMD 边带系统管理接口驱动
                                                                       <mailto:naveenkrishna.chatradhi@amd.com>
0xFD  all    linux/dm-ioctl.h
0xFE  all    linux/isst_if.h
====  =====  ========================================================= ================================================================
