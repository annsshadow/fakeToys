## 閫氳繃 configfs 閰嶇疆鐨?Linux USB gadget


25th April 2013



## 姒傝堪


涓€涓?Linux USB Gadget 鏄嫢鏈?UDC锛圲SB Device Controller锛孶SB 璁惧鎺у埗鍣級鐨勮澶囷紝鍙互
杩炴帴鍒?USB 涓绘満锛圚ost锛夛紝浠ユ墿灞曞叾鍔熻兘锛屼緥濡備覆鍙ｆ垨澶у閲忓瓨鍌ㄨ兘鍔涖€?
浠庝富鏈虹殑瑙掑害鐪嬶紝涓€涓?gadget 鏄竴缁勯厤缃紙configuration锛夛紝姣忎釜閰嶇疆鍖呭惈鑻ュ共鎺ュ彛锛坕nterface锛夛紝
浠?gadget 鐨勮搴︾湅锛岃繖浜涙帴鍙ｈ绉颁负鍔熻兘锛坒unction锛夛紝姣忎釜鍔熻兘浠ｈ〃渚嬪涓€涓覆琛岃繛鎺ユ垨涓€鍧?SCSI 纾佺洏銆?
Linux 涓?gadget 鎻愪緵浜嗕竴绯诲垪鍙緵浣跨敤鐨勫姛鑳姐€?
鍒涘缓涓€涓?gadget 鎰忓懗鐫€鍐冲畾浼氭湁鍝簺閰嶇疆锛屼互鍙婃瘡涓厤缃細鎻愪緵鍝簺鍔熻兘銆?
Configfs锛堣鍙傝 `Documentation/filesystems/configfs.rst`锛夐潪甯搁€傚悎鐢ㄤ簬鍚戝唴鏍稿憡鐭ヤ笂杩板喅绛栥€?鏈枃妗ｈ杩板浣曞仛鍒拌繖涓€鐐广€?
瀹冭繕鎻忚堪浜?configfs 涓?gadget 鐨勯泦鎴愭槸濡備綍璁捐鐨勩€?


## 瑕佹眰


涓轰簡璁╄繖涓€鍒囧伐浣滐紝configfs 蹇呴』鍙敤锛屽洜姝ゅ湪 .config 涓?CONFIGFS_FS 蹇呴』涓?'y' 鎴?'m'銆?鎴嚦鏈枃鎾板啓鏃讹紝USB_LIBCOMPOSITE 浼氳嚜鍔ㄩ€夋嫨 CONFIGFS_FS銆?


## 鐢ㄦ硶


锛堟弿杩伴涓€氳繃 configfs 鍙敤鐨勫姛鑳界殑鍘熷甯栧瓙鍙互鍦ㄨ繖閲岀湅鍒帮細
http://www.spinics.net/lists/linux-usb/msg76388.html锛?
```

	$ modprobe libcomposite
	$ mount none $CONFIGFS_HOME -t configfs

```
鍏朵腑 CONFIGFS_HOME 鏄?configfs 鐨勬寕杞界偣

### 1. 鍒涘缓 gadget


```

	$ mkdir $CONFIGFS_HOME/usb_gadget/<gadget name>

```
```

	$ mkdir $CONFIGFS_HOME/usb_gadget/g1

	...
	...
	...

	$ cd $CONFIGFS_HOME/usb_gadget/g1

```
```

	$ echo <VID> > idVendor
	$ echo <PID> > idProduct

```
涓€涓?gadget 杩橀渶瑕佸畠鐨勫簭鍒楀彿銆佸巶鍟嗗悕涓庝骇鍝佸悕瀛楃涓层€備负浜嗘湁鍦版柟瀛樻斁瀹冧滑锛屽繀椤诲垱寤轰竴涓?strings 瀛愮洰褰?```

	$ mkdir strings/0x409

```
```

	$ echo <serial number> > strings/0x409/serialnumber
	$ echo <manufacturer> > strings/0x409/manufacturer
	$ echo <product> > strings/0x409/product

```
杩涗竴姝ョ殑鑷畾涔夊瓧绗︿覆鎻忚堪绗﹀彲浠ヤ綔涓鸿璇█鐩綍涓嬬殑瀛愮洰褰曞垱寤猴紝瀛楃涓叉枃鏈鍐欏叆 "s" 灞炴€?```

	$ mkdir strings/0x409/xu.0
	$ echo <string text> > strings/0x409/xu.0/s

```
鍦ㄥ姛鑳介┍鍔ㄦ敮鎸佺殑鎯呭喌涓嬶紝鍔熻兘鍙互鍏佽鍒涘缓鎸囧悜杩欎簺鑷畾涔夊瓧绗︿覆鎻忚堪绗︾殑绗﹀彿閾炬帴锛屼互灏嗚繖浜?瀛楃涓蹭笌绫绘弿杩扮鍏宠仈璧锋潵銆?
### 2. 鍒涘缓閰嶇疆


