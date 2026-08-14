## Floppy Driver锛堣蒋椹遍┍鍔級


## FAQ list锛堝父瑙侀棶棰樺垪琛級:


甯歌闂鍒楄〃鍙互鍦?fdutils 杞欢鍖咃紙瑙佷笅鏂囷級涓壘鍒帮紝涔熷彲浠ヨ闂?<https://fdutils.linux.lu/faq.html>銆?

## LILO 閰嶇疆閫夐」锛圱hinkpad 鐢ㄦ埛璇烽槄璇绘湰鑺傦級


杞┍椹卞姩閫氳繃 lilo 涓殑 'floppy=' 閫夐」杩涜閰嶇疆銆傝閫夐」鏃㈠彲浠ュ湪鍚姩鎻愮ず绗︿笅
杈撳叆锛屼篃鍙互鍐欏叆 lilo 閰嶇疆鏂囦欢銆?
绀轰緥锛氬鏋滀綘鐨勫唴鏍稿悕涓?linux-2.6.9锛岃杈撳叆浠ヤ笅涓€琛?```

 linux-2.6.9 floppy=thinkpad

```
浣犱篃鍙互鎶婁笅闈㈣繖琛屽啓鍏?/etc/lilo.conf 鐨勯厤缃弿杩颁腑
```

 append = "floppy=thinkpad"

```
```

 linux-2.6.9 floppy=daring floppy=two_fdc
 append = "floppy=daring floppy=two_fdc"

```
濡傛灉浣犲悓鏃跺湪 lilo 閰嶇疆鏂囦欢鍜屽惎鍔ㄦ彁绀虹涓嬮兘缁欏嚭浜嗛€夐」锛岄偅涔堜袱澶勭殑閫夐」瀛楃涓?浼氳鎷兼帴鍦ㄤ竴璧凤紝鍚姩鎻愮ず绗︿笅鐨勯€夐」鎺掑湪鏈€鍚庛€傝繖涔熸槸涓轰粈涔堣繕鎻愪緵浜嗕竴浜涚敤浜?鎭㈠榛樿琛屼负鐨勯€夐」銆?

## Module configuration options锛堟ā鍧楅厤缃€夐」锛?

```

	modprobe floppy floppy="<options>"

```
```

	modprobe floppy floppy="omnibook messages"

```
濡傛灉浣犳瘡娆″姞杞借蒋椹遍┍鍔ㄦ椂閮介渶瑕佸惎鐢ㄦ煇浜涢€夐」锛屽彲浠?```

	options floppy floppy="omnibook messages"

```
鍐欏叆 /etc/modprobe.d/ 鐩綍涓嬬殑鏌愪釜閰嶇疆鏂囦欢涓€?

