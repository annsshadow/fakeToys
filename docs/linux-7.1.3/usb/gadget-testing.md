## Gadget 娴嬭瘯


鏈枃浠舵€荤粨浜嗗叧浜庡 gadget 鎵€鎻愪緵鐨?USB 鍔熻兘杩涜鍩烘湰娴嬭瘯鐨勪俊鎭€?

   1. ACM 鍔熻兘
   2. ECM 鍔熻兘
   3. ECM subset 鍔熻兘
   4. EEM 鍔熻兘
   5. FFS 鍔熻兘
   6. HID 鍔熻兘
   7. LOOPBACK 鍔熻兘
   8. MASS STORAGE 鍔熻兘
   9. MIDI 鍔熻兘
   10. NCM 鍔熻兘
   11. OBEX 鍔熻兘
   12. PHONET 鍔熻兘
   13. RNDIS 鍔熻兘
   14. SERIAL 鍔熻兘
   15. SOURCESINK 鍔熻兘
   16. UAC1 鍔熻兘锛堟棫瀹炵幇锛?   17. UAC2 鍔熻兘
   18. UVC 鍔熻兘
   19. PRINTER 鍔熻兘
   20. UAC1 鍔熻兘锛堟柊 API锛?   21. MIDI2 鍔熻兘


## 1. ACM 鍔熻兘


璇ュ姛鑳界敱 usb_f_acm.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"acm"銆侫CM 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彧鎻愪緵涓€涓睘鎬э細

	port_num

璇ュ睘鎬ф槸鍙鐨勩€?
绯荤粺涓渶澶氬彲浠ユ湁 4 涓?ACM/閫氱敤涓茶/OBEX 绔彛銆?

### 娴嬭瘯 ACM 鍔熻兘


```
	cat > /dev/ttyACM<X>
```
```
	cat /dev/ttyGS<Y>
```
鐒跺悗鍙嶈繃鏉?
```
	cat > /dev/ttyGS<Y>
```
```
	cat /dev/ttyACM<X>
```
## 2. ECM 鍔熻兘


璇ュ姛鑳界敱 usb_f_ecm.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"ecm"銆侲CM 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	=============== ==================================================
	ifname		涓庢湰鍔熻兘瀹炰緥鍏宠仈鐨勭綉缁滆澶囨帴鍙ｅ悕
	qmult		楂橀€熶笌瓒呴€熶笅鐨勯槦鍒楅暱搴︿箻鏁?	host_addr	鏈?USB 浠ュお閾捐矾涓婁富鏈轰竴渚х殑 MAC 鍦板潃
	dev_addr		鏈?USB 浠ュお閾捐矾涓婅澶囦竴渚х殑 MAC 鍦板潃
	=============== ==================================================

鍦ㄥ垱寤?functions/ecm.<瀹炰緥鍚? 涔嬪悗锛屽畠浠寘鍚粯璁ゅ€硷細qmult 涓?5锛宒ev_addr 涓?host_addr 涓洪殢鏈洪€夋嫨銆傚鏋滃姛鑳芥湭缁戝畾锛宨fname 鍙鍐欏叆銆傚啓鍏ュ唴瀹瑰繀椤绘槸涓€涓帴鍙ｆā寮忥紝
渚嬪 "usb%d"锛岃繖灏嗗鑷寸綉缁滄牳蹇冮€夋嫨涓嬩竴涓┖闂茬殑 usbX 鎺ュ彛銆傞粯璁ゆ儏鍐典笅瀹冭璁句负 "usb%d"銆?
### 娴嬭瘯 ECM 鍔熻兘


閰嶇疆璁惧涓庝富鏈虹殑 IP 鍦板潃銆傜劧鍚庯細

```
	ping <host's IP>
```
```
	ping <device's IP>
```
## 3. ECM subset 鍔熻兘


璇ュ姛鑳界敱 usb_f_ecm_subset.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"geth"銆侲CM subset 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	=============== ==================================================
	ifname		涓庢湰鍔熻兘瀹炰緥鍏宠仈鐨勭綉缁滆澶囨帴鍙ｅ悕
	qmult		楂橀€熶笌瓒呴€熶笅鐨勯槦鍒楅暱搴︿箻鏁?	host_addr	鏈?USB 浠ュお閾捐矾涓婁富鏈轰竴渚х殑 MAC 鍦板潃
	dev_addr		鏈?USB 浠ュお閾捐矾涓婅澶囦竴渚х殑 MAC 鍦板潃
	=============== ==================================================

鍦ㄥ垱寤?functions/ecm.<瀹炰緥鍚? 涔嬪悗锛屽畠浠寘鍚粯璁ゅ€硷細qmult 涓?5锛宒ev_addr 涓?host_addr 涓洪殢鏈洪€夋嫨銆傚鏋滃姛鑳芥湭缁戝畾锛宨fname 鍙鍐欏叆銆傚啓鍏ュ唴瀹瑰繀椤绘槸涓€涓帴鍙ｆā寮忥紝
渚嬪 "usb%d"锛岃繖灏嗗鑷寸綉缁滄牳蹇冮€夋嫨涓嬩竴涓┖闂茬殑 usbX 鎺ュ彛銆傞粯璁ゆ儏鍐典笅瀹冭璁句负 "usb%d"銆?
### 娴嬭瘯 ECM subset 鍔熻兘


閰嶇疆璁惧涓庝富鏈虹殑 IP 鍦板潃銆傜劧鍚庯細

```
	ping <host's IP>
```
```
	ping <device's IP>
```
## 4. EEM 鍔熻兘


璇ュ姛鑳界敱 usb_f_eem.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"eem"銆侲EM 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	=============== ==================================================
	ifname		涓庢湰鍔熻兘瀹炰緥鍏宠仈鐨勭綉缁滆澶囨帴鍙ｅ悕
	qmult		楂橀€熶笌瓒呴€熶笅鐨勯槦鍒楅暱搴︿箻鏁?	host_addr	鏈?USB 浠ュお閾捐矾涓婁富鏈轰竴渚х殑 MAC 鍦板潃
	dev_addr		鏈?USB 浠ュお閾捐矾涓婅澶囦竴渚х殑 MAC 鍦板潃
	=============== ==================================================

