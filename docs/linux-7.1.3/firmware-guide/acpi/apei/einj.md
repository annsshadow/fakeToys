
## APEI 閿欒娉ㄥ叆锛圗rror INJection锛?

EINJ 鎻愪緵浜嗕竴绉嶇‖浠堕敊璇敞鍏ユ満鍒躲€傚畠瀵逛簬璋冭瘯鍜屾祴璇?APEI 浠ュ強 RAS 鍔熻兘
鎬讳綋鑰岃█闈炲父鏈夌敤銆?
浣犻渶瑕佸厛妫€鏌ヤ綘鐨?BIOS 鏄惁鏀寔 EINJ銆備负姝わ紝鏌ユ壘
```
  ACPI: EINJ 0x000000007370A000 000150 (v01 INTEL           00000001 INTL 00000001)
```
杩欒〃鏄?BIOS 鏆撮湶浜嗕竴涓?EINJ 琛?鈥斺€?瀹冩鏄墽琛屾敞鍏ユ墍缁忕敱鐨勬満鍒躲€?
鍙︿竴绉嶆柟娉曟槸鍦?/sys/firmware/acpi/tables 涓煡鎵句竴涓?"EINJ" 鏂囦欢锛屽畠鏄?鍚屼竴浜嬬墿鐨勫彟涓€绉嶈〃绀哄舰寮忋€?
涓婅堪涓よ€呴兘涓嶅瓨鍦紝骞朵笉涓€瀹氭剰鍛崇潃 EINJ 涓嶈鏀寔锛氬湪鏀惧純涔嬪墠锛岃杩涘叆 BIOS
璁剧疆锛岀湅鐪?BIOS 鏄惁鏈変竴涓敤浜庡惎鐢ㄩ敊璇敞鍏ョ殑閫夐」銆傛煡鎵惧悕涓?WHEA 鎴栫被浼肩殑
涓滆タ銆傞€氬父锛屼綘闇€瑕佸厛鍚敤涓€涓?ACPI5 鏀寔閫夐」锛屾墠鑳界湅鍒?BIOS 鑿滃崟鎵€鏀寔骞?鏆撮湶鐨?APEI銆丒INJ鈥︹€﹀姛鑳姐€?
瑕佷娇鐢?EINJ锛岃纭繚浣犵殑鍐呮牳涓惎鐢ㄤ簡浠ヤ笅閫夐」
```
  CONFIG_DEBUG_FS
  CONFIG_ACPI_APEI
  CONFIG_ACPI_APEI_EINJ
```
```
  CONFIG_ACPI_APEI_EINJ_CXL
```
EINJ 鐨勭敤鎴锋帴鍙ｄ綅浜?<debugfs 鎸傝浇鐐?/apei/einj銆?
灞炰簬瀹冪殑鏂囦欢濡備笅锛?
- available_error_type

  璇ユ枃浠舵樉绀轰簡鏀寔鍝簺閿欒绫诲瀷锛?
  ================  ===================================
  Error Type Value	閿欒鎻忚堪
  ================  ===================================
  0x00000001        澶勭悊鍣ㄥ彲绾犳閿欒
  0x00000002        澶勭悊鍣ㄤ笉鍙籂姝ｉ潪鑷村懡閿欒
  0x00000004        澶勭悊鍣ㄤ笉鍙籂姝ｈ嚧鍛介敊璇?  0x00000008        鍐呭瓨鍙籂姝ｉ敊璇?  0x00000010        鍐呭瓨涓嶅彲绾犳闈炶嚧鍛介敊璇?  0x00000020        鍐呭瓨涓嶅彲绾犳鑷村懡閿欒
  0x00000040        PCI Express 鍙籂姝ｉ敊璇?  0x00000080        PCI Express 涓嶅彲绾犳闈炶嚧鍛介敊璇?  0x00000100        PCI Express 涓嶅彲绾犳鑷村懡閿欒
  0x00000200        骞冲彴鍙籂姝ｉ敊璇?  0x00000400        骞冲彴涓嶅彲绾犳闈炶嚧鍛介敊璇?  0x00000800        骞冲彴涓嶅彲绾犳鑷村懡閿欒
  V2_0x00000001     EINJV2 澶勭悊鍣ㄩ敊璇?  V2_0x00000002     EINJV2 鍐呭瓨閿欒
  V2_0x00000004     EINJV2 PCI Express 閿欒
  ================  ===================================

  鏂囦欢鍐呭鐨勬牸寮忓涓婃墍绀猴紝鍙槸鍏朵腑鍙嚭鐜板彲鐢ㄧ殑閿欒绫诲瀷銆?
