
## dm-clone


## 绠€浠?

dm-clone 鏄竴涓澶囨槧灏勫櫒鐩爣锛屽畠灏嗕竴涓凡瀛樺湪鐨勩€佸彧璇荤殑婧愯澶囦竴瀵逛竴鍦版嫹璐濆埌涓€涓彲鍐欑殑鐩湴璁惧涓細瀹冨憟鐜颁竴涓櫄鎷熷潡璁惧锛屼娇鎵€鏈夋暟鎹珛鍗冲彲瑙侊紝骞剁浉搴斿湴瀵硅鍐欒繘琛岄噸瀹氬悜銆?
dm-clone 鐨勪富瑕佺敤渚嬫槸灏嗕竴涓彲鑳戒綅浜庤繙绋嬨€侀珮寤惰繜銆佸彧璇汇€佸綊妗ｇ被鍨嬬殑鍧楄澶囷紝鍏嬮殕鍒颁竴涓彲鍐欍€佸揩閫熴€佷富绫诲瀷鐨勮澶囦腑锛屼互鑾峰緱蹇€熴€佷綆寤惰繜鐨?I/O銆傚厠闅嗗悗鐨勮澶囩珛鍗冲彲瑙?鍙寕杞斤紝鑰屾簮璁惧鍒扮洰鍦拌澶囩殑鎷疯礉鍦ㄥ悗鍙拌繘琛岋紝涓庣敤鎴?I/O 骞惰銆?
渚嬪锛屽彲浠ュ皢涓€涓€氳繃缃戠粶瀹夊叏瀛樺偍鍗忚锛圢BD銆丗ibre Channel銆乮SCSI銆丄oE 绛夛級璁块棶鐨勫彧璇诲壇鏈腑鐨勫簲鐢ㄥ浠斤紝鎭㈠鍒版湰鍦扮殑 SSD 鎴?NVMe 璁惧锛屽苟绔嬪嵆寮€濮嬩娇鐢ㄨ璁惧锛岃€屾棤闇€绛夊緟鎭㈠瀹屾垚銆?
褰撳厠闅嗗畬鎴愭椂锛屽彲浠ュ交搴曠Щ闄?dm-clone 琛紝骞舵浛鎹负渚嬪鐩存帴鏄犲皠鍒扮洰鍦拌澶囩殑绾挎€э紙linear锛夎〃銆?
dm-clone 鐩爣澶嶇敤浜嗙槮渚涚粰锛坱hin-provisioning锛夌洰鏍囨墍浣跨敤鐨勫厓鏁版嵁搴撱€?

## 鏈琛?

   Hydration锛堟按鍚堬級
     灏嗙洰鍦拌澶囩殑涓€涓尯鍩熺敤鏉ヨ嚜婧愯澶囧悓涓€鍖哄煙鐨勬暟鎹～婊＄殑杩囩▼锛屽嵆锛屽皢璇ュ尯鍩熶粠婧愯澶囨嫹璐濆埌鐩湴璁惧銆?
涓€鏃︽煇涓尯鍩熻姘村悎锛坔ydrated锛夛紝鎴戜滑灏卞皢璇ュ尯鍩熺殑鎵€鏈?I/O 閲嶅畾鍚戝埌鐩湴璁惧銆?

## 璁捐


### 瀛愯澶?

璇ョ洰鏍囬€氳繃鍚戝畠浼犲叆涓変釜璁惧锛堜互鍙婄◢鍚庤杩扮殑鍏跺畠鍙傛暟锛夋潵鏋勫缓锛?
1. 涓€涓簮璁惧鈥斺€旇鍏嬮殕鐨勫彧璇昏澶囷紝涔熸槸姘村悎鐨勬潵婧愩€?
2. 涓€涓洰鍦拌澶団€斺€旀按鍚堢殑鐩殑鍦帮紝瀹冨皢鎴愪负婧愯澶囩殑涓€涓厠闅嗐€?
3. 涓€涓皬鐨勫厓鏁版嵁璁惧鈥斺€斿畠璁板綍鍝簺鍖哄煙鍦ㄧ洰鍦拌澶囦腑宸茬粡鏈夋晥锛屽嵆鍝簺鍖哄煙宸茬粡琚按鍚堬紝鎴栬€呭凡缁忛€氳繃鐢ㄦ埛 I/O 琚洿鎺ュ啓鍏ャ€?
鐩湴璁惧鐨勫ぇ灏忓繀椤昏嚦灏戠瓑浜庢簮璁惧鐨勫ぇ灏忋€?

