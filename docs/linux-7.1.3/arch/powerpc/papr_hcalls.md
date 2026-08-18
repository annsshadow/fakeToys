
## 瓒呯骇璋冪敤鎿嶄綔鐮侊紙hcalls锛?


## 姒傝堪

64 浣?Power Book3S 骞冲彴涓婄殑铏氭嫙鍖栧熀浜?PAPR 瑙勮寖 [^1^]_锛岃瑙勮寖鎻忚堪浜嗗鎴锋満
鎿嶄綔绯荤粺鐨勮繍琛屾椂鐜锛屼互鍙婂鎴锋満搴斿浣曚笌 hypervisor 浜や簰浠ユ墽琛岀壒鏉冩搷浣溿€傜洰鍓?
鏈変袱绉嶇鍚?PAPR 鐨?hypervisor锛?

- **IBM PowerVM (PHYP)**锛欼BM 鐨勪笓鏈?hypervisor锛屾敮鎸佸皢 AIX銆両BM-i 鍜?Linux
  浣滀负鍙楁敮鎸佺殑瀹㈡埛鏈猴紙绉颁负閫昏緫鍒嗗尯鎴?LPARS锛夈€傚畠鏀寔瀹屾暣鐨?PAPR 瑙勮寖銆?

- **Qemu/KVM**锛氭敮鎸佽繍琛屽湪 PPC64 Linux 瀹夸富鏈轰笂鐨?PPC64 Linux 瀹㈡埛鏈恒€備笉杩囧畠
  浠呭疄鐜颁簡 PAPR 瑙勮寖鐨勪竴涓悕涓?LoPAPR 鐨勫瓙闆?[^2^]_銆?

鍦?PPC64 鏋舵瀯涓婏紝杩愯浜?PAPR hypervisor 涔嬩笂鐨勫鎴锋満鍐呮牳绉颁负 **pSeries 瀹㈡埛鏈?*銆?
pSeries 瀹㈡埛鏈鸿繍琛屽湪鐗规潈绾фā寮忥紙HV=0锛変笅锛屾瘡褰撻渶瑕佹墽琛?hypervisor 鐗规潈鎿嶄綔 [^3^]_
鎴?hypervisor 绠＄悊鐨勫叾浠栨湇鍔℃椂锛岄兘蹇呴』鍚?hypervisor 鍙戝嚭瓒呯骇璋冪敤銆?

鍥犳锛岃秴绾ц皟鐢紙hcall锛夋湰璐ㄤ笂鏄?pSeries 瀹㈡埛鏈鸿姹?hypervisor 浠ｈ〃瀹㈡埛鏈烘墽琛岀壒鏉?
鎿嶄綔銆傚鎴锋満鍙戝嚭璋冪敤骞舵彁渚涘繀瑕佺殑杈撳叆鎿嶄綔鏁般€俬ypervisor 鎵ц瀹岀壒鏉冩搷浣滃悗锛屽皢鐘舵€佺爜
鍜岃緭鍑烘搷浣滄暟杩斿洖缁欏鎴锋満銆?

## HCALL ABI

hcall 鐨?ABI 瑙勮寖锛坧series 瀹㈡埛鏈轰笌 PAPR hypervisor 涔嬮棿锛夊湪鍙傝€冩枃鐚?[^2^]_ 鐨?
绗?14.5.3 鑺備腑鎻忚堪銆傚垏鎹㈠埌 Hypervisor 涓婁笅鏂囬€氳繃鎸囦护 **HVCS** 瀹屾垚锛岃鎸囦护瑕佹眰灏?
hcall 鐨勬搷浣滅爜璁剧疆鍦?**r3** 涓紝hcall 鐨勪换浣曡緭鍏ュ弬鏁板湪瀵勫瓨鍣?**r4-r12** 涓彁渚涖€?
濡傛灉闇€瑕侀€氳繃鍐呭瓨缂撳啿鍖轰紶閫掑€硷紝瀛樺偍鍦ㄨ缂撳啿鍖轰腑鐨勬暟鎹簲閲囩敤澶х瀛楄妭搴忋€?

