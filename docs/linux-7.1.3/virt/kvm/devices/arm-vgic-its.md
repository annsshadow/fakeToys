
## ARM 铏氭嫙涓柇杞崲鏈嶅姟锛圛TS锛?

鏀寔鐨勮澶囩被鍨嬶細
  KVM_DEV_TYPE_ARM_VGIC_ITS    ARM 涓柇杞崲鏈嶅姟鎺у埗鍣?
ITS 鍏佽灏?MSI(-X) 涓柇娉ㄥ叆鍒板鎴锋満涓€傝鎵╁睍鏄彲閫夌殑銆傚垱寤轰竴涓櫄鎷?ITS 鎺у埗鍣?杩橀渶瑕佷竴涓富鏈?GICv3锛堝弬瑙?arm-vgic-v3.txt锛夛紝浣嗕笉渚濊禆浜庢槸鍚︽湁鐗╃悊 ITS 鎺у埗鍣ㄣ€?
姣忎釜瀹㈡埛鏈哄彲浠ユ湁澶氫釜 ITS 鎺у埗鍣紝姣忎釜閮藉繀椤绘湁鐙珛銆佷笉閲嶅彔鐨?MMIO 鍖哄煙銆?

## 缁?

### KVM_DEV_ARM_VGIC_GRP_ADDR


  灞炴€э細
    KVM_VGIC_ITS_ADDR_TYPE (rw, 64-bit)
      GICv3 ITS 鎺у埗瀵勫瓨鍣ㄥ抚鍦ㄥ鎴锋満鐗╃悊鍦板潃绌洪棿涓殑鍩哄湴鍧€銆?      璇ュ湴鍧€闇€瑕?64K 瀵归綈锛屽苟涓旇鍖哄煙瑕嗙洊 128K銆?
  閿欒锛?
    =======  =================================================
    -E2BIG   鍦板潃瓒呭嚭鍙鍧€鐨?IPA 鑼冨洿
    -EINVAL  鍦板潃瀵归綈涓嶆纭?    -EEXIST  鍦板潃宸查厤缃?    -EFAULT  attr->addr 鐨勭敤鎴锋寚閽堟棤鏁堛€?    -ENODEV  灞炴€т笉姝ｇ‘鎴?ITS 涓嶅彈鏀寔銆?    =======  =================================================


### KVM_DEV_ARM_VGIC_GRP_CTRL


  灞炴€э細
    KVM_DEV_ARM_VGIC_CTRL_INIT
      璇锋眰鍒濆鍖?ITS锛宬vm_device_attr.addr 涓病鏈夐澶栧弬鏁般€?
    KVM_DEV_ARM_ITS_CTRL_RESET
      澶嶄綅 ITS锛宬vm_device_attr.addr 涓病鏈夐澶栧弬鏁般€?      鍙傝鈥淚TS 澶嶄綅鐘舵€佲€濅竴鑺傘€?
    KVM_DEV_ARM_ITS_SAVE_TABLES
      灏?ITS 琛ㄦ暟鎹繚瀛樺埌瀹㈡埛鏈?RAM 涓紝浣嶇疆鐢卞鎴锋満鍦ㄧ浉搴斿瘎瀛樺櫒/琛ㄩ」涓墍鎻愪緵銆?      濡傛灉鐢ㄦ埛绌洪棿闇€瑕佹煇绉嶅舰寮忕殑鑴忛〉璺熻釜鏉ヨ瘑鍒摢浜涢〉琚繚瀛樿繃绋嬩慨鏀癸紝瀹冨簲浣跨敤涓€涓?      浣嶅浘锛屽嵆浣夸娇鐢ㄥ叾瀹冩満鍒舵潵璺熻釜鐢?vCPU 寮勮剰鐨勫唴瀛樸€?
      瀹㈡埛鏈哄唴瀛樹腑琛ㄧ殑甯冨眬瀹氫箟浜嗕竴涓?ABI銆傝〃椤逛互灏忕鏍煎紡鎺掑垪锛屽鏈€鍚庝竴娈垫墍杩般€?
    KVM_DEV_ARM_ITS_RESTORE_TABLES
      灏?ITS 琛ㄤ粠瀹㈡埛鏈?RAM 鎭㈠鍒?ITS 鍐呴儴缁撴瀯銆?
      GICV3 蹇呴』鍦?ITS 涔嬪墠鎭㈠锛屽苟涓旈櫎 GITS_CTLR 涔嬪鐨勬墍鏈?ITS 瀵勫瓨鍣ㄩ兘蹇呴』鍦?      鎭㈠ ITS 琛ㄤ箣鍓嶆仮澶嶃€?
      GITS_IIDR 鍙瀵勫瓨鍣ㄤ篃蹇呴』鍦ㄨ皟鐢?KVM_DEV_ARM_ITS_RESTORE_TABLES 涔嬪墠鎭㈠锛?      鍥犱负 IIDR 淇瀛楁缂栫爜浜?ABI 淇鍙枫€?
      鎭㈠ GICv3/ITS 鏃剁殑棰勬湡椤哄簭鍦ㄢ€淚TS 鎭㈠搴忓垪鈥濅竴鑺備腑鎻忚堪銆?
  閿欒锛?
    =======  ==========================================================
     -ENXIO  ITS 鍦ㄨ缃灞炴€т箣鍓嶆湭鎸夎姹傛纭厤缃?    -ENOMEM  鍒嗛厤 ITS 鍐呴儴鏁版嵁鏃跺唴瀛樹笉瓒?    -EINVAL  鎭㈠鐨勬暩鎹笉涓€鑷?    -EFAULT  鏃犳晥鐨勫鎴锋満 ram 璁块棶
    -EBUSY   涓€涓垨澶氫釜 VCPU 姝ｅ湪杩愯
    -EACCES  铏氭嫙 ITS 鐢辩墿鐞?GICv4 ITS 鏀拺锛屽苟涓斿湪娌℃湁 GICv4.1 鐨勬儏鍐典笅鐘舵€佷笉鍙敤
    =======  ==========================================================

