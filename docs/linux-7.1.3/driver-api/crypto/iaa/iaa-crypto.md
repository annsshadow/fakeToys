
## IAA 鍘嬬缉鍔犻€熷櫒鍔犲瘑椹卞姩


Tom Zanussi <tom.zanussi@linux.intel.com>

IAA 鍔犲瘑椹卞姩鏀寔绗﹀悎 RFC 1951 鎵€鎻忚堪 DEFLATE 鍘嬬缉鏍囧噯鐨勫帇缂?瑙ｅ帇缂╋紝
杩欎篃鏄湰妯″潡瀵煎嚭鐨勫帇缂?瑙ｅ帇缂╃畻娉曘€?

IAA 纭欢瑙勬牸鍙湪姝ゅ鎵惧埌锛?

  https://cdrdv2.intel.com/v1/dl/getContent/721858

iaa_crypto 椹卞姩琚璁′负楂橀樁灞傚帇缂╄澶囷紙濡?zswap锛変箣涓嬬殑涓€灞傘€?

鐢ㄦ埛鍙互閫氳繃鍦ㄥ厑璁搁€夋嫨鍘嬬缉绠楁硶鐨勪换浣曡鏂戒腑鎸囧畾鍙楁敮鎸佺殑 IAA 鍘嬬缉绠楁硶涔嬩竴锛?
鏉ラ€夋嫨 IAA 鍘嬬缉/瑙ｅ帇缂╁姞閫熴€?

渚嬪锛寊swap 璁惧鍙互閫氳繃閫夋嫨 'deflate-iaa' 鍔犲瘑鍘嬬缉绠楁硶鏉ラ€夋嫨 IAA 鐨?
'fixed' 妯″紡锛?
```
  # echo deflate-iaa > /sys/module/zswap/parameters/compressor

```
杩欏皢鍛婄煡 zswap 鍦ㄦ墍鏈夊帇缂╁拰瑙ｅ帇缂╀腑浣跨敤 IAA 鐨?'fixed' 鍘嬬缉妯″紡銆?

鐩墠鍙湁涓€绉嶅帇缂╂ā寮忓彲鐢紝鍗?'fixed' 妯″紡銆?

'fixed' 鍘嬬缉妯″紡瀹炵幇浜?RFC 1951 鎵€鎸囧畾鐨勫帇缂╂柟妗堬紝骞惰璧嬩簣鍔犲瘑绠楁硶鍚嶇О
'deflate-iaa'銆傦紙鐢变簬 IAA 纭欢鍏锋湁 4k 鍘嗗彶绐楀彛闄愬埗锛屽彧鏈?<= 4k 鐨勭紦鍐插尯锛?
鎴栭噰鐢?<= 4k 鍘嗗彶绐楀彛鍘嬬缉鐨勭紦鍐插尯锛屾墠鍦ㄦ妧鏈笂绗﹀悎 deflate 瑙勮寖锛岃€岃瑙勮寖
鍏佽鏈€澶?32k 鐨勭獥鍙ｃ€傜敱浜庢闄愬埗锛孖AA fixed 妯″紡 deflate 绠楁硶琚祴浜堜簡鑷繁鐨?
绠楁硶鍚嶇О锛岃€岄潪绠€鍗曠殑 'deflate'锛夈€?


## 閰嶇疆閫夐」涓庡叾浠栬缃?


IAA 鍔犲瘑椹卞姩鍙€氳繃 menuconfig 浣跨敤濡備笅閫夐」鑾峰緱锛?
```
  Cryptographic API -> Hardware crypto devices -> Support for Intel(R) IAA Compression Accelerator

```
鍦ㄩ厤缃枃浠朵腑锛岃閫夐」鍚嶄负 CONFIG_CRYPTO_DEV_IAA_CRYPTO銆?

IAA 鍔犲瘑椹卞姩杩樻敮鎸佺粺璁″姛鑳斤紝鍙€氳繃浠ヤ笅閫夐」鑾峰緱锛?
```
  Cryptographic API -> Hardware crypto devices -> Support for Intel(R) IAA Compression -> Enable Intel(R) IAA Compression Accelerator Statistics

```
鍦ㄩ厤缃枃浠朵腑锛岃閫夐」鍚嶄负 CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS銆?

```
  CONFIG_IRQ_REMAP=y
  CONFIG_INTEL_IOMMU=y
  CONFIG_INTEL_IOMMU_SVM=y
  CONFIG_PCI_ATS=y
  CONFIG_PCI_PRI=y
  CONFIG_PCI_PASID=y
  CONFIG_INTEL_IDXD=m
  CONFIG_INTEL_IDXD_SVM=y

```
IAA 鏄彲涓?Intel IOMMU 鍗忓悓宸ヤ綔鐨勯鎵?Intel 鍔犻€熷櫒 IP 涔嬩竴銆傚瓨鍦ㄥ绉嶆ā寮忥細
```
  - Scalable
  - Legacy
  - No IOMMU


```
### 鍙墿灞曟ā寮忥紙Scalable mode锛?


鍙墿灞曟ā寮忔敮鎸佸叡浜櫄鎷熷唴瀛橈紙SVM 鎴?SVA锛夈€傚畠閫氳繃浠ヤ笅鏂瑰紡鍚敤锛?
```
  intel_iommu=on,sm_on

```
涓?BIOS 涓紑鍚簡 VT-d銆?

鍦ㄥ彲鎵╁睍妯″紡涓嬶紝鍏变韩鍜屼笓鐢ㄥ伐浣滈槦鍒楀潎鍙娇鐢ㄣ€?

```
  Socket Configuration > IIO Configuration > Intel VT for Directed I/O (VT-d) > Intel VT for Directed I/O

  Socket Configuration > IIO Configuration > PCIe ENQCMD > ENQCMDS


```
### 浼犵粺妯″紡锛圠egacy mode锛?


```
  intel_iommu=off

```
鎴?BIOS 涓湭寮€鍚?VT-d銆?

濡傛灉浣犲凡鍚姩杩涘叆 Linux 浣嗕笉纭畾 VT-d 鏄惁寮€鍚紝鍙墽琛?"dmesg | grep -i dmar"銆?
濡傛灉娌℃湁鐪嬪埌鑻ュ共 DMAR 璁惧琚灇涓撅紝鍒欏緢鍙兘 VT-d 鏈紑鍚€?

鍦ㄤ紶缁熸ā寮忎笅锛屽彧鏈変笓鐢ㄥ伐浣滈槦鍒楀彲渚涗娇鐢ㄣ€?


