## Xen 鐨勮櫄鎷?TPM 鎺ュ彛


Authors: Matthew Fioravante (JHUAPL), Daniel De Graaf (NSA)

鏈枃妗ｆ弿杩扮敤浜?Xen 鐨勮櫄鎷熷彲淇″钩鍙版ā鍧楋紙vTPM锛夊瓙绯荤粺銆傚亣瀹氳鑰呯啛鎮夋瀯寤哄拰瀹夎 Xen銆丩inux锛屽苟瀵?TPM 鍜?vTPM 姒傚康鏈夊熀鏈悊瑙ｃ€?
### 绠€浠?

杩欓」宸ヤ綔鐨勭洰鏍囨槸鍚戣櫄鎷熷鎴锋満鎿嶄綔绯荤粺锛堝湪 Xen 鏈涓嵆 DomU锛夋彁渚?TPM 鍔熻兘銆傝繖璁╃▼搴忚兘澶熶笌铏氭嫙绯荤粺涓殑
TPM 浜や簰锛屽氨鍍忓畠浠笌鐗╃悊绯荤粺涓婄殑 TPM 浜や簰涓€鏍枫€傛瘡涓鎴锋満鑾峰緱鑷繁鐙湁鐨勩€佽浠跨湡鐨勩€佽蒋浠跺疄鐜扮殑 TPM銆傜劧鑰岋紝
姣忎釜 vTPM 鐨勭瀵嗭紙瀵嗛挜銆丯VRAM 绛夛級鐢变竴涓?vTPM 绠＄悊鍣ㄥ煙绠＄悊锛岃鍩熷皢杩欎簺绉樺瘑瀵嗗皝锛坰eal锛夊埌鐗╃悊 TPM 涓娿€?濡傛灉鍒涘缓杩欎簺鍩燂紙绠＄悊鍣ㄣ€乿TPM 鍜屽鎴锋満锛夌殑杩囩▼鏄彲淇＄殑锛岄偅涔?vTPM 瀛愮郴缁熷皢鏍规浜庣‖浠?TPM 鐨勪俊浠婚摼寤朵几鍒?Xen 涓殑铏氭嫙鏈恒€倂TPM 鐨勬瘡涓富瑕佺粍浠堕兘瀹炵幇涓轰竴涓嫭绔嬬殑鍩燂紝鎻愪緵鐢辫櫄鎷熸満鐩戣鍣ㄤ繚璇佺殑瀹夊叏闅旂銆倂TPM 鍩熷湪
mini-os 涓疄鐜帮紝浠ュ噺灏戝唴瀛樺拰澶勭悊鍣ㄥ紑閿€銆?
姝?mini-os vTPM 瀛愮郴缁熷缓绔嬪湪 IBM 鍜?Intel 鍏徃鍏堝墠瀹屾垚鐨?vTPM 宸ヤ綔涔嬩笂銆?

### 璁捐姒傝堪


```

  +------------------+
  |    Linux DomU    | ...
  |       |  ^       |
  |       v  |       |
  |   xen-tpmfront   |
  +------------------+
          |  ^
          v  |
  +------------------+
  | mini-os/tpmback  |
  |       |  ^       |
  |       v  |       |
  |  vtpm-stubdom    | ...
  |       |  ^       |
  |       v  |       |
  | mini-os/tpmfront |
  +------------------+
          |  ^
          v  |
  +------------------+
  | mini-os/tpmback  |
  |       |  ^       |
  |       v  |       |
  | vtpmmgr-stubdom  |
  |       |  ^       |
  |       v  |       |
  | mini-os/tpm_tis  |
  +------------------+
          |  ^
          v  |
  +------------------+
  |   Hardware TPM   |
  +------------------+

```
- Linux DomU:
	       甯屾湜浣跨敤 vTPM 鐨勩€佸熀浜?Linux 鐨勫鎴锋満銆傚彲鑳藉瓨鍦ㄥ涓繖鏍风殑瀹㈡埛鏈恒€?
- xen-tpmfront.ko:
		    Linux 鍐呮牳铏氭嫙 TPM 鍓嶇椹卞姩銆傝椹卞姩涓哄熀浜?Linux 鐨?DomU 鎻愪緵 vTPM 璁块棶銆?
