## Livepatch 妯″潡 ELF 鏍煎紡

鏈枃妗ｆ杩颁簡 livepatch 妯″潡鎵€蹇呴』閬靛畧鐨?ELF 鏍煎紡瑕佹眰銆?
## 1. 鑳屾櫙涓庡姩鏈?
鏃╁厛锛宭ivepatch 闇€瑕佸崟鐙殑鏋舵瀯鐩稿叧浠ｇ爜鏉ュ啓鍏ラ噸瀹氫綅椤癸紙relocations锛夈€傜劧鑰岋紝鐢ㄤ簬鍐欏叆閲嶅畾浣嶉」鐨勬灦鏋勭浉鍏充唬鐮佸凡缁忓瓨鍦ㄤ簬妯″潡鍔犺浇鍣ㄤ腑锛屽洜姝よ繖绉嶆棫鏂规硶浜х敓浜嗗啑浣欎唬鐮併€備簬鏄紝livepatch 涓嶅啀閲嶅閫犺疆瀛愩€佷篃涓嶅啀閲嶆柊瀹炵幇妯″潡鍔犺浇鍣ㄥ凡缁忚兘鍋氬埌鐨勪簨鎯咃紝鑰屾槸鍊熷姪妯″潡鍔犺浇鍣ㄤ腑宸叉湁鐨勪唬鐮佹潵瀹屾垚鎵€鏈夋灦鏋勭浉鍏崇殑閲嶅畾浣嶅伐浣溿€傚叿浣撹€岃█锛宭ivepatch 澶嶇敤浜嗘ā鍧楀姞杞藉櫒涓殑 apply_relocate_add() 鍑芥暟鏉ュ啓鍏ラ噸瀹氫綅椤广€傛湰鏂囨。鎵€鎻忚堪鐨勮ˉ涓佹ā鍧?ELF 鏍煎紡锛屼娇寰?livepatch 鑳藉杩欐牱鍋氥€傛垜浠笇鏈涘€熸璁?livepatch 鏇村鏄撶Щ妞嶅埌鍏朵粬鏋舵瀯锛屽苟鍑忓皯灏?livepatch 绉绘鍒版煇涓壒瀹氭灦鏋勬墍闇€鐨勬灦鏋勭浉鍏充唬鐮侀噺銆?
鐢变簬 apply_relocate_add() 闇€瑕佽闂ā鍧楃殑鑺傚ご琛ㄣ€佺鍙疯〃浠ュ強閲嶅畾浣嶈妭绱㈠紩锛宭ivepatch 妯″潡鐨?ELF 淇℃伅浼氳淇濈暀锛堣绗?5 鑺傦級銆俵ivepatch 绠＄悊瀹冭嚜宸辩殑閲嶅畾浣嶈妭鍜岀鍙凤紝杩欎簺灏嗗湪鏈枃妗ｄ腑鎻忚堪銆傜敤浜庢爣璁?livepatch 绗﹀彿鍜岄噸瀹氫綅鑺傜殑 ELF 甯搁噺锛屾槸鏍规嵁 glibc 鐨勫畾涔変粠 OS 涓撶敤鑼冨洿涓寫閫夌殑銆?
### 涓轰粈涔?livepatch 闇€瑕佸啓鍏ヨ嚜宸辩殑閲嶅畾浣嶉」锛?
涓€涓吀鍨嬬殑 livepatch 妯″潡鍖呭惈琚ˉ涓佸寲鍑芥暟鐨勮ˉ涓佺増鏈紝杩欎簺鐗堟湰鍙兘寮曠敤鏈鍑虹殑鍏ㄥ眬绗﹀彿浠ュ強鏈寘鍚繘鏉ョ殑灞€閮ㄧ鍙枫€傚紩鐢ㄨ繖绫荤鍙风殑閲嶅畾浣嶉」涓嶈兘鍘熸牱淇濈暀锛屽洜涓哄唴鏍告ā鍧楀姞杞藉櫒鏃犳硶瑙ｆ瀽瀹冧滑锛屼粠鑰屼細鎷掔粷璇?livepatch 妯″潡銆傛澶栵紝鎴戜滑鏃犳硶瀵硅ˉ涓佹ā鍧楀姞杞芥椂灏氫笉瀛樺湪鐨勬ā鍧楀簲鐢ㄩ噸瀹氫綅锛堜緥濡傚鏌愪釜灏氭湭鍔犺浇鐨勯┍鍔ㄥ仛琛ヤ竵锛夈€傛棭鍏堬紝livepatch 閫氳繃鍦ㄧ敓鎴愮殑琛ヤ竵妯″潡 ELF 杈撳嚭涓祵鍏ョ壒娈婄殑 鈥渄ynrela鈥濓紙鍔ㄦ€?rela锛夎妭鏉ヨВ鍐宠繖涓棶棰樸€傚€熷姪杩欎簺 dynrela 鑺傦紝livepatch 鍙互鍦ㄨ€冭檻绗﹀彿浣滅敤鍩熶互鍙婄鍙锋墍灞炴ā鍧楃殑鍓嶆彁涓嬭В鏋愮鍙凤紝鐒跺悗鎵嬪姩搴旂敤杩欎簺鍔ㄦ€侀噸瀹氫綅銆傜劧鑰岃繖绉嶆柟娉曡姹?livepatch 鎻愪緵鏋舵瀯鐩稿叧浠ｇ爜鏉ュ啓鍏ヨ繖浜涢噸瀹氫綅椤广€傚湪鏂扮殑鏍煎紡涓紝livepatch 鐢ㄨ嚜韬殑 SHT_RELA 閲嶅畾浣嶈妭鍙栦唬 dynrela 鑺傦紝鑰?rela 鎵€寮曠敤鐨勭鍙锋槸鐗规畩鐨?livepatch 绗﹀彿锛堣绗?2銆? 鑺傦級銆傛灦鏋勭浉鍏崇殑 livepatch 閲嶅畾浣嶄唬鐮佽涓€娆″ apply_relocate_add() 鐨勮皟鐢ㄦ墍鍙栦唬銆?
## 2. Livepatch modinfo 瀛楁

