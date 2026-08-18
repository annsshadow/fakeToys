## cx2341x 椹卞姩


### 闈炲帇缂╂枃浠舵牸寮?

cx23416 鍙互浜х敓锛堣€?cx23415 涔熷彲浠ヨ鍙栵級鍘熷 YUV 杈撳嚭銆俌UV 甯х殑鏍煎紡涓?16x16 绾挎€?骞抽摵鐨?NV12锛圴4L2_PIX_FMT_NV12_16L16锛夈€?
璇ユ牸寮忎负 YUV 4:2:0锛屾瘡涓儚绱犱娇鐢?1 涓?Y 瀛楄妭锛屾瘡鍥涗釜鍍忕礌浣跨敤 1 涓?U 涓?1 涓?V 瀛楄妭銆?
鏁版嵁琚紪鐮佷负涓や釜瀹忓潡骞抽潰锛岀涓€涓寘鍚?Y 鍊硷紝绗簩涓寘鍚?UV 瀹忓潡銆?
Y 骞抽潰浠庡乏鍒板彸銆佷粠涓婂埌涓嬪垝鍒嗕负 16x16 鍍忕礌鐨勫潡銆傛瘡涓潡渚濇鎸夎浼犺緭銆?
鍥犳鍓?16 涓瓧鑺傛槸宸︿笂鍧楃殑绗竴琛岋紝鎺ヤ笅鏉ョ殑 16 涓瓧鑺傛槸宸︿笂鍧楃殑绗簩琛岋紝渚濇绫绘帹銆備紶杈撳畬
璇ュ潡鍚庯紝浼犺緭鍏跺彸渚у潡鐨勭涓€琛岋紝渚濇绫绘帹銆?
UV 骞抽潰浠庡乏鍒板彸銆佷粠涓婂埌涓嬪垝鍒嗕负 16x8 涓?UV 鍊肩殑鍧椼€傛瘡涓潡渚濇鎸夎浼犺緭銆?
鍥犳鍓?16 涓瓧鑺傛槸宸︿笂鍧楃殑绗竴琛岋紝鍖呭惈 8 瀵?UV 鍊硷紙鍏?16 瀛楄妭锛夈€傛帴涓嬫潵鐨?16 涓瓧鑺傛槸
宸︿笂鍧楃殑绗簩琛岋紙8 瀵?UV锛夛紝渚濇绫绘帹銆備紶杈撳畬璇ュ潡鍚庯紝浼犺緭鍏跺彸渚у潡鐨勭涓€琛岋紝渚濇绫绘帹銆?
涓嬮潰鐨勪唬鐮佷綔涓虹ず渚嬶紝灞曠ず浜嗗浣曞皢 V4L2_PIX_FMT_NV12_16L16 杞崲涓虹嫭绔嬬殑 Y銆乁 涓?V 骞抽潰銆?璇ヤ唬鐮佸亣瀹氬抚澶у皬涓?720x576锛圥AL锛夊儚绱犮€?
甯х殑瀹藉害濮嬬粓涓?720 鍍忕礌锛屼笌瀹為檯鎸囧畾鐨勫搴︽棤鍏炽€?
濡傛灉楂樺害涓嶆槸 32 琛岀殑鏁存暟鍊嶏紝鍒欐崟鑾风殑瑙嗛浼氬湪鏈熬涓㈠け瀹忓潡鑰屾棤娉曚娇鐢ㄣ€傚洜姝ら珮搴﹀繀椤绘槸
32 鐨勬暣鏁板€嶃€?
#### 鍘熷鏍煎紡鐨?C 绀轰緥



	#include <stdio.h>
	#include <stdlib.h>
	#include <string.h>

	static unsigned char frame[576**720**3/2];
	static unsigned char framey[576*720];
	static unsigned char frameu[576*720 / 4];
	static unsigned char framev[576*720 / 4];

	static void de_macro_y(unsigned char** dst, unsigned char **src, int dstride, int w, int h)
	{
	unsigned int y, x, i;

	// descramble Y plane
	// dstride = 720 = w
	// The Y plane is divided into blocks of 16x16 pixels
	// Each block in transmitted in turn, line-by-line.
	for (y = 0; y < h; y += 16) {
		for (x = 0; x < w; x += 16) {
		for (i = 0; i < 16; i++) {
			memcpy(dst + x + (y + i) * dstride, src, 16);
			src += 16;
		}
		}
	}
	}

	static void de_macro_uv(unsigned char **dstu, unsigned char **dstv, unsigned char *src, int dstride, int w, int h)
	{
	unsigned int y, x, i;

	// descramble U/V plane
	// dstride = 720 / 2 = w
	// The U/V values are interlaced (UVUV...).
	// Again, the UV plane is divided into blocks of 16x16 UV values.
	// Each block in transmitted in turn, line-by-line.
	for (y = 0; y < h; y += 16) {
		for (x = 0; x < w; x += 8) {
		for (i = 0; i < 16; i++) {
			int idx = x + (y + i) * dstride;

			dstu[idx+0] = src[^0^];  dstv[idx+0] = src[^1^];
			dstu[idx+1] = src[^2^];  dstv[idx+1] = src[^3^];
			dstu[idx+2] = src[^4^];  dstv[idx+2] = src[^5^];
			dstu[idx+3] = src[^6^];  dstv[idx+3] = src[^7^];
			dstu[idx+4] = src[^8^];  dstv[idx+4] = src[^9^];
			dstu[idx+5] = src[^10^]; dstv[idx+5] = src[^11^];
			dstu[idx+6] = src[^12^]; dstv[idx+6] = src[^13^];
			dstu[idx+7] = src[^14^]; dstv[idx+7] = src[^15^];
			src += 16;
		}
		}
	}
	}

	/*************************************************************************/
	int main(int argc, char **argv)
	{
	FILE *fin;
	int i;

	if (argc == 1) fin = stdin;
	else fin = fopen(argv[^1^], "r");

	if (fin == NULL) {
		fprintf(stderr, "cannot open input\n");
		exit(-1);
	}
	while (fread(frame, sizeof(frame), 1, fin) == 1) {
		de_macro_y(framey, frame, 720, 720, 576);
		de_macro_uv(frameu, framev, frame + 720 * 576, 720 / 2, 720 / 2, 576 / 2);
		fwrite(framey, sizeof(framey), 1, stdout);
		fwrite(framev, sizeof(framev), 1, stdout);
		fwrite(frameu, sizeof(frameu), 1, stdout);
	}
	fclose(fin);
	return 0;
	}


