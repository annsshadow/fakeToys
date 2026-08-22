
## EQL 驱动：串IP 负载均衡 HOWTO


  Simon "Guru Aleph-Null" Janes, simon@ncm.com

  v1.1锛?995 骞?2 鏈?27 鏃。
  本手册介EQL 设备驱动。EQL 是一个软件设备，可让你对 IP 串行链路
  （SLIP 或未压缩PPP）进行负载均衡以提升带宽。它不会降低你的延迟
  （即 ping 时间），除非你的链路上本来就有大量流量，那样它会有所帮助  该驱动已1.1.75 内核上测试，并已知可干净地打1.1.86。对 1.1.92
  也做过一些测试，使用的是 v1.1 补丁，该补丁仅为了在最新的内核源码树中
  干净地打上而创建。（是的，工作正常。）

## 1. 简

  哪个更糟6K 专线的昂贵费用，还是两条电话线？很可能是前者。如果你
  渴望更多带宽，并ISP 比较灵活，现在可以将多个调制解调器绑定在一起，
  作为一条点对点链路工作以提升带宽。而且两端都不需要特殊的黑盒子

  eql 驱动仅在 Livingston PortMaster-2e 终端服务器上测试过。我不知道其  终端服务器是否支持负载均衡，但我确知 PortMaster 支持，而且做得几乎  eql 驱动一样好（—遗憾的是，在我目前的测试中，Livingston PortMaster 2e
  的负载均衡比测试机使28.8 Kbps 14.4 Kbps 连接时要慢约 1 2 KB/s  不过我不确定这到底是 PortMaster 的问题，还是 Linux TCP 驱动的问题  不过有人告诉Linux TCP 实现相当快。——）


  我向各位 ISP 建议，对负载均衡客户按第二线75%、第三线50% 等收  大概比较公平…

  嘿，大家都可以做做梦嘛…

## 2. 内核配置


  这里我描述让内核支持 eql 驱动并正常工作的总体步骤，从打补丁、编译到安装

### 2.1. 给内核打补丁


  如果你没有或无法获得已经合入 eql 驱动的内核，可从
  ftp://slaughter.ncm.com/pub/Linux/LOAD_BALANCING/eql-1.1.tar.gz 获取驱动副本  将归档解包到一个明显的位置，例/usr/local/src/。它```
       -rw-r--r-- guru/ncm	198 Jan 19 18:53 1995 eql-1.1/NO-WARRANTY
       -rw-r--r-- guru/ncm	30620 Feb 27 21:40 1995 eql-1.1/eql-1.1.patch
       -rwxr-xr-x guru/ncm	16111 Jan 12 22:29 1995 eql-1.1/eql_enslave
       -rw-r--r-- guru/ncm	2195 Jan 10 21:48 1995 eql-1.1/eql_enslave.c

  Unpack a recent kernel (something after 1.1.92) someplace convenient
  like say /usr/src/linux-1.1.92.eql. Use symbolic links to point
  /usr/src/linux to this development directory.


  Apply the patch by running the commands::

       cd /usr/src
       patch </usr/local/src/eql-1.1/eql-1.1.patch


```
### 2.2. 编译内核


  打完补丁后，运行 make config 并为你的硬件配置内核

  配置完成后，按你的习惯进make 和安装

## 3. 网络配置


  到目前为止，我只eql 设备Matt Dillon DSLIP SLIP 连接管理器一  使用过（—"那个为了快速写出这么多代码而卖掉了灵魂的人 ——）  如何为其他“连接”管理器配置它，由你自己决定。我见过的多数其他连接管理器
  在处理多于一个连接时做得并不好

### 3.1. /etc/rc.d/rc.inet1


  rc.inet1 中，ifconfig eql 设备配置为你机器通常使用IP 地址  以及你偏好的 SLIP 线路 MTU。有人会MTU 对两个调制解调器应大致为通常
  大小的一半，三个为三分之一，四个为四分之一，以此类推……但降到 296 以下
  可能就过度了。下面是一ifconfig 示例
```
       ifconfig eql 198.67.33.239 mtu 1006

  Once the eql device is up and running, add a static default route to
  it in the routing table using the cool new route syntax that makes
  life so much easier::

       route add default eql