### 鏃?IOMMU 妯″紡锛圢o IOMMU mode锛?


```
  iommu=off.

```
鍦ㄦ棤 IOMMU 妯″紡涓嬶紝鍙湁涓撶敤宸ヤ綔闃熷垪鍙緵浣跨敤銆?


## 浣跨敤鏂规硶


### accel-config


鍔犺浇鏃讹紝iaa_crypto 椹卞姩浼氳嚜鍔ㄥ垱寤轰竴涓粯璁ら厤缃苟鍚敤瀹冿紝鍚屾椂鍒嗛厤榛樿椹卞姩灞炴€с€?
濡傛灉闇€瑕佷笉鍚岀殑閰嶇疆鎴栭┍鍔ㄥ睘鎬ч泦鍚堬紝鐢ㄦ埛蹇呴』鍏堢鐢?IAA 璁惧鍜屽伐浣滈槦鍒椼€侀噸缃厤缃紝
鐒跺悗閫氳繃绉婚櫎骞堕噸鏂版彃鍏?iaa_crypto 妯″潡锛屽悜鍔犲瘑瀛愮郴缁熼噸鏂版敞鍐?deflate-iaa 绠楁硶銆?

涓嬮潰銆庣敤渚嬨€忓皬鑺備腑鐨?iaa_disable_script 鍙敤浜庣鐢ㄩ粯璁ら厤缃€?

鏈夊叧榛樿閰嶇疆鐨勮缁嗕俊鎭紝璇峰弬瑙佷笅鏂囩殑 iaa_default_config銆?

涓嶈繃锛岀敱浜庡姞閫熷櫒璁惧鐨勫鏉傛€у拰鍙厤缃€э紝鐢ㄦ埛鏇村彲鑳介渶瑕侀厤缃澶囧苟鎵嬪姩鍚敤鎵€闇€鐨?
璁惧鍜屽伐浣滈槦鍒椼€?

甯姪鐢ㄦ埛瀹屾垚姝ゆ搷浣滅殑鐢ㄦ埛绌洪棿宸ュ叿鍚嶄负 accel-config銆傚己鐑堝缓璁娇鐢?accel-config
鏉ラ厤缃澶囨垨鍔犺浇鍏堝墠淇濆瓨鐨勯厤缃€備篃鍙互閫氳繃 sysfs 鐩存帴鎺у埗璁惧锛屼絾闇€鐗瑰埆璀﹀憡锛?
鍙湁鍦ㄤ綘纭垏鐭ラ亾鑷繁鍦ㄥ仛浠€涔堟椂鎵嶅簲杩欐牱鍋氥€傚悗缁珷鑺備笉浼氭兜鐩?sysfs 鎺ュ彛锛岃€屾槸鍋囧畾
浣犲皢浣跨敤 accel-config銆?

濡傛湁鍏磋叮锛屽彲鏌ラ槄闄勫綍涓殑 iaa_sysfs_config 灏忚妭浠ヤ簡瑙?sysfs 鎺ュ彛璇︽儏銆?

accel-config 宸ュ叿鍙婂叾鏋勫缓璇存槑鍙湪姝ゅ鎵惧埌锛?

  https://github.com/intel/idxd-config/#readme

### 鍏稿瀷鐢ㄦ硶


涓轰簡璁?iaa_crypto 妯″潡鐪熸浠ｈ〃鏌愪釜璁炬柦鎵ц鍘嬬缉/瑙ｅ帇缂╁伐浣滐紝闇€瑕佸皢涓€涓垨澶氫釜
IAA 宸ヤ綔闃熷垪缁戝畾鍒?iaa_crypto 椹卞姩銆?

渚嬪锛屼笅闈㈡槸涓€涓厤缃?IAA 宸ヤ綔闃熷垪骞跺皢鍏剁粦瀹氬埌 iaa_crypto 椹卞姩鐨勭ず渚嬶紙娉ㄦ剰璁惧鍚?
浠?'iax' 鑰岄潪 'iaa' 鎸囧畾鈥斺€旇繖鏄洜涓轰笂娓镐粛鐒?
```
  # configure wq1.0

  accel-config config-wq --group-id=0 --mode=dedicated --type=kernel --priority=10 --name="iaa_crypto" --driver-name="crypto" iax1/wq1.0

  accel-config config-engine iax1/engine1.0 --group-id=0

  # enable IAA device iax1

  accel-config enable-device iax1

  # enable wq1.0 on IAX device iax1

  accel-config enable-wq iax1/wq1.0

```
姣忓綋鏈夋柊鐨勫伐浣滈槦鍒楃粦瀹氬埌鎴栬В缁戣嚜 iaa_crypto 椹卞姩鏃讹紝鍙敤鐨勫伐浣滈槦鍒椾細琚€庨噸鏂板钩琛°€忥紝
浣垮緱浠庣壒瀹?CPU 鎻愪氦鐨勫伐浣滆鍒嗛厤缁欐渶鍚堥€傜殑鍙敤宸ヤ綔闃熷垪銆傚綋鍓嶇殑鏈€浣冲疄璺垫槸涓烘瘡涓?IAA
璁惧閰嶇疆骞剁粦瀹氳嚦灏戜竴涓伐浣滈槦鍒楋紝浣嗗彧瑕佺郴缁熶腑瀛樺湪鑷冲皯涓€涓厤缃苟缁戝畾鍒颁换鎰?IAA 璁惧鐨?
宸ヤ綔闃熷垪锛宨aa_crypto 椹卞姩灏辫兘宸ヤ綔锛屽敖绠℃晥鐜囧緢鍙兘涓嶅鍓嶈€呫€?

鍦ㄧ涓€涓?IAA 宸ヤ綔闃熷垪鎴愬姛缁戝畾鍒?iaa_crypto 椹卞姩鍚庯紝IAA 鍔犲瘑绠楁硶鍗宠繘鍏ュ彲杩愯鐘舵€侊紝
鍘嬬缉鍜岃В鍘嬬缉鎿嶄綔琚畬鍏ㄥ惎鐢ㄣ€?

绫讳技鍦帮紝鍦ㄦ渶鍚庝竴涓?IAA 宸ヤ綔闃熷垪浠?iaa_crypto 椹卞姩瑙ｇ粦鍚庯紝IAA 鍔犲瘑绠楁硶灏嗕笉鍐嶅彲杩愯锛?
鍘嬬缉鍜岃В鍘嬬缉鎿嶄綔琚鐢ㄣ€?

