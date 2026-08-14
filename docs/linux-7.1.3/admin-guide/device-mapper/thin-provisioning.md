## 绮剧畝閰嶇疆锛圱hin provisioning锛?


## 绠€浠嬶紙Introduction锛?


鏈枃妗ｆ弿杩颁簡涓€缁?device-mapper 鐩爣锛坱arget锛夛紝瀹冧滑鍏卞悓瀹炵幇浜嗙簿绠€閰嶇疆锛坱hin-provisioning锛変笌蹇収锛坰napshots锛夈€?

涓庝箣鍓嶇殑蹇収瀹炵幇鐩告瘮锛屾湰瀹炵幇鐨勪富瑕佷寒鐐规槸瀹冨厑璁稿皢璁稿铏氭嫙璁惧瀛樺偍鍦ㄥ悓涓€涓暟鎹嵎涓娿€傝繖绠€鍖栦簡绠＄悊锛屽苟鍏佽鍦ㄥ嵎涔嬮棿鍏变韩鏁版嵁锛屼粠鑰屽噺灏戠鐩樹娇鐢ㄩ噺銆?

鍙︿竴涓噸瑕佺壒鎬ф槸鏀寔浠绘剰娣卞害鐨勯€掑綊蹇収锛堝揩鐓х殑蹇収鐨勫揩鐓р€︹€︼級銆備箣鍓嶇殑蹇収瀹炵幇鏄€氳繃灏嗘煡鎵捐〃閾惧紡杩炴帴鏉ュ疄鐜拌繖涓€鐐圭殑锛屽洜姝ゆ€ц兘涓?O(娣卞害)銆傛湰鏂板疄鐜颁娇鐢ㄥ崟涓€鏁版嵁缁撴瀯鏉ラ伩鍏嶈繖绉嶉殢娣卞害涓嬮檷鐨勬€ц兘閫€鍖栥€備笉杩囷紝鍦ㄦ煇浜涘満鏅笅锛岀鐗囧寲浠嶇劧鍙兘鏄釜闂銆?

鍏冩暟鎹笌鏁版嵁瀛樺偍鍦ㄤ笉鍚岀殑璁惧涓婏紝杩欑粰浜嗙鐞嗗憳涓€浜涜嚜鐢卞害锛屼緥濡傦細

- 閫氳繃灏嗗厓鏁版嵁瀛樺偍鍦ㄤ竴涓暅鍍忓嵎涓娿€佽€屾暟鎹瓨鍌ㄥ湪闈為暅鍍忓嵎涓婏紝鏉ユ彁楂樺厓鏁版嵁鐨勫脊鎬с€?

- 閫氳繃灏嗗厓鏁版嵁瀛樺偍鍒?SSD 涓婃潵鎻愰珮鎬ц兘銆?

## 鐘舵€侊紙Status锛?


杩欎簺鐩爣琚涓哄彲瀹夊叏鐢ㄤ簬鐢熶骇鐜銆備絾涓嶅悓鐨勭敤渚嬩細鏈変笉鍚岀殑鎬ц兘鐗瑰緛锛屼緥濡傜敱浜庢暟鎹嵎鐨勭鐗囧寲銆?

濡傛灉鎮ㄥ彂鐜版湰杞欢鐨勮〃鐜颁笉绗﹀悎棰勬湡锛岃灏嗚缁嗕俊鎭彂閫佽嚦 dm-devel@redhat.com锛屾垜浠皢灏藉姏涓烘偍鏀硅繘銆?

鐢ㄤ簬妫€鏌ュ拰淇鍏冩暟鎹殑鐢ㄦ埛绌洪棿宸ュ叿宸茬粡寮€鍙戝畬鎴愶紝骞朵綔涓?'thin_check' 涓?'thin_repair' 鎻愪緵銆傛彁渚涜繖浜涘伐鍏风殑杞欢鍖呭悕绉板洜鍙戣鐗堣€屽紓锛堝湪 Red Hat 鍙戣鐗堜腑瀹冨悕涓?'device-mapper-persistent-data'锛夈€?

