
## 用于 Linux 的 Davicom DM9102(A)/DM9132/DM9801 快速以太网驱动


注意：该驱动没有维护者。


本程序是自由软件；你可以在自由软件基金会发布的 GNU 通用公共许可证（GNU General Public License）条款下重新分发和/或修改它；许可证版本为第 2 版，或者（由你选择）任何更高版本。

本程序的分发希望它是有用的，但没有任何担保；甚至没有对适销性或特定用途适用性的隐含担保。更多细节请参见 GNU 通用公共许可证。

该驱动为 Davicom DM9102(A)/DM9132/DM9801 以太网卡提供内核支持（CNET 10/100 以太网卡也使用 Davicom 芯片组，因此该驱动也支持 CNET 卡）。如果你没有将该驱动编译为模块，它将在启动时自动加载自身并打印一条
```

	dmfe: Davicom DM9xxx net driver, version 1.36.4 (2002-01-17)

```

```
	insmod dmfe

```
这样它会自动检测设备模式。这是建议的加载模块方式。或者你可以传入
```

	insmod dmfe mode=0 # 强制 10M 半双工
	insmod dmfe mode=1 # 强制 100M 半双工
	insmod dmfe mode=4 # 强制 10M 全双工
	insmod dmfe mode=5 # 强制 100M 全双工

```

```
	ifconfig eth0 172.22.3.18
		      ^^^^^^^^^^^
		     你的 IP 地址

```

```
	route add default eth0



```
现在你的以太网卡应该已经启动并运行。


TODO：

- 实现 pci_driver::suspend() 和 pci_driver::resume() 电源管理方法。
- 在 64 位机器上检查。
- 在 big endian 机器上检查并修复。
- 测试并确保所有情况下 PCI 延迟（latency）现在都正确。


作者：

Sten Wang <sten_wang@davicom.com.tw >   : 原始作者

贡献者：

- Marcelo Tosatti <marcelo@conectiva.com.br>
- Alan Cox <alan@lxorguk.ukuu.org.uk>
- Jeff Garzik <jgarzik@pobox.com>
- Vojtech Pavlik <vojtech@suse.cz>