鍥犳锛屽彧鏈夊綋涓€涓垨澶氫釜宸ヤ綔闃熷垪缁戝畾鍒?iaa_crypto 椹卞姩鏃讹紝IAA 鍔犲瘑绠楁硶浠ュ強 IAA 纭欢
鎵嶅彲鐢ㄣ€?

褰撴病鏈?IAA 宸ヤ綔闃熷垪缁戝畾鍒伴┍鍔ㄦ椂锛屽彲浠ラ€氳繃绉婚櫎妯″潡鏉ユ敞閿€ IAA 鍔犲瘑绠楁硶銆?


### 椹卞姩灞炴€?


鏈夎嫢骞茬敤鎴峰彲閰嶇疆鐨勯┍鍔ㄥ睘鎬у彲鐢ㄤ簬閰嶇疆鍚勭鎿嶄綔妯″紡銆傚畠浠強鍏堕粯璁ゅ€煎涓嬫墍鍒椼€傝璁剧疆
鍏朵腑浠讳竴灞炴€э紝璇峰皢鐩稿簲鍊?echo 鍒颁綅浜?/sys/bus/dsa/drivers/crypto/ 涓嬬殑灞炴€ф枃浠朵腑銆?

鍦?IAA 绠楁硶娉ㄥ唽鏃舵崟鑾风殑灞炴€ц缃細琚繚瀛樺湪鍚勭畻娉曠殑 crypto_ctx 涓紝骞跺湪浣跨敤璇ョ畻娉曟椂
搴旂敤浜庢墍鏈夊帇缂╁拰瑙ｅ帇缂┿€?

鍙敤灞炴€у涓嬶細

  - verify_compress

    鍒囨崲鍘嬬缉鏍￠獙銆傝嫢璁剧疆锛屾瘡娆″帇缂╁皢鍦ㄥ唴閮ㄨ繘琛岃В鍘嬬缉骞舵牎楠屽唴瀹癸紝杩斿洖閿欒锛?
```
      echo 0 > /sys/bus/dsa/drivers/crypto/verify_compress

```
    榛樿璁剧疆涓?'1'鈥斺€旀牎楠屾墍鏈夊帇缂┿€?

  - sync_mode

    閫夋嫨鐢ㄤ簬绛夊緟姣忔鍘嬬缉鍜岃В鍘嬬缉鎿嶄綔瀹屾垚鐨勬ā寮忋€?

    iaa_crypto 瀹炵幇鐨勫姞瀵嗗紓姝ユ帴鍙ｆ敮鎸佹彁渚涗簡涓€涓弧瓒宠鎺ュ彛鐨勫疄鐜帮紝浣嗛噰鐢ㄧ殑鏄悓姝ユ柟寮忊€斺€?
    瀹冨～鍏呭苟鎻愪氦 IDXD 鎻忚堪绗︼紝鐒跺悗寰幆绛夊緟鍏跺畬鎴愬啀杩斿洖銆傜洰鍓嶈繖涓嶆槸闂锛屽洜涓烘墍鏈夌幇鏈?
    璋冪敤鑰咃紙渚嬪 zswap锛夐兘浼氬皢浠讳綍寮傛琚皟鐢ㄨ€呭寘瑁呭湪鍚屾鍖呰鍣ㄤ腑銆?

    iaa_crypto 椹卞姩纭疄涓鸿兘澶熷埄鐢ㄥ畠鐨勮皟鐢ㄨ€呮彁渚涗簡鐪熸鐨勫紓姝ユ敮鎸併€傚湪姝ゆā寮忎笅锛屽畠濉厖骞?
    鎻愪氦 IDXD 鎻忚堪绗︼紝鐒跺悗绔嬪嵆浠?-EINPROGRESS 杩斿洖銆傝皟鐢ㄨ€呴殢鍚庡彲浠ヨ嚜琛岃疆璇㈠畬鎴愶紙杩欓渶瑕佸湪
    璋冪敤鑰呬腑鍖呭惈鐗瑰畾浠ｇ爜锛岀洰鍓嶄笂娓稿唴鏍镐腑娌℃湁浠讳綍瀹炵幇锛夛紝鎴栬€呰繘鍏ョ潯鐪犲苟绛夊緟鍙戝嚭瀹屾垚淇″彿鐨?
    涓柇銆傚悗涓€绉嶆ā寮忓彈鍒板唴鏍镐腑鐜版湁鐢ㄦ埛锛堝閫氳繃鍚屾鍖呰鍣ㄧ殑 zswap锛夌殑鏀寔銆傚敖绠″彈鏀寔锛屼絾
    姝ゆā寮忔瘮鍓嶈堪鍦?iaa_crypto 椹卞姩涓繘琛岃疆璇㈢殑鍚屾妯″紡鏄庢樉鎱㈠緱澶氥€?

    鍙互閫氳繃灏?'async_irq' 鍐欏叆 sync_mode iaa_crypto 椹卞姩灞炴€ф潵鍚敤姝ゆā寮忥細

      echo async_irq > /sys/bus/dsa/drivers/crypto/sync_mode

    鏃犱腑鏂殑寮傛妯″紡锛堣皟鐢ㄨ€呭繀椤昏疆璇級鍙€氳繃鍚戝叾鍐欏叆 'async' 鏉ュ惎鐢紙璇峰弬闃呮敞鎰忎簨椤癸級锛?

      echo async > /sys/bus/dsa/drivers/crypto/sync_mode

    鍦?iaa_crypto 椹卞姩涓繘琛岃疆璇㈢殑妯″紡鍙€氳繃鍚戝叾鍐欏叆 'sync' 鏉ュ惎鐢細

      echo sync > /sys/bus/dsa/drivers/crypto/sync_mode

    榛樿妯″紡涓?'sync'銆?

    娉ㄦ剰浜嬮」锛氱敱浜?iaa_crypto 褰撳墠瀹炵幇鐨勫敮涓€鏃犱腑鏂紓姝ヨ疆璇㈡満鍒舵槸閫氳繃鍓嶈堪鐨?'sync' 妯″紡锛?
    鍚?'/sys/bus/dsa/drivers/crypto/sync_mode' 鍐欏叆 'async' 浼氬湪鍐呴儴鍚敤 'sync' 妯″紡銆?
    杩欐槸涓轰簡纭繚 iaa_crypto 鐨勬纭涓猴紝鐩村埌 iaa_crypto 涓惎鐢ㄧ湡姝ｇ殑鏃犱腑鏂紓姝ヨ疆璇负姝€?