livepatch 妯″潡蹇呴』甯︽湁 鈥渓ivepatch鈥?modinfo 灞炴€с€傚叧浜庡浣曞仛鍒拌繖涓€鐐癸紝璇峰弬瑙?samples/livepatch/ 涓殑绀轰緥 livepatch 妯″潡銆?
鐢ㄦ埛鍙互浣跨敤 'modinfo' 鍛戒护銆佸苟閫氳繃鏌ユ壘鏄惁瀛樺湪 鈥渓ivepatch鈥?瀛楁鏉ヨ瘑鍒?livepatch 妯″潡銆傝瀛楁涔熻鍐呮牳妯″潡鍔犺浇鍣ㄧ敤浜庤瘑鍒?livepatch 妯″潡銆?
### Example锛堢ず渚嬶級锛?

**Modinfo 杈撳嚭锛?*

```

	% modinfo livepatch-meminfo.ko
	filename:		livepatch-meminfo.ko
	livepatch:		Y
	license:		GPL
	depends:
	vermagic:		4.3.0+ SMP mod_unload

```

## 3. Livepatch 閲嶅畾浣嶈妭

涓€涓?livepatch 妯″潡绠＄悊瀹冭嚜宸辩殑 ELF 閲嶅畾浣嶈妭锛屼互渚垮湪鎭板綋鐨勬椂鏈哄皢閲嶅畾浣嶉」搴旂敤鍒版ā鍧椾互鍙婂唴鏍革紙vmlinux锛夈€備緥濡傦紝濡傛灉涓€涓ˉ涓佹ā鍧楀鏌愪釜褰撳墠灏氭湭鍔犺浇鐨勯┍鍔ㄦ墦琛ヤ竵锛宭ivepatch 浼氬湪璇ラ┍鍔ㄥ姞杞芥椂锛屽皢鐩稿簲鐨?livepatch 閲嶅畾浣嶈妭搴旂敤鍒拌椹卞姩銆?
涓€涓ˉ涓佹ā鍧椾腑鐨勬瘡涓?鈥滃璞♀€濓紙渚嬪 vmlinux锛屾垨涓€涓ā鍧楋級鍙兘鍏宠仈鏈夊涓?livepatch 閲嶅畾浣嶈妭锛堜緥濡傚鍚屼竴瀵硅薄鍐呭涓嚱鏁扮殑琛ヤ竵锛夈€備竴涓?livepatch 閲嶅畾浣嶈妭涓庡畠鎵€搴旂敤鐨勯偅涓洰鏍囪妭锛堥€氬父鏄煇鍑芥暟鐨?text 鑺傦級涔嬮棿瀛樺湪涓€涓€瀵瑰簲鍏崇郴銆備竴涓?livepatch 妯″潡涔熸湁鍙兘娌℃湁浠讳綍 livepatch 閲嶅畾浣嶈妭锛岀ず渚?livepatch 妯″潡灏辨槸杩欑鎯呭喌锛堣 samples/livepatch锛夈€?
鐢变簬 ELF 淇℃伅浼氬湪 livepatch 妯″潡涓繚鐣欙紙瑙佺 5 鑺傦級锛屼竴涓?livepatch 閲嶅畾浣嶈妭鍙渶鎶婄浉搴旂殑鑺傜储寮曚紶缁?apply_relocate_add() 鍗冲彲琚簲鐢紝鍚庤€呴殢鍚庣敤瀹冩潵璁块棶璇ラ噸瀹氫綅鑺傚苟搴旂敤閲嶅畾浣嶉」銆?
livepatch 閲嶅畾浣嶈妭涓紝姣忎釜琚?rela 寮曠敤鐨勭鍙烽兘鏄竴涓?livepatch 绗﹀彿銆傚湪 livepatch 璋冪敤 apply_relocate_add() 涔嬪墠锛屽繀椤诲厛瑙ｆ瀽瀹冧滑銆傛洿澶氫俊鎭绗?3 鑺傘€?
## 3.1 Livepatch 閲嶅畾浣嶈妭鏍煎紡

