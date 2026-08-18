## Linux CD-ROM 鏍囧噯


:浣滆€? David van Leeuwen <david@ElseWare.cistron.nl>
:鏃ユ湡: 1999 骞?3 鏈?12 鏃?
:鏇存柊鑰? Erik Andersen (andersee@debian.org)
:鏇存柊鑰? Jens Axboe (axboe@image.dk)


## 寮曡█


Linux 澶ф鏄敮鎸佺‖浠惰澶囩绫绘渶骞挎硾鐨勭被 Unix 鎿嶄綔绯荤粺銆傚叾鍘熷洜澶ф鏄細

- Linux 鐩墠鏀寔鐨勪紬澶氬钩鍙帮紙鍗?i386-PC銆丼parc Sun 绛夛級涓婃湁澶ч噺鍙敤鐨勭‖浠惰澶囥€?
- 鎿嶄綔绯荤粺閲囩敤寮€鏀捐璁★紝浠讳綍浜洪兘鍙互涓?Linux 缂栧啓椹卞姩銆?
- 鏈夊ぇ閲忔簮浠ｇ爜鍙綔涓哄浣曠紪鍐欓┍鍔ㄧ殑绀轰緥銆?

Linux 鐨勫紑鏀炬€э紝浠ュ強绉嶇被绻佸鐨勫彲鐢ㄧ‖浠讹紝浣?Linux 寰椾互鏀寔璁稿涓嶅悓鐨勭‖浠惰澶囥€傞仐鎲剧殑鏄紝姝ｆ槸杩欑鍏佽 Linux 鏀寔鎵€鏈夎繖浜涗笉鍚岃澶囩殑寮€鏀炬€э紝涔熷鑷翠簡姣忎釜璁惧椹卞姩鐨勮涓哄郊姝や箣闂村瓨鍦ㄦ樉钁楀樊寮傘€傝繖绉嶈涓虹殑宸紓鍦?CD-ROM 璁惧涓婅〃鐜板緱灏や负鏄庢樉锛涙煇涓壒瀹氬厜椹卞涓€涓?`standard` **ioctl()** 璋冪敤鐨勫弽搴旓紝鍦ㄤ笉鍚岃澶囬┍鍔ㄤ箣闂村樊寮傚法澶с€備负浜嗛伩鍏嶈鑷繁鐨勯┍鍔ㄥ畬鍏ㄤ笉涓€鑷达紝Linux CD-ROM 椹卞姩鐨勭紪鍐欒€呴€氬父浼氶€氳繃鐞嗚В銆佸鍒躲€佸啀淇敼涓€涓凡鏈夐┍鍔ㄦ潵鍒涘缓鏂扮殑璁惧椹卞姩銆傞仐鎲剧殑鏄紝杩欑鍋氭硶骞舵病鏈夊湪鎵€鏈?Linux CD-ROM 椹卞姩涔嬮棿缁存寔缁熶竴鐨勮涓恒€?

鏈枃妗ｆ弿杩颁簡涓?Linux 鎵€鏈変笉鍚岀殑 CD-ROM 璁惧椹卞姩寤虹珛缁熶竴琛屼负鐨勫姫鍔涖€傛湰鏂囨。杩樺畾涔変簡鍚勭 **ioctl()**锛屼互鍙婂簳灞?CD-ROM 璁惧椹卞姩搴斿綋濡備綍瀹炵幇瀹冧滑銆傚綋鍓嶏紙鍦?Linux 2.1.\ **x** 寮€鍙戝唴鏍镐腑锛夊凡鏈夎嫢骞插簳灞?CD-ROM 璁惧椹卞姩锛堝寘鎷?IDE/ATAPI 鍜?SCSI锛夊紑濮嬩娇鐢ㄨ繖绉嶇粺涓€鎺ュ彛銆?

鍦?CD-ROM 琚紑鍙戝嚭鏉ユ椂锛孋D-ROM 椹卞姩鍣ㄤ笌璁＄畻鏈轰箣闂寸殑鎺ュ彛骞舵湭鍦ㄦ爣鍑嗕腑瑙勫畾銆傜粨鏋滀究鏄嚭鐜颁簡璁稿涓嶅悓鐨?CD-ROM 鎺ュ彛銆傚叾涓竴浜涙嫢鏈夎嚜宸辩殑涓撴湁璁捐锛圫ony銆丮itsumi銆丳anasonic銆丳hilips锛夛紝鍏朵粬鍒堕€犲晢鍒欓噰鐢ㄤ簡宸叉湁鐨勭數姘旀帴鍙ｅ苟鏀瑰彉浜嗗姛鑳斤紙CreativeLabs/SoundBlaster銆乀eac銆丗unai锛夛紝鎴栬€呭共鑴嗚鑷鐨勯┍鍔ㄥ櫒閫傞厤涓€绉嶆垨澶氱宸叉湁鐨勭數姘旀帴鍙ｏ紙Aztech銆丼anyo銆丗unai銆乂ertos銆丩ongshine銆丱ptics Storage 浠ュ強澶у鏁?`NoName` 鍒堕€犲晢锛夈€傚湪鏌愮鏂伴┍鍔ㄥ櫒纭疄甯︽潵浜嗚嚜宸辩殑鎺ュ彛銆佹垨浣跨敤浜嗚嚜宸辩殑鍛戒护闆嗗拰娴佹帶鏂规鐨勬儏鍐典笅锛岃涔堝繀椤荤紪鍐欎竴涓嫭绔嬬殑椹卞姩锛岃涔堝繀椤诲寮轰竴涓凡鏈夌殑椹卞姩銆傚巻鍙插凡缁忎负鎴戜滑鎻愪緵浜嗛拡瀵硅澶氳繖绫讳笉鍚屾帴鍙ｇ殑 CD-ROM 鏀寔銆傚浠婏紝鍑犱箮鎵€鏈夋柊鍑虹殑 CD-ROM 椹卞姩鍣ㄩ兘鏄?IDE/ATAPI 鎴?SCSI锛屽埗閫犲晢鍐嶅垱寤烘柊鎺ュ彛鐨勫彲鑳芥€у井涔庡叾寰€傜敋鑷宠繛涓烘棫鐨勪笓鏈夋帴鍙ｅ鎵鹃┍鍔ㄥ櫒閮藉彉寰楀洶闅句簡銆?

