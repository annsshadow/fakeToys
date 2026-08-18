
## CXL 璁块棶鍧愭爣璁＄畻


## 寤惰繜涓庡甫瀹借绠?

涓€涓唴瀛樺尯鍩熺殑鎬ц兘鍧愭爣锛堝欢杩熷拰甯﹀锛夐€氬父閫氳繃 ACPI 琛?[SRAT <../platform/acpi/srat>](SRAT <../platform/acpi/srat>) 鍜?[HMAT <../platform/acpi/hmat>](HMAT <../platform/acpi/hmat>) 鎻愪緵銆傜劧鑰岋紝骞冲彴鍥轰欢锛圔IOS锛夋棤娉曚负鐑彃鎷旂殑 CXL 璁惧鏍囨敞杩欎簺淇℃伅锛屽洜涓哄畠浠湪骞冲彴鍥轰欢鍒濆鍖栨湡闂村苟涓嶅瓨鍦ㄣ€侰XL 椹卞姩鍙互閫氳繃浠庡涓粍浠舵绱㈡暟鎹潵璁＄畻鎬ц兘鍧愭爣銆?
[SRAT <../platform/acpi/srat>](SRAT <../platform/acpi/srat>) 鎻愪緵浜嗕竴涓?Generic Port Affinity 瀛愯〃锛屽畠鎶婁竴涓?proximity domain 缁戝畾鍒颁竴涓澶囧彞鏌勶紝鍦ㄦ鎯呭喌涓嬪嵆涓?CXL hostbridge銆傚埄鐢ㄨ繖绉嶅叧鑱旓紝鍙互浠?[HMAT <../platform/acpi/hmat>](HMAT <../platform/acpi/hmat>) 瀛愯〃妫€绱?Generic Port 鐨勬€ц兘鍧愭爣銆傝繖涓€閮ㄥ垎琛ㄧず浠?CPU 鍒?Generic Port锛圕XL hostbridge锛変箣闂寸殑鎬ц兘鍧愭爣銆?
[CDAT <../platform/cdat>](CDAT <../platform/cdat>) 鎻愪緵 CXL 璁惧鏈韩鐨勬€ц兘鍧愭爣銆傚嵆璁块棶璇ヨ澶囧唴瀛樺尯鍩熺殑甯﹀鍜屽欢杩熴€侱SMAS 瀛愯〃鎻愪緵涓€涓笌璁惧鐗╃悊鍦板潃锛圖PA锛夎寖鍥寸粦瀹氱殑 DSMADHandle銆侱SLBIS 瀛愯〃鎻愪緵涓?DSMADhandle 缁戝畾鐨勬€ц兘鍧愭爣锛岃繖鎶婁袱涓〃椤硅仈绯诲湪涓€璧凤紝涓烘瘡涓?DPA 鍖哄煙鎻愪緵鎬ц兘鍧愭爣銆備緥濡傦紝濡傛灉涓€涓澶囧鍑轰簡涓€涓?DRAM 鍖哄煙鍜屼竴涓?PMEM 鍖哄煙锛岄偅涔堣繖浜涘尯鍩熶腑鐨勬瘡涓€涓兘浼氭湁涓嶅悓鎬ц兘鐗瑰緛銆?
濡傛灉鎷撴墤涓湁涓€涓?CXL 浜ゆ崲鏈猴紝鍒欎氦鎹㈡満鐨勬€ц兘鍧愭爣鐢?SSLBIS 瀛愯〃鎻愪緵銆傚畠鎻愪緵鍦ㄤ氦鎹㈡満涓婃父绔彛锛坲pstream port锛夊拰鎸囧悜绔偣璁惧鐨勪氦鎹㈡満涓嬫父绔彛锛坉ownstream port锛変箣闂寸┛瓒婁氦鎹㈡満鐨勫甫瀹藉拰寤惰繜銆?
```

 GP0/HB0/ACPI0016-0
        RP0
         |
         | L0
         |
     SW 0 / USP0
     SW 0 / DSP0
         |
         | L1
         |
        EP0

```
鍦ㄦ绀轰緥涓紝绔偣涓庢牴绔彛涔嬮棿鏈変竴涓?CXL 浜ゆ崲鏈恒€傛绀轰緥涓殑寤惰繜璁＄畻濡備笅锛?L(EP0) - 鏉ヨ嚜 EP0 CDAT DSMAS+DSLBIS 鐨勫欢杩?L(L1) - EP0 涓?SW0DSP0 涔嬮棿鐨勯摼璺欢杩?L(SW0) - 鏉ヨ嚜 SW0 CDAT SSLBIS 鐨勪氦鎹㈡満寤惰繜銆?L(L0) - SW0 涓?RP0 涔嬮棿鐨勯摼璺欢杩?L(RP0) - 缁忕敱 SRAT 鍜?HMAT锛圙eneric Port锛変粠鏍圭鍙ｅ埌 CPU 鐨勫欢杩熴€?鎬昏銆佸啓寤惰繜鏄笂杩版墍鏈夐儴鍒嗕箣鍜屻€?
姝ょず渚嬩腑鐨勫甫瀹借绠楀涓嬶細
B(EP0) - 鏉ヨ嚜 EP0 CDAT DSMAS+DSLBIS 鐨勫甫瀹?B(L1) - EP0 涓?SW0DSP0 涔嬮棿鐨勯摼璺甫瀹?B(SW0) - 鏉ヨ嚜 SW0 CDAT SSLBIS 鐨勪氦鎹㈡満甯﹀銆?B(L0) - SW0 涓?RP0 涔嬮棿鐨勯摼璺甫瀹?B(RP0) - 缁忕敱 SRAT 鍜?HMAT锛圙eneric Port锛変粠鏍圭鍙ｅ埌 CPU 鐨勫甫瀹姐€?鎬昏銆佸啓甯﹀鏄笂杩版墍鏈夐儴鍒嗙殑鏈€灏忓€硷紙min()锛夈€?
瑕佽绠楅摼璺甫瀹斤細
LinkOperatingFrequency (GT/s) 鏄綋鍓嶅崗鍟嗙殑閾捐矾閫熷害銆?DataRatePerLink (MB/s) = LinkOperatingFrequency / 8
Bandwidth (MB/s) = PCIeCurrentLinkWidth * DataRatePerLink
鍏朵腑 PCIeCurrentLinkWidth 鏄摼璺腑鐨勯€氶亾鏁般€?
瑕佽绠楅摼璺欢杩燂細
LinkLatency (picoseconds) = FlitSize / LinkBandwidth (MB/s)

