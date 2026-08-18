
## HPSA - Hewlett Packard Smart Array 椹卞姩


鏈枃浠舵弿杩扮敤浜?HP Smart Array 鎺у埗鍣ㄧ殑 hpsa SCSI 椹卞姩銆?hpsa 椹卞姩鏃ㄥ湪鍙栦唬杈冩柊 Smart Array 鎺у埗鍣ㄧ殑 cciss 椹卞姩銆?hpsa 椹卞姩鏄竴涓?SCSI 椹卞姩锛岃€?cciss 椹卞姩鏄竴涓€滃潡鈥濓紙block锛夐┍鍔ㄣ€?瀹為檯涓?cciss 鏃㈡槸涓€涓潡椹卞姩锛堢敤浜庨€昏緫椹卞姩鍣級涔熸槸涓€涓?SCSI 椹卞姩
锛堢敤浜庣甯︽満锛夈€俢ciss 椹卞姩杩欑鈥滃垎瑁傗€濈殑璁捐鏄繃搴﹀鏉傛€х殑涓€涓潵婧愶紝
鑰屾秷闄よ繖绉嶅鏉傛€ф鏄?hpsa 瀛樺湪鐨勭悊鐢变箣涓€銆?
## 鏀寔鐨勮澶?

- Smart Array P212
- Smart Array P410
- Smart Array P410i
- Smart Array P411
- Smart Array P812
- Smart Array P712m
- Smart Array P711m
- StorageWorks P1210m

姝ゅ锛屽鏋滄寚瀹氫簡鍐呮牳鍚姩鍙傛暟 "hpsa_allow_any=1"锛岃緝鏃х殑 Smart Array
涔熷彲鑳戒笌 hpsa 椹卞姩涓€璧峰伐浣滐紝浣嗚繖浜涘苟鏈粡杩?HP 浣跨敤姝ら┍鍔ㄨ繘琛屾祴璇曟垨鏀寔銆?瀵逛簬杈冩棫鐨?Smart Array锛屼粛搴斾娇鐢?cciss 椹卞姩銆?
"hpsa_simple_mode=1" 鍚姩鍙傛暟鍙敤浜庨樆姝㈤┍鍔ㄥ皢鎺у埗鍣ㄧ疆浜庘€減erformant鈥?妯″紡銆傚尯鍒湪浜庯紝鍦?simple 妯″紡涓嬶紝姣忔鍛戒护瀹屾垚閮介渶瑕佷竴涓腑鏂紝鑰屽湪
鈥減erformant 妯″紡鈥濓紙榛樿涓旈€氬父鎬ц兘鏇村ソ锛変笅锛屽彲浠ョ敱鍗曚釜涓柇鎸囩ず澶氫釜
鍛戒护瀹屾垚銆?
## /sys 涓?HPSA 鐗规湁鐨勬潯鐩?

  闄や簡 /sys 涓彲鐢ㄧ殑閫氱敤 SCSI 灞炴€у锛宧psa 杩樻敮鎸佷互涓嬪睘鎬э細

## HPSA 鐗规湁鐨勪富鏈猴紙host锛夊睘鎬?

