
## ARCnet


:Author: Avery Pennarun <apenwarr@worldvisions.ca>


   濡傛灉浣犲拰鎴戜滑涓澶氫汉涓€鏍凤紝纰板阀娌℃湁鎷垮埌 ARCnet 鍗＄殑鎵嬪唽锛岃鍙傝鏈洰褰曚笅鐨?   arcnet-hardware.txt锛屼互鑾峰彇璺崇嚎璁剧疆鍜岀嚎缂嗚繛鎺ヤ俊鎭€?
鏃㈢劧浼间箮娌′汉浼氬惉鎴戠殑锛屼篃璁镐竴棣栬瘲鑳借浣犲惉杩涘幓
```

		This driver's getting fat and beefy,
		But my cat is still named Fifi.

```

鍡紝鎴戣寰楁垜鍙互鎶婇偅绉颁綔涓€棣栬瘲锛屽敖绠″畠鍙湁涓よ銆傚樋锛屾垜鏄璁＄畻鏈虹瀛︾殑锛屼笉鏄?瀛﹁嫳璇殑銆傞ザ浜嗘垜鍚с€?
閲嶇偣鏄細濡傛灉浣犳祴璇曚簡杩欎釜椹卞姩骞惰瀹冨伐浣滐紙鎴栬€呮病宸ヤ綔锛夛紝鎴栬€呭叾浠栦换浣曟儏鍐碉紝
鎴戠湡鐨勭湡鐨勭湡鐨勭湡鐨勭湡鐨勫緢鎯冲惉鍒颁綘鐨勬秷鎭€?
ARCnet 0.32 ALPHA 棣栨杩涘叆 Linux 鍐呮牳 1.1.80 鈥斺€?杩欏緢濂斤紝浣嗗湪閭ｄ箣鍚庯紝鐢氳嚦鏇?灏戠殑浜哄紑濮嬬粰鎴戝啓淇★紝鍥犱负浠栦滑鐢氳嚦涓嶉渶瑕佸畨瑁呰繖涓ˉ涓佷簡銆?鍙规皵>

鏉ュ惂锛屽仛涓鎰忔€濈殑浜猴紒缁欐垜鍙戜竴浠芥垚鍔熸姤鍛婏紒

锛堝樋锛岃繖姣旀垜鍘熸潵鐨勮瘲杩樿濂解€︹€﹁繖瓒婃潵瓒婄碂浜嗭紒锛?
----

浠ヤ笅鏄?Linux 鐨?ARCnet 椹卞姩銆?
杩欎釜鏂扮増鏈紙2.91锛夌敱 David Woodhouse <dwmw2@infradead.org> 鏁寸悊锛岀洰鐨勬槸鍦ㄦ坊鍔?瀵瑰張涓€绉嶈姱鐗囩粍鐨勬敮鎸佷箣鍚庢暣鐞嗚繖涓┍鍔ㄣ€傜幇鍦ㄩ€氱敤鏀寔宸茬粡浠庡悇涓姱鐗囩粍椹卞姩涓垎绂?鍑烘潵锛屾簮鏂囦欢涔熶笉鍐嶅婊?#ifdefs 浜嗭紒鎴戝杩欎釜鏂囦欢鍋氫簡涓€鐐逛慨鏀癸紝浣嗕繚鐣欎簡 Avery 鐨?绗竴浜虹О鍙ｅ惢锛屽洜涓烘垜涓嶆兂瀹屽叏閲嶅啓瀹冦€?
涓婁竴涓増鏈潵鑷垜锛圓very Pennarun锛夋柇鏂画缁暟鏈堢殑鍔姏銆佹潵鑷叾浠栦汉鐨勮澶?bug 鎶ュ憡
/淇鍜屽缓璁紝鐗瑰埆鏄潵鑷?Tomasz Motylewski 鐨勫ぇ閲忚緭鍏ュ拰浠ｇ爜銆備粠 ARCnet 2.10 ALPHA
寮€濮嬶紝Tomasz 鍏ㄦ柊鏀硅繘鐨?RFC1051 鏀寔琚撼鍏ワ紝骞朵笖浼间箮宸ヤ綔姝ｅ父锛?


### 鎴戝湪鍝噷璁ㄨ杩欎簺椹卞姩锛?

ARCnet 鐨勮璁哄湪 netdev 涓婅繘琛屻€傚彧闇€灏嗕綘鐨勯偖浠跺彂閫佸埌 netdev@vger.kernel.org锛?骞剁‘淇濇妱閫侊紙Cc锛塂ocumentation/process/maintainers.rst 涓€淎RCNET NETWORK LAYER鈥?鏍囬涓嬪垪鍑虹殑缁存姢鑰呫€?
### 鍏朵粬椹卞姩涓庝俊鎭?

浣犲彲浠ヨ闂垜鍦ㄤ竾缁寸綉锛圵orld Wide Web锛変笂鐨?ARCNET 椤甸潰锛?
	http://www.qis.net/~jschmitz/arcnet/

鍙﹀锛孲MC锛堢敓浜?ARCnet 鍗＄殑鍏徃涔嬩竴锛夋湁涓€涓綘鍙兘浼氭劅鍏磋叮鐨?WWW 绔欑偣锛屽叾涓寘鍚?澶氫釜鏀寔鍖呮嫭 ARCnet 鍦ㄥ唴鐨勫悇绉嶇綉鍗＄殑椹卞姩銆傝瘯璇曪細

	http://www.smc.com/

