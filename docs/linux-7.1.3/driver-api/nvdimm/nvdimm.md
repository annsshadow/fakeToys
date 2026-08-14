## LIBNVDIMM锛氶潪鏄撳け鎬ц澶?
libnvdimm - 鍐呮牳 / libndctl - 鐢ㄦ埛绌洪棿杈呭姪搴?
nvdimm@lists.linux.dev

鐗堟湰 13

	鏈琛?	姒傝堪
	    鐩稿叧鏂囨。
	    Git 浠ｇ爜鏍?	LIBNVDIMM PMEM
	    PMEM-REGION銆佸師瀛愭墖鍖轰笌 DAX
	NVDIMM 骞冲彴绀轰緥
	LIBNVDIMM 鍐呮牳璁惧妯″瀷涓?LIBNDCTL 鐢ㄦ埛绌洪棿 API
	    LIBNDCTL锛氫笂涓嬫枃
	        libndctl锛氬疄渚嬪寲鏂扮殑搴撲笂涓嬫枃绀轰緥
	    LIBNVDIMM/LIBNDCTL锛氭€荤嚎锛圔us锛?	        libnvdimm锛?sys/class 涓殑鎺у埗绫昏澶?	        libnvdimm锛氭€荤嚎锛坆us锛?	        libndctl锛氭€荤嚎鏋氫妇绀轰緥
	    LIBNVDIMM/LIBNDCTL锛欴IMM锛圢MEM锛?	        libnvdimm锛欴IMM锛圢MEM锛?	        libndctl锛欴IMM 鏋氫妇绀轰緥
	    LIBNVDIMM/LIBNDCTL锛歊egion
	        libnvdimm锛歳egion
	        libndctl锛歳egion 鏋氫妇绀轰緥
	        涓轰綍涓嶆妸 Region 绫诲瀷缂栫爜杩?Region 鍚嶇О锛?	        濡備綍纭畾涓€涓?Region 鐨勪富瑕佺被鍨嬶紵
	    LIBNVDIMM/LIBNDCTL锛歂amespace
	        libnvdimm锛歯amespace
	        libndctl锛歯amespace 鏋氫妇绀轰緥
	        libndctl锛歯amespace 鍒涘缓绀轰緥
	        涓轰綍浣跨敤鏈 "namespace"锛?	    LIBNVDIMM/LIBNDCTL锛氬潡杞崲琛?"btt"
	        libnvdimm锛歜tt 甯冨眬
	        libndctl锛歜tt 鍒涘缓绀轰緥
	LIBNDCTL 鍥捐〃绀烘剰鎬荤粨

## 鏈琛?
PMEM锛?  涓€涓郴缁熺墿鐞嗗湴鍧€鑼冨洿锛屽叾涓殑鍐欏叆鏄寔涔呭寲鐨勩€傜敱 PMEM 缁勬垚鐨勫潡璁惧
  鑳藉鏀寔 DAX銆備竴涓?PMEM 鍦板潃鑼冨洿鍙互璺ㄥ涓?DIMM 鐨勪氦缁囥€?
DPA锛?  DIMM Physical Address锛圖IMM 鐗╃悊鍦板潃锛夛紝鏄浉瀵逛簬 DIMM 鐨勫亸绉婚噺銆?  褰撶郴缁熶腑鍙湁涓€涓?DIMM 鏃讹紝绯荤粺鐗╃悊鍦板潃涓?DPA 涔嬮棿鏄?1:1 鐨勫搴斿叧绯汇€?  涓€鏃﹀姞鍏ユ洿澶?DIMM锛屽氨蹇呴』瀵瑰唴瀛樻帶鍒跺櫒浜ょ粐杩涜瑙ｇ爜锛屼互纭畾涓庣粰瀹?  绯荤粺鐗╃悊鍦板潃鐩稿叧鑱旂殑 DPA銆?
DAX锛?  鏂囦欢绯荤粺鎵╁睍锛岀敤浜庣粫杩囬〉缂撳瓨鍜屽潡灞傦紝灏嗘潵鑷?PMEM 鍧楄澶囩殑鎸佷箙鍖栧唴瀛?  鐩存帴 mmap 鍒拌繘绋嬪湴鍧€绌洪棿涓€?
DSM锛?  Device Specific Method锛堣澶囩壒瀹氭柟娉曪級锛氱敤浜庢帶鍒剁壒瀹氳澶囩殑 ACPI 鏂规硶
  鈥斺€斿湪姝や緥涓嵆鍥轰欢銆?
DCR锛?  NVDIMM Control Region Structure锛圢VDIMM 鎺у埗鍖哄煙缁撴瀯锛夛紝瀹氫箟浜?ACPI 6
  绗?5.2.25.5 鑺傘€傚畠涓轰竴涓粰瀹氱殑 DIMM 瀹氫箟浜?vendor-id銆乨evice-id 浠ュ強
  鎺ュ彛鏍煎紡銆?