杞┍椹卞姩鐩稿叧鐨勯€夐」濡備笅锛?
 floppy=asus_pci
	璁剧疆浣嶆帺鐮侊紝鍙厑璁?0 鍙峰拰 1 鍙疯澶囥€傦紙榛樿锛?
 floppy=daring
	鍛婅瘔杞┍椹卞姩浣犳嫢鏈変竴鍧楄涓鸿壇濂界殑杞洏鎺у埗鍣ㄣ€?	杩欐牱鍙互鑾峰緱鏇撮珮鏁堛€佹洿骞虫粦鐨勬搷浣滐紝浣嗗湪鏌愪簺鎺у埗鍣ㄤ笂鍙兘浼氬け璐ャ€?	杩欐湁鍙兘鍔犲揩鏌愪簺鎿嶄綔鐨勯€熷害銆?
 floppy=0,daring
	鍛婅瘔杞┍椹卞姩浣犵殑杞洏鎺у埗鍣ㄥ簲褰撹皑鎱庝娇鐢ㄣ€?
 floppy=one_fdc
	鍛婅瘔杞┍椹卞姩浣犲彧鏈変竴涓蒋鐩樻帶鍒跺櫒銆?	锛堥粯璁わ級

 floppy=two_fdc / floppy=<address>,two_fdc
	鍛婅瘔杞┍椹卞姩浣犳湁涓や釜杞洏鎺у埗鍣ㄣ€?	绗簩涓蒋鐩樻帶鍒跺櫒鍋囧畾浣嶄簬 <address>銆?	濡傛灉绗簩涓帶鍒跺櫒浣嶄簬鍦板潃 0x370锛屽苟涓斾綘浣跨敤浜?'cmos' 閫夐」锛?	鍒欎笉闇€瑕佹閫夐」銆?
 floppy=thinkpad
	鍛婅瘔杞┍椹卞姩浣犱娇鐢ㄧ殑鏄?Thinkpad銆俆hinkpad 瀵圭鐩樻洿鎹㈢嚎
	浣跨敤浜嗗弽杞殑绾﹀畾銆?
 floppy=0,thinkpad
	鍛婅瘔杞┍椹卞姩浣犳病鏈変娇鐢?Thinkpad銆?
 floppy=omnibook / floppy=nodma
	鍛婅瘔杞┍椹卞姩涓嶈浣跨敤 Dma 杩涜鏁版嵁浼犺緭銆?	HP Omnibook 闇€瑕佷娇鐢ㄦ閫夐」锛屽洜涓哄畠娌℃湁鍙敤鐨勮蒋椹?DMA 閫氶亾銆?	濡傛灉浣犻绻佹敹鍒?"Unable to allocate DMA memory" 娑堟伅锛屾閫夐」涔熷緢鏈夌敤銆?	浜嬪疄涓婏紝dma 鍐呭瓨闇€瑕佸湪鐗╃悊鍐呭瓨涓繛缁紝鍥犳鏇撮毦鎵惧埌锛岃€岄潪 dma 鐨勭紦鍐插尯
	鍙互鍦ㄨ櫄鎷熷唴瀛樹腑鍒嗛厤銆備笉杩囷紝濡傛灉浣犵殑 FDC 娌℃湁 FIFO锛?272A 鎴?82072锛夛紝
	鎴戝缓璁笉瑕佷娇鐢ㄦ閫夐」銆?2072A 鍙婁互鍚庣殑鍨嬪彿閮藉彲浠ャ€備娇鐢?nodma 鑷冲皯闇€瑕?486銆?	濡傛灉浣跨敤 nodma 妯″紡锛屽缓璁綘鍚屾椂鎶?FIFO 闃堝€艰涓?10 鎴栨洿浣庯紝
	浠ラ檺鍒舵暟鎹紶杈撲腑鏂殑娆℃暟銆?
	濡傛灉浣犳嫢鏈夋敮鎸?FIFO 鐨?FDC锛屽綋鎵句笉鍒板彲鐢?DMA 鍐呭瓨鏃讹紝杞┍椹卞姩浼氳嚜鍔?	鍥為€€鍒伴潪 DMA 妯″紡銆傚鏋滀綘鎯抽伩鍏嶈繖绉嶆儏鍐碉紝鍙互鏄惧紡鍦拌姹?'yesdma'銆?
 floppy=yesdma
	鍛婅瘔杞┍椹卞姩瀛樺湪鍙敤鐨?DMA 閫氶亾銆?	锛堥粯璁わ級

 floppy=nofifo
	瀹屽叏绂佺敤 FIFO銆傚綋浣犲湪璁块棶杞┍鏃讹紝缃戝崱锛堟垨鍏朵粬璁惧锛夋姤鍑?	"Bus master arbitration error" 娑堟伅鏃堕渶瑕佷娇鐢ㄦ閫夐」銆?
 floppy=usefifo
	鍚敤 FIFO銆傦紙榛樿锛?
 floppy=<threshold>,fifo_depth
	璁剧疆 FIFO 闃堝€笺€傝繖鍦?DMA 妯″紡涓嬫渶涓虹浉鍏炽€傚鏋滈槇鍊艰緝楂橈紝
	杞┍椹卞姩鍙互瀹瑰繊鏇村鐨勪腑鏂欢杩燂紝浣嗕細瑙﹀彂鏇村鐨勪腑鏂紙鍗崇粰绯荤粺鍏朵綑閮ㄥ垎
	甯︽潵鏇村璐熻浇锛夈€傚鏋滈槇鍊艰緝浣庯紝涓柇寤惰繜涔熷簲璇ユ洿浣庯紙澶勭悊鍣ㄦ洿蹇級銆?	杈冧綆闃堝€肩殑濂藉鏄腑鏂洿灏戙€?
	瑕佽皟鏁?fifo 闃堝€硷紝鍙互浣跨敤 'floppycontrol --messages' 鎵撳紑
	over/underrun 娑堟伅銆傜劧鍚庤闂竴寮犺蒋鐩樸€傚鏋滀綘鏀跺埌澶ч噺
	"Over/Underrun - retrying" 娑堟伅锛岃鏄?fifo 闃堝€艰繃浣庛€傚皾璇曚娇鐢ㄦ洿楂樼殑鍊硷紝
	鐩村埌鍙伓灏斿嚭鐜?Over/Underrun 涓烘銆傚湪杩涜姝ら」璋冧紭鏃讹紝鏈€濂藉皢杞┍椹卞姩
	缂栬瘧涓烘ā鍧椼€傚洜涓鸿繖鏍峰氨鍙互鍦ㄤ笉閲嶅惎鏈哄櫒鐨勬儏鍐典笅灏濊瘯涓嶅悓鐨?fifo 鍊笺€?	娉ㄦ剰姣忔閲嶆柊鎻掑叆妯″潡鏃堕兘闇€瑕佹墽琛?'floppycontrol --messages'銆?
	閫氬父涓嶉渶瑕佽皟鏁?fifo 闃堝€硷紝鍥犱负榛樿鍊硷紙0xa锛夊凡缁忔瘮杈冨悎鐞嗐€?
 floppy=<drive>,<type>,cmos
	灏?<drive> 鐨?CMOS 绫诲瀷璁句负 <type>銆傚鏋滀綘鎷ユ湁瓒呰繃涓や釜杞┍
	锛堢墿鐞?CMOS 鍙兘鎻忚堪涓や釜锛夛紝鎴栬€呬綘鐨?BIOS 浣跨敤浜嗛潪鏍囧噯鐨?CMOS 绫诲瀷锛?	鍒欐椤逛负蹇呭～銆侰MOS 绫诲瀷濡備笅锛?
	       ==  ==================================
		0  Use the value of the physical CMOS
		1  5 1/4 DD
		2  5 1/4 HD
		3  3 1/2 DD
		4  3 1/2 HD
		5  3 1/2 ED
		6  3 1/2 ED
	       16  unknown or not installed
	       ==  ==================================

	锛堟敞锛欵D 椹卞姩鍣ㄦ湁涓や釜鏈夋晥绫诲瀷銆傝繖鏄洜涓烘渶鍒濋€夋嫨 5 鏉ヨ〃绀鸿蒋鐩?*纾佸甫**锛?	鑰?6 琛ㄧず ED 椹卞姩鍣ㄣ€侫MI 蹇界暐浜嗚繖涓€鐐癸紝鎶?5 鐢ㄤ簬 ED 椹卞姩鍣ㄣ€?	杩欏氨鏄负浠€涔堣蒋椹遍┍鍔ㄥ悓鏃跺鐞嗕袱鑰呫€傦級

 floppy=unexpected_interrupts
	褰撴敹鍒版剰澶栦腑鏂椂鎵撳嵃璀﹀憡娑堟伅銆?	锛堥粯璁わ級

 floppy=no_unexpected_interrupts / floppy=L40SX
	褰撴敹鍒版剰澶栦腑鏂椂涓嶆墦鍗版秷鎭€傚湪 IBM L40SX 绗旇鏈數鑴戠殑鏌愪簺瑙嗛妯″紡涓?	闇€瑕佷娇鐢ㄦ閫夐」銆傦紙瑙嗛涓庤蒋椹变箣闂翠技涔庡瓨鍦ㄧ浉浜掍綔鐢ㄣ€傛剰澶栦腑鏂彧褰卞搷鎬ц兘锛?	鍙互瀹夊叏鍦板拷鐣ャ€傦級

 floppy=broken_dcl
	涓嶄娇鐢ㄧ鐩樻洿鎹㈢嚎锛岃€屾槸鍋囪姣忔閲嶆柊鎵撳紑璁惧鑺傜偣鏃剁鐩橀兘宸叉洿鎹€?	鏌愪簺纾佺洏鏇存崲绾挎崯鍧忔垨涓嶈鏀寔鐨勬満鍣ㄤ笂闇€瑕佷娇鐢ㄦ閫夐」銆?	杩欏簲琚涓轰竴绉嶄复鏃跺簲瀵规帾鏂斤紝鍥犱负瀹冧細鍥犱笉蹇呰鐨勭紦瀛樺埛鏂?	鑰岄檷浣庤蒋椹辨搷浣滅殑鏁堢巼锛屽苟涓旂暐寰洿涓嶅彲闈犮€傚鏋滀綘閬囧埌浠讳綍 DCL 闂锛?	璇锋鏌ヤ綘鐨勭嚎缂嗐€佽繛鎺ュ拰璺崇嚎璁剧疆銆備笉杩囷紝涓€浜涜緝鏃х殑椹卞姩鍣紝浠ュ強閮ㄥ垎
	绗旇鏈數鑴戯紝宸茬煡娌℃湁 DCL銆?
 floppy=debug
	鎵撳嵃璋冭瘯娑堟伅銆?
 floppy=messages
	涓烘煇浜涙搷浣滄墦鍗颁俊鎭€ф秷鎭紙纾佺洏鏇存崲閫氱煡銆佸叧浜?over/underrun 鐨勮鍛婏紝
	浠ュ強鍏充簬鑷姩妫€娴嬬殑娑堟伅锛夈€?
 floppy=silent_dcl_clear
	浣跨敤涓€绉嶆洿瀹夐潤鐨勬柟寮忔竻闄ょ鐩樻洿鎹㈢嚎锛堜笉娑夊強瀵婚亾锛夈€?daring' 閫夐」闅愬惈姝ら」銆?
 floppy=<nr>,irq
	灏嗚蒋椹?IRQ 璁句负 <nr>锛岃€屼笉鏄?6銆?
 floppy=<nr>,dma
	灏嗚蒋椹?DMA 閫氶亾璁句负 <nr>锛岃€屼笉鏄?2銆?
 floppy=slow
