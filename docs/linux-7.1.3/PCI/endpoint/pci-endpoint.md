
:浣滆€? Kishon Vijay Abraham I <kishon@ti.com>

鏈枃妗ｆ槸鍏充簬濡備綍浣跨敤 PCI Endpoint Framework锛圥CI 绔偣妗嗘灦锛夌殑鎸囧崡锛屼互鍒涘缓绔偣鎺у埗鍣?
椹卞姩銆佺鐐瑰姛鑳介┍鍔紝骞朵娇鐢?configfs 鎺ュ彛灏嗗姛鑳介┍鍔ㄧ粦瀹氬埌鎺у埗鍣ㄩ┍鍔ㄣ€?

## Introduction


Linux 鏈変竴涓叏闈㈢殑 PCI 瀛愮郴缁燂紝鐢ㄤ簬鏀寔浠?Root Complex锛堟牴澶嶅悎浣擄級妯″紡杩愯鐨?PCI 鎺у埗鍣ㄣ€?
璇ュ瓙绯荤粺鑳藉鎵弿 PCI 鎬荤嚎銆佸垎閰嶅唴瀛樿祫婧愬拰 IRQ 璧勬簮銆佸姞杞?PCI 椹卞姩锛堝熀浜庡巶鍟?ID銆佽澶?
ID锛夛紝骞舵敮鎸佺儹鎻掓嫈銆佺數婧愮鐞嗐€侀珮绾ч敊璇姤鍛婂拰铏氭嫙閫氶亾绛夊叾浠栨湇鍔°€?

鐒惰€岋紝闆嗘垚鍦ㄦ煇浜?SoC 涓殑 PCI 鎺у埗鍣?IP 鏃㈣兘鍦?Root Complex 妯″紡涔熻兘鍦?Endpoint锛堢鐐癸級
妯″紡涓嬭繍琛屻€侾CI Endpoint Framework 灏嗕负 Linux 娣诲姞绔偣妯″紡鏀寔銆傝繖灏嗘湁鍔╀簬鍦?EP 绯荤粺涓?
杩愯 Linux锛屽叾鍙敤浜庝粠娴嬭瘯鎴栭獙璇併€佸崗澶勭悊鍣ㄥ姞閫熷櫒绛夊悇绉嶅箍娉涚殑浣跨敤鍦烘櫙銆?

## PCI Endpoint Core


PCI Endpoint Core锛堢鐐规牳蹇冿級灞傜敱 3 涓儴鍒嗙粍鎴愶細Endpoint Controller 搴撱€丒ndpoint
Function 搴擄紝浠ュ強灏嗙鐐瑰姛鑳戒笌绔偣鎺у埗鍣ㄧ粦瀹氱殑 configfs 灞傘€?

### PCI Endpoint Controller(EPC) Library


EPC 搴撴彁渚涗簡鍙湪绔偣妯″紡涓嬭繍琛岀殑鎺у埗鍣ㄤ娇鐢ㄧ殑 API銆傚畠杩樻彁渚涗簡渚涘姛鑳介┍鍔?搴撳疄鐜扮壒瀹氱鐐?
鍔熻兘鏃朵娇鐢ㄧ殑 API銆?

#### APIs for the PCI controller Driver


鏈妭鍒楀嚭 PCI Endpoint core 鎻愪緵缁?PCI 鎺у埗鍣ㄩ┍鍔ㄤ娇鐢ㄧ殑 API銆?

- devm_pci_epc_create()/pci_epc_create()

   PCI 鎺у埗鍣ㄩ┍鍔ㄥ簲瀹炵幇浠ヤ笅 ops锛?

  - write_header锛氬～鍏呴厤缃┖闂村ご閮ㄧ殑 ops
  - set_bar锛氶厤缃?BAR 鐨?ops
  - clear_bar锛氬浣?BAR 鐨?ops
  - alloc_addr_space锛氬湪 PCI 鎺у埗鍣ㄥ湴鍧€绌洪棿涓垎閰嶇殑 ops
  - free_addr_space锛氶噴鏀惧凡鍒嗛厤鍦板潃绌洪棿鐨?ops
  - raise_irq锛氳Е鍙?legacy銆丮SI 鎴?MSI-X 涓柇鐨?ops
  - start锛氬惎鍔?PCI 閾捐矾鐨?ops
  - stop锛氬仠姝?PCI 閾捐矾鐨?ops

   PCI 鎺у埗鍣ㄩ┍鍔ㄩ殢鍚庡彲浠ラ€氳繃璋冪敤 devm_pci_epc_create()/pci_epc_create() 鍒涘缓涓€涓柊鐨?
   EPC 璁惧銆?

