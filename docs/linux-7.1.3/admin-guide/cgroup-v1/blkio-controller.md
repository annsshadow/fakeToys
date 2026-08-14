## 鍧?IO 鎺у埗鍣?


## 姒傝堪

cgroup 瀛愮郴缁?"blkio" 瀹炵幇浜嗗潡 IO 鎺у埗鍣ㄣ€傚湪瀛樺偍灞傜骇涓紝鏃犺鏄彾瀛?
鑺傜偣杩樻槸涓棿鑺傜偣锛屼技涔庨兘闇€瑕佸悇绉嶇被鍨嬬殑 IO 鎺у埗绛栫暐锛堜緥濡傛寜姣斾緥甯﹀銆?
鏈€澶у甫瀹斤級銆傚叾璁″垝鏄 blkio 鎺у埗鍣ㄤ娇鐢ㄧ浉鍚岀殑鍩轰簬 cgroup 鐨勭鐞嗘帴鍙ｏ紝
骞舵牴鎹敤鎴烽€夐」鍦ㄥ悗鍙板垏鎹?IO 绛栫暐銆?

涓€绉?IO 鎺у埗绛栫暐鏄妭娴侊紙throttling锛夌瓥鐣ワ紝鍙敤浜庡湪璁惧涓婃寚瀹?IO 閫熺巼
涓婇檺銆傝绛栫暐鍦ㄩ€氱敤鍧楀眰涓疄鐜帮紝鏃㈠彲鐢ㄤ簬鍙跺瓙鑺傜偣锛屼篃鍙敤浜庤澶囨槧灏勫櫒
绛夋洿楂樼骇鍒殑閫昏緫璁惧銆?

## HOWTO


### 鑺傛祦/涓婇檺绛栫暐

```

	CONFIG_BLK_CGROUP=y

```
```

	CONFIG_BLK_DEV_THROTTLING=y

```
```

        mount -t cgroup -o blkio none /sys/fs/cgroup/blkio

```
涓?root 缁勫湪鎸囧畾璁惧涓婃寚瀹氬甫瀹介€熺巼銆傚叾鏍煎紡
```

        echo "8:16  1048576" > /sys/fs/cgroup/blkio/blkio.throttle.read_bps_device

```
杩欏皢瀵硅澶囦富/娆¤澶囧彿涓?8:16 涓婂彂鐢熺殑璇绘搷浣滄柦鍔?1MB/绉掔殑闄愬埗銆?

```

        # dd iflag=direct if=/mnt/common/zerofile of=/dev/null bs=4K count=1024
        1024+0 records in
        1024+0 records out
        4194304 bytes (4.2 MB) copied, 4.0001 s, 1.0 MB/s

```
鍐欏叆鐨勯檺鍒跺彲閫氳繃 blkio.throttle.write_bps_device 鏂囦欢璁剧疆銆?

## 灞傜骇寮?cgroups


鑺傛祦瀹炵幇浜嗗眰绾ф敮鎸侊紱涓嶈繃锛岃妭娴佺殑灞傜骇鏀寔浠呭湪 cgroup 渚у惎鐢ㄤ簡
"sane_behavior" 鏃舵墠鐢熸晥锛岃€岃閫夐」鐩墠浠嶆槸寮€鍙戦€夐」锛屽皻鏈叕寮€鎻愪緵銆?

```

			root
			/  \
		     test1 test2
			|
		     test3

```
鍚敤浜?"sane_behavior" 鐨勮妭娴佽兘姝ｇ‘澶勭悊灞傜骇鍏崇郴銆傚浜庤妭娴侊紝鎵€鏈夐檺鍒?
閮戒綔鐢ㄤ簬鏁翠釜瀛愭爲锛岃€屾墍鏈夌粺璁′俊鎭粎閽堝璇?cgroup 鍐呬换鍔＄洿鎺ョ敓鎴愮殑 IO
鐨勬湰鍦版暟鎹€?

鏈惎鐢?cgroup 渚?"sane_behavior" 鐨勮妭娴侊紝瀹為檯涓婁細鎶婃墍鏈夌粍瑙嗕负鍚屼竴
灞傜骇锛屽氨濂藉儚鐪嬭捣鏉ュ儚涓嬮潰鐨?
```

				pivot
			     /  /   \  \
			root  test1 test2  test3

```
## 鍚勭鐢ㄦ埛鍙鐨勯厤缃€夐」


  CONFIG_BLK_CGROUP
	  鍧?IO 鎺у埗鍣ㄣ€?

  CONFIG_BFQ_CGROUP_DEBUG
	  璋冭瘯杈呭姪銆傝嫢鍚敤姝ら€夐」锛宑group 涓細鍑虹幇涓€浜涢澶栫殑缁熻鏂囦欢銆?

  CONFIG_BLK_DEV_THROTTLING
	  鍦ㄥ潡灞傚惎鐢ㄥ潡璁惧鑺傛祦鏀寔銆?

