## kAFS锛欰FS 鏂囦欢绯荤粺锛圓FS FILESYSTEM锛?

 - 姒傝堪锛圤verview锛夈€? - 鐢ㄦ硶锛圲sage锛夈€? - 鎸傝浇鐐癸紙Mountpoints锛夈€? - 鍔ㄦ€佹牴锛圖ynamic root锛夈€? - Proc 鏂囦欢绯荤粺锛圥roc filesystem锛夈€? - 鍗曞厓鏁版嵁搴擄紙The cell database锛夈€? - 瀹夊叏锛圫ecurity锛夈€? - @sys 鏇挎崲锛圱he @sys substitution锛夈€?

## 姒傝堪锛圤verview锛?

璇ユ枃浠剁郴缁熸彁渚涗簡涓€涓浉褰撶畝鍗曠殑銆佸畨鍏ㄧ殑 AFS 鏂囦欢绯荤粺椹卞姩銆傚畠浠嶅浜庡紑鍙戜腑锛屽皻鏈彁渚涘畬鏁寸殑鍔熻兘闆嗐€傚畠鎵€鏀寔鐨勫姛鑳藉寘鎷細

 (*) 瀹夊叏锛堢洰鍓嶄粎鏀寔 AFS kaserver 鍜?KerberosIV 绁ㄦ嵁锛夈€?
 (*) 鏂囦欢璇诲啓銆?
 (*) 鑷姩鎸傝浇锛圓utomounting锛夈€?
 (*) 鏈湴缂撳瓨锛堥€氳繃 fscache锛夈€?
瀹冨皻涓嶆敮鎸佷互涓?AFS 鍔熻兘锛?
 (*) pioctl() 绯荤粺璋冪敤銆?

## 缂栬瘧锛圕ompilation锛?

搴旈€氳繃鎵撳紑浠ヤ笅鍐呮牳閰嶇疆椤规潵鍚敤璇ユ枃浠剁郴缁燂細
```
	CONFIG_AF_RXRPC		- RxRPC 鍗忚浼犺緭
	CONFIG_RXKAD		- RxRPC Kerberos 瀹夊叏澶勭悊绋嬪簭
	CONFIG_AFS_FS		- AFS 鏂囦欢绯荤粺
```
```
	CONFIG_AF_RXRPC_DEBUG	- 鍏佽鍚敤 AF_RXRPC 璋冭瘯
	CONFIG_AFS_DEBUG	- 鍏佽鍚敤 AFS 璋冭瘯
```
瀹冧滑鍏佽閫氳繃鎿嶄綔浠ヤ笅鍐呭鍔ㄦ€佸紑鍚皟璇曟秷鎭細
```
	/sys/module/af_rxrpc/parameters/debug
	/sys/module/kafs/parameters/debug
```

## 鐢ㄦ硶锛圲sage锛?

鍦ㄦ彃鍏ラ┍鍔ㄦā鍧楁椂锛屽繀椤婚殢鍚屾寚瀹氭牴鍗曞厓锛坮oot cell锛夛紝骞堕檮甯︿竴涓?```
	modprobe rxrpc
	modprobe kafs rootcell=cambridge.redhat.com:172.16.18.73:172.16.18.91
```
绗竴涓ā鍧楁槸 AF_RXRPC 缃戠粶鍗忚椹卞姩銆傚畠鎻愪緵 RxRPC 杩滅▼鎿嶄綔鍗忚锛屼篃鍙互浠庣敤鎴风┖闂磋闂€傚弬瑙侊細

	Documentation/networking/rxrpc.rst

绗簩涓ā鍧楁槸 kerberos RxRPC 瀹夊叏椹卞姩锛岀涓変釜妯″潡鏄?AFS 鏂囦欢绯荤粺瀹為檯鐨勬枃浠剁郴缁熼┍鍔ㄣ€?
妯″潡鍔犺浇鍚庯紝鍙互閫氳繃濡備笅鏂瑰紡娣诲姞鏇村妯″潡锛?```
	echo add grand.central.org 18.9.48.14:128.2.203.61:130.237.48.87 >/proc/fs/afs/cells
```
鍏朵腑 "add" 鍛戒护鐨勫弬鏁版槸鍗曞厓鐨勫悕绉帮紝浠ュ強璇ュ崟鍏冨唴涓€缁勫嵎浣嶇疆锛坴olume location锛夋湇鍔″櫒锛屽悗鑰呬互鍐掑彿鍒嗛殧銆?
```
	mount -t afs "%cambridge.redhat.com:root.afs." /afs
	mount -t afs "#cambridge.redhat.com:root.cell." /afs/cambridge
	mount -t afs "#root.afs." /afs
	mount -t afs "#root.cell." /afs/cambridge
```
鍏朵腑棣栧瓧绗︽槸浜曞彿锛?锛夎繕鏄櫨鍒嗗彿锛?锛夛紝鍙栧喅浜庝綘绌剁珶鏄兂瑕佷竴涓?R/W 鍗凤紙鐧惧垎鍙凤級锛岃繕鏄洿鍊惧悜 R/O 鍗蜂絾鎰挎剰鏀圭敤 R/W 鍗凤紙浜曞彿锛夈€?
鍗风殑鍚嶇О鍙互鍔犱笂 ".backup" 鎴?".readonly" 鍚庣紑锛屼互鎸囧畾浠呰繛鎺ヨ繖浜涚被鍨嬬殑鍗枫€?
鍗曞厓鐨勫悕绉版槸鍙€夌殑锛屽鏋滃湪鎸傝浇鏃舵湭缁欏嚭锛屽垯浼氬湪 modprobe 鏃舵寚瀹氱殑鍗曞厓涓煡鎵捐鍛藉悕鍗枫€?
鍙互閫氳繃 /proc 娣诲姞棰濆鐨勫崟鍏冿紙瑙佸悗鏂囷級銆?