```


### IAA 榛樿閰嶇疆


褰撳姞杞?iaa_crypto 椹卞姩鏃讹紝姣忎釜 IAA 璁惧閮芥湁涓€涓崟鐙殑
```
          mode              "dedicated"
          threshold         0
          size              Total WQ Size from WQCAP
          priority          10
          type              IDXD_WQT_KERNEL
          group             0
          name              "iaa_crypto"
          driver_name       "crypto"

```
杩欎簺璁惧鍙婂伐浣滈槦鍒椾篃宸插惎鐢紝鍥犳璇ラ┍鍔ㄦ棤闇€浠讳綍棰濆閰嶇疆鍗冲彲浣跨敤銆?

```
          sync_mode         "sync"
          verify_compress   1

```
瑕佹洿鏀硅澶?宸ヤ綔闃熷垪鎴栭┍鍔ㄥ睘鎬э紝蹇呴』鍏堢鐢ㄥ凡鍚敤鐨勮澶囧拰宸ヤ綔闃熷垪銆備负浜嗚鏂伴厤缃簲鐢ㄥ埌
deflate-iaa 鍔犲瘑绠楁硶锛岄渶瑕侀€氳繃绉婚櫎骞堕噸鏂版彃鍏?iaa_crypto 妯″潡鏉ラ噸鏂版敞鍐屻€備笅闈€庣敤渚嬨€?
灏忚妭涓殑 iaa_disable_script 鍙敤浜庣鐢ㄩ粯璁ら厤缃€?

## 缁熻淇℃伅


濡傛灉鍚敤浜嗗彲閫夌殑 debugfs 缁熻鏀寔锛孖AA 鍔犲瘑
```
  # ls -al /sys/kernel/debug/iaa-crypto/
  total 0
  drwxr-xr-x  2 root root 0 Mar  3 07:55 .
  drwx------ 53 root root 0 Mar  3 07:55 ..
  -rw-r--r--  1 root root 0 Mar  3 07:55 global_stats
  -rw-r--r--  1 root root 0 Mar  3 07:55 stats_reset
  -rw-r--r--  1 root root 0 Mar  3 07:55 wq_stats

```
global_stats 鏂囦欢鏄剧ず鍦ㄤ互涓嬫椂闂翠互鏉ユ敹闆嗙殑涓€缁勫叏灞€缁熻淇℃伅锛?
```
  # cat global_stats
  global stats:
    total_comp_calls: 4300
    total_decomp_calls: 4164
    total_sw_decomp_calls: 0
    total_comp_bytes_out: 5993989
    total_decomp_bytes_in: 5993989
    total_completion_einval_errors: 0
    total_completion_timeout_errors: 0
    total_completion_comp_buf_overflow_errors: 136

```
wq_stats 鏂囦欢鏄剧ず姣忎釜宸ヤ綔闃熷垪鐨勭粺璁′俊鎭紝涓烘瘡涓?iaa 璁惧鍙婂伐浣滈槦鍒楀悇鎻愪緵涓€缁勶細
```
  # cat wq_stats
  iaa device:
    id: 1
    n_wqs: 1
    comp_calls: 0
    comp_bytes: 0
    decomp_calls: 0
    decomp_bytes: 0
    wqs:
      name: iaa_crypto
      comp_calls: 0
      comp_bytes: 0
      decomp_calls: 0
      decomp_bytes: 0

  iaa device:
    id: 3
    n_wqs: 1
    comp_calls: 0
    comp_bytes: 0
    decomp_calls: 0
    decomp_bytes: 0
    wqs:
      name: iaa_crypto
      comp_calls: 0
      comp_bytes: 0
      decomp_calls: 0
      decomp_bytes: 0

  iaa device:
    id: 5
    n_wqs: 1
    comp_calls: 1360
    comp_bytes: 1999776
    decomp_calls: 0
    decomp_bytes: 0
    wqs:
      name: iaa_crypto
      comp_calls: 1360
      comp_bytes: 1999776
      decomp_calls: 0
      decomp_bytes: 0

  iaa device:
    id: 7
    n_wqs: 1
    comp_calls: 2940
    comp_bytes: 3994213
    decomp_calls: 4164
    decomp_bytes: 5993989
    wqs:
      name: iaa_crypto
      comp_calls: 2940
      comp_bytes: 3994213
      decomp_calls: 4164
      decomp_bytes: 5993989
    ...

```
鍐欏叆 'stats_reset' 浼氶噸缃墍鏈夌粺璁′俊鎭紝鍖呮嫭
```
  # echo 1 > stats_reset
  # cat wq_stats
    global stats:
    total_comp_calls: 0
    total_decomp_calls: 0
    total_comp_bytes_out: 0
    total_decomp_bytes_in: 0
    total_completion_einval_errors: 0
    total_completion_timeout_errors: 0
    total_completion_comp_buf_overflow_errors: 0
    ...


```


## 鐢ㄤ緥


### 绠€鍗曠殑 zswap 娴嬭瘯


鍦ㄦ湰绀轰緥涓紝鍐呮牳搴旀寜鐓т笂鏂囨墍杩颁笓鐢ㄦā寮忛€夐」杩涜閰嶇疆锛屽苟涓?zswap 搴旈€氳繃浠ヤ笅鏂瑰紡鍚敤锛?
```
  CONFIG_ZSWAP=y

```
杩欐槸涓€涓畝鍗曠殑娴嬭瘯锛屼娇鐢?iaa_compress 浣滀负浜ゆ崲锛坺swap锛夎澶囩殑鍘嬬缉鍣ㄣ€傚畠璁剧疆 zswap
璁惧锛岀劧鍚庝娇鐢ㄤ笅闈㈠垪鍑虹殑 memory_memadvise 绋嬪簭寮哄埗鎹㈠嚭鍜屾崲鍏ユ寚瀹氭暟閲忕殑椤碉紝婕旂ず鍘嬬缉
鍜岃В鍘嬬缉銆?

zswap 娴嬭瘯鏈熸湜绯荤粺涓婃瘡涓?IAA 璁惧鐨勫伐浣滈槦鍒楅兘琚纭厤缃负鍐呮牳宸ヤ綔闃熷垪锛屼笖宸ヤ綔闃熷垪
driver_name 涓?"crypto"銆?

```
  modprobe iaa_crypto