```

    /sys/class/scsi_host/host*/rescan
    /sys/class/scsi_host/host*/firmware_revision
    /sys/class/scsi_host/host*/resettable
    /sys/class/scsi_host/host*/transport_mode

  host 鐨?"rescan" 灞炴€ф槸涓€涓彧鍐欏睘鎬с€傚啓鍏ヨ灞炴€у皢瀵艰嚧椹卞姩
  鎵弿鏂版坊鍔犮€佹洿鏀规垨绉婚櫎鐨勮澶囷紙渚嬪鐑彃鎷旂殑纾佸甫鏈猴紝鎴栨柊閰嶇疆鎴?  鍒犻櫎鐨勯€昏緫椹卞姩鍣ㄧ瓑锛夛紝骞跺皢妫€娴嬪埌鐨勪换浣曞彉鍖栭€氱煡 SCSI 涓棿灞傦紙midlayer锛夈€?  閫氬父杩欑敱 HP 鐨?Array Configuration Utility锛圙UI 鎴栧懡浠よ鐗堟湰锛夎嚜鍔ㄨЕ鍙戯紝
  鍥犳瀵逛簬閫昏緫椹卞姩鍣ㄧ殑鏇存敼锛岀敤鎴烽€氬父涓嶅繀浣跨敤瀹冦€傚湪鐑彃鎷旇濡傜甯︽満銆?  鎴栧寘鍚閰嶇疆閫昏緫椹卞姩鍣ㄧ殑鏁翠釜瀛樺偍绠辩瓑璁惧鏃讹紝瀹冨彲鑳藉緢鏈夌敤銆?
  "firmware_revision" 灞炴€у寘鍚?Smart Array 鐨勫浐浠剁増鏈€備緥濡?:

	root@host:/sys/class/scsi_host/host4# cat firmware_revision
	7.14

  transport_mode 鎸囩ず鎺у埗鍣ㄥ浜?"performant" 杩樻槸 "simple" 妯″紡銆?  杩欑敱 "hpsa_simple_mode" 妯″潡鍙傛暟鎺у埗銆?
  "resettable" 鍙灞炴€ф寚绀虹壒瀹氭帶鍒跺櫒鏄惁鑳藉鍝嶅簲 "reset_devices"
  鍐呮牳鍙傛暟銆傚鏋滆澶囧彲閲嶇疆锛岃鏂囦欢灏嗗寘鍚?"1"锛屽惁鍒欎负 "0"銆備緥濡傦紝
  kdump 浣跨敤璇ュ弬鏁板湪椹卞姩鍔犺浇鏃堕噸缃帶鍒跺櫒锛屼互娑堥櫎鎺у埗鍣ㄤ笂浠讳綍鏈畬鎴愮殑
  鍛戒护锛屽苟灏嗘帶鍒跺櫒缃簬宸茬煡鐘舵€侊紝浠ヤ究 kdump 鍙戣捣鐨?I/O 鑳藉姝ｅ父宸ヤ綔锛?  鑰屼笉浼氳鏉ヨ嚜鍏堝墠鍐呮牳鐨勯檲鏃у懡浠ゆ垨鎺у埗鍣ㄤ笂娈嬬暀鐨勫叾浠栭檲鏃х姸鎬佷互浠讳綍鏂瑰紡骞叉壈銆?  璇ュ睘鎬т娇 kexec 宸ュ叿鑳藉鍦ㄧ敤鎴疯瘯鍥惧皢涓€涓棤娉曞搷搴?reset_devices 鍐呮牳鍙傛暟鐨?  璁惧鎸囧畾涓鸿浆鍌ㄨ澶囨椂锛屽鐢ㄦ埛鍙戝嚭璀﹀憡銆?
```
### HPSA 鐗规湁鐨勭鐩橈紙disk锛夊睘鎬?

```

    /sys/class/scsi_disk/c:b:t:l/device/unique_id
    /sys/class/scsi_disk/c:b:t:l/device/raid_level
    /sys/class/scsi_disk/c:b:t:l/device/lunid

  锛堝叾涓?c:b:t:l 鍒嗗埆鏄澶囩殑鎺у埗鍣ㄣ€佹€荤嚎銆佺洰鏍囦笌 lun锛?
  渚嬪::

	root@host:/sys/class/scsi_disk/4:0:0:0/device# cat unique_id
	600508B1001044395355323037570F77
	root@host:/sys/class/scsi_disk/4:0:0:0/device# cat lunid
	0x0000004000000000
	root@host:/sys/class/scsi_disk/4:0:0:0/device# cat raid_level
	RAID 0

```
## HPSA 鐗规湁鐨?ioctl


  涓轰簡涓庝负 cciss 椹卞姩缂栧啓鐨勫簲鐢ㄧ▼搴忓吋瀹癸紝hpsa 椹卞姩涔熸敮鎸?cciss 椹卞姩
  鏀寔鐨勮澶氾紙浣嗗苟闈炲叏閮級ioctl銆傝繖浜涙墍浣跨敤鐨勬暟鎹粨鏋勫湪
  include/linux/cciss_ioctl.h 涓弿杩般€?
  CCISS_DEREGDISK, CCISS_REGNEWDISK, CCISS_REGNEWD
	涓婅堪涓変釜 ioctl 鍋氱殑浜嬫儏瀹屽叏鐩稿悓锛屽嵆瀵艰嚧椹卞姩
	閲嶆柊鎵弿鏂拌澶囥€傝繖涓庡啓鍏?hpsa 鐗规湁鐨?host "rescan" 灞炴€у仛鐨勪簨鎯呭畬鍏ㄧ浉鍚屻€?
  CCISS_GETPCIINFO
	杩斿洖 PCI 鍩熴€佹€荤嚎銆佽澶囦笌鍔熻兘浠ュ強 "board ID"锛圥CI 瀛愮郴缁?ID锛夈€?
  CCISS_GETDRIVVER
```

		(major_version << 16) | (minor_version << 8) | (subminor_version)

  CCISS_PASSTHRU, CCISS_BIG_PASSTHRU
	鍏佽灏?"BMIC" 涓?"CISS" 鍛戒护閫忎紶鍒?Smart Array銆?	杩欎簺琚?HP Array Configuration Utility銆丼NMP 瀛樺偍浠ｇ悊绛夊箍娉涗娇鐢ㄣ€?	鏈夊叧涓€浜涚ず渚嬶紝璇峰弬瑙?http://cciss.sf.net 涓婄殑 cciss_vol_status銆?
```
