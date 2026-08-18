## VFIO 鈥?鈥淰irtual Function I/O鈥?[1]_


濡備粖璁稿鐜颁唬绯荤粺閮芥彁渚?DMA 涓庝腑鏂噸鏄犲皠璁炬柦锛屼互甯姪纭繚 I/O 璁惧鍦ㄥ叾琚垎閰嶇殑杈圭晫鍐呰繍琛屻€傝繖鍖呮嫭甯︽湁 AMD-Vi 鍜?Intel VT-d 鐨?x86 纭欢銆佸甫鏈夊彲鍒嗗尯绔偣锛圥artitionable Endpoints锛孭Es锛夌殑 POWER 绯荤粺锛屼互鍙婂祵鍏ュ紡 PowerPC 绯荤粺锛堝 Freescale PAMU锛夈€俈FIO 椹卞姩鏄竴涓笌 IOMMU/璁惧鏃犲叧鐨勬鏋讹紝鐢ㄤ簬鍦ㄥ畨鍏ㄣ€佸彈 IOMMU 淇濇姢鐨勭幆澧冧腑鍚戠敤鎴风┖闂存毚闇茬洿鎺ョ殑璁惧璁块棶銆傛崲鍙ヨ瘽璇达紝瀹冨厑璁稿畨鍏?[^2]_銆侀潪鐗规潈鐨勭敤鎴风┖闂撮┍鍔ㄣ€?
鎴戜滑涓轰粈涔堥渶瑕佸畠锛熻櫄鎷熸満鍦ㄩ厤缃负灏藉彲鑳介珮鐨?I/O 鎬ц兘鏃讹紝閫氬父浼氫娇鐢ㄧ洿鎺ョ殑璁惧璁块棶锛堚€滆澶囩洿閫氣€濓紝device assignment锛夈€備粠璁惧鍜屼富鏈虹殑瑙掑害鏉ョ湅锛岃繖鍙笉杩囨槸鎶婅櫄鎷熸満鍙樻垚浜嗙敤鎴风┖闂撮┍鍔紝鍏跺ソ澶勬槸鏄捐憲鏇翠綆鐨勫欢杩熴€佹洿楂樼殑甯﹀锛屼互鍙婄洿鎺ヤ娇鐢ㄨ８閲戝睘璁惧椹卞姩 [^3^]_銆?
鏌愪簺搴旂敤锛屽挨鍏舵槸楂樻€ц兘璁＄畻棰嗗煙鐨勫簲鐢紝涔熻兘浠庣敤鎴风┖闂寸殑浣庡紑閿€鐩存帴璁惧璁块棶涓彈鐩娿€備緥瀛愬寘鎷綉缁滈€傞厤鍣紙閫氬父闈炲熀浜?TCP/IP锛夊拰璁＄畻鍔犻€熷櫒銆傚湪 VFIO 鍑虹幇涔嬪墠锛岃繖浜涢┍鍔ㄨ涔堝繀椤荤粡杩囧畬鏁寸殑寮€鍙戝懆鏈熸墠鑳芥垚涓哄悎閫傜殑涓婃父椹卞姩锛岃涔堝湪鏍戝缁存姢锛岃涔堜娇鐢?UIO 妗嗘灦鈥斺€旇€?UIO 娌℃湁 IOMMU 淇濇姢鐨勬蹇点€佷腑鏂敮鎸佹湁闄愶紝骞朵笖闇€瑕?root 鏉冮檺鎵嶈兘璁块棶 PCI 閰嶇疆绌洪棿涔嬬被鐨勫唴瀹广€?
VFIO 椹卞姩妗嗘灦鏃ㄥ湪缁熶竴杩欎簺鏂规锛屾棦鍙栦唬 KVM 涓?PCI 鐗瑰畾鐨勮澶囩洿閫氫唬鐮侊紝鍙堟彁渚涙瘮 UIO 鏇村畨鍏ㄣ€佸姛鑳芥洿涓板瘜鐨勭敤鎴风┖闂撮┍鍔ㄧ幆澧冦€?
### 缁勩€佽澶囦笌 IOMMU


