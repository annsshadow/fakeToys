
## Spear PCIe Gadget 驱动


## 作者（Author）

Pratyush Anand (pratyush.anand@gmail.com)

## 位置（Location）

driver/misc/spear13xx_pcie_gadget.c

## 支持的芯片（Supported Chip）：

SPEAr1300
SPEAr1310

## Menuconfig 选项（Menuconfig option）：

Device Drivers
	Misc devices
		PCIe gadget support for SPEAr13XX platform

## 用途（purpose）

该驱动有若干个可由 configfs 接口读写的节点（node）。其主要目的是将所选的双模（dual mode）PCIe 控制器配置为设备（device），然后编程其各种寄存器，将其配置为特定的设备类型。该驱动可用于展示 spear 的 PCIe 设备能力。

## 不同节点的描述（Description of different nodes）：


### 节点的读行为（read behavior of nodes）：


=============== ==============================================================
link 		给出 ltssm 状态。
int_type 	所支持的中断类型
no_of_msi 	若主机未启用 MSI 则为 0。正值即为被授予的 MSI 向量数量。
vendor_id	返回已编程的厂商 ID（hex，十六进制）
device_id	返回已编程的设备 ID（hex，十六进制）
bar0_size:	以十六进制返回 bar0 的大小。
bar0_address	以十六进制返回 bar0 映射区的地址。
bar0_rw_offset	返回 bar0 的偏移量，bar0_data 将返回该偏移处的值。
bar0_data	返回 bar0_rw_offset 处的数据。
=============== ==============================================================

### 节点的写行为（write behavior of nodes）：


=============== ================================================================
link 		写入 UP 以启用 ltsmm，写入 DOWN 以禁用
int_type	写入要配置的中断类型（int_type 可以是 INTA、MSI 或 NO_INT）。仅在你已编程了 no_of_msi 节点时才选择 MSI。
no_of_msi	所需的 MSI 向量数量。
inta		写入 1 以断言（assert）INTA，写入 0 以解除断言。
send_msi	写入要发送的 MSI 向量。
vendor_id	写入要编程的厂商 ID（hex，十六进制）。
device_id	写入要编程的设备 ID（hex，十六进制）。
bar0_size	以十六进制写入 bar0 的大小。默认 bar0 大小为 1000（hex）字节。
bar0_address	以十六进制写入 bar0 映射区的地址。（bar0 的默认映射为 SYSRAM1(E0800000)。务必先编程 bar 大小再编程 bar 地址。内核可能为了对齐而修改 bar 大小和地址，因此写入后应回读 bar 大小和地址以进行核对。
bar0_rw_offset	写入 bar0 的偏移量，bar0_data 将向该偏移写入值。
bar0_data	写入要写到 bar0_rw_offset 的数据。
=============== ================================================================

## 节点编程示例（Node programming example）


将所有的 PCIe 寄存器编程为：当此设备连接到 PCIe 主机时，主机将此设备视为 1MB 的 RAM。

```

    #mount -t configfs none /Config

```
```

    # cd /config/pcie_gadget.n/

```
现在你在该目录下拥有所有节点。
```

    # echo 104A >> vendor_id

```
```

    # echo CD80 >> device_id

```
```

    # echo 100000 >> bar0_size

```
```

    # cat bar0_size

```
将 BAR0 地址编程为 DDR（0x2100000）。这是要暴露给 PCIe 主机的物理内存地址。类似地，任何其它外设也可以暴露给 PCIe 主机。例如，如果你将 UART 的基地址编程为 BAR0 地址，那么当此设备连接到主机时，它将表现为一个 UART。

```

    # echo 2100000 >> bar0_address

```
```

    # echo INTA >> int_type

```
```

    # echo UP >> link

```
必须确保：一旦 gadget 侧完成链路就绪（link up），主机才开始初始化并搜索其端口上的 PCIe 设备。

```

    /*wait till link is up*/
    # cat link

```
等待其返回 UP。
```

    # echo 1 >> inta

```
```

    # echo 0 >> inta

```
```

    # echo 4 >> no_of_msi

```
```

    # echo MSI >> int_type

```
```

    # echo UP >> link

```
```

    # cat link

```
应用程序可以重复读取该节点，直到发现链路为 UP。两次读取之间可以休眠。

```

    # cat no_of_msi

```
应返回 4（请求的 MSI 向量数量）
```

    # echo 2 >> send_msi
    # cd -

```
