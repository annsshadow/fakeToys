
## Switchdev


:Copyright: |copy| 2023, NVIDIA CORPORATION & AFFILIATES. All rights reserved.


## 妗ユ帴鍗歌浇锛圔ridge offload锛?

mlx5 椹卞姩鍦?switchdev 妯″紡涓嬪疄鐜颁簡瀵规ˉ鎺ヨ鍒欏嵏杞界殑鏀寔銆傚綋 mlx5 switchdev
representor锛堜唬琛ㄧ鍙ｏ級琚寕鎺ュ埌妗ワ紙bridge锛夋椂锛孡inux 妗ョ殑 FDB 浼氳鑷姩鍗歌浇銆?
```

    $ devlink dev eswitch set pci/0000:06:00.0 mode switchdev

```
```

    $ ip link set enp8s0f0 master bridge1

```
### VLAN


mlx5 鏀寔浠ヤ笅妗ユ帴 VLAN 鍔熻兘锛?
```

    $ ip link set bridge1 type bridge vlan_filtering 1
    $ bridge vlan add dev enp8s0f0 vid 2-3

```
```

    $ bridge vlan add dev enp8s0f0 vid 3 pvid

```
```

    $ bridge vlan add dev enp8s0f0 vid 3 untagged

```
## 瀛愬姛鑳斤紙Subfunction锛?

閫氳繃 E-switch 鐢熸垚鐨勫瓙鍔熻兘锛圫ubfunction锛変粎閫氳繃 devlink 璁惧鍒涘缓锛岄粯璁ゆ儏鍐典笅鎵€鏈?SF 杈呭姪璁惧閮芥槸绂佺敤鐨勩€傝繖灏嗗厑璁哥敤鎴峰湪 SF 琚畬鍏ㄦ帰娴嬶紙probe锛変箣鍓嶅鍏惰繘琛岄厤缃紝浠庤€?鑺傜渷鏃堕棿銆?
浣跨敤绀轰緥锛?
```

    $ devlink port add pci/0000:08:00.0 flavour pcisf pfnum 0 sfnum 11
    $ devlink port function set pci/0000:08:00.0/32768 hw_addr 00:00:00:00:00:11 state active

```
```

    $ devlink dev param set auxiliary/mlx5_core.sf.1 name enable_eth value true cmode driverinit

```
```

    $ devlink dev reload auxiliary/mlx5_core.sf.1

```
mlx5 鏀寔 ETH銆乺dma 涓?vdpa锛坴net锛夎緟鍔╄澶囩殑 devlink 鍙傛暟锛堝弬瑙?Documentation/networking/devlink/devlink-params.rst <devlink_params_generic>锛夈€?
mlx5 鏀寔浣跨敤 devlink port锛堝弬瑙?Documentation/networking/devlink/devlink-port.rst <devlink_port>锛夋帴鍙ｇ鐞嗗瓙鍔熻兘銆?
瀛愬姛鑳芥嫢鏈夎嚜宸辩殑鍔熻兘鑳藉姏浠ュ強鑷繁鐨勮祫婧愩€傝繖鎰忓懗鐫€瀛愬姛鑳芥嫢鏈夎嚜宸辩殑涓撶敤闃熷垪锛坱xq銆乺xq銆乧q銆乪q锛夈€?杩欎簺闃熷垪鏃笉涓庣埗 PCI 鍔熻兘锛坧arent PCI function锛夊叡浜紝涔熶笉浼氫粠鐖?PCI 鍔熻兘澶勭獌鍙栥€?
褰撳瓙鍔熻兘鍏峰 RDMA 鑳藉姏鏃讹紝瀹冩嫢鏈夎嚜宸辩殑 QP1銆丟ID 琛紝浠ュ強 RDMA 璧勬簮锛屾棦涓嶄笌鐖?PCI 鍔熻兘
鍏变韩锛屼篃涓嶄細浠庡叾绐冨彇銆?
瀛愬姛鑳藉湪 PCI BAR 绌洪棿涓嫢鏈変竴涓笓鐢ㄧ殑绐楀彛锛岃绐楀彛涓嶄笌鍏跺畠瀛愬姛鑳芥垨鐖?PCI 鍔熻兘鍏变韩銆傝繖纭繚浜?瀛愬姛鑳界殑鎵€鏈夎澶囷紙netdev銆乺dma銆乿dpa 绛夛級鍙闂鍒嗛厤鐨?PCI BAR 绌洪棿銆?
瀛愬姛鑳芥敮鎸?eswitch 琛ㄧず锛坮epresentation锛夛紝骞跺€熸鏀寔 tc 鍗歌浇銆傜敤鎴烽厤缃?eswitch 浠ュ悜/浠?瀛愬姛鑳界鍙ｅ彂閫?鎺ユ敹鏁版嵁鍖呫€?
瀛愬姛鑳戒笌鐖?PCI 鍔熻兘鍙婂叾瀹冨瓙鍔熻兘鍏变韩 PCI 绾у埆鐨勮祫婧愶紝渚嬪 PCI MSI-X IRQ銆?
```

       _______
      | admin |
      | user  |----------
      |_______|         |
          |             |
      ____|____       __|______            _________________
     |         |     |         |          |                 |
     | devlink |     | tc tool |          |    user         |
     | tool    |     |_________|          | applications    |
     |_________|         |                |_________________|
           |             |                   |          |
           |             |                   |          |         Userspace
 +---------|-------------|-------------------|----------|--------------------+
           |             |           +----------+   +----------+   Kernel
           |             |           |  netdev  |   | rdma dev |
           |             |           +----------+   +----------+
   (devlink port add/del |              ^               ^
    port function set)   |              |               |
           |             |              +---------------|
      _____|___          |              |        _______|_______
     |         |         |              |       | mlx5 class    |
     | devlink |   +------------+       |       |   drivers     |
     | kernel  |   | rep netdev |       |       |(mlx5_core,ib) |
     |_________|   +------------+       |       |_______________|
           |             |              |               ^
   (devlink ops)         |              |          (probe/remove)
  _________|________     |              |           ____|________
 | subfunction      |    |     +---------------+   | subfunction |
 | management driver|-----     | subfunction   |---|  driver     |
 | (mlx5_core)      |          | auxiliary dev |   | (mlx5_core) |
 |__________________|          +---------------+   |_____________|
           |                                            ^
  (sf add/del, vhca events)                             |
           |                                      (device add/del)
      _____|____                                    ____|________
     |          |                                  | subfunction |
     |  PCI NIC |--- activate/deactivate events--->| host driver |
     |__________|                                  | (mlx5_core) |
                                                   |_____________|

```
瀛愬姛鑳介€氳繃 devlink port 鎺ュ彛鍒涘缓銆?
```

    $ devlink dev eswitch set pci/0000:06:00.0 mode switchdev

```
```

    $ devlink port add pci/0000:06:00.0 flavour pcisf pfnum 0 sfnum 88
    pci/0000:06:00.0/32768: type eth netdev eth6 flavour pcisf controller 0 pfnum 0 sfnum 88 external false splittable false
      function:
        hw_addr 00:00:00:00:00:00 state inactive opstate detached

```
```

    $ devlink port show pci/0000:06:00.0/32768
    pci/0000:06:00.0/32768: type eth netdev enp6s0pf0sf88 flavour pcisf pfnum 0 sfnum 88
      function:
        hw_addr 00:00:00:00:00:00 state inactive opstate detached

```
```

    $ devlink port del pci/0000:06:00.0/32768

```
## 鍔熻兘灞炴€?