```
### 3.2. 手动纳入（enslave）设

  手动纳入设备需要两个实用程序：eql_enslave eql_emancipate（—eql_emancipate
  尚未编写，因为当被纳入的设备“死亡”时会自动退出队列。我还没找到一个好的理  去写它……除了为了完整性，但那不是个好的动机，不是吗？——）


  纳入设备的语法是 "eql_enslave <master-name>
```
       eql_enslave eql sl0 28800
       eql_enslave eql ppp0 14400
       eql_enslave eql sl1 57600

  When you want to free a device from its life of slavery, you can
  either down the device with ifconfig (eql will automatically bury the
  dead slave and remove it from its queue) or use eql_emancipate to free
  it. (-- Or just ifconfig it down, and the eql driver will take it out
  for you.--)::

       eql_emancipate eql sl0
       eql_emancipate eql ppp0
       eql_emancipate eql sl1


```
### 3.3. eql 设备DSLIP 配置


  总体思路是自动建立并保持所需的尽可能多的 SLIP 连接

##### 3.3.1.  /etc/slip/runslip.conf


```
	  name		sl-line-1
	  enabled
	  baud		38400
	  mtu		576
	  ducmd		-e /etc/slip/dialout/cua2-288.xp -t 9
	  command	 eql_enslave eql $interface 28800
	  address	 198.67.33.239
	  line		/dev/cua2

	  name		sl-line-2
	  enabled
	  baud		38400
	  mtu		576
	  ducmd		-e /etc/slip/dialout/cua3-288.xp -t 9
	  command	 eql_enslave eql $interface 28800
	  address	 198.67.33.239
	  line		/dev/cua3


```
### 3.4. 使用 PPP eql 设备


  我尚未对 PPP 设备做任何负载均衡测试，主要是因为我没有SLIP DSLIP 那样
  PPP 连接管理器。我确实LinuxNET:Billy 那里得到一个关PPP 性能的好建议  确保asyncmap 设置为某个值，以免控制字符被转义

  我曾95 2 25-26 日那个周末尝试为 eql 驱动搭一套用于重拨丢PPP 连接  PPP 脚本/系统（此后被称为 小时 PPP 痛恨节”）。也许今年晚些时候吧

## 4. 关于从设备调度算

  从设备调度器很可能可以被十几个其他方案替代，从而更快地推送流量。当前驱  配置中的公式经过调优，以处理比特率“优先级”差异巨大的从设备

  我做过的所有测试都使用两个 28.8 V.FC 调制解调器，一个以 28800 bps 或更慢连接，
  另一个始终以 14400 bps 连接

  调度器的一个版本能够在这两28800 14400 连接上推5.3 K/s 的流量，但当
  链路优先级差距很大（57600 vs. 14400）时，“较快”的调制解调器接收了全部流量  而“较慢”的调制解调器则被饿死

## 5. 测试者报

  有些人用更新的内核（1.1.75 新）试验eql 设备。由于旧式“slave-balancing  驱动配置选项被移除，我此后已将驱动更新为可在更新的内核中干净地打补丁

  - LinuxNET icee 在没有任reject 的情况下1.1.86 打上了补丁，并能     启动内核并纳入几ISDN PPP 链路
### 5.1. Randolph Bentson 的测试报

```
    From bentson@grieg.seaslug.org Wed Feb  8 19:08:09 1995
    Date: Tue, 7 Feb 95 22:57 PST
    From: Randolph Bentson <bentson@grieg.seaslug.org>
    To: guru@ncm.com
    Subject: EQL driver tests


    I have been checking out your eql driver.  (Nice work, that!)
    Although you may already done this performance testing, here
    are some data I've discovered.

    Randolph Bentson
    bentson@grieg.seaslug.org

