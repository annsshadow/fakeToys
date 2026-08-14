## usbmon


## 绠€浠?


灏忓啓鐨勫悕绉?鈥渦sbmon鈥?鎸囦唬鍐呮牳涓殑涓€椤硅鏂斤紝鐢ㄤ簬鏀堕泦 USB 鎬荤嚎涓婄殑 I/O 杩借釜淇℃伅銆傝鍔熻兘绫讳技浜?tcpdump(1) 鎴?Ethereal 绛夌綉缁滅洃鎺у伐鍏锋墍浣跨敤鐨勫寘濂楁帴瀛椼€傜被浼煎湴锛岄鏈熶細浣跨敤涓€涓濡?usbdump 鎴?USBMon锛堝ぇ鍐欏瓧姣嶏級涔嬬被鐨勫伐鍏锋潵妫€鏌ョ敱 usbmon 浜х敓鐨勫師濮嬭拷韪暟鎹€?

usbmon 鎶ュ憡鐨勬槸澶栬鐗瑰畾椹卞姩鍚戜富鏈烘帶鍒跺櫒椹卞姩锛圚CD锛夊彂鍑虹殑璇锋眰銆傚洜姝わ紝濡傛灉 HCD 瀛樺湪缂洪櫡锛寀sbmon 鎶ュ憡鐨勮拷韪彲鑳藉苟涓嶇簿纭搴旀€荤嚎涓婄殑浜嬪姟銆傝繖涓?tcpdump 鐨勬儏鍐电浉鍚屻€?

鐩墠瀹炵幇浜嗕袱濂?API锛氣€渢ext鈥濓紙鏂囨湰锛変笌 鈥渂inary鈥濓紙浜岃繘鍒讹級銆備簩杩涘埗 API 閫氳繃 /dev 鍛藉悕绌洪棿涓嬬殑瀛楃璁惧鎻愪緵锛屾槸涓€涓?ABI銆傛枃鏈?API 鑷?2.6.35 璧峰凡寮冪敤锛屼絾鍑轰簬渚垮埄鎬т粛鍙娇鐢ㄣ€?

## 濡備綍浣跨敤 usbmon 鏀堕泦鍘熷鏂囨湰杩借釜


涓庡寘濂楁帴瀛椾笉鍚岋紝usbmon 鎻愪緵浜嗕竴涓互鏂囨湰鏍煎紡鎻愪緵杩借釜鐨勬帴鍙ｃ€傝繖鐢ㄤ簬涓や釜鐩殑銆傞鍏堬紝鍦ㄦ洿瀹屽杽鐨勬牸寮忔渶缁堢‘瀹氫箣鍓嶏紝瀹冧綔涓哄伐鍏蜂箣闂撮€氱敤鐨勮拷韪氦鎹㈡牸寮忋€傚叾娆★紝鍦ㄥ伐鍏蜂笉鍙敤鏃讹紝浜虹被涔熷彲浠ラ槄璇诲畠銆?

瑕佹敹闆嗗師濮嬫枃鏈拷韪紝璇锋墽琛屼互涓嬫楠ゃ€?

### 1. 鍑嗗


鎸傝浇 debugfs锛堝繀椤诲湪浣犵殑鍐呮牳閰嶇疆涓惎鐢級锛屽苟鍔犺浇 usbmon 妯″潡锛堝鏋滀互鍐呮牳妯″潡鏂瑰紡鏋勫缓锛夈€傜浜屾鍦ㄦā鍧楀唴缃椂浼氳璺宠繃
```
	# mount -t debugfs none_debugs /sys/kernel/debug
	# modprobe usbmon
	#
```
```
	# ls /sys/kernel/debug/usb/usbmon
	0s  0u  1s  1t  1u  2s  2t  2u  3s  3t  3u  4s  4t  4u
	#
```
鐜板湪浣犲彲浠ラ€夋嫨浣跨敤濂楁帴瀛?'0u'锛堟崟鑾锋墍鏈夋€荤嚎涓婄殑鍖咃級锛屽苟璺冲埌姝ラ #3锛涙垨鑰呬娇鐢ㄦ楠?#2 鎵惧嚭浣犵殑璁惧鎵€浣跨敤鐨勬€荤嚎銆傝繖鏍峰彲浠ヨ繃婊ゆ帀閭ｄ簺鎸佺画閫氫俊鐨勭儲浜鸿澶囥€?

