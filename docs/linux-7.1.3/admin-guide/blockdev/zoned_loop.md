## 鍒嗗尯寰幆锛坺loop锛夊潡璁惧


 1) 姒傝堪
 2) 鍒涘缓鍒嗗尯璁惧
 3) 鍒犻櫎鍒嗗尯璁惧
 4) 绀轰緥


### 1) 姒傝堪


鍒嗗尯寰幆鍧楄澶囬┍鍔紙zloop锛夊厑璁哥敤鎴峰垱寤哄垎鍖哄潡璁惧锛屼负姣忎釜鍖轰娇鐢ㄤ竴涓父瑙勬枃浠朵綔涓哄悗绔瓨鍌ㄣ€傝椹卞姩涓嶇洿鎺ユ帶鍒朵换浣曠‖浠讹紝鑰屾槸閫氳繃瀵规枃浠剁郴缁熶腑鐨勫父瑙勬枃浠舵墽琛岃銆佸啓鍜屾埅鏂搷浣滄潵妯℃嫙鍒嗗尯鍧楄澶囥€?
浣跨敤 zloop 鍙互鍒涘缓鍏锋湁鍙厤缃閲忋€佸尯澶у皬浠ュ強甯歌鍖烘暟閲忕殑鍒嗗尯鍧楄澶囥€傝澶囨瘡涓尯鐨勫瓨鍌ㄩ兘浣跨敤涓€涓父瑙勬枃浠跺疄鐜帮紝鍏舵渶澶уぇ灏忕瓑浜庡尯澶у皬銆備綔涓哄父瑙勫尯鍚庣鐨勬枃浠跺ぇ灏忓缁堢瓑浜庡尯澶у皬銆備綔涓洪『搴忓啓鍖哄悗绔殑鏂囦欢澶у皬鍒欐寚绀哄凡椤哄簭鍐欏叆璇ユ枃浠剁殑鏁版嵁閲忥紝涔熷氨鏄锛岃鏂囦欢鐨勫ぇ灏忕洿鎺ユ寚绀轰簡鍖虹殑鍐欐寚閽堜綅缃€?
閲嶇疆涓€涓『搴忓啓鍖烘椂锛屽叾鍚庣鏂囦欢澶у皬浼氳鎴柇涓洪浂銆傜浉鍙嶏紝瀵逛簬鍖虹殑 finish 鎿嶄綔锛屽悗绔枃浠朵細琚埅鏂埌鍖哄ぇ灏忋€傜敱姝わ紝鍒涘缓鐨?zloop 鍒嗗尯鍧楄澶囩殑鏈€澶у閲忓彲浠ラ厤缃负澶т簬鍚庣鏂囦欢绯荤粺涓婂彲鐢ㄧ殑瀛樺偍绌洪棿銆傚綋鐒讹紝瀵逛簬杩欑閰嶇疆锛屽啓鍏ョ殑鏁版嵁閲忚秴杩囧悗绔枃浠剁郴缁熶笂鍙敤瀛樺偍绌洪棿鏃朵細瀵艰嚧鍐欓敊璇€?
鍒嗗尯寰幆鍧楄澶囬┍鍔ㄥ疄鐜颁簡涓€涓畬鏁寸殑鍖虹姸鎬佽浆鎹㈢姸鎬佹満銆備篃灏辨槸璇达紝鍖哄彲浠ユ槸绌恒€侀殣寮忔墦寮€銆佹樉寮忔墦寮€銆佸叧闂垨宸叉弧銆傚綋鍓嶅疄鐜颁笉鏀寔瀵规渶澶ф墦寮€鍖烘暟鍜屾椿鍔ㄥ尯鏁版柦鍔犱换浣曢檺鍒躲€?
鍒涘缓鍜屽垹闄?zloop 璁惧涓嶉渶瑕佷换浣曠敤鎴锋€佸伐鍏枫€?

### 2) 鍒涘缓鍒嗗尯璁惧


涓€鏃﹀姞杞戒簡 zloop 妯″潡锛堟垨鑰?zloop 琚紪璇戣繘鍐呮牳锛夛紝灏卞彲浠ヤ娇鐢ㄥ瓧绗﹁澶囨枃浠?/dev/zloop-control 鏉ユ坊鍔犱竴涓?zloop 璁惧銆傝繖鏄€氳繃鐩存帴鍚?/dev/zloop-control 鍐欏叆涓€涓?"add" 鍛戒护鏉ュ畬鎴愮殑銆?