Performance Technologies 鍑哄搧鍚勭鏀寔 ARCnet 鐨勭綉缁滆蒋浠讹細

	http://www.perftech.com/ 鎴栭€氳繃 FTP 璁块棶 ftp.perftech.com銆?
Novell 鍑哄搧涓€涓寘鍚?ARCnet 椹卞姩鐨?DOS 缃戠粶鍗忚鏍堛€傝瘯璇?FTP 鍒?ftp.novell.com銆?
浣犲彲浠ヤ粠 oak.oakland.edu:/simtel/msdos/pktdrvr 鑾峰彇 Crynwr 鍖呴┍鍔ㄩ泦鍚堬紙鍖呮嫭
arcether.com锛屼篃灏辨槸浣犳兂閰嶅悎 ARCnet 鍗′娇鐢ㄧ殑閭ｄ釜锛夈€備笉杩囧畠鍦ㄦ湭缁忔墦琛ヤ竵鐨勬儏鍐典笅
鍦?386+ 涓婃棤娉曞畬缇庡伐浣滐紝骞朵笖涔熶笉鍠滄鏌愪簺缃戝崱銆備慨姝ｇ増鏈彲浠ュ湪鎴戠殑 WWW 椤甸潰涓婅幏鍙栵紝
鎴栬€呭鏋滀綘娌℃湁 WWW 璁块棶鏉冮檺锛屼篃鍙互閫氳繃鐢靛瓙閭欢鑾峰彇銆?

### 瀹夎椹卞姩


```

	make config
		(be sure to choose ARCnet in the network devices
		and at least one chipset driver.)
	make clean
	make zImage

```

濡傛灉浣犺幏寰楄繖涓?ARCnet 杞欢鍖咃紝鏄綔涓哄浣犲綋鍓嶅唴鏍镐腑 ARCnet 椹卞姩鐨勪竴涓崌绾э紝浣?闇€瑕佸厛鎶?arcnet.c 澶嶅埗鍒?linux/drivers/net 鐩綍涓婅鐩栧師鏈夌殑鏂囦欢銆?
濡傛灉浣犲湪閲嶆柊鍚姩杩涘叆鏂扮殑 Linux 鍐呮牳鏃剁湅鍒颁竴浜?ARCnet 娑堟伅锛屼綘灏辩煡閬撻┍鍔ㄥ凡缁?姝ｇ‘瀹夎浜嗐€?
鏈夊洓绉嶈姱鐗囩粍閫夐」锛?
 1. 鏍囧噯 ARCnet COM90xx 鑺墖缁勩€?
杩欐槸鏅€氱殑 ARCnet 鍗★紝浣犲緢鍙兘鐢ㄧ殑灏辨槸瀹冦€傝繖鏄敮涓€涓€涓湪娌¤鍛婄煡鍗＄殑浣嶇疆鏃朵細
鑷姩鎺㈡祴鐨勮姱鐗囩粍椹卞姩銆?```

 com90xx=[<io>[,<irq>[,<shmem>]]][,<name>] | <name>

```
```

 io=<io> irq=<irq> shmem=<shmem> device=<name>

```

瑕佺鐢ㄨ嚜鍔ㄦ帰娴嬶紝鍙渶鍦ㄥ唴鏍稿懡浠よ涓婃寚瀹?"com90xx="銆傝鍙寚瀹氬悕绉帮紝浣嗗厑璁歌嚜鍔?鎺㈡祴锛屽彧闇€鍐?"com90xx=<name>"

 2. ARCnet COM20020 鑺墖缁勩€?
杩欐槸 SMC 鍑哄搧鐨勬柊鍨嬭姱鐗囩粍锛屾敮鎸佹贩鏉傛ā寮忥紙鏁版嵁鍖呭梾鎺級銆侀澶栫殑璇婃柇淇℃伅绛夈€?涓嶅垢鐨勬槸锛屾病鏈夊悎鐞嗙殑鏂规硶鍙互鑷姩鎺㈡祴杩欎簺鍗°€備綘蹇呴』鍦ㄥ唴鏍稿懡浠よ涓婃寚瀹?I/O 鍦板潃銆?
```

 com20020=<io>[,<irq>[,<node_ID>[,backplane[,CKP[,timeout]]]]][,name]

```
```

 io=<io> irq=<irq> node=<node_ID> backplane=<backplane> clock=<CKP>
 timeout=<timeout> device=<name>

```

COM20020 鑺墖缁勫厑璁镐綘閫氳繃杞欢璁剧疆鑺傜偣 ID锛岃鐩栭粯璁わ紙浠嶇劧鐢卞崱涓婄殑 DIP 寮€鍏宠缃級
鐨勫€笺€傚鏋滀綘娌℃湁 COM20020 鐨勬暟鎹墜鍐岋紝骞朵笖浣犱笉鐭ラ亾鍏朵粬涓変釜閫夐」鎸囩殑鏄粈涔堬紝閭?瀹冧滑涓嶄細璁╀綘鎰熷叴瓒?鈥斺€?蹇樹簡瀹冧滑鍚с€?
 3. IO 鏄犲皠妯″紡涓嬬殑 ARCnet COM90xx 鑺墖缁勩€?
