
## GPMC锛圙eneral Purpose Memory Controller锛岄€氱敤鍐呭瓨鎺у埗鍣級


GPMC 鏄竴涓笓鐢ㄤ簬杩炴帴澶栭儴瀛樺偍璁惧锛堝浠ヤ笅鍚勭被锛夌殑缁熶竴鍐呭瓨鎺у埗鍣細

 - 寮傛 SRAM 绫诲瓨鍌ㄥ櫒浠ュ強涓撶敤闆嗘垚鐢佃矾锛圓SIC锛夎澶囥€? - 寮傛銆佸悓姝ヤ笌椤垫ā寮忕獊鍙戯紙burst锛塏OR 闂瓨璁惧
 - NAND 闂瓨
 - 浼?SRAM 璁惧

GPMC 瑙佷簬寰峰窞浠櫒锛圱exas Instruments锛夌殑 SoC锛堝熀浜?OMAP锛変笂
IP 缁嗚妭锛歨ttps://www.ti.com/lit/pdf/spruh73 绗?7.1 鑺?

## GPMC 閫氱敤鏃跺簭璁＄畻锛?

GPMC 鏈変竴浜涘繀椤荤紪绋嬬殑鏃跺簭锛屾墠鑳戒娇澶栬姝ｅ父宸ヤ綔锛岃€屽璁捐嚜韬張鏈夊彟涓€缁勬椂搴忋€傝璁╁璁句笌 gpmc 鍗忓悓宸ヤ綔锛屽繀椤绘妸澶栬鏃跺簭杞崲涓?gpmc 鑳界悊瑙ｇ殑褰㈠紡銆傝浆鎹㈡柟寮忓彇鍐充簬鎵€杩炴帴鐨勫璁俱€傛澶栵紝鏌愪簺 gpmc 鏃跺簭杩樹緷璧栦簬 gpmc 鏃堕挓棰戠巼銆傚洜姝ゅ紑鍙戜簡閫氱敤鏃跺簭渚嬬▼鏉ユ弧瓒充笂杩伴渶姹傘€?
閫氱敤渚嬬▼鎻愪緵浜嗕竴绉嶄粠 gpmc 澶栬鏃跺簭璁＄畻 gpmc 鏃跺簭鐨勯€氱敤鏂规硶銆俿truct gpmc_device_timings 鐨勫瓧娈靛繀椤荤敤杩炴帴鑷?gpmc 鐨勫璁炬暟鎹墜鍐屼腑鐨勬椂搴忔洿鏂般€傚皯鏁板璁炬椂搴忔棦鍙敤鏃堕棿涔熷彲鐢ㄥ懆鏈熸暟缁欏嚭锛屽凡鎻愪緵澶勭悊姝ゆ儏褰㈢殑鏈哄埗锛堝弬瑙?struct gpmc_device_timings 瀹氫箟锛夈€傚彲鑳戒細鍑虹幇澶栬鏁版嵁鎵嬪唽涓瀹氱殑鏃跺簭鍦ㄦ椂搴忕粨鏋勪腑涓嶅瓨鍦ㄧ殑鎯呭喌锛屾鏃跺簲灏濊瘯鎶婃澶栬鏃跺簭鍏宠仈鍒板凡鏈夊彲鐢ㄦ椂搴忋€傝嫢浠嶄笉琛岋紝鍙皾璇曟牴鎹渶瑕佷负澶栬娣诲姞鏂板瓧娈碉紝骞舵暀浼氶€氱敤鏃跺簭渚嬬▼澶勭悊瀹冿紝鍚屾椂纭繚涓嶇牬鍧忎换浣曞凡鏈夐€昏緫銆傚彟澶栬繕鍙兘瀛樺湪澶栬鏁版嵁鎵嬪唽鏈彁鍙?struct gpmc_device_timings 鏌愪簺瀛楁鐨勬儏鍐碉紝姝ゆ椂灏嗚繖浜涙潯鐩疆闆躲€?
閫氱敤鏃跺簭渚嬬▼宸查獙璇佸彲鍦ㄥ涓?onenand 涓?tusb6010 澶栬涓婃甯稿伐浣溿€?
娉ㄦ剰浜嬮」锛氶€氱敤鏃跺簭渚嬬▼鏄熀浜庡 gpmc 鏃跺簭銆佸璁炬椂搴忋€佸凡鏈夎嚜瀹氫箟鏃跺簭渚嬬▼鐨勭悊瑙ｏ紝鍦ㄧ己涔忓ぇ澶氭暟鏁版嵁鎵嬪唽涓庣‖浠讹紙纭垏鍦拌锛屼富绾夸腑鍙楁敮鎸佺殑閮芥病鏈夎嚜瀹氫箟鏃跺簭渚嬬▼锛夌殑鎯呭喌涓嬶紝閫氳繃鏌愮"閫嗗悜宸ョ▼"鍔犱豢鐪熷紑鍙戝嚭鏉ョ殑銆?
gpmc 鏃跺簭瀵瑰璁炬椂搴忕殑渚濊禆锛?
[<gpmc_timing>: <peripheral timing1>, <peripheral timing2> ...]

