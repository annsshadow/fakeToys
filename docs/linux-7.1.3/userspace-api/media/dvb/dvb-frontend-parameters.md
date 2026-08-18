


######## 鍓嶇鍙傛暟


浼犻€掔粰鍓嶇璁惧杩涜璋冭皭鐨勫弬鏁扮绫诲彇鍐充簬浣犱娇鐢ㄧ殑纭欢绉嶇被銆?

缁撴瀯浣?`dvb_frontend_parameters` 浣跨敤涓€涓?union 鏉ヤ繚瀛樼壒瀹氫簬鍚勭郴缁熺殑鍙傛暟銆傜劧鑰岋紝鐢变簬杈冩柊鐨勪紶杈撶郴缁熼渶瑕佹洿澶氭暟鎹紝璇ョ粨鏋勪綋澶у皬涓嶈冻浠ュ绾筹紝鑰屼粎浠呮墿灞曞叾澶у皬浼氱牬鍧忕幇鏈夌殑搴旂敤绋嬪簭銆傚洜姝わ紝杩欎簺鍙傛暟琚浛鎹负浣跨敤
FE_GET_PROPERTY/FE_SET_PROPERTY <FE_GET_PROPERTY>
ioctl銆傛柊鐨?API 瓒冲鐏垫椿锛屽彲浠ュ悜鐜版湁浼犺緭绯荤粺娣诲姞鏂板弬鏁帮紝涔熷彲浠ユ坊鍔犳洿鏂扮殑浼犺緭绯荤粺銆?

鍥犳锛岃緝鏂扮殑搴旂敤绋嬪簭搴斿綋鏀圭敤
FE_GET_PROPERTY/FE_SET_PROPERTY <FE_GET_PROPERTY>
锛屼互渚胯兘澶熸敮鎸佽緝鏂扮殑浼犺緭绯荤粺锛屽 DVB-S2銆丏VB-T2銆丏VB-C2銆両SDB 绛夈€?

鎵€鏈夌绫荤殑鍙傛暟鍦?`dvb_frontend_parameters` 缁撴瀯浣撲腑缁勫悎涓轰竴涓?union锛?



    struct dvb_frontend_parameters {
	uint32_t frequency;     /** (缁濆) 棰戠巼锛孮AM/OFDM 鍗曚綅涓?Hz **/
		    /** QPSK 鍗曚綅涓?kHz 鐨勪腑棰?**/
	fe_spectral_inversion_t inversion;
	union {
	    struct dvb_qpsk_parameters qpsk;
	    struct dvb_qam_parameters  qam;
	    struct dvb_ofdm_parameters ofdm;
	    struct dvb_vsb_parameters  vsb;
	} u;
    };

瀵逛簬 QPSK 鍓嶇锛宍frequency` 瀛楁鎸囧畾涓锛屽嵆瀹為檯鍔犲埌 LNB 鏈尟棰戠巼锛圠OF锛変笂鐨勫亸绉汇€備腑棰戝繀椤讳互 kHz 涓哄崟浣嶆寚瀹氥€傚浜?QAM 涓?OFDM 鍓嶇锛宍frequency` 鎸囧畾缁濆棰戠巼锛屼互 Hz 缁欏嚭銆?



## QPSK 鍙傛暟


瀵逛簬鍗槦 QPSK 鍓嶇锛屼綘蹇呴』浣跨敤 `dvb_qpsk_parameters` 缁撴瀯浣擄細



     struct dvb_qpsk_parameters {
	 uint32_t        symbol_rate;  /** 绗﹀彿鐜囷紝鍗曚綅锛氱鍙?绉?**/
	 fe_code_rate_t  fec_inner;    /** 鍓嶅悜绾犻敊锛堣涓婃枃锛?**/
     };



## QAM 鍙傛暟


瀵逛簬鏈夌嚎 QAM 鍓嶇锛屼綘浣跨敤 `dvb_qam_parameters` 缁撴瀯浣擄細



     struct dvb_qam_parameters {
	 uint32_t         symbol_rate; /** 绗﹀彿鐜囷紝鍗曚綅锛氱鍙?绉?**/
	 fe_code_rate_t   fec_inner;   /** 鍓嶅悜绾犻敊锛堣涓婃枃锛?**/
	 fe_modulation_t  modulation;  /** 璋冨埗绫诲瀷锛堣涓婃枃锛?**/
     };



## VSB 鍙傛暟


ATSC 鍓嶇鐢?`dvb_vsb_parameters` 缁撴瀯浣撴敮鎸侊細



    struct dvb_vsb_parameters {
	fe_modulation_t modulation; /** 璋冨埗绫诲瀷锛堣涓婃枃锛?**/
    };



## OFDM 鍙傛暟


DVB-T 鍓嶇鐢?`dvb_ofdm_parameters` 缁撴瀯浣撴敮鎸侊細



     struct dvb_ofdm_parameters {
	 fe_bandwidth_t      bandwidth;
	 fe_code_rate_t      code_rate_HP;  /** 楂樹紭鍏堢骇娴佺爜鐜?**/
	 fe_code_rate_t      code_rate_LP;  /** 浣庝紭鍏堢骇娴佺爜鐜?**/
	 fe_modulation_t     constellation; /** 璋冨埗绫诲瀷锛堣涓婃枃锛?**/
	 fe_transmit_mode_t  transmission_mode;
	 fe_guard_interval_t guard_interval;
	 fe_hierarchy_t      hierarchy_information;
     };
