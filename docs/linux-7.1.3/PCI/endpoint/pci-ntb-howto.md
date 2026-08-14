
## PCI 非透明桥（NTB）端点功能（EPF）用户指南


:Author: Kishon Vijay Abraham I <kishon@ti.com>

本文档是一份指南，帮助用户使用 pci-epf-ntb 功能驱动与 ntb_hw_epf 主机驱动来实现 NTB 功能。下面给出了
主机侧与 EP 侧需要遵循的步骤列表。关于使用可配置端点实现 NTB 的硬件配置与内部机制，请参见
`Documentation/PCI/endpoint/pci-ntb-function.rst`。

## 端点设备


### 端点控制器设备


要实现 NTB 功能，至少需要两个端点控制器设备。

```

	# ls /sys/class/pci_epc/
	2900000.pcie-ep  2910000.pcie-ep

```
```

	# ls /sys/kernel/config/pci_ep/controllers
	2900000.pcie-ep  2910000.pcie-ep


```
### 端点功能驱动


```

	# ls /sys/bus/pci-epf/drivers
	pci_epf_ntb   pci_epf_ntb

```
```

	# ls /sys/kernel/config/pci_ep/functions
	pci_epf_ntb   pci_epf_ntb


```
### 创建 pci-epf-ntb 设备


可以使用 configfs 创建 PCI 端点功能设备。要创建
```

	# mount -t configfs none /sys/kernel/config
	# cd /sys/kernel/config/pci_ep/
	# mkdir functions/pci_epf_ntb/func1

```
上面的 "mkdir func1" 创建了将被 pci_epf_ntb 驱动探测的 pci-epf-ntb 功能设备。

PCI 端点框架会用以下内容填充该目录
```

	# ls functions/pci_epf_ntb/func1
	baseclass_code    deviceid          msi_interrupts    pci-epf-ntb.0
	progif_code       secondary         subsys_id         vendorid
	cache_line_size   interrupt_pin     msix_interrupts   primary
	revid             subclass_code     subsys_vendor_id

```
PCI 端点功能驱动会在设备绑定到驱动时，用默认值填充这些条目。pci-epf-ntb 驱动会填充
```

	# cat functions/pci_epf_ntb/func1/vendorid
	0xffff
	# cat functions/pci_epf_ntb/func1/interrupt_pin
	0x0001


```
### 配置 pci-epf-ntb 设备


用户可以使用其 configfs 条目配置 pci-epf-ntb 设备。为了更改 vendorid 与 deviceid，请执行以下
```

	# echo 0x104c > functions/pci_epf_ntb/func1/vendorid
	# echo 0xb00d > functions/pci_epf_ntb/func1/deviceid

```
PCI 端点框架还会自动在功能属性目录中创建一个子目录。该子目录与功能设备的名称相同，并用以下
NTB 特定的内容填充
```

	# ls functions/pci_epf_ntb/func1/pci_epf_ntb.0/
	db_count    mw1         mw2         mw3         mw4         num_mws
	spad_count

```
```

	# echo 4 > functions/pci_epf_ntb/func1/pci_epf_ntb.0/db_count
	# echo 128 > functions/pci_epf_ntb/func1/pci_epf_ntb.0/spad_count
	# echo 2 > functions/pci_epf_ntb/func1/pci_epf_ntb.0/num_mws
	# echo 0x100000 > functions/pci_epf_ntb/func1/pci_epf_ntb.0/mw1
	# echo 0x100000 > functions/pci_epf_ntb/func1/pci_epf_ntb.0/mw2

```
### 将 pci-epf-ntb 设备绑定到 EP 控制器


NTB 功能设备应连接到连接到两台主机的两个 PCI 端点控制器。使用 NTB 功能设备内部的 'primary' 和
'secondary' 条目，将一个 PCI 端点控制器连接到 primary 接口，将另一个 PCI 端点控制器连接到 secondary
```

	# ln -s controllers/2900000.pcie-ep/ functions/pci-epf-ntb/func1/primary
	# ln -s controllers/2910000.pcie-ep/ functions/pci-epf-ntb/func1/secondary

```
完成上述步骤后，两个 PCI 端点控制器都准备好与主机建立链路。


### 启动链路


为了让端点设备与主机建立链路，_start_ 字段应被填充为 '1'。对于 NTB，两个 PCI 端点控制器都
```

	# echo 1 > controllers/2900000.pcie-ep/start
	# echo 1 > controllers/2910000.pcie-ep/start


```
## RootComplex 设备


### lspci 输出


注意，此处列出的设备对应于填充在以下位置的数值
```

	# lspci
	0000:00:00.0 PCI bridge: Texas Instruments Device b00d
	0000:01:00.0 RAM memory: Texas Instruments Device b00d


```
### 使用 ntb_hw_epf 设备


主机侧软件遵循 Linux 中标准的 NTB 软件架构。所有现有的客户端侧 NTB 实用工具，如 NTB Transport Client、
NTB Netdev、NTB Ping Pong Test Client 和 NTB Tool Test Client，都可以与 NTB 功能设备一起使用。

关于 NTB 的更多信息，请参见
[Non-Transparent Bridge <../../driver-api/ntb>](Non-Transparent Bridge <../../driver-api/ntb>)
