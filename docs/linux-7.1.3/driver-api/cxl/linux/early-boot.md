
## Linux Init (Early Boot)


Linux 鐨勯厤缃垎涓轰袱涓富瑕佹楠わ細Early-Boot锛堟棭鏈熷惎鍔級浠ュ強鍏朵綑閮ㄥ垎銆?
鍦ㄦ棭鏈熷惎鍔ㄦ湡闂达紝Linux 璁剧疆涓嶅彲鍙樿祫婧愶紙渚嬪 numa 鑺傜偣锛夛紝鑰屽悗缁殑鎿嶄綔鍖呮嫭椹卞姩鎺㈡祴鍜屽唴瀛樼儹鎻掓嫈绛夈€傚湪鏁翠釜杩囩▼涓紝Linux 鍙兘浼氳鍙?EFI 鍜?ACPI 淇℃伅锛屼互閰嶇疆璁惧鐨勯€昏緫琛ㄧず銆?
鍦?Linux 鏃╂湡鍚姩闃舵锛堝唴鏍镐腑甯︽湁 __init 淇グ绗︾殑鍑芥暟锛夛紝绯荤粺浼氳幏鍙栫敱 EFI/BIOS 鍒涘缓鐨勩€佽祫婧愶紙[ACPI tables <../platform/acpi>](ACPI tables <../platform/acpi>)锛夛紝骞跺皢瀹冧滑杞崲涓哄唴鏍稿彲浠ユ秷璐圭殑璧勬簮銆?

## BIOS, Build and Boot Options


鍦ㄥ唴鏍告瀯寤烘椂鏈?4 涓渶瑕侀鍏堣€冭檻鐨勫惎鍔ㄥ墠閫夐」锛屽畠浠喅瀹氫簡 Linux 鍦ㄦ棭鏈熷惎鍔ㄦ湡闂村浣曠鐞嗗唴瀛樸€?
- EFI_MEMORY_SP

  - BIOS/EFI 閫夐」锛屽喅瀹氬唴瀛樻槸 SystemRAM 杩樻槸 Specific Purpose銆係pecific Purpose 鍐呭瓨灏嗚鎺ㄨ繜浜ょ粰椹卞姩绠＄悊鈥斺€旇€屼笉浼氱珛鍗充綔涓虹郴缁?RAM 鏆撮湶銆?
- CONFIG_EFI_SOFT_RESERVE

  - Linux 鏋勫缓閰嶇疆閫夐」锛屽喅瀹氬唴鏍告槸鍚︽敮鎸?Specific Purpose 鍐呭瓨銆?
- CONFIG_MHP_DEFAULT_ONLINE_TYPE

  - Linux 鏋勫缓閰嶇疆锛屽喅瀹氳浆鎹负 dax 璁惧鐨?Specific Purpose 鍐呭瓨鏄惁浠ュ強濡備綍琚鐞嗭紙淇濈暀涓?DAX锛屾垨浣滀负 ZONE_NORMAL 鎴?ZONE_MOVABLE 涓殑 SystemRAM 涓婄嚎锛夈€?
- nosoftreserve

  - Linux 鍐呮牳鍚姩閫夐」锛屽喅瀹氭槸鍚︽敮鎸?Soft Reserve銆備笌 CONFIG_EFI_SOFT_RESERVE 绫讳技銆?
## Memory Map Creation


