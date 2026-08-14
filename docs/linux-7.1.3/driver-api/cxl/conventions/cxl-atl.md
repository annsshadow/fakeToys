
## ACPI PRM CXL Address Translation锛圓CPI PRM CXL 鍦板潃杞崲锛?


### Document锛堟枃妗ｏ級


CXL Revision 3.2, Version 1.0

### License锛堣鍙瘉锛?


SPDX-License Identifier: CC-BY-4.0

### Creator/Contributors锛堝垱寤鸿€?璐＄尞鑰咃級


- Robert Richter, AMD et al.

### Summary of the Change锛堝彉鏇存憳瑕侊級


CXL 鍥哄畾鍐呭瓨绐楀彛缁撴瀯锛圕FMWS锛夋弿杩颁簡涓庝竴涓垨澶氫釜 CXL 涓绘満妗ョ浉鍏宠仈鐨勯浂涓垨澶氫釜
涓绘満鐗╃悊鍦板潃锛圚PA锛夌獥鍙ｃ€侰XL 涓绘満妗ョ殑姣忎釜 HPA 鑼冨洿鐢变竴涓?CFMWS 琛ㄩ」琛ㄧず銆備竴涓?
HPA 鑼冨洿鍙互鍖呭惈褰撳墠鍒嗛厤缁?CXL.mem 璁惧鐨勫湴鍧€锛屾垨鑰呮搷浣滅郴缁熶篃鍙互灏嗕竴涓湴鍧€
绐楀彛涓殑鑼冨洿鍒嗛厤缁欐煇涓澶囥€?

涓绘満绠＄悊鍨嬭澶囧唴瀛橈紙Host-managed Device Memory锛夋槸鏄犲皠鍒扮郴缁熶竴鑷存€у湴鍧€绌洪棿銆?
涓斾富鏈哄彲浠ヤ娇鐢ㄦ爣鍑嗗洖鍐欙紙write-back锛夎涔夎闂殑璁惧闄勫姞鍐呭瓨銆傝绠＄悊鐨勫湴鍧€鑼冨洿
閰嶇疆鍦ㄨ澶囩殑 CXL HDM Decoder 瀵勫瓨鍣ㄤ腑銆傝澶囦腑鐨?HDM Decoder 璐熻矗閫氳繃鍓ョ鐗瑰畾鐨?
鍦板潃浣嶏紝灏?HPA 杞崲涓?DPA銆?

CXL 璁惧涓?CXL 妗ヤ娇鐢ㄧ浉鍚岀殑 HPA 绌洪棿銆傚畠鍦ㄥ睘浜庡悓涓€涓绘満鍩熺殑鎵€鏈夌粍浠朵箣闂存槸閫氱敤
鐨勩€傚湪涓绘満涓庤澶囦箣闂寸殑 CXL.mem 璺緞涓婏紝鍦板潃鍖哄煙鐨勮鍥惧繀椤讳繚鎸佷竴鑷淬€?

