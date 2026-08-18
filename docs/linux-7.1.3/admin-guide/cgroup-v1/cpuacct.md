## CPU 璁拌处鎺у埗鍣?

CPU 璁拌处鎺у埗鍣ㄧ敤浜庨€氳繃 cgroups 瀵逛换鍔¤繘琛屽垎缁勶紝骞跺杩欎簺浠诲姟缁勭殑 CPU 浣跨敤閲忚繘琛岃璐︺€?
CPU 璁拌处鎺у埗鍣ㄦ敮鎸佸灞傜骇缁勩€備竴涓璐︾粍浼氱疮鍔犲叾鎵€鏈夊瓙缁勪互鍙婄洿鎺ヤ綅浜庡叾缁勪腑鐨勪换鍔＄殑 CPU 浣跨敤閲忋€?
```

  # mount -t cgroup -ocpuacct none /sys/fs/cgroup

```
缁忚繃涓婅堪姝ラ鍚庯紝鍒濆鎴栫埗璁拌处缁勫湪 /sys/fs/cgroup 澶勫彲瑙併€傚湪鍚姩锛坆ootup锛夋椂锛岃缁勫寘鍚郴缁熶腑鐨勬墍鏈変换鍔°€?sys/fs/cgroup/tasks 鍒楀嚭浜嗚 cgroup 涓殑浠诲姟銆?sys/fs/cgroup/cpuacct.usage 缁欏嚭璇ョ粍鑾峰緱鐨?CPU 鏃堕棿锛堜互绾崇涓哄崟浣嶏級锛岃繖鏈川涓婂氨鏄郴缁熶腑鎵€鏈変换鍔¤幏寰楃殑 CPU 鏃堕棿銆?
```

  # cd /sys/fs/cgroup
  # mkdir g1
  # echo $$ > g1/tasks

```
涓婅堪姝ラ鍒涘缓浜嗕竴涓柊缁?g1锛屽苟灏嗗綋鍓?shell 杩涚▼锛坆ash锛夌Щ鍏ュ叾涓€傝 bash 鍙婂叾瀛愯繘绋嬫秷鑰楃殑 CPU 鏃堕棿鍙粠 g1/cpuacct.usage 鑾峰彇锛屽苟涓斿悓鏍蜂細绱姞鍒?/sys/fs/cgroup/cpuacct.usage 涓€?
cpuacct.stat 鏂囦欢鍒楀嚭浜嗕竴浜涚粺璁′俊鎭紝灏?cgroup 鑾峰緱鐨?CPU 鏃堕棿杩涗竴姝ュ垝鍒嗕负鐢ㄦ埛鏃堕棿涓庣郴缁熸椂闂淬€傜洰鍓嶆敮鎸佷互涓嬬粺璁′俊鎭細

user锛歝group 鐨勪换鍔″湪鐢ㄦ埛妯″紡涓嬭姳璐圭殑鏃堕棿銆?system锛歝group 鐨勪换鍔″湪鍐呮牳妯″紡涓嬭姳璐圭殑鏃堕棿銆?
user 鍜?system 浠?USER_HZ 涓哄崟浣嶃€?
cpuacct 鎺у埗鍣ㄤ娇鐢?percpu_counter 鎺ュ彛鏉ユ敹闆嗙敤鎴锋椂闂村拰绯荤粺鏃堕棿銆傝繖鏈変袱涓壇浣滅敤锛?
- 鐞嗚涓婂彲鑳界湅鍒?user 鍜?system 鏃堕棿鐨勯敊璇€笺€傝繖鏄洜涓哄湪 32 浣嶇郴缁熶笂 percpu_counter_read() 瀵逛簬骞跺彂鍐欏叆骞朵笉瀹夊叏銆?- 鐢变簬 percpu_counter 鐨勬壒澶勭悊鐗规€э紝鍙兘浼氱湅鍒扮暐寰繃鏃剁殑 user 鍜?system 鏃堕棿鍊笺€?