livepatch 閲嶅畾浣嶈妭蹇呴』鐢?SHF_RELA_LIVEPATCH 鑺傛爣蹇楁爣璁般€傚畾涔夎 include/uapi/linux/elf.h銆傛ā鍧楀姞杞藉櫒璇嗗埆杩欎竴鏍囧織锛屽苟浼氶伩鍏嶅湪琛ヤ竵妯″潡鍔犺浇鏃跺簲鐢ㄨ繖浜涢噸瀹氫綅鑺傘€傝繖浜涜妭杩樺繀椤荤敤 SHF_ALLOC 鏍囪锛屼互渚挎ā鍧楀姞杞藉櫒鍦ㄥ姞杞芥ā鍧楁椂涓嶄涪寮冨畠浠紙鍗冲畠浠細鍜屽叾浠?SHF_ALLOC 鑺備竴璧疯澶嶅埗鍒板唴瀛樹腑锛夈€?
livepatch 閲嶅畾浣嶈妭鐨勫悕绉板繀椤荤鍚堜互涓嬫牸寮?```

  .klp.rela.objname.section_name
  ^        ^^     ^ ^          ^
  |________||_____| |__________|
     [A]      [B]        [C]

```
[A]
  閲嶅畾浣嶈妭鍚嶇О浠ュ瓧绗︿覆 ".klp.rela." 涓哄墠缂€銆?
[B]
  璇ラ噸瀹氫綅鑺傛墍灞炲璞★紙鍗?"vmlinux" 鎴栨ā鍧楀悕锛夌殑鍚嶇О绱ц窡鍦ㄥ墠缂€涔嬪悗銆?
[C]
  璇ラ噸瀹氫綅鑺傛墍搴旂敤鍒扮殑閭ｄ釜鑺傜殑瀹為檯鍚嶇О銆?
### Examples锛堢ず渚嬶級锛?

**Livepatch 閲嶅畾浣嶈妭鍚嶇О锛?*

