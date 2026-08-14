## Ioctl 缂栧彿

鏈〉鏄唴鏍?ioctl 缂栧彿鍒嗛厤鐧昏琛紝鍒楀嚭浠庣敤鎴风┖闂村彲瑙佺殑鍚?ioctl 鍛戒护锛堟寜鏍囪瘑瀛楁瘝/鏁板瓧涓庡簭鍒楀彿褰掔被锛夛紝骞舵敞鏄庡叾鎵€鍦ㄥご鏂囦欢涓庣敤閫斻€傚畠涓洪┍鍔ㄥ紑鍙戣€呭湪鏂板 ioctl 鏃堕€夋嫨鍞竴缂栧彿銆侀伩鍏嶅啿绐佹彁渚涘弬鑰冿紝涔熸槸 strace 绛夊伐鍏疯В鐮?ioctl 缂栧彿鐨勪緷鎹€?


1999 骞?10 鏈?19 鏃?

Michael Elizabeth Chastain
<mec@shout.net>

濡傛灉浣犳鍦ㄤ负鍐呮牳鏂板 ioctl锛屽簲褰撲娇鐢?<linux/ioctl.h> 涓畾涔夌殑 _IO
瀹忥細

    ====== ===========================
    瀹?    鍙傛暟
    ====== ===========================
    _IO    鏃?
    _IOW   鍐欙紙浠庣敤鎴风┖闂磋鍙栵級
    _IOR   璇伙紙鍐欏叆鐢ㄦ埛绌洪棿锛?
    _IOWR  鍐欏苟璇?
    ====== ===========================

鈥滃啓鈥濆拰鈥滆鈥濇槸浠庣敤鎴风殑瑙掑害鑰岃█锛屾濡傜郴缁熻皟鐢ㄢ€渨rite鈥濆拰鈥渞ead鈥濄€?
渚嬪锛孲ET_FOO ioctl 搴斾负 _IOW锛屽敖绠″唴鏍稿疄闄呬笂浼氫粠鐢ㄦ埛绌洪棿璇诲彇鏁版嵁锛?
GET_FOO ioctl 搴斾负 _IOR锛屽敖绠″唴鏍稿疄闄呬笂浼氬悜鐢ㄦ埛绌洪棿鍐欏叆鏁版嵁銆?

瀹忕殑绗竴涓弬鏁版槸涓嬭〃涓敤浜庢爣璇嗙殑瀛楁瘝鎴栨暟瀛椼€傜敱浜庨┍鍔ㄦ暟閲忎紬澶氾紝
璁稿椹卞姩涓庡叾浠栭┍鍔ㄥ叡浜悓涓€涓瓧姣嶇殑涓€閮ㄥ垎銆?

濡傛灉浣犳鍦ㄤ负鏂拌澶囩紪鍐欓┍鍔ㄥ苟闇€瑕佷竴涓瓧姣嶏紝璇烽€夊彇涓€涓湁瓒冲鎵╁睍
绌洪棿鐨勬湭浣跨敤鍖哄潡锛?2 鍒?256 涓?ioctl 鍛戒护閫氬父瓒冲銆備綘鍙互閫氳繃淇敼
姝ゆ枃浠跺苟缁忕敱 :doc:`甯歌鐨勮ˉ涓佹彁浜ゆ祦绋?
</process/submitting-patches>` 鎻愪氦琛ヤ竵鏉ョ櫥璁拌鍖哄潡銆?

绗簩涓弬鏁版槸涓€涓簭鍒楀彿锛岀敤浜庡尯鍒嗕笉鍚岀殑 ioctl銆傜涓変釜鍙傛暟锛堜笉閫傜敤浜?
_IO锛夋槸杩涘嚭鍐呮牳鐨勬暟鎹被鍨嬶紙渚嬪 'int' 鎴?'struct foo'锛夈€?

   涓嶈浣跨敤 sizeof(arg) 浣滀负绗笁涓弬鏁帮紝鍥犱负杩欎細瀵艰嚧浣犵殑 ioctl 璇互涓?
   瀹冧紶鍏ヤ簡涓€涓?size_t 绫诲瀷鐨勫弬鏁般€?

鏈変簺璁惧浣跨敤鍏朵富璁惧鍙蜂綔涓烘爣璇嗭紱鍙瀹冩槸鍞竴鐨勶紝杩欐牱鍋氬氨鍙互銆?
鏈変簺璁惧鍒欎笉瑙勫垯锛屽畬鍏ㄤ笉閬靛惊浠讳綍绾﹀畾銆?

