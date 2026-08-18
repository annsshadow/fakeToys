## MOXA Smartio/Industio Family Device Driver Installation Guide


Copyright (C) 2008, Moxa Inc.
Copyright (C) 2021, Jiri Slaby


   1. Introduction
   2. System Requirement
   3. Installation
      3.1 Hardware installation
      3.2 Device naming convention
   4. Utilities
   5. Setserial
   6. Troubleshooting

##### 1. Introduction


   Smartio/Industio/UPCI 绯诲垪 Linux 椹卞姩鏀寔浠ヤ笅澶氱鍙ｆ澘鍗★細

    - 2 ports multiport board
	CP-102U, CP-102UL, CP-102UF
	CP-132U-I, CP-132UL,
	CP-132, CP-132I, CP132S, CP-132IS,
	(CP-102, CP-102S)

    - 4 ports multiport board
	CP-104EL,
	CP-104UL, CP-104JU,
	CP-134U, CP-134U-I,
	C104H/PCI, C104HS/PCI,
	CP-114, CP-114I, CP-114S, CP-114IS, CP-114UL,
	(C114HI, CT-114I),
	POS-104UL,
	CB-114,
	CB-134I

    - 8 ports multiport board
	CP-118EL, CP-168EL,
	CP-118U, CP-168U,
	C168H/PCI,
	CB-108

   濡傛灉鍙戠敓鍏煎鎬ч棶棰橈紝璇疯仈绯?Moxa锛歴upport@moxa.com.tw銆?
   闄よ澶囬┍鍔ㄥ锛屾湰鐗堟湰杩樻彁渚涗簡涓€浜涙湁鐢ㄧ殑宸ュ叿銆傚畠浠槸锛?
    - msdiag
		 鐢ㄤ簬鏄剧ず宸插畨瑁呯殑 Moxa Smartio/Industio 鏉垮崱鐨勮瘖鏂▼搴忋€?    - msmon
		 鐢ㄤ簬瑙傚療鏁版嵁璁℃暟鍜岀嚎璺姸鎬佷俊鍙风殑鐩戣绋嬪簭銆?    - msterm     涓€涓敤浜庢祴璇曚覆鍙ｇ殑绠€鍗曠粓绔▼搴忋€?
   鏈増鏈腑鎵€鏈夌殑椹卞姩鍜屽伐鍏烽兘浠ユ簮浠ｇ爜褰㈠紡鍦?GNU General Public License 涓嬪彂甯冦€傝鎯呰鍙傞槄鍚勬簮浠ｇ爜鏂囦欢涓殑 GNU General Public License 澹版槑銆?
   鍦?Moxa 鐨勭綉绔欎笂锛屼綘鎬昏兘鎵惧埌鏈€鏂伴┍鍔細https://www.moxa.com/銆?
   鏈増鏈┍鍔ㄥ彲浠ュ畨瑁呬负鍙姞杞芥ā鍧楋紙Module driver锛夋垨鍐呭缓鍒板唴鏍镐腑锛圫tatic driver锛夈€傚畨瑁呴┍鍔ㄥ墠锛岃鍙傝€冪敤鎴锋墜鍐屼腑鐨勭‖浠跺畨瑁呮楠ゃ€?
   鎴戜滑鍋囪鐢ㄦ埛搴斿綋鐔熸倝浠ヤ笅鏂囨。锛?
   - Serial-HOWTO
   - Kernel-HOWTO

##### 2. System Requirement


   - 鏈€澶氬彲缁勫悎瀹夎 4 鍧楁澘鍗?
##### 3. Installation


## 3.1 Hardware installation


### PCI/UPCI board


   浣犲彲鑳介渶瑕佸湪 BIOS 涓皟鏁?IRQ 浣跨敤浠ラ伩鍏嶄笌鍏朵粬 ISA 璁惧鍙戠敓 IRQ 鍐茬獊銆傝鎻愬墠鍙傝€冪敤鎴锋墜鍐屼腑鐨勭‖浠跺畨瑁呮楠ゃ€?
### PCI IRQ Sharing


   鍚屼竴鍧楀绔彛鏉垮崱鍐呯殑姣忎釜绔彛鍏变韩鍚屼竴涓?IRQ銆傛渶澶氬彲灏?4 鍧?Moxa Smartio/Industio PCI 绯诲垪澶氱鍙ｆ澘鍗″畨瑁呭湪鍚屼竴绯荤粺涓紝骞朵笖瀹冧滑鍙互鍏变韩鍚屼竴涓?IRQ銆?


## 3.2 Device naming convention


   璁惧鑺傜偣鍛藉悕涓?"ttyMxx"銆?
### Device naming when more than 2 boards installed


   Smartio/Industio 姣忓潡澶氱鍙ｆ澘鍗＄殑鍛藉悕绾﹀畾棰勫畾涔夊涓嬨€?
   ============ ===============
   Board Num.	Device node
   1st board	ttyM0  - ttyM7
   2nd board	ttyM8  - ttyM15
   3rd board	ttyM16 - ttyM23
   4th board	ttyM24 - ttyM31
   ============ ===============

