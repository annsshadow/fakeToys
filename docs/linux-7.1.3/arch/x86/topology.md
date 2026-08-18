
## x86 鎷撴墤


鏈枃妗ｈ褰曞苟闃愭槑浜?x86 鎷撴墤鍦ㄥ唴鏍镐腑鐨勫缓妯′笌琛ㄧず鐨勪富瑕佹柟闈€傚湪瀵圭浉搴斾唬鐮佽繘琛屾洿鏀规椂锛岃鍚屾鏇存柊/淇敼鏈枃妗ｃ€?
鏋舵瀯鏃犲叧鐨勬嫇鎵戝畾涔変綅浜?Documentation/admin-guide/cputopology.rst銆傛湰鏂囦欢淇濆瓨鐨勬槸 x86 鐗规湁鐨勫樊寮?鐗规畩鎬э紝杩欎簺涓嶄竴瀹氶€傜敤浜庨€氱敤瀹氫箟銆傚洜姝わ紝鍦?x86 涓婁簡瑙?Linux 鎷撴墤鐨勬柟娉曟槸锛氬厛闃呰閫氱敤瀹氫箟锛屽悓鏃跺鐓ф湰鏂囦欢鏌ョ湅 x86 鐗规湁鐨勯儴鍒嗐€?
涓嶇敤璇达紝浠ｇ爜搴斿綋浣跨敤閫氱敤鐨勫嚱鏁扳€斺€旀湰鏂囦欢**浠呬粎**鏄负浜?*璁板綍** x86 鎷撴墤鐨勫唴閮ㄨ繍浣滄満鍒躲€?
鐢?Thomas Gleixner <tglx@kernel.org> 涓?Borislav Petkov <bp@alien8.de> 鍙戣捣銆?
鎷撴墤璁炬柦鐨勪富瑕佺洰鏍囨槸鍚戦偅浜涢渶瑕佷簡瑙?鏌ヨ/浣跨敤杩愯绯荤粺鐨勭粨鏋勶紙娑夊強绾跨▼銆佹牳銆佸皝瑁呯瓑锛夌殑浠ｇ爜锛屾彁渚涙伆褰撶殑鎺ュ彛銆?
鍐呮牳骞朵笉鍏冲績鐗╃悊鎻掓Ы锛坰ocket锛夎繖涓€姒傚康锛屽洜涓烘彃妲戒笌杞欢鏃犲叧锛屽畠鍙槸涓€涓満鐢电粍浠躲€傝繃鍘讳竴涓彃妲芥€绘槸鍖呭惈涓€涓皝瑁咃紙瑙佷笅鏂囷級锛屼絾闅忕潃澶氳姱鐗囨ā鍧楋紙MCM锛夌殑鍑虹幇锛屼竴涓彃妲藉彲浠ュ绾冲涓皝瑁呫€傚洜姝や唬鐮佷腑鍙兘浠嶆湁瀵规彃妲界殑寮曠敤锛屼絾瀹冧滑灞炰簬鍘嗗彶閬楃暀锛屽簲褰撹娓呯悊鎺夈€?
绯荤粺鐨勬嫇鎵戠敤浠ヤ笅鍗曚綅鎻忚堪锛?
    - 灏佽锛坧ackages锛?    - 鏍革紙cores锛?    - 绾跨▼锛坱hreads锛?
## 灏佽锛圥ackage锛?