### KVM_DEV_ARM_VGIC_GRP_ITS_REGS


  灞炴€э細
      kvm_device_attr 鐨?attr 瀛楁缂栫爜浜?ITS 瀵勫瓨鍣ㄧ浉瀵逛簬 ITS 鎺у埗甯у熀鍦板潃
      锛圛TS_base锛夌殑鍋忕Щ閲忋€?
      kvm_device_attr.addr 鎸囧悜涓€涓?__u64 鍊硷紝鏃犺琚鍧€瀵勫瓨鍣ㄧ殑瀹藉害锛?2/64 浣嶏級
      濡備綍銆?4 浣嶅瘎瀛樺櫒鍙兘浠ュ畬鏁撮暱搴﹁闂€?
      瀵瑰彧璇诲瘎瀛樺櫒鐨勫啓鍏ヤ細琚唴鏍稿拷鐣ワ紝浣嗕互涓嬮櫎澶栵細

      - GITS_CREADR銆傚繀椤绘仮澶嶅畠锛屽惁鍒欓槦鍒椾腑鐨勫懡浠や細鍦ㄦ仮澶?CWRITER 鍚庨噸鏂版墽琛屻€?        GITS_CREADR 蹇呴』鍦ㄦ仮澶?GITS_CTLR锛堝悗鑰呭彲鑳戒細鍚敤 ITS锛変箣鍓嶆仮澶嶃€傚悓鏃跺畠蹇呴』
        鍦?GITS_CBASER 涔嬪悗鎭㈠锛屽洜涓哄 GITS_CBASER 鐨勫啓鍏ヤ細閲嶇疆 GITS_CREADR銆?      - GITS_IIDR銆俁evision 瀛楁缂栫爜浜嗚〃甯冨眬 ABI 淇鍙枫€傚皢鏉ユ垜浠彲鑳藉疄鐜拌櫄鎷?LPI
        鐨勭洿鎺ユ敞鍏ャ€傝繖灏嗛渶瑕佸崌绾ц〃甯冨眬浠ュ強 ABI 鐨勬紨杩涖€侴ITS_IIDR 蹇呴』鍦ㄨ皟鐢?        KVM_DEV_ARM_ITS_RESTORE_TABLES 涔嬪墠鎭㈠銆?
      瀵逛簬鍏跺畠瀵勫瓨鍣紝鑾峰彇鎴栬缃竴涓瘎瀛樺櫒涓庡湪鐪熷疄纭欢涓婅鍙?鍐欏叆璇ュ瘎瀛樺櫒鍏锋湁鐩稿悓鐨?      鏁堟灉銆?
  閿欒锛?
    =======  ====================================================
    -ENXIO   鍋忕Щ閲忎笉瀵瑰簲浜庝换浣曞彈鏀寔鐨勫瘎瀛樺櫒
    -EFAULT  attr->addr 鐨勭敤鎴锋寚閽堟棤鏁?    -EINVAL  鍋忕Щ閲忔湭 64 浣嶅榻?    -EBUSY   涓€涓垨澶氫釜 VCPU 姝ｅ湪杩愯
    =======  ====================================================

### ITS 鎭㈠搴忓垪锛?

鍦ㄦ仮澶?GIC銆両TS 鍜?KVM_IRQFD 璧嬪€兼椂蹇呴』閬靛惊浠ヤ笅椤哄簭锛?
a) 鎭㈠鎵€鏈夊鎴锋満鍐呭瓨骞跺垱寤?vcpu
b) 鎭㈠鎵€鏈夐噸鍒嗗彂鍣紙redistributor锛?c) 鎻愪緵 ITS 鍩哄湴鍧€
   (KVM_DEV_ARM_VGIC_GRP_ADDR)