閬靛惊姝ょ害瀹氭湁濡備笅濂藉锛?

(1) 淇濇寔 ioctl 鍏ㄥ眬鍞竴鏈夊姪浜庨敊璇鏌ワ細
    濡傛灉鏌愪釜绋嬪簭鍦ㄩ敊璇殑璁惧涓婅皟鐢?ioctl锛屽畠浼氬緱鍒颁竴涓敊璇紝
    鑰屼笉鏄煇浜涙剰澶栬涓恒€?

(2) 'strace' 鐨勬瀯寤鸿繃绋嬩細鑷姩鎵惧埌鐢ㄨ繖浜涘畯瀹氫箟鐨?ioctl 缂栧彿銆?

(3) 褰撶紪鍙峰敮涓€鏃讹紝'strace' 鑳藉灏嗙紪鍙疯В鐮佸洖鏈夌敤鐨勫悕绉般€?

(4) 褰撲娇鐢ㄦ绾﹀畾鏉ュ畾涔?ioctl 缂栧彿鏃讹紝瀵绘壘 ioctl 鐨勪汉鍙互鏇磋交鏉惧湴
    鐢?grep 鏌ユ壘瀹冧滑銆?

(5) 閬靛惊绾﹀畾鏃讹紝椹卞姩浠ｇ爜鍙互浣跨敤閫氱敤浠ｇ爜鍦ㄧ敤鎴风┖闂翠笌鍐呮牳绌洪棿涔嬮棿
    澶嶅埗鍙傛暟銆?

鏈〃鍒楀嚭浜嗕粠鐢ㄦ埛绌洪棿鍙鐨?ioctl锛屼笉鍖呮嫭鏉ヨ嚜 drivers/staging/ 鐨勩€?
====  =====  ========================================================= ================================================================
Code  Seq#    Include File                                             Comments
      (hex)
====  =====  ========================================================= ================================================================
0x00  00-1F  linux/fs.h                                                鍐茬獊锛?
0x00  00-1F  scsi/scsi_ioctl.h                                         鍐茬獊锛?
0x00  00-1F  linux/fb.h                                                鍐茬獊锛?
0x00  00-1F  linux/wavefront.h                                         鍐茬獊锛?
0x02  all    linux/fd.h
0x03  all    linux/hdreg.h
0x04  D2-DC  linux/umsdos_fs.h                                         鑷?2.6.11 璧峰凡搴熷純锛屼絾涓嶈澶嶇敤杩欎簺銆?
0x06  all    linux/lp.h
0x07  9F-D0  linux/vmw_vmci_defs.h, uapi/linux/vm_sockets.h
0x09  all    linux/raid/md_u.h
0x10  00-0F  drivers/char/s390/vmcp.h
0x10  10-1F  arch/s390/include/uapi/sclp_ctl.h
0x10  20-2F  arch/s390/include/uapi/asm/hypfs.h
0x12  all    linux/fs.h                                                BLK* ioctl 鍛戒护
             linux/blkpg.h
             linux/blkzoned.h
             linux/blk-crypto.h
0x15  all    linux/fs.h                                                FS_IOC_* ioctl 鍛戒护
0x1b  all                                                              InfiniBand 瀛愮郴缁?
                                                                       <http://infiniband.sourceforge.net/>
0x20  all    drivers/cdrom/cm206.h
0x22  all    scsi/sg.h
0x3E  00-0F  linux/counter.h                                           <mailto:linux-iio@vger.kernel.org>
'!'   00-1F  uapi/linux/seccomp.h
'#'   00-3F                                                            IEEE 1394 瀛愮郴缁?
                                                                       鏁翠釜瀛愮郴缁熺殑鍖哄潡
'$'   00-0F  linux/perf_counter.h, linux/perf_event.h
'%'   00-0F  include/uapi/linux/stm.h                                  绯荤粺璺熻釜妯″潡锛圫TM锛夊瓙绯荤粺
                                                                       <mailto:alexander.shishkin@linux.intel.com>
'&'   00-07  drivers/firewire/nosy-user.h
'*'   00-1F  uapi/linux/user_events.h                                  鐢ㄦ埛浜嬩欢瀛愮郴缁?
                                                                       <mailto:linux-trace-kernel@vger.kernel.org>
