
### 鏁板瓧鐢佃鍓嶇 kABI


#### 鏁板瓧鐢佃鍓嶇


鏁板瓧鐢佃鍓嶇 kABI 瀹氫箟浜嗗皢搴曞眰銆佷笌纭欢鐩稿叧鐨勯┍鍔ㄦ敞鍐屽埌涓€涓笌纭欢鏃犲叧鐨勫墠绔眰鏃舵墍闇€鐨勯┍鍔ㄥ唴閮ㄦ帴鍙ｃ€傚畠浠呭鏁板瓧鐢佃璁惧椹卞姩寮€鍙戣€呮湁鎰忎箟銆傛 API 鐨勫ご鏂囦欢鍚嶄负 `dvb_frontend.h`锛屼綅浜?`include/media/`銆?
##### 瑙ｈ皟鍣ㄩ┍鍔?

瑙ｈ皟鍣ㄩ┍鍔ㄨ礋璐ｄ笌纭欢鐨勮В鐮侀儴鍒嗛€氫俊銆傛绫婚┍鍔ㄥ簲瀹炵幇 `dvb_frontend_ops`锛屽畠璇存槑浜嗘敮鎸佸摢浜涚被鍨嬬殑鏁板瓧鐢佃鏍囧噯锛屽苟鎸囧悜涓€绯诲垪鍑芥暟锛屼娇 DVB 鏍稿績鑳藉閫氳繃 `include/media/dvb_frontend.c` 涓嬬殑浠ｇ爜鎺у埗纭欢銆?
```

	static struct dvb_frontend_ops foo_ops = {
		.delsys = { SYS_DVBT, SYS_DVBT2, SYS_DVBC_ANNEX_A },
		.info = {
			.name	= "foo DVB-T/T2/C driver",
			.caps = FE_CAN_FEC_1_2 |
				FE_CAN_FEC_2_3 |
				FE_CAN_FEC_3_4 |
				FE_CAN_FEC_5_6 |
				FE_CAN_FEC_7_8 |
				FE_CAN_FEC_AUTO |
				FE_CAN_QPSK |
				FE_CAN_QAM_16 |
				FE_CAN_QAM_32 |
				FE_CAN_QAM_64 |
				FE_CAN_QAM_128 |
				FE_CAN_QAM_256 |
				FE_CAN_QAM_AUTO |
				FE_CAN_TRANSMISSION_MODE_AUTO |
				FE_CAN_GUARD_INTERVAL_AUTO |
				FE_CAN_HIERARCHY_AUTO |
				FE_CAN_MUTE_TS |
				FE_CAN_2G_MODULATION,
			.frequency_min = 42000000, /* Hz */
			.frequency_max = 1002000000, /* Hz */
			.symbol_rate_min = 870000,
			.symbol_rate_max = 11700000
		},
		.init = foo_init,
		.sleep = foo_sleep,
		.release = foo_release,
		.set_frontend = foo_set_frontend,
		.get_frontend = foo_get_frontend,
		.read_status = foo_get_status_and_stats,
		.tune = foo_tune,
		.i2c_gate_ctrl = foo_i2c_gate_ctrl,
		.get_frontend_algo = foo_get_algo,
	};

```
涓嬮潰鏄悕涓?`bar` 鐨勯┍鍔ㄤ腑姝ょ被缁撴瀯浣撶殑鍏稿瀷绀轰緥锛屽畠鐢ㄤ簬
```

	static const struct dvb_frontend_ops bar_ops = {
		.delsys = { SYS_DVBS, SYS_DVBS2 },
		.info = {
			.name		= "Bar DVB-S/S2 demodulator",
			.frequency_min	= 500000, /* KHz */
			.frequency_max	= 2500000, /* KHz */
			.frequency_stepsize	= 0,
			.symbol_rate_min = 1000000,
			.symbol_rate_max = 45000000,
			.symbol_rate_tolerance = 500,
			.caps = FE_CAN_INVERSION_AUTO |
				FE_CAN_FEC_AUTO |
				FE_CAN_QPSK,
		},
		.init = bar_init,
		.sleep = bar_sleep,
		.release = bar_release,
		.set_frontend = bar_set_frontend,
		.get_frontend = bar_get_frontend,
		.read_status = bar_get_status_and_stats,
		.i2c_gate_ctrl = bar_i2c_gate_ctrl,
		.get_frontend_algo = bar_get_algo,
		.tune = bar_tune,

		/* Satellite-specific */
		.diseqc_send_master_cmd = bar_send_diseqc_msg,
		.diseqc_send_burst = bar_send_burst,
		.set_tone = bar_set_tone,
		.set_voltage = bar_set_voltage,
	};

```

   #) 瀵逛簬鍗槦鏁板瓧鐢佃鏍囧噯锛圖VB-S銆丏VB-S2銆両SDB-S锛夛紝棰戠巼浠?kHz 涓哄崟浣嶆寚瀹氾紱鑰屽浜庡湴闈㈠拰鏈夌嚎鏍囧噯锛屽垯浠?Hz 涓哄崟浣嶃€傚洜姝わ紝濡傛灉鍚屼竴涓墠绔悓鏃舵敮鎸佷袱绫绘爣鍑嗭紝灏遍渶瑕佹湁涓ゅ鐙珛鐨?`dvb_frontend_ops` 缁撴瀯浣擄紝姣忕鏍囧噯鍚勪竴濂椼€?   #) `.i2c_gate_ctrl` 瀛楁浠呭綋纭欢鏀寔鎺у埗 I2C 闂ㄦ帶锛堢洿鎺ユ帶鍒舵垨閫氳繃鏌愪釜 GPIO 寮曡剼锛夋椂鎵嶅瓨鍦紝浠ヤ究鍦ㄦ煇涓閬撹皟璋愬畬鎴愬悗灏嗚皟璋愬櫒浠?I2C 鎬荤嚎涓婄Щ闄ゃ€?   #) 鎵€鏈夋柊椹卞姩閮藉簲閫氳繃 `.read_status` 瀹炵幇 DVBv5 缁熻淇℃伅 <dvbv5_stats>銆備笉杩囷紝浠嶅瓨鍦ㄤ竴浜涚敤浜庤幏鍙栦俊鍙峰己搴︺€丼/N 鍜?UCB 缁熻淇℃伅鐨勫洖璋冨嚱鏁般€傚畠浠槸涓轰簡鍚戝悗鍏煎閭ｄ簺涓嶆敮鎸?DVBv5 API 鐨勬棫搴旂敤绋嬪簭鑰屼繚鐣欑殑銆傚疄鐜拌繖浜涘洖璋冩槸鍙€夌殑銆傚綋鎵€鏈夌幇鏈夐┍鍔ㄩ兘鏀寔 DVBv5 缁熻淇℃伅鍚庯紝杩欎簺鍥炶皟灏嗘潵鍙兘浼氳绉婚櫎銆?   #) 瀵逛簬鍗槦鐢佃鏍囧噯锛岃繕闇€瑕佸叾浠栧洖璋冩潵鎺у埗 LNBf 鍜?DiSEqC锛歚.diseqc_send_master_cmd`銆乣.diseqc_send_burst`銆乣.set_tone`銆乣.set_voltage`銆?

