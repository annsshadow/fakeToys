锘?
## bttv 椹卞姩


### bttv 鍙戣璇存槑


```
    ./scripts/config -e PCI
    ./scripts/config -m I2C
    ./scripts/config -m INPUT
    ./scripts/config -m MEDIA_SUPPORT
    ./scripts/config -e MEDIA_PCI_SUPPORT
    ./scripts/config -e MEDIA_ANALOG_TV_SUPPORT
    ./scripts/config -e MEDIA_DIGITAL_TV_SUPPORT
    ./scripts/config -e MEDIA_RADIO_SUPPORT
    ./scripts/config -e RC_CORE
    ./scripts/config -m VIDEO_BT848

```
```
    ./scripts/config -m DVB_BT8XX

```
鍦ㄨ繖绉嶆儏鍐典笅锛岃鍙傞槄 Documentation/admin-guide/media/bt8xx.rst 鑾峰彇棰濆璇存槑銆?

### 璁╀綘鐨勫崱浣跨敤 bttv


濡傛灉浣犲凡缁忕紪璇戝苟瀹夎浜?bttv锛屽彧闇€鍚姩鍐呮牳灏卞簲褰撹冻浠ヨ瀹冨皾璇曟帰娴嬨€備笉杩囷紝鍙栧喅浜庡瀷鍙凤紝鍐呮牳鍙兘闇€瑕佸叧浜庣‖浠剁殑棰濆淇℃伅锛屽洜涓鸿澶囧彲鑳芥棤娉曠洿鎺ュ悜鍐呮牳鎻愪緵姝ょ被淇℃伅銆?

濡傛灉 bttv 娌¤兘宸ヤ綔锛屽緢鍙兘鏄畠鏃犳硶鑷姩鎺㈡祴鍒颁綘鐨勫崱锛岄渶瑕佷竴浜?insmod 閫夐」銆俠ttv 鏈€閲嶈鐨?insmod 閫夐」鏄?"card=n"锛岀敤浜庨€夋嫨姝ｇ‘鐨勫崱绫诲瀷銆傚鏋滀綘寰楀埌浜嗚棰戜絾娌℃湁澹伴煶锛屼綘寰堝彲鑳芥寚瀹氫簡閿欒锛堟垨娌℃湁鎸囧畾锛夌殑鍗＄被鍨嬨€傚彈鏀寔鍗＄殑鍒楄〃鍦?Documentation/admin-guide/media/bttv-cardlist.rst銆?

濡傛灉 bttv 鍔犺浇闈炲父鎱紙鍦ㄤ娇鐢ㄦ病鏈夎皟璋愬櫒鐨勫粔浠峰崱鏃舵湁鏃朵細鍙戠敓锛夛紝灏濊瘯鎶婁互涓嬪唴瀹瑰姞鍏ヤ綘鐨勬ā鍧楅厤缃枃浠讹紙閫氬父瀹冩槸 `/etc/modules.conf` 鎴?`/etc/modules-load.d/` 涓嬬殑鏌愪釜鏂囦欢锛屼絾鍏蜂綋浣嶇疆鍙栧喅浜庝綘鐨勫彂琛岀増锛夛細

```
	options i2c-algo-bit bit_test=1

```
鏌愪簺鍗″彲鑳介渶瑕佷竴涓澶栫殑鍥轰欢鏂囦欢鎵嶈兘宸ヤ綔銆備緥濡傦紝瀵逛簬 WinTV/PVR锛屼綘闇€瑕佸叾椹卞姩 CD 涓殑涓€涓悕涓?`hcwamc.rbf` 鐨勫浐浠舵枃浠躲€傚畠鍦ㄤ竴涓悕涓?`pvr45xxx.exe` 鐨勮嚜瑙ｅ帇 zip 鏂囦欢涓€傚彧闇€鎶婂畠鏀惧湪 `/etc/firmware` 鐩綍涓嬶紝灏辫冻浠ヨ瀹冨湪椹卞姩鎺㈡祴妯″紡锛堜緥濡傚唴鏍稿惎鍔ㄦ椂鎴栨墜鍔ㄩ€氳繃 `modprobe` 鍛戒护鍔犺浇椹卞姩鏃讹級琚嚜鍔ㄥ姞杞姐€?

濡傛灉浣犵殑鍗℃病鏈夊垪鍦?Documentation/admin-guide/media/bttv-cardlist.rst锛屾垨鑰呬綘鍦ㄨ闊抽宸ヤ綔鏃堕亣鍒板洶闅撅紝璇烽槄璇?still_doesnt_work銆?

### 鑷姩鎺㈡祴鍗?


bttv 浣跨敤 PCI 瀛愮郴缁?ID 鏉ヨ嚜鍔ㄦ帰娴嬪崱绫诲瀷銆俵spci 鍦ㄧ浜岃鍒楀嚭瀛愮郴缁?ID锛岀湅璧锋潵鍍忚繖鏍凤細

```
	00:0a.0 Multimedia video controller: Brooktree Corporation Bt878 (rev 02)
		Subsystem: Hauppauge computer works Inc. WinTV/GO
		Flags: bus master, medium devsel, latency 32, IRQ 5
		Memory at e2000000 (32-bit, prefetchable) [size=4K]

```
鍙湁鍩轰簬 bt878 鐨勫崱鍙互鏈夊瓙绯荤粺 ID锛堣繖骞朵笉鎰忓懗鐫€姣忓紶鍗＄湡鐨勯兘鏈夛級銆俠t848 鍗′笉鑳芥湁瀛愮郴缁?ID锛屽洜姝ゆ棤娉曡鑷姩鎺㈡祴銆傚湪 Documentation/admin-guide/media/bttv-cardlist.rst 鏈変竴浠藉甫鏈夎繖浜?ID 鐨勫垪琛紙濡傛灉浣犳劅鍏磋叮鎴栨兂鍙戣ˉ涓佹洿鏂帮級銆?


### 浠嶇劧涓嶅伐浣滐紵


鎴戝閲屾病鏈夋憜鐫€ 30 澶氬潡涓嶅悓閲囬泦鏉跨殑瀹為獙瀹わ紝涔熸病鏈?PAL/NTSC/SECAM 娴嬭瘯淇″彿鍙戠敓鍣紝鎵€浠ユ垜甯稿父鏃犳硶澶嶇幇浣犵殑闂銆傝繖浣垮緱璋冭瘯瀵规垜鏉ヨ闈炲父鍥伴毦銆?

濡傛灉浣犳湁涓€浜涚煡璇嗗拰绌洪棽鏃堕棿锛岃灏濊瘯鑷繁淇锛堝綋鐒堕潪甯告杩庤ˉ涓佲€︹€︼級浣犵煡閬撶殑锛歀inux 鐨勫彛鍙锋槸 "Do it yourself"锛堣嚜宸卞姩鎵嬶級銆?

鏈変竴涓偖浠跺垪琛ㄥ湪
http://vger.kernel.org/vger-lists.html#linux-media

濡傛灉浣犲湪鏌愬潡鐗瑰畾鐢佃鍗′笂閬囧埌楹荤儲锛岃灏濊瘯鍦ㄩ偅閲屾彁闂紝鑰屼笉鏄洿鎺ョ粰鎴戝彂閭欢銆傞偅閲屾湁浣跨敤鐩稿悓鍗＄殑浜哄惉鍒扮殑鍙兘鎬ц楂樺緱澶氣€︹€?

瀵逛簬澹伴煶闂锛氫笘鐣屽悇鍦扮敤浜庣數瑙嗗０闊崇殑绯荤粺鏈夊緢澶氫笉鍚岀绫汇€傚苟涓斾篃鏈変笉鍚岀殑鑺墖鏉ヨВ鐮侀煶棰戜俊鍙枫€傚叧浜庡０闊抽棶棰樼殑鎶ュ憡锛?绔嬩綋澹颁笉宸ヤ綔"锛夐櫎闈炰綘鍖呭惈涓€浜涘叧浜庝綘鐨勭‖浠朵互鍙婁綘鎵€鍦ㄥ浗瀹朵娇鐢ㄧ殑鐢佃澹伴煶鍒跺紡鐨勭粏鑺傦紙鎴栬嚦灏戞槸浣犳墍灞呬綇鐨勫浗瀹讹級锛屽惁鍒欏熀鏈病鏈夌敤澶勩€?

### Modprobe 閫夐」



   The following argument list can be outdated, as we might add more
   options if ever needed. In case of doubt, please check with
   `modinfo <module>`.

   璇ュ懡浠ゆ墦鍗板叧浜庡唴鏍告ā鍧楃殑鍚勭淇℃伅锛屽叾涓寘鎷竴浠藉畬鏁翠笖鏈€鏂扮殑 insmod 閫夐」鍒楄〃銆?

   This command prints various information about a kernel
   module, among them a complete and up-to-date list of insmod options.


bttv
	bt848/878锛堥噰闆嗚姱鐗囷級椹卞姩

```
	    card=n		card type, see CARDLIST for a list.
	    tuner=n		tuner type, see CARDLIST for a list.
	    radio=0/1	card supports radio
	    pll=0/1/2	pll settings

			    0: don't use PLL
			    1: 28 MHz crystal installed
			    2: 35 MHz crystal installed

	    triton1=0/1     for Triton1 (+others) compatibility
	    vsfx=0/1	yet another chipset bug compatibility bit
			    see README.quirks for details on these two.

	    bigendian=n	Set the endianness of the gfx framebuffer.
			    Default is native endian.
	    fieldnr=0/1	Count fields.  Some TV descrambling software
			    needs this, for others it only generates
			    50 useless IRQs/sec.  default is 0 (off).
	    autoload=0/1	autoload helper modules (tuner, audio).
			    default is 1 (on).
	    bttv_verbose=0/1/2  verbose level (at insmod time, while
			    looking at the hardware).  default is 1.
	    bttv_debug=0/1	debug messages (for capture).
			    default is 0 (off).
	    irq_debug=0/1	irq handler debug messages.
			    default is 0 (off).
	    gbuffers=2-32	number of capture buffers for mmap'ed capture.
			    default is 4.
	    gbufsize=	size of capture buffers. default and
			    maximum value is 0x208000 (~2MB)
	    no_overlay=0	Enable overlay on broken hardware.  There
			    are some chipsets (SIS for example) which
			    are known to have problems with the PCI DMA
			    push used by bttv.  bttv will disable overlay
			    by default on this hardware to avoid crashes.
			    With this insmod option you can override this.
	    no_overlay=1	Disable overlay. It should be used by broken
			    hardware that doesn't support PCI2PCI direct
			    transfers.
	    automute=0/1	Automatically mutes the sound if there is
			    no TV signal, on by default.  You might try
			    to disable this if you have bad input signal
			    quality which leading to unwanted sound
			    dropouts.
	    chroma_agc=0/1	AGC of chroma signal, off by default.
	    adc_crush=0/1	Luminance ADC crush, on by default.
	    i2c_udelay=     Allow reduce I2C speed. Default is 5 usecs
			    (meaning 66,67 Kbps). The default is the
			    maximum supported speed by kernel bitbang
			    algorithm. You may use lower numbers, if I2C
			    messages are lost (16 is known to work on
			    all supported cards).

	    bttv_gpio=0/1
	    gpiomask=
	    audioall=
	    audiomux=
			    See Sound-FAQ for a detailed description.

	remap, card, radio and pll accept up to four comma-separated arguments
	(for multiple boards).

```
tuner
	璋冭皭鍣ㄩ┍鍔ㄣ€傞櫎闈炰綘鍙兂閰嶅悎鎽勫儚澶翠娇鐢紝鎴栬€呮澘瀛愪笉鎻愪緵妯℃嫙鐢佃璋冭皭锛屽惁鍒欎綘闇€瑕佸畠銆?