## 閫熸煡鎵嬪唽锛圕ookbook锛?


鏈妭鎻忚堪浜嗕竴浜涗娇鐢ㄧ簿绠€閰嶇疆鐨勫揩閫熼厤鏂广€傚畠浠洿鎺ヤ娇鐢?dmsetup 绋嬪簭鏉ユ帶鍒?device-mapper 椹卞姩銆備竴鏃︽坊鍔犳敮鎸侊紝鏈€缁堢敤鎴峰皢琚缓璁娇鐢ㄦ洿楂樺眰鐨勫嵎绠＄悊鍣紙濡?LVM2锛夈€?

### 姹犺澶囷紙Pool device锛?


姹犺澶囧皢鍏冩暟鎹嵎涓庢暟鎹嵎缁戝畾鍦ㄤ竴璧枫€傚畠灏?I/O 绾挎€ф槧灏勫埌鏁版嵁鍗凤紝骞堕€氳繃涓ょ鏈哄埗鏇存柊鍏冩暟鎹細

- 鏉ヨ嚜 thin 鐩爣鐨勫嚱鏁拌皟鐢?

- 鏉ヨ嚜鐢ㄦ埛绌洪棿鐨?device-mapper 'messages'锛岀敤浜庢帶鍒讹紙闄ゅ叾浠栦簨椤瑰锛夋柊铏氭嫙璁惧鐨勫垱寤恒€?

### 寤虹珛涓€涓柊鐨勬睜璁惧


寤虹珛涓€涓睜璁惧闇€瑕佷竴涓湁鏁堢殑鍏冩暟鎹澶囧拰涓€涓暟鎹澶囥€傚鏋滄偍娌℃湁鐜版垚鐨勫厓鏁版嵁璁惧锛屽彲浠ラ€氳繃灏嗗墠 4k 娓呴浂鏉ユ寚绀哄叾涓虹┖鍏冩暟鎹€?

    dd if=/dev/zero of=$metadata_dev bs=4096 count=1

鎮ㄩ渶瑕佺殑鍏冩暟鎹噺浼氭牴鎹?thin 璁惧涔嬮棿鍏变韩鐨勫潡鏁伴噺锛堝嵆閫氳繃蹇収鍏变韩锛夎€屽彉鍖栥€傚鏋滃叡浜▼搴︿綆浜庡钩鍧囨按骞筹紝鎮ㄥ皢闇€瑕佷竴涓ぇ浜庡钩鍧囧ぇ灏忕殑鍏冩暟鎹澶囥€?

浣滀负鍙傝€冿紝鎴戜滑寤鸿鎮ㄥ皢鍏冩暟鎹澶囦腑浣跨敤鐨勫瓧鑺傛暟璁＄畻涓?48 * $data_dev_size / $data_block_size锛屼絾濡傛灉缁撴灉灏忎簬 2MiB 鍒欏悜涓婂彇鏁村埌 2MiB銆傚鏋滄偍姝ｅ湪鍒涘缓澶ч噺璁板綍澶ч噺鍙樻洿鐨勫揩鐓э紝鍙兘浼氬彂鐜伴渶瑕佸澶ц鍊笺€?

鏀寔鐨勬渶澶уぇ灏忎负 16GiB锛氬鏋滆澶囨洿澶э紝灏嗗彂鍑鸿鍛婏紝澶氫綑鐨勭┖闂翠笉浼氳浣跨敤銆?

### 閲嶆柊鍔犺浇姹犺〃锛圧eloading a pool table锛?


鎮ㄥ彲浠ラ噸鏂板姞杞戒竴涓睜鐨勮〃锛屽疄闄呬笂锛屽綋姹犵┖闂磋€楀敖鏃跺氨鏄繖鏍锋潵璋冩暣姹犵殑澶у皬鐨勩€傦紙娉ㄦ剰锛氳櫧鐒剁洰鍓嶅苟涓嶇姝㈠湪閲嶆柊鍔犺浇鏃舵寚瀹氫笉鍚岀殑鍏冩暟鎹澶囷紝浣嗗鏋滃畠娌℃湁灏?I/O 璺敱鍒颁笌涔嬪墠瀹屽叏鐩稿悓鐨勭鐩樹綅缃紝浜嬫儏灏变細鍑洪敊銆傦級