```

  .klp.rela.ext4.text.ext4_attr_store
  .klp.rela.vmlinux.text.cmdline_proc_show

```
**`readelf --sections` 杈撳嚭锛岄拡瀵逛竴涓 vmlinux 浠ュ強妯″潡 9p銆乥trfs銆乪xt4 鎵撹ˉ涓佺殑
琛ヤ竵妯″潡锛?*

```

  Section Headers:
  [Nr] Name                          Type                    Address          Off    Size   ES Flg Lk Inf Al
  [ snip ]
  [29] .klp.rela.9p.text.caches.show RELA                    0000000000000000 002d58 0000c0 18 AIo 64   9  8
  [30] .klp.rela.btrfs.text.btrfs.feature.attr.show RELA     0000000000000000 002e18 000060 18 AIo 64  11  8
  [ snip ]
  [34] .klp.rela.ext4.text.ext4.attr.store RELA              0000000000000000 002fd8 0000d8 18 AIo 64  13  8
  [35] .klp.rela.ext4.text.ext4.attr.show RELA               0000000000000000 0030b0 000150 18 AIo 64  15  8
  [36] .klp.rela.vmlinux.text.cmdline.proc.show RELA         0000000000000000 003200 000018 18 AIo 64  17  8
  [37] .klp.rela.vmlinux.text.meminfo.proc.show RELA         0000000000000000 003218 0000f0 18 AIo 64  19  8
  [ snip ]                                       ^                                             ^
                                                 |                                             |
                                                [*]                                           [*]

```
[*]
  Livepatch 閲嶅畾浣嶈妭鏄?SHT_RELA 鑺傦紝浣嗗甫鏈変竴浜涚壒娈婄壒寰併€傛敞鎰忓畠浠鏍囪涓?SHF_ALLOC锛?A"锛夛紝杩欐牱褰撴ā鍧楄鍔犺浇杩涘唴瀛樻椂涓嶄細琚涪寮冿紝鍚屾椂瀹冧滑涔熻鏍囪涓?SHF_RELA_LIVEPATCH 鏍囧織锛?o" 鈥斺€?琛ㄧず OS 涓撶敤锛夈€?
**`readelf --relocs` 杈撳嚭锛岄拡瀵逛竴涓ˉ涓佹ā鍧楋細**

```

  Relocation section '.klp.rela.btrfs.text.btrfs_feature_attr_show' at offset 0x2ba0 contains 4 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name + Addend
  000000000000001f  0000005e00000002 R_X86_64_PC32          0000000000000000 .klp.sym.vmlinux.printk,0 - 4
  0000000000000028  0000003d0000000b R_X86_64_32S           0000000000000000 .klp.sym.btrfs.btrfs_ktype,0 + 0
  0000000000000036  0000003b00000002 R_X86_64_PC32          0000000000000000 .klp.sym.btrfs.can_modify_feature.isra.3,0 - 4
  000000000000004c  0000004900000002 R_X86_64_PC32          0000000000000000 .klp.sym.vmlinux.snprintf,0 - 4
  [ snip ]                                                                   ^
                                                                             |
                                                                            [*]

```
[*]
  閲嶅畾浣嶉」鎵€寮曠敤鐨勬瘡涓鍙烽兘鏄竴涓?livepatch 绗﹀彿銆?
## 4. Livepatch 绗﹀彿

