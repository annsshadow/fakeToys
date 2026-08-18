## 閫氳繃 initrd 鍗囩骇 ACPI 琛?

## 杩欐槸浠€涔?
濡傛灉 ACPI_TABLE_UPGRADE 缂栬瘧閫夐」涓虹湡锛屽氨鍙互閫氳繃鐢ㄧ粡杩囨敼閫犵殑銆佷慨鏀硅繃鐨勩€佹洿鏂扮殑
鐗堟湰鏇挎崲 BIOS 鎻愪緵鐨?ACPI 琛紝鏉ュ崌绾х敱 ACPI 琛ㄥ畾涔夌殑 ACPI 鎵ц鐜锛屾垨鑰呭畨瑁呭叏鏂扮殑
ACPI 琛ㄣ€?
褰撳湪鍐呮牳涓?initrd 鍚堝苟鍒板崟涓€闀滃儚涓瀯寤烘椂锛屾鍔熻兘杩橀渶灏?ACPI_TABLE_OVERRIDE_VIA_BUILTIN_INITRD 閫夐」涔熻涓虹湡鎵嶈兘宸ヤ綔銆?
鍏充簬鍙崌绾?瀹夎鐨勫叏閮?ACPI 琛紝璇锋煡鐪?drivers/acpi/tables.c 涓?`char *table_sigs[MAX_ACPI_SIGNATURE];` 鐨勫畾涔夈€?
iasl锛圛ntel 鐨?ACPI 缂栬瘧鍣ㄤ笌鍙嶆眹缂栧櫒锛夎璇嗙殑鎵€鏈?ACPI 琛ㄩ兘搴斿彲琚鐩栵紝闄や簡锛?
  - ACPI_SIG_RSDP锛堢鍚嶄负 6 瀛楄妭锛?  - ACPI_SIG_FACS锛堟病鏈夋櫘閫氱殑 ACPI 琛ㄥご锛?
杩欎袱鑰呭皢鏉ヤ篃鍙兘琚疄鐜般€?

## 鐢ㄩ€?
濡傛灉浣犲彂鐜颁竴涓弗閲嶅埌 Linux 鍐呮牳鏃犳硶鎺ュ彈鍙橀€氭柟妗堢殑 bug锛岃鍚戜綘鐨勫钩鍙?BIOS 鍘傚晢鎶曡瘔銆?鑰屾鍔熻兘鍏佽浣犲湪骞冲彴/BIOS 鍘傚晢鍙戝竷鍗囩骇鍚庣殑 BIOS 浜岃繘鍒朵箣鍓嶏紝鍏堝崌绾ф湁 bug 鐨勮〃銆?
骞冲彴/BIOS 鍘傚晢鍙互鍒╃敤姝ゅ姛鑳藉湪涓嶄慨鏀瑰簳灞傚钩鍙板浐浠剁殑鎯呭喌涓嬶紝鎻愪緵涓€涓笌 Linux 鍏煎鐨?鐜銆?
姝ゅ姛鑳借繕鎻愪緵浜嗕竴涓己澶х殑鐗规€э紝鍙互閫氳繃淇敼骞冲彴鎻愪緵鐨勬棫 ACPI 琛ㄦ垨鎻掑叆鏂扮殑 ACPI 琛紝
鏉ヨ交鏉捐皟璇曞拰娴嬭瘯 ACPI BIOS 琛ㄤ笌 Linux 鍐呮牳鐨勫吋瀹规€с€?
瀹冨彲浠ュ湪浠讳綍鍐呮牳涓惎鐢紝鍥犱负瀵规湭缁忚繃鏀归€犵殑 initrd 鏉ヨ娌℃湁浠讳綍鍔熻兘鍙樺寲銆?

## 宸ヤ綔鍘熺悊