鍦ㄥ垱寤?functions/eem.<瀹炰緥鍚? 涔嬪悗锛屽畠浠寘鍚粯璁ゅ€硷細qmult 涓?5锛宒ev_addr 涓?host_addr 涓洪殢鏈洪€夋嫨銆傚鏋滃姛鑳芥湭缁戝畾锛宨fname 鍙鍐欏叆銆傚啓鍏ュ唴瀹瑰繀椤绘槸涓€涓帴鍙ｆā寮忥紝
渚嬪 "usb%d"锛岃繖灏嗗鑷寸綉缁滄牳蹇冮€夋嫨涓嬩竴涓┖闂茬殑 usbX 鎺ュ彛銆傞粯璁ゆ儏鍐典笅瀹冭璁句负 "usb%d"銆?
### 娴嬭瘯 EEM 鍔熻兘


閰嶇疆璁惧涓庝富鏈虹殑 IP 鍦板潃銆傜劧鍚庯細

```
	ping <host's IP>
```
```
	ping <device's IP>
```
## 5. FFS 鍔熻兘


璇ュ姛鑳界敱 usb_f_fs.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"ffs"銆傝鍔熻兘鐩綍琚湁鎰忕暀绌轰笖涓嶅彲淇敼銆?
鍒涘缓鐩綍涔嬪悗锛岀郴缁熶腑浼氬嚭鐜?FunctionFS 鐨勪竴涓柊瀹炰緥锛堜竴涓?"device"锛夈€備竴鏃?"device"
鍙敤锛岀敤鎴峰簲閬靛惊浣跨敤 FunctionFS 鐨勬爣鍑嗘祦绋嬶紙鎸傝浇瀹冦€佽繍琛屽疄鐜拌鍔熻兘鏈韩鐨勭敤鎴风┖闂磋繘绋嬶級銆?gadget 搴旈€氳繃鍚?usb_gadget/<gadget>/UDC 鍐欏叆鍚堥€傜殑瀛楃涓叉潵鍚敤銆?
FFS 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彧鎻愪緵涓€涓睘鎬э細

	ready

璇ュ睘鎬ф槸鍙鐨勶紝鐢ㄤ簬鎸囩ず鍔熻兘鏄惁宸插氨缁紙1锛夊彲渚涗娇鐢紝渚嬪鐢ㄦ埛绌洪棿鏄惁宸插悜 ep0 鍐欏叆
鎻忚堪绗︿笌瀛楃涓诧紝浠庤€屽彲浠ュ惎鐢?gadget銆?
### 娴嬭瘯 FFS 鍔熻兘


璁惧绔細鍚姩璇ュ姛鑳界殑鐢ㄦ埛绌洪棿瀹堟姢杩涚▼锛屽惎鐢?gadget

涓绘満绔細浣跨敤璁惧鎻愪緵鐨?USB 鍔熻兘

## 6. HID 鍔熻兘


璇ュ姛鑳界敱 usb_f_hid.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"hid"銆侶ID 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	=============== ===========================================
	protocol	瑕佷娇鐢ㄧ殑 HID 鍗忚
	report_desc	鐢ㄤ簬 HID report 鐨勬暟鎹紝缁忕敱 /dev/hidg<X>
			浼犲叆鐨勬暟鎹櫎澶?	report_length	HID report 闀垮害
	subclass	瑕佷娇鐢ㄧ殑 HID 瀛愮被
	=============== ===========================================

瀵逛簬閿洏锛宲rotocol 涓?subclass 涓?1锛宺eport_length 涓?8锛?```
  $ hd my_report_desc
  00000000  05 01 09 06 a1 01 05 07  19 e0 29 e7 15 00 25 01  |..........)...%.|
  00000010  75 01 95 08 81 02 95 01  75 08 81 03 95 05 75 01  |u.......u.....u.|
  00000020  05 08 19 01 29 05 91 02  95 01 75 03 91 03 95 06  |....).....u.....|
  00000030  75 08 15 00 25 65 05 07  19 00 29 65 81 00 c0     |u...%e....)e...|
  0000003f
```
```
  $ echo -ne \\x05\\x01\\x09\\x06\\xa1.....
```
### 娴嬭瘯 HID 鍔熻兘


璁惧绔細

- 鍒涘缓 gadget
- 灏?gadget 杩炴帴鍒颁竴涓富鏈猴紝鏈€濂戒笉鏄敤浜庢帶鍒?gadget 鐨勯偅鍙?- 杩愯涓€涓悜 /dev/hidg<N> 鍐欏叆鐨勭▼搴忥紝渚嬪
```
	$ ./hid_gadget_test /dev/hidg0 keyboard
```
涓绘満绔細

- 瑙傚療鏉ヨ嚜 gadget 鐨勬寜閿?
## 7. LOOPBACK 鍔熻兘


璇ュ姛鑳界敱 usb_f_ss_lb.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"Loopback"銆侺OOPBACK 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	=============== =======================
	qlen		鍥炵幆闃熷垪鐨勬繁搴?	bulk_buflen	缂撳啿鍖洪暱搴?	=============== =======================

### 娴嬭瘯 LOOPBACK 鍔熻兘


璁惧绔細杩愯 gadget

涓绘満绔細test-usb锛坱ools/usb/testusb.c锛?
## 8. MASS STORAGE 鍔熻兘


璇ュ姛鑳界敱 usb_f_mass_storage.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"mass_storage"銆侻ASS STORAGE 鍔熻兘鍦ㄥ叾鐩綍涓彁渚涗互涓?灞炴€э紝鏂囦欢锛?
	=============== ==============================================
	stall		璁句负鍏佽鍔熻兘鏆傚仠鎵归噺绔偣銆?			鍦ㄦ煇浜涘凡鐭ユ棤娉曟甯稿伐浣滅殑 USB 璁惧涓婁細琚鐢ㄣ€?			浣犲簲璇ュ皢鍏惰涓?true銆?	num_buffers	娴佹按绾跨紦鍐插尯鐨勬暟閲忋€傛湁鏁堟暟鍊间负
			2..4銆備粎褰撹缃簡 CONFIG_USB_GADGET_DEBUG_FILES
			鏃跺彲鐢ㄣ€?	=============== ==============================================