## cgroup 鏂囦欢璇︽儏


### 鎸夋瘮渚嬫潈閲嶇瓥鐣ユ枃浠?


  blkio.bfq.weight
	  鎸囧畾姣忎釜 cgroup 鐨勬潈閲嶃€傝繖鏄缁勫湪鎵€鏈夎澶囦笂鐨勯粯璁ゆ潈閲嶏紝
	  闄ら潪琚瘡璁惧瑙勫垯瑕嗙洊锛堣涓嬮潰鐨?`blkio.bfq.weight_device`锛夈€?

	  褰撳墠鍏佽鐨勬潈閲嶈寖鍥存槸 1 鍒?1000銆傛洿澶氱粏鑺傦紝鍙傝
	  Documentation/block/bfq-iosched.rst銆?

  blkio.bfq.weight_device
	  鎸囧畾姣忎釜 cgroup 姣忚澶囩殑鏉冮噸锛岃鐩栭粯璁ょ粍鏉冮噸銆傛洿澶氱粏鑺傦紝
	  鍙傝 Documentation/block/bfq-iosched.rst銆?

```

	    # echo dev_maj:dev_minor weight > blkio.bfq.weight_device

	  Configure weight=300 on /dev/sdb (8:16) in this cgroup::

	    # echo 8:16 300 > blkio.bfq.weight_device
	    # cat blkio.bfq.weight_device
	    dev     weight
	    8:16    300

	  Configure weight=500 on /dev/sda (8:0) in this cgroup::

	    # echo 8:0 500 > blkio.bfq.weight_device
	    # cat blkio.bfq.weight_device
	    dev     weight
	    8:0     500
	    8:16    300

	  Remove specific weight for /dev/sda in this cgroup::

	    # echo 8:0 0 > blkio.bfq.weight_device
	    # cat blkio.bfq.weight_device
	    dev     weight
	    8:16    300

  blkio.time
	  鎸夎澶囧垎閰嶇粰 cgroup 鐨勭鐩樻椂闂达紝鍗曚綅涓烘绉掋€傚墠涓や釜瀛楁鎸囧畾
	  璁惧鐨勪富璁惧鍙峰拰娆¤澶囧彿锛岀涓変釜瀛楁鎸囧畾鍒嗛厤缁欒缁勭殑纾佺洏
	  鏃堕棿锛堟绉掞級銆?

  blkio.sectors
	  璇ョ粍鍦ㄧ鐩樹笂浼犺緭锛堣鎴栧啓锛夌殑鎵囧尯鏁般€傚墠涓や釜瀛楁鎸囧畾璁惧鐨?
	  涓昏澶囧彿鍜屾璁惧鍙凤紝绗笁涓瓧娈垫寚瀹氳缁勫湪璁惧涓婁紶杈撶殑
	  鎵囧尯鏁般€?

  blkio.io_service_bytes
	  璇ョ粍鍦ㄧ鐩樹笂浼犺緭鐨勫瓧鑺傛暟銆傝繖浜涘瓧鑺傛暟杩涗竴姝ユ寜鎿嶄綔绫诲瀷
	  锛堣鎴栧啓銆佸悓姝ユ垨寮傛锛夊垝鍒嗐€傚墠涓や釜瀛楁鎸囧畾璁惧鐨勪富璁惧鍙?
	  鍜屾璁惧鍙凤紝绗笁涓瓧娈垫寚瀹氭搷浣滅被鍨嬶紝绗洓涓瓧娈垫寚瀹?
	  瀛楄妭鏁般€?

  blkio.io_serviced
	  璇ョ粍鍚戠鐩樺彂鍑虹殑 IO 鏁帮紙bio锛夈€傝繖浜?IO 鏁拌繘涓€姝ユ寜鎿嶄綔绫诲瀷
	  锛堣鎴栧啓銆佸悓姝ユ垨寮傛锛夊垝鍒嗐€傚墠涓や釜瀛楁鎸囧畾璁惧鐨勪富璁惧鍙?
	  鍜屾璁惧鍙凤紝绗笁涓瓧娈垫寚瀹氭搷浣滅被鍨嬶紝绗洓涓瓧娈垫寚瀹?IO 鏁般€?

  blkio.io_service_time
	  璇?cgroup 鎵€鍙戝嚭 IO 浠庤姹傚垎鍙戝埌璇锋眰瀹屾垚鐨勬€昏€楁椂銆傞噰鐢ㄧ撼绉?
	  鍗曚綅浠ヤ究瀵归棯瀛樿澶囦篃鏈夋剰涔夈€傚浜庨槦鍒楁繁搴︿负 1 鐨勮澶囷紝璇ユ椂闂?
	  浠ｈ〃瀹為檯鏈嶅姟鏃堕棿銆傚綋 queue_depth > 1 鏃讹紝杩欎笉鍐嶆垚绔嬶紝鍥犱负璇锋眰
	  鍙兘涔卞簭寰楀埌鏈嶅姟銆傝繖鍙兘瀵艰嚧鏌愪釜缁欏畾 IO 鐨勬湇鍔℃椂闂村寘鍚簡
	  澶氫釜 IO 鐨勬湇鍔℃椂闂达紙褰撲贡搴忔湇鍔℃椂锛夛紝浠庤€屽彲鑳介€犳垚鎬?
	  io_service_time 澶т簬瀹為檯缁忚繃鐨勬椂闂淬€傝鏃堕棿杩涗竴姝ユ寜鎿嶄綔绫诲瀷
	  锛堣鎴栧啓銆佸悓姝ユ垨寮傛锛夊垝鍒嗐€傚墠涓や釜瀛楁鎸囧畾璁惧鐨勪富璁惧鍙?
	  鍜屾璁惧鍙凤紝绗笁涓瓧娈垫寚瀹氭搷浣滅被鍨嬶紝绗洓涓瓧娈垫寚瀹?
	  io_service_time锛堝崟浣嶄负 ns锛夈€?

  blkio.io_wait_time
	  璇?cgroup 鐨?IO 鍦ㄨ皟搴﹀櫒闃熷垪涓瓑寰呮湇鍔＄殑鎬绘椂闂淬€傜敱浜庡畠鏄?
	  鎵€鏈?IO 绱Н鐨?io_wait_time锛屽洜姝ゅ彲鑳藉ぇ浜庢€荤粡杩囨椂闂淬€傚畠涓嶆槸
	  琛￠噺 cgroup 绛夊緟鎬绘椂闂寸殑鎸囨爣锛岃€屾槸琛￠噺鍏跺悇涓?IO 绛夊緟鏃堕棿鐨?
	  鎸囨爣銆傚浜庨槦鍒楁繁搴?> 1 鐨勮澶囷紝璇ユ寚鏍囦笉鍖呭惈 IO 琚垎鍙戝埌
	  璁惧鍚庣洿鍒板疄闄呭緱鍒版湇鍔℃墍绛夊緟鐨勬椂闂达紙鐢变簬璁惧瀵硅姹傜殑閲嶆帓搴忥紝
	  杩欓噷鍙兘瀛樺湪鏃堕棿宸級銆傞噰鐢ㄧ撼绉掑崟浣嶄互渚垮闂瓨璁惧涔熸湁鎰忎箟銆?
	  璇ユ椂闂磋繘涓€姝ユ寜鎿嶄綔绫诲瀷锛堣鎴栧啓銆佸悓姝ユ垨寮傛锛夊垝鍒嗐€傚墠涓や釜
	  瀛楁鎸囧畾璁惧鐨勪富璁惧鍙峰拰娆¤澶囧彿锛岀涓変釜瀛楁鎸囧畾鎿嶄綔绫诲瀷锛?
	  绗洓涓瓧娈垫寚瀹?io_wait_time锛堝崟浣嶄负 ns锛夈€?

  blkio.io_merged
	  鍚堝苟鍒板睘浜庤 cgroup 鐨勮姹備腑鐨?bios/璇锋眰鎬绘暟銆傝繘涓€姝ユ寜
	  鎿嶄綔绫诲瀷锛堣鎴栧啓銆佸悓姝ユ垨寮傛锛夊垝鍒嗐€?

  blkio.io_queued
	  浠绘剰鏃跺埢涓鸿 cgroup 鎺掗槦鐨勬€昏姹傛暟銆傝繘涓€姝ユ寜鎿嶄綔绫诲瀷
	  锛堣鎴栧啓銆佸悓姝ユ垨寮傛锛夊垝鍒嗐€?

  blkio.avg_queue_size
	  浠呭湪 CONFIG_BFQ_CGROUP_DEBUG=y 鏃跺惎鐢ㄧ殑璋冭瘯杈呭姪銆?
	  璇?cgroup 鍦ㄥ叾鏁翠釜瀛樺湪鏈熼棿鐨勫钩鍧囬槦鍒楀ぇ灏忋€傛瘡褰撹 cgroup 鐨?
	  鏌愪釜闃熷垪鑾峰緱涓€涓椂闂寸墖鏃堕噰闆嗛槦鍒楀ぇ灏忔牱鏈€?

  blkio.group_wait_time
	  浠呭湪 CONFIG_BFQ_CGROUP_DEBUG=y 鏃跺惎鐢ㄧ殑璋冭瘯杈呭姪銆?
	  杩欐槸 cgroup 鑷彉涓虹箒蹇欙紙鍗充粠 0 涓姹傛帓闃熷彉涓?1 涓姹傛帓闃燂級
	  璧凤紝鍒颁负鍏舵煇涓槦鍒楄幏寰楁椂闂寸墖鎵€绛夊緟鐨勬椂闂淬€傝繖涓?io_wait_time
	  涓嶅悓锛屽悗鑰呮槸璇?cgroup 涓瘡涓?IO 鍦ㄨ皟搴﹀櫒闃熷垪涓瓑寰呯殑绱Н
	  鎬绘椂闂淬€傞噰鐢ㄧ撼绉掑崟浣嶃€傚鏋滃湪 cgroup 澶勪簬绛夊緟锛堢瓑寰呮椂闂寸墖锛?
	  鐘舵€佹椂璇诲彇璇ョ粺璁★紝鍒欏彧浼氭姤鍛婃埅鑷充笂娆¤幏寰楁椂闂寸墖涓烘绱Н鐨?
	  group_wait_time锛屼笉鍖呭惈褰撳墠鐨勫閲忋€?

  blkio.empty_time
	  浠呭湪 CONFIG_BFQ_CGROUP_DEBUG=y 鏃跺惎鐢ㄧ殑璋冭瘯杈呭姪銆?
	  杩欐槸 cgroup 鍦ㄦ病鏈夊緟澶勭悊璇锋眰涓旀湭琚湇鍔℃椂锛堝嵆涓嶅寘鍚负 cgroup
	  鏌愪釜闃熷垪绌洪棽绛夊緟鐨勪换浣曟椂闂达級鎵€鑺辫垂鐨勬椂闂淬€傞噰鐢ㄧ撼绉掑崟浣嶃€?
	  濡傛灉鍦?cgroup 澶勪簬绌虹姸鎬佹椂璇诲彇璇ョ粺璁★紝鍒欏彧浼氭姤鍛婃埅鑷充笂娆℃湁
	  寰呭鐞嗚姹備负姝㈢疮绉殑 empty_time锛屼笉鍖呭惈褰撳墠鐨勫閲忋€?

  blkio.idle_time
	  浠呭湪 CONFIG_BFQ_CGROUP_DEBUG=y 鏃跺惎鐢ㄧ殑璋冭瘯杈呭姪銆?
	  杩欐槸 IO 璋冨害鍣ㄤ负浜嗘湡寰呮潵鑷叾浠栭槦鍒?cgroup 鐨勬洿濂借姹傝€?
	  涓虹粰瀹?cgroup 绌洪棽绛夊緟鐨勬椂闂淬€傞噰鐢ㄧ撼绉掑崟浣嶃€傚鏋滃湪 cgroup
	  澶勪簬绌洪棽鐘舵€佹椂璇诲彇璇ョ粺璁★紝鍒欏彧浼氭姤鍛婃埅鑷充笂涓┖闂插懆鏈熶负姝?
	  绱Н鐨?idle_time锛屼笉鍖呭惈褰撳墠鐨勫閲忋€?

  blkio.dequeue
	  浠呭湪 CONFIG_BFQ_CGROUP_DEBUG=y 鏃跺惎鐢ㄧ殑璋冭瘯杈呭姪銆傚畠鎻愪緵
	  鍏充簬涓€涓粍浠庤澶囩殑鏈嶅姟鏍戜腑鍑洪槦娆℃暟鐨勭粺璁°€傚墠涓や釜瀛楁鎸囧畾
	  璁惧鐨勪富璁惧鍙峰拰娆¤澶囧彿锛岀涓変釜瀛楁鎸囧畾涓€涓粍浠庣壒瀹氳澶?
	  鍑洪槦鐨勬鏁般€?

  blkio.*_recursive
	  鍚勭缁熻鐨勯€掑綊鐗堟湰銆傝繖浜涙枃浠舵樉绀虹殑淇℃伅涓庡叾闈為€掑綊瀵瑰簲椤?
	  鐩稿悓锛屼絾鍖呭惈鏉ヨ嚜鎵€鏈夊悗浠?cgroup 鐨勭粺璁°€?

```
### 鑺傛祦/涓婇檺绛栫暐鏂囦欢

  blkio.throttle.read_bps_device
	  鎸囧畾浠庤澶囪鍙栭€熺巼鐨勪笂闄愩€侷O 閫熺巼浠ュ瓧鑺?绉掍负鍗曚綅鎸囧畾銆傝鍒?
	  鎸夎澶囧垝鍒嗐€傛牸寮忓涓?
