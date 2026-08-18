
## IOMMUFD


:Author: Jason Gunthorpe
:Author: Kevin Tian

## 姒傝堪


IOMMUFD 鏄敤鎴锋€?API锛岀敤浜庢帶鍒剁郴缁熶腑鐨?IOMMU 瀛愮郴缁燂紝娑夊強浣跨敤鏂囦欢鎻忚堪绗︿粠鐢ㄦ埛绌洪棿绠＄悊
IO 椤佃〃銆傚畠鏃ㄥ湪閫氱敤涓斿彲琚换浣曞笇鏈涘悜鐢ㄦ埛绌洪棿鏆撮湶 DMA 鐨勯┍鍔ㄤ娇鐢ㄣ€傝繖浜涢┍鍔ㄦ渶缁堥鏈熶細寮冪敤
瀹冧滑鍙兘宸茬粡/鍘嗗彶涓婂疄鐜扮殑浠讳綍鍐呴儴 IOMMU 閫昏緫锛堜緥濡?vfio_iommu_type1.c锛夈€?
鑷冲皯 iommufd 涓烘墍鏈?IOMMU 鎻愪緵绠＄悊 I/O 鍦板潃绌洪棿鍜?I/O 椤佃〃鐨勯€氱敤鏀寔锛屽苟鍦ㄨ璁′腑鐣欐湁绌洪棿
浠ユ坊鍔犻潪閫氱敤鐗规€ф潵杩庡悎鐗瑰畾纭欢鍔熻兘銆?
鍦ㄦ涓婁笅鏂囦腑锛屽ぇ鍐欏瓧姣嶏紙IOMMUFD锛夋寚瀛愮郴缁燂紝鑰屽皬鍐欏瓧姣嶏紙iommufd锛夋寚閫氳繃 /dev/iommu 鍒涘缓
渚涚敤鎴风┖闂翠娇鐢ㄧ殑鏂囦欢鎻忚堪绗︺€?
## 鍏抽敭姒傚康


### 鐢ㄦ埛鍙瀵硅薄


浠ヤ笅 IOMMUFD 瀵硅薄鏆撮湶缁欑敤鎴风┖闂达細

- IOMMUFD_OBJ_IOAS锛屼唬琛ㄤ竴涓?I/O 鍦板潃绌洪棿锛圛OAS锛夛紝鍏佽灏嗙敤鎴风┖闂村唴瀛樻槧灏?瑙ｉ櫎鏄犲皠鍒?  I/O 铏氭嫙鍦板潃锛圛OVA锛夌殑鑼冨洿銆?
  IOAS 鏄?VFIO 瀹瑰櫒鐨勫姛鑳芥€ф浛浠ｏ紝骞朵笖鍍?VFIO 瀹瑰櫒涓€鏍凤紝瀹冨皢涓€浠?IOVA 鏄犲皠澶嶅埗鍒板叾涓寘鍚殑
  iommu_domain 鍒楄〃涓€?
- IOMMUFD_OBJ_DEVICE锛屼唬琛ㄤ竴涓敱澶栭儴椹卞姩缁戝畾鍒?iommufd 鐨勮澶囥€?
- IOMMUFD_OBJ_HWPT_PAGING锛屼唬琛ㄤ竴涓敱 iommu 椹卞姩绠＄悊鐨勫疄闄呯‖浠?I/O 椤佃〃锛堝嵆鍗曚釜 struct
  iommu_domain锛夈€?PAGING" 涓昏琛ㄧず杩欑绫诲瀷鐨?HWPT 搴旇琚摼鎺ュ埌涓€涓?IOAS銆傚畠杩樿〃绀哄畠鐢?  甯︽湁 __IOMMU_DOMAIN_PAGING 鐗规€ф爣蹇楃殑 iommu_domain 鏀拺銆傝繖鍙互鏄湪鐢ㄦ埛绌洪棿杩愯鐨勮澶囩殑
  涓€涓?UNMANAGED stage-1 鍩燂紝鎴栬€呮槸浠庡鎴锋満绾х墿鐞嗗湴鍧€鍒颁富鏈虹骇鐗╃悊鍦板潃鏄犲皠鐨勫祵濂楃埗 stage-2
  鍩熴€?
  IOAS 鏈変竴涓叡浜浉鍚?IOVA 鏄犲皠鐨?HWPT_PAGING 鍒楄〃锛屽苟涓斿畠浼氬皢鍏舵槧灏勪笌姣忎釜鎴愬憳 HWPT_PAGING
  鍚屾銆?