mlx5 椹卞姩鎻愪緵浜嗕竴绉嶆満鍒讹紝浠ョ粺涓€鐨勬柟寮忎负 SmartNIC 涓庨潪 SmartNIC 璁剧疆 PCI VF/SF 鍔熻兘灞炴€с€?
杩欎粎鍦?eswitch 妯″紡璁剧疆涓?switchdev 鏃舵墠鍙楁敮鎸併€侾CI VF/SF 鐨勭鍙ｅ姛鑳介厤缃€氳繃 devlink
eswitch port 鏀寔銆?
绔彛鍔熻兘灞炴€у簲鍦?PCI VF/SF 琚┍鍔ㄦ灇涓句箣鍓嶈缃€?
### MAC 鍦板潃璁剧疆


mlx5 椹卞姩鏀寔 devlink port function attr 鏈哄埗鏉ヨ缃?MAC 鍦板潃銆傦紙鍙傝 Documentation/networking/devlink/devlink-port.rst锛?
#### RoCE 鑳藉姏璁剧疆


骞堕潪鎵€鏈?mlx5 PCI 璁惧/SF 閮介渶瑕?RoCE 鑳藉姏銆?
褰?RoCE 鑳藉姏琚鐢ㄦ椂锛屾瘡涓?PCI 璁惧/SF 鍙妭鐪?1 Mbytes 鐨勭郴缁熷唴瀛樸€?
mlx5 椹卞姩鏀寔 devlink port function attr 鏈哄埗鏉ヨ缃?RoCE 鑳藉姏銆傦紙鍙傝 Documentation/networking/devlink/devlink-port.rst锛?
#### 鍙縼绉伙紙migratable锛夎兘鍔涜缃?

甯屾湜 mlx5 PCI VF 鑳藉杩涜瀹炴椂杩佺Щ锛坙ive migration锛夌殑鐢ㄦ埛锛岄渶瑕佹樉寮忓湴鍚敤 VF 鐨勫彲杩佺Щ鑳藉姏銆?
mlx5 椹卞姩鏀寔 devlink port function attr 鏈哄埗鏉ヨ缃彲杩佺Щ鑳藉姏銆傦紙鍙傝 Documentation/networking/devlink/devlink-port.rst锛?
#### IPsec crypto 鑳藉姏璁剧疆


