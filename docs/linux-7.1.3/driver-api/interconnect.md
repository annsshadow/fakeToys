## 閫氱敤绯荤粺浜掕繛瀛愮郴缁?


### 绠€浠?


璇ユ鏋舵棬鍦ㄦ彁渚涗竴涓爣鍑嗙殑鍐呮牳鎺ュ彛锛岀敤浜庢帶鍒?SoC 涓婁簰杩烇紙interconnect锛夌殑
璁剧疆銆傝繖浜涜缃彲浠ユ槸澶氫釜浜掕繛璁惧鎴栧姛鑳藉潡涔嬮棿鐨勫悶鍚愰噺銆佸欢杩熷拰浼樺厛绾с€傚彲浠?
鍔ㄦ€佹帶鍒惰繖浜涜缃紝浠ヨ妭鐪佸姛鑰楁垨鎻愪緵鏈€澶ф€ц兘銆?

浜掕繛鎬荤嚎鏄竴绉嶅叿鏈夊彲閰嶇疆鍙傛暟鐨勭‖浠讹紝鍙牴鎹粠鍚勪釜椹卞姩鏀跺埌鐨勮姹傚湪鏁版嵁璺緞涓?
杩涜璁剧疆銆備簰杩炴€荤嚎鐨勪竴涓緥瀛愭槸鑺墖缁勪腑鍚勪釜缁勪欢鎴栧姛鑳藉潡涔嬮棿鐨勪簰杩炪€備竴涓?SoC
涓婂彲浠ュ瓨鍦ㄥ鏉′簰杩烇紝骞朵笖鍙互鏄灞傜殑銆?

涓嬮潰鏄竴寮犵湡瀹?SoC 浜掕繛鎬荤嚎鎷撴墤鐨勭畝鍖栫ず鎰忓浘銆?
```

 +----------------+    +----------------+
 | HW Accelerator |--->|      M NoC     |<---------------+
 +----------------+    +----------------+                |
                         |      |                    +------------+
  +-----+  +-------------+      V       +------+     |            |
  | DDR |  |                +--------+  | PCIe |     |            |
  +-----+  |                | Slaves |  +------+     |            |
    ^ ^    |                +--------+     |         |   C NoC    |
    | |    V                               V         |            |
 +------------------+   +------------------------+   |            |   +-----+
 |                  |-->|                        |-->|            |-->| CPU |
 |                  |-->|                        |<--|            |   +-----+
 |     Mem NoC      |   |         S NoC          |   +------------+
 |                  |<--|                        |---------+    |
 |                  |<--|                        |<------+ |    |   +--------+
 +------------------+   +------------------------+       | |    +-->| Slaves |
   ^  ^    ^    ^          ^                             | |        +--------+
   |  |    |    |          |                             | V
 +------+  |  +-----+   +-----+  +---------+   +----------------+   +--------+
 | CPUs |  |  | GPU |   | DSP |  | Masters |-->|       P NoC    |-->| Slaves |
 +------+  |  +-----+   +-----+  +---------+   +----------------+   +--------+
           |
       +-------+
       | Modem |
       +-------+

```
### 鏈


浜掕繛鎻愪緵鑰咃紙Interconnect provider锛夋槸浜掕繛纭欢鐨勮蒋浠跺畾涔夈€備笂鍥句腑鐨勪簰杩炴彁渚涜€?
鏄?M NoC銆丼 NoC銆丆 NoC銆丳 NoC 鍜?Mem NoC銆?

浜掕繛鑺傜偣锛圛nterconnect node锛夋槸浜掕繛纭欢绔彛鐨勮蒋浠跺畾涔夈€傛瘡涓簰杩炴彁渚涜€呯敱澶氫釜
浜掕繛鑺傜偣缁勬垚锛岃繖浜涜妭鐐硅繛鎺ュ埌鍏朵粬 SoC 缁勪欢锛屽寘鎷叾浠栦簰杩炴彁渚涜€呫€傚浘涓?CPU 杩炴帴
鍒板唴瀛樼殑鐐圭О涓轰簰杩炶妭鐐癸紝瀹冨睘浜?Mem NoC 浜掕繛鎻愪緵鑰呫€?

浜掕繛绔偣锛圛nterconnect endpoints锛夋槸璺緞鐨勭涓€涓垨鏈€鍚庝竴涓厓绱犮€傛瘡涓鐐归兘鏄?
涓€涓妭鐐癸紝浣嗗苟闈炴瘡涓妭鐐归兘鏄鐐广€?

浜掕繛璺緞锛圛nterconnect path锛夋槸涓や釜绔偣涔嬮棿鐨勬墍鏈夊唴瀹癸紝鍖呮嫭浠庢簮鑺傜偣鍒拌揪鐩殑
鑺傜偣鎵€蹇呴』閬嶅巻鐨勬墍鏈夎妭鐐广€傚畠鍙互鍖呭惈璺ㄨ秺澶氫釜浜掕繛鎻愪緵鑰呯殑澶氬涓讳粠锛坢aster-slave锛?
缁勫悎銆?