- IOMMUFD_OBJ_HWPT_NESTED锛屼唬琛ㄤ竴涓敱鐢ㄦ埛绌洪棿锛堜緥濡傚鎴锋満 OS锛夌鐞嗙殑瀹為檯纭欢 I/O 椤佃〃
  锛堝嵆鍗曚釜 struct iommu_domain锛夈€?NESTED" 琛ㄧず杩欑绫诲瀷鐨?HWPT 搴旇琚摼鎺ュ埌涓€涓?HWPT_PAGING銆?  瀹冭繕琛ㄧず瀹冪敱绫诲瀷涓?IOMMU_DOMAIN_NESTED 鐨?iommu_domain 鏀拺銆傝繖蹇呴』鏄湪鐢ㄦ埛绌洪棿杩愯鐨勮澶囩殑
  stage-1 鍩燂紙渚嬪鍦ㄥ惎鐢?IOMMU 宓屽缈昏瘧鐗规€х殑瀹㈡埛鏈?VM 涓級銆傚洜姝わ紝瀹冨繀椤讳娇鐢ㄧ粰瀹氱殑宓屽鐖?  stage-2 鍩熸潵鍒涘缓浠ヨ繘琛屽叧鑱斻€傝繖涓敱鐢ㄦ埛绌洪棿绠＄悊鐨勫祵濂?stage-1 椤佃〃閫氬父鍏锋湁浠庡鎴锋満绾?I/O
  铏氭嫙鍦板潃鍒板鎴锋満绾х墿鐞嗗湴鍧€鐨勬槧灏勩€?
- IOMMUFD_FAULT锛屼唬琛ㄤ竴涓敤浜?HWPT 閫氳繃 IOMMU HW 鐨?PRI锛堥〉璇锋眰鎺ュ彛锛夋姤鍛?IO 椤甸敊璇殑杞欢
  闃熷垪銆傝繖涓槦鍒楀璞′负鐢ㄦ埛绌洪棿鎻愪緵涓€涓?FD 鏉ヨ疆璇㈤〉閿欒浜嬩欢骞跺搷搴旇繖浜涗簨浠躲€傚繀椤诲厛鍒涘缓涓€涓?  FAULT 瀵硅薄浠ヨ幏寰椾竴涓?fault_id锛岀劧鍚庡彲浠ラ€氳繃鍦?IOMMU_HWPT_ALLOC 鍛戒护鐨?flags 瀛楁涓缃?  IOMMU_HWPT_FAULT_ID_VALID 浣嶆潵鍒嗛厤涓€涓敮鎸佹晠闅滅殑 HWPT銆?
- IOMMUFD_OBJ_VIOMMU锛屼唬琛ㄧ墿鐞?IOMMU 瀹炰緥鐨勪竴涓垏鐗囷紝琚紶閫掔粰 VM 鎴栦笌 VM 鍏变韩銆傚畠鍙兘鏄竴浜?  HW 鍔犻€熺殑铏氭嫙鍖栫壒鎬т互鍙婁竴浜?VM 浣跨敤鐨?SW 璧勬簮銆備緥濡傦細

  - 瀹㈡埛鏈烘嫢鏈夌殑 ID 鐨勫畨鍏ㄥ懡鍚嶇┖闂达紝渚嬪瀹㈡埛鏈烘帶鍒剁殑缂撳瓨鏍囩
  - 闈炶澶囩浉鍏崇殑浜嬩欢鎶ュ憡锛屼緥濡傚け鏁堥槦鍒楅敊璇?  - 璺ㄧ墿鐞?IOMMU 璁块棶鍙叡浜殑宓屽鐖堕〉琛?  - 鍚勭骞冲彴 ID 鐨勮櫄鎷熷寲锛屼緥濡?RID 绛?  - 鍗婅櫄鎷熷寲澶辨晥鐨勬姇閫?  - 鐩存帴鍒嗛厤鐨勫け鏁堥槦鍒?  - 鐩存帴鍒嗛厤鐨勪腑鏂?
  杩欐牱鐨?vIOMMU 瀵硅薄閫氬父鏈夋潈璁块棶涓€涓祵濂楃埗椤佃〃锛屼互鏀寔涓€浜?HW 鍔犻€熺殑铏氭嫙鍖栫壒鎬с€傚洜姝わ紝蹇呴』
  缁欏畾涓€涓祵濂楃埗 HWPT_PAGING 瀵硅薄鏉ュ垱寤?vIOMMU 瀵硅薄锛岀劧鍚庡畠浼氬皝瑁呰 HWPT_PAGING 瀵硅薄銆傚洜姝わ紝
  vIOMMU 瀵硅薄鍙互鐢ㄦ潵鍒嗛厤涓€涓?HWPT_NESTED 瀵硅薄锛屼互鍙栦唬琚皝瑁呯殑 HWPT_PAGING銆?
