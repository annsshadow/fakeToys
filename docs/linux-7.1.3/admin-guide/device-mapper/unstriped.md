## 璁惧鏄犲皠鍣ㄢ€渦nstriped鈥濈洰鏍?


## 绠€浠?


璁惧鏄犲皠鍣紙device-mapper锛夌殑鈥渦nstriped鈥濈洰鏍囨彁渚涗簡涓€绉嶉€忔槑鏈哄埗锛岀敤浜庡皢璁惧鏄犲皠鍣ㄧ殑鈥渟triped鈥濈洰鏍囪В闄ゆ潯甯﹀寲锛屼互璁块棶搴曞眰纾佺洏锛岃€屾棤闇€瑙﹀強鐪熸鐨勫悗绔潡璁惧銆傚畠涔熷彲鐢ㄤ簬瑙ｉ櫎纭欢 RAID-0 鐨勬潯甯﹀寲浠ヨ闂悗绔鐩樸€?

鍙傛暟锛?
<number of stripes> <chunk size> <stripe #> <dev_path> <offset>

<number of stripes>
        RAID 0 涓殑鏉″甫鏁伴噺銆?

<chunk size>
	鏉″甫鍖栦腑涓€涓尯鍧楋紙chunk锛夋墍鍖呭惈鐨?512B 鎵囧尯鏁伴噺銆?

<dev_path>
	浣犲笇鏈涜В闄ゆ潯甯﹀寲鐨勫潡璁惧銆?

<stripe #>
        璁惧涓搴斾簬浣犲笇鏈涜В闄ゆ潯甯﹀寲鐨勭墿鐞嗛┍鍔ㄥ櫒鐨勬潯甯︾紪鍙枫€傝繖蹇呴』鏄?0 璧峰鐨勭储寮曘€?


## 涓轰綍浣跨敤姝ゆā鍧楋紵


### 鎾ら攢鐜版湁 dm-stripe 鐨勪竴涓ず渚?


杩欎釜灏忓瀷 bash 鑴氭湰灏嗚缃?4 涓?loop 璁惧锛屽苟浣跨敤鐜版湁鐨?striped 鐩爣灏嗚繖 4 涓澶囧悎骞朵负涓€涓€傜劧鍚庡畠浼氬湪 striped 璁惧涔嬩笂浣跨敤 unstriped 鐩爣鏉ヨ闂悇涓悗绔殑 loop 璁惧銆傛垜浠皢鏁版嵁鍐欏叆鏂版毚闇茬殑 unstriped 璁惧锛屽苟楠岃瘉鍐欏叆鐨勬暟鎹笌姝ｇ‘鐨?```

  #!/bin/bash

  MEMBER_SIZE=$((128 * 1024 * 1024))
  NUM=4
  SEQ_END=$((${NUM}-1))
  CHUNK=256
  BS=4096

  RAID_SIZE=$((${MEMBER_SIZE}*${NUM}/512))
  DM_PARMS="0 ${RAID_SIZE} striped ${NUM} ${CHUNK}"
  COUNT=$((${MEMBER_SIZE} / ${BS}))

  for i in $(seq 0 ${SEQ_END}); do
    dd if=/dev/zero of=member-${i} bs=${MEMBER_SIZE} count=1 oflag=direct
    losetup /dev/loop${i} member-${i}
    DM_PARMS+=" /dev/loop${i} 0"
  done

  echo $DM_PARMS | dmsetup create raid0
  for i in $(seq 0 ${SEQ_END}); do
    echo "0 1 unstriped ${NUM} ${CHUNK} ${i} /dev/mapper/raid0 0" | dmsetup create set-${i}
  done;

  for i in $(seq 0 ${SEQ_END}); do
    dd if=/dev/urandom of=/dev/mapper/set-${i} bs=${BS} count=${COUNT} oflag=direct
    diff /dev/mapper/set-${i} member-${i}
  done;

  for i in $(seq 0 ${SEQ_END}); do
    dmsetup remove set-${i}
  done

  dmsetup remove raid0

  for i in $(seq 0 ${SEQ_END}); do
    losetup -d /dev/loop${i}
    rm -f member-${i}
  done

```
### 鍙︿竴涓ず渚?