涓€鏃?hypervisor 澶勭悊瀹?'HVCS' 鎸囦护骞跺皢鎺у埗鏉冭繑鍥炵粰瀹㈡埛鏈猴紝hcall 鐨勮繑鍥炲€煎彲鍦?
**r3** 涓幏鍙栵紝浠讳綍杈撳嚭鍊煎湪瀵勫瓨鍣?**r4-r12** 涓繑鍥炪€備笌杈撳叆鍙傛暟绫讳技锛屽瓨鍌ㄥ湪鍐呭瓨
缂撳啿鍖轰腑鐨勪换浣曡緭鍑哄€奸兘灏嗛噰鐢ㄥぇ绔瓧鑺傚簭銆?

PowerPC 鏋舵瀯浠ｇ爜鎻愪緵浜嗗悕涓?**plpar_hcall_xxx** 鐨勪究鎹峰皝瑁呭嚱鏁帮紝瀹氫箟鍦ㄦ灦鏋勭壒瀹氱殑
澶存枃浠朵腑 [^4^]_锛岀敤浜庝粠浣滀负 pSeries 瀹㈡埛鏈鸿繍琛岀殑 Linux 鍐呮牳涓彂鍑?hcall銆?

## 瀵勫瓨鍣ㄧ害瀹?

浠讳綍 hcall 閮藉簲閬靛惊 "64-Bit ELF V2 ABI Specification: Power Architecture"[^5^]_
绗?2.2.1.1 鑺備腑鎻忚堪鐨勭浉鍚屽瘎瀛樺櫒绾﹀畾銆備笅琛ㄦ眹鎬讳簡杩欎簺绾﹀畾锛?

+----------+----------+-------------------------------------------+
| Register |Volatile  |  Purpose                                  |
| Range    |(Y/N)     |                                           |
+==========+==========+===========================================+
|   r0     |    Y     |  鍙€夌敤閫?                                |
+----------+----------+-------------------------------------------+
|   r1     |    N     |  鏍堟寚閽?                                  |
+----------+----------+-------------------------------------------+
|   r2     |    N     |  TOC                                      |
+----------+----------+-------------------------------------------+
|   r3     |    Y     |  hcall 鎿嶄綔鐮?杩斿洖鍊?                      |
+----------+----------+-------------------------------------------+
|  r4-r10  |    Y     |  杈撳叆涓庤緭鍑哄€?                            |
+----------+----------+-------------------------------------------+
|   r11    |    Y     |  鍙€夌敤閫?鐜鎸囬拡                         |
+----------+----------+-------------------------------------------+
|   r12    |    Y     |  鍙€夌敤閫?鍏ㄥ眬鍏ュ彛鐐瑰鐨勫嚱鏁板叆鍙ｅ湴鍧€       |
|          |          |                                           |
+----------+----------+-------------------------------------------+
|   r13    |    N     |  绾跨▼鎸囬拡                                 |
+----------+----------+-------------------------------------------+
|  r14-r31 |    N     |  灞€閮ㄥ彉閲?                                |
+----------+----------+-------------------------------------------+
|    LR    |    Y     |  閾炬帴瀵勫瓨鍣?                              |
+----------+----------+-------------------------------------------+
|   CTR    |    Y     |  寰幆璁℃暟鍣?                              |
+----------+----------+-------------------------------------------+
|   XER    |    Y     |  瀹氱偣寮傚父瀵勫瓨鍣?                          |
+----------+----------+-------------------------------------------+
|  CR0-1   |    Y     |  鏉′欢瀵勫瓨鍣ㄥ瓧娈?                          |
+----------+----------+-------------------------------------------+
|  CR2-4   |    N     |  鏉′欢瀵勫瓨鍣ㄥ瓧娈?                          |
+----------+----------+-------------------------------------------+
|  CR5-7   |    Y     |  鏉′欢瀵勫瓨鍣ㄥ瓧娈?                          |
+----------+----------+-------------------------------------------+
|  Others  |    N     |                                           |
+----------+----------+-------------------------------------------+

## DRC 涓?DRC 绱㈠紩