```
------------------------------------------------------------------


  EQL 是一个由 Simon Janes 编写的伪设备驱动，可用于将多SLIP 连接捆绑  看似单一的连接。这让人能够逐步改善拨号网络连接，而无需购买昂贵DSU/CSU
  硬件和服务
  我对该软件做了一些测试，心里有两个目标：一是确认它确实如描述的那样工作  二是作为锻炼我的设备驱动的一种方法
  以下性能测量数据来自在一组两Linux 系统.1.84）之间运行的 SLIP 连接  一端使486DX2/66 Cyclom-8Ys，另一端使486SLC/40 Cyclom-16Y  （使用了端口 0,1,2,3。之后的配置会将端口选择分散到板上的不同 Cirrus 芯片上。）
  一旦建立链路，我对一289284 字节数据的二进制 ftp 传输计时。如果没有任  开销（包头部、字符间与包间延迟等），传输```
      bits/sec	seconds
      345600	8.3
      234600	12.3
      172800	16.7
      153600	18.8
      76800	37.6
      57600	50.2
      38400	75.3
      28800	100.4
      19200	150.6
      9600	301.3

  A single line running at the lower speeds and with large packets
  comes to within 2% of this.  Performance is limited for the higher
  speeds (as predicted by the Cirrus databook) to an aggregate of
  about 160 kbits/sec.	The next round of testing will distribute
  the load across two or more Cirrus chips.

  The good news is that one gets nearly the full advantage of the
  second, third, and fourth line's bandwidth.  (The bad news is
  that the connection establishment seemed fragile for the higher
  speeds.  Once established, the connection seemed robust enough.)

  ======  ========	===  ========   ======= ======= ===
  #lines  speed		mtu  seconds	theory  actual  %of
	  kbit/sec	     duration	speed	speed	max
  ======  ========	===  ========   ======= ======= ===
  3	  115200	900	_	345600
  3	  115200	400	18.1	345600  159825  46
  2	  115200	900	_	230400
  2	  115200	600	18.1	230400  159825  69
  2	  115200	400	19.3	230400  149888  65
  4	  57600		900	_	234600
  4	  57600		600	_	234600
  4	  57600		400	_	234600
  3	  57600		600	20.9	172800  138413  80
  3	  57600		900	21.2	172800  136455  78
  3	  115200	600	21.7	345600  133311  38
  3	  57600		400	22.5	172800  128571  74
  4	  38400		900	25.2	153600  114795  74
  4	  38400		600	26.4	153600  109577  71
  4	  38400		400	27.3	153600  105965  68
  2	  57600		900	29.1	115200  99410.3 86
  1	  115200	900	30.7	115200  94229.3 81
  2	  57600		600	30.2	115200  95789.4 83
  3	  38400		900	30.3	115200  95473.3 82
  3	  38400		600	31.2	115200  92719.2 80
  1	  115200	600	31.3	115200  92423	80
  2	  57600		400	32.3	115200  89561.6 77
  1	  115200	400	32.8	115200  88196.3 76
  3	  38400		400	33.5	115200  86353.4 74
  2	  38400		900	43.7	76800	66197.7 86
  2	  38400		600	44	76800	65746.4 85
  2	  38400		400	47.2	76800	61289	79
  4	  19200		900	50.8	76800	56945.7 74
  4	  19200		400	53.2	76800	54376.7 70
  4	  19200		600	53.7	76800	53870.4 70
  1	  57600		900	54.6	57600	52982.4 91
  1	  57600		600	56.2	57600	51474	89
  3	  19200		900	60.5	57600	47815.5 83
  1	  57600		400	60.2	57600	48053.8 83
  3	  19200		600	62	57600	46658.7 81
  3	  19200		400	64.7	57600	44711.6 77
  1	  38400		900	79.4	38400	36433.8 94
  1	  38400		600	82.4	38400	35107.3 91
  2	  19200		900	84.4	38400	34275.4 89
  1	  38400		400	86.8	38400	33327.6 86
  2	  19200		600	87.6	38400	33023.3 85
  2	  19200		400	91.2	38400	31719.7 82
  4	  9600		900	94.7	38400	30547.4 79
  4	  9600		400	106	38400	27290.9 71
  4	  9600		600	110	38400	26298.5 68
  3	  9600		900	118	28800	24515.6 85
  3	  9600		600	120	28800	24107	83
  3	  9600		400	131	28800	22082.7 76
  1	  19200		900	155	19200	18663.5 97
  1	  19200		600	161	19200	17968	93
  1	  19200		400	170	19200	17016.7 88
  2	  9600		600	176	19200	16436.6 85
  2	  9600		900	180	19200	16071.3 83
  2	  9600		400	181	19200	15982.5 83
  1	  9600		900	305	9600	9484.72 98
  1	  9600		600	314	9600	9212.87 95
  1	  9600		400	332	9600	8713.37 90
  ======  ========	===  ========   ======= ======= ===

```
### 5.2. Anthony Healy 的报

```
    Date: Mon, 13 Feb 1995 16:17:29 +1100 (EST)
    From: Antony Healey <ahealey@st.nepean.uws.edu.au>
    To: Simon Janes <guru@ncm.com>
    Subject: Re: Load Balancing

    Hi Simon,
	  I've installed your patch and it works great. I have trialed
	  it over twin SL/IP lines, just over null modems, but I was
	  able to data at over 48Kb/s [ISDN link -Simon]. I managed a
	  transfer of up to 7.5 Kbyte/s on one go, but averaged around
	  6.4 Kbyte/s, which I think is pretty cool.  :)

```