### 鍖哄煙锛圧egions锛?

dm-clone 灏嗘簮璁惧鍜岀洰鍦拌澶囧垝鍒嗕负鍥哄畾澶у皬鐨勫尯鍩熴€傚尯鍩熸槸姘村悎鐨勫崟浣嶏紝鍗充粠婧愯澶囨嫹璐濆埌鐩湴璁惧鐨勬渶灏忔暟鎹噺銆?
鍖哄煙澶у皬鍦ㄤ綘棣栨鍒涘缓 dm-clone 璁惧鏃跺彲閰嶇疆銆傛帹鑽愮殑鍖哄煙澶у皬涓庢枃浠剁郴缁熷潡澶у皬鐩稿悓锛岄€氬父涓?4KB銆傚尯鍩熷ぇ灏忓繀椤诲湪 8 涓墖鍖猴紙4KB锛夊埌 2097152 涓墖鍖猴紙1GB锛変箣闂达紝涓斾负 2 鐨勫箓銆?
瀵瑰凡姘村悎鍖哄煙鐨勮鍐欑敱鐩湴璁惧鎻愪緵鏈嶅姟銆?
瀵瑰皻鏈按鍚堝尯鍩熺殑璇荤洿鎺ヤ粠婧愯澶囨彁渚涙湇鍔°€?
瀵瑰皻鏈按鍚堝尯鍩熺殑鍐欏皢琚欢杩燂紝鐩村埌鐩稿簲鐨勫尯鍩熷凡琚按鍚堝苟绔嬪嵆寮€濮嬫按鍚堛€?
娉ㄦ剰锛屽ぇ灏忕瓑浜庡尯鍩熷ぇ灏忕殑鍐欒姹傚皢璺宠繃浠庢簮璁惧鎷疯礉鐩稿簲鍖哄煙锛屽苟鐩存帴瑕嗙洊鐩湴璁惧鐨勮鍖哄煙銆?

### 涓㈠純锛圖iscards锛?

dm-clone 灏嗛拡瀵瑰皻鏈按鍚堣寖鍥寸殑 discard 璇锋眰瑙ｉ噴涓鸿烦杩囪繖浜涜璇锋眰瑕嗙洊鍖哄煙鐨勬按鍚堢殑鎻愮ず锛屽嵆锛屽畠璺宠繃灏嗚鍖哄煙鐨勬暟鎹粠婧愯澶囨嫹璐濆埌鐩湴璁惧锛岃€屽彧鏇存柊鍏跺厓鏁版嵁銆?
濡傛灉鐩湴璁惧鏀寔 discard锛岄偅涔堥粯璁ゆ儏鍐典笅 dm-clone 浼氬皢 discard 璇锋眰鍚戜笅閫忎紶鍒板畠銆?

### 鍚庡彴姘村悎


dm-clone 鎸佺画鍦颁粠婧愯澶囨嫹璐濆埌鐩湴璁惧锛岀洿鍒版暣涓澶囬兘琚嫹璐濆畬鎴愩€?
浠庢簮璁惧鍒扮洰鍦拌澶囨嫹璐濇暟鎹細鍗犵敤甯﹀銆傜敤鎴峰彲浠ヨ缃竴涓妭娴佸€硷紝浠ラ槻姝㈠湪浠讳綍鏃跺埢鍙戠敓瓒呰繃涓€瀹氭暟閲忕殑鎷疯礉銆傛澶栵紝dm-clone 浼氳€冭檻鍙戝線杩欎簺璁惧鐨勭敤鎴?I/O 娴侀噺锛屽苟鍦ㄦ湁 I/O 鍦ㄩ€旀椂鏆傚仠鍚庡彴姘村悎銆?
鍙互浣跨敤娑堟伅 `hydration_threshold <#regions>` 鏉ヨ缃鎷疯礉鍖哄煙鐨勬渶澶ф暟閲忥紝榛樿涓?1 涓尯鍩熴€?
dm-clone 浣跨敤 dm-kcopyd 鏉ュ皢婧愯澶囩殑閮ㄥ垎鍐呭鎷疯礉鍒扮洰鍦拌澶囥€傞粯璁ゆ儏鍐典笅锛屾垜浠彂鍑哄ぇ灏忕瓑浜庡尯鍩熷ぇ灏忕殑鎷疯礉璇锋眰銆傚彲浠ヤ娇鐢ㄦ秷鎭?`hydration_batch_size <#regions>` 鏉ヨ皟鏁磋繖浜涙嫹璐濊姹傜殑澶у皬銆傚澶ф按鍚堟壒澶у皬浼氫娇 dm-clone 灏濊瘯灏嗚繛缁殑澶氫釜鍖哄煙鍚堝苟鎴愭壒锛屼粠鑰屾垜浠垚鎵瑰湴锛堟瘡鎵硅繖涔堝涓尯鍩燂級浠庢簮璁惧鎷疯礉鏁版嵁鍒扮洰鍦拌澶囥€?
褰撶洰鍦拌澶囩殑姘村悎瀹屾垚鏃讹紝浼氬悜鐢ㄦ埛绌洪棿鍙戦€佷竴涓?dm 浜嬩欢銆?