- pci_epc_destroy()

   PCI 鎺у埗鍣ㄩ┍鍔ㄥ彲浠ヤ娇鐢?pci_epc_destroy() 閿€姣佺敱 pci_epc_create() 鍒涘缓鐨?EPC 璁惧銆?

- pci_epc_linkup()

   涓轰簡閫氱煡鎵€鏈夊姛鑳借澶囷紝瀹冧滑鎵€閾炬帴鐨?EPC 璁惧宸蹭笌涓绘満寤虹珛閾捐矾锛孭CI 鎺у埗鍣ㄩ┍鍔ㄥ簲璋冪敤
   pci_epc_linkup()銆?

- pci_epc_mem_init()

   鍒濆鍖栫敤浜庡垎閰?EPC 鍦板潃绌洪棿鐨?pci_epc_mem 缁撴瀯銆?

- pci_epc_mem_exit()

   娓呯悊鍦?pci_epc_mem_init() 鏈熼棿鍒嗛厤鐨?pci_epc_mem 缁撴瀯銆?


#### EPC APIs for the PCI Endpoint Function Driver


鏈妭鍒楀嚭 PCI Endpoint core 鎻愪緵缁?PCI 绔偣鍔熻兘椹卞姩浣跨敤鐨?API銆?

- pci_epc_write_header()

   PCI 绔偣鍔熻兘椹卞姩搴斾娇鐢?pci_epc_write_header() 灏嗘爣鍑嗛厤缃ご閮ㄥ啓鍏ョ鐐规帶鍒跺櫒銆?

- pci_epc_set_bar()

   PCI 绔偣鍔熻兘椹卞姩搴斾娇鐢?pci_epc_set_bar() 閰嶇疆鍩哄湴鍧€瀵勫瓨鍣紙Base Address Register锛夛紝
   浠ヤ究涓绘満鍒嗛厤 PCI 鍦板潃绌洪棿銆傚姛鑳介┍鍔ㄧ殑瀵勫瓨鍣ㄧ┖闂撮€氬父浣跨敤姝?API 杩涜閰嶇疆銆?

   鏌愪簺绔偣鎺у埗鍣ㄤ篃鏀寔鍦ㄤ负涓绘満宸茬紪绋?BAR 鍩哄湴鍧€鍚庯紝鍐嶆涓哄悓涓€涓?BAR 璋冪敤
   pci_epc_set_bar()锛堟棤闇€璋冪敤 pci_epc_clear_bar()锛夋潵鏇存柊鍏ョ珯鍦板潃杞崲銆傜鐐瑰姛鑳介┍鍔ㄥ彲浠?
   閫氳繃 dynamic_inbound_mapping EPC 鐗规€т綅妫€鏌ユ鑳藉姏銆?

   褰?pci_epf_bar.num_submap 闈為浂鏃讹紝绔偣鍔熻兘椹卞姩姝ｅ湪浣跨敤 pci_epf_bar.submap 璇锋眰 BAR 瀛?
   鑼冨洿鏄犲皠銆傝繖瑕佹眰 EPC 閫氳繃 subrange_mapping EPC 鐗规€т綅澹版槑鏀寔銆?

   褰?EPF 椹卞姩鎯宠浣跨敤鍏ョ珯瀛愯寖鍥存槧灏勭壒鎬ф椂锛屽畠瑕佹眰 BAR 鍩哄湴鍧€宸茬敱涓绘満鍦ㄦ灇涓炬湡闂寸紪绋嬨€傚洜姝わ紝
   瀹冮渶瑕佸鍚屼竴涓?BAR 璋冪敤涓ゆ pci_epc_set_bar()锛堥渶瑕?dynamic_inbound_mapping锛夛細绗竴娆″皢
   num_submap 璁句负闆跺苟閰嶇疆 BAR 澶у皬锛岀劧鍚庡湪 PCIe 閾捐矾寤虹珛涓斾富鏈烘灇涓剧鐐瑰苟缂栫▼ BAR 鍩哄湴鍧€鍚庯紝
   鍐嶆灏?num_submap 璁句负闈為浂鍊笺€?

   娉ㄦ剰锛屽湪浣跨敤鍏ョ珯瀛愯寖鍥存槧灏勭壒鎬ф椂锛孍PF 椹卞姩涓嶅緱鍦ㄤ袱娆?pci_epc_set_bar() 璋冪敤涔嬮棿璋冪敤
   pci_epc_clear_bar()锛屽洜涓烘竻闄?BAR 鍙兘浼氭竻闄?绂佺敤绔偣涓婄殑 BAR 瀵勫瓨鍣ㄦ垨 BAR 瑙ｇ爜锛岃€屾鏃?
   涓绘満浠嶆湡鏈涘凡鍒嗛厤鐨?BAR 鍦板潃淇濇寔鏈夋晥銆?