浠ュ強涓€涓搴斾簬 SCSI LUN #0 鐨勯粯璁?lun.0 鐩綍銆?
```
	$ mkdir functions/mass_storage.0/partition.5
```
LUN 缂栧彿涓嶅繀杩炵画锛岄櫎浜嗛粯璁ゅ垱寤虹殑 lun #0 涔嬪銆傛渶澶氬彲鎸囧畾 8 涓?lun锛屼笖閮藉繀椤婚伒寰?<name>.<number> 鐨勫懡鍚嶆柟寮忋€傜紪鍙峰彲浠ユ槸 0..8銆備竴涓笉閿欑殑绾﹀畾鏄皢 lun 鍛藉悕涓?"lun.<number>"锛屽敖绠¤繖骞堕潪寮哄埗銆?
鍦ㄦ瘡涓?lun 鐩綍涓湁浠ヤ笅灞炴€ф枃浠讹細

	=============== ==============================================
	file		璇?LUN 鍚庣鏂囦欢鐨勮矾寰勩€傚鏋?LUN 鏈鏍囪涓哄彲绉婚櫎锛?			鍒欎负蹇呴渶銆?	ro		鎸囧畾瀵硅 LUN 鐨勮闂簲涓哄彧璇荤殑鏍囧織銆傚綋鍚敤浜?			CD-ROM 妯℃嫙锛屼互鍙婂綋鏃犳硶浠?R/W 妯″紡鎵撳紑 "filename"
			鏃讹紝闅愬惈姝ゆ爣蹇椼€?	removable	鎸囧畾璇?LUN 搴旇鎸囩ず涓哄彲绉婚櫎鐨勬爣蹇椼€?	cdrom		鎸囧畾璇?LUN 搴旇鎶ュ憡涓?CD-ROM 鐨勬爣蹇椼€?	nofua		鎸囧畾 SCSI WRITE(10,12) 涓?FUA 鏍囧織鐨勬爣蹇?	forced_eject	杩欎釜鍙啓鏂囦欢浠呭湪鍔熻兘澶勪簬娲诲姩鐘舵€佹椂鎵嶆湁鐢ㄣ€傚畠浼氬鑷?			鍚庣鏂囦欢琚己鍒朵粠 LUN 鍒嗙锛屾棤璁轰富鏈烘槸鍚﹀厑璁搞€?			鍐欏叆浠绘剰闈為浂瀛楄妭鏁伴兘灏嗗鑷村脊鍑恒€?	=============== ==============================================

### 娴嬭瘯 MASS STORAGE 鍔熻兘


璁惧绔細杩炴帴 gadget锛屽惎鐢ㄥ畠
涓绘満绔細dmesg锛岃瀵?USB 椹卞姩鍣ㄥ嚭鐜帮紙濡傛灉绯荤粺閰嶇疆涓鸿嚜鍔ㄦ寕杞斤級

## 9. MIDI 鍔熻兘


璇ュ姛鑳界敱 usb_f_midi.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"midi"銆侻IDI 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	================ ====================================
	buflen		 MIDI 缂撳啿鍖洪暱搴?	id		 USB MIDI 閫傞厤鍣ㄧ殑 ID 瀛楃涓?	in_ports	  MIDI 杈撳叆绔彛鏁?	index		 USB MIDI 閫傞厤鍣ㄧ殑绱㈠紩鍊?	out_ports	 MIDI 杈撳嚭绔彛鏁?	qlen		 USB 璇昏姹傞槦鍒楅暱搴?	interface_string USB AudioControl 鎺ュ彛瀛楃涓?	================ ====================================

### 娴嬭瘯 MIDI 鍔熻兘


鏈変袱绉嶆儏褰細浠?gadget 鍚戜富鏈烘挱鏀?mid锛屼互鍙婁粠涓绘満鍚?gadget 鎾斁 mid銆?
1) 浠?gadget 鍚戜富鏈烘挱鏀?mid锛?
```
  $ arecordmidi -l
   Port    Client name                      Port name
   14:0    Midi Through                     Midi Through Port-0
   24:0    MIDI Gadget                      MIDI Gadget MIDI 1
  $ arecordmidi -p 24:0 from_gadget.mid
```
```
  $ aplaymidi -l
   Port    Client name                      Port name
   20:0    f_midi                           f_midi

  $ aplaymidi -p 20:0 to_host.mid
```
2) 浠庝富鏈哄悜 gadget 鎾斁 mid

```
  $ arecordmidi -l
   Port    Client name                      Port name
   20:0    f_midi                           f_midi

  $ arecordmidi -p 20:0 from_host.mid
```
```
  $ aplaymidi -l
   Port    Client name                      Port name
   14:0    Midi Through                     Midi Through Port-0
   24:0    MIDI Gadget                      MIDI Gadget MIDI 1

  $ aplaymidi -p24:0 to_gadget.mid
```
from_gadget.mid 鍚捣鏉ュ簲涓?to_host.mid 瀹屽叏鐩稿悓銆?
from_host.id 鍚捣鏉ュ簲涓?to_gadget.mid 瀹屽叏鐩稿悓銆?
```
  $ aplaymidi -l
   Port    Client name                      Port name
   14:0    Midi Through                     Midi Through Port-0
   24:0    MIDI Gadget                      MIDI Gadget MIDI 1
  128:0    TiMidity                         TiMidity port 0
  128:1    TiMidity                         TiMidity port 1
  128:2    TiMidity                         TiMidity port 2
  128:3    TiMidity                         TiMidity port 3

  $ aplaymidi -p 128:0 file.mid
```
```
  $ aconnect 24:0 128:0 # try it on the host
```
灏?gadget 鐨?MIDI 绔彛杩炴帴鍒?timidity 鐨?MIDI 绔彛鍚庯紝鍦?gadget 绔敤 aplaymidi -l
鎾斁鐨勪换浣曞唴瀹归兘鍙互鍦ㄤ富鏈虹殑鎵０鍣?鑰虫満涓惉鍒般€?
## 10. NCM 鍔熻兘


璇ュ姛鑳界敱 usb_f_ncm.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"ncm"銆侼CM 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	======================= ==================================================
	ifname			涓庢湰鍔熻兘瀹炰緥鍏宠仈鐨勭綉缁滆澶囨帴鍙ｅ悕
	qmult			楂橀€熶笌瓒呴€熶笅鐨勯槦鍒楅暱搴︿箻鏁?	host_addr		鏈?USB 浠ュお閾捐矾涓婁富鏈轰竴渚х殑 MAC 鍦板潃
	dev_addr		鏈?USB 浠ュお閾捐矾涓婅澶囦竴渚х殑 MAC 鍦板潃
	max_segment_size	P2P 杩炴帴鎵€闇€鐨勬澶у皬銆傝繖灏嗘妸 MTU 璁句负 14 瀛楄妭
	======================= ==================================================