`include/media/dvb_frontend.c` 涓湁涓€涓唴鏍哥嚎绋嬶紝璐熻矗璋冭皭璁惧銆傚畠鏀寔澶氱鐢ㄤ簬妫€娴嬮閬撶殑绠楁硶锛屽畾涔変簬鏋氫妇 `dvbfe_algo`銆?
鎵€浣跨敤鐨勭畻娉曢€氳繃 `.get_frontend_algo` 鑾峰彇銆傚鏋滈┍鍔ㄦ病鏈夊湪 struct dvb_frontend_ops 涓～鍐欒瀛楁锛屽垯榛樿浣跨敤 `DVBFE_ALGO_SW`锛屾剰鍛崇潃 dvb-core 鍦ㄨ皟璋愭椂浼氭墽琛屸€滀箣瀛楀舰鈥濇悳绱紝渚嬪瀹冨厛灏濊瘯浣跨敤鎸囧畾鐨勪腑蹇冮鐜?`f`锛岀劧鍚庝緷娆″皾璇?`f` + |delta|銆乣f` - |delta|銆乣f` + 2脳|delta|銆乣f` - 2脳|delta|锛屼緷姝ょ被鎺ㄣ€?
濡傛灉纭欢鍐呴儴鑷甫鏌愮涔嬪瓧褰㈢畻娉曪紝鍒欏簲瀹氫箟涓€涓繑鍥?`DVBFE_ALGO_HW` 鐨?`.get_frontend_algo` 鍑芥暟銆?

   鏍稿績鍓嶇鏀寔杩樻彁渚涗簡绗笁绉嶇被鍨嬶紙`DVBFE_ALGO_CUSTOM`锛夛紝浠ュ厑璁搁┍鍔ㄥ畾涔夎嚜宸辩殑纭欢杈呭姪绠楁硶銆傚浠婂嚑涔庝笉闇€瑕佷娇鐢ㄥ畠銆備娇鐢?`DVBFE_ALGO_CUSTOM` 闇€瑕佸湪 struct dvb_frontend_ops 涓彁渚涘叾浠栧嚱鏁板洖璋冦€?
##### 灏嗗墠绔┍鍔ㄦ寕鎺ュ埌妗ユ帴椹卞姩


鍦ㄤ娇鐢ㄦ暟瀛楃數瑙嗗墠绔牳蹇冧箣鍓嶏紝妗ユ帴椹卞姩搴斿厛鎸傛帴鍓嶇瑙ｈ皟鍣ㄣ€佽皟璋愬櫒鍜?SEC 璁惧锛屽苟璋冪敤
`dvb_register_frontend()`锛?浠ヤ究鍚戝瓙绯荤粺娉ㄥ唽鏂扮殑鍓嶇銆傚湪璁惧鍒嗙/绉婚櫎鏃讹紝妗ユ帴椹卞姩搴旇皟鐢?`dvb_unregister_frontend()` 灏嗗墠绔粠鏍稿績涓Щ闄わ紝鐒跺悗鍐嶈皟鐢?`dvb_frontend_detach()`
閲婃斁鍓嶇椹卞姩鍒嗛厤鐨勫唴瀛樸€?
椹卞姩杩樺簲灏?`dvb_frontend_suspend()` 浣滀负鍏?`device_driver` 鐨?`suspend()` 澶勭悊鍑芥暟鐨勪竴閮ㄥ垎鏉ヨ皟鐢紝骞跺皢 `dvb_frontend_resume()` 浣滀负鍏?`device_driver` 鐨?`resume()` 澶勭悊鍑芥暟鐨勪竴閮ㄥ垎鏉ヨ皟鐢ㄣ€?
杩樻彁渚涗簡涓€浜涘叾浠栫殑鍙€夊嚱鏁帮紝鐢ㄤ簬澶勭悊鏌愪簺鐗规畩鎯呭喌銆?

#### 鏁板瓧鐢佃鍓嶇缁熻淇℃伅


##### 绠€浠?

鏁板瓧鐢佃鍓嶇鎻愪緵涓€绯诲垪缁熻淇℃伅 <frontend-stat-properties>锛岀敤浜庤緟鍔╄皟璋愯澶囧苟琛￠噺鏈嶅姟璐ㄩ噺銆?
瀵逛簬姣忔缁熻娴嬮噺锛岄┍鍔ㄥ簲璁剧疆鎵€浣跨敤鐨勫埢搴︾被鍨嬶紱濡傛灉鍦ㄦ煇涓椂鍒荤粺璁′俊鎭笉鍙敤锛屽垯璁剧疆涓?`FE_SCALE_NOT_AVAILABLE`銆傞┍鍔ㄨ繕搴旀彁渚涙瘡绉嶇被鍨嬬殑缁熻閲忎釜鏁帮紝瀵逛簬澶у鏁拌棰戞爣鍑嗚€岃█閫氬父涓?1 [#f2]_銆?
椹卞姩搴斿湪鍏跺垵濮嬪寲浠ｇ爜涓紝浠ラ暱搴﹀拰鍒诲害鍒濆鍖栨瘡涓粺璁¤鏁板櫒銆備緥濡傦紝濡傛灉鍓嶇鎻愪緵淇″彿
```

	struct dtv_frontend_properties *c = &state->fe.dtv_property_cache;

	c->strength.len = 1;
	c->strength.stat[0].scale = FE_SCALE_NOT_AVAILABLE;

