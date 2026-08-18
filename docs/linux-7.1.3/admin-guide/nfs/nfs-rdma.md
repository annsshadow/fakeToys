## 璁剧疆 NFS/RDMA


:Author:
  NetApp and Open Grid Computing (May 29, 2008)

  鏈枃妗ｅ彲鑳藉凡缁忚繃鏃躲€?
## 姒傝堪


鏈枃妗ｆ弿杩颁簡濡備綍瀹夎鍜岄厤缃?Linux NFS/RDMA 瀹㈡埛绔笌鏈嶅姟绔蒋浠躲€?
NFS/RDMA 瀹㈡埛绔娆″寘鍚湪 Linux 2.6.24 涓€侼FS/RDMA 鏈嶅姟绔娆″寘鍚湪姝ゅ悗鐨勪笅涓€涓増鏈?Linux 2.6.25 涓€?
鍦ㄦ垜浠殑娴嬭瘯涓紝鍦ㄥ绉嶅伐浣滆礋杞戒笅閮借幏寰椾簡鍑鸿壊鐨勬€ц兘缁撴灉锛堝湪瀹㈡埛绔?CPU 鍗犵敤鏋佷綆鐨勬儏鍐典笅杈惧埌婊?10Gbit 绾胯矾甯﹀锛夈€傝浠ｇ爜閫氳繃浜嗗畬鏁寸殑 Connectathon 娴嬭瘯濂椾欢锛屽苟涓斿彲浠ュ湪 Infiniband 涓?iWARP 涓ょ RDMA 閫傞厤鍣ㄤ笂杩愯銆?
## 鑾峰彇甯姪


濡傛灉浣犻亣鍒颁簡鍥伴毦锛屽彲浠ュ湪 nfs-rdma-devel@lists.sourceforge.net 閭欢鍒楄〃涓婃彁闂€?
## 瀹夎


浠ヤ笅璇存槑鏄瀯寤轰竴鍙扮敤浜?NFS/RDMA 鐨勬満鍣ㄧ殑鍒嗘鎸囧崡銆?
- 瀹夎 RDMA 璁惧

  鍙鏄?drivers/infiniband/hw 涓殑椹卞姩鎵€鏀寔鐨勪换浣曡澶囬兘鍙互銆?
  鎴戜滑宸茬粡浣跨敤澶氫釜鍩轰簬 Mellanox 鐨?IB 缃戝崱銆丄mmasso AMS1100 iWARP 閫傞厤鍣ㄤ互鍙?Chelsio cxgb3 iWARP 閫傞厤鍣ㄨ繘琛屼簡娴嬭瘯銆?
- 瀹夎 Linux 鍙戣鐗堝強宸ュ叿

  棣栦釜鍚屾椂鍖呭惈 NFS/RDMA 瀹㈡埛绔笌鏈嶅姟绔殑 kernel 鐗堟湰鏄?Linux 2.6.25锛屽洜姝ゅ簲褰撳畨瑁呬笌姝ゅ強鍚庣画 Linux 鍐呮牳鐗堟湰鍏煎鐨勫彂琛岀増銆?
  鏈枃妗ｆ弿杩扮殑姝ラ宸插湪 Red Hat 鐨?Fedora Project锛坔ttp://fedora.redhat.com/锛夊彂琛岀増涓婃祴璇曡繃銆?
