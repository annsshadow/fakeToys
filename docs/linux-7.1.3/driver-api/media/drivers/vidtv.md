
## vidtv锛氳櫄鎷熸暟瀛楃數瑙嗛┍鍔?


浣滆€咃細Daniel W. S. Almeida <dwlsalmeida@gmail.com>锛?020 骞?6 鏈堛€?

### 鑳屾櫙


Vidtv 鏄竴涓櫄鎷?DVB 椹卞姩锛屾棬鍦ㄤ綔涓洪┍鍔ㄥ紑鍙戣€呯殑鍙傝€冩ā鏉裤€傚畠杩樼敤浜庨獙璇佺幇鏈夌殑濯掍綋 DVB API锛屼粠鑰屽府鍔╀笂灞傚簲鐢ㄧ▼搴忕殑寮€鍙戣€呫€?

鐩墠锛屽畠鐢变互涓嬮儴鍒嗙粍鎴愶細

- 涓€涓櫄鎷熻皟璋愬櫒锛坱uner锛夐┍鍔紝濡傛灉鎵€閫夐鐜囪窛绂绘煇涓壒瀹氫紶杈撶郴缁熺殑鏈夋晥棰戠巼琛ㄨ繃杩滐紝瀹冧細鎶ュ憡杈冨樊鐨勪俊鍙疯川閲忋€?

- 涓€涓櫄鎷熻В璋冨櫒锛坉emod锛夐┍鍔紝瀹冧細鎸佺画杞璋冭皭鍣ㄨ繑鍥炵殑铏氭嫙淇″彿璐ㄩ噺锛屾ā鎷熶竴涓彲浠ユ牴鎹?CNR 姘村钩涓㈠け/閲嶆柊鑾峰彇淇″彿閿佸畾鐨勮澶囥€?

- 涓€涓櫄鎷熸ˉ鎺ワ紙bridge锛夐┍鍔紝瀹冭礋璐?modprobe 鍔犺浇铏氭嫙璋冭皭鍣ㄥ拰瑙ｈ皟鍣ㄦā鍧楋紝骞跺疄鐜拌В澶嶇敤锛坉emux锛夐€昏緫銆傝妯″潡鍦ㄥ垵濮嬪寲鏃舵帴鏀跺弬鏁帮紝杩欎簺鍙傛暟灏嗗喅瀹氭ā鎷熺殑琛屼负銆?

- 璐熻矗缂栫爜涓€涓湁鏁?MPEG 浼犺緭娴侊紙Transport Stream锛夌殑浠ｇ爜锛岃娴侀殢鍚庤浼犻€掔粰妗ユ帴椹卞姩銆傝繖涓櫄鎷熸祦鍖呭惈涓€浜涚‖缂栫爜鍐呭銆傜洰鍓嶏紝鎴戜滑鏈変竴涓崟鐙殑銆佷粎鍚煶棰戠殑棰戦亾锛屽叾涓寘鍚竴涓?MPEG 鍩烘湰娴侊紙Elementary Stream锛夛紝瀹冨張鍖呭惈涓€涓?SMPTE 302m 缂栫爜鐨勬寮︽尝銆傝娉ㄦ剰锛岄€夋嫨杩欎釜鐗瑰畾鐨勭紪鐮佸櫒鏄洜涓哄畠鏄湪 MPEG 浼犺緭娴佷腑缂栫爜 PCM 闊抽鏁版嵁鏈€绠€鍗曠殑鏂瑰紡銆?


### 鏋勫缓 vidtv


vidtv 鏄竴涓祴璇曢┍鍔紝鍥犳鍦ㄧ紪璇戝唴鏍告椂**榛樿涓?*鍚敤銆?

涓轰簡鍚敤 vidtv 鐨勭紪璇戯細

- 鍚敤 **DVB_TEST_DRIVERS**锛岀劧鍚?
- 鍚敤 **DVB_VIDTV**

褰撶紪璇戜负妯″潡鏃讹紝棰勬湡浼氱敓鎴愪互涓?.ko 鏂囦欢锛?

- dvb_vidtv_tuner.ko

- dvb_vidtv_demod.ko

- dvb_vidtv_bridge.ko


### 杩愯 vidtv


