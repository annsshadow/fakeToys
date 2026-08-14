
## AdvanSys Driver Notes


AdvanSys锛圓dvanced System Products, Inc.锛夌敓浜т互涓嬪熀浜?RISC銆佹敮鎸佹€荤嚎涓绘帶锛圔us-Mastering锛夈€丗ast锛?0 MHz锛変笌 Ultra锛?0 MHz锛夌獎甯︼紙8 浣嶄紶杈擄級鐨?SCSI 涓绘満閫傞厤鍣紝閫傜敤浜?ISA銆丒ISA銆乂L 鍜?PCI 鎬荤嚎锛涗互鍙婂熀浜?RISC銆佹敮鎸佹€荤嚎涓绘帶銆乁ltra锛?0 MHz锛夊甯︼紙16 浣嶄紶杈擄級鐨?SCSI 涓绘満閫傞厤鍣紝閫傜敤浜?PCI 鎬荤嚎銆?

涓嬫枃鐨?CDB 鏁伴噺琛ㄧず鍙瓨鍌ㄥ湪 RISC 鑺墖缂撳瓨涓庢澘杞?LRAM 涓殑 SCSI CDB锛堝懡浠ゆ弿杩板潡锛孋ommand Descriptor Block锛夎姹備釜鏁般€備竴涓?CDB 鍗充竴鏉?SCSI 鍛戒护銆傞┍鍔ㄧ殑鎺㈡祴渚嬬▼浼氭樉绀烘瘡涓妫€娴嬪埌鐨勯€傞厤鍣ㄥ彲鐢ㄧ殑 CDB 鏁伴噺銆傞€氳繃鍦?BIOS 涓洿鏀归€傞厤鍣ㄧ殑"Host Queue Size"锛堜富鏈洪槦鍒楀ぇ灏忥級璁剧疆锛屽彲浠ラ檷浣庨┍鍔ㄦ墍浣跨敤鐨?CDB 鏁伴噺銆?

Laptop Products:
  - ABP-480 - Bus-Master CardBus (16 CDB)

Connectivity Products:
   - ABP510/5150 - Bus-Master ISA (240 CDB)
   - ABP5140 - Bus-Master ISA PnP (16 CDB)
   - ABP5142 - Bus-Master ISA PnP with floppy (16 CDB)
   - ABP902/3902 - Bus-Master PCI (16 CDB)
   - ABP3905 - Bus-Master PCI (16 CDB)
   - ABP915 - Bus-Master PCI (16 CDB)
   - ABP920 - Bus-Master PCI (16 CDB)
   - ABP3922 - Bus-Master PCI (16 CDB)
   - ABP3925 - Bus-Master PCI (16 CDB)
   - ABP930 - Bus-Master PCI (16 CDB)
   - ABP930U - Bus-Master PCI Ultra (16 CDB)
   - ABP930UA - Bus-Master PCI Ultra (16 CDB)
   - ABP960 - Bus-Master PCI MAC/PC (16 CDB)
   - ABP960U - Bus-Master PCI MAC/PC Ultra (16 CDB)

Single Channel Products:
   - ABP542 - Bus-Master ISA with floppy (240 CDB)
   - ABP742 - Bus-Master EISA (240 CDB)
   - ABP842 - Bus-Master VL (240 CDB)
   - ABP940 - Bus-Master PCI (240 CDB)
   - ABP940U - Bus-Master PCI Ultra (240 CDB)
   - ABP940UA/3940UA - Bus-Master PCI Ultra (240 CDB)
   - ABP970 - Bus-Master PCI MAC/PC (240 CDB)
   - ABP970U - Bus-Master PCI MAC/PC Ultra (240 CDB)
   - ABP3960UA - Bus-Master PCI MAC/PC Ultra (240 CDB)
   - ABP940UW/3940UW - Bus-Master PCI Ultra-Wide (253 CDB)
   - ABP970UW - Bus-Master PCI MAC/PC Ultra-Wide (253 CDB)
   - ABP3940U2W - Bus-Master PCI LVD/Ultra2-Wide (253 CDB)

Multi-Channel Products:
   - ABP752 - Dual Channel Bus-Master EISA (240 CDB Per Channel)
   - ABP852 - Dual Channel Bus-Master VL (240 CDB Per Channel)
   - ABP950 - Dual Channel Bus-Master PCI (240 CDB Per Channel)
   - ABP950UW - Dual Channel Bus-Master PCI Ultra-Wide (253 CDB Per Channel)
   - ABP980 - Four Channel Bus-Master PCI (240 CDB Per Channel)
   - ABP980U - Four Channel Bus-Master PCI Ultra (240 CDB Per Channel)
   - ABP980UA/3980UA - Four Channel Bus-Master PCI Ultra (16 CDB Per Chan.)
   - ABP3950U2W - Bus-Master PCI LVD/Ultra2-Wide and Ultra-Wide (253 CDB)
   - ABP3950U3W - Bus-Master PCI Dual LVD2/Ultra3-Wide (253 CDB)

