## 鍐呮牳椹卞姩 coretemp

鏈〉浠嬬粛 coretemp 纭欢鐩戞帶椹卞姩锛岃鏄庡叾濡備綍璇诲彇 Intel Core/Atom 澶勭悊鍣ㄥ唴缃暟瀛楁俯搴︿紶鎰熷櫒锛圖TS锛夌殑鏍稿績涓庡皝瑁呮俯搴︼紝鍒楀嚭鎵€鏀寔鐨?CPU 鍨嬪彿锛堟寜 CPUID family 0x6 鍖哄垎锛夊強娓╁害璇诲彇鏂瑰紡锛屼緵纭欢鐩戞帶涓庢暎鐑鐞嗗弬鑰冦€?



鏀寔鐨勮姱鐗囷細
  - 鎵€鏈夊唴缃暟瀛楁俯搴︿紶鎰熷櫒锛圖TS锛夌殑 Intel Core 绯诲垪涓?Atom 澶勭悊鍣?

    鍓嶇紑锛圥refix锛夛細'coretemp'

    CPUID锛歠amily 0x6锛屽叿澶?X86_FEATURE_DTHERM 鐗规€х殑鍨嬪彿锛屽寘鎷細

       - 0xe (Pentium M DC), 0xf (Core 2 DC 65nm),
       - 0x16 (Core 2 SC 65nm), 0x17 (Penryn 45nm),
       - 0x1a (Nehalem), 0x1c (Atom), 0x1e (Lynnfield),
       - 0x26 (Tunnel Creek Atom), 0x27 (Medfield Atom),
       - 0x36 (Cedar Trail Atom), 0x37 (Bay Trail Atom),
       - 0x4a (Merrifield Atom), 0x4c (Cherry Trail Atom),
       - 0x5a (Moorefield Atom), 0x5c (Apollo Lake Atom),
       - 0x7a (Gemini Lake Atom),
       - 0x96 (Elkhart Lake Atom), 0x9c (Jasper Lake Atom)

    鏁版嵁鎵嬪唽锛圖atasheet锛夛細

	       Intel 64 and IA-32 Architectures Software Developer's Manual
	       Volume 3A: System Programming Guide

	       http://softwarecommunity.intel.com/Wiki/Mobility/720.htm

浣滆€咃紙Author锛夛細Rudolf Marek

### 鎻忚堪


璇ラ┍鍔ㄥ厑璁歌鍙栧唴缃簬 Intel CPU 涓殑 DTS锛堟暟瀛楁俯搴︿紶鎰熷櫒锛夈€傝椹卞姩鍙互浣跨敤
鐩稿簲鐨勪紶鎰熷櫒璇诲彇姣忔牳涓庢瘡灏佽鐨勬俯搴︺€傛瘡灏佽浼犳劅鍣ㄥ湪 Sandy Bridge 鍙婃墍鏈夋洿鏂?
鐨勫鐞嗗櫒涓婂彲鐢ㄣ€傝椹卞姩浼氬湪 hwmon 鍐呯殑鍗曚釜璁惧鐩綍涓嬫樉绀哄皝瑁呭唴鎵€鏈夋牳鐨勬俯搴︺€?

娓╁害浠ユ憚姘忓害搴﹂噺锛屾祴閲忓垎杈ㄧ巼涓?1 鎽勬皬搴︺€傛湁鏁堟俯搴﹁寖鍥翠负 0 鍒?TjMax 鎽勬皬搴︼紝
鍥犱负娓╁害瀵勫瓨鍣ㄧ殑瀹為檯鍊煎疄闄呬笂鏄浉瀵?TjMax 鐨勫樊鍊笺€?