BTT锛?  Block Translation Table锛堝潡杞崲琛級锛氭寔涔呭寲鍐呭瓨鏄彲鎸夊瓧鑺傚鍧€鐨勩€?  鐜版湁鐨勮蒋浠跺彲鑳芥湡鏈涘啓鍏ョ殑鎺夌數鍘熷瓙鎬ц嚦灏戜负涓€涓墖鍖猴紝鍗?512 瀛楄妭銆侭TT 鏄?  涓€涓叿鏈夊師瀛愭洿鏂拌涔夌殑閲嶆槧灏勮〃锛屼綅浜?PMEM 鍧楄澶囬┍鍔ㄤ箣鍓嶏紝浠ュ憟鐜颁换鎰忕殑
  鍘熷瓙鎵囧尯澶у皬銆?
LABEL锛?  瀛樺偍鍦?DIMM 璁惧涓婄殑鍏冩暟鎹紝鐢ㄤ簬瀵瑰垎閰嶇粰涓嶅悓 PMEM namespace 鐨勫閲忚繘琛?  鍒嗗尯骞舵爣璇嗭紙鎸佷箙鍛藉悕锛夈€傚畠杩樻寚绀烘槸鍚﹀ namespace 搴旂敤浜嗗儚 BTT 杩欐牱鐨?  鍦板潃鎶借薄銆傛敞鎰忥紝浼犵粺鐨勫垎鍖鸿〃 GPT/MBR 鏄彔鍔犲湪 PMEM namespace 涔嬩笂锛屾垨鍦?  瀛樺湪鏃跺彔鍔犲湪鍍?BTT 杩欐牱鐨勫湴鍧€鎶借薄涔嬩笂锛屼絾鍒嗗尯鏀寔浠婂悗灏嗚寮冪敤銆?
## 姒傝堪

LIBNVDIMM 瀛愮郴缁熶负骞冲彴鍥轰欢鎴栬澶囬┍鍔ㄦ墍鎻忚堪鐨?PMEM 鎻愪緵鏀寔銆傚湪鍩轰簬 ACPI 鐨?绯荤粺涓婏紝骞冲彴鍥轰欢閫氳繃 ACPI 6 涓殑 ACPI NFIT锛?NVDIMM Firmware Interface
Table"锛孨VDIMM 鍥轰欢鎺ュ彛琛級浼犻€掓寔涔呭寲鍐呭瓨璧勬簮銆傝櫧鐒?LIBNVDIMM 瀛愮郴缁熺殑瀹炵幇
鏄€氱敤鐨勫苟鏀寔 NFIT 涔嬪墠鐨勫钩鍙帮紝浣嗗畠鍙楀埌浜嗘敮鎸佹 ACPI 6 瀵?NVDIMM 璧勬簮瀹氫箟
鎵€闇€鑳藉姏鍏ㄩ泦鐨勬寚瀵笺€傛渶鍒濈殑瀹炵幇鏀寔 NFIT 涓弿杩扮殑 block-window-aperture锛堝潡
绐楀彛瀛斿緞锛夎兘鍔涳紝浣嗚鏀寔鍚庢潵宸茶鏀惧純锛屼粠鏈湪浠讳綍浜у搧涓彂甯冦€?
### 鐩稿叧鏂囨。

ACPI 6锛?	https://www.uefi.org/sites/default/files/resources/ACPI_6.0.pdf
NVDIMM Namespace锛?	https://pmem.io/documents/NVDIMM_Namespace_Spec.pdf
DSM Interface Example锛?	https://pmem.io/documents/NVDIMM_DSM_Interface_Example.pdf
Driver Writer's Guide锛?	https://pmem.io/documents/NVDIMM_Driver_Writers_Guide.pdf

### Git 浠ｇ爜鏍?
LIBNVDIMM锛?	https://git.kernel.org/cgit/linux/kernel/git/nvdimm/nvdimm.git
LIBNDCTL锛?	https://github.com/pmem/ndctl.git

## LIBNVDIMM PMEM