### 鍐呭祵 V4L2_MPEG_STREAM_VBI_FMT_IVTV VBI 鏁版嵁鐨勬牸寮?

浣滆€咃細Hans Verkuil <hverkuil@kernel.org>


鏈妭鎻忚堪鍐呭祵浜?MPEG-2 鑺傜洰娴佷腑鐨?VBI 鏁版嵁鐨?V4L2_MPEG_STREAM_VBI_FMT_IVTV 鏍煎紡銆傝鏍煎紡
閮ㄥ垎鐢?ivtv 椹卞姩锛堥潰鍚?Conexant cx23415/6 鑺墖鐨勯┍鍔級鐨勬煇浜涚‖浠堕檺鍒跺喅瀹氾紝鐗瑰埆鏄?VBI
鏁版嵁鐨勬渶澶у昂瀵搞€傝秴鍑鸿灏哄鐨勯儴鍒嗕細鍦ㄩ€氳繃 cx23415 鍥炴斁 MPEG 娴佹椂琚埅鏂€?
璇ユ牸寮忕殑浼樼偣鏄潪甯哥揣鍑戯紝骞朵笖鎵€鏈夎鐨?VBI 鏁版嵁閮藉彲浠ュ湪涓嶈秴杩囨渶澶у厑璁稿昂瀵哥殑鎯呭喌涓嬪瓨鍌ㄣ€?
VBI 鏁版嵁鐨勬祦 ID 涓?0xBD銆傚唴宓屾暟鎹殑鏈€澶у昂瀵镐负 4 + 43 ** 36锛屽嵆 4 瀛楄妭鐨勫ご閮紝浠ュ強
2 ** 18 鏉?VBI 琛岋紝姣忚鏈?1 瀛楄妭澶撮儴涓?42 瀛楄妭杞借嵎銆傝秴鍑烘闄愬埗鐨勯儴鍒嗕細琚?cx23415/6
鍥轰欢鎴柇銆傞櫎浜?VBI 琛岀殑鏁版嵁澶栵紝鎴戜滑杩橀渶瑕?36 浣嶇敤浜庣‘瀹氭崟鑾蜂簡鍝簺琛岀殑浣嶆帺鐮侊紝浠ュ強
4 瀛楄妭鐢ㄤ簬涓€涓?magic cookie锛岃〃鏄庤鏁版嵁鍖呭寘鍚?V4L2_MPEG_STREAM_VBI_FMT_IVTV VBI 鏁版嵁銆?濡傛灉鎵€鏈夎閮借浣跨敤锛屽垯涓嶅啀鏈夌┖闂村瓨鏀句綅鎺╃爜銆備负瑙ｅ喅姝ら棶棰橈紝寮曞叆浜嗕袱涓笉鍚岀殑 magic 鏁帮細

'itv0'锛氬湪姝?magic 鏁颁箣鍚庤窡闅忎袱涓?unsigned long銆傜涓€涓?unsigned long 鐨勪綅 0-17 琛ㄧず
绗竴涓満涓崟鑾蜂簡鍝簺琛屻€傜涓€涓?unsigned long 鐨勪綅 18-31 涓庣浜屼釜 unsigned long 鐨勪綅 0-3
鐢ㄤ簬绗簩涓満銆?
'ITV0'锛氭 magic 鏁板亣瀹氭崟鑾蜂簡鎵€鏈?VBI 琛岋紝鍗冲畠闅愬紡鍦拌〃绀轰綅鎺╃爜涓?0xffffffff 涓?0xf銆?
鍦ㄨ繖浜?magic cookie锛堜互鍙婂湪 'itv0' 鎯呭喌涓嬬殑 8 瀛楄妭浣嶆帺鐮侊級涔嬪悗锛屾崟鑾风殑 VBI 琛屽紑濮嬶細

瀵逛簬姣忎竴琛岋紝绗竴涓瓧鑺傜殑鏈€浣?4 浣嶅寘鍚暟鎹被鍨嬨€傚彲鑳界殑鍊煎涓嬭〃鎵€绀恒€傝浇鑽蜂綅浜庨殢鍚庣殑
42 瀛楄妭涓€?
浠ヤ笅鏄彲鑳界殑鏁版嵁绫诲瀷锛?

	#define IVTV_SLICED_TYPE_TELETEXT       0x1     // Teletext (uses lines 6-22 for PAL)
	#define IVTV_SLICED_TYPE_CC             0x4     // Closed Captions (line 21 NTSC)
	#define IVTV_SLICED_TYPE_WSS            0x5     // Wide Screen Signal (line 23 PAL)
	#define IVTV_SLICED_TYPE_VPS            0x7     // Video Programming System (PAL) (line 16)
