
## 甯歌闂锛團AQ锛?

     1. 鍦ㄦ暟瀛楃數瑙嗕腑锛屼竴涓墿鐞嗛閬撲腑鍙兘鍖呭惈涓嶅悓鐨勫唴瀹广€傝鑼冨皢姣忎釜鍐呭绉颁负涓€涓?*涓氬姟锛坰ervice锛?*銆傝繖姝ｆ槸鐢佃鐢ㄦ埛鎵€璇寸殑"棰戦亾"銆傚洜姝わ紝涓轰簡閬垮厤娣锋穯锛屽湪鏈?FAQ 涓垜浠皢**杞彂鍣紙transponder锛?*绉颁负鐗╃悊棰戦亾锛屽皢**涓氬姟锛坰ervice锛?*绉颁负閫昏緫棰戦亾銆?
     2. LinuxTV 绀惧尯缁存姢鐫€涓€浜?Wiki 椤甸潰锛屽叾涓寘鍚ぇ閲忎笌濯掍綋瀛愮郴缁熺浉鍏崇殑淇℃伅銆傚鏋滀綘鍦ㄨ繖閲屾壘涓嶅埌鎵€闇€绛旀锛屽緢鍙兘鍦ㄩ偅閲岃兘鎵惧埌鏈夌敤鐨勫唴瀹广€傚畠鎵樼鍦細

	https://www.linuxtv.org/wiki/

涓€浜涘叧浜?Linux 鏁板瓧鐢佃鏀寔鐨勯潪甯稿父瑙佺殑闂

