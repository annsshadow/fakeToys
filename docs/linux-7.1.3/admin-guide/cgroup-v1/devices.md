## 璁惧鐧藉悕鍗曟帶鍒跺櫒


## 1. 鎻忚堪

瀹炵幇涓€涓?cgroup锛岀敤浜庤窡韪苟寮哄埗瀵硅澶囨枃浠舵柦鍔?open 鍜?mknod 闄愬埗銆備竴涓澶?cgroup
灏嗚澶囪闂櫧鍚嶅崟涓庢瘡涓?cgroup 鍏宠仈銆傜櫧鍚嶅崟椤规湁 4 涓瓧娈点€?type' 涓?a锛坅ll锛屽叏閮級銆?c锛坈har锛屽瓧绗﹁澶囷級鎴?b锛坆lock锛屽潡璁惧锛夈€?all' 琛ㄧず瀹冮€傜敤浜庢墍鏈夌被鍨嬩互鍙婃墍鏈変富璁惧鍙?鍜屾璁惧鍙枫€備富璁惧鍙峰拰娆¤澶囧彿鍙互鏄暣鏁帮紝涔熷彲浠ユ槸琛ㄧず鍏ㄩ儴鐨?*銆傝闂潈闄愶紙Access锛?鐢?r锛坮ead锛岃锛夈€亀锛坵rite锛屽啓锛夊拰 m锛坢knod锛夌粍鍚堣€屾垚銆?
鏍硅澶?cgroup 璧峰鎷ユ湁瀵?'all' 鐨?rwm 鏉冮檺銆傚瓙璁惧 cgroup 浼氳幏寰楀叾鐖剁骇鐨勫壇鏈€傜劧鍚?绠＄悊鍛樺彲浠ヤ粠鐧藉悕鍗曚腑绉婚櫎璁惧锛屾垨娣诲姞鏂扮殑椤广€傚瓙 cgroup 姘歌繙涓嶈兘鑾峰緱琚叾鐖剁骇鎷掔粷鐨?璁惧璁块棶鏉冮檺銆?
## 2. 鐢ㄦ埛鐣岄潰

浣跨敤 devices.allow 娣诲姞涓€椤癸紝浣跨敤
```

	echo 'c 1:3 mr' > /sys/fs/cgroup/1/devices.allow

```
鍏佽 cgroup 1 璇诲彇骞跺閫氬父琚О涓轰互涓嬪悕绉扮殑璁惧鎵ц mknod
```

	echo a > /sys/fs/cgroup/1/devices.deny

```
```

	echo a > /sys/fs/cgroup/1/devices.allow

```
浼氬悜鐧藉悕鍗曟坊鍔?'a **:** rwm' 椤广€?
## 3. 瀹夊叏鎬?
浠讳綍浠诲姟閮藉彲浠ュ湪 cgroup 涔嬮棿绉诲姩鑷韩銆傝繖鏄剧劧涓嶅锛屼絾鎴戜滑鍙互鍦ㄤ汉浠Н绱竴浜涗娇鐢?缁忛獙鍚庯紝鍐冲畾鏈€浣崇殑銆佽冻浠ュ厖鍒嗛檺鍒剁Щ鍔ㄧ殑鏂瑰紡銆傛垜浠篃璁稿彧鎯宠姹?CAP_SYS_ADMIN锛?瀹冭嚦灏戞槸涓?CAP_MKNOD 涓嶅悓鐨勪竴涓綅銆傛垜浠彲鑳芥兂鎷掔粷绉诲姩鍒伴潪褰撳墠 cgroup 鍚庝唬鐨?cgroup銆傛垨鑰呮垜浠彲鑳芥兂浣跨敤 CAP_MAC_ADMIN锛屽洜涓烘垜浠‘瀹炴槸鍦ㄨ瘯鍥鹃攣瀹?root銆?
淇敼鐧藉悕鍗曟垨灏嗗彟涓€涓换鍔＄Щ鍔ㄥ埌鏂?cgroup 闇€瑕?CAP_SYS_ADMIN銆傦紙鍚屾牱锛屾垜浠彲鑳?浼氭兂鏀瑰彉杩欎竴鐐癸級銆?
涓€涓?cgroup 鑾峰緱鐨勬潈闄愪笉鑳借秴杩囧叾 cgroup 鐖剁骇鎷ユ湁鐨勬潈闄愩€?
## 4. 灞傜骇缁撴瀯

