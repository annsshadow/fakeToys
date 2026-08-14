## 璁惧鏄犲皠鍣ㄥ揩鐓ф敮鎸侊紙Device-mapper snapshot support锛?

璁惧鏄犲皠鍣ㄥ厑璁镐綘鍦ㄤ笉杩涜澶ч噺鏁版嵁澶嶅埗鐨勬儏鍐典笅锛?
- 鍒涘缓浠绘剰鍧楄澶囩殑蹇収锛屽嵆鍙寕杞界殑銆佸凡淇濆瓨鐨勫潡璁惧鐘舵€侊紝骞朵笖杩欎簺鐘舵€佽繕鍙啓锛岃€屼笉浼氬共鎵板師濮嬪唴瀹癸紱
- 鍒涘缓璁惧鈥滃垎鏀€濓紙forks锛夛紝鍗冲悓涓€鏁版嵁娴佺殑涓嶅悓鐗堟湰銆?- 灏嗗潡璁惧鐨勫揩鐓у悎骞跺洖璇ュ揩鐓х殑婧愶紙origin锛夎澶囥€?
鍦ㄥ墠涓ょ鎯呭喌涓嬶紝dm 鍙鍒跺彂鐢熷彉鍖栫殑鏁版嵁鍧楋紝骞朵娇鐢ㄤ竴涓嫭绔嬬殑鍐欐椂澶嶅埗锛圕OW锛夊潡璁惧杩涜瀛樺偍銆?
瀵逛簬蹇収鍚堝苟锛孋OW 瀛樺偍涓殑鍐呭浼氳鍚堝苟鍥炴簮璁惧銆?

鍏辨湁涓変釜鍙敤鐨?dm 鐩爣锛歴napshot銆乻napshot-origin 鍜?snapshot-merge銆?
- snapshot-origin <origin>

閫氬父浼氬湪鍏朵笂寤虹珛涓€涓垨澶氫釜蹇収銆傝鎿嶄綔灏嗙洿鎺ユ槧灏勫埌鍚庣璁惧銆傚浜庢瘡娆″啓鎿嶄綔锛屽師濮嬫暟鎹皢淇濆瓨鍦ㄦ瘡涓揩鐓х殑 <COW device> 涓紝浠ヤ繚鎸佸叾鍙鍐呭涓嶅彉锛岃嚦灏戠洿鍒?<COW device> 濉弧涓烘銆?