## 鎸傝浇鐐癸紙Mountpoints锛?

AFS 鏈夋寕杞界偣锛坢ountpoint锛夌殑姒傚康銆傜敤 AFS 鐨勬湳璇锛岃繖浜涙槸鐗规畩鏍煎紡鐨勭鍙烽摼鎺ワ紙涓庝紶缁?mount 鐨勨€滆澶囧悕鈥濆舰寮忕浉鍚岋級銆俴AFS 灏嗚繖浜涗互鍏锋湁 follow-link 鑳藉姏锛堝嵆绗﹀彿閾炬帴璇箟锛夌殑鐩綍褰㈠紡鍛堢幇缁欑敤鎴枫€傚鏋滄湁浜鸿瘯鍥捐闂畠浠紝瀹冧滑浼氳嚜鍔ㄥ鑷寸洰鏍囧嵎琚寕杞斤紙濡傛灉鍙兘锛夊埌璇ヤ綅缃€?
鑷姩鎸傝浇鐨勬枃浠剁郴缁熷皢鍦ㄦ渶鍚庝竴娆′娇鐢ㄥ悗澶х害浜屽崄鍒嗛挓琚嚜鍔ㄥ嵏杞姐€傛垨鑰咃紝涔熷彲浠ラ€氳繃 umount() 绯荤粺璋冪敤鐩存帴鍗歌浇銆?
鎵嬪姩鍗歌浇涓€涓?AFS 鍗蜂細鍏堝墧闄ゅ叾涓婁换浣曠┖闂茬殑瀛愭寕杞界偣銆傚鏋滃叏閮ㄨ鍓旈櫎锛屽垯鎵€璇锋眰鐨勫嵎涔熶細琚嵏杞斤紝鍚﹀垯浼氳繑鍥為敊璇?EBUSY銆?
绠＄悊鍛樺彲浠ュ埄鐢ㄨ繖涓€鐐瑰皾璇曞嵏杞芥暣涓?AFS 鏍戯細
```
	umount /afs
```

## 鍔ㄦ€佹牴锛圖ynamic Root锛?

鍙互閫氳繃涓€涓寕杞介€夐」鍒涘缓鏃犳湇鍔″櫒鐨勬寕杞斤紝瀹冧粎鍙敤
```
	mount -t afs none /afs -o dyn
```
杩欎細鍒涘缓涓€涓寕杞斤紝鍏舵牴鐩綍鍙槸涓€涓┖鐩綍銆傝瘯鍥惧湪璇ョ洰褰曚腑鏌ユ壘涓€涓悕绉板皢瀵艰嚧鍒涘缓涓€涓寕杞界偣锛?```
	ls /afs/grand.central.org/
```

## Proc 鏂囦欢绯荤粺锛圥roc Filesystem锛?

AFS 妯″潡鍒涘缓 "/proc/fs/afs/" 鐩綍骞跺～鍏呭畠锛?
  (*) 涓€涓?"cells" 鏂囦欢锛屽垪鍑?afs 妯″潡褰撳墠宸茬煡鐨勫崟鍏冿細
```
	[root@andromeda ~]# cat /proc/fs/afs/cells
	USE NAME
	  3 cambridge.redhat.com
```
  (*) 姣忎釜鍗曞厓涓€涓洰褰曪紝鍏朵腑鍖呭惈鍒楀嚭璇ュ崟鍏冨唴宸茬煡鍗蜂綅缃湇鍔″櫒銆佸嵎鍜屾椿璺冩湇鍔″櫒鐨勬枃浠讹細
```
	[root@andromeda ~]# cat /proc/fs/afs/cambridge.redhat.com/servers
	USE ADDR            STATE
	  4 172.16.18.91        0
	[root@andromeda ~]# cat /proc/fs/afs/cambridge.redhat.com/vlservers
	ADDRESS
	172.16.18.91
	[root@andromeda ~]# cat /proc/fs/afs/cambridge.redhat.com/volumes
	USE STT VLID[0]  VLID[1]  VLID[2]  NAME
	  1 Val 20000000 20000001 20000002 root.afs
```

## 鍗曞厓鏁版嵁搴擄紙The Cell Database锛?

