## HugeTLB Controller


HugeTLB 鎺у埗鍣ㄥ彲浠ラ€氳繃鍏堟寕杞?cgroup 鏂囦欢绯荤粺鏉ュ垱寤恒€?
# mount -t cgroup -o hugetlb none /sys/fs/cgroup

缁忚繃涓婅堪姝ラ锛屽垵濮嬬殑鎴栫埗 HugeTLB 缁勫湪 /sys/fs/cgroup 澶勫彲瑙併€傚湪鍚姩锛坆ootup锛夋椂锛岃缁勫寘鍚郴缁熶腑鎵€鏈変换鍔°€?sys/fs/cgroup/tasks 鍒楀嚭浜嗚 cgroup 涓殑浠诲姟銆?
```

  # cd /sys/fs/cgroup
  # mkdir g1
  # echo $$ > g1/tasks

```
涓婅堪姝ラ鍒涘缓浜嗕竴涓柊缁?g1锛屽苟鎶婂綋鍓?shell 杩涚▼锛坆ash锛夌Щ鍏ュ叾涓€?
```

 hugetlb.<hugepagesize>.rsvd.limit_in_bytes            # set/show limit of "hugepagesize" hugetlb reservations
 hugetlb.<hugepagesize>.rsvd.max_usage_in_bytes        # show max "hugepagesize" hugetlb reservations and no-reserve faults
 hugetlb.<hugepagesize>.rsvd.usage_in_bytes            # show current reservations and no-reserve faults for "hugepagesize" hugetlb
 hugetlb.<hugepagesize>.rsvd.failcnt                   # show the number of allocation failure due to HugeTLB reservation limit
 hugetlb.<hugepagesize>.limit_in_bytes                 # set/show limit of "hugepagesize" hugetlb faults
 hugetlb.<hugepagesize>.max_usage_in_bytes             # show max "hugepagesize" hugetlb  usage recorded
 hugetlb.<hugepagesize>.usage_in_bytes                 # show current usage for "hugepagesize" hugetlb
 hugetlb.<hugepagesize>.failcnt                        # show the number of allocation failure due to HugeTLB usage limit
 hugetlb.<hugepagesize>.numa_stat                      # show the numa information of the hugetlb memory charged to this cgroup

```
瀵逛簬鏀寔涓夌澶ч〉澶у皬锛?4k銆?2M 鍜?1G锛夌殑绯荤粺锛屾帶鍒?```

  hugetlb.1GB.limit_in_bytes
  hugetlb.1GB.max_usage_in_bytes
  hugetlb.1GB.numa_stat
  hugetlb.1GB.usage_in_bytes
  hugetlb.1GB.failcnt
  hugetlb.1GB.rsvd.limit_in_bytes
  hugetlb.1GB.rsvd.max_usage_in_bytes
  hugetlb.1GB.rsvd.usage_in_bytes
  hugetlb.1GB.rsvd.failcnt
  hugetlb.64KB.limit_in_bytes
  hugetlb.64KB.max_usage_in_bytes
  hugetlb.64KB.numa_stat
  hugetlb.64KB.usage_in_bytes
  hugetlb.64KB.failcnt
  hugetlb.64KB.rsvd.limit_in_bytes
  hugetlb.64KB.rsvd.max_usage_in_bytes
  hugetlb.64KB.rsvd.usage_in_bytes
  hugetlb.64KB.rsvd.failcnt
  hugetlb.32MB.limit_in_bytes
  hugetlb.32MB.max_usage_in_bytes
  hugetlb.32MB.numa_stat
  hugetlb.32MB.usage_in_bytes
  hugetlb.32MB.failcnt
  hugetlb.32MB.rsvd.limit_in_bytes
  hugetlb.32MB.rsvd.max_usage_in_bytes
  hugetlb.32MB.rsvd.usage_in_bytes
  hugetlb.32MB.rsvd.failcnt



```
1. Page fault accounting

```

  hugetlb.<hugepagesize>.limit_in_bytes
  hugetlb.<hugepagesize>.max_usage_in_bytes
  hugetlb.<hugepagesize>.usage_in_bytes
  hugetlb.<hugepagesize>.failcnt

```
HugeTLB 鎺у埗鍣ㄥ厑璁哥敤鎴烽檺鍒舵瘡涓帶鍒剁粍鐨?HugeTLB 浣跨敤閲忥紙page fault锛夛紝骞跺湪缂洪〉鏃跺己鍒舵墽琛岄檺鍒躲€傜敱浜?HugeTLB 涓嶆敮鎸侀〉闈㈠洖鏀讹紙page reclaim锛夛紝鍦ㄧ己椤垫椂寮哄埗闄愬埗鎰忓懗鐫€锛屽鏋滃簲鐢ㄧ▼搴忚瘯鍥剧己椤佃皟鍏ヨ秴鍑哄叾闄愬埗鐨?HugeTLB 椤甸潰锛屽畠灏嗘敹鍒?SIGBUS 淇″彿銆傚洜姝ゅ簲鐢ㄧ▼搴忛渶瑕佷簨鍏堢‘鍒囩煡閬撹嚜宸变娇鐢ㄤ簡澶氬皯 HugeTLB 椤甸潰锛屽苟涓旂郴缁熺鐞嗗憳闇€瑕佺‘淇濇満鍣ㄤ笂鏈夎冻澶熺殑鍙敤椤甸潰渚涙墍鏈夌敤鎴蜂娇鐢紝浠ラ伩鍏嶈繘绋嬫敹鍒?SIGBUS銆?

2. Reservation accounting