```
```

	c->strength.stat[0].scale = FE_SCALE_DECIBEL;
	c->strength.stat[0].uvalue = strength;

```
   锛堢粺璁￠泦鍚堬級銆傚湪杩欑鎯呭喌涓嬶紝len 搴旂瓑浜?4銆傜涓€涓€煎搴斿叏灞€缁熻锛涘叾浣欑殑瀵瑰簲鍚勪釜灞傦紝渚嬪锛?
   - c->cnr.stat[^0^] 瀵瑰簲鍏ㄥ眬淇″櫔姣旓紙S/N锛夎浇鍣瘮锛?   - c->cnr.stat[^1^] 瀵瑰簲灞?A 鐨?S/N 杞藉櫔姣旓紝
   - c->cnr.stat[^2^] 瀵瑰簲灞?B 鐨?S/N 杞藉櫔姣旓紝
   - c->cnr.stat[^3^] 瀵瑰簲灞?C 鐨?S/N 杞藉櫔姣斻€?
   瀵逛簬淇″彿寮哄害鍜?CNR 娴嬮噺锛屼娇鐢?`FE_SCALE_RELATIVE`銆?
##### 缁熻淇℃伅鍒嗙粍


褰撳墠鏀寔浠ヤ笅鍑犵粍缁熻淇℃伅锛?
淇″彿寮哄害锛圖TV-STAT-SIGNAL-STRENGTH锛?  - 娴嬮噺璋冭皭鍣ㄦ垨瑙ｈ皟鍣ㄦā鎷熼儴鍒嗙殑淇″彿寮哄害鐢靛钩銆?
  - 閫氬父鏉ヨ嚜涓烘娴嬭浇娉㈣€屾柦鍔犲埌璋冭皭鍣ㄥ拰/鎴栧墠绔殑澧炵泭銆傚綋鏈娴嬪埌杞芥尝鏃讹紝澧炵泭澶勪簬鏈€澶у€硷紙鍥犳寮哄害澶勪簬鏈€灏忓€硷級銆?
  - 鐢变簬澧炵泭鍙€氳繃璋冩暣澧炵泭鐨勫瘎瀛樺櫒缁勮瀵熷埌锛岄€氬父璇ョ粺璁′俊鎭缁堝彲鐢?[#f3]_銆?
  - 椹卞姩搴斿敖閲忎娇鍏跺缁堝彲鐢紝鍥犱负杩欎簺缁熻淇℃伅鍙敤浜庤皟鏁村ぉ绾挎柟浣嶄互鍙婃鏌ョ嚎缂嗚繛鎺ラ棶棰樸€?
  .. [#f3] 鍦ㄥ皯鏁拌澶囦笂锛岃嫢鏃犺浇娉紝澧炵泭浼氭寔缁诞鍔ㄣ€傚湪姝ょ被璁惧涓婏紝寮哄害鎶ュ憡搴斿厛妫€鏌ヨ皟璋愬櫒鏄惁妫€娴嬪埌杞芥尝锛坄FE_HAS_CARRIER`锛屽弬瑙?`fe_status`锛夛紝鍚﹀垯杩斿洖灏藉彲鑳芥渶浣庣殑鍊笺€?
