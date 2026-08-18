
## Chelsio N210 10Gb 浠ュお缃戠綉缁滄帶鍒跺櫒


Linux 椹卞姩鍙戝竷璇存槑

鐗堟湰 2.1.1

2005 骞?6 鏈?20 鏃?


 寮曡█锛圛NTRODUCTION锛?
 鐗规€э紙FEATURES锛?
 鎬ц兘锛圥ERFORMANCE锛?
 椹卞姩淇℃伅锛圖RIVER MESSAGES锛?
 宸茬煡闂锛圞NOWN ISSUES锛?
 鏀寔锛圫UPPORT锛?


## 寮曡█


 鏈枃妗ｆ弿杩颁簡 Chelsio 10Gb 浠ュお缃戠綉缁滄帶鍒跺櫒
 鐨?Linux 椹卞姩銆傝椹卞姩鏀寔 Chelsio N210 缃戝崱锛屽苟
 鍚戝悗鍏煎 Chelsio N110 鍨嬪彿鐨?10Gb 缃戝崱銆?


## 鐗规€?


### 鑷€傚簲涓柇锛坅daptive-rx锛?


  This feature provides an adaptive algorithm that adjusts the interrupt
  coalescing parameters, allowing the driver to dynamically adapt the latency
  settings to achieve the highest performance during various types of network
  load.

  The interface used to control this feature is ethtool. Please see the
  ethtool manpage for additional usage information.

  By default, adaptive-rx is disabled.
```

      ethtool -C <interface> adaptive-rx on

  To disable adaptive-rx, use ethtool::

      ethtool -C <interface> adaptive-rx off

  After disabling adaptive-rx, the timer latency value will be set to 50us.
  You may set the timer latency after disabling adaptive-rx::

      ethtool -C <interface> rx-usecs <microseconds>

  An example to set the timer latency value to 100us on eth0::

      ethtool -C eth0 rx-usecs 100

  You may also provide a timer latency value while disabling adaptive-rx::

      ethtool -C <interface> adaptive-rx off rx-usecs <microseconds>

  If adaptive-rx is disabled and a timer latency value is specified, the timer
  will be set to the specified value until changed by the user or until
  adaptive-rx is enabled.

  To view the status of the adaptive-rx and timer latency values::

      ethtool -c <interface>


```
### TCP 鍒嗘鍗歌浇锛圱SO锛夋敮鎸?


  This feature, also known as "large send", enables a system's protocol stack
  to offload portions of outbound TCP processing to a network interface card
  thereby reducing system CPU utilization and enhancing performance.

  The interface used to control this feature is ethtool version 1.8 or higher.
  Please see the ethtool manpage for additional usage information.

  By default, TSO is enabled.
```

      ethtool -K <interface> tso off

  To enable TSO::

      ethtool -K <interface> tso on

  To view the status of TSO::

      ethtool -k <interface>


```
## 鎬ц兘


 浠ヤ笅淇℃伅浣滀负涓€涓ず渚嬶紝璇存槑濡備綍鏇存敼绯荤粺
 鍙傛暟浠ヨ繘琛屸€滄€ц兘璋冧紭鈥濅互鍙婂簲浣跨敤浠€涔堝€笺€傛偍鍙兘甯屾湜涔熷彲鑳戒笉甯屾湜
 鏇存敼杩欎簺绯荤粺鍙傛暟锛岃繖鍙栧喅浜庢偍鐨勬湇鍔″櫒/宸ヤ綔绔?
 搴旂敤銆侰helsio Communications 涓嶄互浠讳綍鏂瑰紡淇濊瘉
 杩欐牱鍋氾紝骞朵笖椋庨櫓鐢辨偍鑷鎵挎媴銆侰helsio 涓嶄細瀵?
 鏁版嵁涓㈠け鎴栬澶囨崯鍧忔壙鎷呰矗浠汇€?

 鎮ㄧ殑鍙戣鐗堝彲鑳芥湁涓嶅悓鐨勬搷浣滄柟寮忥紝鎴栬€呮偍鍙兘鏇村枩娆?
 鍏朵粬鏂规硶銆傛樉绀鸿繖浜涘懡浠ゅ彧鏄负浜嗘彁渚涗竴涓?
 鎿嶄綔绀轰緥锛岀粷闈炲畾璁恒€?

 浠ヤ笅浠讳綍绯荤粺鏇存敼浠呭湪鎮ㄩ噸鏂板惎鍔ㄧ郴缁熶箣鍓嶆湁鏁堛€?
 鎮ㄥ彲鑳藉笇鏈涚紪鍐欎竴涓湪鍚姩鏃惰繍琛岀殑鑴氭湰锛?
 鍏朵腑鍖呭惈绯荤粺鐨勬渶浣宠缃€?