```

     The name "vIOMMU" isn't necessarily identical to a virtualized IOMMU in a
     VM. A VM can have one giant virtualized IOMMU running on a machine having
     multiple physical IOMMUs, in which case the VMM will dispatch the requests
     or configurations from this single virtualized IOMMU instance to multiple
     vIOMMU objects created for individual slices of different physical IOMMUs.
     In other words, a vIOMMU object is always a representation of one physical
     IOMMU, not necessarily of a virtualized IOMMU. For VMMs that want the full
     virtualization features from physical IOMMUs, it is suggested to build the
     same number of virtualized IOMMUs as the number of physical IOMMUs, so the
     passed-through devices would be connected to their own virtualized IOMMUs
     backed by corresponding vIOMMU objects, in which case a guest OS would do
     the "dispatch" naturally instead of VMM trappings.

```
- IOMMUFD_OBJ_VDEVICE锛屼唬琛ㄤ竴涓?IOMMUFD_OBJ_DEVICE 閽堝涓€涓?IOMMUFD_OBJ_VIOMMU 鐨勮櫄鎷熻澶囥€?  杩欎釜铏氭嫙璁惧鎸佹湁璁惧鐨勮櫄鎷熶俊鎭垨灞炴€э紙涓?vIOMMU 鐩稿叧锛変簬涓€涓?VM 涓€備竴涓洿鎺ョ殑 vDATA 渚嬪瓙
  鍙互鏄澶囧湪 vIOMMU 涓婄殑铏氭嫙 ID锛岃繖鏄?VMM 涓鸿澶囧垎閰嶇粰 vIOMMU 鐨勭炕璇戦€氶亾/绔彛鐨勫敮涓€ ID锛?  渚嬪 ARM SMMUv3 鐨?vSID銆丄MD IOMMU 鐨?vDeviceID锛屼互鍙?Intel VT-d 鍒?Context Table 鐨?vRID銆?  涓€浜涢珮绾у畨鍏ㄤ俊鎭殑娼滃湪鐢ㄤ緥涔熷彲浠ラ€氳繃姝ゅ璞¤浆鍙戯紝渚嬪鏈哄瘑璁＄畻鏋舵瀯涓殑瀹夊叏绾у埆鎴?realm
  淇℃伅銆傚綋 VMM 灏嗚澶囪繛鎺ュ埌 vIOMMU 鏃讹紝瀹冨簲璇ュ垱寤轰竴涓?vDEVICE 瀵硅薄鏉ヨ浆鍙?VM 涓殑鎵€鏈夎澶囦俊鎭紝
  杩欐槸涓€涓崟鐙殑 ioctl 璋冪敤锛屼笉鍚屼簬灏嗗悓涓€璁惧闄勫姞鍒?vIOMMU 鎸佹湁鐨?HWPT_PAGING銆?
- IOMMUFD_OBJ_VEVENTQ锛屼唬琛ㄤ竴涓敤浜?vIOMMU 鎶ュ憡鍏朵簨浠剁殑杞欢闃熷垪锛屼緥濡傚彂鐢熷湪宓屽 stage-1 鐨?  缈昏瘧鏁呴殰锛堜笉鍖呮嫭搴旈€氳繃 IOMMUFD_OBJ_FAULT 鐨?I/O 椤甸敊璇級浠ュ強 HW 鐗瑰畾浜嬩欢銆傝繖涓槦鍒楀璞′负
  鐢ㄦ埛绌洪棿鎻愪緵涓€涓?FD 鏉ヨ疆璇?璇诲彇 vIOMMU 浜嬩欢銆傚繀椤诲厛鍒涘缓涓€涓?vIOMMU 瀵硅薄浠ヨ幏寰楀叾 viommu_id锛?  鐒跺悗鍙敤浜庡垎閰嶄竴涓?vEVENTQ銆傛瘡涓?vIOMMU 鍙互鏀寔澶氱绫诲瀷鐨?vEVENTS锛屼絾姣忕 vEVENTQ 绫诲瀷闄愬埗
  涓轰竴涓?vEVENTQ銆?