璁惧鏄换浣?I/O 椹卞姩鐨勪富瑕佺洰鏍囥€傝澶囬€氬父浼氬垱寤轰竴涓敱 I/O 璁块棶銆佷腑鏂拰 DMA 缁勬垚鐨勭紪绋嬫帴鍙ｃ€備笉娣卞叆璁ㄨ鍏朵腑姣忎竴椤癸紝DMA 鍒扮洰鍓嶄负姝㈡槸缁存姢瀹夊叏鐜鏈€鍏抽敭鐨勬柟闈紝鍥犱负鍏佽璁惧瀵圭郴缁熷唴瀛樿繘琛岃鍐欒闂細瀵规暣涓郴缁熺殑瀹屾暣鎬ф瀯鎴愭渶澶х殑椋庨櫓銆?
涓轰簡甯姪缂撹В杩欎竴椋庨櫓锛岃澶氱幇浠?IOMMU 鐜板湪鎶婇殧绂诲睘鎬у紩鍏ヤ簡鏈湪璁稿鎯呭喌涓嬪彧鏄敤浜庤浆鎹紙鍗宠В鍐冲湴鍧€绌洪棿鏈夐檺鐨勮澶囩殑瀵诲潃闂锛夌殑鎺ュ彛涓€傛湁浜嗗畠锛岃澶囩幇鍦ㄥ彲浠ヨ褰兼闅旂銆佷篃鍙互涓庝换鎰忓唴瀛樿闂殧绂伙紝浠庤€屽厑璁歌濡傚皢璁惧瀹夊叏鍦扮洿鎺ョ洿閫氬埌铏氭嫙鏈轰箣绫荤殑浜嬫儏銆?
涓嶈繃锛岃繖绉嶉殧绂诲苟涓嶆€绘槸浠ュ崟涓澶囩殑绮掑害杩涜銆傚嵆浣?IOMMU 鍏峰杩欑鑳藉姏锛岃澶囥€佷簰杩炰互鍙?IOMMU 鎷撴墤鍚勮嚜鐨勫睘鎬ч兘浼氬墛寮辫繖绉嶉殧绂汇€備緥濡傦紝鍗曚釜璁惧鍙兘鏄洿澶х殑澶氬姛鑳藉皝瑁呯殑涓€閮ㄥ垎銆傝櫧鐒?IOMMU 鍙兘鑳藉鍖哄垎灏佽鍐呯殑璁惧锛屼絾灏佽鍙兘骞朵笉瑕佹眰璁惧闂寸殑浜ゆ槗鍒拌揪 IOMMU銆傝繖鏂归潰鐨勪緥瀛愪粠鍔熻兘涔嬮棿瀛樺湪鍚庨棬鐨勫鍔熻兘 PCI 璁惧锛屽埌鍏佽鍦ㄤ笉缁忚繃 IOMMU 鐨勬儏鍐典笅杩涜閲嶅畾鍚戙€佷笉鏀寔 PCI-ACS锛圓ccess Control Services锛岃闂帶鍒舵湇鍔★級鐨勬ˉ锛屼笉涓€鑰岃冻銆傛嫇鎵戜篃鍙兘鍦ㄩ殣钘忚澶囨柟闈㈣捣浣滅敤銆侾CIe 杞?PCI 妗ヤ細鎺╃洊鍏跺悗鐨勮澶囷紝浣夸氦鏄撶湅璧锋潵鍍忔槸浠庢ˉ鏈韩鍙戝嚭鐨勩€傛樉鐒讹紝IOMMU 鐨勮璁′篃鏄竴涓富瑕佸洜绱犮€?
鍥犳锛屽敖绠″湪澶у鏁版儏鍐典笅 IOMMU 鍙兘鍏锋湁璁惧绾х矑搴︼紝浣嗕换浣曠郴缁熼兘瀹规槗鍑虹幇绮掑害闄嶄綆鐨勬儏鍐点€傚洜姝?IOMMU API 鏀寔 IOMMU 缁勶紙group锛夌殑姒傚康銆傜粍鏄竴缁勫彲浠ヤ笌绯荤粺涓墍鏈夊叾浠栬澶囬殧绂荤殑璁惧銆傚洜姝わ紝缁勬槸 VFIO 浣跨敤鐨勬墍鏈夋潈鍗曚綅銆?
铏界劧缁勬槸纭繚瀹夊叏鐢ㄦ埛璁块棶鎵€蹇呴』浣跨敤鐨勬渶灏忕矑搴︼紝浣嗗畠涓嶄竴瀹氭槸棣栭€夌矑搴︺€傚湪浣跨敤椤佃〃鐨?IOMMU 涓紝鍙兘鍙互鍦ㄤ笉鍚岀粍涔嬮棿鍏变韩涓€缁勯〉琛紝浠庤€屽噺灏戝钩鍙扮殑寮€閿€锛堝噺灏?TLB 鎶栧姩銆佸噺灏戦噸澶嶇殑椤佃〃锛夊拰鐢ㄦ埛寮€閿€锛堝彧闇€缂栫▼涓€缁勮浆鎹級銆備负姝わ紝VFIO 浣跨敤浜嗗鍣紙container锛夌被锛屽畠鍙互鎸佹湁涓€涓垨澶氫釜缁勩€傚鍣ㄥ彧闇€鎵撳紑 /dev/vfio/vfio 瀛楃璁惧鍗冲彲鍒涘缓銆?
瀹瑰櫒鏈韩鎻愪緵鐨勫姛鑳藉緢灏戯紝闄や簡灏戞暟鐗堟湰鍜屾墿灞曟煡璇㈡帴鍙ｅ閮借閿佷綇銆傜敤鎴烽渶瑕佸悜瀹瑰櫒涓坊鍔犱竴涓粍鎵嶈兘鑾峰緱涓嬩竴绾х殑鍔熻兘銆備负姝わ紝鐢ㄦ埛棣栧厛闇€瑕佺‘瀹氫笌鎵€闇€璁惧鍏宠仈鐨勭粍銆傝繖鍙互閫氳繃涓嬮潰绀轰緥涓弿杩扮殑 sysfs 閾炬帴鏉ュ畬鎴愩€傞€氳繃灏嗚澶囦粠瀹夸富鏈洪┍鍔ㄨВ缁戝苟灏嗗叾缁戝畾鍒?VFIO 椹卞姩锛屼細涓鸿缁勫嚭鐜颁竴涓柊鐨?VFIO 缁?/dev/vfio/$GROUP锛屽叾涓?$GROUP 鏄璁惧鎵€灞炵殑 IOMMU 缁勭紪鍙枫€傚鏋?IOMMU 缁勫寘鍚涓澶囷紝鍒欐瘡涓澶囬兘闇€瑕佸厛缁戝畾鍒?VFIO 椹卞姩锛屾墠鍏佽瀵?VFIO 缁勮繘琛屾搷浣滐紙濡傛灉娌℃湁 VFIO 椹卞姩鍙敤锛屼粎灏嗚澶囦粠瀹夸富鏈洪┍鍔ㄨВ缁戜篃瓒冲锛涜繖浼氳缁勫彲鐢紝浣嗛偅涓壒瀹氳澶囦笉鍙敤锛夈€俆BD鈥斺€旂敤浜庣鐢ㄩ┍鍔ㄦ帰娴?閿佸畾璁惧鐨勬帴鍙ｃ€?
缁勫噯澶囧ソ鍚庯紝鍙互閫氳繃鎵撳紑 VFIO 缁勫瓧绗﹁澶囷紙/dev/vfio/$GROUP锛夊苟浣跨敤 VFIO_GROUP_SET_CONTAINER ioctl銆佷紶鍏ヤ箣鍓嶆墦寮€鐨勫鍣ㄦ枃浠剁殑鏂囦欢鎻忚堪绗︼紝灏嗗叾娣诲姞鍒板鍣ㄤ腑銆傚鏋滈渶瑕侊紝骞朵笖 IOMMU 椹卞姩鏀寔鍦ㄧ粍涔嬮棿鍏变韩 IOMMU 涓婁笅鏂囷紝鍒欏彲浠ュ皢澶氫釜缁勮缃埌鍚屼竴涓鍣ㄤ腑銆傚鏋滀竴涓粍鏃犳硶璁剧疆鍒板惈鏈夊凡鏈夌粍鐨勫鍣紝鍒欓渶瑕佹敼鐢ㄤ竴涓柊鐨勭┖瀹瑰櫒銆?
缁勶紙鎴栬嫢骞茬粍锛夐檮鍔犲埌瀹瑰櫒鍚庯紝鍏朵綑鐨?ioctl 灏卞彲鐢ㄤ簡锛屼粠鑰岃兘澶熻闂?VFIO IOMMU 鎺ュ彛銆傛澶栵紝鐜板湪鍙互閫氳繃瀵?VFIO 缁勬枃浠舵弿杩扮浣跨敤 ioctl 鏉ヨ幏鍙栫粍鍐呮瘡涓澶囩殑鏂囦欢鎻忚堪绗︺€?
VFIO 璁惧 API 鍖呭惈鐢ㄤ簬鎻忚堪璁惧銆両/O 鍖哄煙鍙婂叾鍦ㄨ澶囨弿杩扮涓婄殑 read/write/mmap 鍋忕Щ閲忕殑 ioctl锛屼互鍙婄敤浜庢弿杩板拰娉ㄥ唽涓柇閫氱煡鐨勬満鍒躲€?
### VFIO 浣跨敤绀轰緥


