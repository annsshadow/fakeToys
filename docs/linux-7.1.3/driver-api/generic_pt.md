
## 閫氱敤鍩烘暟椤佃〃


	:doc: Generic Radix Page Table

	:doc: Generic Page Table Language

## 鐢ㄦ硶


閫氱敤 PT 琚瀯寤轰负涓€涓缂栬瘧绯荤粺銆傜敱浜庢瘡绉嶆牸寮忛兘浣跨敤涓€缁勯€氱敤鍚嶇О鏉ユ彁渚?API锛屽洜姝ゅ湪
涓€涓紪璇戝崟鍏冨唴鍙兘鏈変竴绉嶆牸寮忓浜庢椿鍔ㄧ姸鎬併€傝繖绉嶈璁￠伩鍏嶄簡鍥寸粫搴曞眰 API 鐨勫嚱鏁版寚閽堛€?
鐩稿弽锛屽嚱鏁版寚閽堝彲浠ヨ惤鍦ㄦ洿楂樺眰绾х殑 API锛堝嵆 map/unmap 绛夛級涓婏紝鑰屾瘡绉嶆牸寮忕殑浠ｇ爜鍙互
鐩存帴鍐呰仈鍒拌鏍煎紡鐨勭紪璇戝崟鍏冧腑銆傚浜庣被浼?IOMMU 鐨勬儏鍐碉紝姣忕鏍煎紡閮戒細琚紪璇戣繘涓€涓?鎸夋牸寮忓垝鍒嗙殑 IOMMU 鎿嶄綔鍐呮牳妯″潡銆?
涓烘锛屾瘡涓紪璇戝崟鍏冪殑 .c 鏂囦欢灏嗗悓鏃跺寘鍚牸寮忓ご鏂囦欢鍜岀敤浜庡疄鐜扮殑閫氱敤浠ｇ爜銆備緥濡傦紝鍦?涓€涓疄鐜扮紪璇戝崟鍏冧腑锛屽ご鏂囦欢閫氬父浼氭寜濡備笅鏂瑰紡鍖呭惈锛?
```

	#include <linux/generic_pt/common.h>
	#include "defs_amdv1.h"
	#include "../pt_defs.h"
	#include "amdv1.h"
	#include "../pt_common.h"
	#include "../pt_iter.h"
	#include "../iommu_pt.h"  /* The IOMMU implementation */

```
iommu_pt.h 鍖呭惈浜嗗皢鏍规嵁 AMDv1 鎻愪緵鐨勫畯瀹氫箟鏉ョ敓鎴?map/unmap 绛夋搷浣滅殑瀹忓畾涔夈€傜敓鎴愮殑
妯″潡灏嗗叿鏈夎濡?pt_iommu_amdv1_init() 杩欐牱鐨勫鍑虹鍙枫€?
鏈夊叧 IOMMU 瀹炵幇濡備綍浣跨敤澶氱紪璇戞潵鐢熸垚鎸夋牸寮忓垝鍒嗙殑 ops 缁撴瀯浣撴寚閽堬紝璇峰弬闃?drivers/iommu/generic_pt/fmt/iommu_template.h 涓殑绀轰緥銆?
鏍煎紡浠ｇ爜鐨勭紪鍐欐柟寮忔槸锛岄€氱敤鍚嶇О鐢?#define 鏄犲皠鍒板悇鏍煎紡鐗瑰畾鐨勫敮涓€鍚嶇О銆傝繖鏃ㄥ湪閫氳繃
閬垮厤鎵€鏈変笉鍚屾牸寮忎箣闂寸殑绗﹀彿鍐茬獊鏉ヨ緟鍔╄皟璇曘€?
瀵煎嚭鐨勭鍙峰拰鍏朵粬鍏ㄥ眬鍚嶇О閫氳繃 NS() 杈呭姪瀹忎娇鐢ㄦ寜鏍煎紡鍒掑垎鐨勫瓧绗︿覆杩涜淇グ锛坢angle锛夈€?
璇ユ牸寮忎娇鐢?struct pt_common 浣滀负琛ㄧ殑椤跺眰缁撴瀯浣擄紝姣忕鏍煎紡閮戒細鏈夎嚜宸辩殑 struct pt_xxx
鏉ュ唴宓屽畠锛屼互瀛樺偍鏍煎紡鐗瑰畾鐨勪俊鎭€?
璇ュ疄鐜颁細杩涗竴姝ュ皢 struct pt_common 鍖呰鍦ㄥ畠鑷繁鐨勯《灞傜粨鏋勪綋涓紝渚嬪
struct pt_iommu_amdv1銆?
### 浣嶄簬 struct pt_common 绾у埆鐨勬牸寮忓嚱鏁?

	:identifiers:

### 杩唬杈呭姪鍑芥暟



### 缂栧啓涓€绉嶆牸寮?

