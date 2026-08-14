
## AF_XDP TX 鍏冩暟鎹?

鏈枃妗ｆ弿杩板湪浣跨敤 [af_xdp](af_xdp) 鍙戦€佸寘鏃跺浣曞惎鐢ㄥ嵏杞斤紙offload锛夈€傚叧浜庡浣曞湪鎺ユ敹渚?璁块棶绫讳技鐨勫厓鏁版嵁锛岃鍙傞槄 [xdp-rx-metadata](xdp-rx-metadata)銆?
## 鎬讳綋璁捐


鍏冩暟鎹殑澶撮儴绌洪棿锛坔eadroom锛夐€氳繃 `struct xdp_umem_reg` 涓殑 `tx_metadata_len`
鍜?`XDP_UMEM_TX_METADATA_LEN` 鏍囧織棰勭暀銆傚洜姝わ紝瀵逛簬鍏变韩鍚屼竴 umem 鐨勬瘡涓鎺ュ瓧锛?鍏冩暟鎹暱搴︽槸鐩稿悓鐨勩€傚厓鏁版嵁甯冨眬鏄竴涓浐瀹氱殑 UAPI锛岃鍙傞槄
`include/uapi/linux/if_xdp.h` 涓殑 `union xsk_tx_metadata`銆傚洜姝わ紝涓€鑸潵璇达紝涓婇潰
鐨?`tx_metadata_len` 瀛楁搴斿寘鍚?`sizeof(union xsk_tx_metadata)`銆?
娉ㄦ剰锛屽湪鏈€鍒濈殑瀹炵幇涓苟涓嶈姹?`XDP_UMEM_TX_METADATA_LEN` 鏍囧織銆傚簲鐢ㄧ▼搴忓彲浠ュ厛灏濊瘯
甯﹁鏍囧織鍒涘缓涓€涓?umem锛屽鏋滃け璐ワ紝鍐嶅仛涓€娆′笉甯︽爣蹇楃殑灏濊瘯銆?
澶撮儴绌洪棿浠ュ強鍏冩暟鎹湰韬簲浣嶄簬 umem 甯т腑 `xdp_desc->addr` 鐨勬鍓嶆柟銆傚湪涓€涓抚鍐咃紝
鍏冩暟鎹?```

           tx_metadata_len
     /                         \
    +-----------------+---------+----------------------------+
    | xsk_tx_metadata | padding |          payload           |
    +-----------------+---------+----------------------------+
                                ^
                                |
                          xdp_desc->addr

```
涓€涓?AF_XDP 搴旂敤绋嬪簭鍙互璇锋眰澶т簬 ``sizeof(struct xsk_tx_metadata)`` 鐨勫ご閮ㄧ┖闂淬€傚唴鏍?浼氬拷鐣?padding锛堝苟浠嶄娇鐢?`xdp_desc->addr - tx_metadata_len` 鏉ュ畾浣?`xsk_tx_metadata`锛夈€?瀵逛簬閭ｄ簺涓嶅簲鎼哄甫浠讳綍鍏冩暟鎹紙鍗虫病鏈?`XDP_TX_METADATA` 閫夐」锛夌殑甯э紝鍏冩暟鎹尯鍩熷悓鏍疯
鍐呮牳蹇界暐銆?
flags 瀛楁鐢ㄤ簬鍚敤鐗瑰畾鐨勫嵏杞斤細