鍦ㄥ垱寤?functions/ncm.<瀹炰緥鍚? 涔嬪悗锛屽畠浠寘鍚粯璁ゅ€硷細qmult 涓?5锛宒ev_addr 涓?host_addr 涓洪殢鏈洪€夋嫨銆傚鏋滃姛鑳芥湭缁戝畾锛宨fname 鍙鍐欏叆銆傚啓鍏ュ唴瀹瑰繀椤绘槸涓€涓帴鍙ｆā寮忥紝
渚嬪 "usb%d"锛岃繖灏嗗鑷寸綉缁滄牳蹇冮€夋嫨涓嬩竴涓┖闂茬殑 usbX 鎺ュ彛銆傞粯璁ゆ儏鍐典笅瀹冭璁句负 "usb%d"銆?
### 娴嬭瘯 NCM 鍔熻兘


閰嶇疆璁惧涓庝富鏈虹殑 IP 鍦板潃銆傜劧鍚庯細

```
	ping <host's IP>
```
```
	ping <device's IP>
```
## 11. OBEX 鍔熻兘


璇ュ姛鑳界敱 usb_f_obex.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"obex"銆侽BEX 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彧鎻愪緵涓€涓睘鎬э細

	port_num

璇ュ睘鎬ф槸鍙鐨勩€?
绯荤粺涓渶澶氬彲浠ユ湁 4 涓?ACM/閫氱敤涓茶/OBEX 绔彛銆?
### 娴嬭瘯 OBEX 鍔熻兘


```
	seriald -f /dev/ttyGS<Y> -s 1024
```
```
	serialc -v <vendorID> -p <productID> -i<interface#> -a1 -s1024 \
                -t<out endpoint addr> -r<in endpoint addr>
```
鍏朵腑 seriald 涓?serialc 鏄?Felipe 鐨勫伐鍏凤紝鍙湪浠ヤ笅浣嶇疆鎵惧埌锛?
	https://github.com/felipebalbi/usb-tools.git master

## 12. PHONET 鍔熻兘


璇ュ姛鑳界敱 usb_f_phonet.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"phonet"銆侾HONET 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彧鎻愪緵涓€涓睘鎬э細

	=============== ==================================================
	ifname		涓庢湰鍔熻兘瀹炰緥鍏宠仈鐨勭綉缁滆澶囨帴鍙ｅ悕
	=============== ==================================================

### 娴嬭瘯 PHONET 鍔熻兘


娌℃湁鐗瑰畾鐨勭‖浠舵棤娉曟祴璇?SOCK_STREAM 鍗忚锛屽洜姝ゅ彧娴嬭瘯浜?SOCK_DGRAM銆傝浣垮悗鑰呭伐浣滐紝
杩囧幓鎴戜笉寰椾笉搴旂敤杩欓噷鎻愬埌鐨勮ˉ涓侊細

http://www.spinics.net/lists/linux-usb/msg85689.html

闇€瑕佽繖浜涘伐鍏凤細

git://git.gitorious.org/meego-cellular/phonet-utils.git

```
	$ ./phonet -a 0x10 -i usbpn0
	$ ./pnroute add 0x6c usbpn0
	$./pnroute add 0x10 usbpn0
	$ ifconfig usbpn0 up
```
```
	$ ./phonet -a 0x6c -i upnlink0
	$ ./pnroute add 0x10 upnlink0
	$ ifconfig upnlink0 up
```
```
	http://www.spinics.net/lists/linux-usb/msg85690.html
```
```
	$ ./pnxmit -a 0x6c -r
```
```
	$ ./pnxmit -a 0x10 -s 0x6c
```
缁撴灉搴旀湁涓€浜涙暟鎹粠涓绘満鍙戦€佸埌璁惧銆傜劧鍚庡弽杩囨潵锛?
```
	$ ./pnxmit -a 0x10 -r
```
```
	$ ./pnxmit -a 0x6c -s 0x10
```
## 13. RNDIS 鍔熻兘


璇ュ姛鑳界敱 usb_f_rndis.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"rndis"銆俁NDIS 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	=============== ==================================================
	ifname		涓庢湰鍔熻兘瀹炰緥鍏宠仈鐨勭綉缁滆澶囨帴鍙ｅ悕
	qmult		楂橀€熶笌瓒呴€熶笅鐨勯槦鍒楅暱搴︿箻鏁?	host_addr	鏈?USB 浠ュお閾捐矾涓婁富鏈轰竴渚х殑 MAC 鍦板潃
	dev_addr		鏈?USB 浠ュお閾捐矾涓婅澶囦竴渚х殑 MAC 鍦板潃
	=============== ==================================================

鍦ㄥ垱寤?functions/rndis.<瀹炰緥鍚? 涔嬪悗锛屽畠浠寘鍚粯璁ゅ€硷細qmult 涓?5锛宒ev_addr 涓?host_addr 涓洪殢鏈洪€夋嫨銆傚鏋滃姛鑳芥湭缁戝畾锛宨fname 鍙鍐欏叆銆傚啓鍏ュ唴瀹瑰繀椤绘槸涓€涓帴鍙ｆā寮忥紝
渚嬪 "usb%d"锛岃繖灏嗗鑷寸綉缁滄牳蹇冮€夋嫨涓嬩竴涓┖闂茬殑 usbX 鎺ュ彛銆傞粯璁ゆ儏鍐典笅瀹冭璁句负 "usb%d"銆?
### 娴嬭瘯 RNDIS 鍔熻兘


閰嶇疆璁惧涓庝富鏈虹殑 IP 鍦板潃銆傜劧鍚庯細

```
	ping <host's IP>
```
```
	ping <device's IP>
```
## 14. SERIAL 鍔熻兘


璇ュ姛鑳界敱 usb_f_gser.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"gser"銆係ERIAL 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彧鎻愪緵涓€涓睘鎬э細

	port_num

璇ュ睘鎬ф槸鍙鐨勩€?
绯荤粺涓渶澶氬彲浠ユ湁 4 涓?ACM/閫氱敤涓茶/OBEX 绔彛銆?
### 娴嬭瘯 SERIAL 鍔熻兘