杩欎篃鑳界敤浜庢櫘閫氱殑 ARCnet 鍗★紝浣嗕笉浣跨敤鍏变韩鍐呭瓨銆傚畠鐨勬€ц兘涓嶅涓婇潰鐨勯┍鍔紝浣嗘彁渚涘畠
鏄€冭檻鍒颁綘鏈変竴寮犱笉鏀寔鍏变韩鍐呭瓨鐨勫崱锛屾垨鑰咃紙濂囨€湴锛夎€冭檻鍒颁綘鏈哄櫒閲岀殑 ARCnet 鍗?澶鑰屽鑷村叡浜唴瀛樻彃妲界敤瀹屼簡銆傚鏋滀綘涓嶅湪鍐呮牳鍛戒护琛屼笂缁欏嚭 I/O 鍦板潃锛岄偅涔堥┍鍔ㄥ皢
鎵句笉鍒拌繖寮犲崱銆?
```

 com90io=<io>[,<irq>][,<name>]

```

濡傛灉浣犳妸鑺墖缁勬敮鎸佷綔涓烘ā鍧楀姞杞斤紝閫夐」鏄細
 io=<io> irq=<irq> device=<name>

 4. ARCnet RIM I 鍗°€?
杩欎簺鏄?瀹屽叏*鍐呭瓨鏄犲皠鐨?COM90xx 鑺墖銆傚杩欎簺鍗＄殑鏀寔鏈粡娴嬭瘯銆傚鏋滀綘鏈夎繖绉嶅崱锛?璇风粰浣滆€呭彂閭欢骞堕檮涓婃垚鍔熸姤鍛娿€傞櫎璁惧鍚嶅锛屾墍鏈夐€夐」閮藉繀椤绘寚瀹氥€?```

 arcrimi=<shmem>,<irq>,<node_ID>[,<name>]

```
```

 shmem=<shmem> irq=<irq> node=<node_ID> device=<name>


```

### 鍙姞杞芥ā鍧楁敮鎸?

閰嶇疆骞堕噸鏂扮紪璇?Linux銆傚綋琚棶鍒版椂锛岃嫢浣犳兂浣跨敤鍙姞杞芥ā鍧楋紝瀵光€淕eneric ARCnet
support鈥濅互鍙婂浣犵殑 ARCnet 鑺墖缁勭殑鏀寔鍥炵瓟 'm'銆備綘涔熷彲浠ュ鈥淕eneric ARCnet
support鈥濆洖绛?'y'锛岃€屽鑺墖缁勬敮鎸佸洖绛?'m'锛岄殢浣犳効鎰忋€?
```

	make config
	make clean
	make zImage
	make modules

```

濡傛灉浣犱娇鐢ㄥ彲鍔犺浇妯″潡锛屼綘闇€瑕佺敤 insmod 鏉ュ姞杞藉畠锛屽苟涓斿彲浠ュ湪鍛戒护琛屼笂鎸囧畾浣犲崱鐨?鍚勭鐗规€с€傦紙鍦ㄩ┍鍔ㄧ殑杈冩柊鐗堟湰涓紝鑷姩鎺㈡祴鍙潬寰楀锛屽苟涓斾綔涓烘ā鍧椾篃鑳藉伐浣滐紝鎵€浠?杩欎簺鐜板湪澶у涓嶅繀瑕佷簡銆傦級

```

	cd /usr/src/linux/modules
	insmod arcnet.o
	insmod com90xx.o
	insmod com20020.o io=0x2e0 device=eth1


```

### 浣跨敤椹卞姩


濡傛灉浣犵紪璇戝唴鏍告椂鍖呭惈浜?ARCnet COM90xx 鏀寔锛屽畠搴旇鍦ㄤ綘鍚姩鏃惰嚜鍔ㄦ帰娴嬩綘鐨勫崱銆?濡傛灉浣犱娇鐢ㄧ紪璇戣繘鍐呮牳鐨勫叾浠栬姱鐗囩粍椹卞姩锛屼綘蹇呴』濡備笂鎵€杩板湪鍐呮牳鍛戒护琛屼笂缁欏嚭蹇呰鐨?閫夐」銆?
鍘昏 Linux 鐨?NET-2-HOWTO 鍜?ETHERNET-HOWTO锛涘畠浠簲璇ュ拰浣犳嬁鍒拌繖涓┍鍔ㄧ殑鍚屼竴澶?鍙互鑾峰彇鍒般€傛妸浣犵殑 ARCnet 褰撲綔涓€鍧楀姞寮虹増锛堟垨寮卞寲鐗堬紝瑙嗘儏鍐佃€屽畾锛夌殑浠ュお缃戝崱銆?
椤轰究璇翠竴鍙ワ紝涓€瀹氳鍦?HOWTO 涓妸鎵€鏈夊 "eth0" 鐨勫紩鐢ㄦ敼涓?"arc0"銆傝浣?ARCnet 骞?涓嶆槸鈥滅湡姝ｇ殑鈥濅互澶綉锛岃澶囧悕鏄?涓嶅悓*鐨勩€?

### 涓€鍙拌绠楁満涓寮犲崱


