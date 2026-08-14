## Linux 榄旀暟锛坢agic number锛?

鏈枃浠舵槸姝ｅ湪浣跨敤鐨勯瓟鏁扮殑鐧昏鍐屻€傚綋浣犲悜鏌愪釜缁撴瀯浣撲腑娣诲姞涓€涓瓟鏁版椂锛屼篃搴斿皢鍏?娣诲姞鍒版湰鏂囦欢涓紝鍥犱负鏈€濂借鍚勭缁撴瀯浣撴墍浣跨敤鐨勯瓟鏁颁繚鎸佸敮涓€銆?
鐢ㄩ瓟鏁颁繚鎶ゅ唴鏍告暟鎹粨鏋勬槸涓€涓?*闈炲父**濂界殑涓绘剰銆傝繖浣夸綘鑳藉鍦ㄨ繍琛屾椂妫€鏌?(a)
鏌愪釜缁撴瀯浣撴槸鍚﹁鐮村潖锛屾垨 (b) 浣犳槸鍚﹀悜鏌愪釜渚嬬▼浼犻€掍簡閿欒鐨勭粨鏋勪綋銆傚悗鑰呭挨鍏?鏈夌敤鈥斺€旂壒鍒槸褰撲綘閫氳繃 void * 鎸囬拡浼犻€掓寚鍚戠粨鏋勪綋鐨勬寚閽堟椂銆備緥濡傦紝tty 浠ｇ爜
棰戠箒杩欐牱鍋氾紝浠ユ潵鍥炰紶閫掗┍鍔ㄧ壒瀹氬拰绾胯矾瑙勭▼锛坙ine discipline锛夌壒瀹氱殑缁撴瀯浣撱€?
浣跨敤榄旀暟鐨勬柟娉曟槸鍦ㄥ紑澶村０鏄庡畠浠細

```
	struct tty_ldisc {
		int	magic;
		...
	};

```
璇峰湪灏嗘潵鍚戝唴鏍告坊鍔犲寮哄姛鑳芥椂閬靛惊杩欎竴瑙勫垯锛佸畠涓烘垜鑺傜渷浜嗘棤鏁扮殑璋冭瘯鏃堕棿锛?鐗瑰埆鏄湪鏁扮粍瓒婄晫銆佸叾鍚庣殑缁撴瀯浣撹瑕嗗啓鐨勬鎵嬫儏鍐典笅銆備娇鐢ㄨ繖涓€瑙勫垯锛岃繖绫绘儏鍐?鑳藉琚揩閫熶笖瀹夊叏鍦版娴嬪埌銆?
```
					Theodore Ts'o
					31 Mar 94

  The magic table is current to Linux 2.1.55.

					Michael Chastain
					<mailto:mec@shout.net>
					22 Sep 1997

  Now it should be up to date with Linux 2.1.112. Because
  we are in feature freeze time it is very unlikely that
  something will change before 2.2.x. The entries are
  sorted by number field.

					Krzysztof G. Baranowski
					<mailto: kgb@knm.org.pl>
					29 Jul 1998

  Updated the magic table to Linux 2.5.45. Right over the feature freeze,
  but it is possible that some new magic numbers will sneak into the
  kernel before 2.6.x yet.

					Petr Baudis
					<pasky@ucw.cz>
					03 Nov 2002

  Updated the magic table to Linux 2.5.74.

					Fabian Frederick
					<ffrederick@users.sourceforge.net>
					09 Jul 2003


```
===================== ================ ======================== ==========================================
Magic Name            Number           Structure                File
===================== ================ ======================== ==========================================
PG_MAGIC              'P'              pg_{read,write}_hdr      `include/uapi/linux/pg.h`
APM_BIOS_MAGIC        0x4101           apm_user                 `arch/x86/kernel/apm_32.c`
FASYNC_MAGIC          0x4601           fasync_struct            `include/linux/fs.h`
SLIP_MAGIC            0x5302           slip                     `drivers/net/slip/slip.h`
KV_MAGIC              0x5f4b565f       kernel_vars_s            `arch/mips/include/asm/sn/klkernvars.h`
CODA_MAGIC            0xC0DAC0DA       coda_file_info           `fs/coda/coda_fs_i.h`
CCB_MAGIC             0xf2691ad2       ccb                      `drivers/scsi/ncr53c8xx.c`
QUEUE_MAGIC_FREE      0xf7e1c9a3       queue_entry              `drivers/scsi/arm/queue.c`
QUEUE_MAGIC_USED      0xf7e1cc33       queue_entry              `drivers/scsi/arm/queue.c`
NMI_MAGIC             0x48414d4d455201 nmi_s                    `arch/mips/include/asm/sn/nmi.h`
===================== ================ ======================== ==========================================