```
	insmod usbserial
	echo VID PID >/sys/bus/usb-serial/drivers/generic/new_id
```
```
	cat > /dev/ttyUSB<X>
```
```
	cat /dev/ttyGS<Y>
```
鐒跺悗鍙嶈繃鏉?
```
	cat > /dev/ttyGS<Y>
```
```
	cat /dev/ttyUSB<X>
```
## 15. SOURCESINK 鍔熻兘


璇ュ姛鑳界敱 usb_f_ss_lb.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"SourceSink"銆係OURCESINK 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓?灞炴€э細

	=============== ==================================
	pattern		0锛堝叏闆讹級銆?锛坢od63锛夈€?锛堟棤锛?	isoc_interval	1..16
	isoc_maxpacket	0 - 1023锛坒s锛夈€? - 1024锛坔s/ss锛?	isoc_mult	0..2锛堜粎 hs/ss锛?	isoc_maxburst	0..15锛堜粎 ss锛?	bulk_buflen	缂撳啿鍖洪暱搴?	bulk_maxburst	0..15锛堜粎 ss锛?	bulk_qlen	鎵归噺闃熷垪娣卞害
	iso_qlen	绛夋椂闃熷垪娣卞害
	=============== ==================================

### 娴嬭瘯 SOURCESINK 鍔熻兘


璁惧绔細杩愯 gadget

涓绘満绔細test-usb锛坱ools/usb/testusb.c锛?

## 16. UAC1 鍔熻兘锛堟棫瀹炵幇锛?

璇ュ姛鑳界敱 usb_f_uac1_legacy.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"uac1_legacy"銆倁ac1 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	=============== ====================================
	audio_buf_size	闊抽缂撳啿鍖哄ぇ灏?	fn_cap		閲囬泦 pcm 璁惧鏂囦欢鍚?	fn_cntl		鎺у埗璁惧鏂囦欢鍚?	fn_play		鎾斁 pcm 璁惧鏂囦欢鍚?	req_buf_size	ISO OUT 绔偣璇锋眰缂撳啿鍖哄ぇ灏?	req_count	ISO OUT 绔偣璇锋眰璁℃暟
	=============== ====================================

杩欎簺灞炴€ч兘鏈夊悎鐞嗙殑榛樿鍊笺€?
### 娴嬭瘯 UAC1 鍔熻兘


璁惧绔細杩愯 gadget

```
	aplay -l # 搴斿垪鍑烘垜浠殑 USB Audio Gadget
```
## 17. UAC2 鍔熻兘


璇ュ姛鑳界敱 usb_f_uac2.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"uac2"銆倁ac2 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	================ ====================================================
	c_chmask         閲囬泦閫氶亾鎺╃爜
	c_srate          閲囬泦閲囨牱鐜囧垪琛紙閫楀彿鍒嗛殧锛?	c_ssize          閲囬泦閲囨牱澶у皬锛堝瓧鑺傦級
	c_sync           閲囬泦鍚屾绫诲瀷锛坅sync/adaptive锛?	c_mute_present   閲囬泦闈欓煶鎺у埗浣胯兘
	c_volume_present 閲囬泦闊抽噺鎺у埗浣胯兘
	c_volume_min     閲囬泦闊抽噺鎺у埗鏈€灏忓€硷紙鍗曚綅 1/256 dB锛?	c_volume_max     閲囬泦闊抽噺鎺у埗鏈€澶у€硷紙鍗曚綅 1/256 dB锛?	c_volume_res     閲囬泦闊抽噺鎺у埗鍒嗚鲸鐜囷紙鍗曚綅 1/256 dB锛?	c_hs_bint        閲囬泦 HS/SS 鐨?bInterval锛?-4锛氬浐瀹氾紝0锛氳嚜鍔級
	fb_max           寮傛妯″紡涓嬬殑鏈€澶ч澶栧甫瀹?	p_chmask         鎾斁閫氶亾鎺╃爜
	p_srate          鎾斁閲囨牱鐜囧垪琛紙閫楀彿鍒嗛殧锛?	p_ssize          鎾斁閲囨牱澶у皬锛堝瓧鑺傦級
	p_mute_present   鎾斁闈欓煶鎺у埗浣胯兘
	p_volume_present 鎾斁闊抽噺鎺у埗浣胯兘
	p_volume_min     鎾斁闊抽噺鎺у埗鏈€灏忓€硷紙鍗曚綅 1/256 dB锛?	p_volume_max     鎾斁闊抽噺鎺у埗鏈€澶у€硷紙鍗曚綅 1/256 dB锛?	p_volume_res     鎾斁闊抽噺鎺у埗鍒嗚鲸鐜囷紙鍗曚綅 1/256 dB锛?	p_hs_bint        鎾斁 HS/SS 鐨?bInterval锛?-4锛氬浐瀹氾紝0锛氳嚜鍔級
	req_number       涓洪噰闆嗕笌鎾斁棰勫垎閰嶇殑璇锋眰鏁伴噺
	function_name    鎺ュ彛鍚嶇О
	if_ctrl_name     鎷撴墤鎺у埗鍚嶇О
	clksrc_in_name   杈撳叆鏃堕挓鍚嶇О
	clksrc_out_name  杈撳嚭鏃堕挓鍚嶇О
	p_it_name        鎾斁杈撳叆缁堢鍚嶇О
	p_it_ch_name     鎾斁杈撳叆棣栭€氶亾鍚嶇О
	p_ot_name        鎾斁杈撳嚭缁堢鍚嶇О
	p_fu_vol_name    鎾斁鍔熻兘鍗曞厓鍚嶇О
	c_it_name        閲囬泦杈撳叆缁堢鍚嶇О
	c_it_ch_name     閲囬泦杈撳叆棣栭€氶亾鍚嶇О
	c_ot_name        閲囬泦杈撳嚭缁堢鍚嶇О
	c_fu_vol_name    閲囬泦鍔熻兘鍗曞厓鍚嶇О
	c_terminal_type  閲囬泦缁堢绫诲瀷浠ｇ爜
	p_terminal_type  鎾斁缁堢绫诲瀷浠ｇ爜
	================ ====================================================

杩欎簺灞炴€ч兘鏈夊悎鐞嗙殑榛樿鍊笺€?
### 娴嬭瘯 UAC2 鍔熻兘