```

      setpci -d 1425::

```
- 0x0c.l=0x0000F800

```

      sysctl -w net.ipv4.tcp_timestamps=0

  Disabling SACK::

      sysctl -w net.ipv4.tcp_sack=0

  Setting large number of incoming connection requests::

      sysctl -w net.ipv4.tcp_max_syn_backlog=3000

  Setting maximum receive socket buffer size::

      sysctl -w net.core.rmem_max=1024000

  Setting maximum send socket buffer size::

      sysctl -w net.core.wmem_max=1024000

  Set smp_affinity (on a multiprocessor system) to a single CPU::

      echo 1 > /proc/irq/<interrupt_number>/smp_affinity

  Setting default receive socket buffer size::

      sysctl -w net.core.rmem_default=524287

  Setting default send socket buffer size::

      sysctl -w net.core.wmem_default=524287

  Setting maximum option memory buffers::

      sysctl -w net.core.optmem_max=524287

  Setting maximum backlog (# of unprocessed packets before kernel drops)::

      sysctl -w net.core.netdev_max_backlog=300000

  Setting TCP read buffers (min/default/max)::

      sysctl -w net.ipv4.tcp_rmem="10000000 10000000 10000000"

  Setting TCP write buffers (min/pressure/max)::

      sysctl -w net.ipv4.tcp_wmem="10000000 10000000 10000000"

  Setting TCP buffer space (min/pressure/max)::

      sysctl -w net.ipv4.tcp_mem="10000000 10000000 10000000"

  TCP window size for single connections:

   The receive buffer (RX_WINDOW) size must be at least as large as the
   Bandwidth-Delay Product of the communication link between the sender and
   receiver. Due to the variations of RTT, you may want to increase the buffer
   size up to 2 times the Bandwidth-Delay Product. Reference page 289 of
   "TCP/IP Illustrated, Volume 1, The Protocols" by W. Richard Stevens.

   At 10Gb speeds, use the following formula::

       RX_WINDOW >= 1.25MBytes * RTT(in milliseconds)
       Example for RTT with 100us: RX_WINDOW = (1,250,000 * 0.1) = 125,000

   RX_WINDOW sizes of 256KB - 512KB should be sufficient.

   Setting the min, max, and default receive buffer (RX_WINDOW) size::

       sysctl -w net.ipv4.tcp_rmem="<min> <default> <max>"

  TCP window size for multiple connections:
   The receive buffer (RX_WINDOW) size may be calculated the same as single
   connections, but should be divided by the number of connections. The
   smaller window prevents congestion and facilitates better pacing,
   especially if/when MAC level flow control does not work well or when it is
   not supported on the machine. Experimentation may be necessary to attain
   the correct value. This method is provided as a starting point for the
   correct receive buffer size.

   Setting the min, max, and default receive buffer (RX_WINDOW) size is
   performed in the same manner as single connection.


```
## 椹卞姩淇℃伅


 浠ヤ笅鏄?syslog 璁板綍鐨勬渶甯歌淇℃伅銆傝繖浜?
 淇℃伅鍙互鍦?/var/log/messages 涓壘鍒般€?