灏佽鍖呭惈涓€涓垨澶氫釜鏍镐互鍙婂叡浜祫婧愶紝渚嬪 DRAM 鎺у埗鍣ㄣ€佸叡浜紦瀛樼瓑銆?
鐜颁唬绯荤粺涔熷彲鑳界敤鏈 鈥淒ie鈥?鏉ヨ〃绀哄皝瑁呫€?
AMD 瀵瑰皝瑁呯殑鏈鏄?鈥淣ode鈥濄€?
鍐呮牳涓笌灏佽鐩稿叧鐨勬嫇鎵戜俊鎭細

  - topology_num_threads_per_package()

    涓€涓皝瑁呬腑鐨勭嚎绋嬫暟閲忋€?
  - topology_num_cores_per_package()

    涓€涓皝瑁呬腑鐨勬牳鏁伴噺銆?
  - topology_max_dies_per_package()

    涓€涓皝瑁呬腑 die 鐨勬渶澶ф暟閲忋€?
  - cpuinfo_x86.topo.die_id:

    die 鐨勭墿鐞?ID銆?
  - cpuinfo_x86.topo.pkg_id:

    灏佽鐨勭墿鐞?ID銆傝淇℃伅閫氳繃 CPUID 鑾峰彇锛屽苟鐢卞皝瑁呬腑鍚勪釜鏍哥殑 APIC ID 鎺ㄥ鑰屾潵銆?
    鐜颁唬绯荤粺灏嗘鍊肩敤浜庢彃妲姐€備竴涓彃妲藉唴鍙兘瀛樺湪澶氫釜灏佽銆傝鍊煎彲鑳戒笌 topo.die_id 涓嶅悓銆?
  - cpuinfo_x86.topo.logical_pkg_id:

    灏佽鐨勯€昏緫 ID銆傜敱浜庢垜浠笉淇′换 BIOS 浠ヤ竴鑷寸殑鏂瑰紡鏋氫妇灏佽锛屽洜姝ゅ紩鍏ヤ簡閫昏緫灏佽 ID 鐨勬蹇碉紝杩欐牱鎴戜滑灏辫兘鍚堢悊鍦拌绠楀嚭绯荤粺涓渶澶у彲鑳界殑灏佽鏁伴噺锛屽苟璁╁皝瑁呰绾挎€ф灇涓俱€?
  - topology_max_packages():

    绯荤粺涓彲鑳界殑灏佽鏈€澶ф暟閲忋€傚浜庢寜灏佽鐨勮鏂借€岃█锛屽彲鐢ㄤ簬棰勫垎閰嶆瘡涓皝瑁呯殑淇℃伅銆?
  - cpuinfo_x86.topo.llc_id:

      - 鍦?Intel 涓婏紝鏄叡浜湯绾х紦瀛橈紙Last Level Cache锛夌殑 CPU 鍒楄〃涓殑绗竴涓?APIC ID銆?
      - 鍦?AMD 涓婏紝鏄寘鍚湯绾х紦瀛樼殑 Node ID 鎴?Core Complex ID銆備竴鑸潵璇达紝瀹冩槸涓€涓兘鍦ㄧ郴缁熶笂鍞竴鏍囪瘑涓€涓?LLC 鐨勭紪鍙枫€?
## 鏍革紙Cores锛?

涓€涓牳鐢?1 涓垨澶氫釜绾跨▼缁勬垚銆傜嚎绋嬫槸 SMT 绫诲瀷杩樻槸 CMT 绫诲瀷骞舵棤褰卞搷銆?
AMD 瀵?CMT 鏍哥殑鏈鏄?鈥淐ompute Unit鈥濄€傚唴鏍稿缁堜娇鐢?鈥渃ore鈥濄€?
## 绾跨▼锛圱hreads锛?

涓€涓嚎绋嬫槸涓€涓崟涓€鐨勮皟搴﹀崟鍏冦€傚畠绛変环浜庝竴涓€昏緫 Linux CPU銆?
AMD 瀵?CMT 绾跨▼鐨勬湳璇槸 鈥淐ompute Unit Core鈥濄€傚唴鏍稿缁堜娇鐢?鈥渢hread鈥濄€?
鍐呮牳涓笌绾跨▼鐩稿叧鐨勬嫇鎵戜俊鎭細

  - topology_core_cpumask():

    cpumask 鍖呭惈璇ョ嚎绋嬫墍灞炲皝瑁呬腑鐨勬墍鏈夊湪绾跨嚎绋嬨€?
    鍦ㄧ嚎绾跨▼鐨勬暟閲忎篃浼氭墦鍗板湪 /proc/cpuinfo 鐨?鈥渟iblings鈥?涓€?
  - topology_sibling_cpumask():

    cpumask 鍖呭惈璇ョ嚎绋嬫墍灞炴牳涓殑鎵€鏈夊湪绾跨嚎绋嬨€?
  - topology_logical_package_id():

    璇ョ嚎绋嬫墍灞炵殑閫昏緫灏佽 ID銆?
  - topology_physical_package_id():

    璇ョ嚎绋嬫墍灞炵殑鐗╃悊灏佽 ID銆?
  - topology_core_id();

    璇ョ嚎绋嬫墍灞炴牳鐨?ID銆傚畠涔熶細鎵撳嵃鍦?/proc/cpuinfo 鐨?鈥渃ore_id鈥?涓€?
  - topology_logical_core_id();

    璇ョ嚎绋嬫墍灞炵殑閫昏緫鏍?ID銆?


