
## PCI 非透明桥（NTB）端点功能（EPF）用户指南


:作者: Frank Li <Frank.Li@nxp.com>

本文档是一份指南，帮助用户使用 pci-epf-vntb 功能驱动和 ntb_hw_epf 主机驱动
来实现 NTB 功能。下面给出了在主机侧和 EP 侧需要遵循的步骤列表。有关使用
可配置端点的 NTB 的硬件配置与内部机制，请参见
Documentation/PCI/endpoint/pci-vntb-function.rst。

## 端点设备


### 端点控制器设备


```

        # ls /sys/class/pci_epc/
          5f010000.pcie_ep

```
```

        # ls /sys/kernel/config/pci_ep/controllers
          5f010000.pcie_ep

```
### 端点功能驱动


```

	# ls /sys/bus/pci-epf/drivers
	pci_epf_ntb  pci_epf_test  pci_epf_vntb

```
```

	# ls /sys/kernel/config/pci_ep/functions
	pci_epf_ntb  pci_epf_test  pci_epf_vntb


```
### 创建 pci-epf-vntb 设备


PCI 端点功能设备可以使用 configfs 创建。要创建
```

	# mount -t configfs none /sys/kernel/config
	# cd /sys/kernel/config/pci_ep/
	# mkdir functions/pci_epf_vntb/func1

```
上面的 "mkdir func1" 创建了将由 pci_epf_vntb 驱动探测的 pci-epf-vntb 功能设备。

PCI 端点框架会使用以下内容填充该目录
```

	# ls functions/pci_epf_vntb/func1
	baseclass_code    deviceid          msi_interrupts    pci-epf-vntb.0
	progif_code       secondary         subsys_id         vendorid
	cache_line_size   interrupt_pin     msix_interrupts   primary
	revid             subclass_code     subsys_vendor_id

```
PCI 端点功能驱动在设备绑定到驱动时，会用默认值填充这些条目。pci-epf-vntb
驱动会填充
```

	# cat functions/pci_epf_vntb/func1/vendorid
	0xffff
	# cat functions/pci_epf_vntb/func1/interrupt_pin
	0x0001


```
### 配置 pci-epf-vntb 设备


用户可以使用其 configfs 条目配置 pci-epf-vntb 设备。为了更改 vendorid 和
deviceid，请执行以下
```

	# echo 0x1957 > functions/pci_epf_vntb/func1/vendorid
	# echo 0x0809 > functions/pci_epf_vntb/func1/deviceid

```
PCI 端点框架还会在功能属性目录中自动创建一个子目录。该子目录与功能设备的
名称相同，并填充有以下 NTB 特定的
```

	# ls functions/pci_epf_vntb/func1/pci_epf_vntb.0/
	ctrl_bar  db_count  mw1_bar  mw2_bar  mw3_bar  mw4_bar	spad_count
	db_bar	  mw1	    mw2      mw3      mw4      num_mws	vbus_number
	vntb_vid  vntb_pid

```
```

	# echo 4 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/db_count
	# echo 128 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/spad_count
	# echo 1 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/num_mws
	# echo 0x100000 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/mw1

```
默认情况下，每个构造（construct）会按需并按顺序分配一个 BAR。如果平台需要
特定的 BAR 设置，可以使用相关的 `XYZ_bar` 条目将 BAR 分配给每个构造。

```

	# echo 0x1957 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/vntb_vid
	# echo 0x080A > functions/pci_epf_vntb/func1/pci_epf_vntb.0/vntb_pid
	# echo 0x10 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/vbus_number

```
### 将 pci-epf-vntb 设备绑定到 EP 控制器


NTB 功能设备应附着到连接到主机的 PCI 端点控制器。

	# ln -s controllers/5f010000.pcie_ep functions/pci_epf_vntb/func1/primary

完成上述步骤后，PCI 端点控制器已准备好与主机建立链路。

### 启动链路


为了让端点设备与主机建立链路，应将 _start_ 字段填充为 '1'。对于 NTB，两个
PCI 端点控制器都需要
```

	# echo 1 > controllers/5f010000.pcie_ep/start

```
## 根复合体（RootComplex）设备


### 主机侧的 lspci 输出


注意，此处列出的设备对应于在以下位置填充的值
```

	# lspci
        00:00.0 PCI bridge: Freescale Semiconductor Inc Device 0000 (rev 01)
        01:00.0 RAM memory: Freescale Semiconductor Inc Device 0809

```
## 端点设备 / 虚拟 PCI 总线


### EP 侧 / 虚拟 PCI 总线的 lspci 输出


注意，此处列出的设备对应于在以下位置填充的值
```

        # lspci
        10:00.0 Unassigned class [ffff]: Dawicontrol Computersysteme GmbH Device 1234 (rev ff)

```
### 使用 ntb_hw_epf 设备


主机侧软件遵循 Linux 中标准的 NTB 软件架构。所有现有的客户端 NTB 实用工具，
如 NTB Transport Client、NTB Netdev、NTB Ping Pong Test Client 和 NTB Tool
Test Client，都可以与 NTB 功能设备一起使用。

有关 NTB 的更多信息，请参见
[Non-Transparent Bridge <../../driver-api/ntb>](Non-Transparent Bridge <../../driver-api/ntb>)