```
	modprobe vidtv
```
灏辨槸杩欐牱锛佹ˉ鎺ラ┍鍔ㄤ細鍦ㄥ畠鑷韩鐨勫垵濮嬪寲杩囩▼涓垵濮嬪寲璋冭皭鍣ㄥ拰瑙ｈ皟鍣ㄩ┍鍔ㄣ€?

榛樿鎯呭喌涓嬶紝瀹冨皢鎺ュ彈浠ヤ笅棰戠巼锛?

 - 474 MHz锛屽搴?DVB-T/T2/C锛?
 - 11,362 GHz锛屽搴?DVB-S/S2銆?

瀵逛簬鍗槦绯荤粺锛岃椹卞姩妯℃嫙涓€涓€氱敤鐨勬墿灞曞瀷 LNBf锛屽叾棰戠巼浣嶄簬 Ku 娉㈡锛岃寖鍥翠粠 10.7 GHz 鍒?12.75 GHz銆?

浣犲彲浠ラ€夋嫨鎬у湴涓?vidtv 瀹氫箟涓€浜涘懡浠よ鍙傛暟銆?


### vidtv 鐨勫懡浠よ鍙傛暟


浠ヤ笅鏄彲浠ユ彁渚涚粰 vidtv 鐨勬墍鏈夊弬鏁板垪琛細

drop_tslock_prob_on_low_snr
	褰撲俊鍙疯川閲忓樊鏃朵涪澶?TS 閿佸畾鐨勬鐜囥€?
	杩欎釜姒傜巼浼氳铏氭嫙瑙ｈ皟鍣ㄩ┍鍔ㄤ娇鐢紝浠ヤ究鍦ㄤ俊鍙疯川閲忎笉濂芥椂
	鏈€缁堣繑鍥炰竴涓?0 鐘舵€併€?

recover_tslock_prob_on_good_snr:
	褰撲俊鍙锋敼鍠勬椂鎭㈠ TS 閿佸畾鐨勬鐜囥€傝繖涓?
	姒傜巼浼氳铏氭嫙瑙ｈ皟鍣ㄩ┍鍔ㄤ娇鐢紝浠ヤ究鍦ㄤ俊鍙疯川閲忔敼鍠勬椂/鑻ユ敼鍠勬椂
	鏈€缁堣繑鍥炰竴涓?0x1f 鐘舵€併€?

mock_power_up_delay_msec
	妯℃嫙涓婄數寤惰繜銆傞粯璁ゅ€硷細0銆?

mock_tune_delay_msec
	妯℃嫙璋冭皭寤惰繜銆傞粯璁ゅ€?0銆?

vidtv_valid_dvb_t_freqs
	瑕佹ā鎷熺殑鏈夋晥 DVB-T 棰戠巼锛屽崟浣嶄负 Hz銆?

vidtv_valid_dvb_c_freqs
	瑕佹ā鎷熺殑鏈夋晥 DVB-C 棰戠巼锛屽崟浣嶄负 Hz銆?

vidtv_valid_dvb_s_freqs
	瑕佹ā鎷熺殑浣嶄簬 Ku 娉㈡鐨勬湁鏁?DVB-S/S2 棰戠巼锛屽崟浣嶄负 kHz銆?

max_frequency_shift_hz,
	璋冭皭鍒版煇涓閬撴椂鍏佽鐨勬渶澶у亸绉婚噺锛屽崟浣嶄负 Hz銆?

si_period_msec
	鍙戦€?SI 鍖呯殑棰戠巼銆傞粯璁ゅ€硷細40ms銆?

pcr_period_msec
	鍙戦€?PCR 鍖呯殑棰戠巼銆傞粯璁ゅ€硷細40ms銆?

mux_rate_kbytes_sec
	濡傛湁蹇呰锛岄€氳繃鎻掑叆 TS 绌哄寘鏉ョ淮鎸佽姣旂壒鐜囥€傞粯璁ゅ€硷細4096銆?

pcr_pid,
	鎵€鏈夐閬撶殑 PCR PID銆傞粯璁ゅ€硷細0x200銆?

mux_buf_sz_pkts,
	澶嶇敤缂撳啿鍖哄ぇ灏忥紝浠?188 瀛楄妭涓哄崟浣嶃€?

### vidtv 鍐呴儴缁撴瀯