```
	$ modprobe zloop
        $ ls -l /dev/zloop*
        crw-------. 1 root root 10, 123 Jan  6 19:18 /dev/zloop-control

        $ mkdir -p <base directory/<device ID>
        $ echo "add [options]" > /dev/zloop-control
```

鍙敤浜?add 鍛戒护鐨勯€夐」鍙互閫氳繃璇诲彇浠ヤ笅鏂囦欢鍒楀嚭锛?

```
	$ cat /dev/zloop-control
        add id=%d,capacity_mb=%u,zone_size_mb=%u,zone_capacity_mb=%u,conv_zones=%u,max_open_zones=%u,base_dir=%s,nr_queues=%u,queue_depth=%u,buffered_io,zone_append=%u,ordered_zone_append,discard_write_cache
        remove id=%d
```

鏇磋缁嗗湴璇达紝鍙笌 "add" 鍛戒护涓€璧蜂娇鐢ㄧ殑閫夐」濡備笅銆?
====================   =========================================================
id                    璁惧鍙凤紙鍗?/dev/zloopX 涓殑 X锛夈€?                      榛樿鍊硷細鑷姩鍒嗛厤銆?capacity_mb           璁惧鎬诲閲忥紝鍗曚綅涓?MiB銆傝鍊兼€绘槸鍚戜笂鍙栨暣鍒?                      鍖哄ぇ灏忔渶鎺ヨ繎鐨勬洿楂樺€嶆暟銆?                      榛樿鍊硷細16384 MiB锛?6 GiB锛夈€?zone_size_mb          璁惧鍖哄ぇ灏忥紝鍗曚綅涓?MiB銆傞粯璁ゅ€硷細256 MiB銆?zone_capacity_mb      璁惧鍖哄閲忥紙蹇呴』濮嬬粓绛変簬鎴栧皬浜庡尯澶у皬锛夈€傞粯璁ゅ€硷細鍖哄ぇ灏忋€?conv_zones            浠庢墖鍖?0 寮€濮嬬殑甯歌鍖烘€绘暟銆?                      榛樿鍊硷細8
max_open_zones        鎵€闇€鐨勬墦寮€椤哄簭鍐欏尯鐨勬渶澶ф暟閲忥紙0 琛ㄧず鏃犻檺鍒讹級銆?                      榛樿鍊硷細0
base_dir              鐢ㄤ簬鍒涘缓鍖呭惈璇ヨ澶囧尯鏂囦欢鐨勭洰褰曠殑鍩虹鐩綍璺緞銆?                      榛樿鍊?/var/local/zloop銆?                      鍖呭惈鍖烘枃浠剁殑璁惧鐩綍鎬绘槸浠ヨ澶?ID 鍛藉悕銆備緥濡?                      /dev/zloop0 鐨勯粯璁ゅ尯鏂囦欢鐩綍涓?/var/local/zloop/0銆?nr_queues             鍒嗗尯鍧楄澶囩殑 I/O 闃熷垪鏁伴噺銆傝鍊兼€绘槸鍙楀湪绾?                      CPU 鏁伴噺鐨勪笂闄愮害鏉熴€?                      榛樿鍊硷細1
queue_depth           姣忎釜 I/O 闃熷垪鐨勬渶澶?I/O 闃熷垪娣卞害銆?                      榛樿鍊硷細64
buffered_io           鎵ц缂撳啿 I/O 鑰岄潪鐩存帴 I/O锛堥粯璁ゅ€硷細false锛夈€?zone_append           鍚敤鎴栫鐢?zloop 璁惧鐨勫師鐢?zone append 鏀寔銆?                      榛樿鍊硷細1锛堝惎鐢級銆?                      鑻ョ鐢ㄤ簡鍘熺敓 zone append 鏀寔锛屽潡灞傚皢浣跨敤甯歌鍐?                      鎿嶄綔鏉ユā鎷熻鎿嶄綔銆?ordered_zone_append   鍚敤 zloop 瀵?zone append 閲嶆帓搴忕殑缂撹В銆?                      榛樿鍊硷細绂佺敤銆?                      杩欏浜庢祴璇曟枃浠剁郴缁熸枃浠舵暟鎹槧灏勶紙extent锛夊緢鏈夌敤锛?                      鍥犱负鍚敤鍚庯紝鍙互鏄捐憲鍑忓皯鏂囦欢鏁版嵁鏄犲皠鎵€闇€鐨?                      鏁版嵁 extent 鏁伴噺銆?discard_write_cache   璁惧琚Щ闄ゆ椂锛岄€氳繃灏嗘瘡涓尯鏂囦欢鎴柇鍒颁笂涓€娆″埛鏂?                      鎿嶄綔鏈熼棿璁板綍鐨勫ぇ灏忥紝涓㈠純鎵€鏈夋湭閫氳繃鍒锋柊鎿嶄綔
                      鏄惧紡鎸佷箙鍖栫殑鏁版嵁銆傝繖妯℃嫙浜嗘湭鎻愪氦鏁版嵁涓㈠け鐨?                      鎺夌數浜嬩欢銆?====================   =========================================================


