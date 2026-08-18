## Intel Assabet锛圫A-1110 璇勪及锛夋澘


璇峰弬瑙侊細
http://developer.intel.com

浠ュ強鏉ヨ嚜 John G Dorsey <jd5q@andrew.cmu.edu> 鐨勪竴浜涜鏄庯細
http://www.cs.cmu.edu/~wearable/software/assabet.html


### 鏋勫缓鍐呮牳


```
	make assabet_defconfig
	make oldconfig
	make zImage
```
鐢熸垚鐨勫唴鏍告槧鍍忓簲浣嶄簬 linux/arch/arm/boot/zImage銆?

### 瀹夎寮曞鍔犺浇绋嬪簭


鏈夊嚑涓兘澶熶负 Assabet 寮曞 Linux 鐨勫紩瀵煎姞杞界▼搴忓彲鐢細

BLOB (http://www.lartmaker.nl/lartware/blob/)

   BLOB 鏄?LART 椤圭洰涓娇鐢ㄧ殑寮曞鍔犺浇绋嬪簭銆備竴浜涜础鐚殑琛ヤ竵琚悎骞惰繘 BLOB 浠ユ坊鍔犲 Assabet 鐨勬敮鎸併€?
Compaq 鐨?Bootldr + John Dorsey 鐢ㄤ簬 Assabet 鏀寔鐨勮ˉ涓?(http://www.handhelds.org/Compaq/bootldr.html)
(http://www.wearablegroup.org/software/bootldr/)

   Bootldr 鏄?Compaq 涓?iPAQ Pocket PC 寮€鍙戠殑寮曞鍔犺浇绋嬪簭銆?   John Dorsey 鍒朵綔浜嗛檮鍔犺ˉ涓佷互娣诲姞瀵?Assabet 鍜?JFFS 鏂囦欢绯荤粺鐨勬敮鎸併€?
RedBoot (http://sources.redhat.com/redboot/)

   RedBoot 鏄?Red Hat 鍩轰簬 eCos RTOS 纭欢鎶借薄灞傚紑鍙戠殑寮曞鍔犺浇绋嬪簭銆?   瀹冩敮鎸?Assabet 浠ュ強璁稿鍏朵粬纭欢骞冲彴銆?
RedBoot 鐩墠鏄帹鑽愮殑閫夋嫨锛屽洜涓哄畠鏄敮涓€鍏锋湁缃戠粶鏀寔鐨勶紝骞朵笖鏄淮鎶ゆ渶娲昏穬鐨勩€?
涓嬮潰灞曠ず浜嗗浣曚娇鐢?RedBoot 寮曞 Linux 鐨勭畝瑕佺ず渚嬨€備絾棣栧厛
浣犻渶瑕佸皢 RedBoot 瀹夎鍒颁綘鐨勯棯瀛橈紙flash锛変腑銆備竴涓凡鐭ュ彲鐢ㄧ殑
棰勭紪璇?RedBoot 浜岃繘鍒舵枃浠跺彲浠庝互涓嬩綅缃幏鍙栵細

- ftp://ftp.netwinder.org/users/n/nico/
- ftp://ftp.arm.linux.org.uk/pub/linux/arm/people/nico/
- ftp://ftp.handhelds.org/pub/linux/arm/sa-1100-patches/

鏌ユ壘 redboot-assabet*.tgz銆備竴浜涘畨瑁呬俊鎭湪 redboot-assabet*.txt 涓彁渚涖€?

### 鍒濆 RedBoot 閰嶇疆


姝ゅ浣跨敤鐨勫懡浠ゅ湪 RedBoot 鐢ㄦ埛鎸囧崡涓湁瑙ｉ噴锛岃鎸囧崡鍙湪绾胯幏鍙栵細
http://sources.redhat.com/ecos/docs.html銆傝鍙傝€冨畠浜嗚В璇存槑銆?
濡傛灉浣犳湁涓€鍧?CF 缃戝崱锛堟垜鐨?Assabet 濂椾欢鍖呭惈涓€鍧楁潵鑷?Socket Communications Inc. 鐨?CF+ LP-E锛夛紝
浣犲簲璇ュ己鐑堣€冭檻浣跨敤瀹冭繘琛?TFTP 鏂囦欢浼犺緭銆備綘蹇呴』鍦?RedBoot 杩愯鍓嶆彃鍏ュ畠锛屽洜涓哄畠鏃犳硶鍔ㄦ€佹娴嬪埌瀹冦€?
```
	fis init -f
```
瑕佸垵濮嬪寲闈炴槗澶辨€ц缃紝渚嬪浣犳槸鍚︽兂瑕佷娇鐢?BOOTP 鎴?
```
	fconfig -i
```


### 灏嗗唴鏍告槧鍍忓啓鍏ラ棯瀛?

棣栧厛锛屽唴鏍告槧鍍忓繀椤昏鍔犺浇鍒?RAM 涓€傚鏋滀綘鏈?zImage 鏂囦欢

```
	load zImage -r -b 0x100000
```
```
	load -m ymodem -r -b 0x100000
```
```
	fis create "Linux kernel" -b 0x100000 -l 0xc0000
```


### 寮曞鍐呮牳


鍐呮牳浠嶇劧闇€瑕佷竴涓枃浠剁郴缁熸墠鑳藉紩瀵笺€傚彲浠ュ姞杞戒竴涓?ramdisk 鏄犲儚

```
	load ramdisk_image.gz -r -b 0x800000
```
鍚屾牱锛屽彲浠ョ敤 Y-Modem 涓婁紶浠ｆ浛 TFTP锛屽彧闇€灏嗘枃浠跺悕鏇挎崲涓?'-y ymodem'銆?
```
	fis load "Linux kernel"
```
```
	exec -b 0x100000 -l 0xc0000
```
ramdisk 鏄犲儚涔熷彲浠ュ瓨鍌ㄥ埌闂瓨涓紝浣嗗涓嬫枃鎵€杩帮紝鏈夋洿濂界殑鐢ㄤ簬鐗囦笂锛坥n-flash锛夋枃浠剁郴缁熺殑鏂规銆?

### 浣跨敤 JFFS2


浣跨敤 JFFS2锛堢浜屾棩蹇楅棯瀛樻枃浠剁郴缁燂紝the Second Journalling Flash File System锛夊彲鑳芥槸灏嗗彲鍐欐枃浠剁郴缁熷瓨鍏ラ棯瀛樻渶鏂逛究鐨勬柟寮忋€?JFFS2 涓庤礋璐ｅ簳灞傞棯瀛樼鐞嗙殑 MTD 灞傞厤鍚堜娇鐢ㄣ€傚叧浜?Linux MTD 鐨勬洿澶氫俊鎭彲鍦ㄧ嚎鑾峰彇锛?http://www.linux-mtd.infradead.org/銆傚悓涓€绔欑偣涔熸彁渚涗簡甯︽湁涓€浜涘叧浜庡垱寤?JFFS/JFFS2 鏄犲儚淇℃伅鐨?JFFS howto銆?
渚嬪锛屼竴涓ず渚?JFFS2 鏄犲儚鍙粠涓嬮潰鎻愬埌鐨勩€佷负棰勭紪璇?RedBoot 鏄犲儚鎻愪緵鐨勫悓涓€ FTP 绔欑偣鑾峰彇銆?
```
	load sample_img.jffs2 -r -b 0x100000
```
```
	RedBoot> load sample_img.jffs2 -r -b 0x100000
	Raw file loaded 0x00100000-0x00377424
```
```
	fis free
```
```
	RedBoot> fis free
	  0x500E0000 .. 0x503C0000
```
涓婅堪鍊煎彲鑳芥牴鎹枃浠剁郴缁熺殑澶у皬鍜岄棯瀛樼殑绫烩€嬧€嬪瀷鑰屼笉鍚屻€備笅闈綔涓虹ず渚嬪睍绀哄畠浠殑鐢ㄦ硶锛屽苟璇峰姟蹇呴€傚綋鍦版浛鎹负浣犺嚜宸辩殑鍊笺€?
```
	size of unallocated flash:	0x503c0000 - 0x500e0000 = 0x2e0000
	size of the filesystem image:	0x00377424 - 0x00100000 = 0x277424
```
鎴戜滑褰撶劧瑕佽鍏ユ枃浠剁郴缁熸槧鍍忥紝浣嗘垜浠篃鎯虫妸瀹冨叏閮紙鍓╀綑绌洪棿锛夌粰

```
	fis unlock -f 0x500E0000 -l 0x2e0000
	fis erase -f 0x500E0000 -l 0x2e0000
	fis write -b 0x100000 -l 0x277424 -f 0x500E0000
	fis create "JFFS2" -n -f 0x500E0000 -l 0x2e0000
```
鐜板湪璇ユ枃浠剁郴缁熷氨涓?Linux 鍦ㄥ惎鍔ㄨ繃绋嬩腑鍙戠幇鐨?MTD 鈥滃垎鍖衡€濆叧鑱旇捣鏉ヤ簡銆備粠 Redboot 涓紝'fis list' 鍛戒护

```
	RedBoot> fis list
	Name              FLASH addr  Mem addr    Length      Entry point
	RedBoot           0x50000000  0x50000000  0x00020000  0x00000000
	RedBoot config    0x503C0000  0x503C0000  0x00020000  0x00000000
	FIS directory     0x503E0000  0x503E0000  0x00020000  0x00000000
	Linux kernel      0x50020000  0x00100000  0x000C0000  0x00000000
	JFFS2             0x500E0000  0x500E0000  0x002E0000  0x00000000
```
```
	SA1100 flash: probing 32-bit flash bus
	SA1100 flash: Found 2 x16 devices at 0x0 in 32-bit mode
	Using RedBoot partition definition
	Creating 5 MTD partitions on "SA1100 flash":
	0x00000000-0x00020000 : "RedBoot"
	0x00020000-0x000e0000 : "Linux kernel"
	0x000e0000-0x003c0000 : "JFFS2"
	0x003c0000-0x003e0000 : "RedBoot config"
	0x003e0000-0x00400000 : "FIS directory"
```
杩欓噷閲嶈鐨勬槸鎴戜滑鎰熷叴瓒ｇ殑鍒嗗尯浣嶇疆锛屽嵆绗笁涓€傚湪 Linux 涓紝杩欏搴斾簬 /dev/mtdblock2銆?鍥犳锛岃鐢ㄥ唴鏍稿強鍏跺湪闂瓨涓殑鏍规枃浠剁郴缁熷紩瀵?Linux锛屾垜浠?
```
	fis load "Linux kernel"
	exec -b 0x100000 -l 0xc0000 -c "root=/dev/mtdblock2"
```
褰撶劧涔熷彲浠ヤ娇鐢?JFFS 涔嬪鐨勫叾浠栨枃浠剁郴缁燂紝渚嬪 cramfs銆?浣犲彲鑳芥兂閫氳繃 NFS 鐢ㄦ牴鏂囦欢绯荤粺寮曞锛岀瓑绛夈€備篃鍙互锛堣€屼笖鏈夋椂鏇存柟渚匡級鍦ㄤ粠 ramdisk 鎴?NFS 寮曞鏃讹紝
鐩存帴浠?Linux 鍐呴儴灏嗘枃浠剁郴缁熺儳褰曪紙flash锛夊埌闂瓨銆侺inux MTD 浠撳簱涔熸湁璁稿澶勭悊闂瓨鍐呭瓨鐨勫伐鍏凤紝渚嬪鎿﹂櫎瀹冦€傜劧鍚?JFFS2
鍙互鐩存帴鎸傝浇鍒颁竴鍧楁柊鎿﹂櫎鐨勫垎鍖轰笂锛屾枃浠跺彲浠ョ洿鎺ュ鍒惰繃鍘汇€傜瓑绛夆€︹€?

### RedBoot 鑴氭湰


濡傛灉姣忔 Assabet 閲嶅惎閮借鎵嬪姩杈撳叆涓婅堪鎵€鏈夊懡浠わ紝灏辨病閭ｄ箞鏈夌敤浜嗐€傚洜姝ゅ彲浠ヤ娇鐢?RedBoot 鐨勮剼鏈姛鑳?灏嗗紩瀵艰繃绋嬭嚜鍔ㄥ寲銆?
渚嬪锛屾垜浣跨敤杩欎釜鏉ュ紩瀵煎悓鏃跺甫鏈夊唴鏍稿拰 ramdisk 鐨?Linux锛?
```
	RedBoot> fconfig
	Run script at boot: false true
	Boot script:
	Enter script, terminate with empty line
	>> load zImage -r -b 0x100000
	>> load ramdisk_ks.gz -r -b 0x800000
	>> exec -b 0x100000 -l 0xc0000
	>>
	Boot script timeout (1000ms resolution): 3
	Use BOOTP for network configuration: true
	GDB connection绔彛: 9000
	Network debug at boot time: false
	Update RedBoot non-volatile configuration - are you sure (y/n)? y
```
鐒跺悗锛岄噸鍚?Assabet 鍙渶绛夊緟鐧诲綍鎻愮ず鍑虹幇鍗冲彲銆?


Nicolas Pitre
nico@fluxnic.net
2001 骞?6 鏈?12 鏃?

### -rmk 鏍戜腑澶栬鐨勭姸鎬侊紙鏇存柊浜?2001/10/14锛?

Assabet锛? Serial ports锛堜覆鍙ｏ級:
  Radio:		TX, RX, CTS, DSR, DCD, RI
   - PM:		鏈祴璇曘€?   - COM:		TX, RX, CTS, DSR, DCD, RTS, DTR, PM
   - PM:		鏈祴璇曘€?   - I2C:		宸插疄鐜帮紝鏈厖鍒嗘祴璇曘€?   - L3:		宸插厖鍒嗘祴璇曪紝閫氳繃銆?   - PM:		鏈祴璇曘€?
 Video锛堣棰戯級:
  - LCD:		宸插厖鍒嗘祴璇曘€侾M

   锛堣繛鎺?neponset 鏃?LCD 涓嶅枩娆㈣娑堥殣锛?
  - Video out:		鏈畬鍏?
 Audio锛堥煶棰戯級:
  UDA1341:
  - Playback:		宸插厖鍒嗘祴璇曪紝閫氳繃銆?  - Record:		宸插疄鐜帮紝鏈祴璇曘€?  - PM:			鏈祴璇曘€?
  UCB1200:
  - Audio play:	宸插疄鐜帮紝鏈噸搴︽祴璇曘€?  - Audio rec:		宸插疄鐜帮紝鏈噸搴︽祴璇曘€?  - Telco audio play:	宸插疄鐜帮紝鏈噸搴︽祴璇曘€?  - Telco audio rec:	宸插疄鐜帮紝鏈噸搴︽祴璇曘€?  - POTS control:	鍚?  - Touchscreen:	鏄?  - PM:		鏈祴璇曘€?
 Other锛堝叾浠栵級:
  - PCMCIA:
  - LPE:		宸插厖鍒嗘祴璇曪紝閫氳繃銆?  - USB:		鍚?  - IRDA:
  - SIR:		宸插厖鍒嗘祴璇曪紝閫氳繃銆?  - FIR:		宸插厖鍒嗘祴璇曪紝閫氳繃銆?  - PM:			鏈祴璇曘€?
Neponset锛? Serial ports锛堜覆鍙ｏ級:
  - COM1,2:		TX, RX, CTS, DSR, DCD, RTS, DTR
  - PM:			鏈祴璇曘€?  - USB:		宸插疄鐜帮紝鏈噸搴︽祴璇曘€?  - PCMCIA:		宸插疄鐜帮紝鏈噸搴︽祴璇曘€?  - CF:			宸插疄鐜帮紝鏈噸搴︽祴璇曘€?  - PM:			鏈祴璇曘€?
鏇村鍐呭鍙湪 -np锛圢icolas Pitre 鐨勶級鏍戜腑鎵惧埌銆?