- IOMMUFD_OBJ_HW_QUEUE锛屼唬琛ㄤ竴涓‖浠跺姞閫熼槦鍒楋紝浣滀负 IOMMU 铏氭嫙鍖栫壒鎬х殑涓€閮ㄥ垎锛屼緵 IOMMU HW 鐩存帴
  璇诲彇鎴栧啓鍏ョ敱瀹㈡埛鏈?OS 鎷ユ湁鐨勮櫄鎷熼槦鍒楀唴瀛樸€傝繖涓?HW 鍔犻€熺壒鎬у彲浠ュ厑璁?VM 鐩存帴涓?IOMMU HW 鍗忎綔
  鑰屾棤闇€ VM 閫€鍑猴紝浠庤€屽噺灏戞潵鑷秴绾ц皟鐢ㄧ殑寮€閿€銆傝繛鍚?HW QUEUE 瀵硅薄锛宨ommufd 涓虹敤鎴风┖闂存彁渚涗竴涓?  mmap 鎺ュ彛锛屼緵 VMM 灏嗙墿鐞?MMIO 鍖哄煙浠庝富鏈虹墿鐞嗗湴鍧€绌洪棿鏄犲皠鍒板鎴锋満鐗╃悊鍦板潃绌洪棿锛屽厑璁稿鎴锋満 OS
  鐩存帴鎺у埗宸插垎閰嶇殑 HW QUEUE銆傚洜姝わ紝褰撳垎閰嶄竴涓?HW QUEUE 鏃讹紝VMM 蹇呴』璇锋眰涓€瀵?mmap 淇℃伅
  锛坥ffset/length锛夊苟绮剧‘鍦伴€氳繃 offset 鍜?length 鍙傛暟浼犵粰涓€涓?mmap 绯荤粺璋冪敤銆?
鎵€鏈夌敤鎴峰彲瑙佸璞￠兘閫氳繃 IOMMU_DESTROY uAPI 閿€姣併€?
涓嬮潰鐨勫浘琛ㄦ樉绀轰簡鐢ㄦ埛鍙瀵硅薄涓庡唴鏍告暟鎹粨鏋勶紙鍦?iommufd 澶栭儴锛変箣闂寸殑鍏崇郴锛屾暟瀛楁寚浠ｆ搷浣?```

  _______________________________________________________________________
 |                      iommufd (HWPT_PAGING only)                       |
 |                                                                       |
 |        [1]                  [3]                                [2]    |
 |  ________________      _____________                        ________  |
 | |                |    |             |                      |        | |
 | |      IOAS      |<---| HWPT_PAGING |<---------------------| DEVICE | |
 | |________________|    |_____________|                      |________| |
 |         |                    |                                  |     |
 |_________|____________________|__________________________________|_____|
           |                    |                                  |
           |              ______v_____                          ___v__
           | PFN storage |  (paging)  |                        |struct|
           |------------>|iommu_domain|<-----------------------|device|
                         |____________|                        |______|

  _______________________________________________________________________
 |                      iommufd (with HWPT_NESTED)                       |
 |                                                                       |
 |        [1]                  [3]                [4]             [2]    |
 |  ________________      _____________      _____________     ________  |
 | |                |    |             |    |             |   |        | |
 | |      IOAS      |<---| HWPT_PAGING |<---| HWPT_NESTED |<--| DEVICE | |
 | |________________|    |_____________|    |_____________|   |________| |
 |         |                    |                  |               |     |
 |_________|____________________|__________________|_______________|_____|
           |                    |                  |               |
           |              ______v_____       ______v_____       ___v__
           | PFN storage |  (paging)  |     |  (nested)  |     |struct|
           |------------>|iommu_domain|<----|iommu_domain|<----|device|
                         |____________|     |____________|     |______|

  _______________________________________________________________________
 |                      iommufd (with vIOMMU/vDEVICE)                    |
 |                                                                       |
 |                             [5]                [6]                    |
 |                        _____________      _____________               |
 |                       |             |    |             |              |
 |      |----------------|    vIOMMU   |<---|   vDEVICE   |<----|        |
 |      |                |             |    |_____________|     |        |
 |      |                |             |                        |        |
 |      |      [1]       |             |          [4]           | [2]    |
 |      |     ______     |             |     _____________     _|______  |
 |      |    |      |    |     [3]     |    |             |   |        | |
 |      |    | IOAS |<---|(HWPT_PAGING)|<---| HWPT_NESTED |<--| DEVICE | |
 |      |    |______|    |_____________|    |_____________|   |________| |
 |      |        |              |                  |               |     |
 |______|________|______________|__________________|_______________|_____|
        |        |              |                  |               |
  ______v_____   |        ______v_____       ______v_____       ___v__
 |   struct   |  |  PFN  |  (paging)  |     |  (nested)  |     |struct|
 |iommu_device|  |------>|iommu_domain|<----|iommu_domain|<----|device|
 |____________|   storage|____________|     |____________|     |______|

```
1. IOMMUFD_OBJ_IOAS 閫氳繃 IOMMU_IOAS_ALLOC uAPI 鍒涘缓銆備竴涓?iommufd 鍙互鎸佹湁澶氫釜 IOAS 瀵硅薄銆?   IOAS 鏄渶閫氱敤鐨勫璞★紝涓嶆毚闇茬壒瀹氫簬鍗曚釜 IOMMU 椹卞姩鐨勬帴鍙ｃ€傚 IOAS 鐨勬墍鏈夋搷浣滃繀椤诲湪鍏跺唴閮ㄧ殑
   姣忎釜 iommu_domain 涓婂钩绛夊湴杩涜銆?