##### 4. Utilities


   鏈┍鍔ㄥ寘鍚?3 涓伐鍏凤紝鍗?msdiag銆乵smon 鍜?msterm銆傝繖 3 涓伐鍏蜂互婧愪唬鐮佸舰寮忓彂甯冦€傚畠浠簲褰撹缂栬瘧涓哄彲鎵ц鏂囦欢骞跺鍒跺埌 /usr/bin銆?
## msdiag - Diagnostic


   璇ュ伐鍏锋彁渚涙樉绀虹郴缁熶腑椹卞姩鎵€鎵惧埌鐨?Moxa Smartio/Industio 鏉垮崱鐨勫姛鑳姐€?
## msmon - Port Monitoring


   璇ュ伐鍏疯鐢ㄦ埛蹇€熸煡鐪嬫墍鏈?MOXA 绔彛鐨勬椿鍔ㄣ€傚彲浠ヨ交鏉句簡瑙ｆ瘡涓鍙ｈ嚜鐩戣寮€濮嬩互鏉ョ殑鎺ユ敹/鍙戦€侊紙Rx/Tx锛夊瓧绗︽€绘暟銆?
   姣忕鐨?Rx/Tx 鍚炲悙閲忔棦鎸夐棿闅旓紙渚嬪鏈€杩?5 绉掞級鎶ュ憡锛屼篃鎸夊钩鍧囷紙鑷洃瑙嗗紑濮嬩互鏉ワ級鎶ュ憡銆備綘鍙互鎸?<HOME> 閿噸缃墍鏈夌鍙ｈ鏁般€傛寜 <+> <->锛堝姞/鍑忥級閿洿鏀规樉绀虹殑鏃堕棿闂撮殧銆傚湪鍏夋爣鎵€鍦ㄧ鍙ｄ笂鎸?<ENTER> 鍙煡鐪嬭绔彛鐨勯€氫俊鍙傛暟銆佷俊鍙风姸鎬佷互鍙婅緭鍏?杈撳嚭闃熷垪銆?
## msterm - Terminal Emulation


   璇ュ伐鍏锋彁渚涙墍鏈?tty 绔彛锛堝挨鍏舵槸 MOXA 绔彛锛夌殑鏁版嵁鏀跺彂鑳藉姏銆傚畠瀵规祴璇曠畝鍗曞簲鐢ㄥ緢鏈夌敤锛屼緥濡傚悜杩炴帴鍒拌绔彛鐨勮皟鍒惰В璋冨櫒鍙戦€?AT 鍛戒护锛屾垨浣滀负鐧诲綍鐢ㄧ殑缁堢銆傛敞鎰忥紝杩欏彧鏄竴涓搼缁堢浠跨湡锛屼笉澶勭悊鍏ㄥ睆鎿嶄綔銆?
##### 5. Setserial


   鏀寔鐨?Setserial 鍙傛暟濡備笅銆?
   ============== =============================================================
   uart		  set UART type(16450 --> disable FIFO, 16550A --> enable FIFO)
   close_delay	  set the amount of time (in 1/100 of a second) that DTR
		  should be kept low while being closed.
   closing_wait   set the amount of time (in 1/100 of a second) that the
		  serial port should wait for data to be drained while
		  being closed, before the receiver is disabled.
   spd_hi	  Use 57.6kb when the application requests 38.4kb.
   spd_vhi	  Use 115.2kb when the application requests 38.4kb.
   spd_shi	  Use 230.4kb when the application requests 38.4kb.
   spd_warp	  Use 460.8kb when the application requests 38.4kb.
   spd_normal	  Use 38.4kb when the application requests 38.4kb.
   spd_cust	  Use the custom divisor to set the speed when the
		  application requests 38.4kb.
   divisor	  This option sets the custom division.
   baud_base	  This option sets the base baud rate.
   ============== =============================================================

##### 6. Troubleshooting


   鍚姩鏃剁殑閿欒娑堟伅鍙婅В鍐虫柟妗堝凡灏藉彲鑳芥竻鏅板湴璇存槑銆傚鏋滄墍鏈夊彲鑳界殑瑙ｅ喅鏂规閮藉け璐ワ紝璇疯仈绯绘垜浠殑鎶€鏈敮鎸佸洟闃熶互鑾峰彇鏇村甯姪銆?

   Error msg:
	      More than 4 Moxa Smartio/Industio family boards found. Fifth board
              and after are ignored.

   Solution:
   涓洪伩鍏嶆闂锛岃鎷斾笅绗簲鍧楀強涔嬪悗鐨勬澘鍗★紝鍥犱负 Moxa 椹卞姩鏈€澶氭敮鎸?4 鍧楁澘鍗°€?