
## Rmnet 椹卞姩


## 1. 绠€浠?

rmnet 椹卞姩鐢ㄤ簬鏀寔澶嶇敤涓庤仛鍚堝崗璁紙MAP锛孧ultiplexing and aggregation Protocol锛夈€?璇ュ崗璁鎵€鏈変娇鐢?Qualcomm Technologies, Inc. 璋冨埗瑙ｈ皟鍣ㄧ殑鏈€鏂拌姱鐗囩粍鎵€閲囩敤銆?
鏈┍鍔ㄥ彲鐢ㄤ簬娉ㄥ唽鍒颁换鎰忓浜?IP 妯″紡鐨勭墿鐞嗙綉缁滆澶囥€傜墿鐞嗕紶杈撳寘鎷?USB銆丠SIC銆?PCIe 鍜?IP accelerator銆?
澶嶇敤鍏佽鍒涘缓閫昏緫缃戠粶璁惧锛坮mnet 璁惧锛夋潵澶勭悊澶氫釜绉佹湁鏁版嵁缃戠粶锛圥DN锛夛紝渚嬪榛樿
浜掕仈缃戙€佺綉缁滃叡浜紙tethering锛夈€佸僵淇℃湇鍔★紙MMS锛夋垨 IP 濯掍綋瀛愮郴缁燂紙IMS锛夈€傜‖浠跺皢
甯︽湁 MAP 澶寸殑鍖呭彂閫佺粰 rmnet銆俽mnet 鏍规嵁澶嶇敤鍣?id锛屽湪鍘婚櫎 MAP 澶村悗灏嗗寘璺敱鍒?鐩稿簲鐨?PDN銆?
瑕佽揪鍒伴珮鏁版嵁閫熺巼闇€瑕佽仛鍚堛€傝繖娑夊強纭欢鍙戦€佽仛鍚堢殑涓€鎵?MAP 甯с€俽mnet 椹卞姩浼氬皢杩欎簺
MAP 甯у幓鑱氬悎锛屽苟鍙戦€佸埌鐩稿簲鐨?PDN銆?
## 2. 鍖呮牸寮?

### a. MAP packet v1锛堟暟鎹?/ 鎺у埗锛?

MAP 澶村瓧娈甸噰鐢ㄥぇ绔紙big endian锛夋牸寮忋€?
```

  Bit             0             1           2-7      8-15           16-31
  Function   Command / Data   Reserved     Pad   Multiplexer ID    Payload length

  Bit            32-x
  Function      Raw bytes

```
Command (1)/ Data (0) 浣嶇敤浜庢寚绀鸿鍖呮槸 MAP 鍛戒护鍖呰繕鏄暟鎹寘銆傚懡浠ゅ寘鐢ㄤ簬浼犺緭灞傛祦鎺с€?鏁版嵁鍖呮槸鏍囧噯 IP 鍖呫€?
淇濈暀浣嶅彂閫佹椂蹇呴』涓洪浂锛屾帴鏀舵椂蹇界暐銆?
Padding 鏄拷鍔犲埌杞借嵎鏈熬浠ョ‘淇?4 瀛楄妭瀵归綈鐨勫瓧鑺傛暟銆?
Multiplexer ID 鐢ㄤ簬鎸囩ず鏁版嵁瑕佸彂閫佸埌鐨?PDN銆?
杞借嵎闀垮害鍖呭惈 padding 闀垮害锛屼絾涓嶅寘鍚?MAP 澶撮暱搴︺€?
### b. Map packet v4锛堟暟鎹?/ 鎺у埗锛?

MAP 澶村瓧娈甸噰鐢ㄥぇ绔牸寮忋€?
```

  Bit             0             1           2-7      8-15           16-31
  Function   Command / Data   Reserved     Pad   Multiplexer ID    Payload length

  Bit            32-(x-33)      (x-32)-x
  Function      Raw bytes      Checksum offload header

```
Command (1)/ Data (0) 浣嶇敤浜庢寚绀鸿鍖呮槸 MAP 鍛戒护鍖呰繕鏄暟鎹寘銆傚懡浠ゅ寘鐢ㄤ簬浼犺緭灞傛祦鎺с€?鏁版嵁鍖呮槸鏍囧噯 IP 鍖呫€?
淇濈暀浣嶅彂閫佹椂蹇呴』涓洪浂锛屾帴鏀舵椂蹇界暐銆?
Padding 鏄拷鍔犲埌杞借嵎鏈熬浠ョ‘淇?4 瀛楄妭瀵归綈鐨勫瓧鑺傛暟銆?
Multiplexer ID 鐢ㄤ簬鎸囩ず鏁版嵁瑕佸彂閫佸埌鐨?PDN銆?
杞借嵎闀垮害鍖呭惈 padding 闀垮害锛屼絾涓嶅寘鍚?MAP 澶撮暱搴︺€?
Checksum offload 澶村寘鍚‖浠舵墍瀹屾垚鏍￠獙鍜屽鐞嗙殑淇℃伅銆侰hecksum offload 澶村瓧娈甸噰鐢?澶х鏍煎紡銆?
```

  Bit             0-14        15              16-31
  Function      Reserved   Valid     Checksum start offset

  Bit                31-47                    48-64
  Function      Checksum length           Checksum value

```
淇濈暀浣嶅彂閫佹椂蹇呴』涓洪浂锛屾帴鏀舵椂蹇界暐銆?
Valid 浣嶆寚绀洪儴鍒嗘牎楠屽拰鏄惁宸茶璁＄畻涓旀湁鏁堛€傝嫢鏈夋晥鍒欑疆 1锛屽惁鍒欑疆 0銆?
Padding 鏄拷鍔犲埌杞借嵎鏈熬浠ョ‘淇?4 瀛楄妭瀵归綈鐨勫瓧鑺傛暟銆?
Checksum start offset锛堟牎楠屽拰璧峰鍋忕Щ锛夋寚绀轰粠 IP 澶磋捣濮嬪璧风殑瀛楄妭鍋忕Щ锛岃皟鍒惰В璋冨櫒
浠庤鍋忕Щ寮€濮嬭绠楁牎楠屽拰銆?
Checksum length锛堟牎楠屽拰闀垮害锛夋槸浠?CKSUM_START_OFFSET 璧峰鐨勩€佽绠椾簡鏍￠獙鍜岀殑瀛楄妭
闀垮害銆?
Checksum value锛堟牎楠屽拰鍊硷級鎸囩ず璁＄畻寰楀埌鐨勬牎楠屽拰銆?
### c. MAP packet v5锛堟暟鎹?/ 鎺у埗锛?