## Driver Compile Time Options and Debugging


浠ヤ笅甯搁噺鍙湪婧愭枃浠朵腑瀹氫箟銆?

1. ADVANSYS_ASSERT - 鍚敤椹卞姩鏂█锛堥粯璁わ細鍚敤锛?

   鍚敤姝ら€夐」浼氬悜椹卞姩涓坊鍔犳柇瑷€閫昏緫璇彞銆傚鏋滄柇瑷€澶辫触锛屼細鍚戞帶鍒跺彴鏄剧ず涓€鏉℃秷鎭紝浣嗙郴缁熷皢缁х画杩愯銆傞亣鍒扮殑浠讳綍鏂█閮藉簲鎶ュ憡缁欒礋璐ｈ椹卞姩鐨勪汉鍛樸€傛柇瑷€璇彞鍙互涓诲姩鍙戠幇椹卞姩涓殑闂锛屽苟鏈夊姪浜庝慨澶嶈繖浜涢棶棰樸€傚惎鐢ㄦ柇瑷€浼氱粰椹卞姩鐨勬墽琛屽甫鏉ュ皯閲忛澶栧紑閿€銆?

2. ADVANSYS_DEBUG - 鍚敤椹卞姩璋冭瘯锛堥粯璁わ細绂佺敤锛?

   鍚敤姝ら€夐」浼氬悜椹卞姩涓坊鍔犺窡韪嚱鏁帮紝骞舵敮鎸佸湪寮曞鏃惰缃┍鍔ㄨ窡韪骇鍒€傝閫夐」瀵逛簬璋冭瘯椹卞姩闈炲父鏈夌敤锛屼絾浼氬鍔犻┍鍔ㄥ彲鎵ц闀滃儚鐨勪綋绉苟甯︽潵鎵ц寮€閿€銆?

   璋冭瘯杈撳嚭鐨勬暟閲忓彲閫氳繃鍏ㄥ眬鍙橀噺 `asc_dbglvl` 鎺у埗銆傛暟鍊艰秺澶э紝杈撳嚭瓒婂銆傞粯璁よ皟璇曠骇鍒负 0銆?

   濡傛灉椹卞姩鍦ㄥ紩瀵兼椂鍔犺浇锛屼笖绯荤粺涓寘鍚簡 LILO 椹卞姩閫夐」锛屽垯鍙互閫氳繃鎸囧畾绗?5 涓紙ASC_NUM_IOPORT_PROBE + 1锛塈/O 绔彛鏉ユ洿鏀硅皟璇曠骇鍒€備吉 I/O 绔彛鐨勫墠涓変綅鍗佸叚杩涘埗鏁板瓧蹇呴』璁句负 `deb`锛岀鍥涗綅鍗佸叚杩涘埗鏁板瓧鎸囧畾璋冭瘯绾у埆锛? - F銆備互涓嬪懡浠よ灏嗗湪 0x330 澶勬煡鎵鹃€傞厤鍣?
```

      linux advansys=0x330,0,0,0,0xdeb2

   If the driver is built as a loadable module this variable can be
   defined when the driver is loaded. The following insmod command
   will set the debug level to one::

      insmod advansys.o asc_dbglvl=1

   Debugging Message Levels:


      ==== ==================
      0    Errors Only
      1    High-Level Tracing
      2-N  Verbose Tracing
      ==== ==================

   To enable debug output to console, please make sure that:

   a. System and kernel logging is enabled (syslogd, klogd running).
   b. Kernel messages are routed to console output. Check
      /etc/syslog.conf for an entry similar to this::

           kern.*                  /dev/console

   c. klogd is started with the appropriate -c parameter
      (e.g. klogd -c 8)

   This will cause printk() messages to be displayed on the
   current console. Refer to the klogd(8) and syslogd(8) man pages
   for details.

   Alternatively you can enable printk() to console with this
   program. However, this is not the 'official' way to do this.

   Debug output is logged in /var/log/messages.

   ::

     main()
     {
             syscall(103, 7, 0, 0);
     }

   Increasing LOG_BUF_LEN in kernel/printk.c to something like
   40960 allows more debug messages to be buffered in the kernel
   and written to the console or log file.

```
3. ADVANSYS_STATS - 鍚敤缁熻锛堥粯璁わ細鍚敤锛?

   鍚敤姝ら€夐」浼氬悜椹卞姩涓坊鍔犻€氳繃 /proc 杩涜鐨勭粺璁℃敹闆嗕笌鏄剧ず鍔熻兘銆傝淇℃伅鍙敤浜庣洃鎺ч┍鍔ㄤ笌璁惧鎬ц兘銆傚畠浼氬鍔犻┍鍔ㄥ彲鎵ц闀滃儚鐨勪綋绉紝骞剁粰椹卞姩鐨勬墽琛屽甫鏉ュ皯閲忛澶栧紑閿€銆?

   缁熻淇℃伅浠ユ瘡涓€傞厤鍣ㄤ负鍗曚綅杩涜缁存姢銆備細缁存姢椹卞姩鍏ュ彛鐐硅皟鐢ㄦ鏁颁笌浼犺緭澶у皬璁℃暟銆傜粺璁′俊鎭粎閫傜敤浜庣増鏈ぇ浜庢垨绛変簬 v1.3.0銆佷笖閰嶇疆浜?CONFIG_PROC_FS锛?proc锛夋枃浠剁郴缁熺殑鍐呮牳銆?

