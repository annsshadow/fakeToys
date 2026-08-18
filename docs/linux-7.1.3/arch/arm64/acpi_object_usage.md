## ACPI 琛?

涓嬮潰瀵瑰悇涓?ACPI 琛ㄧ殑鏈熸湜杩涜浜嗚璁恒€?
濡傛灉浣跨敤浜嗙珷鑺傚彿锛屽畠鎸囩殑鏄畾涔夎瀵硅薄鐨?ACPI 瑙勮寖涓殑绔犺妭鍙枫€傚鏋滀娇鐢ㄤ簡 "Signature Reserved"锛屽垯琛ㄧ鍚嶏紙琛ㄧ殑鍓嶅洓涓瓧鑺傦級鏄瑙勮寖鍞竴璇嗗埆鐨勯儴鍒嗭紝鑰屽疄闄呰〃鐨勫畾涔夊湪 UEFI Forum 涔嬪锛堝弬瑙佽鑼冪殑 5.2.6 鑺傦級銆?
瀵逛簬 arm64 涓婄殑 ACPI锛岃〃杩樺垎涓轰互涓嬪嚑绫伙細

       - 蹇呴渶锛圧equired锛? DSDT, FADT, GTDT, MADT, MCFG, RSDP, SPCR, XSDT

       - 鎺ㄨ崘锛圧ecommended锛? BERT, EINJ, ERST, HEST, PCCT, SSDT

       - 鍙€夛紙Optional锛? AGDI, BGRT, CEDT, CPEP, CSRT, DBG2, DRTM, ECDT, FACS, FPDT,
          HMAT, IBFT, IORT, MCHI, MPAM, MPST, MSCT, NFIT, PMTT, PPTT, RASF, SBST,
          SDEI, SLIT, SPMI, SRAT, STAO, TCPA, TPM2, UEFI, XENV

       - 涓嶆敮鎸侊紙Not supported锛? AEST, APMT, BOOT, DBGP, DMAR, ETDT, HPET, IVRS, LPIT,
          MSDM, OEMx, PDTT, PSDT, RAS2, RSDT, SLIC, WAET, WDAT, WDRT, WPBT

====== ========================================================================
Table  鐢ㄤ簬 ARMv8 Linux 鐨勭敤娉?====== ========================================================================
AEST   Signature Reserved (signature == "AEST")

       **Arm 閿欒婧愯〃锛圓rm Error Source Table锛?*

       璇ヨ〃鍛婄煡鎿嶄綔绯荤粺绯荤粺涓墍鏈夌鍚?Arm RAS 鏋舵瀯鐨勯敊璇妭鐐广€?
AGDI   Signature Reserved (signature == "AGDI")

       **Arm 閫氱敤璇婃柇杞偍涓庡浣嶈澶囨帴鍙ｈ〃锛圓rm Generic diagnostic Dump and Reset Device Interface Table锛?*

       璇ヨ〃鎻忚堪涓€涓笉鍙睆钄戒簨浠讹紝鐢卞钩鍙板浐浠朵娇鐢紝鐢ㄤ簬璇锋眰鎿嶄綔绯荤粺鐢熸垚璇婃柇杞偍骞跺浣嶈澶囥€?
APMT   Signature Reserved (signature == "APMT")

       **Arm 鎬ц兘鐩戞帶琛紙Arm Performance Monitoring Table锛?*

       璇ヨ〃鎻忚堪绯荤粺涓悇缁勪欢鎵€瀹炵幇鐨?PMU 鏀寔灞炴€с€?
BERT   Section 18.3 (signature == "BERT")

       **鍚姩閿欒璁板綍琛紙Boot Error Record Table锛?*

       濡傛灉骞冲彴鎻愪緵 RAS 鏀寔鍒欏繀椤绘彁渚涖€傚缓璁彁渚涙琛ㄣ€?
BOOT   Signature Reserved (signature == "BOOT")

       **绠€鍗?BOOT 鏍囧織琛紙simple BOOT flag table锛?*

       浠呭井杞娇鐢ㄧ殑琛紝灏嗕笉琚敮鎸併€?
BGRT   Section 5.2.22 (signature == "BGRT")

       **鍚姩鍥惧舰璧勬簮琛紙Boot Graphics Resource Table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸侊紝瀵?ARM 鏈嶅姟鍣ㄦ病鏈夊疄闄呯敤渚嬨€?
CEDT   Signature Reserved (signature == "CEDT")

       **CXL 鏃╂湡鍙戠幇琛紙CXL Early Discovery Table锛?*

       璇ヨ〃鍏佽鎿嶄綔绯荤粺鍙戠幇浠讳綍 CXL 涓绘満妗ュ強鍏朵富鏈烘ˉ瀵勫瓨鍣ㄣ€?
CPEP   Section 5.2.18 (signature == "CPEP")

       **宸蹭慨姝ｅ钩鍙伴敊璇疆璇㈣〃锛圕orrected Platform Error Polling table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸侊紝涓斿湪鍏峰 ARM 鍏煎纭欢骞堕€傚綋淇敼瑙勮寖涔嬪墠涓嶅缓璁娇鐢ㄣ€?
CSRT   Signature Reserved (signature == "CSRT")

       **鏍稿績绯荤粺璧勬簮琛紙Core System Resources Table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸併€?
DBG2   Signature Reserved (signature == "DBG2")

       **璋冭瘯绔彛琛?2锛圖eBuG port table 2锛?*

       璁稿彲璇佸凡鍙樻洿锛屽簲褰撳彲鐢ㄣ€傚鏋滃湪鍛戒护琛屼腑鏇夸唬 earlycon=<device> 浣跨敤鍒欎负鍙€夈€?
DBGP   Signature Reserved (signature == "DBGP")

       **璋冭瘯绔彛琛紙DeBuG Port table锛?*

       浠呭井杞娇鐢ㄧ殑琛紝灏嗕笉琚敮鎸併€?
DSDT   Section 5.2.11.1 (signature == "DSDT")

       **宸紓鍖栫郴缁熸弿杩拌〃锛圖ifferentiated System Description Table锛?*

       DSDT 鏄繀闇€鐨勶紱鍙﹁ SSDT銆?
       ACPI 琛ㄥ彧鍖呭惈涓€涓?DSDT锛屼絾鍙互鍖呭惈涓€涓垨澶氫釜鍙€夌殑 SSDT銆傛瘡涓?SSDT 鍙兘鍚?       ACPI 鍛藉悕绌洪棿娣诲姞鍐呭锛屼笉鑳戒慨鏀规垨鏇挎崲 DSDT 涓殑浠讳綍鍐呭銆?