鍦?NFIT 鍑虹幇涔嬪墠锛岄潪鏄撳け鎬у唴瀛樹互鍚勭涓存椂鐨勬柟寮忔弿杩扮粰绯荤粺銆傞€氬父鍙彁渚涙渶
鍩烘湰鐨勮绱狅紝鍗充竴涓郴缁熺墿鐞嗗湴鍧€鑼冨洿锛屽叾涓殑鍐欏叆棰勬湡鍦ㄧ郴缁熸帀鐢靛悗浠嶇劧鎸佷箙銆?鐜板湪锛孨FIT 瑙勮寖涓嶄粎鏍囧噯鍖栦簡 PMEM 鐨勬弿杩帮紝杩樻爣鍑嗗寲浜嗙敤浜庢帶鍒跺拰閰嶇疆鐨?骞冲彴娑堟伅浼犻€掑叆鍙ｇ偣銆?
PMEM锛坣d_pmem.ko锛夛細椹卞姩涓€涓郴缁熺墿鐞嗗湴鍧€鑼冨洿銆傝鑼冨洿鍦ㄧ郴缁熷唴瀛樹腑鏄繛缁殑锛?骞朵笖鍙互璺ㄥ涓?DIMM 杩涜浜ょ粐锛堢‖浠跺唴瀛樻帶鍒跺櫒鏉″甫鍖栵級銆傚綋杩涜浜ょ粐鏃讹紝骞冲彴
鍙互閫夋嫨鎻愪緵鍝簺 DIMM 鍙備笌浜嗚浜ょ粐鐨勭粏鑺傘€?
鍊煎緱娉ㄦ剰鐨勬槸锛屽綋妫€娴嬪埌鏍囨敞锛坙abeling锛夎兘鍔涙椂锛堟壘鍒颁簡涓€涓?EFI namespace
label index block锛夛紝榛樿涓嶄細鍒涘缓浠讳綍鍧楄澶囷紝鍥犱负鐢ㄦ埛绌洪棿鑷冲皯闇€瑕佸
PMEM 鑼冨洿杩涜涓€娆?DPA 鍒嗛厤銆傜浉姣斾箣涓嬶紝涓€鏃︽敞鍐岋紝ND_NAMESPACE_IO 鑼冨洿鍙互
绔嬪嵆鎸傝浇鍒?nd_pmem銆傚悗涓€绉嶆ā寮忕О涓烘棤鏍囨敞锛坙abel-less锛夋垨"legacy"锛堜紶缁燂級銆?
### PMEM-REGION銆佸師瀛愭墖鍖轰笌 DAX

瀵逛簬搴旂敤绋嬪簭鎴栨枃浠剁郴缁熶粛闇€瑕佸師瀛愭墖鍖烘洿鏂颁繚璇佺殑鎯呭喌锛屽畠鍙互鍦?PMEM 璁惧鎴?鍒嗗尯涓婃敞鍐屼竴涓?BTT銆傚弬瑙?LIBNVDIMM/NDCTL锛欱lock Translation Table "btt"銆?
## NVDIMM 骞冲彴绀轰緥

鏈枃妗ｇ殑鍏朵綑閮ㄥ垎灏嗕娇鐢ㄤ互涓嬬ず鎰忓浘锛?
```

                               (a)               (b)           DIMM
            +-------------------+--------+--------+--------+
  +------+  |       pm0.0       |  free  | pm1.0  |  free  |    0
  | imc0 +--+- - - region0- - - +--------+        +--------+
  +--+---+  |       pm0.0       |  free  | pm1.0  |  free  |    1
     |      +-------------------+--------v        v--------+
  +--+---+                               |                 |
  | cpu0 |                                     region1
  +--+---+                               |                 |
     |      +----------------------------^        ^--------+
  +--+---+  |           free             | pm1.0  |  free  |    2
  | imc1 +--+----------------------------|        +--------+
  +------+  |           free             | pm1.0  |  free  |    3
            +----------------------------+--------+--------+

```
鍦ㄨ骞冲彴涓婏紝鎴戜滑鍦ㄥ崟涓彃妲戒腑鏈夊洓涓?DIMM 鍜屼袱涓唴瀛樻帶鍒跺櫒銆傛瘡涓?PMEM 浜ょ粐
闆嗙敱涓€涓叿鏈夊姩鎬佸垎閰?id 鐨?region 璁惧鏍囪瘑銆?
    1. DIMM0 鍜?DIMM1 鐨勫墠鍗婇儴鍒嗕綔涓?REGION0 浜ょ粐鍦ㄤ竴璧枫€備竴涓崟涓€鐨?       PMEM namespace 鍒涘缓浜?REGION0-SPA-range 涓紝瀹冩í璺ㄥぇ閮ㄥ垎 DIMM0 鍜?       DIMM1锛岀敤鎴锋寚瀹氱殑鍚嶇О涓?"pm0.0"銆傞儴鍒嗕氦缁囩殑绯荤粺鐗╃悊鍦板潃鑼冨洿琚暀浣?       绌洪棽锛屼互渚垮畾涔夊彟涓€涓?PMEM namespace銆?
    2. 鍦?DIMM0 鍜?DIMM1 鐨勬渶鍚庨儴鍒嗭紝鎴戜滑鏈変竴涓氦缁囩殑绯荤粺鐗╃悊鍦板潃鑼冨洿
       REGION1锛屽畠妯法杩欎袱涓?DIMM 浠ュ強 DIMM2 鍜?DIMM3銆俁EGION1 鐨勪竴閮ㄥ垎琚垎閰?       缁欎竴涓悕涓?"pm1.0" 鐨?PMEM namespace銆?
    璇ユ€荤嚎鐢卞唴鏍稿湪鍔犺浇鏉ヨ嚜 tools/testing/nvdimm 鐨?nfit_test.ko 妯″潡鏃讹紝
    浜庤澶?/sys/devices/platform/nfit_test.0 涓嬫彁渚涖€傝妯″潡鏄?LIBNVDIMM 鍜?    acpi_nfit.ko 椹卞姩鐨勪竴涓崟鍏冩祴璇曘€?
