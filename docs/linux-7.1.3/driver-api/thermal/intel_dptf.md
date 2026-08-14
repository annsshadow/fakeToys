
## Intel(R) Dynamic Platform and Thermal Framework Sysfs 鎺ュ彛


:鐗堟潈: 漏 2022 Intel Corporation

:浣滆€? Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>

### 绠€浠?


Intel(R) Dynamic Platform and Thermal Framework锛圖PTF锛屽姩鎬佸钩鍙颁笌鐑鐞嗘鏋讹級鏄竴涓敤浜庣數婧愪笌鐑鐞嗙殑骞冲彴绾х‖浠?杞欢瑙ｅ喅鏂规銆?

浣滀负涓€涓绾冲绉嶇數婧?鐑鐞嗘妧鏈殑瀹瑰櫒锛孌PTF 涓轰笉鍚岀瓥鐣ュ奖鍝嶇郴缁熺‖浠剁姸鎬佹彁渚涗簡涓€绉嶅崗璋冪殑鏂瑰紡銆?

鐢变簬瀹冩槸涓€涓钩鍙扮骇妗嗘灦锛屽洜姝ゅ寘鍚涓粍浠躲€傝鎶€鏈殑閮ㄥ垎鍐呭瀹炵幇鍦ㄥ浐浠朵腑锛屽苟浣跨敤 ACPI 涓?PCI 璁惧鏉ユ毚闇插悇绉嶇洃鎺т笌鎺у埗鍔熻兘銆侺inux 鎷ユ湁涓€缁勫唴鏍稿鐢ㄦ埛绌洪棿鏆撮湶纭欢鎺ュ彛鐨勯┍鍔ㄣ€傝繖浣垮緱璇稿 鈥淟inux Thermal Daemon鈥?涔嬬被鐨勭敤鎴风┖闂寸儹绠＄悊鏂规鑳藉璇诲彇骞冲彴鐗瑰畾鐨勭儹涓庣數婧愯〃锛屼粠鑰屽湪灏嗙郴缁熶繚鎸佸湪鐑檺鍒惰寖鍥村唴鐨勫悓鏃舵彁渚涘厖瓒崇殑鎬ц兘銆?

### DPTF ACPI 椹卞姩鎺ュ彛


`/sys/bus/platform/devices/<N>/uuids`锛屽叾涓?<N>
=INT3400|INTC1040|INTC1041|INTC10A0

`available_uuids` (RO)
	涓€缁?UUID 瀛楃涓诧紝琛ㄧず鍙敤鐨勭瓥鐣ャ€傚綋
	鐢ㄦ埛绌洪棿鑳藉鏀寔杩欎簺绛栫暐鏃讹紝搴斿皢鍏堕€氱煡缁欏浐浠躲€?

	UUID 瀛楃涓诧細

	"42A441D6-AE6A-462b-A84B-4A8CE79027D3" : 琚姩 1锛圥assive 1锛?

	"3A95C389-E4B8-4629-A526-C52C88626BAE" : 涓诲姩锛圓ctive锛?

	"97C68AE7-15FA-499c-B8C9-5DA81D606E0A" : 涓ラ噸锛圕ritical锛?

	"63BE270F-1C11-48FD-A6F7-3AF253FF3E2D" : 鑷€傚簲鎬ц兘锛圓daptive performance锛?

	"5349962F-71E6-431D-9AE8-0A635B710AEE" : 绱ф€ュ懠鍙紙Emergency call锛?

	"9E04115A-AE87-4D1C-9500-0F3E340BFE75" : 琚姩 2锛圥assive 2锛?

	"F5A35014-C209-46A4-993A-EB56DE7530A1" : Power Boss

	"6ED722A7-9240-48A5-B479-31EEF723D7CF" : 铏氭嫙浼犳劅鍣紙Virtual Sensor锛?

	"16CAF1B7-DD38-40ED-B1C1-1B8A1913D531" : 鏁ｇ儹妯″紡锛圕ooling mode锛?

	"BE84BABF-C4D4-403D-B495-3128FD44dAC1" : HDC

`current_uuid` (RW)
	鐢ㄦ埛绌洪棿鍙互涓€娆′竴涓湴鍐欏叆鏉ヨ嚜鍙敤 UUID 鐨勫瓧绗︿覆銆?