```

     DR1                                  Guest
     +--+        +------------+         +---------+
     |  | <----> |            |         |  User   |
     +--+  DRC1  |            |   DRC   |  Space  |
                 |    PAPR    |  Index  +---------+
     DR2         | Hypervisor |         |         |
     +--+        |            | <-----> |  Kernel |
     |  | <----> |            |  Hcall  |         |
     +--+  DRC2  +------------+         +---------+

```
PAPR hypervisor 灏?LPAR 鍙敤鐨勫叡浜‖浠惰祫婧愶紙濡?PCI 璁惧銆丯VDIMM 绛夛級绉颁负鍔ㄦ€佽祫婧?
锛圖ynamic Resource锛孌R锛夈€傚綋 DR 鍒嗛厤缁欐煇涓?LPAR 鏃讹紝PHYP 浼氬垱寤轰竴涓悕涓哄姩鎬佽祫婧?
杩炴帴鍣紙Dynamic Resource Connector锛孌RC锛夌殑鏁版嵁缁撴瀯鏉ョ鐞?LPAR 鐨勮闂€侺PAR 閫氳繃
绉颁负 DRC-Index 鐨勪笉閫忔槑 32 浣嶆暟鍊兼潵寮曠敤 DRC銆侱RC-index 鍊奸€氳繃璁惧鏍戯紙device-tree锛?
鎻愪緵缁?LPAR锛屼綔涓轰笌 DR 鍏宠仈鐨勮澶囨爲鑺傜偣鐨勪竴涓睘鎬у瓨鍦ㄣ€?

## HCALL 杩斿洖鍊?

澶勭悊瀹?hcall 鍚庯紝hypervisor 鍦?**r3** 涓缃繑鍥炲€硷紝琛ㄧず hcall 鎴愬姛鎴栧け璐ャ€傝嫢澶辫触锛?
閿欒鐮佹寚绀哄嚭閿欏師鍥犮€傝繖浜涚爜鍦ㄦ灦鏋勭壒瀹氱殑澶存枃浠朵腑瀹氫箟鍜岃褰?[^4^]_銆?

鍦ㄦ煇浜涙儏鍐典笅锛宧call 鍙兘闇€瑕佸緢闀挎椂闂达紝骞朵笖闇€瑕佸娆″彂鍑烘墠鑳借瀹屽叏澶勭悊銆傝繖浜?hcall
閫氬父浼氬湪鍏跺弬鏁板垪琛ㄤ腑鎺ュ彈涓€涓笉閫忔槑鍊?**continue-token**锛岃繑鍥炲€间负 **H_CONTINUE**
琛ㄧず hypervisor 灏氭湭瀹屾垚瀵硅 hcall 鐨勫鐞嗐€?

涓哄彂鍑烘绫?hcall锛屽鎴锋満闇€瑕佸湪鍒濇璋冪敤鏃惰缃?**continue-token == 0**锛屽苟鍦ㄦ瘡娆?
鍚庣画 hcall 涓娇鐢?hypervisor 杩斿洖鐨?**continue-token** 鍊硷紝鐩村埌 hypervisor 杩斿洖
涓€涓潪 **H_CONTINUE** 鐨勮繑鍥炲€笺€?

## HCALL 鎿嶄綔鐮?

浠ヤ笅鏄?PHYP 鏀寔鐨?HCALL 鐨勯儴鍒嗗垪琛ㄣ€傚搴旂殑鎿嶄綔鐮佸€艰鏌ラ槄鏋舵瀯鐗瑰畾鐨勫ご鏂囦欢 [^4^]_锛?

**H_SCM_READ_METADATA**

| 杈撳叆锛?**drcIndex, offset, buffer-address, numBytesToRead**
| 杈撳嚭锛?**numBytesRead**
| 杩斿洖鍊硷細 **H_Success, H_Parameter, H_P2, H_P3, H_Hardware**

缁欏畾涓€涓?NVDIMM 鐨?DRC 绱㈠紩锛屼粠涓庡叾鍏宠仈鐨勫厓鏁版嵁鍖轰腑鍦ㄦ寚瀹氬亸绉诲璇诲彇 N 瀛楄妭锛屽苟澶嶅埗鍒?
鎵€鎻愪緵鐨勭紦鍐插尯銆傚厓鏁版嵁鍖哄瓨鍌ㄩ厤缃俊鎭紝濡傛爣绛句俊鎭€佸潖鍧楃瓑銆傚厓鏁版嵁鍖轰綅浜?NVDIMM 瀛樺偍鍖?
甯﹀锛屽洜姝ゆ彁渚涗簡鍗曠嫭鐨勮闂涔夈€?

