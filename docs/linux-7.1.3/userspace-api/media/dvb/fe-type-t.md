
######## 鍓嶇绫诲瀷


鐢变簬鍘嗗彶鍘熷洜锛屽墠绔被鍨嬩互浼犺緭涓墍鐢ㄨ皟鍒舵柟寮忕殑绫诲瀷鍛藉悕銆傚墠绔被鍨嬬敱 fe_type_t 绫诲瀷
缁欏嚭锛屽畾涔夊涓嬶細



    :header-rows:  1
    :stub-columns: 0
    :widths:       3 1 4


    - .. row 1

       - fe_type

       - 鎻忚堪

       - DTV_DELIVERY_SYSTEM <DTV-DELIVERY-SYSTEM> 绛変环鐨勭被鍨?
    - .. row 2

       - .. _FE-QPSK:

	  `FE_QPSK`

       - 鐢ㄤ簬 DVB-S 鏍囧噯

       - `SYS_DVBS`

    - .. row 3

       - .. _FE-QAM:

	  `FE_QAM`

       - 鐢ㄤ簬 DVB-C 闄勫綍 A 鏍囧噯

       - `SYS_DVBC_ANNEX_A`

    - .. row 4

       - .. _FE-OFDM:

	  `FE_OFDM`

       - 鐢ㄤ簬 DVB-T 鏍囧噯

       - `SYS_DVBT`

    - .. row 5

       - .. _FE-ATSC:

	  `FE_ATSC`

       - 鐢ㄤ簬 ATSC 鏍囧噯锛堝湴闈級鎴栫編鍥戒娇鐢ㄧ殑 DVB-C 闄勫綍 B锛堟湁绾匡級

       - `SYS_ATSC`锛堝湴闈級鎴?`SYS_DVBC_ANNEX_B`锛堟湁绾匡級


杈冩柊鐨勬牸寮忓 DVB-S2銆両SDB-T銆両SDB-S 鍜?DVB-T2 鏈湪涓婇潰鎻忚堪锛屽洜涓哄畠浠€氳繃鏂扮殑
FE_GET_PROPERTY/FE_GET_SET_PROPERTY <FE_GET_PROPERTY> ioctl锛屼娇鐢?DTV_DELIVERY_SYSTEM <DTV-DELIVERY-SYSTEM> 鍙傛暟寰楀埌鏀寔銆?
鍦ㄨ繃鍘伙紝缁撴瀯浣?`dvb_frontend_info` 鏇剧粡鍖呭惈 `fe_type_t` 瀛楁浠ユ寚绀轰紶杈撶郴缁燂紝濉厖涓?`FE_QPSK`銆乣FE_QAM`銆乣FE_OFDM` 鎴?`FE_ATSC` 涔嬩竴銆傝櫧鐒朵负浜嗕繚鎸佸悜鍚庡吋瀹逛粛浼氬～鍏呰瀛楁锛?浣嗗叾浣跨敤宸茶寮冪敤锛屽洜涓哄畠鍙兘鎶ュ憡涓€涓紶杈撶郴缁燂紝鑰屾煇浜涜澶囨敮鎸佸涓紶杈撶郴缁熴€傝鏀圭敤
DTV_ENUM_DELSYS <DTV-ENUM-DELSYS>銆?
鍦ㄦ敮鎸佸涓紶杈撶郴缁熺殑璁惧涓婏紝缁撴瀯浣?**`dvb_frontend_info`** 涓殑 `fe_type_t` 琚～鍏呬负
褰撳墠鏍囧噯锛岃鏍囧噯鐢辨渶鍚庝竴娆′娇鐢?DTV_DELIVERY_SYSTEM <DTV-DELIVERY-SYSTEM> 灞炴€х殑
FE_SET_PROPERTY <FE_GET_PROPERTY> 璋冪敤鎵€閫夋嫨銆?