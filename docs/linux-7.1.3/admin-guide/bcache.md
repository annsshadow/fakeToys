## A block layer cache (bcache锛屽潡灞傜紦瀛?


鍋囪鎮ㄦ湁涓€涓ぇ鑰屾參鐨?raid 6锛屼互鍙婁竴鍧楁垨涓夊潡 ssd銆傚鏋滆兘鎶婂畠浠敤浣滅紦瀛樺矀涓嶇編鍝夆€︹€︿簬鏄湁浜?bcache銆?

bcache wiki 浣嶄簬锛?
  https://bcache.evilpiepirate.org

杩欐槸 bcache-tools 鐨?git 浠撳簱锛?
  https://git.kernel.org/pub/scm/linux/kernel/git/colyli/bcache-tools.git/

鏈€鏂扮殑 bcache 鍐呮牳浠ｇ爜鍙湪涓荤嚎 Linux 鍐呮牳涓壘鍒帮細
  https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/

瀹冪殑璁捐鍥寸粫 SSD 鐨勬€ц兘鐗瑰緛灞曞紑鈥斺€斿畠鍙湪鎿﹂櫎鍧楋紙erase block锛夊ぇ灏忕殑妗讹紙bucket锛変腑鍒嗛厤锛屽苟浣跨敤娣峰悎鐨?btree/鏃ュ織鏉ヨ窡韪紦瀛樼殑鍖烘锛坋xtent锛夛紙鍖烘澶у皬鍙粠鍗曚釜鎵囧尯鍒版《澶у皬涓嶇瓑锛夈€傚畠涓嶉仐浣欏姏鍦伴伩鍏嶉殢鏈哄啓銆?

write-through锛堥€忓啓锛夊拰 writeback锛堝洖鍐欙級缂撳瓨閮藉彈鏀寔銆倃riteback 榛樿鍏抽棴锛屼絾鍙互鍦ㄨ繍琛屾椂浠绘剰寮€鍚垨鍏抽棴銆俠cache 绔姏淇濇姢鎮ㄧ殑鏁版嵁鈥斺€斿畠鑳藉彲闈犲湴澶勭悊闈炴甯稿叧鏈恒€傦紙瀹冪敋鑷虫病鏈夆€滃共鍑€鍏虫満鈥濈殑姒傚康锛沚cache 鍙湁鍦ㄥ啓鍏ュ埌杈剧ǔ瀹氬瓨鍌ㄥ悗鎵嶄細灏嗗啓鎿嶄綔杩斿洖涓哄凡瀹屾垚锛夈€?

writeback 缂撳瓨鍙互浣跨敤澶ч儴鍒嗙紦瀛樻潵缂撳啿鍐欐搷浣溾€斺€斿皢鑴忔暟鎹啓鍏?backing 璁惧濮嬬粓鏄『搴忚繘琛岀殑锛屼粠绱㈠紩鐨勮捣濮嬫壂鎻忓埌鏈熬銆?

鐢变簬闅忔満 IO 姝ｆ槸 SSD 鎵€鎿呴暱鐨勶紝缂撳瓨澶х殑椤哄簭 IO 閫氬父濂藉涓嶅ぇ銆俠cache 妫€娴嬮『搴?IO 骞惰烦杩囧畠锛涘畠杩樺姣忎釜浠诲姟鐨?IO 澶у皬淇濇寔婊氬姩骞冲潎锛屽彧瑕佸钩鍧囧€奸珮浜?cutoff 灏变細璺宠繃璇ヤ换鍔＄殑鎵€鏈?IO鈥斺€旇€屼笉鏄湪姣忔 seek 鍚庣紦瀛樺墠 512k銆傚洜姝ゅ浠藉拰澶ф枃浠跺鍒跺簲褰撳畬鍏ㄧ粫杩囩紦瀛樸€?

鑻ラ棯瀛樹笂鍙戠敓鏁版嵁 IO 閿欒锛宐cache 浼氬皾璇曢€氳繃浠庣鐩樿鍙栨垨浣跨紦瀛樻潯鐩け鏁堟潵鎭㈠銆傚浜庝笉鍙仮澶嶇殑閿欒锛堝厓鏁版嵁鎴栬剰鏁版嵁锛夛紝缂撳瓨浼氳嚜鍔ㄧ鐢紱鑻ョ紦瀛樹腑瀛樺湪鑴忔暟鎹紝瀹冧細鍏堢鐢?writeback 缂撳瓨骞剁瓑寰呮墍鏈夎剰鏁版嵁琚埛鍑恒€?

Getting started锛堝叆闂級锛?
鎮ㄥ皢闇€瑕佹潵鑷?bcache-tools 浠撳簱鐨?bcache 宸ュ叿銆俢ache 璁惧
```
  bcache make -B /dev/sdb
  bcache make -C /dev/sdc
```
`bcache make` 鑳藉鍚屾椂鏍煎紡鍖栧涓澶団€斺€斿鏋滄偍鍚屾椂鏍煎紡鍖?backing 璁惧鍜?cache 璁惧锛屽氨涓嶄細
```
  bcache make -B /dev/sda /dev/sdb -C /dev/sdc
```
濡傛灉鎮ㄧ殑 bcache-tools 鏈洿鏂板埌鏈€鏂扮増鏈笖涓嶅叿鏈夌粺涓€鐨?`bcache` 宸ュ叿锛屾偍鍙互浣跨敤鏃х殑 `make-bcache` 宸ュ叿锛屼互鐩稿悓鐨?-B 鍜?-C 鍙傛暟鏍煎紡鍖?bcache 璁惧銆?