**H_SCM_WRITE_METADATA**

| 杈撳叆锛?**drcIndex, offset, data, numBytesToWrite**
| 杈撳嚭锛?**None**
| 杩斿洖鍊硷細 **H_Success, H_Parameter, H_P2, H_P4, H_Hardware**

缁欏畾涓€涓?NVDIMM 鐨?DRC 绱㈠紩锛屽湪鎸囧畾鍋忕Щ澶勫皢 N 瀛楄妭鍐欏叆涓庡叾鍏宠仈鐨勫厓鏁版嵁鍖猴紝鏁版嵁鏉ヨ嚜
鎵€鎻愪緵鐨勭紦鍐插尯銆?

**H_SCM_BIND_MEM**

| 杈撳叆锛?**drcIndex, startingScmBlockIndex, numScmBlocksToBind,**
| **targetLogicalMemoryAddress, continue-token**
| 杈撳嚭锛?**continue-token, targetLogicalMemoryAddress, numScmBlocksToBound**
| 杩斿洖鍊硷細 **H_Success, H_Parameter, H_P2, H_P3, H_P4, H_Overlap,**
| **H_Too_Big, H_P5, H_Busy**

缁欏畾涓€涓?NVDIMM 鐨?DRC 绱㈠紩锛屽皢涓€娈佃繛缁殑 SCM 鍧楄寖鍥?
**(startingScmBlockIndex, startingScmBlockIndex+numScmBlocksToBind)** 鏄犲皠鍒板鎴锋満
鐗╃悊鍦板潃绌洪棿涓殑 **targetLogicalMemoryAddress** 澶勩€傚鏋?
**targetLogicalMemoryAddress == 0xFFFFFFFF_FFFFFFFF**锛屽垯鐢?hypervisor 涓哄鎴锋満鍒嗛厤
鐩爣鍦板潃銆傚鏋滃鎴锋満瀵硅缁戝畾鐨?SCM 鍧楀瓨鍦ㄦ椿璺冪殑 PTE 鏉＄洰锛岃 HCALL 鍙兘澶辫触銆?

**H_SCM_UNBIND_MEM**
| 杈撳叆锛?drcIndex, startingScmLogicalMemoryAddress, numScmBlocksToUnbind
| 杈撳嚭锛?numScmBlocksUnbound
| 杩斿洖鍊硷細 **H_Success, H_Parameter, H_P2, H_P3, H_In_Use, H_Overlap,**
| **H_Busy, H_LongBusyOrder1mSec, H_LongBusyOrder10mSec**

缁欏畾涓€涓?NVDIMM 鐨?DRC 绱㈠紩锛屼粠瀹㈡埛鏈虹墿鐞嗗湴鍧€绌洪棿鍙栨秷鏄犲皠浠?
**startingScmLogicalMemoryAddress** 寮€濮嬬殑 **numScmBlocksToUnbind** 涓?SCM 鍧椼€?
濡傛灉瀹㈡埛鏈哄琚В缁戠殑 SCM 鍧楀瓨鍦ㄦ椿璺冪殑 PTE 鏉＄洰锛岃 HCALL 鍙兘澶辫触銆?

**H_SCM_QUERY_BLOCK_MEM_BINDING**

| 杈撳叆锛?**drcIndex, scmBlockIndex**
| 杈撳嚭锛?**Guest-Physical-Address**
| 杩斿洖鍊硷細 **H_Success, H_Parameter, H_P2, H_NotFound**

缁欏畾涓€涓?DRC 绱㈠紩鍜?SCM 鍧楃储寮曪紝杩斿洖璇?SCM 鍧楁墍鏄犲皠鍒扮殑瀹㈡埛鏈虹墿鐞嗗湴鍧€銆?

**H_SCM_QUERY_LOGICAL_MEM_BINDING**