璁惧绔細杩愯 gadget
涓绘満绔細aplay -l # 搴斿垪鍑烘垜浠殑 USB Audio Gadget

璇ュ姛鑳戒笉闇€瑕佺湡瀹炵殑纭欢鏀寔锛屽畠鍙槸鍚戜富鏈哄彂閫?浠庝富鏈烘帴鏀朵竴娈甸煶棰戞暟鎹祦銆備负浜嗙湡姝ｅ湪
璁惧绔惉鍒板０闊筹紝鍙互浣跨敤绫讳技濡備笅鐨勫懡浠?```
	$ arecord -f dat -t wav -D hw:2,0 | aplay -D hw:0,0 &
```
```
	$ arecord -f dat -t wav -D hw:CARD=UAC2Gadget,DEV=0 | \
	  aplay -D default:CARD=OdroidU3
```
## 18. UVC 鍔熻兘


璇ュ姛鑳界敱 usb_f_uvc.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"uvc"銆倁vc 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	=================== ================================================
	streaming_interval  杞绔偣浠ヨ繘琛屾暟鎹紶杈撶殑闂撮殧
	streaming_maxburst  瓒呴€熶即闅忔弿杩扮涓殑 bMaxBurst
	streaming_maxpacket 閫夋嫨姝ら厤缃椂璇ョ鐐硅兘澶熷彂閫佹垨鎺ユ敹鐨勬渶澶у寘澶у皬
	function_name       鎺ュ彛鍚嶇О
	=================== ================================================

杩樻湁 "control" 涓?"streaming" 涓や釜瀛愮洰褰曪紝姣忎釜閮藉寘鍚竴瀹氭暟閲忕殑瀛愮洰褰曘€傛彁渚涗簡涓€浜?鍚堢悊鐨勯粯璁ゅ€硷紝浣嗙敤鎴峰繀椤绘彁渚涗互涓嬪唴瀹癸細

	================== ====================================================
	control header     鍦?control/header 涓垱寤猴紝浠?control/class/fs
			   鍜?鎴?control/class/ss 閾炬帴
	streaming header   鍦?streaming/header 涓垱寤猴紝浠?			   streaming/class/fs 鍜?鎴?streaming/class/hs 鍜?鎴?			   streaming/class/ss 閾炬帴
	format description 鍦?streaming/mjpeg 鍜?鎴?			   streaming/uncompressed 涓垱寤?	frame description  鍦?streaming/mjpeg/<format> 鍜?鎴?			   streaming/uncompressed/<format> 涓垱寤?	================== ====================================================

姣忎釜甯ф弿杩伴兘鍖呭惈甯ч棿闅旇鑼冿紝鑰屾瘡涓繖鏍风殑瑙勮寖鐢辫嫢骞插甫闂撮殧鍊肩殑琛岀粍鎴?```
  # mkdir functions/uvc.usb0/control/header/h
  # cd functions/uvc.usb0/control/
  # ln -s header/h class/fs
  # ln -s header/h class/ss
  # mkdir -p functions/uvc.usb0/streaming/uncompressed/u/360p
  # cat <<EOF > functions/uvc.usb0/streaming/uncompressed/u/360p/dwFrameInterval
  666666
  1000000
  5000000
  EOF
  # cd $GADGET_CONFIGFS_ROOT
  # mkdir functions/uvc.usb0/streaming/header/h
  # cd functions/uvc.usb0/streaming/header/h
  # ln -s ../../uncompressed/u
  # cd ../../class/fs
  # ln -s ../../header/h
  # cd ../../class/hs
  # ln -s ../../header/h
  # cd ../../class/ss
  # ln -s ../../header/h
```
### 娴嬭瘯 UVC 鍔熻兘


```
  # uvc-gadget -u /dev/video<uvc video node #> -v /dev/video<vivid video node #>
```
鍏朵腑 uvc-gadget 鏄繖涓▼搴忥細
	http://git.ideasonboard.org/uvc-gadget.git

搴旂敤杩欎簺琛ヤ竵锛?
	https://lore.kernel.org/r/1386675637-18243-1-git-send-email-r.baldyga@samsung.com/

```
	luvcview -f yuv
```
## 19. PRINTER 鍔熻兘


璇ュ姛鑳界敱 usb_f_printer.ko 妯″潡鎻愪緵銆?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"printer"銆俻rinter 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	==========	===========================================
	pnp_string	鍦?pnp 瀛楃涓蹭腑浼犻€掔粰涓绘満鐨勬暟鎹?	q_len		姣忎釜绔偣鐨勮姹傛暟
	==========	===========================================

### 娴嬭瘯 PRINTER 鍔熻兘


鏈€鍩烘湰鐨勬祴璇曪細

```
	# ls -l /devices/virtual/usb_printer_gadget/
```
搴旀樉绀?g_printer<number>銆?
濡傛灉 udev 澶勪簬娲诲姩鐘舵€侊紝鍒?/dev/g_printer<number> 搴旇嚜鍔ㄥ嚭鐜般€?
涓绘満绔細

濡傛灉 udev 澶勪簬娲诲姩鐘舵€侊紝鍒欎緥濡?/dev/usb/lp0 搴斿嚭鐜般€?
涓绘満鍒拌澶囦紶杈擄細

```
	# cat /dev/g_printer<number>
```
```
	# cat > /dev/usb/lp0
```
```
	# cat > /dev/g_printer<number>
```
```
	# cat /dev/usb/lp0
```
鏇撮珮绾х殑娴嬭瘯鍙互浣跨敤 Documentation/usb/gadget_printer.rst 涓弿杩扮殑 prn_example 杩涜銆?

## 20. UAC1 鍔熻兘锛堣櫄鎷?ALSA 澹板崱锛屼娇鐢?u_audio API锛?