- snapshot <origin> <COW device> <persistent?> <chunksize>
   [<# feature args> [<arg>]*]

灏嗗垱寤?<origin> 鍧楄澶囩殑涓€涓揩鐓с€傚ぇ灏忎负 <chunksize> 鎵囧尯鐨勬暟鎹潡鍙樻洿灏嗗瓨鍌ㄥ湪 <COW device> 涓娿€傚啓鎿嶄綔鍙細鍐欏叆 <COW device>銆傝鎿嶄綔瀵逛簬鏈洿鏀圭殑鏁版嵁灏嗘潵鑷?<COW device> 鎴?<origin>銆?COW device> 閫氬父灏忎簬婧愯澶囷紝濡傛灉瀹冨～婊★紝蹇収灏嗗彉寰楁棤鐢ㄥ苟琚鐢紝杩斿洖閿欒銆傚洜姝ょ洃鎺х┖闂茬┖闂存暟閲忓苟鍦?<COW device> 濉弧涔嬪墠瀵瑰叾杩涜鎵╁睍寰堥噸瑕併€?
<persistent?> 涓?P锛圥ersistent锛屾寔涔咃級鎴?N锛圢ot persistent锛屼笉鎸佷箙鈥斺€旈噸鍚悗涓嶄繚鐣欙級銆侽锛圤verflow锛屾孩鍑猴級鍙綔涓烘寔涔呭瓨鍌ㄩ€夐」娣诲姞锛屼互鍏佽鐢ㄦ埛鎬侀€氬憡鍏舵敮鎸佸湪蹇収鐘舵€佷腑鐪嬪埌鈥淥verflow鈥濄€傚洜姝ゆ敮鎸佺殑瀛樺偍绫诲瀷涓?"P"銆?PO" 鍜?"N"銆?
鎸佷箙涓庣灛鎬侊紙transient锛変箣闂寸殑鍖哄埆鍦ㄤ簬锛氱灛鎬佸揩鐓у繀椤讳繚瀛樺湪纾佺洏涓婄殑鍏冩暟鎹洿灏戔€斺€斿畠浠彲浠ョ敱鍐呮牳淇濆瓨鍦ㄥ唴瀛樹腑銆?
鍔犺浇鎴栧嵏杞藉揩鐓х洰鏍囨椂锛岀浉搴旂殑 snapshot-origin 鎴?snapshot-merge 鐩爣蹇呴』琚寕璧枫€傛湭鑳芥寕璧锋簮鐩爣鍙兘瀵艰嚧鏁版嵁鎹熷潖銆?
鍙€夌壒鎬э紙features锛夛細

   discard_zeroes_cow - 瀵规槧灏勫埌鏁翠釜鏁版嵁鍧楃殑蹇収璁惧鍙戝嚭鐨?discard 浼氬皢
   蹇収寮傚父瀛樺偍涓浉搴旂殑寮傚父锛坋xception锛夋竻闆躲€?
   discard_passdown_origin - 瀵瑰揩鐓ц澶囧彂鍑虹殑 discard 浼氳鍚戜笅浼犻€?   鍒?snapshot-origin 鐨勫簳灞傝澶囥€傝繖涓嶄細瀵艰嚧鍚戝揩鐓у紓甯稿瓨鍌ㄥ鍒讹紝
   鍥犱负 snapshot-origin 鐩爣琚粫杩囦簡銆?
   discard_passdown_origin 鐗规€т緷璧栦簬 discard_zeroes_cow 鐗规€ц鍚敤銆?

- snapshot-merge <origin> <COW device> <persistent> <chunksize>
   [<# feature args> [<arg>]*]

闄ゅ彧閫傜敤浜庢寔涔呭揩鐓у锛屽叾琛ㄥ弬鏁颁笌 snapshot 鐩爣鐩稿悓銆傝鐩爣鎵挎媴
"snapshot-origin" 鐩爣鐨勮鑹诧紝濡傛灉鍦?<origin> 鐨?"snapshot-origin"
浠嶇劧瀛樺湪鏃讹紝涓嶅緱鍔犺浇瀹冦€?
鍒涘缓涓€涓悎骞跺揩鐓э紝閫氳繃浜ゆ帴锛坔andover锛夎繃绋嬫帴绠＄幇鏈夊揩鐓у瓨鍌ㄥ湪
<COW device> 涓殑宸插彉鏇存暟鎹潡锛屽苟灏嗚繖浜涙暟鎹潡鍚堝苟鍥?<origin>銆?涓€鏃﹀悎骞跺紑濮嬶紙鍦ㄥ悗鍙帮級锛?origin> 鍗冲彲琚墦寮€锛屼笖鍚堝苟灏嗗湪 I/O 娴佸悜
瀹冩椂缁х画杩涜銆傚 <origin> 鐨勬洿鏀逛細琚帹杩燂紝鐩村埌鍚堝苟蹇収瀵瑰簲鐨勬暟鎹潡
宸茶鍚堝苟銆備竴鏃﹀悎骞跺紑濮嬶紝涓?"snapshot" 鐩爣鍏宠仈鐨勫揩鐓ц澶囧湪琚闂椂
灏嗚繑鍥?-EIO銆?

## LVM2 濡備綍浣跨敤蹇収锛圚ow snapshot is used by LVM2锛?

褰撲綘鍒涘缓鏌愪釜鍗风殑绗竴涓?LVM2 蹇収鏃讹紝浼氫娇鐢ㄥ洓涓?dm 璁惧锛?
1) 涓€涓寘鍚簮鍗峰師濮嬫槧灏勮〃鐨勮澶囷紱
2) 涓€涓敤浣?<COW device> 鐨勮澶囷紱
3) 涓€涓?"snapshot" 璁惧锛岀粍鍚堜簡 #1 鍜?#2锛屽嵆鍙鐨勫揩鐓у嵎锛?4) 鍘熷锛?original"锛夊嵎锛堜娇鐢ㄥ師濮嬫簮鍗蜂娇鐢ㄧ殑璁惧鍙凤級锛屽叾琛ㄨ
   #1 璁惧鐨?"snapshot-origin" 鏄犲皠鎵€鏇挎崲銆?