| 杈撳叆锛?**Guest-Physical-Address**
| 杈撳嚭锛?**drcIndex, scmBlockIndex**
| 杩斿洖鍊硷細 **H_Success, H_Parameter, H_P2, H_NotFound**

缁欏畾涓€涓鎴锋満鐗╃悊鍦板潃锛岃繑鍥炴槧灏勫埌璇ュ湴鍧€鐨?DRC 绱㈠紩鍜?SCM 鍧椼€?

**H_SCM_UNBIND_ALL**

| 杈撳叆锛?**scmTargetScope, drcIndex**
| 杈撳嚭锛?**None**
| 杩斿洖鍊硷細 **H_Success, H_Parameter, H_P2, H_P3, H_In_Use, H_Busy,**
| **H_LongBusyOrder1mSec, H_LongBusyOrder10mSec**

鏍规嵁鐩爣鑼冨洿锛屼粠 LPAR 鍐呭瓨涓彇娑堟槧灏勫睘浜庢墍鏈?NVDIMM 鐨勬墍鏈?SCM 鍧楋紝鎴栧睘浜庣敱 drcIndex
鏍囪瘑鐨勫崟涓?NVDIMM 鐨勬墍鏈?SCM 鍧椼€?

**H_SCM_HEALTH**

| 杈撳叆锛?drcIndex
| 杈撳嚭锛?**health-bitmap (r4), health-bit-valid-bitmap (r5)**
| 杩斿洖鍊硷細 **H_Success, H_Parameter, H_Hardware**

缁欏畾涓€涓?DRC 绱㈠紩锛岃繑鍥?PMEM 璁惧鐨勯娴嬫€ф晠闅滃拰鏁翠綋鍋ュ悍淇℃伅銆俬ealth-bitmap 涓疆浣嶇殑
浣嶆寚绀?PMEM 璁惧鐨勪竴涓垨澶氫釜鐘舵€侊紙濡備笅琛ㄦ墍杩帮級锛宧ealth-bit-valid-bitmap 鎸囩ず
health-bitmap 涓殑鍝簺浣嶆湁鏁堛€備綅浠ラ€嗗簭浣嶅簭鎶ュ憡锛屼緥濡傚€?0xC400000000000000 琛ㄧず浣?
0銆? 鍜?5 鏈夋晥銆?

鍋ュ悍浣嶅浘鏍囧織锛?

+------+-----------------------------------------------------------------------+
|  Bit |               Definition                                              |
+======+=======================================================================+
|  00  |  PMEM 璁惧鏃犳硶鎸佷箙鍖栧唴瀛樺唴瀹广€傚鏋滅郴缁熸柇鐢碉紝鍒欎笉浼氫繚瀛樹换浣曞唴瀹广€?     |
|      |                                                                       |
+------+-----------------------------------------------------------------------+
|  01  |  PMEM 璁惧鏈兘鎸佷箙鍖栧唴瀛樺唴瀹广€傝涔堟柇鐢垫椂鍐呭鏈垚鍔熶繚瀛橈紝瑕佷箞涓婄數鏃舵湭  |
|      |  姝ｇ‘鎭㈠銆?                                                           |
+------+-----------------------------------------------------------------------+
|  02  |  PMEM 璁惧鍐呭宸蹭粠鍏堝墠鐨?IPL 鎸佷箙鍖栥€備笂娆″惎鍔ㄧ殑鏁版嵁宸叉垚鍔熸仮澶嶃€?       |
|      |                                                                       |
+------+-----------------------------------------------------------------------+
|  03  |  PMEM 璁惧鍐呭鏈粠鍏堝墠鐨?IPL 鎸佷箙鍖栥€備笂娆″惎鍔ㄦ病鏈夊彲鎭㈠鐨勬暟鎹€?       |
|      |                                                                       |
+------+-----------------------------------------------------------------------+
|  04  |  PMEM 璁惧鍓╀綑鍐呭瓨瀵垮懡鏋佷綆                                            |
+------+-----------------------------------------------------------------------+
|  05  |  鐢变簬鏁呴殰锛孭MEM 璁惧灏嗗湪涓嬫 IPL 鏃惰闅旂锛坓arded off锛?              |
+------+-----------------------------------------------------------------------+
|  06  |  鐢变簬褰撳墠骞冲彴鍋ュ悍鐘舵€侊紝PMEM 璁惧鍐呭鏃犳硶鎸佷箙鍖栥€傜‖浠舵晠闅滃彲鑳介樆姝㈡暟鎹? |
|      |  鐨勪繚瀛樻垨鎭㈠銆?                                                       |
+------+-----------------------------------------------------------------------+
|  07  |  鍦ㄦ煇浜涙潯浠朵笅 PMEM 璁惧鏃犳硶鎸佷箙鍖栧唴瀛樺唴瀹?                            |
+------+-----------------------------------------------------------------------+
|  08  |  PMEM 璁惧宸插姞瀵?                                                     |
+------+-----------------------------------------------------------------------+
|  09  |  PMEM 璁惧宸叉垚鍔熷畬鎴愯姹傜殑鎿﹂櫎鎴栧畨鍏ㄦ摝闄よ繃绋嬨€?                       |
|      |                                                                       |
+------+-----------------------------------------------------------------------+
|10:63 |  淇濈暀 / 鏈娇鐢?                                                       |
+------+-----------------------------------------------------------------------+