鏈€濂戒粠涓庣洰鏍囩浉浼肩殑绠€鍗曟牸寮忓紑濮嬨€倄86_64 閫氬父鏄畝鍗曟儏褰㈢殑鑹ソ鍙傝€冿紝鑰?AMDv1 鍒欑浉褰?瀹屾暣銆?
鎵€闇€鐨?inline 鍑芥暟闇€瑕佸湪鏍煎紡澶存枃浠朵腑瀹炵幇銆?```

 static inline pt_oaddr_t amdv1pt_entry_oa(const struct pt_state *pts)
 {
	[..]
 }
 #define pt_entry_oa amdv1pt_entry_oa

```
鍏朵腑锛屼竴涓敮涓€鍛藉悕鐨勬寜鏍煎紡 inline 鍑芥暟鎻愪緵瀹炵幇锛岃€屼竴涓?define 灏嗗叾鏄犲皠鍒伴€氱敤鍚嶇О銆?杩欐棬鍦ㄤ娇璋冭瘯绗﹀彿宸ヤ綔寰楁洿濂姐€傚簲濮嬬粓浣跨敤 inline 鍑芥暟锛屽洜涓?pt_common.h 涓殑鍘熷瀷浼氳
缂栬瘧鍣ㄩ獙璇佸嚱鏁扮鍚嶄互闃叉閿欒銆?
鏌ョ湅 pt_fmt_defaults.h 浠ヤ簡瑙ｄ竴浜涘彲閫夌殑 inline 鍑芥暟銆?
涓€鏃﹁鏍煎紡缂栬瘧閫氳繃锛屽氨搴斿綋璁╁畠閫氳繃閫氱敤椤佃〃
```

   $ tools/testing/kunit/kunit.py run --build_dir build_kunit_x86_64 --arch x86_64 --kunitconfig ./drivers/iommu/generic_pt/.kunitconfig amdv1_fmt_test.*
   [...]
   [11:15:08] Testing complete. Ran 9 tests: passed: 9
   [11:15:09] Elapsed time: 3.137s total, 0.001s configuring, 2.368s building, 0.311s running

```
閫氱敤娴嬭瘯鏃ㄥ湪楠岃瘉鏍煎紡鍑芥暟锛屽苟鎻愪緵鏇存竻鏅扮殑澶辫触淇℃伅浠ュ姞蹇棶棰樺畾浣嶃€備竴鏃﹁繖浜涢€氳繃锛屽氨
搴斿綋杩愯鏁翠釜 kunit 娴嬭瘯濂椾欢銆?
### IOMMU 澶辨晥鐗规€?

澶辨晥鏄〉琛ㄧ畻娉曞浣曚笌椤佃〃鍐呭瓨鐨勭‖浠剁紦瀛橈紙閫氬父绉颁负 TLB锛堝浜?IOMMU 鎯呭舰鍒欎负
IOTLB锛夛級淇濇寔鍚屾鐨勬柟寮忋€?
鏍规嵁璁捐锛孴LB 鍙互瀛樺偍瀛樺湪锛坧resent锛夌殑 PTE銆佷笉瀛樺湪锛坣on-present锛夌殑 PTE 浠ュ強琛?鎸囬拡銆傛瘡涓‖浠堕兘鏈夎嚜宸辨弿杩板摢浜涘唴瀹瑰凡鍙樻洿銆佷粠鑰屽皢宸插彉鏇撮」浠?TLB 涓Щ闄ょ殑鏂规硶銆?
#### PT_FEAT_FLUSH_RANGE


PT_FEAT_FLUSH_RANGE 鏄渶瀹规槗鐞嗚В鐨勬柟妗堛€傚畠璇曞浘涓烘瘡涓搷浣滅敓鎴愬崟涓寖鍥村け鏁堬紝濡傛灉
瀛樺湪涓嶉渶瑕佸け鏁堢殑 VA 闂撮殭锛屽垯浼氳繃搴﹀け鏁堛€傚畠鍦ㄥ彈褰卞搷鐨?VA 鑼冨洿涓庡け鏁堟搷浣滄暟閲忎箣闂村仛
鏉冭　銆傚畠涓嶈窡韪鍦ㄥけ鏁堢殑鍐呭锛涗絾鏄紝濡傛灉蹇呴』閲婃斁椤碉紝鍒欏繀椤讳粠 walk 缂撳瓨涓竻鐞嗛〉琛?鎸囬拡銆傝鑼冨洿鍙互鍦ㄤ换鎰忛〉杈圭晫寮€濮?缁撴潫銆?
#### PT_FEAT_FLUSH_RANGE_NO_GAPS


PT_FEAT_FLUSH_RANGE_NO_GAPS 涓?PT_FEAT_FLUSH_RANGE 绫讳技锛涗絾鏄紝瀹冮€氳繃鍙戝嚭棰濆鐨?鍒锋柊鎿嶄綔鏉ユ渶灏忓寲鍙楀奖鍝嶇殑 VA 鏁伴噺銆傚鏋滃鐞?VA 鐨勪唬浠烽潪甯搁珮锛屼緥濡傚洜涓鸿櫄鎷熸満鐩戞帶鍣?姝ｅ湪浣跨敤褰卞瓙绠楁硶澶勭悊椤佃〃锛岄偅涔堣繖灏卞緢鏈夌敤銆?