```

      /proc/scsi/advansys/{0,1,2,3,...}

   This information can be displayed with cat. For example::

      cat /proc/scsi/advansys/0

   When ADVANSYS_STATS is not defined the AdvanSys /proc files only
   contain adapter and device configuration information.

```
## Driver LILO Option


   濡傛灉瀵?init/main.c 杩涜浜嗕笂鏂?灏?AdvanSys 椹卞姩娣诲姞鍒?Linux"锛圔.4.锛変竴鑺傛墍杩扮殑淇敼锛岄┍鍔ㄥ皢璇嗗埆 `advansys` LILO 鍛戒护琛岄€夐」浠ュ強 /etc/lilo.conf 閫夐」銆傝閫夐」鍙敤浜庣鐢?I/O 绔彛鎵弿锛屾垨灏嗘壂鎻忛檺鍒朵负 1 - 4 涓?I/O 绔彛銆傛棤璁鸿閫夐」濡備綍璁剧疆锛孍ISA 涓?PCI 鏉垮崱浠嶄細琚悳绱㈠苟妫€娴嬪埌銆傝閫夐」浠呭奖鍝嶅 ISA 涓?VL 鏉垮崱鐨勬悳绱€?

绀轰緥锛?

```

	linux advansys=

     or::

	boot: linux advansys=0x0

  2. Limit I/O port scanning to one I/O port:

     boot::

	linux advansys=0x110

  3. Limit I/O port scanning to four I/O ports:

     boot::

	linux advansys=0x110,0x210,0x230,0x330

```
   瀵逛簬鍙姞杞芥ā鍧楋紝鍦ㄥ姞杞芥椂璁剧疆 `asc_iopflag` 鍙橀噺涓?`asc_ioport` 鏁扮粍浜﹀彲杈惧埌鐩稿悓鏁堟灉
```

      insmod advansys.o asc_iopflag=1 asc_ioport=0x110,0x330

```
   濡傛灉瀹氫箟浜?ADVANSYS_DEBUG锛屽彲浠ユ坊鍔犱竴涓 5 涓紙ASC_NUM_IOPORT_PROBE + 1锛塈/O 绔彛鏉ユ寚瀹氶┍鍔ㄨ皟璇曠骇鍒€傛洿澶氫俊鎭鍙傞槄涓婃枃"椹卞姩缂栬瘧鏃堕€夐」涓庤皟璇?涓€鑺傘€?

## Credits (Chronological Order)


Bob Frey <bfrey@turbolinux.com.cn> wrote the AdvanSys SCSI driver
and maintained it up to 3.3F. He continues to answer questions
and help maintain the driver.

Nathan Hartwell <mage@cdc3.cdc.net> provided the directions and
basis for the Linux v1.3.X changes which were included in the
1.2 release.

Thomas E Zerucha <zerucha@shell.portal.com> pointed out a bug
in advansys_biosparam() which was fixed in the 1.3 release.

Erik Ratcliffe <erik@caldera.com> has done testing of the
AdvanSys driver in the Caldera releases.

Rik van Riel <H.H.vanRiel@fys.ruu.nl> provided a patch to
AscWaitTixISRDone() which he found necessary to make the
driver work with a SCSI-1 disk.

Mark Moran <mmoran@mmoran.com> has helped test Ultra-Wide
support in the 3.1A driver.

Doug Gilbert <dgilbert@interlog.com> has made changes and
suggestions to improve the driver and done a lot of testing.

Ken Mort <ken@mort.net> reported a DEBUG compile bug fixed
in 3.2K.

Tom Rini <trini@kernel.crashing.org> provided the CONFIG_ISA
patch and helped with PowerPC wide and narrow board support.

Philip Blundell <philb@gnu.org> provided an
advansys_interrupts_enabled patch.

Dave Jones <dave@denial.force9.co.uk> reported the compiler
warnings generated when CONFIG_PROC_FS was not defined in
the 3.2M driver.

Jerry Quinn <jlquinn@us.ibm.com> fixed PowerPC support (endian
problems) for wide cards.

Bryan Henderson <bryanh@giraffe-data.com> helped debug narrow
card error handling.

Manuel Veloso <veloso@pobox.com> worked hard on PowerPC narrow
board support and fixed a bug in AscGetEEPConfig().

Arnaldo Carvalho de Melo <acme@conectiva.com.br> made
save_flags/restore_flags changes.

Andy Kellner <AKellner@connectcom.net> continued the Advansys SCSI
driver development for ConnectCom (Version > 3.3F).

Ken Witherow for extensive testing during the development of version 3.4.