杞芥尝淇″櫔姣旓紙DTV-STAT-CNR锛?  - 涓昏浇娉㈢殑淇″櫔姣斻€?
  - 淇″櫔姣旀祴閲忓彇鍐充簬璁惧銆傚湪鏌愪簺纭欢涓婏紝涓昏浇娉㈣妫€娴嬪埌鏃跺嵆鍙幏寰椼€傚湪姝ょ被纭欢涓婏紝CNR 娴嬮噺閫氬父鏉ヨ嚜璋冭皭鍣紙渚嬪 `FE_HAS_CARRIER` 涔嬪悗锛屽弬瑙?`fe_status`锛夈€?
    鍦ㄥ叾浠栬澶囦笂锛屽畠闇€瑕佸唴閮?FEC 瑙ｇ爜锛屽洜涓哄墠绔槸浠庡叾浠栧弬鏁伴棿鎺ユ祴閲忕殑锛堜緥濡?`FE_HAS_VITERBI` 涔嬪悗锛屽弬瑙?`fe_status`锛夈€?
    鍦ㄥ唴灞?FEC 涔嬪悗鍗冲彲鑾峰緱鏇翠负甯歌銆?
FEC 涔嬪悗鐨勬瘮鐗硅鏁帮紙DTV-STAT-POST-ERROR-BIT-COUNT 鍜?DTV-STAT-POST-TOTAL-BIT-COUNT锛?  - 杩欎簺璁℃暟鍣ㄦ祴閲忓唴灞傜紪鐮佸潡涓婂墠鍚戠籂閿欙紙FEC锛変箣鍚庣殑姣旂壒鏁颁笌姣旂壒閿欒鏁帮紙鍦?Viterbi銆丩DPC 鎴栧叾浠栧唴灞傜紪鐮佷箣鍚庯級銆?
  - 鐢变簬鍏剁壒鎬э紝杩欎簺缁熻淇℃伅渚濊禆浜庡畬鏁寸殑缂栫爜閿佸畾锛堜緥濡?`FE_HAS_SYNC` 涔嬪悗鎴?`FE_HAS_LOCK` 涔嬪悗锛屽弬瑙?`fe_status`锛夈€?