2. IOMMUFD_OBJ_DEVICE 鍦ㄥ閮ㄩ┍鍔ㄨ皟鐢?IOMMUFD kAPI 灏嗚澶囩粦瀹氬埌 iommufd 鏃跺垱寤恒€傝椹卞姩闇€瑕佸疄鐜?   涓€缁?ioctl 浠ュ厑璁哥敤鎴风┖闂村彂璧风粦瀹氭搷浣溿€傛鎿嶄綔鐨勬垚鍔熷畬鎴愬缓绔嬩簡瀵硅璁惧鐨勬湡鏈?DMA 鎵€鏈夋潈銆?   璇ラ┍鍔ㄨ繕蹇呴』璁剧疆 driver_managed_dma 鏍囧織锛屽苟涓斿湪鎿嶄綔鎴愬姛涔嬪墠涓嶅緱瑙︾璇ヨ澶囥€?
3. IOMMUFD_OBJ_HWPT_PAGING 鍙互閫氳繃涓ょ鏂瑰紡鍒涘缓锛?
   - IOMMUFD_OBJ_HWPT_PAGING 鍦ㄥ閮ㄩ┍鍔ㄨ皟鐢?IOMMUFD kAPI 灏嗙粦瀹氱殑璁惧闄勫姞鍒?IOAS 鏃惰嚜鍔ㄥ垱寤恒€?     绫讳技鍦帮紝澶栭儴椹卞姩 uAPI 鍏佽鐢ㄦ埛绌洪棿鍙戣捣闄勫姞鎿嶄綔銆傚鏋?IOAS 鐨?HWPT_PAGING 鍒楄〃涓瓨鍦ㄤ竴涓?     鍏煎鐨勬垚鍛?HWPT_PAGING 瀵硅薄锛屽垯瀹冧細琚噸鐢ㄣ€傚惁鍒欏皢鍒涘缓涓€涓唬琛ㄩ潰鍚戠敤鎴风┖闂寸殑
     iommu_domain 鐨勬柊 HWPT_PAGING锛岀劧鍚庢坊鍔犲埌鍒楄〃涓€傛鎿嶄綔鐨勬垚鍔熷畬鎴愬缓绔嬩簡 IOAS銆佽澶囧拰
     iommu_domain 涔嬮棿鐨勯摼鎺ャ€備竴鏃﹀畬鎴愶紝璁惧灏卞彲浠ヨ繘琛?DMA銆?
   - IOMMUFD_OBJ_HWPT_PAGING 鍙互閫氳繃 IOMMU_HWPT_ALLOC uAPI 鎵嬪姩鍒涘缓锛岄€氳繃 @pt_id 鎻愪緵 ioas_id
     浠ュ皢鏂扮殑 HWPT_PAGING 鍏宠仈鍒扮浉搴旂殑 IOAS 瀵硅薄銆傝繖绉嶆墜鍔ㄥ垎閰嶇殑濂藉鏄厑璁稿垎閰嶆爣蹇楋紙瀹氫箟浜?     enum iommufd_hwpt_alloc_flags锛夛紝渚嬪锛屽鏋滆缃簡 IOMMU_HWPT_ALLOC_NEST_PARENT 鏍囧織锛屽畠浼?     鍒嗛厤涓€涓祵濂楃埗 HWPT_PAGING銆?
4. IOMMUFD_OBJ_HWPT_NESTED 鍙兘鎵嬪姩閫氳繃 IOMMU_HWPT_ALLOC uAPI 鍒涘缓锛岄€氳繃 @pt_id 鎻愪緵 hwpt_id 鎴?   灏佽浜嗗祵濂楃埗 HWPT_PAGING 鐨?vIOMMU 瀵硅薄鐨?viommu_id锛屼互灏嗘柊鐨?HWPT_NESTED 瀵硅薄鍏宠仈鍒扮浉搴旂殑
   HWPT_PAGING 瀵硅薄銆傚叧鑱旂殑 HWPT_PAGING 瀵硅薄蹇呴』鏄厛鍓嶉€氳繃鍚屼竴 uAPI 璁剧疆浜?   IOMMU_HWPT_ALLOC_NEST_PARENT 鏍囧織鎵嬪姩鍒嗛厤鐨勫祵濂楃埗瀵硅薄锛屽惁鍒欏垎閰嶅皢澶辫触銆傝鍒嗛厤灏嗚繘涓€姝ョ敱
   IOMMU 椹卞姩楠岃瘉锛屼互纭繚琚垎閰嶇殑宓屽鐖跺煙鍜屽祵濂楀煙鏄吋瀹圭殑銆傛鎿嶄綔鐨勬垚鍔熷畬鎴愬缓绔嬩簡 IOAS銆佽澶?   鍜?iommu_domain 涔嬮棿鐨勯摼鎺ャ€備竴鏃﹀畬鎴愶紝璁惧灏卞彲浠ラ€氳繃 2 绾х炕璇戯紙鍗冲祵濂楃炕璇戯級杩涜 DMA銆傛敞鎰忥紝
   澶氫釜 HWPT_NESTED 瀵硅薄鍙互鐢憋紙骞堕殢鍚庡叧鑱斿埌锛夊悓涓€涓祵濂楃埗瀵硅薄鍒嗛厤銆?