```
		debug=1		print some debug info to the syslog
		type=n		type of the tuner chip. n as follows:
				see CARDLIST for a complete list.
		pal=[bdgil]	select PAL variant (used for some tuners
				only, important for the audio carrier).

```
tvaudio
	涓烘墍鏈夌畝鍗曠殑 i2c 闊抽鎺у埗鑺墖锛坱da/tea*锛夋彁渚涘崟涓€椹卞姩銆?

```
		tda8425  = 1	enable/disable the support for the
		tda9840  = 1	various chips.
		tda9850  = 1	The tea6300 can't be autodetected and is
		tda9855  = 1	therefore off by default, if you have
		tda9873  = 1	this one on your card (STB uses these)
		tda9874a = 1	you have to enable it explicitly.
		tea6300  = 0	The two tda985x chips use the same i2c
		tea6420  = 1	address and can't be disturgished from
		pic16c54 = 1	each other, you might have to disable
				the wrong one.
		debug = 1	print debug messages

```
msp3400
	msp34xx 澹伴煶澶勭悊鍣ㄨ姱鐗囩殑椹卞姩銆傚鏋滀綘鏈変竴鍧楃珛浣撳０鍗★紝浣犲彲鑳芥兂 insmod 杩欎釜銆?

```
		debug=1/2	print some debug info to the syslog,
				2 is more verbose.
		simple=1	Use the "short programming" method.  Newer
				msp34xx versions support this.  You need this
				for dbx stereo.  Default is on if supported by
				the chip.
		once=1		Don't check the TV-stations Audio mode
				every few seconds, but only once after
				channel switches.
		amsound=1	Audio carrier is AM/NICAM at 6.5 Mhz.  This
				should improve things for french people, the
				carrier autoscan seems to work with FM only...

```
### 濡傛灉鏈哄櫒鍦?bttv 涓嬬‖鍐荤粨


瀹冨彲鑳芥槸 bttv 椹卞姩鐨?bug锛屼篃鍙兘鏄湁闂鐨勭‖浠讹紝涔熷彲鑳芥槸鍒殑鍘熷洜鈥︹€?

浠呬粎缁欐垜鍙戜竴灏?"bttv freezes"锛坆ttv 鍐荤粨锛夌殑閭欢鏄府涓嶄笂浠€涔堝繖鐨勩€傝繖涓?README 涓湁涓€浜涙彁绀猴紝鍙互甯綘瀹氫綅闂銆?


#### bttv bug


濡傛灉鏌愪釜鐗堟湰宸ヤ綔鑰屽彟涓€涓笉宸ヤ綔锛岄偅寰堝彲鑳芥槸椹卞姩 bug銆傚鏋滀綘鑳借鍑哄畠纭垏鍦ㄥ摢閲屽潖鎺夌殑锛堝嵆鏈€鍚庝竴涓兘宸ヤ綔鐨勭増鏈拰绗竴涓潖鎺夌殑鐗堟湰锛夛紝浼氶潪甯告湁甯姪銆?

瀵逛簬纭喕缁擄紝浣犲ぇ姒備笉浼氬湪鏃ュ織鏂囦欢涓壘鍒颁换浣曚笢瑗裤€傛崟鑾蜂换浣曞唴鏍告秷鎭殑鍞竴鏂规硶鏄帴涓婁竴涓覆鍙ｆ帶鍒跺彴锛屽苟璁╂煇涓粓绔簲鐢ㄧ▼搴忚褰曟秷鎭€?me 浣跨敤 screen銆傚叧浜庤缃覆鍙ｆ帶鍒跺彴鐨勭粏鑺傦紝璇峰弬闃?Documentation/admin-guide/serial-console.rst銆?

闃呰 Documentation/admin-guide/bug-hunting.rst锛屼簡瑙ｅ浣曚粠鍐呮牳鍦ㄤ繚鎶ゆ晠闅滐紙鎵€璋?"kernel oops"锛夋椂鎵撳嵃鐨勫瘎瀛樺櫒+鏍堣浆鍌ㄤ腑鑾峰彇浠讳綍鏈夌敤鐨勪俊鎭€?

濡傛灉浣犻亣鍒版煇绉嶆閿侊紝鍙互灏濊瘯鐢?sysrq-t 杞偍姣忎釜杩涚▼鐨勮皟鐢ㄦ爤璺熻釜锛堝弬瑙?Documentation/admin-guide/sysrq.rst锛夈€傝繖鏍峰氨鑳藉紕娓呮澶勪簬 "D" 鐘舵€佺殑杩涚▼ **纭垏** 鍗″湪鍝噷銆?

鎴戣杩囪繖鏍风殑鎶ュ憡锛氬鏌愪簺浜烘潵璇?bttv 0.7.x 宕╂簝锛岃€?0.8.x 宸ヤ綔寰楅潪甯哥ǔ瀹氥€傚洜姝ゅぇ姒傛槸 bttv 0.7.x 涓煇澶勮繕娈嬬暀鐫€涓€涓皬鐨?buglet銆傛垜涓嶇煡閬撶‘鍒囧湪鍝噷锛屽畠瀵规垜鍜岃澶氬叾浠栦汉閮界ǔ瀹氬伐浣溿€備絾濡傛灉浣犲湪 0.7.x 鐗堟湰涓婇亣鍒伴棶棰橈紝鍙互灏濊瘯涓€涓?0.8.x鈥︹€?


#### 纭欢 bug


鏌愪簺纭欢鏃犳硶澶勭悊 PCI-PCI 浼犺緭锛堝嵆閲囬泦鍣?=> vga锛夈€傛湁鏃堕棶棰樺氨鍥犱负 PCI 鎬荤嚎涓婄殑楂樿礋杞借€屽嚭鐜板湪 bttv 涓娿€俠t848/878 鑺墖瀵瑰凡鐭ョ殑鍏煎鎬ч棶棰樻湁鍑犱釜鍙橀€氭柟娉曪紝鍙傝 README.quirks銆?

鏈変簺浜烘姤鍛婅鎻愰珮 PCI latency锛堝欢杩燂級涔熸湁甯姪锛岃櫧鐒舵垜涓嶇‘瀹氳繖鍒板簳鏄湡姝ｄ慨澶嶄簡闂锛岃繕鏄彧鏄瀹冧笉澶彲鑳藉彂鐢熴€俠ttv 鍜?btaudio 閮芥湁涓€涓?insmod 閫夐」鏉ヨ缃澶囩殑 PCI 寤惰繜銆?

鏌愪簺涓绘澘鍦ㄦ纭鐞嗗涓澶囧悓鏃惰繘琛?DMA 鏃舵湁闂銆俠ttv + ide 鏈夋椂浼氬鑷磋繖绉嶆儏鍐碉紝濡傛灉鏄繖鏍凤紝浣犲ぇ姒傚彧浼氬湪瑙嗛鍜岀‖鐩樿闂悓鏃惰繘琛屾椂鐪嬪埌鍐荤粨銆傛洿鏂?IDE 椹卞姩浠ヨ幏鍙栭拡瀵圭‖浠?bug 鐨勬渶鏂版渶鍏ㄧ殑鍙橀€氭柟娉曪紝鍙兘浼氫慨澶嶈繖浜涢棶棰樸€?


#### 鍏朵粬


濡傛灉浣犱娇鐢ㄤ簡鏌愪簺浠呬簩杩涘埗鐨勪笢瑗匡紙姣斿 nvidia 妯″潡锛夛紝灏濊瘯鍦ㄤ笉浣跨敤瀹冪殑鎯呭喌涓嬪鐜伴棶棰樸€?

IRQ 鍏变韩鍦ㄦ煇浜涙儏鍐典笅宸茬煡浼氬紩璧烽棶棰樸€傜悊璁轰笂鍜屽湪璁稿閰嶇疆涓畠宸ヤ綔寰楀緢濂姐€備笉杩囷紝鍊煎緱涓€璇曞幓閲嶆柊鎽嗘斁 PCI 鍗★紝缁?bttv 鍙︿竴涓?IRQ锛屾垨鑰呰瀹冨拰鍒殑纭欢鍏变韩 IRQ銆備笌 VGA 鍗″叡浜?IRQ 鏈夋椂浼间箮浼氬甫鏉ラ夯鐑︺€傛垜涔熻杩?bttv 涓?ACPI 妗ワ紙浠ュ強鍚敤浜?apci 鐨勫唴鏍革級鍏变韩 IRQ 鏃剁殑濂囨€幇璞°€?

### Bttv 鍏煎鎬э紙quirks锛?


涓嬮潰鏄?bt878 鏁版嵁鎵嬪唽鍏充簬 bt878 鑺墖 PCI bug 鍏煎妯″紡鐨勮鏄庛€?

triton1 insmod 閫夐」璁剧疆鎺у埗瀵勫瓨鍣ㄤ腑鐨?EN_TBFX 浣嶃€倂sfx insmod 閫夐」瀵?EN_VSFX 浣嶅仛鍚屾牱鐨勪簨鎯呫€傚鏋滀綘鏈夌ǔ瀹氭€ч棶棰橈紝鍙互灏濊瘯鍏朵腑涓€涓€夐」鏄惁鑳借浣犵殑鏈哄櫒绋冲畾宸ヤ綔銆?

drivers/pci/quirks.c 浜嗚В杩欎簺闂锛岃繖鏍疯繖浜涗綅灏变細涓哄凡鐭ョ殑鏈?bug 鑺墖缁勮嚜鍔ㄥ惎鐢紙鏌ョ湅鍐呮牳娑堟伅锛宐ttv 浼氬憡璇変綘锛夈€?

#### 鏅€?PCI 妯″紡