1. 淇″彿浼间箮鍦ㄨ皟璋愬悗鍑犵灏辨秷澶变簡銆?
	杩欎笉鏄?bug锛岃€屾槸鐗规€с€傚洜涓哄墠绔紙frontend锛夋湁鐩稿綋澶х殑鍔熻€楅渶姹傦紙鍥犳浼氬彉寰楅潪甯哥儹锛夛紝濡傛灉瀹冧滑鏈浣跨敤锛堝嵆鍓嶇璁惧琚叧闂級锛屽氨浼氳鏂數銆俙dvb-core` 妯″潡鍙傛暟 `dvb_shutdown_timeout` 鍏佽浣犳洿鏀硅秴鏃舵椂闂达紙榛樿 5 绉掞級銆傚皢瓒呮椂璁句负 0 浼氱鐢ㄨ秴鏃剁壒鎬с€?
2. 鎴戝浣曠湅鐢佃锛?
	鏁板瓧鐢佃寮€鍙戣€呬笌 Linux 鍐呮牳涓€璧风淮鎶や簡涓€浜涚畝鍗曠殑宸ュ叿锛屼富瑕佺敤浜庢祴璇曞苟婕旂ず DVB API 鐨勫伐浣滄柟寮忋€傝繖绉颁负 DVB v5 宸ュ叿锛屼笌 `v4l-utils` git 浠撳簱鏀惧湪涓€璧凤細

	    https://git.linuxtv.org/v4l-utils.git/

	浣犲彲浠ュ湪 LinuxTV wiki 鎵惧埌鏇村淇℃伅锛?
	    https://www.linuxtv.org/wiki/index.php/DVBv5_Tools

	绗竴姝ユ槸鑾峰彇鎵€浼犺緭鐨勪笟鍔″垪琛ㄣ€?
	杩欏彲浠ラ€氳繃浣跨敤鑻ュ共鐜版湁宸ュ叿瀹屾垚銆備緥濡傦紝浣犲彲浠ヤ娇鐢?`dvbv5-scan` 宸ュ叿銆備綘鍙互鍦ㄤ互涓嬩綅缃壘鍒板叧浜庡畠鐨勬洿澶氫俊鎭細

	    https://www.linuxtv.org/wiki/index.php/Dvbv5-scan

	杩樻湁鍏朵粬涓€浜涘簲鐢紝濡?`w_scan` [#]_锛屼細杩涜鐩叉壂锛屽姫鍔涘鎵炬墍鏈夊彲鑳界殑棰戦亾锛屼絾閭ｄ簺浼氭秷鑰楀ぇ閲忚繍琛屾椂闂淬€?
	.. [#] https://www.linuxtv.org/wiki/index.php/W_scan

	姝ゅ锛屼竴浜涘簲鐢紙濡?`kaffeine`锛夋湁鑷繁鐨勪唬鐮佹潵鎵弿涓氬姟銆傛墍浠ヤ綘涓嶉渶瑕佷娇鐢ㄥ閮ㄥ簲鐢ㄦ潵鑾峰彇杩欐牱鐨勫垪琛ㄣ€?
	澶у鏁版绫诲伐鍏烽渶瑕佷竴涓寘鍚綘鎵€鍦ㄥ尯鍩熷彲鐢ㄩ閬撹浆鍙戝櫒鍒楄〃鐨勬枃浠躲€傚洜姝わ紝LinuxTV 寮€鍙戣€呯淮鎶ょ潃鏁板瓧鐢佃棰戦亾杞彂鍣ㄨ〃锛屽苟浠庣ぞ鍖烘帴鏀惰ˉ涓佷互淇濇寔鏇存柊銆?
	璇ュ垪琛ㄦ墭绠″湪锛?
	    https://git.linuxtv.org/dtv-scan-tables.git

	骞朵笖琚墦鍖呰繘澶氫釜鍙戣鐗堛€?
	Kaffeine 瀵逛竴浜涘湴闈㈡爣鍑嗘湁涓€瀹氱殑鐩叉壂鏀寔銆傚畠涔熶緷璧?DTV 鎵弿琛紝灏界瀹冨唴閮ㄥ寘鍚簡涓€浠藉壇鏈紙骞朵笖濡傛灉鐢ㄦ埛瑕佹眰锛屽畠浼氫笅杞芥洿鏂扮殑鐗堟湰锛夈€?
	濡傛灉骞歌繍鐨勮瘽锛屼綘鍙互鐩存帴浣跨敤鎻愪緵鐨勬煇涓閬撹浆鍙戝櫒銆傚惁鍒欙紝浣犲彲鑳介渶瑕佸湪浜掕仈缃戜笂鏌ユ壘姝ょ被淇℃伅骞跺垱寤轰竴涓柊鏂囦欢銆傛湁澶氫釜绔欑偣鍖呭惈鐗╃悊棰戦亾鍒楄〃銆傚浜庢湁绾垮拰鍗槦锛岄€氬父鐭ラ亾濡備綍璋冭皭鍒板崟涓閬撳氨瓒充互璁╂壂鎻忓伐鍏疯瘑鍒嚭鍏朵粬棰戦亾銆傚湪鏌愪簺鍦版柟锛岃繖瀵瑰湴闈紶杈撲篃鍙兘鏈夋晥銆?
	涓€鏃︿綘鏈変簡杞彂鍣ㄥ垪琛紝浣犻渶瑕佺敤鍍?`dvbv5-scan` 杩欐牱鐨勫伐鍏风敓鎴愪笟鍔″垪琛ㄣ€?
	鍑犱箮鎵€鏈夌幇浠ｆ暟瀛楃數瑙嗗崱閮芥病鏈夊唴缃殑纭欢 MPEG 瑙ｇ爜鍣ㄣ€傚洜姝わ紝鐢卞簲鐢ㄧ▼搴忚礋璐ｈ幏鍙栨澘鍗℃彁渚涚殑 MPEG-TS 娴侊紝灏嗗叾鎷嗗垎涓洪煶棰戙€佽棰戝拰鍏朵粬鏁版嵁骞惰繘琛岃В鐮併€?
3. 鏈夊摢浜涙暟瀛楃數瑙嗗簲鐢ㄧ▼搴忥紵

	澶氫釜濯掍綋鎾斁鍣ㄥ簲鐢ㄨ兘澶熻皟璋愬埌鏁板瓧鐢佃棰戦亾锛屽寘鎷?Kaffeine銆乂lc銆乵player 鍜?MythTV銆?
	Kaffeine 鏃ㄥ湪闈炲父鐢ㄦ埛鍙嬪ソ锛屽畠鐢变竴浣嶅唴鏍搁┍鍔ㄥ紑鍙戣€呯淮鎶ゃ€?
	杩欎簺鍙婂叾浠栧簲鐢ㄧ殑缁煎悎鍒楄〃鍙湪浠ヤ笅浣嶇疆鎵惧埌锛?
	    https://www.linuxtv.org/wiki/index.php/TV_Related_Software

	涓嬮潰閾炬帴浜嗕竴浜涙渶娴佽鐨勶細

	https://kde.org/applications/multimedia/org.kde.kaffeine
		KDE 濯掍綋鎾斁鍣紝涓撴敞浜庢暟瀛楃數瑙嗘敮鎸?
	https://www.linuxtv.org/vdrwiki/index.php/Main_Page
		Klaus Schmidinger 鐨勮棰戠鐩樺綍鍍忔満锛圴ideo Disk Recorder锛?
	https://linuxtv.org/downloads and https://git.linuxtv.org/
		鏁板瓧鐢佃鍙婂叾浠栧獟浣撶浉鍏冲簲鐢ㄥ拰鍐呮牳椹卞姩銆傚叾涓殑 `v4l-utils` 杞欢鍖呭寘鍚嫢骞茬敤浜庢暟瀛楃數瑙嗙殑鐟炲＋鍐涘垁寮忓伐鍏枫€?
	http://sourceforge.net/projects/dvbtools/
		Dave Chapman 鐨?dvbtools 杞欢鍖咃紝鍖呮嫭 dvbstream 鍜?dvbtune

	http://www.dbox2.info/
		dBox2 涓婄殑 LinuxDVB

	http://www.tuxbox.org/
		TuxBox CVS锛屽寘鍚澶氭湁瓒ｇ殑 DVB 搴旂敤鍜?dBox2 鐨?DVB 婧愮爜

	http://www.nenie.org/misc/mpsys/
		MPSYS锛氫竴涓?MPEG2 绯荤粺搴撳拰宸ュ叿

	https://www.videolan.org/vlc/index.pt.html
		Vlc

	http://mplayerhq.hu/
		MPlayer

	http://xine.sourceforge.net/ and http://xinehq.de/
		Xine

	http://www.mythtv.org/
		MythTV - 妯℃嫙鐢佃鍜屾暟瀛楃數瑙?PVR

	http://dvbsnoop.sourceforge.net/
		DVB 鍡呮帰绋嬪簭锛岀敤浜庣洃瑙嗐€佸垎鏋愩€佽皟璇曘€佽浆鍌ㄦ垨鏌ョ湅 dvb/mpeg/dsm-cc/mhp 娴佷俊鎭紙TS銆丳ES銆丼ECTION锛?
4. 鏃犳硶姝ｇ‘璋冭皭鍒颁俊鍙?
	杩欏彲鑳芥槸鐢变簬寰堝闂銆傛牴鎹垜鐨勪釜浜虹粡楠岋紝閫氬父鐢佃鍗℃瘮鐢佃鏈洪渶瑕佹洿寮虹殑淇″彿锛屽苟涓斿鍣０鏇存晱鎰熴€傛墍浠ワ紝涔熻浣犲彧闇€瑕佹洿濂界殑澶╃嚎鎴栫嚎缂嗐€備笉杩囷紝涔熷彲鑳芥槸鏌愪簺纭欢鎴栭┍鍔ㄩ棶棰樸€?
	渚嬪锛屽鏋滀綘浣跨敤鐨勬槸涓嶅甫妯℃嫙妯″潡鐨?Technotrend/Hauppauge DVB-C 鍗★紝浣犲彲鑳介渶瑕佷娇鐢ㄦā鍧楀弬鏁?adac=-1锛坉vb-ttpci.o锛夈€?
	璇峰弬闃?linuxtv.org 涓婄殑 FAQ 椤甸潰锛屽洜涓哄畠鍙兘鍖呭惈涓€浜涙湁浠峰€肩殑淇℃伅锛?
	    https://www.linuxtv.org/wiki/index.php/FAQ_%26_Troubleshooting

	濡傛灉閭ｆ病鏈夌敤锛岃鏌ョ湅 linux-media 閭欢鍒楄〃褰掓。锛岀湅鐪嬫槸鍚︽湁浜洪亣鍒拌繃涓庝綘鐨勭‖浠跺拰/鎴栨暟瀛楃數瑙嗘湇鍔℃彁渚涘晢绫讳技鐨勯棶棰橈細

	    https://lore.kernel.org/linux-media/

	濡傛灉杩欎簺閮芥病鐢紝浣犲彲浠ュ皾璇曞悜 linux-media 閭欢鍒楄〃鍙戦€佺數瀛愰偖浠讹紝鐪嬬湅鏄惁鏈変汉鑳芥彁渚涗竴浜涚嚎绱€傜數瀛愰偖浠跺湴鍧€鏄?linux-media AT vger.kernel.org銆?
5. dvb_net 璁惧瀹屽叏娌℃湁缁欐垜浠讳綍鏁版嵁鍖?
	鍦?`dvb0_0` 鎺ュ彛涓婅繍琛?`tcpdump`銆傝繖浼氬皢鎺ュ彛璁句负娣锋潅妯″紡锛屼粠鑰屾帴鍙椾綘鐢?`dvbnet` 瀹炵敤绋嬪簭閰嶇疆鐨?PID 浼犳潵鐨勪换浣曟暟鎹寘銆傛鏌ユ槸鍚︽湁浣犵敤 `ifconfig` 鎴?`ip addr` 閰嶇疆鐨?IP 鍦板潃鍜?MAC 鍦板潃鐨勬暟鎹寘銆?
	濡傛灉 `tcpdump` 娌℃湁浠讳綍杈撳嚭锛岃妫€鏌?`ifconfig` 鎴?`netstat -ni` 杈撳嚭鐨勭粺璁′俊鎭€傦紙娉ㄦ剰锛氬鏋?MAC 鍦板潃閿欒锛宍dvb_net` 灏嗕笉浼氭敹鍒颁换浣曡緭鍏ワ紱鍥犳浣犲繀椤诲湪妫€鏌ョ粺璁′俊鎭箣鍓嶅厛杩愯 `tcpdump`銆傦級濡傛灉瀹屽叏娌℃湁鏁版嵁鍖咃紝閭ｄ箞鍙兘鏄?PID 閿欒銆傚鏋滄湁閿欒鏁版嵁鍖咃紝閭ｄ箞瑕佷箞鏄?PID 閿欒锛岃涔堟槸娴佷笉绗﹀悎 MPE 鏍囧噯锛圗N 301 192锛宧ttp://www.etsi.org/锛夈€備緥濡傦紝浣犲彲浠ヤ娇鐢?`dvbsnoop` 杩涜璋冭瘯銆?