```
  lvcreate -L 1G -n base volumeGroup
  lvcreate -L 100M --snapshot -n snap volumeGroup/base

```
```
  # dmsetup table|grep volumeGroup

  volumeGroup-base-real: 0 2097152 linear 8:19 384
  volumeGroup-snap-cow: 0 204800 linear 8:19 2097536
  volumeGroup-snap: 0 2097152 snapshot 254:11 254:12 P 16
  volumeGroup-base: 0 2097152 snapshot-origin 254:11

  # ls -lL /dev/mapper/volumeGroup-*
  brw-------  1 root root 254, 11 29 ago 18:15 /dev/mapper/volumeGroup-base-real
  brw-------  1 root root 254, 12 29 ago 18:15 /dev/mapper/volumeGroup-snap-cow
  brw-------  1 root root 254, 13 29 ago 18:15 /dev/mapper/volumeGroup-snap
  brw-------  1 root root 254, 10 29 ago 18:14 /dev/mapper/volumeGroup-base


```
## LVM2 濡備綍浣跨敤蹇収鍚堝苟锛圚ow snapshot-merge is used by LVM2锛?
鍚堝苟蹇収鍦ㄥ悎骞舵湡闂存壙鎷?"snapshot-origin" 鐨勮鑹层€傚洜姝?"snapshot-origin" 琚浛鎹负 "snapshot-merge"銆?-real" 璁惧涓嶅彉锛?-cow" 璁惧琚噸鍛藉悕涓?<origin name>-cow锛屼互鍗忓姪 LVM2 鍦ㄥ悎骞跺揩鐓у畬鎴愬悗杩涜娓呯悊銆傚皢鍏?COW 璁惧绉讳氦缁?"snapshot-merge" 鐨?"snapshot" 浼氳鍋滅敤锛堥櫎闈炰娇鐢?lvchange --refresh锛夛紱浣嗗鏋滃畠淇濇寔婵€娲荤姸鎬侊紝鍒欏彧浼氳繑鍥?I/O 閿欒銆?
```
  lvconvert --merge volumeGroup/snap

```
```
  # dmsetup table|grep volumeGroup

  volumeGroup-base-real: 0 2097152 linear 8:19 384
  volumeGroup-base-cow: 0 204800 linear 8:19 2097536
  volumeGroup-base: 0 2097152 snapshot-merge 254:11 254:12 P 16

  # ls -lL /dev/mapper/volumeGroup-*
  brw-------  1 root root 254, 11 29 ago 18:15 /dev/mapper/volumeGroup-base-real
  brw-------  1 root root 254, 12 29 ago 18:16 /dev/mapper/volumeGroup-base-cow
  brw-------  1 root root 254, 10 29 ago 18:16 /dev/mapper/volumeGroup-base


```
## 濡備綍鍒ゆ柇鍚堝苟浣曟椂瀹屾垚锛圚ow to determine when a merging is complete锛?
snapshot-merge 鍜?snapshot 鐨勭姸鎬佽浠ヤ互涓嬪唴瀹圭粨灏撅細

  <sectors_allocated>/<total_sectors> <metadata_sectors>

<sectors_allocated> 鍜?<total_sectors> 閮藉悓鏃跺寘鍚暟鎹拰鍏冩暟鎹€傚湪鍚堝苟杩囩▼涓紝宸插垎閰嶆墖鍖烘暟浼氳秺鏉ヨ秺灏忋€傚綋淇濆瓨鏁版嵁鐨勬墖鍖烘暟涓洪浂鏃讹紝鍗?<sectors_allocated> == <metadata_sectors> 鏃讹紝鍚堝苟瀹屾垚銆?
```
  # lvs
    LV      VG          Attr   LSize Origin  Snap%  Move Log Copy%  Convert
    base    volumeGroup owi-a- 4.00g
    snap    volumeGroup swi-a- 1.00g base  18.97

  # dmsetup status volumeGroup-snap
  0 8388608 snapshot 397896/2097152 1560
                                    ^^^^ metadata sectors

  # lvconvert --merge -b volumeGroup/snap
    Merging of volume snap started.

  # lvs volumeGroup/snap
    LV      VG          Attr   LSize Origin  Snap%  Move Log Copy%  Convert
    base    volumeGroup Owi-a- 4.00g          17.23

  # dmsetup status volumeGroup-base
  0 8388608 snapshot-merge 281688/2097152 1104

  # dmsetup status volumeGroup-base
  0 8388608 snapshot-merge 180480/2097152 712

  # dmsetup status volumeGroup-base
  0 8388608 snapshot-merge 16/2097152 16

```
鍚堝苟宸插畬鎴愶紙Merging has finished锛夈€?
```
  # lvs
    LV      VG          Attr   LSize Origin  Snap%  Move Log Copy%  Convert
    base    volumeGroup owi-a- 4.00g


```