PCI REQ 淇″彿鏄杈撳叆鐨勫姛鑳借姹傜殑閫昏緫鎴栵紙logical-or锛夈€傚唴閮ㄧ殑 GNT[0:1] 淇″彿涓?GNT 寮傛閫夐€氾紝骞剁敱闊抽璇锋眰淇″彿瑙ｅ鐢ㄣ€傚洜姝や徊瑁佸櫒鍦ㄥ姞鐢垫椂榛樿涓鸿棰戝姛鑳斤紝骞跺湪娌℃湁鎬荤嚎璁块棶璇锋眰鏃跺仠鍦ㄩ偅閲屻€傝繖鏄彲鍙栫殑锛屽洜涓鸿棰戜細鏇撮绻佸湴璇锋眰鎬荤嚎銆備笉杩囷紝闊抽灏嗘嫢鏈夋渶楂樼殑鎬荤嚎璁块棶浼樺厛绾с€傚洜姝わ紝鍗充娇闊抽鍦ㄨ棰戣姹備箣鍚庛€佷絾鍦?PCI 澶栭儴浠茶鍣ㄦ巿浜堝 Bt879 鐨勮闂箣鍓嶅彂鍑鸿姹傦紝闊抽涔熷皢棣栧厛鑾峰緱鎬荤嚎璁块棶鏉冦€備竴鏃︽煇涓姛鑳戒笂浜嗘€荤嚎锛屽彟涓€涓姛鑳藉氨鏃犳硶鎶㈠崰瀹冦€傛妸鏁翠釜瑙嗛 PCI FIFO 鍊掔┖鍒?PCI 鎬荤嚎涓婄殑鏃堕棿锛岀浉瀵逛簬闊抽 PCI FIFO 鎵€鑳藉蹇嶇殑鎬荤嚎璁块棶寤惰繜鏉ヨ闈炲父鐭€?


#### 430FX 鍏煎妯″紡


浣跨敤 430FX PCI 鏃讹紝浠ヤ笅瑙勫垯灏嗙‘淇濆吋瀹规€э細

 (1) 鍦ㄦ柇瑷€ FRAME 鐨勫悓鏃舵挙閿€锛坉eassert锛塕EQ銆?
 (2) 鍦ㄧ粨鏉熷墠涓€涓簨鍔′箣鍓嶏紝涓嶈閲嶆柊鏂█ REQ 浠ヨ姹傚彟涓€涓€荤嚎浜嬪姟銆?

鐢变簬鍚勪釜鎬荤嚎涓绘帶涓嶈兘鐩存帴鎺у埗 REQ锛岃棰戝拰闊抽璇锋眰绠€鍗曠殑閫昏緫鎴栦細杩濆弽瑙勫垯銆傚洜姝わ紝浠茶鍣ㄥ拰鍙戣捣鏂归兘鍖呭惈 430FX 鍏煎妯″紡閫昏緫銆傝鍚敤 430FX 妯″紡锛岃鎸夌 104 椤佃澶囨帶鍒跺瘎瀛樺櫒涓殑鎸囩ず璁剧疆 EN_TBFX 浣嶃€?

褰撳惎鐢?EN_TBFX 鏃讹紝浠茶鍣ㄧ‘淇濇弧瓒宠繖涓や釜鍏煎瑙勫垯銆傚湪 PCI 浠茶鍣ㄦ柇瑷€ GNT 涔嬪墠锛岃繖涓唴閮ㄤ徊瑁佸櫒浠嶇劧鍙互灏嗕袱涓姹傞€昏緫鎴栬捣鏉ャ€傜劧鑰岋紝涓€鏃?GNT 琚彂鍑猴紝杩欎釜浠茶鍣ㄥ繀椤婚攣瀹氬畠鐨勫喅瀹氾紝鐜板湪鍙妸琚巿浜堢殑璇锋眰璺敱鍒?REQ 寮曡剼銆備徊瑁佸櫒鍐冲畾閿佸畾涓嶇 FRAME 鐨勭姸鎬佸浣曢兘浼氬彂鐢燂紝鍥犱负瀹冧笉鐭ラ亾 FRAME 浣曟椂浼氳鏂█锛堝吀鍨嬫儏鍐垫槸鈥斺€旀瘡涓彂璧锋柟浼氬湪 GNT 涔嬪悗鐨勫懆鏈熸柇瑷€ FRAME锛夈€傚綋 FRAME 琚柇瑷€鏃讹紝绉婚櫎鍏惰姹傛槸鍙戣捣鏂圭殑璐ｄ换銆傚厑璁歌繖涓姹傛祦缁忓埌 REQ 鑰屼笉鍏佽鍙︿竴涓姹備繚鎸?REQ 琚柇瑷€锛屾槸浠茶鍣ㄧ殑璐ｄ换銆傚喅瀹氶攣瀹氬彲浠ュ湪浜嬪姟缁撴潫鏃惰В闄わ細渚嬪锛屽綋鎬荤嚎绌洪棽鏃讹紙FRAME 鍜?IRDY锛夈€傜劧鍚庝徊瑁佸櫒鍐冲畾鍙互缁х画寮傛杩涜锛岀洿鍒?GNT 鍐嶆琚柇瑷€銆?


#### 涓庝笉绗﹀悎 PCI 2.1 鐨勬牳蹇冮€昏緫鎺ュ彛


涓€灏忛儴鍒嗘牳蹇冮€昏緫璁惧鍙兘鍦?GNT 琚挙閿€鐨勫悓涓€鍛ㄦ湡鍚姩涓€涓€荤嚎浜嬪姟銆傝繖涓嶇鍚?PCI 2.1銆備负纭繚涓庝娇鐢ㄨ繖浜?PCI 鎺у埗鍣ㄧ殑 PC 鍏煎锛屽繀椤诲惎鐢?EN_VSFX 浣嶏紙鍙傝绗?104 椤佃澶囨帶鍒跺瘎瀛樺櫒锛夈€傚湪杩欑妯″紡涓嬶紝浠茶鍣ㄤ笉浼氭妸 GNT 浼犻€掔粰鍐呴儴鍔熻兘锛岄櫎闈?REQ 琚柇瑷€銆傝繖闃叉浜嗘€荤嚎浜嬪姟鍦?GNT 琚挙閿€鐨勫悓涓€鍛ㄦ湡鍚姩銆傝繖涔熸湁涓€涓壇浣滅敤锛屽嵆鏃犳硶鍒╃敤鎬荤嚎鍋滄斁锛坆us parking锛夛紝浠庤€岄檷浣庝簡浠茶鎬ц兘銆侭t879 椹卞姩蹇呴』鏌ヨ杩欎簺涓嶅吋瀹圭殑璁惧锛屽苟涓斾粎鍦ㄩ渶瑕佹椂璁剧疆 EN_VSFX 浣嶃€?


#### tvcards 鏁扮粍鐨勫叾浠栧厓绱?


濡傛灉浣犳璇曞浘璁╀竴寮犳柊鍗″伐浣滐紝浣犲彲鑳戒細鍙戠幇鏌ョ湅浠ヤ笅鍐呭寰堟湁鐢細

```
	video_inputs    - # of video inputs the card has
	audio_inputs    - historical cruft, not used any more.
	tuner           - which input is the tuner
	svhs            - which input is svhs (all others are labeled composite)
	muxsel          - video mux, input->registervalue mapping
	pll             - same as pll= insmod option
	tuner_type      - same as tuner= insmod option
	*_modulename    - hint whenever some card needs this or that audio
			module loaded to work properly.
	has_radio	- whenever this TV card has a radio tuner.
	no_msp34xx	- "1" disables loading of msp3400.o module
	no_tda9875	- "1" disables loading of tda9875.o module
	needs_tvaudio	- set to "1" to load tvaudio.o module

```
濡傛灉鏌愪釜閰嶇疆椤瑰悓鏃朵粠 tvcards 鏁扮粍鍜?insmod 閫夐」鎸囧畾锛屽垯 insmod 閫夐」浼樺厛銆?

### 鍗?



   For a more updated list, please check
   https://linuxtv.org/wiki/index.php/Hardware_Device_Information

#### 鍙楁敮鎸佺殑鍗★細Bt848/Bt848a/Bt849/Bt878/Bt879 鍗?


鎵€鏈夊甫鏈?Bt848/Bt848a/Bt849/Bt878/Bt879 浠ュ強鏅€?Composite/S-VHS 杈撳叆鐨勫崱閮藉彈鏀寔銆傞€氳繃杞欢涓殑 VBI 閲囨牱瑙ｇ爜锛屾墍鏈夊崱閮芥敮鎸佸浘鏂囩數瑙嗭紙Teletext锛夊拰 Intercast锛堜粎 PAL锛夈€?

鏌愪簺甯︽湁棰濆杈撳叆澶嶇敤鎴栧叾浠栬姳鍝ㄨ姱鐗囩殑鍗″彧寰楀埌閮ㄥ垎鏀寔锛堥櫎闈炲崱鍒堕€犲晢鎻愪緵浜嗚鏍艰鏄庯級銆傚綋涓€寮犲崱鍒楀湪杩欓噷鏃讹紝瀹冧笉涓€瀹氳瀹屽叏鏀寔銆?

鎵€鏈夊叾浠栧崱鍙槸閫氳繃璋冭皭鍣ㄣ€佸０闊宠В鐮佸櫒銆丒EPROM銆佸浘鏂囩數瑙嗚В鐮佸櫒绛夐澶栫粍浠惰€屼笉鍚屻€?


#### MATRIX Vision


MV-Delta
- Bt848A
- 4 涓?Composite 杈撳叆锛? 涓?S-VHS 杈撳叆锛堜笌绗?4 涓?composite 鍏变韩锛?
- EEPROM

http://www.matrix-vision.de/

杩欏紶鍗℃病鏈夎皟璋愬櫒锛屼絾鏀寔 Bt848A 鐨勫叏閮?4 涓?composite锛堝叾涓?1 涓笌 S-VHS 杈撳叆鍏变韩锛夈€傚鏋滀綘鍙湁鍗槦鐢佃銆佷絾鏈夊涓皟璋愬櫒閫氳繃 composite 杩炲埌鍗′笂锛岃繖鏄竴寮犻潪甯镐笉閿欑殑鍗°€?

闈炲父鎰熻阿 Matrix-Vision 鍏嶈垂缁欎簡鎴戜滑 2 寮犲崱锛屼娇寰?Bt848a/Bt849 鍗曟櫠鎸搷浣滄敮鎸佹垚涓哄彲鑳斤紒锛侊紒


#### Miro/Pinnacle PCTV


- Bt848
  鏈変簺锛堝叏閮紵锛燂級甯?2 涓櫠鎸紝鐢ㄤ簬 PAL/SECAM 鍜?NTSC
- PAL銆丼ECAM 鎴?NTSC 鐢佃璋冭皭鍣紙Philips 鎴?TEMIC锛?
- MSP34xx 澹伴煶瑙ｇ爜鍣ㄥ湪闄勫姞鏉夸笂
  瑙ｇ爜鍣ㄥ彈鏀寔锛屼絾鎹垜鎵€鐭ヨ繕涓嶈兘鐢?
  锛圙PIO 绔彛涓渶瑕佸叾浠栧０闊?MUX 璁剧疆锛燂紵锛熸湁浜轰慨澶嶄簡杩欎釜闂鍚楋紵锛燂紵锛?