- mini-os/tpmback:
		    Mini-os TPM 鍚庣椹卞姩銆侺inux 鍓嶇椹卞姩杩炴帴鍒版鍚庣椹卞姩锛屼互淇冭繘
		    Linux DomU 涓庡叾 vTPM 涔嬮棿鐨勯€氫俊銆倂tpmmgr-stubdom 涔熶娇鐢ㄦ椹卞姩涓?		    vtpm-stubdom 閫氫俊銆?
- vtpm-stubdom:
		 涓€涓疄鐜?vTPM 鐨?mini-os 妗╁煙锛坰tub domain锛夈€傝繍琛屼腑鐨?vtpm-stubdom
		 瀹炰緥涓庣郴缁熶笂鐨勯€昏緫 vtpms 涔嬮棿瀛樺湪涓€涓€鏄犲皠銆倂TPM 鐨勫钩鍙伴厤缃瘎瀛樺櫒锛圥CR锛夐€氬父
		 鍏ㄩ儴鍒濆鍖栦负闆躲€?
- mini-os/tpmfront:
		     Mini-os TPM 鍓嶇椹卞姩銆倂TPM mini-os 鍩?vtpm-stubdom 浣跨敤姝ら┍鍔ㄤ笌
		     vtpmmgr-stubdom 閫氫俊銆傛椹卞姩涔熺敤浜庝笌 vTPM 鍩熼€氫俊鐨?mini-os 鍩燂紙濡?pv-grub锛変腑銆?
- vtpmmgr-stubdom:
		涓€涓疄鐜?vTPM 绠＄悊鍣ㄧ殑 mini-os 鍩熴€傚彧鏈変竴涓?vTPM 绠＄悊鍣紝骞朵笖瀹冨簲璇ュ湪鏁翠釜
		鏈哄櫒鐨勭敓鍛藉懆鏈熷唴杩愯銆傝鍩熻皟鑺傚绯荤粺涓婄墿鐞?TPM 鐨勮闂紝骞朵繚鎶ゆ瘡涓?vTPM 鐨勬寔涔呯姸鎬併€?
- mini-os/tpm_tis:
		    Mini-os TPM 1.2 鐗?TPM 鎺ュ彛瑙勮寖锛圱IS锛夐┍鍔ㄣ€倂tpmmgr-stubdom 浣跨敤姝ら┍鍔?		    鐩存帴涓庣‖浠?TPM 瀵硅瘽銆傞€氫俊閫氳繃灏嗙‖浠跺唴瀛橀〉鏄犲皠鍒?vtpmmgr-stubdom 鏉ュ疄鐜般€?
- Hardware TPM:
		鐒婃帴鍒颁富鏉夸笂鐨勭墿鐞?TPM銆?

### 涓?Xen 鐨勯泦鎴?

瀵?vTPM 椹卞姩鐨勬敮鎸佸湪 Xen 4.3 涓€氳繃 libxl toolstack 鍔犲叆 Xen銆傚叧浜庤缃?vTPM 鍜?vTPM 绠＄悊鍣ㄦ々鍩熺殑缁嗚妭锛?璇峰弬瑙?Xen 鏂囨。锛坉ocs/misc/vtpm.txt锛夈€備竴鏃︽々鍩熻繍琛岃捣鏉ワ紝vTPM 璁惧鐨勮缃柟寮忎笌鍩熼厤缃枃浠朵腑鐨勭鐩樻垨
缃戠粶璁惧鐩稿悓銆?
涓轰簡浣跨敤璇稿 IMA 杩欐牱闇€瑕佸湪 initrd 涔嬪墠鍔犺浇 TPM 鐨勭壒鎬э紝xen-tpmfront 椹卞姩蹇呴』缂栬瘧杩涘唴鏍搞€傚鏋滀笉浣跨敤
杩欑被鐗规€э紝璇ラ┍鍔ㄥ彲浠ョ紪璇戜负妯″潡锛屽苟鍍忓線甯镐竴鏍疯鍔犺浇銆?