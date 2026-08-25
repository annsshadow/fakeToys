
## PCI 测试用户指南


:Author: Kishon Vijay Abraham I <kishon@ti.com>

本文档是一份指南，帮助用户使用 pci-epf-test 功能驱动pci_endpoint_test 主机驱动来测PCI。下面给出在主机侧与 EP 侧需要遵循的步骤列表
## 端点设备


### 端点控制器设

```

	# ls /sys/class/pci_epc/
	  51000000.pcie_ep

```
```

	# ls /sys/kernel/config/pci_ep/controllers
	  51000000.pcie_ep


```

### 端点功能驱动


```

	# ls /sys/bus/pci-epf/drivers
	  pci_epf_test

```
```

	# ls /sys/kernel/config/pci_ep/functions
	  pci_epf_test


```

### 创建 pci-epf-test 设备


可以使用 configfs 创建 PCI 端点功能设备。要创建设备，执行以下命令：

```

	# mount -t configfs none /sys/kernel/config
	# cd /sys/kernel/config/pci_ep/
	# mkdir functions/pci_epf_test/func1

```

上面"mkdir func1" 将创pci-epf-test 功能设备，该设备会被 pci_epf_test 驱动探测到
PCI 端点框架会在该目录下填充以下内容
```

	# ls functions/pci_epf_test/func1
	  baseclass_code	interrupt_pin	progif_code	subsys_id
	  cache_line_size	msi_interrupts	revid		subsys_vendorid
	  deviceid          	msi_interrupts	subclass_code	vendorid

```

当设备绑定到驱动时，PCI 端点功能驱动会用默认值填充这些条目。pci-epf-test 驱动会用类似以下的值填充这些条目：

```

	# cat functions/pci_epf_test/func1/vendorid
	  0xffff
	# cat functions/pci_epf_test/func1/interrupt_pin
	  0x0001


```

### 配置 pci-epf-test 设备


用户可以使用 configfs 条目配置 pci-epf-test 设备。要修改功能所使用vendorid MSI 中断数量，执行以下命令：

```

	# echo 0x104c > functions/pci_epf_test/func1/vendorid
	# echo 0xb500 > functions/pci_epf_test/func1/deviceid
	# echo 32 > functions/pci_epf_test/func1/msi_interrupts
	# echo 2048 > functions/pci_epf_test/func1/msix_interrupts

```
```

	# grep . functions/pci_epf_test/func1/pci_epf_test.0/bar?_size
	  functions/pci_epf_test/func1/pci_epf_test.0/bar0_size:131072
	  functions/pci_epf_test/func1/pci_epf_test.0/bar1_size:131072
	  functions/pci_epf_test/func1/pci_epf_test.0/bar2_size:131072
	  functions/pci_epf_test/func1/pci_epf_test.0/bar3_size:131072
	  functions/pci_epf_test/func1/pci_epf_test.0/bar4_size:131072
	  functions/pci_epf_test/func1/pci_epf_test.0/bar5_size:1048576

```
```

	# echo 1048576 > functions/pci_epf_test/func1/pci_epf_test.0/bar1_size

```

覆盖默认BAR 大小只能在将 pci-epf-test 设备绑定PCI 端点控制器驱动之前进行
注意：某些端点控制器可能具有固定大小或保留的 BAR；对于这类控制器，configfs 中对应的 BAR 大小将被忽略

### pci-epf-test 设备绑定EP 控制

为了让端点功能设备可用，必须将其绑定PCI 端点控制器驱动。使configfs 绑定该功能：

```

	# ln -s functions/pci_epf_test/func1 controllers/51000000.pcie_ep/

```

完成上述步骤后，PCI 端点即可准备与主机建立链路

### 启动链路


端点设备要与主机建立链路，需start 属性写1
```

	# echo 1 > controllers/51000000.pcie_ep/start


```

## RootComplex 设备


### lspci 输出


请注意，此处列出的设备对应于前文配置中填充的值：

```

	00:00.0 PCI bridge: Texas Instruments Device 8888 (rev 01)
	01:00.0 Unassigned class [ff00]: Texas Instruments Device b500


```

### 使用端点测试功能设备


tools/testing/selftests/pci_endpoint 中加入的 Kselftest 可用于运行所有默认的 PCI 端点测试。要构建 PCI 端点Kselftest，执行：

```

	# cd <kernel-dir>
	# make -C tools/testing/selftests/pci_endpoint

```
```

	# cd <kernel-dir>
	# make -C tools/testing/selftests/pci_endpoint INSTALL_PATH=/usr/bin install

```

测试程序将位<rootfs>/usr/bin/ 目录下
#### Kselftest 输出