bcache-tools 鐜板湪闄勫甫 udev 瑙勫垯锛宐cache 璁惧涓哄唴鏍告墍鐭?
```
  echo /dev/sdb > /sys/fs/bcache/register
  echo /dev/sdc > /sys/fs/bcache/register
```
娉ㄥ唽 backing 璁惧浼氫娇 bcache 璁惧鍑虹幇鍦?/dev 涓紱鎮ㄧ幇鍦ㄥ彲浠ュ儚骞冲父涓€鏍锋牸寮忓寲骞朵娇鐢ㄥ畠銆備絾棣栨浣跨敤鏂扮殑 bcache 璁惧鏃讹紝鍦ㄥ皢鍏?attach 鍒扮紦瀛樹箣鍓嶏紝瀹冨皢杩愯鍦?passthrough锛堢洿閫氾級妯″紡銆傚鏋滄偍鎵撶畻绋嶅悗浣跨敤 bcache锛屽缓璁皢鎵€鏈夋參閫熻澶囬兘璁句负涓嶅甫缂撳瓨鐨?bcache backing 璁惧锛屼箣鍚庢偍鍙互閫夋嫨娣诲姞缂撳瓨璁惧銆?
鍙傝涓嬫枃鐨勨€淎TTACHING鈥濈珷鑺傘€?

```
  /dev/bcache<N>
```
```
  /dev/bcache/by-uuid/<uuid>
  /dev/bcache/by-label/<label>
```
```
  mkfs.ext4 /dev/bcache0
  mount /dev/bcache0 /mnt
```
鎮ㄥ彲浠ラ€氳繃 sysfs 鍦?/sys/block/bcache<N>/bcache 鎺у埗 bcache 璁惧銆傛偍涔熷彲浠ラ€氳繃 /sys/fs//bcache/<cset-uuid>/ 鎺у埗瀹冧滑銆?

Cache 璁惧浠ラ泦鍚堬紙set锛夊舰寮忕鐞嗭紱姣忎釜闆嗗悎鐩墠杩樹笉鏀寔澶氫釜缂撳瓨锛屼絾鏈潵灏嗗厑璁稿厓鏁版嵁鍜岃剰鏁版嵁鐨勯暅鍍忋€傛偍鐨勬柊缂撳瓨闆嗗悎鏄剧ず涓?/sys/fs/bcache/<UUID>

### Attaching锛堥檮鍔?缁戝畾锛?


鍦ㄦ偍鐨?cache 璁惧鍜?backing 璁惧娉ㄥ唽鍚庯紝蹇呴』灏?backing 璁惧 attach 鍒扮紦瀛橀泦鍚堜互鍚敤缂撳瓨銆傚皢 backing 璁惧 attach 鍒扮紦瀛橀泦鍚堢殑鎿嶄綔濡備笅锛屼娇鐢ㄧ紦瀛橀泦鍚堢殑 UUID 鍐欏叆
```
  echo <CSET-UUID> > /sys/block/bcache0/bcache/attach
```
杩欏彧闇€鍋氫竴娆°€備笅娆￠噸鍚椂锛屽彧闇€閲嶆柊娉ㄥ唽鎮ㄧ殑鎵€鏈?bcache 璁惧銆傚鏋滄煇涓?backing 璁惧鍦ㄦ煇涓紦瀛樹腑鏈夋暟鎹紝/dev/bcache<N> 璁惧瑕佺瓑鍒扮紦瀛樺嚭鐜板悗鎵嶄細琚垱寤衡€斺€斿鏋滄偍寮€鍚簡 writeback 缂撳瓨锛岃繖涓€鐐瑰挨涓洪噸瑕併€?

濡傛灉鎮ㄥ湪鍚姩鏃剁紦瀛樿澶囦涪澶变笖鍐嶄篃涓嶄細鍥炴潵锛屾偍
```
  echo 1 > /sys/block/sdb/bcache/running
```
锛堟偍闇€瑕佷娇鐢?/sys/block/sdb锛堟垨鎮ㄧ殑 backing 璁惧鍙粈涔堬級锛岃€屼笉鏄?/sys/block/bcache0锛屽洜涓?bcache0 灏氫笉瀛樺湪銆傚鏋滄偍浣跨敤鐨勬槸鍒嗗尯锛宐cache 鐩綍灏嗕綅浜?/sys/block/sdb/sdb2/bcache锛?

璇?backing 璁惧鑻ュ皢鏉ュ嚭鐜颁粛浼氫娇鐢ㄩ偅涓紦瀛橀泦鍚堬紝浣嗘墍鏈夌紦瀛樻暟鎹兘浼氳澶辨晥銆傚鏋滅紦瀛樹腑鏈夎剰鏁版嵁锛屼笉瑕佹寚鏈涙枃浠剁郴缁熷彲鎭㈠鈥斺€旀偍灏嗛潰涓村ぇ瑙勬ā鐨勬枃浠剁郴缁熸崯鍧忥紝灏界 ext4 鐨?fsck 纭疄鑳藉垱閫犲杩广€?