## 绯荤粺鎷撴墤鏋氫妇


x86 绯荤粺涓婄殑鎷撴墤鍙互閫氳繃缁勫悎鍚勫巶鍟嗙壒瀹氱殑 CPUID 鍙跺瓙锛坙eaf锛夋潵鍙戠幇锛岃繖浜涘彾瀛愭灇涓句簡澶勭悊鍣ㄦ嫇鎵戜笌缂撳瓨灞傛缁撴瀯銆?
鍚?x86 鍘傚晢鍦ㄨВ鏋愭椂浼樺厛椤哄簭濡備笅鐨?CPUID 鍙跺瓙锛?
1) AMD

   1) CPUID leaf 0x80000026 [Extended CPU Topology] (Core::X86::Cpuid::ExCpuTopology)

      鎵╁睍 CPUID 鍙跺瓙 0x80000026 鏄?CPUID 鍙跺瓙 0xB 鐨勬墿灞曪紝鎻愪緵浜嗘瘡涓€灞傜骇涓?Core銆丆omplex銆丆CD锛圖ie锛夊拰 Socket 鐨勬嫇鎵戜俊鎭€?
      閫氳繃妫€鏌ユ渶澶ф墿灞?CPUID 绾у埆鏄惁 >= 0x80000026锛岀劧鍚庢鏌ョ壒瀹氬眰绾э紙浠?0 寮€濮嬶級鐨?`EBX[15:0]` 涓殑 `LogProcAtThisLevel` 鏄惁闈為浂锛屾潵鍙戠幇瀵硅鍙跺瓙鐨勬敮鎸併€?
      璇ュ眰绾т腑 `ECX[15:8]` 閲岀殑 `LevelType` 缁欏嚭浜嗚灞傜骇鎵€鎻忚堪鐨勬嫇鎵戝煙鈥斺€擟ore銆丆omplex銆丆CD锛圖ie锛夋垨 Socket銆?
      鍐呮牳浣跨敤 `EAX[4:0]` 涓殑 `CoreMaskWidth` 鏉ヨ幏鐭ラ渶瑕佷粠 `EDX[31:0]` 涓殑 `ExtendedLocalApicId` 鍙崇Щ澶氬皯浣嶏紝浠ュ緱鍒拌鎷撴墤灞傜骇鐨勫敮涓€鎷撴墤 ID銆傚叿鏈夌浉鍚屾嫇鎵?ID 鐨?CPU 鍏变韩璇ュ眰绾х殑璧勬簮銆?
      CPUID 鍙跺瓙 0x80000026 杩樻彁渚涗簡鍏充簬鍔熻€椾笌鏁堣兘绛夌骇銆佷互鍙婂叿鏈夊紓鏋勭壒鎬х殑 AMD 澶勭悊鍣ㄤ笂鏍哥被鍨嬫柟闈㈢殑鏇村淇℃伅銆?
      濡傛灉鏀寔 CPUID 鍙跺瓙 0x80000026锛屽垯鏃犻渶杩涗竴姝ヨВ鏋愩€?
   2) CPUID leaf 0x0000000B [Extended Topology Enumeration] (Core::X86::Cpuid::ExtTopEnum)

      鎵╁睍 CPUID 鍙跺瓙 0x0000000B 鏄墿灞?CPUID 鍙跺瓙 0x80000026 鐨勫墠韬紝浠呮弿杩板鐞嗗櫒鎷撴墤鐨勬牳涓庢彃妲藉煙銆?
      閫氳繃妫€鏌ユ渶澶ф敮鎸佺殑 CPUID 绾у埆鏄惁 >= 0xB锛岀劧鍚庢鏌ョ壒瀹氬眰绾э紙浠?0 寮€濮嬶級鐨?`EBX[31:0]` 鏄惁闈為浂锛屾潵鍙戠幇瀵硅鍙跺瓙鐨勬敮鎸併€?
      璇ュ眰绾т腑 `ECX[15:8]` 閲岀殑 `LevelType` 缁欏嚭浜嗚灞傜骇鎵€鎻忚堪鐨勬嫇鎵戝煙鈥斺€擳hread 鎴?Processor锛圫ocket锛夈€?
      鍐呮牳浣跨敤 `EAX[4:0]` 涓殑 `CoreMaskWidth` 鏉ヨ幏鐭ラ渶瑕佷粠 `EDX[31:0]` 涓殑 `ExtendedLocalApicId` 鍙崇Щ澶氬皯浣嶏紝浠ュ緱鍒拌鎷撴墤灞傜骇鐨勫敮涓€鎷撴墤 ID銆傚叡浜鎷撴墤 ID 鐨?CPU 鍏变韩璇ュ眰绾х殑璧勬簮銆?
      濡傛灉鏀寔 CPUID 鍙跺瓙 0xB锛屽垯鏃犻渶杩涗竴姝ヨВ鏋愩€?

   3) CPUID leaf 0x80000008 ECX [Size Identifiers] (Core::X86::Cpuid::SizeId)

      濡傛灉鏃笉鏀寔 CPUID 鍙跺瓙 0x80000026 涔熶笉鏀寔 0xB锛屽垯浣跨敤 Size Identifier 鍙跺瓙 0x80000008 ECX 鏉ユ娴嬪皝瑁呬笂鐨?CPU 鏁伴噺銆?
      閫氳繃妫€鏌ユ敮鎸佺殑鎵╁睍 CPUID 绾у埆鏄惁 >= 0x80000008锛屾潵鍙戠幇瀵硅鍙跺瓙鐨勬敮鎸併€?
      鑻?`ECX[15:12]` 涓殑 `ApicIdSize` 瀛楁闈為浂锛屽垯浠?APIC ID 鍒?Socket ID 鐨勪綅绉婚噺鐢辫瀛楁璁＄畻寰楀嚭銆?
      濡傛灉 `ApicIdSize` 鎶ュ憡涓洪浂锛屽垯浣嶇Щ閲忔寜 `ECX[7:0]` 涓?`NC` 瀛楁锛堟弿杩板皝瑁呬笂 `绾跨▼鏁?- 1`锛夎绠楀嚭鐨?`绾跨▼鏁癭 鐨勯樁鏉ヨ绠椼€?
      闄ら潪鏀寔 Extended APIC ID锛屽惁鍒欑敤浜庢煡鎵?Socket ID 鐨?APIC ID 鏉ヨ嚜 CPUID 鍙跺瓙 0x00000001 `EBX[31:24]` 涓殑 `LocalApicId` 瀛楁銆?
      鎷撴墤瑙ｆ瀽灏嗙户缁娴嬫槸鍚︽敮鎸?Extended APIC ID銆?

   4) CPUID leaf 0x8000001E [Extended APIC ID, Core Identifiers, Node Identifiers]
      (Core::X86::Cpuid::{ExtApicId,CoreId,NodeId})

      鍙互閫氳繃妫€鏌?CPUID 鍙跺瓙 0x80000001 [Feature Identifiers]
      (Core::X86::Cpuid::FeatureExtIdEcx) 鐨?`ECX[^22^]` 涓槸鍚﹀瓨鍦?`TopologyExtensions`锛屾潵妫€娴嬪 Extended APIC ID 鐨勬敮鎸併€?
      濡傛灉鏀寔 Topology Extensions锛屽垯搴斾紭鍏堜娇鐢?CPUID 鍙跺瓙 0x8000001E `EAX[31:0]` 涓?`ExtendedApicId` 鐨?APIC ID锛岃€岄潪鏉ヨ嚜 CPUID 鍙跺瓙 0x00000001 `EBX[31:24]` 涓?`LocalApicId` 瀛楁鐨?APIC ID锛岀敤浜庢嫇鎵戞灇涓俱€?
      鍦?Family 0x17 鍙婁互涓娿€佷笖涓嶆敮鎸?CPUID 鍙跺瓙 0x80000026 鎴?CPUID 鍙跺瓙 0xB 鐨勫鐞嗗櫒涓婏紝浠?APIC ID 鍒?Core ID 鐨勪綅绉婚噺浣跨敤 `EBX[15:8]` 涓?`ThreadsPerCore` 瀛楁锛堟弿杩?`姣忔牳绾跨▼鏁?- 1`锛夎绠楀嚭鐨?`姣忔牳绾跨▼鏁癭 鐨勯樁鏉ヨ绠椼€?
      鍦?Family 0x15 鐨勫鐞嗗櫒涓婏紝`EBX[7:0]` 涓殑 Core ID 琚敤浣?`cu_id`锛圕ompute Unit ID锛夛紝浠ユ娴嬪叡浜绠楀崟鍏冪殑 CPU銆?

   鎵€鏈夋敮鎸?`TopologyExtensions` 鐗规€х殑 AMD 澶勭悊鍣ㄩ兘浼氬皢 CPUID 鍙跺瓙 0x8000001E
   `ECX[7:0]` 涓殑 `NodeId` (Core::X86::Cpuid::NodeId) 瀛樺偍涓烘瘡 CPU 鐨?`node_id`銆傚湪杈冩棫鐨勫鐞嗗櫒涓婏紝`node_id` 鏄€氳繃 MSR_FAM10H_NODE_ID MSR锛圡SR
   0x0xc001_100c锛夊彂鐜扮殑銆侼ODE_ID MSR 鐨勫瓨鍦ㄦ槸閫氳繃妫€鏌?CPUID 鍙跺瓙 0x80000001 [Feature Identifiers]
   (Core::X86::Cpuid::FeatureExtIdEcx) 鐨?`ECX[^19^]` 鏉ユ娴嬬殑銆?

