
## Early Printk


鍦?x86 绯荤粺涓婁娇鐢?earlyprintk=dbgp 鍚姩閫夐」閰嶅悎 USB2 Debug 绔彛瀵嗛挜鍜岃皟璇曠嚎缂嗙殑 Mini-HOWTO銆?
浣犻渶瑕佷袱鍙扮數鑴戙€佲€淯SB debug key鈥濅笓鐢ㄥ皬璁惧浠ュ強
```

  [host/target] <-------> [USB debug key] <-------> [client/console]

```
## Hardware requirements


  a) 涓绘満/鐩爣绯荤粺闇€瑕佸叿澶?USB debug 绔彛鑳藉姏銆?
     浣犲彲浠ラ€氳繃鏌ョ湅 'Debug port' 浣嶆潵妫€鏌ヨ鑳藉姏锛屾柟娉曟槸杩愯
```

       # lspci -vvv
       ...
       00:1d.7 USB Controller: Intel Corporation 82801H (ICH8 Family) USB2 EHCI Controller #1 (rev 03) (prog-if 20 [EHCI])
               Subsystem: Lenovo ThinkPad T61
               Control: I/O- Mem+ BusMaster+ SpecCycle- MemWINV- VGASnoop- ParErr- Stepping- SERR+ FastB2B- DisINTx-
               Status: Cap+ 66MHz- UDF- FastB2B+ ParErr- DEVSEL=medium >TAbort- <TAbort- <MAbort- >SERR- <PERR- INTx-
               Latency: 0
               Interrupt: pin D routed to IRQ 19
               Region 0: Memory at fe227000 (32-bit, non-prefetchable) [size=1K]
               Capabilities: [50] Power Management version 2
                       Flags: PMEClk- DSI- D1- D2- AuxCurrent=375mA PME(D0+,D1-,D2-,D3hot+,D3cold+)
                       Status: D0 PME-Enable- DSel=0 DScale=0 PME+
               Capabilities: [58] Debug port: BAR=1 offset=00a0
                            ^^^^^^^^^^^ <==================== [ HERE ]
               Kernel driver in use: ehci_hcd
               Kernel modules: ehci-hcd
       ...

     .. note::
       濡傛灉浣犵殑绯荤粺娌℃湁鍒楀嚭 debug 绔彛鑳藉姏锛岄偅涔堜綘鍙兘鏃犳硶浣跨敤 USB debug key銆?
  b) 浣犺繕闇€瑕佷竴涓?NetChip USB 璋冭瘯绾跨紗/瀵嗛挜锛?
        http://www.plxtech.com/products/NET2000/NET20DC/default.asp

     杩欐槸涓€涓甫涓や釜 USB 鎺ュ彛鐨勫皬宸ц摑鑹插鏂欒繛鎺ュ櫒锛涘畠浠?USB 鎺ュ彛鍙栫數銆?
  c) 浣犻渶瑕佺浜屽彴甯﹂珮閫?USB 2.0 绔彛鐨?client/console 绯荤粺銆?
  d) NetChip 璁惧蹇呴』鐩存帴鎻掑叆鈥滀富鏈?鐩爣鈥濈郴缁熶笂鐨勭墿鐞?debug 绔彛銆備笉鑳藉湪鐗╃悊 debug 绔彛涓庘€滀富鏈?鐩爣鈥濈郴缁熶箣闂翠娇鐢?USB 闆嗙嚎鍣ㄣ€?
     EHCI 璋冭瘯鎺у埗鍣ㄧ粦瀹氬埌鐗瑰畾鐨勭墿鐞?USB 绔彛锛孨etChip 璁惧鍙兘鍦ㄨ绔彛涓綔涓?early printk 璁惧宸ヤ綔銆侲HCI 涓绘満鎺у埗鍣ㄧ殑鐢垫皵甯冪嚎浣垮緱 EHCI 璋冭瘯鎺у埗鍣ㄨ繛鎺ュ埌绗竴涓墿鐞嗙鍙ｏ紝骞朵笖鏃犳硶閫氳繃杞欢鏇存敼杩欎竴鐐广€備綘鍙互閫氳繃閫愪釜灏濊瘯绯荤粺涓婃瘡涓墿鐞嗙鍙ｅ苟閲嶅惎鏉ュ彂鐜扮墿鐞嗙鍙ｃ€傛垨鑰呬綘涔熷彲浠ュ皾璇曚娇鐢?lsusb锛屾垨鏌ョ湅鎶?USB 璁惧鎻掑叆鈥滀富鏈?鐩爣鈥濈郴缁熷悇涓鍙ｆ椂 usb 鍗忚鏍堝彂鍑虹殑鍐呮牳淇℃伅娑堟伅銆?
     涓€浜涚‖浠跺巶鍟嗘病鏈夌敤鐗╃悊杩炴帴鍣ㄦ毚闇?usb debug 绔彛锛屽鏋滀綘鍙戠幇杩欐牱鐨勮澶囷紝璇峰悜纭欢鍘傚晢鎶曡瘔锛屽洜涓烘病鏈夌悊鐢变笉鎶婅绔彛鎺ュ埌鏌愪釜鐗╃悊鍙闂殑绔彛涓娿€?
  e) 鍚屾牱閲嶈鐨勬槸锛岃澶氱増鏈殑 NetChip 璁惧瑕佹眰鈥渃lient/console鈥濈郴缁熸彃鍏ヨ澶囩殑鍙充晶锛堜骇鍝?logo 鏈濅笂闈㈠悜锛屼粠宸﹀埌鍙冲彲璇伙級銆傚師鍥犳槸 5 浼忕數婧愬彧浠庤璁惧鐨勪竴渚у彇鐢碉紝涓斿繀椤绘槸涓嶄細琚噸鍚殑閭ｄ竴渚с€?
```
## Software requirements


  a) 鍦ㄤ富鏈?鐩爣绯荤粺涓婏細

