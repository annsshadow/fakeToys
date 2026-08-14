
## 闆堕〉锛圸ero Page锛?

struct boot_params 涓殑闄勫姞瀛楁鏄唴鏍?32 浣嶅紩瀵煎崗璁殑涓€閮ㄥ垎銆傝繖浜涘簲褰撶敱寮曞鍔犺浇绋嬪簭鎴栧唴鏍哥殑 16 浣嶅疄妯″紡锛坮eal-mode锛夎缃唬鐮佸～鍐欍€傚鍏剁殑寮曠敤/璁剧疆涓昏浣嶄簬
```

  arch/x86/include/uapi/asm/bootparam.h

```
===========	=====	=======================	=================================================
Offset/Size	Proto	Name			Meaning

000/040		ALL	screen_info		鏂囨湰妯″紡鎴栧抚缂撳啿淇℃伅
						(struct screen_info)
040/014		ALL	apm_bios_info		APM BIOS 淇℃伅 (struct apm_bios_info)
058/008		ALL	tboot_addr      	tboot 鍏变韩椤电殑鐗╃悊鍦板潃
060/010		ALL	ist_info		Intel SpeedStep (IST) BIOS 鏀寔淇℃伅
						(struct ist_info)
070/008		ALL	acpi_rsdp_addr		ACPI RSDP 琛ㄧ殑鐗╃悊鍦板潃
080/010		ALL	hd0_info		hd0 纾佺洏鍙傛暟锛屽凡搴熷純锛侊紒
090/010		ALL	hd1_info		hd1 纾佺洏鍙傛暟锛屽凡搴熷純锛侊紒
0A0/010		ALL	sys_desc_table		绯荤粺鎻忚堪琛?(struct sys_desc_table)锛?						OBSOLETE!!
0B0/010		ALL	olpc_ofw_header		OLPC 鐨?OpenFirmware CIF 鍙婂叾鐩稿叧缁撴瀯
0C0/004		ALL	ext_ramdisk_image	ramdisk_image 楂?32 浣?0C4/004		ALL	ext_ramdisk_size	ramdisk_size 楂?32 浣?0C8/004		ALL	ext_cmd_line_ptr	cmd_line_ptr 楂?32 浣?13C/004		ALL	cc_blob_address		鏈哄瘑璁＄畻锛圕onfidential Computing锛塨lob 鐨勭墿鐞嗗湴鍧€
140/080		ALL	edid_info		瑙嗛妯″紡璁剧疆 (struct edid_info)
1C0/020		ALL	efi_info		EFI 32 淇℃伅 (struct efi_info)
1E0/004		ALL	alt_mem_k		澶囩敤鍐呭瓨妫€娴嬶紝鍗曚綅 KB
1E4/004		ALL	scratch			鍐呮牳璁剧疆浠ｇ爜鐨勪复鏃讹紙scratch锛夊瓧娈?1E8/001		ALL	e820_entries		e820_table锛堝涓嬶級涓殑鏉＄洰鏁?1E9/001		ALL	eddbuf_entries		eddbuf锛堝涓嬶級涓殑鏉＄洰鏁?1EA/001		ALL	edd_mbr_sig_buf_entries	edd_mbr_sig_buffer 涓殑鏉＄洰鏁?						锛堝涓嬶級
1EB/001		ALL     kbd_status      	Numlock 宸插惎鐢?1EC/001		ALL     secure_boot		鍥轰欢涓凡鍚敤瀹夊叏鍚姩锛圫ecure boot锛?1EF/001		ALL	sentinel		鐢ㄤ簬妫€娴嬫崯鍧忕殑寮曞鍔犺浇绋嬪簭
290/040		ALL	edd_mbr_sig_buffer	EDD MBR 绛惧悕
2D0/A00		ALL	e820_table		E820 鍐呭瓨鏄犲皠琛?						锛坰truct e820_entry 鏁扮粍锛?D00/1EC		ALL	eddbuf			EDD 鏁版嵁 (struct edd_info 鏁扮粍)
===========	=====	=======================	=================================================