`/sys/bus/platform/devices/<N>/`锛屽叾涓?<N>
=INT3400|INTC1040|INTC1041|INTC10A0

`imok` (WO)
	鐢ㄦ埛绌洪棿瀹堟姢杩涚▼鍐欏叆 1 浠ュ搷搴斿浐浠剁殑鍙戦€佷繚娲伙紙keep alive锛夐€氱煡浜嬩欢銆傚綋鍥轰欢璋冪敤 imok ACPI 鏂规硶瑕佹眰鐢ㄦ埛绌洪棿鍝嶅簲鏃讹紝鐢ㄦ埛绌洪棿浼氭敹鍒?
	THERMAL_EVENT_KEEP_ALIVE kobject uevent 閫氱煡銆?

`odvp*` (RO)
	鍥轰欢鐑姸鎬佸彉閲忓€笺€傜儹琛ㄤ細鏍规嵁杩欎簺鍙橀噺鍊肩殑涓嶅悓杩涜涓嶅悓鐨勫鐞嗐€?

`data_vault` (RO)
	浜岃繘鍒剁儹琛ㄣ€傝В鐮佺儹琛ㄨ鍙傞槄
	https:/github.com/intel/thermal_daemon銆?

`production_mode` (RO)
	褰撲笉涓洪浂鏃讹紝鍒堕€犲晢閿佸畾浜嗙儹閰嶇疆锛岀姝㈣繘涓€姝ユ洿鏀广€?

### ACPI 鐑叧绯昏〃鎺ュ彛


`/dev/acpi_thermal_rel`

	璇ヨ澶囨彁渚?IOCTL 鎺ュ彛锛岄€氳繃 ACPI 鏂规硶 _TRT 涓?_ART 璇诲彇鏍囧噯 ACPI 鐑叧绯昏〃銆傝繖浜?IOCTL 瀹氫箟鍦?
	drivers/thermal/intel/int340x_thermal/acpi_thermal_rel.h 涓?

	IOCTL锛?

	ACPI_THERMAL_GET_TRT_LEN: 鑾峰彇 TRT 琛ㄧ殑闀垮害

	ACPI_THERMAL_GET_ART_LEN: 鑾峰彇 ART 琛ㄧ殑闀垮害

	ACPI_THERMAL_GET_TRT_COUNT: TRT 琛ㄤ腑鐨勮褰曟暟

	ACPI_THERMAL_GET_ART_COUNT: ART 琛ㄤ腑鐨勮褰曟暟

	ACPI_THERMAL_GET_TRT: 璇诲彇浜岃繘鍒?TRT 琛紝璇诲彇闀垮害閫氳繃 ioctl() 鐨勫弬鏁版彁渚涖€?

	ACPI_THERMAL_GET_ART: 璇诲彇浜岃繘鍒?ART 琛紝璇诲彇闀垮害閫氳繃 ioctl() 鐨勫弬鏁版彁渚涖€?

### DPTF ACPI 浼犳劅鍣ㄩ┍鍔?


DPTF 浼犳劅鍣ㄩ┍鍔ㄤ互鏍囧噯鐨勭儹绠＄悊 sysfs thermal_zone 褰㈠紡鍛堢幇銆?


### DPTF ACPI 鏁ｇ儹椹卞姩


DPTF 鏁ｇ儹椹卞姩浠ユ爣鍑嗙殑鐑鐞?sysfs cooling_device 褰㈠紡鍛堢幇銆?


### DPTF 澶勭悊鍣ㄧ儹绠＄悊 PCI 椹卞姩鎺ュ彛


`/sys/bus/pci/devices/0000\:00\:04.0/power_limits/`

鏈夊叧 powercap ABI 璇峰弬闃?Documentation/power/powercap/powercap.rst銆?

`power_limit_0_max_uw` (RO)
	Intel RAPL 鐨?powercap sysfs constraint_0_power_limit_uw 鐨勬渶澶у€?

`power_limit_0_step_uw` (RO)
	Intel RAPL 绾︽潫 0 鍔熺巼闄愬埗鐨勫姛鐜囧閲?鍑忛噺

`power_limit_0_min_uw` (RO)
	Intel RAPL 鐨?powercap sysfs constraint_0_power_limit_uw 鐨勬渶灏忓€?