### 浣跨敤鐜版湁鐨勬睜璁惧


```
    dmsetup create pool \
	--table "0 20971520 thin-pool $metadata_dev $data_dev \
		 $data_block_size $low_water_mark"

```
$data_block_size 缁欏嚭浜嗕竴娆″彲浠ュ垎閰嶇殑鏈€灏忕鐩樼┖闂村崟浣嶏紝浠?512 瀛楄妭鎵囧尯涓哄崟浣嶃€?data_block_size 蹇呴』浠嬩簬 128锛?4KiB锛変笌 2097152锛?GiB锛変箣闂达紝涓斾负 128锛?4KiB锛夌殑鍊嶆暟銆?data_block_size 鍦?thin-pool 鍒涘缓鍚庢棤娉曟洿鏀广€備富瑕佸绮剧畝閰嶇疆鎰熷叴瓒ｇ殑浜哄彲鑳芥兂浣跨敤璇稿 1024锛?12KiB锛夎繖鏍风殑鍊笺€傝繘琛屽ぇ閲忓揩鐓х殑浜哄彲鑳芥兂瑕佽緝灏忕殑鍊硷紝渚嬪 128锛?4KiB锛夈€傚鏋滄偍涓嶅鏂板垎閰嶇殑鏁版嵁杩涜娓呴浂锛屽垯寤鸿浣跨敤杈冨ぇ鐨?$data_block_size锛岀害涓?262144锛?28MiB锛夈€?

$low_water_mark 浠?$data_block_size 澶у皬鐨勫潡涓哄崟浣嶃€傚鏋滄暟鎹澶囦笂鐨勭┖闂茬┖闂撮檷鍒版绾у埆浠ヤ笅锛屽垯浼氳Е鍙戜竴涓?dm 浜嬩欢锛岀敤鎴风┖闂村畧鎶よ繘绋嬪簲璇ユ崟鑾疯浜嬩欢浠ユ墿灞曟睜璁惧銆傚彧浼氬彂閫佽繖鏍蜂竴涓簨浠躲€?

濡傛灉鍒氭仮澶嶇殑璁惧鍏剁┖闂茬┖闂翠綆浜庝綆姘翠綅绾匡紝鍒欎笉浼氳Е鍙戠壒娈婁簨浠躲€備絾鏄紝鎭㈠涓€涓澶囨€讳細瑙﹀彂涓€涓簨浠讹紱鐢ㄦ埛绌洪棿瀹堟姢杩涚▼鍦ㄥ鐞嗘浜嬩欢鏃跺簲纭绌洪棽绌洪棿瓒呰繃浜嗕綆姘翠綅绾裤€?

鍏冩暟鎹澶囩殑浣庢按浣嶇嚎鐢卞唴鏍哥淮鎶わ紝濡傛灉鍏冩暟鎹澶囦笂鐨勭┖闂茬┖闂撮檷鍒板叾浠ヤ笅锛屽皢瑙﹀彂涓€涓?dm 浜嬩欢銆?

### 鏇存柊纾佺洏涓婄殑鍏冩暟鎹?


纾佺洏涓婄殑鍏冩暟鎹湪姣忔鍐欏叆 FLUSH 鎴?FUA bio 鏃舵彁浜ゃ€傚鏋滄病鏈夊彂鍑烘绫昏姹傦紝鍒欐瘡绉掓彁浜や竴娆°€傝繖鎰忓懗鐫€绮剧畝閰嶇疆鐩爣鐨勮涓虹被浼间簬鍏锋湁鏄撳け鍐欑紦瀛樼殑鐗╃悊纾佺洏銆傚鏋滄柇鐢碉紝鎮ㄥ彲鑳戒細涓㈠け涓€浜涙渶杩戠殑鍐欏叆銆傚敖绠″彂鐢熶换浣曞穿婧冿紝鍏冩暟鎹簲褰撳缁堜繚鎸佷竴鑷淬€?

