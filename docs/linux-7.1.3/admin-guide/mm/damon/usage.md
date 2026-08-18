
## 璇︾粏鐢ㄦ硶


DAMON 涓轰笉鍚岀敤鎴锋彁渚涗簡浠ヤ笅鎺ュ彛銆?
- **涓撶敤 DAMON 妯″潡銆?* This <damon_modules_special_purpose> 闈㈠悜閭ｄ簺浣跨敤涓撶敤 DAMON 鐢ㄩ€旀潵鏋勫缓銆佸垎鍙戝拰/鎴栫鐞嗗唴鏍哥殑浜恒€備娇鐢ㄥ畠锛岀敤鎴峰彲浠ヤ互绠€鍗曠殑鏂瑰紡鍦ㄦ瀯寤恒€佸惎鍔ㄦ垨杩愯鏃堕拡瀵圭粰瀹氱洰鐨勪娇鐢?DAMON 鐨勪富瑕佺壒鎬с€?- **DAMON 鐢ㄦ埛绌洪棿宸ュ叿銆?* This <https://github.com/damonitor/damo>_ 闈㈠悜绯荤粺绠＄悊鍛樼瓑甯屾湜鑾峰緱寮€绠卞嵆鐢ㄣ€佷汉鎬у寲鐨勬帴鍙ｇ殑鐗规潈鐢ㄦ埛銆備娇鐢ㄥ畠锛岀敤鎴峰彲浠ヤ互浜烘€у寲鐨勬柟寮忎娇鐢?DAMON 鐨勪富瑕佺壒鎬с€備笉杩囷紝瀹冨彲鑳芥病鏈夐拡瀵圭壒娈婂満鏅繘琛岄珮搴︿紭鍖栥€傛洿澶氱粏鑺傦紝璇峰弬鑰冨叾 usage document <https://github.com/damonitor/damo/blob/next/USAGE.md>_銆?- **sysfs 鎺ュ彛銆?* This <sysfs_interface> 闈㈠悜甯屾湜瀵?DAMON 杩涜鏇翠紭鍖栦娇鐢ㄧ殑鐗规潈鐢ㄦ埛绌洪棿绋嬪簭鍛樸€備娇鐢ㄥ畠锛岀敤鎴峰彲浠ラ€氳繃璇诲啓鐗规畩鐨?sysfs 鏂囦欢鏉ヤ娇鐢?DAMON 鐨勪富瑕佺壒鎬с€傚洜姝わ紝浣犲彲浠ョ紪鍐欏苟浣跨敤浣犱釜鎬у寲鐨?DAMON sysfs 鍖呰绋嬪簭鏉ユ浛浣犺鍐欒繖浜?sysfs 鏂囦欢銆侱AMON user space tool <https://github.com/damonitor/damo>_ 灏辨槸姝ょ被绋嬪簭鐨勪竴涓緥瀛愩€?- **鍐呮牳绌洪棿缂栫▼鎺ュ彛銆?* [This </mm/damon/api>](This </mm/damon/api>) 闈㈠悜鍐呮牳绌洪棿绋嬪簭鍛樸€備娇鐢ㄥ畠锛岀敤鎴峰彲浠ラ€氳繃涓轰綘缂栧啓鍐呮牳绌洪棿 DAMON 搴旂敤绋嬪簭锛屾渶鐏垫椿楂樻晥鍦板埄鐢?DAMON 鐨勬瘡涓€椤圭壒鎬с€備綘鐢氳嚦鍙互鎵╁睍 DAMON 浠ユ敮鎸佸悇绉嶅湴鍧€绌洪棿銆傛洿澶氱粏鑺傦紝璇峰弬鑰?interface document </mm/damon/api>](document </mm/damon/api>)銆?

## sysfs 鎺ュ彛


DAMON sysfs 鎺ュ彛鍦ㄥ畾涔変簡 `CONFIG_DAMON_SYSFS` 鏃舵瀯寤恒€傚畠鍦ㄨ嚜宸辩殑 sysfs 鐩綍 `<sysfs>/kernel/mm/damon/` 涓嬪垱寤哄涓洰褰曞拰鏂囦欢銆備綘鍙互閫氳繃璇诲啓璇ョ洰褰曚笅鐨勬枃浠舵潵鎺у埗 DAMON銆?
浣滀负涓€涓畝鐭ず渚嬶紝鐢ㄦ埛鍙互鐩戞帶缁欏畾杩涚▼鐨勮櫄鎷熷湴鍧€绌洪棿锛屾柟娉曞涓嬶細

```

    # cd /sys/kernel/mm/damon/admin/
    # echo 1 > kdamonds/nr_kdamonds && echo 1 > kdamonds/0/contexts/nr_contexts
    # echo vaddr > kdamonds/0/contexts/0/operations
    # echo 1 > kdamonds/0/contexts/0/targets/nr_targets
    # echo $(pidof <workload>) > kdamonds/0/contexts/0/targets/0/pid_target
    # echo on > kdamonds/0/state

```
### 鏂囦欢灞傜骇


DAMON sysfs 鎺ュ彛鐨勬枃浠跺眰绾у涓嬫墍绀恒€傚湪涓嬪浘涓紝鐖跺瓙鍏崇郴鐢ㄧ缉杩涜〃绀猴紝姣忎釜鐩綍甯︽湁 `/` 鍚庣紑锛屾瘡涓洰褰曚腑鐨勬枃浠剁敤閫楀彿锛?,"锛夊垎闅斻€?

    /sys/kernel/mm/damon <sysfs_root>/admin
    鈹?kdamonds <sysfs_kdamonds>/nr_kdamonds
    鈹?鈹?0 <sysfs_kdamond>/state,pid,refresh_ms
    鈹?鈹?鈹?contexts <sysfs_contexts>/nr_contexts
    鈹?鈹?鈹?鈹?0 <sysfs_context>/avail_operations,operations,addr_unit
    鈹?鈹?鈹?鈹?鈹?monitoring_attrs <sysfs_monitoring_attrs>/
    鈹?鈹?鈹?鈹?鈹?鈹?intervals/sample_us,aggr_us,update_us
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?intervals_goal/access_bp,aggrs,min_sample_us,max_sample_us
    鈹?鈹?鈹?鈹?鈹?鈹?nr_regions/min,max
    鈹?鈹?鈹?鈹?鈹?targets <sysfs_targets>/nr_targets
    鈹?鈹?鈹?鈹?鈹?鈹?0 <sysfs_target>/pid_target,obsolete_target
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?regions <sysfs_regions>/nr_regions
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?0 <sysfs_region>/start,end
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?...
    鈹?鈹?鈹?鈹?鈹?鈹?...
    鈹?鈹?鈹?鈹?鈹?schemes <sysfs_schemes>/nr_schemes
    鈹?鈹?鈹?鈹?鈹?鈹?0 <sysfs_scheme>/action,target_nid,apply_interval_us
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?access_pattern <sysfs_access_pattern>/
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?sz/min,max
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?nr_accesses/min,max
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?age/min,max
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?quotas <sysfs_quotas>/ms,bytes,reset_interval_ms,effective_bytes,goal_tuner
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?weights/sz_permil,nr_accesses_permil,age_permil
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?goals <sysfs_schemes_quota_goals>/nr_goals
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?0/target_metric,target_value,current_value,nid,path
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?watermarks <sysfs_watermarks>/metric,interval_us,high,mid,low
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?{core_,ops_,}filters <sysfs_filters>/nr_filters
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?0/type,matching,allow,memcg_path,addr_start,addr_end,target_idx,min,max
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?dests <damon_sysfs_dests>/nr_dests
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?0/id,weight
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?stats <sysfs_schemes_stats>/nr_tried,sz_tried,nr_applied,sz_applied,sz_ops_filter_passed,qt_exceeds,nr_snapshots,max_nr_snapshots
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?tried_regions <sysfs_schemes_tried_regions>/total_bytes
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?0/start,end,nr_accesses,age,sz_filter_passed
    鈹?鈹?鈹?鈹?鈹?鈹?鈹?鈹?...
    鈹?鈹?鈹?鈹?鈹?鈹?...
    鈹?鈹?鈹?鈹?...
    鈹?鈹?...


