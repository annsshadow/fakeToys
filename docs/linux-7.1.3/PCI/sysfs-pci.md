
## 閫氳繃 sysfs 璁块棶 PCI 璁惧璧勬簮


sysfs 閫氬父鎸傝浇浜?/sys锛屽湪骞冲彴涓婃彁渚涘 PCI 璧勬簮鐨勮闂?
```

     /sys/devices/pci0000:17
     |-- 0000:17:00.0
     |   |-- class
     |   |-- config
     |   |-- device
     |   |-- enable
     |   |-- irq
     |   |-- local_cpus
     |   |-- remove
     |   |-- resource
     |   |-- resource0
     |   |-- resource1
     |   |-- resource2
     |   |-- revision
     |   |-- rom
     |   |-- subsystem_device
     |   |-- subsystem_vendor
     |   `-- vendor
     `-- ...

```
鏈€椤跺眰鐨勫厓绱犳弿杩?PCI 鍩熷拰鎬荤嚎鍙枫€傚湪鏈緥涓紝鍩熷彿涓?0000锛屾€荤嚎鍙蜂负 17锛堜袱涓€煎潎涓哄崄鍏繘鍒讹級銆傝鎬荤嚎涓婃湁涓€涓綅浜庢彃妲?0 鐨勫崟鍔熻兘璁惧銆傚煙鍙峰拰鎬荤嚎鍙蜂細涓轰簡鏂逛究鑰岄噸澶嶇粰鍑恒€傝澶囩洰褰曚笅鏈夊嚑涓枃浠讹紝姣忎釜鏂囦欢鍚勬湁鍏跺姛鑳姐€?

       =================== =====================================================
       file		   function
       =================== =====================================================
       class		   PCI class (ascii, ro)
       config		   PCI config space (binary, rw)
       device		   PCI device (ascii, ro)
       enable	           Whether the device is enabled (ascii, rw)
       irq		   IRQ number (ascii, ro)
       local_cpus	   nearby CPU mask (cpumask, ro)
       remove		   remove device from kernel's list (ascii, wo)
       resource		   PCI resource host addresses (ascii, ro)
       resource0..N	   PCI resource N, if present (binary, mmap, rw\ [^1^]_)
       resource0_wc..N_wc  PCI WC map resource N, if prefetchable (binary, mmap)
       revision		   PCI revision (ascii, ro)
       rom		   PCI ROM resource, if present (binary, ro)
       subsystem_device	   PCI subsystem device (ascii, ro)
       subsystem_vendor	   PCI subsystem vendor (ascii, ro)
       vendor		   PCI vendor (ascii, ro)
       =================== =====================================================

```

  ro - read only file
  rw - file is readable and writable
  wo - write only file
  mmap - file is mmapable
  ascii - file contains ascii text
  binary - file contains binary data
  cpumask - file contains a cpumask type

```

鍙鏂囦欢鏄俊鎭€х殑锛屽瀹冧滑鐨勫啓鍏ュ皢琚拷鐣ワ紝'rom' 鏂囦欢闄ゅ銆傚彲鍐欐枃浠跺彲鐢ㄤ簬瀵硅澶囨墽琛屾搷浣滐紙渚嬪鏇存敼閰嶇疆绌洪棿銆佸嵏杞借澶囷級銆傚彲閫氳繃鍦ㄥ亸绉婚噺 0 澶勫鏂囦欢杩涜 mmap 鏉ヨ幏寰楀彲鏄犲皠鏂囦欢锛屽苟鍙敤浜庝粠鐢ㄦ埛绌洪棿瀹為檯瀵硅澶囪繘琛岀紪绋嬨€傛敞鎰忥紝鏌愪簺骞冲彴涓嶆敮鎸佸鏌愪簺璧勬簮杩涜 mmap锛屽洜姝ゅ姟蹇呮鏌ヤ换浣曚竴娆″皾璇?mmap 鐨勮繑鍥炲€笺€傚叾涓渶鍊煎緱娉ㄦ剰鐨勬槸 I/O 绔彛璧勬簮锛屽畠浠篃鎻愪緵璇?鍐欒闂€?

'enable' 鏂囦欢鎻愪緵涓€涓鏁板櫒锛屾寚绀鸿澶囪鍚敤鐨勬鏁般€傚鏋?'enable' 鏂囦欢褰撳墠杩斿洖 '4'锛屽苟涓斿悜鍏朵腑鍐欏叆涓€涓?'1'锛屽畠灏嗚繑鍥?'5'銆傚悜鍏朵腑鍐欏叆 '0' 浼氬皢璁℃暟鍑忓皯銆備笉杩囷紝鍗充娇瀹冨洖鍒?0锛屾煇浜涘垵濮嬪寲鎿嶄綔涔熷彲鑳戒笉浼氳鎾ら攢銆?

