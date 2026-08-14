
## PCI 娴嬭瘯鍔熻兘锛圥CI Test Function锛?

:Author: Kishon Vijay Abraham I <kishon@ti.com>

浼犵粺涓婏紝PCI RC锛圧oot Complex锛変竴鐩撮€氳繃浣跨敤鏍囧噯鐨?PCI 鍗★紙濡備互澶綉 PCI 鍗°€乁SB PCI 鍗℃垨 SATA PCI 鍗★級鏉ラ獙璇併€備笉杩囷紝闅忕潃 Linux 鍐呮牳涓姞鍏?EP-core锛屽彲浠ュ皢涓€涓彲杩愯浜?EP 妯″紡鐨?PCI 鎺у埗鍣ㄩ厤缃负浣滀负娴嬭瘯璁惧宸ヤ綔銆?
PCI 绔偣娴嬭瘯璁惧鏄竴涓櫄鎷熻澶囷紙鍦ㄨ蒋浠朵腑瀹氫箟锛夛紝鐢ㄤ簬娴嬭瘯绔偣鍔熻兘锛屽苟浣滀负鍏朵粬 PCI 绔偣璁惧锛堜娇鐢?EP 妗嗘灦锛夌殑绀轰緥椹卞姩銆?
PCI 绔偣娴嬭瘯璁惧鍏锋湁浠ヤ笅瀵勫瓨鍣細

 1) PCI_ENDPOINT_TEST_MAGIC
 2) PCI_ENDPOINT_TEST_COMMAND
 3) PCI_ENDPOINT_TEST_STATUS
 4) PCI_ENDPOINT_TEST_SRC_ADDR
 5) PCI_ENDPOINT_TEST_DST_ADDR
 6) PCI_ENDPOINT_TEST_SIZE
 7) PCI_ENDPOINT_TEST_CHECKSUM
 8) PCI_ENDPOINT_TEST_IRQ_TYPE
 9) PCI_ENDPOINT_TEST_IRQ_NUMBER

- PCI_ENDPOINT_TEST_MAGIC

璇ュ瘎瀛樺櫒灏嗙敤浜庢祴璇?BAR0銆備細鍐欏叆涓€涓凡鐭ユā寮忓苟浠?MAGIC 瀵勫瓨鍣ㄨ鍥烇紝浠ラ獙璇?BAR0銆?
- PCI_ENDPOINT_TEST_COMMAND

璇ュ瘎瀛樺櫒鐢变富鏈洪┍鍔ㄧ敤鏉ユ寚绀虹鐐硅澶囧繀椤绘墽琛岀殑鍔熻兘銆?
========	================================================================
Bitfield	Description
========	================================================================
Bit 0		瑙﹀彂浼犵粺锛坙egacy锛塈RQ
Bit 1		瑙﹀彂 MSI IRQ
Bit 2		瑙﹀彂 MSI-X IRQ
Bit 3		璇诲懡浠わ紙浠?RC 缂撳啿鍖鸿鍙栨暟鎹級
Bit 4		鍐欏懡浠わ紙鍚?RC 缂撳啿鍖哄啓鍏ユ暟鎹級
Bit 5		澶嶅埗鍛戒护锛堝皢涓€浠?RC 缂撳啿鍖虹殑鏁版嵁澶嶅埗鍒板彟涓€浠?RC 缂撳啿鍖猴級
========	================================================================

- PCI_ENDPOINT_TEST_STATUS

璇ュ瘎瀛樺櫒鍙嶆槧 PCI 绔偣璁惧鐨勭姸鎬併€?
========	==============================
Bitfield	Description
========	==============================
Bit 0		璇绘垚鍔?Bit 1		璇诲け璐?Bit 2		鍐欐垚鍔?Bit 3		鍐欏け璐?Bit 4		澶嶅埗鎴愬姛
Bit 5		澶嶅埗澶辫触
Bit 6		宸茶Е鍙?IRQ
Bit 7		婧愬湴鍧€鏃犳晥
Bit 8		鐩殑鍦板潃鏃犳晥
========	==============================

- PCI_ENDPOINT_TEST_SRC_ADDR

璇ュ瘎瀛樺櫒鍖呭惈 COPY/READ 鍛戒护鐨勬簮鍦板潃锛圧C 缂撳啿鍖哄湴鍧€锛夈€?
- PCI_ENDPOINT_TEST_DST_ADDR

璇ュ瘎瀛樺櫒鍖呭惈 COPY/WRITE 鍛戒护鐨勭洰鐨勫湴鍧€锛圧C 缂撳啿鍖哄湴鍧€锛夈€?
- PCI_ENDPOINT_TEST_IRQ_TYPE

璇ュ瘎瀛樺櫒鍖呭惈涓?READ/WRITE/COPY 浠ュ強瑙﹀彂 IRQ锛圠egacy/MSI锛夊懡浠ゆ墍瑙﹀彂鐨勪腑鏂被鍨嬨€?
鍙€夌被鍨嬶細

======	==
Legacy	0
MSI	1
MSI-X	2
======	==

- PCI_ENDPOINT_TEST_IRQ_NUMBER

璇ュ瘎瀛樺櫒鍖呭惈琚Е鍙戠殑涓柇 ID銆?
鍙彇鐨勫€硷細

======	===========
Legacy	0
MSI	[1 .. 32]
MSI-X	[1 .. 2048]
======	===========
