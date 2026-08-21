## PCI 端点测试功能驱动


如果根复合体连接到运`pci_epf_test` 功能驱动的可配置 PCI 端点（按[^1^]_
配置），则该驱动应作为主机端驱动使用
“pci_endpoint_test”驱动可用于执行以下测试
测试设备PCI 驱动执行以下测试
	#) 验证 BAR 中编程的地址
	#) 触发传统 IRQ
	#) 触发 MSI IRQ
	#) 触发 MSI-X IRQ
	#) 读取数据
	#) 写入数据
	#) 复制数据

misc 驱动为每个连接到根复合体`pci_epf_test` 功能创建
/dev/pci-endpoint-test.<num>，并应使用“ioctls”来执行上述测试
### ioctl


 PCITEST_BAR:
	      测试 BAR。应传入要测试的 BAR 编号作为参数 PCITEST_LEGACY_IRQ:
	      测试传统 IRQ
 PCITEST_MSI:
	      测试消息信号中断。应传入要测试的 MSI 编号作为参数 PCITEST_MSIX:
	      测试消息信号中断。应传入要测试的 MSI-X 编号作为参数 PCITEST_SET_IRQTYPE:
	      更改驱动 IRQ 类型配置。应传入 IRQ 类型作为参数
	      ：Legacy：MSI：MSI-X） PCITEST_GET_IRQTYPE:
	      获取驱动 IRQ 类型配置 PCITEST_WRITE:
	      执行写测试。应传入缓冲区大小作为参数 PCITEST_READ:
	      执行读测试。应传入缓冲区大小作为参数 PCITEST_COPY:
	      执行读测试。应传入缓冲区大小作为参数