### 鏇存柊纾佺洏涓婄殑鍏冩暟鎹?

姣忔鍐欏叆涓€涓?FLUSH 鎴?FUA bio 鏃讹紝纾佺洏涓婄殑鍏冩暟鎹兘浼氳鎻愪氦銆傚鏋滄病鏈夊彂鍑烘绫昏姹傦紝鍒欐彁浜や細姣忎竴绉掑彂鐢熶竴娆°€傝繖鎰忓懗鐫€ dm-clone 璁惧鐨勮涓虹被浼间簬涓€涓甫鏈夋槗澶辨€у啓缂撳瓨鐨勭墿鐞嗙鐩樸€傚鏋滄柇鐢碉紝浣犲彲鑳戒細涓㈠け涓€浜涙渶杩戠殑鍐欏叆銆傚敖绠″姝わ紝鍦ㄤ换浣曞穿婧冧箣鍚庡厓鏁版嵁閮藉簲褰撲繚鎸佷竴鑷淬€?

## 鐩爣鎺ュ彛


### 鏋勯€犲嚱鏁?

```
   clone <metadata dev> <destination dev> <source dev> <region size>
         [<#feature args> [<feature arg>]* [<#core args> [<core arg>]*]]

 ================ ==============================================================
 metadata dev     Fast device holding the persistent metadata
 destination dev  The destination device, where the source will be cloned
 source dev       Read only device containing the data that gets cloned
 region size      The size of a region in sectors

 #feature args    Number of feature arguments passed
 feature args     no_hydration or no_discard_passdown

 #core args       An even number of arguments corresponding to key/value pairs
                  passed to dm-clone
 core args        Key/value pairs passed to dm-clone, e.g. `hydration_threshold
                  256`
 ================ ==============================================================

```
鍙€夌殑鐗规€у弬鏁版湁锛?
 ==================== =========================================================
 no_hydration         鍒涘缓涓€涓鐢ㄤ簡鍚庡彴姘村悎鐨?dm-clone 瀹炰緥
 no_discard_passdown  绂佹灏?discard 鍚戜笅閫忎紶鍒扮洰鍦拌澶? ==================== =========================================================

鍙€夌殑鏍稿績鍙傛暟鏈夛細

 ================================ ==============================================
 hydration_threshold <#regions>   鍦ㄥ悗鍙版按鍚堟湡闂达紝浠讳綍鏃跺埢浠庢簮璁惧鎷疯礉鍒扮洰鍦?                                  璁惧鐨勫尯鍩熺殑鏈€澶ф暟閲忋€? hydration_batch_size <#regions>  鍦ㄥ悗鍙版按鍚堟湡闂达紝灏濊瘯灏嗚繛缁殑澶氫釜鍖哄煙鍚堝苟鎴?                                  鎵癸紝浠庤€屾垜浠垚鎵瑰湴锛堟瘡鎵硅繖涔堝涓尯鍩燂級浠庢簮
                                  璁惧鎷疯礉鏁版嵁鍒扮洰鍦拌澶囥€? ================================ ==============================================

### 鐘舵€?