Linux 鐜板湪瀵规鏈夌浉褰撳ソ鐨勬敮鎸侊紝浣嗙敱浜庢垜涓€鐩村緢蹇欙紝ARCnet 椹卞姩鍦ㄨ繖鏂归潰澶氬皯鏈変簺
钀藉悗銆傚鏋滅紪璇戣繘鍐呮牳锛孋OM90xx 鏀寔浼氾紙灏濊瘯锛夎嚜鍔ㄦ帰娴嬫墍鏈夊凡瀹夎鐨勫崱銆?
濡傛灉浣犳湁鍏朵粬鍗★紝骞朵笖鍏舵敮鎸佺紪璇戣繘浜嗗唴鏍革紝閭ｄ箞浣犲彲浠?```

	LILO: linux com20020=0x2e0 com20020=0x380 com90io=0x260

```

濡傛灉浣犳妸鑺墖缁勬敮鎸佹瀯寤轰负鍙姞杞芥ā鍧楋紝閭ｄ箞浣犻渶瑕?```

	insmod -o arc0 com90xx
	insmod -o arc1 com20020 io=0x2e0
	insmod -o arc2 com90xx

```

ARCnet 椹卞姩鐜板湪浼氳嚜鍔ㄦ暣鐞嗗畠浠殑鍚嶇О銆?

### 鎴戝浣曡瀹冧笌鈥︹€︿竴璧峰伐浣滐紵


NFS锛?	linux 鍒?linux 搴旇娌￠棶棰橈紝灏卞綋鑷繁鍦ㄤ娇鐢ㄤ互澶綉鍗°€?	oak.oakland.edu:/simtel/msdos/nfs 鏈変竴浜涗笉閿欑殑 DOS 瀹㈡埛绔€傝繕鏈?	涓€涓悕涓?SOSS 鐨勩€佸熀浜?DOS 鐨?NFS 鏈嶅姟鍣ㄣ€傚畠鐨勫浠诲姟鏂瑰紡鍜?Linux
	涓嶅お涓€鏍凤紙瀹為檯涓婏紝瀹冩牴鏈笉 multitask锛夛紝浣嗕綘姘歌繙涓嶇煡閬撲綘浼氶渶瑕佷粈涔堛€?
	瀵逛簬 AmiTCP锛堝彲鑳借繕鏈夊叾浠栵級锛屼綘鍙兘闇€瑕佸湪浣犵殑 Amiga nfstab 涓?	璁剧疆浠ヤ笅閫夐」锛歁D 1024 MR 1024 MW 1024
	锛堟劅璋?Christian Gottschling <ferksy@indigo.tng.oche.de>
	鎻愪緵姝や俊鎭€傦級

	澶ф杩欎簺鎸囩殑鏄渶澶?NFS 鏁版嵁/璇?鍐欏潡澶у皬銆傛垜涓嶇煡閬撲负浠€涔?Amiga 涓婄殑
	榛樿鍊间笉琛岋紱濡傛灉浣犵煡閬撴洿澶氾紝璇峰啓淇＄粰鎴戙€?
DOS锛?	濡傛灉浣犱娇鐢ㄧ殑鏄厤璐硅蒋浠?arcether.com锛屼綘鍙兘鎯冲畨瑁呮潵鑷垜缃戦〉涓婄殑
	椹卞姩琛ヤ竵銆傚畠瀵?PC/TCP 鏈夊府鍔╋紝骞朵笖涔熻兘璁?arcether 鍦ㄥ垵濮嬪寲鏃?	瓒呮椂澶揩鐨勬儏鍐典笅鍔犺浇銆備簨瀹炰笂锛屽鏋滀綘鍦?386+ 涓婁娇鐢ㄥ畠锛屼綘纭疄
	鐪熺殑闇€瑕佽繖涓ˉ涓併€?
Windows锛?	鍙傝 DOS :) Trumpet Winsock 閰嶅悎 Novell 鎴?Arcether 瀹㈡埛绔兘鑳?	姝ｅ父宸ヤ綔锛屽綋鐒跺墠鎻愭槸浣犺寰楀姞杞?winpkt銆?
LAN Manager 鍜?Windows for Workgroups锛?	杩欎簺绋嬪簭浣跨敤鐨勫崗璁笌 Internet 鏍囧噯涓嶅吋瀹广€傚畠浠瘯鍥惧亣瑁呰繖浜涘崱鏄?	浠ュお缃戯紝骞舵妸缃戠粶涓婂叾浠栨墍鏈変汉閮芥悶绯婃秱銆?
	涓嶈繃锛寁2.00 鍙婃洿楂樼増鏈殑 Linux ARCnet 椹卞姩閫氳繃 'arc0e' 璁惧鏀寔
	杩欎釜鍗忚銆傛洿澶氫俊鎭弬瑙佲€滃鍗忚鏀寔鈥濅竴鑺傘€?
	浣跨敤鍏嶈垂鐨?Linux Samba 鏈嶅姟鍣ㄥ拰瀹㈡埛绔紝浣犵幇鍦ㄥ彲浠ヤ笌鍩轰簬 TCP/IP 鐨?	WfWg 鎴?Lan Manager 缃戠粶鐩稿綋鍙嬪ソ鍦颁簰鑱斻€?
