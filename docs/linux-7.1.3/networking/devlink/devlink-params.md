
## Devlink 鍙傛暟


`devlink` 鎻愪緵浜嗚椹卞姩鏆撮湶璁惧鍙傛暟浠ユ帶鍒跺簳灞傝澶囧姛鑳界殑鑳藉姏銆傜敱浜?devlink 鍙互鍦ㄨ澶囩骇锛坉evice-wide锛夎繍浣滐紝鍥犳瀹冨彲浠ョ敤浜庢彁渚涘彲鑳藉奖鍝嶅崟涓澶囦笂澶氫釜绔彛鐨勯厤缃€?

鏈枃妗ｆ弿杩板涓┍鍔ㄥ叡鍚屾敮鎸佺殑涓€浜涢€氱敤鍙傛暟銆傛瘡涓┍鍔ㄤ篃鍙互鑷敱娣诲姞鑷繁鐨勫弬鏁般€傛瘡涓┍鍔ㄩ兘蹇呴』璁板綍瀹冧滑鎵€鏀寔鐨勫叿浣撳弬鏁帮紝鏃犺鏄惁閫氱敤銆?

## 閰嶇疆妯″紡


鍙傛暟鍙互鍦ㄤ笉鍚岀殑閰嶇疆妯″紡涓嬭缃€?

   :widths: 5 90

   - - Name
     - Description
   - - `runtime`
     - 鍦ㄩ┍鍔ㄨ繍琛屾椂璁剧疆锛岀珛鍗崇敓鏁堛€備笉闇€瑕佸浣嶃€?
   - - `driverinit`
     - 鍦ㄩ┍鍔ㄥ垵濮嬪寲鏃跺簲鐢ㄣ€傞渶瑕佺敤鎴蜂娇鐢?`devlink` reload 鍛戒护閲嶅惎椹卞姩銆?
   - - `permanent`
     - 鍐欏叆璁惧鐨勯潪鏄撳け鎬у瓨鍌ㄥ櫒銆傞渶瑕佺‖澶嶄綅鎵嶈兘鐢熸晥銆?

### 閲嶆柊鍔犺浇


涓轰簡璁?`driverinit` 鍙傛暟鐢熸晥锛岄┍鍔ㄥ繀椤绘敮鎸侀€氳繃 `devlink-reload` 鍛戒护閲嶆柊鍔犺浇銆傝鍛戒护浼氳姹傞噸鏂板姞杞借澶囬┍鍔ㄣ€?

## 榛樿鍙傛暟鍊?


椹卞姩鍙互鍙€夊湴瀵煎嚭 `runtime` 鍜?`permanent` 妯″紡鍙傛暟鐨勯粯璁ゅ€笺€傚浜?`driverinit` 鍙傛暟锛岄┍鍔ㄨ缃殑鏈€鍚庝竴涓€煎皢鐢ㄤ綔榛樿鍊笺€傞┍鍔ㄨ繕鍙互鏀寔灏?`runtime` 鍜?`permanent` 妯″紡鐨勫弬鏁伴噸缃负鍏堕粯璁ゅ€笺€傞噸缃?`driverinit` 鍙傛暟鐢?devlink 鏍稿績鏀寔锛屾棤闇€棰濆鐨勯┍鍔ㄦ敮鎸併€?

## 閫氱敤閰嶇疆鍙傛暟