### Error Handling锛堥敊璇鐞嗭級


bcache 灏濊瘯閫忔槑鍦板鐞嗚繘鍑虹紦瀛樿澶囩殑 IO 閿欒锛岃€屼笉褰卞搷姝ｅ父鎿嶄綔锛涘鏋滃畠鐪嬪埌杩囧閿欒锛堥槇鍊兼槸鍙厤缃殑锛岄粯璁や负 0锛夛紝瀹冧細鍏抽棴缂撳瓨璁惧骞跺皢鎵€鏈?backing 璁惧鍒囨崲鍒?passthrough 妯″紡銆?

 - 瀵逛簬鏉ヨ嚜缂撳瓨鐨勮锛岃嫢鍑洪敊锛屾垜浠彧鏄粠 backing 璁惧閲嶈瘯璇ヨ銆?

 - 瀵逛簬 write-through 鍐欙紝鑻ュ缂撳瓨鐨勫啓鍑洪敊锛屾垜浠彧鏄垏鎹㈠埌浣跨紦瀛樹腑璇?lba 鐨勬暟鎹け鏁堬紙鍗筹紝涓庣粫杩囩紦瀛樼殑鍐欐墍鍋氱殑鐩稿悓锛夈€?

 - 瀵逛簬 writeback 鍐欙紝鎴戜滑鐩墠灏嗚閿欒浼犲洖缁欐枃浠剁郴缁?鐢ㄦ埛绌洪棿銆傝繖鍙互寰楀埌鏀硅繘鈥斺€旀垜浠彲浠ュ皢鍏朵綔涓鸿烦杩囩紦瀛樼殑鍐欐潵閲嶈瘯锛屼粠鑰屼笉蹇呬娇璇ュ啓鍑洪敊銆?

 - 褰撴垜浠?detach 鏃讹紝鎴戜滑棣栧厛灏濊瘯鍒峰嚭浠讳綍鑴忔暟鎹紙濡傛灉鎴戜滑杩愯鍦?writeback 妯″紡锛夈€備笉杩囷紝濡傛灉鏌愪簺鑴忔暟鎹鍙栧け璐ワ紝瀹冪洰鍓嶄笉浼氬仛浠讳綍鏅鸿兘澶勭悊銆?

### Howto/cookbook锛堟搷浣滄寚鍗?绉樼睄锛?


A) 浣跨敤缂哄け鐨勭紦瀛樿澶囧惎鍔?bcache

濡傛灉娉ㄥ唽 backing 璁惧娌℃湁甯姪锛岃鏄庡畠宸茬粡瀛樺湪锛屾偍鍙渶
```
	host:~# echo /dev/sdb1 > /sys/fs/bcache/register
	[  119.844831] bcache: register_bcache() error opening /dev/sdb1: device already registered

```
鎺ヤ笅鏉ワ紝濡傛灉缂撳瓨璁惧瀛樺湪锛屾偍灏濊瘯娉ㄥ唽瀹冦€備絾濡傛灉瀹冪己澶憋紝鎴栧洜鏌愮鍘熷洜娉ㄥ唽澶辫触锛屾偍浠嶇劧鍙互
```
	host:/sys/block/sdb/sdb1/bcache# echo 1 > running

```
娉ㄦ剰锛屽鏋滄偍杩愯鍦?writeback 妯″紡锛岃繖鍙兘浼氬鑷存暟鎹涪澶便€?

```
	host:/sys/block/md5/bcache# echo 0226553a-37cf-41d5-b3ce-8b1e944543a8 > attach
	[ 1933.455082] bcache: bch_cached_dev_attach() Couldn't find uuid for md5 in set
	[ 1933.478179] bcache: __cached_dev_store() Can't attach 0226553a-37cf-41d5-b3ce-8b1e944543a8
	[ 1933.478179] : cache set not found

```
鍦ㄨ繖绉嶆儏鍐典笅锛岀紦瀛樿澶囧彧鏄湪鍚姩鏃舵湭娉ㄥ唽
```
	host:/sys/block/md5/bcache# echo /dev/sdh2 > /sys/fs/bcache/register


```
C) 鎹熷潖鐨?bcache 鍦ㄨ澶囨敞鍐屾椂瀵艰嚧鍐呮牳宕╂簝锛?

杩欑粷涓嶅簲璇ュ彂鐢熴€傚鏋滅‘瀹炲彂鐢熶簡锛岄偅涔堟偍鍙戠幇浜嗕竴涓?bug锛?
璇峰皢鍏舵姤鍛婄粰 bcache 寮€鍙戦偖浠跺垪琛細linux-bcache@vger.kernel.org

璇峰姟蹇呮彁渚涘敖鍙兘澶氱殑淇℃伅锛屽寘鎷唴鏍?dmesg 杈撳嚭锛堝鏋滃彲寰楋級锛屼互渚挎垜浠彁渚涘府鍔┿€?


D) 鍦ㄦ病鏈?bcache 鐨勬儏鍐典笅鎭㈠鏁版嵁锛?