- 1 涓皟璋愬櫒锛? 涓?composite 鍜?1 涓?S-VHS 杈撳叆
- 璋冭皭鍣ㄧ被鍨嬭嚜鍔ㄦ帰娴?

http://www.miro.de/
http://www.miro.com/


闈炲父鎰熻阿杩欏紶鍏嶈垂鍗★紝浣?1997 骞寸殑棣栦釜 NTSC 鏀寔鎴愪负鍙兘锛?


#### Hauppauge Win/TV pci


鏈夎澶氫笉鍚岀増鏈殑 Hauppauge 鍗★紝甯︽湁涓嶅悓鐨勮皟璋愬櫒锛圱V+Radio鈥︹€︼級銆佸浘鏂囩數瑙嗚В鐮佸櫒銆傛敞鎰忥紝鍗充娇鍨嬪彿缂栧彿鐩稿悓鐨勫崱锛堝彇鍐充簬淇鐗堟湰锛変笂闈㈢殑鑺墖涔熶笉鍚屻€?

- Bt848锛堜互鍙婂叾浠栵紝浣嗘€绘槸浠?2 鏅舵尟鎿嶄綔锛燂紵锛燂級
  杈冩柊鐨勫崱鏈?Bt878

- PAL銆丼ECAM銆丯TSC 璋冭皭鍣紝甯︽垨涓嶅甫 Radio 鏀寔

渚嬪锛?

- PAL:

  - TDA5737: VHF銆佽秴楂橀甯︼紙hyperband锛夊拰 UHF 娣烽鍣?鎸崱鍣紝鐢ㄤ簬 TV 鍜?VCR 3 棰戞璋冭皭鍣?
  - TSA5522: 1.4 GHz I2C 鎬荤嚎鎺у埗鍚堟垚鍣紝I2C 0xc2-0xc3

- NTSC:

  - TDA5731: VHF銆佽秴楂橀甯﹀拰 UHF 娣烽鍣?鎸崱鍣紝鐢ㄤ簬 TV 鍜?VCR 3 棰戞璋冭皭鍣?
  - TSA5518: Philips 绔欑偣涓婃病鏈夋暟鎹墜鍐屽彲鐢?

- Philips SAA5246 鎴?SAA5284锛堟垨鏃狅級鍥炬枃鐢佃瑙ｇ爜鍣ㄨ姱鐗?
  甯︾紦鍐?RAM锛堜緥濡?Winbond W24257AS-35: 32Kx8 CMOS 闈欐€?RAM锛?
  SAA5246锛圛2C 0x22锛夊彈鏀寔

- 256 瀛楄妭 EEPROM: Microchip 24LC02B 鎴?Philips 8582E2Y
  甯︽湁閰嶇疆淇℃伅
  I2C 鍦板潃 0xa0锛?4LC02B 涔熷搷搴?0xa2-0xaf锛?

- 1 涓皟璋愬櫒锛? 涓?composite 鍜岋紙鍙栧喅浜庡瀷鍙凤級1 涓?S-VHS 杈撳叆

- 14052B: 鐢ㄤ簬閫夋嫨澹伴煶婧愮殑澶嶇敤鍣紙mux锛?

- 澹伴煶瑙ｇ爜鍣? TDA9800銆丮SP34xx锛堢珛浣撳０鍗★級


#### Askey CPH 绯诲垪


鐢?TelSignal锛? 寮€鍙戯紝鐢辫澶氬巶鍟?OEM锛圱yphoon銆丄nubis銆丏ynalink锛?

- 鍗＄郴鍒?
  - CPH01x: BT848 浠呴噰闆?
  - CPH03x: BT848
  - CPH05x: BT878 甯?FM
  - CPH06x: BT878锛堟棤 FM锛?
  - CPH07x: BT878 浠呴噰闆?

- 鐢佃鏍囧噯:
  - CPH0x0: NTSC-M/M
  - CPH0x1: PAL-B/G
  - CPH0x2: PAL-I/I
  - CPH0x3: PAL-D/K
  - CPH0x4: SECAM-L/L
  - CPH0x5: SECAM-B/G
  - CPH0x6: SECAM-D/K
  - CPH0x7: PAL-N/N
  - CPH0x8: PAL-B/H
  - CPH0x9: PAL-M/M

- CPH03x 甯镐綔涓?"TV capturer" 鍑哄敭銆?

璇嗗埆:

  #) 878 鍗″彲浠ラ€氳繃 PCI 瀛愮郴缁?ID 璇嗗埆:
     - 144f:3000 = CPH06x
     - 144F:3002 = CPH05x w/ FM
     - 144F:3005 = CPH06x_LC锛堟棤閬ユ帶锛?
  #) 鍗¤儗闈㈡湁涓€涓甫 "CPH" 鍨嬪彿鐨勮创绾搞€?
  #) 杩欎簺鍗″湪璋冭皭鍣ㄩ噾灞炵洅姝ｄ笂鏂圭殑 PCB 涓婂嵃鏈変竴涓暟瀛?
     - "80-CP2000300-x" = CPH03X
     - "80-CP2000500-x" = CPH05X
     - "80-CP2000600-x" = CPH06X / CPH06x_LC

  Askey 鎶婅繖浜涘崱浣滀负 "Magic TView series" 鍑哄敭锛屽搧鐗屼负 "MagicXpress"銆?
  鍏朵粬 OEM 甯哥О杩欎簺涓?"Tview"銆?TView99" 绛夈€?

#### Lifeview Flyvideo 绯诲垪:


杩欎簺绯诲垪鐨勫懡鍚嶉殢鏃堕棿涓庡湴鍩熻€屼笉鍚屻€?

璇嗗埆:
  #) 鏌愪簺鍨嬪彿鍙互閫氳繃 PCI 瀛愮郴缁?ID 璇嗗埆:

     - 1852:1852 = Flyvideo 98 FM
     - 1851:1850 = Flyvideo 98
     - 1851:1851 = Flyvideo 98 EZ锛堜粎閲囬泦锛?

  #) PCB 涓婃湁涓€涓嵃瀛?

     - LR25       = Flyvideo锛圸oran ZR36120, SAA7110A锛?
     - LR26 Rev.N = Flyvideo II锛圔t848锛?
     - LR26 Rev.O = Flyvideo II锛圔t878锛?
     - LR37 Rev.C = Flyvideo EZ锛堜粎閲囬泦, ZR36120 + SAA7110锛?
     - LR38 Rev.A1= Flyvideo II EZ锛圔t848 浠呴噰闆嗭級
     - LR50 Rev.Q = Flyvideo 98锛堝甫 eeprom 鍜?PCI 瀛愮郴缁?ID锛?
     - LR50 Rev.W = Flyvideo 98锛堟棤 eeprom锛?
     - LR51 Rev.E = Flyvideo 98 EZ锛堜粎閲囬泦锛?
     - LR90       = Flyvideo 2000锛圔t878锛?
     - LR90 Flyvideo 2000S锛圔t878锛夊甫绔嬩綋澹?TV锛堝寘瑁呭惈 LR91 瀛愭澘锛?
     - LR91       = LR90 鐨勭珛浣撳０瀛愬崱
     - LR97       = Flyvideo DVBS
     - LR99 Rev.E = 鐢ㄤ簬 OEM 闆嗘垚鐨勮杽鍨嬪崱锛堜粎鍐呴儴闊抽锛侊級bt878
     - LR136	 = Flyvideo 2100/3100锛堣杽鍨? SAA7130/SAA7134锛?
     - LR137      = Flyvideo DV2000/DV3000锛圫AA7130/SAA7134 + IEEE1394锛?
     - LR138 Rev.C= Flyvideo 2000锛圫AA7130锛?
     - LR138 Flyvideo 3000锛圫AA7134锛夊甫绔嬩綋澹?TV

 - 杩欎簺瀛樺湪甯?FM 鍜屽甫 Remote 鐨勫彉浣擄紝鏈夋椂鐢ㄥ悗缂€ "FM" 鍜?"R" 琛ㄧず銆?

  #) 浣犳湁涓€鍙扮瑪璁版湰锛坢iniPCI 鍗★級:

      - Product    = FlyTV Platinum Mini
      - Model/Chip = LR212/saa7135

      - Lifeview.com.tw 璇存槑锛?002 骞?2 鏈堬級:
        "FlyVideo2000 鍜?FlyVideo2000s 浜у搧鍚嶅凡閲嶅懡鍚嶄负 FlyVideo98銆?
        瀹冧滑鐨?Bt8x8 鍗¤鍒椾负宸插仠浜с€?
      - Flyvideo 2000S 鍦ㄦ煇浜涘浗瀹讹紙娆ф床锛燂級鍙兘浣滀负 Flyvideo 3000 鍑哄敭銆?
        鏂扮殑 Flyvideo 2000/3000 鏄熀浜?SAA7130/SAA7134 鐨勩€?

"Flyvideo II" 鏇炬槸 848 鍗＄殑鍚嶇О锛屽浠婏紙鍦ㄥ痉鍥斤級杩欎釜鍚嶅瓧琚噸鏂扮敤浜?LR50 Rev.W銆?

Lifeview 缃戠珯鏇惧湪鏌愪簺鏃跺€欐彁鍒?Flyvideo III锛屼絾杩欐牱鐨勫崱灏氭湭瑙佽繃锛堜篃璁稿畠鏄?LR90 [绔嬩綋澹癩 鐨勫痉鏂囧悕锛夈€傝繖浜涘崱涔熻璁稿 OEM 鍑哄敭銆?

FlyVideo A2锛圗lta 8680锛? LR90 Rev.F锛堝甫 Remote锛屾棤 FM锛岀珛浣撳０ TV 鐢?tda9821锛墈寰峰浗}

Lifeview 3000锛圗lta 8681锛夋寜 Plus锛?002 骞?4 鏈堬紝寰峰浗锛夊嚭鍞?= LR138 w/ saa7134

##### lifeview 鍦?gpio 寮曡剼 0-9 涓婄殑閰嶇疆缂栫爜


- LR50 rev. Q锛?PARTS: 7031505116锛夛紝璋冭皭鍣ㄨ璇嗗埆涓?Nr. 5锛岃緭鍏?
  SVideo銆乀V銆丆omposite銆丄udio銆丷emote:

 - CP9..1=100001001锛?: 0 娆у鐢甸樆鏈剨鎺ュ埌 GND锛?: 宸茬剨鎺ワ級


#### Typhoon 鐢佃鍗＄郴鍒?


杩欎簺鍙互鏄?CPH銆丗lyvideo銆丳ixelview 鎴?KNC1 绯诲垪銆?

Typhoon 鏄?Anubis 鐨勫搧鐗屻€?

