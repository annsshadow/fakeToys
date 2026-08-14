## NFS ID 鏄犲皠鍣?

Id 鏄犲皠鍣ㄨ NFS 鐢ㄤ簬灏嗙敤鎴峰拰缁?id 杞崲涓哄悕绉帮紝骞跺皢鐢ㄦ埛鍜岀粍鍚嶇О杞崲涓?id銆?璇ヨ浆鎹㈢殑涓€閮ㄥ垎娑夊強鍚戠敤鎴锋€佸彂璧?upcall 浠ヨ姹備俊鎭€侼FS 鍙€氳繃涓ょ鏂瑰紡鑾峰彇
杩欎簺淇℃伅锛氳皟鐢?/sbin/request-key锛屾垨璋冪敤 rpc.idmap 瀹堟姢杩涚▼銆?
NFS 浼氬厛灏濊瘯璋冪敤 /sbin/request-key銆傝嫢鎴愬姛锛岀粨鏋滃皢浣跨敤閫氱敤鐨?request-key
缂撳瓨杩涜缂撳瓨銆傚彧鏈夊綋 /etc/request-key.conf 鏈负 id_resolver 瀵嗛挜绫诲瀷閰嶇疆鏃?璇ヨ皟鐢ㄦ墠浼氬け璐ワ紝鑻ユ兂浣跨敤 request-key 鏂规硶锛岃鍙傝涓嬫枃鈥滈厤缃€濅竴鑺傘€?
鑻ュ /sbin/request-key 鐨勮皟鐢ㄥけ璐ワ紙鍗?/etc/request-key.conf 鏈娇鐢?id_resolver 瀵嗛挜绫诲瀷閰嶇疆锛夛紝鍒?id 鏄犲皠鍣ㄥ皢鍚戦仐鐣欑殑 rpc.idmap 瀹堟姢杩涚▼璇锋眰
id 鏄犲皠銆傝缁撴灉灏嗗瓨鍌ㄥ湪鑷畾涔夌殑 NFS idmap 缂撳瓨涓€?
## 閰嶇疆


闇€瑕佷慨鏀规枃浠?/etc/request-key.conf锛屼互渚?/sbin/request-key 鑳藉寮曞璇?upcall銆傚簲娣诲姞浠ヤ笅琛岋細

`#OP	TYPE	DESCRIPTION	CALLOUT INFO	PROGRAM ARG1 ARG2 ARG3 ...`
`#======	=======	===============	===============	===============================`
`create	id_resolver	**	**		/usr/sbin/nfs.idmap %k %d 600`


杩欏皢鎶婃墍鏈?id_resolver 璇锋眰瀵煎悜绋嬪簭 /usr/sbin/nfs.idmap銆傛渶鍚庝竴涓弬鏁?600
瀹氫箟浜嗗瘑閽ュ皢鍦ㄦ湭鏉ュ灏戠鍚庤繃鏈熴€傝鍙傛暟瀵?/usr/sbin/nfs.idmap 鏄彲閫夌殑銆?鏈寚瀹氳秴鏃舵椂锛宯fs.idmap 榛樿浣跨敤 600 绉掋€?
```
  uid:  鏌ユ壘缁欏畾鐢ㄦ埛鐨?UID
  gid:  鏌ユ壘缁欏畾缁勭殑 GID
 user:  鏌ユ壘缁欏畾 UID 鐨勭敤鎴峰悕
group:  鏌ユ壘缁欏畾 GID 鐨勭粍鍚?
```
浣犲彲浠ュ崟鐙鐞嗗叾涓换鎰忎竴绉嶏紝鑰屼笉蹇呬娇鐢ㄩ€氱敤鐨?upcall 绋嬪簭銆傝嫢鎯充娇鐢ㄨ嚜宸辩殑绋嬪簭
杩涜 uid 鏌ユ壘锛屽彲浠ョ紪杈?request-key.conf锛屼娇鍏剁被浼煎涓嬶細

`#OP	TYPE	DESCRIPTION	CALLOUT INFO	PROGRAM ARG1 ARG2 ARG3 ...`
`#======	=======	===============	===============	===============================`
`create	id_resolver	uid:**	**		/some/other/program %k %d 600`
`create	id_resolver	**	**		/usr/sbin/nfs.idmap %k %d 600`


娉ㄦ剰鏂拌琚坊鍔犲湪閫氱敤绋嬪簭鎵€鍦ㄨ涔嬩笂銆俽equest-key 浼氭壘鍒扮涓€涓尮閰嶇殑琛屽強鐩稿簲
鐨勭▼搴忋€傚湪姝や緥涓紝/some/other/program 灏嗗鐞嗘墍鏈?uid 鏌ユ壘锛岃€?/usr/sbin/nfs.idmap 灏嗗鐞?gid銆乽ser 鍜?group 鏌ユ壘銆?
鏈夊叧 request-key 鍑芥暟鐨勬洿澶氫俊鎭紝璇峰弬瑙?Documentation/security/keys/request-key.rst銆?
## nfs.idmap


nfs.idmap 璁捐涓虹敱 request-key 璋冪敤锛屼笉搴斺€滄墜鍔ㄢ€濊繍琛屻€傝绋嬪簭鎺ュ彈涓や釜鍙傛暟锛?涓€涓簭鍒楀寲鐨勫瘑閽ュ拰涓€涓瘑閽ユ弿杩般€傚簭鍒楀寲瀵嗛挜棣栧厛琚浆鎹负 key_serial_t锛岀劧鍚?浣滀负鍙傛暟浼犻€掔粰 keyctl_instantiate锛堜簩鑰呴兘鏄?keyutils.h 鐨勪竴閮ㄥ垎锛夈€?
瀹為檯鐨勬煡鎵剧敱 nfsidmap.h 涓殑鍑芥暟鎵ц銆俷fs.idmap 閫氳繃鏌ョ湅鎻忚堪瀛楃涓茬殑绗竴閮ㄥ垎
鏉ョ‘瀹氳璋冪敤鐨勬纭嚱鏁般€備緥濡傦紝uid 鏌ユ壘鎻忚堪灏嗗舰濡?鈥渦id:user@domain鈥濄€?
鑻ュ瘑閽ヨ瀹炰緥鍖栵紝nfs.idmap 杩斿洖 0锛屽惁鍒欒繑鍥為潪 0銆?