```

	$ readlink /sys/bus/pci/devices/0000:06:0d.0/iommu_group
	../../../../kernel/iommu_groups/26

```
鍥犳璇ヨ澶囦綅浜?IOMMU 缁?26銆傝璁惧鍦?pci 鎬荤嚎涓婏紝鍥犳鐢ㄦ埛灏嗕娇鐢?vfio-pci 鏉ョ鐞?
```

	# modprobe vfio-pci

```
灏嗚璁惧缁戝畾鍒?vfio-pci 椹卞姩浼氬垱寤?VFIO 缁?
```

	$ lspci -n -s 0000:06:0d.0
	06:0d.0 0401: 1102:0002 (rev 08)
	# echo 0000:06:0d.0 > /sys/bus/pci/devices/0000:06:0d.0/driver/unbind
	# echo 1102 0002 > /sys/bus/pci/drivers/vfio-pci/new_id

```
鐜板湪鎴戜滑闇€瑕佹煡鐪嬬粍閲岃繕鏈夊摢浜涘叾浠栬澶囦互閲婃斁

```

	$ ls -l /sys/bus/pci/devices/0000:06:0d.0/iommu_group/devices
	total 0
	lrwxrwxrwx. 1 root root 0 Apr 23 16:13 0000:00:1e.0 ->
		../../../../devices/pci0000:00/0000:00:1e.0
	lrwxrwxrwx. 1 root root 0 Apr 23 16:13 0000:06:0d.0 ->
		../../../../devices/pci0000:00/0000:00:1e.0/0000:06:0d.0
	lrwxrwxrwx. 1 root root 0 Apr 23 16:13 0000:06:0d.1 ->
		../../../../devices/pci0000:00/0000:00:1e.0/0000:06:0d.1

```
璇ヨ澶囦綅浜庝竴涓?PCIe 杞?PCI 妗?[^4^]_ 涔嬪悗锛屽洜姝ゆ垜浠繕闇€瑕佹妸璁惧 0000:06:0d.1 鎸変笌涓婅堪鐩稿悓鐨勬楠ゅ姞鍏ョ粍銆傝澶?0000:00:1e.0 鏄竴涓綋鍓嶆病鏈夊涓绘満椹卞姩鐨勬ˉ锛屽洜姝や笉瑕佹眰灏嗚璁惧缁戝畾鍒?vfio-pci 椹卞姩锛坴fio-pci 鐩墠涓嶆敮鎸?PCI 妗ワ級銆?
濡傛灉甯屾湜杩涜闈炵壒鏉冩搷浣滐紝鏈€鍚庝竴姝ユ槸璧嬩簣鐢ㄦ埛瀵硅缁勭殑璁块棶鏉冮檺锛堟敞鎰?/dev/vfio/vfio 鏈韩涓嶆彁渚涗换浣曡兘鍔涳紝鍥犳棰勬湡灏嗗叾璁剧疆涓?
```

	# chown user:user /dev/vfio/26

```
鐢ㄦ埛鐜板湪瀵规瀹瑰櫒涓殑鎵€鏈夎澶囧強鍏?iommu 鎷ユ湁瀹屽叏璁块棶鏉?
```

	int container, group, device, i;
	struct vfio_group_status group_status =
					{ .argsz = sizeof(group_status) };
	struct vfio_iommu_type1_info iommu_info = { .argsz = sizeof(iommu_info) };
	struct vfio_iommu_type1_dma_map dma_map = { .argsz = sizeof(dma_map) };
	struct vfio_device_info device_info = { .argsz = sizeof(device_info) };

	/* Create a new container */
	container = open("/dev/vfio/vfio", O_RDWR);

	if (ioctl(container, VFIO_GET_API_VERSION) != VFIO_API_VERSION)
		/* Unknown API version */

	if (!ioctl(container, VFIO_CHECK_EXTENSION, VFIO_TYPE1_IOMMU))
		/* Doesn't support the IOMMU driver we want. */

	/* Open the group */
	group = open("/dev/vfio/26", O_RDWR);

	/* Test the group is viable and available */
	ioctl(group, VFIO_GROUP_GET_STATUS, &group_status);

	if (!(group_status.flags & VFIO_GROUP_FLAGS_VIABLE))
		/* Group is not viable (ie, not all devices bound for vfio) */

	/* Add the group to the container */
	ioctl(group, VFIO_GROUP_SET_CONTAINER, &container);

	/* Enable the IOMMU model we want */
	ioctl(container, VFIO_SET_IOMMU, VFIO_TYPE1_IOMMU);

	/* Get addition IOMMU info */
	ioctl(container, VFIO_IOMMU_GET_INFO, &iommu_info);

	/* Allocate some space and setup a DMA mapping */
	dma_map.vaddr = mmap(0, 1024 * 1024, PROT_READ | PROT_WRITE,
			     MAP_PRIVATE | MAP_ANONYMOUS, 0, 0);
	dma_map.size = 1024 * 1024;
	dma_map.iova = 0; /* 1MB starting at 0x0 from device view */
	dma_map.flags = VFIO_DMA_MAP_FLAG_READ | VFIO_DMA_MAP_FLAG_WRITE;

	ioctl(container, VFIO_IOMMU_MAP_DMA, &dma_map);

	/* Get a file descriptor for the device */
	device = ioctl(group, VFIO_GROUP_GET_DEVICE_FD, "0000:06:0d.0");

	/* Test and setup the device */
	ioctl(device, VFIO_DEVICE_GET_INFO, &device_info);

	for (i = 0; i < device_info.num_regions; i++) {
		struct vfio_region_info reg = { .argsz = sizeof(reg) };

		reg.index = i;

		ioctl(device, VFIO_DEVICE_GET_REGION_INFO, &reg);

		/* Setup mappings... read/write offsets, mmaps
		 * For PCI devices, config space is a region */
	}

	for (i = 0; i < device_info.num_irqs; i++) {
		struct vfio_irq_info irq = { .argsz = sizeof(irq) };

		irq.index = i;

		ioctl(device, VFIO_DEVICE_GET_IRQ_INFO, &irq);

		/* Setup IRQs... eventfds, VFIO_DEVICE_SET_IRQS */
	}

	/* Gratuitous device reset and go... */
	ioctl(device, VFIO_DEVICE_RESET);

```
### IOMMUFD 涓?vfio_iommu_type1