d) 鎸変互涓嬮『搴忔仮澶?ITS锛?
     1. 鎭㈠ GITS_CBASER
     2. 鎭㈠鎵€鏈夊叾瀹?`GITS_` 瀵勫瓨鍣紝浣?GITS_CTLR 闄ゅ锛?     3. 鍔犺浇 ITS 琛ㄦ暟鎹紙KVM_DEV_ARM_ITS_RESTORE_TABLES锛?     4. 鎭㈠ GITS_CTLR

e) 鎭㈠ MSI 鐨?KVM_IRQFD 璧嬪€?
鐒跺悗 vcpu 鍙互鍚姩銆?
### ITS 琛?ABI REV0锛?

ABI 鐨勪慨璁?0 浠呮敮鎸佽櫄鎷?GICv3 鐨勭壒鎬э紝涓嶆敮鎸佸甫鏈夊祵濂楄櫄鎷熸満鐩戞帶绋嬪簭铏氭嫙涓柇
鐩存帴娉ㄥ叆鏀寔鐨勮櫄鎷?GICv4銆?
璁惧琛ㄥ拰 ITT 鍒嗗埆鐢?DeviceID 鍜?EventID 绱㈠紩銆傞泦鍚堣〃涓嶇敱 CollectionID 绱㈠紩锛岄泦鍚?涓殑琛ㄩ」浠ヤ换鎰忛『搴忓垪鍑恒€傛墍鏈夎〃椤瑰潎涓?8 瀛楄妭銆?
```

   bits:     | 63| 62 ... 49 | 48 ... 5 | 4 ... 0 |
   values:   | V |   next    | ITT_addr |  Size   |

 where:

 - V 鎸囩ず璇ヨ〃椤规槸鍚︽湁鏁堛€傚鏋滄棤鏁堬紝鍏跺畠瀛楁娌℃湁鎰忎箟銆? - next锛氬鏋滄琛ㄩ」鏄渶鍚庝竴涓紝鍒欑瓑浜?0锛涘惁鍒欏畠瀵瑰簲浜庡埌涓嬩竴涓?DTE 鐨?DeviceID
   鍋忕Щ閲忥紝涓婇檺涓?2^14 -1銆? - ITT_addr 鍖归厤 ITT 鍦板潃鐨?[51:8] 浣嶏紙256 瀛楄妭瀵归綈锛夈€? - Size 鎸囧畾 EventID 鏀寔鐨勪綅鏁板噺涓€

 Collection Table Entry (CTE)::

   bits:     | 63| 62 ..  52  | 51 ... 16 | 15  ...   0 |
   values:   | V |    RES0    |  RDBase   |    ICID     |

 where:

 - V 鎸囩ず璇ヨ〃椤规槸鍚︽湁鏁堛€傚鏋滄棤鏁堬紝鍏跺畠瀛楁娌℃湁鎰忎箟銆? - RES0锛氫繚鐣欏瓧娈碉紝鍏锋湁 Should-Be-Zero-or-Preserved 琛屼负銆? - RDBase 鏄?PE 缂栧彿锛圙ICR_TYPER.Processor_Number 璇箟锛夛紝
 - ICID 鏄泦鍚?ID

 Interrupt Translation Entry (ITE)::

   bits:     | 63 ... 48 | 47 ... 16 | 15 ... 0 |
   values:   |    next   |   pINTID  |  ICID    |

 where:

 - next锛氬鏋滄琛ㄩ」鏄渶鍚庝竴涓紝鍒欑瓑浜?0锛涘惁鍒欏畠瀵瑰簲浜庡埌涓嬩竴涓?ITE 鐨?EventID
   鍋忕Щ閲忥紝涓婇檺涓?2^16 -1銆? - pINTID 鏄墿鐞?LPI ID锛涘鏋滀负闆讹紝鎰忓懗鐫€璇ヨ〃椤规棤鏁堬紝鍏跺畠瀛楁娌℃湁鎰忎箟銆? - ICID 鏄泦鍚?ID

```
### ITS 澶嶄綅鐘舵€侊細


RESET 灏?ITS 杩斿洖鍒板畠棣栨琚垱寤哄拰鍒濆鍖栨椂鐨勭浉鍚岀姸鎬併€傚綋 RESET 鍛戒护杩斿洖鏃讹紝淇濊瘉
浠ヤ笅浜嬮」锛?
- ITS 鏈惎鐢ㄤ笖闈欐
  GITS_CTLR.Enabled = 0 .Quiescent=1
- 娌℃湁鍐呴儴缂撳瓨鐨勭姸鎬?- 娌℃湁浣跨敤闆嗗悎琛ㄦ垨璁惧琛?  GITS_BASER<n>.Valid = 0
- GITS_CBASER = 0, GITS_CREADR = 0, GITS_CWRITER = 0
- ABI 鐗堟湰涓嶅彉锛屽苟淇濇寔涓?ITS 璁惧棣栨鍒涘缓鏃舵墍璁剧疆鐨勭増鏈€?