### 2. 鎵惧嚭杩炴帴鐩爣璁惧鐨勬€荤嚎


杩愯 鈥渃at /sys/kernel/debug/usb/devices鈥濓紝骞舵壘鍒颁笌璇ヨ澶囧搴旂殑 T 琛屻€傞€氬父浣犻€氳繃鏌ユ壘鍘傚晢瀛楃涓叉潵鍋氬埌杩欎竴鐐广€傚鏋滀綘鏈夎澶氱浉浼肩殑璁惧锛屽彲浠ユ嫈鎺夊叾涓竴涓苟瀵规瘮涓ゆ
/sys/kernel/debug/usb/devices 鐨勮緭鍑恒€俆 琛屼細甯︽湁涓€涓€荤嚎鍙枫€?

```
  T:  Bus=03 Lev=01 Prnt=01 Port=00 Cnt=01 Dev#=  2 Spd=12  MxCh= 0
  D:  Ver= 1.10 Cls=00(>ifc ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
  P:  Vendor=0557 ProdID=2004 Rev= 1.00
  S:  Manufacturer=ATEN
  S:  Product=UC100KM V2.00
```
鈥淏us=03鈥?琛ㄧず杩欐槸鎬荤嚎 3銆傛垨鑰咃紝浣犱篃鍙互鏌ョ湅 鈥渓susb鈥?鐨勮緭鍑猴紝骞朵粠鐩稿簲琛岃幏鍙栨€荤嚎鍙枫€備緥濡傦細

Bus 003 Device 002: ID 0557:2004 ATEN UC100KM V2.00

### 3. 鍚姩 'cat'


```
	# cat /sys/kernel/debug/usb/usbmon/3u > /tmp/1.mon.out
```
```
	# cat /sys/kernel/debug/usb/usbmon/0u > /tmp/1.mon.out
```
璇ヨ繘绋嬩細涓€鐩磋鍙栵紝鐩村埌琚潃姝汇€傝嚜鐒讹紝杈撳嚭鍙互閲嶅畾鍚戝埌鏈熸湜鐨勪綅缃€傝繖鏄帹鑽愬仛娉曪紝鍥犱负杈撳嚭浼氱浉褰撻暱銆?

### 4. 鍦?USB 鎬荤嚎涓婃墽琛屾湡鏈涚殑鎿嶄綔


鍦ㄨ繖閲屼綘鎵ц鏌愪簺浼氫骇鐢熸祦閲忕殑鎿嶄綔锛氭彃鍏?U 鐩樸€佸鍒舵枃浠躲€佹帶鍒舵憚鍍忓ご绛夈€?

### 5. 鏉€姝?cat


閫氬父杩欓€氳繃閿洏涓柇锛圕ontrol-C锛夊畬鎴愩€?

姝ゆ椂杈撳嚭鏂囦欢锛堟湰渚嬩腑涓?/tmp/1.mon.out锛夊彲浠ヨ淇濆瓨銆侀€氳繃鐢靛瓙閭欢鍙戦€侊紝鎴栫敤鏂囨湰缂栬緫鍣ㄦ煡鐪嬨€傚湪鍚庝竴绉嶆儏鍐典笅锛岃纭繚鏂囦欢澶у皬瀵逛綘鐨勫父鐢ㄧ紪杈戝櫒鏉ヨ涓嶆槸杩囧ぇ銆?

## 鍘熷鏂囨湰鏁版嵁鏍煎紡


鐩墠鏀寔涓ょ鏍煎紡锛氬師濮嬬殑 '1t' 鏍煎紡涓?'1u' 鏍煎紡銆?1t' 鏍煎紡鍦?2.6.21 鍐呮牳涓凡寮冪敤銆?1u' 鏍煎紡澧炲姞浜嗚嫢骞插瓧娈碉紝渚嬪 ISO 甯ф弿杩扮銆侀棿闅旂瓑銆傚畠浜х敓鐨勮绋嶉暱涓€浜涳紝浣嗗湪鍏朵粬鏂归潰鏄?'1t' 鏍煎紡鐨勫畬缇庤秴闆嗐€?

濡傛灉甯屾湜鍦ㄧ▼搴忎腑鍖哄垎浜岃€咃紝鍙互鏌ョ湅 鈥渁ddress鈥?瀛楁锛堣涓嬫枃锛夛紝鍏朵腑 '1u' 鏍煎紡浼氶檮鍔犱竴涓€荤嚎鍙枫€傚鏋滃嚭鐜颁袱涓啋鍙凤紝鍒欎负 '1t' 鏍煎紡锛屽惁鍒欎负 '1u'銆?

