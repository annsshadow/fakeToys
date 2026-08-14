## request_firmware API


浣犻€氬父浼氬厛鍔犺浇鍥轰欢锛岀劧鍚庝互鏌愮鏂瑰紡灏嗗叾鍔犺浇鍒颁綘鐨勮澶囦腑銆?```

	 if(request_firmware(&fw_entry, $FIRMWARE, device) == 0)
                copy_fw_to_device(fw_entry->data, fw_entry->size);
	 release_firmware(fw_entry);

```
## 鍚屾鍥轰欢璇锋眰


鍚屾鍥轰欢璇锋眰浼氫竴鐩寸瓑寰咃紝鐩村埌鎵惧埌鍥轰欢鎴栬繑鍥為敊璇€?
### request_firmware

   :functions: request_firmware

### firmware_request_nowarn

   :functions: firmware_request_nowarn

### firmware_request_platform

   :functions: firmware_request_platform

### request_firmware_direct

   :functions: request_firmware_direct

### request_firmware_into_buf

   :functions: request_firmware_into_buf

## 寮傛鍥轰欢璇锋眰


寮傛鍥轰欢璇锋眰鍏佽椹卞姩浠ｇ爜涓嶅繀绛夊緟鍥轰欢鎴栭敊璇繑鍥炪€傛彁渚涗簡鍑芥暟鍥炶皟锛屼互渚垮湪鎵惧埌鍥轰欢鎴栭敊璇椂閫氳繃鍥炶皟閫氱煡椹卞姩銆俽equest_firmware_nowait() 涓嶈兘鍦ㄥ師瀛愪笂涓嬫枃涓皟鐢ㄣ€?
### request_firmware_nowait

   :functions: request_firmware_nowait

## 閲嶅惎鏃剁殑鐗规畩浼樺寲


鏌愪簺璁惧鍏锋湁涓€椤逛紭鍖栵紝浣垮浐浠跺湪绯荤粺閲嶅惎鏈熼棿寰椾互淇濈暀銆備娇鐢ㄨ繖绫讳紭鍖栨椂锛岄┍鍔ㄤ綔鑰呭繀椤荤‘淇濆浐浠跺湪浠庢寕璧锋仮澶嶆椂浠嶇劧鍙敤锛岃繖鍙互閫氳繃 firmware_request_cache() 鏉ヤ唬鏇胯姹傚姞杞藉浐浠跺疄鐜般€?
### firmware_request_cache()

   :functions: firmware_request_cache

## 璇锋眰鍥轰欢 API 棰勬湡鐨勯┍鍔ㄤ娇鐢ㄦ柟寮?

涓€鏃?API 璋冪敤杩斿洖锛屼綘灏卞鐞嗗浐浠讹紝鐒跺悗閲婃斁鍥轰欢銆備緥濡傦紝濡傛灉浣犱娇鐢ㄤ簡 request_firmware() 骞朵笖瀹冭繑鍥炰簡锛岄┍鍔ㄥ氨鍙互鍦?fw_entry->{data,size} 涓闂浐浠堕暅鍍忋€傚鏋滃嚭浜嗛棶棰橈紝request_firmware() 杩斿洖闈為浂鍊硷紝骞朵笖 fw_entry 琚涓?NULL銆備竴鏃︿綘鐨勯┍鍔ㄥ鐞嗗畬鍥轰欢锛屽畠灏卞彲浠ヨ皟鐢?release_firmware(fw_entry) 鏉ラ噴鏀惧浐浠堕暅鍍忎互鍙婁换浣曠浉鍏宠祫婧愩€?