**H_SCM_PERFORMANCE_STATS**

| 杈撳叆锛?drcIndex, resultBuffer Addr
| 杈撳嚭锛?None
| 杩斿洖鍊硷細  **H_Success, H_Parameter, H_Unsupported, H_Hardware, H_Authority, H_Privilege**

缁欏畾涓€涓?DRC 绱㈠紩锛屾敹闆?NVDIMM 鐨勬€ц兘缁熻淇℃伅骞跺皢鍏跺鍒跺埌 resultBuffer銆?

**H_SCM_FLUSH**

| 杈撳叆锛?**drcIndex, continue-token**
| 杈撳嚭锛?**continue-token**
| 杩斿洖鍊硷細 **H_SUCCESS, H_Parameter, H_P2, H_BUSY**

缁欏畾涓€涓?DRC 绱㈠紩锛屽皢鏁版嵁鍒锋柊鍒板悗绔?NVDIMM 璁惧銆?

褰撳埛鏂拌€楁椂杈冮暱鏃讹紝hcall 杩斿洖 H_BUSY锛屽苟涓旈渶瑕佸娆″彂鍑鸿 hcall 鎵嶈兘琚畬鍏ㄥ鐞嗐€傛潵鑷?
杈撳嚭鐨?**continue-token** 搴斾紶鍏ュ悗缁彂缁?hypervisor 鐨?hcall 鐨勫弬鏁板垪琛ㄤ腑锛岀洿鍒?hcall
琚畬鍏ㄥ鐞嗭紝姝ゆ椂 hypervisor 杩斿洖 H_SUCCESS 鎴栧叾浠栭敊璇€?

**H_HTM**

| 杈撳叆锛?flags, target, operation (op), op-param1, op-param2, op-param3
| 杈撳嚭锛?**dumphtmbufferdata**
| 杩斿洖鍊硷細 *H_Success,H_Busy,H_LongBusyOrder,H_Partial,H_Parameter,
		 H_P2,H_P3,H_P4,H_P5,H_P6,H_State,H_Not_Available,H_Authority*

H_HTM 鏀寔纭欢璺熻釜瀹忥紙Hardware Trace Macro锛孒TM锛夊姛鑳藉強鍏舵暟鎹殑璁剧疆銆侀厤缃€佹帶鍒跺拰
杞偍銆侶TM 缂撳啿鍖哄瓨鍌ㄦ牳蹇冩寚浠ゃ€佹牳蹇?LLAT 鍜?nest 绛夊姛鑳界殑璺熻釜鏁版嵁銆?

**H_PKS_GEN_KEY**

| 杈撳叆锛?authorization, objectlabel, objectlabellen, policy, out, outlen
| 杈撳嚭锛?**Hypervisor 鐢熸垚鐨勫瘑閽ワ紝鎴栧綋璁剧疆浜嗗寘瑁呭瘑閽ョ瓥鐣ユ椂涓?None**
| 杩斿洖鍊硷細 *H_SUCCESS, H_Function, H_State, H_R_State, H_Parameter, H_P2,
                H_P3, H_P4, H_P5, H_P6, H_Authority, H_Nomem, H_Busy, H_Resource,
                H_Aborted*

