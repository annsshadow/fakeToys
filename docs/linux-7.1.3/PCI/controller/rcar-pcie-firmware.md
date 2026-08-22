
## Renesas R-Car V4H PCIe 控制器固

Renesas R-Car V4H (r8a779g0) 有一PCIe 控制器，需要特定的
启动期间下载固件
不过，瑞萨电子目前无法免费分发该固件
固件文件 04_PCIe_fw_addr_data_ver1.05.txt”（请注意，文件名在不同
版本的数据手册之间可能有所不同）可以在数据手册中以文本编码的形式找到，
因此必须将该文件的内容转换回二进制形式。可以使用以下示例脚本完此操作：


	$ awk '/^\s*0x[0-9A-Fa-f]{4}\s+0x[0-9A-Fa-f]{4}/ { print substr($2,5,2) substr($2,3,2) }' \
		104_PCIe_fw_addr_data_ver1.05.txt | \
			xxd -p -r > rcar_gen4_pcie.bin

将文本内容转换为二进制固件文件后，按如下方式验证其校验和

	$ sha1sum rcar_gen4_pcie.bin
	1d0bd4b189b4eb009f5d564b1f93a79112994945  rcar_gen4_pcie.bin

生成的名“rcar_gen4_pcie.bin的二进制文件应在驱动程序运行之前
放置lib/firmware目录中