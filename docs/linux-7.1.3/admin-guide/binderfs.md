## Android binderfs 鏂囦欢绯荤粺


Android binderfs 鏄?Android binder IPC 鏈哄埗鎵€鐢ㄧ殑鏂囦欢绯荤粺銆傚畠鍏佽鍦ㄨ繍琛屾椂
鍔ㄦ€佹坊鍔犲拰绉婚櫎 binder 璁惧銆備綅浜庢柊鐨?binderfs 瀹炰緥涓殑 binder 璁惧鐙珛浜?鍏朵粬 binderfs 瀹炰緥涓殑 binder 璁惧銆傛寕杞戒竴涓柊鐨?binderfs 瀹炰緥鍙互鑾峰彇涓€缁?绉佹湁鐨?binder 璁惧銆?
### 鎸傝浇 binderfs


```
mkdir /dev/binderfs
mount -t binder binder /dev/binderfs
```
姝ゆ椂灏嗗湪 `/dev/binderfs` 澶勫嚭鐜颁竴涓柊鐨?binderfs 瀹炰緥銆傚湪鍏ㄦ柊鐨?binderfs
瀹炰緥涓笉瀛樺湪浠讳綍 binder 璁惧銆傚彧浼氭湁涓€涓?`binder-control` 璁惧锛屼綔涓?binderfs 鐨勮姹傚鐞嗙▼搴忋€傚湪鍏朵粬浣嶇疆鎸傝浇鍙︿竴涓?binderfs 瀹炰緥锛屽皢鍒涘缓涓€涓?鐙珛浜庢墍鏈夊叾浠?binderfs 鎸傝浇鐨勬柊瀹炰緥銆傝繖涓?`devpts` 鍜?`tmpfs`
绛夎涓虹浉鍚屻€侫ndroid binderfs 鏂囦欢绯荤粺鍙互鎸傝浇鍦ㄧ敤鎴峰懡鍚嶇┖闂翠腑銆?
### 閫夐」

max
  binderfs 瀹炰緥鎸傝浇鏃跺彲瀵瑰彲鍒嗛厤鐨?binder 璁惧鏁伴噺璁剧疆闄愬埗銆俙max=<count>`
  鎸傝浇閫夐」鍏呭綋姣忓疄渚嬮檺鍒躲€傚鏋滆缃簡 `max=<count>`锛屽垯鍦ㄦ binderfs
  瀹炰緥涓彧鑳藉垎閰?`<count>` 涓?binder 璁惧銆?
stats
  浣跨敤 `stats=global` 鍙惎鐢ㄥ叏灞€ binder 缁熻淇℃伅銆俙stats=global` 浠呴€傜敤浜?  鎸傝浇鍦ㄥ垵濮嬬敤鎴峰懡鍚嶇┖闂翠腑鐨?binderfs 瀹炰緥銆傚皾璇曚娇鐢ㄨ閫夐」鎸傝浇浣嶄簬鍏朵粬
  鐢ㄦ埛鍛藉悕绌洪棿涓殑 binderfs 瀹炰緥灏嗚繑鍥炴潈闄愰敊璇€?
### 鍒嗛厤 binder 璁惧


瑕佸湪涓€涓?binderfs 瀹炰緥涓垎閰嶆柊鐨?binder 璁惧锛岄渶瑕侀€氳繃 `binder-control`
璁惧鑺傜偣鍙戦€佽姹傘€傝姹備互 `ioctl() <ioctl_>`_ 鐨勫舰寮忓彂閫併€?
绋嬪簭闇€瑕佸仛鐨勬槸鎵撳紑 `binder-control` 璁惧鑺傜偣锛屽苟鍚戝唴鏍稿彂閫佷竴涓?`BINDER_CTL_ADD` 璇锋眰銆俠inderfs 鐨勭敤鎴烽渶瑕佸憡璇夊唴鏍告柊 binder 璁惧搴斿彇
浠€涔堝悕绉般€傞粯璁ゆ儏鍐典笅锛屽悕绉版渶澶氬彧鑳藉寘鍚?`BINDERFS_MAX_NAME` 涓瓧绗?锛堝惈缁撳熬鐨勯浂瀛楄妭锛夈€?
涓€鏃﹂€氳繃 `ioctl() <ioctl_>`_ 灏嗗甫鏈夊悕绉扮殑 ``struct
binder_device`` 浼犻€掔粰鍐呮牳鍙戣捣璇锋眰锛屽唴鏍稿氨浼氬垎閰嶄竴涓柊鐨?binder 璁惧锛?骞跺湪缁撴瀯浣撲腑杩斿洖鏂拌澶囩殑涓汇€佹璁惧鍙凤紙杩欐槸蹇呴渶鐨勶紝鍥犱负 binderfs 浼氬姩鎬?鍒嗛厤涓昏澶囧彿锛夈€俙ioctl() <ioctl_>`_ 杩斿洖鍚庯紝鍦?/dev/binderfs 涓?灏嗗嚭鐜颁竴涓互鎵€閫夊悕绉板懡鍚嶇殑鏂?binder 璁惧銆?
### 鍒犻櫎 binder 璁惧


binderfs 鐨?binder 璁惧鍙€氳繃 `unlink() <unlink_>`_ 鍒犻櫎銆傝繖鎰忓懗鐫€鍙互浣跨敤
`rm() <rm_>`_ 宸ュ叿鍒犻櫎瀹冧滑銆傛敞鎰?`binder-control` 璁惧鏃犳硶琚垹闄わ紝鍥犱负
閭ｆ牱浼氫娇 binderfs 瀹炰緥涓嶅彲鐢ㄣ€俙binder-control` 璁惧浼氬湪 binderfs 瀹炰緥
琚嵏杞戒笖瀵瑰叾鐨勬墍鏈夊紩鐢ㄩ兘琚噴鏀炬椂琚垹闄ゃ€?
### binder 鐗规€?

鍋囪宸插湪 `/dev/binderfs` 鎸傝浇浜嗕竴涓?binderfs 瀹炰緥锛宐inder 椹卞姩鎵€鏀寔鐨?鐗规€у彲浣嶄簬 `/dev/binderfs/features/` 涓嬨€傚彲浠ラ€氳繃娴嬭瘯鍚勪釜鏂囦欢鐨勫瓨鍦?鏉ュ垽鏂┍鍔ㄦ槸鍚︽敮鎸佹煇涓壒瀹氱壒鎬с€?
```
cat /dev/binderfs/features/oneway_spam_detection
1
```