FEC 涔嬪墠鐨勬瘮鐗硅鏁帮紙DTV-STAT-PRE-ERROR-BIT-COUNT 鍜?DTV-STAT-PRE-TOTAL-BIT-COUNT锛?  - 杩欎簺璁℃暟鍣ㄦ祴閲忓唴灞傜紪鐮佸潡涓婂墠鍚戠籂閿欙紙FEC锛変箣鍓嶇殑姣旂壒鏁颁笌姣旂壒閿欒鏁帮紙鍦?Viterbi銆丩DPC 鎴栧叾浠栧唴灞傜紪鐮佷箣鍓嶏級銆?
  - 骞堕潪鎵€鏈夊墠绔兘鎻愪緵姝ょ被缁熻淇℃伅銆?
  - 鐢变簬鍏剁壒鎬э紝杩欎簺缁熻淇℃伅渚濊禆浜庡唴灞傜紪鐮侀攣瀹氾紙渚嬪 `FE_HAS_VITERBI` 涔嬪悗锛屽弬瑙?`fe_status`锛夈€?
鍧楄鏁帮紙DTV-STAT-ERROR-BLOCK-COUNT 鍜?DTV-STAT-TOTAL-BLOCK-COUNT锛?  - 杩欎簺璁℃暟鍣ㄦ祴閲忓唴灞傜紪鐮佸潡涓婂墠鍚戠籂閿欙紙FEC锛変箣鍚庣殑鍧楁暟涓庡潡閿欒鏁帮紙鍦?Viterbi銆丩DPC 鎴栧叾浠栧唴灞傜紪鐮佷箣鍓嶏級銆?
  - 鐢变簬鍏剁壒鎬э紝杩欎簺缁熻淇℃伅渚濊禆浜庡畬鏁寸殑缂栫爜閿佸畾锛堜緥濡?`FE_HAS_SYNC` 涔嬪悗鎴?    `FE_HAS_LOCK`锛屽弬瑙?`fe_status`锛夈€?
   - 浠庣‖浠堕噰闆嗚€屾潵銆?