濡傛灉鍐呮牳涓病鏈?bcache锛宐acking 璁惧涓婄殑鏂囦欢绯荤粺浠嶇劧浣嶄簬 8KiB 鍋忕Щ澶勫彲鐢ㄣ€傚洜姝わ紝鍙互閫氳繃鐢?--offset 8K 鍒涘缓鐨?backing 璁惧鐨?loopdev锛屾垨鑰呴€氳繃鎮ㄦ渶鍒濈敤 `bcache make` 鏍煎紡鍖?bcache 鏃剁敱 --data-offset 瀹氫箟鐨勪换浣曞€兼潵璁块棶銆?

```
	losetup -o 8192 /dev/loop0 /dev/your_bcache_backing_dev

```
杩欏皢鍦?/dev/loop0 涓憟鐜版偍鏈慨鏀圭殑 backing 璁惧鏁版嵁銆?

濡傛灉鎮ㄧ殑缂撳瓨澶勪簬 write-through 妯″紡锛岄偅涔堟偍鍙互瀹夊叏鍦颁涪寮冪紦瀛樿澶囪€屼笉涓㈠け鏁版嵁銆?


E) 鎿﹂櫎缂撳瓨璁惧

```
	host:~# wipefs -a /dev/sdh2
	16 bytes were erased at offset 0x1018 (bcache)
	they were: c6 85 73 f6 4e 1a 45 ca 82 65 f5 7f 48 ba 6d 81

```
```
	host:~# bcache make -C /dev/sdh2
	UUID:                   7be7e175-8f4c-4f99-94b2-9c904d227045
	Set UUID:               5bc072a8-ab17-446d-9744-e247949913c1
	version:                0
	nbuckets:               106874
	block_size:             1
	bucket_size:            1024
	nr_in_set:              1
	nr_this_dev:            0
	first_bucket:           1
	[  650.511912] bcache: run_cache_set() invalidating existing data
	[  650.549228] bcache: register_cache() registered cache device sdh2

```
```
	host:/sys/block/md5/bcache# echo 1 > running

```
```
	host:/sys/block/md5/bcache# echo 5bc072a8-ab17-446d-9744-e247949913c1 > attach
	[  865.276616] bcache: bch_cached_dev_attach() Caching md5 as bcache0 on set 5bc072a8-ab17-446d-9744-e247949913c1


```
```
	host:/sys/block/sda/sda7/bcache# echo 1 > detach
	[  695.872542] bcache: cached_dev_detach_finish() Caching disabled for sda7

	host:~# wipefs -a /dev/nvme0n1p4
	wipefs: error: /dev/nvme0n1p4: probing initialization failed: Device or resource busy
	Ooops, it's disabled, but not unregistered, so it's still protected

```
```
	host:/sys/fs/bcache/b7ba27a1-2398-4649-8ae3-0959f57ba128# ls -l cache0
	lrwxrwxrwx 1 root root 0 Feb 25 18:33 cache0 -> ../../../devices/pci0000:00/0000:00:1d.0/0000:70:00.0/nvme/nvme0/nvme0n1/nvme0n1p4/bcache/
	host:/sys/fs/bcache/b7ba27a1-2398-4649-8ae3-0959f57ba128# echo 1 > stop
	kernel: [  917.041908] bcache: cache_set_free() Cache set b7ba27a1-2398-4649-8ae3-0959f57ba128 unregistered

```
```
	host:~# wipefs -a /dev/nvme0n1p4
	/dev/nvme0n1p4: 16 bytes were erased at offset 0x00001018 (bcache): c6 85 73 f6 4e 1a 45 ca 82 65 f5 7f 48 ba 6d 81


```
G) dm-crypt 涓?bcache

棣栧厛璁剧疆鏈姞瀵嗙殑 bcache锛岀劧鍚庡湪 /dev/bcache<N> 涔嬩笂瀹夎 dmcrypt銆傝繖姣斿悓鏃?dmcrypt 鍔犲瘑 backing 鍜?caching 璁惧鍐嶅湪鍏朵笂瀹夎 bcache 瑕佸揩銆俒benchmarks?]


H) 鍋滄/閲婃斁宸叉敞鍐岀殑 bcache 浠ユ摝闄ゅ拰/鎴栭噸寤哄畠

鍋囪鎮ㄩ渶瑕侀噴鏀炬墍鏈?bcache 寮曠敤锛屼互渚胯繍琛?fdisk 骞堕噸鏂版敞鍐屽凡鏇存敼鐨勫垎鍖鸿〃锛岃€屽彧瑕佷笂闈㈣繕鏈変换浣曟椿璺冪殑 backing 鎴?caching 璁惧锛岃繖灏辨棤娉曞伐浣滐細

1) 瀹冩槸鍚﹀嚭鐜板湪 /dev/bcache* 涓紵锛堟湁鏃跺畠涓嶄細锛?