绉颁负 TjMax 鐨勬俯搴︽槸澶勭悊鍣ㄧ殑鏈€澶х粨娓╋紝鍙栧喅浜?CPU 鍨嬪彿銆傝琛ㄤ笅鏂广€傝揪鍒拌娓╁害
鏃讹紝淇濇姢鏈哄埗浼氭墽琛屽姩浣滀互寮哄埗涓哄鐞嗗櫒闄嶆俯銆傚鏋滄俯搴︿笂鍗囧埌瓒充互瑙﹀彂 Out-Of-Spec
浣嶏紙瓒呰繃 TjMax锛夛紝鍙兘浼氬彂鍑哄憡璀︺€備笅琛ㄦ眹鎬讳簡瀵煎嚭鐨?sysfs 鏂囦欢锛?

鎵€鏈?sysfs 鏉＄洰鍧囦互鍚勮嚜鐨?core_id 鍛藉悕锛堟澶勭敤 'X' 琛ㄧず锛夈€?

================= ========================================================
tempX_input	  鏍稿績娓╁害锛堝崟浣嶄负姣憚姘忓害锛夈€?
tempX_max	  搴斿紑鍚叏閮ㄦ暎鐑澶囷紙鍦?Core2 涓婏級銆?
tempX_crit	  鏈€楂樼粨娓╋紙鍗曚綅涓烘鎽勬皬搴︼級銆?
tempX_crit_alarm  褰?Out-of-spec 浣嶇疆浣嶆椂缃捣锛屼笖涓嶄細鑷姩娓呴櫎銆?
		  姝ゆ椂鏃犳硶鍐嶄繚璇?CPU 鐨勬纭繍琛屻€?
tempX_label	  鍖呭惈瀛楃涓?"Core X"锛屽叾涓?X 涓哄鐞嗗櫒缂栧彿銆?
		  瀵逛簬灏佽娓╁害锛岃鍊间负 "Physical id Y"锛?
		  鍏朵腑 Y 涓哄皝瑁呯紪鍙枫€?
================= ========================================================

鍦ㄧ幇浠?CPU锛圢ehalem 鍙婃洿鏂帮級涓婏紝TjMax 浠?MSR_IA32_TEMPERATURE_TARGET 瀵勫瓨鍣?
璇诲彇銆傚湪娌℃湁璇?MSR 鐨勬棫鍨嬪彿涓婏紝TjMax 閫氳繃鏌ユ壘琛ㄦ垨鍚彂寮忔柟娉曠‘瀹氥€傚鏋滆繖浜涘
鎮ㄧ殑 CPU 涓嶈捣浣滅敤锛屽彲浠ュ皢姝ｇ‘鐨?TjMax 鍊间綔涓烘ā鍧楀弬鏁帮紙tjmax锛変紶鍏ャ€?

闄勫綍 A. 宸茬煡 TjMax 鍒楄〃锛堝緟瀹氾級锛?
閮ㄥ垎淇℃伅鏉ヨ嚜 ark.intel.com

=============== =============================================== ================
鍒剁▼锛圥rocess锛?澶勭悊鍣紙Processor锛?				TjMax(掳C)

22nm		Core i5/i7 Processors
		i7 3920XM, 3820QM, 3720QM, 3667U, 3520M		105
		i5 3427U, 3360M/3320M				105
		i7 3770/3770K					105
		i5 3570/3570K, 3550, 3470/3450			105
		i7 3770S					103
		i5 3570S/3550S, 3475S/3470S/3450S		103
		i7 3770T					94
		i5 3570T					94
		i5 3470T					91

32nm		Core i3/i5/i7 Processors
		i7 2600						98
		i7 660UM/640/620, 640LM/620, 620M, 610E		105
		i5 540UM/520/430, 540M/520/450/430		105
		i3 330E, 370M/350/330				90 rPGA, 105 BGA
		i3 330UM					105

32nm		Core i7 Extreme Processors
		980X						100

32nm		Celeron Processors
		U3400						105
		P4505/P4500 					90

32nm		Atom Processors
		S1260/1220					95
		S1240						102
		Z2460						90
		Z2760						90
		D2700/2550/2500					100
		N2850/2800/2650/2600				100

