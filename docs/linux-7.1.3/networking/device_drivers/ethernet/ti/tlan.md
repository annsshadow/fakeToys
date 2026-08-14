## Linux 鐨?TLAN 椹卞姩


:Version: 1.14a

(C) 1997-1998 Caldera, Inc.

(C) 1998 James Banks

(C) 1999-2001 Torben Mathiasen <tmm@image.dk, torben.mathiasen@compaq.com>

椹卞姩淇℃伅/鏇存柊璇疯闂?http://www.compaq.com


## I. 鏀寔鐨勮澶?

    鍙湁 PCI 璁惧鑳介厤鍚堣椹卞姩宸ヤ綔銆?
    鏀寔鐨勮澶囷細

    =========	=========	===========================================
    Vendor ID	Device ID	Name
    =========	=========	===========================================
    0e11	ae32		Compaq Netelligent 10/100 TX PCI UTP
    0e11	ae34		Compaq Netelligent 10 T PCI UTP
    0e11	ae35		Compaq Integrated NetFlex 3/P
    0e11	ae40		Compaq Netelligent Dual 10/100 TX PCI UTP
    0e11	ae43		Compaq Netelligent Integrated 10/100 TX UTP
    0e11	b011		Compaq Netelligent 10/100 TX Embedded UTP
    0e11	b012		Compaq Netelligent 10 T/2 PCI UTP/Coax
    0e11	b030		Compaq Netelligent 10/100 TX UTP
    0e11	f130		Compaq NetFlex 3/P
    0e11	f150		Compaq NetFlex 3/P
    108d	0012		Olicom OC-2325
    108d	0013		Olicom OC-2183
    108d	0014		Olicom OC-2326
    =========	=========	===========================================


    娉ㄦ剰浜嬮」锛?
    鎴戜笉纭畾 100BaseTX 瀛愭澘锛堥拡瀵归偅浜涙敮鎸佹绫绘墿灞曠殑鍗★級鏄惁鑳藉伐浣溿€傛垜娌℃湁浠讳綍鍙潬
    鐨勮瘉鎹兘琛ㄦ槑鍙互鎴栦笉鍙互銆?
    浣嗘槸锛屽鏋滀竴寮犲崱鏀寔 100BaseTx 鑰屾棤闇€棰濆鐨勫瓙鏉匡紝瀹冨簲褰撹兘鍦?100BaseTx 涓嬪伐浣溿€?
    鈥淣etelligent 10 T/2 PCI UTP/Coax鈥濓紙b012锛夎澶囨湭缁忔祴璇曪紝浣嗘垜涓嶈涓轰細鏈変换浣曢棶棰樸€?

## II. 椹卞姩閫夐」


 1. 浣犲彲浠ュ湪 insmod 鍛戒护琛屾湯灏捐拷鍔?debug=x 鏉ヨ幏鍙栬皟璇曚俊鎭紝鍏朵腑 x 鏄竴涓綅鍩燂紝鍚?	   浣嶅惈涔夊涓嬶細

	   ====		=====================================
	   0x01		寮€鍚€氱敤璋冭瘯淇℃伅銆?	   0x02		寮€鍚帴鏀惰皟璇曚俊鎭€?	   0x04		寮€鍚彂閫佽皟璇曚俊鎭€?	   0x08		寮€鍚摼琛ㄨ皟璇曚俊鎭€?	   ====		=====================================

 2. 浣犲彲浠ュ湪 insmod 鍛戒护琛屾湯灏捐拷鍔?aui=1锛屼娇閫傞厤鍣ㄤ娇鐢?AUI 鎺ュ彛鑰岄潪 10 Base T
	   鎺ュ彛銆傚鏋滀綘鎯冲湪鍩轰簬 TLAN 鐨勮澶囦笂浣跨敤 BNC 杩炴帴鍣紝涔熷簲杩欎箞鍋氥€傦紙鍦?	   娌℃湁 AUI/BNC 杩炴帴鍣ㄧ殑璁惧涓婅缃閫夐」鍙兘浼氬鑷村叾鏃犳硶姝ｅ父宸ヤ綔銆傦級

 3. 浣犲彲浠ヨ缃?duplex=1 寮哄埗鍗婂弻宸ワ紝璁剧疆 duplex=2 寮哄埗鍏ㄥ弻宸ャ€?
 4. 浣犲彲浠ヨ缃?speed=10 寮哄埗 10Mbs 鎿嶄綔锛岃缃?speed=100 寮哄埗 100Mbs 鎿嶄綔銆?	   锛堝鏋滀竴寮犲彧鏀寔 10Mbs 鐨勫崱琚己鍒惰繘鍏?100Mbs 妯″紡锛屾垜涓嶆竻妤氫細鍙戠敓浠€涔堛€傦級

 5. 浣犵幇鍦ㄥ繀椤诲悓鏃朵娇鐢?speed=X duplex=Y銆傚鏋滀綘鍙墽琛屸€渋nsmod tlan.o speed=100鈥濓紝
	   椹卞姩浼氳繘琛岃嚜鍔ㄥ崗鍟嗭紙Auto-Neg锛夈€傝寮哄埗涓€涓?10Mbps 鍗婂弻宸ラ摼璺紝鎵ц
	   鈥渋nsmod tlan.o speed=10 duplex=1鈥濄€?
 6. 濡傛灉椹卞姩琚紪鍏ュ唴鏍革紝浣犲彲浠ヤ娇鐢ㄧ 3 鍜岀 4 涓弬鏁板垎鍒缃?aui 鍜?debug銆備緥濡傦細