```
   <metadata block size> <#used metadata blocks>/<#total metadata blocks>
   <region size> <#hydrated regions>/<#total regions> <#hydrating regions>
   <#feature args> <feature args>* <#core args> <core args>*
   <clone metadata mode>

 ======================= =======================================================
 metadata block size     Fixed block size for each metadata block in sectors
 #used metadata blocks   Number of metadata blocks used
 #total metadata blocks  Total number of metadata blocks
 region size             Configurable region size for the device in sectors
 #hydrated regions       Number of regions that have finished hydrating
 #total regions          Total number of regions to hydrate
 #hydrating regions      Number of regions currently hydrating
 #feature args           Number of feature arguments to follow
 feature args            Feature arguments, e.g. `no_hydration`
 #core args              Even number of core arguments to follow
 core args               Key/value pairs for tuning the core, e.g.
                         `hydration_threshold 256`
 clone metadata mode     ro if read-only, rw if read-write

                         In serious cases where even a read-only mode is deemed
                         unsafe no further I/O will be permitted and the status
                         will just contain the string 'Fail'. If the metadata
                         mode changes, a dm event will be sent to user space.
 ======================= =======================================================

```
### 娑堟伅


  `disable_hydration`
      绂佺敤鐩湴璁惧鐨勫悗鍙版按鍚堛€?
  `enable_hydration`
      鍚敤鐩湴璁惧鐨勫悗鍙版按鍚堛€?
  `hydration_threshold <#regions>`
      璁剧疆鍚庡彴姘村悎闃堝€笺€?
  `hydration_batch_size <#regions>`
      璁剧疆鍚庡彴姘村悎鎵瑰ぇ灏忋€?

## 绀轰緥


### 鍏嬮殕涓€涓寘鍚枃浠剁郴缁熺殑璁惧


1. 鍒涘缓 dm-clone 璁惧銆?
```

    dmsetup create clone --table "0 1048576000 clone $metadata_dev $dest_dev \
      $source_dev 8 1 no_hydration"

```
2. 鎸傝浇璇ヨ澶囧苟瀵规枃浠剁郴缁熻繘琛?trim銆俤m-clone 浼氳В閲婃枃浠剁郴缁熷彂鍑虹殑 discard锛屽苟涓嶄細瀵规湭浣跨敤鐨勭┖闂磋繘琛屾按鍚堛€?
```

    mount /dev/mapper/clone /mnt/cloned-fs
    fstrim /mnt/cloned-fs

```
3. 鍚敤鐩湴璁惧鐨勫悗鍙版按鍚堛€?
```

    dmsetup message clone 0 enable_hydration

```
4. 褰撴按鍚堝畬鎴愭椂锛屾垜浠彲浠ョ敤涓€涓嚎鎬ц〃鏇挎崲 dm-clone 琛ㄣ€?
```

    dmsetup suspend clone
    dmsetup load clone --table "0 1048576000 linear $dest_dev 0"
    dmsetup resume clone

   The metadata device is no longer needed and can be safely discarded or reused
   for other purposes.

```
## 宸茬煡闂


1. 鎴戜滑灏嗗灏氭湭姘村悎鍖哄煙鐨勮閲嶅畾鍚戝埌婧愯澶囥€傚鏋滆鍙栨簮璁惧鐨勫欢杩熷緢楂橈紝鑰岀敤鎴峰弽澶嶈鍙栫浉鍚岀殑鍖哄煙锛岃繖绉嶈涓轰細闄嶄綆鎬ц兘銆傛垜浠簲璇ュ皢杩欎簺璇讳綔涓烘彁绀猴紝浠ヤ究灏藉揩姘村悎鐩稿叧鍖哄煙銆傜洰鍓嶆垜浠緷璧栭〉缂撳瓨鏉ョ紦瀛樿繖浜涘尯鍩燂紝鎵€浠ュ笇鏈涙垜浠笉浼氫粠婧愯澶囧娆¤鍙栧畠浠€?
2. 鍦ㄦ按鍚堝畬鎴愬悗锛岄噴鏀炬牳蹇冨唴璧勬簮锛堝嵆璺熻釜鍝簺鍖哄煙宸茶姘村悎鐨勪綅鍥撅級銆?
3. 鍦ㄥ悗鍙版按鍚堟湡闂达紝濡傛灉鎴戜滑鏃犳硶璇诲彇婧愯澶囨垨鍐欏叆鐩湴璁惧锛屾垜浠細鎵撳嵃涓€鏉￠敊璇秷鎭紝浣嗘按鍚堣繃绋嬩細鏃犻檺鏈熷湴缁х画锛岀洿鍒版垚鍔熶负姝€傛垜浠簲璇ュ湪澶辫触鑻ュ共娆″悗鍋滄鍚庡彴姘村悎锛屽苟鍙戝嚭涓€涓?dm 浜嬩欢浠ヤ究鐢ㄦ埛绌洪棿瀵熻銆?

## 涓轰粈涔堜笉鐢ㄢ€︹€︼紵