`power_limit_0_tmin_us` (RO)
	Intel RAPL 鐨?powercap sysfs constraint_0_time_window_us 鐨勬渶灏忓€?

`power_limit_0_tmax_us` (RO)
	Intel RAPL 鐨?powercap sysfs constraint_0_time_window_us 鐨勬渶澶у€?

`power_limit_1_max_uw` (RO)
	Intel RAPL 鐨?powercap sysfs constraint_1_power_limit_uw 鐨勬渶澶у€?

`power_limit_1_step_uw` (RO)
	Intel RAPL 绾︽潫 1 鍔熺巼闄愬埗鐨勫姛鐜囧閲?鍑忛噺

`power_limit_1_min_uw` (RO)
	Intel RAPL 鐨?powercap sysfs constraint_1_power_limit_uw 鐨勬渶灏忓€?

`power_limit_1_tmin_us` (RO)
	Intel RAPL 鐨?powercap sysfs constraint_1_time_window_us 鐨勬渶灏忓€?

`power_limit_1_tmax_us` (RO)
	Intel RAPL 鐨?powercap sysfs constraint_1_time_window_us 鐨勬渶澶у€?

`power_floor_status` (RO)
	褰撶疆涓?1 鏃讹紝琛ㄧず褰撳墠閰嶇疆涓嬪凡杈惧埌绯荤粺鐨勫姛鐜囦笅闄愩€傞渶瑕侀噸鏂伴厤缃墠鑳借繘涓€姝ラ檷浣庡姛鐜囥€?

`power_floor_enable` (RW)
	褰撶疆涓?1 鏃讹紝鍚敤鍔熺巼涓嬮檺鐘舵€佺殑璇诲彇涓庨€氱煡銆傚綋 power_floor_status 灞炴€у€煎彂鐢熷彉鍖栨椂浼氳Е鍙戦€氱煡銆?

`/sys/bus/pci/devices/0000\:00\:04.0/`

`tcc_offset_degree_celsius` (RW)
	纭欢灏嗛檺鍒?CPU 鐨勪复鐣屾俯搴︿箣涓婄殑 TCC 鍋忕Щ閲忋€?

`/sys/bus/pci/devices/0000\:00\:04.0/workload_request`

`workload_available_types` (RO)
	鍙敤鐨勫伐浣滆礋杞界被鍨嬨€傜敤鎴风┖闂村彲浠ラ€氳繃 workload_type 鎸囧畾瀹冨綋鍓嶆鍦ㄦ墽琛岀殑鏌愪竴宸ヤ綔璐熻浇绫诲瀷銆備緥濡傦細idle锛堢┖闂诧級銆乥ursty锛堢獊鍙戯級銆乻ustained锛堟寔缁級绛夈€?

`workload_type` (RW)
	鐢ㄦ埛绌洪棿鍙互閫氳繃姝ゆ帴鍙ｆ寚瀹氫换鎰忎竴涓彲鐢ㄧ殑宸ヤ綔璐熻浇绫诲瀷銆?

`/sys/bus/pci/devices/0000\:00\:04.0/ptc_0_control`
`/sys/bus/pci/devices/0000\:00\:04.0/ptc_1_control`
`/sys/bus/pci/devices/0000\:00\:04.0/ptc_2_control`

鎵€鏈夎繖浜涙帶鍒堕兘闇€瑕佺鐞嗗憳鏉冮檺鎵嶈兘鏇存柊銆?

`enable` (RW)
	1 琛ㄧず鍚敤锛? 琛ㄧず绂佺敤銆傛樉绀哄钩鍙版俯搴︽帶鍒跺姛鑳藉綋鍓嶇殑鍚敤鐘舵€併€傜敤鎴风┖闂村彲浠ュ惎鐢?绂佺敤纭欢鎺у埗銆?

`temperature_target` (RW)
	鏇存柊纭欢鐢ㄤ簬娓╁害鎺у埗鐨勬柊娓╁害鐩爣锛屽崟浣嶄负姣憚姘忓害銆?

