
## Linux HP WMI 浼犳劅鍣ㄩ┍鍔?

:Copyright: |copy| 2023 James Seo <james@equiv.tech>

## 鎻忚堪


鎯犳櫘锛堜互鍙婇儴鍒?HP Compaq锛夊晢鍔＄骇璁＄畻鏈洪€氳繃 Windows Management Instrumentation锛圵MI锛夋姤鍛婄‖浠剁洃鎺т俊鎭€傝椹卞姩灏嗚繖浜涗俊鎭毚闇茬粰 Linux hwmon 瀛愮郴缁燂紝鍏佽鍍?`sensors` 杩欐牱鐨勭敤鎴风┖闂村伐鍏锋敹闆嗘暟鍊间紶鎰熷櫒璇绘暟銆?
## sysfs 鎺ュ彛


褰撻┍鍔ㄥ姞杞芥椂锛屽畠浼氬彂鐜扮郴缁熶笂鍙敤鐨勪紶鎰熷櫒锛屽苟鍦ㄥ繀瑕佹椂鍦?`/sys/class/hwmon/hwmon[X]` 涓垱寤轰互涓?sysfs 灞炴€э細

锛坄[X]` 鏄彇鍐充簬鍏朵粬绯荤粺缁勪欢鐨勬煇涓暟瀛椼€傦級

======================= ======= ===================================
Name                    Perm    鎻忚堪
======================= ======= ===================================
`curr[X]_input`       RO      鐢垫祦锛屽崟浣嶆瀹夛紙mA锛夈€?`curr[X]_label`       RO      鐢垫祦浼犳劅鍣ㄦ爣绛俱€?`fan[X]_input`        RO      椋庢墖杞€燂紝鍗曚綅 RPM銆?`fan[X]_label`        RO      椋庢墖浼犳劅鍣ㄦ爣绛俱€?`fan[X]_fault`        RO      椋庢墖浼犳劅鍣ㄦ晠闅滄寚绀哄櫒銆?`fan[X]_alarm`        RO      椋庢墖浼犳劅鍣ㄦ姤璀︽寚绀哄櫒銆?`in[X]_input`         RO      鐢靛帇锛屽崟浣嶆浼忥紙mV锛夈€?`in[X]_label`         RO      鐢靛帇浼犳劅鍣ㄦ爣绛俱€?`temp[X]_input`       RO      娓╁害锛屽崟浣嶆鎽勬皬搴?                               锛坢\ |deg|\ C锛夈€?`temp[X]_label`       RO      娓╁害浼犳劅鍣ㄦ爣绛俱€?`temp[X]_fault`       RO      娓╁害浼犳劅鍣ㄦ晠闅滄寚绀哄櫒銆?`temp[X]_alarm`       RO      娓╁害浼犳劅鍣ㄦ姤璀︽寚绀哄櫒銆?`intrusion[X]_alarm`  RW      鏈虹鍏ヤ镜鎶ヨ鎸囩ず鍣ㄣ€?======================= ======= ===================================

`fault` 灞炴€?  璇诲彇鍒?`fault` 灞炴€х殑鍊间负 `1` 鑰岄潪 `0`锛岃〃绀鸿浼犳劅鍣ㄥ湪杩愯杩囩▼涓亣鍒颁簡鏌愪簺闂锛屽洜姝ゅ叾娴嬮噺鍊间笉搴旇淇′换銆傚鏋滃浜庢晠闅滅姸鎬佺殑浼犳劅鍣ㄥ悗鏉ユ仮澶嶏紝鍐嶆璇诲彇璇ュ睘鎬у皢閲嶆柊杩斿洖 `0`銆?
`alarm` 灞炴€?  璇诲彇鍒?`alarm` 灞炴€х殑鍊间负 `1` 鑰岄潪 `0`锛岃〃绀烘牴鎹叾绫诲瀷锛屽彂鐢熶簡浠ヤ笅涔嬩竴锛?
  - `fan`锛氶鎵囧湪杩愯浆鏃跺凡鍋滆浆鎴栨柇寮€杩炴帴銆?  - `temp`锛氫紶鎰熷櫒璇绘暟宸茶揪鍒颁复鐣岄槇鍊笺€傚叿浣撶殑闃堝€煎彇鍐充簬绯荤粺銆?  - `intrusion`锛氱郴缁熸満绠辫鎵撳紑銆?
  璇诲彇鍒?`alarm` 灞炴€х殑 `1` 鍚庯紝璇ュ睘鎬т細鑷澶嶄綅锛屽苟鍦ㄥ悗缁鍙栨椂杩斿洖 `0`銆備綔涓轰緥澶栵紝`intrusion[X]_alarm` 鍙兘閫氳繃鍚戝畠鍐欏叆 `0` 鏉ユ墜鍔ㄥ浣嶃€?
## debugfs 鎺ュ彛


             骞朵笖浠呭湪鍐呮牳缂栬瘧鏃跺畾涔変簡 `CONFIG_DEBUG_FS` 鏃舵墠鍙敤銆?
sysfs 涓殑鏍囧噯 hwmon 鎺ュ彛鏆撮湶浜嗗湪椹卞姩鍒濆鍖栨椂杩炴帴鐨勫嚑绉嶅父瑙佺被鍨嬬殑浼犳劅鍣ㄣ€傜劧鑰岋紝WMI 涓€氬父杩樻湁鍏朵粬涓嶇鍚堣繖浜涙潯浠剁殑浼犳劅鍣ㄣ€傛澶栵紝鍙兘杩樺瓨鍦ㄤ竴浜涚郴缁熺浉鍏崇殑銆佺敤浜?`alarm` 灞炴€х殑鈥滃钩鍙颁簨浠跺璞★紙platform events objects锛夆€濄€傚洜姝ゆ彁渚涗簡涓€涓?debugfs 鎺ュ彛锛岀敤浜庡彧璇昏闂墍鏈夊彲鐢ㄧ殑 HP WMI 浼犳劅鍣ㄥ拰骞冲彴浜嬩欢瀵硅薄銆?
`/sys/kernel/debug/hp-wmi-sensors-[X]/sensor`
涓烘瘡涓紶鎰熷櫒鍖呭惈涓€涓甫缂栧彿鐨勬潯鐩紝鍏锋湁浠ヤ笅灞炴€э細

=============================== =======================================
Name                            Example
=============================== =======================================
`name`                        `CPU0 Fan`
`description`                 `Reports CPU0 fan speed`
`sensor_type`                 `12`
`other_sensor_type`           锛堢┖瀛楃涓诧級
`operational_status`          `2`
`possible_states`             `Normal,Caution,Critical,Not Present`
`current_state`               `Normal`
`base_units`                  `19`
`unit_modifier`               `0`
`current_reading`             `1008`
`rate_units`                  `0`锛堜粎瀛樺湪浜庢煇浜涚郴缁熶笂锛?=============================== =======================================

濡傛灉骞冲彴浜嬩欢瀵硅薄鍙敤锛?`/sys/kernel/debug/hp-wmi-sensors-[X]/platform_events`
涓烘瘡涓璞″寘鍚竴涓甫缂栧彿鐨勬潯鐩紝鍏锋湁浠ヤ笅灞炴€э細

=============================== ====================
Name                            Example
=============================== ====================
`name`                        `CPU0 Fan Stall`
`description`                 `CPU0 Fan Speed`
`source_namespace`            `root\wmi`
`source_class`                `HPBIOS_BIOSEvent`
`category`                    `3`
`possible_severity`           `25`
`possible_status`             `5`
=============================== ====================

杩欎簺浠ｈ〃浜嗗簳灞?`HPBIOS_BIOSNumericSensor` 鍜?`HPBIOS_PlatformEvents` WMI 瀵硅薄鐨勫睘鎬э紝瀹冧滑鍦ㄤ笉鍚岀郴缁熶箣闂存湁鎵€宸紓銆?鏇村缁嗚妭鍜屾墭绠″璞℃牸寮忥紙MOF锛夊畾涔夎鍙傝 [#]_銆?
## 宸茬煡闂涓庨檺鍒?

- 濡傛灉閽堝闈炲晢鍔＄骇 HP 绯荤粺鐨勭幇鏈?hp-wmi 椹卞姩宸茬粡鍔犺浇锛岄偅涔堝嵆浣垮湪涓嶆敮鎸佽繖浜涘睘鎬х殑绯荤粺涓婏紝`alarm` 灞炴€т篃灏嗕笉鍙敤銆傝繖鏄洜涓鸿椹卞姩鐢ㄤ簬 `alarm` 灞炴€х殑鍚屼竴涓?WMI 浜嬩欢 GUID 鍦ㄨ繖浜涚郴缁熶笂琚敤浜庝緥濡傜瑪璁版湰鐑敭銆?- 宸茶瀵熷埌鍙枒鐨勪紶鎰熷櫒纭欢鍜屼笉涓€鑷寸殑 BIOS WMI 瀹炵幇浼氬鑷翠笉鍑嗙‘鐨勮鏁板拰寮傚父琛屼负锛屼緥濡傛姤璀︿笉鍙戠敓鎴栨瘡娆″惎鍔ㄥ彧鍙戠敓涓€娆°€?- 杩勪粖涓烘鍦ㄧ幇瀹炰腑鍙杩囨俯搴︺€侀鎵囪浆閫熷拰鍏ヤ镜杩欏嚑绉嶄紶鎰熷櫒绫诲瀷銆傚洜姝ゅ鐢靛帇鍜岀數娴佷紶鎰熷櫒鐨勬敮鎸佹槸鏆傚畾鐨勩€?- 灏界 HP WMI 浼犳劅鍣ㄥ彲鑳藉０绉版槸浠讳綍绫诲瀷锛屼絾 hwmon 涓嶈璇嗙殑浠讳綍濂囨€紶鎰熷櫒绫诲瀷灏嗕笉鍙楁敮鎸併€?
## 鍙傝€冭祫鏂?

       鈥淗P Client Management Interface Technical White Paper鈥濓紝2005銆?[Online].
       Available: https://h20331.www2.hp.com/hpsub/downloads/cmi_whitepaper.pdf