2) Intel

   鍦?Intel 骞冲彴涓婏紝鏋氫妇澶勭悊鍣ㄦ嫇鎵戠殑 CPUID 鍙跺瓙濡備笅锛?
   1) CPUID leaf 0x1F (V2 Extended Topology Enumeration Leaf)

      CPUID 鍙跺瓙 0x1F 鏄?CPUID 鍙跺瓙 0xB 鐨勬墿灞曪紝鎻愪緵浜嗘瘡涓€灞傜骇涓?Core銆丮odule銆乀ile銆丏ie銆丏ieGrp 鍜?Socket 鐨勬嫇鎵戜俊鎭€?
      閫氳繃妫€鏌ユ敮鎸佺殑 CPUID 绾у埆鏄惁 >= 0x1F锛岀劧鍚庣壒瀹氬眰绾э紙浠?0 寮€濮嬶級鐨?`EBX[31:0]` 鏄惁闈為浂锛屾潵鍙戠幇瀵硅鍙跺瓙鐨勬敮鎸併€?
      瀛愬彾瀛愪腑 `ECX[15:8]` 閲岀殑 `Domain Type` 缁欏嚭浜嗚灞傜骇鎵€鎻忚堪鐨勬嫇鎵戝煙鈥斺€擟ore銆丮odule銆乀ile銆丏ie銆丏ieGrp 鍜?Socket銆?
      鍐呮牳浣跨敤 `EAX[4:0]` 涓殑鍊兼潵鑾风煡闇€瑕佷粠 `EDX[31:0]` 涓殑 `x2APIC ID` 鍙崇Щ澶氬皯浣嶏紝浠ュ緱鍒拌鎷撴墤灞傜骇鐨勫敮涓€鎷撴墤 ID銆傚叿鏈夌浉鍚屾嫇鎵?ID 鐨?CPU 鍏变韩璇ュ眰绾х殑璧勬簮銆?
      濡傛灉鏀寔 CPUID 鍙跺瓙 0x1F锛屽垯鏃犻渶杩涗竴姝ヨВ鏋愩€?

   2) CPUID leaf 0x0000000B (Extended Topology Enumeration Leaf)

      鎵╁睍 CPUID 鍙跺瓙 0x0000000B 鏄?V2 鎵╁睍鎷撴墤鏋氫妇鍙跺瓙 0x1F 鐨勫墠韬紝浠呮弿杩板鐞嗗櫒鎷撴墤鐨勬牳涓庢彃妲藉煙銆?
      閫氳繃妫€鏌ユ敮鎸佺殑 CPUID 绾у埆鏄惁 >= 0xB锛岀劧鍚庢鏌ョ壒瀹氬眰绾э紙浠?0 寮€濮嬶級鐨?`EBX[31:0]` 鏄惁闈為浂锛屾潵鍙戠幇瀵硅鍙跺瓙鐨勬敮鎸併€?
      CPUID 鍙跺瓙 0x0000000B 涓?CPUID 鍙跺瓙 0x1F 鍏锋湁鐩稿悓鐨勫竷灞€锛屽簲浠ョ被浼兼柟寮忔灇涓俱€?
      濡傛灉鏀寔 CPUID 鍙跺瓙 0xB锛屽垯鏃犻渶杩涗竴姝ヨВ鏋愩€?

   3) CPUID leaf 0x00000004 (Deterministic Cache Parameters Leaf)

      鍦ㄦ棦涓嶆敮鎸?CPUID 鍙跺瓙 0x1F 涔熶笉鏀寔 CPUID 鍙跺瓙 0xB 鐨?Intel 澶勭悊鍣ㄤ笂锛孲MT 鍩熺殑浣嶇Щ閲忎娇鐢ㄥ叡浜?L1 缂撳瓨鐨?CPU 鏁伴噺鏉ヨ绠椼€?
      鏀寔瓒呯嚎绋嬶紙Hyper-Threading锛夌殑澶勭悊鍣ㄩ€氳繃 CPUID 鍙跺瓙 0x1锛圔asic CPUID Information锛夌殑 `EDX[^28^]` 鏉ユ娴嬨€?
      鏉ヨ嚜 CPUID 0x4 绗?0 灞?`EAX[25:14]` 鐨?`Maximum number of addressable IDs for logical processors sharing this cache` 鐨勯樁锛屾彁渚涗簡浠?APIC ID 璁＄畻 Core ID 鎵€闇€鐨勪綅绉婚噺銆?
      APIC ID 涓庡皝瑁呬俊鎭娇鐢ㄦ潵鑷?CPUID 鍙跺瓙 0x1 鐨勬暟鎹绠椼€?

   4) CPUID leaf 0x00000001 (Basic CPUID Information)

      鐢ㄤ簬鎺ㄥ鐗╃悊灏佽锛堟彃妲斤級ID 鐨勬帺鐮佷笌浣嶇Щ锛屼娇鐢?CPUID 鍙跺瓙 0x1 `EBX[23:16]` 涓殑 `Maximum number of addressable IDs for logical processors in this physical package` 鏉ヨ绠椼€?
     浼犵粺骞冲彴涓婄殑 APIC ID 鐢?CPUID 鍙跺瓙 0x1 `EBX[31:24]` 涓殑 `Initial APIC ID` 瀛楁鎺ㄥ銆?