`thermal_tolerance` (RW)
	璇ュ睘鎬у彇鍊艰寖鍥翠负 0 鍒?7锛屽叾涓?0 琛ㄧず鏈€婵€杩涚殑鎺у埗浠ラ伩鍏嶄换浣曟俯搴﹁秴璋冿紝7 琛ㄧず鏇村钩缂撶殑鏂瑰紡锛屽嵆渚夸互娓╁害瓒呰皟涓轰唬浠蜂篃鍋忓悜鎬ц兘銆?
	娉ㄦ剰锛氳绾у埆鍙兘骞堕潪绾挎€х缉鏀俱€備緥濡傦紝鍊?3 骞朵笉涓€瀹氭剰鍛崇潃鐩告瘮鍊?0 鏈?50% 鐨勬€ц兘鎻愬崌銆?

閴翠簬杩欐槸骞冲彴娓╁害鎺у埗锛屾湡鏈涚敱鍗曚竴鐨勭敤鎴风骇绠＄悊鍣ㄦ嫢鏈夊苟绠＄悊杩欎簺鎺у埗銆傚鏋滃涓敤鎴风骇杞欢搴旂敤灏濊瘯鍐欏叆涓嶅悓鐨勭洰鏍囷紝鍙兘瀵艰嚧闈為鏈熺殑琛屼负銆?


### DPTF 澶勭悊鍣ㄧ儹绠＄悊 RFIM 鎺ュ彛


RFIM 鎺ュ彛鍏佽璋冩暣 FIVR锛堝叏闆嗘垚鐢靛帇璋冭妭鍣級銆丏DR锛堝弻鍊嶆暟鎹€熺巼锛変笌 DLVR锛堟暟瀛楃嚎鎬х數鍘嬭皟鑺傚櫒锛夌殑棰戠巼锛屼互閬垮厤瀵?WiFi 涓?5G 鐨勫皠棰戝共鎵般€?

寮€鍏崇數鍘嬭皟鑺傚櫒锛圴R锛変細鍦ㄥ熀棰戝強鍏惰皭娉㈠浜х敓杈愬皠 EMI 鎴?RFI銆傛煇浜涜皭娉㈠彲鑳戒細骞叉壈闆嗘垚鍒扮瑪璁版湰绛変富鏈虹郴缁熶腑闈炲父鏁忔劅鐨勬棤绾挎帴鏀跺櫒锛屽 Wi-Fi 涓庤渹绐濈綉缁溿€傜紦瑙ｆ柟娉曚箣涓€鏄姹傚皢 SoC 闆嗘垚鐨?VR锛圛VR锛夊紑鍏抽鐜囪皟鏁翠竴涓皬鐨勭櫨鍒嗘瘮锛屽苟灏嗗紑鍏冲櫔澹扮殑璋愭尝骞叉壈浠庢棤绾夸俊閬撶Щ寮€銆侽EM 鎴?ODM 鍙互浣跨敤璇ラ┍鍔ㄥ湪涓嶄細褰卞搷 IVR 鎬ц兘鐨勮寖鍥村唴鎺у埗 SoC IVR 鐨勮繍琛屻€?

鏌愪簺浜у搧浣跨敤 DLVR 鑰岄潪 FIVR 浣滀负寮€鍏崇數鍘嬭皟鑺傚櫒銆傚湪杩欑鎯呭喌涓嬶紝蹇呴』璋冩暣 DLVR 鐨勫睘鎬ц€岄潪 FIVR銆?

鍦ㄧЩ鍔ㄩ鐜囨椂鍙兘浼氬紩鍏ラ澶栫殑鏃堕挓鍣０锛岃繖鍙互閫氳繃璋冩暣鎵╅鐧惧垎姣旀潵琛ュ伩銆傝繖鏈夊姪浜庨檷浣庢椂閽熷櫔澹颁互婊¤冻娉曡鍚堣瑕佹眰銆傝鎵╅鐧惧垎姣斿鍔犱簡淇″彿浼犺緭鐨勫甫瀹斤紝浠庤€屽噺灏戝共鎵般€佸櫔澹颁笌淇″彿琛拌惤鐨勫奖鍝嶃€?

