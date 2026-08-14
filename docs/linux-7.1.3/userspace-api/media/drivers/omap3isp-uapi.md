

## OMAP 3 鍥惧儚淇″彿澶勭悊鍣紙ISP锛夐┍鍔?

Copyright |copy| 2010 Nokia Corporation

Copyright |copy| 2009 Texas Instruments, Inc.

鑱旂郴浜猴細Laurent Pinchart <laurent.pinchart@ideasonboard.com>銆?Sakari Ailus <sakari.ailus@iki.fi>銆丏avid Cohen <dacohen@gmail.com>


### 浜嬩欢


OMAP 3 ISP 椹卞姩鍦?CCDC 鍜岀粺璁★紙AEWB銆丄F 鍜?histogram锛夊瓙璁惧涓婃敮鎸?V4L2 浜嬩欢鎺ュ彛銆?
CCDC 瀛愯澶囧湪 HS_VS 涓柇涓婁骇鐢?V4L2_EVENT_FRAME_SYNC 绫诲瀷浜嬩欢锛岀敤浜?鎸囩ず甯у紑濮嬨€傝椹卞姩鐨勬棭鏈熺増鏈负姝や娇鐢?V4L2_EVENT_OMAP3ISP_HS_VS銆傝浜嬩欢
鎭板ソ鍦?CCDC 妯″潡涓帴鏀跺埌甯х殑绗竴琛屾椂瑙﹀彂銆傝浜嬩欢鍙互鍦?CCDC 瀛愯澶囦笂
璁㈤槄銆?
锛堜娇鐢ㄥ苟琛屾帴鍙ｆ椂锛屽繀椤绘纭厤缃?VS 淇″彿鏋佹€с€備娇鐢ㄤ覆琛屾帴鏀跺櫒鏃朵細鑷姩姝ｇ‘銆傦級

姣忎釜缁熻瀛愯澶囬兘鑳藉浜х敓浜嬩欢銆傛瘡褰撶敤鎴风┖闂村簲鐢ㄧ▼搴忓彲浠ヤ娇鐢?VIDIOC_OMAP3ISP_STAT_REQ IOCTL 灏嗙粺璁＄紦鍐插尯鍑洪槦鏃讹紝灏变細鐢熸垚涓€涓簨浠躲€?鍙敤鐨勪簨浠舵湁锛?
- V4L2_EVENT_OMAP3ISP_AEWB
- V4L2_EVENT_OMAP3ISP_AF
- V4L2_EVENT_OMAP3ISP_HIST

杩欎簺 ioctl 鐨勪簨浠舵暟鎹被鍨嬩负 struct omap3isp_stat_event_status銆傚鏋滆绠?缁熻淇℃伅鍑洪敊锛屼粛浼氬儚寰€甯镐竴鏍蜂骇鐢熶簨浠讹紝浣嗕笉浼氭湁鍏宠仈鐨勭粺璁＄紦鍐插尯銆傚湪杩欑
鎯呭喌涓嬶紝omap3isp_stat_event_status.buf_err 琚涓洪潪闆躲€?

### 绉佹湁 IOCTL


OMAP 3 ISP 椹卞姩鍦ㄥ彲鑳戒笖鍙鐨勬儏鍐典笅鏀寔鏍囧噯鐨?V4L2 IOCTL 鍜屾帶浠躲€傜劧鑰岋紝
ISP 鎻愪緵鐨勮澶氬姛鑳藉苟涓嶅睘浜庢爣鍑?IOCTL鈥斺€斾緥濡備冀椹〃浠ュ強缁熻閲囬泦鐨勯厤缃€?
涓€鑸潵璇达紝瀵逛簬姣忎釜鍖呭惈纭欢鐩稿叧鍔熻兘鐨勬ā鍧楋紝閮芥湁涓€涓鏈夌殑 ioctl 鐢ㄤ簬閰嶇疆銆?
鏀寔鐨勭鏈?IOCTL 濡備笅锛?
- VIDIOC_OMAP3ISP_CCDC_CFG
- VIDIOC_OMAP3ISP_PRV_CFG
- VIDIOC_OMAP3ISP_AEWB_CFG
- VIDIOC_OMAP3ISP_HIST_CFG
- VIDIOC_OMAP3ISP_AF_CFG
- VIDIOC_OMAP3ISP_STAT_REQ
- VIDIOC_OMAP3ISP_STAT_EN