```

	   PS/2 杞┍鐨勬杩涢€熺巼姣旀櫘閫氳蒋椹辨參寰楀銆傚湪鏌愪簺鏇存瀬绔殑鎯呭舰涓嬶紝
	   寤鸿灏嗛€熷害闄嶅埌榛樿鍊肩殑绾?1/4銆?

```
## Supporting utilities and additional documentation锛堟敮鎸佸伐鍏蜂笌闄勫姞鏂囨。锛?


杞┍椹卞姩鐨勯澶栧弬鏁板彲浠ュ湪杩愯鏃堕厤缃€傚畬鎴愭鍔熻兘鐨勫伐鍏峰彲浠ュ湪 fdutils 杞欢鍖呬腑鎵惧埌銆?璇ヨ蒋浠跺寘杩樺寘鍚竴涓柊鐗堟湰鐨?mtools锛屽厑璁歌闂ぇ瀹归噺纾佺洏锛堝湪楂樺瘑搴?3 1/2 杞洏涓?鏈€楂樺彲杈?1992K锛侊級銆傚畠杩樺寘鍚叧浜庤蒋椹遍┍鍔ㄧ殑闄勫姞鏂囨。銆?
鏈€鏂扮増鏈彲浠ュ湪 fdutils 涓婚〉鎵惧埌锛?
 https://fdutils.linux.lu