璇ュ姛鑳界敱 usb_f_uac1.ko 妯″潡鎻愪緵銆?瀹冨皢鍒涘缓涓€涓櫄鎷?ALSA 澹板崱锛岄煶棰戞祦绠€鍗曞湴姹囧叆/婧愯嚜璇ュ０鍗°€?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"uac1"銆倁ac1 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬪睘鎬э細

	================ ====================================================
	c_chmask         閲囬泦閫氶亾鎺╃爜
	c_srate          閲囬泦閲囨牱鐜囧垪琛紙閫楀彿鍒嗛殧锛?	c_ssize          閲囬泦閲囨牱澶у皬锛堝瓧鑺傦級
	c_mute_present   閲囬泦闈欓煶鎺у埗浣胯兘
	c_volume_present 閲囬泦闊抽噺鎺у埗浣胯兘
	c_volume_min     閲囬泦闊抽噺鎺у埗鏈€灏忓€硷紙鍗曚綅 1/256 dB锛?	c_volume_max     閲囬泦闊抽噺鎺у埗鏈€澶у€硷紙鍗曚綅 1/256 dB锛?	c_volume_res     閲囬泦闊抽噺鎺у埗鍒嗚鲸鐜囷紙鍗曚綅 1/256 dB锛?	p_chmask         鎾斁閫氶亾鎺╃爜
	p_srate          鎾斁閲囨牱鐜囧垪琛紙閫楀彿鍒嗛殧锛?	p_ssize          鎾斁閲囨牱澶у皬锛堝瓧鑺傦級
	p_mute_present   鎾斁闈欓煶鎺у埗浣胯兘
	p_volume_present 鎾斁闊抽噺鎺у埗浣胯兘
	p_volume_min     鎾斁闊抽噺鎺у埗鏈€灏忓€硷紙鍗曚綅 1/256 dB锛?	p_volume_max     鎾斁闊抽噺鎺у埗鏈€澶у€硷紙鍗曚綅 1/256 dB锛?	p_volume_res     鎾斁闊抽噺鎺у埗鍒嗚鲸鐜囷紙鍗曚綅 1/256 dB锛?	req_number       涓洪噰闆嗕笌鎾斁棰勫垎閰嶇殑璇锋眰鏁伴噺
	function_name    鎺ュ彛鍚嶇О
	p_it_name        鎾斁杈撳叆缁堢鍚嶇О
	p_it_ch_name     鎾斁閫氶亾鍚嶇О
	p_ot_name        鎾斁杈撳嚭缁堢鍚嶇О
	p_fu_vol_name    鎾斁闈欓煶/闊抽噺鍔熻兘鍗曞厓鍚嶇О
	c_it_name        閲囬泦杈撳叆缁堢鍚嶇О
	c_it_ch_name     閲囬泦閫氶亾鍚嶇О
	c_ot_name        閲囬泦杈撳嚭缁堢鍚嶇О
	c_fu_vol_name    閲囬泦闈欓煶/闊抽噺鍔熻兘鍗曞厓鍚嶇О
	================ ====================================================

杩欎簺灞炴€ч兘鏈夊悎鐞嗙殑榛樿鍊笺€?
### 娴嬭瘯 UAC1 鍔熻兘


璁惧绔細杩愯 gadget
涓绘満绔細aplay -l # 搴斿垪鍑烘垜浠殑 USB Audio Gadget

璇ュ姛鑳戒笉闇€瑕佺湡瀹炵殑纭欢鏀寔锛屽畠鍙槸鍚戜富鏈哄彂閫?浠庝富鏈烘帴鏀朵竴娈甸煶棰戞暟鎹祦銆備负浜嗙湡姝ｅ湪
璁惧绔惉鍒板０闊筹紝鍙互浣跨敤绫讳技濡備笅鐨勫懡浠?```
	$ arecord -f dat -t wav -D hw:2,0 | aplay -D hw:0,0 &
```
```
	$ arecord -f dat -t wav -D hw:CARD=UAC1Gadget,DEV=0 | \
	  aplay -D default:CARD=OdroidU3
```
## 21. MIDI2 鍔熻兘


璇ュ姛鑳界敱 usb_f_midi2.ko 妯″潡鎻愪緵銆?瀹冨皢鍒涘缓涓€涓寘鍚?UMP rawmidi 璁惧鐨勮櫄鎷?ALSA 澹板崱锛屽叾涓?UMP 鍖呰鍥炵幆銆傛澶栵紝杩樹細
鍒涘缓涓€涓紶缁熺殑 rawmidi 璁惧銆俇MP rawmidi 涔熶笌 ALSA sequencer 瀹㈡埛绔粦瀹氥€?
### 鍔熻兘鐗瑰畾鐨?configfs 鎺ュ彛


鍒涘缓鍔熻兘鐩綍鏃惰浣跨敤鐨勫姛鑳藉悕鏄?"midi2"銆俶idi2 鍔熻兘鍦ㄥ叾鍔熻兘鐩綍涓彁渚涗互涓嬩綔涓哄０鍗?椤跺眰淇℃伅鐨勫睘鎬э細

	=============	=================================================
	process_ump	鐢ㄤ簬澶勭悊 UMP Stream 娑堟伅鐨勫竷灏旀爣蹇楋紙0 鎴?1锛?	static_block	鐢ㄤ簬闈欐€佸潡鐨勫竷灏旀爣蹇楋紙0 鎴?1锛?	iface_name	鍙€夌殑鎺ュ彛鍚嶇О瀛楃涓?	=============	=================================================

璇ョ洰褰曞寘鍚竴涓?"ep.0" 瀛愮洰褰曪紝瀹冩彁渚?UMP Endpoint锛堜竴瀵?USB MIDI 绔偣锛夌殑灞炴€э細

	=============	=================================================
	protocol_caps	MIDI 鍗忚鑳藉姏锛?			1锛歁IDI 1.0锛?锛歁IDI 2.0锛屾垨 3锛氫袱绉嶅崗璁?	protocol	榛樿 MIDI 鍗忚锛? 鎴?2锛?	ep_name		UMP Endpoint 鍚嶇О瀛楃涓?	product_id	浜у搧 ID 瀛楃涓?	manufacturer	鍒堕€犲晢 ID 鍙凤紙24 浣嶏級
	family		璁惧绯诲垪 ID 鍙凤紙16 浣嶏級
	model		璁惧鍨嬪彿 ID 鍙凤紙16 浣嶏級
	sw_revision	杞欢鐗堟湰锛?2 浣嶏級
	=============	=================================================