## LIBNVDIMM 鍐呮牳璁惧妯″瀷涓?LIBNDCTL 鐢ㄦ埛绌洪棿 API

涓嬮潰鏄 LIBNVDIMM sysfs 甯冨眬浠ュ強閫氳繃 LIBNDCTL API 鏌ョ湅鐨勭浉搴斿璞″眰绾?绀烘剰鍥剧殑鎻忚堪銆傜ず渚?sysfs 璺緞鍜岀ず鎰忓浘鏄浉瀵逛簬 NVDIMM 骞冲彴绀轰緥鐨勶紝璇ョず渚?鍚屾椂涔熸槸 LIBNDCTL 鍗曞厓娴嬭瘯涓娇鐢ㄧ殑 LIBNVDIMM 鎬荤嚎銆?
### LIBNDCTL锛氫笂涓嬫枃

LIBNDCTL 搴撲腑鐨勬瘡涓?API 璋冪敤閮介渶瑕佷竴涓?context锛堜笂涓嬫枃锛夛紝瀹冧繚瀛樻棩蹇楀弬鏁板拰
鍏朵粬搴撳疄渚嬬姸鎬併€傝搴撳熀浜?libabc 妯℃澘锛?
	https://git.kernel.org/cgit/linux/kernel/git/kay/libabc.git

##### LIBNDCTL锛氬疄渚嬪寲鏂扮殑搴撲笂涓嬫枃绀轰緥

```
	struct ndctl_ctx *ctx;

	if (ndctl_new(&ctx) == 0)
		return ctx;
	else
		return NULL;
```

### LIBNVDIMM/LIBNDCTL锛氭€荤嚎锛圔us锛?
涓€涓€荤嚎锛坆us锛変笌涓€涓?NFIT 涔嬮棿瀛樺湪 1:1 鐨勫叧绯汇€傚浜庡熀浜?ACPI 鐨勭郴缁燂紝褰撳墠
鐨勯鏈熸槸鍙湁涓€涓钩鍙板叏灞€鐨?NFIT銆備篃灏辨槸璇达紝娉ㄥ唽澶氫釜 NFIT 鏄交鑰屾槗涓剧殑锛岃鑼?骞朵笉鎺掗櫎杩欑鎯呭喌銆傝鍩虹璁炬柦鏀寔澶氫釜鎬荤嚎锛屾垜浠湪鍗曞厓娴嬭瘯涓埄鐢ㄨ繖涓€鑳藉姏鏉?娴嬭瘯澶氱 NFIT 閰嶇疆銆?
### LIBNVDIMM锛?sys/class 涓殑鎺у埗绫昏澶?
璇ュ瓧绗﹁澶囨帴鍙楄浼犻€掔粰 DIMM 鐨?DSM 娑堟伅锛?
```
	/sys/class/nd/ndctl0
	|-- dev
	|-- device -> ../../../ndbus0
	|-- subsystem -> ../../../../../../../class/nd
```

### LIBNVDIMM锛氭€荤嚎锛坆us锛?
```
	struct nvdimm_bus *nvdimm_bus_register(struct device *parent,
	       struct nvdimm_bus_descriptor *nfit_desc);
```

```
	/sys/devices/platform/nfit_test.0/ndbus0
	|-- commands
	|-- nd
	|-- nfit
	|-- nmem0
	|-- nmem1
	|-- nmem2
	|-- nmem3
	|-- power
	|-- provider
	|-- region0
	|-- region1
	|-- region2
	|-- region3
	|-- region4
	|-- region5
	|-- uevent
	`-- wait_probe
```

##### LIBNDCTL锛氭€荤嚎鏋氫妇绀轰緥

```
	static struct ndctl_bus *get_bus_by_provider(struct ndctl_ctx *ctx,
			const char *provider)
	{
		struct ndctl_bus *bus;

		ndctl_bus_foreach(ctx, bus)
			if (strcmp(provider, ndctl_bus_get_provider(bus)) == 0)
				return bus;

		return NULL;
	}

	bus = get_bus_by_provider(ctx, "nfit_test.0");
