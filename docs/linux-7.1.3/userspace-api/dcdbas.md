## Dell 绯荤粺绠＄悊鍩虹椹卞姩


## 姒傝堪


Dell 绯荤粺绠＄悊鍩虹椹卞姩鎻愪緵浜嗕竴涓?sysfs 鎺ュ彛锛屼緵 Dell OpenManage 绛夌郴缁熺鐞嗚蒋浠跺湪鐗瑰畾鐨?Dell 绯荤粺涓婃墽琛岀郴缁熺鐞嗕腑鏂紙SMI锛変笌涓绘満鎺у埗鍔ㄤ綔锛堝湪 OS 鍏虫満鍚庤繘琛岀郴缁熺數婧愬惊鐜垨鏂數锛夈€?
Dell OpenManage 鍦ㄤ互涓?Dell PowerEdge 绯荤粺涓婇渶瑕佹椹卞姩锛?00銆?300銆?400銆?00SC銆?00SC銆?1500SC銆?550銆?00SC銆?600SC銆?50銆?655MC銆?00 涓?750銆傚叾浠?Dell 杞欢锛堝寮€婧愮殑 libsmbios
椤圭洰锛夐鏈熶細鍒╃敤姝ら┍鍔紝鍏朵腑鍙兘鍖呮嫭鍦ㄥ叾浠?Dell 绯荤粺涓婁娇鐢ㄦ椹卞姩銆?
Dell libsmbios 椤圭洰鑷村姏浜庡敖鍙兘澶氬湴鎻愪緵瀵?BIOS 淇℃伅鐨勮闂€傚叧浜?libsmbios 椤圭洰鐨勬洿澶?淇℃伅锛岃鍙傝 http://linux.dell.com/libsmbios/main/銆?

## 绯荤粺绠＄悊涓柇


鍦ㄦ煇浜?Dell 绯荤粺涓婏紝绯荤粺绠＄悊杞欢蹇呴』閫氳繃绯荤粺绠＄悊涓柇锛圫MI锛夎闂煇浜涚鐞嗕俊鎭€係MI 鏁版嵁
缂撳啿鍖哄繀椤讳綅浜?32 浣嶅湴鍧€绌洪棿涓紝涓?SMI 闇€瑕佽缂撳啿鍖虹殑鐗╃悊鍦板潃銆傞┍鍔ㄧ淮鎶?SMI 鎵€闇€鐨?鍐呭瓨锛屽苟涓哄簲鐢ㄧ▼搴忔彁渚涚敓鎴?SMI 鐨勬柟寮忋€?椹卞姩涓虹郴缁熺鐞嗚蒋浠跺垱寤轰互涓?sysfs 鏉＄洰锛?```

	/sys/devices/platform/dcdbas/smi_data
	/sys/devices/platform/dcdbas/smi_data_buf_phys_addr
	/sys/devices/platform/dcdbas/smi_data_buf_size
	/sys/devices/platform/dcdbas/smi_request

```
绯荤粺绠＄悊杞欢蹇呴』鎵ц浠ヤ笅姝ラ浠ヤ娇鐢ㄨ椹卞姩鎵ц涓€娆?SMI锛?
1) 閿佸畾 smi_data銆?2) 灏嗙郴缁熺鐞嗗懡浠ゅ啓鍏?smi_data銆?3) 鍚?smi_request 鍐欏叆 "1" 浠ョ敓鎴愯皟鐢ㄦ帴鍙?SMI锛屾垨鍐欏叆 "2" 浠ョ敓鎴愬師濮?SMI銆?4) 浠?smi_data 璇诲彇绯荤粺绠＄悊鍛戒护鐨勫搷搴斻€?5) 瑙ｉ攣 smi_data銆?

## 涓绘満鎺у埗鍔ㄤ綔


Dell OpenManage 鏀寔涓€绉嶄富鏈烘帶鍒剁壒鎬э紝鍏佽绠＄悊鍛樺湪 OS 瀹屾垚鍏虫満鍚庡绯荤粺鎵ц鐢垫簮寰幆鎴?鏂數銆傚湪鏌愪簺 Dell 绯荤粺涓婏紝璇ヤ富鏈烘帶鍒剁壒鎬ц姹傞┍鍔ㄥ湪 OS 瀹屾垚鍏虫満鍚庢墽琛屼竴娆?SMI銆?
椹卞姩涓虹郴缁熺鐞嗚蒋浠跺垱寤轰互涓?sysfs 鏉＄洰锛屼互瀹夋帓椹卞姩鍦ㄧ郴缁熷畬鎴愬叧鏈哄悗鎵ц鐢垫簮寰幆鎴栨柇鐢?涓绘満鎺у埗鍔ㄤ綔锛?
/sys/devices/platform/dcdbas/host_control_action
/sys/devices/platform/dcdbas/host_control_smi_type
/sys/devices/platform/dcdbas/host_control_on_shutdown

Dell OpenManage 浣跨敤姝ら┍鍔ㄦ墽琛岀數婧愬惊鐜垨鏂數涓绘満鎺у埗鍔ㄤ綔鐨勬楠ゅ涓嬶細

1) 灏嗗緟鎵ц鐨勪富鏈烘帶鍒跺姩浣滃啓鍏?host_control_action銆?2) 灏嗛┍鍔ㄩ渶瑕佹墽琛岀殑 SMI 绫诲瀷鍐欏叆 host_control_smi_type銆?3) 鍚?host_control_on_shutdown 鍐欏叆 "1" 浠ュ惎鐢ㄤ富鏈烘帶鍒跺姩浣溿€?4) 鍙戣捣 OS 鍏虫満銆?   锛堝綋椹卞姩鏀跺埌 OS 宸插畬鎴愬叧鏈虹殑閫氱煡鏃讹紝浼氭墽琛屼富鏈烘帶鍒?SMI銆傦級


## 涓绘満鎺у埗 SMI 绫诲瀷


涓嬭〃鏄剧ず浜嗕负鎵ц鐢垫簮寰幆鎴栨柇鐢典富鏈烘帶鍒跺姩浣滈渶瑕佸啓鍏?host_control_smi_type 鐨勫€硷細

=================== =====================
PowerEdge 绯荤粺      Host Control SMI 绫诲瀷
=================== =====================
      300             HC_SMITYPE_TYPE1
     1300             HC_SMITYPE_TYPE1
     1400             HC_SMITYPE_TYPE2
      500SC           HC_SMITYPE_TYPE2
     1500SC           HC_SMITYPE_TYPE2
     1550             HC_SMITYPE_TYPE2
      600SC           HC_SMITYPE_TYPE2
     1600SC           HC_SMITYPE_TYPE2
      650             HC_SMITYPE_TYPE2
     1655MC           HC_SMITYPE_TYPE2
      700             HC_SMITYPE_TYPE3
      750             HC_SMITYPE_TYPE3
=================== =====================
