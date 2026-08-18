
## 闈㈠悜 netlink 鐢ㄦ埛鐨勯€氱敤缃戠粶缁熻


缁熻璁℃暟鍣ㄨ鍒嗙粍鍒扮粨鏋勪綋涓細

==================== ===================== =====================
Struct               TLV type              鎻忚堪
==================== ===================== =====================
gnet_stats_basic     TCA_STATS_BASIC       鍩烘湰缁熻
gnet_stats_rate_est  TCA_STATS_RATE_EST    閫熺巼浼拌鍣?gnet_stats_queue     TCA_STATS_QUEUE       闃熷垪缁熻
none                 TCA_STATS_APP         搴旂敤鐗瑰畾
==================== ===================== =====================


### 鏀堕泦锛?

```

	struct mystruct {
		struct gnet_stats_basic	bstats;
		struct gnet_stats_queue	qstats;
		...
	};

```
```

	mystruct->tstats.packet++;
	mystruct->qstats.backlog += skb->pkt_len;


```
### 瀵煎嚭鍒扮敤鎴风┖闂达紙Dump锛夛細


```

    my_dumping_routine(struct sk_buff *skb, ...)
    {
	    struct gnet_dump dump;

	    if (gnet_stats_start_copy(skb, TCA_STATS2, &mystruct->lock, &dump,
				    TCA_PAD) < 0)
		    goto rtattr_failure;

	    if (gnet_stats_copy_basic(&dump, &mystruct->bstats) < 0 ||
		gnet_stats_copy_queue(&dump, &mystruct->qstats) < 0 ||
		    gnet_stats_copy_app(&dump, &xstats, sizeof(xstats)) < 0)
		    goto rtattr_failure;

	    if (gnet_stats_finish_copy(&dump) < 0)
		    goto rtattr_failure;
	    ...
    }

```
### TCA_STATS/TCA_XSTATS 鍚戝悗鍏煎鎬э細


struct tc_stats 鍜?xstats 鐨勬棭鏈熶娇鐢ㄨ€呭彲浠ラ€氳繃璋冪敤鍏煎鍖呰鍑芥暟鏉ヤ繚鎸佸悜鍚庡吋瀹规€э紝浠ョ户缁彁渚?```

    my_dumping_routine(struct sk_buff *skb, ...)
    {
	if (gnet_stats_start_copy_compat(skb, TCA_STATS2, TCA_STATS,
					TCA_XSTATS, &mystruct->lock, &dump,
					TCA_PAD) < 0)
		    goto rtattr_failure;
	    ...
    }

```
涓€涓?struct tc_stats 灏嗗湪 gnet_stats_copy_* 璋冪敤鏈熼棿琚～鍏呭苟杩藉姞鍒?skb銆傚鏋滆皟鐢ㄤ簡
gnet_stats_copy_app锛屽垯鎻愪緵 TCA_XSTATS銆?

### 鍔犻攣锛?

鍦ㄥ啓鍏ュ墠鑾峰彇閿侊紝骞跺湪鎵€鏈夌粺璁″啓鍏ュ畬鎴愬悗閲婃斁銆傚湪鍙戠敓閿欒鐨勬儏鍐典笅閿佷篃鎬绘槸琚噴鏀俱€備綘鏈夎矗浠荤‘淇?閿佸凡鍒濆鍖栥€?

### 閫熺巼浼拌鍣細


0) 鍑嗗涓€涓及璁″櫒灞炴€с€傝繖寰堝彲鑳藉湪鐢ㄦ埛绌洪棿瀹屾垚銆傛 TLV 鐨勫€煎簲鍖呭惈 tc_estimator 缁撴瀯銆傚儚寰€甯?   涓€鏍凤紝杩欐牱鐨?TLV 闇€瑕?32 浣嶅榻愶紝鍥犳闀垮害闇€瑕侀€傚綋璁剧疆绛夈€備及璁″櫒闂撮殧鍜?ewma 瀵规暟闇€瑕佽浆鎹负
   閫傚綋鐨勫€笺€傚缓璁娇鐢?tc_estimator.c::tc_setup_estimator() 浣滀负杞崲渚嬬▼銆傚畠鍋氫簡涓€浜涘阀濡欑殑浜嬫儏銆?   瀹冩帴鍙椾竴涓互寰涓哄崟浣嶇殑鏃堕棿闂撮殧銆佸悓鏍蜂互寰涓哄崟浣嶇殑鏃堕棿甯告暟锛屼互鍙婁竴涓濉厖鐨?struct
   tc_estimator銆傝繑鍥炵殑 tc_estimator 鍙互琚紶杈撳埌鍐呮牳銆傞€氳繃绫诲瀷涓?TCA_RATE 鐨?TLV 灏嗚缁撴瀯
   浼犺緭鍒颁綘鍦ㄥ唴鏍镐腑鐨勪唬鐮併€?
鍦ㄥ唴鏍镐腑璁剧疆鏃讹細

1) 纭繚浣犻鍏堝凡璁剧疆鍩烘湰缁熻鍜岄€熺巼缁熻銆?2) 纭繚浣犲凡鍒濆鍖栫敤浜庤缃绫荤粺璁＄殑 stats 閿併€?```

    int ret = gen_new_estimator(my_basicstats,my_rate_est_stats,
	mystats_lock, attr_with_tcestimator_struct);

    if ret == 0
	success
    else
	failed

```
浠庣幇鍦ㄨ捣锛屾瘡娆′綘 dump my_rate_est_stats 鏃讹紝瀹冨皢鍖呭惈鏈€鏂扮殑淇℃伅銆?
瀹屾垚鍚庯紝璋冪敤 gen_kill_estimator(my_basicstats, my_rate_est_stats)銆傜‘淇濆湪杩涜姝よ皟鐢ㄦ椂
my_basicstats 鍜?my_rate_est_stats 浠嶇劧鏈夋晥锛堝嵆浠嶇劧瀛樺湪锛夈€?

### 浣滆€咃細


- Thomas Graf <tgraf@suug.ch>
- Jamal Hadi Salim <hadi@cyberus.ca>