6. `dvb_net` 璁惧娌℃湁缁欐垜浠讳綍澶氭挱鏁版嵁鍖?
	妫€鏌ヤ綘鐨勮矾鐢辨槸鍚﹀寘鍚鎾湴鍧€鑼冨洿銆傛澶栵紝纭繚"鍩轰簬鍙嶅悜璺緞鐨勬簮楠岃瘉锛坰ource validation by reversed path锛?
```
	  $ "echo 0 > /proc/sys/net/ipv4/conf/dvb0/rp_filter"

```
7. 閭ｄ簺闇€瑕佸姞杞界殑妯″潡閮芥槸浠€涔堬紵

	涓轰簡浣夸箣鏇寸伒娲诲苟鏀寔涓嶅悓鐨勭‖浠剁粍鍚堬紝濯掍綋瀛愮郴缁熶互妯″潡鍖栫殑鏂瑰紡缂栧啓銆?
	鍥犳锛岄櫎浜嗕富鑺墖缁勭殑鏁板瓧鐢佃纭欢妯″潡澶栵紝瀹冭繕闇€瑕佸姞杞戒竴涓墠绔┍鍔紝浠ュ強鏁板瓧鐢佃鏍稿績銆傚鏋滄澘鍗¤繕甯︽湁閬ユ帶鍣紝瀹冭繕闇€瑕侀仴鎺у櫒鏍稿績鍜岄仴鎺у櫒琛ㄣ€傚鏋滄澘鍗℃敮鎸佹ā鎷熺數瑙嗭紝鎯呭喌涔熶竴鏍凤細闇€瑕佸姞杞?video4linux 鐨勬牳蹇冩敮鎸併€?
	瀹為檯鐨勬ā鍧楀悕绉版槸鐗瑰畾浜?Linux 鍐呮牳鐗堟湰鐨勶紝鍥犱负涓轰簡澧炲己濯掍綋鏀寔鐨勭伒娲绘€э紝鎯呭喌浼氫笉鏃跺彂鐢熷彉鍖栥€?