```
	host:/sys/block/bcache0/bcache# echo 1 > stop

```
```
	host:/sys/block/bcache0# cd bcache
	bash: cd: bcache: No such file or directory

   In this case, you may have to unregister the dmcrypt block device that
   references this bcache to free it up::

	host:~# dmsetup remove oldds1
	bcache: bcache_device_free() bcache0 stopped
	bcache: cache_set_free() Cache set 5bc072a8-ab17-446d-9744-e247949913c1 unregistered

   This causes the backing bcache to be removed from /sys/fs/bcache and
   then it can be reused.  This would be true of any block device stacking
   where bcache is a lower device.

```
```
	host:/sys/fs/bcache# ls -l */{cache?,bdev?}
	lrwxrwxrwx 1 root root 0 Mar  5 09:39 0226553a-37cf-41d5-b3ce-8b1e944543a8/bdev1 -> ../../../devices/virtual/block/dm-1/bcache/
	lrwxrwxrwx 1 root root 0 Mar  5 09:39 0226553a-37cf-41d5-b3ce-8b1e944543a8/cache0 -> ../../../devices/virtual/block/dm-4/bcache/
	lrwxrwxrwx 1 root root 0 Mar  5 09:39 5bc072a8-ab17-446d-9744-e247949913c1/cache0 -> ../../../devices/pci0000:00/0000:00:01.0/0000:01:00.0/ata10/host9/target9:0:0/9:0:0:0/block/sdl/sdl2/bcache/

   The device names will show which UUID is relevant, cd in that directory
   and stop the cache::

	host:/sys/fs/bcache/5bc072a8-ab17-446d-9744-e247949913c1# echo 1 > stop

   This will free up bcache references and let you reuse the partition for
   other purposes.

```
### Troubleshooting performance锛堟帓鏌ユ€ц兘闂锛?


Bcache 鏈変竴鍫嗛厤缃€夐」鍜屽彲璋冨弬鏁般€傞粯璁ゅ€兼棬鍦ㄥ鍏稿瀷鐨勬闈㈠拰鏈嶅姟鍣ㄥ伐浣滆礋杞藉悎鐞嗭紝浣嗗湪鍩哄噯娴嬭瘯鏃舵兂鑾峰緱灏藉彲鑳藉ソ鐨勬暟瀛楋紝瀹冧滑骞朵笉鏄偍鎯宠鐨勩€?

 - Backing device alignment锛坆acking 璁惧瀵归綈锛?

   鍦?bcache 涓紝榛樿鐨勫厓鏁版嵁澶у皬鏄?8k銆傚鏋滄偍鐨?backing 璁惧鍩轰簬 RAID锛岄偅涔堝姟蹇呬娇鐢?`bcache make --data-offset` 鎸?stride 瀹藉害鐨勫€嶆暟瀵归綈銆傚鏋滄偍鎵撶畻灏嗘潵鎵╁睍纾佺洏闃靛垪锛屽垯灏嗕竴绯诲垪绱犳暟涔樹互鎮ㄧ殑 raid stripe 澶у皬锛屼互鑾峰緱鎮ㄦ兂瑕佺殑纾佺洏鍊嶆暟銆?

   渚嬪锛氬鏋滄偍鏈?64k 鐨?stripe 澶у皬锛岄偅涔堜互涓嬪亸绉婚噺

```

	64k * 2*2*2*3*3*5*7 bytes = 161280k

   That space is wasted, but for only 157.5MB you can grow your RAID 5
   volume to the following data-spindle counts without re-aligning::

	3,4,5,6,7,8,9,10,12,14,15,18,20,21 ...

 - Bad write performance

   If write performance is not what you expected, you probably wanted to be
   running in writeback mode, which isn't the default (not due to a lack of
   maturity, but simply because in writeback mode you'll lose data if something
   happens to your SSD)::

	# echo writeback > /sys/block/bcache0/bcache/cache_mode

 - Bad performance, or traffic not going to the SSD that you'd expect

   By default, bcache doesn't cache everything. It tries to skip sequential IO -
   because you really want to be caching the random IO, and if you copy a 10
   gigabyte file you probably don't want that pushing 10 gigabytes of randomly
   accessed data out of your cache.

   But if you want to benchmark reads from cache, and you start out with fio
   writing an 8 gigabyte test file - so you want to disable that::

	# echo 0 > /sys/block/bcache0/bcache/sequential_cutoff

   To set it back to the default (4 mb), do::

	# echo 4M > /sys/block/bcache0/bcache/sequential_cutoff

 - Traffic's still going to the spindle/still getting cache misses

   In the real world, SSDs don't always keep up with disks - particularly with
   slower SSDs, many disks being cached by one SSD, or mostly sequential IO. So
   you want to avoid being bottlenecked by the SSD and having it slow everything
   down.

   To avoid that bcache tracks latency to the cache device, and gradually
   throttles traffic if the latency exceeds a threshold (it does this by
   cranking down the sequential bypass).

   You can disable this if you need to by setting the thresholds to 0::

	# echo 0 > /sys/fs/bcache/<cache set>/congested_read_threshold_us
	# echo 0 > /sys/fs/bcache/<cache set>/congested_write_threshold_us

   The default is 2000 us (2 milliseconds) for reads, and 20000 for writes.

 - Still getting cache misses, of the same data

   One last issue that sometimes trips people up is actually an old bug, due to
   the way cache coherency is handled for cache misses. If a btree node is full,
   a cache miss won't be able to insert a key for the new data and the data
   won't be written to the cache.

   In practice this isn't an issue because as soon as a write comes along it'll
   cause the btree node to be split, and you need almost no write traffic for
   this to not show up enough to be noticeable (especially since bcache's btree
   nodes are huge and index large regions of the device). But when you're
   benchmarking, if you're trying to warm the cache by reading a bunch of data
   and there's no other traffic - that can be a problem.

   Solution: warm the cache by doing writes, or use the testing branch (there's
   a fix for the issue there).


```