DMAR   Signature Reserved (signature == "DMAR")

       **DMA 閲嶆槧灏勮〃锛圖MA Remapping table锛?*

       浠?x86 浣跨敤鐨勮〃锛屽皢涓嶈鏀寔銆?
DRTM   Signature Reserved (signature == "DRTM")

       **鍔ㄦ€佸害閲忎俊浠绘牴琛紙Dynamic Root of Trust for Measurement table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸併€?
ECDT   Section 5.2.16 (signature == "ECDT")

       **宓屽叆寮忔帶鍒跺櫒鎻忚堪琛紙Embedded Controller Description Table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸侊紝浣嗕粎鍦ㄧ‖浠剁簿绠€妯″紡涓嬩娇鐢?GPE_BIT 瀛楁鏉ヨ〃绀?IRQ 鍙锋椂鎵嶅彲鑳藉湪
       ARM 涓婁娇鐢紝鍥犱负鍦ㄧ‖浠剁簿绠€妯″紡涓嬫病鏈夊畾涔?GPE 鍧椼€傝繖闇€瑕佸湪 ACPI 瑙勮寖涓綔鍑轰慨鏀广€?
EINJ   Section 18.6 (signature == "EINJ")

       **閿欒娉ㄥ叆琛紙Error Injection table锛?*

       璇ヨ〃瀵逛簬娴嬭瘯骞冲彴瀵归敊璇潯浠剁殑鍝嶅簲闈炲父鏈夌敤锛涘畠鍏佽鍚戠郴缁熶腑娉ㄥ叆涓€涓敊璇紝灏卞儚瀹冨疄闄?       鍙戠敓涓€鏍枫€備絾鏄紝姝よ〃涓嶅簲闅忕敓浜х郴缁熷彂甯冿紱瀹冨簲浠呭湪娴嬭瘯鏈熼棿浣跨敤 ACPICA 宸ュ叿鍔ㄦ€佸姞杞?       鍜屾墽琛屻€?
ERST   Section 18.5 (signature == "ERST")

       **閿欒璁板綍搴忓垪鍖栬〃锛圗rror Record Serialization Table锛?*

       鍦ㄦ敮鎸?RAS 鐨勫钩鍙颁笂锛岃嫢鍏堕潪鍩轰簬 UEFI锛屽垯蹇呴』鎻愪緵姝よ〃锛涜嫢鍩轰簬 UEFI锛屽垯鍙互鎻愪緵姝よ〃銆?       褰撴病鏈夋琛ㄦ椂锛屽皢浣跨敤 UEFI 杩愯鏃舵湇鍔℃潵鍦ㄦ寔涔呭寲瀛樺偍涓繚瀛樺拰鍙栧洖纭欢閿欒淇℃伅銆?
ETDT   Signature Reserved (signature == "ETDT")

       **浜嬩欢瀹氭椂鍣ㄦ弿杩拌〃锛圗vent Timer Description Table锛?*

       宸插簾寮冪殑琛紝灏嗕笉琚敮鎸併€?
FACS   Section 5.2.10 (signature == "FACS")

       **鍥轰欢 ACPI 鎺у埗缁撴瀯锛團irmware ACPI Control Structure锛?*

       璇ヨ〃涓嶅お鍙兘闈炲父鏈夌敤銆傚鏋滄彁渚涳紝灏嗕笉浼氫娇鐢ㄥ叏灞€閿侊紝鍥犱负瀹冧笉灞炰簬纭欢绮剧畝閰嶇疆鐨勪竴閮ㄥ垎锛?       骞朵笖鍙湁 64 浣嶅湴鍧€瀛楁浼氳瑙嗕负鏈夋晥銆?
FADT   Section 5.2.9 (signature == "FACP")

       **鍥哄畾 ACPI 鎻忚堪琛紙Fixed ACPI Description Table锛?*
       瀵?arm64 鏄繀闇€鐨勩€?

       HW_REDUCED_ACPI 鏍囧織蹇呴』璁剧疆銆傚綋璁剧疆 HW_REDUCED_ACPI 鏃跺簲蹇界暐鐨勬墍鏈夊瓧娈甸兘搴斾负闆躲€?
       濡傛灉鎻愪緵浜?FACS 琛紝搴斾娇鐢?X_FIRMWARE_CTRL 瀛楁锛岃€屼笉鏄?FIRMWARE_CTRL銆?
       濡傛灉浣跨敤 PSCI锛堝寤鸿锛夛紝璇风‘淇濇纭～鍐?ARM_BOOT_ARCH 鈥斺€?璁剧疆 PSCI_COMPLIANT 鏍囧織锛?       骞舵牴鎹渶瑕佽缃垨娓呴櫎 PSCI_USE_HVC锛堣琛?5-37锛夈€?
       瀵逛簬鍚屾牱蹇呴渶鐨?DSDT锛屽簲浣跨敤 X_DSDT 瀛楁锛岃€屼笉鏄?DSDT 瀛楁銆?
FPDT   Section 5.2.23 (signature == "FPDT")

       **鍥轰欢鎬ц兘鏁版嵁琛紙Firmware Performance Data Table锛?*

       鍙€夛紝瀵瑰惎鍔ㄦ€ц兘鍒嗘瀽鏈夌敤銆?
GTDT   Section 5.2.24 (signature == "GTDT")

       **閫氱敤瀹氭椂鍣ㄦ弿杩拌〃锛圙eneric Timer Description Table锛?*

       瀵?arm64 鏄繀闇€鐨勩€?
HEST   Section 18.3.2 (signature == "HEST")

       **纭欢閿欒婧愯〃锛圚ardware Error Source Table锛?*

       宸茬粡瀹氫箟浜?ARM 鐗规湁鐨勯敊璇簮锛涜浣跨敤杩欎簺锛屾垨鑰呬娇鐢?PCI 绫诲瀷锛屼緥濡傜被鍨?6锛圓ER 鏍圭鍙ｏ級銆?       7锛圓ER 绔偣锛夋垨 8锛圓ER 妗ワ級锛屾垨鑰呬娇鐢ㄧ被鍨?9锛堥€氱敤纭欢閿欒婧愶級銆備粎鍦?arm64 涓婁娇鐢?       Trusted Firmware 鏃舵墠鍙兘杩涜鍥轰欢浼樺厛鐨勯敊璇鐞嗐€?
       濡傛灉骞冲彴鎻愪緵 RAS 鏀寔鍒欏繀椤绘彁渚涖€傚缓璁彁渚涙琛ㄣ€?