```

      Either a manual IOMMUFD_OBJ_HWPT_PAGING or an IOMMUFD_OBJ_HWPT_NESTED is
      created via the same IOMMU_HWPT_ALLOC uAPI. The difference is at the type
      of the object passed in via the @pt_id field of struct iommufd_hwpt_alloc.

```
5. IOMMUFD_OBJ_VIOMMU 鍙兘鎵嬪姩閫氳繃 IOMMU_VIOMMU_ALLOC uAPI 鍒涘缓锛屾彁渚涗竴涓?dev_id锛堢敤浜庤澶囩殑
   鐗╃悊 IOMMU 鏉ユ敮鎾戣 vIOMMU锛夊拰涓€涓?hwpt_id锛堝皢 vIOMMU 鍏宠仈鍒颁竴涓祵濂楃埗 HWPT_PAGING锛夈€?   iommufd 鏍稿績浼氬皢 vIOMMU 瀵硅薄閾炬帴鍒拌 struct device 鑳屽悗鐨?struct iommu_device銆傚苟涓?IOMMU
   椹卞姩鍙互瀹炵幇 viommu_alloc op 鏉ュ垎閰嶅畠鑷繁鐨?vIOMMU 鏁版嵁缁撴瀯锛屽唴宓屾牳蹇冪骇缁撴瀯 iommufd_viommu
   鍜屼竴浜涢┍鍔ㄧ壒瀹氭暟鎹€傚鏈夊繀瑕侊紝椹卞姩杩樺彲浠ヤ负璇?vIOMMU锛堝苟鍥犳涓?VM锛夐厤缃叾 HW 铏氭嫙鍖栫壒鎬с€傛
   鎿嶄綔鐨勬垚鍔熷畬鎴愬缓绔嬩簡 vIOMMU 瀵硅薄鍜?HWPT_PAGING 涔嬮棿鐨勯摼鎺ワ紝鐒跺悗璇?vIOMMU 瀵硅薄鍙敤浣滃祵濂楃埗
   瀵硅薄鏉ュ垎閰嶄笂闈㈡弿杩扮殑 HWPT_NESTED 瀵硅薄銆?
6. IOMMUFD_OBJ_VDEVICE 鍙兘鎵嬪姩閫氳繃 IOMMU_VDEVICE_ALLOC uAPI 鍒涘缓锛屾彁渚涗竴涓?iommufd_viommu 瀵硅薄鐨?   viommu_id 鍜屼竴涓?iommufd_device 瀵硅薄鐨?dev_id銆倂DEVICE 瀵硅薄灏嗘槸杩欎袱涓埗瀵硅薄涔嬮棿鐨勭粦瀹氥€傚彟涓€涓?   @virt_id 涔熷皢閫氳繃 uAPI 璁剧疆锛屼负 iommufd 鏍稿績鎻愪緵涓€涓储寮曪紝浠ュ皢 vDEVICE 瀵硅薄瀛樺偍鍒版瘡涓?vIOMMU
   鐨?vDEVICE 鏁扮粍涓€傚鏈夊繀瑕侊紝IOMMU 椹卞姩鍙互閫夋嫨瀹炵幇 vdevice_alloc op 鏉ュ垵濮嬪寲鍏?HW 浠ョ敤浜庝笌
   vDEVICE 鐩稿叧鐨勮櫄鎷熷寲鐗规€с€傛鎿嶄綔鐨勬垚鍔熷畬鎴愬缓绔嬩簡 vIOMMU 鍜岃澶囦箣闂寸殑閾炬帴銆?
涓€涓澶囧彧鑳界粦瀹氬埌涓€涓?iommufd锛岃繖鏄敱浜?DMA 鎵€鏈夋潈澹版槑锛屽苟涓旀渶澶氶檮鍔犲埌涓€涓?IOAS 瀵硅薄锛堝皻涓?鏀寔 PASID锛夈€?
### 鍐呮牳鏁版嵁缁撴瀯


