
## Enabling the driver and kconfig options


:Copyright: |copy| 2023, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

| mlx5 core 鏄ā鍧楀寲鐨勶紝澶у鏁颁富瑕佺殑 mlx5 core 椹卞姩鐗规€ч兘鍙互鍦ㄦ瀯寤烘椂閫氳繃鍐呮牳 Kconfig 鏍囧織杩涜閫夋嫨锛堢紪璇戣繘鍐呮牳鎴栨帓闄わ級銆?| 鍩烘湰鐗规€с€佷互澶綉缃戠粶璁惧 rx/tx 鍗歌浇鍜?XDP锛屽湪浣跨敤鏈€鍩烘湰鐨勬爣蹇?| CONFIG_MLX5_CORE=y/m 鍜?CONFIG_MLX5_CORE_EN=y 鏃跺嵆鍙娇鐢ㄣ€?| 楂樼骇鐗规€у垪琛ㄨ瑙佷笅鏂囥€?
**CONFIG_MLX5_BRIDGE=(y/n)**

|    鍚敤浠ュお缃戞ˉ鎺ワ紙BRIDGE锛夊嵏杞芥敮鎸?<mlx5_bridge_offload>銆?|    杩欏皢鎻愪緵鎶?mlx5 uplink 鍜?VF 绔彛鐨勪唬琛紙representor锛夊姞鍏?Bridge锛?|    浠ュ強涓鸿繖浜涚鍙ｄ箣闂寸殑娴侀噺鍗歌浇瑙勫垯鐨勮兘鍔涖€?|    鏀寔 VLAN锛坱runk 鍜?access 妯″紡锛夈€?

**CONFIG_MLX5_CORE=(y/m/n)** (module mlx5_core.ko)

|    鍙互閫氳繃鍦ㄥ唴鏍搁厤缃腑閫夋嫨 CONFIG_MLX5_CORE=y/m 鏉ュ惎鐢ㄨ椹卞姩銆?|    杩欏皢涓?mlx5 ulp 鎻愪緵鐢ㄤ簬鎺ュ彛鐨?mlx5 core 椹卞姩锛坢lx5e銆乵lx5_ib锛夈€?

**CONFIG_MLX5_CORE_EN=(y/n)**

|    閫夋嫨姝ら€夐」灏嗘彁渚涘叿鏈夋墍鏈夋爣鍑?rx/tx 鍗歌浇鐨勫熀鏈互澶綉缁滅綉缁滆澶囨敮鎸併€?|    mlx5e 鏄彁渚涚綉缁滆澶囧唴鏍告帴鍙ｇ殑 mlx5 ulp 椹卞姩锛岄€変腑鍚?mlx5e 灏嗗唴寤哄埌 mlx5_core.ko 涓€?

**CONFIG_MLX5_CORE_EN_DCB=(y/n)**:

|    鍚敤 `Data Center Bridging (DCB) Support <https://enterprise-support.nvidia.com/s/article/howto-auto-config-pfc-and-ets-on-connectx-4-via-lldp-dcbx>`_銆?

**CONFIG_MLX5_CORE_IPOIB=(y/n)**

|    IPoIB 鍗歌浇涓庡姞閫熸敮鎸併€?|    闇€瑕?CONFIG_MLX5_CORE_EN 涓?rdma IPoIB ulp 缃戠粶璁惧鎻愪緵鍔犻€熸帴鍙ｃ€?

**CONFIG_MLX5_CLS_ACT=(y/n)**

|    鍚敤 TC 鍒嗙被鍣ㄥ姩浣滐紙NET_CLS_ACT锛夌殑鍗歌浇鏀寔銆?|    鍦ㄥ師鐢?NIC 妯″紡鍜?Switchdev SRIOV 妯″紡涓嬪潎鍙敤銆?|    鍩轰簬娴佺殑鍒嗙被鍣紙渚嬪閫氳繃 `tc-flower(8)` 娉ㄥ唽鐨勯偅浜涳級鐢辫澶囧鐞嗭紝鑰岄潪涓绘満銆?|    闅忓悗浼氳鐩栧尮閰嶅垎绫荤粨鏋滅殑鍔ㄤ綔鐢变簬鍗歌浇鑰屽嵆鏃剁敓鏁堛€?

