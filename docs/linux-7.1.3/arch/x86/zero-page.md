
## 零页（Zero Page）


struct boot_params 中的附加字段是内核 32 位引导协议的一部分。这些应当由引导加载程序或内核的 16 位实模式（real-mode）设置代码填写。对其的引用/设置主要位于
```

  arch/x86/include/uapi/asm/bootparam.h

```
===========	=====	=======================	=================================================
Offset/Size	Proto	Name			Meaning

000/040		ALL	screen_info		文本模式或帧缓冲信息
						(struct screen_info)
040/014		ALL	apm_bios_info		APM BIOS 信息 (struct apm_bios_info)
058/008		ALL	tboot_addr      	tboot 共享页的物理地址
060/010		ALL	ist_info		Intel SpeedStep (IST) BIOS 支持信息
						(struct ist_info)
070/008		ALL	acpi_rsdp_addr		ACPI RSDP 表的物理地址
080/010		ALL	hd0_info		hd0 磁盘参数，已废弃！！
090/010		ALL	hd1_info		hd1 磁盘参数，已废弃！！
0A0/010		ALL	sys_desc_table		系统描述表 (struct sys_desc_table)，
						OBSOLETE!!
0B0/010		ALL	olpc_ofw_header		OLPC 的 OpenFirmware CIF 及其相关结构
0C0/004		ALL	ext_ramdisk_image	ramdisk_image 高 32 位
0C4/004		ALL	ext_ramdisk_size	ramdisk_size 高 32 位
0C8/004		ALL	ext_cmd_line_ptr	cmd_line_ptr 高 32 位
13C/004		ALL	cc_blob_address		机密计算（Confidential Computing）blob 的物理地址
140/080		ALL	edid_info		视频模式设置 (struct edid_info)
1C0/020		ALL	efi_info		EFI 32 信息 (struct efi_info)
1E0/004		ALL	alt_mem_k		备用内存检测，单位 KB
1E4/004		ALL	scratch			内核设置代码的临时（scratch）字段
1E8/001		ALL	e820_entries		e820_table（如下）中的条目数
1E9/001		ALL	eddbuf_entries		eddbuf（如下）中的条目数
1EA/001		ALL	edd_mbr_sig_buf_entries	edd_mbr_sig_buffer 中的条目数
						（如下）
1EB/001		ALL     kbd_status      	Numlock 已启用
1EC/001		ALL     secure_boot		固件中已启用安全启动（Secure boot）
1EF/001		ALL	sentinel		用于检测损坏的引导加载程序
290/040		ALL	edd_mbr_sig_buffer	EDD MBR 签名
2D0/A00		ALL	e820_table		E820 内存映射表
						（struct e820_entry 数组）
D00/1EC		ALL	eddbuf			EDD 数据 (struct edd_info 数组)
===========	=====	=======================	=================================================