```
濡傛灉 IAA 璁惧鍜屽伐浣滈槦鍒椾箣鍓嶆湭琚鐢ㄥ拰閲嶆柊閰嶇疆锛屽垯搴斿綋澶勪簬榛樿閰嶇疆鐘舵€侊紝鏃犻渶杩涗竴姝ョ殑
IAA 閰嶇疆銆傛湁鍏抽粯璁ら厤缃殑璇︾粏淇℃伅锛岃鍙傝涓嬫枃鐨?iaa_default_config銆?

濡傛灉榛樿閰嶇疆宸插氨缁紝浣犲簲褰撶湅鍒?iaa
```
  # cat /sys/bus/dsa/devices/iax1/state
  enabled
  # cat /sys/bus/dsa/devices/iax1/wq1.0/state
  enabled

```
涓轰簡婕旂ず鍚庣画姝ラ鎸夐鏈熷伐浣滐紝杩欎簺
```
  # echo -n 'module iaa_crypto +p' > /sys/kernel/debug/dynamic_debug/control
  # echo -n 'module idxd +p' > /sys/kernel/debug/dynamic_debug/control

```
```
  # echo 0 > /sys/module/zswap/parameters/enabled
  # echo 50 > /sys/module/zswap/parameters/max_pool_percent
  # echo deflate-iaa > /sys/module/zswap/parameters/compressor
  # echo 1 > /sys/module/zswap/parameters/enabled
  # echo 100 > /proc/sys/vm/swappiness
  # echo never > /sys/kernel/mm/transparent_hugepage/enabled
  # echo 1 > /proc/sys/vm/overcommit_memory

```
鐜板湪浣犲彲浠ヨ繍琛屾兂瑕佹祴閲忕殑 zswap 宸ヤ綔璐熻浇浜嗐€備緥濡傦紝浣跨敤涓嬮潰鐨?memory_memadvise 浠ｇ爜锛?
浠ヤ笅鍛戒护
```
  ./memory_madvise 100

  Allocating 100 pages to swap in/out
  Swapping out 100 pages
  Swapping in 100 pages
  Swapped out and in 100 pages

```
```
  [  404.202972] idxd 0000:e7:02.0: iaa_comp_acompress: dma_map_sg, src_addr 223925c000, nr_sgs 1, req->src 00000000ee7cb5e6, req->slen 4096, sg_dma_len(sg) 4096
  [  404.202973] idxd 0000:e7:02.0: iaa_comp_acompress: dma_map_sg, dst_addr 21dadf8000, nr_sgs 1, req->dst 000000008d6acea8, req->dlen 4096, sg_dma_len(sg) 8192
  [  404.202975] idxd 0000:e7:02.0: iaa_compress: desc->src1_addr 223925c000, desc->src1_size 4096, desc->dst_addr 21dadf8000, desc->max_dst_size 4096, desc->src2_addr 2203543000, desc->src2_size 1568
  [  404.202981] idxd 0000:e7:02.0: iaa_compress_verify: (verify) desc->src1_addr 21dadf8000, desc->src1_size 228, desc->dst_addr 223925c000, desc->max_dst_size 4096, desc->src2_addr 0, desc->src2_size 0
  ...

```
鏃㈢劧鍩烘湰鍔熻兘宸叉紨绀哄畬姣曪紝鍙互娓呴櫎榛樿鍊煎苟鏇挎崲涓轰笉鍚岀殑閰嶇疆銆備负姝わ紝
```
  # echo lzo > /sys/module/zswap/parameters/compressor
  # swapoff -a
  # echo 0 > /sys/module/zswap/parameters/accept_threshold_percent
  # echo 0 > /sys/module/zswap/parameters/max_pool_percent
  # echo 0 > /sys/module/zswap/parameters/enabled
  # echo 0 > /sys/module/zswap/parameters/enabled

```
鐒跺悗杩愯涓嬮潰銆庣敤渚嬨€忓皬鑺備腑鐨?iaa_disable_script 鏉ョ鐢ㄩ粯璁ら厤缃€?

```
  # swapon -a

```
瀹屾垚浠ヤ笂鎵€鏈夋楠ゅ悗锛屽彲浠ユ牴鎹渶瑕侀噸鏂伴厤缃苟鍚敤 IAA 璁惧浠ヨ繘琛岃繘涓€姝ユ祴璇曘€備笅闈㈡槸涓€涓?
绀轰緥銆?

zswap 娴嬭瘯鏈熸湜绯荤粺涓婃瘡涓?IAA 璁惧鐨勫伐浣滈槦鍒楅兘琚纭厤缃负鍐呮牳宸ヤ綔闃熷垪锛屼笖宸ヤ綔闃熷垪
driver_name 涓?"crypto"銆?

```
  #!/bin/bash

  echo "IAA devices:"
  lspci -d:0cfe
  echo "# IAA devices:"
  lspci -d:0cfe | wc -l

  #
  # count iaa instances
  #
  iaa_dev_id="0cfe"
  num_iaa=$(lspci -d:${iaa_dev_id} | wc -l)
  echo "Found ${num_iaa} IAA instances"

  #
  # disable iaa wqs and devices
  #
  echo "Disable IAA"

  for ((i = 1; i < ${num_iaa} * 2; i += 2)); do
      echo disable wq iax${i}/wq${i}.0
      accel-config disable-wq iax${i}/wq${i}.0
      echo disable iaa iax${i}
      accel-config disable-device iax${i}
  done

  echo "End Disable IAA"

  echo "Reload iaa_crypto module"

  rmmod iaa_crypto
  modprobe iaa_crypto

  echo "End Reload iaa_crypto module"

  #
  # configure iaa wqs and devices
  #
  echo "Configure IAA"
  for ((i = 1; i < ${num_iaa} * 2; i += 2)); do
      accel-config config-wq --group-id=0 --mode=dedicated --wq-size=128 --priority=10 --type=kernel --name="iaa_crypto" --driver-name="crypto" iax${i}/wq${i}.0
      accel-config config-engine iax${i}/engine${i}.0 --group-id=0
  done

  echo "End Configure IAA"

  #
  # enable iaa wqs and devices
  #
  echo "Enable IAA"

  for ((i = 1; i < ${num_iaa} * 2; i += 2)); do
      echo enable iaa iax${i}
      accel-config enable-device iax${i}
      echo enable wq iax${i}/wq${i}.0
      accel-config enable-wq iax${i}/wq${i}.0
  done

  echo "End Enable IAA"