Windows 95锛?	Win95 鑷甫宸ュ叿锛岃浣犱娇鐢?LANMAN 椋庢牸鐨勭綉缁滈┍鍔紙NDIS锛夋垨 Novell
	椹卞姩锛圤DI锛夋潵澶勭悊浣犵殑 ARCnet 鏁版嵁鍖呫€傚鏋滀綘浣跨敤 ODI锛屼綘闇€瑕佸
	Linux 浣跨敤 'arc0' 璁惧銆傚鏋滀綘浣跨敤 NDIS锛岄偅涔堣瘯璇?'arc0e' 璁惧銆?	濡傛灉浣犻渶瑕?arc0e锛屼綘瀹屽叏鐤簡锛屽拰/鎴栦綘闇€瑕佹瀯寤烘煇绉嶅悓鏃朵娇鐢ㄤ袱绉?	灏佽绫诲瀷鐨勬贩鍚堢綉缁滐紝璇峰弬瑙佷笅闈㈢殑鈥滃鍗忚鏀寔鈥濅竴鑺傘€?
OS/2锛?	鏈変汉鍛婅瘔鎴戝畠鍦?Warp Connect 涓嬮厤鍚堟潵鑷?SMC 鐨?ARCnet 椹卞姩鍙互宸ヤ綔銆?	涓烘浣犻渶瑕佷娇鐢?'arc0e' 鎺ュ彛銆傚鏋滀綘璁?SMC 椹卞姩閰嶅悎鈥滄櫘閫氣€漌arp
	Bonus Pack 涓寘鍚殑 TCP/IP 閮ㄥ垎宸ヤ綔锛岃鍛婅瘔鎴戙€?
	ftp.microsoft.com 涓婅繕鏈変竴涓厤璐圭殑鈥淟an Manager for OS/2鈥濆鎴风锛?	瀹冨簲璇ヤ娇鐢ㄥ拰 WfWg 鐩稿悓鐨勫崗璁€備笉杩囨垜鍦?Warp 涓嬪畨瑁呭畠娌℃垚鍔熴€?	濡傛湁浠讳綍缁撴灉璇峰憡璇夋垜銆?
NetBSD/AmiTCP锛?	瀹冧滑浣跨敤鏃х増鏈殑 Internet 鏍囧噯 ARCnet 鍗忚锛圧FC1051锛夛紝璇ュ崗璁笌
	Linux 椹卞姩 v2.10 ALPHA 鍙婃洿楂樼増鏈娇鐢?arc0s 璁惧鍏煎銆傦紙鍙傝涓嬮潰
	鐨勨€淢ultiprotocol ARCnet鈥濄€傦級** 杈冩柊鐗堟湰鐨?NetBSD 鏄剧劧鏀寔 RFC1201銆?

### 浣跨敤澶氬崗璁?ARCnet


ARCnet 椹卞姩 v2.10 ALPHA 鏀寔涓夌鍗忚锛屾瘡绉嶉兘鍦ㄥ叾鑷繁鐨勨€滆櫄鎷熺綉缁滆澶団€濅笂锛?
	======  ===============================================================
	arc0	RFC1201 鍗忚锛屾槸瀹樻柟鐨?Internet 鏍囧噯锛屾伆濂戒笌 Novell 鐨?TRXNET
		椹卞姩 100% 鍏煎銆侫RCnet 椹卞姩鐨?1.00 鐗堟湰*鍙?鏀寔杩欎竴鍗忚銆?		arc0 鏄笁绉嶅崗璁腑閫熷害鏈€蹇殑锛堜笉绠′粈涔堝師鍥狅級锛屽苟涓斿厑璁镐娇鐢?		鏇村ぇ鐨勬暟鎹寘锛屽洜涓哄畠鏀寔 RFC1201 鐨勨€滄暟鎹寘鎷嗗垎鈥濇搷浣溿€傞櫎闈?		浣犳湁鐗瑰畾闇€瑕佷娇鐢ㄤ笉鍚岀殑鍗忚锛屾垜寮虹儓寤鸿浣犲潥鎸佷娇鐢ㄨ繖涓€绉嶃€?
	arc0e	鈥滀互澶綉灏佽锛圗thernet-Encapsulation锛夆€濓紝閫氳繃 ARCnet 鍙戦€?		瀹為檯涓婇潪甯稿儚浠ュお缃戞暟鎹寘鐨勬暟鎹寘锛屽寘鎷?6 瀛楄妭鐨勭‖浠跺湴鍧€銆?		璇ュ崗璁笌 Microsoft 鐨?NDIS ARCnet 椹卞姩鍏煎锛屽 WfWg 鍜?LANMAN
		涓殑閭ｄ釜銆傜敱浜?493 鐨?MTU 瀹為檯涓婃瘮 TCP/IP鈥滆姹傗€濈殑锛?76锛夋洿灏忥紝
		鏌愪簺缃戠粶鎿嶄綔鏈夊彲鑳芥棤娉曟甯稿伐浣溿€備笉杩囷紝Linux 鐨?TCP/IP 灞傚湪
		澶у鏁版儏鍐典笅鍙互閫氳繃鑷姩鍒嗙墖 TCP/IP 鏁版嵁鍖呮潵浣垮畠浠€傚簲銆俛rc0e
		涔熸瘮 arc0 绋嶆參涓€浜涳紝鍘熷洜灏氭湭纭畾銆傦紙澶ф灏辨槸鏇村皬鐨?MTU 閫犳垚鐨勩€傦級

	arc0s	鈥淸s]imple鈥?RFC1051 鍗忚鏄柊鏍囧噯瀹屽叏涓嶅吋瀹圭殑鈥滄棫鈥?Internet
		鏍囧噯銆備笉杩囷紝浠婂ぉ鏈変簺杞欢缁х画鏀寔锛堜笖鍙敮鎸侊級鏃ф爣鍑嗭紝鍖呮嫭
		NetBSD 鍜?AmiTCP銆俁FC1051 涔熶笉鏀寔 RFC1201 鐨勬暟鎹寘鎷嗗垎锛岃€?		507 鐨?MTU 浠嶇劧灏忎簬 Internet鈥滆姹傗€濓紝鎵€浠ヤ綘寰堝彲鑳戒細閬囧埌闂銆?		鍑轰簬鍜?arc0e 鐩稿悓鐨勫師鍥狅紝瀹冧篃姣?RFC1201 鎱㈢害 25%銆?
		arc0s 鏀寔鐢?Tomasz Motylewski 璐＄尞锛屽苟鐢辨垜鍋氫簡涓€浜涗慨鏀广€俠ug
		澶ф鏄垜鐨勯敊銆?	======  ===============================================================

