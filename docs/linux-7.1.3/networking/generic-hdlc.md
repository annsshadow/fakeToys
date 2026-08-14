## 閫氱敤 HDLC 灞?

Krzysztof Halasa <khc@pm.waw.pl>


閫氱敤 HDLC 灞傜洰鍓嶆敮鎸侊細

1. 甯т腑缁э紙Frame Relay锛孉NSI銆丆CITT銆丆isco 浠ュ強鏃?LMI锛?
   - 鏅€氾紙璺敱锛夊拰浠ュお缃戞ˉ鎺ワ紙浠ュお缃戣澶囦豢鐪燂級鎺ュ彛鍙互鍏变韩鍚屼竴涓?PVC銆?   - ARP 鏀寔锛堝唴鏍镐笉鏀寔 InARP 鈥斺€?鍦ㄤ互涓嬪湴鍧€鏈変竴涓疄楠屾€х殑 InARP 鐢ㄦ埛鎬佸畧鎶よ繘绋嬶細
     http://www.kernel.org/pub/linux/utils/net/hdlc/锛夈€?
2. 鍘熷 HDLC 鈥斺€?鍙互鏄?IP锛圛Pv4锛夋帴鍙ｆ垨浠ュお缃戣澶囦豢鐪?3. Cisco HDLC
4. PPP
5. X.25锛堜娇鐢?X.25 渚嬬▼锛夈€?
閫氱敤 HDLC 鍙槸涓€涓崗璁┍鍔?鈥斺€?瀹冮渶瑕侀拡瀵逛綘鐗瑰畾纭欢鐨勪綆灞傞┍鍔ㄣ€?
浣跨敤 HDLC 鎴栧抚涓户 PVC 鐨勪互澶綉璁惧浠跨湡鍏煎 IEEE 802.1Q锛圴LAN锛夊拰 802.1D锛堜互澶綉妗ユ帴锛夈€?

纭繚 hdlc.o 鍜岀‖浠堕┍鍔ㄥ凡鍔犺浇銆傚畠搴斿綋鍒涘缓鑻ュ共涓?鈥渉dlc鈥濓紙hdlc0 绛夛級缃戠粶璁惧锛屾瘡涓?WAN 绔彛涓€涓€備綘闇€瑕?鈥渟ethdlc鈥?宸ュ叿锛屽彲浠庝互涓嬪湴鍧€鑾峰彇锛?
	http://www.kernel.org/pub/linux/utils/net/hdlc/

```

	gcc -O2 -Wall -o sethdlc sethdlc.c

```
纭繚浣犱娇鐢ㄧ殑鏄笌鍐呮牳鐗堟湰鍖归厤鐨?sethdlc銆?
浣跨敤 sethdlc 鏉ヨ缃墿鐞嗘帴鍙ｃ€佹椂閽熼€熺巼銆佹墍浣跨敤鐨?HDLC 妯″紡锛屽苟鍦ㄤ娇鐢ㄥ抚涓户鏃舵坊鍔?鎵€闇€鐨?PVC銆?```

	sethdlc hdlc0 clock int rate 128000
	sethdlc hdlc0 cisco interval 10 timeout 25

```
```

	sethdlc hdlc0 rs232 clock ext
	sethdlc hdlc0 fr lmi ansi
	sethdlc hdlc0 create 99
	ifconfig hdlc0 up
	ifconfig pvc0 localIP pointopoint remoteIP

```
鍦ㄥ抚涓户妯″紡涓嬶紝鍦ㄤ娇鐢?pvc 璁惧涔嬪墠锛屽厛鐢?ifconfig 灏嗕富 hdlc 璁惧 up锛堜笉瑕佺粰瀹冨垎閰?浠讳綍 IP 鍦板潃锛夈€?

璁剧疆鎺ュ彛锛?
- v35 | rs232 | x21 | t1 | e1
    - 褰撳崱鍏锋湁杞欢鍙€夋帴鍙ｆ椂锛岃缃粰瀹氱鍙ｇ殑鐗╃悊鎺ュ彛
  loopback
    - 婵€娲荤‖浠跺洖鐜紙浠呯敤浜庢祴璇曪級
- clock ext
    - RX 鏃堕挓鍜?TX 鏃堕挓鍧囦负澶栭儴
- clock int
    - RX 鏃堕挓鍜?TX 鏃堕挓鍧囦负鍐呴儴
- clock txint
    - RX 鏃堕挓澶栭儴锛孴X 鏃堕挓鍐呴儴