### Sysfs - backing device锛圫ysfs - backing 璁惧锛?


浣嶄簬 /sys/block/<bdev>/bcache銆?sys/block/bcache*/bcache 浠ュ強锛堣嫢宸?attach锛?sys/fs/bcache/<cset-uuid>/bdev*

attach
  灏嗚缂撳瓨闆嗗悎鐨?UUID 鍐欏叆姝ゆ枃浠朵互鍚敤缂撳瓨銆?

cache_mode
  鍙互鏄?writethrough銆亀riteback銆亀ritearound 鎴?none 涔嬩竴銆?

clear_stats
  鍐欏叆姝ゆ枃浠朵細閲嶇疆绱缁熻锛堜笉鏄寜澶?灏忔椂/5 鍒嗛挓鐨勮“鍑忕増鏈級銆?

detach
  鍐欏叆姝ゆ枃浠朵互浠庣紦瀛橀泦鍚?detach銆傚鏋滅紦瀛樹腑鏈夎剰鏁版嵁锛屼細鍏堝皢鍏跺埛鍑恒€?

dirty_data
  姝?backing 璁惧鍦ㄧ紦瀛樹腑鐨勮剰鏁版嵁閲忋€備笌缂撳瓨闆嗗悎鐨勭増鏈笉鍚岋紝瀹冩寔缁洿鏂帮紝浣嗗彲鑳界暐鏈夊亸宸€?

label
  搴曞眰璁惧鐨勫悕绉般€?

readahead
  搴旀墽琛岀殑棰勮鐨勫瓧鑺傛暟銆傞粯璁や负 0銆傝嫢璁句负渚嬪 1M锛屽畠浼氬皢缂撳瓨鏈懡涓殑璇诲悜涓婂彇鏁村埌璇ュぇ灏忥紝浣嗕笉涓庣幇鏈夌紦瀛樻潯鐩噸鍙犮€?

running
  濡傛灉 bcache 姝ｅ湪杩愯鍒欎负 1锛堝嵆 /dev/bcache 璁惧鏄惁瀛樺湪锛屾棤璁哄畠澶勪簬 passthrough 妯″紡杩樻槸缂撳瓨妯″紡锛夈€?

sequential_cutoff
  椤哄簭 IO 涓€鏃﹁秴杩囨闃堝€煎氨浼氱粫杩囩紦瀛橈紱浼氳窡韪渶杩?128 娆?IO锛屽洜姝ゅ嵆浣夸笉鏄竴娆℃€у畬鎴愮殑椤哄簭 IO 涔熻兘琚娴嬪嚭鏉ャ€?

sequential_merge
  鑻ラ潪闆讹紝bcache 淇濈暀鏈€杩?128 涓姹傜殑鍒楄〃锛屼笌鎵€鏈夋柊璇锋眰姣旇緝锛屼互纭畾鍝簺鏂拌姹傛槸鍏堝墠璇锋眰鐨勯『搴忓欢缁紝浠庤€屽喅瀹氶『搴?cutoff銆傚鏋滈『搴?cutoff 鍊煎ぇ浜庝换浣曞崟涓姹傜殑鏈€澶у彲鎺ュ彈椤哄簭澶у皬锛屽垯杩欐槸蹇呰鐨勩€?

state
  backing 璁惧鍙互澶勪簬浠ヤ笅鍥涚鐘舵€佷箣涓€锛?

  no cache锛氫粠鏈?attach 鍒扮紦瀛橀泦鍚堛€?

  clean锛氱紦瀛橀泦鍚堢殑涓€閮ㄥ垎锛屼笖娌℃湁缂撳瓨鐨勮剰鏁版嵁銆?

  dirty锛氱紦瀛橀泦鍚堢殑涓€閮ㄥ垎锛屼笖鏈夌紦瀛樼殑鑴忔暟鎹€?

  inconsistent锛氬綋瀛樺湪缂撳瓨鐨勮剰鏁版嵁浣嗙紦瀛橀泦鍚堜笉鍙敤鏃讹紝鐢ㄦ埛寮鸿杩愯浜?backing 璁惧锛沚acking 璁惧涓婄殑浠讳綍鏁版嵁鍙兘閮藉凡鎹熷潖銆?

stop
  鍐欏叆姝ゆ枃浠朵互鍏抽棴 bcache 璁惧骞跺叧闂?backing 璁惧銆?

writeback_delay
  褰撹剰鏁版嵁鍐欏叆缂撳瓨涓斿叾涔嬪墠涓嶅寘鍚换浣曡剰鏁版嵁鏃讹紝浼氱瓑寰呰嫢骞茬鍚庡啀鍚姩 writeback銆傞粯璁や负 30銆?

writeback_percent
  鑻ラ潪闆讹紝bcache 灏濊瘯閫氳繃闄愬埗鍚庡彴 writeback 骞朵娇鐢?PD 鎺у埗鍣ㄥ钩婊戣皟鏁撮€熺巼锛屽皢姝ょ櫨鍒嗘瘮鐨勭紦瀛樹繚鎸佷负鑴忋€?

