## DAX 椹卞姩鎿嶄綔


`Direct Access Device` 椹卞姩鏈€鍒濊璁＄敤浜庝负绫诲唴瀛樺潡璁惧鎻愪緵绫诲唴瀛樼殑璁块棶鏈哄埗銆?瀹冭鎵╁睍浠ユ敮鎸?CXL 鍐呭瓨璁惧锛屽悗鑰呮彁渚涚敤鎴烽厤缃殑鍐呭瓨璁惧銆?
CXL 瀛愮郴缁熶緷璧?DAX 瀛愮郴缁熸潵瀹炵幇浠ヤ笅涔嬩竴锛?
- 閫氳繃 `/dev/daxN.Y` 鐢熸垚闈㈠悜鐢ㄦ埛绌洪棿鐨勬枃浠跺紡鎺ュ彛锛屾垨
- 璋冪敤 memory-hotplug 鎺ュ彛灏?CXL 鍐呭瓨鍔犲叆椤靛垎閰嶅櫒銆?
DAX 瀛愮郴缁熼€氳繃 `cxl_dax_region` 椹卞姩鏆撮湶姝よ兘鍔涖€俙dax_region` 鎻愪緵 CXL
`memory_region` 涓?`DAX Device` 涔嬮棿鐨勮浆鎹€?
## DAX 璁惧


`DAX Device` 鏄湪 `/dev/daxN.Y` 涓毚闇茬殑鏂囦欢寮忔帴鍙ｃ€傞€氳繃 DAX 璁惧鏆撮湶鐨勫唴瀛?鍖哄煙鍙敱鐢ㄦ埛绌洪棿杞欢閫氳繃 `mmap()` 绯荤粺璋冪敤璁块棶銆傜粨鏋滄槸鍦ㄤ换鍔＄殑椤佃〃涓洿鎺?鏄犲皠鍒?CXL 瀹归噺銆?
甯屾湜鎵嬪姩澶勭悊 CXL 鍐呭瓨鍒嗛厤鐨勭敤鎴峰簲浣跨敤姝ゆ帴鍙ｃ€?
## kmem 杞崲


`dax_kmem` 椹卞姩灏?`DAX Device` 杞崲涓虹敱 `kernel/memory-hotplug.c` 绠＄悊鐨勪竴
绯诲垪 `hotplug memory blocks`銆傛瀹归噺灏嗗湪鐢ㄦ埛閫夋嫨鐨勫唴瀛樺尯涓毚闇茬粰鍐呮牳椤靛垎閰嶅櫒銆?
`memmap_on_memory` 璁剧疆锛堝叏灞€涓?DAX 璁惧鏈湴锛夊喅瀹氫簡鍐呮牳灏嗕粠浣曞鍒嗛厤姝ゅ唴瀛樼殑
`struct folio` 鎻忚堪绗︺€傚鏋滆缃簡 `memmap_on_memory`锛屽唴瀛樼儹鎻掓嫈灏嗛鐣欎竴閮ㄥ垎
鍐呭瓨鍧楀閲忔潵鍒嗛厤 folio銆傚鏋滄湭璁剧疆锛屽唴瀛樺皢閫氳繃姝ｅ父鐨?`GFP_KERNEL` 鍒嗛厤鈥斺€斿洜姝?寰堝彲鑳戒細钀藉埌鎵ц鐑彃鎷旀搷浣滅殑 CPU 鐨勬湰鍦?NUMA 鑺傜偣涓娿€?