```

	    echo "<major>:<minor>  <rate_bytes_per_second>" > /cgrp/blkio.throttle.read_bps_device

  blkio.throttle.write_bps_device
	  鎸囧畾鍚戣澶囧啓鍏ラ€熺巼鐨勪笂闄愩€侷O 閫熺巼浠ュ瓧鑺?绉掍负鍗曚綅鎸囧畾銆傝鍒?
	  鎸夎澶囧垝鍒嗐€傛牸寮忓涓?:

	    echo "<major>:<minor>  <rate_bytes_per_second>" > /cgrp/blkio.throttle.write_bps_device

  blkio.throttle.read_iops_device
	  鎸囧畾浠庤澶囪鍙栭€熺巼鐨勪笂闄愩€侷O 閫熺巼浠?IO/绉掍负鍗曚綅鎸囧畾銆傝鍒?
	  鎸夎澶囧垝鍒嗐€傛牸寮忓涓?:

	   echo "<major>:<minor>  <rate_io_per_second>" > /cgrp/blkio.throttle.read_iops_device

  blkio.throttle.write_iops_device
	  鎸囧畾鍚戣澶囧啓鍏ラ€熺巼鐨勪笂闄愩€侷O 閫熺巼浠?IO/绉掍负鍗曚綅鎸囧畾銆傝鍒?
	  鎸夎澶囧垝鍒嗐€傛牸寮忓涓?:

	    echo "<major>:<minor>  <rate_io_per_second>" > /cgrp/blkio.throttle.write_iops_device

          娉ㄦ剰锛氬鏋滀负鏌愪釜璁惧鍚屾椂鎸囧畾浜?BW 鍜?IOPS 瑙勫垯锛屽垯璇?IO 鍙?
          杩欎袱绉嶇害鏉熺殑闄愬埗銆?

  blkio.throttle.io_serviced
	  璇ョ粍鍚戠鐩樺彂鍑虹殑 IO 鏁帮紙bio锛夈€傝繖浜?IO 鏁拌繘涓€姝ユ寜鎿嶄綔绫诲瀷
	  锛堣鎴栧啓銆佸悓姝ユ垨寮傛锛夊垝鍒嗐€傚墠涓や釜瀛楁鎸囧畾璁惧鐨勪富璁惧鍙?
	  鍜屾璁惧鍙凤紝绗笁涓瓧娈垫寚瀹氭搷浣滅被鍨嬶紝绗洓涓瓧娈垫寚瀹?IO 鏁般€?

  blkio.throttle.io_service_bytes
	  璇ョ粍鍦ㄧ鐩樹笂浼犺緭鐨勫瓧鑺傛暟銆傝繖浜涘瓧鑺傛暟杩涗竴姝ユ寜鎿嶄綔绫诲瀷
	  锛堣鎴栧啓銆佸悓姝ユ垨寮傛锛夊垝鍒嗐€傚墠涓や釜瀛楁鎸囧畾璁惧鐨勪富璁惧鍙?
	  鍜屾璁惧鍙凤紝绗笁涓瓧娈垫寚瀹氭搷浣滅被鍨嬶紝绗洓涓瓧娈垫寚瀹氬瓧鑺傛暟銆?

```
### 鍚勭瓥鐣ヤ箣闂撮€氱敤鐨勬枃浠?

  blkio.reset_stats
	  鍚戣鏂囦欢鍐欏叆涓€涓?int 灏嗛噸缃 cgroup 鐨勬墍鏈夌粺璁′俊鎭€?