IOMMUFD 鏄敤浜庝粠鐢ㄦ埛绌洪棿绠＄悊 I/O 椤佃〃鐨勬柊鐢ㄦ埛 API銆傚畠鏃ㄥ湪鎴愪负浜や粯楂樼骇鐢ㄦ埛绌洪棿 DMA 鐗规€э紙宓屽杞崲 [^5^]_銆丳ASID [^6^]_ 绛夛級鐨勯棬鎴凤紝鍚屾椂涓虹幇鏈夌殑 VFIO_TYPE1v2_IOMMU 鐢ㄤ緥鎻愪緵鍚戝悗鍏煎鎺ュ彛銆傛渶缁堬紝vfio_iommu_type1 椹卞姩浠ュ強浼犵粺鐨?vfio 瀹瑰櫒涓庣粍妯″瀷閮借鍒掕寮冪敤銆?
IOMMUFD 鍚戝悗鍏煎鎺ュ彛鍙互閫氳繃涓ょ鏂瑰紡鍚敤銆傜涓€绉嶆柟寮忥紝鍐呮牳鍙互鐢?CONFIG_IOMMUFD_VFIO_CONTAINER 閰嶇疆锛屽湪杩欑鎯呭喌涓?IOMMUFD 瀛愮郴缁熼€忔槑鍦颁负 VFIO 瀹瑰櫒鍜?IOMMU 鍚庣鎺ュ彛鎻愪緵瀹屾暣鐨勫熀纭€璁炬柦銆傚鏋?VFIO 瀹瑰櫒鎺ュ彛锛堝嵆 /dev/vfio/vfio锛夎绠€鍗曞湴绗﹀彿閾炬帴鍒?/dev/iommu锛屼篃鍙互璁块棶鍏煎妯″紡銆傝娉ㄦ剰锛屽湪鎾板啓鏈枃鏃讹紝鐩稿浜?VFIO_TYPE1v2_IOMMU锛堜緥濡?DMA 鏄犲皠 MMIO锛夛紝鍏煎妯″紡鐨勫姛鑳藉皻涓嶅畬鏁达紝骞朵笖涓嶆墦绠椾负 VFIO_SPAPR_TCE_IOMMU 鎺ュ彛鎻愪緵鍏煎鎬с€傚洜姝わ紝鐩墠涓€鑸笉寤鸿浠庡師鐢熺殑 VFIO 瀹炵幇鍒囨崲鍒?IOMMUFD 鍏煎鎺ュ彛銆?
浠庨暱杩滄潵鐪嬶紝VFIO 鐢ㄦ埛搴斿綋杩佺Щ鍒伴€氳繃涓嬮潰鎻忚堪鐨?cdev 鎺ュ彛杩涜璁惧璁块棶锛屼互鍙婇€氳繃 IOMMUFD 鎻愪緵鐨勬帴鍙ｈ繘琛屽師鐢熻闂€?
### VFIO 璁惧 cdev


浼犵粺涓婏紝鐢ㄦ埛閫氳繃 VFIO 缁勪腑鐨?VFIO_GROUP_GET_DEVICE_FD 鑾峰彇璁惧 fd銆?
鍚敤 CONFIG_VFIO_DEVICE_CDEV=y 鍚庯紝鐢ㄦ埛鐜板湪鍙互閫氳繃鐩存帴鎵撳紑瀛楃璁惧 /dev/vfio/devices/vfioX 鏉ヨ幏鍙栬澶?fd锛屽叾涓€淴鈥濇槸 VFIO 涓哄凡娉ㄥ唽璁惧鍞竴鍒嗛厤鐨勬暟瀛椼€俢dev 鎺ュ彛涓嶆敮鎸?noiommu 璁惧锛屽洜姝ゅ鏋滈渶瑕?noiommu锛岀敤鎴峰簲浣跨敤浼犵粺鐨勭粍鎺ュ彛銆?
cdev 浠呬笌 IOMMUFD 閰嶅悎宸ヤ綔銆俈FIO 椹卞姩鍜屽簲鐢ㄧ▼搴忛兘蹇呴』閫傚簲鏂扮殑 cdev 瀹夊叏妯″瀷锛岃妯″瀷瑕佹眰鍦ㄥ疄闄呭紑濮嬩娇鐢ㄨ澶囦箣鍓嶄娇鐢?VFIO_DEVICE_BIND_IOMMUFD 鏉ュ０鏄?DMA 鎵€鏈夋潈銆備竴鏃?BIND 鎴愬姛锛孷FIO 璁惧灏辫兘琚敤鎴峰畬鍏ㄨ闂€?
VFIO 璁惧 cdev 涓嶄緷璧?VFIO 缁?瀹瑰櫒/IOMMU 椹卞姩銆傚洜姝わ紝鍦ㄦ病鏈変紶缁?VFIO 搴旂敤鐨勭幆澧冧腑锛岄偅浜涙ā鍧楀彲浠ヨ瀹屽叏缂栬瘧鎺夈€?
杩勪粖涓烘锛孲PAPR 灏氫笉鏀寔 IOMMUFD銆傚洜姝ゅ畠涔熶笉鑳芥敮鎸佽澶?cdev銆?
vfio 璁惧 cdev 璁块棶浠嶇劧鍙?IOMMU 缁勮涔夌害鏉燂紝鍗充竴涓粍鍙兘鏈変竴涓?DMA 鎵€鏈夎€呫€傚睘浜庡悓涓€缁勭殑璁惧涓嶈兘缁戝畾鍒板涓?iommufd_ctx锛屼篃涓嶈兘鍦ㄥ師鐢熷唴鏍镐笌 vfio 鎬荤嚎椹卞姩鎴栨敮鎸?driver_managed_dma 鏍囧織鐨勫叾浠栭┍鍔ㄤ箣闂村叡浜€傝繚鍙嶆鎵€鏈夋潈瑕佹眰浼氬湪 VFIO_DEVICE_BIND_IOMMUFD ioctl 澶勫け璐ワ紝璇?ioctl 鏄畬鏁磋澶囪闂殑闂ㄦ銆?
### 璁惧 cdev 绀轰緥