濡傛灉鏁版嵁绌洪棿鑰楀敖锛屾睜灏嗘牴鎹厤缃姤閿欐垨鎺掗槦 IO锛堝弬瑙侊細error_if_no_space锛夈€傚鏋滃厓鏁版嵁绌洪棿鑰楀敖鎴栧厓鏁版嵁鎿嶄綔澶辫触锛氭睜灏嗘姤閿?IO锛岀洿鍒版睜琚笅绾垮苟瀵瑰厓鏁版嵁鎵ц淇浠?1) 淇浠讳綍娼滃湪鐨勪笉涓€鑷达紝浠ュ強 2) 娓呴櫎鏂藉姞淇瑕佹眰鐨勬爣蹇椼€備竴鏃︽睜鐨勫厓鏁版嵁璁惧琚慨澶嶏紝灏卞彲浠ュ鍏惰皟鏁村ぇ灏忥紝杩欏皢浣挎睜鎭㈠姝ｅ父鎿嶄綔銆傝娉ㄦ剰锛屽鏋滀竴涓睜琚爣璁颁负闇€瑕佷慨澶嶏紝鍒欏湪鎵ц淇涔嬪墠锛屾睜鐨勬暟鎹拰鍏冩暟鎹澶囬兘鏃犳硶璋冩暣澶у皬銆傝繕搴旀寚鍑猴紝褰撴睜鐨勫厓鏁版嵁绌洪棿鑰楀敖鏃讹紝褰撳墠鐨勫厓鏁版嵁浜嬪姟浼氳涓銆傞壌浜庢睜浼氱紦瀛樺叾瀹屾垚鍙兘宸茬粡鍚戜笂灞?IO锛堜緥濡傛枃浠剁郴缁燂級纭鐨?IO锛屽己鐑堝缓璁湪闇€瑕佸姹犺繘琛屼慨澶嶆椂锛屽杩欎簺灞傛墽琛屼竴鑷存€ф鏌ワ紙渚嬪 fsck锛夈€?

### 绮剧畝閰嶇疆


i) 鍒涘缓涓€涓柊鐨勭簿绠€閰嶇疆鍗枫€?

  瑕佸垱寤轰竴涓柊鐨勭簿绠€閰嶇疆鍗凤紝鎮ㄥ繀椤诲悜姹犲彂閫佷竴鏉℃秷鎭紝鍐呭濡備笅锛?

```
    dmsetup message /dev/mapper/pool 0 "create_thin 0"

```
  杩欓噷鐨?'0' 鏄嵎鐨勬爣璇嗙锛屼竴涓?24 浣嶆暟瀛椼€傜敱璋冪敤鑰呰礋璐ｅ垎閰嶅拰绠＄悊杩欎簺鏍囪瘑绗︺€傚鏋滆鏍囪瘑绗﹀凡鍦ㄤ娇鐢ㄤ腑锛屾秷鎭皢浠?-EEXIST 澶辫触銆?

```
```
ii) 浣跨敤绮剧畝閰嶇疆鍗枫€?

```
    dmsetup create thin --table "0 2097152 thin /dev/mapper/pool 0"

```
  鏈€鍚庝竴涓弬鏁版槸 thinp 璁惧鐨勬爣璇嗙銆?

### 鍐呴儴蹇収锛圛nternal snapshots锛?


i) 鍒涘缓涓€涓唴閮ㄥ揩鐓с€?

  蹇収鏄€氳繃鍚戞睜鍙戦€佸彟涓€鏉℃秷鎭潵鍒涘缓鐨勩€?

  娉ㄦ剰锛氬鏋滄偍甯屾湜蹇収鐨勬簮璁惧锛坥rigin device锛夊浜庢椿鍔ㄧ姸鎬侊紝蹇呴』鍦ㄥ垱寤哄揩鐓т箣鍓嶅皢鍏舵寕璧凤紙suspend锛変互閬垮厤鎹熷潖銆傝繖涓€鐐圭洰鍓嶅苟鏈寮哄埗锛屾墍浠ヨ灏忓績锛?