DDR IO 鎺ュ彛鐨?DRAM 璁惧鍙婂叾鐢垫簮骞抽潰鍙兘鍦ㄦ暟鎹€熺巼澶勪骇鐢?EMI銆備笌 IVR 鎺у埗鏈哄埗绫讳技锛孖ntel 鎻愪緵浜嗕竴绉嶆満鍒讹紝鍦ㄦ弧瓒宠嫢骞叉潯浠舵椂鏀瑰彉 DDR 鏁版嵁閫熺巼锛氱敱浜?DDR 瀛樺湪寮虹儓鐨?RFI 骞叉壈锛汣PU 鐢垫簮绠＄悊鍦ㄦ敼鍙?DDR 鏁版嵁閫熺巼鏂归潰娌℃湁鍏朵粬闄愬埗锛汸C ODM 鍦?BIOS 涓负姝ゅ惎鐢ㄤ簡璇ョ壒鎬э紙瀹炴椂 DDR RFI 缂撹В锛岀О涓?DDR-RFIM锛変互鐢ㄤ簬 Wi-Fi銆?


FIVR 灞炴€?

`/sys/bus/pci/devices/0000\:00\:04.0/fivr/`

`vco_ref_code_lo` (RW)
	VCO 鍙傝€冪爜鏄竴涓?11 浣嶅瓧娈碉紝鎺у埗 FIVR 鐨勫紑鍏抽鐜囥€傝繖鏄?3 浣嶇殑浣庡瓧鑺傦紙LSB锛夊瓧娈点€?

`vco_ref_code_hi` (RW)
	VCO 鍙傝€冪爜鏄竴涓?11 浣嶅瓧娈碉紝鎺у埗 FIVR 鐨勫紑鍏抽鐜囥€傝繖鏄?8 浣嶇殑楂樺瓧鑺傦紙MSB锛夊瓧娈点€?

`spread_spectrum_pct` (RW)
	璁剧疆 FIVR 鎵╅鏃堕挓鐧惧垎姣?

`spread_spectrum_clk_enable` (RW)
	FIVR 鎵╅鏃堕挓鐗规€х殑鍚敤/绂佺敤

`rfi_vco_ref_code` (RW)
	璇ュ瓧娈垫槸涓€涓彧璇荤姸鎬佸瘎瀛樺櫒锛屽弽鏄犲綋鍓?FIVR 寮€鍏抽鐜?

`fivr_fffc_rev` (RW)
	璇ュ瓧娈垫寚绀?FIVR 纭欢鐨勪慨璁㈢増鏈€?


DVFS 灞炴€?

`/sys/bus/pci/devices/0000\:00\:04.0/dvfs/`

`rfi_restriction_run_busy` (RW)
	璇锋眰闄愬埗鐗瑰畾鐨?DDR 鏁版嵁閫熺巼锛屽苟灏嗘鍊肩疆涓?1銆傛搷浣滃畬鎴愬悗鑷姩澶嶄綅涓?0銆?

`rfi_restriction_err_code` (RW)
	0锛氳姹傝鎺ュ彈锛?锛氱壒鎬ц绂佺敤锛?
	2锛氳姹傞檺鍒剁殑鐐规暟瓒呰繃鍏佽鍊?

`rfi_restriction_data_rate_Delta` (RW)
	鐢ㄤ簬 RFI 淇濇姢鐨勫彈闄?DDR 鏁版嵁閫熺巼锛氫笅闄?

`rfi_restriction_data_rate_Base` (RW)
	鐢ㄤ簬 RFI 淇濇姢鐨勫彈闄?DDR 鏁版嵁閫熺巼锛氫笂闄?

`ddr_data_rate_point_0` (RO)
	DDR 鏁版嵁閫熺巼閫夋嫨绗?1 涓偣

`ddr_data_rate_point_1` (RO)
	DDR 鏁版嵁閫熺巼閫夋嫨绗?2 涓偣

`ddr_data_rate_point_2` (RO)
	DDR 鏁版嵁閫熺巼閫夋嫨绗?3 涓偣

`ddr_data_rate_point_3` (RO)
	DDR 鏁版嵁閫熺巼閫夋嫨绗?4 涓偣

`rfi_disable (RW)`
	绂佺敤 DDR 閫熺巼鏀瑰彉鐗规€?

DLVR 灞炴€?

`/sys/bus/pci/devices/0000\:00\:04.0/dlvr/`

`dlvr_hardware_rev` (RO)
	DLVR 纭欢淇鐗堟湰銆?

`dlvr_freq_mhz` (RO)
	褰撳墠 DLVR PLL 棰戠巼锛屽崟浣嶄负 MHz銆?

