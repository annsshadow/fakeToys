
## PCI 椹卞姩

鏈枃瑙ｉ噴濯掍綋璁惧濡備綍閫氳繃 PCI ID锛堝巶鍟?璁惧 ID銆佸瓙绯荤粺 ID锛夎瘑鍒澘鍗★紝骞剁ず鑼冪敤 lspci 鍛戒护鏌ヨ PCI ID锛屽悓鏃惰鏄庝负浣曢儴鍒嗛┍鍔ㄩ渶瑕?card= 鍙傛暟鏉ュ尮閰嶇浉鍚屽瓙绯荤粺 ID 鐨勪笉鍚屼骇鍝併€?


PCI 鏉垮崱閫氳繃绉颁负 PCI ID 鐨勬爣璇嗘潵璇嗗埆銆侾CI ID 瀹為檯涓婄敱涓ら儴鍒嗙粍鎴愶細

 - 鍘傚晢 ID锛圴endor ID锛夊拰璁惧 ID锛坉evice ID锛夛紱
 - 瀛愮郴缁?ID锛圫ubsystem ID锛夊拰瀛愮郴缁熻澶?ID锛圫ubsystem device ID锛夛紱

`lspci -nn` 鍛戒护鍙敤浜庤瘑鍒巶鍟?璁惧鐨?PCI ID锛?

   :emphasize-lines: 3

    $ lspci -nn
    ...
    00:0a.0 Multimedia controller [^0480^]: Philips Semiconductors SAA7131/SAA7133/SAA7135 Video Broadcast Decoder [1131:7133] (rev d1)
    00:0b.0 Multimedia controller [^0480^]: Brooktree Corporation Bt878 Audio Capture [109e:0878] (rev 11)
    01:00.0 Multimedia video controller [^0400^]: Conexant Systems, Inc. CX23887/8 PCIe Broadcast Audio and Video Decoder with 3D Comb [14f1:8880] (rev 0f)
    02:01.0 Multimedia video controller [^0400^]: Internext Compression Inc iTVC15 (CX23415) Video Decoder [4444:0803] (rev 01)
    02:02.0 Multimedia video controller [^0400^]: Conexant Systems, Inc. CX23418 Single-Chip MPEG-2 Encoder with Integrated Analog Video/Broadcast Audio Decoder [14f1:5b7a]
    02:03.0 Multimedia video controller [^0400^]: Brooktree Corporation Bt878 Video Capture [109e:036e] (rev 11)
    ...

瀛愮郴缁?ID 鍙互浣跨敤 `lspci -vn` 鑾峰彇

   :emphasize-lines: 4

    $ lspci -vn
    ...
	00:0a.0 0480: 1131:7133 (rev d1)
		Subsystem: 1461:f01d
		Flags: bus master, medium devsel, latency 32, IRQ 209
		Memory at e2002000 (32-bit, non-prefetchable) [size=2K]
		Capabilities: [^40^] Power Management version 2
    ...

鍦ㄤ笂杩扮ず渚嬩腑锛岀涓€鍧楀崱浣跨敤 `saa7134` 椹卞姩锛屽叾鍘傚晢/璁惧 PCI ID 涓?`1131:7133`锛?
PCI 瀛愮郴缁?ID 涓?`1461:f01d`锛堝弬瑙?[Saa7134 鍗″垪琛?saa7134-cardlist>](Saa7134 card list<saa7134-cardlist>)锛夈€?

閬楁喚鐨勬槸锛屾湁鏃朵笉鍚岀殑浜у搧浼氫娇鐢ㄧ浉鍚岀殑 PCI 瀛愮郴缁?ID銆傚洜姝わ紝鑻ュ共濯掍綋椹卞姩鍏佽
浼犲叆 `card=` 鍙傛暟锛屼互渚胯缃竴涓笌鐗瑰畾鏉垮崱姝ｇ‘璁剧疆鐩稿尮閰嶇殑鍗″彿銆?

涓嬮潰鍒楀嚭浜嗗綋鍓嶅彈鏀寔鐨?PCI/PCIe 鍗★紙涓嶅寘鎷?staging 椹卞姩锛塡 [#]_銆?

涓嬭〃姹囨€讳簡鍚勫獟浣撻┍鍔ㄥ強鍏舵敮鎸佺殑璁惧璇存槑锛?

================  ========================================================
Driver            Name锛堟敮鎸佺殑璁惧锛?
================  ========================================================
altera-ci         Altera FPGA CI 妯″潡
b2c2-flexcop-pci  Technisat/B2C2 Air/Sky/Cable2PC PCI 鍗?
bt878             鍩轰簬 bt878 鐨勭數瑙嗗崱 DVB/ATSC 鏀寔
bttv             BT8x8 Video For Linux 瑙嗛閲囬泦鍗?
cobalt            Cisco Cobalt 璁惧
cx18              Conexant cx23418 MPEG 缂栫爜鍣?
cx23885           Conexant cx23885锛?388x 鐨勫悗缁у瀷鍙凤級
cx25821           Conexant cx25821 璁惧
cx88xx            Conexant 2388x锛坆t878 鐨勫悗缁у瀷鍙凤級
ddbridge          Digital Devices 妗ユ帴璁惧
dm1105            鍩轰簬 SDMC DM1105 鐨?PCI 鍗?
dt3155            DT3155 甯ф姄鍙栧崱
dvb-ttpci         AV7110 鍗?
earth-pt1         PT1 鍗?
earth-pt3         Earthsoft PT3 鍗?
hexium_gemini     Hexium Gemini 甯ф姄鍙栧崱
hexium_orion      Hexium HV-PCI6 涓?Orion 甯ф姄鍙栧崱
hopper            鍩轰簬 HOPPER 鐨勫崱
ipu3-cio2         Intel ipu3-cio2 椹卞姩
ivtv              Conexant cx23416/cx23415 MPEG 缂栫爜/瑙ｇ爜鍣?
ivtvfb            Conexant cx23415 甯х紦鍐?
mantis            鍩轰簬 MANTIS 鐨勫崱
mgb4              Digiteq Automotive MGB4 甯ф姄鍙栧崱
mxb               Siemens-Nixdorf 澶氬獟浣撴墿灞曟澘锛圡XB锛?
netup-unidvb      NetUP 閫氱敤 DVB 鍗?
ngene             Micronas nGene 璁惧
pluto2            Pluto2 鍗?
saa7134           Philips SAA7134 璁惧
saa7164           NXP SAA7164 璁惧
smipcie           SMI PCIe DVBSky 鍗?
solo6x10          Bluecherry / Softlogic 6x10 閲囬泦鍗★紙MPEG-4/H.264锛?
tw5864            Techwell TW5864 瑙嗛/闊抽鎶撳彇涓庣紪鐮佸崱
tw686x            Intersil/Techwell TW686x 璁惧
tw68              Techwell tw68x Video For Linux 瑙嗛閲囬泦鍗?
zoran             Zoran-36057/36067 JPEG 缂栬В鐮佸櫒
================  ========================================================

鍏朵腑閮ㄥ垎椹卞姩鏀寔澶氫釜璁惧锛屽涓嬮潰鐨勫崱鍒楄〃鎵€绀猴細

- [bttv-cardlist](bttv-cardlist)
- [cx18-cardlist](cx18-cardlist)
- [cx23885-cardlist](cx23885-cardlist)
- [cx88-cardlist](cx88-cardlist)
- [ivtv-cardlist](ivtv-cardlist)
- [saa7134-cardlist](saa7134-cardlist)
- [saa7164-cardlist](saa7164-cardlist)
- [zoran-cardlist](zoran-cardlist)

