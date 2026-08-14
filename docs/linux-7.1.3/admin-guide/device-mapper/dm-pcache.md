
## dm-pcache 鈥?鎸佷箙鍖栫紦瀛橈紙Persistent Cache锛?


**浣滆€咃細Dongsheng Yang <dongsheng.yang@linux.dev>**

鏈枃妗ｆ弿杩?**dm-pcache**锛岃繖鏄竴涓?Device-Mapper 鐩爣锛屽畠璁╀竴涓彲鎸夊瓧鑺傚鍧€鐨?**DAX**锛堟寔涔呭唴瀛橈紝鈥減mem鈥濓級鍖哄煙鍏呭綋浣嶄簬杈冩參鍧楄澶囦箣鍓嶇殑銆侀珮鎬ц兘涓斿穿婧冩寔涔呭寲鐨勭紦瀛樸€傜浉鍏充唬鐮佷綅浜?`drivers/md/dm-pcache/`銆?

## 鐗规€ч€熻


- **鍥炲啓锛坵rite-back锛?* 缂撳瓨锛堢洰鍓嶅敮涓€鏀寔鐨勬ā寮忥級銆?
- 鍦?pmem 璁惧涓婂垎閰嶇殑 **16 MiB 娈碉紙segment锛?*銆?
- **鏁版嵁 CRC32** 鏍￠獙锛堝彲閫夛紝鎸夌紦瀛樿缃級銆?
- 宕╂簝瀹夊叏锛氭瘡涓厓鏁版嵁缁撴瀯閮藉仛浜嗗弻浠藉鍒讹紙`PCACHE_META_INDEX_MAX == 2`锛夛紝骞朵娇鐢?CRC 鍔犲簭鍒楀彿杩涜淇濇姢銆?
- **澶氭爲绱㈠紩**锛堟寜閫昏緫鍦板潃鍒嗙墖鐨勭储寮曟爲锛夛紝浠ヨ幏寰楄緝楂樼殑 PMem 骞惰搴?
- 绾?**DAX 璺緞** I/O 鈥斺€?娌℃湁棰濆鐨?BIO 寰€杩?
- **鏃ュ織缁撴瀯鍥炲啓锛坙og-structured write-back锛?*锛屼繚鎸佸悗绔穿婧冧竴鑷存€?


## 鏋勯€犲嚱鏁?


