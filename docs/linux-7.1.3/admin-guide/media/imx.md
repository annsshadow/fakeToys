
## i.MX 瑙嗛鎹曡幏椹卞姩

### 绠€浠?
Freescale i.MX5/6 鍖呭惈涓€涓浘鍍忓鐞嗗崟鍏冿紙Image Processing Unit锛孖PU锛夛紝
瀹冭礋璐ｅ浘鍍忓抚鍦ㄦ崟鑾疯澶囧拰鏄剧ず璁惧涔嬮棿鐨勬祦鍚戙€?
瀵逛簬鍥惧儚鎹曡幏锛孖PU 鍖呭惈浠ヤ笅鍐呴儴瀛愬崟鍏冿細

- 鍥惧儚 DMA 鎺у埗鍣紙Image DMA Controller锛孖DMAC锛?- 鎽勫儚澶翠覆琛屾帴鍙ｏ紙Camera Serial Interface锛孋SI锛?- 鍥惧儚杞崲鍣紙Image Converter锛孖C锛?- 浼犳劅鍣ㄥ FIFO 鎺у埗鍣紙Sensor Multi-FIFO Controller锛孲MFC锛?- 鍥惧儚鏃嬭浆鍣紙Image Rotator锛孖RT锛?- 瑙嗛鍘婚殧琛屾垨鍚堟垚妯″潡锛圴ideo De-Interlacing or Combining Block锛孷DIC锛?
IDMAC 鏄敤浜庡湪鍐呭瓨涓庡浘鍍忓抚涔嬮棿杩涜浼犺緭鐨?DMA 鎺у埗鍣ㄣ€傞拡瀵硅棰戞崟鑾峰拰鏄剧ず
璺緞鍒嗗埆瀛樺湪鍚勭涓撶敤 DMA 閫氶亾銆傚湪浼犺緭杩囩▼涓紝IDMAC 杩樿兘澶熻繘琛屽瀭鐩村浘鍍忕炕杞€?8x8 鍧椾紶杈擄紙鍙傝 IRT 鎻忚堪锛夈€佸悓涓€鑹插僵绌洪棿鍐呯殑鍍忕礌鍒嗛噺閲嶆帓搴忥紙渚嬪 UYVY 鍒?YUYV锛夛紝浠ュ強鎵撳寘锛坧acked锛?--> 骞抽潰锛坧lanar锛夎浆鎹€侷DMAC 杩樺彲浠ュ湪浼犺緭鏃堕€氳繃
浜ら敊鍋舵暟琛屽拰濂囨暟琛岀殑鏂瑰紡鎵ц绠€鍗曠殑鍘婚殧琛岋紙涓嶅甫鏈夐渶瑕?VDIC 鏀寔鐨勮繍鍔ㄨˉ鍋匡級銆?
CSI 鏄悗绔崟鑾峰崟鍏冿紝閫氳繃骞惰锛圥arallel锛夈€丅T.656/1120 鍜?MIPI CSI-2 鎬荤嚎
鐩存帴涓庢憚鍍忓ご浼犳劅鍣ㄦ帴鍙ｃ€?
IC 璐熻矗鑹插僵绌洪棿杞崲銆佺缉鏀撅紙缂╁皬鍜屾斁澶э級銆佹按骞崇炕杞互鍙?90/270 搴︽棆杞搷浣溿€?
IC 鍐呴儴鏈変笁涓彲骞跺彂鎵ц杞崲鐨勭嫭绔嬧€滀换鍔♀€濓細棰勫鐞嗗櫒缂栫爜锛坧re-process
encoding锛夈€侀澶勭悊鍣ㄥ彇鏅櫒锛坧re-process viewfinder锛夊拰鍚庡鐞嗭紙post-processing锛夈€?鍦ㄦ瘡涓换鍔″唴锛岃浆鎹㈣鍒嗕负涓変釜閮ㄥ垎锛氱缉灏忛儴鍒嗐€佷富澶勭悊閮ㄥ垎锛堟斁澶с€佺炕杞€佽壊褰╃┖闂?杞崲浠ュ強鍥惧舰骞抽潰鍚堟垚锛夊拰鏃嬭浆閮ㄥ垎銆?
IPU 浠ユ椂闂寸墖鏂瑰紡鍏变韩 IC 浠诲姟鎿嶄綔銆傛椂闂寸墖绮掑害鍦ㄧ缉灏忛儴鍒嗕负涓€娆＄獊鍙?8 涓儚绱狅紝
鍦ㄤ富澶勭悊閮ㄥ垎涓轰竴琛屽浘鍍忥紝鍦ㄦ棆杞儴鍒嗕负涓€甯у浘鍍忋€?
SMFC 鐢卞洓涓嫭绔嬬殑 FIFO 缁勬垚锛屾瘡涓?FIFO 閮藉彲浠ラ€氳繃鍥涗釜 IDMAC 閫氶亾骞跺彂鍦板皢
鎹曡幏鐨勫抚浠庝紶鎰熷櫒鐩存帴浼犻€佸埌鍐呭瓨銆?
IRT 鎵ц 90 搴﹀拰 270 搴﹀浘鍍忔棆杞搷浣溿€傝鏃嬭浆鎿嶄綔姣忔鍦?8x8 鍍忕礌鍧椾笂杩涜銆傝
鎿嶄綔鐢?IDMAC 閰嶅悎瀹屾垚锛孖DMAC 璐熻矗 8x8 鍧椾紶杈撲互鍙婂潡閲嶆帓搴忥紝骞朵笌鍨傜洿缈昏浆鍗忓悓
宸ヤ綔銆?
VDIC 璐熻矗灏嗛殧琛岃棰戣浆鎹负閫愯瑙嗛锛屾敮鎸佷笉鍚岀殑杩愬姩琛ュ伩妯″紡锛堜綆銆佷腑銆侀珮杩愬姩锛夈€?VDIC 鍘婚殧琛屽悗鐨勮緭鍑哄抚鍙互鍙戦€佸埌 IC 棰勫鐞嗗櫒鍙栨櫙鍣ㄤ换鍔″仛杩涗竴姝ヨ浆鎹€俈DIC 杩?鍖呭惈涓€涓悎鎴愬櫒锛圕ombiner锛夛紝鍙娇鐢?Alpha 娣峰悎鍜岃壊褰╅敭鎺у皢涓ゅ箙鍥惧儚骞抽潰鍚堟垚
鍦ㄤ竴璧枫€?
闄や簡 IPU 鍐呴儴瀛愬崟鍏冨锛宨.MX 涓婅繕鏈変袱涓綅浜?IPU 澶栭儴鐨勫崟鍏冧篃鍙備笌瑙嗛鎹曡幏锛?
- 鐢ㄤ簬甯?MIPI CSI-2 鎬荤嚎鎺ュ彛鐨勬憚鍍忓ご浼犳劅鍣ㄧ殑 MIPI CSI-2 鎺ユ敹鍣ㄣ€傝繖鏄竴涓?  Synopsys DesignWare 鏍稿績銆?- 涓や釜瑙嗛澶氳矾澶嶇敤鍣紝鐢ㄤ簬鍦ㄥ涓紶鎰熷櫒杈撳叆涔嬮棿閫夋嫨骞跺彂閫佸埌鏌愪釜 CSI銆?
鏇村淇℃伅锛岃鍙傝€冩渶鏂扮増鏈殑 i.MX5/6 鍙傝€冩墜鍐?[#f1]_ 鍜?[#f2]_銆?
### 鐗规€?
鏈┍鍔ㄧ殑閮ㄥ垎鐗规€у寘鎷細

- 鍙€氳繃 media controller API 閰嶇疆璁稿涓嶅悓鐨勬祦姘寸嚎锛坧ipeline锛夛紝瀹冧滑瀵瑰簲 i.MX
  涓敮鎸佺殑纭欢瑙嗛鎹曡幏娴佹按绾裤€?
- 鏀寔骞惰銆丅T.565 浠ュ強 MIPI CSI-2 鎺ュ彛銆?
- 閫氳繃閰嶇疆娴佹按绾垮埌澶氫釜瑙嗛鎹曡幏鎺ュ彛锛堜娇鐢ㄧ嫭绔嬬殑瀹炰綋锛夛紝鏀寔骞跺彂鐨勭嫭绔嬫暟鎹祦銆?
- 閫氳繃 IC 浠诲姟瀛愯澶囷紙subdev锛夊疄鐜扮缉鏀俱€佽壊褰╃┖闂磋浆鎹€佹按骞冲拰鍨傜洿缈昏浆浠ュ強
  鍥惧儚鏃嬭浆銆?
- 鏀寔澶氱鍍忕礌鏍煎紡锛圧GB銆佹墦鍖呭拰骞抽潰 YUV銆侀儴鍒嗗钩闈?YUV锛夈€?
- VDIC 瀛愯澶囨敮鎸佽繍鍔ㄨˉ鍋垮幓闅旇锛屽叿鏈変笁绉嶈繍鍔ㄨˉ鍋挎ā寮忥細浣庛€佷腑銆侀珮杩愬姩銆傚畾涔変簡
  鍏佽浠?CSI 鐩存帴鍚?VDIC 瀛愯澶囧彂閫佸抚鐨勬祦姘寸嚎銆傛湭鏉ヨ繕鏀寔閫氳繃杈撳嚭/鍐呭瓨鍒板唴瀛?  锛坢em2mem锛夎澶囦粠鍐呭瓨缂撳啿鍖哄悜 VDIC 鍙戦€佸抚銆?
- 鍖呭惈涓€涓抚闂撮殧鐩戣鍣紙Frame Interval Monitor锛孎IM锛夛紝鍙互绾犳 ADV718x
  瑙嗛瑙ｇ爜鍣ㄧ殑鍨傜洿鍚屾闂銆?
### 鎷撴墤缁撴瀯

涓嬮潰灞曠ず浜?i.MX6Q SabreSD 鍜?i.MX6Q SabreAuto 鐨?media 鎷撴墤缁撴瀯銆傝鍙傝€冧笅涓€
灏忚妭瀹炰綋鎻忚堪涓殑杩欎簺鍥俱€?
i.MX5/6 鐨勬嫇鎵戠粨鏋勫湪 IPUv3 CSI 瑙嗛澶氳矾澶嶇敤鍣ㄤ笂娓稿彲鑳芥湁鎵€涓嶅悓锛屼絾浠庨偅閲屽線涓嬬殑
鍐呴儴 IPUv3 鎷撴墤瀵规墍鏈?i.MX5/6 骞冲彴閮芥槸閫氱敤鐨勩€備緥濡傦紝甯?MIPI CSI-2 OV5640 浼犳劅鍣ㄧ殑
SabreSD 闇€瑕?i.MX6 MIPI CSI-2 鎺ユ敹鍣ㄣ€傝€?SabreAuto 鍦ㄥ苟琛?bt.656 鎬荤嚎涓婂彧鏈?ADV7180 瑙ｇ爜鍣紝鍥犳涓嶉渶瑕?MIPI CSI-2 鎺ユ敹鍣紝鎵€浠ュ湪瀹冪殑鍥句腑娌℃湁璇ラ儴鍒嗐€?
    :alt:   Diagram of the i.MX6Q SabreSD media pipeline topology
    :align: center

    Media pipeline graph on i.MX6Q SabreSD

    :alt:   Diagram of the i.MX6Q SabreAuto media pipeline topology
    :align: center

    Media pipeline graph on i.MX6Q SabreAuto

### 瀹炰綋

### imx6-mipi-csi2

杩欐槸 MIPI CSI-2 鎺ユ敹鍣ㄥ疄浣撱€傚畠鏈変竴涓?sink 绠¤剼锛坧ad锛夌敤浜庢帴鏀?MIPI CSI-2 娴?锛堥€氬父鏉ヨ嚜 MIPI CSI-2 鎽勫儚澶翠紶鎰熷櫒锛夈€傚畠鏈夊洓涓?source 绠¤剼锛屽搴斿洓涓?MIPI CSI-2
瑙ｅ鐢紙demuxed锛夌殑铏氭嫙閫氶亾杈撳嚭銆傚彲浠ュ惎鐢ㄥ涓?source 绠¤剼浠ヤ粠澶氫釜铏氭嫙閫氶亾
鐙珛鍦拌繘琛屾祦浼犺緭銆?
璇ュ疄浣撳疄闄呬笂鐢变袱涓瓙鍧楃粍鎴愩€備竴涓槸 MIPI CSI-2 鏍稿績锛岃繖鏄竴涓?Synopsys
Designware MIPI CSI-2 鏍稿績銆傚彟涓€涓瓙鍧楁槸鈥淐SI-2 鍒?IPU 鐨勫灚鐗囷紙gasket锛夆€濄€傝
鍨墖鍏呭綋鍥涗釜铏氭嫙閫氶亾娴佺殑瑙ｅ鐢ㄥ櫒锛屾彁渚涘洓鏉＄嫭绔嬬殑骞惰鎬荤嚎锛屾瘡鏉″寘鍚悇鑷殑
铏氭嫙閫氶亾锛屽苟濡備笂鎵€杩拌矾鐢卞埌 CSI 鎴栬棰戝璺鐢ㄥ櫒銆?
鍦?i.MX6 solo/dual-lite 涓婏紝鍏ㄩ儴鍥涗釜铏氭嫙閫氶亾鎬荤嚎閮借璺敱鍒颁袱涓棰戝璺鐢ㄥ櫒銆?CSI0 鍜?CSI1 閮藉彲浠ラ€氳繃瑙嗛澶氳矾澶嶇敤鍣ㄩ€夋嫨鎺ユ敹浠绘剰铏氭嫙閫氶亾銆?
鍦?i.MX6 Quad 涓婏紝铏氭嫙閫氶亾 0 璺敱鍒?IPU1-CSI0锛堢粡杩囪棰戝璺鐢ㄥ櫒閫夋嫨锛夛紝铏氭嫙
閫氶亾 1 鍜?2 鍒嗗埆纭繛绾垮埌 IPU1-CSI1 鍜?IPU2-CSI0锛岃櫄鎷熼€氶亾 3 璺敱鍒?IPU2-CSI1
锛堝悓鏍风粡杩囪棰戝璺鐢ㄥ櫒閫夋嫨锛夈€?
### ipuX_csiY_mux

杩欎簺鏄棰戝璺鐢ㄥ櫒銆傚畠浠湁涓や釜鎴栨洿澶?sink 绠¤剼锛岀敤浜庝粠甯﹀苟琛屾帴鍙ｇ殑鎽勫儚澶?浼犳劅鍣ㄩ€夋嫨锛屾垨浠?imx6-mipi-csi2 瀹炰綋鐨?MIPI CSI-2 铏氭嫙閫氶亾閫夋嫨銆傚畠浠湁涓€涓?鍗曠嫭鐨?source 绠¤剼锛岃矾鐢卞埌鏌愪釜 CSI锛坕puX_csiY 瀹炰綋锛夈€?
鍦?i.MX6 solo/dual-lite 涓婏紝鏈変袱涓棰戝璺鐢ㄥ櫒瀹炰綋銆備竴涓綅浜?IPU1-CSI0 涔嬪墠锛?鐢ㄤ簬鍦ㄥ苟琛屼紶鎰熷櫒鍜屽洓涓?MIPI CSI-2 铏氭嫙閫氶亾涓换閫夊叾涓€锛堝叡浜斾釜 sink 绠¤剼锛夈€傚彟涓€涓?澶氳矾澶嶇敤鍣ㄤ綅浜?IPU1-CSI1 涔嬪墠锛屽悓鏍锋湁浜斾釜 sink 绠¤剼锛岀敤浜庡湪骞惰浼犳劅鍣ㄥ拰鍥涗釜
MIPI CSI-2 铏氭嫙閫氶亾涓换閫夊叾涓€銆?
鍦?i.MX6 Quad 涓婏紝鏈変袱涓棰戝璺鐢ㄥ櫒瀹炰綋銆備竴涓綅浜?IPU1-CSI0 涔嬪墠锛岀敤浜庡湪
骞惰浼犳劅鍣ㄥ拰 MIPI CSI-2 铏氭嫙閫氶亾 0 涔嬮棿閫夋嫨锛堜袱涓?sink 绠¤剼锛夈€傚彟涓€涓璺鐢ㄥ櫒
浣嶄簬 IPU2-CSI1 涔嬪墠锛岀敤浜庡湪骞惰浼犳劅鍣ㄥ拰 MIPI CSI-2 铏氭嫙閫氶亾 3 涔嬮棿閫夋嫨锛堜袱涓?sink 绠¤剼锛夈€?
### ipuX_csiY

杩欎簺鏄?CSI 瀹炰綋銆傚畠浠湁涓€涓崟鐙殑 sink 绠¤剼锛屽涓婃墍杩颁粠瑙嗛澶氳矾澶嶇敤鍣ㄦ垨 MIPI
CSI-2 铏氭嫙閫氶亾鎺ユ敹銆?
璇ュ疄浣撴湁涓や釜 source 绠¤剼銆傜涓€涓?source 绠¤剼鍙互浣跨敤纭欢閾捐矾鐩存帴閾炬帴鍒?ipuX_vdic 瀹炰綋鎴?ipuX_ic_prp 瀹炰綋锛岃繖绉嶉摼鎺ヤ笉闇€瑕?IDMAC 鍐呭瓨缂撳啿鍖轰紶杈撱€?
褰撶洿鎺?source 绠¤剼璺敱鍒?ipuX_ic_prp 瀹炰綋鏃讹紝鏉ヨ嚜 CSI 鐨勫抚鍙互鐢变竴涓垨涓や釜 IC
棰勫鐞嗕换鍔″鐞嗐€?
褰撶洿鎺?source 绠¤剼璺敱鍒?ipuX_vdic 瀹炰綋鏃讹紝VDIC 灏嗕娇鐢ㄢ€滈珮杩愬姩鈥濇ā寮忔墽琛岃繍鍔?琛ュ伩鍘婚殧琛岋紙鍙傝 ipuX_vdic 瀹炰綋鎻忚堪锛夈€?
绗簩涓?source 绠¤剼閫氳繃 SMFC 鍜屾煇涓?IDMAC 閫氶亾灏嗚棰戝抚鐩存帴鍙戦€佸埌鍐呭瓨缂撳啿鍖猴紝
缁曡繃 IC 棰勫鐞嗐€傝 source 绠¤剼璺敱鍒颁竴涓崟鑾疯澶囪妭鐐癸紝鑺傜偣鍚嶇О鏍煎紡涓?鈥渋puX_csiY capture鈥濄€?
娉ㄦ剰锛岀敱浜?IDMAC source 绠¤剼浣跨敤浜?IDMAC 閫氶亾锛屽洜姝ゅ悓涓€鑹插僵绌洪棿鍐呯殑鍍忕礌閲嶆帓搴?鍙互鐢?IDMAC 閫氶亾瀹屾垚銆備緥濡傦紝濡傛灉 CSI sink 绠¤剼浠?UYVY 椤哄簭鎺ユ敹锛屽垯閾炬帴鍒?IDMAC
source 绠¤剼鐨勬崟鑾疯澶囧彲浠ヤ互 YUYV 椤哄簭鎹曡幏銆傛澶栵紝濡傛灉 CSI sink 绠¤剼鎺ユ敹鐨勬槸
鎵撳寘锛坧acked锛塝UV 鏍煎紡锛屽垯鎹曡幏璁惧鍙互鎹曡幏骞抽潰锛坧lanar锛塝UV 鏍煎紡锛屼緥濡?YUV420銆?
IDMAC source 绠¤剼澶勭殑 IDMAC 閫氶亾杩樻敮鎸佹棤杩愬姩琛ュ伩鐨勭畝鍗曚氦缁囷紙interweave锛夛紝褰?source 绠¤剼鐨勫満锛坒ield锛夌被鍨嬩负椤哄簭椤?搴曪紙sequential top-bottom锛夋垨搴?椤?锛坆ottom-top锛夛紝涓旇姹傜殑鎹曡幏鎺ュ彛鍦虹被鍨嬭缃负闅旇锛坕nterlaced锛宼-b銆乥-t 鎴栨湭
闄愬畾闅旇锛夋椂婵€娲汇€傛崟鑾锋帴鍙ｅ皢寮哄埗閲囩敤涓?source 绠¤剼鐩稿悓鐨勫満椤哄簭锛堝鏋?source
绠¤剼涓?seq-bt锛屽垯涓?interlaced-bt锛涘鏋?source 绠¤剼涓?seq-tb锛屽垯涓?interlaced-tb锛夈€?
鍏充簬 ipuX_csiY 浜х敓鐨勪簨浠讹紝璇峰弬瑙?ref:`imx_api_ipuX_csiY`銆?
### ipuX_csiY 涓殑瑁佸壀

CSI 鏀寔瀵硅緭鍏ョ殑鍘熷浼犳劅鍣ㄥ抚杩涜瑁佸壀銆傝繖鍦?ipuX_csiY 瀹炰綋鐨?sink 绠¤剼澶勯€氳繃
crop selection 瀛愯澶?API 瀹炵幇銆?
CSI 杩樻敮鎸佸湪瀹藉害鍜岄珮搴︿笂鐙珛鐨勫浐瀹氫簩鍒嗭紙divide-by-two锛夌缉灏忋€傝繖鍦?ipuX_csiY
瀹炰綋鐨?sink 绠¤剼澶勯€氳繃 compose selection 瀛愯澶?API 瀹炵幇銆?
ipuX_csiY source 绠¤剼澶勭殑杈撳嚭鐭╁舰涓?sink 绠¤剼澶勭殑 compose 鐭╁舰鐩稿悓銆傚洜姝?source
绠¤剼鐭╁舰鏃犳硶杩涜鍗忓晢锛屽繀椤讳娇鐢?sink 绠¤剼澶勭殑 compose selection API 鏉ヨ缃紙濡傛灉
闇€瑕?/2 缂╁皬锛涘惁鍒?source 绠¤剼鐭╁舰绛変簬杈撳叆鐭╁舰锛夈€?
浣滀负 crop 鍜?/2 缂╁皬鐨勭ず渚嬶紝杩欎細灏嗕竴涓?1280x960 鐨勮緭鍏ュ抚瑁佸壀涓?640x480锛岀劧鍚?鍦ㄤ袱涓淮搴︿笂 /2 缂╁皬鍒?320x240锛堝亣璁?ipu1_csi0 閾炬帴鍒?ipu1_csi0_mux锛夛細

   media-ctl -V "'ipu1_csi0_mux':2[fmt:UYVY2X8/1280x960]"
   media-ctl -V "'ipu1_csi0':0[crop:(0,0)/640x480]"
   media-ctl -V "'ipu1_csi0':0[compose:(0,0)/320x240]"

### ipuX_csiY 涓殑璺冲抚

CSI 鏀寔閫氳繃璺冲抚杩涜甯х巼鎶藉彇锛坒rame rate decimation锛夈€傚抚鐜囨娊鍙栭€氳繃鍦?sink 鍜?source 绠¤剼璁剧疆甯ч棿闅旀潵鎸囧畾銆傜劧鍚?ipuX_csiY 瀹炰綋灏嗘渶浣宠烦甯ц缃簲鐢ㄥ埌 CSI锛屼互鍦?source 绠¤剼杈惧埌鏈熸湜鐨勫抚鐜囥€?
浠ヤ笅绀轰緥灏?IDMAC 杈撳嚭 source 绠¤剼涓婂亣璁剧殑 60 Hz 杈撳叆甯х巼鍑忓崐锛?
   media-ctl -V "'ipu1_csi0':0[fmt:UYVY2X8/640x480@1/60]"
   media-ctl -V "'ipu1_csi0':2[fmt:UYVY2X8/640x480@1/30]"

### ipuX_csiY 涓殑甯ч棿闅旂洃瑙嗗櫒

璇峰弬瑙?ref:`imx_api_FIM`銆?
### ipuX_vdic

VDIC 鎵ц杩愬姩琛ュ伩鍘婚殧琛岋紝鍏锋湁涓夌杩愬姩琛ュ伩妯″紡锛氫綆銆佷腑銆侀珮杩愬姩銆傛ā寮忛€氳繃鑿滃崟
鎺т欢 V4L2_CID_DEINTERLACING_MODE 鎸囧畾銆俈DIC 鏈変袱涓?sink 绠¤剼鍜屼竴涓崟鐙殑
source 绠¤剼銆?
鐩存帴 sink 绠¤剼浠?ipuX_csiY 鐩存帴绠¤剼鎺ユ敹銆備娇鐢ㄨ閾炬帴鏃讹紝VDIC 鍙兘浠ラ珮杩愬姩妯″紡
杩愯銆?
褰?IDMAC sink 绠¤剼琚縺娲绘椂锛屽畠浠庤緭鍑烘垨 mem2mem 璁惧鑺傜偣鎺ユ敹銆備娇鐢ㄨ娴佹按绾挎椂锛?VDIC 涔熷彲浠ヤ互浣庡拰涓ā寮忚繍琛岋紝鍥犱负杩欎簺妯″紡闇€瑕佷粠鍐呭瓨缂撳啿鍖烘帴鏀跺抚銆傛敞鎰忥紝杈撳嚭
鎴?mem2mem 璁惧灏氭湭瀹炵幇锛屽洜姝よ sink 绠¤剼褰撳墠娌℃湁浠讳綍閾炬帴銆?
source 绠¤剼璺敱鍒?IC 棰勫鐞嗗疄浣?ipuX_ic_prp銆?
### ipuX_ic_prp

杩欐槸 IC 棰勫鐞嗗疄浣撱€傚畠鍏呭綋璺敱鍣紝灏嗗叾 sink 绠¤剼鐨勬暟鎹矾鐢卞埌鍏朵竴涓垨涓や釜 source
绠¤剼銆?
璇ュ疄浣撴湁涓€涓崟鐙殑 sink 绠¤剼銆俿ink 绠¤剼鍙互浠?ipuX_csiY 鐩存帴绠¤剼鎴?ipuX_vdic
鎺ユ敹銆?
璇ュ疄浣撴湁涓や釜 source 绠¤剼銆備竴涓?source 绠¤剼璺敱鍒伴澶勭悊鍣ㄧ紪鐮佷换鍔″疄浣?锛坕puX_ic_prpenc锛夛紝鍙︿竴涓矾鐢卞埌棰勫鐞嗗櫒鍙栨櫙鍣ㄤ换鍔″疄浣擄紙ipuX_ic_prpvf锛夈€傚鏋?sink 绠¤剼浠?ipuX_csiY 鎺ユ敹锛屽垯涓や釜 source 绠¤剼鍙互鍚屾椂婵€娲汇€傚鏋?sink 绠¤剼浠?ipuX_vdic 鎺ユ敹锛屽垯鍙兘婵€娲诲埌棰勫鐞嗗櫒鍙栨櫙鍣ㄤ换鍔″疄浣撶殑 source 绠¤剼锛堟潵鑷?VDIC 鐨?甯у彧鑳界敱棰勫鐞嗗櫒鍙栨櫙鍣ㄤ换鍔″鐞嗭級銆?
### ipuX_ic_prpenc

杩欐槸 IC 棰勫鐞嗙紪鐮佸疄浣撱€傚畠鏈変竴涓潵鑷?ipuX_ic_prp 鐨勫崟鐙?sink 绠¤剼锛屼互鍙婁竴涓?鍗曠嫭鐨?source 绠¤剼銆俿ource 绠¤剼璺敱鍒颁竴涓崟鑾疯澶囪妭鐐癸紝鑺傜偣鍚嶇О鏍煎紡涓?鈥渋puX_ic_prpenc capture鈥濄€?
璇ュ疄浣撴墽琛?IC 棰勫鐞嗙紪鐮佷换鍔℃搷浣滐細鑹插僵绌洪棿杞崲銆佺缉鏀撅紙缂╁皬鍜屾斁澶э級銆佹按骞冲拰鍨傜洿
缈昏浆浠ュ強 90/270 搴︽棆杞€傜炕杞拰鏃嬭浆閫氳繃鏍囧噯 V4L2 鎺т欢鎻愪緵銆?
涓?ipuX_csiY IDMAC source 绫讳技锛岃瀹炰綋涔熸敮鎸佹棤杩愬姩琛ュ伩鐨勭畝鍗曞幓闅旇锛屼互鍙婂儚绱?閲嶆帓搴忋€?
### ipuX_ic_prpvf

杩欐槸 IC 棰勫鐞嗗彇鏅櫒瀹炰綋銆傚畠鏈変竴涓潵鑷?ipuX_ic_prp 鐨勫崟鐙?sink 绠¤剼锛屼互鍙婁竴涓?鍗曠嫭鐨?source 绠¤剼銆俿ource 绠¤剼璺敱鍒颁竴涓崟鑾疯澶囪妭鐐癸紝鑺傜偣鍚嶇О鏍煎紡涓?鈥渋puX_ic_prpvf capture鈥濄€?
璇ュ疄浣撶殑鎿嶄綔涓?ipuX_ic_prpenc 鐩稿悓锛屽叿鏈夌浉鍚岀殑缂╂斁鍜?CSC 鎿嶄綔浠ュ強缈昏浆/鏃嬭浆
鎺т欢銆傚鏋?ipuX_ic_prp 浠?ipuX_vdic 鎺ユ敹锛屽畠灏嗘帴鏀跺苟澶勭悊鏉ヨ嚜 ipuX_vdic 鐨勫幓闅旇
甯с€?
涓?ipuX_csiY IDMAC source 绫讳技锛岃瀹炰綋鏀寔鏃犺繍鍔ㄨˉ鍋跨殑绠€鍗曚氦缁囷紙interweaving锛夈€?浣嗘槸璇锋敞鎰忥紝濡傛灉 ipuX_vdic 鍖呭惈鍦ㄦ祦姘寸嚎涓紙ipuX_ic_prp 浠?ipuX_vdic 鎺ユ敹锛夛紝鍒?鏃犳硶鍦?ipuX_ic_prpvf 涓娇鐢ㄤ氦缁囷紝鍥犱负 ipuX_vdic 宸茬粡鎵ц浜嗗幓闅旇锛堝甫杩愬姩琛ュ伩锛夛紝
鍥犳 ipuX_vdic 杈撳嚭鐨勫満绫诲瀷鍙兘鏄?none锛堥€愯锛夈€?
### 鎹曡幏娴佹按绾?
涓嬮潰鎻忚堪娴佹按绾挎敮鎸佺殑鍚勭鐢ㄤ緥銆?
鎵€绀洪摼鎺ヤ笉鍖呭惈鍚庣浼犳劅鍣ㄣ€佽棰戝璺鐢ㄥ櫒鎴?mipi csi-2 鎺ユ敹鍣ㄩ摼鎺ャ€傝繖鍙栧喅浜?浼犳劅鍣ㄦ帴鍙ｇ被鍨嬶紙骞惰鎴?mipi csi-2锛夈€傚洜姝よ繖浜涙祦姘寸嚎浠庝互涓嬪唴瀹瑰紑濮嬶細

sensor -> ipuX_csiY_mux -> ...

鐢ㄤ簬骞惰浼犳劅鍣紝鎴栵細

sensor -> imx6-mipi-csi2 -> (ipuX_csiY_mux) -> ...

鐢ㄤ簬 mipi csi-2 浼犳劅鍣ㄣ€傝 mipi csi-2 铏氭嫙閫氶亾鑰屽畾锛宨mx6-mipi-csi2 鎺ユ敹鍣ㄥ彲鑳介渶瑕?鍏堣矾鐢卞埌瑙嗛澶氳矾澶嶇敤鍣紙ipuX_csiY_mux锛夊啀鍙戦€佸埌 CSI锛屽洜姝?ipuX_csiY_mux 鐢?鎷彿琛ㄧず銆?
### 鏈鐞嗚棰戞崟鑾凤細

閫氳繃 ipuX_csiY IDMAC source 绠¤剼锛屽皢甯т粠浼犳劅鍣ㄧ洿鎺ュ彂閫佸埌鎽勫儚澶磋澶囨帴鍙ｈ妭鐐癸紝
涓嶅仛浠讳綍杞崲锛?
-> ipuX_csiY:2 -> ipuX_csiY capture

### IC 鐩存帴杞崲锛?
璇ユ祦姘寸嚎浣跨敤棰勫鐞嗙紪鐮佸疄浣撳皢甯х洿鎺ヤ粠 CSI 璺敱鍒?IC锛屼互鎵ц鏈€楂?1024x1024
鍒嗚鲸鐜囩殑缂╂斁銆丆SC銆佺炕杞互鍙婂浘鍍忔棆杞細

-> ipuX_csiY:1 -> 0:ipuX_ic_prp:1 -> 0:ipuX_ic_prpenc:1 -> ipuX_ic_prpenc capture

### 杩愬姩琛ュ伩鍘婚殧琛岋細

璇ユ祦姘寸嚎灏嗗抚浠?CSI 鐩存帴绠¤剼璺敱鍒?VDIC 瀹炰綋锛屼互鏀寔杩愬姩琛ュ伩鍘婚殧琛岋紙浠呴珮杩愬姩
妯″紡锛夈€佹渶楂?1024x1024 鐨勭缉鏀俱€丆SC銆佺炕杞互鍙婃棆杞細

-> ipuX_csiY:1 -> 0:ipuX_vdic:2 -> 0:ipuX_ic_prp:2 -> 0:ipuX_ic_prpvf:1 -> ipuX_ic_prpvf capture

### 浣跨敤璇存槑

涓轰簡杈呭姪閰嶇疆骞朵笌鍙粠瑙嗛璁惧鑺傜偣璁块棶鎺т欢锛坈ontrol锛夌殑 V4L2 搴旂敤鍚戝悗鍏煎锛屾崟鑾?璁惧鎺ュ彛浼氱户鎵垮綋鍓嶆祦姘寸嚎涓椿鍔ㄥ疄浣撶殑鎺т欢锛屽洜姝ゆ帶浠舵棦鍙互鐩存帴浠庡瓙璁惧璁块棶锛屼篃
鍙互浠庢椿鍔ㄦ崟鑾疯澶囨帴鍙ｈ闂€備緥濡傦紝FIM 鎺т欢鏃㈠彲浠?ipuX_csiY 瀛愯澶囪幏寰楋紝涔熷彲浠?娲诲姩鎹曡幏璁惧鑾峰緱銆?
浠ヤ笅鏄拡瀵?Sabre* 鍙傝€冩澘鐨勫叿浣撲娇鐢ㄨ鏄庯細

### 甯?OV5642 鍜?OV5640 鐨?i.MX6Q SabreLite

璇ュ钩鍙伴渶瑕佸甫骞惰鎽勫儚澶存帴鍙ｇ殑 OmniVision OV5642 妯″潡锛屼互鍙婂甫 MIPI CSI-2 鎺ュ彛鐨?OV5640 妯″潡銆備袱涓ā鍧楀潎鍙粠 Boundary Devices 鑾峰緱锛?
- https://boundarydevices.com/product/nit6x_5mp
- https://boundarydevices.com/product/nit6x_5mp_mipi

娉ㄦ剰锛屽鏋滃彧鏈変竴涓憚鍍忓ご妯″潡鍙敤锛屽垯鍙互鍦ㄨ澶囨爲涓鐢ㄥ彟涓€涓紶鎰熷櫒鑺傜偣銆?
OV5642 妯″潡杩炴帴鍒?i.MX 鍐呴儴瑙嗛澶氳矾澶嶇敤鍣ㄥ埌 IPU1 CSI0 鐨勫苟琛屾€荤嚎杈撳叆銆傚畠鐨?i2c
鎬荤嚎杩炴帴鍒?i2c 鎬荤嚎 2銆?
MIPI CSI-2 OV5640 妯″潡杩炴帴鍒?i.MX 鍐呴儴 MIPI CSI-2 鎺ユ敹鍣紝鏉ヨ嚜鎺ユ敹鍣ㄧ殑鍥涗釜铏氭嫙
閫氶亾杈撳嚭璺敱濡備笅锛歷c0 鍒?IPU1 CSI0 澶氳矾澶嶇敤鍣紝vc1 鐩存帴鍒?IPU1 CSI1锛寁c2 鐩存帴
鍒?IPU2 CSI0锛寁c3 鍒?IPU2 CSI1 澶氳矾澶嶇敤鍣ㄣ€侽V5640 涔熻繛鎺ュ埌 SabreLite 涓婄殑 i2c
鎬荤嚎 2锛屽洜姝?OV5642 鍜?OV5640 涓嶈兘鍏变韩鐩稿悓鐨?i2c 浠庡湴鍧€銆?
浠ヤ笅鍩烘湰绀轰緥涓轰袱涓紶鎰熷櫒閰嶇疆鏈鐞嗚棰戞崟鑾锋祦姘寸嚎銆侽V5642 璺敱鍒?ipu1_csi0锛?鑰岄€氳繃 MIPI CSI-2 铏氭嫙閫氶亾 1锛堝嵆 imx6-mipi-csi2 绠¤剼 2锛変紶杈撶殑 OV5640 璺敱鍒?ipu1_csi1銆備袱涓紶鎰熷櫒閮介厤缃负杈撳嚭 640x480锛孫V5642 杈撳嚭 YUYV2X8锛孫V5640 杈撳嚭
UYVY2X8锛?
   # Setup links for OV5642
   media-ctl -l "'ov5642 1-0042':0 -> 'ipu1_csi0_mux':1[^1^]"
   media-ctl -l "'ipu1_csi0_mux':2 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':2 -> 'ipu1_csi0 capture':0[^1^]"
   # Setup links for OV5640
   media-ctl -l "'ov5640 1-0040':0 -> 'imx6-mipi-csi2':0[^1^]"
   media-ctl -l "'imx6-mipi-csi2':2 -> 'ipu1_csi1':0[^1^]"
   media-ctl -l "'ipu1_csi1':2 -> 'ipu1_csi1 capture':0[^1^]"
   # Configure pads for OV5642 pipeline
   media-ctl -V "'ov5642 1-0042':0 [fmt:YUYV2X8/640x480 field:none]"
   media-ctl -V "'ipu1_csi0_mux':2 [fmt:YUYV2X8/640x480 field:none]"
   media-ctl -V "'ipu1_csi0':2 [fmt:AYUV32/640x480 field:none]"
   # Configure pads for OV5640 pipeline
   media-ctl -V "'ov5640 1-0040':0 [fmt:UYVY2X8/640x480 field:none]"
   media-ctl -V "'imx6-mipi-csi2':2 [fmt:UYVY2X8/640x480 field:none]"
   media-ctl -V "'ipu1_csi1':2 [fmt:AYUV32/640x480 field:none]"

鐒跺悗鍙互鍦ㄦ崟鑾疯澶囪妭鐐光€渋pu1_csi0 capture鈥濆拰鈥渋pu1_csi1 capture鈥濅笂鐙珛寮€濮?娴佷紶杈撱€倂4l2-ctl 宸ュ叿鍙敤浜庡湪鎹曡幏璁惧鑺傜偣涓婇€夋嫨浠讳綍鍙楁敮鎸佺殑 YUV 鍍忕礌鏍煎紡锛?鍖呮嫭骞抽潰鏍煎紡銆?
### 甯?ADV7180 瑙ｇ爜鍣ㄧ殑 i.MX6Q SabreAuto

鍦?i.MX6Q SabreAuto 涓婏紝鏉胯浇 ADV7180 SD 瑙ｇ爜鍣ㄨ繛鎺ュ埌鍐呴儴瑙嗛澶氳矾澶嶇敤鍣ㄥ埌 IPU1
CSI0 鐨勫苟琛屾€荤嚎杈撳叆銆?
浠ヤ笅绀轰緥閰嶇疆涓€鏉℃祦姘寸嚎锛屼互浠?ADV7180 瑙嗛瑙ｇ爜鍣ㄦ崟鑾凤紝鍋囪 NTSC 720x480 杈撳叆
淇″彿锛屼娇鐢ㄧ畝鍗曚氦缁囷紙鏈浆鎹笖鏃犻渶杩愬姩琛ュ伩锛夈€俛dv7180 蹇呴』杈撳嚭椤哄簭鎴栦氦鏇垮満锛圢TSC
鐨勫満绫诲瀷涓衡€渟eq-bt鈥濓紝鎴栤€渁lternate鈥濓級锛?
   # Setup links
   media-ctl -l "'adv7180 3-0021':0 -> 'ipu1_csi0_mux':1[^1^]"
   media-ctl -l "'ipu1_csi0_mux':2 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':2 -> 'ipu1_csi0 capture':0[^1^]"
   # Configure pads
   media-ctl -V "'adv7180 3-0021':0 [fmt:UYVY2X8/720x480 field:seq-bt]"
   media-ctl -V "'ipu1_csi0_mux':2 [fmt:UYVY2X8/720x480]"
   media-ctl -V "'ipu1_csi0':2 [fmt:AYUV32/720x480]"
   # Configure "ipu1_csi0 capture" interface (assumed at /dev/video4)
   v4l2-ctl -d4 --set-fmt-video=field=interlaced_bt

鐒跺悗鍙互鍦?/dev/video4 涓婂紑濮嬫祦浼犺緭銆倂4l2-ctl 宸ュ叿涔熷彲鐢ㄤ簬鍦?/dev/video4 涓婇€夋嫨
浠讳綍鍙楁敮鎸佺殑 YUV 鍍忕礌鏍煎紡銆?
姝ょず渚嬮厤缃竴鏉℃祦姘寸嚎锛屼互浠?ADV7180 瑙嗛瑙ｇ爜鍣ㄦ崟鑾凤紝鍋囪 PAL 720x576 杈撳叆淇″彿锛?浣跨敤杩愬姩琛ュ伩鍘婚殧琛屻€俛dv7180 蹇呴』杈撳嚭椤哄簭鎴栦氦鏇垮満锛圥AL 鐨勫満绫诲瀷涓衡€渟eq-tb鈥濓紝
鎴栤€渁lternate鈥濓級锛?
   # Setup links
   media-ctl -l "'adv7180 3-0021':0 -> 'ipu1_csi0_mux':1[^1^]"
   media-ctl -l "'ipu1_csi0_mux':2 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':1 -> 'ipu1_vdic':0[^1^]"
   media-ctl -l "'ipu1_vdic':2 -> 'ipu1_ic_prp':0[^1^]"
   media-ctl -l "'ipu1_ic_prp':2 -> 'ipu1_ic_prpvf':0[^1^]"
   media-ctl -l "'ipu1_ic_prpvf':1 -> 'ipu1_ic_prpvf capture':0[^1^]"
   # Configure pads
   media-ctl -V "'adv7180 3-0021':0 [fmt:UYVY2X8/720x576 field:seq-tb]"
   media-ctl -V "'ipu1_csi0_mux':2 [fmt:UYVY2X8/720x576]"
   media-ctl -V "'ipu1_csi0':1 [fmt:AYUV32/720x576]"
   media-ctl -V "'ipu1_vdic':2 [fmt:AYUV32/720x576 field:none]"
   media-ctl -V "'ipu1_ic_prp':2 [fmt:AYUV32/720x576 field:none]"
   media-ctl -V "'ipu1_ic_prpvf':1 [fmt:AYUV32/720x576 field:none]"
   # Configure "ipu1_ic_prpvf capture" interface (assumed at /dev/video2)
   v4l2-ctl -d2 --set-fmt-video=field=none

鐒跺悗鍙互鍦?/dev/video2 涓婂紑濮嬫祦浼犺緭銆倂4l2-ctl 宸ュ叿涔熷彲鐢ㄤ簬鍦?/dev/video2 涓婇€夋嫨
浠讳綍鍙楁敮鎸佺殑 YUV 鍍忕礌鏍煎紡銆?
璇ュ钩鍙版帴鍙?ADV7180 涓?Ain1锛堣繛鎺ュ櫒 J42锛夌殑澶嶅悎瑙嗛锛圕omposite Video锛夋ā鎷熻緭鍏ャ€?
### 甯?ADV7180 瑙ｇ爜鍣ㄧ殑 i.MX6DL SabreAuto

鍦?i.MX6DL SabreAuto 涓婏紝鏉胯浇 ADV7180 SD 瑙ｇ爜鍣ㄨ繛鎺ュ埌鍐呴儴瑙嗛澶氳矾澶嶇敤鍣ㄥ埌 IPU1
CSI0 鐨勫苟琛屾€荤嚎杈撳叆銆?
浠ヤ笅绀轰緥閰嶇疆涓€鏉℃祦姘寸嚎锛屼互浠?ADV7180 瑙嗛瑙ｇ爜鍣ㄦ崟鑾凤紝鍋囪 NTSC 720x480 杈撳叆
淇″彿锛屼娇鐢ㄧ畝鍗曚氦缁囷紙鏈浆鎹笖鏃犻渶杩愬姩琛ュ伩锛夈€俛dv7180 蹇呴』杈撳嚭椤哄簭鎴栦氦鏇垮満锛圢TSC
鐨勫満绫诲瀷涓衡€渟eq-bt鈥濓紝鎴栤€渁lternate鈥濓級锛?
   # Setup links
   media-ctl -l "'adv7180 4-0021':0 -> 'ipu1_csi0_mux':4[^1^]"
   media-ctl -l "'ipu1_csi0_mux':5 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':2 -> 'ipu1_csi0 capture':0[^1^]"
   # Configure pads
   media-ctl -V "'adv7180 4-0021':0 [fmt:UYVY2X8/720x480 field:seq-bt]"
   media-ctl -V "'ipu1_csi0_mux':5 [fmt:UYVY2X8/720x480]"
   media-ctl -V "'ipu1_csi0':2 [fmt:AYUV32/720x480]"
   # Configure "ipu1_csi0 capture" interface (assumed at /dev/video0)
   v4l2-ctl -d0 --set-fmt-video=field=interlaced_bt

鐒跺悗鍙互鍦?/dev/video0 涓婂紑濮嬫祦浼犺緭銆倂4l2-ctl 宸ュ叿涔熷彲鐢ㄤ簬鍦?/dev/video0 涓婇€夋嫨
浠讳綍鍙楁敮鎸佺殑 YUV 鍍忕礌鏍煎紡銆?
姝ょず渚嬮厤缃竴鏉℃祦姘寸嚎锛屼互浠?ADV7180 瑙嗛瑙ｇ爜鍣ㄦ崟鑾凤紝鍋囪 PAL 720x576 杈撳叆淇″彿锛?浣跨敤杩愬姩琛ュ伩鍘婚殧琛屻€俛dv7180 蹇呴』杈撳嚭椤哄簭鎴栦氦鏇垮満锛圥AL 鐨勫満绫诲瀷涓衡€渟eq-tb鈥濓紝
鎴栤€渁lternate鈥濓級锛?
   # Setup links
   media-ctl -l "'adv7180 4-0021':0 -> 'ipu1_csi0_mux':4[^1^]"
   media-ctl -l "'ipu1_csi0_mux':5 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':1 -> 'ipu1_vdic':0[^1^]"
   media-ctl -l "'ipu1_vdic':2 -> 'ipu1_ic_prp':0[^1^]"
   media-ctl -l "'ipu1_ic_prp':2 -> 'ipu1_ic_prpvf':0[^1^]"
   media-ctl -l "'ipu1_ic_prpvf':1 -> 'ipu1_ic_prpvf capture':0[^1^]"
   # Configure pads
   media-ctl -V "'adv7180 4-0021':0 [fmt:UYVY2X8/720x576 field:seq-tb]"
   media-ctl -V "'ipu1_csi0_mux':5 [fmt:UYVY2X8/720x576]"
   media-ctl -V "'ipu1_csi0':1 [fmt:AYUV32/720x576]"
   media-ctl -V "'ipu1_vdic':2 [fmt:AYUV32/720x576 field:none]"
   media-ctl -V "'ipu1_ic_prp':2 [fmt:AYUV32/720x576 field:none]"
   media-ctl -V "'ipu1_ic_prpvf':1 [fmt:AYUV32/720x576 field:none]"
   # Configure "ipu1_ic_prpvf capture" interface (assumed at /dev/video2)
   v4l2-ctl -d2 --set-fmt-video=field=none

鐒跺悗鍙互鍦?/dev/video2 涓婂紑濮嬫祦浼犺緭銆倂4l2-ctl 宸ュ叿涔熷彲鐢ㄤ簬鍦?/dev/video2 涓婇€夋嫨
浠讳綍鍙楁敮鎸佺殑 YUV 鍍忕礌鏍煎紡銆?
璇ュ钩鍙版帴鍙?ADV7180 涓?Ain1锛堣繛鎺ュ櫒 J42锛夌殑澶嶅悎瑙嗛锛圕omposite Video锛夋ā鎷熻緭鍏ャ€?
### 甯?MIPI CSI-2 OV5640 鐨?i.MX6Q SabreSD

涓?i.MX6Q SabreLite 绫讳技锛宨.MX6Q SabreSD 鍦?IPU1 CSI0 涓婃敮鎸佸苟琛屾帴鍙ｇ殑 OV5642
妯″潡锛屼互鍙?MIPI CSI-2 OV5640 妯″潡銆侽V5642 杩炴帴鍒?i2c 鎬荤嚎 1锛孫V5640 杩炴帴鍒?i2c
鎬荤嚎 2銆?
SabreSD 鐨勮澶囨爲鍖呭惈浜嗗苟琛?OV5642 鍜?MIPI CSI-2 OV5640 鐨?OF 鍥撅紙OF graphs锛夛紝
浣嗘埅鑷虫湰鏂囨挵鍐欐椂锛屼粎 MIPI CSI-2 OV5640 缁忚繃娴嬭瘯锛屽洜姝?OV5642 鑺傜偣褰撳墠琚鐢ㄣ€?OV5640 妯″潡杩炴帴鍒?MIPI 杩炴帴鍣?J5銆傝繛鎺ュ埌 SabreSD 鏉跨殑 OV5640 妯″潡鐨?NXP 閮ㄤ欢鍙?涓?H120729銆?
浠ヤ笅绀轰緥閰嶇疆鏈鐞嗚棰戞崟鑾锋祦姘寸嚎锛屼互浠庨€氳繃 MIPI CSI-2 铏氭嫙閫氶亾 0 浼犺緭鐨?OV5640
鎹曡幏锛?
   # Setup links
   media-ctl -l "'ov5640 1-003c':0 -> 'imx6-mipi-csi2':0[^1^]"
   media-ctl -l "'imx6-mipi-csi2':1 -> 'ipu1_csi0_mux':0[^1^]"
   media-ctl -l "'ipu1_csi0_mux':2 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':2 -> 'ipu1_csi0 capture':0[^1^]"
   # Configure pads
   media-ctl -V "'ov5640 1-003c':0 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'imx6-mipi-csi2':1 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'ipu1_csi0_mux':0 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'ipu1_csi0':0 [fmt:AYUV32/640x480]"

鐒跺悗鍙互鍦ㄢ€渋pu1_csi0 capture鈥濊妭鐐逛笂寮€濮嬫祦浼犺緭銆倂4l2-ctl 宸ュ叿鍙敤浜庡湪鎹曡幏璁惧
鑺傜偣涓婇€夋嫨浠讳綍鍙楁敮鎸佺殑鍍忕礌鏍煎紡銆?
瑕佺‘瀹氫笌鈥渋pu1_csi0 capture鈥濆搴旂殑 /dev/video 鑺傜偣锛?
   media-ctl -e "ipu1_csi0 capture"
   /dev/video0

/dev/video0 鏄繖绉嶆儏鍐典笅鐨勬祦浼犺緭鍏冪礌銆?
閫氳繃 v4l2-ctl 鍚姩娴佷紶杈擄細

   v4l2-ctl --stream-mmap -d /dev/video0

閫氳繃 Gstreamer 鍚姩娴佷紶杈撳苟灏嗗唴瀹瑰彂閫佸埌鏄剧ず鍣細

   gst-launch-1.0 v4l2src device=/dev/video0 ! kmssink

浠ヤ笅绀轰緥閰嶇疆涓€鏉＄洿鎺ヨ浆鎹㈡祦姘寸嚎锛屼互浠庨€氳繃 MIPI CSI-2 铏氭嫙閫氶亾 0 浼犺緭鐨?OV5640
鎹曡幏銆傚畠杩樺睍绀轰簡鍦?IC 杈撳嚭澶勭殑鑹插僵绌洪棿杞崲鍜岀缉鏀俱€?
   # Setup links
   media-ctl -l "'ov5640 1-003c':0 -> 'imx6-mipi-csi2':0[^1^]"
   media-ctl -l "'imx6-mipi-csi2':1 -> 'ipu1_csi0_mux':0[^1^]"
   media-ctl -l "'ipu1_csi0_mux':2 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':1 -> 'ipu1_ic_prp':0[^1^]"
   media-ctl -l "'ipu1_ic_prp':1 -> 'ipu1_ic_prpenc':0[^1^]"
   media-ctl -l "'ipu1_ic_prpenc':1 -> 'ipu1_ic_prpenc capture':0[^1^]"
   # Configure pads
   media-ctl -V "'ov5640 1-003c':0 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'imx6-mipi-csi2':1 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'ipu1_csi0_mux':2 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'ipu1_csi0':1 [fmt:AYUV32/640x480]"
   media-ctl -V "'ipu1_ic_prp':1 [fmt:AYUV32/640x480]"
   media-ctl -V "'ipu1_ic_prpenc':1 [fmt:ARGB8888_1X32/800x600]"
   # Set a format at the capture interface
   v4l2-ctl -d /dev/video1 --set-fmt-video=pixelformat=RGB3

鐒跺悗鍙互鍦ㄢ€渋pu1_ic_prpenc capture鈥濊妭鐐逛笂寮€濮嬫祦浼犺緭銆?
瑕佺‘瀹氫笌鈥渋pu1_ic_prpenc capture鈥濆搴旂殑 /dev/video 鑺傜偣锛?
   media-ctl -e "ipu1_ic_prpenc capture"
   /dev/video1

/dev/video1 鏄繖绉嶆儏鍐典笅鐨勬祦浼犺緭鍏冪礌銆?
閫氳繃 v4l2-ctl 鍚姩娴佷紶杈擄細

   v4l2-ctl --stream-mmap -d /dev/video1

閫氳繃 Gstreamer 鍚姩娴佷紶杈撳苟灏嗗唴瀹瑰彂閫佸埌鏄剧ず鍣細

   gst-launch-1.0 v4l2src device=/dev/video1 ! kmssink

### 宸茬煡闂

1. 褰撳湪鎺ヨ繎 IC 缂╂斁鍣?1024x1024 闄愬埗鐨勫垎杈ㄧ巼涓嬩娇鐢?90 鎴?270 搴︽棆杞帶浠讹紝
   骞朵笖涓庡钩闈㈠儚绱犳牸寮忥紙YUV420銆乊UV422p锛夌粨鍚堜娇鐢ㄦ椂锛屽抚鎹曡幏缁忓父浼氬け璐ワ紝涓?   IDMAC 閫氶亾娌℃湁甯х粨鏉熶腑鏂€傚彉閫氭柟娉曟槸锛屽湪闇€瑕?90 鎴?270 搴︽棆杞椂锛屼娇鐢ㄨ緝浣庣殑
   鍒嗚鲸鐜囧拰/鎴栨墦鍖呮牸寮忥紙YUYV銆丷GB3 绛夛級銆?
### 鏂囦欢鍒楄〃

drivers/staging/media/imx/
include/media/imx.h
include/linux/imx-media.h

### 鍙傝€冭祫鏂?
### 浣滆€?
- Steve Longerbeam <steve_longerbeam@mentor.com>
- Philipp Zabel <kernel@pengutronix.de>
- Russell King <linux@armlinux.org.uk>

Copyright (C) 2012-2017 Mentor Graphics Inc.