### 鏍圭洰褰?

DAMON sysfs 鎺ュ彛鐨勬牴鏄?`<sysfs>/kernel/mm/damon/`锛屽畠鏈変竴涓悕涓?`admin` 鐨勭洰褰曘€傝鐩綍鍖呭惈渚涚壒鏉冪敤鎴风┖闂寸▼搴忔帶鍒?DAMON 鐨勬枃浠躲€傛嫢鏈?root 鏉冮檺鐨勭敤鎴风┖闂村伐鍏锋垨瀹堟姢杩涚▼鍙互浣跨敤璇ョ洰褰曘€?

### kdamonds/


鍦?`admin` 鐩綍涓嬶紝鏈変竴涓?`kdamonds` 鐩綍锛屽叾涓寘鍚敤浜庢帶鍒?kdamonds 鐨勬枃浠讹紙鏇村缁嗚妭璇峰弬鑰?design <damon_design_execution_model_and_data_structures>锛夈€傝捣鍒濓紝璇ョ洰褰曞彧鏈変竴涓枃浠?`nr_kdamonds`銆傚悜璇ユ枃浠跺啓鍏ヤ竴涓暟瀛楋紙`N`锛変細鍒涘缓鍚嶄负 `0` 鍒?`N-1` 鐨勫瓙鐩綍銆傛瘡涓洰褰曚唬琛ㄤ竴涓?kdamond銆?

### kdamonds/<N>/


鍦ㄦ瘡涓?kdamond 鐩綍涓紝瀛樺湪涓変釜鏂囦欢锛坄state`銆乣pid` 鍜?`refresh_ms`锛変互鍙婁竴涓洰褰曪紙`contexts`锛夈€?
璇诲彇 `state` 浼氳繑鍥?`on`锛堝鏋?kdamond 姝ｅ湪杩愯锛夛紝鎴?`off`锛堝鏋滄湭杩愯锛夈€?
鐢ㄦ埛鍙互鍚戜笅闈㈢殑 `state` 鏂囦欢鍐欏叆浠ヤ笅鍛戒护鏉ユ帶鍒?kdamond銆?
- `on`锛氬紑濮嬭繍琛屻€?- `off`锛氬仠姝㈣繍琛屻€?- `commit`锛氬啀娆¤鍙?sysfs 鏂囦欢涓櫎 `state` 鏂囦欢涔嬪鐨勭敤鎴疯緭鍏ャ€傚鏋滄湭鎸囧畾鐩爣鍖哄煙锛岀洃鎺х洰鏍囧尯鍩?<sysfs_regions> 鐨勮緭鍏ヤ篃浼氳蹇界暐銆?- `update_tuned_intervals`锛氱敤鑷姩璋冭皭鎵€搴旂敤鐨?``sampling interval` and `aggregation interval`` 鏇存柊璇?kdamond 鐨?`sample_us` 鍜?`aggr_us` 鏂囦欢鍐呭銆傛洿澶氱粏鑺傝鍙傝€?intervals_goal section <damon_usage_sysfs_monitoring_intervals_goal>銆?- `commit_schemes_quota_goals`锛氳鍙栧熀浜?DAMON 鐨勬搷浣滄柟妗堢殑 quota goals <sysfs_schemes_quota_goals>銆?- `update_schemes_stats`锛氭洿鏂拌 kdamond 姣忎釜鍩轰簬 DAMON 鐨勬搷浣滄柟妗堢殑缁熻鏂囦欢鍐呭銆傚叧浜庣粺璁＄殑缁嗚妭锛岃鍙傝€?stats section <sysfs_schemes_stats>銆?- `update_schemes_tried_regions`锛氭洿鏂拌 kdamond 姣忎釜鍩轰簬 DAMON 鐨勬搷浣滄柟妗堢殑鍔ㄤ綔宸插皾璇曞尯鍩熺洰褰曘€傚叧浜庡熀浜?DAMON 鐨勬搷浣滄柟妗堝姩浣滃凡灏濊瘯鍖哄煙鐩綍鐨勭粏鑺傦紝璇峰弬鑰?tried_regions section <sysfs_schemes_tried_regions>銆?- `update_schemes_tried_bytes`锛氫粎鏇存柊 `.../tried_regions/total_bytes` 鏂囦欢銆?- `clear_schemes_tried_regions`锛氭竻闄よ kdamond 姣忎釜鍩轰簬 DAMON 鐨勬搷浣滄柟妗堢殑鍔ㄤ綔宸插皾璇曞尯鍩熺洰褰曘€?- `update_schemes_effective_quotas`锛氭洿鏂拌 kdamond 姣忎釜鍩轰簬 DAMON 鐨勬搷浣滄柟妗堢殑 `effective_bytes` 鏂囦欢鍐呭銆傛洿澶氱粏鑺傝鍙傝€?quotas directory <sysfs_quotas>銆?
濡傛灉鐘舵€佷负 `on`锛岃鍙?`pid` 浼氭樉绀?kdamond 绾跨▼鐨?pid銆?
鐢ㄦ埛鍙互璇锋眰鍐呮牳鍛ㄦ湡鎬у湴鏇存柊鏄剧ず鑷姩璋冭皭鍙傛暟鍜?DAMOS 缁熻鐨勬枃浠讹紝鑰屼笉蹇呮墜鍔ㄥ皢 `update_tuned_intervals` 涔嬬被鐨勫叧閿瓧鍐欏叆 `state` 鏂囦欢銆備负姝わ紝鐢ㄦ埛搴斿皢鏈熸湜鐨勬洿鏂版椂闂撮棿闅旓紙姣锛夊啓鍏?`refresh_ms` 鏂囦欢銆傚鏋滆闂撮殧涓洪浂锛屽垯绂佺敤鍛ㄦ湡鎬ф洿鏂般€傝鍙栬鏂囦欢浼氭樉绀哄綋鍓嶈缃殑鏃堕棿闂撮殧銆?
`contexts` 鐩綍鍖呭惈鐢ㄤ簬鎺у埗璇?kdamond 灏嗘墽琛岀殑鐩戞帶涓婁笅鏂囩殑鏂囦欢銆?