浠ヤ笅鏄┍鍔ㄥ彲浠ユ坊鍔犵殑閫氱敤閰嶇疆鍙傛暟鍒楄〃銆備紭鍏堜娇鐢ㄩ€氱敤鍙傛暟锛岃€屼笉鏄姣忎釜椹卞姩鍒涘缓鑷繁鐨勫悕绉般€?

   :widths: 5 5 90

   - - Name
     - Type
     - Description
   - - `enable_sriov`
     - Boolean
     - 鍦ㄨ澶囦腑鍚敤鍗曟牴 I/O 铏氭嫙鍖栵紙SRIOV锛夈€?
   - - `ignore_ari`
     - Boolean
     - 蹇界暐鏇夸唬璺敱 ID 瑙ｉ噴锛圓RI锛夎兘鍔涖€傚鏋滃惎鐢紝鍗充娇骞冲彴宸叉敮鎸?ARI锛岄€傞厤鍣ㄤ篃浼氬拷鐣?ARI 鑳藉姏銆傝澶囧皢鍒涘缓涓庡钩鍙颁笉鏀寔 ARI 鏃剁浉鍚屾暟閲忕殑鍒嗗尯銆?
   - - `msix_vec_per_pf_max`
     - u32
     - 鎻愪緵璁惧鍙互鍒涘缓鐨勬渶澶?MSI-X 涓柇鏁伴噺銆傝鍊煎璁惧涓墍鏈夌墿鐞嗗姛鑳斤紙PF锛夌浉鍚屻€?
   - - `msix_vec_per_pf_min`
     - u32
     - 鎻愪緵璁惧鍒濆鍖栨墍闇€鐨勬渶灏?MSI-X 涓柇鏁伴噺銆傝鍊煎璁惧涓墍鏈夌墿鐞嗗姛鑳斤紙PF锛夌浉鍚屻€?
   - - `fw_load_policy`
     - u8
     - 鎺у埗璁惧鐨勫浐浠跺姞杞界瓥鐣ャ€?
        - `DEVLINK_PARAM_FW_LOAD_POLICY_VALUE_DRIVER` (0)
          鍔犺浇椹卞姩鍋忓ソ鐨勫浐浠剁増鏈€?
        - `DEVLINK_PARAM_FW_LOAD_POLICY_VALUE_FLASH` (1)
          鍔犺浇褰撳墠瀛樺偍鍦ㄩ棯瀛樹腑鐨勫浐浠躲€?
        - `DEVLINK_PARAM_FW_LOAD_POLICY_VALUE_DISK` (2)
          鍔犺浇褰撳墠鍦ㄤ富鏈虹鐩樹笂鍙敤鐨勫浐浠躲€?
   - - `reset_dev_on_drv_probe`
     - u8
     - 鎺у埗椹卞姩鎺㈡祴锛坧robe锛夋椂璁惧鐨勫浣嶇瓥鐣ャ€?
        - `DEVLINK_PARAM_RESET_DEV_ON_DRV_PROBE_VALUE_UNKNOWN` (0)
          鏈煡鎴栨棤鏁堝€笺€?
        - `DEVLINK_PARAM_RESET_DEV_ON_DRV_PROBE_VALUE_ALWAYS` (1)
          椹卞姩鎺㈡祴鏃舵€绘槸澶嶄綅璁惧銆?
        - `DEVLINK_PARAM_RESET_DEV_ON_DRV_PROBE_VALUE_NEVER` (2)
          椹卞姩鎺㈡祴鏃朵粠涓嶅浣嶈澶囥€?
        - `DEVLINK_PARAM_RESET_DEV_ON_DRV_PROBE_VALUE_DISK` (3)
          浠呭綋鑳藉湪鏂囦欢绯荤粺涓壘鍒板浐浠舵椂鎵嶅浣嶈澶囥€?
   - - `enable_roce`
     - Boolean
     - 鍦ㄨ澶囦腑鍚敤 RoCE 娴侀噺澶勭悊銆?
   - - `enable_eth`
     - Boolean
     - 鍚敤鏃讹紝璁惧椹卞姩灏嗗疄渚嬪寲璇?devlink 璁惧鐨勪互澶綉鐗瑰畾杈呭姪璁惧銆?
   - - `enable_rdma`
     - Boolean
     - 鍚敤鏃讹紝璁惧椹卞姩灏嗗疄渚嬪寲璇?devlink 璁惧鐨?RDMA 鐗瑰畾杈呭姪璁惧銆?
   - - `enable_vnet`
     - Boolean
     - 鍚敤鏃讹紝璁惧椹卞姩灏嗗疄渚嬪寲璇?devlink 璁惧鐨?VDPA 缃戠粶鐗瑰畾杈呭姪璁惧銆?
   - - `enable_iwarp`
     - Boolean
     - 鍦ㄨ澶囦腑鍚敤 iWARP 娴侀噺澶勭悊銆?
   - - `internal_err_reset`
     - Boolean
     - 鍚敤鏃讹紝璁惧椹卞姩灏嗗湪鍐呴儴閿欒鏃跺浣嶈澶囥€?
   - - `max_macs`
     - u32
     - 閫氬父 macvlan銆乿lan 缃戠粶璁惧鐨?mac 涔熶細缂栫▼鍒板叾鐖剁綉缁滆澶囩殑 Function rx 杩囨护鍣ㄤ腑銆傝鍙傛暟闄愬埗姣忎釜浠ュお缃戠鍙ｅ彲浠庤璁惧鎺ユ敹娴侀噺鐨勬渶澶у崟鎾?mac 鍦板潃杩囨护鍣ㄦ暟閲忋€?
   - - `region_snapshot_enable`
     - Boolean
     - 鍚敤 `devlink-region` 蹇収鐨勬崟鑾枫€?
   - - `enable_remote_dev_reset`
     - Boolean
     - 鍚敤鐢辫繙绋嬩富鏈鸿繘琛岀殑璁惧澶嶄綅銆傛竻闄ゆ椂锛岃澶囬┍鍔ㄥ皢鎷掔粷锛圢ACK锛夊叾瀹冧富鏈哄浣嶈澶囩殑浠讳綍灏濊瘯銆傝鍙傛暟瀵逛簬璁惧琚笉鍚屼富鏈哄叡浜紙濡傚涓绘満璁剧疆锛夌殑鍦烘櫙寰堟湁鐢ㄣ€?
   - - `io_eq_size`
     - u32
     - 鎺у埗 I/O 瀹屾垚 EQ 鐨勫ぇ灏忋€?
   - - `event_eq_size`
     - u32
     - 鎺у埗寮傛鎺у埗浜嬩欢 EQ 鐨勫ぇ灏忋€?
   - - `enable_phc`
     - Boolean
     - 鍦ㄨ澶囦腑鍚敤 PHC锛圥TP 纭欢鏃堕挓锛夊姛鑳姐€?
   - - `clock_id`
     - u64
     - 璁惧鐢ㄤ簬娉ㄥ唽 DPLL 璁惧鍜屽紩鑴氱殑鏃堕挓 ID銆?
   - - `total_vfs`
     - u32
     - PF 鏆撮湶鐨勮櫄鎷熷姛鑳斤紙VF锛夌殑鏈€澶ф暟閲忋€傚湪閲嶅惎/PCI 澶嶄綅鍚庯紝璁惧 sysfs 鐩綍涓嬬殑 'sriov_totalvfs' 鏉＄洰灏嗘姤鍛婃鍊笺€?
   - - `num_doorbells`
     - u32
     - 鎺у埗璁惧浣跨敤鐨勯棬閾冿紙doorbell锛夋暟閲忋€?
   - - `max_mac_per_vf`
     - u32
     - 鎺у埗鍙互鍒嗛厤缁欒櫄鎷熷姛鑳斤紙VF锛夌殑鏈€澶?MAC 鍦板潃杩囨护鍣ㄦ暟閲忋€?