浠讳綍鏂囨湰鏍煎紡鏁版嵁閮界敱涓€涓蹭簨浠剁粍鎴愶紝渚嬪 URB 鎻愪氦銆乁RB 鍥炶皟銆佹彁浜ら敊璇€傛瘡涓簨浠堕兘鏄竴琛屾枃鏈紝鐢辩┖鐧藉垎闅旂殑鍗曡瘝缁勬垚銆傚崟璇嶇殑鏁伴噺鎴栦綅缃彲鑳藉彇鍐充簬浜嬩欢绫诲瀷锛屼絾鏈変竴缁勫浜庢墍鏈夌被鍨嬮兘閫氱敤鐨勫崟璇嶃€?

浠ヤ笅鏄崟璇嶇殑鍒楄〃锛屼粠宸﹀埌鍙筹細

- URB 鏍囩锛圲RB Tag锛夈€傜敤浜庢爣璇?URB锛岄€氬父鏄?URB 缁撴瀯鍦ㄥ唴鏍镐腑鐨勫崄鍏繘鍒跺湴鍧€锛屼絾涔熷彲浠ユ槸涓€涓簭鍙锋垨浠讳綍鍏朵粬鍚堢悊鐨勫敮涓€瀛楃涓层€?

- 浠ュ井绉掍负鍗曚綅鐨勬椂闂存埑锛屼竴涓崄杩涘埗鏁板瓧銆傛椂闂存埑鐨勫垎杈ㄧ巼鍙栧喅浜庡彲鐢ㄧ殑鏃堕挓锛屽洜姝ゅ畠鍙兘杩滃樊浜庝竴寰锛堜緥濡傦紝濡傛灉瀹炵幇浣跨敤 jiffies锛夈€?

- 浜嬩欢绫诲瀷锛圗vent Type锛夈€傝绫诲瀷鎸囩殑鏄簨浠剁殑鏍煎紡锛岃€岄潪 URB 绫诲瀷銆傚彲鐢ㄧ被鍨嬫湁锛歋 - 鎻愪氦锛坰ubmission锛夛紝C - 鍥炶皟锛坈allback锛夛紝E - 鎻愪氦閿欒锛坰ubmission error锛夈€?

- 鈥淎ddress鈥?瀛楁锛堝師绉?鈥減ipe鈥濓級銆傚畠鐢卞洓涓互鍐掑彿鍒嗛殧鐨勫瓧娈电粍鎴愶細URB 绫诲瀷涓庢柟鍚戙€佹€荤嚎鍙枫€佽澶囧湴鍧€銆佺鐐瑰彿銆傜被鍨嬩笌鏂瑰悜鐢变袱涓瓧鑺傛寜濡備笅鏂瑰紡缂栫爜锛?

    == ==   =============================
    Ci Co   鎺у埗杈撳叆涓庤緭鍑?
    Zi Zo   绛夋椂杈撳叆涓庤緭鍑?
    Ii Io   涓柇杈撳叆涓庤緭鍑?
    Bi Bo   鎵归噺杈撳叆涓庤緭鍑?
    == ==   =============================

  鎬荤嚎鍙枫€佽澶囧湴鍧€鍜岀鐐归兘鏄崄杩涘埗鏁板瓧锛屼絾涓轰簡渚夸簬浜虹被闃呰锛屽畠浠彲鑳藉甫鏈夊墠瀵奸浂銆?