```

### LIBNVDIMM/LIBNDCTL锛欴IMM锛圢MEM锛?
DIMM 璁惧鎻愪緵浜嗕竴涓瓧绗﹁澶囩敤浜庡悜纭欢鍙戦€佸懡浠わ紝骞朵笖瀹冩槸 LABEL 鐨勫鍣ㄣ€傚鏋?DIMM 鐢?NFIT 瀹氫箟锛屽垯鎻愪緵涓€涓彲閫夌殑 'nfit' 灞炴€у瓙鐩綍鏉ユ坊鍔?NFIT 鐗规湁鐨勫唴瀹广€?
娉ㄦ剰锛?DIMM"鐨勫唴鏍歌澶囧悕鏄?"nmemX"銆侼FIT 閫氳繃"Memory Device to System
Physical Address Range Mapping Structure"锛堝唴瀛樿澶囧埌绯荤粺鐗╃悊鍦板潃鑼冨洿鏄犲皠
缁撴瀯锛夋弿杩拌繖浜涜澶囷紝骞朵笖涓嶈姹傚畠浠疄闄呬笂蹇呴』鏄墿鐞?DIMM锛屽洜姝ゆ垜浠娇鐢ㄤ簡涓€涓?鏇撮€氱敤鐨勫悕绉般€?
##### LIBNVDIMM锛欴IMM锛圢MEM锛?
```
	struct nvdimm *nvdimm_create(struct nvdimm_bus *nvdimm_bus, void *provider_data,
			const struct attribute_group **groups, unsigned long flags,
			unsigned long *dsm_mask);
```

```
	/sys/devices/platform/nfit_test.0/ndbus0
	|-- nmem0
	|   |-- available_slots
	|   |-- commands
	|   |-- dev
	|   |-- devtype
	|   |-- driver -> ../../../../../bus/nd/drivers/nvdimm
	|   |-- modalias
	|   |-- nfit
	|   |   |-- device
	|   |   |-- format
	|   |   |-- handle
	|   |   |-- phys_id
	|   |   |-- rev_id
	|   |   |-- serial
	|   |   `-- vendor
	|   |-- state
	|   |-- subsystem -> ../../../../../bus/nd
	|   `-- uevent
	|-- nmem1
	[..]
```

##### LIBNDCTL锛欴IMM 鏋氫妇绀轰緥

娉ㄦ剰锛屽湪姝ょず渚嬩腑鎴戜滑鍋囪鐨勬槸鐢?NFIT 瀹氫箟鐨?DIMM锛屽畠浠敱涓€涓?32 浣嶅€肩殑
"nfit_handle" 鏍囪瘑锛屽叾涓細

   - Bit 3:0 鍐呭瓨閫氶亾鍐呯殑 DIMM 缂栧彿
   - Bit 7:4 鍐呭瓨閫氶亾缂栧彿
   - Bit 11:8 鍐呭瓨鎺у埗鍣?ID
   - Bit 15:12 鎻掓Ы ID锛堝鏋滃瓨鍦ㄨ妭鐐规帶鍒跺櫒锛屽垯鍦ㄨ妭鐐规帶鍒跺櫒鑼冨洿鍐咃級
   - Bit 27:16 鑺傜偣鎺у埗鍣?ID
   - Bit 31:28 淇濈暀

```
	static struct ndctl_dimm *get_dimm_by_handle(struct ndctl_bus *bus,
	       unsigned int handle)
	{
		struct ndctl_dimm *dimm;

		ndctl_dimm_foreach(bus, dimm)
			if (ndctl_dimm_get_handle(dimm) == handle)
				return dimm;

		return NULL;
	}

	#define DIMM_HANDLE(n, s, i, c, d) \
		(((n & 0xfff) << 16) | ((s & 0xf) << 12) | ((i & 0xf) << 8) \
		 | ((c & 0xf) << 4) | (d & 0xf))

	dimm = get_dimm_by_handle(bus, DIMM_HANDLE(0, 0, 0, 0, 0));
