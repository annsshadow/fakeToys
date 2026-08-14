## 使用 spi-intel 升级 BIOS


许多 Intel CPU（如 Baytrail 和 Braswell）包含 SPI 串行闪存主控制器，用于保存 BIOS 和其他平台特定数据。由于 SPI 串行闪存的内容对于机器运行至关重要，它通常受到不同硬件保护机制的保护，以避免意外（或蓄意）覆盖内容。

并非所有制造商都保护 SPI 串行闪存，主要是因为这允许直接从操作系统升级 BIOS 镜像。

spi-intel 驱动使得在特定的保护位未被设置并锁定的情况下，可以读写 SPI 串行闪存。如果它发现其中任何一位被设置，整个 MTD 设备将被设为只读，以防止部分覆盖。默认情况下，驱动将 SPI 串行闪存内容作为只读暴露，但可以通过内核命令行传递 “spi_intel.writeable=1” 来更改。

请记住，覆盖 SPI 串行闪存上的 BIOS 镜像可能会使机器无法启动，并需要像 Dediprog 这样的特殊设备来恢复。已经警告过你了！

以下是从 Linux 直接升级 MinnowBoard MAX BIOS 的步骤。

 1) 下载并解压最新的 Minnowboard MAX BIOS SPI 镜像
    [^1^]。撰写本文时最新镜像是 v92。

 2) 安装 mtd-utils 软件包 [^2^]。我们需要它来擦除 SPI
    串行闪存。像 Debian 和 Fedora 这样的发行版已将其打包，名为 “mtd-utils”。

 3) 将 “spi_intel.writeable=1” 添加到内核命令行并重启
    开发板（你也可以重新加载驱动，将 “writeable=1” 作为模块参数传递给 modprobe）。

 4) 开发板重新启动运行后，找到正确的 MTD 分区
```

	# cat /proc/mtd
	dev:    size   erasesize  name
	mtd0: 00800000 00001000 "BIOS"

    因此这里将是 /dev/mtd0，但可能有所不同。

 5) 首先备份现有镜像：：

	# dd if=/dev/mtd0ro of=bios.bak
	16384+0 records in
	16384+0 records out
	8388608 bytes (8.4 MB) copied, 10.0269 s, 837 kB/s

 6) 验证备份：：

	# sha1sum /dev/mtd0ro bios.bak
	fdbb011920572ca6c991377c4b418a0502668b73  /dev/mtd0ro
	fdbb011920572ca6c991377c4b418a0502668b73  bios.bak

    SHA1 校验和必须匹配。否则不要继续！

 7) 擦除 SPI 串行闪存。此步骤之后，不要重启
    开发板！否则它将无法再启动：：

	# flash_erase /dev/mtd0 0 0
	Erasing 4 Kibyte @ 7ff000 -- 100 % complete

 8) 无错误地完成后，你可以写入新的 BIOS 镜像：：

    # dd if=MNW2MAX1.X64.0092.R01.1605221712.bin of=/dev/mtd0

 9) 验证 SPI 串行闪存的新内容是否与新的 BIOS 镜像匹配：：

	# sha1sum /dev/mtd0ro MNW2MAX1.X64.0092.R01.1605221712.bin
	9b4df9e4be2057fceec3a5529ec3d950836c87a2  /dev/mtd0ro
	9b4df9e4be2057fceec3a5529ec3d950836c87a2 MNW2MAX1.X64.0092.R01.1605221712.bin

    SHA1 校验和应当匹配。

 10) 现在你可以重启开发板，观察新的 BIOS 正常启动。

```
### 参考文献


[^1^] https://firmware.intel.com/sites/default/files/MinnowBoard%2EMAX_%2EX64%2E92%2ER01%2Ezip

[^2^] http://www.linux-mtd.infradead.org/