- 鍦ㄥ鎴风涓婂畨瑁?nfs-utils-1.1.2 鎴栨洿楂樼増鏈?
  浣跨敤 nfs-utils-1.1.2 鎴栨洿楂樼増鏈腑鐨?mount.nfs 鍛戒护锛坣fs-utils-1.1.1 鏄涓敮鎸?NFS/RDMA 鎸傝浇鐨?nfs-utils 鐗堟湰锛屼絾鍑轰簬鍚勭鍘熷洜鎴戜滑寤鸿浣跨敤 nfs-utils-1.1.2 鎴栨洿楂樼増鏈級鍗冲彲鑾峰緱 NFS/RDMA 鎸傝浇鐐广€傝鏌ョ湅浣犳鍦ㄤ娇鐢ㄧ殑 mount.nfs 鐗堟湰锛岃杈撳叆锛?
  .. code-block:: sh

    $ /sbin/mount.nfs -V

  濡傛灉鐗堟湰浣庝簬 1.1.2 鎴栬€呰鍛戒护涓嶅瓨鍦紝浣犲簲褰撳畨瑁呮渶鏂扮増鏈殑 nfs-utils銆?
  浠庝互涓嬪湴鍧€涓嬭浇鏈€鏂扮殑杞欢鍖咃細https://www.kernel.org/pub/linux/utils/nfs

  瑙ｅ帇璇ヨ蒋浠跺寘骞舵寜鐓у畨瑁呰鏄庤繘琛屾搷浣溿€?
  濡傛灉浣犱笉闇€瑕?idmapper 鍜?gssd 鍙墽琛屾枃浠讹紙鍒涘缓鍚敤 NFS/RDMA 鐨勬寕杞藉懡浠ゅ苟涓嶉渶瑕佸畠浠級锛屽垯鍙互鍦ㄨ繍琛?configure 鏃剁鐢ㄨ繖浜涚壒鎬ф潵绠€鍖栧畨瑁呰繃绋嬶細

  .. code-block:: sh

    $ ./configure --disable-gss --disable-nfsv4

  瑕佹瀯寤?nfs-utils锛屼綘闇€瑕佸畨瑁?tcp_wrappers 杞欢鍖呫€傛湁鍏虫洿澶氫俊鎭紝璇峰弬闃呰杞欢鍖呯殑 README 鍜?INSTALL 鏂囦欢銆?
  鏋勫缓 nfs-utils 杞欢鍖呭悗锛屽湪 utils/mount 鐩綍涓嬩細鏈変竴涓?mount.nfs 浜岃繘鍒舵枃浠躲€傝浜岃繘鍒舵枃浠跺彲鐢ㄤ簬鍙戣捣 NFS v2銆乿3 鎴?v4 鎸傝浇銆傝鍙戣捣 v4 鎸傝浇锛岃浜岃繘鍒舵枃浠跺繀椤昏鍛藉悕涓?mount.nfs4銆傛爣鍑嗗仛娉曟槸灏嗕竴涓悕涓?mount.nfs4 鐨勭鍙烽摼鎺ユ寚鍚?mount.nfs銆?
  搴斿綋灏嗚 mount.nfs 浜岃繘鍒舵枃浠舵寜濡備笅鏂瑰紡瀹夎鍒?/sbin/mount.nfs锛?
  .. code-block:: sh

    $ sudo cp utils/mount/mount.nfs /sbin/mount.nfs

  鍦ㄨ浣嶇疆锛岀郴缁?mount 鍛戒护浼氳嚜鍔ㄨ皟鐢?mount.nfs 鏉ヨ繘琛?NFS 鎸傝浇銆?
```
      mount.nfs 浠ュ強 nfs-utils-1.1.2 鎴栨洿楂樼増鏈彧闇€瑕佸湪 NFS 瀹㈡埛绔満鍣ㄤ笂瀹夎銆?      鏈嶅姟绔笂骞朵笉闇€瑕佽繖涓壒瀹氱増鏈殑 nfs-utils銆傛澶栵紝瀹㈡埛绔笂鍙渶瑕?nfs-utils-1.1.2
      涓殑 mount.nfs 鍛戒护銆?
```
- 瀹夎甯︽湁 NFS/RDMA 鐨?Linux 鍐呮牳

  NFS/RDMA 瀹㈡埛绔笌鏈嶅姟绔兘鍖呭惈鍦ㄤ富绾?Linux 鍐呮牳鐗堟湰 2.6.25 鍙婁箣鍚庛€傛鐗堟湰鍙婂叾浠栫増鏈殑 Linux 鍐呮牳鍙湪浠ヤ笅鍦板潃鑾峰彇锛歨ttps://www.kernel.org/pub/linux/kernel/

  涓嬭浇婧愮爜骞跺皢鍏舵斁缃埌鍚堥€傜殑浣嶇疆銆?