'1'   00-1F  linux/timepps.h                                           鏉ヨ嚜 Ulrich Windl 鐨?PPS 宸ュ叿鍖?
                                                                       <ftp://ftp.de.kernel.org/pub/linux/daemons/ntp/PPS/>
'2'   01-04  linux/i2o.h
'3'   00-0F  drivers/s390/char/raw3270.h                               鍐茬獊锛?
'3'   00-1F  linux/suspend_ioctls.h,                                   鍐茬獊锛?
             kernel/power/user.c
'8'   all                                                              SNP8023 楂樼骇缃戝崱
                                                                       <mailto:mcr@solidum.com>
';'   64-7F  linux/vfio.h
';'   80-FF  linux/iommufd.h
'='   00-3f  uapi/linux/ptp_clock.h                                    <mailto:richardcochran@gmail.com>
'@'   00-0F  linux/radeonfb.h                                          鍐茬獊锛?
'@'   00-0F  drivers/video/aty/aty128fb.c                              鍐茬獊锛?
'A'   00-1F  linux/apm_bios.h                                          鍐茬獊锛?
'A'   00-0F  linux/agpgart.h,                                          鍐茬獊锛?
             drivers/char/agp/compat_ioctl.h
'A'   00-7F  sound/asound.h                                            鍐茬獊锛?
'B'   00-1F  linux/cciss_ioctl.h                                       鍐茬獊锛?
'B'   00-0F  include/linux/pmu.h                                       鍐茬獊锛?
'B'   C0-FF  楂樼骇 bbus                                             <mailto:maassen@uni-freiburg.de>
'B'   00-0F  xen/xenbus_dev.h                                          鍐茬獊锛?
'C'   all    linux/soundcard.h                                         鍐茬獊锛?
'C'   01-2F  linux/capi.h                                              鍐茬獊锛?
'C'   F0-FF  drivers/net/wan/cosa.h                                    鍐茬獊锛?
'D'   all    arch/s390/include/asm/dasd.h
'D'   40-5F  drivers/scsi/dpt/dtpi_ioctl.h                             鑷?2022 璧峰凡搴熷純
'D'   05     drivers/scsi/pmcraid.h
'E'   all    linux/input.h                                             鍐茬獊锛?
'E'   00-0F  xen/evtchn.h                                              鍐茬獊锛?
'F'   all    linux/fb.h                                                鍐茬獊锛?
'F'   01-02  drivers/scsi/pmcraid.h                                    鍐茬獊锛?
'F'   20     drivers/video/fsl-diu-fb.h                                鍐茬獊锛?
'F'   20     linux/ivtvfb.h                                            鍐茬獊锛?
'F'   20     linux/matroxfb.h                                          鍐茬獊锛?
'F'   20     drivers/video/aty/atyfb_base.c                            鍐茬獊锛?
'F'   00-0F  video/da8xx-fb.h                                          鍐茬獊锛?
'F'   80-8F  linux/arcfb.h                                             鍐茬獊锛?
'F'   DD     video/sstfb.h                                             鍐茬獊锛?
'G'   00-3F  drivers/misc/sgi-gru/grulib.h                             鍐茬獊锛?
'G'   00-0F  xen/gntalloc.h, xen/gntdev.h                              鍐茬獊锛?
'H'   00-7F  linux/hiddev.h                                            鍐茬獊锛?
'H'   00-0F  linux/hidraw.h                                            鍐茬獊锛?
'H'   01     linux/mei.h                                               鍐茬獊锛?
'H'   02     linux/mei.h                                               鍐茬獊锛?
'H'   03     linux/mei.h                                               鍐茬獊锛?
'H'   00-0F  sound/asound.h                                            鍐茬獊锛?
'H'   20-40  sound/asound_fm.h                                         鍐茬獊锛?
'H'   80-8F  sound/sfnt_info.h                                         鍐茬獊锛?
'H'   10-8F  sound/emu10k1.h                                           鍐茬獊锛?
'H'   10-1F  sound/sb16_csp.h                                          鍐茬獊锛?
'H'   10-1F  sound/hda_hwdep.h                                         鍐茬獊锛?
'H'   40-4F  sound/hdspm.h                                             鍐茬獊锛?
'H'   40-4F  sound/hdsp.h                                              鍐茬獊锛?
'H'   90     sound/usb/usx2y/usb_stream.h
'H'   00-0F  uapi/misc/habanalabs.h                                    鍐茬獊锛?
'H'   A0     uapi/linux/usb/cdc-wdm.h
'H'   C0-F0  net/bluetooth/hci.h                                       鍐茬獊锛?
'H'   C0-DF  net/bluetooth/hidp/hidp.h                                 鍐茬獊锛?
'H'   C0-DF  net/bluetooth/cmtp/cmtp.h                                 鍐茬獊锛?
'H'   C0-DF  net/bluetooth/bnep/bnep.h                                 鍐茬獊锛?
'H'   F1     linux/hid-roccat.h                                        <mailto:erazor_de@users.sourceforge.net>
'H'   F8-FA  sound/firewire.h
'I'   all    linux/isdn.h                                              鍐茬獊锛?
'I'   00-0F  drivers/isdn/divert/isdn_divert.h                         鍐茬獊锛?
'I'   40-4F  linux/mISDNif.h                                           鍐茬獊锛?
'K'   all    linux/kd.h
'L'   00-1F  linux/loop.h                                              鍐茬獊锛?
'L'   10-1F  drivers/scsi/mpt3sas/mpt3sas_ctl.h                        鍐茬獊锛?
'L'   E0-FF  linux/ppdd.h                                              鍔犲瘑纾佺洏璁惧椹卞姩
                                                                       <http://linux01.gwdg.de/~alatham/ppdd.html>