缁嗚妭璇峰弬瑙?`CXL Memory Device SW Guide r1.0 <https://www.intel.com/content/www/us/en/content-details/643805/cxl-memory-device-software-guide.html>`_ 绗?2.11.3 鍜?2.11.4 鑺傘€?
鏈€缁堬紝鎵€鏋勯€犲唴瀛樺尯鍩熺殑璁块棶鍧愭爣鐢变竴涓垨澶氫釜 CXL 璁惧鐨勫悇涓唴瀛樺垎鍖鸿绠楀緱鍑恒€?
## 鍏变韩涓婃父閾捐矾璁＄畻


瀵逛簬鏌愪簺绔偣浣嶄簬 CXL 浜ゆ崲鏈猴紙SW锛夋垨鏍圭鍙ｏ紙RP锛変箣鍚庣殑 CXL 鍖哄煙鏋勯€狅紝鎵€鏈変綅浜庝氦鎹㈡満涔嬪悗鐨勭鐐圭殑鎬诲甫瀹芥湁鍙兘瓒呰繃浜ゆ崲鏈轰笂娓搁摼璺€傚湪涓绘満鍐呴儴銆佹牴绔彛涓婃父涔熷彲鑳藉嚭鐜扮被浼兼儏鍐点€侰XL 椹卞姩鍦ㄦ墍鏈夌洰鏍囬兘宸插埌杈炬煇涓尯鍩熷悗锛屼細鎵ц涓€涓澶栫殑閬嶅巻锛屼互渚垮湪鑰冭檻涓婃父閾捐矾鍙兘鎴愪负闄愬埗鍥犵礌鐨勬儏鍐典笅閲嶆柊璁＄畻甯﹀銆?
璇ョ畻娉曞亣璁鹃厤缃槸瀵圭О鎷撴墤锛屽洜涓鸿繖鏍疯兘鏈€澶у寲鎬ц兘銆傚綋妫€娴嬪埌闈炲绉版嫇鎵戞椂锛岃绠椾腑姝€傞潪瀵圭О鎷撴墤鏄湪鎷撴墤閬嶅巻鏈熼棿妫€娴嬪埌鐨勶細妫€娴嬩负绁栫埗鑺傜偣鐨?RP 鏁伴噺涓嶇瓑浜庡湪鍚屼竴閬嶅巻寰幆涓凯浠ｇ殑璁惧鏁伴噺銆傚叾鍋囪鏄睘鎬т笂缁嗗井鐨勪笉瀵圭О涓嶄細鍙戠敓锛屼笖鍒?EP 鐨勬墍鏈夎矾寰勯兘鐩哥瓑銆?
涓€涓?RP 涓嬪彲浠ユ湁澶氫釜浜ゆ崲鏈恒€備竴涓?CXL Host Bridge锛圚B锛変笅鍙互鏈夊涓?RP銆備竴涓?[CEDT <../platform/acpi/cedt>](CEDT <../platform/acpi/cedt>) 涓殑 CXL Fixed Memory Window Structure锛圕FMWS锛変笅鍙互鏈夊涓?HB銆?
```

                CFMWS 0
                  |
         _________|_________
        |                   |
    ACPI0017-0          ACPI0017-1
 GP0/HB0/ACPI0016-0   GP1/HB1/ACPI0016-1
    |          |        |           |
   RP0        RP1      RP2         RP3
    |          |        |           |
  SW 0       SW 1     SW 2        SW 3
  |   |      |   |    |   |       |   |
 EP0 EP1    EP2 EP3  EP4  EP5    EP6 EP7

```
绀轰緥灞傛缁撴瀯鐨勮绠楋細