```

	# pci_endpoint_test
	TAP version 13
	1..16
	# Starting 16 tests from 9 test cases.
	#  RUN           pci_ep_bar.BAR0.BAR_TEST ...
	#            OK  pci_ep_bar.BAR0.BAR_TEST
	ok 1 pci_ep_bar.BAR0.BAR_TEST
	#  RUN           pci_ep_bar.BAR1.BAR_TEST ...
	#            OK  pci_ep_bar.BAR1.BAR_TEST
	ok 2 pci_ep_bar.BAR1.BAR_TEST
	#  RUN           pci_ep_bar.BAR2.BAR_TEST ...
	#            OK  pci_ep_bar.BAR2.BAR_TEST
	ok 3 pci_ep_bar.BAR2.BAR_TEST
	#  RUN           pci_ep_bar.BAR3.BAR_TEST ...
	#            OK  pci_ep_bar.BAR3.BAR_TEST
	ok 4 pci_ep_bar.BAR3.BAR_TEST
	#  RUN           pci_ep_bar.BAR4.BAR_TEST ...
	#            OK  pci_ep_bar.BAR4.BAR_TEST
	ok 5 pci_ep_bar.BAR4.BAR_TEST
	#  RUN           pci_ep_bar.BAR5.BAR_TEST ...
	#            OK  pci_ep_bar.BAR5.BAR_TEST
	ok 6 pci_ep_bar.BAR5.BAR_TEST
	#  RUN           pci_ep_basic.CONSECUTIVE_BAR_TEST ...
	#            OK  pci_ep_basic.CONSECUTIVE_BAR_TEST
	ok 7 pci_ep_basic.CONSECUTIVE_BAR_TEST
	#  RUN           pci_ep_basic.LEGACY_IRQ_TEST ...
	#            OK  pci_ep_basic.LEGACY_IRQ_TEST
	ok 8 pci_ep_basic.LEGACY_IRQ_TEST
	#  RUN           pci_ep_basic.MSI_TEST ...
	#            OK  pci_ep_basic.MSI_TEST
	ok 9 pci_ep_basic.MSI_TEST
	#  RUN           pci_ep_basic.MSIX_TEST ...
	#            OK  pci_ep_basic.MSIX_TEST
	ok 10 pci_ep_basic.MSIX_TEST
	#  RUN           pci_ep_data_transfer.memcpy.READ_TEST ...
	#            OK  pci_ep_data_transfer.memcpy.READ_TEST
	ok 11 pci_ep_data_transfer.memcpy.READ_TEST
	#  RUN           pci_ep_data_transfer.memcpy.WRITE_TEST ...
	#            OK  pci_ep_data_transfer.memcpy.WRITE_TEST
	ok 12 pci_ep_data_transfer.memcpy.WRITE_TEST
	#  RUN           pci_ep_data_transfer.memcpy.COPY_TEST ...
	#            OK  pci_ep_data_transfer.memcpy.COPY_TEST
	ok 13 pci_ep_data_transfer.memcpy.COPY_TEST
	#  RUN           pci_ep_data_transfer.dma.READ_TEST ...
	#            OK  pci_ep_data_transfer.dma.READ_TEST
	ok 14 pci_ep_data_transfer.dma.READ_TEST
	#  RUN           pci_ep_data_transfer.dma.WRITE_TEST ...
	#            OK  pci_ep_data_transfer.dma.WRITE_TEST
	ok 15 pci_ep_data_transfer.dma.WRITE_TEST
	#  RUN           pci_ep_data_transfer.dma.COPY_TEST ...
	#            OK  pci_ep_data_transfer.dma.COPY_TEST
	ok 16 pci_ep_data_transfer.dma.COPY_TEST
	# PASSED: 16 / 16 tests passed.
	# Totals: pass:16 fail:0 xfail:0 xpass:0 skip:0 error:0


```

对于大多数支DMA 的端点控制器，测试用16（pci_ep_data_transfer.dma.COPY_TEST）会因缺少基DMA MEMCPY 而失败。对于这类控制器，建议使用以下命令跳过该测试用例
```

	# pci_endpoint_test -f pci_ep_bar -f pci_ep_basic -v memcpy -T COPY_TEST -v dma

```

#### Kselftest EP Doorbell


如果端点 MSI 控制器用doorbell（门铃）用例，请运行以下命令进行测试
	# pci_endpoint_test -f pcie_ep_doorbell

	# Starting 1 tests from 1 test cases.
	#  RUN           pcie_ep_doorbell.DOORBELL_TEST ...
	#            OK  pcie_ep_doorbell.DOORBELL_TEST
	ok 1 pcie_ep_doorbell.DOORBELL_TEST
	# PASSED: 1 / 1 tests passed.
	# Totals: pass:1 fail:0 xfail:0 xpass:0 skip:0 error:0