'M'   all    linux/soundcard.h                                         鍐茬獊锛?
'M'   01-16  mtd/mtd-abi.h                                             鍐茬獊锛?
      and    drivers/mtd/mtdchar.c
'M'   01-03  drivers/scsi/megaraid/megaraid_sas.h
'M'   00-0F  drivers/video/fsl-diu-fb.h                                鍐茬獊锛?
'N'   00-1F  drivers/usb/scanner.h
'N'   40-7F  drivers/block/nvme.c
'N'   80-8F  uapi/linux/ntsync.h                                       NT 鍚屾鍘熻
                                                                       <mailto:wine-devel@winehq.org>
'O'   00-06  mtd/ubi-user.h                                            UBI
'P'   all    linux/soundcard.h                                         鍐茬獊锛?
'P'   60-6F  sound/sscape_ioctl.h                                      鍐茬獊锛?
'P'   00-0F  drivers/usb/class/usblp.c                                 鍐茬獊锛?
'P'   01-09  drivers/misc/pci_endpoint_test.c                          鍐茬獊锛?
'P'   00-0F  xen/privcmd.h                                             鍐茬獊锛?
'P'   00-05  linux/tps6594_pfsm.h                                      鍐茬獊锛?
'Q'   all    linux/soundcard.h
'R'   00-1F  linux/random.h                                            鍐茬獊锛?
'R'   01     linux/rfkill.h                                            鍐茬獊锛?
'R'   20-2F  linux/trace_mmap.h
'R'   C0-DF  net/bluetooth/rfcomm.h
'R'   E0     uapi/linux/fsl_mc.h
'S'   all    linux/cdrom.h                                             鍐茬獊锛?
'S'   80-81  scsi/scsi_ioctl.h                                         鍐茬獊锛?
'S'   82-FF  scsi/scsi.h                                               鍐茬獊锛?
'S'   00-7F  sound/asequencer.h                                        鍐茬獊锛?
'T'   all    linux/soundcard.h                                         鍐茬獊锛?
'T'   00-AF  sound/asound.h                                            鍐茬獊锛?
'T'   all    arch/x86/include/asm/ioctls.h                             鍐茬獊锛?
'T'   C0-DF  linux/if_tun.h                                            鍐茬獊锛?
'U'   all    sound/asound.h                                            鍐茬獊锛?
'U'   00-CF  linux/uinput.h                                            鍐茬獊锛?
'U'   00-EF  linux/usbdevice_fs.h
'U'   C0-CF  drivers/bluetooth/hci_uart.h
'V'   all    linux/vt.h                                                鍐茬獊锛?
'V'   all    linux/videodev2.h                                         鍐茬獊锛?
'V'   C0     linux/ivtvfb.h                                            鍐茬獊锛?
'V'   C0     linux/ivtv.h                                              鍐茬獊锛?
'V'   C0     media/si4713.h                                            鍐茬獊锛?
'W'   00-1F  linux/watchdog.h                                          鍐茬獊锛?
'W'   00-1F  linux/wanrouter.h                                         鍐茬獊锛?(pre 3.9)
'W'   00-3F  sound/asound.h                                            鍐茬獊锛?
'W'   40-5F  drivers/pci/switch/switchtec.c
'W'   60-61  linux/watch_queue.h
'X'   all    fs/xfs/xfs_fs.h,                                          鍐茬獊锛?
             fs/xfs/linux-2.6/xfs_ioctl32.h,
             include/linux/falloc.h,
             linux/fs.h,