### kdamonds/<N>/contexts/


璧峰垵锛岃鐩綍鍙湁涓€涓枃浠?`nr_contexts`銆傚悜璇ユ枃浠跺啓鍏ヤ竴涓暟瀛楋紙`N`锛変細鍒涘缓鍚嶄负 `0` 鍒?`N-1` 鐨勫瓙鐩綍銆傛瘡涓洰褰曚唬琛ㄤ竴涓洃鎺т笂涓嬫枃锛堟洿澶氱粏鑺傝鍙傝€?design <damon_design_execution_model_and_data_structures>锛夈€傜洰鍓嶆瘡涓?kdamond 鍙敮鎸佷竴涓笂涓嬫枃锛屽洜姝ゅ彧鑳藉悜璇ユ枃浠跺啓鍏?`0` 鎴?`1`銆?

### contexts/<N>/


鍦ㄦ瘡涓笂涓嬫枃鐩綍涓紝瀛樺湪涓変釜鏂囦欢锛坄avail_operations`銆乣operations` 鍜?`addr_unit`锛変互鍙婁笁涓洰褰曪紙`monitoring_attrs`銆乣targets` 鍜?`schemes`锛夈€?
DAMON 鏀寔澶氱绫诲瀷鐨?:ref:`monitoring operations <damon_design_configurable_operations_set>`锛屽寘鎷敤浜庤櫄鎷熷湴鍧€绌洪棿鍜岀墿鐞嗗湴鍧€绌洪棿鐨勯偅浜涖€備綘鍙互閫氳繃璇诲彇 `avail_operations` 鏂囦欢鑾峰緱褰撳墠杩愯鍐呮牳涓婂彲鐢ㄧ殑鐩戞帶鎿嶄綔闆嗗垪琛ㄣ€傛牴鎹唴鏍搁厤缃紝璇ユ枃浠朵細鍒楀嚭涓嶅悓鐨勫彲鐢ㄦ搷浣滈泦銆傚叧浜庢墍鏈夊彲鐢ㄦ搷浣滈泦鍙婂叾绠€瑕佽鏄庣殑鍒楄〃锛岃鍙傝€?:ref:`design <damon_operations_set>`銆?
浣犲彲浠ラ€氳繃鍚?`avail_operations` 鏂囦欢鍐欏叆鍏朵腑鍒楀嚭鐨勪竴涓叧閿瓧锛屽苟浠?`operations` 鏂囦欢璇诲彇锛屾潵璁剧疆鍜岃幏鍙?DAMON 灏嗙敤浜庤涓婁笅鏂囩殑鐩戞帶鎿嶄綔绫诲瀷銆?
`addr_unit` 鏂囦欢鐢ㄤ簬璁剧疆鍜岃幏鍙栨搷浣滈泦鐨?:ref:`address unit <damon_design_addr_unit>` 鍙傛暟銆?

### contexts/<N>/monitoring_attrs/


鐢ㄤ簬鎸囧畾鐩戞帶灞炴€х殑鏂囦欢锛堝寘鎷洃鎺ф墍闇€鐨勮川閲忓拰鏁堢巼锛変綅浜?`monitoring_attrs` 鐩綍銆傚叿浣撹€岃█锛岃鐩綍涓湁涓や釜瀛愮洰褰曪細`intervals` 鍜?`nr_regions`銆?
鍦?`intervals` 鐩綍涓嬶紝瀛樺湪涓変釜鏂囦欢锛屽垎鍒搴?DAMON 鐨勯噰鏍烽棿闅旓紙`sample_us`锛夈€佽仛鍚堥棿闅旓紙`aggr_us`锛夊拰鏇存柊闂撮殧锛坄update_us`锛夈€備綘鍙互閫氳繃璇诲啓杩欎簺鏂囦欢浠ュ井绉掍负鍗曚綅璁剧疆鍜岃幏鍙栬繖浜涘€笺€?
鍦?`nr_regions` 鐩綍涓嬶紝瀛樺湪涓や釜鏂囦欢锛屽垎鍒搴?DAMON 鐩戞帶鍖哄煙鐨勪笅鐣屽拰涓婄晫锛坄min` 鍜?`max`锛夛紝瀹冧滑鎺у埗鐩戞帶寮€閿€銆備綘鍙互閫氳繃璇诲啓杩欎簺鏂囦欢鏉ヨ缃拰鑾峰彇杩欎簺鍊笺€?
鍏充簬闂撮殧鍜岀洃鎺у尯鍩熻寖鍥寸殑鏇村缁嗚妭锛岃鍙傝€?Design 鏂囨。 ([/mm/damon/design](/mm/damon/design))銆?

### contexts/<N>/monitoring_attrs/intervals/intervals_goal/


鍦?`intervals` 鐩綍涓嬶紝杩樺瓨鍦ㄤ竴涓敤浜庤嚜鍔ㄨ皟璋?`sample_us` 鍜?`aggr_us` 鐨勭洰褰曪紝鍗?`intervals_goal` 鐩綍銆傝鐩綍涓嬫湁鍥涗釜鐢ㄤ簬鑷姩璋冭皭鎺у埗鐨勬枃浠讹細`access_bp`銆乣aggrs`銆乣min_sample_us` 鍜?`max_sample_us`銆傚叧浜庤皟璋愭満鍒剁殑鍐呴儴鍘熺悊锛岃鍙傝€?:ref:`design document of the feature <damon_design_monitoring_intervals_autotuning>`銆傝鍐?`intervals_goal` 鐩綍涓嬬殑鍥涗釜鏂囦欢浼氭樉绀哄苟鏇存柊 :ref:design doc <damon_design_monitoring_intervals_autotuning> 涓弿杩扮殑鍚屽悕璋冭皭鍙傛暟銆傝皟璋愪粠鐢ㄦ埛璁剧疆鐨?`sample_us` 鍜?`aggr_us` 寮€濮嬨€傚湪灏?`update_tuned_intervals` 鍐欏叆 `state` 鏂囦欢鍚庯紝鍙互浠?`sample_us` 鍜?`aggr_us` 鏂囦欢璇诲彇涓や釜闂撮殧鐨勮皟璋愬悗褰撳墠鍊笺€?

### contexts/<N>/targets/


璧峰垵锛岃鐩綍鍙湁涓€涓枃浠?`nr_targets`銆傚悜璇ユ枃浠跺啓鍏ヤ竴涓暟瀛楋紙`N`锛変細鍒涘缓鍚嶄负 `0` 鍒?`N-1` 鐨勫瓙鐩綍銆傛瘡涓洰褰曚唬琛ㄤ竴涓洃鎺х洰鏍囥€?

### targets/<N>/


鍦ㄦ瘡涓洰鏍囩洰褰曚腑锛屽瓨鍦ㄤ袱涓枃浠讹紙`pid_target` 鍜?`obsolete_target`锛変互鍙婁竴涓洰褰曪紙`regions`锛夈€?
濡傛灉浣犲悜 `contexts/<N>/operations` 鍐欏叆浜?`vaddr`锛屽垯姣忎釜鐩爣閮藉簲鏄竴涓繘绋嬨€備綘鍙互閫氳繃灏嗚繘绋嬬殑 pid 鍐欏叆 `pid_target` 鏂囦欢鏉ュ皢璇ヨ繘绋嬫寚瀹氱粰 DAMON銆?
鐢ㄦ埛鍙互閫氳繃鍚?`obsolete_target` 鏂囦欢鍐欏叆闈為浂鍊煎苟鎻愪氦锛堝悜 `state` 鏂囦欢鍐欏叆 `commit`锛夋潵閫夋嫨鎬у湴绉婚櫎鐩爣鏁扮粍涓棿鐨勬煇浜涚洰鏍囥€侱AMON 浼氫粠鍏跺唴閮ㄧ洰鏍囨暟缁勪腑绉婚櫎鍖归厤鐨勭洰鏍囥€傜敤鎴锋湁璐ｄ换閲嶆柊鏋勫缓鐩爣鐩綍锛屼互渚垮畠浠纭〃绀哄彉鏇村悗鐨勫唴閮ㄧ洰鏍囨暟缁勩€?


### targets/<N>/regions


瀵逛簬 `fvaddr` 鎴?`paddr` 鐩戞帶鎿嶄綔闆嗭紝鐢ㄦ埛蹇呴』璁剧疆鐩戞帶鐩爣鍦板潃鑼冨洿銆傚浜?`vaddr` 鎿嶄綔闆嗭紝杩欎笉鏄己鍒剁殑锛屼絾鐢ㄦ埛鍙互閫夋嫨灏嗗垵濮嬬洃鎺у尯鍩熻缃负鐗瑰畾鍦板潃鑼冨洿銆傛洿澶氱粏鑺傝鍙傝€?:ref:`design <damon_design_vaddr_target_regions_construction>`銆?
瀵逛簬姝ょ被鎯呭喌锛岀敤鎴峰彲浠ラ€氳繃鍚戣鐩綍涓嬬殑鏂囦欢鍐欏叆閫傚綋鐨勫€硷紝鎸夎嚜宸辩殑鎰忔効鏄惧紡璁剧疆鍒濆鐩戞帶鐩爣鍖哄煙銆?
璧峰垵锛岃鐩綍鍙湁涓€涓枃浠?`nr_regions`銆傚悜璇ユ枃浠跺啓鍏ヤ竴涓暟瀛楋紙`N`锛変細鍒涘缓鍚嶄负 `0` 鍒?`N-1` 鐨勫瓙鐩綍銆傛瘡涓洰褰曚唬琛ㄤ竴涓垵濮嬬洃鎺х洰鏍囧尯鍩熴€?
濡傛灉鍦ㄦ彁浜ゆ柊鐨?DAMON 鍙傛暟锛堝悜 kdamond <sysfs_kdamond> 鐨?`state` 鏂囦欢鍐欏叆 `commit`锛夋椂 `nr_regions` 涓洪浂锛屾彁浜ら€昏緫浼氬拷鐣ョ洰鏍囧尯鍩熴€傛崲鍙ヨ瘽璇达紝璇ョ洰鏍囩殑褰撳墠鐩戞帶缁撴灉浼氳淇濈暀銆?

### regions/<N>/


鍦ㄦ瘡涓尯鍩熺洰褰曚腑锛屼綘浼氬彂鐜颁袱涓枃浠讹紙`start` 鍜?`end`锛夈€備綘鍙互鍒嗗埆閫氳繃鍐欏拰璇昏繖浜涙枃浠舵潵璁剧疆鍜岃幏鍙栧垵濮嬬洃鎺х洰鏍囧尯鍩熺殑璧峰鍜岀粨鏉熷湴鍧€銆?
姣忎釜鍖哄煙涓嶅簲涓庡叾浠栧尯鍩熼噸鍙犮€傜洰褰?`N` 鐨?`end` 搴斿皬浜庢垨绛変簬鐩綍 `N+1` 鐨?`start`銆?

### contexts/<N>/schemes/


鐢ㄤ簬鍩轰簬 DAMON 鐨勬搷浣滄柟妗堬紙:ref:`DAMOS <damon_design_damos>`锛夌殑鐩綍銆傜敤鎴峰彲浠ラ€氳繃璇诲啓璇ョ洰褰曚笅鐨勬枃浠舵潵鑾峰彇鍜岃缃繖浜涙柟妗堛€?
璧峰垵锛岃鐩綍鍙湁涓€涓枃浠?`nr_schemes`銆傚悜璇ユ枃浠跺啓鍏ヤ竴涓暟瀛楋紙`N`锛変細鍒涘缓鍚嶄负 `0` 鍒?`N-1` 鐨勫瓙鐩綍銆傛瘡涓洰褰曚唬琛ㄤ竴涓熀浜?DAMON 鐨勬搷浣滄柟妗堛€?

### schemes/<N>/


鍦ㄦ瘡涓柟妗堢洰褰曚腑锛屽瓨鍦ㄥ叓涓洰褰曪紙`access_pattern`銆乣quotas`銆乣watermarks`銆乣core_filters`銆乣ops_filters`銆乣filters`銆乣dests`銆乣stats` 鍜?`tried_regions`锛変互鍙婁笁涓枃浠讹紙`action`銆乣target_nid` 鍜?`apply_interval`锛夈€?
`action` 鏂囦欢鐢ㄤ簬璁剧疆鍜岃幏鍙栨柟妗堢殑 :ref:`action <damon_design_damos_action>`銆傚彲浠ュ啓鍏ュ拰璇诲彇璇ユ枃浠剁殑鍏抽敭瀛楀強鍏跺惈涔変笌璁捐鏂囨。 design doc <damon_design_damos_action> 涓婄殑鍒楄〃鐩稿悓銆?
`target_nid` 鏂囦欢鐢ㄤ簬璁剧疆杩佺Щ鐩爣鑺傜偣锛屼粎褰?`action` 涓?`migrate_hot` 鎴?`migrate_cold` 鏃舵墠鏈夋剰涔夈€?
`apply_interval_us` 鏂囦欢鐢ㄤ簬浠ュ井绉掍负鍗曚綅璁剧疆鍜岃幏鍙栨柟妗堢殑 apply_interval <damon_design_damos>銆?

### schemes/<N>/access_pattern/


鐢ㄤ簬缁欏畾鍩轰簬 DAMON 鐨勬搷浣滄柟妗堢殑鐩爣璁块棶 :ref:`pattern <damon_design_damos_access_pattern>` 鐨勭洰褰曘€?
鍦?`access_pattern` 鐩綍涓嬶紝瀛樺湪涓変釜鐩綍锛坄sz`銆乣nr_accesses` 鍜?`age`锛夛紝姣忎釜鐩綍閮芥湁涓や釜鏂囦欢锛坄min` 鍜?`max`锛夈€備綘鍙互閫氳繃鍒嗗埆鍐欏拰璇?`sz`銆乣nr_accesses` 鍜?`age` 鐩綍涓嬬殑 `min` 鍜?`max` 鏂囦欢锛屾潵璁剧疆鍜岃幏鍙栫粰瀹氭柟妗堢殑璁块棶妯″紡銆傛敞鎰?`min` 鍜?`max` 鏋勬垚涓€涓棴鍖洪棿銆?

### schemes/<N>/quotas/


鐢ㄤ簬缁欏畾鍩轰簬 DAMON 鐨勬搷浣滄柟妗堢殑 quotas <damon_design_damos_quotas> 鐨勭洰褰曘€?
鍦?`quotas` 鐩綍涓嬶紝瀛樺湪浜斾釜鏂囦欢锛坄ms`銆乣bytes`銆乣reset_interval_ms`銆乣effective_bytes` 鍜?`goal_tuner`锛変互鍙婁袱涓洰褰曪紙`weights` 鍜?`goals`锛夈€?
閫氳繃灏嗗€煎垎鍒啓鍏ヨ繖涓変釜鏂囦欢锛屼綘鍙互璁剧疆 `time quota`锛堟绉掞級銆乣size quota`锛堝瓧鑺傦級鍜?`reset interval`锛堟绉掞級銆傜劧鍚庯紝DAMON 浼氬皾璇曟渶澶氫娇鐢?`time quota` 姣灏?`action` 搴旂敤浜?`access_pattern` 鐨勫唴瀛樺尯鍩燂紝骞朵笖鍦?`reset_interval_ms` 鍐呬粎灏嗗姩浣滃簲鐢ㄤ簬鏈€澶?`bytes` 瀛楄妭鐨勫唴瀛樺尯鍩熴€傚皢 `ms` 鍜?`bytes` 閮借涓洪浂浼氱鐢ㄩ厤棰濋檺鍒讹紝闄ら潪鑷冲皯璁剧疆浜嗕竴涓?goal <sysfs_schemes_quota_goals>銆?
浣犲彲浠ラ€氳繃灏嗙畻娉曞悕绉板啓鍏?`goal_tuner` 鏂囦欢锛屾潵璁剧疆瑕佷娇鐢ㄧ殑鍩轰簬鐩爣鐨勬湁鏁堥厤棰濊嚜鍔ㄨ皟璋愮畻娉曘€傝鍙栬鏂囦欢浼氳繑鍥炲綋鍓嶉€夊畾鐨勮皟璋愮畻娉曘€傚叧浜庤鐗规€х殑鑳屾櫙璁捐浠ュ強鍙€夌畻娉曠殑鍚嶇О锛岃鍙傝€?automatic quota tuning goals <damon_design_damos_quotas_auto_tuning> 鐨勮璁℃枃妗ｃ€傚叧浜庣洰鏍囩殑璁剧疆锛岃鍙傝€?goals directory <sysfs_schemes_quota_goals>銆?
鏃堕棿閰嶉鍦ㄥ唴閮ㄤ細琚浆鎹负澶у皬閰嶉銆傚湪杞崲鍚庣殑澶у皬閰嶉涓庣敤鎴锋寚瀹氱殑澶у皬閰嶉涔嬮棿锛岄噰鐢ㄨ緝灏忕殑涓€涓€傚熀浜庣敤鎴锋寚瀹氱殑 goal <sysfs_schemes_quota_goals>锛屾湁鏁堝ぇ灏忛厤棰濅細杩涗竴姝ヨ皟鏁淬€傝鍙?`effective_bytes` 浼氳繑鍥炲綋鍓嶇殑鏈夋晥澶у皬閰嶉銆傝鏂囦欢涓嶄細瀹炴椂鏇存柊锛屽洜姝ょ敤鎴峰簲閫氳繃鍚戠浉鍏崇殑 `kdamonds/<N>/state` 鏂囦欢鍐欏叆涓€涓壒娈婂叧閿瓧 `update_schemes_effective_quotas`锛屾潵璇锋眰 DAMON sysfs 鎺ュ彛鏇存柊璇ユ枃浠剁殑缁熻鍐呭銆?
鍦?`weights` 鐩綍涓嬶紝瀛樺湪涓変釜鏂囦欢锛坄sz_permil`銆乣nr_accesses_permil` 鍜?`age_permil`锛夈€備綘鍙互閫氳繃灏嗗€煎啓鍏?`weights` 鐩綍涓嬬殑杩欎笁涓枃浠讹紝浠ュ崈鍒嗕箣涓€涓哄崟浣嶈缃拡瀵瑰ぇ灏忋€佽闂鐜囧拰骞撮緞鐨?:ref:`prioritization weights <damon_design_damos_quotas_prioritization>`銆?