浜掕繛浣跨敤鑰咃紙Interconnect consumers锛夋槸鍒╃敤鎻愪緵鑰呮墍鏆撮湶鐨勬暟鎹矾寰勭殑瀹炰綋銆備娇鐢ㄨ€?
鍚戞彁渚涜€呭彂閫佽姹傦紝瑕佹眰鍚勭涓嶅悓鐨勫悶鍚愰噺銆佸欢杩熷拰浼樺厛绾с€傞€氬父浣跨敤鑰呮槸璁惧椹卞姩锛?
瀹冧滑鏍规嵁鑷韩闇€姹傚彂閫佽姹傘€備娇鐢ㄨ€呯殑涓€涓緥瀛愭槸鏀寔澶氱鏍煎紡鍜屽浘鍍忓昂瀵哥殑瑙嗛瑙ｇ爜鍣ㄣ€?

### 浜掕繛鎻愪緵鑰?


浜掕繛鎻愪緵鑰呮槸瀹炵幇鍒濆鍖栧拰閰嶇疆浜掕繛鎬荤嚎纭欢鏂规硶鐨勫疄浣撱€備簰杩炴彁渚涜€呴┍鍔ㄥ簲褰撳悜浜掕繛
鎻愪緵鑰呮牳蹇冿紙interconnect provider core锛夋敞鍐屻€?


   :functions: icc_provider_init icc_provider_register icc_provider_deregister
               icc_node_create icc_node_create_dyn icc_node_destroy
               icc_node_add icc_node_del icc_nodes_remove icc_node_set_name
               icc_link_create icc_link_nodes

### 浜掕繛浣跨敤鑰?


浜掕繛浣跨敤鑰呮槸浣跨敤浜掕繛 API 鏉ヨ幏鍙栫鐐逛箣闂磋矾寰勩€佸苟涓鸿繖浜涗簰杩炶矾寰勮缃叾
甯﹀/寤惰繜/QoS 瑕佹眰鐨勫鎴风銆?

   :functions: devm_of_icc_get of_icc_get_by_index of_icc_get icc_get
               icc_put icc_enable icc_disable icc_set_bw icc_set_tag
               icc_get_name


### 浜掕繛 debugfs 鎺ュ彛


涓庡叾浠栦竴浜涘瓙绯荤粺绫讳技锛屼簰杩炰篃浼氬垱寤轰竴浜涚敤浜庤皟璇曞拰鍐呯渷鐨勬枃浠躲€俤ebugfs 涓殑鏂囦欢
涓嶈瑙嗕负 ABI锛屽洜姝ゅ簲鐢ㄧ▼搴忎笉搴斾緷璧栧悇鍐呮牳鐗堟湰涔嬮棿鏍煎紡缁嗚妭鐨勫彉鍖栥€?

`/sys/kernel/debug/interconnect/interconnect_summary`锛?

鏄剧ず绯荤粺涓墍鏈変簰杩炶妭鐐瑰強鍏惰仛鍚堢殑甯﹀璇锋眰銆傚湪姣忎釜鑺傜偣涓嬫柟锛岀缉杩涙樉绀哄悇璁惧鍙戝嚭鐨?
甯﹀璇锋眰銆?

`/sys/kernel/debug/interconnect/interconnect_graph`锛?

浠?graphviz dot 鏍煎紡鏄剧ず浜掕繛鍥俱€傚畠鏄剧ず绯荤粺涓墍鏈夌殑浜掕繛鑺傜偣鍜岄摼鎺ワ紝骞跺皢鏉ヨ嚜
鍚屼竴鎻愪緵鑰呯殑鑺傜偣褰掍负瀛愬浘銆傝鏍煎紡鍏锋湁浜虹被鍙鎬э紝涔熷彲浠ョ閬擄紙pipe锛夎緭鍑?
```

        $ cat /sys/kernel/debug/interconnect/interconnect_graph | \
                dot -Tsvg > interconnect_graph.svg

```
`test-client` 鐩綍鎻愪緵浜嗗悜浠绘剰璺緞鍙戝嚭甯﹀锛圔W锛夎姹傜殑鎺ュ彛銆傝娉ㄦ剰锛屽嚭浜庡畨鍏?
鍘熷洜锛岃鐗规€ч粯璁ゆ槸绂佺敤鐨勶紝涓旀病鏈夌敤浜庡惎鐢ㄥ畠鐨?Kconfig銆傚惎鐢ㄥ畠闇€瑕佷慨鏀逛唬鐮?
```

        cd /sys/kernel/debug/interconnect/test-client/

        # Configure node endpoints for the path from CPU to DDR on
        # qcom/sm8550.
        echo chm_apps > src_node
        echo ebi > dst_node

        # Get path between src_node and dst_node. This is only
        # necessary after updating the node endpoints.
        echo 1 > get

        # Set desired BW to 1GBps avg and 2GBps peak.
        echo 1000000 > avg_bw
        echo 2000000 > peak_bw

        # Vote for avg_bw and peak_bw on the latest path from "get".
        # Voting for multiple paths is possible by repeating this
        # process for different nodes endpoints.
        echo 1 > commit

```