- error_type

  璁剧疆姝ｅ湪娉ㄥ叆鐨勯敊璇被鍨嬬殑鍊笺€傚彲鑳界殑閿欒绫诲瀷瀹氫箟鍦ㄤ笂鏂圭殑
  available_error_type 鏂囦欢涓€?
- error_inject

  鍚戣鏂囦欢鍐欏叆浠绘剰鏁存暟浠ヨЕ鍙戦敊璇敞鍏ャ€傝纭繚浣犲凡缁忔寚瀹氫簡鎵€鏈夊繀瑕佺殑閿欒
  鍙傛暟锛屽嵆杩欐鍐欏叆搴斿綋鏄敞鍏ラ敊璇椂鐨勬渶鍚庝竴姝ャ€?
- flags

  鍦ㄥ唴鏍哥増鏈?3.13 鍙婁互涓婂瓨鍦ㄣ€傜敤浜庢寚瀹?param{1..4} 涓摢浜涙湁鏁堛€佸苟搴斿湪娉ㄥ叆
  鏈熼棿琚浐浠朵娇鐢ㄣ€傚叾鍊兼槸涓€涓綅鎺╃爜锛屽畾涔変簬 ACPI5.0 瑙勮寖涓?  SET_ERROR_TYPE_WITH_ADDRESS 鏁版嵁缁撴瀯锛?
    Bit 0
      澶勭悊鍣?APIC 瀛楁鏈夋晥锛堣涓嬫柟 param3锛夈€?    Bit 1
      鍐呭瓨鍦板潃鍜屾帺鐮佹湁鏁堬紙param1 鍜?param2锛夈€?    Bit 2
      PCIe锛坰eg銆乥us銆乨ev銆乫n锛夋湁鏁堬紙瑙佷笅鏂?param4锛夈€?    Bit 3
      EINJv2 鎵╁睍缁撴瀯鏈夋晥

  濡傛灉缃负闆讹紝鍒欐ā鎷熶紶缁熻涓猴紝姝ゆ椂娉ㄥ叆绫诲瀷鍙寚瀹氫竴涓疆浣嶇殑浣嶏紝鑰?param1
  琚璺鐢ㄣ€?
- param1

  璇ユ枃浠剁敤浜庤缃涓€涓敊璇弬鏁板€笺€傚叾浣滅敤鍙栧喅浜?error_type 涓寚瀹氱殑閿欒绫诲瀷銆?  渚嬪锛屽鏋滈敊璇被鍨嬫槸鍐呭瓨鐩稿叧绫诲瀷锛屽垯 param1 搴斿綋鏄竴涓湁鏁堢殑鐗╃悊鍐呭瓨鍦板潃銆?  [闄ら潪璁剧疆浜?"flag" 鈥斺€?瑙佷笂]

- param2

  鐢ㄩ€斿悓涓婃柟鐨?param1銆備緥濡傦紝濡傛灉閿欒绫诲瀷鏄唴瀛樼浉鍏崇被鍨嬶紝鍒?param2 搴斿綋鏄竴涓?  鐗╃悊鍐呭瓨鍦板潃鎺╃爜銆侺inux 瑕佹眰椤电矑搴︽垨鏇寸粏锛屼緥濡?0xfffffffffffff000銆?
- param3

  褰?"flags" 涓殑 0x1 浣嶇疆浣嶆椂浣跨敤锛岀敤浜庢寚瀹?APIC id