```

	$ ls /sys/bus/pci/devices/0000:6a:01.0/vfio-dev/
	vfio0

```
鍥犳璇ヨ澶囪〃绀轰负 vfio0銆傜敤鎴峰彲浠ラ獙璇?
```

	$ ls -l /dev/vfio/devices/vfio0
	crw------- 1 root root 511, 0 Feb 16 01:22 /dev/vfio/devices/vfio0
	$ cat /sys/bus/pci/devices/0000:6a:01.0/vfio-dev/vfio0/dev
	511:0
	$ ls -l /dev/char/511\:0
	lrwxrwxrwx 1 root root 21 Feb 16 01:22 /dev/char/511:0 -> ../vfio/devices/vfio0

```
濡傛灉甯屾湜闈炵壒鏉冭闂紝鍒欒祴浜堢敤鎴峰璇ヨ澶囩殑璁块棶鏉冮檺

```

	$ chown user:user /dev/vfio/devices/vfio0

```
```

	cdev_fd = open("/dev/vfio/devices/vfio0", O_RDWR);

```
鎵撳紑鐨?cdev_fd 涓嶄細璧嬩簣鐢ㄦ埛璁块棶璁惧鐨勪换浣曟潈闄愶紝鍙兘灏?cdev_fd 缁戝畾鍒颁竴涓?iommufd銆傚湪閭ｄ箣鍚庯紝璁惧鎵嶈瀹屽叏璁块棶锛屽寘鎷皢鍏堕檮鍔犲埌涓€涓?
```

	struct vfio_device_bind_iommufd bind = {
		.argsz = sizeof(bind),
		.flags = 0,
	};
	struct iommu_ioas_alloc alloc_data  = {
		.size = sizeof(alloc_data),
		.flags = 0,
	};
	struct vfio_device_attach_iommufd_pt attach_data = {
		.argsz = sizeof(attach_data),
		.flags = 0,
	};
	struct iommu_ioas_map map = {
		.size = sizeof(map),
		.flags = IOMMU_IOAS_MAP_READABLE |
			 IOMMU_IOAS_MAP_WRITEABLE |
			 IOMMU_IOAS_MAP_FIXED_IOVA,
		.__reserved = 0,
	};

	iommufd = open("/dev/iommu", O_RDWR);

	bind.iommufd = iommufd;
	ioctl(cdev_fd, VFIO_DEVICE_BIND_IOMMUFD, &bind);

	ioctl(iommufd, IOMMU_IOAS_ALLOC, &alloc_data);
	attach_data.pt_id = alloc_data.out_ioas_id;
	ioctl(cdev_fd, VFIO_DEVICE_ATTACH_IOMMUFD_PT, &attach_data);

	/* Allocate some space and setup a DMA mapping */
	map.user_va = (int64_t)mmap(0, 1024 * 1024, PROT_READ | PROT_WRITE,
				    MAP_PRIVATE | MAP_ANONYMOUS, 0, 0);
	map.iova = 0; /* 1MB starting at 0x0 from device view */
	map.length = 1024 * 1024;
	map.ioas_id = alloc_data.out_ioas_id;

	ioctl(iommufd, IOMMU_IOAS_MAP, &map);

	/* Other device operations as stated in "VFIO Usage Example" */

```
### VFIO 鐢ㄦ埛 API


瀹屾暣鐨?API 鏂囨。璇峰弬瑙?include/uapi/linux/vfio.h銆?
### VFIO 鎬荤嚎椹卞姩 API


VFIO 鎬荤嚎椹卞姩锛堝 vfio-pci锛夊彧浣跨敤灏戞暟鍑犱釜杩涘叆 VFIO 鏍稿績鐨勬帴鍙ｃ€傚綋璁惧琚粦瀹氬埌椹卞姩浠ュ強浠庨┍鍔ㄨВ缁戞椂锛屽湪璁惧琚粦瀹氬埌浠ュ強

```

	int vfio_register_group_dev(struct vfio_device *device);
	int vfio_register_emulated_iommu_dev(struct vfio_device *device);
	void vfio_unregister_group_dev(struct vfio_device *device);

```
椹卞姩搴斿綋鎶?vfio_device 宓屽叆鍒拌嚜宸辩殑缁撴瀯浣撲腑锛屽苟浣跨敤 vfio_alloc_device() 鏉ュ垎閰嶈缁撴瀯浣擄紝杩樺彲浠ユ敞鍐?@init/@release 鍥炶皟鏉ョ鐞嗗寘瑁硅

