## dm-delay


Device-Mapper 鐨?"delay" 鐩爣浼氬欢杩熻鍜?鎴栧啓
鍜?鎴栧埛鏂版搷浣滐紝骞跺彲閫夋嫨鎬у湴灏嗗畠浠槧灏勫埌涓嶅悓鐨勮澶囥€?

```
<device> <offset> <delay> [<write_device> <write_offset> <write_delay>
			       [<flush_device> <flush_offset> <flush_delay>]]

```
Table 琛屽繀椤诲寘鍚?3銆? 鎴?9 涓弬鏁帮細

3锛氬璁惧涓婄殑璇汇€佸啓鍜屽埛鏂版搷浣滃簲鐢ㄥ亸绉诲拰寤惰繜

6锛氬璁惧搴旂敤鍋忕Щ鍜屽欢杩燂紝鍚屾椂搴旂敤 write_offset 鍜?write_delay
   瀵瑰彲閫夌殑涓嶅悓 write_device 涓婄殑鍐欏拰鍒锋柊鎿嶄綔锛?
   浣跨敤鍙€夌殑涓嶅悓鎵囧尯鍋忕Щ

9锛氫笌 6 涓弬鏁扮浉鍚岋紝棰濆鏄惧紡瀹氫箟 flush_offset 鍜?flush_delay
   浣嶄簬/浣跨敤鍙€夌殑涓嶅悓 flush_device/flush_offset銆?

鍋忕Щ浠ユ墖鍖轰负鍗曚綅鎸囧畾銆?

寤惰繜浠ユ绉掍负鍗曚綅鎸囧畾銆?


## 绀轰緥鑴氭湰


```
	#!/bin/sh
	#
	# Create mapped device named "delayed" delaying read, write and flush operations for 500ms.
	#
	dmsetup create delayed --table  "0 `blockdev --getsz $1` delay $1 0 500"

```
```
	#!/bin/sh
	#
	# Create mapped device delaying write and flush operations for 400ms and
	# splitting reads to device $1 but writes and flushes to different device $2
	# to different offsets of 2048 and 4096 sectors respectively.
	#
	dmsetup create delayed --table "0 `blockdev --getsz $1` delay $1 2048 0 $2 4096 400"

```
```
	#!/bin/sh
	#
	# Create mapped device delaying reads for 50ms, writes for 100ms and flushes for 333ms
	# onto the same backing device at offset 0 sectors.
	#
	dmsetup create delayed --table "0 `blockdev --getsz $1` delay $1 0 50 $2 0 100 $1 0 333"

```