鍨嬪彿 50680 琚噸鏂颁娇鐢紝鏌愪簺鍨嬪彿缂栧彿闅忔椂闂存湁涓嶅悓鍐呭銆?

鍨嬪彿:

  - 50680 "TV Tuner PCI Pal BG"锛堟棫锛岀孩鑹插寘瑁咃級= 鍙互鏄?CPH03x(bt848) 鎴?CPH06x(bt878)
  - 50680 "TV Tuner Pal BG"锛堣摑鑹插寘瑁咃級= Pixelview PV-BT878P+锛圧ev 9B锛?
  - 50681 "TV Tuner PCI Pal I"锛?0680 鐨勫彉浣擄級
  - 50682 "TView TV/FM Tuner Pal BG"       = Flyvideo 98FM锛圠R50 Rev.Q锛?

```
	 The package has a picture of CPH05x (which would be a real TView)

  - 50683 "TV Tuner PCI SECAM"锛?0680 鐨勫彉浣擄級
  - 50684 "TV Tuner Pal BG"                = Pixelview 878TV(Rev.3D)
  - 50686 "TV Tuner"                       = KNC1 TV Station
  - 50687 "TV Tuner stereo"                = KNC1 TV Station pro
  - 50688 "TV Tuner RDS"锛堥粦鑹插寘瑁咃級   = KNC1 TV Station RDS
  - 50689  TV SAT DVB-S CARD CI PCI (SAA7146AH, SU1278锛?= "KNC1 TV Station DVB-S"
  - 50692 "TV/FM Tuner"锛堝皬 PCB锛?
  - 50694  TV TUNER CARD RDS锛圥HILIPS CHIPSET SAA7134HL锛?
  - 50696  TV TUNER STEREO锛圥HILIPS CHIPSET SAA7134HL, MK3ME Tuner锛?
  - 50804  PC-SAT TV/Audio Karte = Techni-PC-Sat锛圸ORAN 36120PQC, Tuner:Alps锛?
  - 50866  TVIEW SAT RECEIVER+ADR
  - 50868 "TV/FM Tuner Pal I"锛?0682 鐨勫彉浣擄級
  - 50999 "TV/FM Tuner Secam"锛?0682 鐨勫彉浣擄級

```
#### Guillemot


鍨嬪彿:

- Maxi-TV PCI锛圸R36120锛?
- Maxi TV Video 2 = LR50 Rev.Q锛團I1216MF, PAL BG+SECAM锛?
- Maxi TV Video 3 = CPH064锛圥AL BG + SECAM锛?

#### Mentor


Mentor TV card锛?55-878TV-U1"锛? Pixelview 878TV(Rev.3F)锛堝甫 FM 甯?Remote锛?

#### Prolink


- 鐢佃鍗?

  - PixelView Play TV pro - (Model: PV-BT878P+ REV 8E)
  - PixelView Play TV pro - (Model: PV-BT878P+ REV 9D)
  - PixelView Play TV pro - (Model: PV-BT878P+ REV 4C / 8D / 10A )
  - PixelView Play TV - (Model: PV-BT848P+)
  - 878TV - (Model: PV-BT878TV)

- 澶氬獟浣撶數瑙嗗瑁咃紙鍗?+ 杞欢鍖咃級:

  - PixelView Play TV Theater - (Model: PV-M4200) =  PixelView Play TV pro + Software
  - PixelView Play TV PAK -     (Model: PV-BT878P+ REV 4E)
  - PixelView Play TV/VCR -     (Model: PV-M3200 REV 4C / 8D / 10A )
  - PixelView Studio PAK -      (Model:    M2200 REV 4C / 8D / 10A )
  - PixelView PowerStudio PAK - (Model: PV-M3600 REV 4E)
  - PixelView DigitalVCR PAK -  (Model: PV-M2400 REV 4C / 8D / 10A )
  - PixelView PlayTV PAK II (TV/FM card + usb camera)  PV-M3800
  - PixelView PlayTV XP PV-M4700,PV-M4700(w/FM)
  - PixelView PlayTV DVR PV-M4600  鍖呰鍐呭:PixelView PlayTV pro, windvr & videoMail s/w

- 鏇村鍗?

  - PV-BT878P+rev.9B锛圥lay TV Pro, 鍙€夊甫 FM 甯?NICAM锛?
  - PV-BT878P+rev.2F
  - PV-BT878P Rev.1D (bt878, 浠呴噰闆?

  - XCapture PV-CX881P (cx23881)
  - PlayTV HD PV-CX881PL+, PV-CX881PL+(w/FM) (cx23881)

  - DTV3000 PV-DTV3000P+ DVB-S CI = Twinhan VP-1030
  - DTV2000 DVB-S = Twinhan VP-1020

- 瑙嗛浼氳:

  - PixelView Meeting PAK - (Model: PV-BT878P)
  - PixelView Meeting PAK Lite - (Model: PV-BT878P)
  - PixelView Meeting PAK plus - (Model: PV-BT878P+rev 4C/8D/10A)
  - PixelView Capture - (Model: PV-BT848P)
  - PixelView PlayTV USB pro
  - Model No. PV-NT1004+, PV-NT1004+ (w/FM) = NT1004 USB 瑙ｇ爜鑺墖 + SAA7113 瑙嗛瑙ｇ爜鑺墖

#### Dynalink


杩欎簺鏄?CPH 绯诲垪銆?

#### Phoebemicro


- TV Master    = CPH030 鎴?CPH060
- TV Master FM = CPH050

#### Genius/Kye


- Video Wonder/Genius Internet Video Kit = LR37 Rev.C
- Video Wonder Pro II锛?48 鎴?878锛? LR26

#### Tekram


- VideoCap C205锛圔t848锛?
- VideoCap C210锛坺r36120 +Philips锛?
- CaptureTV M200锛圛SA锛?
- CaptureTV M205锛圔t848锛?

#### Lucky Star


- Image World Conference TV = LR50 Rev. Q

#### Leadtek


- WinView 601锛圔t848锛?
- WinView 610锛圸oran锛?
- WinFast2000
- WinFast2000 XP

##### 瀵?Leadtek WinView 601 TV/FM 鐨勬敮鎸?


鏈妭鐨勪綔鑰? Jon Tombs <jon@gte.esi.us.es>

杩欏紶鍗″熀鏈笂鍜屾墍鏈夊叾浠栧崱涓€鏍凤紙Bt484A, Philips 璋冭皭鍣級锛屼富瑕佸尯鍒槸瀹冧滑鎶婂彲缂栫▼琛板噺鍣ㄦ帴鍒颁簡 3 涓?GPIO 绾夸笂锛屼互鎻愪緵涓€浜涢煶閲忔帶鍒躲€傚畠浠繕鍦ㄦ澘涓婅浜嗕竴涓孩澶栭仴鎺цВ鐮佸櫒锛岀瓑鏈夋椂闂存垜浼氬姞涓婂瀹冪殑鏀寔锛堝畠寰堢畝鍗曪紝姣忔鎸夐敭浜х敓涓€涓腑鏂紝閿爜鏀惧湪 GPIO 绔彛涓級銆?

鎴戣繕娌℃湁浠讳綍搴旂敤绋嬪簭鏉ユ祴璇曟敹闊虫満鏀寔銆傝皟璋愬櫒棰戠巼璁剧疆搴斿綋鑳界敤锛屼絾闊抽澶嶇敤鍣ㄥ彲鑳芥槸閿欑殑銆傚鏋滃畠涓嶅伐浣滐紝缁欐垜鍙戦偖浠躲€?

- 涓嶆劅璋?Leadtek锛屼粬浠嫆缁濆洖绛斾换浣曞叧浜庡叾纭欢鐨勯棶棰樸€傝繖涓┍鍔ㄦ槸閫氳繃鐩妫€鏌ュ崱鍐欏嚭鏉ョ殑銆傚鏋滀綘浣跨敤杩欎釜椹卞姩锛岀粰浠栦滑鍙戜竴灏佽颈楠傞偖浠讹紝鍛婅瘔浠栦滑闄ら潪浠栦滑鏀寔 Linux锛屽惁鍒欎綘涓嶄細鍐嶈喘涔颁粬浠殑纭欢銆?

- 鐣ュ井鎰熻阿鏅灄鏂】绉戞妧锛圥rinceton Technology Corp锛宧ttp://www.princeton.com.tw锛夛紝浠栦滑鍒堕€犱簡闊抽琛板噺鍣ㄣ€備粬浠綉绔欎笂鍏紑鍙敤鐨勬暟鎹墜鍐屼笉鍖呭惈鑺墖缂栫▼淇℃伅锛佷粬浠湇鍔″櫒涓婅棌鐫€瀹屾暣鐨勬暟鎹墜鍐岋紝浣嗗埆闂垜鏄€庝箞鎵惧埌鐨勩€?

瑕佷娇鐢ㄨ繖涓┍鍔紝鎴戜娇鐢ㄤ互涓嬮€夐」锛岃皟璋愬櫒鍜?pll 璁剧疆鍙兘鍦ㄤ綘鐨勫浗瀹朵笉鍚屻€備綘鍙互閫氳繃 modprobe 鍙傛暟寮哄埗璁剧疆銆?

```
    modprobe bttv  tuner=1 pll=28 radio=1 card=17

```
璁剧疆璋冭皭鍣ㄧ被鍨?1锛圥hilips PAL_I锛夛紝甯?28 MHz 鏅舵尟鐨?PLL锛屽惎鐢?FM 鏀堕煶鏈猴紝骞堕€夋嫨 bttv 鍗?ID 17锛圠eadtek WinView 601锛夈€?


#### KNC One


- TV-Station
- TV-Station SE锛?杞欢鍖咃級
- TV-Station pro锛?鐢佃绔嬩綋澹帮級
- TV-Station FM锛?鏀堕煶鏈猴級
- TV-Station RDS锛?RDS锛?
- TV Station SAT锛堟ā鎷熷崼鏄燂級
- TV-Station DVB-S


#### Provideo


- PV951 鎴?PV-951锛岀幇鍦ㄥ懡鍚嶄负 PV-951T
   锛堜篃浣滀负浠ヤ笅鍚嶇О鍑哄敭:
   Boeder TV-FM Video Capture Card,
   Titanmedia Supervision TV-2400,
   Provideo PV951 TF,
   3DeMon PV951,
   MediaForte TV-Vision PV951,
   Yoko PV951,
   Vivanco Tuner Card PCI Art.-Nr.: 68404
   )

- 鐩戞帶绯诲垪:

 - PV-141
 - PV-143
 - PV-147
 - PV-148锛堜粎閲囬泦锛?
 - PV-150
 - PV-151

- TV-FM 璋冭皭鍣ㄧ郴鍒?

 - PV-951TDV锛坱v tuner + 1394锛?
 - PV-951T/TF
 - PV-951PT/TF
 - PV-956T/TF 钖勫瀷
 - PV-911

#### Highscreen


鍨嬪彿:

- TV Karte = LR50 Rev.S
- TV-Boostar = Terratec Terra TV+ Version 1.0锛圔t848, tda9821锛?ceb105.pcb"

#### Zoltrix


鍨嬪彿:

- Face to Face Capture锛圔t848 浠呴噰闆嗭級锛圥CB "VP-2848"锛?
- Face To Face TV MAX锛圔t848锛夛紙PCB "VP-8482 Rev1.3"锛?
- Genie TV锛圔t878锛夛紙PCB "VP-8790 Rev 2.1"锛?
- Genie Wonder Pro

#### AVerMedia


- AVer FunTV Lite锛圛SA, AV3001 鑺墖缁勶級  "M101.C"
- AVerTV
- AVerTV Stereo
- AVerTV Studio锛堝甫 FM锛?
- AVerMedia TV98 甯?Remote
- AVerMedia TV/FM98 Stereo
- AVerMedia TVCAM98
- TVCapture锛圔t848锛?
- TVPhone锛圔t848锛?
- TVCapture98锛?"AVerMedia TV98" 鍦ㄧ編鍥斤級锛圔t878锛?
- TVPhone98锛圔t878, 甯?FM锛?

======== =========== =============== ======= ====== ======== =======================
PCB      PCI-ID      Model-Name      Eeprom  Tuner  Sound    Country
======== =========== =============== ======= ====== ======== =======================
M101.C   ISA !
M108-B      Bt848                     --     FR1236		 US   [#f2]_, [#f3]_
M1A8-A      Bt848    AVer TV-Phone           FM1216  --
M168-T   1461:0003   AVerTV Studio   48:17   FM1216 TDA9840T  D    [#f1]_ w/FM w/Remote
M168-U   1461:0004   TVCapture98     40:11   FI1216   --      D    w/Remote
M168II-B 1461:0003   Medion MD9592   48:16   FM1216 TDA9873H  D    w/FM
======== =========== =============== ======= ====== ======== =======================


- 缇庡浗绔欑偣瀵硅繖浜涘瀷鍙锋湁涓嶅悓鐨勯┍鍔紙鎴嚦 2002 骞?09 鏈堬級:

  - EZ Capture/InterCam PCI锛圔T-848 鑺墖锛?
  - EZ Capture/InterCam PCI锛圔T-878 鑺墖锛?
  - TV-Phone锛圔T-848 鑺墖锛?
  - TV98锛圔T-848 鑺墖锛?
  - TV98 With Remote锛圔T-848 鑺墖锛?
  - TV98锛圔T-878 鑺墖锛?
  - TV98 With Remote锛圔T-878锛?
  - TV/FM98锛圔T-878 鑺墖锛?
  - AVerTV
  - AverTV Stereo
  - AVerTV Studio

DE 瀵硅繖浜涘瀷鍙锋湁鍚勭椹卞姩锛堟埅鑷?2002 骞?09 鏈堬級:

  - TVPhone锛?48锛夊甫 Philips 璋冭皭鍣?FR12X6锛堝甫 FM 鏀堕煶鏈猴級
  - TVPhone锛?48锛夊甫 Philips 璋冭皭鍣?FM12X6锛堝甫 FM 鏀堕煶鏈猴級
  - TVCapture锛?48锛夊甫 Philips 璋冭皭鍣?FI12X6
  - TVCapture锛?48锛夐潪 Philips 璋冭皭鍣?
  - TVCapture98锛圔t878锛?
  - TVPhone98锛圔t878锛?
  - AVerTV 鍜?TVCapture98 甯?VCR锛圔t 878锛?
  - AVerTVStudio 鍜?TVPhone98 甯?VCR锛圔t878锛?
  - AVerTV GO Series锛堟棤 SVideo 杈撳叆锛?
  - AVerTV98锛圔T-878 鑺墖锛?
  - AVerTV98 甯?Fernbedienung锛堥仴鎺э級锛圔T-878 鑺墖锛?
  - AVerTV/FM98锛圔T-878 鑺墖锛?

  - VDOmate锛坵ww.averm.com.cn锛? M168U 锛?

#### Aimslab


鍨嬪彿:

- Video Highway 鎴?"Video Highway TR200"锛圛SA锛?
- Video Highway Xtreme锛坅ka "VHX"锛夛紙Bt848, FM w/ TEA5757锛?

#### IXMicro锛堝墠: IMS=Integrated Micro Solutions锛?


鍨嬪彿:

- IXTV BT848锛?TurboTV锛?
- IXTV BT878
- IMS TurboTV锛圔t848锛?

#### Lifetec/Medion/Tevion/Aldi


鍨嬪彿:

- LT9306/MD9306 = CPH061
- LT9415/MD9415 = LR90 Rev.F 鎴?Rev.G
- MD9592 = Avermedia TVphone98锛圥CI_ID=1461:0003锛? PCB-Rev=M168II-B锛堝甫 TDA9873H锛?
- MD9717 = KNC One锛圧ev D4, saa7134, FM1216 MK2 璋冭皭鍣級
- MD5044 = KNC One锛圧ev D4, saa7134, FM1216ME MK3 璋冭皭鍣級

#### Modular Technologies锛坵ww.modulartech.com锛塙K


鍨嬪彿:

- MM100 PCTV锛圔t848锛?
- MM201 PCTV锛圔t878, Bt832锛夊甫 Quartzsight 鎽勫儚澶?
- MM202 PCTV锛圔t878, Bt832, tda9874锛?
- MM205 PCTV锛圔t878锛?
- MM210 PCTV锛圔t878锛夛紙Galaxy TV, Galaxymedia ?锛?

#### Terratec


鍨嬪彿:

- Terra TV+ Version 1.0锛圔t848锛? PCB 涓婂嵃鏈?"ceb105.PCB", TDA9821
- Terra TV+ Version 1.1锛圔t878锛? PCB 涓婂嵃鏈?"LR74 Rev.E", TDA9821
- Terra TValueRadio,             PCB 涓婂嵃鏈?"LR102 Rev.C"
- Terra TV/Radio+ Version 1.0,   PCB 涓婂嵃鏈?"80-CP2830100-0" TTTV3,
  PCB 鑳岄潰鏈?"CPH010-E83", SAA6588T, TDA9873H
- Terra TValue Version BT878,    PCB 涓婂嵃鏈?"80-CP2830110-0 TTTV4",
  鑳岄潰鏈?"CPH011-D83"
- Terra TValue Version 1.0       "ceb105.PCB"锛堜笌 Terra TV+ Version 1.0 瀹屽叏鐩稿悓锛?
- Terra TValue New Revision	  "LR102 Rec.C"
- Terra Active Radio Upgrade锛坱ea5757h, saa6588t锛?

- LR74 鏄?ceb105 鐨勪竴涓緝鏂?PCB 淇鐗堬紙涓よ€呴兘鍚敤浜?Active Radio Upgrade 鐨勮繛鎺ュ櫒锛?

- Cinergy 400锛坰aa7134锛? PCB 涓婂嵃鏈?"E877 11(S)", "PM820092D"
- Cinergy 600锛坰aa7134锛?

#### Technisat


鍨嬪彿:

- Discos ADR PC-Karte ISA锛堟棤 TV锛侊級
- Discos ADR PC-Karte PCI锛堝ぇ姒傛棤 TV锛燂級
- Techni-PC-Sat锛圫at. analog锛?
  Rev 1.2锛坺r36120, vpx3220, stv0030, saa5246, BSJE3-494A锛?
- Mediafocus I锛坺r36120/zr36125, drp3510, Sat. analog + ADR Radio锛?
- Mediafocus II锛坰aa7146, Sat. analog锛?
- SatADR Rev 2.1锛坰aa7146a, saa7113h, stv0056a, msp3400c, drp3510a, BSKE3-307A锛?
- SkyStar 1 DVB  (AV7110) = Technotrend Premium
- SkyStar 2 DVB  (B2C2) (=Sky2PC)

#### Siemens


Multimedia eXtension Board锛圡XB锛夛紙SAA7146, SAA7111锛?

#### Powercolor


鍨嬪彿:

- MTV878
       鍖呰甯︽湁涓嶅悓鍐呭:

           a) pcb "MTV878"锛圕ARD=75锛?
           b) Pixelview Rev. 4\_

- MTV878R 甯?Remote Control
- MTV878F 甯?Remote Control 甯?FM 鏀堕煶鏈?

#### Pinnacle


PCTV 鍨嬪彿:

- Mirovideo PCTV锛圔t848锛?
- Mirovideo PCTV SE锛圔t848锛?
- Mirovideo PCTV Pro锛圔t848 + 鐢ㄤ簬 TV 绔嬩綋澹板拰 FM 鐨勫瓙鏉匡級
- Studio PCTV Rave锛圔t848 Version = Mirovideo PCTV锛?
- Studio PCTV Rave锛圔t878 鍖呰锛屾棤绾㈠锛?
- Studio PCTV      (Bt878)
- Studio PCTV Pro  (Bt878 stereo 甯?FM)
- Pinnacle PCTV    (Bt878, MT2032)
- Pinnacle PCTV Pro (Bt878, MT2032)
- Pinncale PCTV Sat (bt878a, HM1821/1221) ["Conexant CX24110 with CX24108 tuner, aka HM1221/HM1811"]
- Pinnacle PCTV Sat XE

M(J)PEG 閲囬泦涓庡洖鏀惧瀷鍙?