### schemes/<N>/quotas/goals/


鐢ㄤ簬缁欏畾鍩轰簬 DAMON 鐨勬搷浣滄柟妗堢殑 :ref:`automatic quota tuning goals <damon_design_damos_quotas_auto_tuning>` 鐨勭洰褰曘€?
璧峰垵锛岃鐩綍鍙湁涓€涓枃浠?`nr_goals`銆傚悜璇ユ枃浠跺啓鍏ヤ竴涓暟瀛楋紙`N`锛変細鍒涘缓鍚嶄负 `0` 鍒?`N-1` 鐨勫瓙鐩綍銆傛瘡涓洰褰曚唬琛ㄤ竴涓洰鏍囧強鍏跺綋鍓嶈揪鎴愭儏鍐点€傚湪澶氫釜鍙嶉涓紝浣跨敤鏈€浣崇殑涓€涓€?
姣忎釜鐩爣鐩綍鍖呭惈浜斾釜鏂囦欢锛屽嵆 `target_metric`銆乣target_value`銆乣current_value`銆乣nid` 鍜?`path`銆傜敤鎴峰彲浠ラ€氳繃璇诲啓杩欎簺鏂囦欢涓殑姣忎竴涓紝鏉ヨ缃拰鑾峰彇璁捐鏂囨。 design doc <damon_design_damos_quotas_auto_tuning> 涓寚瀹氱殑閰嶉鑷姩璋冭皭鐩爣鐨勪簲涓弬鏁般€傛敞鎰忥紝鐢ㄦ埛杩樺簲杩涗竴姝ュ皢 `commit_schemes_quota_goals` 鍐欏叆 :ref:`kdamond directory <sysfs_kdamond>` 鐨?`state` 鏂囦欢锛屼互灏嗗弽棣堜紶閫掔粰 DAMON銆?

### schemes/<N>/watermarks/


鐢ㄤ簬缁欏畾鍩轰簬 DAMON 鐨勬搷浣滄柟妗堢殑 watermarks <damon_design_damos_watermarks> 鐨勭洰褰曘€?
鍦?watermarks 鐩綍涓嬶紝瀛樺湪浜斾釜鏂囦欢锛坄metric`銆乣interval_us`銆乣high`銆乣mid` 鍜?`low`锛夛紝鐢ㄤ簬璁剧疆搴﹂噺鎸囨爣銆佹寚鏍囨鏌ョ殑鏃堕棿闂撮殧浠ュ強涓変釜姘翠綅绾裤€備綘鍙互閫氳繃鍒嗗埆鍐欒繖浜涙枃浠舵潵璁剧疆鍜岃幏鍙栬繖浜斾釜鍊笺€?
鍙互鍐欏叆 `metric` 鏂囦欢鐨勫叧閿瓧鍙婂叾鍚箟濡備笅銆?
 - none: 蹇界暐姘翠綅绾? - free_mem_rate: 绯荤粺鐨勭┖闂插唴瀛樼巼锛堟瘡鍗冿級

`interval` 搴斾互寰涓哄崟浣嶅啓鍏ャ€?

### schemes/<N>/{core\_,ops\_,}filters/


鐢ㄤ簬缁欏畾鍩轰簬 DAMON 鐨勬搷浣滄柟妗堢殑 filters <damon_design_damos_filters> 鐨勭洰褰曘€?
`core_filters` 鍜?`ops_filters` 鐩綍鍒嗗埆鐢ㄤ簬鐢?DAMON 鏍稿績灞傚拰鎿嶅眰闆嗗眰澶勭悊鐨勮繃婊ゅ櫒銆俙filters` 鐩綍鍙敤浜庡畨瑁呬笌鎵€澶勭悊灞傛棤鍏崇殑杩囨护鍣ㄣ€傜敱 `core_filters` 鍜?`ops_filters` 璇锋眰鐨勮繃婊ゅ櫒浼氬厛浜?`filters` 鐨勮繃婊ゅ櫒瀹夎銆傝繖涓変釜鐩綍鎷ユ湁鐩稿悓鐨勬枃浠躲€?
浣跨敤 `filters` 鐩綍鍙兘浼氳瀵圭粰瀹氳繃婊ゅ櫒鍙婂叾鐩綍涓嬫枃浠剁殑姹傚€奸『搴忎骇鐢熸贩娣嗐€傚洜姝ゅ缓璁敤鎴蜂娇鐢?`core_filters` 鍜?`ops_filters` 鐩綍銆俙filters` 鐩綍灏嗘潵鍙兘浼氳寮冪敤銆?
璧峰垵锛岃鐩綍鍙湁涓€涓枃浠?`nr_filters`銆傚悜璇ユ枃浠跺啓鍏ヤ竴涓暟瀛楋紙`N`锛変細鍒涘缓鍚嶄负 `0` 鍒?`N-1` 鐨勫瓙鐩綍銆傛瘡涓洰褰曚唬琛ㄤ竴涓繃婊ゅ櫒銆傝繃婊ゅ櫒鎸夋暟瀛楅『搴忔眰鍊笺€?
姣忎釜杩囨护鍣ㄧ洰褰曞寘鍚節涓枃浠讹紝鍗?`type`銆乣matching`銆乣allow`銆乣memcg_path`銆乣addr_start`銆乣addr_end`銆乣min`銆乣max` 鍜?`target_idx`銆備綘鍙互鍚?`type` 鏂囦欢鍐欏叆杩囨护鍣ㄧ殑绫诲瀷銆傚叧浜庡彲鐢ㄧ殑绫诲瀷鍚嶃€佸叾鍚箟浠ュ強瀹冧滑鐢卞摢涓€灞傚鐞嗭紝璇峰弬鑰?design doc <damon_design_damos_filters>銆?
瀵逛簬 `memcg` 绫诲瀷锛屼綘鍙互閫氳繃灏嗗唴瀛?cgroup 浠?cgroups 鎸傝浇鐐瑰埌 `memcg_path` 鏂囦欢鐨勮矾寰勬潵鎸囧畾鎰熷叴瓒ｇ殑 memory cgroup銆傚浜?`addr` 绫诲瀷锛屼綘鍙互灏嗚寖鍥达紙寮€鍖洪棿锛夌殑璧峰鍜岀粨鏉熷湴鍧€鍒嗗埆鎸囧畾缁?`addr_start` 鍜?`addr_end` 鏂囦欢銆傚浜?`hugepage_size` 绫诲瀷锛屼綘鍙互灏嗚寖鍥达紙闂尯闂达級鐨勬渶灏忓拰鏈€澶уぇ灏忓垎鍒寚瀹氱粰 `min` 鍜?`max` 鏂囦欢銆傚浜?`target` 绫诲瀷锛屼綘鍙互灏?DAMON 涓婁笅鏂囩洃鎺х洰鏍囧垪琛ㄤ腑鐩爣鐨勭储寮曟寚瀹氱粰 `target_idx` 鏂囦欢銆?
浣犲彲浠ュ悜 `matching` 鏂囦欢鍐欏叆 `Y` 鎴?`N`锛屼互鎸囧畾璇ヨ繃婊ゅ櫒鏄惁閽堝涓?`type` 鍖归厤鐨勫唴瀛樸€備綘鍙互鍚?`allow` 鏂囦欢鍐欏叆 `Y` 鎴?`N`锛屼互鎸囧畾鏄惁鍏佽瀵规弧瓒?`type` 鍜?`matching` 鐨勫唴瀛樺簲鐢ㄥ姩浣溿€?
渚嬪锛屼笅闈㈠皢涓€涓?DAMOS 鍔ㄤ綔闄愬埗涓轰粎搴旂敤浜庨潪鍖垮悕