鍐呮牳妯″潡鎸変互涓嬫柟寮忔媶鍒嗭細

vidtv_tuner.[ch]
	瀹炵幇涓€涓櫄鎷熻皟璋愬櫒 DVB 椹卞姩銆?

vidtv_demod.[ch]
	瀹炵幇涓€涓櫄鎷熻В璋冨櫒 DVB 椹卞姩銆?

vidtv_bridge.[ch]
	瀹炵幇涓€涓ˉ鎺ラ┍鍔ㄣ€?

涓?MPEG 鐩稿叧鐨勪唬鐮佹寜浠ヤ笅鏂瑰紡鎷嗗垎锛?

vidtv_ts.[ch]
	澶勭悊 MPEG TS 鍖呯殑浠ｇ爜锛屼緥濡?TS 澶淬€侀€傞厤瀛楁銆?
	PCR 鍖呭拰 NULL 鍖呫€?

vidtv_psi.[ch]
	杩欐槸 PSI 鐢熸垚鍣ㄣ€侾SI 鍖呭寘鍚叧浜?MPEG 浼犺緭娴佺殑
	涓€鑸俊鎭€傞渶瑕佷竴涓?PSI 鐢熸垚鍣紝杩欐牱涓婂眰搴旂敤鎵嶈兘
	鑾峰彇鍏充簬浼犺緭娴佺殑淇℃伅锛屽苟鏈€缁堣皟璋愬埌涓€涓紙铏氭嫙锛夐閬撱€?

	鐢变簬璇ョ敓鎴愬櫒瀹炵幇鍦ㄤ竴涓崟鐙殑鏂囦欢涓紝瀹冨彲浠ュ湪濯掍綋瀛愮郴缁熺殑鍏朵粬鍦版柟琚鐢ㄣ€?

	鐩墠 vidtv 鏀寔澶勭悊 5 绉?PSI 琛細PAT銆丳MT銆?
	SDT銆丯IT 鍜?EIT銆?

	PAT 涓?PMT 鐨勮鑼冨彲鍙傝 *ISO 13818-1:
	Systems**锛岃€?SDT銆丯IT銆丒IT 鐨勮鑼冨彲鍙傝 **ETSI
	EN 300 468: Specification for Service Information (SI) in DVB
	systems*銆?

	杩欏苟闈炰弗鏍煎繀瑕侊紝浣嗗湪璋冭瘯 PSI 琛ㄦ椂浣跨敤涓€涓湡瀹炵殑 TS 鏂囦欢浼氬緢鏈夊府鍔┿€俈idtv 鐩墠灏濊瘯澶嶅埗姝ゆ枃浠朵腑鐨?PSI 缁撴瀯锛歚TS1Globo.ts
	<https://tsduck.io/streams/brazil-isdb-tb/TS1globo.ts>`_銆?

	涓€绉嶅彲瑙嗗寲娴佺粨鏋勭殑濂芥柟娉曟槸浣跨敤
	`DVBInspector <https://sourceforge.net/projects/dvbinspector/>`_銆?

vidtv_pes.[ch]
	瀹炵幇 PES 閫昏緫锛屽皢缂栫爜鍣ㄦ暟鎹浆鎹负 MPEG TS 鍖呫€?
	杩欎簺鍖呴殢鍚庡彲浠ヨ閫佸叆 TS 澶嶇敤鍣紝骞舵渶缁堣繘鍏ョ敤鎴风┖闂淬€?

vidtv_encoder.h
	vidtv 缂栫爜鍣ㄧ殑鎺ュ彛銆傚彲浠ラ€氳繃瀹炵幇姝ゆ枃浠朵腑鐨勮皟鐢ㄦ潵鍚戣椹卞姩娣诲姞鏂扮殑缂栫爜鍣ㄣ€?