```

### LIBNVDIMM/LIBNDCTL锛歊egion

涓烘瘡涓?PMEM 浜ょ粐闆?鑼冨洿娉ㄥ唽涓€涓€氱敤鐨?REGION 璁惧銆傛寜绀轰緥锛屽湪 "nfit_test.0"
鎬荤嚎涓婃湁 2 涓?PMEM region銆俽egion 鐨勪富瑕佽鑹叉槸浣滀负 "mappings"锛堟槧灏勶級鐨勫鍣ㄣ€?涓€涓?mapping 鏄竴涓厓缁?<DIMM, DPA-start-offset, length>銆?
LIBNVDIMM 涓?REGION 璁惧鎻愪緵浜嗕竴涓唴缃┍鍔ㄣ€傝椹卞姩璐熻矗瑙ｆ瀽鎵€鏈?LABEL锛堝鏋?瀛樺湪锛夛紝鐒跺悗鍙戝嚭渚?nd_pmem 椹卞姩浣跨敤鐨?NAMESPACE 璁惧銆?
闄や簡 "mapping"銆?interleave_ways"锛堜氦缁囪矾鏁帮級鍜?"size"锛堝ぇ灏忥級杩欎簺閫氱敤灞炴€?澶栵紝REGION 璁惧杩樺鍑轰簡涓€浜涗究鍒╁睘鎬с€?nstype" 鎸囩ず璇?region 鍙戝嚭鐨?namespace 璁惧鐨勬暣鏁扮被鍨嬶紱"devtype" 澶嶅埗浜?udev 鍦?'add' 浜嬩欢鏃跺瓨鍌ㄧ殑
DEVTYPE 鍙橀噺锛?modalias" 澶嶅埗浜?udev 鍦?'add' 浜嬩欢鏃跺瓨鍌ㄧ殑 MODALIAS 鍙橀噺锛?鏈€鍚庯紝鍦?region 鐢?SPA 瀹氫箟鐨勬儏鍐典笅锛屾彁渚涘彲閫夌殑 "spa_index"銆?
```
	struct nd_region *nvdimm_pmem_region_create(struct nvdimm_bus *nvdimm_bus,
			struct nd_region_desc *ndr_desc);
```

```
	/sys/devices/platform/nfit_test.0/ndbus0
	|-- region0
	|   |-- available_size
	|   |-- btt0
	|   |-- btt_seed
	|   |-- devtype
	|   |-- driver -> ../../../../../bus/nd/drivers/nd_region
	|   |-- init_namespaces
	|   |-- mapping0
	|   |-- mapping1
	|   |-- mappings
	|   |-- modalias
	|   |-- namespace0.0
	|   |-- namespace_seed
	|   |-- numa_node
	|   |-- nfit
	|   |   `-- spa_index
	|   |-- nstype
	|   |-- set_cookie
	|   |-- size
	|   |-- subsystem -> ../../../../../bus/nd
	|   `-- uevent
	|-- region1
	[..]
```

##### LIBNDCTL锛歳egion 鏋氫妇绀轰緥

鍩轰簬 NFIT 鍞竴鏁版嵁锛堝 "spa_index"锛屽嵆浜ょ粐闆?id锛夌殑绀轰緥 region 妫€绱緥绋嬨€?
```
	static struct ndctl_region *get_pmem_region_by_spa_index(struct ndctl_bus *bus,
			unsigned int spa_index)
	{
		struct ndctl_region *region;

		ndctl_region_foreach(bus, region) {
			if (ndctl_region_get_type(region) != ND_DEVICE_REGION_PMEM)
				continue;
			if (ndctl_region_get_spa_index(region) == spa_index)
				return region;
		}
		return NULL;
	}
```

### LIBNVDIMM/LIBNDCTL锛歂amespace

涓€涓?REGION 鍦ㄨВ鏋愬畬 DPA 鍒悕鍜?LABEL 鎸囧畾鐨勮竟鐣屽悗锛屼細鍛堢幇鍑轰竴涓垨澶氫釜
"namespace" 璁惧銆?namespace" 璁惧鐨勫嚭鐜板綋鍓嶄細瑙﹀彂 nd_pmem 椹卞姩鍔犺浇骞舵敞鍐?涓€涓鐩?鍧楄澶囥€?
##### LIBNVDIMM锛歯amespace

浠ヤ笅鏄袱澶х被 NAMESPACE 鐨勭ず渚嬪竷灞€锛屽叾涓?namespace0.0 浠ｈ〃鐢?DIMM 淇℃伅鏀拺鐨?PMEM锛堟敞鎰忓畠鏈変竴涓?'uuid' 灞炴€э級锛岃€?namespace1.0 浠ｈ〃涓€涓尶鍚嶇殑 PMEM
namespace锛堟敞鎰忕敱浜庢病鏈?LABEL 鏀寔锛屽畠娌℃湁 'uuid' 灞炴€э級銆?
```
	/sys/devices/platform/nfit_test.0/ndbus0/region0/namespace0.0
	|-- alt_name
	|-- devtype
	|-- dpa_extents
	|-- force_raw
	|-- modalias
	|-- numa_node
	|-- resource
	|-- size
	|-- subsystem -> ../../../../../../bus/nd
	|-- type
	|-- uevent
	`-- uuid
	/sys/devices/platform/nfit_test.1/ndbus1/region1/namespace1.0
	|-- block
	|   `-- pmem0
	|-- devtype
	|-- driver -> ../../../../../../bus/nd/drivers/pmem
	|-- force_raw
	|-- modalias
	|-- numa_node
	|-- resource
	|-- size
	|-- subsystem -> ../../../../../../bus/nd
	|-- type
	`-- uevent
```