writeback_rate
  浠ユ瘡绉掓墖鍖烘暟璁＄殑閫熺巼鈥斺€旇嫢 writeback_percent 闈為浂锛屽悗鍙?writeback 琚檺鍒跺埌姝ら€熺巼銆傜敱 bcache 鎸佺画璋冩暣锛屼絾涔熷彲鐢辩敤鎴疯缃€?

writeback_running
  鑻ュ叧闂紝鑴忔暟鎹殑 writeback 灏嗗畬鍏ㄤ笉杩涜銆傝剰鏁版嵁浠嶄細琚姞鍏ョ紦瀛樼洿鍒板畠鍑犱箮婊★紱浠呯敤浜庡熀鍑嗘祴璇曘€傞粯璁や负寮€鍚€?

#### Sysfs - backing device stats锛圫ysfs - backing 璁惧缁熻锛?


瀛樺湪甯︽湁杩欎簺鏁板瓧鐨勭洰褰曠敤浜庣疮璁℃€绘暟锛屼互鍙婅繃鍘讳竴澶┿€佷竴灏忔椂鍜?5 鍒嗛挓鍐呰“鍑忕殑鐗堟湰锛涘畠浠篃鍦ㄧ紦瀛橀泦鍚堢洰褰曚腑琚仛鍚堛€?

bypassed
  缁曡繃缂撳瓨鐨?IO 閲忥紙璇诲拰鍐欓兘鏈夛級

cache_hits, cache_misses, cache_hit_ratio
  鍛戒腑涓庢湭鍛戒腑鎸?bcache 鎵€瑙佺殑姣忎釜鐙珛 IO 璁℃暟锛涢儴鍒嗗懡涓涓烘湭鍛戒腑銆?

cache_bypass_hits, cache_bypass_misses
  閽堝鏈簲璺宠繃缂撳瓨鐨?IO 鐨勫懡涓笌鏈懡涓粛浼氳璁℃暟锛屼絾鍦ㄦ鍗曠嫭鍒楀嚭銆?

cache_miss_collisions
  璁℃暟鏁版嵁鏈皢浠庣紦瀛樻湭鍛戒腑鎻掑叆缂撳瓨锛屼絾涓庝竴娆″啓绔炰簤涓旀暟鎹凡瀛樺湪鐨勬儏鍐碉紙閫氬父涓?0锛屽洜涓虹紦瀛樻湭鍛戒腑鐨勫悓姝ュ凡琚噸鍐欙級

#### Sysfs - cache set锛圫ysfs - 缂撳瓨闆嗗悎锛?


浣嶄簬 /sys/fs/bcache/<cset-uuid>

average_key_size
  btree 涓瘡涓敭鐨勫钩鍧囨暟鎹噺銆?

bdev<0..n>
  鎸囧悜姣忎釜宸?attach 鐨?backing 璁惧鐨勭鍙烽摼鎺ャ€?

block_size
  缂撳瓨璁惧鐨勫潡澶у皬銆?

btree_cache_size
  btree 缂撳瓨褰撳墠浣跨敤鐨勫唴瀛橀噺

bucket_size
  妗剁殑澶у皬

cache<0..n>
  鎸囧悜缁勬垚姝ょ紦瀛橀泦鍚堢殑姣忎釜缂撳瓨璁惧鐨勭鍙烽摼鎺ャ€?

cache_available_percent
  涓嶅寘鍚剰鏁版嵁銆佸彲鑳界敤浜?writeback 鐨勭紦瀛樿澶囩櫨鍒嗘瘮銆傝繖骞朵笉鎰忓懗姝ょ┖闂存湭琚敤浜庡共鍑€鐨勭紦瀛樻暟鎹紱鏈娇鐢ㄧ粺璁★紙鍦?priority_stats 涓級閫氬父浣庡緱澶氥€?

clear_stats
  娓呴櫎涓庢缂撳瓨鐩稿叧鐨勭粺璁?

dirty_data
  缂撳瓨涓殑鑴忔暟鎹噺锛堝湪鍨冨溇鍥炴敹杩愯鏃舵洿鏂帮級銆?

flash_vol_create
  灏嗗ぇ灏忥紙浠ヤ汉绫诲彲璇诲崟浣?k/M/G 鍥炴樉锛夊啓鍏ユ鏂囦欢锛屼細鍒涘缓涓€涓敱缂撳瓨闆嗗悎鏀拺鐨勭簿绠€閰嶇疆鍗枫€?

io_error_halflife, io_error_limit
  杩欎簺鍐冲畾鎴戜滑鍦ㄧ鐢ㄧ紦瀛樹箣鍓嶆帴鍙楀灏戦敊璇€傛瘡涓敊璇寜鍗婅“鏈燂紙浠?IO 鏁拌锛夎“鍑忋€傚鏋滆“鍑忚鏁拌揪鍒?io_error_limit锛岃剰鏁版嵁浼氳鍐欏嚭涓旂紦瀛樿绂佺敤銆?

journal_delay_ms
  鏃ュ織鍐欎細寤惰繜鑷冲杩欎簺姣锛岄櫎闈炵紦瀛樺埛鏂板彂鐢熷緱鏇存棭銆傞粯璁や负 100銆?

root_usage_percent
  鏍?btree 鑺傜偣鐨勪娇鐢ㄧ櫨鍒嗘瘮銆傚鏋滆繃楂橈紝鑺傜偣浼氭媶鍒嗭紝澧炲姞鏍戠殑娣卞害銆?