姣忎釜 gadget 鐢辫嫢骞查厤缃粍鎴愶紝瀹冧滑鐩稿簲鐨?```

        $ mkdir configs/<name>.<number>

```
鍏朵腑 <name> 鍙互鏄枃浠剁郴缁熶腑鍚堟硶鐨勪换鎰忓瓧绗︿覆锛岃€?```

	$ mkdir configs/c.1

	...
	...
	...

```
姣忎釜閰嶇疆涔熼渶瑕佸畠鑷繁鐨勫瓧绗︿覆锛屽洜姝ゅ繀椤诲垱寤轰竴涓瓙鐩綍
```

	$ mkdir configs/c.1/strings/0x409

```
```

	$ echo <configuration> > configs/c.1/strings/0x409/configuration

```
```

	$ echo 120 > configs/c.1/MaxPower

```
### 3. 鍒涘缓鍔熻兘


璇?gadget 灏嗘彁渚涗竴浜涘姛鑳斤紝姣忎釜鍔熻兘瀵瑰簲鐨?```

	$ mkdir functions/<name>.<instance name>

```
鍏朵腑 <name> 瀵瑰簲浜庢煇涓厑璁哥殑鍔熻兘鍚嶏紝instance name锛堝疄渚嬪悕锛?```

  $ mkdir functions/ncm.usb0 # usb_f_ncm.ko gets loaded with request_module()

  ...
  ...
  ...

```
姣忎釜鍔熻兘鎻愪緵鍏剁壒瀹氱殑涓€缁勫睘鎬э紝鍙互鏄彧璇绘垨璇诲啓璁块棶銆傚湪閫傜敤鐨勬儏鍐典笅锛岄渶瑕佷互閫傚綋鐨勬柟寮?鍐欏叆瀹冧滑銆傛洿澶氫俊鎭鍙傝€?Documentation/ABI/testing/configfs-usb-gadget銆?
### 4. 灏嗗姛鑳戒笌閰嶇疆鍏宠仈


姝ゅ埢宸茬粡鍒涘缓浜嗚嫢骞?gadget锛屾瘡涓?gadget 閮芥寚瀹氫簡鑻ュ共閰嶇疆骞舵彁渚涗簡鑻ュ共鍙敤鍔熻兘銆傚墿涓嬬殑灏辨槸
鎸囧畾鍝釜鍔熻兘鍦ㄥ摢涓厤缃腑鍙敤锛堝悓涓€涓姛鑳藉彲浠ュ湪澶氫釜閰嶇疆涓娇鐢級銆傝繖閫氳繃浠ヤ笅鏂瑰紡瀹炵幇
```

	$ ln -s functions/<name>.<instance name> configs/<name>.<number>

```
```

	$ ln -s functions/ncm.usb0 configs/c.1

	...
	...
	...

```
### 5. 鍚敤 gadget