- pci_epc_clear_bar()

   PCI 绔偣鍔熻兘椹卞姩搴斾娇鐢?pci_epc_clear_bar() 澶嶄綅 BAR銆?

- pci_epc_raise_irq()

   PCI 绔偣鍔熻兘椹卞姩搴斾娇鐢?pci_epc_raise_irq() 瑙﹀彂 Legacy 涓柇銆丮SI 鎴?MSI-X 涓柇銆?

- pci_epc_mem_alloc_addr()

   PCI 绔偣鍔熻兘椹卞姩搴斾娇鐢?pci_epc_mem_alloc_addr() 浠?EPC 鍦板潃绌洪棿鍒嗛厤鍐呭瓨鍦板潃锛岃鍦板潃鐢ㄤ簬
   璁块棶 RC 鐨勭紦鍐插尯銆?

- pci_epc_mem_free_addr()

   PCI 绔偣鍔熻兘椹卞姩搴斾娇鐢?pci_epc_mem_free_addr() 閲婃斁浣跨敤 pci_epc_mem_alloc_addr() 鍒嗛厤鐨?
   鍐呭瓨绌洪棿銆?

- pci_epc_map_addr()

   PCI 绔偣鍔熻兘椹卞姩搴斾娇鐢?pci_epc_map_addr() 灏嗛€氳繃 pci_epc_mem_alloc_addr() 鑾峰彇鐨勬湰鍦板唴瀛?
   CPU 鍦板潃鏄犲皠鍒?RC 鐨?PCI 鍦板潃銆?

- pci_epc_unmap_addr()

   PCI 绔偣鍔熻兘椹卞姩搴斾娇鐢?pci_epc_unmap_addr() 瑙ｉ櫎浣跨敤 pci_epc_map_addr() 鏄犲皠鍒?RC 鍦板潃鐨?
   鏈湴鍐呭瓨 CPU 鍦板潃鐨勬槧灏勩€?

- pci_epc_mem_map()

   PCI 绔偣鎺у埗鍣ㄥ彲鑳藉鍙槧灏勭殑 RC PCI 鍦板潃鏂藉姞绾︽潫銆傚嚱鏁?pci_epc_mem_map() 鍏佽绔偣鍔熻兘
   椹卞姩鍦ㄥ簲瀵规绫荤害鏉熸椂鍒嗛厤骞舵槧灏勬帶鍒跺櫒鍐呭瓨銆傝鍑芥暟灏嗙‘瀹氬繀椤婚€氳繃浣跨敤 pci_epc_mem_alloc_addr()
   鍒嗛厤鐨勫唴瀛樺ぇ灏忥紝浠ユ垚鍔熸槧灏勪竴涓?RC PCI 鍦板潃鑼冨洿銆傝鍑芥暟杩樺皢鎸囩ず瀹為檯鏄犲皠鐨?PCI 鍦板潃鑼冨洿澶у皬
   锛堝彲鑳藉皬浜庤姹傜殑澶у皬锛夛紝浠ュ強鐢ㄤ簬璁块棶宸叉槧灏?RC PCI 鍦板潃鑼冨洿鐨勫凡鍒嗛厤鍐呭瓨涓殑鍋忕Щ閲忋€?

- pci_epc_mem_unmap()

   PCI 绔偣鍔熻兘椹卞姩鍙互浣跨敤 pci_epc_mem_unmap() 瑙ｉ櫎骞堕噴鏀句娇鐢?pci_epc_mem_map() 鍒嗛厤鍜屾槧灏勭殑
   鎺у埗鍣ㄥ唴瀛樸€?


#### Other EPC APIs


EPC 搴撹繕鎻愪緵浜嗗叾浠?API銆傝繖浜涚敤浜庣粦瀹?EPF 璁惧涓?EPC 璁惧銆俻ci-ep-cfs.c 鍙綔涓轰娇鐢ㄨ繖浜?
API 鐨勫弬鑰冦€?

- pci_epc_get()

   鍩轰簬鎺у埗鍣ㄧ殑璁惧鍚嶈幏鍙栧 PCI 绔偣鎺у埗鍣ㄧ殑寮曠敤銆?

- pci_epc_put()

   閲婃斁浣跨敤 pci_epc_get() 鑾峰緱鐨勫 PCI 绔偣鎺у埗鍣ㄧ殑寮曠敤銆?