##### LIBNDCTL锛歯amespace 鏋氫妇绀轰緥

Namespace 鏄浉瀵逛簬鍏剁埗 region 寤虹珛绱㈠紩鐨勶紝绀轰緥濡備笅銆傝繖浜涚储寮曚粠鍚姩鍒板惎鍔ㄥぇ澶?鏄潤鎬佺殑锛屼絾瀛愮郴缁熷湪杩欐柟闈笉浣滀换浣曚繚璇併€傝鑾峰緱闈欐€佺殑 namespace 鏍囪瘑绗︼紝璇蜂娇鐢?鍏?'uuid' 灞炴€с€?
```
  static struct ndctl_namespace
  *get_namespace_by_id(struct ndctl_region *region, unsigned int id)
  {
          struct ndctl_namespace *ndns;

          ndctl_namespace_foreach(region, ndns)
                  if (ndctl_namespace_get_id(ndns) == id)
                          return ndns;

          return NULL;
  }
```

##### LIBNDCTL锛歯amespace 鍒涘缓绀轰緥

濡傛灉缁欏畾 region 鏈夎冻澶熺殑鍙敤瀹归噺鏉ュ垱寤烘柊鐨?namespace锛岀┖闂茬殑 namespace 浼氱敱
鍐呮牳鑷姩鍒涘缓銆俷amespace 瀹炰緥鍖栨秹鍙婃壘鍒颁竴涓┖闂?namespace 骞堕厤缃畠銆傚湪澶у鏁?鎯呭喌涓嬶紝namespace 灞炴€х殑璁剧疆鍙互浠ヤ换鎰忛『搴忚繘琛岋紝鍞竴鐨勭害鏉熸槸 'uuid' 蹇呴』鍦?'size' 涔嬪墠璁剧疆銆傝繖浣垮緱鍐呮牳鑳藉璺熻釜 DPA 鍒嗛厤銆?
```
  static int configure_namespace(struct ndctl_region *region,
                  struct ndctl_namespace *ndns,
                  struct namespace_parameters *parameters)
  {
          char devname[50];

          snprintf(devname, sizeof(devname), "namespace%d.%d",
                          ndctl_region_get_id(region), parameters->id);

          ndctl_namespace_set_alt_name(ndns, devname);
          /* 'uuid' 蹇呴』鍦ㄨ缃?size 涔嬪墠璁剧疆锛?*/
          ndctl_namespace_set_uuid(ndns, parameters->uuid);
          ndctl_namespace_set_size(ndns, parameters->size);
          /* 涓?pmem namespace 涓嶅悓锛宐lk namespace 鏈変竴涓墖鍖哄ぇ灏?*/
          if (parameters->lbasize)
                  ndctl_namespace_set_sector_size(ndns, parameters->lbasize);
          ndctl_namespace_enable(ndns);
  }
```

##### 涓轰綍浣跨敤鏈 "namespace"锛?
    1. 渚嬪涓轰粈涔堜笉鐢?"volume"锛堝嵎锛夛紵"volume" 鏈夊皢 ND锛坙ibnvdimm 瀛愮郴缁燂級涓?       鍍?device-mapper 杩欐牱鐨勫嵎绠＄悊鍣ㄦ贩娣嗙殑椋庨櫓銆?
    2. 璇ユ湳璇捣婧愪簬鎻忚堪鍙湪 NVME 鎺у埗鍣ㄥ唴鍒涘缓鐨勫瓙璁惧锛堝弬瑙?nvme 瑙勮寖锛?       https://www.nvmexpress.org/specifications/锛夛紝鑰?NFIT namespace 鏃ㄥ湪
       涓?NVME-namespace 鐨勮兘鍔涘拰鍙€氳繃閰嶇疆鎬х浉骞宠銆?
### LIBNVDIMM/LIBNDCTL锛氬潡杞崲琛?"btt"

BTT锛堣璁℃枃妗ｏ細https://pmem.io/2014/09/23/btt.html锛夋槸涓€涓?namespace 鐨?personality 椹卞姩锛屽畠灏嗘暣涓?namespace 浣滀负"鍦板潃鎶借薄"鍛堢幇浜庡墠绔€?
##### LIBNVDIMM锛歜tt 甯冨眬