- clock txfromrx
    - RX 鏃堕挓澶栭儴锛孴X 鏃堕挓鐢?RX 鏃堕挓娲剧敓
- rate
    - 璁剧疆鏃堕挓閫熺巼锛坆ps锛夛紙浠呯敤浜?鈥渋nt鈥?鎴?鈥渢xint鈥?鏃堕挓锛?

璁剧疆鍗忚锛?
- hdlc - 璁剧疆鍘熷 HDLC锛堜粎 IP锛夋ā寮?
  nrz / nrzi / fm-mark / fm-space / manchester - 璁剧疆浼犺緭缂栫爜

  no-parity / crc16 / crc16-pr0锛堥缃浂鐨?CRC16锛? crc32-itu

  crc16-itu锛堜娇鐢?ITU-T 澶氶」寮忕殑 CRC16锛? crc16-itu-pr0 - 璁剧疆濂囧伓鏍￠獙

- hdlc-eth - 浣跨敤 HDLC 鐨勪互澶綉璁惧浠跨湡銆傚鍋舵牎楠屽拰缂栫爜鍚屼笂銆?
- cisco - 璁剧疆 Cisco HDLC 妯″紡锛堟敮鎸?IP銆両Pv6 鍜?IPX锛?
  interval - 淇濇椿鍖呬箣闂寸殑鏃堕棿闂撮殧锛堢锛?
  timeout - 鍦ㄥ亣瀹氶摼璺柇寮€鍓嶏紝璺濇渶鍚庝竴娆℃敹鍒颁繚娲诲寘鐨勬椂闂达紙绉掞級

- ppp - 璁剧疆鍚屾 PPP 妯″紡

- x25 - 璁剧疆 X.25 妯″紡

- fr - 甯т腑缁фā寮?
  lmi ansi / ccitt / cisco / none - LMI锛堥摼璺鐞嗭級绫诲瀷

  dce - 甯т腑缁?DCE锛堢綉缁滀晶锛塋MI锛岃€岄潪榛樿鐨?DTE锛堢敤鎴蜂晶锛夈€?
  瀹冧笌鏃堕挓姣棤鍏崇郴锛?
  - t391 - 閾捐矾瀹屾暣鎬ч獙璇佽疆璇㈠畾鏃跺櫒锛堢锛? 鐢ㄦ埛渚?  - t392 - 杞楠岃瘉瀹氭椂鍣紙绉掞級- 缃戠粶渚?  - n391 - 鍏ㄧ姸鎬佽疆璇㈣鏁板櫒 - 鐢ㄦ埛渚?  - n392 - 閿欒闃堝€?- 鐢ㄦ埛渚у拰缃戠粶渚?  - n393 - 鍙楃洃鎺т簨浠惰鏁?- 鐢ㄦ埛渚у拰缃戠粶渚?
浠呭抚涓户锛?
- create n | delete n - 娣诲姞/鍒犻櫎 DLCI 涓?#n 鐨?PVC 鎺ュ彛銆?  鏂板垱寤虹殑鎺ュ彛灏嗗懡鍚嶄负 pvc0銆乸vc1 绛夈€?
- create ether n | delete ether n - 娣诲姞涓€涓敤浜庝互澶綉妗ユ帴甯х殑璁惧銆傝璁惧灏嗗懡鍚嶄负
  pvceth0銆乸vceth1 绛夈€?

### 鏉跨骇鐗瑰畾闂

```

	insmod n2 hw=io,irq,ram,ports[:io,irq,...]

```
```

	insmod n2 hw=0x300,10,0xD0000,01

```
```

	insmod c101 hw=irq,ram[:irq,...]

```
```

	insmod c101 hw=9,0xdc000

```
```

	n2.hw=io,irq,ram,ports:...

```
```

	c101.hw=irq,ram:...


```
濡傛灉浣犲湪浣跨敤 N2銆丆101 鎴?PLX200SYN 鍗℃椂閬囧埌闂锛屽彲浠ユ墽琛?```

	sethdlc hdlc0 private

```
纭欢椹卞姩蹇呴』鍦ㄤ娇鐢?#define DEBUG_RINGS 缂栬瘧鏃舵瀯寤恒€傚皢姝や俊鎭檮鍦?bug 鎶ュ憡涓細寰堟湁甯姪銆?鏃犺濡備綍锛屽鏋滃湪浣跨敤涓亣鍒伴棶棰橈紝璇峰憡璇夋垜銆?
琛ヤ竵鍜屽叾瀹冧俊鎭锛?<http://www.kernel.org/pub/linux/utils/net/hdlc/>銆?