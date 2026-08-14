
## AdvanSys Driver Notes


AdvanSys（Advanced System Products, Inc.）生产以下基于 RISC、支持总线主控（Bus-Mastering）、Fast（10 MHz）与 Ultra（20 MHz）窄带（8 位传输）的 SCSI 主机适配器，适用于 ISA、EISA、VL 和 PCI 总线；以及基于 RISC、支持总线主控、Ultra（20 MHz）宽带（16 位传输）的 SCSI 主机适配器，适用于 PCI 总线。

下文的 CDB 数量表示可存储在 RISC 芯片缓存与板载 LRAM 中的 SCSI CDB（命令描述块，Command Descriptor Block）请求个数。一个 CDB 即一条 SCSI 命令。驱动的探测例程会显示每个被检测到的适配器可用的 CDB 数量。通过在 BIOS 中更改适配器的"Host Queue Size"（主机队列大小）设置，可以降低驱动所使用的 CDB 数量。

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


以下常量可在源文件中定义。

1. ADVANSYS_ASSERT - 启用驱动断言（默认：启用）

   启用此选项会向驱动中添加断言逻辑语句。如果断言失败，会向控制台显示一条消息，但系统将继续运行。遇到的任何断言都应报告给负责该驱动的人员。断言语句可以主动发现驱动中的问题，并有助于修复这些问题。启用断言会给驱动的执行带来少量额外开销。

2. ADVANSYS_DEBUG - 启用驱动调试（默认：禁用）

   启用此选项会向驱动中添加跟踪函数，并支持在引导时设置驱动跟踪级别。该选项对于调试驱动非常有用，但会增加驱动可执行镜像的体积并带来执行开销。

   调试输出的数量可通过全局变量 `asc_dbglvl` 控制。数值越大，输出越多。默认调试级别为 0。

   如果驱动在引导时加载，且系统中包含了 LILO 驱动选项，则可以通过指定第 5 个（ASC_NUM_IOPORT_PROBE + 1）I/O 端口来更改调试级别。伪 I/O 端口的前三位十六进制数字必须设为 `deb`，第四位十六进制数字指定调试级别：0 - F。以下命令行将在 0x330 处查找适配器
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
3. ADVANSYS_STATS - 启用统计（默认：启用）

   启用此选项会向驱动中添加通过 /proc 进行的统计收集与显示功能。该信息可用于监控驱动与设备性能。它会增加驱动可执行镜像的体积，并给驱动的执行带来少量额外开销。

   统计信息以每个适配器为单位进行维护。会维护驱动入口点调用次数与传输大小计数。统计信息仅适用于版本大于或等于 v1.3.0、且配置了 CONFIG_PROC_FS（/proc）文件系统的内核。

```

      /proc/scsi/advansys/{0,1,2,3,...}

   This information can be displayed with cat. For example::

      cat /proc/scsi/advansys/0

   When ADVANSYS_STATS is not defined the AdvanSys /proc files only
   contain adapter and device configuration information.

```
## Driver LILO Option


   如果对 init/main.c 进行了上文"将 AdvanSys 驱动添加到 Linux"（B.4.）一节所述的修改，驱动将识别 `advansys` LILO 命令行选项以及 /etc/lilo.conf 选项。该选项可用于禁用 I/O 端口扫描，或将扫描限制为 1 - 4 个 I/O 端口。无论该选项如何设置，EISA 与 PCI 板卡仍会被搜索并检测到。该选项仅影响对 ISA 与 VL 板卡的搜索。

示例：

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
   对于可加载模块，在加载时设置 `asc_iopflag` 变量与 `asc_ioport` 数组亦可达到相同效果
```

      insmod advansys.o asc_iopflag=1 asc_ioport=0x110,0x330

```
   如果定义了 ADVANSYS_DEBUG，可以添加一个第 5 个（ASC_NUM_IOPORT_PROBE + 1）I/O 端口来指定驱动调试级别。更多信息请参阅上文"驱动编译时选项与调试"一节。

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