```

    # cd ops_filters/0/
    # echo 2 > nr_filters
    # # disallow anonymous pages
    echo anon > 0/type
    echo Y > 0/matching
    echo N > 0/allow
    # # further filter out all cgroups except one at '/having_care_already'
    echo memcg > 1/type
    echo /having_care_already > 1/memcg_path
    echo Y > 1/matching
    echo N > 1/allow

```
鍏充簬鏇村缁嗚妭锛屽寘鎷叿鏈変笉鍚?`allow` 鐨勫涓繃婊ゅ櫒濡備綍宸ヤ綔銆佸悇涓繃婊ゅ櫒浣曟椂琚敮鎸佷互鍙婄粺璁′笂鐨勫樊寮傦紝璇峰弬鑰?:ref:`DAMOS filters design documentation <damon_design_damos_filters>`銆?

### schemes/<N>/dests/


鐢ㄤ簬鎸囧畾缁欏畾鍩轰簬 DAMON 鐨勬搷浣滄柟妗堝姩浣滅洰鏍囦綅缃殑鐩綍銆傚鏋滅粰瀹氭柟妗堢殑鍔ㄤ綔涓嶆敮鎸佸涓洰鏍囷紝鍒欏拷鐣ユ鐩綍銆傚彧鏈?`DAMOS_MIGRATE_{HOT,COLD}` 鍔ㄤ綔鏀寔澶氫釜鐩爣銆?
璧峰垵锛岃鐩綍鍙湁涓€涓枃浠?`nr_dests`銆傚悜璇ユ枃浠跺啓鍏ヤ竴涓暟瀛楋紙`N`锛変細鍒涘缓鍚嶄负 `0` 鍒?`N-1` 鐨勫瓙鐩綍銆傛瘡涓洰褰曚唬琛ㄤ竴涓姩浣滅洰鏍囥€?
姣忎釜鐩爣鐩綍鍖呭惈涓や釜鏂囦欢锛屽嵆 `id` 鍜?`weight`銆傜敤鎴峰彲浠ュ悜 `id` 鏂囦欢鍐欏叆鍜岃鍙栫洰鏍囩殑鏍囪瘑绗︺€傚浜?`DAMOS_MIGRATE_{HOT,COLD}` 鍔ㄤ綔锛岃縼绉荤洰鏍囪妭鐐圭殑鑺傜偣 id 搴斿啓鍏?`id` 鏂囦欢銆傜敤鎴峰彲浠ュ悜 `weight` 鏂囦欢鍐欏叆鍜岃鍙栬鐩爣鍦ㄧ粰瀹氱洰鏍囦腑鐨勬潈閲嶃€傛潈閲嶅彲浠ユ槸浠绘剰鏁存暟銆傚綋 DAMOS 灏嗚鍔ㄤ綔搴旂敤浜庡唴瀛樺尯鍩熺殑姣忎釜瀹炰綋鏃讹紝瀹冧細鏍规嵁鐩爣鐨勭浉瀵规潈閲嶆潵閫夋嫨鍔ㄤ綔鐨勭洰鏍囦綅缃€?