HMAT   Section 5.2.28 (signature == "HMAT")

       **寮傛瀯鍐呭瓨灞炴€ц〃锛圚eterogeneous Memory Attribute Table锛?*

       璇ヨ〃鎻忚堪涓庡唴瀛橀偦杩戝煙鐩稿叧鐨勫唴瀛樺睘鎬э紝渚嬪鍐呭瓨渚х紦瀛樺睘鎬т互鍙婂甫瀹藉拰寤惰繜缁嗚妭銆傛搷浣滅郴缁?       浣跨敤杩欎簺淇℃伅鏉ヤ紭鍖栫郴缁熷唴瀛橀厤缃€?
HPET   Signature Reserved (signature == "HPET")

       **楂樼簿搴︿簨浠跺畾鏃跺櫒琛紙High Precision Event timer Table锛?*

       浠?x86 浣跨敤鐨勮〃锛屽皢涓嶈鏀寔銆?
IBFT   Signature Reserved (signature == "IBFT")

       **iSCSI 鍚姩鍥轰欢琛紙iSCSI Boot Firmware Table锛?*

       寰蒋瀹氫箟鐨勮〃锛屾敮鎸佹儏鍐靛緟瀹氥€?
IORT   Signature Reserved (signature == "IORT")

       **杈撳叆杈撳嚭閲嶆槧灏勮〃锛圛nput Output Remapping Table锛?*

       浠?arm64 浣跨敤鐨勮〃锛岀敤浜庢弿杩?IO 鎷撴墤銆丼MMU 鍜?GIC ITS锛屼互鍙婅繖浜涗笉鍚岀粍浠跺浣曡繛鎺ュ湪涓€璧凤紝
       渚嬪鏍囪瘑鍝簺缁勪欢浣嶄簬鍝簺 SMMU/ITS 涔嬪悗銆傝琛ㄤ粎鍦ㄧ壒瀹?SBSA 骞冲彴涓婃槸蹇呴渶鐨勶紙渚嬪浣跨敤
       GICv3-ITS 鍜?SMMU 鏃讹級锛涘湪 SBSA Level 0 骞冲彴涓婂畠浠嶆槸鍙€夌殑銆?
IVRS   Signature Reserved (signature == "IVRS")

       **I/O 铏氭嫙鍖栨姤鍛婄粨鏋勶紙I/O Virtualization Reporting Structure锛?*

       浠?x86_64锛圓MD锛変娇鐢ㄧ殑琛紝灏嗕笉琚敮鎸併€?
LPIT   Signature Reserved (signature == "LPIT")

       **浣庡姛鑰楃┖闂茶〃锛圠ow Power Idle Table锛?*

       鍦?ACPI 5.1 涔嬪墠浠?x86 浣跨敤鐨勮〃锛涗粠 ACPI 6.0 璧凤紝ARM 骞冲彴涓婄殑澶勭悊鍣ㄦ弿杩板拰鐢垫簮鐘舵€佸簲
       浣跨敤 DSDT 骞跺畾涔夊鐞嗗櫒瀹瑰櫒璁惧锛坃HID ACPI0010锛岀 8.4 鑺傦紝鏇村叿浣撳湴鏄?8.4.3 鍜?8.4.4锛夈€?
MADT   Section 5.2.12 (signature == "APIC")

       **澶?APIC 鎻忚堪琛紙Multiple APIC Description Table锛?*

       瀵?arm64 鏄繀闇€鐨勩€傚彧搴斾娇鐢?GIC 涓柇鎺у埗鍣ㄧ粨鏋勶紙绫诲瀷 0xA - 0xF锛夈€?
MCFG   Signature Reserved (signature == "MCFG")

       **鍐呭瓨鏄犲皠閰嶇疆绌洪棿锛圡emory-mapped ConFiGuration space锛?*

       濡傛灉骞冲彴鏀寔 PCI/PCIe锛屽垯闇€瑕?MCFG 琛ㄣ€?
MCHI   Signature Reserved (signature == "MCHI")

       **绠＄悊鎺у埗鍣ㄤ富鏈烘帴鍙ｈ〃锛圡anagement Controller Host Interface table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸併€?
MPAM   Signature Reserved (signature == "MPAM")

       **鍐呭瓨鍒嗗尯涓庣洃鎺ц〃锛圡emory Partitioning And Monitoring table锛?*

       璇ヨ〃鍏佽鎿嶄綔绯荤粺鍙戠幇鍚勫瓙绯荤粺瀹炵幇鐨?MPAM 鎺у埗銆?
MPST   Section 5.2.21 (signature == "MPST")

       **鍐呭瓨鐢垫簮鐘舵€佽〃锛圡emory Power State Table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸併€?
MSCT   Section 5.2.19 (signature == "MSCT")

       **鏈€澶х郴缁熺壒鎬ц〃锛圡aximum System Characteristic Table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸併€?
MSDM   Signature Reserved (signature == "MSDM")

       **寰蒋鏁版嵁绠＄悊琛紙Microsoft Data Management table锛?*

       浠呭井杞娇鐢ㄧ殑琛紝灏嗕笉琚敮鎸併€?
NFIT   Section 5.2.25 (signature == "NFIT")

       **NVDIMM 鍥轰欢鎺ュ彛琛紙NVDIMM Firmware Interface Table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸併€?
OEMx   Signature of "OEMx" only

       **OEM 鐗瑰畾琛紙OEM Specific Tables锛?*

       鎵€鏈変互 "OEM" 绛惧悕鐨勮〃閮戒繚鐣欑粰 OEM 浣跨敤銆傜敱浜庤繖浜涜〃骞堕潪鐢ㄤ簬閫氱敤鐩殑锛岃€屾槸闄愪簬闈炲父
       鐗瑰畾鐨勬渶缁堢敤鎴凤紝鍥犳涓嶅缓璁娇鐢紝骞朵笖 arm64 鐨勫唴鏍镐篃涓嶆敮鎸佸畠浠€?