鍦ㄥ疄鐜?dm-clone 涔嬪墠锛屾垜浠帰璁ㄤ簡浠ヤ笅鏇夸唬鏂规锛?
1. 浣跨敤 dm-cache锛屽叾缂撳瓨澶у皬绛変簬婧愯澶囷紝骞跺疄鐜颁竴绉嶆柊鐨勫厠闅嗙瓥鐣ワ細

   - 鐢熸垚鐨勭紦瀛樿澶囧苟涓嶆槸婧愯澶囩殑涓€瀵逛竴闀滃儚锛屽洜姝ゆ垜浠湪鍏嬮殕瀹屾垚鍚庢棤娉曠Щ闄ょ紦瀛樿澶囥€?
   - dm-cache 浼氬啓鍏ユ簮璁惧锛岃繖杩濆弽浜嗘垜浠姹傛簮璁惧蹇呴』琚涓哄彧璇荤殑鏉′欢銆?
   - 缂撳瓨涓庡厠闅嗗湪璇箟涓婃槸涓嶅悓鐨勩€?
2. 浣跨敤 dm-snapshot锛屽叾 COW 璁惧绛変簬婧愯澶囷細

   - dm-snapshot 灏嗗叾鍏冩暟鎹瓨鍌ㄥ湪 COW 璁惧涓紝鍥犳鐢熸垚鐨勮澶囧苟涓嶆槸婧愯澶囩殑涓€瀵逛竴闀滃儚銆?
   - 娌℃湁鍚庡彴鎷疯礉鏈哄埗銆?
   - dm-snapshot 闇€瑕佸湪姣忎釜寰呭鐞嗗紓甯革紙pending exception锛夊畬鎴愭椂鎻愪氦鍏跺厓鏁版嵁锛屼互淇濊瘉蹇収涓€鑷存€с€傝€屽湪鍏嬮殕鐨勬儏鍐典笅锛屾垜浠笉闇€瑕佸姝や弗鏍硷紝鍙互渚濊禆姣忔鍐欏叆 FLUSH 鎴?FUA bio 鏃舵彁浜ゅ厓鏁版嵁锛屾垨鑰呭懆鏈熸€у湴鎻愪氦锛屽氨鍍?dm-thin 鍜?dm-cache 鎵€鍋氱殑閭ｆ牱銆傝繖鏄捐憲鎻愬崌浜嗘€ц兘銆?
3. 浣跨敤 dm-mirror锛歮irror 鐩爣鏈夊悗鍙版嫹璐?闀滃儚鏈哄埗锛屼絾瀹冧細鍐欏叆鎵€鏈夌殑闀滃儚锛屼粠鑰岃繚鍙嶄簡鎴戜滑瑕佹眰婧愯澶囧繀椤昏瑙嗕负鍙鐨勬潯浠躲€?
4. 浣跨敤 dm-thin 鐨勫閮ㄥ揩鐓у姛鑳姐€傚湪鎵€鏈夋浛浠ｆ柟妗堜腑锛岃繖绉嶆柟娉曟渶鏈夊墠閫旓紝鍥犱负鐦︿緵缁欏嵎鏄簮璁惧鐨勪竴瀵逛竴闀滃儚锛屽苟涓斾互涓?dm-clone 鐩稿悓鐨勬柟寮忓鐞嗗鏈緵缁?灏氭湭鍏嬮殕鍖哄煙鐨勮鍐欍€?
   灏界濡傛锛?
   - 娌℃湁鍚庡彴鎷疯礉鏈哄埗锛屽敖绠″彲浠ュ疄鐜颁竴涓€?
   - 鏈€閲嶈鐨勬槸锛屾垜浠笇鏈涙敮鎸佷换鎰忓潡璁惧浣滀负鍏嬮殕杩囩▼鐨勭洰鍦帮紝鑰屼笉鏄皢鑷繁闄愬埗鍦ㄧ槮渚涚粰鍗蜂笂銆傜槮渚涚粰涓轰簡缁存姢鐦﹀嵎鏄犲皠鑰屽叿鏈夊浐鏈夌殑鍏冩暟鎹紑閿€锛岃繖浼氭樉钁楅檷浣庢€ц兘銆?
   姝ゅ锛屽厠闅嗕竴涓澶囦笉搴斿己鍒朵娇鐢ㄧ槮渚涚粰銆傚彟涓€鏂归潰锛屽鏋滄垜浠笇鏈涗娇鐢ㄧ槮渚涚粰锛屾垜浠彧闇€浣跨敤涓€涓槮 LV 浣滀负 dm-clone 鐨勭洰鍦拌澶囧嵆鍙€?