```

	vfio_alloc_device(dev_struct, member, dev, ops);
	void vfio_put_device(struct vfio_device *device);

```
vfio_register_group_dev() 閫氱煡鏍稿績寮€濮嬭窡韪寚瀹?dev 鐨?iommu_group锛屽苟灏嗚 dev 娉ㄥ唽涓虹敱 VFIO 鎬荤嚎椹卞姩鎷ユ湁銆備竴鏃?vfio_register_group_dev() 杩斿洖锛岀敤鎴风┖闂村氨鍙互寮€濮嬭闂椹卞姩锛屽洜姝ら┍鍔ㄥ簲褰撶‘淇濆湪璋冪敤瀹冧箣鍓嶅凡瀹屽叏鍑嗗灏辩华銆傞┍鍔ㄦ彁渚涗竴涓敤浜庡洖璋冪殑 ops 缁撴瀯浣?
```

	struct vfio_device_ops {
		char	*name;
		int	(*init)(struct vfio_device *vdev);
		void	(*release)(struct vfio_device *vdev);
		int	(*bind_iommufd)(struct vfio_device *vdev,
					struct iommufd_ctx *ictx, u32 *out_device_id);
		void	(*unbind_iommufd)(struct vfio_device *vdev);
		int	(*attach_ioas)(struct vfio_device *vdev, u32 *pt_id);
		void	(*detach_ioas)(struct vfio_device *vdev);
		int	(*open_device)(struct vfio_device *vdev);
		void	(*close_device)(struct vfio_device *vdev);
		ssize_t	(*read)(struct vfio_device *vdev, char __user *buf,
				size_t count, loff_t *ppos);
		ssize_t	(*write)(struct vfio_device *vdev, const char __user *buf,
			 size_t count, loff_t *size);
		long	(*ioctl)(struct vfio_device *vdev, unsigned int cmd,
				 unsigned long arg);
		int	(*mmap)(struct vfio_device *vdev, struct vm_area_struct *vma);
		void	(*request)(struct vfio_device *vdev, unsigned int count);
		int	(*match)(struct vfio_device *vdev, char *buf);
		void	(*dma_unmap)(struct vfio_device *vdev, u64 iova, u64 length);
		int	(*device_feature)(struct vfio_device *device, u32 flags,
					  void __user *arg, size_t argsz);
	};

```
姣忎釜鍑芥暟閮戒細浼犲叆鏈€鍒濆湪涓婇潰鐨?vfio_register_group_dev() 鎴?vfio_register_emulated_iommu_dev() 璋冪敤涓敞鍐岀殑 vdev銆傝繖璁╂€荤嚎椹卞姩鍙互浣跨敤 container_of() 鑾峰彇鍏剁鏈夋暟鎹€?
```

	- The init/release callbacks are issued when vfio_device is initialized
	  and released.

	- The open/close device callbacks are issued when the first
	  instance of a file descriptor for the device is created (eg.
	  via VFIO_GROUP_GET_DEVICE_FD) for a user session.

	- The ioctl callback provides a direct pass through for some VFIO_DEVICE_*
	  ioctls.

	- The [un]bind_iommufd callbacks are issued when the device is bound to
	  and unbound from iommufd.

	- The [de]attach_ioas callback is issued when the device is attached to
	  and detached from an IOAS managed by the bound iommufd. However, the
	  attached IOAS can also be automatically detached when the device is
	  unbound from iommufd.

	- The read/write/mmap callbacks implement the device region access defined
	  by the device's own VFIO_DEVICE_GET_REGION_INFO ioctl.

	- The request callback is issued when device is going to be unregistered,
	  such as when trying to unbind the device from the vfio bus driver.

	- The dma_unmap callback is issued when a range of iovas are unmapped
	  in the container or IOAS attached by the device. Drivers which make
	  use of the vfio page pinning interface must implement this callback in
	  order to unpin pages within the dma_unmap range. Drivers must tolerate
	  this callback even before calls to open_device().

```
### PPC64 sPAPR 瀹炵幇璇存槑


鏈疄鐜版湁涓€浜涚壒瀹氫箣澶勶細

1) 鍦ㄨ緝鏃х殑绯荤粺锛堝甫 P5IOC2/IODA1 鐨?POWER7锛変笂锛屾瘡涓鍣ㄥ彧鏀寔涓€涓?IOMMU 缁勶紝鍥犱负 IOMMU 琛ㄦ槸鍦ㄥ惎鍔ㄦ椂鍒嗛厤鐨勶紝姣忎釜 IOMMU 缁勶紙鍗冲彲鍒嗗尯绔偣 PE锛変竴寮犺〃锛圥E 閫氬父鏄竴涓?PCI 鍩燂紝浣嗕笉涓€瀹氾級銆?
   杈冩柊鐨勭郴缁燂紙甯?IODA2 鐨?POWER8锛夋敼杩涗簡纭欢璁捐锛屽彲浠ユ秷闄よ繖涓€闄愬埗锛屼粠鑰屾瘡涓?VFIO 瀹瑰櫒鍙互鏈夊涓?IOMMU 缁勩€?
2) 纭欢鏀寔鎵€璋撶殑 DMA 绐楀彛鈥斺€斿嵆鍏佽杩涜 DMA 浼犺緭鐨?PCI 鍦板潃鑼冨洿锛屼换浣曡闂獥鍙ｅ鍦板潃绌洪棿鐨勫皾璇曢兘浼氬鑷存暣涓?PE 琚殧绂汇€?
3) PPC64 瀹㈡埛鏈烘槸鍗婅櫄鎷熷寲鐨勶紝浣嗕笉鏄畬鍏ㄦā鎷熺殑銆傛湁涓€涓敤浜庝负 DMA 鏄犲皠/鍙栨秷鏄犲皠椤电殑 API锛岄€氬父姣忔璋冪敤鏄犲皠 1..32 椤碉紝鐩墠鏃犳硶鍑忓皯璋冪敤娆℃暟銆備负浜嗚浜嬫儏鏇村揩锛屾槧灏?鍙栨秷鏄犲皠鐨勫鐞嗗凡鍦ㄥ疄妯″紡锛坮eal mode锛変腑瀹炵幇锛屾彁渚涗簡鍑鸿壊鐨勬€ц兘锛屼絾涔熷瓨鍦ㄨ濡傛棤娉曞疄鏃惰繘琛岄攣瀹氶〉璁拌处涔嬬被鐨勯檺鍒躲€?
4) 鏍规嵁 sPAPR 瑙勮寖锛屽彲鍒嗗尯绔偣锛圥E锛夋槸涓€涓?I/O 瀛愭爲锛屽湪鍒嗗尯鍜岄敊璇仮澶嶆椂鍙褰撲綔涓€涓崟鍏冨鐞嗐€侾E 鍙互鏄崟鍔熻兘鎴栧鍔熻兘 IOA锛圛O 閫傞厤鍣級銆佸鍔熻兘 IOA 鐨勪竴涓姛鑳斤紝鎴栧涓?IOA锛堝彲鑳藉寘鍚涓?IOA 涔嬩笂鐨勪氦鎹㈡満鍜屾ˉ缁撴瀯锛夈€侾PC64 瀹㈡埛鏈洪€氳繃 EEH RTAS 鏈嶅姟妫€娴嬪苟浠?PCI 閿欒涓仮澶嶏紝璇ユ湇鍔″熀浜庨澶栫殑 ioctl 鍛戒护杩愪綔銆?
   鍥犳鏂板浜?4 涓澶栫殑 ioctl锛?
	VFIO_IOMMU_SPAPR_TCE_GET_INFO
		returns the size and the start of the DMA window on the PCI bus.

	VFIO_IOMMU_ENABLE
		enables the container. The locked pages accounting
		is done at this point. This lets user first to know what
		the DMA window is and adjust rlimit before doing any real job.

	VFIO_IOMMU_DISABLE
		disables the container.

	VFIO_EEH_PE_OP
		provides an API for EEH setup, error detection and recovery.