3) Centaur 涓?Zhaoxin

   涓?Intel 绫讳技锛孋entaur 涓?Zhaoxin 浣跨敤 CPUID 鍙跺瓙 0x00000004锛圖eterministic Cache Parameters Leaf锛変笌 CPUID 鍙跺瓙 0x00000001锛圔asic CPUID Information锛夌殑缁勫悎鏉ユ帹瀵兼嫇鎵戜俊鎭€?


## 绯荤粺鎷撴墤绀轰緥


  Linux 鐨勫彟涓€绉?CPU 鏋氫妇鏂瑰紡鍙栧喅浜?BIOS 濡備綍鏋氫妇绾跨▼銆傝澶?BIOS 浼氬厛鏋氫妇鎵€鏈夌殑绾跨▼ 0锛岀劧鍚庡啀鏋氫妇鎵€鏈夌殑绾跨▼ 1銆傝繖鏍峰仛鏈変竴涓€滃ソ澶勨€濓細鏃犺鏄惁鍚敤绾跨▼锛岀嚎绋?0 鐨勯€昏緫 Linux CPU 缂栧彿閮戒繚鎸佷笉鍙樸€傝繖浠呬粎鏄竴涓疄鐜扮粏鑺傦紝娌℃湁瀹為檯褰卞搷銆?