'X'   all    fs/ocfs2/ocfs_fs.h                                        鍐茬獊锛?
'Z'   14-15  drivers/message/fusion/mptctl.h
'['   00-3F  linux/usb/tmc.h                                           USB 娴嬭瘯涓庢祴閲忚澶?
                                                                       <mailto:gregkh@linuxfoundation.org>
'a'   all    linux/atm*.h, linux/sonet.h                               Linux 涓婄殑 ATM
                                                                       <http://lrcwww.epfl.ch/>
'b'   00-FF                                                            鍐茬獊锛?bit3 vme 涓绘満妗?
                                                                       <mailto:natalia@nikhefk.nikhef.nl>
'b'   00-0F  linux/dma-buf.h                                           鍐茬獊锛?
'c'   00-7F  linux/comstats.h                                          鍐茬獊锛?
'c'   00-7F  linux/coda.h                                              鍐茬獊锛?
'c'   00-1F  linux/chio.h                                              鍐茬獊锛?
'c'   80-9F  arch/s390/include/asm/chsc.h                              鍐茬獊锛?
'c'   A0-AF  arch/x86/include/asm/msr.h 鍐茬獊锛?
'd'   00-FF  linux/char/drm/drm.h                                      鍐茬獊锛?
'd'   02-40  pcmcia/ds.h                                               鍐茬獊锛?
'd'   F0-FF  linux/digi1.h
'e'   all    linux/digi1.h                                             鍐茬獊锛?
'f'   00-1F  linux/ext2_fs.h                                           鍐茬獊锛?
'f'   00-1F  linux/ext3_fs.h                                           鍐茬獊锛?
'f'   00-0F  fs/jfs/jfs_dinode.h                                       鍐茬獊锛?
'f'   00-0F  fs/ext4/ext4.h                                            鍐茬獊锛?
'f'   00-0F  linux/fs.h                                                鍐茬獊锛?
'f'   00-0F  fs/ocfs2/ocfs2_fs.h                                       鍐茬獊锛?
'f'   13-27  linux/fscrypt.h
'f'   81-8F  linux/fsverity.h
'g'   00-0F  linux/usb/gadgetfs.h
'g'   20-2F  linux/usb/g_printer.h
'h'   00-7F                                                            鍐茬獊锛?Charon 鏂囦欢绯荤粺
                                                                       <mailto:zapman@interlan.net>
'h'   00-1F  linux/hpet.h                                              鍐茬獊锛?
'h'   80-8F  fs/hfsplus/ioctl.c
'i'   00-3F  linux/i2o-dev.h                                           鍐茬獊锛?
'i'   0B-1F  linux/ipmi.h                                              鍐茬獊锛?
'i'   80-8F  linux/i8k.h
'i'   90-9F  `linux/iio/*.h`                                           IIO
'j'   00-3F  linux/joystick.h
'k'   00-0F  linux/spi/spidev.h                                        鍐茬獊锛?
'k'   00-05  video/kyro.h                                              鍐茬獊锛?
'k'   10-17  linux/hsi/hsi_char.h                                      HSI 瀛楃璁惧
'l'   00-3F  linux/tcfs_fs.h                                           閫忔槑鍔犲瘑鏂囦欢绯荤粺
                                                                       <http://web.archive.org/web/%2A/http://mikonos.dia.unisa.it/tcfs>
'l'   40-7F  linux/udf_fs_i.h                                          寮€鍙戜腑锛?
                                                                       <https://github.com/pali/udftools>