濡傛灉浣犳効鎰忥紝浣犲彲浠ラ€夋嫨涓嶆妸 arc0e 鍜?arc0s 缂栬瘧杩涢┍鍔?鈥斺€?杩欎細鐪佷笅涓€鐐瑰唴瀛橈紝骞?閬垮厤渚嬪鍦ㄤ娇鐢ㄨ繎鏈?Linux 鍐呮牳涓殑鈥淣FS-root鈥濆姛鑳芥椂鐨勬贩涔便€?
褰撲綘绗竴娆?ifconfig arc0 璁惧鏃讹紝arc0e 鍜?arc0s 璁惧浼氳嚜鍔ㄥ垱寤恒€備絾瑕佺湡姝ｄ娇鐢?瀹冧滑锛屼綘杩橀渶瑕?ifconfig 浣犻渶瑕佺殑鍏朵粬铏氭嫙璁惧銆傜劧鍚庝綘鍙互鐢ㄥ绉嶆柟寮忚缃綘鐨?缃戠粶锛?

1. 鍗曚竴鍗忚銆?
   杩欐槸閰嶇疆缃戠粶鏈€绠€鍗曠殑鏂瑰紡锛氬彧浣跨敤涓ょ鍙敤鍗忚涔嬩竴銆傚涓婃墍杩帮紝闄ら潪浣犳湁鍏呭垎
   鐞嗙敱锛堟瘮濡傛煇浜涘叾浠栬蒋浠讹紝鍗?WfWg锛屽彧涓?arc0e 涓€璧峰伐浣滐級锛屽惁鍒欏彧浣跨敤 arc0 鏄?   涓ソ涓绘剰銆?
```

	ifconfig arc0 MY.IP.ADD.RESS
	route add MY.IP.ADD.RESS arc0
	route add -net SUB.NET.ADD.RESS arc0
	[add other local routes here]

   If you need arc0e (and only arc0e), it's a little different::

	ifconfig arc0 MY.IP.ADD.RESS
	ifconfig arc0e MY.IP.ADD.RESS
	route add MY.IP.ADD.RESS arc0e
	route add -net SUB.NET.ADD.RESS arc0e

   arc0s works much the same way as arc0e.


```

2. 鍚屼竴鏍圭嚎涓婁娇鐢ㄥ涓崗璁€?
   鐜板湪浜嬫儏寮€濮嬪彉寰楁贩涔变簡銆傝灏濊瘯瀹冿紝浣犲彲鑳藉緱鏈夌偣鐤€傝繖鏄垜锛?*鎴?*锛夌殑鍋氭硶銆?   :) 娉ㄦ剰鎴戠殑瀹跺涵缃戠粶涓病鏈夊寘鍚?arc0s锛涙垜娌℃湁 NetBSD 鎴?AmiTCP 璁＄畻鏈猴紝鎵€浠ユ垜
   鍙湪鏈夐檺鐨勬祴璇曚腑浣跨敤 arc0s銆?
   鎴戠殑瀹跺涵缃戠粶涓婃湁涓夊彴璁＄畻鏈猴細涓ゅ彴 Linux 鏈哄櫒锛堢敱浜庝笂闈㈠垪鍑虹殑鍘熷洜鍋忓ソ
   RFC1201 鍗忚锛夊拰涓€鍙颁笉鑳借繍琛?Linux銆佷絾杩愯鍏嶈垂鐨?Microsoft LANMAN 瀹㈡埛绔殑
   XT銆?
   鏇寸碂鐨勬槸锛屽叾涓竴鍙?Linux 璁＄畻鏈猴紙freedom锛夎繕鏈変竴涓皟鍒惰В璋冨櫒锛屽苟鍏呭綋鍒版垜
   Internet 鎻愪緵鍟嗙殑璺敱鍣ㄣ€傚彟涓€鍙?Linux 鏈哄櫒锛坕nsight锛変篃鏈夎嚜宸辩殑 IP 鍦板潃锛?   骞堕渶瑕佷娇鐢?freedom 浣滀负鍏堕粯璁ょ綉鍏炽€傝€?XT锛坧atience锛夋病鏈夎嚜宸辩殑 Internet IP
   鍦板潃锛屾墍浠ユ垜鍦ㄤ竴涓€滅鏈夊瓙缃戔€濅笂锛堝 RFC1597 瀹氫箟鐨勶級缁欏畠鍒嗛厤浜嗕竴涓€?
   鍏堜粠涓€涓彧鏈?insight 鍜?freedom 鐨勭畝鍗曠綉缁滃紑濮嬨€俰nsight 闇€瑕侊細

 - 閫氳繃 RFC1201锛坅rc0锛夊崗璁笌 freedom 閫氫俊锛屽洜涓烘垜鏇村枩娆㈠畠锛岃€屼笖瀹冩洿蹇€? - 浣跨敤 freedom 浣滀负鍏?Internet 缃戝叧銆?