鐢ㄦ埛鍙瀵硅薄鐢变互涓嬫暟鎹粨鏋勬敮鎾戯細

- iommufd_ioas 瀵瑰簲 IOMMUFD_OBJ_IOAS銆?- iommufd_device 瀵瑰簲 IOMMUFD_OBJ_DEVICE銆?- iommufd_hwpt_paging 瀵瑰簲 IOMMUFD_OBJ_HWPT_PAGING銆?- iommufd_hwpt_nested 瀵瑰簲 IOMMUFD_OBJ_HWPT_NESTED銆?- iommufd_fault 瀵瑰簲 IOMMUFD_OBJ_FAULT銆?- iommufd_viommu 瀵瑰簲 IOMMUFD_OBJ_VIOMMU銆?- iommufd_vdevice 瀵瑰簲 IOMMUFD_OBJ_VDEVICE銆?- iommufd_veventq 瀵瑰簲 IOMMUFD_OBJ_VEVENTQ銆?- iommufd_hw_queue 瀵瑰簲 IOMMUFD_OBJ_HW_QUEUE銆?
鐪嬭繖浜涙暟鎹粨鏋勬椂鐨勪竴浜涙湳璇細

- 鑷姩鍩?- 鎸囧湪灏嗚澶囬檮鍔犲埌 IOAS 瀵硅薄鏃惰嚜鍔ㄥ垱寤虹殑 iommu 鍩熴€傝繖涓?VFIO type1 鐨勮涔夊吋瀹广€?
- 鎵嬪姩鍩?- 鎸囩敤鎴锋寚瀹氱殑銆佷綔涓鸿澶囪闄勫姞鐨勭洰鏍囬〉琛ㄧ殑 iommu 鍩熴€傝櫧鐒剁洰鍓嶆病鏈?uAPI 鐩存帴鍒涘缓杩欐牱鐨?  鍩燂紝浣嗘暟鎹粨鏋勫拰绠楁硶宸插噯澶囧ソ澶勭悊璇ョ敤渚嬨€?
- 鍐呮牳鍐呯敤鎴?- 鎸囧儚 VFIO mdev 杩欐牱浣跨敤 IOMMUFD access 鎺ュ彛鏉ヨ闂?IOAS 鐨勪笢瑗裤€傝繖棣栧厛鍒涘缓涓€涓?  iommufd_access 瀵硅薄锛岀被浼间簬鐗╃悊璁惧缁戝畾鍩熸墍鍋氱殑閭ｆ牱銆傜劧鍚?access 瀵硅薄灏嗗厑璁稿皢 IOVA 鑼冨洿
  杞崲涓?struct page * 鍒楄〃锛屾垨瀵?IOVA 杩涜鐩存帴璇?鍐欍€?
iommufd_ioas 浣滀负鍏冩暟鎹暟鎹粨鏋勶紝鐢ㄤ簬绠＄悊 IOVA 鑼冨洿濡備綍鏄犲皠鍒板唴瀛橀〉锛岀敱浠ヤ笅缁勬垚锛?
- struct io_pagetable 鎸佹湁 IOVA 鏄犲皠
- struct iopt_area 浠ｈ〃 IOVA 宸插～鍏呯殑閮ㄥ垎
- struct iopt_pages 浠ｈ〃 PFN 鐨勫瓨鍌?- struct iommu_domain 浠ｈ〃 IOMMU 涓殑 IO 椤佃〃
- struct iopt_pages_access 浠ｈ〃 PFN 鐨勫唴鏍稿唴鐢ㄦ埛
- struct xarray pinned_pfns 鎸佹湁鐢卞唴鏍稿唴鐢ㄦ埛鍥哄畾鐨勯〉鍒楄〃

姣忎釜 iopt_pages 浠ｈ〃涓€涓畬鏁寸殑 PFN 鐨勯€昏緫绾挎€ф暟缁勩€侾FN 鏈€缁堥€氳繃 mm_struct 浠庣敤鎴风┖闂?VA 娲剧敓銆?涓€鏃﹀畠浠鍥哄畾锛孭FN 灏辫瀛樺偍鍦?iommu_domain 鐨?IOPTE 涓紝鎴栬€呭鏋滃畠浠槸琚?iommufd_access 鍥哄畾
鐨勶紝鍒欏瓨鍌ㄥ湪 pinned_pfns xarray 涓€?
PFN 蹇呴』鍦ㄥ瓨鍌ㄤ綅缃殑鎵€鏈夌粍鍚堜箣闂村鍒讹紝杩欏彇鍐充簬瀛樺湪鍝簺鍩熶互鍙婂瓨鍦ㄥ摢浜涚被鍨嬬殑鍐呮牳鍐?杞欢璁块棶"
鐢ㄦ埛銆傝鏈哄埗纭繚涓€涓〉鍙鍥哄畾涓€娆°€?
涓€涓?io_pagetable 鐢辨寚鍚?iopt_pages 鐨?iopt_area 浠ュ強闀滃儚 IOVA 鍒?PFN 鏄犲皠鐨?iommu_domain 鍒楄〃
缁勬垚銆?
澶氫釜 io_pagetable锛堥€氳繃鍏?iopt_area锛夊彲浠ュ叡浜竴涓崟涓€鐨?iopt_pages锛岃繖閬垮厤浜嗗閲嶅浐瀹氬拰椤垫秷鑰楃殑
閲嶅璁拌处銆?
鍙鐢变笉鍚屽瓙绯荤粺绠＄悊鐨勮澶囩粦瀹氬埌鍚屼竴涓?iommufd锛宨ommufd_ioas 灏卞彲浠ュ湪瀛愮郴缁熶箣闂村叡浜紝渚嬪
VFIO 鍜?VDPA銆?
## IOMMUFD 鐢ㄦ埛 API



