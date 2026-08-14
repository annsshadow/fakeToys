
## 澶氶槦鍒楃綉缁滆澶囨敮鎸?HOWTO


## 绗?1 鑺傦細瀹炵幇澶氶槦鍒楁敮鎸佺殑搴曞眰椹卞姩瑕佹眰


### 绠€浠嬶細鍐呮牳瀵瑰闃熷垪璁惧鐨勬敮鎸?


鍐呮牳濮嬬粓鎻愪緵瀵瑰闃熷垪璁惧鐨勬敮鎸併€?

搴曞眰椹卞姩闇€瑕佷娇鐢ㄦ柊鐨?alloc_etherdev_mq() 鎴?alloc_netdev_mq() 鍑芥暟鏉ヤ负璁惧鍒嗛厤瀛愰槦鍒椼€傚簳灞傜殑 kernel API 灏嗚礋璐ｅ瓙闃熷垪鍐呭瓨鐨勫垎閰嶄笌閲婃斁锛屼互鍙婇槦鍒楀湪鍐呭瓨涓綅浜庝綍澶勭殑 netdev 閰嶇疆銆?

搴曞眰椹卞姩杩橀渶瑕佸儚浠婂ぉ绠＄悊鍏ㄥ眬 netdev->queue_lock 閭ｆ牱绠＄悊杩欎簺闃熷垪銆傚洜姝わ紝鍦ㄨ澶囦粛澶勪簬杩愯鐘舵€佹椂锛屽簳灞傞┍鍔ㄥ簲浣跨敤 netif_{start|stop|wake}_subqueue() 鍑芥暟鏉ョ鐞嗘瘡涓槦鍒椼€俷etdev->queue_lock 浠嶇敤浜庤澶囦笂绾挎垨瀹屽叏鍏抽棴鏃讹紙unregister_netdev() 绛夛級銆?

## 绗?2 鑺傦細qdisc 瀵瑰闃熷垪璁惧鐨勬敮鎸?


鐩墠鏈変袱涓?qdisc 閽堝澶氶槦鍒楄澶囪繘琛屼簡浼樺寲銆傜涓€涓槸榛樿鐨?pfifo_fast qdisc銆傝 qdisc 姣忎釜纭欢闃熷垪鏀寔涓€涓?qdisc銆備竴涓柊鐨勮疆璇?qdisc锛宻ch_multiq锛屼篃鏀寔澶氫釜纭欢闃熷垪銆俼disc 璐熻矗鍒嗙被 skb锛岀劧鍚庢牴鎹?skb->queue_mapping 鐨勫€煎皢 skb 瀵煎悜瀵瑰簲鐨?band 涓庨槦鍒椼€傚湪搴曞眰椹卞姩涓娇鐢ㄦ瀛楁鏉ュ喅瀹氬皢 skb 鍙戦€佸埌鍝釜闃熷垪銆?

sch_multiq 宸蹭负甯屾湜閬垮厤闃熷ご闃诲锛坔ead-of-line blocking锛夌殑纭欢娣诲姞銆傚畠灏嗗湪鍚勪釜 band 闂村惊鐜紝骞跺湪鍑洪槦涓€涓暟鎹寘涔嬪墠楠岃瘉涓庤 band 鍏宠仈鐨勭‖浠堕槦鍒楁湭琚仠姝€?

鍦?qdisc 鍔犺浇鏃讹紝band 鐨勬暟閲忓熀浜庣‖浠朵笂鐨勯槦鍒楁暟閲忋€備竴鏃﹀缓绔嬪叧鑱旓紝浠讳綍璁剧疆浜?skb->queue_mapping 鐨?skb 閮藉皢琚帓闃熷埌涓庣‖浠堕槦鍒楀叧鑱旂殑 band銆?

## 绗?3 鑺傦細浣跨敤 MULTIQ 澶勭悊澶氶槦鍒楄澶囩殑绠€瑕?HOWTO


鐢ㄦ埛绌洪棿鍛戒护 'tc'锛坕proute2 杞欢鍖呯殑涓€閮ㄥ垎锛夌敤浜庨厤缃?qdisc銆傝灏?MULTIQ qdisc 娣诲姞鍒颁綘鐨勭綉缁滆澶囷紝鍋囪璁惧
```

    # tc qdisc add dev eth0 root handle 1: multiq

```

qdisc 灏嗗垎閰嶄笌璁惧鎶ュ憡鐨勯槦鍒楁暟閲忕浉绛夌殑 band 鏁帮紝骞朵娇 qdisc 涓婄嚎銆傚亣璁?eth0 鏈?4 涓?Tx
```

    band 0 => queue 0
    band 1 => queue 1
    band 2 => queue 2
    band 3 => queue 3

```

娴侀噺灏嗗熀浜?simple_tx_hash 鍑芥暟锛屾垨鑰呭鏋滀綘瀹氫箟浜?netdev->select_queue()锛屽垯鍩轰簬瀹冩祦缁忔瘡涓槦鍒椼€?

tc 杩囨护鍣ㄧ殑琛屼负淇濇寔涓嶅彉銆備笉杩囨柊澧炰簡涓€涓?tc 鍔ㄤ綔 skbedit銆傚亣璁句綘鎯冲皢鎵€鏈夊埌鐗瑰畾涓绘満锛堜緥濡?192.168.0.3锛夌殑娴侀噺閫氳繃鐗瑰畾闃熷垪璺敱锛屼綘鍙互浣跨敤
```

    tc filter add dev eth0 parent 1: protocol ip prio 1 u32 \
	    match ip dst 192.168.0.3 \
	    action skbedit queue_mapping 3

```

:Author: Alexander Duyck <alexander.h.duyck@intel.com>
:Original Author: Peter P. Waskiewicz Jr. <peter.p.waskiewicz.jr@intel.com>