- 閰嶇疆 RDMA 鏍?
  纭繚浣犵殑鍐呮牳閰嶇疆宸插惎鐢?RDMA 鏀寔銆傚湪 Device Drivers -> InfiniBand support 涓嬶紝鏇存柊鍐呮牳閰嶇疆浠ュ惎鐢?InfiniBand support [娉ㄦ剰锛氳閫夐」鍚嶇О鍏锋湁璇鎬с€傚惎鐢?InfiniBand support 瀵逛簬鎵€鏈?RDMA 璁惧锛圛B銆乮WARP 绛夛級閮芥槸蹇呴渶鐨刔銆?
  鍚敤鐩稿簲鐨?IB HCA 鏀寔锛坢lx4銆乵thca銆乪hca銆乮path 绛夛級鎴?iWARP 閫傞厤鍣ㄦ敮鎸侊紙amso銆乧xgb3 绛夛級銆?
  濡傛灉浣犱娇鐢ㄧ殑鏄?InfiniBand锛岃鍔″繀鍚敤 IP-over-InfiniBand 鏀寔銆?
- 閰嶇疆 NFS 瀹㈡埛绔笌鏈嶅姟绔?
  浣犵殑鍐呮牳閰嶇疆杩樺繀椤诲惎鐢?NFS 鏂囦欢绯荤粺鏀寔鍜?鎴?NFS 鏈嶅姟绔敮鎸併€傝繖浜涗互鍙婂叾瀹?NFS 鐩稿叧鐨勯厤缃€夐」鍙互鍦?File Systems -> Network File Systems 涓嬫壘鍒般€?
- 鏋勫缓銆佸畨瑁呫€侀噸鍚?
  濡傛灉 NFS 鍜?RDMA 鍧囧凡寮€鍚紝NFS/RDMA 浠ｇ爜灏嗚嚜鍔ㄥ惎鐢ㄣ€侼FS/RDMA 瀹㈡埛绔笌鏈嶅姟绔槸閫氳繃渚濊禆浜?SUNRPC 鍜?INFINIBAND 鐨勯殣钘忛厤缃€夐」 SUNRPC_XPRT_RDMA 杩涜閰嶇疆鐨勩€係UNRPC_XPRT_RDMA 鐨勫€煎皢涓猴細

    #. 濡傛灉 SUNRPC 鎴?INFINIBAND 浠讳竴涓?N锛屽垯鍊间负 N锛屾鏃?NFS/RDMA 瀹㈡埛绔笌鏈嶅姟绔皢涓嶄細琚瀯寤?
    #. 濡傛灉 SUNRPC 鍜?INFINIBAND 閮藉凡寮€鍚紙M 鎴?Y锛変笖鑷冲皯鏈変竴涓负 M锛屽垯鍊间负 M锛屾鏃?NFS/RDMA 瀹㈡埛绔笌鏈嶅姟绔皢琚瀯寤轰负妯″潡

    #. 濡傛灉 SUNRPC 鍜?INFINIBAND 閮戒负 Y锛屽垯鍊间负 Y锛屾鏃?NFS/RDMA 瀹㈡埛绔笌鏈嶅姟绔皢琚瀯寤鸿繘鍐呮牳

  鍥犳锛屽鏋滀綘宸叉寜鐓т笂杩版楠ゅ紑鍚?NFS 鍜?RDMA锛孨FS/RDMA 瀹㈡埛绔笌鏈嶅姟绔氨浼氳鏋勫缓銆?
  鏋勫缓鏂板唴鏍革紝瀹夎瀹冿紝骞跺惎鍔ㄥ畠銆?
## 妫€鏌?RDMA 涓?NFS 鐨勫畨瑁?

鍦ㄩ厤缃?NFS/RDMA 杞欢涔嬪墠锛屾祴璇曚竴涓嬩綘鐨勬柊鍐呮牳浠ョ‘淇濆叾宸ヤ綔姝ｅ父鏄釜濂戒富鎰忋€傜壒鍒槸锛岄獙璇?RDMA 鏍堟槸鍚︽寜棰勬湡杩愯锛屼互鍙婂熀浜?TCP/IP 鍜?鎴?UDP/IP 鐨勬爣鍑?NFS 鏄惁姝ｅ父宸ヤ綔锛岄兘鏄ソ鍋氭硶銆?
- 妫€鏌?RDMA 瀹夎

  濡傛灉浣犲皢 RDMA 缁勪欢鏋勫缓涓烘ā鍧楋紝姝ゆ椂鍔犺浇瀹冧滑銆備緥濡傦紝濡傛灉浣犱娇鐢ㄧ殑鏄?Mellanox Tavor/Sinai/Arbel 缃戝崱锛?
  .. code-block:: sh

    $ modprobe ib_mthca
    $ modprobe ib_ipoib

  濡傛灉浣犱娇鐢ㄧ殑鏄?InfiniBand锛岃纭繚缃戠粶涓婃鍦ㄨ繍琛屼竴涓瓙缃戠鐞嗗櫒锛圫M锛夈€傚鏋滀綘鐨?IB 浜ゆ崲鏈哄甫鏈夊唴宓岀殑 SM锛屽彲浠ヤ娇鐢ㄥ畠銆傚惁鍒欙紝浣犲皢闇€瑕佸湪鏌愪釜缁堢鑺傜偣涓婅繍琛屼竴涓?SM锛屼緥濡?OpenSM銆?
  濡傛灉浣犵殑缃戠粶涓婅繍琛岀潃 SM锛屼綘搴旇鐪嬪埌濡備笅杈撳嚭锛?
  .. code-block:: sh

    $ cat /sys/class/infiniband/driverX/ports/1/state
    4: ACTIVE

  鍏朵腑 driverX 涓?mthca0銆乮path5銆乪hca3 绛夈€?
  瑕佽繘涓€姝ユ祴璇?InfiniBand 杞欢鏍堬紝鍙互浣跨敤 IPoIB锛堣繖鍋囪浣犳湁涓ゅ彴鍚嶄负 host1 鍜?host2 鐨?IB 涓绘満锛夛細

  .. code-block:: sh

    host1$ ip link set dev ib0 up
    host1$ ip address add dev ib0 a.b.c.x
    host2$ ip link set dev ib0 up
    host2$ ip address add dev ib0 a.b.c.y
    host1$ ping a.b.c.y
    host2$ ping a.b.c.x

  瀵逛簬鍏跺畠璁惧绫诲瀷锛岃閬靛惊鐩稿簲鐨勬楠ゃ€?
- 妫€鏌?NFS 瀹夎

  瀵逛簬涓婇潰鍚敤鐨?NFS 缁勪欢锛堝鎴风鍜?鎴栨湇鍔＄锛夛紝鍦ㄦ爣鍑嗕互澶綉锛堜娇鐢?TCP/IP 鎴?UDP/IP锛変笂娴嬭瘯瀹冧滑鐨勫姛鑳姐€?
## NFS/RDMA 閰嶇疆


鎴戜滑寤鸿浣犱娇鐢ㄤ袱鍙版満鍣紝涓€鍙颁綔涓哄鎴风锛屼竴鍙颁綔涓烘湇鍔＄銆?
### 涓€娆℃€ч厤缃細


- 鍦ㄦ湇鍔＄绯荤粺涓婏紝閰嶇疆 /etc/exports 鏂囦欢骞跺惎鍔?NFS/RDMA 鏈嶅姟绔€?
```
  /vol0   192.168.0.47(fsid=0,rw,async,insecure,no_root_squash)
  /vol0   192.168.0.0/255.255.255.0(fsid=0,rw,async,insecure,no_root_squash)

  IP 鍦板潃鏄鎴风鐨?IPoIB 鍦板潃锛堝浜?InfiniBand HCA锛夋垨瀹㈡埛绔殑 iWARP 鍦板潃锛堝浜?RNIC锛夈€?
  .. note::
    蹇呴』浣跨敤 "insecure" 閫夐」锛屽洜涓?NFS/RDMA 瀹㈡埛绔笉浣跨敤淇濈暀绔彛銆?
```
### 姣忔寮€鏈烘椂锛?

- 鍔犺浇骞堕厤缃?RDMA 椹卞姩

  瀵逛簬浣跨敤 Mellanox 閫傞厤鍣ㄧ殑 InfiniBand锛?
  .. code-block:: sh

    $ modprobe ib_mthca
    $ modprobe ib_ipoib
    $ ip li set dev ib0 up
    $ ip addr add dev ib0 a.b.c.d

```
    璇蜂负瀹㈡埛绔笌鏈嶅姟绔娇鐢ㄥ敮涓€鐨勫湴鍧€锛?
```
- 鍚姩 NFS 鏈嶅姟绔?
  濡傛灉 NFS/RDMA 鏈嶅姟绔鏋勫缓涓烘ā鍧楋紙鍐呮牳閰嶇疆涓?CONFIG_SUNRPC_XPRT_RDMA=m锛夛紝鍒欏姞杞?RDMA 浼犺緭妯″潡锛?
  .. code-block:: sh

    $ modprobe svcrdma

  鏃犺鏈嶅姟绔互浣曠鏂瑰紡鏋勫缓锛堟ā鍧楁垨鍐呭缓锛夛紝鍚姩鏈嶅姟绔細

  .. code-block:: sh

    $ /etc/init.d/nfs start

  鎴?
  .. code-block:: sh

    $ service nfs start

  鎸囩ず鏈嶅姟绔洃鍚?RDMA 浼犺緭锛?
  .. code-block:: sh

    $ echo rdma 20049 > /proc/fs/nfsd/portlist

- 鍦ㄥ鎴风绯荤粺涓?
  濡傛灉 NFS/RDMA 瀹㈡埛绔鏋勫缓涓烘ā鍧楋紙鍐呮牳閰嶇疆涓?CONFIG_SUNRPC_XPRT_RDMA=m锛夛紝鍔犺浇 RDMA 瀹㈡埛绔ā鍧楋細

  .. code-block:: sh

    $ modprobe xprtrdma.ko

  鏃犺瀹㈡埛绔互浣曠鏂瑰紡鏋勫缓锛堟ā鍧楁垨鍐呭缓锛夛紝浣跨敤浠ヤ笅鍛戒护鎸傝浇 NFS/RDMA 鏈嶅姟绔細

  .. code-block:: sh

    $ mount -o rdma,port=20049 <IPoIB-server-name-or-address>:/<export> /mnt

  瑕侀獙璇佽鎸傝浇鏄惁姝ｅ湪浣跨敤 RDMA锛岃杩愯 "cat /proc/mounts" 骞舵鏌ヨ鎸傝浇鐨?"proto" 瀛楁銆?
  鎭枩锛佷綘姝ｅ湪浣跨敤 NFS/RDMA锛?