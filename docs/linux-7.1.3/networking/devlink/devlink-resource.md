
## Devlink 璧勬簮


`devlink` 鎻愪緵浜嗚椹卞姩娉ㄥ唽璧勬簮鐨勮兘鍔涳紝杩欏彲浠ヨ绠＄悊鍛樻煡鐪嬬粰瀹氳祫婧愮殑璁惧闄愬埗锛屼互鍙?璇ョ粰瀹氳祫婧愬綋鍓嶇殑浣跨敤閲忋€傛澶栵紝杩欎簺璧勬簮鍙互閫夋嫨鍏锋湁鍙厤缃殑澶у皬銆傝繖鍙互浣垮緱绠＄悊鍛?鑳藉闄愬埗鎵€浣跨敤鐨勮祫婧愭暟閲忋€?
渚嬪锛宍netdevsim` 椹卞姩灏?`/IPv4/fib` 鍜?`/IPv4/fib-rules` 浣滀负璧勬簮鏉ラ檺鍒剁粰瀹氳澶?鐨?IPv4 FIB 鏉＄洰鍜岃鍒欑殑鏁伴噺銆?
## 璧勬簮 Id


姣忎釜璧勬簮鐢变竴涓?id 琛ㄧず锛屽苟鍖呭惈鏈夊叧鍏跺綋鍓嶅ぇ灏忎互鍙婄浉鍏冲瓙璧勬簮鐨勪俊鎭€傝璁块棶瀛愯祫婧愶紝
浣犻渶瑕佹寚瀹氳璧勬簮鐨勮矾寰勩€備緥濡?`/IPv4/fib` 鏄?`IPv4` 璧勬簮涓?`fib` 瀛愯祫婧愮殑 id銆?
## 閫氱敤璧勬簮


閫氱敤璧勬簮鐢ㄤ簬鎻忚堪鍙澶氫釜璁惧椹卞姩鍏变韩鐨勮祫婧愶紝鍏舵弿杩板繀椤绘坊鍔犲埌涓嬭〃锛?
   :widths: 10 90

   - - Name
     - Description
   - - `physical_ports`
     - 浜ゆ崲 ASIC 鑳藉鏀寔鐨勭墿鐞嗙鍙ｇ殑鏈夐檺瀹归噺

### 浣跨敤绀轰緥


椹卞姩鏆撮湶鐨勮祫婧愬彲浠ヨ瑙傚療锛屼緥濡傦細


    $devlink resource show pci/0000:03:00.0
    pci/0000:03:00.0:
      name kvd size 245760 unit entry
        resources:
          name linear size 98304 occ 0 unit entry size_min 0 size_max 147456 size_gran 128
          name hash_double size 60416 unit entry size_min 32768 size_max 180224 size_gran 128
          name hash_single size 87040 unit entry size_min 65536 size_max 212992 size_gran 128

鏌愪簺璧勬簮鐨勫ぇ灏忓彲浠ユ洿鏀广€備緥濡傦細


    $devlink resource set pci/0000:03:00.0 path /kvd/hash_single size 73088
    $devlink resource set pci/0000:03:00.0 path /kvd/hash_double size 74368

鏇存敼涓嶄細绔嬪嵆鐢熸晥锛岃繖鍙互閫氳繃 'size_new' 灞炴€ф潵楠岃瘉锛屽畠浠ｈ〃寰呭畾锛坧ending锛夌殑澶у皬
鏇存敼銆備緥濡傦細


    $devlink resource show pci/0000:03:00.0
    pci/0000:03:00.0:
      name kvd size 245760 unit entry size_valid false
      resources:
        name linear size 98304 size_new 147456 occ 0 unit entry size_min 0 size_max 147456 size_gran 128
        name hash_double size 60416 unit entry size_min 32768 size_max 180224 size_gran 128
        name hash_single size 87040 unit entry size_min 65536 size_max 212992 size_gran 128

璇锋敞鎰忥紝璧勬簮澶у皬鐨勬洿鏀瑰彲鑳介渶瑕侀噸鏂板姞杞借澶囨墠鑳芥纭敓鏁堛€?
## 绔彛绾ц祫婧愪笌瀹屾暣杞偍


闄や簡璁惧绾ц祫婧愬锛宍devlink` 杩樻敮鎸佺鍙ｇ骇璧勬簮銆傝繖浜涜祫婧愪笌鐗瑰畾鐨?devlink 绔彛鍏宠仈锛?鑰岄潪鏁翠釜璁惧銆?
瑕佸垪鍑烘墍鏈?devlink 璁惧鍜岀鍙ｇ殑璧勬簮锛?

    $ devlink resource show
    pci/0000:03:00.0:
      name max_local_SFs size 128 unit entry dpipe_tables none
      name max_external_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.0/196608:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.0/196609:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1:
      name max_local_SFs size 128 unit entry dpipe_tables none
      name max_external_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1/196708:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1/196709:
      name max_SFs size 128 unit entry dpipe_tables none

瑕佹樉绀虹壒瀹氱鍙ｇ殑璧勬簮锛?

    $ devlink resource show pci/0000:03:00.0/196608
    pci/0000:03:00.0/196608:
      name max_SFs size 128 unit entry dpipe_tables none

## 璧勬簮浣滅敤鍩熻繃婊?

鍦ㄨ浆鍌ㄦ墍鏈夎澶囩殑璧勬簮鏃讹紝`devlink resource show` 鎺ュ彈涓€涓彲閫夌殑 `scope` 鍙傛暟锛屼互灏?鍝嶅簲闄愬埗涓鸿澶囩骇璧勬簮銆佺鍙ｇ骇璧勬簮锛屾垨涓よ€咃紙榛樿锛夈€?
瑕佷粎杞偍鎵€鏈夎澶囩殑璁惧绾ц祫婧愶細


    $ devlink resource show scope dev
    pci/0000:03:00.0:
      name max_local_SFs size 128 unit entry dpipe_tables none
      name max_external_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1:
      name max_local_SFs size 128 unit entry dpipe_tables none
      name max_external_SFs size 128 unit entry dpipe_tables none

瑕佷粎杞偍鎵€鏈夎澶囩殑绔彛绾ц祫婧愶細


    $ devlink resource show scope port
    pci/0000:03:00.0/196608:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.0/196609:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1/196708:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1/196709:
      name max_SFs size 128 unit entry dpipe_tables none

璇锋敞鎰忥紝绔彛绾ц祫婧愭槸鍙鐨勩€?