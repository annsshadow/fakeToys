## SCSI Interfaces Guide


:Author: James Bottomley
:Author: Rob Landley

## Introduction


### 鍗忚 瀵规瘮 鎬荤嚎


涓€鏃?upon 涓€涓?time, the Small Computer 绯荤粺 鎺ュ彛 瀹氫箟 涓よ€?涓€涓?
骞惰 I/O 鎬荤嚎 鍜?涓€涓?鏁版嵁 鍗忚 鍒?connect 涓€涓?wide variety 鐨?
peripherals (disk drives, tape drives, modems, printers, scanners,
optical drives, test equipment, 鍜?medical 璁惧) 鍒?涓€涓?host computer.

灏界 the 鏃?骞惰 (fast/wide/鑷冲皧鐗? SCSI 鎬荤嚎 鍏锋湁 largely fallen
瓒呭嚭 浣跨敤, the SCSI 鍛戒护 set 鏄?鏇村 widely 浣跨敤 姣?ever 鍒?
communicate 涓?璁惧 鍦ㄢ€︿笂 涓€涓?鏁板瓧 鐨?涓嶅悓 buses.

The `SCSI protocol <https://www.t10.org/scsi-3.htm>`__ 鏄?涓€涓?big-endian
peer-to-peer 鏁版嵁鍖?based 鍗忚. SCSI 鍛戒护 鏄?6, 10, 12, 鎴?16
bytes long, 閫氬父 followed 鐢?涓€涓?associated 鏁版嵁 payload.

SCSI 鍛戒护 鍙?涓?transported 鍦ㄢ€︿笂 just 鍏充簬 浠讳綍 kind 鐨?鎬荤嚎, 鍜?
鏄?the 榛樿 鍗忚 鐢ㄤ簬 storage 璁惧 attached 鍒?USB, SATA, SAS,
Fibre Channel, FireWire, 鍜?ATAPI 璁惧. SCSI packets 鏄?涔?
commonly exchanged 鍦ㄢ€︿笂 Infiniband,
TCP/IP (`iSCSI <https://en.wikipedia.org/wiki/ISCSI>`__), even `骞惰
ports <http://cyberelk.net/tim/parport/parscsi.html>`__.

### Design 鐨?the Linux SCSI 瀛愮郴缁?


The SCSI 瀛愮郴缁?uses 涓€涓?three layer design, 涓?upper, mid, 鍜?low
layers. Every 鎿嶄綔 involving the SCSI 瀛愮郴缁?(渚嬪 reading 涓€涓?
鎵囧尯 鏉ヨ嚜 涓€涓?disk) uses one 椹卞姩 鍦?姣忎釜 鐨?the 3 levels: one upper
layer 椹卞姩, one lower layer 椹卞姩, 鍜?the SCSI midlayer.

The SCSI upper layer 鎻愪緵 the 鎺ュ彛 涔嬮棿 userspace 鍜?the
鍐呮牳, 鍦?the form 鐨?鍧?鍜?char 璁惧 nodes 鐢ㄤ簬 I/O 鍜?ioctl().
The SCSI lower layer 鍖呭惈 椹卞姩 鐢ㄤ簬 鐗瑰畾 纭欢 璁惧.

鍦?涔嬮棿 鏄?the SCSI mid-layer, analogous 鍒?涓€涓?缃戠粶 routing layer
渚嬪 the IPv4 鏍? The SCSI mid-layer routes 涓€涓?鏁版嵁鍖?based 鏁版嵁
鍗忚 涔嬮棿 the upper layer's /dev nodes 鍜?the corresponding
璁惧 鍦?the lower layer. 瀹?manages 鍛戒护 queues, 鎻愪緵 閿欒
handling 鍜?鐢垫簮绠＄悊 鍑芥暟, 鍜?responds 鍒?ioctl()
requests.

## SCSI upper layer


The upper layer supports the user-kernel 鎺ュ彛 鐢?providing 璁惧
nodes.

### sd (SCSI Disk)


sd (sd_mod.o)

### sr (SCSI CD-ROM)


sr (sr_mod.o)

### st (SCSI Tape)


st (st.o)

### sg (SCSI Generic)


sg (sg.o)

### ch (SCSI Media Changer)


ch (ch.c)

## SCSI mid layer


### SCSI midlayer implementation


#### 鍖呭惈/SCSI/SCSI_璁惧.h


   :internal:

#### 椹卞姩/SCSI/SCSI.c


涓昏 鏂囦欢 鐢ㄤ簬 the SCSI midlayer.

   :export:

#### 椹卞姩/SCSI/scsicam.c


`SCSI 閫氱敤 Access
鏂规硶 <http://www.t10.org/ftp/t10/drafts/cam/cam-r12b.pdf>`__ 鏀寔
鍑芥暟, 鐢ㄤ簬 浣跨敤 涓?HDIO_GETGEO, 绛?

   :export:

#### 椹卞姩/SCSI/SCSI_閿欒.c


閫氱敤 SCSI 閿欒/瓒呮椂 handling routines.

   :export:

#### 椹卞姩/SCSI/SCSI_devinfo.c


Manage SCSI_dev_info_鍒楀嚭, 鍏?tracks blacklisted 鍜?whitelisted
璁惧.

   :export:

#### 椹卞姩/SCSI/SCSI_ioctl.c


Handle ioctl() calls 鐢ㄤ簬 SCSI 璁惧.

   :export:

#### 椹卞姩/SCSI/SCSI_lib.c


SCSI queuing 搴?

   :export:

#### 椹卞姩/SCSI/SCSI_lib_dma.c


SCSI 搴?鍑芥暟 depending 鍦?DMA (map 鍜?unmap scatter-gather
鍒楄〃).

   :export:

#### 椹卞姩/SCSI/SCSI_proc.c


The 鍑芥暟 鍦?姝?鏂囦欢 鎻愪緵 涓€涓?鎺ュ彛 涔嬮棿 the PROC 鏂囦欢
绯荤粺 鍜?the SCSI 璁惧 椹卞姩 瀹冩槸 mainly 浣跨敤 鐢ㄤ簬 debugging,
statistics 鍜?鍒?pass information directly 鍒?the lowlevel 椹卞姩. I.E.
plumbing 鍒?manage /proc/SCSI/\*


#### 椹卞姩/SCSI/SCSI_netlink.c


Infrastructure 鍒?鎻愪緵 async 浜嬩欢 鏉ヨ嚜 transports 鍒?userspace 閫氳繃
netlink, 浣跨敤 涓€涓?鍗曚釜 NETLINK_SCSITRANSPORT 鍗忚 鐢ㄤ簬 鍏ㄩ儴
transports. 鍙傝 `the original patch submission
<https://lore.kernel.org/linux-scsi/1155070439.6275.5.camel@localhost.localdomain/>`__
鐢ㄤ簬 鏇村 details.

   :internal:

#### 椹卞姩/SCSI/SCSI_scan.c


Scan 涓€涓?host 鍒?determine 鍏?(鑻?浠讳綍) 璁惧 鏄?attached. The
閫氱敤 scanning/probing algorithm 鏄?浣滀负 follows, exceptions 鏄?made 鍒?
瀹?depending 鍦?璁惧 鐗瑰畾 鏍囧織, compilation 閫夐」, 鍜?鍏ㄥ眬
variable (boot 鎴?妯″潡 鍔犺浇 time) 璁剧疆. 涓€涓?鐗瑰畾 LUN 鏄?scanned
閫氳繃 涓€涓?INQUIRY 鍛戒护; 鑻?the LUN 鍏锋湁 涓€涓?璁惧 attached, 涓€涓?SCSI_璁惧
鏄?allocated 鍜?setup 鐢ㄤ簬 瀹? 鐢ㄤ簬 every id 鐨?every channel 鍦?the
given host, 鍚姩 鐢?scanning LUN 0. Skip hosts 璇?don't respond 鍦?
鍏ㄩ儴 鍒?涓€涓?scan 鐨?LUN 0. 鍚﹀垯, 鑻?LUN 0 鍏锋湁 涓€涓?璁惧 attached,
allocate 鍜?setup 涓€涓?SCSI_璁惧 鐢ㄤ簬 瀹? 鑻?target 鏄?SCSI-3 鎴?up,
issue 涓€涓?REPORT LUN, 鍜?scan 鍏ㄩ儴 鐨?the LUNs returned 鐢?the REPORT LUN;
else, sequentially scan LUNs up 鐩村埌 涓€浜?鏈€澶?鏄?reached, 鎴?涓€涓?LUN
鏄?seen 璇?cannot 鍏锋湁 涓€涓?璁惧 attached 鍒?瀹?

   :export:

#### 椹卞姩/SCSI/SCSI_sysctl.c


Set up the sysctl 鏉＄洰: "/dev/SCSI/logging_level"
(DEV_SCSI_LOGGING_LEVEL) 鍏?sets/returns SCSI_logging_level.

#### 椹卞姩/SCSI/SCSI_sysfs.c


SCSI sysfs 鎺ュ彛 routines.

   :export:

#### 椹卞姩/SCSI/hosts.c


mid 鍒?lowlevel SCSI 椹卞姩 鎺ュ彛

   :export:

#### 椹卞姩/SCSI/SCSI_閫氱敤.c


閫氱敤 鏀寔 鍑芥暟

   :export:

### Transport classes


Transport classes 鏄?service 搴?鐢ㄤ簬 椹卞姩 鍦?the SCSI lower
layer, 鍏?expose transport attributes 鍦?sysfs.

#### Fibre Channel transport


The 鏂囦欢 椹卞姩/SCSI/SCSI_transport_fc.c defines transport attributes
鐢ㄤ簬 Fibre Channel.

   :export:

#### iSCSI transport 绫?


The 鏂囦欢 椹卞姩/SCSI/SCSI_transport_iscsi.c defines transport
attributes 鐢ㄤ簬 the iSCSI 绫? 鍏?sends SCSI packets 鍦ㄢ€︿笂 TCP/IP
connections.

   :export:

#### 涓茶 Attached SCSI (SAS) transport 绫?


The 鏂囦欢 椹卞姩/SCSI/SCSI_transport_sas.c defines transport
attributes 鐢ㄤ簬 涓茶 Attached SCSI, 涓€涓?variant 鐨?SATA aimed 鍦?large
high-end 绯荤粺.

The SAS transport 绫?鍖呭惈 閫氱敤 code 鍒?deal 涓?SAS HBAs, 涓€涓?
approximated representation 鐨?SAS topologies 鍦?the 椹卞姩 鍨嬪彿, 鍜?
鍚勭 sysfs attributes 鍒?expose 杩欎簺 topologies 鍜?绠＄悊
interfaces 鍒?userspace.

姝ゅ 鍒?the 鍩烘湰 SCSI 鏍稿績 objects 姝?transport 绫?
introduces two 棰濆 intermediate objects: The SAS PHY 浣滀负
represented 鐢?缁撴瀯浣?sas_phy defines 涓€涓?"outgoing" PHY 鍦?涓€涓?SAS HBA 鎴?
Expander, 鍜?the SAS remote PHY represented 鐢?缁撴瀯浣?sas_rphy defines
涓€涓?"incoming" PHY 鍦?涓€涓?SAS Expander 鎴?end 璁惧. 娉ㄦ剰 璇?杩欐槸
purely 涓€涓?杞欢 concept, the underlying 纭欢 鐢ㄤ簬 涓€涓?PHY 鍜?涓€涓?
remote PHY 鏄?the exactly the 鐩稿悓.

瀛樺湪 鏃?concept 鐨?涓€涓?SAS 绔彛 鍦?姝?code, users 鍙?鍙傝 浠€涔?PHYs
form 涓€涓?wide 绔彛 鍩轰簬 the 绔彛_identifier attribute, 鍏?鏄?the
鐩稿悓 鐢ㄤ簬 鍏ㄩ儴 PHYs 鍦?涓€涓?绔彛.

   :export:

#### SATA transport 绫?


The SATA transport 鏄?handled 鐢?libata, 鍏?鍏锋湁 鍏?own book 鐨?
documentation 鍦?姝?directory.

#### 骞惰 SCSI (SPI) transport 绫?


The 鏂囦欢 椹卞姩/SCSI/SCSI_transport_spi.c defines transport
attributes 鐢ㄤ簬 traditional (fast/wide/鑷冲皧鐗? SCSI buses.

   :export:

#### SCSI RDMA (SRP) transport 绫?


The 鏂囦欢 椹卞姩/SCSI/SCSI_transport_srp.c defines transport
attributes 鐢ㄤ簬 SCSI 鍦ㄢ€︿笂 Remote Direct 鍐呭瓨 Access.

   :export:

## SCSI lower layer


### Host 鎬荤嚎 Adapter transport types


璁稿 modern 璁惧 鎺у埗鍣?浣跨敤 the SCSI 鍛戒护 set 浣滀负 涓€涓?鍗忚 鍒?
communicate 涓?瀹冧滑鐨?璁惧 through 璁稿 涓嶅悓 types 鐨?鐗╃悊
connections.

鍦?SCSI language 涓€涓?鎬荤嚎 capable 鐨?carrying SCSI 鍛戒护 鏄?called 涓€涓?
"transport", 鍜?涓€涓?鎺у埗鍣?connecting 鍒?姝ょ被 涓€涓?鎬荤嚎 鏄?called 涓€涓?"host
鎬荤嚎 adapter" (HBA).

#### Debug transport


The 鏂囦欢 椹卞姩/SCSI/SCSI_debug.c simulates 涓€涓?host adapter 涓?涓€涓?
variable 鏁板瓧 鐨?disks (鎴?disk 绫讳技 璁惧) attached, sharing 涓€涓?
閫氱敤 amount 鐨?RAM. 鎵ц 涓€涓?lot 鐨?checking 鍒?纭繚 璇?鎴戜滑 鏄?
涓?getting 鍧?mixed up, 鍜?panics the 鍐呮牳 鑻?anything 瓒呭嚭
the ordinary 鏄?seen.

鍒?涓?鏇村 realistic, the simulated 璁惧 鍏锋湁 the transport
attributes 鐨?SAS disks.

鐢ㄤ簬 documentation 鍙傝 http://sg.danny.cz/sg/scsi_debug.html

#### todo


骞惰 (fast/wide/鑷冲皧鐗? SCSI, USB, SATA, SAS, Fibre Channel,
FireWire, ATAPI 璁惧, Infiniband, 骞惰 ports,
netlink...