```

     Chelsio Network Driver - version 2.1.1

  NIC detected::

     eth#: Chelsio N210 1x10GBaseX NIC (rev #), PCIX 133MHz/64-bit

  Link up::

     eth#: link is up at 10 Gbps, full duplex

  Link down::

     eth#: link is down


```
## 宸茬煡闂


 杩欎簺闂鏄湪娴嬭瘯鏈熼棿鍙戠幇鐨勩€備互涓嬩俊鎭?
 浣滀负璇ラ棶棰樼殑涓存椂瑙ｅ喅鍔炴硶鎻愪緵銆傚湪鏌愪簺鎯呭喌涓嬶紝姝ら棶棰?
 鏄?Linux 鎴栫壒瀹?Linux 鍙戣鐗堝拰/鎴栫‖浠跺钩鍙?
 鎵€鍥烘湁鐨勩€?

  1. 澶氬鐞嗗櫒锛圫MP锛夌郴缁熶笂鍑虹幇澶ч噺 TCP 閲嶄紶銆?

      On a system with multiple CPUs, the interrupt (IRQ) for the network
      controller may be bound to more than one CPU. This will cause TCP
      retransmits if the packet data were to be split across different CPUs
      and re-assembled in a different order than expected.

      To eliminate the TCP retransmits, set smp_affinity on the particular
      interrupt to a single CPU. You can locate the interrupt (IRQ) used on
```

	  ifconfig <dev_name> | grep Interrupt

      Set the smp_affinity to a single CPU::

	  echo 1 > /proc/irq/<interrupt_number>/smp_affinity

      It is highly suggested that you do not run the irqbalance daemon on your
      system, as this will change any smp_affinity setting you have applied.
      The irqbalance daemon runs on a 10 second interval and binds interrupts
      to the least loaded CPU determined by the daemon. To disable this daemon::

	  chkconfig --level 2345 irqbalance off

      By default, some Linux distributions enable the kernel feature,
      irqbalance, which performs the same function as the daemon. To disable
      this feature, add the following line to your bootloader::

	  noirqbalance

	  Example using the Grub bootloader::

	      title Red Hat Enterprise Linux AS (2.4.21-27.ELsmp)
	      root (hd0,0)
	      kernel /vmlinuz-2.4.21-27.ELsmp ro root=/dev/hda3 noirqbalance
	      initrd /initrd-2.4.21-27.ELsmp.img

  2. After running insmod, the driver is loaded and the incorrect network
     interface is brought up without running ifup.

      When using 2.4.x kernels, including RHEL kernels, the Linux kernel
      invokes a script named "hotplug". This script is primarily used to
      automatically bring up USB devices when they are plugged in, however,
      the script also attempts to automatically bring up a network interface
      after loading the kernel module. The hotplug script does this by scanning
      the ifcfg-eth# config files in /etc/sysconfig/network-scripts, looking
      for HWADDR=<mac_address>.

      If the hotplug script does not find the HWADDRR within any of the
      ifcfg-eth# files, it will bring up the device with the next available
      interface name. If this interface is already configured for a different
      network card, your new interface will have incorrect IP address and
      network settings.

      To solve this issue, you can add the HWADDR=<mac_address> key to the
      interface config file of your network controller.

      To disable this "hotplug" feature, you may add the driver (module name)
      to the "blacklist" file located in /etc/hotplug. It has been noted that
      this does not work for network devices because the net.agent script
      does not use the blacklist file. Simply remove, or rename, the net.agent
      script located in /etc/hotplug to disable this feature.

  3. Transport Protocol (TP) hangs when running heavy multi-connection traffic
     on an AMD Opteron system with HyperTransport PCI-X Tunnel chipset.

      If your AMD Opteron system uses the AMD-8131 HyperTransport PCI-X Tunnel
      chipset, you may experience the "133-Mhz Mode Split Completion Data
      Corruption" bug identified by AMD while using a 133Mhz PCI-X card on the
      bus PCI-X bus.

      AMD states, "Under highly specific conditions, the AMD-8131 PCI-X Tunnel
      can provide stale data via split completion cycles to a PCI-X card that
      is operating at 133 Mhz", causing data corruption.

      AMD's provides three workarounds for this problem, however, Chelsio
      recommends the first option for best performance with this bug:

	For 133Mhz secondary bus operation, limit the transaction length and
	the number of outstanding transactions, via BIOS configuration
	programming of the PCI-X card, to the following:

	   Data Length (bytes): 1k

	   Total allowed outstanding transactions: 2

      Please refer to AMD 8131-HT/PCI-X Errata 26310 Rev 3.08 August 2004,
      section 56, "133-MHz Mode Split Completion Data Corruption" for more
      details with this bug and workarounds suggested by AMD.

      It may be possible to work outside AMD's recommended PCI-X settings, try
      increasing the Data Length to 2k bytes for increased performance. If you
      have issues with these settings, please revert to the "safe" settings
      and duplicate the problem before submitting a bug or asking for support.

      .. note::

	    The default setting on most systems is 8 outstanding transactions
	    and 2k bytes data length.

  4. On multiprocessor systems, it has been noted that an application which
     is handling 10Gb networking can switch between CPUs causing degraded
     and/or unstable performance.

      If running on an SMP system and taking performance measurements, it
      is suggested you either run the latest netperf-2.4.0+ or use a binding
      tool such as Tim Hockin's procstate utilities (runon)
      <http://www.hockin.org/~thockin/procstate/>.

      Binding netserver and netperf (or other applications) to particular
      CPUs will have a significant difference in performance measurements.
      You may need to experiment which CPU to bind the application to in
      order to achieve the best performance for your system.

      If you are developing an application designed for 10Gb networking,
      please keep in mind you may want to look at kernel functions
      sched_setaffinity & sched_getaffinity to bind your application.

      If you are just running user-space applications such as ftp, telnet,
      etc., you may want to try the runon tool provided by Tim Hockin's
      procstate utility. You could also try binding the interface to a
      particular CPU: runon 0 ifup eth0


```
## 鏀寔


 濡傛灉鎮ㄥ湪杞欢鎴栫‖浠舵柟闈㈤亣鍒伴棶棰橈紝璇烽€氳繃鐢靛瓙閭欢
 鑱旂郴鎴戜滑鐨勫鎴锋敮鎸佸洟闃燂細support@chelsio.com锛屾垨璁块棶鎴戜滑鐨勭綉绔?
 http://www.chelsio.com