```

    pcache <cache_dev> <backing_dev> [<number_of_optional_arguments> <cache_mode writeback> <data_crc true|false>]

```
=========================  ====================================================
`cache_dev`               Any DAX-capable block device (`/dev/pmem0`鈥?.
                            All metadata **and** cached blocks are stored here.

`backing_dev`             The slow block device to be cached.

`cache_mode`              Optional, Only `writeback` is accepted at the
                            moment.

`data_crc`                Optional, default to `false`

                            - `true`  鈥?store CRC32 for every cached entry
			      and verify on reads
                            - `false` 鈥?skip CRC (faster)
=========================  ====================================================

### 绀轰緥



   dmsetup create pcache_sdb --table \
     "0 $(blockdev --getsz /dev/sdb) pcache /dev/pmem0 /dev/sdb 4 cache_mode writeback data_crc true"

棣栨浣跨敤鏌愪釜 pmem 璁惧鏃讹紝dm-pcache 浼氳嚜鍔ㄦ牸寮忓寲瀹冿紙瓒呯骇鍧椼€乧ache_info 绛夛級銆?


## 鐘舵€佽


`dmsetup status <device>`锛坄STATUSTYPE_INFO`锛変細鎵撳嵃锛?

```

   <sb_flags> <seg_total> <cache_segs> <segs_used> \
   <gc_percent> <cache_flags> \
   <key_head_seg>:<key_head_off> \
   <dirty_tail_seg>:<dirty_tail_off> \
   <key_tail_seg>:<key_tail_off>

```
### 瀛楁鍚箟


===============================  =============================================
`sb_flags`                     Super-block flags (e.g. endian marker).

`seg_total`                    Number of physical **pmem** segments.

`cache_segs`                   Number of segments used for cache.

`segs_used`                    Segments currently allocated (bitmap weight).

`gc_percent`                   Current GC high-water mark (0-90).

`cache_flags`                  Bit 0 鈥?DATA_CRC enabled
                                 Bit 1 鈥?INIT_DONE (cache initialised)
                                 Bits 2-5 鈥?cache mode (0 == WB).

`key_head`                     Where new key-sets are being written.

`dirty_tail`                   First dirty key-set that still needs
                                 write-back to the backing device.

`key_tail`                     First key-set that may be reclaimed by GC.
===============================  =============================================


## 娑堟伅


**鏇存敼 GC 瑙﹀彂闃堝€?*

```

   dmsetup message <dev> 0 gc_percent <0-90>


```
## 宸ヤ綔鍘熺悊


### 瀛愯澶?


====================  =========================================================
backing_dev             Any block device (SSD/HDD/loop/LVM, etc.).
cache_dev               DAX device; must expose direct-access memory.
====================  =========================================================

### 娈典笌閿泦鍚堬紙key-set锛?


- pmem 绌洪棿琚垝鍒嗕负 **16 MiB 娈碉紙segment锛?*銆?
- 姣忔鍐欏叆浼氫粠娈靛唴姣忎釜 CPU 鐨?**data_head** 鍒嗛厤绌洪棿銆?
- 涓€涓?**cache-key锛堢紦瀛橀敭锛?* 璁板綍浜嗘簮璁惧涓婄殑涓€娈甸€昏緫鑼冨洿锛屼互鍙婂畠鍦?pmem 涓殑浣嶇疆锛堟 + 鍋忕Щ + 浠ｏ紙generation锛夛級銆?
- 128 涓敭缁勬垚涓€涓?**key-set锛坘set锛?*锛沰set 鍦?pmem 涓『搴忓啓鍏ワ紝骞朵笖鑷韩鏄穿婧冨畨鍏ㄧ殑锛圕RC锛夈€?
- 杩欎竴瀵?**(key_tail, dirty_tail)** 鐣屽畾浜嗗共鍑€/鑴忎互鍙婂瓨娲?姝讳骸 kset 鐨勮竟鐣屻€?

### 鍥炲啓


鑴忛敭琚帓鍏ヤ竴妫垫爲涓紱涓€涓悗鍙板伐浣滅嚎绋嬪皢鏁版嵁澶嶅埗鍥?backing_dev锛屽苟鎺ㄨ繘 **dirty_tail**銆傛潵鑷笂灞傜殑 FLUSH/FUA bio 浼氬己鍒剁珛鍗虫彁浜ゅ厓鏁版嵁銆?

### 鍨冨溇鍥炴敹


褰?`segs_used >= seg_total * gc_percent / 100` 鏃讹紝GC 鍚姩銆傚畠浠?**key_tail** 寮€濮嬮亶鍘嗭紝閲婃斁鍏朵腑姣忎釜閿兘宸插け鏁堢殑娈碉紝骞舵帹杩?**key_tail**銆?

### CRC 鏍￠獙


鑻?`data_crc 宸插惎鐢╜锛宒m-pcache 浼氬湪姣忔鎻掑叆鏃朵负姣忎釜缂撳瓨鏁版嵁鑼冨洿璁＄畻 CRC32锛屽苟灏嗗叾瀛樺偍鍦ㄤ粙璐ㄤ笂鐨勯敭涓€傝鍙栨椂浼氬湪澶嶅埗鍒拌皟鐢ㄨ€呬箣鍓嶉獙璇?CRC銆?


## 鏁呴殰澶勭悊


- **pmem 浠嬭川閿欒** 鈥斺€?鎵€鏈夊厓鏁版嵁鍓湰閮介€氳繃 `copy_mc_to_kernel` 璇诲彇锛涗笉鍙籂姝ｇ殑閿欒浼氳褰曟棩蹇楀苟涓鍒濆鍖栥€?
- **缂撳瓨宸叉弧** 鈥斺€?濡傛灉鎵句笉鍒扮┖闂叉锛屽啓鍏ヨ繑鍥?`-EBUSY`锛沝m-pcache 浼氬湪鍐呴儴閲嶈瘯锛堣姹傚欢杩燂級銆?
- **绯荤粺宕╂簝** 鈥斺€?鍦ㄦ寕杞芥椂锛岄┍鍔ㄤ細浠?**key_tail** 閲嶆斁 kset 浠ラ噸寤哄唴瀛樹腑鐨勬爲锛涙瘡涓鐨勪唬锛坓eneration锛夊彲闃叉鍑虹幇鎮┖锛坲se-after-free锛夐敭銆?


## 闄愬埗涓?TODO


- 浠?**鍥炲啓** 妯″紡锛涘叾瀹冩ā寮忓湪璁″垝涓€?
- 浠?FIFO 缂撳瓨澶辨晥锛涘叾瀹冿紙LRU銆丄RC鈥︹€︼級鍦ㄨ鍒掍腑銆?
- 鐩墠涓嶆敮鎸佽〃閲嶈浇锛坱able reload锛夈€?
- 涓㈠純锛坉iscard锛夊湪璁″垝涓€?


## 绀轰緥宸ヤ綔娴?



   # 1.  鍒涘缓璁惧
   dmsetup create pcache_sdb --table \
     "0 $(blockdev --getsz /dev/sdb) pcache /dev/pmem0 /dev/sdb 4 cache_mode writeback data_crc true"

   # 2.  鍦ㄥ叾涓婂垱寤烘枃浠剁郴缁?
   mkfs.ext4 /dev/mapper/pcache_sdb
   mount /dev/mapper/pcache_sdb /mnt

   # 3.  灏?GC 闃堝€艰皟鏁翠负 80%
   dmsetup message pcache_sdb 0 gc_percent 80

   # 4.  瑙傚療鐘舵€?
   watch -n1 'dmsetup status pcache_sdb'

   # 5.  鍏抽棴
   umount /mnt
   dmsetup remove pcache_sdb


`dm-pcache` 浠嶅湪绉瀬寮€鍙戜腑锛涢潪甯告杩庡弽棣堛€乥ug 鎶ュ憡鍜岃ˉ涓侊紒
