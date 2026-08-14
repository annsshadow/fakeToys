
## Spear PCIe Gadget 椹卞姩


## 浣滆€咃紙Author锛?
Pratyush Anand (pratyush.anand@gmail.com)

## 浣嶇疆锛圠ocation锛?
driver/misc/spear13xx_pcie_gadget.c

## 鏀寔鐨勮姱鐗囷紙Supported Chip锛夛細

SPEAr1300
SPEAr1310

## Menuconfig 閫夐」锛圡enuconfig option锛夛細

Device Drivers
	Misc devices
		PCIe gadget support for SPEAr13XX platform

## 鐢ㄩ€旓紙purpose锛?
璇ラ┍鍔ㄦ湁鑻ュ共涓彲鐢?configfs 鎺ュ彛璇诲啓鐨勮妭鐐癸紙node锛夈€傚叾涓昏鐩殑鏄皢鎵€閫夌殑鍙屾ā锛坉ual mode锛塒CIe 鎺у埗鍣ㄩ厤缃负璁惧锛坉evice锛夛紝鐒跺悗缂栫▼鍏跺悇绉嶅瘎瀛樺櫒锛屽皢鍏堕厤缃负鐗瑰畾鐨勮澶囩被鍨嬨€傝椹卞姩鍙敤浜庡睍绀?spear 鐨?PCIe 璁惧鑳藉姏銆?
## 涓嶅悓鑺傜偣鐨勬弿杩帮紙Description of different nodes锛夛細


### 鑺傜偣鐨勮琛屼负锛坮ead behavior of nodes锛夛細


=============== ==============================================================
link 		缁欏嚭 ltssm 鐘舵€併€?int_type 	鎵€鏀寔鐨勪腑鏂被鍨?no_of_msi 	鑻ヤ富鏈烘湭鍚敤 MSI 鍒欎负 0銆傛鍊煎嵆涓鸿鎺堜簣鐨?MSI 鍚戦噺鏁伴噺銆?vendor_id	杩斿洖宸茬紪绋嬬殑鍘傚晢 ID锛坔ex锛屽崄鍏繘鍒讹級
device_id	杩斿洖宸茬紪绋嬬殑璁惧 ID锛坔ex锛屽崄鍏繘鍒讹級
bar0_size:	浠ュ崄鍏繘鍒惰繑鍥?bar0 鐨勫ぇ灏忋€?bar0_address	浠ュ崄鍏繘鍒惰繑鍥?bar0 鏄犲皠鍖虹殑鍦板潃銆?bar0_rw_offset	杩斿洖 bar0 鐨勫亸绉婚噺锛宐ar0_data 灏嗚繑鍥炶鍋忕Щ澶勭殑鍊笺€?bar0_data	杩斿洖 bar0_rw_offset 澶勭殑鏁版嵁銆?=============== ==============================================================

### 鑺傜偣鐨勫啓琛屼负锛坵rite behavior of nodes锛夛細