'rom' 鏂囦欢鐗规畩涔嬪鍦ㄤ簬锛屽鏋滃彲鐢紝瀹冩彁渚涘璁惧 ROM 鏂囦欢鐨勫彧璇昏闂€備笉杩囧畠榛樿鏄鐢ㄧ殑锛屽洜姝ゅ簲鐢ㄧ▼搴忓簲褰撳湪灏濊瘯璇诲彇璋冪敤涔嬪墠鍚戣鏂囦欢鍐欏叆瀛楃涓?"1" 鏉ュ惎鐢ㄥ畠锛屽苟鍦ㄨ闂箣鍚庨€氳繃鍚戣鏂囦欢鍐欏叆 "0" 鏉ョ鐢ㄥ畠銆傛敞鎰忥紝璁惧蹇呴』澶勪簬鍚敤鐘舵€侊紝ROM 璇诲彇鎵嶈兘鎴愬姛杩斿洖鏁版嵁銆傚湪娌℃湁椹卞姩缁戝畾鍒拌璁惧鐨勬儏鍐典笅锛屽彲浠ヤ娇鐢ㄤ笂鏂囪杞界殑 'enable' 鏂囦欢灏嗗叾鍚敤銆?

'remove' 鏂囦欢鐢ㄤ簬绉婚櫎 PCI 璁惧锛屾柟娉曟槸鍚戣鏂囦欢鍐欏叆涓€涓潪闆舵暣鏁般€傝繖涓嶆秹鍙婁换浣曠被鍨嬬殑鐑彃鎷斿姛鑳斤紝渚嬪鍏抽棴璁惧鐢垫簮銆傝璁惧浼氫粠鍐呮牳鐨?PCI 璁惧鍒楄〃涓绉婚櫎锛屽叾瀵瑰簲鐨?sysfs 鐩綍琚垹闄わ紝骞朵笖璇ヨ澶囦細浠庝换浣曢檮鍔犲埌瀹冪殑椹卞姩涓绉婚櫎銆備笉鍏佽绉婚櫎 PCI 鏍规€荤嚎銆?

### 閫氳繃 sysfs 璁块棶浼犵粺璧勬簮


濡傛灉搴曞眰骞冲彴鏀寔锛屼紶缁?I/O 绔彛鍜?ISA 鍐呭瓨璧勬簮涔熶細鍦?sysfs 涓彁渚涖€傚畠浠綅浜?PCI 绫诲眰绾х粨鏋勪腑锛?
```

	/sys/class/pci_bus/0000:17/
	|-- bridge -> ../../../devices/pci0000:17
	|-- cpuaffinity
	|-- legacy_io
	`-- legacy_mem

```
legacy_io 鏂囦欢鏄竴涓/鍐欐枃浠讹紝搴旂敤绋嬪簭鍙敤瀹冩潵杩涜浼犵粺绔彛 I/O銆傚簲鐢ㄧ▼搴忓簲褰撴墦寮€璇ユ枃浠讹紝瀹氫綅鍒版湡鏈涚殑绔彛锛堜緥濡?0x3e8锛夊苟杩涜 1銆? 鎴?4 瀛楄妭鐨勮鎴栧啓銆俵egacy_mem 鏂囦欢搴斿綋浠ュ搴斾簬鏈熸湜鍐呭瓨鍋忕Щ閲忕殑鍋忕Щ閲忚繘琛?mmap锛屼緥濡?VGA 甯х紦鍐茬殑 0xa0000銆傜劧鍚庡簲鐢ㄧ▼搴忓彲浠ワ紙鍦ㄦ鏌ヨ繃閿欒涔嬪悗锛夌洿鎺ヨВ寮曠敤杩斿洖鐨勬寚閽堟潵璁块棶浼犵粺鍐呭瓨绌洪棿銆?

### 鍦ㄦ柊骞冲彴涓婃敮鎸?PCI 璁块棶


涓轰簡鏀寔濡備笂鎵€杩扮殑 PCI 璧勬簮鏄犲皠锛孡inux 骞冲彴浠ｇ爜鐞嗘兂鎯呭喌涓嬪簲褰撳畾涔?ARCH_GENERIC_PCI_MMAP_RESOURCE 骞朵娇鐢ㄨ鍔熻兘鐗规€х殑閫氱敤瀹炵幇銆備负浜嗘敮鎸?/proc/bus/pci 涓€氳繃鏂囦欢杩涜 mmap() 鐨勫巻鍙叉帴鍙ｏ紝骞冲彴涔熷彲浠ヨ缃?HAVE_PCI_MMAP銆?

鎴栬€咃紝璁剧疆浜?HAVE_PCI_MMAP 鐨勫钩鍙板彲浠ユ彁渚涘畠浠嚜宸辩殑 pci_mmap_resource_range() 瀹炵幇锛岃€屼笉鏄畾涔?ARCH_GENERIC_PCI_MMAP_RESOURCE銆?

鏀寔 PCI 璧勬簮鍐欏悎骞舵槧灏勭殑骞冲彴蹇呴』瀹氫箟 arch_can_pci_mmap_wc()锛屽綋鍏佽鍐欏悎骞舵椂锛屽畠鍦ㄨ繍琛屾椂搴旀眰鍊间负闈為浂銆傜被浼煎湴锛屾敮鎸?I/O 璧勬簮鏄犲皠鐨勫钩鍙板畾涔?arch_can_pci_mmap_io()銆?

浼犵粺璧勬簮鐢?HAVE_PCI_LEGACY 瀹氫箟淇濇姢銆傚笇鏈涙敮鎸佷紶缁熷姛鑳界殑骞冲彴搴斿綋瀹氫箟瀹冿紝骞舵彁渚?pci_legacy_read銆乸ci_legacy_write 鍜?pci_mmap_legacy_page_range 鍑芥暟銆?