vidtv_s302m.[ch]
	瀹炵幇涓€涓?S302M 缂栫爜鍣紝浠ヤ究灏?PCM 闊抽鏁版嵁鎻掑叆鐢熸垚鐨?
	MPEG 浼犺緭娴佷腑銆傜浉鍏宠鑼冨彲鍦ㄧ嚎鑾峰彇锛屽悕涓?*SMPTE 302M-2007:
	Television - Mapping of AES3 Data into MPEG-2 Transport Stream*銆?


	鐢熸垚缁撴灉 MPEG 鍩烘湰娴侀€氳繃闄勫甫涓€涓?S302M 娉ㄥ唽鎻忚堪绗︾殑绉佹湁娴佷紶閫併€?

	杩欐牱灏卞彲浠ュ皢闊抽淇″彿浼犲叆鐢ㄦ埛绌洪棿锛屼粠鑰岃濯掍綋杞欢瑙ｇ爜鍜屾挱鏀俱€俧fmpeg 涓搴旂殑瑙ｇ爜鍣ㄤ綅浜?'libavcodec/s302m.c'锛岀洰鍓嶄粛鏄疄楠屾€х殑銆?

vidtv_channel.[ch]
	瀹炵幇涓€涓€滈閬擄紙channel锛夆€濇娊璞°€?

	褰?vidtv 鍚姩鏃讹紝瀹冧細鍒涘缓涓€浜涚‖缂栫爜鐨勯閬擄細

	#. 瀹冧滑鐨勬湇鍔′細琚嫾鎺ヨ捣鏉ヤ互濉厖 SDT銆?

	#. 瀹冧滑鐨勮妭鐩細琚嫾鎺ヨ捣鏉ヤ互濉厖 PAT銆?

	#. 瀹冧滑鐨勪簨浠朵細琚嫾鎺ヨ捣鏉ヤ互濉厖 EIT銆?

	#. 瀵逛簬 PAT 涓殑姣忎釜鑺傜洰锛岄兘浼氬垱寤轰竴涓?PMT 娈点€?

	#. 鏌愪釜棰戦亾鐨?PMT 娈典細琚垎閰嶅畠鐨勬祦銆?

	#. 姣忎釜娴侀兘浼氬湪鍏跺搴旂殑缂栫爜鍣ㄤ笂琚惊鐜疆璇互浜х敓 TS 鍖呫€?
	   杩欎簺鍖呭彲鑳借澶嶇敤鍣ㄤ氦閿欙紝鐒跺悗浼犻€掔粰妗ユ帴椹卞姩銆?

vidtv_mux.[ch]
	瀹炵幇涓€涓?MPEG TS 澶嶇敤鍣紝澶ц嚧鍩轰簬 ffmpeg 鍦?
	"libavcodec/mpegtsenc.c" 涓殑瀹炵幇銆?

	澶嶇敤鍣ㄨ繍琛屼竴涓惊鐜紝璐熻矗锛?

	#. 璺熻釜鑷笂娆¤凯浠ｄ互鏉ョ粡杩囩殑鏃堕棿閲忋€?

	#. 杞缂栫爜鍣ㄤ互鑾峰彇鈥渆lapsed_time鈥濆ぇ灏忕殑鏁版嵁銆?

	#. 濡傛湁闇€瑕侊紝鎻掑叆 PSI 鍜?鎴?PCR 鍖呫€?

	#. 濡傛湁蹇呰锛岀敤 NULL 鍖呭～鍏呯粨鏋滄祦锛屼互缁存寔鎵€閫夌殑姣旂壒鐜囥€?

	#. 灏嗙粨鏋?TS 鍖呬紶閫掔粰妗ユ帴椹卞姩锛屼互渚垮畠鑳藉皢瀹冧滑浼犵粰瑙ｅ鐢ㄥ櫒銆?


### 浣跨敤 v4l-utils 娴嬭瘯 vidtv


浣跨敤 v4l-utils 涓殑宸ュ叿鏄祴璇曞拰妫€鏌?vidtv 杈撳嚭鐨勫ソ鏂规硶銆傚畠鎵樼鍦ㄨ繖閲岋細`v4l-utils Documentation
<https://linuxtv.org/wiki/index.php/V4l-utils>`_銆?

```
	The v4l-utils are a series of packages for handling media devices.

	It is hosted at http://git.linuxtv.org/v4l-utils.git, and packaged
	on most distributions.

	It provides a series of libraries and utilities to be used to
	control several aspect of the media boards.
```
```
	modprobe dvb_vidtv_bridge
```
濡傛灉椹卞姩姝ｅ父锛屽畠搴斿綋浼氳鍔犺浇锛屽苟涓斿畠鐨勬帰娴嬩唬鐮佷細杩愯銆傝繖浼氬皢璋冭皭鍣ㄥ拰瑙ｈ皟鍣ㄩ┍鍔ㄤ竴骞舵媺鍏ャ€?