```

	ifconfig arc0 insight
	route add insight arc0
	route add freedom arc0	/* I would use the subnet here (like I said
					to in "single protocol" above),
					but the rest of the subnet
					unfortunately lies across the PPP
					link on freedom, which confuses
					things. */
	route add default gw freedom

   And freedom gets configured like so::

	ifconfig arc0 freedom
	route add freedom arc0
	route add insight arc0
	/* and default gateway is configured by pppd */

   Great, now insight talks to freedom directly on arc0, and sends packets
   to the Internet through freedom.  If you didn't know how to do the above,
   you should probably stop reading this section now because it only gets
   worse.

   Now, how do I add patience into the network?  It will be using LANMAN
   Client, which means I need the arc0e device.  It needs to be able to talk
   to both insight and freedom, and also use freedom as a gateway to the
   Internet.  (Recall that patience has a "private IP address" which won't
   work on the Internet; that's okay, I configured Linux IP masquerading on
   freedom for this subnet).

   So patience (necessarily; I don't have another IP number from my
   provider) has an IP address on a different subnet than freedom and
   insight, but needs to use freedom as an Internet gateway.  Worse, most
   DOS networking programs, including LANMAN, have braindead networking
   schemes that rely completely on the netmask and a 'default gateway' to
   determine how to route packets.  This means that to get to freedom or
   insight, patience WILL send through its default gateway, regardless of
   the fact that both freedom and insight (courtesy of the arc0e device)
   could understand a direct transmission.

   I compensate by giving freedom an extra IP address - aliased 'gatekeeper' -
   that is on my private subnet, the same subnet that patience is on.  I
   then define gatekeeper to be the default gateway for patience.

   To configure freedom (in addition to the commands above)::

	ifconfig arc0e gatekeeper
	route add gatekeeper arc0e
	route add patience arc0e

   This way, freedom will send all packets for patience through arc0e,
   giving its IP address as gatekeeper (on the private subnet).  When it
   talks to insight or the Internet, it will use its "freedom" Internet IP
   address.

   You will notice that we haven't configured the arc0e device on insight.
   This would work, but is not really necessary, and would require me to
   assign insight another special IP number from my private subnet.  Since
   both insight and patience are using freedom as their default gateway, the
   two can already talk to each other.

   It's quite fortunate that I set things up like this the first time (cough
   cough) because it's really handy when I boot insight into DOS.  There, it
   runs the Novell ODI protocol stack, which only works with RFC1201 ARCnet.
   In this mode it would be impossible for insight to communicate directly
   with patience, since the Novell stack is incompatible with Microsoft's
   Ethernet-Encap.  Without changing any settings on freedom or patience, I
   simply set freedom as the default gateway for insight (now in DOS,
   remember) and all the forwarding happens "automagically" between the two
   hosts that would normally not be able to communicate at all.

   For those who like diagrams, I have created two "virtual subnets" on the
   same physical ARCnet wire.  You can picture it like this::


	  [RFC1201 NETWORK]                   [ETHER-ENCAP NETWORK]
      (registered Internet subnet)           (RFC1597 private subnet)

			     (IP Masquerade)
	  /---------------\         *            /---------------\
	  |               |         *            |               |
	  |               +-Freedom-*-Gatekeeper-+               |
	  |               |    |    *            |               |
	  \-------+-------/    |    *            \-------+-------/
		  |            |                         |
	       Insight         |                      Patience
			   (Internet)


```

### 瀹冨伐浣滀簡锛氱幇鍦ㄥ仛浠€涔堬紵


鎸夌収 arcnet-netdev 鍙戦偖浠躲€傛弿杩颁綘鐨勮缃紝鏈€濂藉寘鎷┍鍔ㄧ増鏈€佸唴鏍哥増鏈€丄RCnet 鍗?鍨嬪彿銆丆PU 绫诲瀷銆佺綉缁滀笂绯荤粺鏁伴噺锛屼互鍙婃鍦ㄤ娇鐢ㄧ殑杞欢鍒楄〃銆?
### 瀹冧笉宸ヤ綔锛氱幇鍦ㄥ仛浠€涔堬紵