- `XDP_TXMD_FLAGS_TIMESTAMP`锛氳姹傝澶囧皢鍙戦€佹椂闂存埑鏀惧叆 `union xsk_tx_metadata` 鐨?  `tx_timestamp` 瀛楁銆?- `XDP_TXMD_FLAGS_CHECKSUM`锛氳姹傝澶囪绠?L4 鏍￠獙鍜屻€俙csum_start` 鎸囧畾鏍￠獙鍜屽簲寮€濮?  鐨勫瓧鑺傚亸绉伙紝`csum_offset` 鎸囧畾璁惧搴斿瓨鏀捐绠楁墍寰楁牎楠屽拰鐨勫瓧鑺傚亸绉汇€?- `XDP_TXMD_FLAGS_LAUNCH_TIME`锛氳姹傝澶囧湪绉颁负 launch time锛堝彂灏勬椂闂达級鐨勯瀹氭椂闂?  璋冨害鍙戦€佽鍖呫€俵aunch time 鐨勫€肩敱 `union xsk_tx_metadata` 鐨?`launch_time` 瀛楁
  鎸囩ず銆?
闄や簡涓婅堪鏍囧織澶栵紝涓轰簡瑙﹀彂鍗歌浇锛岀涓€涓寘鐨?`struct xdp_desc` 鎻忚堪绗﹀簲鍦?`options`
瀛楁涓缃?`XDP_TX_METADATA` 浣嶃€傝繕瑕佹敞鎰忥紝鍦ㄥ缂撳啿鍖猴紙multi-buffer锛夊寘涓紝鍙湁
绗竴涓垎鐗囧簲鎼哄甫鍏冩暟鎹€?
## 杞欢 TX 鏍￠獙鍜?

鍑轰簬寮€鍙戝拰娴嬭瘯鐩殑锛屽彲浠ュ悜 `XDP_UMEM_REG` UMEM 娉ㄥ唽璋冪敤浼犲叆 `XDP_UMEM_TX_SW_CSUM`
鏍囧織銆傚湪杩欑鎯呭喌涓嬶紝褰撹繍琛屽湪 `XDK_COPY` 妯″紡鏃讹紝TX 鏍￠獙鍜屽湪 CPU 涓婅绠椼€備笉瑕佸湪鐢熶骇
鐜涓惎鐢ㄦ閫夐」锛屽洜涓哄畠浼氬鎬ц兘浜х敓璐熼潰褰卞搷銆?
## Launch Time


鎵€璇锋眰鐨?launch time 鐨勫€煎簲鍩轰簬璁惧鐨?PTP 纭欢鏃堕挓锛圥HC锛変互纭繚鍑嗙‘鎬с€備笌 ETF
鎺掗槦瑙勫垯锛堝畠缁勭粐鍖呭苟寤惰繜鍏跺彂閫侊級涓嶅悓锛孉F_XDP 璧扮殑鏄笉鍚岀殑鏁版嵁璺緞銆傜浉鍙嶏紝AF_XDP
浼氱珛鍗冲皢鍖呬氦缁欒澶囬┍鍔紝鑰屼笉閲嶆柊鎺掑垪鍏堕『搴忔垨鍦ㄥ彂閫佸墠鏆傜暀瀹冧滑銆傜敱浜庨┍鍔ㄤ繚鎸?FIFO
琛屼负涓斾笉杩涜鍖呴噸鎺掑簭锛屽甫鏈?launch time 璇锋眰鐨勫寘浼氶樆濉炲悓涓€ Tx 闃熷垪涓殑鍏朵粬鍖咃紝鐩村埌
瀹冭鍙戦€併€傚洜姝わ紝寤鸿涓鸿鍒掑湪鏈潵鍙戦€佺殑娴侀噺鍒嗛厤鍗曠嫭鐨勯槦鍒椼€?
鍦?launch time 鍗歌浇鐗规€ц绂佺敤鐨勬儏鍐典笅锛岃澶囬┍鍔ㄥ簲蹇界暐 launch time 璇锋眰銆備负浜嗘纭?瑙ｉ噴鍜屾湁鎰忎箟鍦版搷浣滐紝launch time 缁濅笉搴旇璁剧疆涓哄ぇ浜庢湭鏉ユ渶杩滃彲缂栫▼鏃堕棿锛坔orizon锛?鍦板钩绾匡級鐨勫€笺€備笉鍚岀殑璁惧瀵?launch time 鍗歌浇鐗规€ф湁涓嶅悓鐨勭‖浠堕檺鍒躲€?
### stmmac 椹卞姩


瀵逛簬 stmmac锛孴SO 涓?launch time锛圱BS锛夌壒鎬у浜庢瘡涓嫭绔嬬殑 Tx 闃熷垪鏄簰鏂ョ殑銆傞粯璁?鎯呭喌涓嬶紝椹卞姩灏?Tx 闃熷垪 0 閰嶇疆涓烘敮鎸?TSO锛屽叾浣?Tx 闃熷垪閰嶇疆涓烘敮鎸?TBS銆俵aunch time
纭欢鍗歌浇鐗规€у彲浠ラ€氳繃浣跨敤 tc-etf 鍛戒护璋冪敤椹卞姩鐨?ndo_setup_tc() 鍥炶皟鏉ュ惎鐢ㄦ垨绂佺敤銆?
缂栫▼鍒?Enhanced Normal Transmit Descriptors 涓殑 launch time 鍊兼槸涓€涓?32 浣嶅€硷紝
鍏朵腑鏈€楂?8 浣嶈〃绀轰互绉掍负鍗曚綅鐨勬椂闂达紝鍓╀綑 24 浣嶈〃绀轰互 256 ns 涓哄閲忕殑鏃堕棿銆傜紪绋嬬殑
launch time 涓?PTP 鏃堕棿锛坆its[39:8]锛夎繘琛屾瘮杈冿紝骞跺湪 256 绉掑悗鍥炵粫銆傚洜姝わ紝瀵逛簬
dwmac4 鍜?dwxlgmac2锛宭aunch time 鐨?horizon 鏄湭鏉?128 绉掋€?
### igc 椹卞姩


瀵逛簬 igc锛屽叏閮ㄥ洓涓?Tx 闃熷垪閮芥敮鎸?launch time 鐗规€с€俵aunch time 纭欢鍗歌浇鐗规€у彲浠?閫氳繃浣跨敤 tc-etf 鍛戒护璋冪敤椹卞姩鐨?ndo_setup_tc() 鍥炶皟鏉ュ惎鐢ㄦ垨绂佺敤銆傚綋杩涘叆 TSN 妯″紡鏃讹紝
igc 椹卞姩浼氬浣嶈澶囷紝骞跺垱寤轰竴涓粯璁?Qbv 璋冨害锛屽懆鏈熶负 1 绉掞紝鎵€鏈?Tx 闃熷垪濮嬬粓寮€鏀俱€?
缂栫▼鍒?Advanced Transmit Context Descriptor 涓殑 launch time 鍊兼槸鐩稿浜庨槦鍒?Qbv
鍙戦€佺獥鍙ｈ捣濮嬫椂闂寸殑鐩稿鍋忕Щ銆傛弿杩扮鐨?Frst 鏍囧織鍙璁剧疆锛屼互灏嗚鍖呰皟搴﹀埌涓嬩竴涓?Qbv
鍛ㄦ湡銆傚洜姝わ紝瀵逛簬 i225 鍜?i226锛宭aunch time 鐨?horizon 鏄槦鍒椾笅涓€涓?Qbv 鍙戦€佺獥鍙?鍛ㄦ湡鐨勭粨鏉熸椂闂淬€備緥濡傦紝褰?Qbv 鍛ㄦ湡璁句负 1 绉掓椂锛宭aunch time 鐨?horizon 鑼冨洿浠?1 绉?鍒?2 绉掞紝鍏蜂綋鍙栧喅浜?Qbv 鍛ㄦ湡褰撳墠杩愯鍒颁綍澶勩€?
## 鏌ヨ璁惧鑳藉姏


姣忎釜璁惧閫氳繃鍏?netlink netdev family 瀵煎嚭鍏跺嵏杞借兘鍔涖€傝鍙傞槄
`Documentation/netlink/specs/netdev.yaml` 涓殑 `xsk-flags` 鐗规€т綅鎺╃爜銆?
- `tx-timestamp`锛氳澶囨敮鎸?`XDP_TXMD_FLAGS_TIMESTAMP`
- `tx-checksum`锛氳澶囨敮鎸?`XDP_TXMD_FLAGS_CHECKSUM`
- `tx-launch-time-fifo`锛氳澶囨敮鎸?`XDP_TXMD_FLAGS_LAUNCH_TIME`

鍏充簬濡備綍鏌ヨ姝や俊鎭紝璇峰弬闃?`tools/net/ynl/samples/netdev.c`銆?
## 绀轰緥


鍏充簬澶勭悊 TX 鍏冩暟鎹殑绀轰緥绋嬪簭锛岃鍙傞槄
`tools/testing/selftests/bpf/xdp_hw_metadata.c`銆傚彟璇峰弬闃?https://github.com/fomichev/xskgen 鑾峰彇涓€涓洿绮剧畝鐨勭ず渚嬨€?