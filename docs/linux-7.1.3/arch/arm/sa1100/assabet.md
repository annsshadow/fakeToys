## Intel Assabet（SA-1110 评估）板


请参见：
http://developer.intel.com

以及来自 John G Dorsey <jd5q@andrew.cmu.edu> 的一些说明：
http://www.cs.cmu.edu/~wearable/software/assabet.html


### 构建内核


```
	make assabet_defconfig
	make oldconfig
	make zImage
```
生成的内核映像应位于 linux/arch/arm/boot/zImage

### 安装引导加载程序


有几个能够为 Assabet 引导 Linux 的引导加载程序可用：

BLOB (http://www.lartmaker.nl/lartware/blob/)

   BLOB LART 项目中使用的引导加载程序。一些贡献的补丁被合并进 BLOB 以添加对 Assabet 的支持
Compaq Bootldr + John Dorsey 用于 Assabet 支持的补(http://www.handhelds.org/Compaq/bootldr.html)
(http://www.wearablegroup.org/software/bootldr/)

   Bootldr Compaq iPAQ Pocket PC 开发的引导加载程序   John Dorsey 制作了附加补丁以添加Assabet JFFS 文件系统的支持
RedBoot (http://sources.redhat.com/redboot/)

   RedBoot Red Hat 基于 eCos RTOS 硬件抽象层开发的引导加载程序   它支Assabet 以及许多其他硬件平台
RedBoot 目前是推荐的选择，因为它是唯一具有网络支持的，并且是维护最活跃的
下面展示了如何使RedBoot 引导 Linux 的简要示例。但首先
你需要将 RedBoot 安装到你的闪存（flash）中。一个已知可用的
预编RedBoot 二进制文件可从以下位置获取：

- ftp://ftp.netwinder.org/users/n/nico/
- ftp://ftp.arm.linux.org.uk/pub/linux/arm/people/nico/
- ftp://ftp.handhelds.org/pub/linux/arm/sa-1100-patches/

查找 redboot-assabet*.tgz。一些安装信息在 redboot-assabet*.txt 中提供

### 初始 RedBoot 配置


此处使用的命令在 RedBoot 用户指南中有解释，该指南可在线获取：
http://sources.redhat.com/ecos/docs.html。请参考它了解说明
如果你有一CF 网卡（我Assabet 套件包含一块来Socket Communications Inc. CF+ LP-E），
你应该强烈考虑使用它进TFTP 文件传输。你必须RedBoot 运行前插入它，因为它无法动态检测到它
```
	fis init -f
```
要初始化非易失性设置，例如你是否想要使BOOTP 
```
	fconfig -i
```


### 灏嗗唴鏍告槧鍍忓啓鍏ラ棯瀛。

首先，内核映像必须被加载RAM 中。如果你zImage 文件

```
	load zImage -r -b 0x100000
```
```
	load -m ymodem -r -b 0x100000
```
```
	fis create "Linux kernel" -b 0x100000 -l 0xc0000
```


### 引导内核


内核仍然需要一个文件系统才能引导。可以加载一ramdisk 映像

```
	load ramdisk_image.gz -r -b 0x800000
```
同样，可以用 Y-Modem 上传代替 TFTP，只需将文件名替换'-y ymodem'
```
	fis load "Linux kernel"
```
```
	exec -b 0x100000 -l 0xc0000
```
ramdisk 映像也可以存储到闪存中，但如下文所述，有更好的用于片上（on-flash）文件系统的方案

### 使用 JFFS2


使用 JFFS2（第二日志闪存文件系统，the Second Journalling Flash File System）可能是将可写文件系统存入闪存最方便的方式JFFS2 与负责底层闪存管理的 MTD 层配合使用。关Linux MTD 的更多信息可在线获取http://www.linux-mtd.infradead.org/。同一站点也提供了带有一些关于创JFFS/JFFS2 映像信息JFFS howto
例如，一个示JFFS2 映像可从下面提到的、为预编RedBoot 映像提供的同一 FTP 站点获取
```
	load sample_img.jffs2 -r -b 0x100000
```
```
	RedBoot> load sample_img.jffs2 -r -b 0x100000
	Raw file loaded 0x00100000-0x00377424
```
```
	fis free
```
```
	RedBoot> fis free
	  0x500E0000 .. 0x503C0000
```
上述值可能根据文件系统的大小和闪存的类​​型而不同。下面作为示例展示它们的用法，并请务必适当地替换为你自己的值
```
	size of unallocated flash:	0x503c0000 - 0x500e0000 = 0x2e0000
	size of the filesystem image:	0x00377424 - 0x00100000 = 0x277424
```
我们当然要装入文件系统映像，但我们也想把它全部（剩余空间）给

```
	fis unlock -f 0x500E0000 -l 0x2e0000
	fis erase -f 0x500E0000 -l 0x2e0000
	fis write -b 0x100000 -l 0x277424 -f 0x500E0000
	fis create "JFFS2" -n -f 0x500E0000 -l 0x2e0000
```
现在该文件系统就Linux 在启动过程中发现MTD “分区”关联起来了。从 Redboot 中，'fis list' 命令

```
	RedBoot> fis list
	Name              FLASH addr  Mem addr    Length      Entry point
	RedBoot           0x50000000  0x50000000  0x00020000  0x00000000
	RedBoot config    0x503C0000  0x503C0000  0x00020000  0x00000000
	FIS directory     0x503E0000  0x503E0000  0x00020000  0x00000000
	Linux kernel      0x50020000  0x00100000  0x000C0000  0x00000000
	JFFS2             0x500E0000  0x500E0000  0x002E0000  0x00000000
```
```
	SA1100 flash: probing 32-bit flash bus
	SA1100 flash: Found 2 x16 devices at 0x0 in 32-bit mode
	Using RedBoot partition definition
	Creating 5 MTD partitions on "SA1100 flash":
	0x00000000-0x00020000 : "RedBoot"
	0x00020000-0x000e0000 : "Linux kernel"
	0x000e0000-0x003c0000 : "JFFS2"
	0x003c0000-0x003e0000 : "RedBoot config"
	0x003e0000-0x00400000 : "FIS directory"
```
这里重要的是我们感兴趣的分区位置，即第三个。在 Linux 中，这对应于 /dev/mtdblock2因此，要用内核及其在闪存中的根文件系统引Linux，我
```
	fis load "Linux kernel"
	exec -b 0x100000 -l 0xc0000 -c "root=/dev/mtdblock2"
```
当然也可以使JFFS 之外的其他文件系统，例如 cramfs你可能想通过 NFS 用根文件系统引导，等等。也可以（而且有时更方便）在从 ramdisk NFS 引导时，
直接Linux 内部将文件系统烧录（flash）到闪存。Linux MTD 仓库也有许多处理闪存内存的工具，例如擦除它。然JFFS2
可以直接挂载到一块新擦除的分区上，文件可以直接复制过去。等等…

### RedBoot 脚本


如果每次 Assabet 重启都要手动输入上述所有命令，就没那么有用了。因此可以使RedBoot 的脚本功将引导过程自动化
例如，我使用这个来引导同时带有内核和 ramdisk Linux
```
	RedBoot> fconfig
	Run script at boot: false true
	Boot script:
	Enter script, terminate with empty line
	>> load zImage -r -b 0x100000
	>> load ramdisk_ks.gz -r -b 0x800000
	>> exec -b 0x100000 -l 0xc0000
	>>
	Boot script timeout (1000ms resolution): 3
	Use BOOTP for network configuration: true
	GDB connection端口: 9000
	Network debug at boot time: false
	Update RedBoot non-volatile configuration - are you sure (y/n)? y
```
然后，重Assabet 只需等待登录提示出现即可


Nicolas Pitre
nico@fluxnic.net
2001 骞?6 鏈?12 鏃。

### -rmk 树中外设的状态（更新2001/10/14

Assabet Serial ports（串口）:
  Radio:		TX, RX, CTS, DSR, DCD, RI
   - PM:		未测试   - COM:		TX, RX, CTS, DSR, DCD, RTS, DTR, PM
   - PM:		未测试   - I2C:		已实现，未充分测试   - L3:		已充分测试，通过   - PM:		未测试
 Video（视频）:
  - LCD:		已充分测试。PM

   （连neponset LCD 不喜欢被消隐
  - Video out:		未完
 Audio（音频）:
  UDA1341:
  - Playback:		已充分测试，通过  - Record:		已实现，未测试  - PM:			未测试
  UCB1200:
  - Audio play:	已实现，未重度测试  - Audio rec:		已实现，未重度测试  - Telco audio play:	已实现，未重度测试  - Telco audio rec:	已实现，未重度测试  - POTS control:	  - Touchscreen:	  - PM:		未测试
 Other（其他）:
  - PCMCIA:
  - LPE:		已充分测试，通过  - USB:		  - IRDA:
  - SIR:		已充分测试，通过  - FIR:		已充分测试，通过  - PM:			未测试
Neponset Serial ports（串口）:
  - COM1,2:		TX, RX, CTS, DSR, DCD, RTS, DTR
  - PM:			未测试  - USB:		已实现，未重度测试  - PCMCIA:		已实现，未重度测试  - CF:			已实现，未重度测试  - PM:			未测试
更多内容可在 -np（Nicolas Pitre 的）树中找到