- DC1+锛圛SA锛?
- DC10  (zr36057,     zr36060,      saa7110, adv7176)
- DC10+ (zr36067,     zr36060,      saa7110, adv7176)
- DC20  (ql16x24b,zr36050, zr36016, saa7110, saa7187 ...)
- DC30  (zr36057, zr36050, zr36016, vpx3220, adv7176, ad1843, tea6415, miro FST97A1)
- DC30+ (zr36067, zr36050, zr36016, vpx3220, adv7176)
- DC50  (zr36067, zr36050, zr36016, saa7112, adv7176 (2 pcs.锛? ad1843, miro FST97A1, Lattice ??锛?

#### Lenco


鍨嬪彿:

- MXR-9565 (=Technisat Mediafocus锛?
- MXR-9571锛圔t848锛?=CPH031锛?
- MXR-9575
- MXR-9577锛圔t878锛?=Prolink 878TV Rev.3x)
- MXTV-9578CP锛圔t878锛?= Prolink PV-BT878P+4E)

#### Iomega


Buz锛坺r36067, zr36060, saa7111, saa7185锛?

#### LML

   LML33锛坺r36067, zr36060, bt819, bt856锛?

#### Grandtec


鍨嬪彿:

- Grand Video Capture锛圔t848锛?
- Multi Capture Card  (Bt878)

#### Koutech


鍨嬪彿:

- KW-606锛圔t848锛?
- KW-607锛圔t848 浠呴噰闆嗭級
- KW-606RSF
- KW-607A锛堜粎閲囬泦锛?
- KW-608锛圸oran 浠呴噰闆嗭級

#### IODATA锛坖p锛?


鍨嬪彿:

- GV-BCTV/PCI
- GV-BCTV2/PCI
- GV-BCTV3/PCI
- GV-BCTV4/PCI
- GV-VCP/PCI锛堜粎閲囬泦锛?
- GV-VCP2/PCI锛堜粎閲囬泦锛?

#### Canopus锛坖p锛?


WinDVR	= Kworld "KW-TVL878RF"

#### www.sigmacom.co.kr


Sigma Cyber TV II

#### www.sasem.co.kr


Litte OnAir TV

#### hama


TV/Radio-Tuner Card, PCI锛圡odel 44677锛? CPH051

#### Sigma Designs


Hollywood plus锛坋m8300, em9010, adv7175锛? (PCB "M340-10") MPEG DVD 瑙ｇ爜鍣?

#### Formac


鍨嬪彿:

- iProTV锛堢敤浜?iMac Mezzanine 妲界殑鍗? Bt848+SCSI锛?
- ProTV锛圔t848锛?
- ProTV II = ProTV Stereo锛圔t878锛塠"stereo" 鎸?FM 绔嬩綋澹? tv 浠嶆槸鍗曞０閬揮

#### ATI


鍨嬪彿:

- TV-Wonder
- TV-Wonder VE

#### Diamond Multimedia


DTV2000锛圔t848, tda9875锛?

#### Aopen


- VA1000 Plus锛堝甫 Stereo锛?
- VA1000 Lite
- VA1000 (=LR90)

#### Intel


鍨嬪彿:

- Smart Video Recorder锛圛SA 鍏ㄩ暱鑰咃級
- Smart Video Recorder pro锛圛SA 鍗婇暱鑰咃級
- Smart Video Recorder III锛圔t848锛?

#### STB


鍨嬪彿:

- STB Gateway 6000704锛坆t878锛?
- STB Gateway 6000699锛坆t848锛?
- STB Gateway 6000402锛坆t848锛?
- STB TV130 PCI

#### Videologic


鍨嬪彿:

- Captivator Pro/TV锛圛SA锛燂級
- Captivator PCI/VC锛圔t848 涓庢憚鍍忓ご鎹嗙粦锛夛紙浠呴噰闆嗭級

#### Technotrend


鍨嬪彿:

- TT-SAT PCI锛圥CB "Sat-PCI Rev.:1.3.1"; zr36125, vpx3225d, stc0056a, Tuner:BSKE6-155A
- TT-DVB-Sat
   - 淇鐗?1.1, 1.3, 1.5, 1.6 鍜?2.1
   - 杩欏紶鍗′綔涓?OEM 鍑哄敭鑷?

 - Siemens DVB-s Card
 - Hauppauge WinTV DVB-S
 - Technisat SkyStar 1 DVB
 - Galaxis DVB Sat

   - 濡備粖杩欏紶鍗＄О涓?TT-PCline Premium Family
   - TT-Budget锛坰aa7146, bsru6-701a锛?
     杩欏紶鍗′綔涓?OEM 鍑哄敭鑷?

 - Hauppauge WinTV Nova
 - Satelco Standard PCI锛圖VB-S锛?
   - TT-DVB-C PCI

#### Teles


 DVB-s锛圧ev. 2.2, BSRV2-301A, 浠呮暟鎹紵锛?

#### Remote Vision


MX RV605锛圔t848 浠呴噰闆嗭級

#### Boeder


鍨嬪彿:

- PC ChatCam锛圡odel 68252锛夛紙Bt848 浠呴噰闆嗭級
- Tv/Fm Capture Card  (Model 68404) = PV951

#### Media-Surfer  (esc-kathrein.de)


鍨嬪彿:

- Sat-Surfer锛圛SA锛?
- Sat-Surfer PCI = Techni-PC-Sat
- Cable-Surfer 1
- Cable-Surfer 2
- Cable-Surfer PCI锛坺r36120锛?
- Audio-Surfer锛圛SA Radio card锛?

#### Jetway锛坵ww.jetway.com.tw锛?


鍨嬪彿:

- JW-TV 878M
- JW-TV 878  = KWorld KW-TV878RF

#### Galaxis


鍨嬪彿:

- Galaxis DVB Card S CI
- Galaxis DVB Card C CI
- Galaxis DVB Card S
- Galaxis DVB Card C
- Galaxis plug.in S [neuer Name: Galaxis DVB Card S CI

#### Hauppauge


鍨嬪彿:

- 璁稿璁稿 WinTV 鍨嬪彿鈥︹€?
- WinTV DVBs = Technotrend Premium 1.3
- WinTV NOVA = Technotrend Budget 1.1 "S-DVB DATA"
- WinTV NOVA-CI "SDVBACI"
- WinTV Nova USB (=Technotrend USB 1.0)
- WinTV-Nexus-s (=Technotrend Premium 2.1 鎴?2.2)
- WinTV PVR
- WinTV PVR 250
- WinTV PVR 450

缇庡浗鍨嬪彿

-990 WinTV-PVR-350 (249USD) (iTVC15 chipset + radio)
-980 WinTV-PVR-250 (149USD) (iTVC15 chipset)
-880 WinTV-PVR-PCI (199USD) (KFIR chipset + bt878)
-881 WinTV-PVR-USB
-190 WinTV-GO
-191 WinTV-GO-FM
-404 WinTV
-401 WinTV-radio
-495 WinTV-Theater
-602 WinTV-USB
-621 WinTV-USB-FM
-600 USB-Live
-698 WinTV-HD
-697 WinTV-D
-564 WinTV-Nexus-S

Deutsche Modelle锛堝痉鍥藉瀷鍙凤級:

-603 WinTV GO
-719 WinTV Primio-FM
-718 WinTV PCI-FM
-497 WinTV Theater
-569 WinTV USB
-568 WinTV USB-FM
-882 WinTV PVR
-981 WinTV PVR 250
-891 WinTV-PVR-USB
-541 WinTV Nova
-488 WinTV Nova-Ci
-564 WinTV-Nexus-s
-727 WinTV-DVB-c
-545 Common Interface
-898 WinTV-Nova-USB

UK 鍨嬪彿:

-607 WinTV Go
-693,793 WinTV Primio FM
-647,747 WinTV PCI FM
-498 WinTV Theater
-883 WinTV PVR
-893 WinTV PVR USB  (Duplicate entry)
-566 WinTV USB (UK)
-573 WinTV USB FM
-429 Impact VCB (bt848)
-600 USB Live (Video-In 1x Comp, 1xSVHS)
-542 WinTV Nova
-717 WinTV DVB-S
-909 Nova-t PCI
-893 Nova-t USB   (Duplicate entry)
-802 MyTV
-804 MyView
-809 MyVideo
-872 MyTV2Go FM
-546 WinTV Nova-S CI
-543 WinTV Nova
-907 Nova-S USB
-908 Nova-T USB
-717 WinTV Nexus-S
-157 DEC3000-s Standalone + USB

Spain锛堣タ鐝墮锛?

-685 WinTV-Go
-690 WinTV-PrimioFM
-416 WinTV-PCI Nicam Estereo
-677 WinTV-PCI-FM
-699 WinTV-Theater
-683 WinTV-USB
-678 WinTV-USB-FM
-983 WinTV-PVR-250
-883 WinTV-PVR-PCI
-993 WinTV-PVR-350
-893 WinTV-PVR-USB
-728 WinTV-DVB-C PCI
-832 MyTV2Go
-869 MyTV2Go-FM
-805 MyVideo (USB)


#### Matrix-Vision


鍨嬪彿:

- MATRIX-Vision MV-Delta
- MATRIX-Vision MV-Delta 2
- MVsigma-SLC锛圔t848锛?

#### Conceptronic锛?net锛?


鍨嬪彿:

- TVCON FM,  TV card w/ FM = CPH05x
- TVCON = CPH06x

#### BestData


鍨嬪彿:

- HCC100 = VCC100rev1 + camera
- VCC100 rev1锛坆t848锛?
- VCC100 rev2锛坆t878锛?

#### Gallant  (www.gallantcom.com) www.minton.com.tw


鍨嬪彿:

- Intervision IV-510锛堜粎閲囬泦 bt8x8锛?
- Intervision IV-550锛坆t8x8锛?
- Intervision IV-100锛坺oran锛?
- Intervision IV-1000锛坆t8x8锛?

#### Asonic锛坵ww.asonic.com.cn锛夛紙缃戠珯宸插叧闂級


SkyEye tv 878

#### Hoontech


878TV/FM

#### Teppro锛坵ww.itcteppro.com.tw锛?


鍨嬪彿:

- ITC PCITV锛圕ard Ver 1.0锛?Teppro TV1/TVFM1 Card"
- ITC PCITV锛圕ard Ver 2.0锛?
- ITC PCITV锛圕ard Ver 3.0锛? "PV-BT878P+ (REV.9D)"
- ITC PCITV锛圕ard Ver 4.0锛?
- TEPPRO IV-550锛團or BT848 Main Chip锛?
- ITC DSTTV锛坆t878, satellite锛?
- ITC VideoMaker锛坰aa7146, StreamMachine sm2110, tvtuner锛?PV-SM2210P+ (REV:1C)"

#### Kworld锛坵ww.kworld.com.tw锛?


PC TV Station:

- KWORLD KW-TV878R  TV锛堟棤鏀堕煶鏈猴級
- KWORLD KW-TV878RF TV锛堝甫鏀堕煶鏈猴級
- KWORLD KW-TVL878RF锛堣杽鍨嬶級
- KWORLD KW-TV713XRF锛坰aa7134锛?


 MPEG TV Station锛堜笌涓婅堪鐩稿悓鐨勫崱锛屽姞涓?WinDVR 杞欢 MPEG 缂?瑙ｇ爜鍣級

- KWORLD KW-TV878R -Pro   TV锛堟棤 Radio锛?
- KWORLD KW-TV878RF-Pro   TV锛堝甫 Radio锛?
- KWORLD KW-TV878R -Ultra TV锛堟棤 Radio锛?
- KWORLD KW-TV878RF-Ultra TV锛堝甫 Radio锛?

#### JTT/ Justy Corp.(http://www.jtt.ne.jp/)


JTT-02锛圝TT TV锛?TV watchmate pro"锛坆t848锛?

#### ADS www.adstech.com


鍨嬪彿:

- Channel Surfer TV锛?CHX-950 锛?
- Channel Surfer TV+FM锛?CHX-960FM 锛?

#### AVEC www.prochips.com


AVEC Intercapture锛坆t848, tea6320锛?

#### NoBrand


TV Excel = "PV-BT878P+ 8E" 鎴?"878TV Rev.3\_" 鐨勬境澶у埄浜氬悕

#### Mach www.machspeed.com


Mach TV 878

#### Eline www.eline-net.com/


鍨嬪彿:

- Eline Vision TVMaster / TVMaster FM (ELV-TVM/ ELV-TVM-FM) = LR26  (bt878)
- Eline Vision TVMaster-2000 (ELV-TVM-2000, ELV-TVM-2000-FM)= LR138 (saa713x)

#### Spirit


- Spirit TV Tuner/Video Capture Card锛坆t848锛?

#### Boser www.boser.com.tw


鍨嬪彿:

- HS-878 Mini PCI Capture Add-on Card
- HS-879 Mini PCI 3D Audio and Capture Add-on Card (w/ ES1938 Solo-1)

#### Satelco www.citycom-gmbh.de, www.satelco.de


鍨嬪彿:

- TV-FM =KNC1 saa7134
- Standard PCI锛圖VB-S锛? Technotrend Budget
- Standard PCI锛圖VB-S锛夊甫 CI
- Satelco Highend PCI锛圖VB-S锛? Technotrend Premium


#### Sensoray www.sensoray.com


鍨嬪彿:

- Sensoray 311锛圥C/104 鎬荤嚎锛?
- Sensoray 611锛圥CI锛?

#### CEI锛圕hartered Electronics Industries Pte Ltd [CEI] [FCC ID HBY]锛?


鍨嬪彿:

- TV Tuner  -  HBY-33A-RAFFLES  Brooktree Bt848KPF + Philips
- TV Tuner MG9910  -  HBY33A-TVO  CEI + Philips SAA7110 + OKI M548262 + ST STV8438CV
- Primetime TV锛圛SA锛?

  - 琚柊鍔犲潯绉戞妧锛圫ingapore Technologies锛夋敹璐?
  - 鐜板湪浣滀负 Chartered Semiconductor Manufacturing 杩愯惀
  - 鏄惧崱鍒堕€犲晢鍒椾负:

    - Cogent Electronics Industries [CEI]

#### AITech


鍨嬪彿:

- Wavewatcher TV锛圛SA锛?
- AITech WaveWatcher TV-PCI = 鍙互鏄?LR26锛圔t848锛夋垨 LR50锛圔T878锛?
- WaveWatcher TVR-202 TV/FM Radio Card锛圛SA锛?

#### MAXRON


Maxron MaxTV/FM Radio锛圞W-TV878-FNT锛? Kworld 鎴?JW-TV878-FBK

#### www.ids-imaging.de


鍨嬪彿:

- Falcon Series锛堜粎閲囬泦锛?

In USA: http://www.theimagingsource.com/
- DFG/LC1

#### www.sknet-web.co.jp


SKnet Monster TV锛坰aa7134锛?

#### A-Max www.amaxhk.com锛圕olormax, Amax, Napa锛?


APAC Viewcomp 878

#### Cybertainment


鍨嬪彿:

- CyberMail AV Video Email Kit w/ PCI Capture Card锛堜粎閲囬泦锛?
- CyberMail Xtreme

These are Flyvideo锛堣繖浜涙槸 Flyvideo锛?

#### VCR锛坔ttp://www.vcrinc.com/锛?


Video Catcher 16

#### Twinhan


鍨嬪彿:

- DST Card/DST-IP锛坆t878, twinhan asic锛塚P-1020
  - 浣滀负浠ヤ笅鍚嶇О鍑哄敭:

    - KWorld DVBS Satellite TV-Card
    - Powercolor DSTV Satellite Tuner Card
    - Prolink Pixelview DTV2000
    - Provideo PV-911 Digital Satellite TV Tuner Card With Common Interface ?

- DST-CI Card锛圖VB Satellite锛塚P-1030
- DCT Card锛圖VB cable锛?

#### MSI


鍨嬪彿:

- MSI TV@nywhere Tuner Card锛圡S-8876锛夛紙CX23881/883锛変笉鍏煎 Bt878銆?
- MS-8401 DVB-S

#### Focus www.focusinfo.com


InVideo PCI锛坆t878锛?

#### Sdisilk www.sdisilk.com/


鍨嬪彿:

- SDI Silk 100
- SDI Silk 200 SDI Input Card

#### www.euresys.com


PICOLO series锛圥ICOLO 绯诲垪锛?

#### PMC/Pace


www.pacecom.co.uk 缃戠珯宸插叧闂?

#### Mercury www.kobian.com锛圲K and FR锛?


鍨嬪彿:

- LR50
- LR138RBG-Rx  == LR138

#### TEC sound


TV-Mate = Zoltrix VP-8482

铏界劧閫氳繃鏈夋妧宸х殑璋锋瓕鎼滅储鎵惧埌浜? www.techmakers.com

锛堝寘瑁呭拰鎵嬪唽娌℃湁浠讳綍鍏朵粬鍒堕€犲晢淇℃伅锛塗ecSound

#### Lorenzen www.lorenzen.de


SL DVB-S PCI = Technotrend Budget PCI锛坰u1278 鎴?bsru 鐗堟湰锛?

#### Origo锛?uk锛墂ww.origo2000.com


PC TV Card = LR50

#### I/O Magic www.iomagic.com


PC PVR - Desktop TV Personal Video Recorder DR-PCTV100 = Pinnacle ROB2D-51009464 4.0 + Cyberlink PowerVCR II

#### Arowana


TV-Karte / Poso Power TV锛?锛? Zoltrix VP-8482锛堬紵锛?

#### iTVC15 鏉?


kuroutoshikou.com ITVC15

yuan.com MPG160 PCI TV锛圛nternal PCI MPEG2 encoder card plus TV-tuner锛?

#### Asus www.asuscom.com


鍨嬪彿:

- Asus TV Tuner Card 880 NTSC锛堣杽鍨? cx23880锛?
- Asus TV锛坰aa7134锛?

#### Hoontech


http://www.hoontech.de/

- HART Vision 848锛圚-ART Vision 848锛?
- HART Vision 878锛圚-Art Vision 878锛?



### bttv 璁惧浣跨敤鐨勮姱鐗?


- 鎵€鏈夋澘:

  - Brooktree Bt848/848A/849/878/879: 瑙嗛閲囬泦鑺墖

- 鏉跨壒瀹?

  - Miro PCTV:

    - Philips 鎴?Temic 璋冭皭鍣?

  - Hauppauge Win/TV pci锛坴ersion 405锛?

    - Microchip 24LC02B 鎴?Philips 8582E2Y:

       - 256 瀛楄妭 EEPROM 甯﹂厤缃俊鎭?
       - I2C 0xa0-0xa1,锛?4LC02B 涔熷搷搴?0xa2-0xaf锛?

    - Philips SAA5246AGP/E: 鍥炬枃鐢佃瑙ｇ爜鍣ㄨ姱鐗? I2C 0x22-0x23

    - TDA9800: 澹伴煶瑙ｇ爜鍣?

    - Winbond W24257AS-35: 32Kx8 CMOS 闈欐€?RAM锛堝浘鏂囩數瑙嗙紦鍐插唴瀛橈級

    - 14052B: 鐢ㄤ簬閫夋嫨澹伴煶婧愮殑妯℃嫙寮€鍏?

- PAL:

  - TDA5737: VHF銆佽秴楂橀甯﹀拰 UHF 娣烽鍣?鎸崱鍣紝鐢ㄤ簬 TV 鍜?VCR 3 棰戞璋冭皭鍣?
  - TSA5522: 1.4 GHz I2C 鎬荤嚎鎺у埗鍚堟垚鍣? I2C 0xc2-0xc3

- NTSC:

  - TDA5731: VHF銆佽秴楂橀甯﹀拰 UHF 娣烽鍣?鎸崱鍣紝鐢ㄤ簬 TV 鍜?VCR 3 棰戞璋冭皭鍣?
  - TSA5518: Philips 绔欑偣涓婃病鏈夋暟鎹墜鍐屽彲鐢?

- STB TV pci:

  - ???
  - 濡傛灉浣犳兂瑕佸 STB 鍗℃洿濂界殑鏀寔锛岀粰鎴戝彂淇℃伅锛?
    鐪嬬湅鏉垮瓙锛佷笂闈㈡湁鍝簺鑺墖锛?



### 瑙勬牸锛圫pecs锛?


Philips		http://www.Semiconductors.COM/pip/

Conexant	http://www.conexant.com/

Micronas	http://www.micronas.com/en/home/index.html

### 鑷磋阿


闈炲父鎰熻阿:

- Markus Schroeder <schroedm@uni-duesseldorf.de>锛屾彁渚涗簡鍏充簬 Bt848 鍜岃皟璋愬櫒缂栫▼鐨勪俊鎭互鍙婁粬鐨勬帶鍒剁▼搴?xtvc銆?

- Martin Buck <martin-2.buck@student.uni-ulm.de>锛屾彁渚涗簡浠栧嚭鑹茬殑鍥炬枃鐢佃鍖呫€?

- Gerd Hoffmann锛屾彁渚涗簡 MSP3400 鏀寔鍜屾ā鍧楀寲鐨?I2C銆佽皟璋愬櫒鈥︹€︽敮鎸併€?


- MATRIX Vision锛屽厤璐圭粰浜嗘垜浠?2 寮犲崱锛屼娇鍗曟櫠鎸搷浣滅殑鏀寔鎴愪负鍙兘銆?

- MIRO锛屾彁渚涗簡涓€寮犲厤璐圭殑 PCTV 鍗′互鍙婂叧浜庝粬浠崱涓婄粍浠剁殑璇︾粏淇℃伅銆傦紙渚嬪璋冭皭鍣ㄧ被鍨嬫槸濡備綍鎺㈡祴鐨勶級娌℃湁浠栦滑鐨勫崱锛屾垜鏃犳硶璋冭瘯 NTSC 妯″紡銆?

- Hauppauge锛屽憡鐭ュ浣曢€夋嫨澹伴煶杈撳叆锛屼互鍙婁粬浠湪鏀堕煶鏈哄崱涓婁娇鐢ㄥ拰灏嗕細浣跨敤鍝簺缁勪欢銆備篃闈炲父鎰熻阿缁欐垜浼犵湡 FM1216 鏁版嵁鎵嬪唽銆?

### 璐＄尞鑰?


Michael Chu <mmchu@pobox.com>
  AverMedia 淇浠ュ強鏇寸伒娲荤殑鍗¤瘑鍒?

Alan Cox <alan@lxorguk.ukuu.org.uk>
  Video4Linux 鎺ュ彛浠ュ強 2.1.x 鍐呮牳閫傞厤

Chris Kleitsch
  Hardware I2C

Gerd Hoffmann
  Radio card锛圛TT 澹伴煶澶勭悊鍣級

bigfoot <bigfoot@net-way.net>

Ragnar Hojland Espinosa <ragnar@macula.net>
  ConferenceTV card


- 杩樻湁鏇村浜猴紙濡傛灉浣犱笉鍦ㄨ繖涓垪琛ㄤ腑浣嗗笇鏈涜鎻愬強锛岃缁欐垜鍙戦偖浠讹級