#### 浣跨敤 dvb-fe-tool


```
	$ dvb-fe-tool
	Device Dummy demod for DVB-T/T2/C/S/S2 (/dev/dvb/adapter0/frontend0) capabilities:
	    CAN_FEC_1_2
	    CAN_FEC_2_3
	    CAN_FEC_3_4
	    CAN_FEC_4_5
	    CAN_FEC_5_6
	    CAN_FEC_6_7
	    CAN_FEC_7_8
	    CAN_FEC_8_9
	    CAN_FEC_AUTO
	    CAN_GUARD_INTERVAL_AUTO
	    CAN_HIERARCHY_AUTO
	    CAN_INVERSION_AUTO
	    CAN_QAM_16
	    CAN_QAM_32
	    CAN_QAM_64
	    CAN_QAM_128
	    CAN_QAM_256
	    CAN_QAM_AUTO
	    CAN_QPSK
	    CAN_TRANSMISSION_MODE_AUTO
	DVB API Version 5.11, Current v5 delivery system: DVBC/ANNEX_A
	Supported delivery systems:
	    DVBT
	    DVBT2
	    [DVBC/ANNEX_A]
	    DVBS
	    DVBS2
	Frequency range for the current standard:
	From:            51.0 MHz
	To:              2.15 GHz
	Step:            62.5 kHz
	Tolerance:       29.5 MHz
	Symbol rate ranges for the current standard:
	From:            1.00 MBauds
	To:              45.0 MBauds
```
```
	static const struct dvb_frontend_ops vidtv_demod_ops = {
		.delsys = {
			SYS_DVBT,
			SYS_DVBT2,
			SYS_DVBC_ANNEX_A,
			SYS_DVBS,
			SYS_DVBS2,
		},

		.info = {
			.name                   = "Dummy demod for DVB-T/T2/C/S/S2",
			.frequency_min_hz       = 51 * MHz,
			.frequency_max_hz       = 2150 * MHz,
			.frequency_stepsize_hz  = 62500,
			.frequency_tolerance_hz = 29500 * kHz,
			.symbol_rate_min        = 1000000,
			.symbol_rate_max        = 45000000,

			.caps = FE_CAN_FEC_1_2 |
				FE_CAN_FEC_2_3 |
				FE_CAN_FEC_3_4 |
				FE_CAN_FEC_4_5 |
				FE_CAN_FEC_5_6 |
				FE_CAN_FEC_6_7 |
				FE_CAN_FEC_7_8 |
				FE_CAN_FEC_8_9 |
				FE_CAN_QAM_16 |
				FE_CAN_QAM_64 |
				FE_CAN_QAM_32 |
				FE_CAN_QAM_128 |
				FE_CAN_QAM_256 |
				FE_CAN_QAM_AUTO |
				FE_CAN_QPSK |
				FE_CAN_FEC_AUTO |
				FE_CAN_INVERSION_AUTO |
				FE_CAN_TRANSMISSION_MODE_AUTO |
				FE_CAN_GUARD_INTERVAL_AUTO |
				FE_CAN_HIERARCHY_AUTO,
		}

		....

```
鏈夊叧 dvb-fe-tools 鐨勬洿澶氫俊鎭紝璇锋煡鐪嬪叾鍦ㄧ嚎鏂囨。锛?
`dvb-fe-tool Documentation
<https://www.linuxtv.org/wiki/index.php/Dvb-fe-tool>`_銆?


#### 浣跨敤 dvb-scan


涓轰簡璋冭皭鍒版煇涓閬撳苟璇诲彇 PSI 琛紝鎴戜滑鍙互浣跨敤 dvb-scan銆?