livepatch 绗﹀彿鏄 livepatch 閲嶅畾浣嶈妭鎵€寮曠敤鐨勭鍙枫€傝繖浜涙槸鏉ヨ嚜琛ヤ竵瀵硅薄鐨勬柊鐗堟湰鍑芥暟鎵€璁块棶鐨勭鍙凤紝鍏跺湴鍧€鏃犳硶琚ā鍧楀姞杞藉櫒瑙ｆ瀽锛堝洜涓哄畠浠槸灞€閮ㄧ殑鎴栨湭瀵煎嚭鐨勫叏灞€绗﹀彿锛夈€傜敱浜庢ā鍧楀姞杞藉櫒鍙В鏋愬凡瀵煎嚭鐨勭鍙凤紝鑰屾柊琛ヤ竵鍑芥暟鎵€寮曠敤鐨勭鍙峰苟闈炴瘡涓€涓兘宸插鍑猴紝浜庢槸寮曞叆浜?livepatch 绗﹀彿銆傚湪琛ヤ竵妯″潡鍔犺浇鏃舵垜浠棤娉曠珛鍗冲緱鐭ユ煇涓鍙峰湴鍧€鐨勬儏鍐典笅锛屼篃浼氱敤鍒板畠浠€備緥濡傦紝褰?livepatch 瀵规煇涓皻鏈姞杞界殑妯″潡鎵撹ˉ涓佹椂灏辨槸杩欑鎯呭喌銆傚湪杩欑鎯呭喌涓嬶紝鐩稿叧鐨?livepatch 绗﹀彿浼氬湪鐩爣妯″潡鍔犺浇鏃剁畝鍗曞湴瀹屾垚瑙ｆ瀽銆傛棤璁哄浣曪紝瀵逛簬浠讳綍 livepatch 閲嶅畾浣嶈妭锛岃鑺傛墍寮曠敤鐨勬墍鏈?livepatch 绗﹀彿閮藉繀椤诲湪 livepatch 鑳藉瀵硅閲嶅畾浣嶈妭璋冪敤 apply_relocate_add() 涔嬪墠琚В鏋愩€?
livepatch 绗﹀彿蹇呴』鐢?SHN_LIVEPATCH 鏍囪锛屼互渚挎ā鍧楀姞杞藉櫒鑳藉璇嗗埆骞跺拷鐣ュ畠浠€俵ivepatch 妯″潡灏嗚繖浜涚鍙蜂繚鐣欏湪瀹冧滑鐨勭鍙疯〃涓紝鑰岀鍙疯〃閫氳繃 module->symtab 鍙樺緱鍙闂€?
## 4.1 涓€涓?livepatch 妯″潡鐨勭鍙疯〃

閫氬父锛屾ā鍧楃鍙疯〃鐨勪竴涓簿绠€鍓湰锛堜粎鍖呭惈 鈥滄牳蹇冣€?绗﹀彿锛変細閫氳繃 module->symtab 鎻愪緵锛堣 kernel/module/kallsyms.c 涓殑 layout_symtab()锛夈€傚浜?livepatch 妯″潡锛屽湪妯″潡鍔犺浇鏃跺鍒跺埌鍐呭瓨涓殑绗﹀彿琛ㄥ繀椤讳笌琛ヤ竵妯″潡缂栬瘧鏃剁敓鎴愮殑绗﹀彿琛ㄥ畬鍏ㄤ竴鑷淬€傝繖鏄洜涓烘瘡涓?livepatch 閲嶅畾浣嶈妭涓殑閲嶅畾浣嶉」閮芥槸閫氳繃鍚勮嚜鐨勭鍙风储寮曟潵寮曠敤鐩稿簲绗﹀彿鐨勶紝鑰屽師濮嬬殑绗﹀彿绱㈠紩锛堜互鍙婄鍙疯〃鐨勬帓搴忥級蹇呴』琚繚鐣欙紝浠ヤ究 apply_relocate_add() 鑳芥壘鍒版纭殑绗﹀彿銆?
```

  Relocation section '.klp.rela.btrfs.text.btrfs_feature_attr_show' at offset 0x2ba0 contains 4 entries:
      Offset             Info             Type               Symbol's Value  Symbol's Name + Addend
  000000000000001f  0000005e00000002 R_X86_64_PC32          0000000000000000 .klp.sym.vmlinux.printk,0 - 4

```
杩欎釜 rela 寮曠敤绗﹀彿 '.klp.sym.vmlinux.printk,0'锛岀鍙风储寮曠紪鐮佸湪 'Info' 涓€傝繖閲屽畠鐨勭鍙风储寮曟槸 0x5e锛屽嵆鍗佽繘鍒剁殑 94锛屾寚鍚戠鍙风储寮?94銆?
鑰屽湪璇ヨˉ涓佹ā鍧楀搴旂殑绗﹀彿琛ㄤ腑锛岀鍙风储寮?94 鎸囧悜
```

  [ snip ]
  94: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT OS [0xff20] .klp.sym.vmlinux.printk,0
  [ snip ]

```