- param4
  褰?"flags" 涓殑 0x4 浣嶇疆浣嶆椂浣跨敤锛岀敤浜庢寚瀹氱洰鏍?PCIe 璁惧

- notrigger

  閿欒娉ㄥ叆鏈哄埗鏄竴涓袱姝ヨ繃绋嬨€傚厛娉ㄥ叆閿欒锛屽啀鎵ц涓€浜涙搷浣滄潵瑙﹀彂瀹冦€傚皢
  "notrigger" 璁句负 1 浼氳烦杩囪Е鍙戦樁娈碉紝杩?*鍙兘**鍏佽鐢ㄦ埛閫氳繃瀵逛綔涓洪敊璇敞鍏?  鐩爣鐨?CPU銆佸唴瀛樹綅缃垨璁惧杩涜绠€鍗曡闂紝鑰屽湪鍏朵粬鏌愪釜涓婁笅鏂囦腑寮曞彂閿欒銆?  杩欏疄闄呮槸鍚︽湁鏁堬紝鍙栧喅浜?BIOS 鍦ㄨЕ鍙戦樁娈靛疄闄呭寘鍚簡鍝簺鎿嶄綔銆?
- component_id0 .. component_idN, component_syndrome0 .. component_syndromeN

  杩欎簺鏂囦欢鐢ㄤ簬璁剧疆 EINJv2 鎵╁睍缁撴瀯鐨?"Component Array"锛堢粍浠舵暟缁勶級瀛楁銆?  姣忎釜鏂囦欢淇濆瓨涓€涓?128 浣嶇殑鍗佸叚杩涘埗鍊笺€傚悜杩欎簺鏂囦欢涓殑浠绘剰涓€涓彧鍐欏叆涓€涓?  鎹㈣绗︼紝浼氬皢鍏惰缃负鏃犳晥锛堝叏 1锛夊€笺€?
CXL 閿欒绫诲瀷鑷?ACPI 6.5 璧峰緱鍒版敮鎸侊紙鍓嶆彁鏄瓨鍦?CXL 绔彛锛夈€傜敤浜?CXL 閿欒
绫诲瀷鐨?EINJ 鐢ㄦ埛鎺ュ彛浣嶄簬 <debugfs 鎸傝浇鐐?/cxl銆傚睘浜庡畠鐨勬枃浠跺涓嬶細

- einj_types:

  鎻愪緵涓庝笂鏂?available_error_types 鐩稿悓鐨勫姛鑳斤紝浣嗛拡瀵?CXL 閿欒绫诲瀷

- $dport_dev/einj_inject:

  灏嗕竴涓?CXL 閿欒绫诲瀷娉ㄥ叆鍒扮敱 $dport_dev 琛ㄧず鐨?CXL 绔彛锛屽叾涓?$dport_dev
  鏄?CXL 绔彛鐨勫悕绉帮紙閫氬父鏄竴涓?PCIe 璁惧鍚嶏級銆傞拡瀵?CXL 2.0+ 绔彛鐨勯敊璇敞鍏?  鍙互浣跨敤浣嶄簬 <debugfs 鎸傝浇鐐?/apei/einj 涓嬬殑浼犵粺鎺ュ彛锛岃€?CXL 1.1/1.0 绔彛
  鐨勬敞鍏ュ繀椤讳娇鐢ㄨ繖涓枃浠躲€?