姣忎釜 Endpoint 瀛愮洰褰曞寘鍚竴涓?"block.0" 瀛愮洰褰曪紝瀹冧唬琛?Block 0 淇℃伅鐨?Function Block銆?鍏跺睘鎬т负锛?
	=================	===============================================
	name			Function Block 鍚嶇О瀛楃涓?	direction		璇?FB 鐨勬柟鍚?				1锛氳緭鍏ワ紝2锛氳緭鍑猴紝鎴?3锛氬弻鍚?	first_group		棣栦釜 UMP Group 缂栧彿锛?-15锛?	num_groups		璇?FB 涓殑 group 鏁伴噺锛?-16锛?	midi1_first_group	MIDI 1.0 鐨勯涓?UMP Group 缂栧彿锛?-15锛?	midi1_num_groups	MIDI 1.0 鐨?group 鏁伴噺锛?-16锛?	ui_hint			璇?FB 鐨?UI 鎻愮ず
				0锛氭湭鐭ワ紝1锛氭帴鏀舵柟锛?锛氬彂閫佹柟锛?锛氫袱鑰?	midi_ci_version		鏀寔鐨?MIDI-CI 鐗堟湰鍙凤紙8 浣嶏級
	is_midi1		浼犵粺 MIDI 1.0 璁惧锛?-2锛?				0锛歁IDI 2.0 璁惧锛?				1锛氭棤闄愬埗鐨?MIDI 1.0锛屾垨
				2锛氫綆閫熺殑 MIDI 1.0
	sysex8_streams		SysEx8 娴佺殑鏈€澶ф暟閲忥紙8 浣嶏級
	active			鎸囩ず FB 娲诲姩鐘舵€佺殑甯冨皵鏍囧織锛? 鎴?1锛?	=================	===============================================

濡傛灉闇€瑕佸涓?Function Block锛屽彲浠ラ€氳繃鍒涘缓甯︾浉搴?Function Block 缂栧彿锛?銆?銆佲€︹€︼級鐨?"block.<num>" 瀛愮洰褰曟潵娣诲姞鏇村 Function Block銆侳B 瀛愮洰褰曚篃鍙互鍔ㄦ€佺Щ闄ゃ€傛敞鎰?Function
Block 缂栧彿蹇呴』鏄繛缁殑銆?
绫讳技鍦帮紝濡傛灉闇€瑕佸涓?UMP Endpoint锛屽彲浠ラ€氳繃鍒涘缓 "ep.<num>" 瀛愮洰褰曟潵娣诲姞鏇村 Endpoint銆?缂栧彿蹇呴』鏄繛缁殑銆?
涓轰簡妯℃嫙涓嶆敮鎸?UMP v1.1 鐨勬棫 MIDI 2.0 璁惧锛屽皢 0 浼犵粰 `process_ump` 鏍囧織銆傝繖鏍锋暣涓?UMP v1.1 璇锋眰閮戒細琚拷鐣ャ€?
### 娴嬭瘯 MIDI2 鍔熻兘


```
  $ cat /proc/asound/cards
```
灏嗘樉绀轰竴涓寘鍚?MIDI2 璁惧鐨勬柊澹板崱銆?
```
  $ cat /proc/asound/cards
```
灏嗘樉绀轰竴涓寘鍚?MIDI1 鎴?MIDI2 璁惧鐨勬柊澹板崱锛屽彇鍐充簬 USB 闊抽椹卞姩鐨勯厤缃€?
鍦ㄤ袱鑰呬笂锛屽綋涓绘満鍚敤浜?ALSA sequencer 鏃讹紝浣犲彲浠ユ壘鍒拌濡?"MIDI 2.0 Gadget" 杩欐牱鐨?UMP MIDI 瀹㈡埛绔€?
鐢变簬椹卞姩鍙槸鍥炵幆鏁版嵁锛屾祴璇曟椂涓嶉渶瑕佺湡瀹炶澶囥€?
涓轰簡娴嬭瘯浠?gadget 鍒颁富鏈虹殑 MIDI 杈撳叆锛堜緥濡傛ā鎷?MIDI 閿洏锛夛紝浣犲彲浠ュ彂閫佸涓嬬殑 MIDI
娴併€?
```
  $ aconnect -o
  ....
  client 20: 'MIDI 2.0 Gadget' [type=kernel,card=1]
      0 'MIDI 2.0        '
      1 'Group 1 (MIDI 2.0 Gadget I/O)'
  $ aplaymidi -p 20:1 to_host.mid
```
```
  $ aconnect -i
  ....
  client 24: 'MIDI 2.0 Gadget' [type=kernel,card=2]
      0 'MIDI 2.0        '
      1 'Group 1 (MIDI 2.0 Gadget I/O)'
  $ arecordmidi -p 24:1 from_gadget.mid
```
濡傛灉浣犳湁鏀寔 UMP 鐨勫簲鐢ㄧ▼搴忥紝涔熷彲浠ヤ娇鐢?UMP 绔彛鏉ュ彂閫?鎺ユ敹鍘熷 UMP 鍖呫€備緥濡?aseqdump
绋嬪簭
```
  $ aseqdump -u 2 -p 24:1
  Waiting for data. Press Ctrl+C to end.
  Source  Group    Event                  Ch  Data
   24:1   Group  0, Program change          0, program 0, Bank select 0:0
   24:1   Group  0, Channel pressure        0, value 0x80000000
```
涓轰簡娴嬭瘯鍒?gadget 鐨?MIDI 杈撳嚭锛堜緥濡傛ā鎷?MIDI 鍚堟垚鍣級锛屽彧闇€鍙嶈繃鏉ュ嵆鍙€?
```
  $ arecordmidi -p 20:1 from_host.mid
```
```
  $ aplaymidi -p 24:1 to_gadget.mid
```
涓绘満绔 altset 0 涓?MIDI 1.0 鐨勮闂彈鏀寔锛屽苟涓斿畠浼氬湪 gadget 涓婅杞崲涓?UMP 鍖呫€傚畠
鍙粦瀹氬埌 Function Block 0銆?
褰撳墠鐨勬搷浣滄ā寮忓彲浠ュ湪 ALSA 鎺у埗鍏冪礌涓瀵熷埌
```
  $ amixer -c1 contents
  numid=1,iface=RAWMIDI,name='Operation Mode'
    ; type=INTEGER,access=r--v----,values=1,min=0,max=2,step=0
    : values=2
```
鍏朵腑 0 = 鏈娇鐢紝1 = MIDI 1.0锛坅ltset 0锛夛紝2 = MIDI 2.0锛坅ltset 1锛夈€備笂闈㈢殑渚嬪瓙鏄剧ず
瀹冭繍琛屽湪 2锛屽嵆 MIDI 2.0銆?