```
褰撳伐浣滈槦鍒楃粦瀹氬埌 iaa_crypto 椹卞姩鏃讹紝濡傛灉浣犲凡鍚敤璋冭瘯杈撳嚭锛坋cho -n 'module iaa_crypto +p' >锛夛紝
浣犲簲褰撳湪 dmesg 杈撳嚭涓湅鍒扮被浼间互涓嬪唴瀹癸細
```
  [   60.752344] idxd 0000:f6:02.0: add_iaa_wq: added wq 000000004068d14d to iaa 00000000c9585ba2, n_wq 1
  [   60.752346] iaa_crypto: rebalance_wq_table: nr_nodes=2, nr_cpus 160, nr_iaa 8, cpus_per_iaa 20
  [   60.752347] iaa_crypto: rebalance_wq_table: iaa=0
  [   60.752349] idxd 0000:6a:02.0: request_iaa_wq: getting wq from iaa_device 0000000042d7bc52 (0)
  [   60.752350] idxd 0000:6a:02.0: request_iaa_wq: returning unused wq 00000000c8bb4452 (0) from iaa device 0000000042d7bc52 (0)
  [   60.752352] iaa_crypto: rebalance_wq_table: assigned wq for cpu=0, node=0 = wq 00000000c8bb4452
  [   60.752354] iaa_crypto: rebalance_wq_table: iaa=0
  [   60.752355] idxd 0000:6a:02.0: request_iaa_wq: getting wq from iaa_device 0000000042d7bc52 (0)
  [   60.752356] idxd 0000:6a:02.0: request_iaa_wq: returning unused wq 00000000c8bb4452 (0) from iaa device 0000000042d7bc52 (0)
  [   60.752358] iaa_crypto: rebalance_wq_table: assigned wq for cpu=1, node=0 = wq 00000000c8bb4452
  [   60.752359] iaa_crypto: rebalance_wq_table: iaa=0
  [   60.752360] idxd 0000:6a:02.0: request_iaa_wq: getting wq from iaa_device 0000000042d7bc52 (0)
  [   60.752361] idxd 0000:6a:02.0: request_iaa_wq: returning unused wq 00000000c8bb4452 (0) from iaa device 0000000042d7bc52 (0)
  [   60.752362] iaa_crypto: rebalance_wq_table: assigned wq for cpu=2, node=0 = wq 00000000c8bb4452
  [   60.752364] iaa_crypto: rebalance_wq_table: iaa=0
  .
  .
  .

```
涓€鏃﹀伐浣滈槦鍒楀拰璁惧宸插惎鐢紝IAA 鍔犲瘑绠楁硶鍗宠鍚敤骞跺彲鐢ㄣ€傚綋 IAA 鍔犲瘑绠楁硶鎴愬姛鍚敤鍚庯紝
浣犲簲褰撶湅鍒板涓?dmesg
```
  [   64.893759] iaa_crypto: iaa_crypto_enable: iaa_crypto now ENABLED

```
鐜板湪杩愯浠ヤ笅 zswap 涓撶敤璁剧疆鍛戒护锛屼娇 zswap 浣跨敤
```
  echo 0 > /sys/module/zswap/parameters/enabled
  echo 50 > /sys/module/zswap/parameters/max_pool_percent
  echo deflate-iaa > /sys/module/zswap/parameters/compressor
  echo 1 > /sys/module/zswap/parameters/enabled

  echo 100 > /proc/sys/vm/swappiness
  echo never > /sys/kernel/mm/transparent_hugepage/enabled
  echo 1 > /proc/sys/vm/overcommit_memory

```
鏈€鍚庯紝鐜板湪浣犲彲浠ヨ繍琛屾兂瑕佹祴閲忕殑 zswap 宸ヤ綔璐熻浇浜嗐€備緥濡傦紝浣跨敤涓嬮潰鐨勪唬鐮侊紝浠ヤ笅鍛戒护灏?
鎹㈠叆鍜?
```
  ./memory_madvise 100

  Allocating 100 pages to swap in/out
  Swapping out 100 pages
  Swapping in 100 pages
  Swapped out and in 100 pages

```
濡傛灉浣犲凡鍚敤璋冭瘯杈撳嚭锛坋cho -n 'module iaa_crypto +p' >锛夛紝浣犲簲褰撳湪 dmesg 杈撳嚭涓湅鍒?
绫讳技浠ヤ笅鍐呭锛?
```
  [  404.202972] idxd 0000:e7:02.0: iaa_comp_acompress: dma_map_sg, src_addr 223925c000, nr_sgs 1, req->src 00000000ee7cb5e6, req->slen 4096, sg_dma_len(sg) 4096
  [  404.202973] idxd 0000:e7:02.0: iaa_comp_acompress: dma_map_sg, dst_addr 21dadf8000, nr_sgs 1, req->dst 000000008d6acea8, req->dlen 4096, sg_dma_len(sg) 8192
  [  404.202975] idxd 0000:e7:02.0: iaa_compress: desc->src1_addr 223925c000, desc->src1_size 4096, desc->dst_addr 21dadf8000, desc->max_dst_size 4096, desc->src2_addr 2203543000, desc->src2_size 1568
  [  404.202981] idxd 0000:e7:02.0: iaa_compress_verify: (verify) desc->src1_addr 21dadf8000, desc->src1_size 228, desc->dst_addr 223925c000, desc->max_dst_size 4096, desc->src2_addr 0, desc->src2_size 0
  [  409.203227] idxd 0000:e7:02.0: iaa_comp_adecompress: dma_map_sg, src_addr 21ddd8b100, nr_sgs 1, req->src 0000000084adab64, req->slen 228, sg_dma_len(sg) 228
  [  409.203235] idxd 0000:e7:02.0: iaa_comp_adecompress: dma_map_sg, dst_addr 21ee3dc000, nr_sgs 1, req->dst 000000004e2990d0, req->dlen 4096, sg_dma_len(sg) 4096
  [  409.203239] idxd 0000:e7:02.0: iaa_decompress: desc->src1_addr 21ddd8b100, desc->src1_size 228, desc->dst_addr 21ee3dc000, desc->max_dst_size 4096, desc->src2_addr 0, desc->src2_size 0
  [  409.203254] idxd 0000:e7:02.0: iaa_comp_adecompress: dma_map_sg, src_addr 21ddd8b100, nr_sgs 1, req->src 0000000084adab64, req->slen 228, sg_dma_len(sg) 228
  [  409.203256] idxd 0000:e7:02.0: iaa_comp_adecompress: dma_map_sg, dst_addr 21f1551000, nr_sgs 1, req->dst 000000004e2990d0, req->dlen 4096, sg_dma_len(sg) 4096
  [  409.203257] idxd 0000:e7:02.0: iaa_decompress: desc->src1_addr 21ddd8b100, desc->src1_size 228, desc->dst_addr 21f1551000, desc->max_dst_size 4096, desc->src2_addr 0, desc->src2_size 0