Intel NVMe 椹卞姩鍣ㄥ湪鐗╃悊璁惧涓婂寘鍚袱涓牳蹇冦€?
椹卞姩鍣ㄧ殑姣忎釜鏍稿績瀵瑰叾 LBA 鑼冨洿鏈夐殧绂荤殑璁块棶銆?
褰撳墠鐨?LBA 妯″瀷鍦ㄦ瘡涓牳蹇冧笂鏈変竴涓?RAID 0 128k 鍖哄潡锛屽鑷?```

   Core 0:       Core 1:
  __________    __________
  | LBA 512|    | LBA 768|
  | LBA 0  |    | LBA 256|
  ----------    ----------

```
姝よВ闄ゆ潯甯﹀寲鐨勭洰鐨勬槸鍦ㄥ槇鏉傞偦灞呯幆澧冧腑鎻愪緵鏇村ソ鐨?QoS銆傚綋鍦ㄤ笉杩涜姝よВ闄ゆ潯甯﹀寲鐨勬儏鍐典笅鍦ㄨ仛鍚堥┍鍔ㄥ櫒涓婂垱寤轰袱涓垎鍖烘椂锛屽涓€涓垎鍖虹殑璇诲彇浼氬奖鍝嶅彟涓€涓垎鍖轰笂鐨勫啓鍏ャ€傝繖鏄洜涓哄垎鍖烘槸璺ㄤ袱涓牳蹇冩潯甯﹀寲鐨勩€傚綋鎴戜滑瑙ｉ櫎姝ょ‖浠?RAID 0 鐨勬潯甯﹀寲锛屽苟鍦ㄦ瘡涓柊鏆撮湶鐨勮澶囦笂鍒涘缓鍒嗗尯鏃讹紝涓や釜鍒嗗尯鐜板湪鍦ㄧ墿鐞嗕笂鏄垎绂荤殑銆?

鍊熷姪 dm-unstriped 鐩爣锛屾垜浠兘澶熼殧绂讳竴涓?fio 鑴氭湰锛屽叾涓殑璇讳綔涓氬拰鍐欎綔涓氬郊姝ょ嫭绔嬨€備笌鍦ㄥ甫鏈夊垎鍖虹殑鍚堝苟椹卞姩鍣ㄤ笂杩愯娴嬭瘯鐩告瘮锛屼娇鐢ㄦ璁惧鏄犲皠鍣ㄧ洰鏍囷紝鎴戜滑灏嗚鍙栧欢杩熼檷浣庝簡 92%銆?


## dmsetup 浣跨敤绀轰緥


### 鍦ㄥ叿鏈?2 涓牳蹇冪殑 Intel NVMe 璁惧涔嬩笂瑙ｉ櫎鏉″甫鍖?


```

  dmsetup create nvmset0 --table '0 512 unstriped 2 256 0 /dev/nvme0n1 0'
  dmsetup create nvmset1 --table '0 512 unstriped 2 256 1 /dev/nvme0n1 0'

```
鐜板湪灏嗘湁涓や釜璁惧鍒嗗埆鏆撮湶 Intel NVMe 鏍稿績 0 鍜?1
```

  /dev/mapper/nvmset0
  /dev/mapper/nvmset1

```
### 鍦ㄥ叿鏈?4 涓┍鍔ㄥ櫒銆佷娇鐢?128K 鍖哄潡澶у皬鐨?striped 涔嬩笂瑙ｉ櫎鏉″甫鍖?


```

  dmsetup create raid_disk0 --table '0 512 unstriped 4 256 0 /dev/mapper/striped 0'
  dmsetup create raid_disk1 --table '0 512 unstriped 4 256 1 /dev/mapper/striped 0'
  dmsetup create raid_disk2 --table '0 512 unstriped 4 256 2 /dev/mapper/striped 0'
  dmsetup create raid_disk3 --table '0 512 unstriped 4 256 3 /dev/mapper/striped 0'

```