```

      CONFIG_EARLY_PRINTK_DBGP=y

    骞朵笖浣犻渶瑕佹坊鍔犲惎鍔ㄥ懡浠よ锛?earlyprintk=dbgp"銆?
    .. note::
      濡傛灉浣犱娇鐢?Grub锛岃灏嗗叾杩藉姞鍒?/etc/grub.conf 鐨?'kernel' 琛屻€傚鏋滀綘鍦?BIOS 鍥轰欢绯荤粺涓婁娇鐢?Grub2锛岃灏嗗叾杩藉姞鍒?/boot/grub2/grub.cfg 鐨?'linux' 琛屻€傚鏋滀綘鍦?EFI 鍥轰欢绯荤粺涓婁娇鐢?Grub2锛岃灏嗗叾杩藉姞鍒?/boot/grub2/grub.cfg 鎴?/boot/efi/EFI/<distro>/grub.cfg 鐨?'linux' 鎴?'linuxefi' 琛屻€?
    鍦ㄦ湁澶氫釜 EHCI 璋冭瘯鎺у埗鍣ㄧ殑绯荤粺涓婏紝浣犲繀椤绘寚瀹氭纭殑 EHCI 璋冭瘯鎺у埗鍣ㄧ紪鍙枫€傚叾椤哄簭鏉ヨ嚜 EHCI 鎺у埗鍣ㄧ殑 PCI 鎬荤嚎鏋氫妇銆備笉甯︾紪鍙峰弬鏁扮殑榛樿鍊兼槸 "0"锛屽嵆绗竴涓?EHCI 璋冭瘯鎺у埗鍣ㄣ€傝浣跨敤绗簩涓?EHCI 璋冭瘯鎺у埗鍣紝浣犲彲浠ヤ娇鐢ㄥ懡浠よ锛?earlyprintk=dbgp1"

    .. note::
      閫氬父 earlyprintk 鎺у埗鍙板湪甯歌鎺у埗鍙板氨缁悗浼氳鍏抽棴鈥斺€斾娇鐢?"earlyprintk=dbgp,keep" 鍙互鍦ㄦ棭鏈熷惎鍔ㄤ箣鍚庝繚鎸佽閫氶亾鎵撳紑銆傝繖瀵硅皟璇?Xorg 涓嬬殑宕╂簝绛夊満鏅緢鏈夌敤銆?
  b) 鍦?client/console 绯荤粺涓婏細

    浣犲簲璇ュ惎鐢ㄤ互涓嬪唴鏍搁厤缃€夐」锛氾細

      CONFIG_USB_SERIAL_DEBUG=y

    涓嬩竴娆′娇鐢ㄤ慨鏀瑰悗鐨勫唴鏍稿惎鍔ㄦ椂锛屼綘搴旇浼氬緱鍒颁竴涓?/dev/ttyUSBx 璁惧锛堟垨澶氫釜锛夈€?
    鐜板湪杩欎釜鍐呮牳娑堟伅閫氶亾宸插噯澶囧ソ浣跨敤锛氬惎鍔ㄤ綘鍠滄鐨勭粓绔豢鐪熷櫒锛坢inicom 绛夛級骞跺皢鍏堕厤缃负浣跨敤 /dev/ttyUSB0鈥斺€旀垨鑰呬娇鐢ㄥ師濮嬬殑 'cat /dev/ttyUSBx' 鏉ユ煡鐪嬪師濮嬭緭鍑恒€?
  c) 鍦ㄥ熀浜?Nvidia 鍗楁ˉ鐨勭郴缁熶笂锛氬唴鏍镐細灏濊瘯鎺㈡祴骞舵壘鍑哄摢涓鍙ｈ繛鎺ヤ簡璋冭瘯璁惧銆?
```
## Testing


浣犲彲浠ラ€氳繃浣跨敤 earlyprintk=dbgp,keep 骞跺湪涓绘満/鐩爣绯荤粺涓婅Е鍙戝唴鏍告秷鎭潵娴嬭瘯杈撳嚭銆備綘鍙互瑙﹀彂涓€涓棤瀹崇殑
```

     echo h > /proc/sysrq-trigger

```

```

     SysRq : HELP : loglevel(0-9) reBoot Crashdump terminate-all-tasks(E) memory-full-oom-kill(F) kill-all-tasks(I) saK show-backtrace-all-active-cpus(L) show-memory-usage(M) nice-all-RT-tasks(N) powerOff show-registers(P) show-all-timers(Q) unRaw Sync show-task-states(T) Unmount show-blocked-tasks(W) dump-ftrace-buffer(Z)

```

```

       cat /dev/ttyUSB0

```
鍦ㄤ綘浜庝富鏈虹郴缁熶笂瑙﹀彂鍚庯紝搴旇寰堝揩灏辫兘鐪嬪埌涓婇潰鐨勫府鍔╄銆?
濡傛灉瀹冧笉宸ヤ綔锛岃鍦?linux-kernel@vger.kernel.org 閭欢鍒楄〃涓婅闂紝鎴栬仈绯?x86 缁存姢鑰呫€?