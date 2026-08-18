## dm-log-writes


璇?target 鎺ユ敹 2 涓澶囷紝涓€涓敤浜庢甯歌浆鍙戞墍鏈?IO锛屽彟涓€涓敤浜庤褰曟墍鏈?
鍐欐搷浣溿€傚叾闈㈠悜甯屾湜鍦ㄦ枃浠剁郴缁熷啓鍏ヨ繃绋嬩腑楠岃瘉鍏冩暟鎹垨鏁版嵁瀹屾暣鎬х殑
鏂囦欢绯荤粺寮€鍙戣€呫€傛瘡娆?WRITE 璇锋眰閮戒細鍐欏叆涓€涓?log_write_entry锛屽苟涓旇
target 鑳藉浠庣敤鎴风┖闂磋幏鍙栦换鎰忔暟鎹彃鍏ュ埌鏃ュ織涓€俉RITE 璇锋眰涓殑鏁版嵁浼氳
澶嶅埗鍒版棩蹇楅噷锛屼粠鑰屼娇閲嶆斁鑳藉瀹屽叏鎸夌収鍘熷鍙戠敓鐨勯『搴忚繘琛屻€?

## Log Ordering


鎴戜滑鎸夌収瀹屾垚鐨勯『搴忚褰曪紝鍓嶆彁鏄垜浠‘璁よ鍐欐搷浣滃凡涓嶅湪缂撳瓨涓€?
杩欐剰鍛崇潃鏅€氱殑 WRITE 璇锋眰瀹為檯涓婅绛夊埌涓嬩竴娆?REQ_PREFLUSH 璇锋眰
鍑虹幇鏃舵墠浼氳璁板綍銆傝繖鏍峰仛鏄负浜嗚鐢ㄦ埛绌洪棿鑳藉浠ヤ笌纾佺洏涓婏紙鑰岄潪缂撳瓨涓級
涓€鑷寸殑鏂瑰紡閲嶆斁鏃ュ織锛屼粠鑰屾洿瀹规槗鍙戠幇涓嶆纭殑绛夊緟/鍒锋柊琛屼负銆?

鍏跺伐浣滄柟寮忔槸灏嗘墍鏈?WRITE 璇锋眰鍦ㄥ啓瀹屾垚鍚庢寕鍒颁竴涓摼琛ㄤ笂銆備竴鏃︾湅鍒?
REQ_PREFLUSH 璇锋眰锛屾垜浠氨鎶婅閾捐〃鎷兼帴杩涜姹傦紝寰?FLUSH 璇锋眰瀹屾垚鍚庯紝
鎴戜滑璁板綍鎵€鏈?WRITE 浠ュ強闅忓悗鐨?FLUSH銆傚彧鏈夊湪 REQ_PREFLUSH 鍙戣捣鏃?
宸茬粡瀹屾垚鐨?WRITE 鎵嶄細琚寜椤哄簭鍔犲叆锛屼互妯℃嫙鏂數鎯呭喌涓嬬殑鏈€鍧忓満鏅€?
鑰冭檻涓嬮潰杩欎釜渚嬪瓙锛圵 琛ㄧず鍐欏叆锛孋 琛ㄧず瀹屾垚锛夛細

	W1,W2,W3,C3,C2,Wflush,C1,Cflush

鏃ュ織涓樉绀虹殑灏嗘槸锛?

	W3,W2,flush,W1....

鍚屾牱锛岃繖涔熸槸涓轰簡妯℃嫙纾佺洏涓婄殑鐪熷疄鎯呭喌锛屼粠鑰岃鎴戜滑鑳藉妫€娴?
鍦ㄦ煇涓壒瀹氭椂鍒诲彂鐢熸柇鐢典細瀵艰嚧鏂囦欢绯荤粺涓嶄竴鑷寸殑鎯呭喌銆?

浠讳綍 REQ_FUA 璇锋眰浼氱粫杩囪鍒锋柊鏈哄埗锛屽苟鍦ㄥ叾瀹屾垚鍚庣珛鍗宠璁板綍锛?
鍥犱负杩欎簺璇锋眰鏄剧劧浼氱粫杩囪澶囩紦瀛樸€?

浠讳綍 REQ_OP_DISCARD 璇锋眰閮借褰撲綔 WRITE 璇锋眰澶勭悊銆傚惁鍒欐垜浠氨浼?
鍏堣褰曟墍鏈夌殑 DISCARD 璇锋眰锛岀劧鍚庢槸 WRITE 璇锋眰锛屾渶鍚庢墠鏄?FLUSH
璇锋眰銆傝€冭檻涓嬮潰鐨勪緥瀛愶細

	WRITE block 1, DISCARD block 1, FLUSH