褰擄紙鍦?1.3.70 鏃朵唬锛夋垜鏌ョ湅閫氳繃 `cdrom.h` 琛ㄨ揪鐨勭幇鏈夎蒋浠舵帴鍙ｆ椂锛屽畠鐪嬭捣鏉ユ槸涓€缁勭浉褰撴潅涔辩殑鍛戒护鍜屾暟鎹牸寮?[#f1]_銆備技涔庤蒋浠舵帴鍙ｇ殑璁稿鐗规€ч兘鏄互涓€绉?**ad hoc**锛堜复鏃跺簲浠橈級鐨勬柟寮忚娣诲姞杩涙潵锛屼互杩佸氨鏌愪釜鐗瑰畾椹卞姩鍣ㄧ殑鑳藉姏銆傛洿閲嶈鐨勬槸锛屽ぇ澶氭暟涓嶅悓椹卞姩瀵?`standard` 鍛戒护鐨勮涓轰技涔庡悇涓嶇浉鍚岋細渚嬪锛屾湁浜涢┍鍔ㄥ湪鎵樼洏鎵撳紑鏃惰嫢鍙戠敓涓€娆?**open()** 璋冪敤灏变細鍚堜笂鎵樼洏锛岃€屽彟涓€浜涘垯涓嶄細銆傛湁浜涢┍鍔ㄥ湪鎵撳紑璁惧鏃朵細閿佷笂闂紝浠ラ槻姝㈡枃浠剁郴缁熶笉涓€鑷达紝鑰屽彟涓€浜涘垯涓嶄細锛屼互渚垮厑璁歌蒋浠跺脊鍑恒€傛鏃犵枒闂紝涓嶅悓椹卞姩鍣ㄧ殑鑳藉姏瀛樺湪宸紓锛屼絾鍗充究鏄袱涓嫢鏈夌浉鍚岃兘鍔涚殑椹卞姩鍣紝鍏堕┍鍔ㄧ殑琛屼负閫氬父涔熶笉鍚屻€?

   鎴戣涓嶆竻褰撴椂鐪嬬殑鏄摢涓唴鏍哥増鏈簡锛屽ぇ姒傛槸 1.2.13 鍜?1.3.34 鈥斺€?鎴戦棿鎺ュ弬涓庤繃鐨勬渶鍚庝竴涓唴鏍搞€?

鎴戝喅瀹氬氨濡備綍璁╂墍鏈?Linux CD-ROM 椹卞姩鐨勮涓烘洿鍔犵粺涓€灞曞紑涓€娆¤璁恒€傛垜棣栧厛鑱旂郴浜?Linux 鍐呮牳涓紬澶?CD-ROM 椹卞姩鐨勫紑鍙戣€呫€備粬浠殑鍙嶅簲榧撹垶浜嗘垜鍘荤紪鍐欐湰鏂囨。鏃ㄥ湪鎻忚堪鐨勭粺涓€ CD-ROM 椹卞姩锛圲niform CD-ROM Driver锛夈€傜粺涓€ CD-ROM 椹卞姩鐨勫疄鐜颁綅浜庢枃浠?`cdrom.c` 涓€傝椹卞姩鎰忓湪鎴愪负浣嶄簬姣忎釜 CD-ROM 椹卞姩鍣ㄥ簳灞傝澶囬┍鍔ㄤ箣涓婄殑涓€灞傞檮鍔犺蒋浠跺眰銆傞€氳繃澧炲姞杩欎竴灞傦紝渚挎湁鍙兘璁╂墍鏈変笉鍚岀殑 CD-ROM 璁惧琛ㄧ幇寰?**瀹屽叏** 涓€鑷达紙鍦ㄥ簳灞傜‖浠跺厑璁哥殑鑼冨洿鍐咃級銆?

缁熶竴 CD-ROM 椹卞姩鐨勭洰鏍?**骞堕潪** 鐤忚繙閭ｄ簺灏氭湭閲囧彇鎺柦鏀寔璇ュ姫鍔涚殑椹卞姩寮€鍙戣€呫€傜粺涓€ CD-ROM 椹卞姩鐨勭洰鏍囦粎浠呮槸锛屼负缂栧啓闈㈠悜 CD-ROM 椹卞姩鍣ㄧ殑搴旂敤绋嬪簭鐨勪汉鎻愪緵 **涓€涓?* 瀵瑰叏閮?CD-ROM 璁惧琛屼负涓€鑷寸殑 Linux CD-ROM 鎺ュ彛銆傛澶栵紝杩欎篃涓哄簳灞傝澶囬┍鍔ㄤ唬鐮佷笌 Linux 鍐呮牳涔嬮棿鎻愪緵浜嗕竴鑷寸殑鎺ュ彛銆傛垜浠‘淇濅笌 `cdrom.h` 涓畾涔夌殑鏁版嵁缁撴瀯鍜岀▼搴忓憳鎺ュ彛淇濇寔 100% 鍏煎銆傛湰鎸囧崡鐨勭紪鍐欐槸涓轰簡甯姪 CD-ROM 椹卞姩寮€鍙戣€呰皟鏁翠粬浠殑浠ｇ爜浠ヤ娇鐢?`cdrom.c` 涓畾涔夌殑缁熶竴 CD-ROM 椹卞姩浠ｇ爜銆?

灏变釜浜鸿€岃█锛屾垜璁や负鏈€閲嶈鐨勭‖浠舵帴鍙ｆ槸 IDE/ATAPI 椹卞姩鍣紝褰撶劧杩樻湁 SCSI 椹卞姩鍣紝浣嗛殢鐫€纭欢浠锋牸鎸佺画涓嬮檷锛屼汉浠篃鍙兘鎷ユ湁澶氬彴 CD-ROM 椹卞姩鍣ㄣ€佺敋鑷冲彲鑳界被鍨嬫贩鏉傦紝杩欏悓鏍峰緢鍙兘鍙戠敓銆傞噸瑕佺殑鏄紝杩欎簺椹卞姩鍣ㄧ殑琛屼负搴斿綋涓€鑷淬€?994 骞?12 鏈堬紝鏈€渚垮疁鐨?CD-ROM 椹卞姩鍣ㄤ箣涓€鏄?Philips cm206锛屼竴鍙板弻鍊嶉€熶笓鏈夐┍鍔ㄥ櫒銆傚湪鎴戝繖浜庝负瀹冪紪鍐?Linux 椹卞姩鐨勯偅浜涙湀浠介噷锛屼笓鏈夐┍鍔ㄥ櫒鍙樺緱杩囨椂锛岃€?IDE/ATAPI 椹卞姩鍣ㄦ垚浜嗘爣鍑嗐€傚湪鏈枃妗ｆ渶鍚庝竴娆℃洿鏂版椂锛?997 骞?11 鏈堬級锛岃鎵惧埌浣庝簬 16 鍊嶉€熺殑 CD-ROM 椹卞姩鍣ㄧ敋鑷抽兘鍙樺緱鍥伴毦锛岃€?24 鍊嶉€熼┍鍔ㄥ櫒宸茬粡寰堝父瑙佷簡銆?


## 閫氳繃鍙︿竴杞欢灞傚疄鐜版爣鍑嗗寲


鍦ㄦ瀯鎬濇湰鏂囨。涔嬫椂锛屾墍鏈夐┍鍔ㄩ兘鐩存帴閫氳繃鍚勮嚜鐨勪緥绋嬪疄鐜?CD-ROM 鐨?**ioctl()** 璋冪敤銆傝繖瀵艰嚧浜嗕竴绉嶉闄╋細涓嶅悓椹卞姩鍙兘浼氬繕璁板仛璇稿妫€鏌ョ敤鎴锋槸鍚﹀悜椹卞姩鎻愪緵浜嗘湁鏁堟暟鎹繖绫婚噸瑕佷簨鎯呫€傛洿閲嶈鐨勬槸锛岃繖瀵艰嚧浜嗗墠闈㈠凡缁忚璁鸿繃鐨勮涓哄垎姝с€?

鍑轰簬杩欎竴鍘熷洜锛屽垱寤轰簡缁熶竴 CD-ROM 椹卞姩锛屼互寮哄埗瀹炵幇涓€鑷寸殑 CD-ROM 椹卞姩鍣ㄨ涓猴紝骞朵负鍚勭搴曞眰 CD-ROM 璁惧椹卞姩鎻愪緵涓€缁勯€氱敤鏈嶅姟銆傜粺涓€ CD-ROM 椹卞姩鐜板湪鎻愪緵浜嗗彟涓€杞欢灞傦紝瀹冩妸 **ioctl()** 鍜?**open()** 鐨勫疄鐜颁笌瀹為檯鐨勭‖浠跺疄鐜板垎绂诲紑鏉ャ€傝娉ㄦ剰锛岃繖涓€鍔姏鏋佸皯鏀瑰姩浼氬奖鍝嶇敤鎴峰簲鐢ㄧ▼搴忕殑鍦版柟銆傛渶澶х殑鏀瑰姩鏄妸鍚勭搴曞眰 CD-ROM 椹卞姩鐨勫ご鏂囦欢鍐呭绉诲埌浜嗗唴鏍哥殑 cdrom 鐩綍涓€傝繖鏍峰仛鏄负浜嗗府鍔╃敤鎴峰彧闈㈠鍞竴鐨?cdrom 鎺ュ彛锛屽嵆 `cdrom.h` 涓畾涔夌殑鎺ュ彛銆?

CD-ROM 椹卞姩鍣ㄧ殑鐗规€ц冻澶熺壒娈婏紙鍗充笉鍚屼簬杞洏鎴栫‖鐩樼瓑鍏朵粬鍧楄澶囷級锛屽洜姝ゅ彲浠ュ畾涔変竴缁勯€氱敤鐨?**CD-ROM 璁惧鎿嶄綔**锛屽嵆 **<cdrom-device>_dops**銆傝繖浜涙搷浣滀笉鍚屼簬缁忓吀鐨勫潡璁惧鏂囦欢鎿嶄綔 **<block-device>_fops**銆?

缁熶竴 CD-ROM 椹卞姩鎺ュ彛灞傜殑渚嬬▼瀹炵幇鍦ㄦ枃浠?`cdrom.c` 涓€傚湪璇ユ枃浠朵腑锛岀粺涓€ CD-ROM 椹卞姩閫氳繃娉ㄥ唽浠ヤ笅閫氱敤鍐呭锛屼互鍐呮牳鍧楄澶囩殑鏂瑰紡涓庡唴鏍镐氦浜掞細

```
	struct file_operations cdrom_fops = {
		NULL,			/* lseek */
		block _read ,		/* read--general block-dev read */
		block _write,		/* write--general block-dev write */
		NULL,			/* readdir */
		NULL,			/* select */
		cdrom_ioctl,		/* ioctl */
		NULL,			/* mmap */
		cdrom_open,		/* open */
		cdrom_release,		/* release */
		NULL,			/* fsync */
		NULL,			/* fasync */
		NULL			/* revalidate */
	};

```
姣忎釜娲昏穬鐨?CD-ROM 璁惧閮藉叡浜繖涓€ **struct**銆備笂闈㈠０鏄庣殑渚嬬▼鍏ㄩ儴瀹炵幇鍦?`cdrom.c` 涓紝鍥犱负璇ユ枃浠舵鏄畾涔夊拰鏍囧噯鍖栨墍鏈?CD-ROM 璁惧琛屼负鐨勫湴鏂广€傚鍚勭绫诲瀷 CD-ROM 纭欢鐨勫疄闄呮帴鍙ｄ粛鐒剁敱鍚勭搴曞眰 CD-ROM 璁惧椹卞姩鎵ц銆傝繖浜涗緥绋嬪彧鏄疄鐜颁簡鎵€鏈?CD-ROM锛堜簨瀹炰笂锛屾墍鏈夊彲绉诲姩浠嬭川璁惧锛夐€氱敤鐨勬煇浜?**capabilities**銆?

搴曞眰 CD-ROM 璁惧椹卞姩鐨勬敞鍐岀幇鍦ㄩ€氳繃 `cdrom.c` 涓殑閫氱敤渚嬬▼瀹屾垚锛岃€屼笉鍐嶇粡鐢辫櫄鎷熸枃浠剁郴缁燂紙VFS锛夈€俙cdrom.c` 涓疄鐜扮殑鎺ュ彛閫氳繃涓や釜閫氱敤缁撴瀯鏉ユ墽琛岋紝杩欎袱涓粨鏋勫寘鍚簡鍏充簬椹卞姩鑳藉姏銆佷互鍙婇┍鍔ㄦ墍鎿嶄綔鐨勭壒瀹氶┍鍔ㄥ櫒鐨勪俊鎭€傝繖涓や釜缁撴瀯鏄細

cdrom_device_ops
  璇ョ粨鏋勫寘鍚叧浜庢煇涓?CD-ROM 璁惧鐨勫簳灞傞┍鍔ㄧ殑淇℃伅銆傝缁撴瀯鍦ㄦ蹇典笂杩炴帴鍒拌澶囩殑涓昏澶囧彿锛堝敖绠℃煇浜涢┍鍔ㄥ彲鑳芥嫢鏈変笉鍚岀殑涓昏澶囧彿锛孖DE 椹卞姩渚挎槸濡傛锛夈€?

cdrom_device_info
  璇ョ粨鏋勫寘鍚叧浜庢煇涓壒瀹?CD-ROM 椹卞姩鍣ㄧ殑淇℃伅锛屼緥濡傚畠鐨勮澶囧悕銆侀€熷害绛夈€傝缁撴瀯鍦ㄦ蹇典笂杩炴帴鍒拌澶囩殑娆¤澶囧彿銆?

鐢ㄧ粺涓€ CD-ROM 椹卞姩娉ㄥ唽鏌愪釜鐗瑰畾鐨?CD-ROM 椹卞姩鍣細

```
	register_cdrom(struct cdrom_device_info * <device>_info)

```
璁惧淇℃伅缁撴瀯 **<device>_info** 鍖呭惈浜嗗唴鏍镐笌搴曞眰 CD-ROM 璁惧椹卞姩浜や簰鎵€闇€鐨勫叏閮ㄤ俊鎭€傝缁撴瀯涓渶閲嶈鐨勬潯鐩箣涓€锛屾槸鎸囧悜搴曞眰椹卞姩鐨?**cdrom_device_ops** 缁撴瀯鐨勬寚閽堛€?

璁惧鎿嶄綔缁撴瀯 **cdrom_device_ops** 鍖呭惈涓€缁勬寚鍚戝簳灞傝澶囬┍鍔ㄤ腑鎵€瀹炵幇鍑芥暟鐨勬寚閽堛€傚綋 `cdrom.c` 璁块棶涓€涓?CD-ROM 璁惧鏃讹紝瀹冮€氳繃璇ョ粨鏋勪腑鐨勫嚱鏁版潵杩涜銆傛湭鏉ョ殑 CD-ROM 椹卞姩鍣ㄧ殑鍏ㄩ儴鑳藉姏鏃犳硶棰勭煡锛屽洜姝ら璁￠殢鐫€鏂版妧鏈寮€鍙戝嚭鏉ワ紝杩欎竴鍒楄〃鍙兘闇€瑕佷笉鏃舵墿灞曘€備緥濡傦紝CD-R 鍜?CD-R/W 椹卞姩鍣ㄦ寮€濮嬪彉寰楁祦琛岋紝寰堝揩灏遍渶瑕佷负瀹冧滑娣诲姞鏀寔銆傜洰鍓嶏細

```
	struct cdrom_device_ops {
		int (*open)(struct cdrom_device_info *, int)
		void (*release)(struct cdrom_device_info *);
		int (*drive_status)(struct cdrom_device_info *, int);
		unsigned int (*check_events)(struct cdrom_device_info *,
					     unsigned int, int);
		int (*media_changed)(struct cdrom_device_info *, int);
		int (*tray_move)(struct cdrom_device_info *, int);
		int (*lock_door)(struct cdrom_device_info *, int);
		int (*select_speed)(struct cdrom_device_info *, unsigned long);
		int (*get_last_session) (struct cdrom_device_info *,
					 struct cdrom_multisession *);
		int (*get_mcn)(struct cdrom_device_info *, struct cdrom_mcn *);
		int (*reset)(struct cdrom_device_info *);
		int (*audio_ioctl)(struct cdrom_device_info *,
				   unsigned int, void *);
		const int capability;		/* capability flags */
		int (*generic_packet)(struct cdrom_device_info *,
				      struct packet_command *);
	};

```
褰撳簳灞傝澶囬┍鍔ㄥ疄鐜颁簡杩欎簺鑳藉姏涓殑鏌愪竴涓椂锛屽畠搴斿綋鍦ㄨ **struct** 涓姞鍏ヤ竴涓嚱鏁版寚閽堛€傝€屽綋鏌愪釜鐗瑰畾鍑芥暟鏈瀹炵幇鏃讹紝璇?**struct** 涓簲鍖呭惈 NULL銆傚湪鎶婁竴涓?CD-ROM 椹卞姩鍣ㄥ悜缁熶竴 CD-ROM 椹卞姩娉ㄥ唽鏃讹紝**capability** 鏍囧織鎸囨槑浜?CD-ROM 纭欢鍜?鎴栧簳灞?CD-ROM 椹卞姩鐨勮兘鍔涖€?

璇锋敞鎰忥紝澶у鏁板嚱鏁扮殑鍙傛暟閮芥瘮瀹冧滑鐨?**blkdev_fops** 瀵瑰簲椤硅灏戙€傝繖鏄洜涓?**inode** 鍜?**file** 缁撴瀯涓殑淇℃伅寰堝皯琚敤鍒般€傚澶у鏁伴┍鍔ㄨ€岃█锛屼富瑕佸弬鏁版槸 **struct** **cdrom_device_info**锛屼粠涓彲浠ユ彁鍙栧嚭涓昏澶囧彿鍜屾璁惧鍙枫€傦紙涓嶈繃澶у鏁板簳灞?CD-ROM 椹卞姩鐢氳嚦涓嶇湅涓昏澶囧彿鍜屾璁惧鍙凤紝鍥犱负瀹冧滑涓澶氬彧鏀寔涓€涓澶囥€傦級杩欎竴鐐瑰皢閫氳繃涓嬮潰鎻忚堪鐨?**cdrom_device_info** 涓殑 **dev** 鍙敤銆?

涓庨┍鍔ㄥ櫒鐩稿叧鐨勩€佺被浼兼璁惧鍙风殑淇℃伅鍦ㄦ敞鍐屾椂閫氳繃浠ヤ笅缁撴瀯锛?

```
  struct cdrom_device_info {
	const struct cdrom_device_ops * ops;	/* device operations for this major */
	struct list_head list;			/* linked list of all device_info */
	struct gendisk * disk;			/* matching block layer disk */
	void *  handle;				/* driver-dependent data */

	int mask;				/* mask of capability: disables them */
	int speed;				/* maximum speed for reading data */
	int capacity;				/* number of discs in a jukebox */

	unsigned int options:30;		/* options flags */
	unsigned mc_flags:2;			/*  media-change buffer flags */
	unsigned int vfs_events;		/*  cached events for vfs path */
	unsigned int ioctl_events;		/*  cached events for ioctl path */
	int use_count;				/*  number of times device is opened */
	char name[20];				/*  name of the device type */

	__u8 sanyo_slot : 2;			/*  Sanyo 3-CD changer support */
	__u8 keeplocked : 1;			/*  CDROM_LOCKDOOR status */
	__u8 reserved : 5;			/*  not used yet */
	int cdda_method;			/*  see CDDA_* flags */
	__u8 last_sense;			/*  saves last sense key */
	__u8 media_written;			/*  dirty flag, DVD+RW bookkeeping */
	unsigned short mmc3_profile;		/*  current MMC3 profile */
	int for_data;				/*  unknown:TBD */
	int mrw_mode_page;			/*  which MRW mode page is in use */
  };

```
浣跨敤杩欎竴 **struct**锛屽€熷姪 **next** 瀛楁锛屾瀯寤哄嚭涓€涓凡娉ㄥ唽娆¤澶囧彿鐨勯摼琛ㄣ€傝澶囧彿銆佽澶囨搷浣滅粨鏋勪互鍙婇┍鍔ㄥ櫒灞炴€х殑瑙勬牸閮藉瓨鍌ㄥ湪璇ョ粨鏋勪腑銆?

**mask** 鏍囧織鍙敤浜庡睆钄芥帀 **ops->capability** 涓垪鍑虹殑鏌愪簺鑳藉姏锛屽鏋滄煇涓壒瀹氶┍鍔ㄥ櫒涓嶆敮鎸侀┍鍔ㄧ殑鏌愰」鐗规€с€傛暟鍊?**speed** 鎸囨槑浜嗛┍鍔ㄥ櫒鐨勬渶澶х澶撮€熺巼锛屼互姝ｅ父闊抽閫熷害涓哄崟浣嶏紙176kB/sec 鍘熷鏁版嵁鎴?150kB/sec 鏂囦欢绯荤粺鏁版嵁锛夈€傝繖浜涘弬鏁拌澹版槑涓?**const**锛屽洜涓哄畠浠弿杩伴┍鍔ㄥ櫒鐨勫睘鎬э紝鍦ㄦ敞鍐屼箣鍚庝笉浼氭敼鍙樸€?

灏戞暟瀵勫瓨鍣ㄥ寘鍚笓灞炰簬 CD-ROM 椹卞姩鍣ㄧ殑鍙橀噺銆?*options** 鏍囧織鐢ㄤ簬鎸囧畾閫氱敤 CD-ROM 渚嬬▼搴斿綋濡備綍琛ㄧ幇銆傝繖浜涗笉鍚岀殑鏍囧織瀵勫瓨鍣ㄥ簲褰撴彁渚涜冻澶熺殑鐏垫椿鎬э紝浠ラ€傚簲涓嶅悓鐢ㄦ埛鐨勬剰鎰匡紙鑰?**涓嶆槸** 鍍忔棫鏂规閭ｆ牱杩佸氨搴曞眰璁惧椹卞姩浣滆€呯殑 `arbitrary`锛堜换鎰忥級鎰忔効锛夈€傚瘎瀛樺櫒 **mc_flags** 鐢ㄤ簬鎶婃潵鑷?**media_changed()** 鐨勪俊鎭紦鍐插埌涓や釜鐙珛鐨勯槦鍒椼€傚叾浠栦笓灞炰簬鏌愪釜娆¤澶囩殑銆佺壒瀹氱殑鏁版嵁锛屽彲浠ラ€氳繃 **handle** 璁块棶锛?*handle** 鍙互鎸囧悜涓€涓簳灞傞┍鍔ㄧ壒鏈夌殑鏁版嵁缁撴瀯銆傚瓧娈?**use_count**銆?*next**銆?*options** 鍜?**mc_flags** 鏃犻渶鍒濆鍖栥€?

`cdrom.c` 鏋勬垚鐨勪腑闂磋蒋浠跺眰灏嗘墽琛屼竴浜涢澶栫殑绨胯宸ヤ綔銆傝澶囩殑浣跨敤璁℃暟锛堟墦寮€浜嗚璁惧鐨勮繘绋嬫暟锛夌櫥璁板湪 **use_count** 涓€傚嚱鏁?**cdrom_ioctl()** 灏嗛獙璇佷緵璇诲啓鐨勯€傚綋鐢ㄦ埛鍐呭瓨鍖哄煙锛屽苟涓斿湪闇€瑕佷紶閫?CD 涓婃煇涓綅缃殑鏁版嵁鏃讹紝瀹冧細閫氳繃浠ユ爣鍑嗘牸寮忓悜搴曞眰椹卞姩鍙戝嚭璇锋眰鏉?`sanitize`锛堣鏁达級鏍煎紡锛屽苟鍦ㄧ敤鎴疯蒋浠朵笌搴曞眰椹卞姩涔嬮棿缈昏瘧鎵€鏈夋牸寮忋€傝繖鍏嶅幓浜嗛┍鍔ㄥぇ閲忕殑鍐呭瓨妫€鏌ャ€佹牸寮忔鏌ュ拰缈昏瘧宸ヤ綔銆傚悓鏃讹紝鎵€闇€鐨勭粨鏋勫皢鍦ㄧ▼搴忔爤涓婂０鏄庛€?

鍑芥暟鐨勫疄鐜板簲濡傚悗缁悇鑺傛墍瀹氫箟銆傛湁涓や釜鍑芥暟 **蹇呴』** 瀹炵幇锛屽嵆 **open()** 鍜?**release()**銆傚叾浠栧嚱鏁板彲浠ュ拷鐣ワ紝瀹冧滑瀵瑰簲鐨勮兘鍔涙爣蹇椾細鍦ㄦ敞鍐屾椂琚竻闄ゃ€傞€氬父锛屽嚱鏁版垚鍔熸椂杩斿洖闆讹紝鍑洪敊鏃惰繑鍥炶礋鍊笺€傚嚱鏁拌皟鐢ㄥ簲褰撳彧鍦ㄥ懡浠ゅ畬鎴愪箣鍚庢墠杩斿洖锛屼絾褰撶劧锛岀瓑寰呰澶囨椂涓嶅簲鍗犵敤澶勭悊鍣ㄦ椂闂淬€?

```
	int open(struct cdrom_device_info *cdi, int purpose)

```
**Open()** 搴斿綋灏濊瘯涓虹壒瀹氱殑 **purpose**锛堢洰鐨勶級鎵撳紑璁惧锛岃鐩殑鍙互鏄細

- 涓鸿鍙栨暟鎹€屾墦寮€锛屽 `mount()` (2) 鎴栫敤鎴峰懡浠?`dd`銆乣cat` 鎵€鍋氥€?
- 涓烘墽琛?**ioctl** 鍛戒护鑰屾墦寮€锛屽鎾斁闊抽 CD 鐨勭▼搴忔墍鍋氥€?

娉ㄦ剰锛屼换浣曠瓥鐣ユ€т唬鐮侊紙鍦?**open()** 鏃跺悎涓婃墭鐩樼瓑锛夐兘鐢?`cdrom.c` 涓殑璋冪敤渚嬬▼瀹屾垚锛屽洜姝ゅ簳灞備緥绋嬪彧闇€鍏虫敞閫傚綋鐨勫垵濮嬪寲锛屼緥濡傝鐩樼墖杞捣鏉ョ瓑銆?

```
	void release(struct cdrom_device_info *cdi)

```
搴斿綋鎵ц璁惧鐩稿叧鐨勫姩浣滐紝渚嬪璁╄澶囧噺閫熷仠姝€備笉杩囷紝绛栫暐鎬у姩浣滃寮瑰嚭鎵樼洏鎴栬В閿佽埍闂紝搴旂暀缁欓€氱敤渚嬬▼ **cdrom_release()** 澶勭悊銆傝繖鏄敮涓€涓€涓繑鍥炵被鍨嬩负 **void** 鐨勫嚱鏁般€?


```
	int drive_status(struct cdrom_device_info *cdi, int slot_nr)

```
濡傛灉瀹炵幇浜嗗嚱鏁?**drive_status**锛屽畠搴斿綋鎻愪緵鍏充簬椹卞姩鍣ㄧ姸鎬侊紙鑰屼笉鏄洏鐗囩殑鐘舵€侊紝鐩樼墖鍙兘鍦ㄤ篃鍙兘涓嶅湪椹卞姩鍣ㄤ腑锛夌殑淇℃伅銆傚鏋滈┍鍔ㄥ櫒涓嶆槸鎹㈢墖鍣紙changer锛夛細


	CDS_NO_INFO		/* no information available */
	CDS_NO_DISC		/* no disc is inserted, tray is closed */
	CDS_TRAY_OPEN		/* tray is opened */
	CDS_DRIVE_NOT_READY	/* something is wrong, tray is moving? */
	CDS_DISC_OK		/* a disc is loaded and everything is fine */

```
```
	int tray_move(struct cdrom_device_info *cdi, int position)

```
濡傛灉瀹炵幇浜嗚鍑芥暟锛屽畠搴斿綋鎺у埗鎵樼洏鐨勮繍鍔ㄣ€傦紙娌℃湁鍏朵粬鍑芥暟搴斿綋鎺у埗杩欎釜銆傦級鍙傛暟 **position** 鎺у埗鏈熸湜鐨勮繍鍔ㄦ柟鍚戯細

- 0 鍚堜笂鎵樼洏
- 1 鎵撳紑鎵樼洏

璇ュ嚱鏁板湪鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖闈為浂鍊笺€傛敞鎰忥紝濡傛灉鎵樼洏宸茬粡澶勪簬鏈熸湜鐨勪綅缃紝鍒欐棤闇€閲囧彇浠讳綍鍔ㄤ綔锛岃繑鍥炲€煎簲涓?0銆?

```
	int lock_door(struct cdrom_device_info *cdi, int lock)

```
濡傛灉椹卞姩鍣ㄥ厑璁革紝璇ュ嚱鏁帮紙涓旀病鏈夊叾浠栦唬鐮侊級鎺у埗鑸遍棬鐨勯攣瀹氥€傛暟鍊?**lock** 鎺у埗鏈熸湜鐨勯攣瀹氱姸鎬侊細

- 0 瑙ｉ攣鑸遍棬锛屽厑璁告墜鍔ㄦ墦寮€
- 1 閿佸畾鑸遍棬锛屾墭鐩樻棤娉曡鎵嬪姩寮瑰嚭

璇ュ嚱鏁板湪鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖闈為浂鍊笺€傛敞鎰忥紝濡傛灉鑸遍棬宸茬粡澶勪簬鎵€璇锋眰鐨勭姸鎬侊紝鍒欐棤闇€閲囧彇浠讳綍鍔ㄤ綔锛岃繑鍥炲€煎簲涓?0銆?

```
	int select_speed(struct cdrom_device_info *cdi, unsigned long speed)

```
鏌愪簺 CD-ROM 椹卞姩鍣ㄨ兘澶熸敼鍙樺叾纾佸ご閫熷害銆傛敼鍙?CD-ROM 椹卞姩鍣ㄩ€熷害鏈夎嫢骞插師鍥犮€傚帇鍒跺緱涓嶅ソ鐨?CD-ROM 鍙兘浼氫粠浣庝簬鏈€澶х殑纾佸ご閫熺巼涓彈鐩娿€傜幇浠?CD-ROM 椹卞姩鍣ㄥ彲浠ヨ幏寰楅潪甯搁珮鐨勭澶撮€熺巼锛堟渶楂樿揪鍒?**24x** 鏄緢甯歌鐨勶級銆傛湁鎶ュ憡绉拌繖浜涢┍鍔ㄥ櫒鍦ㄥ姝ら珮閫熶笅鍙兘浜х敓璇诲彇閿欒锛岄檷浣庨€熷害鍙互鍦ㄨ繖绉嶆儏鍐典笅闃叉鏁版嵁涓㈠け銆傛渶鍚庯紝杩欎簺椹卞姩鍣ㄤ腑鐨勬煇浜涗細鍙戝嚭鎭间汉鐨勫法澶у櫔澹帮紝闄嶄綆閫熷害鍙兘浼氬噺寮辫繖绉嶅櫔澹般€?

璇ュ嚱鏁版寚瀹氳鍙栨暟鎹垨鎾斁闊抽鏃剁殑閫熷害銆傛暟鍊?**speed** 浠ユ爣鍑?cdrom 閫熷害涓哄崟浣嶏紙176kB/sec 鍘熷鏁版嵁鎴?150kB/sec 鏂囦欢绯荤粺鏁版嵁锛夋寚鏄庨┍鍔ㄥ櫒鐨勭澶撮€熷害銆傚洜姝わ紝鑻ヨ璇锋眰 CD-ROM 椹卞姩鍣ㄤ互 300kB/sec 杩愯锛屼綘灏嗕娇鐢?**speed=2** 璋冪敤 CDROM_SELECT_SPEED **ioctl**銆傜壒娈婂€?`0` 琛ㄧず `auto-selection`锛堣嚜鍔ㄩ€夋嫨锛夛紝鍗虫渶澶ф暟鎹€熺巼鎴栧疄鏃堕煶棰戦€熺巼銆傚鏋滈┍鍔ㄥ櫒娌℃湁杩欑 `auto-selection` 鑳藉姏锛屽垯搴斿綋鏍规嵁褰撳墠瑁呭叆鐨勭洏鐗囧仛鍑哄喅瀹氾紝骞朵笖杩斿洖鍊煎簲涓烘銆傝礋鐨勮繑鍥炲€艰〃绀哄嚭閿欍€?

```
	int get_last_session(struct cdrom_device_info *cdi,
			     struct cdrom_multisession *ms_info)

```
璇ュ嚱鏁板簲褰撳疄鐜版棫鐨勫搴?**ioctl()**銆傚浜庤澶?**cdi->dev**锛屽綋鍓嶇洏鐗囨渶鍚庝竴涓細璇濈殑璧风偣搴斿綋閫氳繃鎸囬拡鍙傛暟 **ms_info** 杩斿洖銆傛敞鎰忥紝`cdrom.c` 涓殑渚嬬▼宸茬粡瀵硅鍙傛暟鍋氫簡瑙勬暣锛氭棤璁鸿皟鐢ㄨ蒋浠惰姹備綍绉嶆牸寮忥紝鍏惰姹傜殑鏍煎紡灏?**濮嬬粓** 涓?**CDROM_LBA** 绫诲瀷锛堢嚎鎬у潡瀵诲潃妯″紡锛夈€備絾瑙勬暣杩樻洿杩涗竴姝ワ細`cdrom.c` 涓殑渚嬬▼濡傛灉蹇呰浼氬仛杞崲锛岃€屽簳灞傚疄鐜板鏋滄効鎰忥紝鍙互鐢?**CDROM_MSF** 鏍煎紡杩斿洖鎵€璇锋眰鐨勪俊鎭紙褰撶劧瑕侀€傚綋鍦拌缃?**ms_info->addr_format** 瀛楁锛夈€傛垚鍔熸椂杩斿洖鍊间负 0銆?

```
	int get_mcn(struct cdrom_device_info *cdi,
		    struct cdrom_mcn *mcn)

```
鏌愪簺鐩樼墖甯︽湁 `Media Catalog Number`锛堝獟浣撶洰褰曞彿锛孧CN锛夛紝涔熺О涓?`Universal Product Code`锛堥€氱敤浜у搧浠ｇ爜锛孶PC锛夈€傝缂栧彿搴斿綋鍙嶆槧閫氬父鍗板湪浜у搧鏉″舰鐮佷笂鐨勭紪鍙枫€傞仐鎲剧殑鏄紝鐩樼墖涓婂甫鏈夎繖绉嶇紪鍙风殑灏戞暟鐩樼墖鐢氳嚦娌℃湁浣跨敤鐩稿悓鐨勬牸寮忋€傝鍑芥暟鐨勮繑鍥炲弬鏁版槸涓€涓寚鍚戦鍏堝０鏄庣殑鍐呭瓨鍖哄煙鐨勬寚閽堬紝鍖哄煙绫诲瀷涓?**struct cdrom_mcn**銆侻CN 搴斾负涓€涓?13 瀛楃鐨勫瓧绗︿覆锛屼互 null 瀛楃缁撳熬銆?

```
	int reset(struct cdrom_device_info *cdi)

```
璇ヨ皟鐢ㄥ簲褰撳椹卞姩鍣ㄦ墽琛屼竴娆＄‖澶嶄綅锛堝敖绠″湪纭疄闇€瑕佺‖澶嶄綅鐨勬儏鍐典笅锛岄┍鍔ㄥ櫒寰堝彲鑳藉凡缁忎笉鍐嶅惉浠庡懡浠や簡锛夈€傛渶濂芥槸鍦ㄩ┍鍔ㄥ櫒瀹屾垚澶嶄綅涔嬪悗鎵嶆妸鎺у埗鏉冭繑鍥炵粰璋冪敤鑰呫€傚鏋滈┍鍔ㄥ櫒涓嶅啀鍚粠锛屽簳灞傜殑搴曞眰 cdrom 椹卞姩鏄庢櫤鐨勫仛娉曟槸瓒呮椂閫€鍑恒€?

```
	int audio_ioctl(struct cdrom_device_info *cdi,
			unsigned int cmd, void *arg)

```
`cdrom.h` 涓畾涔夌殑鏌愪簺 CD-ROM-\ **ioctl()** 鍙互鐢变笂闈㈡弿杩扮殑渚嬬▼瀹炵幇锛屽洜姝ゅ嚱鏁?**cdrom_ioctl** 灏嗕娇鐢ㄥ畠浠€傜劧鑰岋紝澶у鏁?**ioctl()** 澶勭悊闊抽鎺у埗銆傛垜浠喅瀹氭妸瀹冧滑鐣欎綔閫氳繃鍗曚竴鍑芥暟璁块棶锛屽苟閲嶅鍙傛暟 **cmd** 鍜?**arg**銆傛敞鎰忓悗鑰呯被鍨嬩负 **void**锛岃€屼笉鏄?**unsigned long int**銆?*cdrom_ioctl()** 渚嬬▼纭疄鍋氫簡涓€浜涙湁鐢ㄧ殑浜嬫儏锛氬畠涓烘墍鏈夐煶棰戣皟鐢ㄦ妸鍦板潃鏍煎紡绫诲瀷瑙勬暣涓?**CDROM_MSF**锛堝垎銆佺銆佸抚锛夈€傚畠杩橀獙璇?**arg** 鐨勫唴瀛樹綅缃紝骞朵负鍙傛暟淇濈暀鏍堝唴瀛樸€傝繖浣垮緱 **audio_ioctl()** 鐨勫疄鐜版瘮鏃ч┍鍔ㄦ柟妗堢畝鍗曞緱澶氥€備緥濡傦紝浣犲彲浠ユ煡鐪嬪嚱鏁?**cm206_audio_ioctl()**锛坄cm206.c` 涓級浠ラ厤鍚堟湰鏂囨。鏇存柊銆?

鏈疄鐜扮殑 ioctl 搴斿綋杩斿洖 **-ENOSYS**锛屼絾鏃犲鐨勮姹傦紙渚嬪 **CDROMSTART**锛夊彲浠ラ€氳繃杩斿洖 0锛堟垚鍔燂級鏉ュ拷鐣ャ€傚叾浠栭敊璇簲閬靛惊鐩稿簲鐨勬爣鍑嗐€傚綋搴曞眰椹卞姩杩斿洖閿欒鏃讹紝缁熶竴 CD-ROM 椹卞姩鍦ㄥ彲鑳芥椂灏介噺鎶婇敊璇爜杩斿洖缁欒皟鐢ㄧ▼搴忋€傦紙涓嶈繃鎴戜滑涔熷彲鑳藉喅瀹氬湪 **cdrom_ioctl()** 涓鏁磋繑鍥炲€硷紝浠ヤ繚璇佸闊抽鎾斁鍣ㄨ蒋浠舵彁渚涚粺涓€鎺ュ彛銆傦級

```
	int dev_ioctl(struct cdrom_device_info *cdi,
		      unsigned int cmd, unsigned long arg)

```
鏌愪簺 **ioctl()** 浼间箮鏄壒瀹氫簬鏌愪簺 CD-ROM 椹卞姩鍣ㄧ殑銆備篃灏辨槸璇达紝瀹冧滑琚紩鍏ユ槸涓轰簡鏈嶅姟浜庢煇浜涢┍鍔ㄥ櫒鐨勬煇浜涜兘鍔涖€備簨瀹炰笂锛屾湁 6 绉嶄笉鍚岀殑 **ioctl()** 鐢ㄤ簬璇诲彇鏁版嵁锛岃涔堜互鏌愮鐗瑰畾鏍煎紡锛岃涔堟槸闊抽鏁版嵁銆傛垜璁や负鏀寔鎶婇煶杞ㄤ綔涓烘暟鎹鍙栫殑椹卞姩鍣ㄤ笉澶氾紝杩欐槸鍥犱负瑕佷繚鎶よ壓鏈鐨勭増鏉冦€傛澶栵紝鎴戣涓哄鏋滄敮鎸侀煶杞紝搴斿綋閫氳繃 VFS 鑰岄潪 **ioctl()** 鏉ュ疄鐜般€傝繖閲岀殑涓€涓棶棰樺彲鑳芥槸闊抽甯ч暱 2352 瀛楄妭锛屽洜姝ら煶棰戞枃浠剁郴缁熻涔堝簲褰撲竴娆℃€ц姹?75264 瀛楄妭锛?12 鍜?2352 鐨勬渶灏忓叕鍊嶆暟锛夛紝瑕佷箞椹卞姩搴斿綋璐瑰姴鍘诲簲瀵硅繖绉嶄笉涓€鑷达紙鎴戝姝ゆ寔鍙嶅鎬佸害锛夈€傚啀鑰咃紝纭欢寰堥毦鎵惧埌绮剧‘鐨勫抚杈圭晫锛屽洜涓洪煶棰戝抚涓病鏈夊悓姝ュご銆備竴鏃﹁В鍐充簡杩欎簺闂锛岃繖娈典唬鐮佸氨搴斿綋鍦?`cdrom.c` 涓爣鍑嗗寲銆?

鍥犱负鏈夊姝ゅ鐨?**ioctl()** 浼间箮鏄嚭浜庤縼灏辨煇浜涢┍鍔ㄨ€屽紩鍏ョ殑 [#f2]_锛屼换浣曢潪鏍囧噯鐨?**ioctl()** 閮介€氳繃璋冪敤 **dev_ioctl()** 璺敱銆傚師鍒欎笂锛宍private`锛堢鏈夛級**ioctl()** 鐨勭紪鍙峰簲褰撴寜鐓ц澶囩殑涓昏澶囧彿鏉ュ畾锛岃€屼笉鏄€氱敤鐨?CD-ROM **ioctl** 缂栧彿 `0x53`銆傜洰鍓嶄笉琚敮鎸佺殑 **ioctl()** 鏈夛細

	CDROMREADMODE1, CDROMREADMODE2, CDROMREADAUDIO, CDROMREADRAW,
	CDROMREADCOOKED, CDROMSEEK, CDROMPLAY-BLK and CDROM-READALL


   鏈夌湡姝ｄ娇鐢ㄨ繖浜涙帴鍙ｇ殑杞欢鍚楋紵鎴戝緢鎰熷叴瓒ｏ紒


### CD-ROM 鑳藉姏


闄や簡浠呬粎瀹炵幇鏌愪簺 **ioctl** 璋冪敤涔嬪锛宍cdrom.c` 涓殑鎺ュ彛杩樻彁渚涗簡琛ㄦ槑 CD-ROM 椹卞姩鍣?**capabilities**锛堣兘鍔涳級鐨勫彲鑳芥€с€傝繖鍙互閫氳繃鍦ㄦ敞鍐屾椂鎶?`cdrom.h` 涓畾涔夌殑浠绘剰鏁伴噺鐨勮兘鍔涘父閲忓仛 OR锛堟垨锛夎繍绠楁潵瀹炵幇锛?

```
	CDC_CLOSE_TRAY		/* can close tray by software control */
	CDC_OPEN_TRAY		/* can open tray */
	CDC_LOCK		/* can lock and unlock the door */
	CDC_SELECT_SPEED	/* can select speed, in units of * sim*150 ,kB/s */
	CDC_SELECT_DISC		/* drive is juke-box */
	CDC_MULTI_SESSION	/* can read sessions *> rm1* */
	CDC_MCN			/* can read Media Catalog Number */
	CDC_MEDIA_CHANGED	/* can report if disc has changed */
	CDC_PLAY_AUDIO		/* can perform audio-functions (play, pause, etc) */
	CDC_RESET		/* hard reset device */
	CDC_IOCTLS		/* driver has non-standard ioctls */
	CDC_DRIVE_STATUS	/* driver implements drive status */

```
鑳藉姏鏍囧織琚０鏄庝负 **const**锛屼互闃叉椹卞姩鎰忓绡℃敼鍏跺唴瀹广€傝兘鍔涙爣蹇楀疄闄呬笂鍛婅瘔 `cdrom.c` 椹卞姩鑳藉仛浠€涔堛€傚鏋滈┍鍔ㄦ壘鍒扮殑椹卞姩鍣ㄤ笉鍏峰璇ヨ兘鍔涳紝鍙互閫氳繃 **cdrom_device_info** 鍙橀噺 **mask** 鎶婂畠灞忚斀鎺夈€備緥濡傦紝SCSI CD-ROM 椹卞姩宸茬粡瀹炵幇浜嗚鍏ュ拰寮瑰嚭 CD-ROM 鐨勪唬鐮侊紝鍥犳瀹冪殑 **capability** 涓浉搴旂殑鏍囧織浼氳璁剧疆銆備絾涓€涓?SCSI CD-ROM 椹卞姩鍣ㄥ彲鑳芥槸鍖ｅ紡锛坈addy锛夌郴缁燂紝鏃犳硶瑁呭叆鎵樼洏锛屽洜姝ゅ浜庤繖涓┍鍔ㄥ櫒锛?*cdrom_device_info** 缁撴瀯浼氬湪 **mask** 涓缃?**CDC_CLOSE_TRAY** 浣嶃€?

```
	if (cdo->capability & ~cdi->mask & CDC _<capability>) ...

```
娌℃湁鐢ㄤ簬璁剧疆 mask 鐨?**ioctl**鈥︹€﹀師鍥犳槸鎴戣涓烘帶鍒?**behavior**锛堣涓猴級姣旀帶鍒?**capabilities**锛堣兘鍔涳級鏇村ソ銆?

### 閫夐」


鏈€鍚庝竴涓爣蹇楀瘎瀛樺櫒鎺у埗 CD-ROM 椹卞姩鍣ㄧ殑 **behavior**锛堣涓猴級锛屼互婊¤冻涓嶅悓鐢ㄦ埛鐨勬剰鎰匡紝甯屾湜杩欒兘鐙珛浜庣浉搴斾綔鑰咃紙纰板阀璁╄椹卞姩鍣ㄧ殑鏀寔杩涘叆 Linux 绀惧尯鐨勪汉锛夌殑鎯虫硶銆傝瀵勫瓨鍣ㄥ垵濮嬪€间负锛?

```
	CDO_AUTO_CLOSE	/* try to close tray upon device open() */
	CDO_AUTO_EJECT	/* try to open tray on last device close() */
	CDO_USE_FFLAGS	/* use file_pointer->f_flags to indicate purpose for open() */
	CDO_LOCK	/* try to lock door if device is opened */
	CDO_CHECK_TYPE	/* ensure disc type is data if opened for data */

```
璇ュ瘎瀛樺櫒鐨勫垵濮嬪€间负 `CDO_AUTO_CLOSE | CDO_USE_FFLAGS | CDO_LOCK`锛屽弽鏄犳垜涓汉瀵圭敤鎴风晫闈㈠拰杞欢鏍囧噯鐨勭湅娉曘€傚湪浣犳姉璁箣鍓嶏紝`cdrom.c` 涓疄鐜颁簡涓や釜鏂扮殑 **ioctl()**锛屽厑璁镐綘鎺у埗锛?

```
	CDROM_SET_OPTIONS	/* set options specified in (int)arg */
	CDROM_CLEAR_OPTIONS	/* clear options specified in (int)arg */

```
鏈変竴涓€夐」闇€瑕佹洿澶氳В閲婏細**CDO_USE_FFLAGS**銆傚湪涓嬩竴鑺備腑鎴戜滑灏嗚В閲婁负浠€涔堥渶瑕佽繖涓€夐」銆?

涓€涓悕涓?`setcd` 鐨勮蒋浠跺寘锛屽彲浠?Debian 鍙戣鐗堝拰 `sunsite.unc.edu` 鑾峰彇锛屽厑璁哥敤鎴风骇鎺у埗杩欎簺鏍囧織銆?


## 浜嗚В鎵撳紑 CD-ROM 璁惧涔嬬洰鐨勭殑闇€瑕?


浼犵粺涓婏紝Unix 璁惧鍙互浠ヤ袱绉嶄笉鍚岀殑 `modes`锛堟ā寮忥級浣跨敤锛岃涔堥€氳繃瀵硅澶囨枃浠惰繘琛岃/鍐欙紝瑕佷箞閫氳繃瀵硅澶囧彂鍑烘帶鍒跺懡浠わ紝鍗宠澶囩殑 **ioctl()** 璋冪敤銆侰D-ROM 椹卞姩鍣ㄧ殑闂鍦ㄤ簬锛屽畠浠彲浠ョ敤浜庝袱涓畬鍏ㄤ笉鍚岀殑鐩殑銆傚叾涓€鏄寕杞藉彲绉诲姩鏂囦欢绯荤粺锛屽嵆 CD-ROM锛涘叾浜屾槸鎾斁闊抽 CD銆傞煶棰戝懡浠ゅ畬鍏ㄩ€氳繃 **ioctl()** 瀹炵幇锛屽ぇ姒傛槸鍥犱负鏈€鍒濈殑瀹炵幇锛圫UN?锛夊氨鏄姝ゃ€傚師鍒欎笂杩欐病浠€涔堥棶棰橈紝浣嗗 `CD player`锛圕D 鎾斁鍣級鐨勮壇濂芥帶鍒惰姹傝澶?**濮嬬粓** 鑳藉琚墦寮€锛屼互渚垮彂鍑?**ioctl** 鍛戒护锛岃€屼笉绠￠┍鍔ㄥ櫒澶勪簬浠€涔堢姸鎬併€?

鍙︿竴鏂归潰锛屽綋鐢ㄤ綔鍙Щ鍔ㄤ粙璐ㄧ鐩橀┍鍔ㄥ櫒锛堣繖姝ｆ槸 CD-ROM 鐨勬渶鍒濈洰鐨勶級鏃讹紝鎴戜滑甯屾湜纭繚鍦ㄦ墦寮€璁惧鏃剁鐩橀┍鍔ㄥ櫒宸插噯澶囧ソ鎿嶄綔銆傚湪鏃ф柟妗堜腑锛屾煇浜?CD-ROM 椹卞姩涓嶅仛浠讳綍瀹屾暣鎬ф鏌ワ紝瀵艰嚧鍦ㄨ瘯鍥惧湪涓€涓┖椹卞姩鍣ㄤ笂鎸傝浇 CD-ROM 鏃讹紝VFS 鍚戝唴鏍告姤鍛婅嫢骞?i/o 閿欒銆傝繖涓嶆槸涓€绉嶅彂鐜版病鏈夋彃鍏?CD-ROM 鐨勭壒鍒紭闆呯殑鏂瑰紡锛涘畠澶氬皯鏈夌偣鍍忚€佺殑 IBM-PC 璇曞浘璇讳竴涓┖杞┍鍑犵閽燂紝鐒跺悗绯荤粺鎶辨€ㄦ棤娉曡鍙栥€傚浠婃垜浠彲浠?**sense**锛堟劅鐭ワ級椹卞姩鍣ㄤ腑鏄惁瀛樺湪鍙Щ鍔ㄤ粙璐紝鎴戜滑鐩镐俊鎴戜滑搴斿綋鍒╃敤杩欎竴浜嬪疄銆傚湪鎵撳紑璁惧鏃惰繘琛屼竴娆″畬鏁存€ф鏌ワ紝楠岃瘉 CD-ROM 鏄惁鍙敤鍙婂叾姝ｇ‘绫诲瀷锛堟暟鎹級锛屽皢鏄彲鍙栫殑銆?

杩欎袱绉嶄娇鐢?CD-ROM 椹卞姩鍣ㄧ殑鏂瑰紡鈥斺€斾富瑕佺敤浜庢暟鎹紝鍏舵鐢ㄤ簬鎾斁闊抽鐩樷€斺€斿 **open()** 璋冪敤鐨勮涓烘湁涓嶅悓瑕佹眰銆傞煶棰戠敤閫斿彧鏄兂鎵撳紑璁惧浠ヨ幏寰楃敤浜庡彂鍑?**ioctl** 鍛戒护鎵€闇€鐨勬枃浠跺彞鏌勶紝鑰屾暟鎹敤閫旀兂涓烘纭彲闈犵殑鏁版嵁浼犺緭鑰屾墦寮€銆?

鐢ㄦ埛绋嬪簭鑳藉琛ㄦ槑鍏舵墦寮€璁惧涔?**purpose**锛堢洰鐨勶級鐨勫敮涓€鏂瑰紡锛屾槸閫氳繃 **flags**锛堟爣蹇楋級鍙傛暟锛堣 `open(2)`锛夈€傚 CD-ROM 璁惧鑰岃█锛岃繖浜涙爣蹇楀苟鏈疄鐜帮紙鏌愪簺椹卞姩瀹炵幇浜嗗鍐欑浉鍏虫爣蹇楃殑妫€鏌ワ紝浣嗗鏋滆澶囨枃浠舵嫢鏈夋纭殑鏉冮檺鏍囧織锛岃繖骞堕潪涓ユ牸蹇呰锛夈€傚ぇ澶氭暟閫夐」鏍囧織瀵?CD-ROM 璁惧鏍规湰娌℃湁鎰忎箟锛?*O_CREAT**銆?*O_NOCTTY**銆?*O_TRUNC**銆?*O_APPEND** 鍜?**O_SYNC** 瀵?CD-ROM 姣棤鎰忎箟銆?

鍥犳鎴戜滑鎻愯浣跨敤鏍囧織 **O_NONBLOCK** 鏉ヨ〃鏄庤澶囪鎵撳紑浠呬粎鏄负浜嗗彂鍑?**ioctl** 鍛戒护銆備弗鏍煎湴璇达紝**O_NONBLOCK** 鐨勫惈涔夋槸鎵撳紑鍙婇殢鍚庡璁惧鐨勮皟鐢ㄤ笉浼氬鑷磋皟鐢ㄨ繘绋嬬瓑寰呫€傛垜浠彲浠ユ妸瀹冪悊瑙ｄ负锛氫笉瑕佺瓑寰呮湁浜烘彃鍏ユ煇涓湁鏁堢殑鏁版嵁 CD-ROM銆傚洜姝わ紝鎴戜滑瀵?CD-ROM 鐨?**open()** 璋冪敤鐨勫疄鐜版彁璁涓嬶細

- 濡傛灉娌℃湁璁剧疆闄?**O_RDONLY** 涔嬪鐨勫叾浠栨爣蹇楋紝璁惧琚墦寮€鐢ㄤ簬鏁版嵁浼犺緭锛屽苟涓斿彧鏈夊湪浼犺緭鎴愬姛鍒濆鍖栧悗鎵嶈繑鍥?0銆傝璋冪敤鐢氳嚦鍙兘鍦?CD-ROM 涓婂紩鍙戜竴浜涘姩浣滐紝渚嬪鍚堜笂鎵樼洏銆?
- 濡傛灉璁剧疆浜嗛€夐」鏍囧織 **O_NONBLOCK**锛岄櫎闈炴暣涓澶囦笉瀛樺湪锛屽惁鍒欐墦寮€灏嗘€绘槸鎴愬姛銆傞┍鍔ㄥ櫒涓嶄細閲囧彇浠讳綍鍔ㄤ綔銆?

### 閭ｄ箞鏍囧噯鍛紵


浣犲彲鑳戒細鐘硅鲍鏄惁鎺ュ彈杩欎竴鎻愯锛屽洜涓哄畠鏉ヨ嚜 Linux 绀惧尯锛岃€屼笉鏄潵鑷煇涓爣鍑嗗寲鏈烘瀯銆係UN銆丼GI銆丠P 浠ュ強鎵€鏈夐偅浜涘叾浠?Unix 鍜岀‖浠跺巶鍟嗘€庝箞璇达紵鍡紝杩欎簺鍏徃澶勪簬涓€绉嶅垢杩愮殑浣嶇疆锛氬畠浠€氬父鍚屾椂鎺у埗鎵€鏀寔浜у搧鐨勭‖浠跺拰杞欢锛屽苟涓旇妯¤冻澶熷ぇ浠ヨ瀹氳嚜宸辩殑鏍囧噯銆傚畠浠笉蹇呭簲浠樺崄鍑犵鎴栨洿澶氫笉鍚屼笖鐩镐簰绔炰簤鐨勭‖浠堕厤缃甛 [#f3]_銆?


   椤轰究璇翠竴鍙ワ紝鎴戣涓?SUN 鎸傝浇 CD-ROM 鐨勬柟寮忓湪鏍规簮涓婃槸寰堝ソ鐨勶細鍦?Solaris 涓嬶紝涓€涓嵎瀹堟姢杩涚▼鑷姩鎶婃柊鎻掑叆鐨?CD-ROM 鎸傝浇鍒?`/cdrom/**<volume-name>**`銆?

   鍦ㄦ垜鐪嬫潵锛屽畠浠湰搴旀妸杩欎竴鐐规帹寰楁洿杩滐紝璁╁眬鍩熺綉涓婄殑 **姣忎釜** CD-ROM 閮芥寕杞藉湪绫讳技鐨勪綅缃紝鍗虫棤璁轰綘鎶?CD-ROM 鎻掑叆鍝彴鐗瑰畾鏈哄櫒锛屽畠鎬绘槸鍑虹幇鍦ㄧ洰褰曟爲涓殑鐩稿悓浣嶇疆锛屽湪姣忎釜绯荤粺涓婇兘鏄姝ゃ€傚綋鎴戞兂涓?Linux 瀹炵幇杩欐牱涓€涓敤鎴风▼搴忔椂锛屾垜閬囧埌浜嗗悇绉嶉┍鍔ㄥ湪琛屼负涓婄殑宸紓锛屼互鍙婂涓€涓姤鍛婁粙璐ㄥ彉鏇寸殑 **ioctl** 鐨勯渶瑕併€?

鎴戜滑鐩镐俊锛屼娇鐢?**O_NONBLOCK** 鏉ヨ〃鏄庤澶囪鎵撳紑浠呯敤浜?**ioctl** 鍛戒护锛屽彲浠ヨ交鏄撳湴鍦?Linux 绀惧尯涓紩鍏ャ€傛墍鏈?CD 鎾斁鍣ㄧ殑浣滆€呴兘蹇呴』琚憡鐭ワ紝鎴戜滑鐢氳嚦鍙互鑷繁缁欒繖浜涚▼搴忓彂閫佽ˉ涓併€?*O_NONBLOCK** 鐨勪娇鐢ㄥ鍏朵粬鎿嶄綔绯荤粺锛圠inux 涔嬪锛変笂鐨?CD 鎾斁鍣ㄧ殑琛屼负鏋佹湁鍙兘娌℃湁褰卞搷銆傛渶鍚庯紝鐢ㄦ埛鎬昏兘閫氳繃璋冪敤 **ioctl(file_descriptor, CDROM_CLEAR_OPTIONS, CDO_USE_FFLAGS)** 鎭㈠鍒版棫鐨勮涓恒€?

### *open()* 鐨勬帹鑽愮瓥鐣?


`cdrom.c` 涓殑渚嬬▼琚璁℃垚鍙互閫氳繃 **CDROM_SET/CLEAR_OPTIONS** **ioctls** 鍦ㄨ繍琛屾椂閰嶇疆 CD-ROM 璁惧锛?*浠讳綍** 绫诲瀷锛夌殑琛屼负銆傚洜姝わ紝鍙互璁剧疆澶氱鎿嶄綔妯″紡锛?

`CDO_AUTO_CLOSE | CDO_USE_FFLAGS | CDO_LOCK`
   杩欐槸榛樿璁剧疆銆傦紙灏嗘潵鍔犱笂 **CDO_CHECK_TYPE** 浼氭洿濂姐€傦級濡傛灉娌℃湁鍏朵粬杩涚▼鎵撳紑璇ヨ澶囷紝骞朵笖璁惧琚墦寮€鐢ㄤ簬鏁版嵁锛堟湭璁剧疆 **O_NONBLOCK**锛変笖鍙戠幇鎵樼洏鏄墦寮€鐨勶紝鍒欎細灏濊瘯鍚堜笂鎵樼洏銆傜劧鍚庯紝楠岃瘉椹卞姩鍣ㄤ腑鏈変竴寮犵洏鐗囷紝骞朵笖濡傛灉璁剧疆浜?**CDO_CHECK_TYPE**锛岄獙璇佸叾涓寘鍚?`data mode 1` 绫诲瀷鐨勮建閬撱€傚彧鏈夊綋鎵€鏈夋祴璇曢兘閫氳繃鏃讹紝杩斿洖鍊兼墠涓洪浂銆傞棬琚攣瀹氫互闃叉鏂囦欢绯荤粺鎹熷潖銆傚鏋滈┍鍔ㄥ櫒琚墦寮€鐢ㄤ簬闊抽锛堣缃簡 **O_NONBLOCK**锛夛紝鍒欎笉閲囧彇浠讳綍鍔ㄤ綔锛岃繑鍥炲€间负 0銆?

`CDO_AUTO_CLOSE | CDO_AUTO_EJECT | CDO_LOCK`
   杩欐ā浠夸簡褰撳墠 sbpcd-driver 鐨勮涓恒€傞€夐」鏍囧織琚拷鐣ワ紝蹇呰鏃跺湪绗竴娆℃墦寮€鏃跺悎涓婃墭鐩樸€傜被浼煎湴锛屽湪鏈€鍚庝竴娆?release 鏃舵墦寮€鎵樼洏锛屽嵆濡傛灉鍗歌浇浜嗕竴涓?CD-ROM锛屽畠浼氳嚜鍔ㄥ脊鍑猴紝浠ヤ究鐢ㄦ埛鏇存崲銆?

鎴戜滑甯屾湜杩欎簺閫夐」鑳藉璇存湇鎵€鏈変汉锛堥┍鍔ㄧ淮鎶よ€呭拰鐢ㄦ埛绋嬪簭寮€鍙戣€咃級閲囩撼鏂扮殑 CD-ROM 椹卞姩鏂规鍜岄€夐」鏍囧織瑙ｉ噴銆?

## `cdrom.c` 涓緥绋嬬殑鎻忚堪


`cdrom.c` 涓彧鏈夊皯鏁颁緥绋嬪鍑虹粰浜嗛┍鍔ㄣ€傚湪杩欎竴鏂拌妭涓垜浠皢璁ㄨ杩欎簺渚嬬▼锛屼互鍙婇偅浜?`take over`锛堟帴绠★級瀵瑰唴鏍哥殑 CD-ROM 鎺ュ彛鐨勭殑鍑芥暟銆俙cdrom.c` 鎵€灞炵殑澶存枃浠跺彨鍋?`cdrom.h`銆備互鍓嶏紝杩欎釜鏂囦欢鐨勯儴鍒嗗唴瀹规斁鍦ㄦ枃浠?`ucdrom.h` 涓紝浣嗙幇鍦ㄨ鏂囦欢宸茬粡鍚堝苟鍥炰簡 `cdrom.h`銆?

```
	struct file_operations cdrom_fops

```
璇ョ粨鏋勭殑鍐呭宸插湪 cdrom_api_ 涓弿杩般€傛寚鍚戣缁撴瀯鐨勬寚閽堣璧嬬粰 **struct gendisk** 鐨?**fops** 瀛楁銆?

```
	int register_cdrom(struct cdrom_device_info *cdi)

```
璇ュ嚱鏁扮殑浣跨敤鏂瑰紡锛屽ぇ鑷翠笂涓庢妸 **cdrom_fops** 娉ㄥ唽鍒板唴鏍哥殑鏂瑰紡鐩稿悓鈥斺€旇澶囨搷浣滃拰淇℃伅缁撴瀯锛屽 cdrom_api_ 涓墍杩帮紝搴斿綋鐢ㄤ互涓嬫柟寮忔敞鍐岋細

```
	register_cdrom(&<device>_info);

```
璇ュ嚱鏁板湪鎴愬姛鏃惰繑鍥為浂锛屽け璐ユ椂杩斿洖闈為浂銆傜粨鏋?**<device>_info** 搴斿綋鏈変竴涓寚鍚戜互涓嬪唴瀹圭殑鎸囬拡锛?

```
	struct cdrom_device_info <device>_info = {
		<device>_dops;
		...
	}

```
娉ㄦ剰锛屼竴涓┍鍔ㄥ繀椤绘湁涓€涓潤鎬佺粨鏋?**<device>_dops**锛岃€屽畠鍙互鏍规嵁娲昏穬鐨勬璁惧鏁伴噺鎷ユ湁浠绘剰澶氫釜缁撴瀯 **<device>_info**銆?*Register_cdrom()** 鐢ㄨ繖浜涙瀯寤轰竴涓摼琛ㄣ€?


```
	void unregister_cdrom(struct cdrom_device_info *cdi)

```
鎶婃璁惧鍙蜂负 **MINOR(cdi->dev)** 鐨勮澶?**cdi** 娉ㄩ攢锛屼細浠庡垪琛ㄤ腑绉婚櫎璇ユ璁惧銆傚鏋滃畠鏄搴曞眰椹卞姩娉ㄥ唽鐨勬渶鍚庝竴涓璁惧锛屽垯鏂紑宸叉敞鍐岀殑璁惧鎿嶄綔渚嬬▼涓?CD-ROM 鎺ュ彛鐨勮繛鎺ャ€傝鍑芥暟鍦ㄦ垚鍔熸椂杩斿洖闆讹紝澶辫触鏃惰繑鍥為潪闆躲€?

```
	int cdrom_open(struct inode * ip, struct file * fp)

```
璇ュ嚱鏁颁笉浼氳搴曞眰椹卞姩鐩存帴璋冪敤锛屽畠鍒楀湪鏍囧噯 **cdrom_fops** 涓€傚鏋?VFS 鎵撳紑涓€涓枃浠讹紝璇ュ嚱鏁拌婵€娲汇€傝渚嬬▼涓疄鐜颁簡涓€绉嶇瓥鐣ワ紝澶勭悊杩炴帴鍒拌璁惧鐨?**cdrom_device_ops** 涓缃殑鎵€鏈夎兘鍔涘拰閫夐」銆傜劧鍚庯紝绋嬪簭娴佺▼杞Щ鍒拌澶囩浉鍏崇殑 **open()** 璋冪敤銆?

```
	void cdrom_release(struct inode *ip, struct file *fp)

```
璇ュ嚱鏁板疄鐜颁簡 **cdrom_open()** 鐨勯€嗛€昏緫锛岀劧鍚庤皟鐢ㄨ澶囩浉鍏崇殑 **release()** 渚嬬▼銆傚綋浣跨敤璁℃暟杈惧埌 0 鏃讹紝閫氳繃璋冪敤 **sync_dev(dev)** 鍜?**invalidate_buffers(dev)** 鍒锋柊宸插垎閰嶇殑缂撳啿鍖恒€?



```
	int cdrom_ioctl(struct inode *ip, struct file *fp,
			unsigned int cmd, unsigned long arg)

```
璇ュ嚱鏁颁互缁熶竴鐨勬柟寮忓鐞?CD-ROM 璁惧鐨勬墍鏈夋爣鍑?**ioctl** 璇锋眰銆傝繖浜涗笉鍚岀殑璋冪敤鍒嗕负涓夌被锛氬彲浠ョ洿鎺ョ敱璁惧鎿嶄綔瀹炵幇鐨?**ioctl()**銆侀€氳繃璋冪敤 **audio_ioctl()** 璺敱鐨勶紝浠ュ強鍏朵綑閭ｄ簺澶ф鏄澶囩浉鍏崇殑銆傞€氬父锛岃礋鐨勮繑鍥炲€艰〃绀哄嚭閿欍€?

### 鐩存帴瀹炵幇鐨?*ioctl()*


涓嬪垪 `old`锛堟棫鐨勶級CD-ROM **ioctl()** 鍦?**cdrom_device_ops** 涓疄鐜颁笖鏈灞忚斀鐨勬儏鍐典笅锛岄€氳繃鐩存帴璋冪敤璁惧鎿嶄綔鏉ュ疄鐜帮細

`CDROMMULTISESSION`
	璇锋眰 CD-ROM 涓婄殑鏈€鍚庝竴涓細璇濄€?
`CDROMEJECT`
	鎵撳紑鎵樼洏銆?
`CDROMCLOSETRAY`
	鍚堜笂鎵樼洏銆?
`CDROMEJECT_SW`
	濡傛灉 **arg\not=0**锛岃缃涓轰负鑷姩鍚堜笂锛堢涓€娆℃墦寮€鏃跺悎涓婃墭鐩橈級鍜岃嚜鍔ㄥ脊鍑猴紙鏈€鍚庝竴娆￠噴鏀炬椂寮瑰嚭锛夛紝鍚﹀垯璁剧疆琛屼负涓哄湪 **open()** 鍜?**release()** 璋冪敤鏃朵笉绉诲姩銆?
`CDROM_GET_MCN`
	浠?CD 鑾峰彇濯掍綋鐩綍鍙枫€?

### 閫氳繃 *audio_ioctl()* 璺敱鐨?*ioctl*


涓嬮潰杩欑粍 **ioctl()** 閮介€氳繃璋冪敤 **cdrom_fops** 鍑芥暟 **audio_ioctl()** 瀹炵幇銆傚唴瀛樻鏌ュ拰鍒嗛厤鍦?**cdrom_ioctl()** 涓墽琛岋紝鍦板潃鏍煎紡锛?*CDROM_LBA**/**CDROM_MSF**锛夌殑瑙勬暣涔熷湪鍏朵腑瀹屾垚銆?

`CDROMSUBCHNL`
	鍦ㄧ被鍨嬩负 `struct cdrom_subchnl *` 鐨勫弬鏁?**arg** 涓幏鍙栧瓙閫氶亾鏁版嵁銆?
`CDROMREADTOCHDR`
	璇诲彇鐩綍锛圱able of Contents锛夊ご锛屽湪绫诲瀷涓?`struct cdrom_tochdr *` 鐨?**arg** 涓€?
`CDROMREADTOCENTRY`
	鍦?**arg** 涓鍙栦竴涓洰褰曢」锛屽苟鐢辩被鍨嬩负 `struct cdrom_tocentry *` 鐨?**arg** 鎸囧畾銆?
`CDROMPLAYMSF`
	鎾斁浠ュ垎銆佺銆佸抚鏍煎紡鎸囧畾鐨勯煶棰戠墖娈碉紝鐢辩被鍨嬩负 `struct cdrom_msf *` 鐨?**arg** 鐣屽畾銆?
`CDROMPLAYTRKIND`
	浠ヨ建-绱㈠紩鏍煎紡鎾斁闊抽鐗囨锛岀敱绫诲瀷涓?`struct cdrom_ti *` 鐨?**arg** 鐣屽畾銆?
`CDROMVOLCTRL`
	璁剧疆鐢辩被鍨嬩负 `struct cdrom_volctrl *` 鐨?**arg** 鎸囧畾鐨勯煶閲忋€?
`CDROMVOLREAD`
	鎶婇煶閲忚鍏ョ敱绫诲瀷涓?`struct cdrom_volctrl *` 鐨?**arg** 鎸囧畾鐨勪綅缃€?
`CDROMSTART`
	浣跨洏鐗囧姞閫熸棆杞€?
`CDROMSTOP`
	鍋滄鎾斁闊抽鐗囨銆?
`CDROMPAUSE`
	鏆傚仠鎾斁闊抽鐗囨銆?
`CDROMRESUME`
	鎭㈠鎾斁銆?

### `cdrom.c` 涓柊鐨?*ioctl()*


涓嬪垪 **ioctl()** 琚紩鍏ワ紝浠ュ厑璁哥敤鎴风▼搴忔帶鍒跺悇涓?CD-ROM 璁惧鐨勮涓恒€傛柊鐨?**ioctl** 鍛戒护鍙互閫氳繃鍏跺悕绉颁腑鐨勪笅鍒掔嚎鏉ヨ瘑鍒€?

`CDROM_SET_OPTIONS`
	璁剧疆鐢?**arg** 鎸囧畾鐨勯€夐」銆傝繑鍥炰慨鏀瑰悗鐨勯€夐」鏍囧織瀵勫瓨鍣ㄣ€備娇鐢?**arg = \rm0** 璇诲彇褰撳墠鏍囧織銆?
`CDROM_CLEAR_OPTIONS`
	娓呴櫎鐢?**arg** 鎸囧畾鐨勯€夐」銆傝繑鍥炰慨鏀瑰悗鐨勯€夐」鏍囧織瀵勫瓨鍣ㄣ€?
`CDROM_SELECT_SPEED`
	閫夋嫨鐩樼墖纾佸ご閫熺巼锛岀敱 **arg** 浠ユ爣鍑?cdrom 閫熷害涓哄崟浣嶏紙176\,kB/sec 鍘熷鏁版嵁鎴?150kB/sec 鏂囦欢绯荤粺鏁版嵁锛夋寚瀹氥€傚€?0 琛ㄧず `auto-select`锛堣嚜鍔ㄩ€夋嫨锛夛紝鍗抽煶棰戠洏浠ュ疄鏃堕€熷害鎾斁锛屾暟鎹洏浠ユ渶澶ч€熷害璇诲彇銆傛暟鍊?**arg** 浼氬鐓у湪 **cdrom_dops** 涓壘鍒扮殑椹卞姩鍣ㄦ渶澶х澶撮€熺巼杩涜妫€鏌ャ€?
`CDROM_SELECT_DISC`
	浠庢崲鐗囧櫒锛坖uke-box锛変腑閫夋嫨缂栧彿涓?**arg** 鐨勭洏鐗囥€?

	绗竴寮犵洏鐗囩紪鍙蜂负 0銆?*arg** 浼氬鐓у湪 **cdrom_dops** 涓壘鍒扮殑鎹㈢墖鍣ㄤ腑鐩樼墖鐨勬渶澶ф暟閲忚繘琛屾鏌ャ€?
`CDROM_MEDIA_CHANGED`
	濡傛灉鑷笂娆¤皟鐢ㄤ互鏉ョ洏鐗囧凡鏇存崲鍒欒繑鍥?1銆傚浜庢崲鐗囧櫒锛岄澶栫殑鍙傛暟 **arg** 鎸囧畾浜嗘彁渚涗俊鎭殑妲戒綅銆傜壒娈婂€?**CDSL_CURRENT** 璇锋眰杩斿洖鍏充簬褰撳墠閫変腑妲戒綅鐨勪俊鎭€?
`CDROM_TIMED_MEDIA_CHANGE`
	妫€鏌ヨ嚜鐢ㄦ埛鎻愪緵鐨勬煇涓椂闂翠互鏉ョ洏鐗囨槸鍚﹀凡鏇存崲锛屽苟杩斿洖鏈€鍚庝竴娆＄洏鐗囨洿鎹㈢殑鏃堕棿銆?

	**arg** 鏄寚鍚?**cdrom_timed_media_change_info** 缁撴瀯鐨勬寚閽堛€?*arg->last_media_change** 鍙敱璋冪敤浠ｇ爜璁剧疆锛屼互琛ㄧず宸茬煡鐨勬渶鍚庝竴娆′粙璐ㄥ彉鏇寸殑鏃堕棿鎴筹紙鐢辫皟鐢ㄨ€呯粰鍑猴級銆傛垚鍔熻繑鍥炴椂锛岃 ioctl 璋冪敤浼氭妸 **arg->last_media_change** 璁句负鍐呮牳/椹卞姩鎵€鐭ョ殑鏈€鏂颁粙璐ㄥ彉鏇存椂闂存埑锛堜互姣璁★級锛屽苟鎶?**arg->has_changed** 璁句负 1锛堝鏋滆鏃堕棿鎴虫瘮璋冪敤鑰呰缃殑鏃堕棿鎴虫洿鏂帮級銆?
`CDROM_DRIVE_STATUS`
	閫氳繃璋冪敤 **drive_status()** 杩斿洖椹卞姩鍣ㄧ殑鐘舵€併€傝繑鍥炲€煎湪 cdrom_drive_status_ 涓畾涔夈€傛敞鎰忥紝璇ヨ皟鐢ㄤ笉杩斿洖椹卞姩鍣ㄥ綋鍓嶆挱鏀炬椿鍔ㄧ殑淇℃伅锛涜繖鍙€氳繃鍚?**CDROMSUBCHNL** 鍙戝嚭 **ioctl** 璋冪敤鏉ヨ疆璇€傚浜庢崲鐗囧櫒锛岄澶栫殑鍙傛暟 **arg** 鎸囧畾浜嗘彁渚涳紙鍙兘鍙楅檺鐨勶級淇℃伅鐨勬Ы浣嶃€傜壒娈婂€?**CDSL_CURRENT** 璇锋眰杩斿洖鍏充簬褰撳墠閫変腑妲戒綅鐨勪俊鎭€?
`CDROM_DISC_STATUS`
	杩斿洖椹卞姩鍣ㄤ腑褰撳墠鐩樼墖鐨勭被鍨嬨€傚畠搴斿綋琚湅浣滄槸瀵?**CDROM_DRIVE_STATUS** 鐨勮ˉ鍏呫€傝 **ioctl** 鍙互鎻愪緵鍏充簬椹卞姩鍣ㄤ腑鎻掑叆鐨勫綋鍓嶇洏鐗囩殑 **鏌愪簺** 淇℃伅銆傝繖涓€鍔熻兘杩囧幓鐢卞簳灞傞┍鍔ㄥ疄鐜帮紝浣嗙幇鍦ㄥ畬鍏ㄥ湪缁熶竴 CD-ROM 椹卞姩涓墽琛屻€?

	CD 浣滀负鍚勭鏁板瓧淇℃伅杞戒綋浠嬭川鐨勪娇鐢ㄥ彂灞曞彶锛屽鑷翠簡璁稿涓嶅悓鐨勭洏鐗囩被鍨嬨€傝 **ioctl** 浠呭湪 CD 涓婂彧鏈?**涓€绉?* 绫诲瀷鐨勬暟鎹椂鎵嶆湁鐢ㄣ€傝櫧鐒惰繖缁忓父鏄簨瀹烇紝浣?CD 鍚屾椂鎷ユ湁涓€浜涙暟鎹建閬撳拰涓€浜涢煶棰戣建閬撲篃闈炲父甯歌銆傚洜涓鸿繖鏄竴涓凡鏈夌殑鎺ュ彛锛岃€屼笉鏄€氳繃鏀瑰彉鍏舵墍鍩轰簬鐨勫亣璁炬潵淇璇ユ帴鍙ｏ紙浠庤€岀牬鍧忔墍鏈変娇鐢ㄨ鍔熻兘鐨勭敤鎴风▼搴忥級锛岀粺涓€ CD-ROM 椹卞姩鎸夊涓嬫柟寮忓疄鐜拌 **ioctl**锛氬鏋滄墍璁ㄨ鐨?CD 涓婃湁闊抽杞ㄩ亾锛屽苟涓斿畠涓婇潰缁濆娌℃湁 CD-I銆乆A 鎴栨暟鎹建閬擄紝瀹冨皢琚姤鍛婁负 **CDS_AUDIO**銆傚鏋滃畠鍚屾椂鏈夐煶棰戝拰鏁版嵁杞ㄩ亾锛屽畠灏嗚繑鍥?**CDS_MIXED**銆傚鏋滅洏鐗囦笂娌℃湁闊抽杞ㄩ亾锛屽苟涓旀墍璁ㄨ鐨?CD 涓婃湁浠讳綍 CD-I 杞ㄩ亾锛屽畠灏嗚鎶ュ憡涓?**CDS_XA_2_2**銆傚鏋滆繕涓嶈锛屽鏋滄墍璁ㄨ鐨?CD 涓婃湁浠讳綍 XA 杞ㄩ亾锛屽畠灏嗚鎶ュ憡涓?**CDS_XA_2_1**銆傛渶鍚庯紝濡傛灉鎵€璁ㄨ鐨?CD 涓婃湁浠讳綍鏁版嵁杞ㄩ亾锛屽畠灏嗚鎶ュ憡涓烘暟鎹?CD锛?*CDS_DATA_1**锛夈€?

```
		CDS_NO_INFO	/* no information available */
		CDS_NO_DISC	/* no disc is inserted, or tray is opened */
		CDS_AUDIO	/* Audio disc (2352 audio bytes/frame) */
		CDS_DATA_1	/* data disc, mode 1 (2048 user bytes/frame) */
		CDS_XA_2_1	/* mixed data (XA), mode 2, form 1 (2048 user bytes) */
		CDS_XA_2_2	/* mixed data (XA), mode 2, form 1 (2324 user bytes) */
		CDS_MIXED	/* mixed audio/data disc */

	For some information concerning frame layout of the various disc
	types, see a recent version of `cdrom.h`.

```
`CDROM_CHANGER_NSLOTS`
	杩斿洖鎹㈢墖鍣ㄤ腑鐨勬Ы浣嶆暟閲忋€?
`CDROMRESET`
	澶嶄綅椹卞姩鍣ㄣ€?
`CDROM_GET_CAPABILITY`
	杩斿洖椹卞姩鍣ㄧ殑 **capability** 鏍囧織銆傚叧浜庤繖浜涙爣蹇楃殑鏇村淇℃伅锛屽弬瑙?cdrom_capabilities_ 涓€鑺傘€?
`CDROM_LOCKDOOR`
	閿佸畾椹卞姩鍣ㄧ殑闂ㄣ€俙arg == 0` 瑙ｉ攣闂紝浠讳綍鍏朵粬鍊奸攣瀹氬畠銆?
`CDROM_DEBUG`
	鎵撳紑璋冭瘯淇℃伅銆傚彧鍏佽 root 杩欐牱鍋氥€傝涔変笌 CDROM_LOCKDOOR 鐩稿悓銆?


### 璁惧鐩稿叧鐨?*ioctl()*


鏈€鍚庯紝鎵€鏈夊叾浠?**ioctl()** 閮借浼犵粰鍑芥暟 **dev_ioctl()**锛堝鏋滃凡瀹炵幇锛夈€備笉鎵ц鍐呭瓨鍒嗛厤鎴栭獙璇併€?

## 濡備綍鏇存柊浣犵殑椹卞姩


- 澶囦唤浣犲綋鍓嶇殑椹卞姩銆?
- 鑾峰彇鏂囦欢 `cdrom.c` 鍜?`cdrom.h`锛屽畠浠簲褰撲綅浜庨殢鏈枃妗ｄ竴鍚屾彁渚涚殑鐩綍鏍戜腑銆?
- 纭繚浣犲寘鍚簡 `cdrom.h`銆?
- 鎶?**register_blkdev** 鐨勭 3 涓弬鏁颁粠 `&<your-drive>_fops` 鏀逛负 `&cdrom_fops`銆?
- 灏卞湪璇ヨ涔嬪悗锛屾坊鍔犱互涓嬪唴瀹逛互鍚戠粺涓€ CD-ROM 椹卞姩娉ㄥ唽锛?

```
	register_cdrom(&<your-drive>_info);*

  Similarly, add a call to *unregister_cdrom()* at the appropriate place.
```
- 鎶婅澶囨搷浣?**struct** 鐨勪竴涓緥瀛愬鍒跺埌浣犵殑婧愮爜涓紝渚嬪鏉ヨ嚜 `cm206.c` 鐨?**cm206_dops**锛屽苟鎶婃墍鏈夋潯鐩敼鎴愪笌浣犻┍鍔ㄥ搴旂殑鍚嶅瓧锛屾垨浣犵宸у枩娆㈢殑鍚嶅瓧銆傚鏋滀綘鐨勯┍鍔ㄤ笉鏀寔鏌愪釜鍑芥暟锛屾妸璇ユ潯鐩涓?**NULL**銆傚湪 **capability** 鏉＄洰澶勶紝浣犲簲鍒楀嚭浣犵殑椹卞姩褰撳墠鏀寔鐨勬墍鏈夎兘鍔涖€傚鏋滀綘鐨勯┍鍔ㄦ嫢鏈夋煇涓湭鍒楀嚭鐨勮兘鍔涳紝璇风粰鎴戝彂娑堟伅銆?
- 浠庡悓涓€涓ず渚嬮┍鍔ㄥ鍒?**cdrom_device_info** 澹版槑锛屽苟鏍规嵁浣犵殑闇€瑕佷慨鏀规潯鐩€傚鏋滀綘鐨勯┍鍔ㄥ姩鎬佺‘瀹氱‖浠剁殑鑳藉姏锛岃缁撴瀯涔熷簲褰撳姩鎬佸０鏄庛€?
- 鏍规嵁 `cdrom.h` 涓垪鍑虹殑鍘熷瀷鍜?cdrom_api_ 涓粰鍑虹殑瑙勬牸锛屽疄鐜颁綘鐨?`<device>_dops` 缁撴瀯涓殑鎵€鏈夊嚱鏁般€備綘寰堝彲鑳藉凡缁忓疄鐜颁簡鍏朵腑寰堝ぇ涓€閮ㄥ垎浠ｇ爜锛屽苟涓斾綘鍑犱箮鑲畾闇€瑕佽皟鏁村師鍨嬪拰杩斿洖鍊笺€?
- 鎶婁綘鐨?`<device>_ioctl()` 鍑芥暟閲嶅懡鍚嶄负 **audio_ioctl** 骞剁◢寰敼鍔ㄥ師鍨嬨€傜Щ闄?cdrom_ioctl_ 绗竴閮ㄥ垎涓垪鍑虹殑鏉＄洰锛屽鏋滀綘鐨勪唬鐮佹病闂锛岃繖浜涘彧鏄浣犲湪鍓嶄竴姝ヨ皟鏁寸殑渚嬬▼鐨勮皟鐢ㄣ€?
- 浣犲彲浠ョЩ闄?**audio_ioctl()** 鍑芥暟涓墍鏈夊鐞嗛煶棰戝懡浠ょ殑鍐呭瓨妫€鏌ヤ唬鐮侊紙杩欎簺鍒楀湪 cdrom_ioctl_ 鐨勭浜岄儴鍒嗭級銆備篃涓嶉渶瑕佸唴瀛樺垎閰嶏紝鍥犳 **switch** 涓殑澶у鏁?**case** 褰㈠锛?

```
	case CDROMREADTOCENTRY:
		get_toc_entry\bigl((struct cdrom_tocentry *) arg);

- 鎵€鏈夊墿浣欑殑 **ioctl** case 蹇呴』绉诲埌涓€涓嫭绔嬬殑鍑芥暟 **<device>_ioctl** 涓紝鍗宠澶囩浉鍏崇殑 **ioctl()**銆傛敞鎰忥紝鍐呭瓨妫€鏌ュ拰鍒嗛厤蹇呴』淇濈暀鍦ㄨ繖娈典唬鐮佷腑锛?
- 鏀瑰彉 **<device>_open()** 鍜?**<device>_release()** 鐨勫師鍨嬶紝骞剁Щ闄や换浣曠瓥鐣ユ€т唬鐮侊紙鍗虫墭鐩樿繍鍔ㄣ€佽埍闂ㄩ攣瀹氱瓑锛夈€?
- 灏濊瘯閲嶆柊缂栬瘧椹卞姩銆傛垜浠缓璁綘浣跨敤妯″潡锛屾棤璁烘槸 `cdrom.o` 杩樻槸浣犵殑椹卞姩锛屽洜涓鸿繖鏍疯皟璇曡瀹规槗寰楀銆?

## 鑷磋阿


鎰熻阿鎵€鏈夊弬涓庣殑浜恒€傞鍏堟劅璋?Erik Andersen锛屼粬鎺ヨ繃浜嗙淮鎶?`cdrom.c` 骞跺湪 2.1 鍐呮牳涓暣鍚堝ぇ閲?CD-ROM 鐩稿叧浠ｇ爜鐨勭伀鐐€傛劅璋?Scott Snyder 鍜?Gerd Knorr锛屼粬浠槸鐜囧厛涓?SCSI 鍜?IDE-CD 椹卞姩瀹炵幇杩欎竴鎺ュ彛銆佸苟鐩稿浜?kernel~2.0 涓烘暟鎹粨鏋勬墿灞曟彁鍑鸿澶氭兂娉曠殑浜恒€傝繘涓€姝ユ劅璋?Heiko Ei脽feldt銆乀homas Quinot銆丣on Tombs銆並en Pizzini銆丒berhard M枚nkeberg 鍜?Andrew Kroll锛岃繖浜?Linux CD-ROM 璁惧椹卞姩寮€鍙戣€呭湪鎾板啓杩囩▼涓杽鎰忓湴缁欏嚭浜嗗缓璁拰鎵硅瘎銆傛渶鍚庡綋鐒惰鎰熻阿 Linus Torvalds锛屾槸浠栭鍏堣杩欎竴鍒囨垚涓哄彲鑳姐€?