'm'   00-09  linux/mmtimer.h                                           鍐茬獊锛?
'm'   all    linux/mtio.h                                              鍐茬獊锛?
'm'   all    linux/soundcard.h                                         鍐茬獊锛?
'm'   all    linux/synclink.h                                          鍐茬獊锛?
'm'   00-19  drivers/message/fusion/mptctl.h                           鍐茬獊锛?
'm'   00     drivers/scsi/megaraid/megaraid_ioctl.h                    鍐茬獊锛?
'n'   00-7F  linux/ncp_fs.h and fs/ncpfs/ioctl.c
'n'   80-8F  uapi/linux/nilfs2_api.h                                   NILFS2
'n'   E0-FF  linux/matroxfb.h                                          matroxfb
'o'   00-1F  fs/ocfs2/ocfs2_fs.h                                       OCFS2
'o'   00-03  mtd/ubi-user.h                                            鍐茬獊锛?(OCFS2 and UBI overlaps)
'o'   40-41  mtd/ubi-user.h                                            UBI
'o'   01-A1  `linux/dvb/*.h`                                           DVB
'p'   00-0F  linux/phantom.h                                           鍐茬獊锛?(OpenHaptics needs this)
'p'   00-1F  linux/rtc.h                                               鍐茬獊锛?
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
'u'   00-2F  linux/ublk_cmd.h                                          鍐茬獊锛?
'u'   20-3F  linux/uvcvideo.h                                          USB 瑙嗛绫讳富鏈洪┍鍔?
'u'   40-4f  linux/udmabuf.h                                           鐢ㄦ埛绌洪棿 dma-buf 鏉傞」璁惧
'v'   00-1F  linux/ext2_fs.h                                           鍐茬獊锛?
'v'   00-1F  linux/fs.h                                                鍐茬獊锛?
'v'   00-0F  linux/sonypi.h                                            鍐茬獊锛?
'v'   00-0F  media/v4l2-subdev.h                                       鍐茬獊锛?
'v'   20-27  arch/powerpc/include/uapi/asm/vas-api.h                   VAS API
'v'   C0-FF  linux/meye.h                                              鍐茬獊锛?
'w'   all                                                              CERN SCI 椹卞姩
'y'   00-1F                                                            鍩轰簬鏁版嵁鍖呯殑鐢ㄦ埛鎬侀€氫俊
                                                                       <mailto:zapman@interlan.net>
'z'   00-3F                                                            CAN 鎬荤嚎鍗?鍐茬獊锛?
                                                                       <mailto:hdstich@connectu.ulm.circular.de>
'z'   40-7F                                                            CAN 鎬荤嚎鍗?鍐茬獊锛?
                                                                       <mailto:oe@port.de>
'z'   10-4F  drivers/s390/crypto/zcrypt_api.h                          鍐茬獊锛?
'|'   00-7F  linux/media.h
'|'   80-9F  samples/                                                  浠绘剰绀轰緥涓庢牱渚嬮┍鍔?
0x80  00-1F  linux/fb.h
0x81  00-1F  linux/vduse.h
0x89  00-06  arch/x86/include/asm/sockios.h
0x89  0B-DF  linux/sockios.h
0x89  E0-EF  linux/sockios.h                                           SIOCPROTOPRIVATE range
0x89  F0-FF  linux/sockios.h                                           SIOCDEVPRIVATE range
0x8A  00-1F  linux/eventpoll.h
0x8B  all    linux/wireless.h
0x8C  00-3F                                                            WiNRADiO 椹卞姩
                                                                       <http://www.winradio.com.au/>
0x90  00     drivers/cdrom/sbpcd.h
0x92  00-0F  drivers/usb/mon/mon_bin.c
0x93  60-7F  linux/auto_fs.h
0x94  all    fs/btrfs/ioctl.h                                          Btrfs 鏂囦欢绯荤粺
             and linux/fs.h                                            閮ㄥ垎宸叉彁鍗囧埌 vfs/generic
0x97  00-7F  fs/ceph/ioctl.h                                           Ceph 鏂囦欢绯荤粺
0x99  00-0F                                                            537-Addinboard 椹卞姩
                                                                       <mailto:buk@buks.ipn.de>
0x9A  00-0F  include/uapi/fwctl/fwctl.h
0xA0  all    linux/sdp/sdp.h                                           宸ヤ笟璁惧椤圭洰
                                                                       <mailto:kenji@bitgate.com>