- URB 鐘舵€佸瓧娈碉紙URB Status word锛夈€傝繖鏄竴涓瓧姣嶏紝鎴栨槸鑻ュ共浠ュ啋鍙峰垎闅旂殑鏁板瓧锛歎RB 鐘舵€併€侀棿闅斻€佽捣濮嬪抚涓庨敊璇鏁般€備笌 鈥渁ddress鈥?瀛楁涓嶅悓锛岄櫎鐘舵€佸鐨勬墍鏈夊瓧娈甸兘鏄彲閫夌殑銆傞棿闅斾粎瀵逛腑鏂拰绛夋椂 URB 鎵撳嵃銆傝捣濮嬪抚浠呭绛夋椂 URB 鎵撳嵃銆傞敊璇鏁颁粎瀵圭瓑鏃跺洖璋冧簨浠舵墦鍗般€?

  鐘舵€佸瓧娈垫槸涓€涓崄杩涘埗鏁板瓧锛屾湁鏃朵负璐燂紝琛ㄧず URB 鐨?鈥渟tatus鈥?瀛楁銆傝瀛楁瀵规彁浜ゆ病鏈夋剰涔夛紝浣嗘棤璁哄浣曢兘浼氬瓨鍦ㄤ互甯姪鑴氭湰瑙ｆ瀽銆傚綋鍙戠敓閿欒鏃讹紝璇ュ瓧娈靛寘鍚敊璇爜銆?

  鍦ㄦ彁浜ゆ帶鍒跺寘鐨勬儏鍐典笅锛岃瀛楁鍖呭惈鐨勬槸 Setup 鏍囩锛圫etup Tag锛夛紝鑰岄潪涓€缁勬暟瀛椼€傚緢瀹规槗鍒ゆ柇 Setup 鏍囩鏄惁瀛樺湪锛屽洜涓哄畠姘歌繙涓嶄細鏄暟瀛椼€傚洜姝わ紝濡傛灉鑴氭湰鍦ㄨ瀛楁涓彂鐜颁竴缁勬暟瀛楋紝瀹冧滑浼氱户缁鍙栨暟鎹暱搴︼紙绛夋椂 URB 闄ゅ锛夈€傚鏋滃彂鐜板叾浠栧唴瀹癸紝渚嬪瀛楁瘝锛屽畠浠細鍦ㄨ鍙栨暟鎹暱搴︽垨绛夋椂鎻忚堪绗︿箣鍓嶅厛璇诲彇 setup 鍖呫€?

- Setup 鍖咃紙濡傛灉瀛樺湪锛夌敱 5 涓崟璇嶇粍鎴愶細鍒嗗埆瀵瑰簲 bmRequestType銆乥Request銆亀Value銆亀Index銆亀Length 鍚勪竴涓紝濡?USB Specification 2.0 鎵€瑙勫畾銆傚鏋?Setup 鏍囩涓?'s'锛岃繖浜涘崟璇嶅彲浠ュ畨鍏ㄨВ鐮併€傚惁鍒欙紝setup 鍖呮浘瀛樺湪浣嗘湭琚崟鑾凤紝瀛楁涓寘鍚～鍏呭€笺€?

- 绛夋椂甯ф弿杩扮鐨勬暟閲忎互鍙婃弿杩扮鏈韩銆傚鏋滀竴涓瓑鏃朵紶杈撲簨浠跺甫鏈変竴缁勬弿杩扮锛屼細鍏堟墦鍗颁竴涓?URB 涓殑鎻忚堪绗︽€绘暟锛岀劧鍚庢瘡涓弿杩扮涓€涓崟璇嶏紝鏈€澶?5 涓€傚崟璇嶇敱 3 涓互鍐掑彿鍒嗛殧鐨勫崄杩涘埗鏁板瓧缁勬垚锛屽垎鍒搴旂姸鎬併€佸亸绉讳笌闀垮害銆傚鎻愪氦鑰岃█锛屾姤鍛婄殑鏄垵濮嬮暱搴︺€傚鍥炶皟鑰岃█锛屾姤鍛婄殑鏄疄闄呴暱搴︺€?

- 鏁版嵁闀垮害锛圖ata Length锛夈€傚鎻愪氦鑰岃█锛岃繖鏄姹傜殑闀垮害銆傚鍥炶皟鑰岃█锛岃繖鏄疄闄呴暱搴︺€?

- 鏁版嵁鏍囩锛圖ata tag锛夈€傚嵆浣块暱搴﹂潪闆讹紝usbmon 涔熷彲鑳藉苟涓嶆€绘槸鎹曡幏鏁版嵁銆備粎褰撹鏍囩涓?'=' 鏃舵暟鎹崟璇嶆墠瀛樺湪銆?

