## 机密计算密钥


本文档描述机密计算（Confidential Computing）的密钥注入是如何从固件传递到操作系统，经由 EFI 驱动和 efi_secret 内核模块处理的。

## 简介


机密计算（coco）硬件，例如 AMD SEV（安全加密虚拟化，Secure Encrypted Virtualization），允许客户机所有者将密钥注入到虚拟机的内存中，而宿主机/hypervisor 无法读取它们。在 SEV 中，密钥注入在虚拟机启动过程的早期、客户机开始运行之前执行。

efi_secret 内核模块允许用户空间应用程序通过 securityfs 访问这些密钥。

## 密钥数据流


客户机固件可以预留一个指定的内存区域用于密钥注入，并在 EFI 配置表中以一个 `LINUX_EFI_COCO_SECRET_AREA_GUID` 条目（`adf956ad-e98c-484c-ae11-b51c7d336447`）发布其位置（基址 GPA 与长度）。该内存区域应由固件标记为 `EFI_RESERVED_TYPE`，因此内核不应将其用于自身目的。

在虚拟机启动期间，虚拟机管理器可以向该区域注入一个密钥。在 AMD SEV 和 SEV-ES 中，这是使用 `KVM_SEV_LAUNCH_SECRET` 命令执行的（参见 [sev]_）。注入的客户机所有者密钥数据的结构应当是一个由 GUID 引导的密钥值表；其二进制格式在 `drivers/virt/coco/efi_secret/efi_secret.c` 的“EFI 密钥区域的结构”一节中描述。

在内核启动时，内核的 EFI 驱动将密钥区域的位置（取自 EFI 配置表）保存在 `efi.coco_secret` 字段中。随后它会检查密钥区域是否已被填充：它映射该区域并检查其内容是否以 `EFI_SECRET_TABLE_HEADER_GUID`（`1e74f542-71dd-4d66-963e-ef4287ff173b`）开头。如果密钥区域已被填充，EFI 驱动会自动加载 efi_secret 内核模块，该模块通过 securityfs 将密钥暴露给用户空间应用程序。efi_secret 文件系统接口的详细信息见 [secrets-coco-abi]_。

## 应用使用示例


考虑一个在加密文件上执行计算的客户机。客户机所有者使用密钥注入机制提供解密密钥（= secret）。客户机应用程序从 efi_secret 文件系统读取该密钥，进而将文件解密到内存中，然后对内容执行所需的计算。

在此示例中，宿主机无法从磁盘映像中读取文件，因为它们已被加密。宿主机无法读取解密密钥，因为它通过密钥注入机制（= 安全通道）传递。宿主机无法从内存中读取解密后的内容，因为它是一个机密（内存加密）的客户机。

以下是一个在客户机中使用 efi_secret 模块的简单示例
```

	# ls -la /sys/kernel/security/secrets/coco
	total 0
	drwxr-xr-x 2 root root 0 Jun 28 11:54 .
	drwxr-xr-x 3 root root 0 Jun 28 11:54 ..
	-r--r----- 1 root root 0 Jun 28 11:54 736870e5-84f0-4973-92ec-06879ce3da0b
	-r--r----- 1 root root 0 Jun 28 11:54 83c83f7f-1356-4975-8b7e-d3a0b54312c6
	-r--r----- 1 root root 0 Jun 28 11:54 9553f55d-3da2-43ee-ab5d-ff17f78864d2
	-r--r----- 1 root root 0 Jun 28 11:54 e6f5a162-d67f-4750-a67c-5d065f2a9910

	# hd /sys/kernel/security/secrets/coco/e6f5a162-d67f-4750-a67c-5d065f2a9910
	00000000  74 68 65 73 65 2d 61 72  65 2d 74 68 65 2d 6b 61  |these-are-the-ka|
	00000010  74 61 2d 73 65 63 72 65  74 73 00 01 02 03 04 05  |ta-secrets......|
	00000020  06 07                                             |..|
	00000022

	# rm /sys/kernel/security/secrets/coco/e6f5a162-d67f-4750-a67c-5d065f2a9910

	# ls -la /sys/kernel/security/secrets/coco
	total 0
	drwxr-xr-x 2 root root 0 Jun 28 11:55 .
	drwxr-xr-x 3 root root 0 Jun 28 11:54 ..
	-r--r----- 1 root root 0 Jun 28 11:54 736870e5-84f0-4973-92ec-06879ce3da0b
	-r--r----- 1 root root 0 Jun 28 11:54 83c83f7f-1356-4975-8b7e-d3a0b54312c6
	-r--r----- 1 root root 0 Jun 28 11:54 9553f55d-3da2-43ee-ab5d-ff17f78864d2


```
## 参考资料


有关 SEV `LAUNCH_SECRET` 操作的更多信息，请参见 [sev-api-spec]_。