```
    dmsetup suspend /dev/mapper/thin
    dmsetup message /dev/mapper/pool 0 "create_snap 1 0"
    dmsetup resume /dev/mapper/thin

```
  杩欓噷鐨?'1' 鏄嵎鐨勬爣璇嗙锛屼竴涓?24 浣嶆暟瀛椼€?0' 鏄簮璁惧鐨勬爣璇嗙銆?

```
```
ii) 浣跨敤鍐呴儴蹇収銆?

  涓€鏃﹀垱寤猴紝鐢ㄦ埛涓嶅繀鎷呭績婧愪笌蹇収涔嬮棿鐨勪换浣曡繛鎺ャ€傚疄闄呬笂锛岃蹇収涓庝换浣曞叾浠栫簿绠€閰嶇疆璁惧骞舵棤涓嶅悓锛屽苟涓斿彲浠ラ€氳繃鐩稿悓鐨勬柟娉曞鍏惰繘琛屽揩鐓с€傚彧婵€娲诲叾涓箣涓€鏄畬鍏ㄥ悎娉曠殑锛屽苟涓斿婵€娲绘垨绉婚櫎瀹冧滑涓よ€呮病鏈夐『搴忚姹傘€傦紙杩欎笌浼犵粺鐨?device-mapper 蹇収涓嶅悓銆傦級

```
    dmsetup create snap --table "0 2097152 thin /dev/mapper/pool 1"

```
### 澶栭儴蹇収锛圗xternal snapshots锛?


鎮ㄥ彲浠ヤ娇鐢ㄤ竴涓閮ㄧ殑**鍙**璁惧浣滀负绮剧畝閰嶇疆鍗风殑婧愩€傚 thin 璁惧鏈厤缃尯鍩熺殑浠讳綍璇诲彇閮戒細閫忎紶鍒拌婧愩€傚啓鍏ヤ細鍍忓線甯镐竴鏍疯Е鍙戞柊鍧楃殑鍒嗛厤銆?

涓€涓敤渚嬫槸 VM 瀹夸富鏈哄笇鏈涘湪绮剧畝閰嶇疆鍗蜂笂杩愯瀹㈡埛鏈猴紝浣嗗皢鍩虹闀滃儚鏀惧湪鍙︿竴涓澶囦笂锛堝彲鑳藉湪澶氫釜 VM 涔嬮棿鍏变韩锛夈€?

濡傛灉鎮ㄤ娇鐢ㄦ鎶€鏈紝缁濅笉鑳藉啓鍏ユ簮璁惧锛佸綋鐒讹紝鎮ㄥ彲浠ュ啓鍏?thin 璁惧骞跺 thin 鍗锋媿鎽勫唴閮ㄥ揩鐓с€?

i) 鍒涘缓澶栭儴璁惧鐨勫揩鐓?

  杩欎笌鍒涘缓涓€涓?thin 璁惧鐩稿悓銆傚湪姝ら樁娈垫偍鏃犻渶鎻愬強婧愩€?

```
    dmsetup message /dev/mapper/pool 0 "create_thin 0"

```
ii) 浣跨敤澶栭儴璁惧鐨勫揩鐓с€?

```
    dmsetup create snap --table "0 2097152 thin /dev/mapper/pool 0 /dev/image"

```
  娉ㄦ剰锛氭蹇収鐨勬墍鏈夊悗浠ｏ紙鍐呴儴蹇収锛夐兘闇€瑕佺浉鍚岀殑棰濆婧愬弬鏁般€?

### 鍋滅敤锛圖eactivation锛?


鎵€鏈変娇鐢ㄦ煇涓睜鐨勮澶囬兘蹇呴』鍦ㄨ姹犳湰韬箣鍓嶈鍋滅敤銆?