涓烘锛岄渶瑕佹彁渚涗竴浠界О涓衡€滄壂鎻忔枃浠讹紙scan file锛夆€濈殑閰嶇疆鏂囦欢锛?
```
	[Channel]
	FREQUENCY = 474000000
	MODULATION = QAM/AUTO
	SYMBOL_RATE = 6940000
	INNER_FEC = AUTO
	DELIVERY_SYSTEM = DVBC/ANNEX_A
```
	鍙傛暟鍙栧喅浜庝綘鎵€娴嬭瘯鐨勭數瑙嗘爣鍑嗐€?

	Vidtv 鏄竴涓櫄鎷熼┍鍔紝涓嶄細瀵规壂鎻忔枃浠朵腑鐨勫ぇ閮ㄥ垎淇℃伅杩涜楠岃瘉銆傚浜?DVB-T/DVB-T2锛屽彧闇€鎸囧畾 'FREQUENCY' 鍜?'DELIVERY_SYSTEM' 灏辫冻澶熶簡銆備笉杩囧浜?DVB-S/DVB-C锛屼綘杩樺簲褰撴彁渚?'SYMBOL_RATE'銆?

浣犲彲浠ュ湪绾挎祻瑙堟壂鎻忚〃锛歚dvb-scan-tables
<https://git.linuxtv.org/dtv-scan-tables.git>`_銆?

```
	$ dvbv5-scan channel.conf
	dvbv5-scan ~/vidtv.conf
	ERROR    command BANDWIDTH_HZ (5) not found during retrieve
	Cannot calc frequency shift. Either bandwidth/symbol-rate is unavailable (yet).
	Scanning frequency #1 330000000
	    (0x00) Signal= -68.00dBm
	Scanning frequency #2 474000000
	Lock   (0x1f) Signal= -34.45dBm C/N= 33.74dB UCB= 0
	Service Beethoven, provider LinuxTV.org: digital television
```
鏈夊叧 dvb-scan 鐨勬洿澶氫俊鎭紝璇锋煡鐪嬪叾鍦ㄧ嚎鏂囨。锛?
`dvb-scan Documentation <https://www.linuxtv.org/wiki/index.php/Dvbscan>`_銆?


#### 浣跨敤 dvb-zap


dvbv5-zap 鏄竴涓懡浠よ宸ュ叿锛屽彲鐢ㄤ簬灏?MPEG-TS 褰曞埗鍒扮鐩樸€傚吀鍨嬬敤娉曟槸璋冭皭鍒版煇涓閬撳苟灏嗗叾缃簬褰曞埗妯″紡銆傜ず渚?
```
	$ dvbv5-zap -c dvb_channel.conf "beethoven" -o music.ts -P -t 10
	using demux 'dvb0.demux0'
	reading channels from file 'dvb_channel.conf'
	tuning to 474000000 Hz
	pass all PID's to TS
	dvb_set_pesfilter 8192
	dvb_dev_set_bufsize: buffer set to 6160384
	Lock   (0x1f) Quality= Good Signal= -34.66dBm C/N= 33.41dB UCB= 0 postBER= 0 preBER= 1.05x10^-3 PER= 0
	Lock   (0x1f) Quality= Good Signal= -34.57dBm C/N= 33.46dB UCB= 0 postBER= 0 preBER= 1.05x10^-3 PER= 0
	Record to file 'music.ts' started
	received 24587768 bytes (2401 Kbytes/sec)
	Lock   (0x1f) Quality= Good Signal= -34.42dBm C/N= 33.89dB UCB= 0 postBER= 0 preBER= 2.44x10^-3 PER= 0
```
       閽堝 music.ts 鏂囦欢銆?


鍙互閫氳繃浣跨敤鑳借瘑鍒?MPEG-TS 鏍煎紡鐨勬挱鏀惧櫒锛堜緥濡?`mplayer` 鎴?`vlc`锛夋挱鏀炬祦鐨勫唴瀹规潵瑙傜湅璇ラ閬撱€?

閫氳繃鎾斁娴佺殑鍐呭锛屽彲浠ョ洿瑙傚湴妫€鏌?
```
	$ mplayer music.ts
```
```
	$ dvbv5-zap -c dvb_channel.conf "beethoven" -P -r &
```
```
	$ mplayer /dev/dvb/adapter0/dvr0
```
鏈夊叧 dvb-zap 鐨勬洿澶氫俊鎭紝璇锋煡鐪嬪叾鍦ㄧ嚎鏂囨。锛?
`dvb-zap Documentation
<https://www.linuxtv.org/wiki/index.php/Dvbv5-zap>`_銆?
鍙﹁锛歚zap <https://www.linuxtv.org/wiki/index.php/Zap>`_銆?