### schemes/<N>/stats/


DAMON 涓烘瘡涓柟妗堢粺璁¤鏁般€傝繖浜涚粺璁℃暟鎹彲鐢ㄤ簬鏂规鐨勫湪绾垮垎鏋愭垨璋冧紭銆傚叧浜庣粺璁＄殑鏇村缁嗚妭锛岃鍙傝€?:ref:`design doc <damon_design_damos_stat>`銆?
鍙互閫氳繃璇诲彇 `stats` 鐩綍涓嬬殑鏂囦欢锛坄nr_tried`銆乣sz_tried`銆乣nr_applied`銆乣sz_applied`銆乣sz_ops_filter_passed`銆乣qt_exceeds`銆乣nr_snapshots` 鍜?`max_nr_snapshots`锛夊垎鍒幏鍙栬繖浜涚粺璁℃暟鎹€?
榛樿鎯呭喌涓嬶紝杩欎簺鏂囦欢涓嶄細瀹炴椂鏇存柊銆傜敤鎴峰簲璇锋眰 DAMON sysfs 鎺ュ彛浣跨敤 `refresh_ms` 鍛ㄦ湡鎬ф洿鏂板畠浠紝鎴栬€呴€氳繃鍚戠浉鍏崇殑 `kdamonds/<N>/state` 鏂囦欢鍐欏叆鐗规畩鍏抽敭瀛?`update_schemes_stats` 杩涜涓€娆℃洿鏂般€傛洿澶氱粏鑺傝鍙傝€?:ref:`kdamond directory <sysfs_kdamond>`銆?

### schemes/<N>/tried_regions/


璇ョ洰褰曡捣鍒濇湁涓€涓枃浠?`total_bytes`銆?
褰撳悜鐩稿叧鐨?`kdamonds/<N>/state` 鏂囦欢鍐欏叆涓€涓壒娈婂叧閿瓧 `update_schemes_tried_regions` 鏃讹紝DAMON 浼氭洿鏂?`total_bytes` 鏂囦欢锛屼娇璇诲彇瀹冭繑鍥炴柟妗堝凡灏濊瘯鍖哄煙鐨勬€诲ぇ灏忥紝骞跺垱寤轰粠璇ョ洰褰曚笅浠?`0` 寮€濮嬩互鏁存暟鍛藉悕鐨勭洰褰曘€傛瘡涓洰褰曞寘鍚枃浠讹紝鏆撮湶鐩稿簲鏂规鐨?`action` 灏濊瘯搴旂敤鐨勬瘡涓唴瀛樺尯鍩熺殑璇︾粏淇℃伅锛岃繖浜涗俊鎭湪鐩稿簲鏂规鐨勪笅涓€涓?apply interval <damon_design_damos> 鏈熼棿鐢熸垚銆傝繖浜涗俊鎭寘鎷尯鍩熺殑鍦板潃鑼冨洿銆乣nr_accesses` 鍜?`age`銆?
鍚戠浉鍏崇殑 `kdamonds/<N>/state` 鏂囦欢鍐欏叆 `update_schemes_tried_bytes` 鍙細鏇存柊 `total_bytes` 鏂囦欢锛屼笉浼氬垱寤哄瓙鐩綍銆?
褰撳彟涓€涓壒娈婂叧閿瓧 `clear_schemes_tried_regions` 琚啓鍏ョ浉鍏崇殑 `kdamonds/<N>/state` 鏂囦欢鏃讹紝杩欎簺鐩綍浼氳绉婚櫎銆?
璇ョ洰褰曠殑棰勬湡鐢ㄩ€旀槸璋冩煡鏂规鐨勮涓猴紝浠ュ強绫绘煡璇㈢殑楂樻晥鏁版嵁璁块棶鐩戞帶缁撴灉妫€绱€傜壒鍒槸瀵逛簬鍚庝竴绉嶇敤渚嬶紝鐢ㄦ埛鍙互灏?`action` 璁句负 `stat`锛屽苟灏?`access pattern` 璁句负鍏舵兂瑕佹煡璇㈢殑鎰熷叴瓒ｆā寮忋€?