```

	static int foo_get_status_and_stats(struct dvb_frontend *fe)
	{
		struct foo_state *state = fe->demodulator_priv;
		struct dtv_frontend_properties *c = &fe->dtv_property_cache;

		int rc;
		enum fe_status *status;

		/* Both status and strength are always available */
		rc = foo_read_status(fe, &status);
		if (rc < 0)
			return rc;

		rc = foo_read_strength(fe);
		if (rc < 0)
			return rc;

		/* Check if CNR is available */
		if (!(fe->status & FE_HAS_CARRIER))
			return 0;

		rc = foo_read_cnr(fe);
		if (rc < 0)
			return rc;

		/* Check if pre-BER stats are available */
		if (!(fe->status & FE_HAS_VITERBI))
			return 0;

		rc = foo_get_pre_ber(fe);
		if (rc < 0)
			return rc;

		/* Check if post-BER stats are available */
		if (!(fe->status & FE_HAS_SYNC))
			return 0;

		rc = foo_get_post_ber(fe);
		if (rc < 0)
			return rc;
	}

	static const struct dvb_frontend_ops ops = {
		/* ... */
		.read_status = foo_get_status_and_stats,
	};

```
##### 缁熻淇℃伅閲囬泦


鍦ㄥ嚑涔庢墍鏈夊墠绔‖浠朵笂锛屾瘮鐗瑰拰瀛楄妭璁℃暟浼氱敱纭欢鍦ㄤ竴娈电壒瀹氭椂闂翠箣鍚庛€佹垨鎬绘瘮鐗?鍧楄鏁板櫒杈惧埌鏌愪釜鍊硷紙閫氬父鍙紪绋嬶級涔嬪悗杩涜瀛樺偍锛屼緥濡傛瘡 1000 ms 涓€娆★紝鎴栧湪鎺ユ敹鍒?1,000,000 姣旂壒涔嬪悗銆?
鍥犳锛屽鏋滆鍙栧瘎瀛樺櫒杩囨棭锛屾渶缁堜細璇诲埌涓庝笂涓€娆＄浉鍚岀殑鍊硷紝瀵艰嚧鍗曡皟鍊艰杩囦簬棰戠箒鍦扮疮鍔犮€?
椹卞姩搴旇礋璐ｉ伩鍏嶈繃浜庨绻佺殑璇诲彇銆傝繖鍙互閫氳繃浠ヤ笅涓ょ鏂瑰紡瀹炵幇锛?
濡傛灉椹卞姩鏈変竴涓寚绀洪噰闆嗘暟鎹綍鏃跺氨缁殑浣?%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

椹卞姩搴斿湪缁熻淇℃伅鍙敤涔嬪墠妫€鏌ヨ浣嶃€?
姝ょ被琛屼负鐨勭ず渚嬪彲鍦ㄤ互涓嬩唬鐮佺墖娈碉紙鏀圭紪鑷?```

	static int foo_get_pre_ber(struct dvb_frontend *fe)
	{
		struct foo_state *state = fe->demodulator_priv;
		struct dtv_frontend_properties *c = &fe->dtv_property_cache;
		int rc, bit_error;

		/* Check if the BER measures are already available */
		rc = foo_read_u8(state, 0x54);
		if (rc < 0)
			return rc;

		if (!rc)
			return 0;

		/* Read Bit Error Count */
		bit_error = foo_read_u32(state, 0x55);
		if (bit_error < 0)
			return bit_error;

		/* Read Total Bit Count */
		rc = foo_read_u32(state, 0x51);
		if (rc < 0)
			return rc;

		c->pre_bit_error.stat[0].scale = FE_SCALE_COUNTER;
		c->pre_bit_error.stat[0].uvalue += bit_error;
		c->pre_bit_count.stat[0].scale = FE_SCALE_COUNTER;
		c->pre_bit_count.stat[0].uvalue += rc;

		return 0;
	}

```
濡傛灉椹卞姩娌℃湁鎻愪緵鈥滅粺璁″彲鐢ㄢ€濇鏌ヤ綅
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