- 鍏跺悗鐨勬暟鎹崟璇嶏紝閲囩敤澶х鍗佸叚杩涘埗鏍煎紡銆傛敞鎰忓畠浠苟闈炴満鍣ㄥ瓧锛岃€屽彧鏄鎷嗗垎鎴愬崟璇嶇殑瀛楄妭娴侊紝浠ヤ究浜庨槄璇汇€傚洜姝わ紝鏈€鍚庝竴涓崟璇嶅彲鑳藉寘鍚?1 鍒?4 涓瓧鑺傘€傛敹闆嗗埌鐨勬暟鎹暱搴︽槸鍙楅檺鐨勶紝鍙兘灏忎簬鏁版嵁闀垮害瀛楁涓姤鍛婄殑闀垮害銆傚湪绛夋椂杈撳叆锛圸i锛夊畬鎴愩€佷笖鎺ユ敹鏁版嵁鍦ㄧ紦鍐插尯涓█鐤忕殑鎯呭喌涓嬶紝鏀堕泦鍒扮殑鏁版嵁闀垮害鍙兘澶т簬鏁版嵁闀垮害鍊硷紙鍥犱负鏁版嵁闀垮害鍙粺璁″凡鎺ユ敹鐨勫瓧鑺傦紝鑰屾暟鎹崟璇嶅寘鍚暣涓紶杈撶紦鍐插尯锛夈€?

绀轰緥锛?

```
  d5ea89a0 3575914555 S Ci:1:001:0 s a3 00 0000 0003 0004 4 <
  d5ea89a0 3575914560 C Ci:1:001:0 0 4 = 01050000
```
涓€涓悜鍙戦€?SCSI 鍛戒护 0x28锛圧EAD_10锛夌殑 31 瀛楄妭杈撳嚭鎵归噺浼犺緭
```
  dd65f0e8 4128379752 S Bo:1:005:2 -115 31 = 55534243 ad000000 00800000 80010a28 20000000 20000040 00000000 000000
  dd65f0e8 4128379808 C Bo:1:005:2 0 31 >
```
## 鍘熷浜岃繘鍒舵牸寮忎笌 API


璇?API 鐨勬暣浣撴灦鏋勪笌涓婅堪鍩烘湰鐩稿悓锛屽彧鏄簨浠朵互浜岃繘鍒舵牸寮忎氦浠樸€傛瘡涓簨浠跺湪
```
  struct usbmon_packet {
	u64 id;			/*  0: URB ID - from submission to callback */
	unsigned char type;	/*  8: Same as text; extensible. */
	unsigned char xfer_type; /*    ISO (0), Intr, Control, Bulk (3) */
	unsigned char epnum;	/*     Endpoint number and transfer direction */
	unsigned char devnum;	/*     Device address */
	u16 busnum;		/* 12: Bus number */
	char flag_setup;	/* 14: Same as text */
	char flag_data;		/* 15: Same as text; Binary zero is OK. */
	s64 ts_sec;		/* 16: gettimeofday */
	s32 ts_usec;		/* 24: gettimeofday */
	int status;		/* 28: */
	unsigned int length;	/* 32: Length of data (submitted or actual) */
	unsigned int len_cap;	/* 36: Delivered length */
	union {			/* 40: */
		unsigned char setup[SETUP_LEN];	/* Only for Control S-type */
		struct iso_rec {		/* Only for ISO */
			int error_count;
			int numdesc;
		} iso;
	} s;
	int interval;		/* 48: Only for Interrupt and ISO */
	int start_frame;	/* 52: For ISO */
	unsigned int xfer_flags; /* 56: copy of URB's transfer_flags */
	unsigned int ndesc;	/* 60: Actual number of ISO descriptors */
  };				/* 64 total length */
```
杩欎簺浜嬩欢鍙互閫氳繃 read(2) 璇诲彇銆侀€氳繃 ioctl(2) 璋冪敤锛屾垨閫氳繃 mmap 璁块棶缂撳啿鍖烘潵浠庡瓧绗﹁澶囨帴鏀躲€備笉杩囧嚭浜庡吋瀹规€у師鍥狅紝read(2) 鍙繑鍥炲墠 48 瀛楄妭銆?

瀛楃璁惧閫氬父绉颁负 /dev/usbmonN锛屽叾涓?N 鏄?USB 鎬荤嚎鍙枫€傞浂鍙凤紙/dev/usbmon0锛夋槸鐗规畩鐨勶紝琛ㄧず 鈥滄墍鏈夋€荤嚎鈥濄€傛敞鎰忥紝鍏蜂綋鐨勫懡鍚嶇瓥鐣ョ敱浣犵殑 Linux 鍙戣鐗堝喅瀹氥€?