杩欎簺 ioctl 鎵€浣跨敤鐨勫弬鏁扮粨鏋勪綋鍦?include/linux/omap3isp.h 涓弿杩般€備笌鐗瑰畾
ISP 妯″潡鐩稿叧鐨?ISP 鏈韩鐨勮缁嗗姛鑳藉湪銆婃妧鏈弬鑰冩墜鍐屻€嬶紙TRM锛変腑鎻忚堪鈥斺€旀枃妗?鏈熬鍒楀嚭浜嗚繖浜涙墜鍐屻€?
铏界劧鍙互鍦ㄥ畬鍏ㄤ笉浣跨敤杩欎簺绉佹湁 IOCTL 鐨勬儏鍐典笅浣跨敤 ISP 椹卞姩锛屼絾浠ユ鏂瑰紡鏃犳硶
鑾峰緱鏈€浣冲浘鍍忚川閲忋€傚鏋滀笉浣跨敤鐩稿簲鐨勭鏈?IOCTL 杩涜閰嶇疆锛屽氨鏃犳硶浣跨敤 AEWB銆?AF 鍜?histogram 妯″潡銆?

### CCDC 涓庨瑙堟ā鍧?IOCTL


VIDIOC_OMAP3ISP_CCDC_CFG 鍜?VIDIOC_OMAP3ISP_PRV_CFG IOCTL 鍒嗗埆鐢ㄤ簬閰嶇疆銆?鍚敤鍜岀鐢?CCDC 涓庨瑙堟ā鍧椾腑鐨勫姛鑳姐€傝繖涓や釜 IOCTL 閮芥帶鍒跺叾瀵瑰簲妯″潡涓殑
澶氫釜鍔熻兘銆俈IDIOC_OMAP3ISP_CCDC_CFG IOCTL 鎺ュ彈涓€涓寚鍚?struct
omap3isp_ccdc_update_config 鐨勬寚閽堜綔涓哄弬鏁般€傜被浼煎湴锛孷IDIOC_OMAP3ISP_PRV_CFG
鎺ュ彈涓€涓寚鍚?struct omap3isp_prev_update_config 鐨勬寚閽堛€傝繖涓や釜缁撴瀯浣撶殑瀹氫箟
瑙?[#]_銆?
缁撴瀯浣撲腑鐨?update 瀛楁鎸囨槑鏄惁鏇存柊璇ョ壒瀹氬姛鑳界殑閰嶇疆锛宖lag 瀛楁鎸囨槑鏄惎鐢?杩樻槸绂佺敤璇ュ姛鑳姐€?
update 鍜?flag 浣嶆帺鐮佹帴鍙椾互涓嬪€笺€侰CDC 鍜岄瑙堟ā鍧椾腑鐨勬瘡涓嫭绔嬪姛鑳介兘鍏宠仈
涓€涓?flag锛堢鐢ㄦ垨鍚敤锛屽睘浜庣粨鏋勪綋 flag 瀛楁鐨勪竴閮ㄥ垎锛変互鍙婁竴涓寚鍚戣鍔熻兘
閰嶇疆鏁版嵁鐨勬寚閽堛€?
VIDIOC_OMAP3ISP_CCDC_CFG 鐨勬湁鏁?update 鍜?flag 瀛楁鍊煎湪姝ゅ垪鍑恒€傝繖浜涘€煎彲浠?鐢ㄦ垨杩愮畻缁勫悎锛屼互鍦ㄥ悓涓€娆?IOCTL 璋冪敤涓厤缃涓姛鑳姐€?
- OMAP3ISP_CCDC_ALAW
- OMAP3ISP_CCDC_LPF
- OMAP3ISP_CCDC_BLCLAMP
- OMAP3ISP_CCDC_BCOMP
- OMAP3ISP_CCDC_FPC
- OMAP3ISP_CCDC_CULL
- OMAP3ISP_CCDC_CONFIG_LSC
- OMAP3ISP_CCDC_TBL_LSC

VIDIOC_OMAP3ISP_PRV_CFG 鐨勫搴斿€煎涓嬶細

- OMAP3ISP_PREV_LUMAENH
- OMAP3ISP_PREV_INVALAW
- OMAP3ISP_PREV_HRZ_MED
- OMAP3ISP_PREV_CFA
- OMAP3ISP_PREV_CHROMA_SUPP
- OMAP3ISP_PREV_WB
- OMAP3ISP_PREV_BLKADJ
- OMAP3ISP_PREV_RGB2RGB
- OMAP3ISP_PREV_COLOR_CONV
- OMAP3ISP_PREV_YC_LIMIT
- OMAP3ISP_PREV_DEFECT_COR
- OMAP3ISP_PREV_GAMMABYPASS
- OMAP3ISP_PREV_DRK_FRM_CAPTURE
- OMAP3ISP_PREV_DRK_FRM_SUBTRACT
- OMAP3ISP_PREV_LENS_SHADING
- OMAP3ISP_PREV_NF
- OMAP3ISP_PREV_GAMMA

鍚敤鏌愰」鍔熻兘鏃讹紝鍏跺叧鑱旂殑閰嶇疆鎸囬拡涓嶅緱涓?NULL銆傜鐢ㄦ煇椤瑰姛鑳芥椂锛岃閰嶇疆鎸囬拡
浼氳蹇界暐銆?

### 缁熻妯″潡 IOCTL


缁熻瀛愯澶囨瘮鍏跺畠瀛愯澶囨彁渚涙洿鍔ㄦ€佺殑閰嶇疆閫夐」銆傚畠浠彲浠ュ湪娴佹按绾垮浜?streaming 鐘舵€佹椂鍚敤銆佺鐢ㄥ拰閲嶆柊閰嶇疆銆?
缁熻妯″潡濮嬬粓浠?CCDC 鑾峰彇杈撳叆鍥惧儚鏁版嵁锛堝洜涓烘湭瀹炵幇 histogram 鍐呭瓨璇诲彇锛夈€?鐢ㄦ埛鍙互浣跨敤绉佹湁 IOCTL 浠庣粺璁″瓙璁惧鑺傜偣灏嗙粺璁′俊鎭嚭闃熴€?
AEWB銆丄F 鍜?histogram 瀛愯澶囨彁渚涚殑绉佹湁 IOCTL 鍦ㄥ緢澶х▼搴︿笂鍙嶆槧浜?ISP 纭欢
鎵€鎻愪緵鐨勫瘎瀛樺櫒绾ф帴鍙ｃ€傛湁浜涙柟闈㈢函绮逛笌椹卞姩瀹炵幇鐩稿叧锛屾帴涓嬫潵灏嗚璁鸿繖浜涙柟闈€?
### VIDIOC_OMAP3ISP_STAT_EN


璇ョ鏈?IOCTL 鍚敤/绂佺敤涓€涓粺璁℃ā鍧椼€傚鏋滃湪 streaming 涔嬪墠鍙戝嚭姝よ姹傦紝瀹?灏嗗湪娴佹按绾垮紑濮?streaming 鏃剁珛鍗崇敓鏁堛€傚鏋滄祦姘寸嚎宸茬粡鍦?streaming锛屽畠灏嗗湪
CCDC 鍙樹负绌洪棽鏃剁珛鍗崇敓鏁堛€?
### VIDIOC_OMAP3ISP_AEWB_CFG銆乂IDIOC_OMAP3ISP_HIST_CFG 涓?VIDIOC_OMAP3ISP_AF_CFG


杩欎簺 IOCTL 鐢ㄤ簬閰嶇疆鍚勬ā鍧椼€傚畠浠姹傜敤鎴峰簲鐢ㄧ▼搴忓纭欢鏈夋繁鍏ョ殑浜嗚В銆傚ぇ閮ㄥ垎
瀛楁鐨勮鏄庡彲浠ュ湪 OMAP 鐨?TRM 涓壘鍒般€備笂杩版墍鏈夐厤缃敤绉佹湁 IOCTL 鍏辨湁鐨勪互涓?涓や釜瀛楁闇€瑕佽繘涓€姝ヨ鏄庯紝浠ヤ究鏇村ソ鍦扮悊瑙ｏ紝鍥犱负瀹冧滑涓嶅睘浜?TRM 鐨勫唴瀹广€?
omap3isp_[h3a_af/h3a_aewb/hist]\_config.buf_size锛?
杩欎簺妯″潡鍦ㄥ唴閮ㄥ鐞嗚嚜宸辩殑缂撳啿鍖恒€傛ā鍧楁暟鎹緭鍑烘墍闇€鐨勭紦鍐插尯澶у皬鍙栧喅浜庢墍璇锋眰
鐨勯厤缃€傚敖绠￠┍鍔ㄦ敮鎸佸湪 streaming 鏃堕噸鏂伴厤缃紝浣嗗鏋滄ā鍧楀凡鍚敤锛屽畠涓嶆敮鎸?闇€瑕佹瘮鍐呴儴宸插垎閰嶇紦鍐插尯鏇村ぇ灏哄鐨勯噸鏂伴厤缃紝杩欑鎯呭喌涓嬩細杩斿洖 -EBUSY銆備负閬垮厤
杩欑鎯呭喌锛屽彲浠ョ鐢?閲嶆柊閰嶇疆/鍚敤妯″潡锛屾垨鑰呭湪妯″潡绂佺敤鏈熼棿浜庨娆￠厤缃椂璇锋眰
鎵€闇€鐨勭紦鍐插尯澶у皬銆?
鍐呴儴缂撳啿鍖哄ぇ灏忕殑鍒嗛厤浼氳€冭檻鎵€璇锋眰閰嶇疆鐨勬渶灏忕紦鍐插尯澶у皬锛屼互鍙?buf_size 瀛楁
璁剧疆鐨勫€笺€傚鏋?buf_size 瀛楁瓒呭嚭 [鏈€灏? 鏈€澶 缂撳啿鍖哄ぇ灏忚寖鍥达紝鍒欎細琚挸鍒?浠ラ€傞厤璇ヨ寖鍥淬€傞殢鍚庨┍鍔ㄤ細閫夋嫨鏈€澶х殑鍊笺€備慨姝ｅ悗鐨?buf_size 鍊间細琚啓鍥炵敤鎴?搴旂敤绋嬪簭銆?
omap3isp_[h3a_af/h3a_aewb/hist]\_config.config_counter锛?
鐢变簬閰嶇疆涓嶄細涓庤姹傚悓姝ョ敓鏁堬紝椹卞姩蹇呴』鎻愪緵涓€绉嶆柟寮忔潵璺熻釜姝や俊鎭紝浠ユ彁渚涙洿鍑嗙‘
鐨勬暟鎹€傚湪璇锋眰鏌愰」閰嶇疆鍚庯紝杩斿洖缁欑敤鎴风┖闂村簲鐢ㄧ▼搴忕殑 config_counter 灏嗘槸涓庤
璇锋眰鍏宠仈鐨勫敮涓€鍊笺€傚綋鐢ㄦ埛搴旂敤绋嬪簭鏀跺埌缂撳啿鍖哄彲鐢ㄧ殑浜嬩欢锛屾垨璇锋眰鏂扮殑缂撳啿鍖烘椂锛?姝?config_counter 鐢ㄤ簬灏嗙紦鍐插尯鏁版嵁涓庨厤缃繘琛屽尮閰嶃€?
### VIDIOC_OMAP3ISP_STAT_REQ


灏嗗唴閮ㄧ紦鍐插尯闃熷垪涓渶鏃╁彲鐢ㄧ殑鏁版嵁鍙戦€佸埌鐢ㄦ埛绌洪棿锛屽苟闅忓悗涓㈠純璇ョ紦鍐插尯銆傚瓧娈?omap3isp_stat_data.frame_number 涓庤棰戠紦鍐插尯鐨?field_count 鐩稿尮閰嶃€?

### 鍙傝€冭祫鏂?