22nm		Atom Processors (Silvermont/Bay Trail)
		E3845/3827/3826/3825/3815/3805			110
		Z3795/3775/3770/3740/3736/3735/3680		90

22nm		Atom Processors (Silvermont/Moorefield)
		Z3580/3570/3560/3530				90

14nm		Atom Processors (Airmont/Cherry Trail)
		x5-Z8550/Z8500/Z8350/Z8330/Z8300		90
		x7-Z8750/Z8700					90

14nm		Atom Processors (Goldmont/Apollo Lake)
		x5-E3940/E3930					105
		x7-E3950					105

14nm		Celeron/Pentium Processors
		(Goldmont/Apollo Lake)
		J3455/J3355					105
		N3450/N3350					105
		N4200						105

14nm		Celeron/Pentium Processors
		(Goldmont Plus/Gemini Lake)
		J4105/J4005					105
		N4100/N4000					105
		N5000						105

10nm		Atom Processors (Tremont/Elkhart Lake)
		x6000E						105

10nm		Celeron/Pentium Processors
		(Tremont/Jasper Lake)
		N4500/N5100/N6000 series			105

45nm		Xeon Processors 5400 Quad-Core
		X5492, X5482, X5472, X5470, X5460, X5450	85
		E5472, E5462, E5450/40/30/20/10/05		85
		L5408						95
		L5430, L5420, L5410				70

45nm		Xeon Processors 5200 Dual-Core
		X5282, X5272, X5270, X5260			90
		E5240						90
		E5205, E5220					70, 90
		L5240						70
		L5238, L5215					95

45nm		Atom Processors
		D525/510/425/410				100
		K525/510/425/410				100
		Z670/650					90
		Z560/550/540/530P/530/520PT/520/515/510PT/510P	90
		Z510/500					90
		N570/550					100
		N475/470/455/450				100
		N280/270					90
		330/230						125
		E680/660/640/620				90
		E680T/660T/640T/620T				110
		E665C/645C					90
		E665CT/645CT					110
		CE4170/4150/4110				110
		CE4200 series					unknown
		CE5300 series					unknown

45nm		Core2 Processors
		Solo ULV SU3500/3300				100
		T9900/9800/9600/9550/9500/9400/9300/8300/8100	105
		T6670/6500/6400					105
		T6600						90
		SU9600/9400/9300				105
		SP9600/9400					105
		SL9600/9400/9380/9300				105
		P9700/9600/9500/8800/8700/8600/8400/7570	105
		P7550/7450					90

45nm		Core2 Quad Processors
		Q9100/9000					100

45nm		Core2 Extreme Processors
		X9100/9000					105
		QX9300						100

45nm		Core i3/i5/i7 Processors
		i7 940XM/920					100
		i7 840QM/820/740/720				100

45nm		Celeron Processors
		SU2300						100
		900 						105

65nm		Core2 Duo Processors
		Solo U2200, U2100				100
		U7700/7600/7500					100
		T7800/7700/7600/7500/7400/7300/7250/7200/7100	100
		T5870/5670/5600/5550/5500/5470/5450/5300/5270	100
		T5250						100
		T5800/5750/5200					85
		L7700/7500/7400/7300/7200			100

65nm		Core2 Extreme Processors
		X7900/7800					100

65nm		Core Duo Processors
		U2500/2400					100
		T2700/2600/2450/2400/2350/2300E/2300/2250/2050	100
		L2500/2400/2300					100

65nm		Core Solo Processors
		U1500/1400/1300					100
		T1400/1350/1300/1250				100

65nm		Xeon Processors 5000 Quad-Core
		X5000						90-95
		E5000						80
		L5000						70
		L5318						95

65nm		Xeon Processors 5000 Dual-Core
		5080, 5063, 5060, 5050, 5030			80-90
		5160, 5150, 5148, 5140, 5130, 5120, 5110	80
		L5138						100

65nm		Celeron Processors
		T1700/1600					100
		560/550/540/530					100
=============== =============================================== ================