```
    dmsetup remove thin
    dmsetup remove snap
    dmsetup remove pool

```
## 鍙傝€冿紙Reference锛?


### 'thin-pool' 鐩爣


i) 鏋勯€犲嚱鏁帮紙Constructor锛?

```
      thin-pool <metadata dev> <data dev> <data block size (sectors)> \
	        <low water mark (blocks)> [<number of feature args> [<arg>]*]

    Optional feature arguments:

      skip_block_zeroing:
	Skip the zeroing of newly-provisioned blocks.

      ignore_discard:
	Disable discard support.

      no_discard_passdown:
	Don't pass discards down to the underlying
	data device, but just remove the mapping.

      read_only:
		 Don't allow any changes to be made to the pool
		 metadata.  This mode is only available after the
		 thin-pool has been created and first used in full
		 read/write mode.  It cannot be specified on initial
		 thin-pool creation.

      error_if_no_space:
	Error IOs, instead of queueing, if no space.

    Data block size must be between 64KiB (128 sectors) and 1GiB
    (2097152 sectors) inclusive.


```
ii) 鐘舵€侊紙Status锛?

```
      <transaction id> <used metadata blocks>/<total metadata blocks>
      <used data blocks>/<total data blocks> <held metadata root>
      ro|rw|out_of_data_space [no_]discard_passdown [error|queue]_if_no_space
      needs_check|- metadata_low_watermark

    transaction id:
	A 64-bit number used by userspace to help synchronise with metadata
	from volume managers.

    used data blocks / total data blocks
	If the number of free blocks drops below the pool's low water mark a
	dm event will be sent to userspace.  This event is edge-triggered and
	it will occur only once after each resume so volume manager writers
	should register for the event and then check the target's status.

    held metadata root:
	The location, in blocks, of the metadata root that has been
	'held' for userspace read access.  '-' indicates there is no
	held root.

    discard_passdown|no_discard_passdown
	Whether or not discards are actually being passed down to the
	underlying device.  When this is enabled when loading the table,
	it can get disabled if the underlying device doesn't support it.

    ro|rw|out_of_data_space
	If the pool encounters certain types of device failures it will
	drop into a read-only metadata mode in which no changes to
	the pool metadata (like allocating new blocks) are permitted.

	In serious cases where even a read-only mode is deemed unsafe
	no further I/O will be permitted and the status will just
	contain the string 'Fail'.  The userspace recovery tools
	should then be used.

    error_if_no_space|queue_if_no_space
	If the pool runs out of data or metadata space, the pool will
	either queue or error the IO destined to the data device.  The
	default is to queue the IO until more space is added or the
	'no_space_timeout' expires.  The 'no_space_timeout' dm-thin-pool 妯″潡鍙傛暟
	鍙互鐢ㄦ潵鏀瑰彉姝よ秴鏃?-- it
	defaults to 60 seconds but may be disabled using a value of 0.

    needs_check
	A metadata operation has failed, resulting in the needs_check
	flag being set in the metadata's superblock.  The metadata
	device must be deactivated and checked/repaired before the
	thin-pool can be made fully operational again.  '-' indicates
	needs_check is not set.

    metadata_low_watermark:
	Value of metadata low watermark in blocks.  The kernel sets this
	value internally but userspace needs to know this value to
	determine if an event was caused by crossing this threshold.

```
iii) 娑堟伅锛圡essages锛?

    create_thin <dev id>
	鍒涘缓涓€涓柊鐨勭簿绠€閰嶇疆璁惧銆?
	<dev id> 鏄敱璋冪敤鏂归€夋嫨鐨勪换鎰忓敮涓€鐨?24 浣嶆爣璇嗙锛?
	鐢辫皟鐢ㄦ柟閫夋嫨銆?

    create_snap <dev id> <origin id>
	鍒涘缓鍙︿竴涓簿绠€閰嶇疆璁惧鐨勫揩鐓с€?
	<dev id> 鏄敱璋冪敤鏂归€夋嫨鐨勪换鎰忓敮涓€鐨?24 浣嶆爣璇嗙锛?
	鐢辫皟鐢ㄦ柟閫夋嫨銆?
	<origin id> 鏄蹇収鐨勭簿绠€閰嶇疆璁惧鐨勬爣璇嗙锛?
	鏂拌澶囧嵆璇ヨ澶囩殑蹇収銆?

    delete <dev id>
	鍒犻櫎涓€涓簿绠€璁惧銆備笉鍙€嗐€?

    set_transaction_id <current id> <new id>
	鐢ㄦ埛鎬佸嵎绠＄悊鍣紙濡?LVM锛夐渶瑕佷竴绉嶆柟寮忔潵
	灏嗗叾澶栭儴鍏冩暟鎹笌姹犵洰鏍囩殑鍐呴儴鍏冩暟鎹繘琛屽悓姝ャ€?
	姹犵洰鏍囥€倀hin-pool 鐩爣鎻愪緵瀛樺偍涓€涓?
	浠绘剰鐨?64 浣嶄簨鍔?ID锛屽苟鍦ㄧ洰鏍囩殑
	鐘舵€佽涓婅繑鍥炲畠銆備负閬垮厤绔炴€侊紝浣犲繀椤绘彁渚涗綘鎵€璁や负鐨?
	褰撳墠浜嬪姟 ID锛屾墠鑳界敤杩欐潯
	compare-and-swap 娑堟伅淇敼瀹冩椂銆?

    reserve_metadata_snap
        涓虹敤鎴锋€佷繚鐣欎竴浠芥暟鎹槧灏?btree 鐨勫壇鏈€?
        杩欏厑璁哥敤鎴锋€佹鏌ユ墽琛屾娑堟伅鏃?
        鐨勬槧灏勩€備娇鐢ㄦ睜鐨勭姸鎬佸懡浠ゆ潵
        鑾峰彇涓庡厓鏁版嵁蹇収鍏宠仈鐨勬牴鍧椼€?

    release_metadata_snap
        閲婃斁涔嬪墠淇濈暀鐨勬暟鎹槧灏?btree 鍓湰銆?