-------------------------------------------------------------------------------

```

 Chelsio Communications
 370 San Aleso Ave.
 Suite 100
 Sunnyvale, CA 94085
 http://www.chelsio.com

```
鏈▼搴忔槸鑷敱杞欢锛涙偍鍙互閲嶆柊鍒嗗彂鍜?鎴栦慨鏀?
瀹冿紝閬靛惊 GNU 閫氱敤鍏叡璁稿彲璇佺 2 鐗堬紙鐢?
鑷敱杞欢鍩洪噾浼氬彂甯冿級鐨勬潯娆俱€?

鎮ㄥ簲璇ュ凡缁忛殢鏈▼搴忔敹鍒颁竴浠?GNU 閫氱敤鍏叡璁稿彲璇侊紱
濡傛灉娌℃湁锛岃鑷翠俊 Free Software Foundation, Inc.锛?
59 Temple Place - Suite 330, Boston, MA  02111-1307, USA銆?

鏈蒋浠舵寜 `AS IS` 鎻愪緵锛屼笉闄勫甫浠讳綍鏄庣ず鎴栨殫绀虹殑
鎷呬繚锛屽寘鎷絾涓嶉檺浜庡
閫傞攢鎬у拰鐗瑰畾鐢ㄩ€旈€傜敤鎬х殑鏆楃ず鎷呬繚銆?

鐗堟潈鎵€鏈?|copy| 2003-2005 Chelsio Communications銆備繚鐣欐墍鏈夋潈鍒┿€?

