


######## 灞炴€х被鍨?

瑕佽皟璋愬埌涓€涓暟瀛楃數瑙嗭紙Digital TV锛夌墿鐞嗛閬撳苟寮€濮嬭В鐮侊紝闇€瑕佹敼鍙樹竴缁勫弬鏁帮紝浠ユ帶鍒?璋冭皭鍣紙tuner锛夈€佽В璋冨櫒锛坉emodulator锛夈€佺嚎鎬т綆鍣０鏀惧ぇ鍣紙LNA锛夛紝骞堕€氳繃鍗槦璁惧
鎺у埗锛圫atellite Equipment Control锛孲EC锛岀敤浜庡崼鏄熺郴缁燂級鏉ヨ缃ぉ绾垮瓙绯荤粺銆傚叿浣撶殑
鍙傛暟鍥犳瘡绉嶆暟瀛楃數瑙嗘爣鍑嗚€屽紓锛屽苟涓斿彲鑳介殢鐫€鏁板瓧鐢佃瑙勮寖鐨勬紨杩涜€屽彉鍖栥€?
杩囧幓锛堢洿鍒?DVB API 绗?3 鐗堚€斺€擠VBv3锛夛紝鎵€浣跨敤鐨勭瓥鐣ユ槸鎻愪緵涓€涓?union锛屽皢璋冭皭
DVB-S銆丏VB-C銆丏VB-T 鍜?ATSC 浼犺緭绯荤粺鎵€闇€鐨勫弬鏁板綊鍦ㄤ竴璧枫€傞棶棰樺湪浜庯紝闅忕潃绗簩浠?鏍囧噯鐨勫嚭鐜帮紝杩欐牱涓€涓?union 鐨勫ぇ灏忎笉瓒充互瀹圭撼閭ｄ簺鏂版爣鍑嗘墍闇€鐨?struct銆傝€屼笖锛屾墿灞曞畠
浼氱牬鍧忕敤鎴风┖闂淬€?
鍥犳锛屽熀浜庢棫 union/struct 鐨勬柟娉曡寮冪敤锛岃浆鑰岄噰鐢ㄥ熀浜庡睘鎬ч泦锛坧roperties set锛夌殑
鏂规硶銆傚湪杩欑鏂规硶涓紝浣跨敤 FE_GET_PROPERTY 鍜?FE_SET_PROPERTY <FE_GET_PROPERTY> 鏉?璁剧疆鍓嶇锛坒rontend锛夊苟璇诲彇鍏剁姸鎬併€?
鍏蜂綋鐨勬搷浣滅敱涓€缁?dtv_property 鐨?cmd/data 瀵规潵鍐冲畾銆傞€氳繃涓€娆?ioctl锛屾渶澶氬彲浠?鑾峰彇/璁剧疆 64 涓睘鎬с€?
鏈妭鎻忚堪浜嗚缃墠绔殑鏂版帹鑽愭柟寮忥紝瀹冩敮鎸佹墍鏈夌殑鏁板瓧鐢佃浼犺緭绯荤粺銆?

   1. 鍦?Linux DVB API 绗?3 鐗堜腑锛岃缃墠绔槸閫氳繃 struct `dvb_frontend_parameters`
      瀹屾垚鐨勩€?
   2. 涓嶈鍦ㄦ敮鎸佹柊鏍囧噯鐨勭‖浠朵笂浣跨敤 DVB API 绗?3 鐗堣皟鐢ㄣ€傝 API 瀵规柊鏍囧噯鍜?鎴栨柊纭欢
      涓嶆彁渚涙垨浠呮彁渚涢潪甯告湁闄愮殑鏀寔銆?
   3. 濡備粖锛屽ぇ澶氭暟鍓嶇鏀寔澶氱浼犺緭绯荤粺銆傚彧鏈変娇鐢?DVB API 绗?5 鐗堣皟鐢紝鎵嶈兘鍦ㄥ墠绔?      鏀寔鐨勫绉嶄紶杈撶郴缁熶箣闂村垏鎹€?
   4. DVB API 绗?5 鐗堜篃绉颁负 **S2API**锛屽洜涓哄姞鍏ュ畠鐨勭涓€涓柊鏍囧噯鏄?DVB-S2銆?