```

	struct vfio_eeh_pe_op pe_op = { .argsz = sizeof(pe_op), .flags = 0 };

	.....
	/* Add the group to the container */
	ioctl(group, VFIO_GROUP_SET_CONTAINER, &container);

	/* Enable the IOMMU model we want */
	ioctl(container, VFIO_SET_IOMMU, VFIO_SPAPR_TCE_IOMMU)

	/* Get addition sPAPR IOMMU info */
	vfio_iommu_spapr_tce_info spapr_iommu_info;
	ioctl(container, VFIO_IOMMU_SPAPR_TCE_GET_INFO, &spapr_iommu_info);

	if (ioctl(container, VFIO_IOMMU_ENABLE))
		/* Cannot enable container, may be low rlimit */

	/* Allocate some space and setup a DMA mapping */
	dma_map.vaddr = mmap(0, 1024 * 1024, PROT_READ | PROT_WRITE,
			     MAP_PRIVATE | MAP_ANONYMOUS, 0, 0);

	dma_map.size = 1024 * 1024;
	dma_map.iova = 0; /* 1MB starting at 0x0 from device view */
	dma_map.flags = VFIO_DMA_MAP_FLAG_READ | VFIO_DMA_MAP_FLAG_WRITE;

	/* Check here is .iova/.size are within DMA window from spapr_iommu_info */
	ioctl(container, VFIO_IOMMU_MAP_DMA, &dma_map);

	/* Get a file descriptor for the device */
	device = ioctl(group, VFIO_GROUP_GET_DEVICE_FD, "0000:06:0d.0");

	....

	/* Gratuitous device reset and go... */
	ioctl(device, VFIO_DEVICE_RESET);

	/* Make sure EEH is supported */
	ioctl(container, VFIO_CHECK_EXTENSION, VFIO_EEH);

	/* Enable the EEH functionality on the device */
	pe_op.op = VFIO_EEH_PE_ENABLE;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/* You're suggested to create additional data struct to represent
	 * PE, and put child devices belonging to same IOMMU group to the
	 * PE instance for later reference.
	 */

	/* Check the PE's state and make sure it's in functional state */
	pe_op.op = VFIO_EEH_PE_GET_STATE;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/* Save device state using pci_save_state().
	 * EEH should be enabled on the specified device.
	 */

	....

	/* Inject EEH error, which is expected to be caused by 32-bits
	 * config load.
	 */
	pe_op.op = VFIO_EEH_PE_INJECT_ERR;
	pe_op.err.type = EEH_ERR_TYPE_32;
	pe_op.err.func = EEH_ERR_FUNC_LD_CFG_ADDR;
	pe_op.err.addr = 0ul;
	pe_op.err.mask = 0ul;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	....

	/* When 0xFF's returned from reading PCI config space or IO BARs
	 * of the PCI device. Check the PE's state to see if that has been
	 * frozen.
	 */
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/* Waiting for pending PCI transactions to be completed and don't
	 * produce any more PCI traffic from/to the affected PE until
	 * recovery is finished.
	 */

	/* Enable IO for the affected PE and collect logs. Usually, the
	 * standard part of PCI config space, AER registers are dumped
	 * as logs for further analysis.
	 */
	pe_op.op = VFIO_EEH_PE_UNFREEZE_IO;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/*
	 * Issue PE reset: hot or fundamental reset. Usually, hot reset
	 * is enough. However, the firmware of some PCI adapters would
	 * require fundamental reset.
	 */
	pe_op.op = VFIO_EEH_PE_RESET_HOT;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);
	pe_op.op = VFIO_EEH_PE_RESET_DEACTIVATE;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/* Configure the PCI bridges for the affected PE */
	pe_op.op = VFIO_EEH_PE_CONFIGURE;
	ioctl(container, VFIO_EEH_PE_OP, &pe_op);

	/* Restored state we saved at initialization time. pci_restore_state()
	 * is good enough as an example.
	 */

	/* Hopefully, error is recovered successfully. Now, you can resume to
	 * start PCI traffic to/from the affected PE.
	 */

	....

```
5) SPAPR TCE IOMMU 鏈?v2 鐗堟湰銆傚畠寮冪敤浜?VFIO_IOMMU_ENABLE/VFIO_IOMMU_DISABLE锛屽苟瀹炵幇浜?2 涓柊鐨?ioctl锛歏FIO_IOMMU_SPAPR_REGISTER_MEMORY 鍜?VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY锛堝湪 v1 IOMMU 涓笉鍙楁敮鎸侊級銆?
   PPC64 鍗婅櫄鎷熷寲瀹㈡埛鏈轰細浜х敓澶ч噺鐨勬槧灏?鍙栨秷鏄犲皠璇锋眰锛岃繖浜涜姹傜殑澶勭悊鍖呭惈閿佸畾/瑙ｉ攣椤碉紝骞舵洿鏂?mm::locked_vm 璁℃暟鍣ㄤ互纭繚涓嶈秴杩?rlimit銆倂2 IOMMU 灏嗚璐︿笌閿佸畾鎷嗗垎涓虹嫭绔嬬殑鎿嶄綔锛?
   - VFIO_IOMMU_SPAPR_REGISTER_MEMORY/VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY ioctl 鎺ユ敹涓€涓敤鎴风┖闂村湴鍧€浠ュ強瑕佽閿佸畾鍧楃殑澶у皬銆備笉鏀寔浜屽垎锛坆isecting锛夛紝骞朵笖鏈熸湜 VFIO_IOMMU_UNREGISTER_MEMORY 浣跨敤娉ㄥ唽璇ュ唴瀛樺潡鏃舵墍鐢ㄧ殑纭垏鍦板潃鍜屽ぇ灏忔潵璋冪敤銆備笉鏈熸湜鐢ㄦ埛绌洪棿棰戠箒璋冪敤杩欎簺銆傝繖浜涜寖鍥村瓨鍌ㄥ湪 VFIO 瀹瑰櫒鐨勯摼琛ㄤ腑銆?
   - VFIO_IOMMU_MAP_DMA/VFIO_IOMMU_UNMAP_DMA ioctl 鍙洿鏂板疄闄呯殑 IOMMU 琛紝涓嶈繘琛岄攣瀹氾紱鐩稿弽锛屽畠浠鏌ョ敤鎴风┖闂村湴鍧€鏄惁鏉ヨ嚜棰勫厛娉ㄥ唽鐨勮寖鍥淬€?
   杩欑鍒嗙鏈夊姪浜庝紭鍖栧鎴锋満鐨?DMA銆?