0xA1  0      linux/vtpm_proxy.h                                        TPM 妯℃嫙鍣ㄤ唬鐞嗛┍鍔?
0xA2  all    uapi/linux/acrn.h                                         ACRN 铏氭嫙鏈虹洃鎺у櫒
0xA3  80-8F                                                            绔彛 ACL 寮€鍙戜腑锛?
                                                                       <mailto:tlewis@mindspring.com>
0xA3  90-9F  linux/dtlk.h
0xA4  00-1F  uapi/linux/tee.h                                          閫氱敤 TEE 瀛愮郴缁?
0xA4  00-1F  uapi/asm/sgx.h                                            <mailto:linux-sgx@vger.kernel.org>
0xA5  01-05  linux/surface_aggregator/cdev.h                           Microsoft Surface 骞冲彴绯荤粺鑱氬悎鍣?
                                                                       <mailto:luzmaximilian@gmail.com>
0xA5  20-2F  linux/surface_aggregator/dtx.h                            Microsoft Surface DTX 椹卞姩
                                                                       <mailto:luzmaximilian@gmail.com>
0xAA  00-3F  linux/uapi/linux/userfaultfd.h
0xAB  00-1F  linux/nbd.h
0xAC  00-1F  linux/raw.h
0xAD  00                                                               Netfilter device 寮€鍙戜腑锛?
                                                                       <mailto:rusty@rustcorp.com.au>
0xAE  00-1F  linux/kvm.h                                               鍩轰簬鍐呮牳鐨勮櫄鎷熸満锛圞VM锛?
                                                                       <mailto:kvm@vger.kernel.org>
0xAE  40-FF  linux/kvm.h                                               鍩轰簬鍐呮牳鐨勮櫄鎷熸満锛圞VM锛?
                                                                       <mailto:kvm@vger.kernel.org>
0xAE  20-3F  linux/nitro_enclaves.h                                    Nitro Enclaves
0xAF  00-1F  linux/fsl_hypervisor.h                                    Freescale 铏氭嫙鏈虹洃鎺у櫒
0xB0  all                                                              RATIO 璁惧 寮€鍙戜腑锛?
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
0xCA  00-0F  uapi/misc/cxl.h                                           鑷?6.15 璧峰凡搴熷純
0xCA  10-2F  uapi/misc/ocxl.h
0xCA  80-BF  uapi/scsi/cxlflash_ioctl.h                                鑷?6.15 璧峰凡搴熷純
0xCB  00-1F                                                            CBM 涓茶 IEC 鎬荤嚎 寮€鍙戜腑锛?
                                                                       <mailto:michael.klein@puffin.lb.shuttle.de>
0xCC  00-0F  drivers/misc/ibmvmc.h                                     pseries VMC 椹卞姩
0xCD  01     linux/reiserfs_fs.h                                       鑷?6.13 璧峰凡搴熷純
0xCE  01-02  uapi/linux/cxl_mem.h                                      Compute Express Link Memory Devices
0xCF  02     fs/smb/client/cifs_ioctl.h
0xDD  00-3F                                                            ZFCP 璁惧椹卞姩锛岃 drivers/s390/scsi/
                                                                       <mailto:aherrman@de.ibm.com>
0xE5  00-3F  linux/fuse.h
0xEC  00-01  drivers/platform/chrome/cros_ec_dev.h                     ChromeOS EC 椹卞姩
0xEE  00-09  uapi/linux/pfrut.h                                        骞冲彴鍥轰欢杩愯鏃舵洿鏂颁笌閬ユ祴
0xF3  00-3F  drivers/usb/misc/sisusbvga/sisusb.h                       sisfb锛堝紑鍙戜腑锛?
                                                                       <mailto:thomas@winischhofer.net>
0xF6  all                                                              LTTng Linux 璺熻釜宸ュ叿闆嗕笅涓€浠?
                                                                       <mailto:mathieu.desnoyers@efficios.com>
0xF8  all    arch/x86/include/uapi/asm/amd_hsmp.h                      AMD HSMP EPYC 绯荤粺绠＄悊鎺ュ彛椹卞姩
                                                                       <mailto:nchatrad@amd.com>
0xF9  00-0F  uapi/misc/amd-apml.h                                      AMD 杈瑰甫绯荤粺绠＄悊鎺ュ彛椹卞姩
                                                                       <mailto:naveenkrishna.chatradhi@amd.com>
0xFD  all    linux/dm-ioctl.h
0xFE  all    linux/isst_if.h
====  =====  ========================================================= ================================================================