褰撳唴鏍歌В鏋?EFI 鍐呭瓨鏄犲皠鏃讹紝濡傛灉鏀寔骞舵娴嬪埌浜?`Specific Purpose` 鍐呭瓨锛屽畠浼氬皢璇ュ尯鍩熷崟鐙垝鍑轰负 `SOFT_RESERVED`銆?
濡傛灉 `EFI_MEMORY_SP=0`銆乣CONFIG_EFI_SOFT_RESERVE=n` 鎴?`nosoftreserve=y`锛孡inux 浼氬皢 CXL 璁惧鍐呭瓨鍖哄煙榛樿浣滀负 SystemRAM銆傝繖浼氭妸璇ュ唴瀛樻毚闇茬粰鍐呮牳椤靛垎閰嶅櫒涓殑 `ZONE_NORMAL`锛屼娇鍏跺彲鐢ㄤ簬澶у鏁板垎閰嶏紙鍖呮嫭 `struct page` 鍜岄〉琛級銆?
濡傛灉璁剧疆浜?`Specific Purpose` 涓斿彈鏀寔锛宍CONFIG_MHP_DEFAULT_ONLINE_TYPE_*` 鍐冲畾璇ュ唴瀛樻槸鍚﹂粯璁や笂绾匡紙`_OFFLINE` 鎴?`_ONLINE_*`锛夛紝浠ュ強濡傛灉涓婄嚎锛岄粯璁ゅ皢鍏朵笂绾垮埌鍝釜 zone锛坄_NORMAL` 鎴?`_MOVABLE`锛夈€?
濡傛灉鏀剧疆鍦?`ZONE_MOVABLE`锛岃鍐呭瓨灏嗕笉鍙敤浜庡ぇ澶氭暟鍐呮牳鍒嗛厤锛堜緥濡?`struct page` 鎴栭〉琛級銆傛牴鎹郴缁熺殑鍐呭瓨瀹归噺锛岃繖鍙兘浼氬鎬ц兘浜х敓鏄捐憲褰卞搷銆?

## NUMA Node Reservation


Linux 寮曠敤 :doc:`SRAT <../platform/acpi/srat>` 涓畾涔夌殑 proximity 鍩燂紙`PXM`锛夋潵鍦?`acpi_numa_init` 涓垱寤?NUMA 鑺傜偣銆傞€氬父锛宍PXM` 涓?NUMA 鑺傜偣 ID 涔嬮棿瀛樺湪 1:1 鐨勫叧绯汇€?
SRAT 鏄畾涔?Proximity 鍩熺殑鍞竴 ACPI 瀹氫箟鏂瑰紡銆侺inux 鏈€澶氶€夋嫨灏嗗畠浠笌 NUMA 鑺傜偣 1:1 鏄犲皠銆俒CEDT <../platform/acpi/cedt>](CEDT <../platform/acpi/cedt>) 澧炲姞浜嗗 SPA 鑼冨洿鐨勬弿杩帮紝Linux 鍙兘浼氬皢鍏舵槧灏勫埌涓€涓垨澶氫釜 NUMA 鑺傜偣銆?
濡傛灉 CFMWS 涓瓨鍦ㄤ絾 SRAT 涓病鏈夌殑 CXL 鑼冨洿锛屽垯浼氬垱寤轰竴涓吉 `PXM`锛堣嚜 v6.15 璧凤級銆傛湭鏉ワ紝鐢变簬 proximity 鍩熷叧鑱旂殑妯＄硦鎬э紝Linux 鍙兘浼氭嫆缁?SRAT 鏈弿杩扮殑 CFMWS銆?
闇€瑕佹敞鎰忕殑鏄紝NUMA 鑺傜偣鐨勫垱寤轰笉鑳藉湪杩愯鏃惰繘琛屻€傛墍鏈夊彲鑳界殑 NUMA 鑺傜偣閮藉湪 `__init` 鏃讹紙鏇村叿浣撳湴璇达紝鍦?`mm_init` 鏈熼棿锛夎璇嗗埆銆侰EDT 鍜?SRAT 蹇呴』鍖呭惈瓒冲鐨?`PXM` 鏁版嵁锛屼互渚?Linux 璇嗗埆 NUMA 鑺傜偣鍙婂叾鍏宠仈鐨勫唴瀛樺尯鍩熴€?
鐩稿叧浠ｇ爜浣嶄簬锛歚linux/drivers/acpi/numa/srat.c`銆?
鏇村淇℃伅璇峰弬闃?[Example Platform Configurations <../platform/example-configs>](Example Platform Configurations <../platform/example-configs>)銆?
## Memory Tiers Creation


鍐呭瓨鍒嗗眰锛坢emory tier锛夋槸鎸夋€ц兘鐗瑰緛鍒嗙粍鐨?NUMA 鑺傜偣闆嗗悎銆傚湪 `__init` 鏈熼棿锛孡inux 浼氫娇鐢ㄥ寘鍚爣璁颁负 `N_MEMORY` 鐨勬墍鏈夎妭鐐圭殑榛樿鍐呭瓨鍒嗗眰鏉ュ垵濮嬪寲绯荤粺銆?
榛樿鎯呭喌涓嬶紝`memory_tier_init` 鍦ㄥ惎鍔ㄦ椂瀵规墍鏈夊凡涓婄嚎鍐呭瓨鐨勮妭鐐硅皟鐢ㄣ€俙memory_tier_late_init` 鍦?late-init 鏈熼棿瀵归┍鍔ㄩ厤缃樁娈佃缃殑鑺傜偣璋冪敤銆?
鑺傜偣鍙湁鍦ㄦ嫢鏈?*鍦ㄧ嚎**鍐呭瓨鏃舵墠浼氳鏍囪涓?`N_MEMORY`銆?
```

  /sys/devices/virtual/memory_tiering/memory_tierN/nodelist
  0-1

```
濡傛灉鍒嗙粍鐨勮妭鐐瑰湪鎬ц兘涓婂瓨鍦ㄦ槑鏄惧樊寮傦紝璇锋鏌?CXL 鑺傜偣鐨?[HMAT <../platform/acpi/hmat>](HMAT <../platform/acpi/hmat>) 鍜?CDAT 淇℃伅銆傞櫎闈為€氳繃 `access_coordinates` 鍚?memory_tier 缁勪欢鎶ュ憡浜?HMAT/CDAT 淇℃伅锛屽惁鍒欐墍鏈夎妭鐐归粯璁ら兘灞炰簬 DRAM 鍒嗗眰銆?
鏇村鍐呭璇峰弬闃?:doc:`CXL access coordinates 鏂囨。 <../linux/access-coordinates>`銆?
## Contiguous Memory Allocation


杩炵画鍐呭瓨鍒嗛厤鍣紙CMA锛夎兘澶熷湪鏃╂湡鍚姩鏈熼棿鍦?NUMA 鑺傜偣涓婇鐣欒繛缁殑鐗╃悊鍐呭瓨鍖哄煙銆傜劧鑰岋紝CMA 鏃犳硶棰勭暀鍐呭瓨锛?
```

  void __init hugetlb_cma_reserve(void) {
    if (!node_online(nid))
      /* 涓嶅厑璁搁鐣?*/
  }

```
杩欐剰鍛崇潃锛屽鏋滅敤鎴锋墦绠楀皢 CXL 鍐呭瓨鐨勭鐞嗘帹杩熷埌椹卞姩锛屽垯 CMA 涓嶈兘鐢ㄤ簬淇濊瘉澶ч〉鍒嗛厤銆傚鏋滃湪鏃╂湡鍚姩鏈熼棿灏?CXL 鍐呭瓨浣滀负 `ZONE_NORMAL` 涓殑 SystemRAM 鍚敤锛屽垯鍙互浣跨敤 `cma_pernuma` 鎴?`numa_cma` 鍐呮牳鍛戒护琛屽弬鏁颁负姣忎釜鑺傜偣杩涜 CMA 棰勭暀銆?