
## Linux 涓嬬殑 Adaptec AHA-1520/1522 SCSI 椹卞姩锛坅ha152x锛?

Copyright |copy| 1993-1999 J眉rgen Fischer <fischer@norbit.de>

TC1550 琛ヤ竵鐢?Luuk van Dijk (ldz@xs4all.nl) 鎻愪緵


绗?2 鐗堜腑璇ラ┍鍔ㄨ繘琛屼簡澶ч噺淇敼锛堝挨鍏舵槸搴曞崐閮ㄥ鐞嗗嚱鏁?complete()锛夈€?
椹卞姩鐜板湪鏇村姞鏁存磥锛屾敮鎸?2.3 涓殑鏂伴敊璇鐞嗕唬鐮侊紝浜х敓鐨?CPU 璐熻浇鏇翠綆
锛堣疆璇㈠惊鐜ぇ骞呭噺灏戯級锛屽悶鍚愰噺涔熺暐鏈夋彁楂橈紙鑷冲皯鍦ㄦ垜閭ｅ彴鑰佹棫鐨勬祴璇曟満涓婏紱
涓€鍙?i486/33Mhz/20MB锛夈€?

## 閰嶇疆鍙傛暟


============  ========================================  ======================
IOPORT        IO 鍩哄潃                                   (0x340/0x140)
IRQ           涓柇绾у埆                                  (9-12; 榛樿 11)
SCSI_ID       鎺у埗鍣ㄧ殑 SCSI ID                          (0-7; 榛樿 7)
RECONNECT     鍏佽鐩爣璁惧浠庢€荤嚎鏂紑                    (0/1; 榛樿 1 [寮€])
PARITY        鍚敤濂囧伓鏍￠獙                              (0/1; 榛樿 1 [寮€])
SYNCHRONOUS   鍚敤鍚屾浼犺緭                              (0/1; 榛樿 1 [寮€])
DELAY:        鎬荤嚎澶嶄綅寤惰繜                              (榛樿 100)
EXT_TRANS:    鍚敤鎵╁睍杞崲                              (0/1: 榛樿 0 [鍏砞)
              (瑙佲€滄敞鎰忎簨椤光€?
============  ========================================  ======================

## 缂栬瘧鏈熼厤缃?

锛堣繘鍏?drivers/scsi/Makefile 涓殑 AHA152X锛夛細

- DAUTOCONF
    浣跨敤鎺у埗鍣ㄦ姤鍛婂嚭鏉ョ殑閰嶇疆锛堜粎 AHA-152x锛?
- DSKIP_BIOSTEST
    涓嶆祴璇?BIOS 绛惧悕锛圓HA-1510 鎴?BIOS 琚鐢ㄦ椂锛?
- DSETUP0="{ IOPORT, IRQ, SCSI_ID, RECONNECT, PARITY, SYNCHRONOUS, DELAY, EXT_TRANS }"
    瀵圭涓€涓帶鍒跺櫒鐨勮鐩栭厤缃?
- DSETUP1="{ IOPORT, IRQ, SCSI_ID, RECONNECT, PARITY, SYNCHRONOUS, DELAY, EXT_TRANS }"
    瀵圭浜屼釜鎺у埗鍣ㄧ殑瑕嗙洊閰嶇疆

- DAHA152X_DEBUG
    鍚敤璋冭瘯杈撳嚭

- DAHA152X_STAT
    鍚敤涓€浜涚粺璁′俊鎭?

## LILO 鍛戒护琛岄€夐」


```

    aha152x=<IOPORT>[,<IRQ>[,<SCSI-ID>[,<RECONNECT>[,<PARITY>[,<SYNCHRONOUS>[,<DELAY> [,<EXT_TRANS]]]]]]]

 姝ｅ父鐨勯厤缃彲浠ラ€氳繃鎸囧畾鍛戒护琛屾潵瑕嗙洊銆傝繖鏍峰仛鏃朵細璺宠繃 BIOS 娴嬭瘯銆傝緭鍏? 鐨勫€煎繀椤绘湁鏁堬紙涓哄凡鐭ュ€硷級銆備笉瑕佷娇鐢ㄥ湪姝ｅ父鎿嶄綔涓笉琚敮鎸佺殑鍊笺€傚鏋滀綘
 璁や负闇€瑕佸叾浠栧€硷細璇疯仈绯绘垜銆傚浜庝袱涓帶鍒跺櫒锛岃浣跨敤涓ゆ aha152x 璇彞銆?
```
## 妯″潡閰嶇疆鐨勭鍙?

鏈変袱绉嶉€夋嫨锛?
```

    aha152x=IOPORT,IRQ,SCSI_ID,RECONNECT,PARITY,SYNCHRONOUS,DELAY,EXT_TRANS

  绗竴涓帶鍒跺櫒鐨勯厤缃鐩?
  ::

    aha152x1=IOPORT,IRQ,SCSI_ID,RECONNECT,PARITY,SYNCHRONOUS,DELAY,EXT_TRANS

  绗簩涓帶鍒跺櫒鐨勯厤缃鐩?
```
2. 鍙寚瀹氫綘闇€瑕佺殑锛坕rq 鎴?io 鏄繀闇€鐨勶紱鏂板锛?
io=IOPORT0[,IOPORT1]
  绗竴涓拰绗簩涓帶鍒跺櫒鐨?IOPORT

irq=IRQ0[,IRQ1]
  绗竴涓拰绗簩涓帶鍒跺櫒鐨?IRQ

scsiid=SCSIID0[,SCSIID1]
  绗竴涓拰绗簩涓帶鍒跺櫒鐨?SCSIID

reconnect=RECONNECT0[,RECONNECT1]
  绗竴涓拰绗簩涓帶鍒跺櫒鏄惁鍏佽鐩爣璁惧鏂紑

parity=PAR0[PAR1]
  绗竴涓拰绗簩涓帶鍒跺櫒鏄惁浣跨敤濂囧伓鏍￠獙

sync=SYNCHRONOUS0[,SYNCHRONOUS1]
  绗竴涓拰绗簩涓帶鍒跺櫒鏄惁鍚敤鍚屾浼犺緭

delay=DELAY0[,DELAY1]
  绗竴涓拰绗簩涓帶鍒跺櫒鐨勫浣?DELAY

exttrans=EXTTRANS0[,EXTTRANS1]
  绗竴涓拰绗簩涓帶鍒跺櫒鏄惁鍚敤鎵╁睍杞崲


濡傛灉涓ょ鏂瑰紡閮戒娇鐢紝鍒欓噰鐢ㄧ涓€绉嶃€?

## 鍏充簬 EXT_TRANS 鐨勮鏄?

SCSI 浣跨敤鍧楀彿鏉ュ鍧€璁惧涓婄殑鍧?鎵囧尯銆傝€?BIOS 浣跨敤鐨勬槸鏌遍潰/纾佸ご/鎵囧尯
锛圕/H/S锛夊鍧€鏂规銆侱OS 鏈熸湜涓€涓兘鐞嗚В杩欑 C/H/S 瀵诲潃鐨?BIOS 鎴栭┍鍔ㄣ€?
鏌遍潰/纾佸ご/鎵囧尯鐨勬暟閲忕О涓哄嚑浣曞弬鏁帮紙geometry锛夛紝鏄?C/H/S 瀵诲潃璇锋眰鐨勫熀纭€銆?SCSI 鍙簡瑙ｇ鐩樹互鍧楋紙鎵囧尯锛夎鐨勬€诲閲忋€?
鍥犳 SCSI 鐨?BIOS/DOS 椹卞姩蹇呴』璁＄畻鍑轰竴涓€昏緫/铏氭嫙鍑犱綍鍙傛暟锛屾墠鑳芥敮鎸?杩欑瀵诲潃鏂规銆係CSI BIOS 杩斿洖鐨勫嚑浣曞弬鏁扮函灞炶绠楃粨鏋滐紝涓庣鐩樼湡瀹?鐗╃悊
鐨勫嚑浣曞弬鏁版鏃犲叧绯伙紙鑰屽悗鑰呴€氬父涔熸棤鍏崇揣瑕侊級銆?
鍩烘湰涓婅繖瀵?Linux 姣棤褰卞搷锛屽洜涓哄畠鍚屾牱浣跨敤鍧楄€岄潪 C/H/S 瀵诲潃銆備笉骞哥殑鏄紝
C/H/S 瀵诲潃涔熺敤浜庡垎鍖鸿〃涓紝鍥犳姣忎釜鎿嶄綔绯荤粺閮藉繀椤荤煡閬撴纭殑鍑犱綍鍙傛暟
鎵嶈兘瑙ｈ瀹冦€?
姝ゅ锛孋/H/S 瀵诲潃鏂规瀛樺湪鏌愪簺闄愬埗锛屽嵆鍦板潃绌洪棿琚檺鍒跺湪鏈€澶?255 涓澶淬€?鏈€澶?63 涓墖鍖猴紝浠ュ強鏈€澶?1023 涓煴闈€?
AHA-1522 鐨?BIOS 閫氳繃灏嗙澶存暟鍥哄畾涓?64銆佹墖鍖烘暟鍥哄畾涓?32锛屽苟鐢ㄧ鐩樻姤鍛婄殑
瀹归噺闄や互 64*32锛? MB锛夋潵璁＄畻鏌遍潰鏁帮紝浠庤€屽緱鍑哄嚑浣曞弬鏁般€傝繖琚涓洪粯璁よ浆鎹€?
鑰冭檻鍒?C/H/S 鐨?1023 鏌遍潰闄愬埗锛屼綘鍦ㄥ垎鍖鸿〃涓彧鑳藉鍧€纾佺洏鐨勫墠 1 GB銆傚洜姝わ紝
鍩轰簬 AIC-6260/6360 鐨勪竴浜涜緝鏂版帶鍒跺櫒鐨?BIOS 鏀寔鎵╁睍杞崲銆傝繖鎰忓懗鐫€涓€鏃?BIOS 鐪嬪埌澶т簬 1 GB 鐨勭鐩橈紝瀹冨氨浼氬皢纾佸ご鏁板彇 255銆佹墖鍖烘暟鍙?63锛岀劧鍚庣敤
纾佺洏瀹归噺闄や互 255*63锛堢害 8 MB锛夈€傝繖鏍峰垎鍖鸿〃涓彲瀵诲潃鐨勭鐩樼┖闂存渶澶х害涓?8 GB锛堜笉杩囧浠婂凡缁忔湁鏇村ぇ鐨勭鐩樹簡锛夈€?
鏇村鏉傜殑鏄紝鍦ㄦ煇浜?BIOS 璁剧疆涓紝杞崲妯″紡鍙兘鍙互銆佷篃鍙兘涓嶅彲閰嶇疆銆?
鏈┍鍔ㄤ細杩涜涓€浜涙垨澶氭垨灏戠殑鈥滄晠闅滃畨鍏ㄢ€濈寽娴嬶紝浠ヤ究鍦ㄥぇ澶氭暟鎯呭喌涓嬪緱鍒?姝ｇ‘鐨勫嚑浣曞弬鏁帮細

- 瀵逛簬 <1GB 鐨勭鐩橈細浣跨敤榛樿杞崲锛圕/32/64锛?
- 瀵逛簬 >1GB 鐨勭鐩橈細

  - 浠庡垎鍖鸿〃鑾峰彇褰撳墠鍑犱綍鍙傛暟锛堜娇鐢?scsicam_bios_param锛屼笖鍙帴鍙椻€滄湁鏁堚€?    鐨勫嚑浣曞弬鏁帮紝鍗?(C/32/64) 鎴?(C/63/255)锛夈€傚嵆浣块┍鍔ㄦ湭鍚敤鎵╁睍杞崲锛?    杩欎篃鍙兘鏄墿灞曡浆鎹€?
  - 濡傛灉澶辫触锛屽垯閲囩敤鐢辫鐩栭厤缃€佸唴鏍告垨妯″潡鍙傛暟鍚敤鐨勬墿灞曡浆鎹紱鍚﹀垯
    閲囩敤榛樿杞崲锛屽苟璇锋眰鐢ㄦ埛纭銆傝繖绉嶆儏鍐靛彲鑳藉嚭鐜板湪灏氭湭鍒嗗尯鐨勭鐩樹笂銆?

## 鍙傝€冩枃妗?

 "AIC-6260 SCSI Chip Specification", Adaptec Corporation.

 "SCSI COMPUTER SYSTEM INTERFACE - 2 (SCSI-2)", X3T9.2/86-109 rev. 10h

 "Writing a SCSI device driver for Linux", Rik Faith (faith@cs.unc.edu)

 "Kernel Hacker's Guide", Michael K. Johnson (johnsonm@sunsite.unc.edu)

 "Adaptec 1520/1522 User's Guide", Adaptec Corporation.

 Michael K. Johnson (johnsonm@sunsite.unc.edu)

 Drew Eckhardt (drew@cs.colorado.edu)

 Eric Youngdale (eric@andante.org)

 鐗瑰埆鎰熻阿 Eric Youngdale 鍏嶈垂锛堬紒锛夋彁渚涘叧浜庤鑺墖鐨勬枃妗ｃ€?