```

   [package 0] -> [core 0] -> [thread 0] -> Linux CPU 0

```
2) 鍗曞皝瑁咃紝鍙屾牳

```

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
		    -> [core 1] -> [thread 0] -> Linux CPU 1

   b) 姣忔牳涓や釜绾跨▼::

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
				-> [thread 1] -> Linux CPU 1
		    -> [core 1] -> [thread 0] -> Linux CPU 2
				-> [thread 1] -> Linux CPU 3

      Alternative enumeration::

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
				-> [thread 1] -> Linux CPU 2
		    -> [core 1] -> [thread 0] -> Linux CPU 1
				-> [thread 1] -> Linux CPU 3

      AMD nomenclature for CMT systems::

	[node 0] -> [Compute Unit 0] -> [Compute Unit Core 0] -> Linux CPU 0
				     -> [Compute Unit Core 1] -> Linux CPU 1
		 -> [Compute Unit 1] -> [Compute Unit Core 0] -> Linux CPU 2
				     -> [Compute Unit Core 1] -> Linux CPU 3

```
4) 鍙屽皝瑁咃紝鍙屾牳

```

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
		    -> [core 1] -> [thread 0] -> Linux CPU 1

	[package 1] -> [core 0] -> [thread 0] -> Linux CPU 2
		    -> [core 1] -> [thread 0] -> Linux CPU 3

   b) 姣忔牳涓や釜绾跨▼::

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
				-> [thread 1] -> Linux CPU 1
		    -> [core 1] -> [thread 0] -> Linux CPU 2
				-> [thread 1] -> Linux CPU 3

	[package 1] -> [core 0] -> [thread 0] -> Linux CPU 4
				-> [thread 1] -> Linux CPU 5
		    -> [core 1] -> [thread 0] -> Linux CPU 6
				-> [thread 1] -> Linux CPU 7

      Alternative enumeration::

	[package 0] -> [core 0] -> [thread 0] -> Linux CPU 0
				-> [thread 1] -> Linux CPU 4
		    -> [core 1] -> [thread 0] -> Linux CPU 1
				-> [thread 1] -> Linux CPU 5

	[package 1] -> [core 0] -> [thread 0] -> Linux CPU 2
				-> [thread 1] -> Linux CPU 6
		    -> [core 1] -> [thread 0] -> Linux CPU 3
				-> [thread 1] -> Linux CPU 7

      AMD nomenclature for CMT systems::

	[node 0] -> [Compute Unit 0] -> [Compute Unit Core 0] -> Linux CPU 0
				     -> [Compute Unit Core 1] -> Linux CPU 1
		 -> [Compute Unit 1] -> [Compute Unit Core 0] -> Linux CPU 2
				     -> [Compute Unit Core 1] -> Linux CPU 3

	[node 1] -> [Compute Unit 0] -> [Compute Unit Core 0] -> Linux CPU 4
				     -> [Compute Unit Core 1] -> Linux CPU 5
		 -> [Compute Unit 1] -> [Compute Unit Core 0] -> Linux CPU 6
				     -> [Compute Unit Core 1] -> Linux CPU 7

```
