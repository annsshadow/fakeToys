## 杩涚▼鏁版帶鍒跺櫒锛圥rocess Number Controller锛?

### 鎽樿


杩涚▼鏁版帶鍒跺櫒鐢ㄤ簬鍏佽 cgroup 灞傜骇鍦ㄨ揪鍒版煇涓檺鍒跺悗闃绘浠讳綍鏂颁换鍔¤ fork() 鎴?clone()銆?
鐢变簬鍦ㄤ笉瑙﹀強浠讳綍 kmemcg 闄愬埗鐨勬儏鍐典笅灏卞緢瀹规槗杈惧埌浠诲姟涓婇檺锛孭ID 鏄竴绉嶅熀鏈祫婧愩€傚洜姝わ紝蹇呴』閫氳繃鍏佽瀵?cgroup 涓换鍔℃暟閲忚繘琛岃祫婧愰檺鍒讹紝鍦?cgroup 灞傜骇鑼冨洿鍐呴闃?PID 鑰楀敖銆?
### 鐢ㄦ硶


瑕佷娇鐢?`pids` 鎺у埗鍣紝璁剧疆 pids.max 涓殑鏈€澶т换鍔℃暟锛堝嚭浜庢樉鑰屾槗瑙佺殑鍘熷洜锛岃繖鍦ㄦ牴 cgroup 涓笉鍙敤锛夈€俢group 涓綋鍓嶇殑杩涚▼鏁扮敱 pids.current 缁欏嚭銆?
缁勭粐鎿嶄綔涓嶄細琚?cgroup 绛栫暐闃诲锛屽洜姝ゅ彲鑳藉嚭鐜?pids.current > pids.max銆傝繖鍙兘鏄€氳繃灏嗛檺鍒惰缃负灏忎簬 pids.current锛屾垨鑰呭皢瓒冲澶氱殑杩涚▼闄勫姞鍒?cgroup 浣垮緱 pids.current > pids.max 鏉ュ疄鐜扮殑銆備絾鏄紝涓嶅彲鑳介€氳繃 fork() 鎴?clone() 杩濆弽 cgroup 绛栫暐銆傚鏋滃垱寤烘柊杩涚▼浼氬鑷磋繚鍙?cgroup 绛栫暐锛宖ork() 鍜?clone() 灏嗚繑鍥?-EAGAIN銆?
瑕佸皢鏌愪釜 cgroup 璁句负鏃犻檺鍒讹紝灏?pids.max 璁句负 鈥渕ax鈥濄€傝繖鏄墍鏈夋柊 cgroup 鐨勯粯璁ゅ€硷紙娉ㄦ剰锛歅ID 闄愬埗鏄垎灞傜殑锛屽洜姝ら伒寰眰绾т腑鏈€涓ユ牸鐨勯檺鍒讹級銆?
pids.current 杩借釜鎵€鏈夊瓙 cgroup 灞傜骇锛屽洜姝?parent/pids.current 鏄?parent/child/pids.current 鐨勮秴闆嗐€?
pids.events 鏂囦欢鍖呭惈浜嬩欢璁℃暟鍣細

  - max锛氬湪鑷韩鎴栫鍏堜腑鍥犺揪鍒伴檺鍒惰€屽鑷?fork 澶辫触鐨勬鏁般€?
### 绀轰緥


```

	# mkdir -p /sys/fs/cgroup/pids
	# mount -t cgroup -o pids none /sys/fs/cgroup/pids

```
```

	# mkdir -p /sys/fs/cgroup/pids/parent/child
	# echo 2 > /sys/fs/cgroup/pids/parent/pids.max
	# echo $$ > /sys/fs/cgroup/pids/parent/cgroup.procs
	# cat /sys/fs/cgroup/pids/parent/pids.current
	2
	#

```
搴旀敞鎰忥紝璇曞浘绐佺牬璁惧畾鐨勯檺鍒讹紙鏈緥涓负 2锛夊皢
```

	# cat /sys/fs/cgroup/pids/parent/pids.current
	2
	# ( /bin/echo "Here's some processes for you." | cat )
	sh: fork: Resource temporary unavailable
	#

```
鍗充娇鎴戜滑杩佺Щ鍒板瓙 cgroup锛堝畠娌℃湁璁惧畾闄愬埗锛夛紝鎴戜滑涔熸棤娉曠獊鐮村眰绾т腑鏈€涓ユ牸鐨勯檺鍒讹紙鏈緥涓紝
```

	# echo $$ > /sys/fs/cgroup/pids/parent/child/cgroup.procs
	# cat /sys/fs/cgroup/pids/parent/pids.current
	2
	# cat /sys/fs/cgroup/pids/parent/child/pids.current
	2
	# cat /sys/fs/cgroup/pids/parent/child/pids.max
	max
	# ( /bin/echo "Here's some processes for you." | cat )
	sh: fork: Resource temporary unavailable
	#

```
鎴戜滑鍙互璁剧疆涓€涓皬浜?pids.current 鐨勯檺鍒讹紝杩欏皢瀹屽叏闃绘浠讳綍鏂拌繘绋嬭 fork锛堟敞鎰?shell 鏈韩涔熺畻浣?```

	# echo 1 > /sys/fs/cgroup/pids/parent/pids.max
	# /bin/echo "We can't even spawn a single process now."
	sh: fork: Resource temporary unavailable
	# echo 0 > /sys/fs/cgroup/pids/parent/pids.max
	# /bin/echo "We can't even spawn a single process now."
	sh: fork: Resource temporary unavailable
	#

```