濡傛灉浣犳墜鍔ㄥ垱寤?/dev/usbmon0锛岃纭繚瀹冪敱 root 鎷ユ湁涓旀潈闄愭ā寮忎负 0600銆傚惁鍒欙紝闈炵壒鏉冪敤鎴峰皢鑳藉绐ユ帰閿洏娴侀噺銆?

浠ヤ笅鏄彲鐢ㄧ殑 ioctl 璋冪敤锛屽叾 MON_IOC_MAGIC 涓?0x92锛?

 MON_IOCQ_URB_LEN, 瀹氫箟涓?_IO(MON_IOC_MAGIC, 1)

璇ヨ皟鐢ㄨ繑鍥炰笅涓€涓簨浠朵腑鏁版嵁鐨勯暱搴︺€傛敞鎰忓ぇ澶氭暟浜嬩欢涓嶅寘鍚暟鎹紝鍥犳濡傛灉璇ヨ皟鐢ㄨ繑鍥為浂锛屽苟涓嶆剰鍛崇潃娌℃湁浜嬩欢鍙敤銆?

 MON_IOCG_STATS, 瀹氫箟涓?_IOR(MON_IOC_MAGIC, 3, struct mon_bin_stats)

```
  struct mon_bin_stats {
	u32 queued;
	u32 dropped;
  };
```
鎴愬憳 鈥渜ueued鈥?鎸囩殑鏄綋鍓嶅湪缂撳啿鍖轰腑鎺掗槦锛坬ueued锛夌殑浜嬩欢鏁帮紙鑰岄潪鑷笂娆￠噸缃互鏉ュ凡澶勭悊鐨勪簨浠舵暟锛夈€?

鎴愬憳 鈥渄ropped鈥?鏄嚜涓婃璋冪敤 MON_IOCG_STATS 浠ユ潵涓㈠け鐨勪簨浠舵暟銆?

 MON_IOCT_RING_SIZE, 瀹氫箟涓?_IO(MON_IOC_MAGIC, 4)