fdutils 鍙戝竷鐗堟湰鍙互鍦ㄤ互涓嬪湴鍧€鎵惧埌锛?
 https://fdutils.linux.lu/download.html

 http://www.tux.org/pub/knaff/fdutils/

 ftp://metalab.unc.edu/pub/Linux/utils/disk-management/

## Reporting problems about the floppy driver锛堟姤鍛婅蒋椹遍┍鍔ㄧ殑闂锛?

濡傛灉浣犳湁鍏充簬杞┍椹卞姩鐨勯棶棰樻垨缂洪櫡鎶ュ憡锛岃鍙戦偖浠剁粰鎴戯細Alain.Knaff@poboxes.com銆?濡傛灉浣犲湪 Usenet 涓婂彂甯栵紝鏈€濂戒娇鐢?comp.os.linux.hardware銆傜敱浜庤繖浜涙柊闂荤粍
娴侀噺鐩稿綋澶э紝璇峰姟蹇呭湪涓婚琛屼腑鍖呭惈 "floppy"锛堟垨 "FLOPPY"锛夊瓧鏍枫€?濡傛灉鎶ュ憡鐨勯棶棰樺彂鐢熷湪鎸傝浇杞洏鏃讹紝璇峰姟蹇呭湪涓婚琛屼腑鍚屾椂鎻愬強鏂囦欢绯荤粺鐨勭被鍨嬨€?
鍦ㄥ彂閭欢鎴栧彂甯栨姤鍛婁换浣曠己闄蜂箣鍓嶏紝璇峰姟蹇呭厛闃呰 FAQ锛?
Alain

## Changelog锛堝彉鏇存棩蹇楋級


10-30-2004 :
		Cleanup, updating, add reference to module configuration.
		James Nelson <james4765@gmail.com>

6-3-2000 :
		Original Document