- pci_epc_add_epf()

   鍚?PCI 绔偣鎺у埗鍣ㄦ坊鍔?PCI 绔偣鍔熻兘銆傛牴鎹鑼冿紝涓€涓?PCIe 璁惧鏈€澶氬彲鏈?8 涓姛鑳姐€?

- pci_epc_remove_epf()

   浠?PCI 绔偣鎺у埗鍣ㄧЩ闄?PCI 绔偣鍔熻兘銆?

- pci_epc_start()

   PCI 绔偣鍔熻兘椹卞姩鍦ㄩ厤缃畬绔偣鍔熻兘骞跺笇鏈涘惎鍔?PCI 閾捐矾鏃讹紝搴旇皟鐢?pci_epc_start()銆?

- pci_epc_stop()

   PCI 绔偣鍔熻兘椹卞姩搴旇皟鐢?pci_epc_stop() 鍋滄 PCI 閾捐矾銆?


### PCI Endpoint Function(EPF) Library


EPF 搴撴彁渚涗簡渚涘姛鑳介┍鍔ㄥ拰 EPC 搴撲娇鐢ㄧ殑 API锛屼互鎻愪緵绔偣妯″紡鍔熻兘銆?

#### EPF APIs for the PCI Endpoint Function Driver


鏈妭鍒楀嚭 PCI Endpoint core 鎻愪緵缁?PCI 绔偣鍔熻兘椹卞姩浣跨敤鐨?API銆?

- pci_epf_register_driver()

   PCI Endpoint Function 椹卞姩搴斿疄鐜颁互涓?ops锛?
  - bind锛氬綋 EPC 璁惧宸茬粦瀹氬埌 EPF 璁惧鏃舵墽琛岀殑鎿嶄綔
  - unbind锛氬綋 EPC 璁惧涓?EPF 璁惧涔嬮棿鐨勭粦瀹氫涪澶辨椂鎵ц鐨勬搷浣?
  - add_cfs锛氬彲閫夌殑 ops锛岀敤浜庡垱寤哄姛鑳界壒瀹氱殑 configfs 灞炴€?

   PCI Function 椹卞姩闅忓悗鍙互浣跨敤 pci_epf_register_driver() 娉ㄥ唽 PCI EPF 椹卞姩銆?

- pci_epf_unregister_driver()

   PCI Function 椹卞姩鍙互浣跨敤 pci_epf_unregister_driver() 娉ㄩ攢 PCI EPF 椹卞姩銆?

- pci_epf_alloc_space()

   PCI Function 椹卞姩鍙互浣跨敤 pci_epf_alloc_space() 涓虹壒瀹?BAR 鍒嗛厤绌洪棿銆?

- pci_epf_free_space()

   PCI Function 椹卞姩鍙互閫氳繃璋冪敤 pci_epf_free_space() 閲婃斁宸插垎閰嶇殑绌洪棿锛堜娇鐢?
   pci_epf_alloc_space 鍒嗛厤鐨勶級銆?

#### APIs for the PCI Endpoint Controller Library


鏈妭鍒楀嚭 PCI Endpoint core 鎻愪緵缁?PCI 绔偣鎺у埗鍣ㄥ簱浣跨敤鐨?API銆?

- pci_epf_linkup()

   褰?EPC 璁惧宸蹭笌涓绘満寤虹珛杩炴帴鏃讹紝PCI 绔偣鎺у埗鍣ㄥ簱浼氳皟鐢?pci_epf_linkup()銆?

#### Other EPF APIs


EPF 搴撹繕鎻愪緵浜嗗叾浠?API銆傝繖浜涚敤浜庡湪 EPF 璁惧缁戝畾鍒?EPC 璁惧鏃堕€氱煡鍔熻兘椹卞姩銆俻ci-ep-cfs.c
鍙綔涓轰娇鐢ㄨ繖浜?API 鐨勫弬鑰冦€?

- pci_epf_create()

   閫氳繃浼犲叆 PCI EPF 璁惧鐨勫悕绉板垱寤轰竴涓柊鐨?PCI EPF 璁惧銆傝鍚嶇О灏嗙敤浜庡皢 EPF 璁惧缁戝畾鍒?
   EPF 椹卞姩銆?

- pci_epf_destroy()

   閿€姣佸凡鍒涘缓鐨?PCI EPF 璁惧銆?

- pci_epf_bind()

   褰?EPF 璁惧宸茬粦瀹氬埌 EPC 璁惧鏃讹紝搴旇皟鐢?pci_epf_bind()銆?

- pci_epf_unbind()

   褰?EPC 璁惧涓?EPF 璁惧涔嬮棿鐨勭粦瀹氫涪澶辨椂锛屽簲璋冪敤 pci_epf_unbind()銆?
