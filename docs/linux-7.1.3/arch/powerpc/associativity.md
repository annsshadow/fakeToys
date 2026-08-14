## NUMA 璧勬簮浜插拰鎬?

浜插拰鎬э紙associativity锛夎〃绀哄皢鍚勭骞冲彴璧勬簮鍒嗙粍涓轰竴浜涘煙锛岃繖浜涘煙鐩稿浜庤鍩熶箣澶?鐨勮祫婧愬叿鏈夊疄璐ㄤ笂鐩歌繎鐨勫钩鍧囨€ц兘銆傛煇涓粰瀹氬煙涓€佸郊姝や箣闂寸浉姣斿煙澶栧叾瀹冭祫婧愬瓙闆?琛ㄧ幇鍑烘洿濂芥€ц兘鐨勮祫婧愬瓙闆嗭紝琚〃绀轰负鏌愪釜瀛愬垎缁勫煙鐨勬垚鍛樸€傝繖涓€鎬ц兘鐗瑰緛鍦?Linux
鍐呮牳涓互 NUMA 鑺傜偣璺濈鐨勫舰寮忓憟鐜般€備粠骞冲彴鐨勮搴︾湅锛岃繖浜涚粍涔熻绉颁负鍩熴€?
PAPR 鎺ュ彛鐩墠鏀寔浠ヤ笉鍚屾柟寮忓皢杩欎簺璧勬簮鍒嗙粍缁嗚妭浼犺揪缁欐搷浣滅郴缁熴€傚畠浠绉颁负
Form 0銆丗orm 1 鍜?Form2 鍏宠仈鍒嗙粍銆侳orm 0 鏄渶鏃х殑鏍煎紡锛岀幇鍦ㄥ凡琚涓鸿繃鏃躲€?
Hypervisor 閫氳繃 "ibm,architecture-vec-5 property" 鎸囩ず鎵€浣跨敤鐨勫叧鑱旂被鍨?鏍煎紡銆?"ibm,architecture-vec-5" 灞炴€т腑绗?5 瀛楄妭鐨勭 0 浣嶆寚绀轰娇鐢?Form 0 杩樻槸 Form 1銆?鍊间负 1 琛ㄧず浣跨敤 Form 1 鍏宠仈銆傚浜?Form 2 鍏宠仈锛屼娇鐢?"ibm,architecture-vec-5"
灞炴€т腑绗?5 瀛楄妭鐨勭 2 浣嶃€?
### Form 0

Form 0 鍏宠仈浠呮敮鎸佷袱绉?NUMA 璺濈锛圠OCAL 鍜?REMOTE锛夈€?
### Form 1

Form 1 閫氳繃缁勫悎 ibm,associativity-reference-points 鍜?ibm,associativity 璁惧鏍?灞炴€ф潵纭畾璧勬簮缁?鍩熶箣闂寸殑 NUMA 璺濈銆?
"ibm,associativity" 灞炴€у寘鍚竴涓垨澶氫釜鏁板瓧锛坉omainID锛夌殑鍒楄〃锛岃〃绀鸿祫婧愮殑骞冲彴
鍒嗙粍鍩熴€?
"ibm,associativity-reference-points" 灞炴€у寘鍚竴涓垨澶氫釜鏁板瓧锛坉omainID 绱㈠紩锛夌殑
鍒楄〃锛岃〃绀哄叧鑱斿垪琛ㄤ腑鐨勪粠 1 寮€濮嬬殑搴忔暟銆俤omainID 绱㈠紩鍒楄〃琛ㄧず璧勬簮鍒嗙粍涓嶆柇鍗囬珮鐨?灞傜骇銆?
渚嬪锛?{ primary domainID index, secondary domainID index, tertiary domainID index.. }

Linux 鍐呮牳浣跨敤涓?domainID 绱㈠紩澶勭殑 domainID 浣滀负 NUMA 鑺傜偣 id銆侺inux 鍐呮牳閫氳繃
閫掑綊姣旇緝涓や釜鍩熸槸鍚﹀睘浜庣浉鍚岀殑鏇撮珮灞傜骇鍩熸潵璁＄畻涓や釜鍩熶箣闂寸殑 NUMA 璺濈銆傚浜庤祫婧?缁勪腑姣忛珮涓€灞傜殑涓嶅尮閰嶏紝鍐呮牳灏嗘瘮杈冨煙涔嬮棿鐨?NUMA 璺濈鍔犲€嶃€?
### Form 2