鍋氬拰涓婇潰涓€鏍风殑浜嬶紝浣嗚繕瑕佸湪閭欢涓檮涓?ifconfig 鍜?route 鍛戒护鐨勮緭鍑猴紝浠ュ強浠讳綍
鐩稿叧鐨勬棩蹇楁潯鐩紙鍗充粠涓婃閲嶅惎浠ユ潵鍑虹幇鐨勩€佷互 "arcnet:" 寮€澶寸殑浠讳綍鍐呭锛夈€?
濡傛灉浣犳兂灏濊瘯鑷繁淇瀹冿紙鎴戝己鐑堝缓璁綘鍏堝氨杩欎釜闂缁欐垜鍙戦偖浠讹紝鍥犱负瀹冨彲鑳藉凡缁忚
瑙ｅ喅浜嗭級锛屼綘涔熻鎯冲皾璇曚竴浜涘彲鐢ㄧ殑璋冭瘯绾у埆銆傚浜?D_DURING 鎴栨洿楂樼骇鍒殑閲嶅害娴嬭瘯锛?鍏堟潃鎺変綘鐨?klogd 瀹堟姢杩涚▼浼氭槸涓?闈炲父*濂界殑涓绘剰锛丏_DURING 涓烘瘡涓彂閫佹垨鎺ユ敹鐨勬暟鎹?鍖呮樉绀?4-5 琛屻€侱_TX銆丏_RX 鍜?D_SKB 瀹為檯涓婁細鏄剧ず姣忎釜鍙戦€佹垨鎺ユ敹鐨勬暟鎹寘锛岃繖鏄剧劧
鐩稿綋澶с€?
浠?v2.40 ALPHA 寮€濮嬶紝鑷姩鎺㈡祴渚嬬▼鏈変簡閲嶅ぇ鏀瑰彉銆傜壒鍒槸锛岄櫎闈炰綘鎵撳紑 D_INIT_REASONS
璋冭瘯鏍囧織锛屽惁鍒欏畠浠笉浼氬憡璇変綘涓轰粈涔堟病鎵惧埌鍗°€?
涓€鏃﹂┍鍔ㄨ繍琛岃捣鏉ワ紝浣犲彲浠ヤ綔涓?root 闅忔椂杩愯 arcdump shell 鑴氭湰锛堝彲浠ヤ粠鎴戣繖閲岋紝鎴?鍦ㄤ綘鏈夌殑瀹屾暣 ARCnet 杞欢鍖呬腑鑾峰彇锛夋潵鍒楀嚭 arcnet 缂撳啿鍖虹殑鍐呭銆傝浠庝腑鐪嬪嚭浠讳綍
鎰忎箟锛屼綘搴旇鑾峰彇鐩稿叧鐨?RFC銆傦紙鏈変簺鍒楀湪 arcnet.c 椤堕儴闄勮繎銆傦級arcdump 鍋囧畾浣犵殑鍗?鍦?0xD0000銆傚鏋滀笉鏄紝璇风紪杈戣鑴氭湰銆?
缂撳啿鍖?0 鍜?1 鐢ㄤ簬鎺ユ敹锛岀紦鍐插尯 2 鍜?3 鐢ㄤ簬鍙戦€併€備箳涔撶紦鍐诧紙ping-pong buffers锛?鍦ㄤ袱涓柟鍚戜笂閮藉疄鐜颁簡銆?
濡傛灉浣犵殑璋冭瘯绾у埆鍖呭惈 D_DURING 骞朵笖浣犳病鏈夊畾涔?SLOW_XMIT_COPY锛岄偅涔堟瘡娆″崱琚浣?鏃讹紙杩欏彧搴旇鍙戠敓鍦ㄤ綘鍋?ifconfig up 鏃讹紝鎴栬€呭綋 Linux 鍒ゅ畾椹卞姩宸叉崯鍧忔椂锛夛紝缂撳啿鍖?閮戒細琚竻鎴愪竴涓父閲忓€?0x42銆傚湪鍙戦€佽繃绋嬩腑锛岀紦鍐插尯鐨勬湭浣跨敤閮ㄥ垎涔熶細琚竻鎴?0x42銆?杩欐槸涓轰簡鏇村鏄撳紕娓呮涓€涓暟鎹寘浣跨敤浜嗗摢浜涘瓧鑺傘€?
```

	ifconfig arc0 down metric 1xxx
	/etc/rc.d/rc.inet1

```

鍏朵腑 "xxx" 鏄綘鎯宠鐨勮皟璇曠骇鍒€備緥濡傦紝"metric 1015" 浼氭妸浣犵疆浜庤皟璇曠骇鍒?15銆傝皟璇?绾у埆 7 鐩墠鏄粯璁ゅ€笺€?
娉ㄦ剰锛堜粠 v1.90 ALPHA 寮€濮嬶級璋冭瘯绾у埆鏄笉鍚岃皟璇曟爣蹇楃殑浜岃繘鍒剁粍鍚堬紱鎵€浠ヨ皟璇曠骇鍒?7
瀹為檯涓婃槸 1+2+4锛屽嵆 D_NORMAL+D_EXTRA+D_INIT銆傝鍖呭惈 D_DURING锛屼綘闇€瑕佸啀鍔犱笂 16锛?寰楀埌璋冭瘯绾у埆 23銆?
濡傛灉浣犱笉鏄庣櫧杩欎釜锛屼綘鍙兘鍙嶆涔熶笉鎯崇煡閬撱€傚氨浣犳墍閬囧埌鐨勯棶棰樼粰鎴戝彂閭欢鍚с€?

### 鎴戞兂瀵勯挶锛氱幇鍦ㄥ仛浠€涔堬紵


鍘荤潯涓崍瑙夋垨鍋氱偣鍒殑銆備綘鏃╀笂璧锋潵浼氭劅瑙夊ソ浜涖€?