MAP 澶村瓧娈甸噰鐢ㄥぇ绔牸寮忋€?
```

  Bit             0             1         2-7      8-15           16-31
  Function   Command / Data  Next header  Pad   Multiplexer ID   Payload length

  Bit            32-x
  Function      Raw bytes

```
Command (1)/ Data (0) 浣嶇敤浜庢寚绀鸿鍖呮槸 MAP 鍛戒护鍖呰繕鏄暟鎹寘銆傚懡浠ゅ寘鐢ㄤ簬浼犺緭灞傛祦鎺с€?鏁版嵁鍖呮槸鏍囧噯 IP 鍖呫€?
Next header 鐢ㄤ簬鎸囩ず鏄惁瀛樺湪鍙︿竴涓ご锛岀洰鍓嶄粎闄愪簬鏍￠獙鍜屽ご銆?
Padding 鏄拷鍔犲埌杞借嵎鏈熬浠ョ‘淇?4 瀛楄妭瀵归綈鐨勫瓧鑺傛暟銆?
Multiplexer ID 鐢ㄤ簬鎸囩ず鏁版嵁瑕佸彂閫佸埌鐨?PDN銆?
杞借嵎闀垮害鍖呭惈 padding 闀垮害锛屼絾涓嶅寘鍚?MAP 澶撮暱搴︺€?
### d. Checksum offload header v5


Checksum offload 澶村瓧娈甸噰鐢ㄥぇ绔牸寮忋€?
```

  Bit            0 - 6          7               8-15              16-31
  Function     Header Type    Next Header     Checksum Valid    Reserved

```
Header Type 鐢ㄤ簬鎸囩ず澶寸殑绫诲瀷锛岄€氬父璁句负 CHECKSUM

Header types

= ===============
0 Reserved
1 Reserved
2 checksum header
= ===============

Checksum Valid 鐢ㄤ簬鎸囩ず璇ュご鏍￠獙鍜屾槸鍚︽湁鏁堛€傚€间负 1 琛ㄧず宸插鏈寘璁＄畻鏍￠獙鍜屼笖鏈夋晥锛?鍊间负 0 琛ㄧず璁＄畻寰楀埌鐨勫寘鏍￠獙鍜屾棤鏁堛€?
淇濈暀浣嶅彂閫佹椂蹇呴』涓洪浂锛屾帴鏀舵椂蹇界暐銆?
### e. MAP packet v1/v5锛堝懡浠ょ浉鍏筹級


```

    Bit             0             1         2-7      8 - 15           16 - 31
    Function   Command         Reserved     Pad   Multiplexer ID    Payload length
    Bit          32 - 39        40 - 45    46 - 47       48 - 63
    Function   Command name    Reserved   Command Type   Reserved
    Bit          64 - 95
    Function   Transaction ID
    Bit          96 - 127
    Function   Command data

```
鍛戒护 1 琛ㄧず绂佺敤娴佹帶锛岃€?2 琛ㄧず鍚敤娴佹帶銆?
Command types

= ==========================================
0 for MAP command request
1 is to acknowledge the receipt of a command
2 is for unsupported commands
3 is for error during processing of commands
= ==========================================

### f. 鑱氬悎


鑱氬悎鏄湪鍗曚釜绾挎€?skb 涓紶閫掔粰 rmnet 鐨勫涓?MAP 鍖咃紙鍙互鏄暟鎹垨鍛戒护锛夈€俽mnet 浼?澶勭悊鍚勪釜鍖咃紝骞跺 MAP 鍛戒护杩涜 ACK锛屾垨灏?IP 鍖呮寜闇€閫掍氦缁欑綉缁滄爤銆?
```

  MAP header|IP Packet|Optional padding|MAP header|IP Packet|Optional padding....

  MAP header|IP Packet|Optional padding|MAP header|Command Packet|Optional pad...

```
## 3. 鐢ㄦ埛绌洪棿閰嶇疆


rmnet 鐨勭敤鎴风┖闂撮厤缃€氳繃 netlink 浣跨敤 iproute2 瀹屾垚
https://git.kernel.org/pub/scm/network/iproute2/iproute2.git/

椹卞姩浣跨敤 rtnl_link_ops 杩涜閫氫俊銆?