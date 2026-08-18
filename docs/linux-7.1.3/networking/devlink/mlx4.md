
## mlx4 devlink 鏀寔


鏈枃妗ｆ弿杩?`mlx4` 璁惧椹卞姩瀹炵幇鐨?devlink 鐗规€с€?
## 鍙傛暟


   - - 鍚嶇О
     - 妯″紡
   - - `internal_err_reset`
     - driverinit, runtime
   - - `max_macs`
     - driverinit
   - - `region_snapshot_enable`
     - driverinit, runtime

`mlx4` 椹卞姩杩樺疄鐜颁互涓嬮┍鍔ㄧ壒瀹氱殑鍙傛暟銆?
   :widths: 5 5 5 85

   - - 鍚嶇О
     - 绫诲瀷
     - 妯″紡
     - 鎻忚堪
   - - `enable_64b_cqe_eqe`
     - Boolean
     - driverinit
     - 濡傛灉 FW 鏀寔锛屽惎鐢?64 瀛楄妭 CQEs/EQEs銆?   - - `enable_4k_uar`
     - Boolean
     - driverinit
     - 鍚敤浣跨敤 4k UAR銆?
`mlx4` 椹卞姩鏀寔閫氳繃 `DEVLINK_CMD_RELOAD` 閲嶆柊鍔犺浇銆?
## 鍖哄煙


`mlx4` 椹卞姩鏀寔鍦ㄥ嚭鐜颁弗閲嶅浐浠堕棶棰樻椂杞偍鍥轰欢 PCI crspace 涓庡仴搴风紦鍐插尯銆?
濡傛灉鍥轰欢鍛戒护瓒呮椂銆佸浐浠跺崱浣忥紝鎴?catastrophic 缂撳啿鍖哄嚭鐜伴潪闆跺€硷紝椹卞姩灏嗘媿鎽勫揩鐓с€?
`cr-space` 鍖哄煙灏嗗寘鍚浐浠?PCI crspace 鍐呭銆俙fw-health` 鍖哄煙灏嗗寘鍚澶囧浐浠剁殑
鍋ュ悍缂撳啿鍖恒€傝繖涓や釜鍖哄煙鐨勫揩鐓ч兘鍦ㄧ浉鍚岀殑浜嬩欢瑙﹀彂鏃舵媿鎽勩€?