## 4.2 Livepatch 绗﹀彿鏍煎紡

livepatch 绗﹀彿鐨勮妭绱㈠紩蹇呴』鏍囪涓?SHN_LIVEPATCH锛屼互渚挎ā鍧楀姞杞藉櫒鑳藉璇嗗埆瀹冧滑銆佸苟涓嶅幓灏濊瘯瑙ｆ瀽瀹冧滑銆傚疄闄呭畾涔夎 include/uapi/linux/elf.h銆?
```

  .klp.sym.objname.symbol_name,sympos
  ^       ^^     ^ ^         ^ ^
  |_______||_____| |_________| |
     [A]     [B]       [C]    [D]

```
[A]
  绗﹀彿鍚嶇О浠ュ瓧绗︿覆 ".klp.sym." 涓哄墠缂€銆?
[B]
  璇ョ鍙锋墍灞炲璞★紙鍗?"vmlinux" 鎴栨ā鍧楀悕锛夌殑鍚嶇О绱ц窡鍦ㄥ墠缂€涔嬪悗銆?
[C]
  绗﹀彿鐨勫疄闄呭悕绉般€?
[D]
  绗﹀彿鍦ㄥ璞′腑鐨勪綅缃紙鎸?kallsyms 璁＄畻锛夈€傝繖鐢ㄤ簬鍖哄垎鍚屼竴瀵硅薄鍐呯殑閲嶅绗﹀彿銆傜鍙蜂綅缃互鏁板瓧琛ㄧず锛?銆?銆?鈥︹€︼級銆傚敮涓€绗﹀彿鐨勭鍙蜂綅缃负 0銆?
### Examples锛堢ず渚嬶級锛?

**Livepatch 绗﹀彿鍚嶇О锛?*

```

	.klp.sym.vmlinux.snprintf,0
	.klp.sym.vmlinux.printk,0
	.klp.sym.btrfs.btrfs_ktype,0

```
**`readelf --symbols` 杈撳嚭锛岄拡瀵逛竴涓ˉ涓佹ā鍧楋細**

```

  Symbol table '.symtab' contains 127 entries:
     Num:    Value          Size Type    Bind   Vis     Ndx         Name
     [ snip ]
      73: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT OS [0xff20] .klp.sym.vmlinux.snprintf,0
      74: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT OS [0xff20] .klp.sym.vmlinux.capable,0
      75: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT OS [0xff20] .klp.sym.vmlinux.find_next_bit,0
      76: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT OS [0xff20] .klp.sym.vmlinux.si_swapinfo,0
    [ snip ]                                               ^
                                                           |
                                                          [*]

```
[*]
  娉ㄦ剰杩欎簺绗﹀彿鐨?'Ndx'锛堣妭绱㈠紩锛夋槸 SHN_LIVEPATCH锛?xff20锛夈€?  "OS" 琛ㄧず OS 涓撶敤銆?
## 5. 绗﹀彿琛ㄤ笌 ELF 鑺傝闂?
涓€涓?livepatch 妯″潡鐨勭鍙疯〃鍙€氳繃 module->symtab 璁块棶銆?
鐢变簬 apply_relocate_add() 闇€瑕佽闂ā鍧楃殑鑺傚ご銆佺鍙疯〃浠ュ強閲嶅畾浣嶈妭绱㈠紩锛宭ivepatch 妯″潡鐨?ELF 淇℃伅浼氳淇濈暀锛屽苟鐢辨ā鍧楀姞杞藉櫒閫氳繃 module->klp_info锛堝畠鏄竴涓?`klp_modinfo` 缁撴瀯浣擄級鎻愪緵璁块棶銆傚綋涓€涓?livepatch 妯″潡鍔犺浇鏃讹紝璇ョ粨鏋勪綋鐢辨ā鍧楀姞杞藉櫒濉厖銆?