Min (GP0 to CPU BW,
     Min(SW 0 Upstream Link to RP0 BW,
         Min(SW0SSLBIS for SW0DSP0 (EP0), EP0 DSLBIS, EP0 Upstream Link) +
         Min(SW0SSLBIS for SW0DSP1 (EP1), EP1 DSLBIS, EP1 Upstream link)) +
     Min(SW 1 Upstream Link to RP1 BW,
         Min(SW1SSLBIS for SW1DSP0 (EP2), EP2 DSLBIS, EP2 Upstream Link) +
         Min(SW1SSLBIS for SW1DSP1 (EP3), EP3 DSLBIS, EP3 Upstream link))) +
Min (GP1 to CPU BW,
     Min(SW 2 Upstream Link to RP2 BW,
         Min(SW2SSLBIS for SW2DSP0 (EP4), EP4 DSLBIS, EP4 Upstream Link) +
         Min(SW2SSLBIS for SW2DSP1 (EP5), EP5 DSLBIS, EP5 Upstream link)) +
     Min(SW 3 Upstream Link to RP3 BW,
         Min(SW3SSLBIS for SW3DSP0 (EP6), EP6 DSLBIS, EP6 Upstream Link) +
         Min(SW3SSLBIS for SW3DSP1 (EP7), EP7 DSLBIS, EP7 Upstream link))))