`dlvr_freq_select` (RW)
	璁剧疆 DLVR PLL 鏃堕挓棰戠巼銆備竴鏃﹁缃苟閫氳繃 dlvr_rfim_enable 鍚敤锛宒lvr_freq_mhz 灏嗘樉绀哄綋鍓?DLVR PLL 棰戠巼銆?

`dlvr_pll_busy` (RO)
	缃綅鏃?PLL 鏃犳硶鎺ュ彈棰戠巼鍙樻洿銆?

`dlvr_rfim_enable` (RW)
	0锛氱鐢ㄥ皠棰戣烦棰戯紝1锛氬惎鐢ㄥ皠棰戣烦棰戙€?

`dlvr_spread_spectrum_pct` (RW)
	璁剧疆 DLVR 鎵╅鐧惧垎姣斿€笺€?

`dlvr_control_mode` (RW)
        鎸囧畾浣跨敤鎵╅鏃堕鐜囧浣曞睍寮€銆?
        0锛氬悜涓嬪睍寮€锛圖own spread锛夛紝
        1锛氫腑蹇冨睍寮€锛圫pread in the Center锛夈€?

`dlvr_control_lock` (RW)
    1锛氬悗缁啓鍏ヨ蹇界暐銆?

### DPTF 鐢垫簮涓庣數姹犳帴鍙?


璇峰弬闃?Documentation/ABI/testing/sysfs-platform-dptf

### DPTF 椋庢墖鎺у埗


璇峰弬闃?Documentation/admin-guide/acpi/fan_performance_states.rst

### 宸ヤ綔璐熻浇绫诲瀷鎻愮ず


Meteor Lake 澶勭悊鍣ㄤ唬鐨勫浐浠惰兘澶熻瘑鍒伐浣滆礋杞界被鍨嬶紝骞跺皢鏈夊叧瀹冪殑鎻愮ず浼犻€掔粰 OS銆傛彁渚涗簡涓€涓壒娈婄殑 sysfs 鎺ュ彛锛屽厑璁哥敤鎴风┖闂翠粠鍥轰欢鑾峰彇宸ヤ綔璐熻浇绫诲瀷鎻愮ず锛屽苟鎺у埗鍏舵彁渚涚殑閫熺巼銆?

鐢ㄦ埛绌洪棿鍙互杞灞炴€?鈥渨orkload_type_index鈥?鑾峰彇褰撳墠鎻愮ず锛屼篃鍙互鍦ㄨ灞炴€у€兼洿鏂版椂鏀跺埌閫氱煡銆?

file:`/sys/bus/pci/devices/0000:00:04.0/workload_hint/`
娈?0銆佹€荤嚎 0銆佽澶?4銆佸姛鑳?0 鍦ㄦ墍鏈?Intel 瀹㈡埛绔鐞嗗櫒涓婇兘淇濈暀缁欏鐞嗗櫒鐑澶囥€傚洜姝わ紝涓婅堪璺緞涓嶄細闅忓鐞嗗櫒浠ｇ殑鏇磋凯鑰屾敼鍙樸€?

`workload_hint_enable` (RW)
	鍚敤鍥轰欢鍚戠敤鎴风┖闂村彂閫佸伐浣滆礋杞界被鍨嬫彁绀恒€?

`workload_slow_hint_enable` (RW)
	鍚敤鍥轰欢鍚戠敤鎴风┖闂村彂閫佹參閫熷伐浣滆礋杞界被鍨嬫彁绀恒€?

`notification_delay_ms` (RW)
	鍥轰欢閫氱煡 OS 涔嬪墠鐨勬渶灏忓欢杩燂紝鍗曚綅涓烘绉掋€傝繖鐢ㄤ簬鎺у埗閫氱煡鐨勯€熺巼銆傝寤惰繜浠嬩簬鍥轰欢鏀瑰彉宸ヤ綔璐熻浇绫诲瀷棰勬祴涓庡皢鏀瑰彉閫氱煡 OS 涔嬮棿銆傞粯璁ゅ欢杩熶负 1024 ms銆傚欢杩熶负 0 鏄棤鏁堢殑銆傚欢杩熶細琚悜涓婂彇鏁村埌鏈€鎺ヨ繎鐨?2 鐨勫箓锛屼互绠€鍖栧浐浠跺寤惰繜鍊肩殑缂栫▼銆傝鍙?notification_delay_ms 灞炴€т細鏄剧ず鎵€浣跨敤鐨勬湁鏁堝€笺€?

