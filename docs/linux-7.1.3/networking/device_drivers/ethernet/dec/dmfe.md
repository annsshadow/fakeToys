
## 鐢ㄤ簬 Linux 鐨?Davicom DM9102(A)/DM9132/DM9801 蹇€熶互澶綉椹卞姩


娉ㄦ剰锛氳椹卞姩娌℃湁缁存姢鑰呫€?

鏈▼搴忔槸鑷敱杞欢锛涗綘鍙互鍦ㄨ嚜鐢辫蒋浠跺熀閲戜細鍙戝竷鐨?GNU 閫氱敤鍏叡璁稿彲璇侊紙GNU General Public License锛夋潯娆句笅閲嶆柊鍒嗗彂鍜?鎴栦慨鏀瑰畠锛涜鍙瘉鐗堟湰涓虹 2 鐗堬紝鎴栬€咃紙鐢变綘閫夋嫨锛変换浣曟洿楂樼増鏈€?
鏈▼搴忕殑鍒嗗彂甯屾湜瀹冩槸鏈夌敤鐨勶紝浣嗘病鏈変换浣曟媴淇濓紱鐢氳嚦娌℃湁瀵归€傞攢鎬ф垨鐗瑰畾鐢ㄩ€旈€傜敤鎬х殑闅愬惈鎷呬繚銆傛洿澶氱粏鑺傝鍙傝 GNU 閫氱敤鍏叡璁稿彲璇併€?
璇ラ┍鍔ㄤ负 Davicom DM9102(A)/DM9132/DM9801 浠ュお缃戝崱鎻愪緵鍐呮牳鏀寔锛圕NET 10/100 浠ュお缃戝崱涔熶娇鐢?Davicom 鑺墖缁勶紝鍥犳璇ラ┍鍔ㄤ篃鏀寔 CNET 鍗★級銆傚鏋滀綘娌℃湁灏嗚椹卞姩缂栬瘧涓烘ā鍧楋紝瀹冨皢鍦ㄥ惎鍔ㄦ椂鑷姩鍔犺浇鑷韩骞舵墦鍗颁竴鏉?```

	dmfe: Davicom DM9xxx net driver, version 1.36.4 (2002-01-17)

```

```
	insmod dmfe

```
杩欐牱瀹冧細鑷姩妫€娴嬭澶囨ā寮忋€傝繖鏄缓璁殑鍔犺浇妯″潡鏂瑰紡銆傛垨鑰呬綘鍙互浼犲叆
```

	insmod dmfe mode=0 # 寮哄埗 10M 鍗婂弻宸?	insmod dmfe mode=1 # 寮哄埗 100M 鍗婂弻宸?	insmod dmfe mode=4 # 寮哄埗 10M 鍏ㄥ弻宸?	insmod dmfe mode=5 # 寮哄埗 100M 鍏ㄥ弻宸?
```

```
	ifconfig eth0 172.22.3.18
		      ^^^^^^^^^^^
		     浣犵殑 IP 鍦板潃

```

```
	route add default eth0



```
鐜板湪浣犵殑浠ュお缃戝崱搴旇宸茬粡鍚姩骞惰繍琛屻€?

TODO锛?
- 瀹炵幇 pci_driver::suspend() 鍜?pci_driver::resume() 鐢垫簮绠＄悊鏂规硶銆?- 鍦?64 浣嶆満鍣ㄤ笂妫€鏌ャ€?- 鍦?big endian 鏈哄櫒涓婃鏌ュ苟淇銆?- 娴嬭瘯骞剁‘淇濇墍鏈夋儏鍐典笅 PCI 寤惰繜锛坙atency锛夌幇鍦ㄩ兘姝ｇ‘銆?

浣滆€咃細

Sten Wang <sten_wang@davicom.com.tw >   : 鍘熷浣滆€?
璐＄尞鑰咃細

- Marcelo Tosatti <marcelo@conectiva.com.br>
- Alan Cox <alan@lxorguk.ukuu.org.uk>
- Jeff Garzik <jgarzik@pobox.com>
- Vojtech Pavlik <vojtech@suse.cz>