1. common锛堥€氱敤锛?
cs_on:
	t_ceasu
adv_on:
	t_avdasu, t_ceavd

2. sync common锛堝悓姝ラ€氱敤锛?
sync_clk:
	clk
page_burst_access:
	t_bacc
clk_activation:
	t_ces, t_avds

3. read async muxed锛堝紓姝ュ鐢ㄨ锛?
adv_rd_off:
	t_avdp_r
oe_on:
	t_oeasu, t_aavdh
access:
	t_iaa, t_oe, t_ce, t_aa
rd_cycle:
	t_rd_cycle, t_cez_r, t_oez

4. read async non-muxed锛堝紓姝ラ潪澶嶇敤璇伙級

adv_rd_off:
	t_avdp_r
oe_on:
	t_oeasu
access:
	t_iaa, t_oe, t_ce, t_aa
rd_cycle:
	t_rd_cycle, t_cez_r, t_oez

5. read sync muxed锛堝悓姝ュ鐢ㄨ锛?
adv_rd_off:
	t_avdp_r, t_avdh
oe_on:
	t_oeasu, t_ach, cyc_aavdh_oe
access:
	t_iaa, cyc_iaa, cyc_oe
rd_cycle:
	t_cez_r, t_oez, t_ce_rdyz

6. read sync non-muxed锛堝悓姝ラ潪澶嶇敤璇伙級

adv_rd_off:
	t_avdp_r
oe_on:
	t_oeasu
access:
	t_iaa, cyc_iaa, cyc_oe
rd_cycle:
	t_cez_r, t_oez, t_ce_rdyz

7. write async muxed锛堝紓姝ュ鐢ㄥ啓锛?
adv_wr_off:
	t_avdp_w
we_on, wr_data_mux_bus:
	t_weasu, t_aavdh, cyc_aavhd_we
we_off:
	t_wpl
cs_wr_off:
	t_wph
wr_cycle:
	t_cez_w, t_wr_cycle

8. write async non-muxed锛堝紓姝ラ潪澶嶇敤鍐欙級

adv_wr_off:
	t_avdp_w
we_on, wr_data_mux_bus:
	t_weasu
we_off:
	t_wpl
cs_wr_off:
	t_wph
wr_cycle:
	t_cez_w, t_wr_cycle

9. write sync muxed锛堝悓姝ュ鐢ㄥ啓锛?
adv_wr_off:
	t_avdp_w, t_avdh
we_on, wr_data_mux_bus:
	t_weasu, t_rdyo, t_aavdh, cyc_aavhd_we
we_off:
	t_wpl, cyc_wpl
cs_wr_off:
	t_wph
wr_cycle:
	t_cez_w, t_ce_rdyz

10. write sync non-muxed锛堝悓姝ラ潪澶嶇敤鍐欙級

adv_wr_off:
	t_avdp_w
we_on, wr_data_mux_bus:
	t_weasu, t_rdyo
we_off:
	t_wpl, cyc_wpl
cs_wr_off:
	t_wph
wr_cycle:
	t_cez_w, t_ce_rdyz


Note锛堟敞鎰忥級锛?  璁稿 gpmc 鏃跺簭渚濊禆浜庡叾瀹?gpmc 鏃跺簭锛堝皯鏁?gpmc 鏃跺簭绾补渚濊禆鍏跺畠 gpmc 鏃跺簭锛岃繖涔熸槸涓婇潰缂哄け閮ㄥ垎 gpmc 鏃跺簭鐨勫師鍥狅級锛岃繖灏嗗鑷村璁炬椂搴忓闄や笂杩颁箣澶栫殑鍏跺畠 gpmc 鏃跺簭浜х敓闂存帴渚濊禆锛屾洿澶氱粏鑺傚弬瑙佹椂搴忎緥绋嬨€傝浜嗚В杩欎簺澶栬鏃跺簭瀵瑰簲鐨勫惈涔夛紝璇峰弬瑙?struct gpmc_device_timings 瀹氫箟涓殑璇存槑銆傚浜?gpmc 鏃跺簭锛岃鍙傝€?IP 缁嗚妭锛堜笂闈㈤摼鎺ワ級銆?