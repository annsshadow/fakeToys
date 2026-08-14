## 宓屽叆寮忚澶囧懡浠よ鍒嗗尯瑙ｆ瀽


鈥渂lkdevparts鈥濆懡浠よ閫夐」娣诲姞浜嗗浠庡唴鏍稿懡浠よ璇诲彇鍧楄澶囧垎鍖鸿〃鐨勬敮鎸併€?
瀹冮€氬父鐢ㄤ簬鍥哄畾鍧楋紙eMMC锛夊祵鍏ュ紡璁惧銆傚畠娌℃湁 MBR锛屽洜姝よ妭鐪佸瓨鍌ㄧ┖闂淬€傚紩瀵煎姞杞?绋嬪簭鍙互閫氳繃鍧楄澶囦笂鏁版嵁鐨勭粷瀵瑰湴鍧€杞绘澗璁块棶銆傜敤鎴峰彲浠ヨ交鏉炬洿鏀瑰垎鍖恒€?
鍛戒护琛岀殑鏍煎紡涓?mtdparts 绫讳技锛?
blkdevparts=<blkdev-def>[;<blkdev-def>]
  <blkdev-def> := <blkdev-id>:<partdef>[,<partdef>]
    <partdef> := <size>[@<offset>](part-name)

<blkdev-id>
    鍧楄澶囩鐩樺悕銆傚祵鍏ュ紡璁惧浣跨敤鍥哄畾鍧楄澶囥€傚叾纾佺洏鍚嶄篃鏄浐瀹氱殑锛屼緥濡傦細
    mmcblk0銆乵mcblk1銆乵mcblk0boot0銆?
<size>
    鍒嗗尯澶у皬锛屼互瀛楄妭涓哄崟浣嶏紝渚嬪锛?12銆?m銆?G銆傚ぇ灏忓彲鍖呭惈鍙€夊悗缂€
    锛堝ぇ鍐欐垨灏忓啓锛夛細

      K, M, G, T, P, E銆?
    鈥?鈥?琛ㄧず鎵€鏈夊墿浣欑┖闂淬€?
<offset>
    鍒嗗尯璧峰鍦板潃锛屼互瀛楄妭涓哄崟浣嶃€傚亸绉诲彲鍖呭惈鍙€夊悗缂€锛堝ぇ鍐欐垨灏忓啓锛夛細

      K, M, G, T, P, E銆?
(part-name)
    鍒嗗尯鍚嶃€傚唴鏍稿彂閫佸甫鏈夆€淧ARTNAME鈥濈殑 uevent銆傚簲鐢ㄧ▼搴忓彲浠ュ垱寤烘寚鍚戣鍚嶇О
    鈥淧ARTNAME鈥濈殑鍧楄澶囧垎鍖虹殑閾炬帴銆傜敤鎴风┖闂村簲鐢ㄧ▼搴忓彲浠ラ€氳繃鍒嗗尯鍚嶈闂垎鍖恒€?
ro
    鍙銆傚皢鍒嗗尯鏍囪涓哄彧璇汇€?
绀轰緥锛?
    eMMC 纾佺洏鍚嶄负 "mmcblk0" 鍜?"mmcblk0boot0"銆?
```
    'blkdevparts=mmcblk0:1G(data0),1G(data1),-;mmcblk0boot0:1m(boot)ro,-(kernel)'

  dmesg::

    mmcblk0: p1(data0) p2(data1) p3()
    mmcblk0boot0: p1(boot) p2(kernel)
```