璁惧 cgroup 閫氳繃纭繚涓€涓?cgroup 姘歌繙涓嶅叿鏈夋瘮鍏剁埗绾ф洿澶氱殑璁块棶鏉冮檺鏉ョ淮鎶ゅ眰绾х粨鏋勩€?姣忔鍚戞煇涓?cgroup 鐨?devices.deny 鏂囦欢鍐欏叆涓€椤规椂锛屽叾鎵€鏈夊瓙绾ч兘浼氫粠鐧藉悕鍗曚腑绉婚櫎璇ラ」锛?骞朵笖鎵€鏈夋湰鍦拌缃殑鐧藉悕鍗曢」閮戒細琚噸鏂拌瘎浼般€傚鏋滄煇涓湰鍦拌缃殑鐧藉悕鍗曢」浼氭彁渚涙瘮璇?cgroup 鐖剁骇鏇村鐨勮闂潈闄愶紝瀹冧細琚粠鐧藉悕鍗曚腑绉婚櫎銆?
```

      A
     / \
        B

    group        behavior	exceptions
    A            allow		"b 8:* rwm", "c 116:1 rw"
    B            deny		"c 1:3 rwm", "c 116:2 rwm", "b 3:* rwm"

```
```

	# echo "c 116:* r" > A/devices.deny

```
瀹冧細鍚戜笅浼犳挱锛屽湪閲嶆柊楠岃瘉 B 鐨勯」涔嬪悗锛岀櫧鍚嶅崟椤瑰彉涓?```

    group        whitelist entries                        denied devices
    A            all                                      "b 8:* rwm", "c 116:* rw"
    B            "c 1:3 rwm", "b 3:* rwm"                 all the rest

```
濡傛灉鐖剁骇鐨勪緥澶栧彂鐢熷彉鍖栵紝涓旀湰鍦颁緥澶栦笉鍐嶈鍏佽锛屽畠浠皢琚垹闄ゃ€?
```

      A
     / \
        B

    group        whitelist entries                        denied devices
    A            "c 1:3 rwm", "c 1:5 r"                   all the rest
    B            "c 1:3 rwm", "c 1:5 r"                   all the rest

```
```

	# echo "c *:3 rwm" >A/devices.allow

```
```

    group        whitelist entries                        denied devices
    A            "c *:3 rwm", "c 1:5 r"                   all the rest
    B            "c 1:3 rwm", "c 1:5 r"                   all the rest

```
```

	# echo "c 2:3 rwm" >B/devices.allow
	# echo "c 50:3 r" >B/devices.allow

```
```

	# echo "c *:3 rwm" >B/devices.allow

```
涓€鏃﹁澶?cgroup 鎷ユ湁瀛愮骇锛屽氨涓嶈兘鍐嶉€氳繃鍚?devices.allow 鎴?devices.deny 鍐欏叆 'a'
鏉ュ厑璁告垨鎷掔粷鍏ㄩ儴銆?
### 4.1 灞傜骇缁撴瀯锛堝唴閮ㄥ疄鐜帮級

璁惧 cgroup 鍦ㄥ唴閮ㄤ娇鐢ㄤ竴涓涓猴紙ALLOW銆丏ENY锛夊拰涓€涓緥澶栧垪琛ㄦ潵瀹炵幇銆傚唴閮ㄧ姸鎬佷娇鐢?鐩稿悓鐨勭敤鎴锋帴鍙ｆ潵鎺у埗锛屼互淇濇寔涓庝箣鍓嶄粎鐧藉悕鍗曞疄鐜扮殑鍏煎鎬с€備細鍑忓皯璁惧璁块棶鏉冮檺鐨勪緥澶栫殑
绉婚櫎鎴栨坊鍔狅紝浼氭部灞傜骇缁撴瀯鍚戜笅浼犳挱銆傚浜庢瘡涓€涓浼犳挱鐨勪緥澶栵紝鏈夋晥瑙勫垯浼氬熀浜庡綋鍓嶇埗绾х殑
璁块棶瑙勫垯琚噸鏂拌瘎浼般€?