**CONFIG_MLX5_EN_ARFS=(y/n)**

|    鍚敤纭欢鍔犻€熺殑鎺ユ敹娴佸鍚戯紙arfs锛夋敮鎸侊紝浠ュ強 ntuple 杩囨护銆?|    https://enterprise-support.nvidia.com/s/article/howto-configure-arfs-on-connectx-4


**CONFIG_MLX5_EN_IPSEC=(y/n)**

|    鍚敤 IPSec XFRM 鍔犲瘑鍗歌浇鍔犻€?<xfrm_device>銆?

**CONFIG_MLX5_MACSEC=(y/n)**

|    鏋勫缓瀵?NIC 涓?MACsec 鍔犲瘑鍗歌浇鍔犻€熺殑鏀寔銆?

**CONFIG_MLX5_EN_RXNFC=(y/n)**

|    鍚敤 ethtool 鎺ユ敹缃戠粶娴佸垎绫伙紝鍏佽鐢ㄦ埛閫氳繃 ethtool set/get_rxnfc API
|    鐢ㄨ嚜瀹氫箟娴佽鍒欐妸娴侀噺瀵煎悜浠绘剰 rx 闃熷垪銆?

**CONFIG_MLX5_EN_TLS=(y/n)**

|    TLS 鍔犲瘑鍗歌浇鍔犻€熴€?

**CONFIG_MLX5_ESWITCH=(y/n)**

|    ConnectX NIC 涓殑浠ュお缃?SRIOV E-Switch 鏀寔銆侲-Switch 涓哄惎鐢ㄧ殑 VF 鍜?PF 鎻愪緵鍐呴儴 SRIOV 鏁版嵁鍖呭鍚戜笌浜ゆ崲锛屾湁涓ょ鍙敤妯″紡锛?|           1) `Legacy SRIOV mode (L2 mac vlan steering based) <https://enterprise-support.nvidia.com/s/article/HowTo-Configure-SR-IOV-for-ConnectX-4-ConnectX-5-ConnectX-6-with-KVM-Ethernet>`_銆?|           2) Switchdev mode (eswitch offloads) <switchdev>銆?

**CONFIG_MLX5_FPGA=(y/n)**

|    鏋勫缓瀵?Mellanox Technologies 鐨?Innova 绯诲垪缃戝崱鐨勬敮鎸併€?|    Innova 缃戝崱鐢变竴鍧?ConnectX 鑺墖鍜屼竴鍧?FPGA 鑺墖缁勬垚銆?|    濡傛灉閫夋嫨姝ら€夐」锛宮lx5_core 椹卞姩灏嗗寘鍚?Innova FPGA core锛屽苟鍏佽鏋勫缓鐗瑰畾浜庢矙绠辩殑瀹㈡埛绔┍鍔ㄣ€?

**CONFIG_MLX5_INFINIBAND=(y/n/m)** (module mlx5_ib.ko)

|    鎻愪緵搴曞眰 InfiniBand/RDMA 鍜?`RoCE <https://enterprise-support.nvidia.com/s/article/recommended-network-configuration-examples-for-roce-deployment>`_ 鏀寔銆?

**CONFIG_MLX5_MPFS=(y/n)**

|    ConnectX NIC 涓殑浠ュお缃戝鐗╃悊鍔熻兘浜ゆ崲锛圡PFS锛夋敮鎸併€?|    鍦ㄥ惎鐢?`Multi-Host <https://www.nvidia.com/en-us/networking/multi-host/>`_ 閰嶇疆鏃堕渶瑕佷娇鐢?MPFs锛?|    浠ュ厑璁告妸鐢ㄦ埛閰嶇疆鐨勫崟鎾?MAC 鍦板潃浼犻€掔粰璇锋眰鐨?PF銆?