```
涓轰簡娉ㄩ攢 IAA 鍔犲瘑绠楁硶骞朵娇鐢ㄤ笉鍚屽弬鏁版敞鍐屾柊绠楁硶锛屽簲褰撳仠姝㈠綋鍓嶇畻娉曠殑浠讳綍浣跨敤鑰咃紝骞剁鐢?
IAA 宸ヤ綔闃熷垪鍜岃澶囥€?

瀵逛簬 zswap锛岄渶瑕佸皢 IAA 鍔犲瘑绠楁硶绉诲嚭鍘嬬缉鍣ㄥ苟鍏抽棴浜ゆ崲锛堜互绉婚櫎瀵?
```
  echo lzo > /sys/module/zswap/parameters/compressor
  swapoff -a

  echo 0 > /sys/module/zswap/parameters/accept_threshold_percent
  echo 0 > /sys/module/zswap/parameters/max_pool_percent
  echo 0 > /sys/module/zswap/parameters/enabled

```
涓€鏃?zswap 琚鐢ㄤ笖涓嶅啀浣跨敤 iaa_crypto锛屽氨鍙互绂佺敤 IAA 宸ヤ綔闃熷垪鍜岃澶囥€?


### IAA 绂佺敤鑴氭湰


```
  #!/bin/bash

  echo "IAA devices:"
  lspci -d:0cfe
  echo "# IAA devices:"
  lspci -d:0cfe | wc -l

  #
  # count iaa instances
  #
  iaa_dev_id="0cfe"
  num_iaa=$(lspci -d:${iaa_dev_id} | wc -l)
  echo "Found ${num_iaa} IAA instances"

  #
  # disable iaa wqs and devices
  #
  echo "Disable IAA"

  for ((i = 1; i < ${num_iaa} * 2; i += 2)); do
      echo disable wq iax${i}/wq${i}.0
      accel-config disable-wq iax${i}/wq${i}.0
      echo disable iaa iax${i}
      accel-config disable-device iax${i}
  done

  echo "End Disable IAA"

```
鏈€鍚庯紝姝ゆ椂鍙互绉婚櫎 iaa_crypto 妯″潡锛岃繖
```
  rmmod iaa_crypto