鐒惰€岋紝灏戞暟璁惧鍙兘涓嶆彁渚涙鏌ョ粺璁℃槸鍚﹀彲鐢ㄧ殑鏂瑰紡锛堟垨妫€鏌ユ柟寮忔湭鐭ワ級銆傚畠浠敋鑷冲彲鑳戒笉鎻愪緵鐩存帴璇诲彇鎬绘瘮鐗规暟鎴栨€诲潡鏁扮殑鏂瑰紡銆?
鍦ㄦ绫昏澶囦笂锛岄┍鍔ㄩ渶瑕佺‘淇濅笉浼氳繃浜庨绻佸湴浠庡瘎瀛樺櫒璇诲彇锛屽拰/鎴栦及绠楁€绘瘮鐗规暟/鍧楁暟銆?
鍦ㄦ绫婚┍鍔ㄤ笂锛岃幏鍙栫粺璁′俊鎭殑鍏稿瀷渚嬬▼绫讳技浜?```

	struct foo_state {
		/* ... */

		unsigned long per_jiffies_stats;
	}

	static int foo_get_pre_ber(struct dvb_frontend *fe)
	{
		struct foo_state *state = fe->demodulator_priv;
		struct dtv_frontend_properties *c = &fe->dtv_property_cache;
		int rc, bit_error;
		u64 bits;

		/* Check if time for stats was elapsed */
		if (!time_after(jiffies, state->per_jiffies_stats))
			return 0;

		/* Next stat should be collected in 1000 ms */
		state->per_jiffies_stats = jiffies + msecs_to_jiffies(1000);

		/* Read Bit Error Count */
		bit_error = foo_read_u32(state, 0x55);
		if (bit_error < 0)
			return bit_error;

		/*
		 * On this particular frontend, there's no register that
		 * would provide the number of bits per 1000ms sample. So,
		 * some function would calculate it based on DTV properties
		 */
		bits = get_number_of_bits_per_1000ms(fe);

		c->pre_bit_error.stat[0].scale = FE_SCALE_COUNTER;
		c->pre_bit_error.stat[0].uvalue += bit_error;
		c->pre_bit_count.stat[0].scale = FE_SCALE_COUNTER;
		c->pre_bit_count.stat[0].uvalue += bits;

		return 0;
	}

```
璇锋敞鎰忥紝鍦ㄨ繖涓ょ鎯呭喌涓嬶紝鎴戜滑閮芥槸浣跨敤 `dvb_frontend_ops` 鐨?`.read_status` 鍥炶皟鏉ヨ幏鍙栫粺璁′俊鎭殑銆傚叾鍘熷洜鏄紝鍓嶇鏍稿績浼氳嚜鍔ㄥ懆鏈熸€у湴璋冪敤璇ュ嚱鏁帮紙閫氬父褰撳墠绔攣瀹氭椂姣忕 3 娆★級銆?
杩欎繚璇佷簡鎴戜滑涓嶄細閿欒繃閲囬泦鏌愪釜璁℃暟鍣ㄥ苟鍦ㄦ纭椂闂寸疮鍔犲崟璋冪粺璁″€笺€?
#### 鏁板瓧鐢佃鍓嶇鍑芥暟涓庣被鍨?