`workload_type_index` (RO)
	棰勬祴鐨勫伐浣滆礋杞界被鍨嬬储寮曘€傜敤鎴风┖闂村彲浠ラ€氳繃鐜版湁鐨?sysfs 灞炴€у彉鏇撮€氱煡鏈哄埗鑾峰緱鍙樻洿閫氱煡銆?

	Meteor Lake 澶勭悊鍣ㄤ唬鎵€鏀寔鐨勭储寮曞€煎強鍏跺惈涔夊涓嬶細

	0 -  绌洪棽锛圛dle锛夛細绯荤粺涓嶆墽琛屼换浣曚换鍔★紝鍔熻€椾笌绌洪棽椹荤暀鏃堕棿闀挎椂闂存寔缁亸浣庛€?

	1 鈥?鐢垫睜缁埅锛圔attery Life锛夛細鍔熻€楃浉瀵硅緝浣庯紝浣嗗鐞嗗櫒鍙兘浠嶅湪涓诲姩鎵ц浠诲姟锛屼緥濡傞暱鏃堕棿鐨勮棰戞挱鏀俱€?

	2 鈥?鎸佺画锛圫ustained锛夛細鍦ㄨ緝闀夸竴娈垫椂闂村唴鍔熻€楃浉瀵硅緝楂橈紝鍑犱箮娌℃湁绌洪棽鏃舵锛屾渶缁堜細鑰楀敖 RAPL Power Limit 1 涓?2銆?

	3 鈥?绐佸彂锛圔ursty锛夛細娑堣€楃浉瀵规亽瀹氱殑骞冲潎鍔熺巼锛屼絾鐩稿绌洪棽鐨勬椂娈典細琚獊鍙戞椿鍔ㄦ墦鏂€傜獊鍙戠浉瀵硅緝鐭紝鍏堕棿鐩稿绌洪棽鐨勬椂娈甸€氬父鑳介槻姝?RAPL Power Limit 1 琚€楀敖銆?

	4 鈥?鏈煡锛圲nknown锛夛細鏃犳硶鍒嗙被銆?

	浠?Panther Lake 寮€濮嬬殑澶勭悊鍣ㄦ彁渚涗簡棰濆鐨勬彁绀恒€傜‖浠跺湪杈冮暱涓€娈垫椂闂村唴鍒嗘瀽宸ヤ綔璐熻浇椹荤暀鎯呭喌锛屼互纭畾璇ュ伐浣滆礋杞藉垎绫诲€惧悜浜庣┖闂?鐢垫睜缁埅鐘舵€佽繕鏄寔缁?鎬ц兘鐘舵€併€傚熀浜庢闀挎湡鍒嗘瀽锛屽畠鍒嗙被濡備笅锛?

	鍔熻€楀垎绫伙紙Power Classification锛夛細濡傛灉宸ヤ綔璐熻浇琛ㄧ幇鍑烘洿澶氱殑绌洪棽鎴栫數姹犵画鑸┗鐣欙紝鍒欏綊绫讳负 鈥減ower鈥濓紙鍔熻€楋級銆?

	鎬ц兘鍒嗙被锛圥erformance Classification锛夛細濡傛灉宸ヤ綔璐熻浇琛ㄧ幇鍑烘洿澶氱殑鎸佺画鎴栨€ц兘椹荤暀锛屽垯褰掔被涓?鈥減erformance鈥濓紙鎬ц兘锛夈€?

	杩欑鏂瑰紡浣垮簲鐢ㄥ彲浠ュ拷鐣ョ煭鏈熺殑宸ヤ綔璐熻浇娉㈠姩锛岃浆鑰屽搷搴旈暱鏈熺殑鍔熻€椾笌鎬ц兘瓒嬪娍銆?

	璇ュ垎绫荤殑椹荤暀闃堝€兼槸 CPU 浠ｇ壒瀹氱殑銆傚垎绫婚€氳繃 workload_type_index 鐨勭 4 浣嶆姤鍛婏細

	绗?4 浣?= 1锛氬姛鑰楀垎绫伙紙Power classification锛?

	绗?4 浣?= 0锛氭€ц兘鍒嗙被锛圥erformance classification锛?
