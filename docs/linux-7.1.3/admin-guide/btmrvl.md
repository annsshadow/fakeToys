## btmrvl 驱动


所有命令都通过 debugfs 接口使用
## 设置/获取驱动配置


路径debug/btmrvl/config/

gpiogap=[n], hscfgcmd
```
	bit 8:0  -- Gap
	bit 16:8 -- GPIO

	其中 GPIO 是用于唤醒主机的 GPIO 引脚编号	可以是任意有GPIO 引脚号（例如 0-7）或 0xff（此时改SDIO 接口
	来唤醒）
	其中 Gap 是唤醒信号与唤醒事件之间的间隔（单位为毫秒），或0xff
	表示特殊的宿主睡眠（host sleep）设置
	用法::

		# 使用 SDIO 接口唤醒主机并将 GAP 设为 0x80		echo 0xff80 > /debug/btmrvl/config/gpiogap
		echo 1 > /debug/btmrvl/config/hscfgcmd

		# 使用 GPIO 引脚 #3 唤醒主机并将 GAP 设为 0xff		echo 0x03ff >  /debug/btmrvl/config/gpiogap
		echo 1 > /debug/btmrvl/config/hscfgcmd

```
psmode=[n], pscmd
	这些命令用于启用/禁用自动睡眠模式

```

			1 	-- 启用自动睡眠模式
			0 	-- 禁用自动睡眠模式

	用法::

		# 启用自动睡眠模式
		echo 1 > /debug/btmrvl/config/psmode
		echo 1 > /debug/btmrvl/config/pscmd

		# 禁用自动睡眠模式
		echo 0 > /debug/btmrvl/config/psmode
		echo 1 > /debug/btmrvl/config/pscmd


```
hsmode=[n], hscmd
	这些命令用于启用宿主睡眠或唤醒固
```

			1	-- 启用宿主睡眠
			0	-- 唤醒固件

	用法::

		# 启用宿主睡眠
		echo 1 > /debug/btmrvl/config/hsmode
		echo 1 > /debug/btmrvl/config/hscmd

		# 唤醒固件
		echo 0 > /debug/btmrvl/config/hsmode
		echo 1 > /debug/btmrvl/config/hscmd


```
## 获取驱动状

路径debug/btmrvl/status/

```

	cat /debug/btmrvl/status/<args>

```
其中 args 为：

curpsmode
	该命令显示当前的自动睡眠状态
psstate
	该命令显示电源节省状态
hsstate
	该命令显示宿主睡眠状态
txdnldrdy
	该命令显Tx 下载就绪标志的值
## 发出原始 HCI 命令


使用 hcitool 发出原始 HCI 命令，请参阅 hcitool 手册

```

	Hcitool cmd <ogf> <ocf> [Parameters]

```
```

	hcitool cmd 0x3f 0x5b 0xf5 0x01 0x00    --启用全部接口
	hcitool cmd 0x3f 0x5b 0xf5 0x01 0x01    --启用 Wlan 接口
	hcitool cmd 0x3f 0x5b 0xf5 0x01 0x02    --启用 BT 接口
	hcitool cmd 0x3f 0x5b 0xf5 0x00 0x00    --禁用全部接口
	hcitool cmd 0x3f 0x5b 0xf5 0x00 0x01    --禁用 Wlan 接口
	hcitool cmd 0x3f 0x5b 0xf5 0x00 0x02    --禁用 BT 接口

```
## SD8688 固件


镜像文件
- /lib/firmware/sd8688_helper.bin
- /lib/firmware/sd8688.bin


这些镜像可以从以下地址下载
git.infradead.org/users/dwmw2/linux-firmware.git/libertas/