```
```
  #include <stdio.h>
  #include <stdlib.h>
  #include <string.h>
  #include <unistd.h>
  #include <sys/mman.h>
  #include <linux/mman.h>

  #ifndef MADV_PAGEOUT
  #define MADV_PAGEOUT    21      /* force pages out immediately */
  #endif

  #define PG_SZ           4096

  int main(int argc, char **argv)
  {
        int i, nr_pages = 1;
        int64_t *dump_ptr;
        char *addr, *a;
        int loop = 1;

        if (argc > 1)
                nr_pages = atoi(argv[1]);

        printf("Allocating %d pages to swap in/out\n", nr_pages);

        /* allocate pages */
        addr = mmap(NULL, nr_pages * PG_SZ, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_ANONYMOUS, -1, 0);
        *addr = 1;

        /* initialize data in page to all '*' chars */
        memset(addr, '*', nr_pages * PG_SZ);

         printf("Swapping out %d pages\n", nr_pages);

        /* Tell kernel to swap it out */
        madvise(addr, nr_pages * PG_SZ, MADV_PAGEOUT);

        while (loop > 0) {
                /* Wait for swap out to finish */
                sleep(5);

                a = addr;

                printf("Swapping in %d pages\n", nr_pages);

                /* Access the page ... this will swap it back in again */
                for (i = 0; i < nr_pages; i++) {
                        if (a[0] != '*') {
                                printf("Bad data from decompress!!!!!\n");

                                dump_ptr = (int64_t *)a;
                                 for (int j = 0; j < 100; j++) {
                                        printf("  page %d data: %#llx\n", i, *dump_ptr);
                                        dump_ptr++;
                                }
                        }

                        a += PG_SZ;
                }

                loop --;
        }

       printf("Swapped out and in %d pages\n", nr_pages);

```
## 闄勫綍



### IAA sysfs 閰嶇疆鎺ュ彛


浠ヤ笅鏄 IAA sysfs 鎺ュ彛鐨勬弿杩帮紝姝ｅ涓绘枃妗ｄ腑鎵€鎻愬強锛屽彧鏈夊湪浣犵‘鍒囩煡閬撹嚜宸卞湪鍋氫粈涔堟椂鎵?
搴斾娇鐢ㄥ畠銆傚嵆渚垮姝わ紝涔熸病鏈夊厖鍒嗙殑鐞嗙敱鐩存帴浣跨敤瀹冿紝鍥犱负 accel-config 鑳藉瀹屾垚 sysfs 鎺ュ彛
鍙互鍋氱殑涓€鍒囷紝浜嬪疄涓?accel-config 鍦ㄥ簳灞傛鏄熀浜庡畠瀹炵幇鐨勩€?

銆嶪AA 閰嶇疆璺緞銆忎负 /sys/bus/dsa/devices锛屽叾涓寘鍚唬琛ㄦ瘡涓?IAA 璁惧銆佸伐浣滈槦鍒椼€佸紩鎿庡拰
缁勭殑瀛愮洰褰曘€傛敞鎰忓湪 sysfs 鎺ュ彛涓紝IAA 璁惧瀹為檯涓婁互 iax 鍛藉悕锛屼緥濡?iax1銆乮ax3 绛夈€傦紙娉ㄦ剰
IAA 璁惧鏄鏁扮紪鍙风殑璁惧锛涘伓鏁扮紪鍙风殑璁惧鏄?DSA 璁惧锛屽浜?IAA 鍙互蹇界暐銆傦級

銆嶪AA 璁惧缁戝畾璺緞銆忎负 /sys/bus/dsa/drivers/idxd/bind锛屾槸鍐欏叆浠ュ惎鐢?IAA 璁惧鐨勬枃浠躲€?

銆嶪AA 宸ヤ綔闃熷垪缁戝畾璺緞銆忎负 /sys/bus/dsa/drivers/crypto/bind锛屾槸鍐欏叆浠ュ惎鐢?IAA 宸ヤ綔闃熷垪
鐨勬枃浠躲€?

绫讳技鍦帮紝/sys/bus/dsa/drivers/idxd/unbind 鍜?/sys/bus/dsa/drivers/crypto/unbind 鐢ㄤ簬
绂佺敤 IAA 璁惧鍜屽伐浣滈槦鍒椼€?

璁剧疆 IAA 璁惧鍜屽伐浣滈槦鍒楁墍闇€鐨勫熀鏈懡浠ゅ簭鍒楀涓嬶細

```
鏈€鍚庯紝姝ゆ椂鍙互绉婚櫎 iaa_crypto 妯″潡锛岃繖
```
  rmmod iaa_crypto


```
```
  #include <stdio.h>
  #include <stdlib.h>
  #include <string.h>
  #include <unistd.h>
  #include <sys/mman.h>
  #include <linux/mman.h>

  #ifndef MADV_PAGEOUT
  #define MADV_PAGEOUT    21      /* force pages out immediately */
  #endif

  #define PG_SZ           4096

  int main(int argc, char **argv)
  {
        int i, nr_pages = 1;
        int64_t *dump_ptr;
        char *addr, *a;
        int loop = 1;

        if (argc > 1)
                nr_pages = atoi(argv[1]);

        printf("Allocating %d pages to swap in/out
", nr_pages);

        /* allocate pages */
        addr = mmap(NULL, nr_pages * PG_SZ, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_ANONYMOUS, -1, 0);
        *addr = 1;

        /* initialize data in page to all '*' chars */
        memset(addr, '*', nr_pages * PG_SZ);

         printf("Swapping out %d pages
", nr_pages);

        /* Tell kernel to swap it out */
        madvise(addr, nr_pages * PG_SZ, MADV_PAGEOUT);

        while (loop > 0) {
                /* Wait for swap out to finish */
                sleep(5);

                a = addr;

                printf("Swapping in %d pages
", nr_pages);

                /* Access the page ... this will swap it back in again */
                for (i = 0; i < nr_pages; i++) {
                        if (a[0] != '*') {
                                printf("Bad data from decompress!!!!!
");

                                dump_ptr = (int64_t *)a;
                                 for (int j = 0; j < 100; j++) {
                                        printf("  page %d data: %#llx
", i, *dump_ptr);
                                        dump_ptr++;
                                }
                        }

                        a += PG_SZ;
                }

                loop --;
        }

       printf("Swapped out and in %d pages
", nr_pages);

```
## 闄勫綍



### IAA sysfs 閰嶇疆鎺ュ彛


浠ヤ笅鏄 IAA sysfs 鎺ュ彛鐨勬弿杩帮紝姝ｅ涓绘枃妗ｄ腑鎵€鎻愬強锛屽彧鏈夊湪浣犵‘鍒囩煡閬撹嚜宸卞湪鍋氫粈涔堟椂鎵?
搴斾娇鐢ㄥ畠銆傚嵆渚垮姝わ紝涔熸病鏈夊厖鍒嗙殑鐞嗙敱鐩存帴浣跨敤瀹冿紝鍥犱负 accel-config 鑳藉瀹屾垚 sysfs 鎺ュ彛
鍙互鍋氱殑涓€鍒囷紝浜嬪疄涓?accel-config 鍦ㄥ簳灞傛鏄熀浜庡畠瀹炵幇鐨勩€?

銆嶪AA 閰嶇疆璺緞銆忎负 /sys/bus/dsa/devices锛屽叾涓寘鍚唬琛ㄦ瘡涓?IAA 璁惧銆佸伐浣滈槦鍒椼€佸紩鎿庡拰
缁勭殑瀛愮洰褰曘€傛敞鎰忓湪 sysfs 鎺ュ彛涓紝IAA 璁惧瀹為檯涓婁互 iax 鍛藉悕锛屼緥濡?iax1銆乮ax3 绛夈€傦紙娉ㄦ剰
IAA 璁惧鏄鏁扮紪鍙风殑璁惧锛涘伓鏁扮紪鍙风殑璁惧鏄?DSA 璁惧锛屽浜?IAA 鍙互蹇界暐銆傦級

銆嶪AA 璁惧缁戝畾璺緞銆忎负 /sys/bus/dsa/drivers/idxd/bind锛屾槸鍐欏叆浠ュ惎鐢?IAA 璁惧鐨勬枃浠躲€?

銆嶪AA 宸ヤ綔闃熷垪缁戝畾璺緞銆忎负 /sys/bus/dsa/drivers/crypto/bind锛屾槸鍐欏叆浠ュ惎鐢?IAA 宸ヤ綔闃熷垪
鐨勬枃浠躲€?

绫讳技鍦帮紝/sys/bus/dsa/drivers/idxd/unbind 鍜?/sys/bus/dsa/drivers/crypto/unbind 鐢ㄤ簬
绂佺敤 IAA 璁惧鍜屽伐浣滈槦鍒椼€?

璁剧疆 IAA 璁惧鍜屽伐浣滈槦鍒楁墍闇€鐨勫熀鏈懡浠ゅ簭鍒楀涓嬶細

```
  1) Disable any workqueues enabled on the device.  For example to
     disable workques 0 and 1 on IAA device 3::

       # echo wq3.0 > /sys/bus/dsa/drivers/crypto/unbind
       # echo wq3.1 > /sys/bus/dsa/drivers/crypto/unbind

  2) Disable the device. For example to disable IAA device 3::

       # echo iax3 > /sys/bus/dsa/drivers/idxd/unbind

  3) configure the desired workqueues.  For example, to configure
     workqueue 3 on IAA device 3::

       # echo dedicated > /sys/bus/dsa/devices/iax3/wq3.3/mode
       # echo 128 > /sys/bus/dsa/devices/iax3/wq3.3/size
       # echo 0 > /sys/bus/dsa/devices/iax3/wq3.3/group_id
       # echo 10 > /sys/bus/dsa/devices/iax3/wq3.3/priority
       # echo "kernel" > /sys/bus/dsa/devices/iax3/wq3.3/type
       # echo "iaa_crypto" > /sys/bus/dsa/devices/iax3/wq3.3/name
       # echo "crypto" > /sys/bus/dsa/devices/iax3/wq3.3/driver_name

  4) Enable the device. For example to enable IAA device 3::

       # echo iax3 > /sys/bus/dsa/drivers/idxd/bind

  5) Enable the desired workqueues on the device.  For example to
     enable workques 0 and 1 on IAA device 3::

       # echo wq3.0 > /sys/bus/dsa/drivers/crypto/bind
       # echo wq3.1 > /sys/bus/dsa/drivers/crypto/bind

```