浠ヤ笂鎵€鏈夋楠ょ殑鐩殑閮芥槸缁勫悎鍑虹敱閰嶇疆涓庡姛鑳芥瀯鎴愮殑 gadget銆?
```

  .
  ./strings
  ./strings/0x409
  ./strings/0x409/serialnumber
  ./strings/0x409/product
  ./strings/0x409/manufacturer
  ./configs
  ./configs/c.1
  ./configs/c.1/ncm.usb0 -> ../../../../usb_gadget/g1/functions/ncm.usb0
  ./configs/c.1/strings
  ./configs/c.1/strings/0x409
  ./configs/c.1/strings/0x409/configuration
  ./configs/c.1/bmAttributes
  ./configs/c.1/MaxPower
  ./functions
  ./functions/ncm.usb0
  ./functions/ncm.usb0/ifname
  ./functions/ncm.usb0/qmult
  ./functions/ncm.usb0/host_addr
  ./functions/ncm.usb0/dev_addr
  ./UDC
  ./bcdUSB
  ./bcdDevice
  ./idProduct
  ./idVendor
  ./bMaxPacketSize0
  ./bDeviceProtocol
  ./bDeviceSubClass
  ./bDeviceClass


```
杩欐牱涓€涓?gadget 鏈€缁堝繀椤昏鍚敤锛岃繖鏍?USB 涓绘満鎵嶈兘鏋氫妇瀹冦€?
涓轰簡鍚敤 gadget锛屽繀椤诲皢瀹冪粦瀹氬埌涓€涓?UDC锛圲SB Device Controller锛?```

	$ echo <udc name> > UDC

```
鍏朵腑 <udc name> 鏄?/sys/class/udc/* 涓壘鍒扮殑鍚嶅瓧涔嬩竴
```

	$ echo s3c-hsotg > UDC


```
### 6. 绂佺敤 gadget


```

	$ echo "" > UDC

```
### 7. 娓呯悊


```

	$ rm configs/<config name>.<number>/<function>

```
鍏朵腑 <config name>.<number> 鎸囧畾閰嶇疆锛?function> 鏄?```

	$ rm configs/c.1/ncm.usb0

	...
	...
	...

```
```

	$ rmdir configs/<config name>.<number>/strings/<lang>

```
```

	$ rmdir configs/c.1/strings/0x409

	...
	...
	...

```
```

	$ rmdir configs/<config name>.<number>

```
```

	rmdir configs/c.1

	...
	...
	...

```
```

	$ rmdir functions/<name>.<instance name>

```
```

	$ rmdir functions/ncm.usb0

	...
	...
	...

```
```

	$ rmdir strings/<lang>

```
```

	$ rmdir strings/0x409

```
```

	$ cd ..
	$ rmdir <gadget name>

```
```

	$ rmdir g1



```
## 瀹炵幇璁捐


涓嬮潰浠嬬粛 configfs 鏄浣曞伐浣滅殑銆傚湪 configfs 涓湁 item锛堥」锛変笌 group锛堢粍锛夛紝涓よ€呴兘琛ㄧず涓?鐩綍銆俰tem 涓?group 鐨勫尯鍒湪浜庯紝group 鍙互鍖呭惈鍏跺畠鐨?group銆備笅闈㈢殑鍥句腑鍙樉绀轰簡涓€涓?item銆?item 涓?group 閮藉彲浠ユ湁灞炴€э紙attribute锛夛紝瀹冧滑琛ㄧず涓烘枃浠躲€傜敤鎴峰彲浠ュ垱寤哄拰鍒犻櫎鐩綍锛屼絾涓嶈兘
鍒犻櫎鏂囦欢锛屾枃浠跺彲浠ユ槸鍙鎴栬鍐欑殑锛屽彇鍐充簬瀹冧滑鎵€浠ｈ〃鐨勫唴瀹广€?
configfs 鐨勬枃浠剁郴缁熼儴鍒嗘搷浣滅殑鏄?config_items/groups 涓?configfs_attributes锛屽畠浠浜?鎵€鏈夎閰嶇疆鐨勫厓绱犻兘鏄€氱敤鐨勩€佸悓涓€绫诲瀷鐨勩€傜劧鑰岋紝瀹冧滑琚唴宓屼簬鐗瑰畾鐢ㄩ€旂殑鏇村ぇ缁撴瀯涓€傚湪涓嬮潰鐨?鍥句腑鏈変竴涓?鈥渃s鈥濓紝瀹冨寘鍚竴涓?config_item锛屼互鍙婁竴涓?鈥渟a鈥濓紝瀹冨寘鍚竴涓?configfs_attribute銆?
```

  ./
  ./cs        (directory)
     |
     +--sa    (file)
     |
     .
     .
     .

```
姣忓綋鐢ㄦ埛璇诲彇/鍐欏叆 鈥渟a鈥?鏂囦欢鏃讹紝浼氳皟鐢ㄤ竴涓嚱鏁帮紝璇ュ嚱鏁版帴鍙椾竴涓?struct config_item 涓?涓€涓?struct configfs_attribute銆傚湪璇ュ嚱鏁颁腑锛屼娇鐢ㄤ紬鎵€鍛ㄧ煡鐨?container_of 鎶€鏈彇鍥?鈥渃s鈥?涓?鈥渟a鈥濓紝骞惰皟鐢ㄧ浉搴旂殑 sa 鍑芥暟锛坰how 鎴?store锛夛紝灏?鈥渃s鈥?涓庝竴涓瓧绗︾紦鍐插尯浼犵粰瀹冦€傗€渟how鈥?鐢ㄤ簬鏄剧ず鏂囦欢鐨勫唴瀹癸紙灏嗘暟鎹粠 cs 澶嶅埗鍒扮紦鍐插尯锛夛紝鑰?鈥渟tore鈥?鐢ㄤ簬淇敼鏂囦欢鐨勫唴瀹癸紙灏嗘暟鎹粠
缂撳啿鍖哄鍒跺埌 cs锛夛紝浣嗚繖涓や釜鍑芥暟瀹為檯鍋氫粈涔堢敱瀹炵幇鑰呭喅瀹氥€?
```

  typedef struct configured_structure cs;
  typedef struct specific_attribute sa;

                                         sa
                         +----------------------------------+
          cs             |  (*show)(cs *, buffer);          |
  +-----------------+    |  (*store)(cs *, buffer, length); |
  |                 |    |                                  |
  | +-------------+ |    |       +------------------+       |
  | | struct      |-|----|------>|struct            |       |
  | | config_item | |    |       |configfs_attribute|       |
  | +-------------+ |    |       +------------------+       |
  |                 |    +----------------------------------+
  | data to be set  |                .
  |                 |                .
  +-----------------+                .

```
鏂囦欢鍚嶇敱 config item/group 鐨勮璁¤€呭喅瀹氾紝鑰岀洰褰曚竴鑸彲浠ラ殢鎰忓懡鍚嶃€備竴涓?group 鍙互鏈夎嫢骞?榛樿瀛愮粍琚嚜鍔ㄥ垱寤恒€?
鏈夊叧 configfs 鐨勬洿澶氫俊鎭紝璇峰弬瑙?`Documentation/filesystems/configfs.rst`銆?
涓婅堪姒傚康鏄犲皠鍒?USB gadget 涓婂涓嬶細

1. 涓€涓?gadget 鏈夊畠鐨?config group锛屽畠鏈変竴浜涘睘鎬э紙idVendor銆乮dProduct 绛夛級浠ュ強榛樿瀛愮粍
   锛坈onfigs銆乫unctions銆乻trings锛夈€傚啓鍏ヨ繖浜涘睘鎬т細浣夸俊鎭瀛樺偍鍒伴€傚綋鐨勪綅缃€傚湪 configs銆?   functions 涓?strings 瀛愮粍涓紝鐢ㄦ埛鍙互鍒涘缓浠栦滑鑷繁鐨勫瓙缁勶紝浠ヨ〃绀虹粰瀹氳瑷€涓嬬殑閰嶇疆銆佸姛鑳?   涓庡瓧绗︿覆缁勩€?
2. 鐢ㄦ埛鍒涘缓閰嶇疆涓庡姛鑳斤紝骞跺湪閰嶇疆涓垱寤烘寚鍚戝姛鑳界殑绗﹀彿閾炬帴銆傝繖浜涗俊鎭湪鍐欏叆 gadget 鐨?UDC
   灞炴€ф椂琚娇鐢紝杩欐剰鍛崇潃灏?gadget 缁戝畾鍒?UDC銆俤rivers/usb/gadget/configfs.c 涓殑浠ｇ爜閬嶅巻
   鎵€鏈夐厤缃紝骞跺湪姣忎釜閰嶇疆涓亶鍘嗘墍鏈夊姛鑳藉苟灏嗗畠浠粦瀹氥€傝繖鏍锋暣涓?gadget 灏辫缁戝畾浜嗐€?
3. drivers/usb/gadget/configfs.c 鏂囦欢涓寘鍚敤浜庝互涓嬬敤閫旂殑浠ｇ爜锛?
 - gadget 鐨?config_group
 - gadget 鐨勯粯璁ょ粍锛坈onfigs銆乫unctions銆乻trings锛? - 灏嗗姛鑳戒笌閰嶇疆鍏宠仈锛堢鍙烽摼鎺ワ級

4. 姣忎釜 USB 鍔熻兘鑷劧鏈夊畠鑷繁鎯宠閰嶇疆鐨勫唴瀹圭殑瑙嗗浘锛屽洜姝ょ壒瀹氬姛鑳界殑 config_groups 瀹氫箟鍦ㄥ悇
   鍔熻兘鐨勫疄鐜版枃浠?drivers/usb/gadget/f_*.c 涓€?
5. 鍔熻兘鐨勪唬鐮佺紪鍐欐柟寮忎娇寰楀畠浣跨敤 usb_get_function_instance()锛岃€屽悗鑰呭張浼氳皟鐢?request_module銆?   鍥犳锛屽彧瑕?modprobe 鑳芥甯稿伐浣滐紝鐗瑰畾鍔熻兘鐨勬ā鍧楀氨浼氳鑷姩鍔犺浇銆傝娉ㄦ剰鍙嶄箣涓嶆垚绔嬶細鍦ㄤ竴涓?   gadget 琚鐢ㄥ苟鎷嗛櫎涔嬪悗锛屾ā鍧椾粛鐒朵繚鎸佸姞杞界姸鎬併€?