姣忎釜 region 涓€寮€濮嬭嚦灏戜細鏈変竴涓?BTT 璁惧锛屽嵆绉嶅瓙锛坰eed锛夎澶囥€傝婵€娲诲畠锛岄渶璁剧疆
"namespace"銆?uuid" 鍜?"sector_size" 灞炴€э紝鐒跺悗灏嗚澶囩粦瀹氬埌 nd_pmem 鎴栵細

```
	/sys/devices/platform/nfit_test.1/ndbus0/region0/btt0/
	|-- namespace
	|-- delete
	|-- devtype
	|-- modalias
	|-- numa_node
	|-- sector_size
	|-- subsystem -> ../../../../../bus/nd
	|-- uevent
	`-- uuid
```

##### LIBNDCTL锛歜tt 鍒涘缓绀轰緥

涓?namespace 绫讳技锛屾瘡涓?region 浼氳嚜鍔ㄥ垱寤轰竴涓┖闂茬殑 BTT 璁惧銆傛瘡娆￠厤缃苟鍚敤
杩欎釜"绉嶅瓙"btt 璁惧鏃讹紝閮戒細鍒涘缓涓€涓柊鐨勭瀛愩€傚垱寤轰竴涓?BTT 閰嶇疆娑夊強涓ゆ锛氭壘鍒?绌洪棽 BTT 骞跺皢鍏跺垎閰嶄互娑堣垂涓€涓?namespace銆?
```
	static struct ndctl_btt *get_idle_btt(struct ndctl_region *region)
	{
		struct ndctl_btt *btt;

		ndctl_btt_foreach(region, btt)
			if (!ndctl_btt_is_enabled(btt)
					&& !ndctl_btt_is_configured(btt))
				return btt;

		return NULL;
	}

	static int configure_btt(struct ndctl_region *region,
			struct btt_parameters *parameters)
	{
		btt = get_idle_btt(region);

		ndctl_btt_set_uuid(btt, parameters->uuid);
		ndctl_btt_set_sector_size(btt, parameters->sector_size);
		ndctl_btt_set_namespace(btt, parameters->ndns);
		/* 鍏抽棴鍘熷妯″紡璁惧 */
		ndctl_namespace_disable(parameters->ndns);
		/* 寮€鍚?btt 璁块棶 */
		ndctl_btt_enable(btt);
	}
```

涓€鏃﹀疄渚嬪寲锛屼竴涓柊鐨勬湭婵€娲?btt 绉嶅瓙璁惧灏嗗嚭鐜板湪 region 涔嬩笅銆?
涓€鏃︿竴涓?"namespace" 浠?BTT 涓Щ闄わ紝璇?BTT 璁惧瀹炰緥灏嗚鍒犻櫎鎴栦互鍏朵粬鏂瑰紡閲嶇疆涓?榛樿鍊笺€傝繖绉嶅垹闄や粎鍙戠敓鍦ㄨ澶囨ā鍨嬪眰闈€備负浜嗛攢姣佷竴涓?BTT锛岄渶瑕侀攢姣佸叾 "info
block"锛堜俊鎭潡锛夈€傛敞鎰忥紝瑕侀攢姣佷竴涓?BTT锛岄渶瑕佷互鍘熷妯″紡鍐欏叆浠嬭川銆傞粯璁ゆ儏鍐典笅锛?鍐呮牳浼氳嚜鍔ㄦ娴?BTT 鐨勫瓨鍦ㄥ苟绂佺敤鍘熷妯″紡銆傛鑷姩妫€娴嬭涓哄彲浠ラ€氳繃涓?namespace
鍚敤鍘熷妯″紡鏉ユ姂鍒讹紝浣跨敤 ndctl_namespace_set_raw_mode() API銆?
### LIBNDCTL 鍥捐〃绀烘剰鎬荤粨

瀵逛簬涓婇潰缁欏嚭鐨勭ず渚嬶紝浠ヤ笅鏄瀵硅薄閫氳繃 API 鎵€鐪嬪埌鐨勮鍥撅細

```
              +---+
              |CTX|
              +-+-+
                |
  +-------+     |
  | DIMM0 <-+   |      +---------+   +--------------+  +---------------+
  +-------+ |   |    +-> REGION0 +---> NAMESPACE0.0 +--> PMEM8 "pm0.0" |
  | DIMM1 <-+ +-v--+ | +---------+   +--------------+  +---------------+
  +-------+ +-+BUS0+-| +---------+   +--------------+  +----------------------+
  | DIMM2 <-+ +----+ +-> REGION1 +---> NAMESPACE1.0 +--> PMEM6 "pm1.0" | BTT1 |
  +-------+ |        | +---------+   +--------------+  +---------------+------+
  | DIMM3 <-+
  +-------+
```
