## 鐢ㄤ簬 FPGA 缂栫▼鐨勫唴鏍告€?API


### 姒傝堪


鐢ㄤ簬 FPGA 缂栫▼鐨勫唴鏍告€?API 鏄潵鑷?FPGA manager銆乥ridge锛堟ˉ锛変笌 region锛堝尯鍩燂級
鐨?API 鐨勭粍鍚堛€傚疄闄呯敤浜庤Е鍙?FPGA 缂栫▼鐨勫嚱鏁版槸 fpga_region_program_fpga()銆?
fpga_region_program_fpga() 浣跨敤鐢?FPGA manager 涓?bridges 鎻愪緵鐨勫姛鑳姐€傚畠浼氾細

 - 閿佸畾鍖哄煙鐨?mutex
 - 閿佸畾璇ュ尯鍩熺殑 FPGA manager 鐨?mutex
 - 濡傛灉鎸囧畾浜嗙浉搴旀柟娉曪紝鍒欐瀯寤轰竴浠?FPGA bridges 鍒楄〃
 - 绂佺敤杩欎簺 bridges
 - 浣跨敤閫氳繃 :c`fpga_region->info` 浼犲叆鐨勪俊鎭 FPGA 杩涜缂栫▼
 - 閲嶆柊鍚敤杩欎簺 bridges
 - 閲婃斁閿?
struct fpga_image_info 鎸囧畾浜嗚瀵瑰摢涓?FPGA 闀滃儚杩涜缂栫▼銆傚畠鐢?fpga_image_info_alloc() 鍒嗛厤/閲婃斁锛屽苟鐢?fpga_image_info_free() 閲婃斁銆?
### 濡備綍浣跨敤涓€涓?region 鏉ョ紪绋?FPGA


褰?FPGA region 椹卞姩瀹屾垚鎺㈡祴锛坧robed锛夋椂锛屽畠浼氳幏寰椾竴涓寚鍚?FPGA manager 椹卞姩鐨?鎸囬拡锛屼粠鑰岀煡閬撹浣跨敤鍝釜 manager銆傝 region 瑕佷箞鎸佹湁涓€涓鍦ㄧ紪绋嬫湡闂存帶鍒剁殑
bridges 鍒楄〃锛岃涔堟寔鏈変竴涓寚鍚戞煇涓嚱鏁扮殑鎸囬拡锛岃鍑芥暟浼氾細

```

	#include <linux/fpga/fpga-mgr.h>
	#include <linux/fpga/fpga-region.h>

	struct fpga_image_info *info;
	int ret;

	/*
	 * 棣栧厛锛屽垎閰嶆弿杩拌缂栫▼鐨?FPGA 闀滃儚淇℃伅鐨勭粨鏋勪綋
	 */
	info = fpga_image_info_alloc(dev);
	if (!info)
		return -ENOMEM;

	/* 鎸夐渶璁剧疆鏍囧織锛屼緥濡傦細 */
	info->flags = FPGA_MGR_PARTIAL_RECONFIG;

	/*
	 * 鎸囨槑 FPGA 闀滃儚鎵€鍦ㄤ綅缃€備笅闈㈡槸浼唬鐮侊紱浣犲皢浣跨敤杩欎笁鑰呬箣涓€銆?	 */
	if (image is in a scatter gather table) {

		info->sgt = [your scatter gather table]

	} else if (image is in a buffer) {

		info->buf = [your image buffer]
		info->count = [image buffer size]

	} else if (image is in a firmware file) {

		info->firmware_name = devm_kstrdup(dev, firmware_name,
						   GFP_KERNEL);

	}

	/* 灏?info 娣诲姞鍒?region 骞舵墽琛岀紪绋?*/
	region->info = info;
	ret = fpga_region_program_fpga(region);

	/* 濡傛灉涓嶅啀闇€瑕侊紝閲婃斁闀滃儚 info */
	region->info = NULL;
	fpga_image_info_free(info);

	if (ret)
		return ret;

	/* 鐜板湪鏋氫妇 FPGA 涓嚭鐜扮殑浠讳綍纭欢銆?*/

```
### 鐢ㄤ簬缂栫▼ FPGA 鐨?API


- fpga_region_program_fpga() -  缂栫▼涓€涓?FPGA
- fpga_image_info() -  鎸囧畾瑕佸鍝釜 FPGA 闀滃儚杩涜缂栫▼
- fpga_image_info_alloc() -  鍒嗛厤涓€涓?FPGA 闀滃儚 info 缁撴瀯浣?- fpga_image_info_free() -  閲婃斁涓€涓?FPGA 闀滃儚 info 缁撴瀯浣?
   :functions: fpga_region_program_fpga

FPGA Manager 鏍囧織

   :doc: FPGA Manager flags

   :functions: fpga_image_info

   :functions: fpga_image_info_alloc

   :functions: fpga_image_info_free