### tried_regions/<N>/


鍦ㄦ瘡涓尯鍩熺洰褰曚腑锛屼綘浼氬彂鐜颁簲涓枃浠讹紙`start`銆乣end`銆乣nr_accesses`銆乣age` 鍜?`sz_filter_passed`锛夈€傝鍙栬繖浜涙枃浠朵細鏄剧ず鐩稿簲鍩轰簬 DAMON 鐨勬搷浣滄柟妗?`action` 灏濊瘯搴旂敤鐨勫尯鍩熺殑灞炴€с€?
#### 绀轰緥


浠ヤ笅鍛戒护搴旂敤涓€涓柟妗堬紝鍏跺惈涔夋槸锛氣€滃鏋滀竴涓ぇ灏忎负 [4KiB, 8KiB] 鐨勫唴瀛樺尯鍩熷湪 [10, 20] 鐨勮仛鍚堥棿闅斿唴锛屾瘡鑱氬悎闂撮殧鐨勮闂鏁板湪 [0, 5] 鑼冨洿鍐咃紝鍒欏皢璇ュ尯鍩熸崲鍑恒€傚浜庢崲鍑猴紝姣忕鏈€澶氫娇鐢?10ms锛屽苟涓旀瘡绉掓崲鍑轰笉瓒呰繃 1GiB銆傚湪姝ら檺鍒朵笅锛屼紭鍏堟崲鍑哄勾榫勬洿闀跨殑鍐呭瓨鍖哄煙銆傛澶栵紝姣?5 绉掓鏌ョ郴缁熺殑绌洪棽鍐呭瓨鐜囷紝褰撶┖闂插唴瀛樼巼浣庝簬 50% 鏃跺紑濮嬬洃鎺у拰鎹㈠嚭锛屼絾濡傛灉绌洪棽

```

    # cd <sysfs>/kernel/mm/damon/admin
    # # populate directories
    # echo 1 > kdamonds/nr_kdamonds; echo 1 > kdamonds/0/contexts/nr_contexts;
    # echo 1 > kdamonds/0/contexts/0/schemes/nr_schemes
    # cd kdamonds/0/contexts/0/schemes/0
    # # set the basic access pattern and the action
    # echo 4096 > access_pattern/sz/min
    # echo 8192 > access_pattern/sz/max
    # echo 0 > access_pattern/nr_accesses/min
    # echo 5 > access_pattern/nr_accesses/max
    # echo 10 > access_pattern/age/min
    # echo 20 > access_pattern/age/max
    # echo pageout > action
    # # set quotas
    # echo 10 > quotas/ms
    # echo $((1024*1024*1024)) > quotas/bytes
    # echo 1000 > quotas/reset_interval_ms
    # # set watermark
    # echo free_mem_rate > watermarks/metric
    # echo 5000000 > watermarks/interval_us
    # echo 600 > watermarks/high
    # echo 500 > watermarks/mid
    # echo 300 > watermarks/low