### 'thin' 鐩爣


i) 鏋勯€犲嚱鏁帮紙Constructor锛?

```
        thin <pool dev> <dev id> [<external origin dev>]

    pool dev:
	the thin-pool device, e.g. /dev/mapper/my_pool or 253:0

    dev id:
	the internal device identifier of the device to be
	activated.

    external origin dev:
	an optional block device outside the pool to be treated as a
	read-only snapshot origin: reads to unprovisioned areas of the
	thin target will be mapped to this device.

```
姹犱笉浼氶拡瀵?thin 璁惧瀛樺偍浠讳綍澶у皬銆傚鏋滄偍鍔犺浇鐨?thin 鐩爣姣斾箣鍓嶄娇鐢ㄧ殑灏忥紝閭ｄ箞鎮ㄥ皢鏃犳硶璁块棶鏄犲皠鍒版湯灏句箣澶栫殑鍧椼€傚鏋滄偍鍔犺浇鐨勭洰鏍囨瘮浠ュ墠澶э紝鍒欓澶栫殑鍧楀皢鍦ㄩ渶瑕佹椂鎸夐渶閰嶇疆銆?

ii) 鐘舵€侊紙Status锛?

    <nr mapped sectors> <highest mapped sector>
	濡傛灉姹犻亣鍒拌澶囬敊璇苟澶辫触锛屽叾鐘舵€?
	灏嗕粎鍖呭惈瀛楃涓?'Fail'銆傚簲褰撲娇鐢ㄧ敤鎴锋€佹仮澶?
	宸ュ叿銆?

    褰?<nr mapped sectors> 涓?0 鏃讹紝涓嶅瓨鍦ㄦ渶楂樼殑
    宸叉槧灏勬墖鍖猴紝涓?<highest mapped sector> 鐨勫€兼湭鎸囧畾銆?