**CONFIG_MLX5_SF=(y/n)**

|    鏋勫缓瀵瑰瓙鍔熻兘锛坰ubfunction锛夌殑鏀寔銆?|    瀛愬姛鑳芥瘮 PCI SRIOV VF 鏇磋交閲忋€傞€夋嫨姝ら€夐」灏嗗惎鐢ㄥ垱寤哄瓙鍔熻兘璁惧鐨勬敮鎸併€?

**CONFIG_MLX5_SF_MANAGER=(y/n)**

|    鏋勫缓瀵?NIC 涓瓙鍔熻兘绔彛鐨勬敮鎸併€侻ellanox 瀛愬姛鑳界鍙ｉ€氳繃 devlink 绠＄悊銆傚瓙鍔熻兘鏀寔 RDMA銆佺綉缁滆澶囧拰 vdpa 璁惧銆?|    瀹冪被浼间簬 SRIOV VF锛屼絾涓嶉渶瑕?SRIOV 鏀寔銆?

**CONFIG_MLX5_SW_STEERING=(y/n)**

|    鏋勫缓瀵?NIC 涓蒋浠剁鐞嗗鍚戯紙steering锛夌殑鏀寔銆?

**CONFIG_MLX5_HW_STEERING=(y/n)**

|    鏋勫缓瀵?NIC 涓‖浠剁鐞嗗鍚戯紙steering锛夌殑鏀寔銆?

**CONFIG_MLX5_TC_CT=(y/n)**

|    鏀寔閫氳繃 tc ct 鍔ㄤ綔鍗歌浇杩炴帴璺熻釜瑙勫垯銆?

**CONFIG_MLX5_TC_SAMPLE=(y/n)**

|    鏀寔閫氳繃 tc sample 鍔ㄤ綔鍗歌浇閲囨牱瑙勫垯銆?

**CONFIG_MLX5_VDPA=(y/n)**

|    鐢ㄤ簬 Mellanox VDPA 椹卞姩鐨勬敮鎸佸簱銆傛彁渚涙墍鏈夌被鍨?VDPA 椹卞姩閫氱敤鐨勪唬鐮併€?|    璁″垝鍖呭惈浠ヤ笅椹卞姩锛歯et銆乥lock銆?

**CONFIG_MLX5_VDPA_NET=(y/n)**

|    鐢ㄤ簬 ConnectX6 鍙婃洿鏂扮増鏈殑 VDPA 缃戠粶椹卞姩銆傛彁渚涘 virtio net 鏁版嵁璺緞鐨勫嵏杞斤紝
|    浣垮緱鏀惧湪鐜笂鐨勬弿杩扮灏嗙敱纭欢鎵ц銆傚畠杩樻牴鎹墍浣跨敤鐨勫疄闄呰澶囧拰鍥轰欢鐗堟湰鏀寔澶氱鏃犵姸鎬佸嵏杞姐€?

**CONFIG_MLX5_VFIO_PCI=(y/n)**

|    杩欐彁渚涗娇鐢?VFIO 妗嗘灦鐨?MLX5 璁惧杩佺Щ鏀寔銆?

**External options** ( 濡傛灉鐩稿簲鐨?mlx5 鐗规€ф槸蹇呴渶鐨勫垯閫夋嫨 )

- CONFIG_MLXFW: 閫変腑鍚庯紝灏嗗惎鐢?mlx5 鍥轰欢鍒峰啓鏀寔锛堥€氳繃 devlink 鍜?ethtool锛夈€?- CONFIG_PTP_1588_CLOCK: 閫変腑鍚庯紝灏嗗惎鐢?mlx5 ptp 鏀寔
- CONFIG_VXLAN: 閫変腑鍚庯紝灏嗗惎鐢?mlx5 vxlan 鏀寔銆?