```

  # 鎻愬彇鏈満鐨?ACPI 琛細
  cd /tmp
  acpidump >acpidump
  acpixtract -a acpidump
  # 鍙嶆眹缂栥€佷慨鏀瑰苟閲嶆柊缂栬瘧锛?  iasl -d *.dat
  # 渚嬪灏嗗涓嬭鍙ュ姞鍏?DSDT锛圥CI 璺敱琛級鍑芥暟鐨?_PRT 涓細
  Store("HELLO WORLD", debug)
  # 骞跺澶?OEM Revision銆備緥濡傦紝淇敼鍓嶏細
  DefinitionBlock ("DSDT.aml", "DSDT", 2, "INTEL ", "TEMPLATE", 0x00000000)
  # 淇敼鍚庯細
  DefinitionBlock ("DSDT.aml", "DSDT", 2, "INTEL ", "TEMPLATE", 0x00000001)
  iasl -sa dsdt.dsl
  # 灏嗗師濮?ACPI 琛ㄥ姞鍏ヤ竴涓湭鍘嬬缉鐨?cpio 褰掓。銆?  # 瀹冧滑蹇呴』鏀惧湪 cpio 褰掓。鍐呯殑 /kernel/firmware/acpi 鐩綍涓嬨€傛敞鎰忥紝濡傛灉鏀惧湪杩欓噷鐨勮〃
  # 涓庡钩鍙拌〃锛堢浉浼肩殑琛ㄧ鍚嶃€佺浉浼肩殑 OEMID銆佺浉浼肩殑 OEM 琛?ID锛夊尮閰嶄笖鎷ユ湁鏇存柊鐨?OEM
  # Revision锛屽钩鍙拌〃灏嗚姝よ〃鍗囩骇銆傚鏋滄斁鍦ㄨ繖閲岀殑琛ㄤ笌骞冲彴琛ㄤ笉鍖归厤锛堜笉鍚岀殑琛ㄧ鍚嶏紝鎴?  # 涓嶅悓鐨?OEMID锛屾垨涓嶅悓鐨?OEM 琛?ID锛夛紝姝よ〃灏嗚杩藉姞銆?  mkdir -p kernel/firmware/acpi
  cp dsdt.aml kernel/firmware/acpi
  # 鐩墠鏈€澶氬厑璁?"NR_ACPI_INITRD_TABLES (64)" 涓〃锛堣 osl.c锛夛細
  iasl -sa facp.dsl
  iasl -sa ssdt1.dsl
  cp facp.aml kernel/firmware/acpi
  cp ssdt1.aml kernel/firmware/acpi
  # 鏈帇缂╃殑 cpio 褰掓。蹇呴』鏀惧湪鏈€鍓嶉潰銆傚叾瀹冿紙閫氬父鏄帇缂╃殑锛塩pio 褰掓。蹇呴』鎷兼帴鍦ㄥ叾鍚庛€?  # 涓嬮潰鍛戒护鍒涘缓鏈帇缂╃殑 cpio 褰掓。锛屽苟灏嗗師濮?initrd 鎷兼帴鍏跺悗锛?  find kernel | cpio -H newc --create > /boot/instrumented_initrd
  cat /boot/initrd >>/boot/instrumented_initrd
  # 浠ュ澶х殑 acpi 璋冭瘯绾у埆閲嶅惎锛屼緥濡傚惎鍔ㄥ弬鏁帮細
  acpi.debug_level=0x2 acpi.debug_layer=0xFFFFFFFF
  # 鐒跺悗妫€鏌ヤ綘鐨?syslog锛?  [    1.268089] ACPI: PCI Interrupt Routing Table [\_SB_.PCI0._PRT]
  [    1.272091] [ACPI Debug]  String [0x0B] "HELLO WORLD"

```
iasl 鑳藉鍙嶆眹缂栧苟閲嶆柊缂栬瘧鐩稿綋澶氱涓嶅悓鐨勯潤鎬?ACPI 琛ㄣ€?

## 鍦ㄥ摢閲岃幏鍙栫敤鎴锋€佸伐鍏?
iasl 鍜?acpixtract 鏄?Intel ACPICA 椤圭洰鐨勪竴閮ㄥ垎锛?https://acpica.org/

骞朵笖搴斿綋鐢卞彂琛岀増鎵撳寘鎻愪緵锛堜緥濡傚湪 SUSE 涓婄殑 acpica 鍖呬腑锛夈€?
acpidump 鍙互鍦?Len Brown 鐨?pmtools 涓壘鍒帮細
ftp://kernel.org/pub/linux/kernel/people/lenb/acpi/utils/pmtools/acpidump

璇ュ伐鍏峰湪 SUSE 涓婁篃鏄?acpica 鍖呯殑涓€閮ㄥ垎銆傚彟澶栵紝鍦ㄦ渶鏂板唴鏍镐腑鍙互閫氳繃 sysfs 鑾峰彇宸茬敤鐨?ACPI 琛細
/sys/firmware/acpi/tables