鍩轰簬 ACPI 4.0 瑙勮寖鐨?BIOS 鐗堟湰鍦ㄦ帶鍒堕敊璇敞鍏ヤ綅缃柟闈㈢殑閫夐」鏈夐檺銆備綘鐨?BIOS
鍙兘鏀寔涓€涓墿灞曪紙閫氳繃 param_extension=1 妯″潡鍙傛暟锛屾垨鍚姩鍛戒护琛?einj.param_extension=1 鍚敤锛夈€傝繖鍏佽鍐呭瓨娉ㄥ叆鐨勫湴鍧€鍜屾帺鐮佺敱 apei/einj 涓殑
param1 鍜?param2 鏂囦欢鎸囧畾銆?
鍩轰簬 ACPI 5.0 瑙勮寖鐨?BIOS 鐗堟湰瀵规敞鍏ョ洰鏍囨湁鏇村己鐨勬帶鍒惰兘鍔涖€傚浜庡鐞嗗櫒鐩稿叧鐨?閿欒锛堢被鍨?0x1銆?x2 鍜?0x4锛夛紝浣犲彲浠ュ皢 flags 璁句负 0x3锛坆it 0 瀵瑰簲 param3锛?bit 1 瀵瑰簲 param1 鍜?param2锛夛紝浠ヤ究鍚戦敊璇坊鍔犳洿澶氫俊鎭?```
	memory_address = param1;
	memory_address_range = param2;
	apicid = param3;
	pcie_sbdf = param4;
```
瀵逛簬鍐呭瓨閿欒锛堢被鍨?0x8銆?x10 鍜?0x20锛夛紝鍦板潃鐢?param1 璁剧疆锛屾帺鐮佸湪 param2 涓?锛?x0 绛変环浜庡叏 1锛夈€傚浜?PCI Express 閿欒锛堢被鍨?0x40銆?x80 鍜?0x100锛夛紝娈点€?鎬荤嚎銆佽澶囧拰
```
         31     24 23    16 15    11 10      8  7        0
	+-------------------------------------------------+
	| segment |   bus  | device | function | reserved |
	+-------------------------------------------------+
```
鎬讳箣锛屼綘鏄庣櫧杩欎釜鎰忔€濆氨澶熶簡锛屽鏋滄湁鐤戦棶锛岀湅涓€涓?drivers/acpi/apei/einj.c
涓殑浠ｇ爜銆?
鍩轰簬 ACPI 5.0 鐨?BIOS 涔熷彲鑳藉厑璁告敞鍏ュ巶鍟嗙壒瀹氱殑閿欒銆傚湪杩欑鎯呭喌涓嬶紝涓€涓悕涓?vendor 鐨勬枃浠朵細鍖呭惈鏉ヨ嚜 BIOS 鐨勬爣璇嗕俊鎭紝甯屾湜鑳借鎯宠浣跨敤璇ュ巶鍟嗙壒瀹氭墿灞曠殑
搴旂敤绋嬪簭鍒ゆ柇鑷繁鏄惁杩愯鍦ㄦ敮鎸佸畠鐨?BIOS 涓娿€傛墍鏈夊巶鍟嗘墿灞曞湪 error_type 涓?閮芥湁 0x80000000 浣嶇疆浣嶃€備竴涓悕涓?vendor_flags 鐨勬枃浠舵帶鍒?param1 鍜?param2
鐨勮В閲婏紙1 = PROCESSOR銆? = MEMORY銆? = PCI锛夈€傝鎯呰鍙傞槄浣犵殑 BIOS 鍘傚晢鏂囨。
锛堝苟涓斿鏋滃巶鍟嗗湪浣跨敤姝ゅ姛鑳戒笂鐨勫垱鎰忚秴鍑烘垜浠鏈燂紝杩欎釜 API 杩樹細鏈夊彉鍔級銆?