```
璇锋敞鎰忥紝寮虹儓寤鸿浣跨敤鍍?`damo <https://github.com/damonitor/damo>`_ 杩欐牱鐨勭敤鎴风┖闂村伐鍏凤紝鑰屼笉鏄儚涓婇潰杩欐牱鎵嬪姩璇诲啓鏂囦欢銆備互涓婁粎浣滀负绀轰緥銆?

## 鐢ㄤ簬鐩戞帶缁撴灉鐨?Tracepoints


鐢ㄦ埛鍙互閫氳繃 :ref:`tried_regions <sysfs_schemes_tried_regions>` 鑾峰彇鐩戞帶缁撴灉銆傝鎺ュ彛瀵逛簬鑾峰彇蹇収寰堟湁鐢紝浣嗗浜庡畬鏁磋褰曟墍鏈夌洃鎺х粨鏋滃彲鑳芥晥鐜囦笉楂樸€備负姝わ紝鎻愪緵浜嗕袱涓窡韪偣锛屽嵆 `damon:damon_aggregated` 鍜?`damon:damos_before_apply`銆俙damon:damon_aggregated` 鎻愪緵瀹屾暣鐨勭洃鎺х粨鏋滐紝鑰?`damon:damos_before_apply` 鎻愪緵姣忎釜鍩轰簬 DAMON 鐨勬搷浣滄柟妗堬紙DAMOS <damon_design_damos>锛夊皢瑕佸簲鐢ㄧ殑鍖哄煙鐨勭洃鎺х粨鏋溿€傚洜姝わ紝`damon:damos_before_apply` 瀵逛簬璁板綍 DAMOS 鐨勫唴閮ㄨ涓猴紝鎴栧熀浜?DAMOS 鐩爣璁块棶妯″紡 <damon_design_damos_access_pattern> 鐨勭被鏌ヨ楂樻晥鐩戞帶缁撴灉璁板綍鏇存湁鐢ㄣ€?
鍦ㄧ洃鎺у紑鍚湡闂达紝浣犲彲浠ヨ褰曡窡韪偣浜嬩欢锛屾柟娉曞涓嬶細

```

    # echo on > kdamonds/0/state
    # perf record -e damon:damon_aggregated &
    # sleep 5
    # kill 9 $(pidof perf)
    # echo off > kdamonds/0/state
    # perf script
    kdamond.0 46568 [027] 79357.842179: damon:damon_aggregated: target_id=0 nr_regions=11 122509119488-135708762112: 0 864
    [...]

```
perf script 杈撳嚭鐨勬瘡涓€琛屼唬琛ㄤ竴涓洃鎺у尯鍩熴€傚墠浜斾釜瀛楁涓庡叾浠栬窡韪偣杈撳嚭涓€鏍枫€傜鍏釜瀛楁锛坄target_id=X`锛夋樉绀鸿鍖哄煙鐨勭洃鎺х洰鏍囩殑 id銆傜涓冧釜瀛楁锛坄nr_regions=X`锛夋樉绀鸿鐩爣鐨勭洃鎺у尯鍩熸€绘暟銆傜鍏釜瀛楁锛坄X-Y:`) 鏄剧ず璇ュ尯鍩熶互瀛楄妭涓哄崟浣嶇殑璧峰锛坄X`锛夊拰缁撴潫锛坄Y`锛夊湴鍧€銆傜涔濅釜瀛楁锛坄X`锛夋樉绀鸿鍖哄煙鐨?`nr_accesses`锛堝叧浜庤璁℃暟鍣ㄧ殑鏇村缁嗚妭锛岃鍙傝€?design <damon_design_region_based_sampling>锛夈€傛渶鍚庣鍗佷釜瀛楁锛坄X`锛夋樉绀鸿鍖哄煙鐨?`age`锛堝叧浜庤璁℃暟鍣ㄧ殑鏇村缁嗚妭锛岃鍙傝€?design <damon_design_age_tracking>锛夈€?
濡傛灉浜嬩欢鏄?`damon:damos_beofre_apply`锛屽垯 `perf script` 杈撳嚭浼?
```

    kdamond.0 47293 [000] 80801.060214: damon:damos_before_apply: ctx_idx=0 scheme_idx=0 target_idx=0 nr_regions=11 121932607488-135128711168: 0 136
    [...]

```
杈撳嚭鐨勬瘡涓€琛屼唬琛ㄥ湪璺熻釜鏃跺埢姣忎釜鍩轰簬 DAMON 鐨勬搷浣滄柟妗堝嵆灏嗗簲鐢ㄧ殑姣忎釜鐩戞帶鍖哄煙銆傚墠浜斾釜瀛楁濡傚父銆傞櫎浜?`damon_aggregated` 璺熻釜鐐圭殑杈撳嚭澶栵紝瀹冭繕鏄剧ず鏂规涓?DAMON 涓婁笅鏂囧湪鍏?kdamond 涓婁笅鏂囧垪琛ㄤ腑鐨勭储寮曪紙`ctx_idx=X`锛夛紝浠ュ強鏂规鍦ㄥ叾涓婁笅鏂囨柟妗堝垪琛ㄤ腑鐨勭储寮曪紙`scheme_idx=X`锛夈€?