```

  hugetlb.<hugepagesize>.rsvd.limit_in_bytes
  hugetlb.<hugepagesize>.rsvd.max_usage_in_bytes
  hugetlb.<hugepagesize>.rsvd.usage_in_bytes
  hugetlb.<hugepagesize>.rsvd.failcnt

```
HugeTLB 鎺у埗鍣ㄥ厑璁搁檺鍒舵瘡涓帶鍒剁粍鐨?HugeTLB 棰勭暀锛屽苟鍦ㄩ鐣欐椂浠ュ強涓轰笉瀛樺湪棰勭暀鐨?HugeTLB 鍐呭瓨缂洪〉鏃跺己鍒舵墽琛屾帶鍒跺櫒闄愬埗銆傜敱浜庨鐣欓檺鍒跺湪棰勭暀鏃讹紙mmap 鎴?shget 鏃讹級寮哄埗鎵ц锛屽鏋滃唴瀛樹簨鍏堝凡棰勭暀锛岄鐣欓檺鍒舵案杩滀笉浼氬鑷村簲鐢ㄧ▼搴忔敹鍒?SIGBUS 淇″彿銆傚浜?MAP_NORESERVE 鍒嗛厤锛岄鐣欓檺鍒剁殑琛屼负涓庣己椤甸檺鍒剁浉鍚岋紝鍦ㄧ己椤垫椂寮哄埗鎵ц鍐呭瓨浣跨敤锛屽苟鍦ㄨ秺杩囬檺鍒舵椂瀵艰嚧搴旂敤绋嬪簭鏀跺埌 SIGBUS銆?
棰勭暀闄愬埗浼樹簬涓婇潰鎻忚堪鐨勭己椤甸檺鍒讹紝鍥犱负棰勭暀闄愬埗鍦ㄩ鐣欐椂锛坢map 鎴?shget 鏃讹級寮哄埗鎵ц锛屽鏋滃唴瀛樹簨鍏堝凡棰勭暀锛屽氨姘歌繙涓嶄細瀵艰嚧搴旂敤绋嬪簭鏀跺埌 SIGBUS 淇″彿銆傝繖浣垮緱鏇村鏄撳洖閫€鍒版浛浠ｆ柟妗堬紝渚嬪闈?HugeTLB 鍐呭瓨銆傝€屽湪缂洪〉璁拌处鐨勬儏鍐典笅锛岀敱浜庣郴缁熺鐞嗗憳闇€瑕佺簿纭煡閬撶郴缁熶腑鎵€鏈変换鍔＄殑 HugeTLB 浣跨敤閲忓苟纭繚鍦ㄦ墍鏈夎姹傚墠鏈夎冻澶熼〉闈紝瑕侀伩鍏嶈繘绋嬫敹鍒?SIGBUS 闈炲父鍥伴毦銆傚湪杩囬噺鎵胯锛坥vercommitted锛夌殑绯荤粺涓婏紝鐢ㄧ己椤佃璐﹀疄闄呬笂涓嶅彲鑳介伩鍏嶄换鍔℃敹鍒?SIGBUS銆?

3. Caveats with shared memory

瀵逛簬鍏变韩鐨?HugeTLB 鍐呭瓨锛孒ugeTLB 棰勭暀鍜岀己椤甸兘璁″叆绗竴涓鑷磋鍐呭瓨琚鐣欐垨缂洪〉鐨勪换鍔★紝鑰岄殢鍚庡璇ュ凡棰勭暀鎴栧凡缂洪〉鍐呭瓨鐨勬墍鏈変娇鐢ㄩ兘涓嶈鍏ャ€?

鍏变韩鐨?HugeTLB 鍐呭瓨鍙湁鍦ㄨ瑙ｉ櫎棰勭暀鎴栭噴鏀炬椂鎵嶈В闄よ璐广€傝繖閫氬父鍙戠敓鍦?HugeTLB 鏂囦欢琚垹闄ゆ椂锛岃€屼笉鏄湪瀵艰嚧棰勭暀鎴栫己椤电殑浠诲姟閫€鍑烘椂銆?

4. Caveats with HugeTLB cgroup offline.

褰撲竴涓?HugeTLB cgroup 鍦ㄤ粛鏈夋煇浜涢鐣欐垨缂洪〉璁″叆瀹冪殑鎯呭喌涓嬩笅绾挎椂锛岃涓哄涓嬶細

- 缂洪〉璁¤垂琚鍏ョ埗 HugeTLB cgroup锛堥噸鏂板綊灞烇紝reparented锛夛紝
- 棰勭暀璁¤垂淇濈暀鍦ㄨ绂荤嚎鐨?HugeTLB cgroup 涓娿€?
杩欐剰鍛崇潃锛屽鏋滀竴涓?HugeTLB cgroup 鍦ㄤ笅绾挎椂浠嶆湁 HugeTLB 棰勭暀璁″叆锛岃 cgroup 浼氫綔涓哄兊灏革紙zombie锛変竴鐩村瓨鍦紝鐩村埌鎵€鏈?HugeTLB 棰勭暀閮借В闄よ璐广€侶ugeTLB 棰勭暀浠ヨ繖绉嶆柟寮忓伐浣滐紝鏄负浜嗕笌鍐呭瓨鎺у埗鍣ㄤ繚鎸佷竴鑷达紝鍚庤€呯殑 cgroup 涔熶細浣滀负鍍靛案涓€鐩村瓨鍦紝鐩村埌鎵€鏈夎璐瑰唴瀛橀兘瑙ｉ櫎璁¤垂銆傛澶栵紝涓庤拷韪?HugeTLB 缂洪〉鐩告瘮锛岃拷韪?HugeTLB 棰勭暀瑕佹洿澶嶆潅涓€浜涳紝鍥犳鍦ㄤ笅绾挎椂閲嶆柊褰掑睘棰勭暀涔熻鍥伴毦寰楀銆?