璁＄畻浠?cxl_region_shared_upstream_perf_update() 寮€濮嬨€傚垱寤轰竴涓?xarray 鏉ラ€氳繃 cxl_endpoint_gather_bandwidth() 鍑芥暟鏀堕泦鎵€鏈夌鐐瑰甫瀹姐€傝绠楁潵鑷鐐?CDAT 鐨勫甫瀹戒笌涓婃父閾捐矾甯﹀鐨勬渶灏忓€硷紙min()锛夈€傚鏋滅鐐圭殑鐖惰妭鐐规槸涓€涓?CXL 浜ゆ崲鏈猴紝鍒欒绠楀甫瀹戒笌鍏宠仈鍒拌绔偣鐨勪氦鎹㈡満涓嬫父绔彛鐨?SSLBIS 甯﹀鐨勬渶灏忓€笺€傛渶缁堝甫瀹藉瓨鍌ㄥ湪 xarray 涓敱璁惧鎸囬拡绱㈠紩鐨?鈥榮truct cxl_perf_ctx鈥?閲屻€傚鏋滅鐐圭洿鎺ユ寕鎺ュ埌鏍圭鍙ｏ紙RP锛夛紝璁惧鎸囬拡灏嗘槸涓€涓?RP 璁惧銆傚鏋滅鐐逛綅浜庝氦鎹㈡満涔嬪悗锛岃澶囨寚閽堝皢鏄埗浜ゆ崲鏈虹殑涓婃父璁惧銆?
鍦ㄤ笅涓€涓樁娈碉紝浠ｇ爜閬嶅巻鎷撴墤涓竴涓垨澶氫釜锛堝鏋滃瓨鍦級浜ゆ崲鏈恒€傚浜庣洿鎺ユ寕鎺ュ埌 RP 鐨勭鐐癸紝璺宠繃姝ゆ銆傚鏋滀笂娓歌繕鏈夊彟涓€涓氦鎹㈡満锛屼唬鐮佸彇褰撳墠鏀堕泦鍒扮殑甯﹀涓庝笂娓搁摼璺甫瀹界殑鏈€灏忓€笺€傚鏋滀笂娓告湁浜ゆ崲鏈猴紝鍒欏彇涓婃父浜ゆ崲鏈虹殑 SSLBIS銆?
涓€鏃︽嫇鎵戦亶鍘嗗埌杈?RP锛堟棤璁烘槸鐩存帴鎸傛帴鐨勭鐐癸紝杩樻槸缁忕敱浜ゆ崲鏈洪亶鍘嗭級锛屽氨浼氳皟鐢?cxl_rp_gather_bandwidth()銆傛鏃舵墍鏈夊甫瀹芥寜姣忎釜 host bridge 鑱氬悎锛岃繖涔熸槸缁撴灉 xarray 鐨勭储寮曘€?
涓嬩竴姝ユ槸鍙栨瘡涓?host bridge 鐨勫甫瀹戒笌 Generic Port锛圙P锛夊甫瀹界殑鏈€灏忓€笺€侴P 鐨勫甫瀹介€氳繃 ACPI 琛紙[SRAT <../platform/acpi/srat>](SRAT <../platform/acpi/srat>) 鍜?[HMAT <../platform/acpi/hmat>](HMAT <../platform/acpi/hmat>)锛夋绱€傛渶灏忓甫瀹藉湪鍚屼竴涓?ACPI0017 璁惧涓嬭仛鍚堬紝褰㈡垚涓€涓柊鐨?xarray銆?
鏈€鍚庯紝璋冪敤 cxl_region_update_bandwidth()锛屽苟灏嗘渶鍚庝竴涓?xarray 涓墍鏈夋垚鍛樼殑鑱氬悎甯﹀鏇存柊鍒伴┗鐣欏湪 cxl 鍖哄煙锛坈xlr锛変笂涓嬫枃涓殑璁块棶鍧愭爣銆?
## QTG ID


姣忎釜 [CEDT <../platform/acpi/cedt>](CEDT <../platform/acpi/cedt>) 閮芥湁涓€涓?QTG ID 瀛楁銆傝瀛楁鎻愪緵涓?CFMWS 绐楀彛鐨?QoS Throttling Group锛圦TG锛夊叧鑱旂殑 ID銆備竴鏃﹁绠楀嚭璁块棶鍧愭爣锛屽氨鍙互鍚?ACPI0016 璁惧鍙戝嚭涓€涓?ACPI Device Specific Method锛屼互鏍规嵁鎵€鎻愪緵鐨勮闂潗鏍囨绱?QTG ID銆傝澶囩殑 QTG ID 鍙敤浣滃尮閰?CFMWS 鐨勬寚寮曪紝浠ヤ究涓鸿澶囨€ц兘璁剧疆鏈€浣崇殑 Linux 鏍硅В鐮佸櫒銆?