```
  # cd /sys/kernel/debug/apei/einj
  # cat available_error_type		# See which errors can be injected
  0x00000002	Processor Uncorrectable non-fatal
  0x00000008	Memory Correctable
  0x00000010	Memory Uncorrectable non-fatal
  # echo 0x12345000 > param1		# Set memory address for injection
  # echo 0xfffffffffffff000 > param2		# Mask - anywhere in this page
  # echo 0x8 > error_type			# Choose correctable memory error
  # echo 1 > error_inject			# Inject now
```
```
  # cd /sys/kernel/debug/apei/einj
  # cat available_error_type			# See which errors can be injected
  0x00000002	Processor Uncorrectable non-fatal
  0x00000008	Memory Correctable
  0x00000010	Memory Uncorrectable non-fatal
  V2_0x00000001	EINJV2 Processor Error
  V2_0x00000002	EINJV2 Memory Error

  # echo 0x12345000 > param1			# Set memory address for injection
  # echo 0xfffffffffffff000 > param2		# Range - anywhere in this page
  # echo 0x1 > component_id0			# First device ID
  # echo 0x4 > component_syndrome0		# First error syndrome
  # echo 0x2 > component_id1			# Second device ID
  # echo 0x4 > component_syndrome1		# Second error syndrome
  # echo '' > component_id2			# Mark id2 invalid to terminate list
  # echo V2_0x2 > error_type			# Choose EINJv2 memory error
  # echo 0xa > flags				# set flags to indicate EINJv2
  # echo 1 > error_inject			# Inject now
```
```
  [22715.830801] EDAC sbridge MC3: HANDLING MCE MEMORY ERROR
  [22715.834759] EDAC sbridge MC3: CPU 0: Machine Check Event: 0 Bank 7: 8c00004000010090
  [22715.834759] EDAC sbridge MC3: TSC 0
  [22715.834759] EDAC sbridge MC3: ADDR 12345000 EDAC sbridge MC3: MISC 144780c86
  [22715.834759] EDAC sbridge MC3: PROCESSOR 0:306e7 TIME 1422553404 SOCKET 0 APIC 0
  [22716.616173] EDAC MC3: 1 CE memory read error on CPU_SrcID#0_Channel#0_DIMM#0 (channel:0 slot:0 page:0x12345 offset:0x0 grain:32 syndrome:0x0 -  area:DRAM err_code:0001:0090 socket:0 channel_mask:1 rank:0)
```
```
    # cd /sys/kernel/debug/cxl/
    # ls
    0000:e0:01.1 0000:0c:00.0
    # cat einj_types                # See which errors can be injected
	0x00008000  CXL.mem Protocol Correctable
	0x00010000  CXL.mem Protocol Uncorrectable non-fatal
	0x00020000  CXL.mem Protocol Uncorrectable fatal
    # cd 0000:e0:01.1               # Navigate to dport to inject into
    # echo 0x8000 > einj_inject     # Inject error
```
閽堝 SGX enclave 娉ㄥ叆鐨勭壒娈婅鏄庯細

鍙兘浼氭湁涓€涓崟鐙殑 BIOS 璁剧疆閫夐」鐢ㄤ簬鍚敤 SGX 娉ㄥ叆銆?
娉ㄥ叆杩囩▼鍖呮嫭璁剧疆鏌愪釜鐗规畩鐨勫唴瀛樻帶鍒跺櫒瑙﹀彂鍣紝瀹冧細鍦ㄤ笅涓€娆″鐩爣鍦板潃鐨勫啓鍏ユ椂
娉ㄥ叆閿欒銆備絾纭欢闃绘 SGX enclave 涔嬪鐨勪换浣曡蒋浠讹紙鐢氳嚦 BIOS SMM 妯″紡锛夎闂?enclave 椤点€?
鍙互浣跨敤浠ヤ笅椤哄簭锛?  1) 纭畾 enclave 椤电殑鐗╃悊鍦板潃
  2) 浣跨敤 "notrigger=1" 妯″紡杩涜娉ㄥ叆锛堣繖浼氳缃敞鍏ュ湴鍧€锛屼絾骞朵笉浼氬疄闄呮敞鍏ワ級
  3) 杩涘叆 enclave
  4) 鍚戜笌绗?1 姝ョ墿鐞嗗湴鍧€鍖归厤鐨勮櫄鎷熷湴鍧€鍐欏叆鏁版嵁
  5) 瀵硅铏氭嫙鍦板潃鎵ц CLFLUSH
  6) 鑷棆寤惰繜 250ms
  7) 浠庤铏氭嫙鍦板潃璇诲彇銆傝繖浼氳Е鍙戦敊璇?
鍏充簬 EINJ 鐨勬洿澶氫俊鎭紝璇峰弬闃?ACPI 瑙勮寖 4.0 鐗堢 17.5 鑺傚拰 ACPI 5.0 鐗?绗?18.6 鑺傘€?