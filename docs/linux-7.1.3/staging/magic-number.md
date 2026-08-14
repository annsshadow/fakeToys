## Linux 魔数（magic number）


本文件是正在使用的魔数的登记册。当你向某个结构体中添加一个魔数时，也应将其
添加到本文件中，因为最好让各种结构体所使用的魔数保持唯一。

用魔数保护内核数据结构是一个**非常**好的主意。这使你能够在运行时检查 (a)
某个结构体是否被破坏，或 (b) 你是否向某个例程传递了错误的结构体。后者尤其
有用——特别是当你通过 void * 指针传递指向结构体的指针时。例如，tty 代码
频繁这样做，以来回传递驱动特定和线路规程（line discipline）特定的结构体。

使用魔数的方法是在开头声明它们：

```
	struct tty_ldisc {
		int	magic;
		...
	};

```
请在将来向内核添加增强功能时遵循这一规则！它为我节省了无数的调试时间，
特别是在数组越界、其后的结构体被覆写的棘手情况下。使用这一规则，这类情况
能够被快速且安全地检测到。

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