鏂囦欢绯荤粺缁存姢涓€涓唴閮ㄦ暟鎹簱锛岃褰曞畠鐭ラ亾鐨勬墍鏈夊崟鍏冿紝浠ュ強杩欎簺鍗曞厓鐨勫嵎浣嶇疆鏈嶅姟鍣ㄧ殑 IP 鍦板潃銆傜郴缁熸墍灞炵殑鍗曞厓鍦?modprobe 鏃堕€氳繃 "rootcell=" 鍙傛暟鍔犲叆鏁版嵁搴擄紱濡傛灉缂栬瘧杩涘唴鏍革紝鍒欎娇鐢ㄥ唴鏍稿懡浠よ涓婄殑 "kafs.rootcell=" 鍙傛暟銆?
```
	echo add CELLNAME VLADDR[:VLADDR][:VLADDR]... >/proc/fs/afs/cells
	echo add grand.central.org 18.9.48.14:128.2.203.61:130.237.48.87 >/proc/fs/afs/cells
```
鐩墠娌℃湁鍏朵粬鍗曞厓鏁版嵁搴撴搷浣滃彲鐢ㄣ€?

## 瀹夊叏锛圫ecurity锛?

瀹夊叏鎿嶄綔閫氳繃鐢?klog 绋嬪簭鑾峰彇涓€涓瘑閽ユ潵鍙戣捣銆備竴涓潪甯稿師濮嬬殑 klog 绋嬪簭浣嶄簬锛?
	https://people.redhat.com/~dhowells/rxrpc/klog.c
```
	make klog LDLIBS="-lcrypto -lcrypt -lkrb4 -lkeyutils"
```
```
	./klog
```
鍋囪鎴愬姛锛岃繖浼氭坊鍔犱竴涓被鍨嬩负 RxRPC銆佷互鏈嶅姟鍜屽崟鍏冨懡鍚嶇殑瀵嗛挜锛屼緥濡傦細"afs@<cellname>"銆傚彲浠ョ敤 keyctl 绋嬪簭鏌ョ湅瀹冿細
```
	[root@andromeda ~]# keyctl show
	Session Keyring
	       -3 --alswrv      0     0  keyring: _ses.3268
		2 --alswrv      0     0   \_ keyring: _uid.0
	111416553 --als--v      0     0   \_ rxrpc: afs@CAMBRIDGE.REDHAT.COM
```
鐩墠锛岀敤鎴峰悕銆佸煙锛坮ealm锛夈€佸瘑鐮佸拰寤鸿鐨勭エ鎹敓瀛樻湡閮借缂栬瘧杩涚▼搴忎腑銆?
鍦ㄤ娇鐢?AFS 鍔熻兘涔嬪墠鑾峰彇瀵嗛挜涓嶆槸蹇呴渶鐨勶紝浣嗗鏋滀笉鑾峰彇锛屽垯鎵€鏈夋搷浣滈兘灏嗗彈 ACL 鐨勫尶鍚嶇敤鎴烽儴鍒嗙害鏉熴€?
濡傛灉鑾峰彇浜嗗瘑閽ワ紝鍒欐嫢鏈夎瀵嗛挜鑰呭彂鍑虹殑鎵€鏈?AFS 鎿嶄綔锛堝寘鎷寕杞藉拰鑷姩鎸傝浇锛夐兘灏嗕娇鐢ㄨ瀵嗛挜杩涜瀹夊叏淇濇姢銆?
濡傛灉涓€涓枃浠剁敤鏌愪釜鐗瑰畾瀵嗛挜鎵撳紑锛岀劧鍚庤鏂囦欢鎻忚堪绗﹁浼犻€掔粰涓€涓病鏈夎瀵嗛挜鐨勮繘绋嬶紙鍙兘閫氳繃 AF_UNIX 濂楁帴瀛楋級锛岄偅涔堣鏂囦欢涓婄殑鎿嶄綔灏嗕娇鐢ㄦ墦寮€璇ユ枃浠舵椂鎵€鐢ㄧ殑瀵嗛挜杩涜銆?

## @sys 鏇挎崲锛圱he @sys Substitution锛?

褰撳墠缃戠粶鍛藉悕绌洪棿鐨勮嚦澶?16 涓?@sys 鏇挎崲鍒楄〃鍙互
```
	[root@andromeda ~]# echo foo amd64_linux_26 >/proc/fs/afs/sysname
```
```
	[root@andromeda ~]# echo >/proc/fs/afs/sysname
```
```
	[root@andromeda ~]# cat /proc/fs/afs/sysname
	foo
	amd64_linux_26
```
杩涜 @sys 鏇挎崲鏃讹紝浼氭寜缁欏畾椤哄簭灏濊瘯鍒楄〃涓殑姣忎釜鍏冪礌銆?
榛樿鎯呭喌涓嬶紝璇ュ垪琛ㄥ皢鍖呭惈涓€涓鍚?"<arch>_linux_26" 妯″紡鐨勯」鐩紝鍏朵腑 amd64 鏄?x86_64 鐨勫悕绉般€?