PCCT   Section 14.1 (signature == "PCCT)

       **骞冲彴閫氫俊閫氶亾琛紙Platform Communications Channel Table锛?*

       鎺ㄨ崘鍦?arm64 涓婁娇鐢紱褰撲娇鐢?CPPC 鎺у埗骞冲彴澶勭悊鍣ㄧ殑鎬ц兘鍜屽姛鑰楁椂锛屽缓璁娇鐢?PCC銆?
PDTT   Section 5.2.29 (signature == "PDTT")

       **骞冲彴璋冭瘯瑙﹀彂琛紙Platform Debug Trigger Table锛?*

       璇ヨ〃鎻忚堪鐢ㄤ簬鏀堕泦闈炴灦鏋勭壒鎬ц皟璇曟棩蹇楃殑 PCC 閫氶亾銆?

PMTT   Section 5.2.21.12 (signature == "PMTT")

       **骞冲彴鍐呭瓨鎷撴墤琛紙Platform Memory Topology Table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸併€?
PPTT   Section 5.2.30 (signature == "PPTT")

       **澶勭悊鍣ㄥ睘鎬ф嫇鎵戣〃锛圥rocessor Properties Topology Table锛?*

       璇ヨ〃鎻愪緵澶勭悊鍣ㄥ拰缂撳瓨鎷撴墤銆?
PSDT   Section 5.2.11.3 (signature == "PSDT")

       **鎸佷箙绯荤粺鎻忚堪琛紙Persistent System Description Table锛?*

       宸插簾寮冪殑琛紝灏嗕笉琚敮鎸併€?
RAS2   Section 5.2.21 (signature == "RAS2")

       **RAS 鐗规€?2 琛紙RAS Features 2 table锛?*

       璇ヨ〃涓哄钩鍙板疄鐜扮殑 RAS 鑳藉姏鎻愪緵鎺ュ彛銆?
RASF   Section 5.2.20 (signature == "RASF")

       **RAS 鐗规€ц〃锛圧AS Feature table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸併€?
RSDP   Section 5.2.5 (signature == "RSD PTR")

       **鏍圭郴缁熸弿杩版寚閽堬紙Root System Description PoinTeR锛?*

       瀵?arm64 鏄繀闇€鐨勩€?
RSDT   Section 5.2.7 (signature == "RSDT")

       **鏍圭郴缁熸弿杩拌〃锛圧oot System Description Table锛?*

       鐢变簬璇ヨ〃鍙兘鎻愪緵 32 浣嶅湴鍧€锛屽畠鍦?arm64 涓婂凡琚簾寮冿紝灏嗕笉浼氳浣跨敤銆傚鏋滄彁渚涳紝瀹冨皢琚拷鐣ャ€?
SBST   Section 5.2.14 (signature == "SBST")

       **鏅鸿兘鐢垫睜瀛愮郴缁熻〃锛圫mart Battery Subsystem Table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸併€?
SDEI   Signature Reserved (signature == "SDEI")

       **杞欢濮旀墭寮傚父鎺ュ彛琛紙Software Delegated Exception Interface table锛?*

       璇ヨ〃閫氬憡 SDEI 鎺ュ彛鐨勫瓨鍦ㄣ€?
SLIC   Signature Reserved (signature == "SLIC")

       **杞欢璁稿彲琛紙Software LIcensing table锛?*

       浠呭井杞娇鐢ㄧ殑琛紝灏嗕笉琚敮鎸併€?
SLIT   Section 5.2.17 (signature == "SLIT")

       **绯荤粺灞€閮ㄦ€ц窛绂讳俊鎭〃锛圫ystem Locality distance Information Table锛?*

       涓€鑸潵璇村彲閫夛紝浣嗗 NUMA 绯荤粺鏄繀闇€鐨勩€?
SPCR   Signature Reserved (signature == "SPCR")

       **涓插彛鎺у埗鍙伴噸瀹氬悜琛紙Serial Port Console Redirection table锛?*

       瀵?arm64 鏄繀闇€鐨勩€?
SPMI   Signature Reserved (signature == "SPMI")

       **鏈嶅姟鍣ㄥ钩鍙扮鐞嗘帴鍙ｈ〃锛圫erver Platform Management Interface table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸併€?
SRAT   Section 5.2.16 (signature == "SRAT")

       **绯荤粺璧勬簮浜插拰鎬ц〃锛圫ystem Resource Affinity Table锛?*

       鍙€夛紝浣嗗鏋滀娇鐢紝鍒欏彧璇诲彇 GICC Affinity 缁撴瀯銆備负浜嗘敮鎸?arm64 NUMA锛屾琛ㄦ槸蹇呴渶鐨勩€?
SSDT   Section 5.2.11.2 (signature == "SSDT")

       **杈呭姪绯荤粺鎻忚堪琛紙Secondary System Description Table锛?*

       杩欎簺琛ㄦ槸 DSDT 鐨勫欢缁紱寤鸿灏嗗畠浠敤浜庡彲浠ユ坊鍔犲埌杩愯涓殑绯荤粺鐨勮澶囷紝浣嗕篃鍙互璧峰埌灏?       璁惧鎻忚堪鎷嗗垎涓烘洿鏄撶鐞嗙殑鐗囨鐨勪綔鐢ㄣ€?
       SSDT 鍙兘鍚?ACPI 鍛藉悕绌洪棿娣诲姞鍐呭銆傚畠涓嶈兘淇敼鎴栨浛鎹㈠懡鍚嶇┖闂翠腑宸叉湁鐨勮澶囨弿杩般€?
       涓嶈繃杩欎簺琛ㄦ槸鍙€夌殑銆侫CPI 琛ㄥ簲鍙寘鍚竴涓?DSDT锛屼絾鍙互鍖呭惈澶氫釜 SSDT銆?
STAO   Signature Reserved (signature == "STAO")

       **_STA 瑕嗙洊琛紙_STA Override table锛?*

       鍙€夛紝浣嗕粎鍦ㄨ櫄鎷熷寲鐜涓负浜嗗悜瀹㈡埛鏈烘搷浣滅郴缁熼殣钘忚澶囨椂鎵嶉渶瑕併€?
TCPA   Signature Reserved (signature == "TCPA")

       **鍙俊璁＄畻骞冲彴鑱旂洘琛紙Trusted Computing Platform Alliance table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸侊紝骞朵笖鍙兘闇€瑕佷慨鏀规墠鑳戒笌 arm64 瀹屽叏浜掓搷浣溿€?
TPM2   Signature Reserved (signature == "TPM2")

       **鍙俊骞冲彴妯″潡 2 琛紙Trusted Platform Module 2 table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸侊紝骞朵笖鍙兘闇€瑕佷慨鏀规墠鑳戒笌 arm64 瀹屽叏浜掓搷浣溿€?
UEFI   Signature Reserved (signature == "UEFI")

       **UEFI ACPI 鏁版嵁琛紙UEFI ACPI data table锛?*

       鍙€夛紝褰撳墠涓嶆敮鎸併€傜洰鍓嶅 arm64 娌℃湁宸茬煡鐢ㄤ緥銆?
WAET   Signature Reserved (signature == "WAET")

       **Windows ACPI 妯℃嫙璁惧琛紙Windows ACPI Emulated devices Table锛?*

       浠呭井杞娇鐢ㄧ殑琛紝灏嗕笉琚敮鎸併€?
WDAT   Signature Reserved (signature == "WDAT")

       **鐪嬮棬鐙楀姩浣滆〃锛圵atch Dog Action Table锛?*

       浠呭井杞娇鐢ㄧ殑琛紝灏嗕笉琚敮鎸併€?
WDRT   Signature Reserved (signature == "WDRT")

       **鐪嬮棬鐙楄祫婧愯〃锛圵atch Dog Resource Table锛?*

       浠呭井杞娇鐢ㄧ殑琛紝灏嗕笉琚敮鎸併€?
WPBT   Signature Reserved (signature == "WPBT")

       **Windows 骞冲彴浜岃繘鍒惰〃锛圵indows Platform Binary Table锛?*

       浠呭井杞娇鐢ㄧ殑琛紝灏嗕笉琚敮鎸併€?
XENV   Signature Reserved (signature == "XENV")

       **Xen 椤圭洰琛紙Xen project table锛?*

       鍙€夛紝鐩墠浠?Xen 浣跨敤銆?
XSDT   Section 5.2.8 (signature == "XSDT")

       **鎵╁睍绯荤粺鎻忚堪琛紙eXtended System Description Table锛?*

       瀵?arm64 鏄繀闇€鐨勩€?====== ========================================================================

### ACPI 瀵硅薄

涓嬮潰鍒楀嚭浜嗗彲鑳戒娇鐢ㄧ殑鍚勪釜 ACPI 瀵硅薄鐨勬湡鏈涳紱浠讳綍鏈湪涓嬮潰鏄庣‘鎻愬強鐨勫璞￠兘搴旀牴鎹壒瀹氬钩鍙版垨鐗瑰畾
瀛愮郴缁燂紙渚嬪鐢垫簮绠＄悊鎴?PCI锛夌殑闇€瑕佹潵浣跨敤銆?
===== ================ ========================================================
Name   Section         Usage for ARMv8 Linux
===== ================ ========================================================
_CCA   6.2.17         璇ユ柟娉曞繀椤讳负 arm64 涓婃墍鏈夋€荤嚎涓昏澶囧畾涔?鈥斺€?涓嶄細瀵硅繖浜涜澶?                      鏄惁缂撳瓨涓€鑷村仛浠讳綍鍋囪銆俖CCA 鍊肩敱杩欎簺璁惧鐨勬墍鏈夊悗浠ｇ户鎵匡紝鍥犳鏃犻渶
                      閲嶅瀹氫箟銆傚湪 arm64 涓婅嫢娌℃湁 _CCA锛屽唴鏍镐笉鐭ラ亾璇ュ浣曚负璇ヨ澶囪缃?DMA銆?
                      NB锛氳鏂规硶鎻愪緵榛樿鐨勭紦瀛樹竴鑷存€у睘鎬э紱涓嶈繃锛孲MMU 鐨勫瓨鍦ㄥ彲浠ュ姝よ繘琛?                      淇敼銆備緥濡傦紝鏌愪釜涓昏澶囬粯璁ゅ彲鑳芥槸闈炰竴鑷寸殑锛屼絾閫氳繃閫傚綋鐨?SMMU 閰嶇疆
                      鍙互鍙樹负涓€鑷寸殑锛堝弬瑙?IORT 瑙勮寖琛?17锛孉RM 鏂囨。 DEN 0049B锛夈€?
_CID   6.1.2          鎸夐渶浣跨敤锛屽彟瑙?_HID銆?
_CLS   6.1.3          鎸夐渶浣跨敤锛屽彟瑙?_HID銆?
_CPC   8.4.7.1        鎸夐渶浣跨敤锛岀壒瀹氫簬鐢垫簮绠＄悊銆傚湪 arm64 涓婃帹鑽愪娇鐢?CPPC銆?
_CRS   6.2.2          鍦?arm64 涓婃槸蹇呴渶鐨勩€?
_CSD   8.4.2.2        鎸夐渶浣跨敤锛屼粎涓?_CST 閰嶅悎浣跨敤銆?
_CST   8.4.2.1        鎺ㄨ崘浣跨敤浣庡姛鑰楃┖闂茬姸鎬侊紙8.4.4锛夎€岄潪 C-states銆?
_DDN   6.1.4          璇ュ瓧娈靛彲鐢ㄤ簬璁惧鍚嶇О銆備絾鏄紝瀹冩湰鎰忕敤浜?DOS 璁惧鍚嶇О锛堜緥濡?COM1锛夛紝
                      鍥犳璺ㄦ搷浣滅郴缁熶娇鐢ㄦ椂瑕佸皬蹇冦€?
_DSD   6.2.5          浣跨敤鏃跺簲璋ㄦ厧銆傚鏋滀娇鐢ㄦ瀵硅薄锛岃灏介噺鍦ㄨ澶囧睘鎬?UUID 宸插畾涔夌殑绾︽潫
                      鑼冨洿鍐呬娇鐢ㄥ畠銆傚彧鏈夊湪鏋佸皯鏁版儏鍐典笅鎵嶉渶瑕佸垱寤烘柊鐨?_DSD UUID銆?
                      鏃犺鍝鎯呭喌锛岄兘搴旀彁浜?_DSD 瀹氫箟浠ュ強浠讳綍椹卞姩琛ヤ竵浠ヤ緵璁ㄨ锛屽挨鍏舵槸鍦?                      浣跨敤璁惧灞炴€ф椂銆傛病鏈夌浉搴旂殑 _DSD 鎻忚堪锛岄┍鍔ㄥ皢琚涓轰笉瀹屾暣銆備竴鏃﹁幏寰?                      鍐呮牳缁存姢鑰呮壒鍑嗭紝UUID 鎴栬澶囧睘鎬ц繕蹇呴』鍚?UEFI Forum 娉ㄥ唽锛涚敱浜庝細鏈?                      澶氫釜鎿嶄綔绯荤粺娉ㄥ唽鏉＄洰锛岃繖鍙兘寮曡捣涓€浜涘弽澶嶃€?
_DSM   9.1.1          涓嶈浣跨敤姝ゆ柟娉曘€傚畠鏈爣鍑嗗寲锛岃繑鍥炲€兼枃妗ｄ笉鍏紝骞朵笖鐩墠鏄绻佺殑閿欒鏉ユ簮銆?
\_GL   5.7.1          璇ュ璞′笉搴斿湪纭欢绮剧畝妯″紡涓嬩娇鐢紝鍥犳涓嶅簲鍦?arm64 涓婁娇鐢ㄣ€?
_GLK   6.5.7          璇ュ璞￠渶瑕佸畾涔変竴涓叏灞€閿侊紱鐢变簬 arm64 杩愯鍦ㄧ‖浠剁簿绠€妯″紡涓嬶紝娌℃湁
                      鍏ㄥ眬閿併€傚洜姝わ紝涓嶈鍦?arm64 涓婁娇鐢ㄦ瀵硅薄銆?
\_GPE  5.3.1          姝ゅ懡鍚嶇┖闂翠粎鐢ㄤ簬 x86銆備笉瑕佸湪 arm64 涓婁娇鐢ㄥ畠銆?
_HID   6.1.5          杩欐槸鍦ㄨ澶囨帰娴嬩腑浣跨敤鐨勪富瑕佸璞★紝涓嶈繃涔熷彲浠ヤ娇鐢?_CID 鍜?_CLS銆?
_INI   6.5.1          闈炲繀闇€锛屼絾褰?UEFI 灏嗚澶囩暀鍦ㄩ┍鍔ㄥ紑濮嬫帰娴嬪墠鍙兘涓嶆湡鏈涚殑鐘舵€佹椂锛屽畠瀵?                      璁剧疆璁惧鍙兘鏈夌敤銆?
_LPI   8.4.4.3        鎺ㄨ崘鍦?arm64 涓婁笌澶勭悊鍣ㄥ畾涔夛紙_HID ACPI0010锛変竴璧蜂娇鐢ㄣ€傚彟瑙?_RDI銆?
_MLS   6.1.7          寮虹儓鎺ㄨ崘鐢ㄤ簬鍥介檯鍖栥€?
_OFF   7.2.2          寤鸿涓轰换浣曞彲浠ユ墦寮€鎴栧叧闂殑璁惧瀹氫箟姝ゆ柟娉曘€?
_ON    7.2.3          寤鸿涓轰换浣曞彲浠ユ墦寮€鎴栧叧闂殑璁惧瀹氫箟姝ゆ柟娉曘€?
\_OS   5.7.3          榛樿鎯呭喌涓嬫鏂规硶杩斿洖 "Linux"锛堣繖鏄?Linux 涓婂畯 ACPI_OS_NAME 鐨?                      鍊硷級銆傚懡浠よ鍙傛暟 acpi_os=<string> 鍙敤浜庡皢鍏惰缃负鍏朵粬鍊笺€?
_OSC   6.2.11         姝ゆ柟娉曞彲浠ユ槸 ACPI 涓殑鍏ㄥ眬鏂规硶锛堝嵆 \_SB._OSC锛夛紝涔熷彲浠ヤ笌鐗瑰畾璁惧鍏宠仈
                      锛堜緥濡?\_SB.DEV0._OSC锛夛紝鎴栦袱鑰呭吋鍏枫€傚綋鐢ㄤ綔鍏ㄥ眬鏂规硶鏃讹紝鍙厑璁镐娇鐢?ACPI
                      瑙勮寖涓彂甯冪殑鑳藉姏銆傚綋鐢ㄤ綔璁惧鐗瑰畾鏂规硶鏃讹紝蹇呴』浣跨敤涓轰娇鐢?_DSD 鎵€鎻忚堪鐨?                      杩囩▼鏉ュ垱寤?_OSC 瀹氫箟锛涗笉鍏佽杩涚▼澶栦娇鐢?_OSC銆備篃灏辨槸璇达紝灏嗚澶囩壒瀹氱殑
                      _OSC 鐢ㄦ硶鎻忚堪浣滀负鍐呮牳椹卞姩鎻愪氦鐨勪竴閮ㄥ垎鎻愪氦锛岃幏寰楀唴鏍哥ぞ鍖烘壒鍑嗭紝鐒跺悗鍚?                      UEFI Forum 娉ㄥ唽銆?
\_OSI  5.7.2          鍦?ARM64 涓婂凡搴熷純銆傚氨 ACPI 鍥轰欢鑰岃█锛宊OSI 涓嶅簲鐢ㄤ簬纭畾姝ｅ湪浣跨敤浣曠
		      绯荤粺鎴栨彁渚涗綍绉嶅姛鑳姐€傚簲浣跨敤 _OSC 鏂规硶浠ｆ浛銆?
_PDC   8.4.1          宸插簾寮冿紝涓嶈鍦?arm64 涓婁娇鐢ㄣ€?
\_PIC  5.8.1          涓嶅簲浣跨敤姝ゆ柟娉曘€傚湪 arm64 涓婏紝鍞竴鍙敤鐨勪腑鏂ā鍨嬫槸 GIC銆?
\_PR   5.3.1          姝ゅ懡鍚嶇┖闂翠粎鐢ㄤ簬浼犵粺绯荤粺涓婄殑 x86銆備笉瑕佸湪 arm64 涓婁娇鐢ㄥ畠銆?
_PRT   6.2.13         浣滀负鎵€鏈?PCI 鏍硅澶囧畾涔夌殑涓€閮ㄥ垎鏄繀闇€鐨勩€?
_PRx   7.3.8-11       鎸夐渶浣跨敤锛涚壒瀹氫簬鐢垫簮绠＄悊銆傚鏋滃畾涔変簡 _PR0锛屽垯涔熷繀椤诲畾涔?_PR3銆?
_PSx   7.3.2-5        鎸夐渶浣跨敤锛涚壒瀹氫簬鐢垫簮绠＄悊銆傚鏋滃畾涔変簡 _PS0锛屽垯涔熷繀椤诲畾涔?_PS3銆傚鏋?                      鏃堕挓鎴栬皟鑺傚櫒闇€瑕佽皟鏁翠互涓庡姛鑰椾竴鑷达紝璇峰湪杩欎簺鏂规硶涓洿鏀瑰畠浠€?
_RDI   8.4.4.4        鎺ㄨ崘鍦?arm64 涓婁笌澶勭悊鍣ㄥ畾涔夛紙_HID ACPI0010锛変竴璧蜂娇鐢ㄣ€傝繖鍙簲涓?		      _LPI 閰嶅悎浣跨敤銆?
\_REV  5.7.4          濮嬬粓杩斿洖鎵€鏀寔鐨勬渶鏂?ACPI 鐗堟湰銆?
\_SB   5.3.1          鍦?arm64 涓婃槸蹇呴渶鐨勶紱鎵€鏈夎澶囬兘蹇呴』鍦ㄦ鍛藉悕绌洪棿涓畾涔夈€?
_SLI   6.2.15         褰撲娇鐢?SLIT 琛ㄦ椂寤鸿浣跨敤銆?
_STA   6.3.7,         寤鸿涓轰换浣曞彲浠ユ墦寮€鎴栧叧闂殑璁惧瀹氫箟姝ゆ柟娉曘€傚彟瑙?STAO 琛紝瀹冩彁渚涘湪
       7.2.4          铏氭嫙鍖栫幆澧冧腑闅愯棌璁惧鐨勮鐩栥€?
_SRS   6.2.16         鎸夐渶浣跨敤锛涘彟瑙?_PRS銆?
_STR   6.1.10         鎺ㄨ崘鐢ㄤ簬鍚戞渶缁堢敤鎴蜂紶杈捐澶囧悕绉帮紱杩欎紭浜庝娇鐢?_DDN銆?
_SUB   6.1.9          鎸夐渶浣跨敤锛涗紭鍏堜娇鐢?_HID 鎴?_CID銆?
_SUN   6.1.11         鎸夐渶浣跨敤锛屼絾寤鸿浣跨敤銆?
_SWS   7.4.3          鎸夐渶浣跨敤锛涚壒瀹氫簬鐢垫簮绠＄悊锛涘湪 arm64 涓婁娇鐢ㄥ彲鑳介渶瑕佽鑼冨彉鏇淬€?
_UID   6.1.12         鎺ㄨ崘鐢ㄤ簬鍖哄垎鍚屼竴绫荤殑璁惧锛涘敖鍙兘瀹氫箟瀹冦€?===== ================ ========================================================




### ACPI 浜嬩欢妯″瀷

涓嶈浣跨敤 GPE 鍧楄澶囷紱杩欎簺鍦?arm64 浣跨敤鐨勭‖浠剁簿绠€閰嶇疆涓笉鍙楁敮鎸併€傜敱浜庡湪 ARM 骞冲彴涓婃湭瀹氫箟鐢ㄤ簬
浣跨敤鐨?GPE 鍧楋紝ACPI 浜嬩欢蹇呴』浠ヤ笉鍚屾柟寮忓彂鍑轰俊鍙枫€?
鏈変袱绉嶉€夋嫨锛欸PIO 淇″彿涓柇锛堢 5.6.5 鑺傦級鍜屼腑鏂俊鍙蜂簨浠讹紙绗?5.6.9 鑺傦級銆備腑鏂俊鍙蜂簨浠舵槸 ACPI 6.1
瑙勮寖涓殑鏂扮壒鎬с€傚湪缁欏畾鐨勫钩鍙颁笂鍙互浣跨敤鍏朵腑涓€绉嶆垨涓ょ锛涗娇鐢ㄥ摢绉嶅彲鑳藉彇鍐充簬鐗瑰畾 SoC 鐨勯檺鍒躲€傚鏋?鍙兘锛屽缓璁娇鐢ㄤ腑鏂俊鍙蜂簨浠躲€?

### ACPI 澶勭悊鍣ㄦ帶鍒?
ACPI 瑙勮寖绗?8 鑺傚湪 6.0 鐗堟湰涓彂鐢熶簡閲嶅ぇ鍙樺寲銆傚鐞嗗櫒鐜板湪搴斿畾涔変负甯︽湁 _HID ACPI0007 鐨?Device
瀵硅薄锛涗笉瑕佷娇鐢?ASL 涓凡搴熷純鐨?Processor 璇彞銆傛墍鏈夊澶勭悊鍣ㄧ郴缁熻繕搴斾娇鐢ㄥ鐞嗗櫒瀹瑰櫒璁惧
锛堣绗?8.4.3.1 鑺傦紝_HID ACPI0010锛夊畾涔夊鐞嗗櫒鐨勫眰娆＄粨鏋勶紱涓嶈浣跨敤澶勭悊鍣ㄨ仛鍚堝櫒璁惧锛堢 8.5 鑺傦級
鏉ユ弿杩板鐞嗗櫒鎷撴墤銆傝鑼冪 8.4 鑺傛弿杩颁簡杩欎簺瀵硅薄瀹氫箟鐨勮涔変互鍙婂畠浠浣曠浉浜掑叧鑱斻€?
鏈€閲嶈鐨勬槸锛屾墍瀹氫箟鐨勫鐞嗗櫒灞傛缁撴瀯杩樺畾涔変簡骞冲彴鍙敤鐨勪綆鍔熻€楃┖闂茬姸鎬侊紝浠ュ強纭畾鍝簺澶勭悊鍣ㄥ彲浠?鎵撳紑鎴栧叧闂強鍏舵帶鍒舵潯浠剁殑瑙勫垯銆傛病鏈夎繖浜涗俊鎭紝澶勭悊鍣ㄥ皢杩愯鍦?UEFI 灏嗗叾鐣欏湪鐨勪换鎰忕數婧愮姸鎬佷腑銆?
杩樿娉ㄦ剰锛屾墍瀹氫箟鐨勫鐞嗗櫒 Device 瀵硅薄涓?MADT 涓?GIC 鐨勬潯鐩簲褰撳悓姝ャ€侱evice 瀵硅薄鐨?_UID 蹇呴』
瀵瑰簲浜?MADT 涓娇鐢ㄧ殑澶勭悊鍣?ID銆?
寤鸿鍦?arm64 涓婁娇鐢?CPPC锛?.4.5锛変綔涓哄鐞嗗櫒鎬ц兘鎺у埗鐨勪富瑕佹ā鍨嬨€侰-states 鍜?P-states 鍙兘鍦?灏嗘潵鐨勬煇涓椂鍊欏彉寰楀彲鐢紝浣嗙洰鍓嶅ぇ澶氭暟璁捐宸ヤ綔浼间箮鍊惧悜浜?CPPC銆?
姝ゅ锛孉RMv8 SoC 蹇呴』鎻愪緵鍔熻兘瀹屾暣鐨?PSCI 瀹炵幇锛涜繖灏嗘槸 ACPI 鏀寔鐨勭敤浜庢帶鍒?CPU 鐢垫簮鐘舵€佺殑鍞竴
鏈哄埗銆備娇鐢?ACPI parking 鍗忚鍚姩杈呭姪 CPU 鏄彲鑳界殑锛屼絾涓嶆帹鑽愶紝鍥犱负 ARM 鏈嶅姟鍣ㄤ粎鏀寔 PSCI銆?

### ACPI 绯荤粺鍦板潃鏄犲皠鎺ュ彛

鍦?ACPI 瑙勮寖绗?15 鑺備腑锛屾彁鍒颁簡鍑犵浣滀负鍚戝唴鏍镐紶閫掑唴瀛樿祫婧愪俊鎭殑鍙兘鏈哄埗鐨勬柟娉曘€傚浜?arm64锛?鎴戜滑灏嗗彧鏀寔浣跨敤 UEFI 閫氳繃 ACPI 鍚姩锛屽洜姝?UEFI GetMemoryMap() 鍚姩鏈嶅姟灏嗘槸鍞竴浣跨敤鐨勬満鍒躲€?

### ACPI 骞冲彴閿欒鎺ュ彛锛圓PEI锛?
涓婇潰宸叉弿杩版墍鏀寔鐨?APEI 琛ㄣ€?
APEI 鍦?ARMv8 涓婇渶瑕佺瓑鍚屼簬 SCI 鍜?NMI 鐨勬満鍒躲€係CI 鐢ㄤ簬閫氱煡 OSPM 宸茬粡鍙戠敓浣嗗彲浠ョ籂姝ｇ殑閿欒锛岀郴缁?鍙互缁х画姝ｇ‘杩愯锛屽嵆浣垮彲鑳芥湁鎵€闄嶇骇銆侼MI 鐢ㄤ簬鎸囩ず鏃犳硶绾犳鐨勮嚧鍛介敊璇紝闇€瑕佺珛鍗冲鐞嗐€?
鐢变簬娌℃湁鐩存帴绛夊悓浜?x86 SCI 鎴?NMI 鐨勬満鍒讹紝arm64 鐨勫鐞嗘柟寮忕暐鏈変笉鍚屻€係CI 浣滀负楂樹紭鍏堢骇涓柇澶勭悊锛?閴翠簬鎶ュ憡鐨勬槸宸茬籂姝ｏ紙鎴栧彲绾犳锛夌殑閿欒锛岃繖宸茬粡瓒冲銆侼MI 琚ā鎷熶负鍙兘鐨勬渶楂樹紭鍏堢骇涓柇銆傝繖鎰忓懗鐫€
蹇呴』淇濇寔涓€瀹氳皑鎱庯紝鍥犱负鍙兘瀛樺湪鏇撮珮鐗规潈绾х殑涓柇锛岀敋鑷冲瓨鍦ㄤ笌妯℃嫙 NMI 鍚屼紭鍏堢骇鐨勪腑鏂€傚湪 Linux 涓紝
涓嶅簲鍑虹幇杩欑鎯呭喌锛屼絾搴旀剰璇嗗埌瀹冨彲鑳藉彂鐢熴€?

### ARM64 涓婁笉鏀寔鐨?ACPI 瀵硅薄

铏界劧杩欏湪灏嗘潵鍙兘鏀瑰彉锛屼絾鏈夊嚑绫诲璞″彲浠ュ畾涔夛紝浣嗙洰鍓嶅 ARM 鏈嶅姟鍣ㄦ病鏈夋櫘閬嶆剰涔夈€傚叾涓竴浜涘璞℃湁 x86
瀵瑰簲鐗╋紝骞朵笖鍙兘鍦?ARM 鏈嶅姟鍣ㄤ笂纭疄鏈夐亾鐞嗐€備絾鏄紝鐩墠瑕佷箞娌℃湁鍙敤鐨勭‖浠讹紝瑕佷箞鐢氳嚦鍙兘杩樻病鏈夐潪
ARM 鐨勫疄鐜般€傚洜姝わ紝鐩墠涓嶆敮鎸佸畠浠€?
浠ヤ笅绫诲埆鐨勫璞′笉鍙楁敮鎸侊細

       - 绗?9.2 鑺傦細鐜鍏変紶鎰熷櫒璁惧

       - 绗?9.3 鑺傦細鐢垫睜璁惧

       - 绗?9.4 鑺傦細鐩栵紙渚嬪绗旇鏈洊锛?
       - 绗?9.8.2 鑺傦細IDE 鎺у埗鍣?
       - 绗?9.9 鑺傦細杞洏鎺у埗鍣?
       - 绗?9.10 鑺傦細GPE 鍧楄澶?
       - 绗?9.15 鑺傦細PC/AT RTC/CMOS 璁惧

       - 绗?9.16 鑺傦細鐢ㄦ埛瀛樺湪妫€娴嬭澶?
       - 绗?9.17 鑺傦細I/O APIC 璁惧锛涙墍鏈?GIC 閮藉繀椤诲彲閫氳繃 MADT 鏋氫妇

       - 绗?9.18 鑺傦細鏃堕棿鍜岄椆閽熻澶囷紙瑙?9.15锛?
       - 绗?10 鑺傦細鐢垫簮鍜屽姛鐜囪璁惧

       - 绗?11 鑺傦細鐑鐞?
       - 绗?12 鑺傦細宓屽叆寮忔帶鍒跺櫒鎺ュ彛

       - 绗?13 鑺傦細SMBus 鎺ュ彛


杩欎篃鎰忓懗鐫€浠ヤ笅瀵硅薄涓嶅彈鏀寔锛?
====   =========================== ====   ==========
Name   Section                     Name   Section
====   =========================== ====   ==========
_ALC   9.3.4                       _FDM   9.10.3
_ALI   9.3.2                       _FIX   6.2.7
_ALP   9.3.6                       _GAI   10.4.5
_ALR   9.3.5                       _GHL   10.4.7
_ALT   9.3.3                       _GTM   9.9.2.1.1
_BCT   10.2.2.10                   _LID   9.5.1
_BDN   6.5.3                       _PAI   10.4.4
_BIF   10.2.2.1                    _PCL   10.3.2
_BIX   10.2.2.1                    _PIF   10.3.3
_BLT   9.2.3                       _PMC   10.4.1
_BMA   10.2.2.4                    _PMD   10.4.8
_BMC   10.2.2.12                   _PMM   10.4.3
_BMD   10.2.2.11                   _PRL   10.3.4
_BMS   10.2.2.5                    _PSR   10.3.1
_BST   10.2.2.6                    _PTP   10.4.2
_BTH   10.2.2.7                    _SBS   10.1.3
_BTM   10.2.2.9                    _SHL   10.4.6
_BTP   10.2.2.8                    _STM   9.9.2.1.1
_DCK   6.5.2                       _UPD   9.16.1
_EC    12.12                       _UPP   9.16.2
_FDE   9.10.1                      _WPC   10.5.2
_FDI   9.10.2                      _WPP   10.5.3
====   =========================== ====   ==========