6) sPAPR 瑙勮寖鍏佽瀹㈡埛鏈哄湪 PCI 鎬荤嚎涓婃嫢鏈夐澶栫殑 DMA 绐楀彛锛堝彲鍙橀〉澶у皬锛夈€備负姝ゆ柊澧炰簡涓や釜 ioctl锛歏FIO_IOMMU_SPAPR_TCE_CREATE 鍜?VFIO_IOMMU_SPAPR_TCE_REMOVE銆傚钩鍙板繀椤绘敮鎸佽鍔熻兘锛屽惁鍒欎細鍚戠敤鎴风┖闂磋繑鍥為敊璇€傜幇鏈夌‖浠舵渶澶氭敮鎸?2 涓?DMA 绐楀彛锛屼竴涓槸 2GB 闀裤€佷娇鐢?4K 椤碉紝绉颁负鈥滈粯璁?32 浣嶇獥鍙ｏ紙default 32bit window锛夆€濓紱鍙︿竴涓彲浠ュぇ鍒版暣鐗?RAM銆佷娇鐢ㄤ笉鍚岀殑椤靛ぇ灏忥紝瀹冩槸鍙€夌殑鈥斺€斿鏋滃鎴锋満椹卞姩鏀寔 64 浣?DMA锛屽鎴锋満鍦ㄨ繍琛屾椂鍒涘缓瀹冧滑銆?
   VFIO_IOMMU_SPAPR_TCE_CREATE 鎺ユ敹涓€涓〉鍋忕Щ锛坧age shift锛夈€丏MA 绐楀彛澶у皬浠ュ強 TCE 琛ㄧ骇鏁帮紙濡傛灉 TCE 琛ㄥ皢瓒冲澶с€佽€屽唴鏍稿彲鑳芥棤娉曞垎閰嶈冻澶熺殑鐗╃悊杩炵画鍐呭瓨锛夈€傚畠鍦ㄥ彲鐢ㄧ殑妲戒腑鍒涘缓涓€涓柊绐楀彛锛屽苟杩斿洖鏂扮獥鍙ｅ紑濮嬬殑 bus 鍦板潃銆傚彈纭欢闄愬埗锛岀敤鎴风┖闂存棤娉曢€夋嫨 DMA 绐楀彛鐨勪綅缃€?
   VFIO_IOMMU_SPAPR_TCE_REMOVE 鎺ユ敹绐楀彛鐨勬€荤嚎璧峰鍦板潃骞跺皢鍏剁Щ闄ゃ€?
-------------------------------------------------------------------------------

   鏈€鍒濈敱 Tom Lyon 鍦?Cisco 鏃跺疄鐜般€備粠閭ｄ互鍚庢垜浠凡缁忚秴鍑轰簡杩欎釜缂╁啓鐨勬湰鎰忥紝浣嗗畠寰堜笂鍙ｃ€?
   澶氬姛鑳借澶囨湁鍙兘鍦ㄥ姛鑳戒箣闂村瓨鍦ㄥ悗闂紝鐢氳嚦鍗曞姛鑳借澶囦篃鏈夊彲鑳介€氳繃 MMIO 瀵勫瓨鍣ㄨ幏寰楀 PCI 閰嶇疆绌洪棿涔嬬被鐨勬浛浠ｈ闂€備负浜嗛槻姝㈠墠鑰咃紝鎴戜滑鍙互鍦?IOMMU 椹卞姩涓姞鍏ラ澶栫殑棰勯槻鎺柦锛屽皢澶氬姛鑳?PCI 璁惧鍒嗙粍鍦ㄤ竴璧凤紙iommu=group_mf锛夈€傚悗鑰呮垜浠棤娉曢槻姝紝浣?IOMMU 浠嶅簲鎻愪緵闅旂銆傚浜?PCI锛孲R-IOV 铏氭嫙鍔熻兘锛圴irtual Functions锛夋槸鈥滆涓鸿壇濂解€濈殑鏈€浣虫寚鏍囷紝鍥犱负瀹冧滑鏄负铏氭嫙鍖栦娇鐢ㄦā鍨嬭璁＄殑銆?
   瓒呭嚭 VFIO 鑼冨洿鐨勮澶囧垎閰嶏紙assignment锛夈€傞璁℃湭鏉ョ殑 IOMMU 鎶€鏈細鍑忓皯鍏朵腑閮ㄥ垎锛堜絾涔熻涓嶆槸鍏ㄩ儴锛夌殑鍙栬垗銆?
```

	-[0000:00]-+-1e.0-[06]--+-0d.0
				\-0d.1

	00:1e.0 PCI bridge: Intel Corporation 82801 PCI Bridge (rev 90)

```
   鍦板潃杞崲銆傝繖鎻愰珮浜?IOMMU 铏氭嫙鍖栦腑鐨勫湴鍧€杞崲鏁堢巼銆?
   Express銆傚畠鏄叡浜櫄鎷熷鍧€锛圫hared Virtual Addressing锛孲VA锛夊拰鍙墿灞?I/O 铏氭嫙鍖栵紙Scalable I/O Virtualization锛孲calable IOV锛夌殑鍏堝喅鏉′欢銆?