## IOMMUFD 鍐呮牳 API


IOMMUFD kAPI 鏄互璁惧涓轰腑蹇冪殑锛屼笌缁勭浉鍏崇殑鎶€宸у湪骞曞悗绠＄悊銆傝繖浣垮緱璋冪敤姝ょ被 kAPI 鐨勫閮ㄩ┍鍔ㄨ兘澶?瀹炵幇涓€涓畝鍗曠殑浠ヨ澶囦负涓績鐨?uAPI锛岀敤浜庡皢瀹冪殑璁惧杩炴帴鍒?iommufd锛岃€屼笉鏄儚 VFIO 閭ｆ牱鍦ㄥ叾 uAPI
涓樉寮忓己鍔犵粍璇箟銆?
   :export:

   :export:

### VFIO 鍜?IOMMUFD


灏?VFIO 璁惧杩炴帴鍒?iommufd 鍙互閫氳繃涓ょ鏂瑰紡瀹屾垚銆?
绗竴绉嶆槸 VFIO 鍏煎鐨勬柟寮忥紝閫氳繃灏嗚繖浜?IOCTL 鏄犲皠鍒?io_pagetable 鎿嶄綔鏉ョ洿鎺ュ疄鐜?/dev/vfio/vfio 瀹瑰櫒 IOCTL銆傝繖鏍峰仛鍏佽閫氳繃鍦?/dev/vfio/vfio 鍒?/dev/iommufd 涔嬮棿寤虹珛绗﹀彿閾炬帴锛?鎴栨墿灞?VFIO 浣跨敤 iommufd 鑰岄潪瀹瑰櫒 fd 鏉?SET_CONTAINER锛屼粠鑰屽湪閬楃暀 VFIO 搴旂敤绋嬪簭涓娇鐢?iommufd銆?
绗簩绉嶆柟寮忕洿鎺ユ墿灞?VFIO 浠ユ敮鎸佷竴缁勬柊鐨勫熀浜庝笂杩?IOMMUFD 鍐呮牳 API 鐨勪互璁惧涓轰腑蹇冪殑鐢ㄦ埛 API銆傚畠
闇€瑕佺敤鎴风┖闂存洿鏀癸紝浣嗕笌 IOMMUFD API 璇箟鏇村尮閰嶏紝骞朵笖涓庣涓€绉嶆柟寮忕浉姣旀洿瀹规槗鏀寔鏂扮殑 iommufd
鐗规€с€?
鐩墠涓ょ鏂瑰紡浠嶅湪杩涜涓€?
涓?VFIO type1 鐩告瘮浠嶆湁涓€浜涘樊璺濋渶瑕佽В鍐筹紝濡?iommufd_vfio_check_extension() 涓墍杩般€?
## 鏈潵鐨?TODO


鐩墠 IOMMUFD 浠呮敮鎸佸唴鏍哥鐞嗙殑 I/O 椤佃〃锛岀被浼间簬 VFIO type1銆傞浄杈句笂鐨勬柊鐗规€у寘鎷細

 - 灏?iommu_domain 缁戝畾鍒?PASID/SSID
 - 鐢ㄦ埛绌洪棿椤佃〃锛岄拡瀵?ARM銆亁86 鍜?S390
 - 鍐呮牳鏃佽矾鐨勭敤鎴烽〉琛ㄥけ鏁? - 鍦?IOMMU 涓鐢?KVM 椤佃〃
 - IOMMU 涓殑鑴忛〉璺熻釜
 - IOPTE 澶у皬鐨勮繍琛屾椂澧炲姞/鍑忓皯
 - 鍦ㄧ敤鎴风┖闂磋В鍐虫晠闅滅殑 PRI 鏀寔
