
## Linux 瀹夊叏妯″潡锛圠SM锛?

:Author: Casey Schaufler
:Date: 2023 骞?7 鏈?
Linux 瀹夊叏妯″潡锛圠SM锛夋彁渚涗簡涓€绉嶆満鍒讹紝鐢ㄤ簬涓?Linux 瀹夊叏绛栫暐瀹炵幇棰濆鐨?璁块棶鎺у埗銆?
鍚勪釜瀹夊叏妯″潡鍙互鏀寔浠ヤ笅浠绘剰灞炴€э細

`LSM_ATTR_CURRENT` 鏄繘绋嬪綋鍓嶃€佹椿璺冪殑瀹夊叏涓婁笅鏂囥€?proc 鏂囦欢绯荤粺閫氳繃 `/proc/self/attr/current` 鎻愪緵璇ュ€笺€?SELinux銆丼mack 涓?AppArmor 瀹夊叏妯″潡鏀寔姝ゅ睘鎬с€?Smack 杩橀€氳繃 `/proc/self/attr/smack/current` 鎻愪緵璇ュ€笺€?AppArmor 杩橀€氳繃 `/proc/self/attr/apparmor/current` 鎻愪緵璇ュ€笺€?
`LSM_ATTR_EXEC` 鏄綋鍓嶆槧鍍忚鎵ц鏃惰繘绋嬬殑瀹夊叏涓婁笅鏂囥€?proc 鏂囦欢绯荤粺閫氳繃 `/proc/self/attr/exec` 鎻愪緵璇ュ€笺€?SELinux 涓?AppArmor 瀹夊叏妯″潡鏀寔姝ゅ睘鎬с€?AppArmor 杩橀€氳繃 `/proc/self/attr/apparmor/exec` 鎻愪緵璇ュ€笺€?
`LSM_ATTR_FSCREATE` 鏄繘绋嬪湪鍒涘缓鏂囦欢绯荤粺瀵硅薄鏃朵娇鐢ㄧ殑瀹夊叏涓婁笅鏂囥€?proc 鏂囦欢绯荤粺閫氳繃 `/proc/self/attr/fscreate` 鎻愪緵璇ュ€笺€?SELinux 瀹夊叏妯″潡鏀寔姝ゅ睘鎬с€?
`LSM_ATTR_KEYCREATE` 鏄繘绋嬪湪鍒涘缓瀵嗛挜瀵硅薄鏃朵娇鐢ㄧ殑瀹夊叏涓婁笅鏂囥€?proc 鏂囦欢绯荤粺閫氳繃 `/proc/self/attr/keycreate` 鎻愪緵璇ュ€笺€?SELinux 瀹夊叏妯″潡鏀寔姝ゅ睘鎬с€?
`LSM_ATTR_PREV` 鏄缃綋鍓嶅畨鍏ㄤ笂涓嬫枃鏃惰繘绋嬬殑瀹夊叏涓婁笅鏂囥€?proc 鏂囦欢绯荤粺閫氳繃 `/proc/self/attr/prev` 鎻愪緵璇ュ€笺€?SELinux 涓?AppArmor 瀹夊叏妯″潡鏀寔姝ゅ睘鎬с€?AppArmor 杩橀€氳繃 `/proc/self/attr/apparmor/prev` 鎻愪緵璇ュ€笺€?
`LSM_ATTR_SOCKCREATE` 鏄繘绋嬪湪鍒涘缓濂楁帴瀛楀璞℃椂浣跨敤鐨勫畨鍏ㄤ笂涓嬫枃銆?proc 鏂囦欢绯荤粺閫氳繃 `/proc/self/attr/sockcreate` 鎻愪緵璇ュ€笺€?SELinux 瀹夊叏妯″潡鏀寔姝ゅ睘鎬с€?
## 鍐呮牳鎺ュ彛


### 璁剧疆褰撳墠杩涚▼鐨勫畨鍏ㄥ睘鎬?

    :identifiers: sys_lsm_set_self_attr

### 鑾峰彇褰撳墠杩涚▼鐨勬寚瀹氬畨鍏ㄥ睘鎬?

    :identifiers: sys_lsm_get_self_attr

    :identifiers: sys_lsm_list_modules

## 闄勫姞鏂囨。


- Documentation/security/lsm.rst
- Documentation/security/lsm-development.rst
