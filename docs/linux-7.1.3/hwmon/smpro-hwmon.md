
## 鍐呮牳椹卞姩 Ampere(R) Altra(R) SMpro hwmon


鏀寔鐨勮姱鐗囷細

  - Ampere(R) Altra(R)

    Prefix锛堝墠缂€锛? `smpro`

    鍙傝€? `Altra SoC BMC Interface Specification`

浣滆€? Thu Nguyen <thu@os.amperecomputing.com>

### 鎻忚堪

smpro-hwmon 椹卞姩鍩轰簬 SMpro 鍗忓鐞嗗櫒锛圫Mpro锛変负 Ampere(R) Altra(R) SoC 鎻愪緵纭欢鐩戞帶鏀寔銆傝椹卞姩鏀寔浠ヤ笅浼犳劅鍣ㄦ寚鏍囷細

  - 娓╁害锛坱emperature锛?  - 鐢靛帇锛坴oltage锛?  - 鐢垫祦锛坈urrent锛?  - 鍔熺巼锛坧ower锛?
璇ユ帴鍙ｆ彁渚涚敤浜庢煡璇㈠悇绉嶄紶鎰熷櫒鍙婂叾鍊肩殑瀵勫瓨鍣紝鐒跺悗鐢辨湰椹卞姩瀵煎嚭鍒扮敤鎴风┖闂淬€?
### 浣跨敤璇存槑


璇ラ┍鍔ㄤ负姣忎釜浼犳劅鍣ㄨ嚦灏戝垱寤轰袱涓?sysfs 鏂囦欢銆?
- `<sensor_type><idx>_label` 鎶ュ憡浼犳劅鍣ㄦ爣绛俱€?- `<sensor_type><idx>_input` 杩斿洖浼犳劅鍣ㄥ€笺€?
sysfs 鏂囦欢鍒嗛厤鍦?SMpro 鏍圭洰褰曟枃浠跺す涓紝姣忎釜瀹炰緥瀵瑰簲涓€涓牴鐩綍銆?
褰?SoC 鍏抽棴鏃讹紝椹卞姩璇诲彇瀵勫瓨鍣ㄤ細澶辫触骞惰繑鍥?`-ENXIO`銆?
### Sysfs 鏉＄洰


鏀寔浠ヤ笅 sysfs 鏂囦欢锛?
- Ampere(R) Altra(R)锛?
  ============    =============  ======  ===============================================
  Name            Unit           Perm    璇存槑
  ============    =============  ======  ===============================================
  temp1_input     millicelsius   RO      SoC 娓╁害
  temp2_input     millicelsius   RO      SoC VRD 涓姤鍛婄殑鏈€楂樻俯搴?  temp2_crit      millicelsius   RO      SoC VRD HOT 闃堝€兼俯搴?  temp3_input     millicelsius   RO      DIMM VRD 涓姤鍛婄殑鏈€楂樻俯搴?  temp4_input     millicelsius   RO      Core VRD 涓姤鍛婄殑鏈€楂樻俯搴?  temp5_input     millicelsius   RO      CH0 涓?DIMM0 鐨勬俯搴?  temp5_crit      millicelsius   RO      鎵€鏈?DIMM 鐨?MEM HOT 闃堝€?  temp6_input     millicelsius   RO      CH1 涓?DIMM0 鐨勬俯搴?  temp6_crit      millicelsius   RO      鎵€鏈?DIMM 鐨?MEM HOT 闃堝€?  temp7_input     millicelsius   RO      CH2 涓?DIMM0 鐨勬俯搴?  temp7_crit      millicelsius   RO      鎵€鏈?DIMM 鐨?MEM HOT 闃堝€?  temp8_input     millicelsius   RO      CH3 涓?DIMM0 鐨勬俯搴?  temp8_crit      millicelsius   RO      鎵€鏈?DIMM 鐨?MEM HOT 闃堝€?  temp9_input     millicelsius   RO      CH4 涓?DIMM0 鐨勬俯搴?  temp9_crit      millicelsius   RO      鎵€鏈?DIMM 鐨?MEM HOT 闃堝€?  temp10_input    millicelsius   RO      CH5 涓?DIMM0 鐨勬俯搴?  temp10_crit     millicelsius   RO      鎵€鏈?DIMM 鐨?MEM HOT 闃堝€?  temp11_input    millicelsius   RO      CH6 涓?DIMM0 鐨勬俯搴?  temp11_crit     millicelsius   RO      鎵€鏈?DIMM 鐨?MEM HOT 闃堝€?  temp12_input    millicelsius   RO      CH7 涓?DIMM0 鐨勬俯搴?  temp12_crit     millicelsius   RO      鎵€鏈?DIMM 鐨?MEM HOT 闃堝€?  temp13_input    millicelsius   RO      RCA VRD 涓姤鍛婄殑鏈€楂樻俯搴?  in0_input       millivolts     RO      Core 鐢靛帇
  in1_input       millivolts     RO      SoC 鐢靛帇
  in2_input       millivolts     RO      DIMM VRD1 鐢靛帇
  in3_input       millivolts     RO      DIMM VRD2 鐢靛帇
  in4_input       millivolts     RO      RCA VRD 鐢靛帇
  cur1_input      milliamperes   RO      Core VRD 鐢垫祦
  cur2_input      milliamperes   RO      SoC VRD 鐢垫祦
  cur3_input      milliamperes   RO      DIMM VRD1 鐢垫祦
  cur4_input      milliamperes   RO      DIMM VRD2 鐢垫祦
  cur5_input      milliamperes   RO      RCA VRD 鐢垫祦
  power1_input    microwatts     RO      Core VRD 鍔熺巼
  power2_input    microwatts     RO      SoC VRD 鍔熺巼
  power3_input    microwatts     RO      DIMM VRD1 鍔熺巼
  power4_input    microwatts     RO      DIMM VRD2 鍔熺巼
  power5_input    microwatts     RO      RCA VRD 鍔熺巼
  ============    =============  ======  ===============================================

```

    # cat in0_input
    830
    # cat temp1_input
    37000
    # cat curr1_input
    9000
    # cat power5_input
    19500000

```