甯屾湜 mlx5 PCI VF 鑳藉杩涜 IPsec crypto 鍗歌浇鐨勭敤鎴凤紝闇€瑕佹樉寮忓湴鍚敤 VF 鐨?ipsec_crypto 鑳藉姏銆?浠?ConnectX6dx 鍙婁互涓婅澶囧紑濮嬫敮鎸佷负 VF 鍚敤 IPsec 鑳藉姏銆傚綋 VF 鍚敤浜?IPsec 鑳藉姏鏃讹紝PF 涓婄殑浠讳綍
IPsec 鍗歌浇閮戒細琚樆濉炪€?
mlx5 椹卞姩鏀寔 devlink port function attr 鏈哄埗鏉ヨ缃?ipsec_crypto 鑳藉姏銆傦紙鍙傝 Documentation/networking/devlink/devlink-port.rst锛?
#### IPsec packet 鑳藉姏璁剧疆


甯屾湜 mlx5 PCI VF 鑳藉杩涜 IPsec packet 鍗歌浇鐨勭敤鎴凤紝闇€瑕佹樉寮忓湴鍚敤 VF 鐨?ipsec_packet 鑳藉姏銆?浠?ConnectX6dx 鍙婁互涓婅澶囧紑濮嬫敮鎸佷负 VF 鍚敤 IPsec 鑳藉姏銆傚綋 VF 鍚敤浜?IPsec 鑳藉姏鏃讹紝PF 涓婄殑浠讳綍
IPsec 鍗歌浇閮戒細琚樆濉炪€?
mlx5 椹卞姩鏀寔 devlink port function attr 鏈哄埗鏉ヨ缃?ipsec_packet 鑳藉姏銆傦紙鍙傝 Documentation/networking/devlink/devlink-port.rst锛?
### SF 鐘舵€佽缃?

瑕佷娇鐢?SF锛岀敤鎴峰繀椤婚€氳繃 SF 鍔熻兘鐘舵€侊紙function state锛夊睘鎬ф潵婵€娲?SF銆?
```

   $ devlink port show ens2f0npf0sf88
   pci/0000:06:00.0/32768: type eth netdev ens2f0npf0sf88 flavour pcisf controller 0 pfnum 0 sfnum 88 external false splittable false
     function:
       hw_addr 00:00:00:00:88:88 state inactive opstate detached

```
```

   $ devlink port function set ens2f0npf0sf88 state active

   $ devlink port show ens2f0npf0sf88
   pci/0000:06:00.0/32768: type eth netdev ens2f0npf0sf88 flavour pcisf controller 0 pfnum 0 sfnum 88 external false splittable false
     function:
       hw_addr 00:00:00:00:88:88 state active opstate detached

```
鍔熻兘婵€娲诲悗锛孭F 椹卞姩瀹炰緥浼氫粠璁惧鏀跺埌鏌愪釜鐗瑰畾 SF 宸茶婵€娲荤殑浜嬩欢銆傝繖鏄皢璇ヨ澶囨斁鍒版€荤嚎涓娿€?瀵瑰叾杩涜鎺㈡祴锛坧robe锛夊苟涓哄叾瀹炰緥鍖?devlink 瀹炰緥浠ュ強绫荤壒瀹氱殑杈呭姪璁惧鐨勪俊鍙枫€?
```

    $ devlink dev show
    devlink dev show auxiliary/mlx5_core.sf.4

    $ devlink port show auxiliary/mlx5_core.sf.4/1
    auxiliary/mlx5_core.sf.4/1: type eth netdev p0sf88 flavour virtual port 0 splittable false

    $ rdma link show mlx5_0/1
    link mlx5_0/1 state ACTIVE physical_state LINK_UP netdev p0sf88

    $ rdma dev show
    8: rocep6s0f1: node_type ca fw 16.29.0550 node_guid 248a:0703:00b3:d113 sys_image_guid 248a:0703:00b3:d112
    13: mlx5_0: node_type ca fw 16.29.0550 node_guid 0000:00ff:fe00:8888 sys_image_guid 248a:0703:00b3:d112

```
```

                 mlx5_core.sf.4
          (subfunction auxiliary device)
                       /\
                      /  \
                     /    \
                    /      \
                   /        \
      mlx5_core.eth.4     mlx5_core.rdma.4
     (sf eth aux dev)     (sf rdma aux dev)
         |                      |
         |                      |
      p0sf88                  mlx5_0
     (sf netdev)          (sf rdma device)

```
姝ゅ锛屽綋椹卞姩鎸傛帴鍒板瓙鍔熻兘鐨勮緟鍔╄澶囨椂锛孲F 绔彛涔熶細鏀跺埌璇ヤ簨浠躲€傝繖浼氭敼鍙樺姛鑳界殑鎿嶄綔锛坥perational锛?鐘舵€併€傝繖璁╃敤鎴疯兘澶熷垽鏂綍鏃跺彲浠ュ畨鍏ㄥ湴鍒犻櫎 SF 绔彛锛屼互瀹炵幇瀛愬姛鑳界殑浼橀泤缁堟銆?
```

    $ devlink port show ens2f0npf0sf88
    pci/0000:06:00.0/32768: type eth netdev ens2f0npf0sf88 flavour pcisf controller 0 pfnum 0 sfnum 88 external false splittable false
      function:
        hw_addr 00:00:00:00:88:88 state active opstate attached

```