```

		ether=0,0,0x1,0x7,eth0

	   杩欏皢 aui 璁句负 0x1銆乨ebug 璁句负 0x7锛屽亣瀹?eth0 鏄竴涓彈鏀寔鐨?TLAN 璁惧銆?
	   绗笁涓瓧鑺備腑鐨勪綅鍒嗛厤濡備笅锛?
		====   ===============
		0x01   aui
		0x02   浣跨敤鍗婂弻宸?		0x04   浣跨敤鍏ㄥ弻宸?		0x08   浣跨敤 10BaseT
		0x10   浣跨敤 100BaseTx
		====   ===============

	   鍦ㄤ娇鐢ㄥ唴鏍稿弬鏁板己鍒堕€熺巼鏃讹紝浣犱篃闇€瑕佸悓鏃惰缃?speed 鍜?duplex銆?	   ether=0,0,0x12,0,eth0 灏嗗己鍒堕摼璺负 100Mbps 鍗婂弻宸ャ€?
	7. 濡傛灉浣犵殑绯荤粺涓湁澶氬潡 tlan 閫傞厤鍣紝浣犲彲浠ュ熀浜庢瘡鍧楅€傞厤鍣ㄤ娇鐢ㄤ笂杩伴€夐」銆傝寮哄埗
	   浣犵殑 eth1 閫傞厤鍣ㄤ负 100Mbit/HD 閾捐矾锛屼娇鐢?:

		insmod tlan speed=0,100 duplex=0,1

	   杩欐牱 eth0 灏嗕娇鐢ㄨ嚜鍔ㄥ崗鍟嗭紝eth1 灏嗚寮哄埗涓?100Mbit/HD銆傛敞鎰?tlan 椹卞姩鏈€澶?	   鏀寔 8 鍧楅€傞厤鍣ㄣ€?

```
## III. 閬囧埌闂鏃跺彲灏濊瘯鐨勪簨椤?

 1. 纭浣犵殑鍗＄殑 PCI id 鍦ㄤ笂闈㈢殑绗?I 鑺傛墍鍒椾箣涓€? 2. 纭璺敱姝ｇ‘銆? 3. 灏濊瘯寮哄埗涓嶅悓鐨?speed/duplex 璁剧疆銆?

杩樻湁涓€涓?tlan 閭欢鍒楄〃锛屼綘鍙互閫氳繃鍚?majordomo@vuser.vu.union.edu 鍙戦€侀偖浠讹紝鍦?姝ｆ枃涓啓鈥渟ubscribe tlan鈥濇潵鍔犲叆銆?
鍙︽湁涓€涓?tlan 缃戠珯锛歨ttp://www.compaq.com