### vidtv 涓粛鍙敼杩涗箣澶?


#### 娣诲姞 *debugfs* 闆嗘垚


灏界鍓嶇椹卞姩閫氳繃 .read_status 璋冪敤鎻愪緵 DVBv5 缁熻淇℃伅锛屼竴涓笉閿欑殑琛ュ厖鏄€氳繃 debugfs 鍚戜笂灞傜┖闂存彁渚涢澶栫殑缁熻淇℃伅锛宒ebugfs 鏄竴涓畝鍗曟槗鐢ㄣ€佸熀浜?RAM 鐨勬枃浠剁郴缁燂紝涓撻棬鐢ㄤ簬璋冭瘯鐩殑銆?

涓轰簡閬垮厤姹℃煋鍓嶇椹卞姩锛岃繖閮ㄥ垎閫昏緫搴斿疄鐜板湪涓€涓崟鐙殑鏂囦欢涓€傝繖浜涚粺璁′俊鎭槸椹卞姩鐗瑰畾鐨勶紝鍦ㄦ祴璇曟湡闂村彲鑳藉緢鏈夌敤銆?

Siano 椹卞姩鏄娇鐢?debugfs 鍚戠敤鎴风┖闂翠紶閫掗┍鍔ㄧ壒瀹氱粺璁′俊鎭殑涓€涓緥瀛愶紝鍙互浣滀负鍙傝€冦€?

涓轰簡鏂逛究锛岃繖搴斿綋杩涗竴姝ラ€氳繃涓€涓?Kconfig 閫夐」鏉ュ惎鐢ㄥ拰绂佺敤銆?


#### 娣诲姞娴嬭瘯瑙嗛鐨勬柟娉?


鐩墠锛寁idtv 鍙兘缂栫爜 PCM 闊抽銆傚鏋滆兘瀹炵幇涓€涓渶绠€鐗堟湰鐨?MPEG-2 瑙嗛缂栫爜锛屾垜浠氨鍙互鍚屾椂娴嬭瘯瑙嗛锛岄偅灏嗛潪甯稿ソ銆傞鍏堝簲褰撴煡闃?*ISO 13818-2锛氫俊鎭妧鏈€斺€旇繍鍔ㄥ浘鍍忓強鍏朵即闊充俊鎭殑閫氱敤缂栫爜鈥斺€旂 2 閮ㄥ垎锛氳棰?锛屽叾涓兜鐩栦簡 MPEG 浼犺緭娴佷腑鍘嬬缉瑙嗛鐨勭紪鐮併€?

杩欏彲浠ラ€夋嫨鎬у湴浣跨敤 Video4Linux2 娴嬭瘯鍥炬鐢熸垚鍣?v4l2-tpg锛?
```
	drivers/media/common/v4l2-tpg/
```


#### 娣诲姞鐧藉櫔澹版ā鎷?


vidtv 璋冭皭鍣ㄥ凡缁忔湁浠ｇ爜鏉ヨ瘑鍒墍閫夐鐜囨槸鍚﹁窛绂绘湁鏁堥鐜囪〃杩囪繙銆傜洰鍓嶏紝杩欐剰鍛崇潃瑙ｈ皟鍣ㄦ渶缁堝彲鑳戒細涓㈠け淇″彿閿佸畾锛屽洜涓鸿皟璋愬櫒浼氭姤鍛婅緝宸殑淇″彿璐ㄩ噺銆?

涓€涓笉閿欑殑琛ュ厖鏄湪淇″彿璐ㄩ噺宸椂妯℃嫙涓€浜涘櫔澹帮細

- 闅忔満涓㈠純涓€浜?TS 鍖呫€傚鏋滆繛缁€ц鏁板櫒琚洿鏂颁絾鍖呮病鏈夎浼犻€掔粰瑙ｅ鐢ㄥ櫒锛岃繖灏嗚Е鍙戜竴涓繛缁€ч敊璇€?

- 鐩稿簲鍦版洿鏂伴敊璇粺璁′俊鎭紙渚嬪 BER 绛夛級銆?

- 鍦ㄧ紪鐮佹暟鎹腑妯℃嫙涓€浜涘櫔澹般€?


### vidtv 涓娇鐢ㄧ殑鍑芥暟鍜岀粨鏋勪綋

