stop
  鍐欏叆姝ゆ枃浠朵互鍏抽棴缂撳瓨闆嗗悎鈥斺€旂瓑寰呮墍鏈夊凡 attach 鐨?backing 璁惧閮借鍏抽棴銆?

tree_depth
  btree 鐨勬繁搴︼紙鍗曡妭鐐?btree 娣卞害涓?0锛夈€?

unregister
  鍒嗙鎵€鏈?backing 璁惧骞跺叧闂紦瀛樿澶囷紱濡傛灉瀛樺湪鑴忔暟鎹紝瀹冧細绂佺敤 writeback 缂撳瓨骞剁瓑寰呭叾琚埛鍑恒€?

#### Sysfs - cache set internal锛圫ysfs - 缂撳瓨闆嗗悎鍐呴儴锛?


姝ょ洰褰曡繕鏆撮湶浜嗚澶氬唴閮ㄦ搷浣滅殑璁℃椂锛屽垎鍒湁骞冲潎鏃堕暱銆佸钩鍧囬鐜囥€佹渶杩戝彂鐢熷拰鏈€澶ф椂闀跨殑鏂囦欢锛氬瀮鍦惧洖鏀躲€乥tree 璇汇€乥tree 鑺傜偣鎺掑簭鍜?btree 鎷嗗垎銆?

active_journal_entries
  姣旂储寮曟洿鏂扮殑鏃ュ織鏉＄洰鏁般€?

btree_nodes
  btree 涓殑鑺傜偣鎬绘暟銆?

btree_used_percent
  btree 骞冲潎浣跨敤姣斾緥銆?

bset_tree_stats
  鍏充簬杈呭姪鎼滅储鏍戠殑缁熻

btree_cache_max_chain
  btree 鑺傜偣缂撳瓨鐨勫搱甯岃〃涓渶闀跨殑閾?

cache_read_races
  璁℃暟鍦ㄤ粠缂撳瓨璇诲彇鏁版嵁鏈熼棿锛屾《琚噸鐢ㄥ苟澶辨晥鐨勬儏鍐碘€斺€斿嵆璇诲彇瀹屾垚鍚庢寚閽堝凡澶辨晥銆傚彂鐢熸鎯呭喌鏃讹紝鏁版嵁浼氫粠 backing 璁惧閲嶆柊璇诲彇銆?

trigger_gc
  鍐欏叆姝ゆ枃浠朵細寮哄埗杩愯鍨冨溇鍥炴敹銆?

#### Sysfs - Cache device锛圫ysfs - 缂撳瓨璁惧锛?


浣嶄簬 /sys/block/<cdev>/bcache

block_size
  鍐欐搷浣滅殑鏈€灏忕矑搴︹€斺€斿簲涓庣‖浠舵墖鍖哄ぇ灏忓尮閰嶃€?

btree_written
  鎵€鏈?btree 鍐欑殑鎬诲拰锛屼互锛堝崈/鍏?鍚夛級瀛楄妭璁?

bucket_size
  妗剁殑澶у皬

cache_replacement_policy
  涓?lru銆乫ifo 鎴?random 涔嬩竴銆?

freelist_percent
  绌洪棽鍒楄〃澶у皬鍗?nbuckets 鐨勭櫨鍒嗘瘮銆傚彲鍐欏叆浠ュ鍔犵┖闂插垪琛ㄤ笂淇濈暀鐨勬《鏁帮紝浠庤€岃鎮ㄥ湪杩愯鏃朵汉涓哄噺灏忕紦瀛樺ぇ灏忋€備富瑕佺敤浜庢祴璇曠洰鐨勶紙鍗虫祴璇曚笉鍚屽ぇ灏忕殑缂撳瓨濡備綍褰卞搷鎮ㄧ殑鍛戒腑鐜囷級銆?

io_errors
  宸插彂鐢熺殑閿欒鏁帮紝鎸?io_error_halflife 琛板噺銆?

metadata_written
  鎵€鏈夐潪鏁版嵁鍐欑殑鎬诲拰锛坆tree 鍐欏拰鎵€鏈夊叾浠栧厓鏁版嵁锛夈€?

nbuckets
  姝ょ紦瀛樹腑鐨勬《鎬绘暟

priority_stats
  鍏充簬缂撳瓨涓暟鎹渶杩戣璁块棶鎯呭喌鐨勭粺璁°€傝繖鍙互鎻ず鎮ㄧ殑宸ヤ綔闆嗗ぇ灏忋€俇nused 鏄笉鍖呭惈浠讳綍鏁版嵁鐨勭紦瀛樼殑鐧惧垎姣斻€侻etadata 鏄?bcache 鐨勫厓鏁版嵁寮€閿€銆侫verage 鏄紦瀛樻《鐨勫钩鍧囦紭鍏堢骇銆侼ext 鏄竴涓甫鏈夋瘡涓紭鍏堢骇闃堝€肩殑鍒嗕綅鏁板垪琛ㄣ€?

written
  宸插啓鍏ョ紦瀛樼殑鎵€鏈夋暟鎹殑鎬诲拰锛涗笌 btree_written 姣旇緝鍙緱 bcache 涓殑鍐欒啫鑳€閲忋€?