杩欎竴鐐瑰湪 **CXL 3.2 瑙勮寖**锛堣〃 1-1銆?.3.1銆?.2.4.20銆?.13.1銆?.18.1.3锛変腑
鏈夋墍鎻忚堪銆?[#cxl-spec-3.2]_

鍙栧喅浜庡钩鍙扮殑浜掕仈鏋舵瀯锛屾寕鎺ュ埌涓绘満鐨勭粍浠跺彲鑳戒笉鍏变韩鐩稿悓鐨勪富鏈虹墿鐞嗗湴鍧€绌洪棿銆傞偅浜?
骞冲彴闇€瑕佸湴鍧€杞崲锛屼互鍦ㄤ富鏈轰笌鎵€鎸傛帴鐨勭粍浠讹紙渚嬪 CXL 璁惧锛変箣闂磋浆鎹?HPA銆傝浆鎹?
鏈哄埗鏄富鏈虹壒瀹氱殑锛屼笖渚濊禆浜庡叿浣撳疄鐜般€?

渚嬪锛寈86 AMD 骞冲彴浣跨敤鏁版嵁 fabric锛圖ata Fabric锛夋潵绠＄悊瀵圭墿鐞嗗唴瀛樼殑璁块棶銆傝澶?
鎷ユ湁鑷繁鐨勫唴瀛樼┖闂达紝骞跺彲琚厤缃负浣跨敤涓庣郴缁熺墿鐞嗗湴鍧€锛圫PA锛変笉鍚岀殑鈥滃綊涓€鍖栧湴鍧€
锛圢ormalized addresses锛夆€濄€傚洜姝ゅ氨闇€瑕佸湴鍧€杞崲銆傝鎯呭弬瑙?
[x86 AMD Address Translation </admin-guide/RAS/address-translation>](x86 AMD Address Translation </admin-guide/RAS/address-translation>)銆?

閭ｄ簺 AMD 骞冲彴鍦ㄥ浐浠朵腑鎻愪緵 PRM [#prm-spec]_ 澶勭悊绋嬪簭锛屼互鎵ц鍚勭绫诲瀷鐨勫湴鍧€杞崲锛?
鍖呮嫭閽堝 CXL 绔偣銆侫MD Zen5 绯荤粺瀹炵幇浜?ACPI PRM CXL 鍦板潃杞崲鍥轰欢璋冪敤銆侫CPI PRM
澶勭悊绋嬪簭鏈変竴涓壒瀹氱殑 GUID锛岀敤浜庡敮涓€鏍囪瘑鏀寔褰掍竴鍖栧湴鍧€鐨勫钩鍙般€傝繖鍦?**ACPI v6.5
绉绘鎸囧崡**锛圓ddress Translation - CXL DPA to System Physical Address锛変腑鏈夎杞姐€?
[#amd-ppr-58088]_

鍦ㄥ綊涓€鍖栧湴鍧€妯″紡涓嬶紝HDM 瑙ｇ爜鍣ㄧ殑鍦板潃鑼冨洿蹇呴』浠ヤ笉鍚岀殑鏂瑰紡閰嶇疆鍜屽鐞嗐€傜鐐?HDM
瑙ｇ爜鍣ㄩ厤缃腑浣跨敤鐨勭‖浠跺湴鍧€涓嶆槸 SPA锛岄渶瑕佷粠鍏跺湴鍧€鑼冨洿杞崲鍒?CXL 涓绘満妗ョ殑鍦板潃
鑼冨洿銆傝繖瀵逛簬鍦?CFMWS 涓煡鎵剧鐐圭浉鍏宠仈鐨?CXL 涓绘満妗ヤ笌 HPA 绐楀彛灏や负閲嶈銆傛澶栵紝
浜ら敊锛坕nterleave锛夎В鐮佺敱鏁版嵁 fabric 瀹屾垚锛岀鐐瑰湪灏?HPA 杞崲涓?DPA 鏃跺苟涓嶆墽琛?
瑙ｇ爜銆傜浉鍙嶏紝绔偣鐨勪氦閿欒鍏抽棴锛?-way锛夈€傛渶鍚庯紝鍦板潃杞崲鍦ㄦ鏌ョ鐐圭殑纭欢鍦板潃鏃?
涔熷彲鑳借闇€瑕侊紝渚嬪鍦ㄦ€ц兘鍓栨瀽銆佽窡韪垨閿欒澶勭悊鏈熼棿銆?

```
                          -------------------------------
                          | Root Decoder (CFMWS)        |
                          | SPA Range: 0x850000000      |
                          | Size: 0x8000000000 (512 GB) |
                          | Interleave Ways: 1          |
                          -------------------------------
                                        |
                                        v
                          -------------------------------
                          | Host Bridge Decoder (HDM)   |
                          | SPA Range: 0x850000000      |
                          | Size: 0x8000000000 (512 GB) |
                          | Interleave Ways: 4          |
                          | Targets: endpoint5,8,11,13  |
                          | Granularity: 256            |
                          -------------------------------
                                        |
           -----------------------------+------------------------------
           |                  |                   |                   |
           v                  v                   v                   v
 ------------------- ------------------- ------------------- -------------------
 | endpoint5       | | endpoint8       | | endpoint11      | | endpoint13      |
 | decoder5.0      | | decoder8.0      | | decoder11.0     | | decoder13.0     |
 | PCIe:           | | PCIe:           | | PCIe:           | | PCIe:           |
 |   0000:e2:00.0  | |   0000:e3:00.0  | |   0000:e4:00.0  | |   0000:e1:00.0  |
 | DPA:            | | DPA:            | | DPA:            | | DPA:            |
 |   Start: 0x0    | |   Start: 0x0    | |   Start: 0x0    | |   Start: 0x0    |
 |   Size:         | |   Size:         | |   Size:         | |   Size:         |
 |    0x2000000000 | |    0x2000000000 | |    0x2000000000 | |    0x2000000000 |
 |    (128 GB)     | |    (128 GB)     | |    (128 GB)     | |    (128 GB)     |
 | Interleaving:   | | Interleaving:   | | Interleaving:   | | Interleaving:   |
 |   Ways: 1       | |   Ways: 1       | |   Ways: 1       | |   Ways: 1       |
 |   Gran: 256     | |   Gran: 256     | |   Gran: 256     | |   Gran: 256     |
 ------------------- ------------------- ------------------- -------------------
          |                   |                   |                   |
          v                   v                   v                   v
         DPA                 DPA                 DPA                 DPA

```
杩欏睍绀轰簡鍦?sysfs 涓殑琛ㄧず锛?


 /sys/bus/cxl/devices/endpoint5/decoder5.0/interleave_granularity:256
 /sys/bus/cxl/devices/endpoint5/decoder5.0/interleave_ways:1
 /sys/bus/cxl/devices/endpoint5/decoder5.0/size:0x2000000000
 /sys/bus/cxl/devices/endpoint5/decoder5.0/start:0x0
 /sys/bus/cxl/devices/endpoint8/decoder8.0/interleave_granularity:256
 /sys/bus/cxl/devices/endpoint8/decoder8.0/interleave_ways:1
 /sys/bus/cxl/devices/endpoint8/decoder8.0/size:0x2000000000
 /sys/bus/cxl/devices/endpoint8/decoder8.0/start:0x0
 /sys/bus/cxl/devices/endpoint11/decoder11.0/interleave_granularity:256
 /sys/bus/cxl/devices/endpoint11/decoder11.0/interleave_ways:1
 /sys/bus/cxl/devices/endpoint11/decoder11.0/size:0x2000000000
 /sys/bus/cxl/devices/endpoint11/decoder11.0/start:0x0
 /sys/bus/cxl/devices/endpoint13/decoder13.0/interleave_granularity:256
 /sys/bus/cxl/devices/endpoint13/decoder13.0/interleave_ways:1
 /sys/bus/cxl/devices/endpoint13/decoder13.0/size:0x2000000000
 /sys/bus/cxl/devices/endpoint13/decoder13.0/start:0x0

娉ㄦ剰锛岀鐐逛氦閿欓厤缃娇鐢ㄧ洿鎺ユ槧灏勶紙1-way锛夈€?

鍊熷姪 PRM 璋冪敤锛屽唴鏍稿彲浠ョ‘瀹氫互涓嬫槧灏勶細


 cxl decoder5.0: address mapping found for 0000:e2:00.0 (hpa -> spa):
   0x0+0x2000000000 -> 0x850000000+0x8000000000 ways:4 granularity:256
 cxl decoder8.0: address mapping found for 0000:e3:00.0 (hpa -> spa):
   0x0+0x2000000000 -> 0x850000000+0x8000000000 ways:4 granularity:256
 cxl decoder11.0: address mapping found for 0000:e4:00.0 (hpa -> spa):
   0x0+0x2000000000 -> 0x850000000+0x8000000000 ways:4 granularity:256
 cxl decoder13.0: address mapping found for 0000:e1:00.0 (hpa -> spa):
   0x0+0x2000000000 -> 0x850000000+0x8000000000 ways:4 granularity:256

鐩稿簲鐨?CXL 涓绘満妗ワ紙HDM锛夎В鐮佸櫒涓庢牴瑙ｇ爜鍣紙CFMWS锛夊尮閰嶄笂闈㈡墍绀虹殑璁＄畻鍑虹殑绔偣
鏄犲皠锛?


 /sys/bus/cxl/devices/port1/decoder1.0/interleave_granularity:256
 /sys/bus/cxl/devices/port1/decoder1.0/interleave_ways:4
 /sys/bus/cxl/devices/port1/decoder1.0/size:0x8000000000
 /sys/bus/cxl/devices/port1/decoder1.0/start:0x850000000
 /sys/bus/cxl/devices/port1/decoder1.0/target_list:0,1,2,3
 /sys/bus/cxl/devices/port1/decoder1.0/target_type:expander
 /sys/bus/cxl/devices/root0/decoder0.0/interleave_granularity:256
 /sys/bus/cxl/devices/root0/decoder0.0/interleave_ways:1
 /sys/bus/cxl/devices/root0/decoder0.0/size:0x8000000000
 /sys/bus/cxl/devices/root0/decoder0.0/start:0x850000000
 /sys/bus/cxl/devices/root0/decoder0.0/target_list:7

闇€瑕佸瑙勮寖杩涜浠ヤ笅鍙樻洿锛?

- 鍏佽 CXL 璁惧澶勪簬涓绘満鍦板潃绌洪棿涔嬪鐨?HPA 绌洪棿涓€?

- 鍏佽骞冲彴鍦ㄤ富鏈轰笌璁惧涔嬮棿 CXL.mem 璺緞涓婅法瓒婂唴瀛樺煙鏃朵娇鐢ㄧ壒瀹氫簬瀹炵幇鐨勫湴鍧€
  杞崲銆?

- 瀹氫箟涓€绉嶅皢璁惧鍦板潃杞崲涓?SPA 鐨?PRM 澶勭悊绋嬪簭鏂规硶銆?

- 瑙勫畾骞冲彴搴斿悜鎿嶄綔绯荤粺鎻愪緵 PRM 澶勭悊绋嬪簭鏂规硶锛屼互妫€娴嬪綊涓€鍖栧湴鍧€锛屽苟纭畾绔偣
  SPA 鑼冨洿涓庝氦閿欓厤缃€?

- 娣诲姞瀵逛互涓嬫枃妗ｇ殑寮曠敤锛?

  | 骞冲彴杩愯鏃舵満鍒惰鑼冿紝鐗堟湰 1.1 鈥?2020 骞?11 鏈?
  | https://uefi.org/sites/default/files/resources/PRM_Platform_Runtime_Mechanism_1_1_release_candidate.pdf

### Benefits of the Change锛堝彉鏇寸殑濂藉锛?


濡傛灉涓嶅仛姝ゅ彉鏇达紝鎿嶄綔绯荤粺鍙兘鏃犳硶纭畾绔偣鐨勫唴瀛樺尯鍩熶笌鏍硅В鐮佸櫒锛屼互鍙婂叾瀵瑰簲鐨?
HDM 瑙ｇ爜鍣ㄣ€傚尯鍩熷垱寤轰細澶辫触銆傚叿鏈変笉鍚屼簰鑱旀灦鏋勭殑骞冲彴灏嗘棤娉曞缓绔嬪苟浣跨敤 CXL銆?

### References锛堝弬鑰冭祫鏂欙級


   https://www.computeexpresslink.org/

   ACPI v6.5 绉绘鎸囧崡锛屽嚭鐗堢墿缂栧彿 # 58088锛?
   https://www.amd.com/en/search/documentation/hub.html

   https://uefi.org/sites/default/files/resources/PRM_Platform_Runtime_Mechanism_1_1_release_candidate.pdf

### Detailed Description of the Change锛堝彉鏇寸殑璇︾粏鎻忚堪锛?


浠ヤ笅鎻忚堪浜嗗 **CXL 3.2 瑙勮寖** [#cxl-spec-3.2]_ 鎵€闇€鐨勫彉鏇达細

鍚戣〃涓坊鍔犱互涓嬪紩鐢細

Table 1-2. Reference Documents锛堝弬鑰冩枃妗ｏ級

+----------------------------+-------------------+---------------------------+
| Document锛堟枃妗ｏ級           | Chapter Reference | Document No./Location     |
|                            | 锛堢珷鑺傚紩鐢級      | 锛堟枃妗ｇ紪鍙?浣嶇疆锛?        |
+============================+===================+===========================+
| Platform Runtime Mechanism | Chapter 8, 9      | https://www.uefi.org/acpi |
| Version: 1.1               |                   |                           |
+----------------------------+-------------------+---------------------------+

鍦ㄧ珷鑺傛湯灏炬坊鍔犱互涓嬫钀斤細

**8.2.4.20 CXL HDM 瑙ｇ爜鍣ㄨ兘鍔涚粨鏋?*

鈥滀竴涓澶囧彲浠ヤ娇鐢ㄤ笌鍏朵富鏈哄煙鍏朵粬缁勪欢涓嶉€氱敤鐨?HPA 绌洪棿銆傚钩鍙拌礋璐ｅ湪璺ㄨ秺 HPA 绌洪棿
鏃惰繘琛屽湴鍧€杞崲銆傛搷浣滅郴缁熷繀椤荤‘瀹氫氦閿欓厤缃紝骞跺湪闇€瑕佹椂鎵ц鍒?HDM 瑙ｇ爜鍣?HPA 鑼冨洿
鐨勫湴鍧€杞崲銆傝浆鎹㈡満鍒舵槸涓绘満鐗瑰畾鐨勶紝涓斾緷璧栦簬鍏蜂綋瀹炵幇銆?

骞冲彴閫氳繃鎻愪緵涓€涓钩鍙拌繍琛屾椂鏈哄埗锛圥RM锛夊鐞嗙▼搴忔潵琛ㄦ槑瀵圭嫭绔?HPA 绌洪棿鐨勬敮鎸佷互鍙?
瀵瑰湴鍧€杞崲鐨勯渶瑕併€傛搷浣滅郴缁熷簲浣跨敤璇ュ鐞嗙▼搴忔墽琛屼粠 DPA 绌洪棿鍒?HPA 绌洪棿鎵€闇€鐨勮浆鎹€?
璇ュ鐞嗙▼搴忓湪 9.18.4 鑺?*PRM Handler for CXL DPA 鍒扮郴缁熺墿鐞嗗湴鍧€杞崲* 涓畾涔夈€傗€?

娣诲姞浠ヤ笅绔犺妭涓庡皬鑺傦紙鍚〃鏍硷級锛?

**9.18.4 鐢ㄤ簬 CXL DPA 鍒扮郴缁熺墿鐞嗗湴鍧€杞崲鐨?PRM 澶勭悊绋嬪簭**

鈥滀竴涓钩鍙板彲琚厤缃负浣跨敤鈥樺綊涓€鍖栧湴鍧€鈥欍€備富鏈虹墿鐞嗗湴鍧€锛圚PA锛夌┖闂存槸缁勪欢鐗瑰畾鐨勶紝
骞朵笖涓嶅悓浜庣郴缁熺墿鐞嗗湴鍧€锛圫PA锛夈€傜鐐规嫢鏈夎嚜宸辩殑鐗╃悊鍦板潃绌洪棿銆傚憟鐜扮粰璁惧鐨勬墍鏈?
璇锋眰宸茬粡浣跨敤璁惧鐗╃悊鍦板潃锛圖PA锛夈€侰XL 绔偣瑙ｇ爜鍣ㄥ叧闂氦閿欙紙1-way 浜ら敊锛夛紝骞朵笖
璁惧涓嶆墽琛?HPA 瑙ｇ爜鏉ョ‘瀹?DPA銆?

骞冲彴鎻愪緵涓€涓敤浜?CXL DPA 鍒扮郴缁熺墿鐞嗗湴鍧€杞崲鐨?PRM 澶勭悊绋嬪簭銆傝 PRM 澶勭悊绋嬪簭灏?
鎸囧畾 CXL 绔偣鐨勮澶囩墿鐞嗗湴鍧€锛圖PA锛夎浆鎹负绯荤粺鐗╃悊鍦板潃锛圫PA锛夈€傚湪涓绘満鐨勫湴鍧€绌洪棿涓紝
SPA 涓?HPA 鏄瓑浠风殑锛屾搷浣滅郴缁熷簲浣跨敤璇ュ鐞嗙▼搴忔潵纭畾涓庤澶囧湴鍧€瀵瑰簲鐨?HPA锛屼緥濡?
鍦ㄩ厤缃簡褰掍竴鍖栧湴鍧€鐨勫钩鍙颁笂閰嶇疆 HDM 瑙ｇ爜鍣ㄦ椂銆傚鐞嗙▼搴忕殑 GUID 涓庡弬鏁扮紦鍐插尯鏍煎紡
鍦?9.18.4.1 鑺備腑瑙勫畾銆傚鏋滄搷浣滅郴缁熻瘑鍒嚭璇?PRM 澶勭悊绋嬪簭锛屽垯璇存槑骞冲彴鏀寔褰掍竴鍖?
鍦板潃锛屼笖鎿嶄綔绯荤粺蹇呴』鍦ㄩ渶瑕佹椂鎵ц DPA 鍦板潃杞崲銆傗€?

**9.18.4.1 PRM 澶勭悊绋嬪簭璋冪敤**

鈥滄搷浣滅郴缁熶娇鐢ㄧ洿鎺ヨ皟鐢ㄦ満鍒舵潵璋冪敤 CXL DPA 鍒扮郴缁熺墿鐞嗗湴鍧€杞崲鐨?PRM 澶勭悊绋嬪簭銆?
璋冪敤 PRM 澶勭悊绋嬪簭鐨勭粏鑺傚湪骞冲彴杩愯鏃舵満鍒讹紙PRM锛夎鑼冧腑鎻忚堪銆?

璇?PRM 澶勭悊绋嬪簭鐢变互涓?GUID 鏍囪瘑锛?

 EE41B397-25D4-452C-AD54-48C6E3480B94

璋冪敤鑰呭垎閰嶅苟鍑嗗涓€涓弬鏁扮紦鍐插尯锛岀劧鍚庝紶鍏?PRM 澶勭悊绋嬪簭 GUID 涓庢寚鍚戝弬鏁扮紦鍐插尯鐨?
鎸囬拡鏉ヨ皟鐢ㄨ澶勭悊绋嬪簭銆傚弬鏁扮紦鍐插尯鍦ㄨ〃 9-32 涓弿杩般€傗€?

**琛?9-32. 鐢ㄤ簬 CXL DPA 鍒扮郴缁熺墿鐞嗗湴鍧€杞崲鐨?PRM 鍙傛暟缂撳啿鍖?*
锛堢敤浜?CXL DPA 鍒扮郴缁熺墿鐞嗗湴鍧€杞崲鐨?PRM 鍙傛暟缂撳啿鍖猴級

+-------------+-----------+------------------------------------------------------------------------+
| Byte Offset | Length in | Description                                                            |
| 锛堝瓧鑺傚亸绉伙級|   Bytes   | 锛堟弿杩帮級                                                               |
+=============+===========+========================================================================+
| 00h         | 8         | **CXL Device Physical Address (DPA)**锛欳XL DPA锛堜緥濡傛潵鑷?            |
|             |           | CXL Component Event Log锛?                                             |
+-------------+-----------+------------------------------------------------------------------------+
| 08h         | 4         | **CXL Endpoint SBDF**锛?                                                |
|             |           |                                                                        |
|             |           | - Byte 3 - PCIe Segment锛圥CIe 娈碉級                                      |
|             |           | - Byte 2 - Bus Number锛堟€荤嚎鍙凤級                                         |
|             |           | - Byte 1:                                                              |
|             |           |          - Device Number Bits[7:3]锛堣澶囧彿浣嶏級                          |
|             |           |          - Function Number Bits[2:0]锛堝姛鑳藉彿浣嶏級                        |
|             |           | - Byte 0 - RESERVED (MBZ)锛堜繚鐣欙級                                       |
|             |           |                                                                        |
+-------------+-----------+------------------------------------------------------------------------+
| 0Ch         | 8         | **Output Buffer**锛氭寚鍚戠紦鍐插尯鐨勮櫄鎷熷湴鍧€鎸囬拡锛?                         |
|             |           | 濡?Table 9-33 鎵€瀹氫箟銆?                                                 |
+-------------+-----------+------------------------------------------------------------------------+

**琛?9-33. 鐢ㄤ簬 CXL DPA 鍒扮郴缁熺墿鐞嗗湴鍧€杞崲鐨?PRM 杈撳嚭缂撳啿鍖?*
锛堢敤浜?CXL DPA 鍒扮郴缁熺墿鐞嗗湴鍧€杞崲鐨?PRM 杈撳嚭缂撳啿鍖猴級

+-------------+-----------+------------------------------------------------------------------------+
| Byte Offset | Length in | Description                                                            |
| 锛堝瓧鑺傚亸绉伙級|   Bytes   | 锛堟弿杩帮級                                                               |
+=============+===========+========================================================================+
| 00h         | 8         | **System Physical Address (SPA)**锛氫粠 CXL DPA 杞崲鑰屾潵鐨?SPA銆?       |
|             |           |                                                                        |
+-------------+-----------+------------------------------------------------------------------------+