### 3) 鍒犻櫎鍒嗗尯璁惧


鍒犻櫎涓€涓湭浣跨敤鐨勫垎鍖哄惊鐜潡璁惧鏄€氳繃鍙戝嚭 "remove" 鍛戒护鏉ュ畬鎴愮殑銆?

```
        $ echo "remove id=X" > /dev/zloop-control
```

remove 鍛戒护娌℃湁浠讳綍閫夐」銆?
琚Щ闄ょ殑鍒嗗尯璁惧鍙互鍦ㄤ笉鏀瑰彉璁惧鍖虹姸鎬佺殑鎯呭喌涓嬪啀娆℃坊鍔狅細璁惧鍖轰細琚仮澶嶅埌璁惧琚Щ闄や箣鍓嶇殑鐘舵€併€傚湪璁惧琚Щ闄や箣鍚庡啀娆℃坊鍔犲垎鍖鸿澶囨椂锛屽繀椤诲缁堜娇鐢ㄤ笌棣栨娣诲姞璁惧鏃剁浉鍚岀殑閰嶇疆銆傚鏋滄娴嬪埌鍖洪厤缃彂鐢熷彉鍖栵紝灏嗚繑鍥為敊璇紝骞朵笖涓嶄細鍒涘缓鍒嗗尯璁惧銆?
瑕佸交搴曞垹闄や竴涓垎鍖鸿澶囷紝鍦ㄦ墽琛?remove 鎿嶄綔鍚庯紝蹇呴』鍒犻櫎鍖呭惈璇ヨ澶囧悇鍖虹殑鍚庣鏂囦欢鐨勮澶囧熀纭€鐩綍銆?

### 4) 绀轰緥


浠ヤ笅鍛戒护搴忓垪鍒涘缓浜嗕竴涓?2GB 鐨勫垎鍖鸿澶囷紝鍏跺尯澶у皬涓?64


```
        $ modprobe zloop
        $ mkdir -p /var/local/zloop/0
        $ echo "add capacity_mb=2048,zone_size_mb=64,zone_capacity_mb=63" > /dev/zloop-control
```

瀵逛簬鎵€鍒涘缓鐨勮澶囷紙/dev/zloop0锛夛紝鍏跺尯鍚庣鏂囦欢鍏ㄩ儴鍒涘缓涓?

```
        $ ls -l /var/local/zloop/0
        total 0
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000000
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000001
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000002
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000003
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000004
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000005
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000006
        -rw-------. 1 root root 67108864 Jan  6 22:23 cnv-000007
        -rw-------. 1 root root        0 Jan  6 22:23 seq-000008
        -rw-------. 1 root root        0 Jan  6 22:23 seq-000009
        ...
```

```
        $ lsblk -z
        NAME   ZONED        ZONE-SZ ZONE-NR ZONE-AMAX ZONE-OMAX ZONE-APP ZONE-WGRAN
        zloop0 host-managed     64M      32         0         0       1M         4K
        $ blkzone report /dev/zloop0
          start: 0x000000000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x000020000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x000040000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x000060000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x000080000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x0000a0000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x0000c0000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x0000e0000, len 0x020000, cap 0x020000, wptr 0x000000 reset:0 non-seq:0, zcond: 0(nw) [type: 1(CONVENTIONAL)]
          start: 0x000100000, len 0x020000, cap 0x01f800, wptr 0x000000 reset:0 non-seq:0, zcond: 1(em) [type: 2(SEQ_WRITE_REQUIRED)]
          start: 0x000120000, len 0x020000, cap 0x01f800, wptr 0x000000 reset:0 non-seq:0, zcond: 1(em) [type: 2(SEQ_WRITE_REQUIRED)]
          ...
```

```
        $ echo "remove id=0" > /dev/zloop-control
```

琚Щ闄ょ殑璁惧鍙互浣跨敤涓庨娆″垱寤鸿澶囨椂鐩稿悓鐨?"add" 鍛戒护鍐嶆娣诲姞銆傝褰诲簳鍒犻櫎涓€涓垎鍖鸿澶囷紝鍏跺悗绔枃浠?

```
        $ rm -r /var/local/zloop/0
```