=============== ================================================================
link 		鍐欏叆 UP 浠ュ惎鐢?ltsmm锛屽啓鍏?DOWN 浠ョ鐢?int_type	鍐欏叆瑕侀厤缃殑涓柇绫诲瀷锛坕nt_type 鍙互鏄?INTA銆丮SI 鎴?NO_INT锛夈€備粎鍦ㄤ綘宸茬紪绋嬩簡 no_of_msi 鑺傜偣鏃舵墠閫夋嫨 MSI銆?no_of_msi	鎵€闇€鐨?MSI 鍚戦噺鏁伴噺銆?inta		鍐欏叆 1 浠ユ柇瑷€锛坅ssert锛塈NTA锛屽啓鍏?0 浠ヨВ闄ゆ柇瑷€銆?send_msi	鍐欏叆瑕佸彂閫佺殑 MSI 鍚戦噺銆?vendor_id	鍐欏叆瑕佺紪绋嬬殑鍘傚晢 ID锛坔ex锛屽崄鍏繘鍒讹級銆?device_id	鍐欏叆瑕佺紪绋嬬殑璁惧 ID锛坔ex锛屽崄鍏繘鍒讹級銆?bar0_size	浠ュ崄鍏繘鍒跺啓鍏?bar0 鐨勫ぇ灏忋€傞粯璁?bar0 澶у皬涓?1000锛坔ex锛夊瓧鑺傘€?bar0_address	浠ュ崄鍏繘鍒跺啓鍏?bar0 鏄犲皠鍖虹殑鍦板潃銆傦紙bar0 鐨勯粯璁ゆ槧灏勪负 SYSRAM1(E0800000)銆傚姟蹇呭厛缂栫▼ bar 澶у皬鍐嶇紪绋?bar 鍦板潃銆傚唴鏍稿彲鑳戒负浜嗗榻愯€屼慨鏀?bar 澶у皬鍜屽湴鍧€锛屽洜姝ゅ啓鍏ュ悗搴斿洖璇?bar 澶у皬鍜屽湴鍧€浠ヨ繘琛屾牳瀵广€?bar0_rw_offset	鍐欏叆 bar0 鐨勫亸绉婚噺锛宐ar0_data 灏嗗悜璇ュ亸绉诲啓鍏ュ€笺€?bar0_data	鍐欏叆瑕佸啓鍒?bar0_rw_offset 鐨勬暟鎹€?=============== ================================================================

## 鑺傜偣缂栫▼绀轰緥锛圢ode programming example锛?

灏嗘墍鏈夌殑 PCIe 瀵勫瓨鍣ㄧ紪绋嬩负锛氬綋姝よ澶囪繛鎺ュ埌 PCIe 涓绘満鏃讹紝涓绘満灏嗘璁惧瑙嗕负 1MB 鐨?RAM銆?
```

    #mount -t configfs none /Config

```
```

    # cd /config/pcie_gadget.n/

```
鐜板湪浣犲湪璇ョ洰褰曚笅鎷ユ湁鎵€鏈夎妭鐐广€?```

    # echo 104A >> vendor_id

```
```

    # echo CD80 >> device_id

```
```

    # echo 100000 >> bar0_size

```
```

    # cat bar0_size

```
灏?BAR0 鍦板潃缂栫▼涓?DDR锛?x2100000锛夈€傝繖鏄鏆撮湶缁?PCIe 涓绘満鐨勭墿鐞嗗唴瀛樺湴鍧€銆傜被浼煎湴锛屼换浣曞叾瀹冨璁句篃鍙互鏆撮湶缁?PCIe 涓绘満銆備緥濡傦紝濡傛灉浣犲皢 UART 鐨勫熀鍦板潃缂栫▼涓?BAR0 鍦板潃锛岄偅涔堝綋姝よ澶囪繛鎺ュ埌涓绘満鏃讹紝瀹冨皢琛ㄧ幇涓轰竴涓?UART銆?
```

    # echo 2100000 >> bar0_address

```
```

    # echo INTA >> int_type

```
```

    # echo UP >> link

```
蹇呴』纭繚锛氫竴鏃?gadget 渚у畬鎴愰摼璺氨缁紙link up锛夛紝涓绘満鎵嶅紑濮嬪垵濮嬪寲骞舵悳绱㈠叾绔彛涓婄殑 PCIe 璁惧銆?
```

    /*wait till link is up*/
    # cat link

```
绛夊緟鍏惰繑鍥?UP銆?```

    # echo 1 >> inta

```
```

    # echo 0 >> inta

```
```

    # echo 4 >> no_of_msi

```
```

    # echo MSI >> int_type

```
```

    # echo UP >> link

```
```

    # cat link

```
搴旂敤绋嬪簭鍙互閲嶅璇诲彇璇ヨ妭鐐癸紝鐩村埌鍙戠幇閾捐矾涓?UP銆備袱娆¤鍙栦箣闂村彲浠ヤ紤鐪犮€?
```

    # cat no_of_msi

```
搴旇繑鍥?4锛堣姹傜殑 MSI 鍚戦噺鏁伴噺锛?```

    # echo 2 >> send_msi
    # cd -

```