H_PKS_GEN_KEY 鐢ㄤ簬璁?hypervisor 鐢熸垚涓€涓柊闅忔満瀵嗛挜銆傝瀵嗛挜浣滀负瀵硅薄瀛樺偍鍦?Power LPAR
骞冲彴瀵嗛挜搴擄紙Platform KeyStore锛変腑锛屽甫鏈夋彁渚涚殑瀵硅薄鏍囩銆傝缃寘瑁呭瘑閽ョ瓥鐣ュ悗锛岃瀵嗛挜浠?
瀵?hypervisor 鍙锛岃€屽瘑閽ョ殑鏍囩瀵圭敤鎴蜂粛鍙銆傚寘瑁呭瘑閽ョ殑鐢熸垚浠呮敮鎸?32 瀛楄妭鐨勫瘑閽ュぇ灏忋€?

**H_PKS_WRAP_OBJECT**

| 杈撳叆锛?authorization, wrapkeylabel, wrapkeylabellen, objectwrapflags, in,
|        inlen, out, outlen, continue-token
| 杈撳嚭锛?**continue-token, 鍖呰鍚庡璞＄殑瀛楄妭澶у皬, 鍖呰鍚庣殑瀵硅薄**
| 杩斿洖鍊硷細 *H_SUCCESS, H_Function, H_State, H_R_State, H_Parameter, H_P2,
                H_P3, H_P4, H_P5, H_P6, H_P7, H_P8, H_P9, H_Authority, H_Invalid_Key,
                H_NOT_FOUND, H_Busy, H_LongBusy, H_Aborted*

H_PKS_WRAP_OBJECT 鐢ㄤ簬浣跨敤瀛樺偍鍦?Power LPAR 骞冲彴瀵嗛挜搴撲腑鐨勫寘瑁呭瘑閽ュ瀵硅薄杩涜鍖呰锛屽苟灏?
鍖呰鍚庣殑瀵硅薄杩斿洖缁欒皟鐢ㄨ€呫€傝皟鐢ㄨ€呮彁渚涘甫鏈?'wrapping key' 绛栫暐璁剧疆鐨勫寘瑁呭瘑閽ユ爣绛撅紝璇ュ瘑閽?
蹇呴』宸蹭娇鐢?H_PKS_GEN_KEY 棰勫厛鍒涘缓銆傜劧鍚庡鎻愪緵鐨勫璞′娇鐢ㄥ寘瑁呭瘑閽ュ拰闄勫姞鍏冩暟鎹繘琛屽姞瀵嗭紝
骞跺皢缁撴灉杩斿洖缁欒皟鐢ㄨ€呫€?

**H_PKS_UNWRAP_OBJECT**

| 杈撳叆锛?authorization, objectwrapflags, in, inlen, out, outlen, continue-token
| 杈撳嚭锛?**continue-token, 瑙ｅ寘鍚庡璞＄殑瀛楄妭澶у皬, 瑙ｅ寘鍚庣殑瀵硅薄**
| 杩斿洖鍊硷細 *H_SUCCESS, H_Function, H_State, H_R_State, H_Parameter, H_P2,
                H_P3, H_P4, H_P5, H_P6, H_P7, H_Authority, H_Unsupported, H_Bad_Data,
                H_NOT_FOUND, H_Invalid_Key, H_Busy, H_LongBusy, H_Aborted*

H_PKS_UNWRAP_OBJECT 鐢ㄤ簬瑙ｅ寘鍏堝墠浣跨敤 H_PKS_WRAP_OBJECT 鍖呰鐨勫璞°€?

## 鍙傝€冩枃鐚?

       https://en.wikipedia.org/wiki/Power_Architecture_Platform_Reference
       https://members.openpowerfoundation.org/document/dl/469
       https://openpowerfoundation.org/?resource_lib=power-isa-version-3-0
       https://openpowerfoundation.org/?resource_lib=64-bit-elf-v2-abi-specification-power-architecture