濡傛灉鎴戜滑鎸?DISCARD 瀹屾垚鏃惰褰曪紝閲嶆斁鐪嬭捣鏉ヤ細鏄繖鏍凤細

	DISCARD 1, WRITE 1, FLUSH

杩欎笌瀹為檯鍙戠敓鐨勬儏鍐靛苟涓嶅畬鍏ㄧ浉绗︼紝涔熶細鍦ㄦ棩蹇楅噸鏀句腑琚紡鎺夈€?

## Target interface


i) 鏋勯€犲嚱鏁?

   log-writes <dev_path> <log_dev_path>

   ============= ==============================================
   dev_path	 鎵€鏈?IO 姝ｅ父杞彂鍒扮殑璁惧銆?
   log_dev_path  鏃ュ織鏉＄洰鍐欏叆鍒扮殑璁惧銆?
   ============= ==============================================

ii) 鐘舵€?

    <#logged entries> <highest allocated sector>

    =========================== ========================
    #logged entries	         宸茶褰曠殑鏉＄洰鏁伴噺
    highest allocated sector    宸插垎閰嶇殑鏈€楂樻墖鍖?
    =========================== ========================

iii) 娑堟伅

    mark <description>

	浣犲彲浠ヤ娇鐢?dmsetup message 鍦ㄦ棩蹇椾腑璁剧疆涓€涓换鎰忔爣璁般€?
	渚嬪锛屽亣璁句綘鎯冲湪姣忔鍐欏叆鍚庨兘瀵规枃浠剁郴缁熻繘琛?fsck锛屼絾棣栧厛
	闇€瑕侀噸鏀惧埌 mkfs 浠ョ‘淇濇垜浠?fsck 鐨勫璞℃槸鍚堢悊鐨勶紝浣犲彲浠?
	鍋氱被浼艰繖鏍风殑浜嬫儏
```

	  mkfs.btrfs -f /dev/mapper/log
	  dmsetup message log 0 mark mkfs
	  <run test>

	This would allow you to replay the log up to the mkfs mark and
	then replay from that point on doing the fsck check in the
	interval that you want.

	Every log has a mark at the end labeled "dm-log-writes-end".

```
## Userspace component


鏈変竴涓敤鎴风┖闂村伐鍏峰彲浠ョ敤澶氱鏂瑰紡涓轰綘閲嶆斁鏃ュ織銆?
瀹冨彲浠ュ湪杩欓噷鎵惧埌锛歨ttps://github.com/josefbacik/log-writes

## Example usage


鍋囪浣犳兂娴嬭瘯鏂囦欢绯荤粺涓婄殑 fsync銆備綘浼氬仛绫讳技杩欐牱鐨勪簨鎯?
```

  TABLE="0 $(blockdev --getsz /dev/sdb) log-writes /dev/sdb /dev/sdc"
  dmsetup create log --table "$TABLE"
  mkfs.btrfs -f /dev/mapper/log
  dmsetup message log 0 mark mkfs

  mount /dev/mapper/log /mnt/btrfs-test
  <some test that does fsync at the end>
  dmsetup message log 0 mark fsync
  md5sum /mnt/btrfs-test/foo
  umount /mnt/btrfs-test

  dmsetup remove log
  replay-log --log /dev/sdc --replay /dev/sdb --end-mark fsync
  mount /dev/sdb /mnt/btrfs-test
  md5sum /mnt/btrfs-test/foo
  <verify md5sum's are correct>

  Another option is to do a complicated file system operation and verify the file
  system is consistent during the entire operation.  You could do this with:

  TABLE="0 $(blockdev --getsz /dev/sdb) log-writes /dev/sdb /dev/sdc"
  dmsetup create log --table "$TABLE"
  mkfs.btrfs -f /dev/mapper/log
  dmsetup message log 0 mark mkfs

  mount /dev/mapper/log /mnt/btrfs-test
  <fsstress to dirty the fs>
  btrfs filesystem balance /mnt/btrfs-test
  umount /mnt/btrfs-test
  dmsetup remove log

  replay-log --log /dev/sdc --replay /dev/sdb --end-mark mkfs
  btrfsck /dev/sdb
  replay-log --log /dev/sdc --replay /dev/sdb --start-mark mkfs \
	--fsck "btrfsck /dev/sdb" --check fua

```
瀹冧細涓€鐩撮噸鏀炬棩蹇楃洿鍒伴亣鍒颁竴涓?FUA 璇锋眰锛岃繍琛?fsck 鍛戒护锛屽鏋?
fsck 閫氳繃锛屽垯閲嶆斁鍒颁笅涓€涓?FUA锛岀洿鍒板叏閮ㄥ畬鎴愭垨 fsck 鍛戒护寮傚父閫€鍑恒€?