璇ヨ皟鐢ㄨ缃紦鍐插尯澶у皬銆傚弬鏁版槸浠ュ瓧鑺備负鍗曚綅鐨勫ぇ灏忋€傝澶у皬鍙兘琚悜涓嬪彇鏁村埌涓嬩竴涓潡锛堟垨椤碉級銆傚鏋滆姹傜殑澶у皬瓒呭嚭姝ゅ唴鏍哥殑 [鏈寚瀹歖 杈圭晫锛岃皟鐢ㄥ皢浠?-EINVAL 澶辫触銆?

 MON_IOCQ_RING_SIZE, 瀹氫箟涓?_IO(MON_IOC_MAGIC, 5)

璇ヨ皟鐢ㄨ繑鍥炵紦鍐插尯褰撳墠鐨勫瓧鑺傚ぇ灏忋€?

 MON_IOCX_GET, 瀹氫箟涓?_IOW(MON_IOC_MAGIC, 6, struct mon_get_arg)
 MON_IOCX_GETX, 瀹氫箟涓?_IOW(MON_IOC_MAGIC, 10, struct mon_get_arg)

濡傛灉鍐呮牳缂撳啿鍖轰腑娌℃湁浜嬩欢锛岃繖浜涜皟鐢ㄤ細绛夊緟浜嬩欢鍒拌揪锛岀劧鍚庤繑鍥炵涓€涓簨浠躲€傚弬鏁版槸鎸囧悜濡備笅缁撴瀯鐨勬寚閽?
```
  struct mon_get_arg {
	struct usbmon_packet *hdr;
	void *data;
	size_t alloc;		/* Length of data (can be zero) */
  };
```
鍦ㄨ皟鐢ㄤ箣鍓嶏紝hdr銆乨ata 涓?alloc 搴旇濉厖銆傝繑鍥炴椂锛宧dr 鎵€鎸囧悜鐨勫尯鍩熷寘鍚笅涓€涓簨浠剁粨鏋勶紝data 缂撳啿鍖哄寘鍚暟鎹紙濡傛灉鏈夛級銆傝浜嬩欢浼氫粠鍐呮牳缂撳啿鍖轰腑绉婚櫎銆?

MON_IOCX_GET 鍚?hdr 鍖哄煙澶嶅埗 48 瀛楄妭锛孧ON_IOCX_GETX 澶嶅埗 64 瀛楄妭銆?

 MON_IOCX_MFETCH, 瀹氫箟涓?_IOWR(MON_IOC_MAGIC, 7, struct mon_mfetch_arg)

璇?ioctl 涓昏鍦ㄥ簲鐢ㄧ▼搴忛€氳繃浠ヤ笅鏂瑰紡璁块棶缂撳啿鍖烘椂浣跨敤
```
  struct mon_mfetch_arg {
	uint32_t *offvec;	/* Vector of events fetched */
	uint32_t nfetch;	/* Number of events to fetch (out: fetched) */
	uint32_t nflush;	/* Number of events to flush */
  };
```
璇?ioctl 鍒?3 涓樁娈佃繍琛屻€?

棣栧厛锛屽畠浠庡唴鏍哥紦鍐插尯涓Щ闄ゅ苟涓㈠純鏈€澶?nflush 涓簨浠躲€傚疄闄呬涪寮冪殑浜嬩欢鏁拌繑鍥炲湪 nflush 涓€?

鍏舵锛岄櫎闈炰吉璁惧浠?O_NONBLOCK 鎵撳紑锛屽惁鍒欏畠浼氱瓑寰呯紦鍐插尯涓嚭鐜颁竴涓簨浠躲€?

绗笁锛屽畠灏嗘渶澶?nfetch 涓亸绉绘彁鍙栧埌 mmap 缂撳啿鍖轰腑锛屽苟瀛樺叆 offvec銆傚疄闄呯殑浜嬩欢鍋忕Щ鏁伴噺瀛樺叆 nfetch銆?

 MON_IOCH_MFLUSH, 瀹氫箟涓?_IO(MON_IOC_MAGIC, 8)

璇ヨ皟鐢ㄤ粠鍐呮牳缂撳啿鍖轰腑绉婚櫎涓€瀹氭暟閲忕殑浜嬩欢銆傚叾鍙傛暟鏄绉婚櫎鐨勪簨浠舵暟銆傚鏋滅紦鍐插尯涓凡鏈夌殑浜嬩欢灏戜簬璇锋眰鏁伴噺锛屽垯绉婚櫎鎵€鏈夊瓨鍦ㄧ殑浜嬩欢锛屼笖涓嶆姤鍛婇敊璇€傚湪娌℃湁浜嬩欢鍙敤鏃跺畠鍚屾牱鏈夋晥銆?

 FIONBIO

濡傛灉鏈夐渶瑕侊紝鏈潵鍙兘浼氬疄鐜?ioctl FIONBIO銆?

闄や簡 ioctl(2) 鍜?read(2)锛屼簩杩涘埗 API 鐨勭壒娈婃枃浠惰繕鍙互鐢?select(2) 鍜?poll(2) 杩涜杞銆備絾 lseek(2) 鏃犳硶宸ヤ綔銆?

- 浜岃繘鍒?API 鍐呮牳缂撳啿鍖虹殑鍐呭瓨鏄犲皠璁块棶

鍩烘湰鎬濊矾寰堢畝鍗曪細

鍏堣幏鍙栧綋鍓嶅ぇ灏忥紝鐒跺悗浣跨敤 mmap(2) 鏄犲皠缂撳啿鍖轰互鍋氬噯澶囥€?
```
   struct mon_mfetch_arg fetch;
   struct usbmon_packet *hdr;
   int nflush = 0;
   for (;;) {
      fetch.offvec = vec; // Has N 32-bit words
      fetch.nfetch = N;   // Or less than N
      fetch.nflush = nflush;
      ioctl(fd, MON_IOCX_MFETCH, &fetch);   // Process errors, too
      nflush = fetch.nfetch;       // This many packets to flush when done
      for (i = 0; i < nflush; i++) {
         hdr = (struct ubsmon_packet *) &mmap_area[vec[i]];
         if (hdr->type == '@')     // Filler packet
            continue;
         caddr_t data = &mmap_area[vec[i]] + 64;
         process_packet(hdr, data);
      }
   }
```
鍥犳锛屼富瑕佹€濇兂鏄瘡 N 涓簨浠朵粎鎵ц涓€娆?ioctl銆?

灏界缂撳啿鍖烘槸鐜舰鐨勶紝杩斿洖鐨勫ご閮ㄥ拰鏁版嵁涓嶄細璺ㄨ秺缂撳啿鍖烘湯灏撅紝鍥犳涓婇潰鐨勪吉浠ｇ爜涓嶉渶瑕佷换浣曡仛闆嗘搷浣溿€?