Form 2 鍏宠仈鏍煎紡鏂板浜嗙嫭绔嬬殑璁惧鏍戝睘鎬ф潵琛ㄧず NUMA 鑺傜偣璺濈锛屼粠鑰屼娇鑺傜偣璺濈璁＄畻
鏇村姞鐏垫椿銆侳orm 2 杩樺厑璁哥伒娲荤殑涓诲煙缂栧彿銆傜敱浜?NUMA 璺濈璁＄畻鐜板湪涓?"ibm,associativity-reference-points" 灞炴€т腑鐨勭储寮曞€艰В鑰︼紝Form 2 鍏佽鍦ㄧ浉鍚?domainID 绱㈠紩澶勫瓨鍦ㄥぇ閲忎富 domainID锛屼互琛ㄧず鍏锋湁涓嶅悓鎬ц兘/寤惰繜鐗瑰緛鐨勮祫婧愮粍銆?
Hypervisor 浣跨敤 "ibm,architecture-vec-5" 灞炴€т腑绗?5 瀛楄妭鐨勭 2 浣嶆潵鎸囩ず浣跨敤
FORM2 鍏宠仈銆?
"ibm,numa-lookup-index-table" 灞炴€у寘鍚竴涓垨澶氫釜鏁板瓧锛堣〃绀虹郴缁熶腑瀛樺湪鐨?domainID锛夌殑鍒楄〃銆傝灞炴€т腑 domainID 鐨勫亸绉昏鐢ㄤ綔閫氳繃 "ibm,numa-distance-table"
璁＄畻 NUMA 璺濈淇℃伅鏃剁殑绱㈠紩銆?
prop-encoded-array锛氫互 encode-int 鏂瑰紡缂栫爜鐨?domainID 鏁伴噺 N锛屽悗璺?N 涓互
encode-int 鏂瑰紡缂栫爜鐨?domainID銆?
渚嬪锛?"ibm,numa-lookup-index-table" =  {4, 0, 8, 250, 252}銆傚湪璁＄畻 domain 8 涓庣郴缁熶腑
鍏跺畠鍩熺殑璺濈鏃讹紝浣跨敤 domainID 8 鐨勫亸绉伙紙2锛夈€傚湪鏈枃妗ｇ殑鍏朵綑閮ㄥ垎锛岃鍋忕Щ灏嗚绉颁负
鍩熻窛绂诲亸绉伙紙domain distance offset锛夈€?
"ibm,numa-distance-table" 灞炴€у寘鍚竴涓垨澶氫釜鏁板瓧锛堣〃绀虹郴缁熶腑瀛樺湪鐨勮祫婧愮粍/鍩熶箣闂寸殑
NUMA 璺濈锛夌殑鍒楄〃銆?
prop-encoded-array锛氫互 encode-int 鏂瑰紡缂栫爜鐨勮窛绂诲€兼暟閲?N锛屽悗璺?N 涓互
encode-bytes 鏂瑰紡缂栫爜鐨勮窛绂诲€笺€傛垜浠兘澶熺紪鐮佺殑鏈€澶ц窛绂诲€间负 255銆侼 蹇呴』绛変簬 m 鐨?骞虫柟锛屽叾涓?m 鏄?numa-lookup-index-table 涓?domainID 鐨勬暟閲忋€?
渚嬪锛?ibm,numa-lookup-index-table = <3 0 8 40>;
ibm,numa-distace-table = <9>, /bits/ 8 < 10  20  80 20  10 160 80 160  10>;

```
	  | 0    8   40
	--|------------
	  |
	0 | 10   20  80
	  |
	8 | 20   10  160
	  |
	40| 80   160  10

```
鑺傜偣 0銆? 鍜?40 涓祫婧愬彲鑳界殑 "ibm,associativity" 灞炴€?
{ 3, 6, 7, 0 }
{ 3, 6, 9, 8 }
{ 3, 6, 7, 40}

閰嶅悎 "ibm,associativity-reference-points"  { 0x3 }

"ibm,lookup-index-table" 鏈夊姪浜庝互绱у噾鐨勬柟寮忚〃绀鸿窛绂荤煩闃点€傜敱浜?domainID 鍙互鏄?绋€鐤忕殑锛岃窛绂荤煩闃典篃鍙互鏈夋晥鍦版槸绋€鐤忕殑銆傚€熷姪 "ibm,lookup-index-table"锛屾垜浠彲浠?瀹炵幇璺濈淇℃伅鐨勭揣鍑戣〃绀恒€?