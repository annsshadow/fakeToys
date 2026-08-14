
## WorkBiT NinjaSCSI-3/32Bi Linux 驱动


## 1. 说明


这是 Workbit corp.（http://www.workbit.co.jp/）的 NinjaSCSI-3 的 Linux 驱动。

## 2. 我的 Linux 环境


:Linux 内核: 2.4.7 / 2.2.19
:pcmcia-cs:    3.1.27
:gcc:          gcc-2.95.4
:PC 卡:        I-O data PCSC-F (NinjaSCSI-3),
               I-O data CBSC-II 16 位模式 (NinjaSCSI-32Bi)
:SCSI 设备:    I-O data CDPS-PX24 (CD-ROM 驱动器),
               Media Intelligent MMO-640GT (光盘驱动器)

## 3. 安装


(a) 确认你的 PC 卡是真正的 “NinjaSCSI-3” 卡。

    如果你已经安装了 pcmcia-cs，pcmcia 会将你的卡报告为 UNKNOWN 卡，并向你的控制台或
    日志文件写入 ["WBT", "NinjaSCSI-3", "R1.0"] 或其他字符串。

    你也可以使用 “cardctl” 程序（该程序位于 pcmcia-cs 源码中）来获取更多信息。

```

	# cat /var/log/messages
	...
	Jan  2 03:45:06 lindberg cardmgr[78]: unsupported card in socket 1
	Jan  2 03:45:06 lindberg cardmgr[78]:   product info: "WBT", "NinjaSCSI-3", "R1.0"
	...
	# cardctl ident
	Socket 0:
	  no product info available
	Socket 1:
	  product info: "IO DATA", "CBSC16       ", "1"


```
(b) 获取 Linux 内核源码，并将其解压到 /usr/src。由于 NinjaSCSI 驱动需要 Linux 内核
    源码中的一些 SCSI 头文件，我建议重新构建你的内核；这可以消除一些版本问题。

```

	$ cd /usr/src
	$ tar -zxvf linux-x.x.x.tar.gz
	$ cd linux
	$ make config
	...

```
(c) 如果你将该驱动与内核 2.2 配合使用，在某个目录中解压 pcmcia-cs 并 make & install。
    该驱动需要 pcmcia-cs 头文件。

```

	$ cd /usr/src
	$ tar zxvf cs-pcmcia-cs-3.x.x.tar.gz
	...

```
```

	$ tar -zxvf nsp_cs-x.x.tar.gz
	$ cd nsp_cs-x.x
	$ emacs Makefile
	...
	$ make

```
(e) 将 nsp_cs.ko 复制到合适的位置，例如 /lib/modules/<内核版本>/pcmcia/ 。

(f) 将这些行加入 /etc/pcmcia/config 。

    如果你使用 pcmcia-cs-3.1.8 或更高版本，我们可以使用 “nsp_cs.conf” 文件。
    因此，你无需编辑文件，只需复制到 /etc/pcmcia/ 即可。

```

	device "nsp_cs"
	  class "scsi" module "nsp_cs"

	card "WorkBit NinjaSCSI-3"
	  version "WBT", "NinjaSCSI-3", "R1.0"
	  bind "nsp_cs"

	card "WorkBit NinjaSCSI-32Bi (16bit)"
	  version "WORKBIT", "UltraNinja-16", "1"
	  bind "nsp_cs"

	# OEM
	card "WorkBit NinjaSCSI-32Bi (16bit) / IO-DATA"
	  version "IO DATA", "CBSC16       ", "1"
	  bind "nsp_cs"

	# OEM
	card "WorkBit NinjaSCSI-32Bi (16bit) / KME-1"
	  version "KME    ", "SCSI-CARD-001", "1"
	  bind "nsp_cs"
	card "WorkBit NinjaSCSI-32Bi (16bit) / KME-2"
	  version "KME    ", "SCSI-CARD-002", "1"
	  bind "nsp_cs"
	card "WorkBit NinjaSCSI-32Bi (16bit) / KME-3"
	  version "KME    ", "SCSI-CARD-003", "1"
	  bind "nsp_cs"
	card "WorkBit NinjaSCSI-32Bi (16bit) / KME-4"
	  version "KME    ", "SCSI-CARD-004", "1"
	  bind "nsp_cs"

```
```

	# /etc/rc.d/rc.pcmcia start        (BSD 风格)

    或::

	# /etc/init.d/pcmcia start         (SYSV 风格)


```
## 4. 历史


参见 README.nin_cs 。

## 5. 注意事项


如果在对 SCSI 设备执行某些操作，或挂起计算机时弹出卡片，你会遇到一些**严重**错误，
例如磁盘崩溃。

当我正确使用该驱动时它工作良好。但我不保证你的数据。使用该驱动时请备份你的数据。

## 6. 已知缺陷


在 2.4 内核中，你无法使用 640MB 光盘。该错误来自高层 SCSI 驱动。

## 7. 测试


请向我们发送该软件的一些报告（缺陷报告等）。发送报告时，请告知我们以下或更多信息。

 - 卡名称
 - 内核版本
 - 你的 SCSI 设备名称（硬盘、CD-ROM 等……）

## 8. 版权


 参见 GPL。


2001/08/08 yokota@netlab.is.tsukuba.ac.jp <YOKOTA Hiroshi>