**绀轰緥**锛氫负浜嗗皢纭欢璁剧疆涓鸿皟璋愬埌 651 kHz 鐨?DVB-C 棰戦亾锛岄噰鐢?256-QAM 璋冨埗銆丗EC 3/4
浠ュ強 5.217 Mbauds 鐨勭鍙风巼锛屽簲灏嗕互涓嬪睘鎬у彂閫佺粰 FE_SET_PROPERTY <FE_GET_PROPERTY>
ioctl锛?
  DTV_DELIVERY_SYSTEM <DTV-DELIVERY-SYSTEM> = SYS_DVBC_ANNEX_A

  DTV_FREQUENCY <DTV-FREQUENCY> = 651000000

  DTV_MODULATION <DTV-MODULATION> = QAM_256

  DTV_INVERSION <DTV-INVERSION> = INVERSION_AUTO

  DTV_SYMBOL_RATE <DTV-SYMBOL-RATE> = 5217000

  DTV_INNER_FEC <DTV-INNER-FEC> = FEC_3_4

  DTV_TUNE <DTV-TUNE>

瀹炵幇涓婅堪鍔熻兘鐨勪唬鐮佸睍绀哄湪 dtv-prop-example 涓€?
    :caption: 绀轰緥锛氳缃暟瀛楃數瑙嗗墠绔睘鎬?    :name: dtv-prop-example

    #include <stdio.h>
    #include <fcntl.h>
    #include <sys/ioctl.h>
    #include <linux/dvb/frontend.h>

    static struct dtv_property props[] = {
	{ .cmd = DTV_DELIVERY_SYSTEM, .u.data = SYS_DVBC_ANNEX_A },
	{ .cmd = DTV_FREQUENCY,       .u.data = 651000000 },
	{ .cmd = DTV_MODULATION,      .u.data = QAM_256 },
	{ .cmd = DTV_INVERSION,       .u.data = INVERSION_AUTO },
	{ .cmd = DTV_SYMBOL_RATE,     .u.data = 5217000 },
	{ .cmd = DTV_INNER_FEC,       .u.data = FEC_3_4 },
	{ .cmd = DTV_TUNE }
    };

    static struct dtv_properties dtv_prop = {
	.num = 6, .props = props
    };

    int main(void)
    {
	int fd = open("/dev/dvb/adapter0/frontend0", O_RDWR);

	if (!fd) {
	    perror ("open");
	    return -1;
	}
	if (ioctl(fd, FE_SET_PROPERTY, &dtv_prop) == -1) {
	    perror("ioctl");
	    return -1;
	}
	printf("Frontend set\\n");
	return 0;
    }

   涓婅堪绀轰緥寮虹儓寤鸿浣跨敤 `libdvbv5 <https://linuxtv.org/docs/libdvbv5/index.html>`__锛?鍥犱负瀹冩彁渚涗簡浣跨敤鎵€鏀寔鏁板瓧鐢佃鏍囧噯鐨勬娊璞★紝骞舵彁渚涗簡鐢ㄤ簬甯歌鎿嶄綔锛堝鑺傜洰鎵弿浠ュ強
璇?鍐欓閬撴弿杩扮鏂囦欢锛夌殑鏂规硶銆?
- [fe_property_parameters](fe_property_parameters)
- [frontend-stat-properties](frontend-stat-properties)
- [frontend-property-terrestrial-systems](frontend-property-terrestrial-systems)
- [frontend-property-cable-systems](frontend-property-cable-systems)
- [frontend-property-satellite-systems](frontend-property-satellite-systems)
- [frontend-header](frontend-header)
