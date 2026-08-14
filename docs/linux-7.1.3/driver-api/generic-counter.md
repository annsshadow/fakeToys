
## Generic Counter Interface


## 绠€浠?

璁℃暟鍣ㄨ澶囧箍娉涘瓨鍦ㄤ簬鍚勮鍚勪笟銆傝繖浜涜澶囩殑鏅亶瀛樺湪锛岄渶瑕佷竴涓€氱敤鐨勪氦浜掍笌鏆撮湶鎺ュ彛鍙婃爣鍑嗐€傛湰椹卞姩 API 璇曞浘閫氳繃寮曞叆涓€涓緵浣跨敤鐨勯€氱敤璁℃暟鍣ㄦ帴鍙ｏ紝鏉ヨВ鍐崇幇鏈夎鏁板櫒璁惧椹卞姩涓瓨鍦ㄧ殑浠ｇ爜閲嶅闂銆傞€氱敤璁℃暟鍣ㄦ帴鍙ｄ娇椹卞姩鑳藉鏀寔骞舵毚闇茶鏁板櫒璁惧鎵€鍏辨湁鐨勪竴缁勭粍浠朵笌鍔熻兘銆?
## 鍘熺悊


璁℃暟鍣ㄨ澶囧湪璁捐涓婂彲鑳藉ぇ鐩稿緞搴紝浣嗘棤璁烘槸姝ｄ氦缂栫爜鍣ㄨ鏁板櫒杩樻槸璁℃暟绱姞鍣紝鎵€鏈夎鏁板櫒璁惧閮界敱涓€缁勬牳蹇冪粍浠舵瀯鎴愩€傝繖缁勮鎵€鏈夎鏁板櫒璁惧鍏变韩鐨勬牳蹇冪粍浠讹紝姝ｆ槸閫氱敤璁℃暟鍣ㄦ帴鍙ｇ殑鏈川鎵€鍦ㄣ€?
涓€涓鏁板櫒鏈変笁涓牳蹇冪粍浠讹細

- Signal锛氱敱璁℃暟鍣ㄨ瘎浼扮殑鏁版嵁娴併€?
- Synapse锛氬皢 Signal 涓庤瘎浼拌Е鍙戞潯浠跺叧鑱斿埌 Count 鐨勫叧绯汇€?
- Count锛氭墍杩炴帴 Synapse 鏁堟灉鐨勭疮绉€?
### SIGNAL

Signal 琛ㄧず涓€鏉℃暟鎹祦銆傝繖鏄敱璁℃暟鍣ㄨ瘎浼颁互纭畾璁℃暟鏁版嵁鐨勮緭鍏ユ暟鎹紱渚嬪鏃嬭浆缂栫爜鍣ㄧ殑姝ｄ氦淇″彿杈撳嚭绾裤€傚苟闈炴墍鏈夎鏁板櫒璁惧閮藉悜鐢ㄦ埛鎻愪緵 Signal 鏁版嵁鐨勮闂紝鍥犳瀵归┍鍔ㄨ€岃█鏆撮湶璇ユ暟鎹槸鍙€夌殑銆?
褰?Signal 鏁版嵁鍙緵鐢ㄦ埛璁块棶鏃讹紝閫氱敤璁℃暟鍣ㄦ帴鍙ｆ彁渚涗互涓嬪彲鐢ㄧ殑淇″彿鍙栧€硷細

- SIGNAL_LOW锛氫俊鍙风嚎澶勪簬浣庣數骞崇姸鎬併€?
- SIGNAL_HIGH锛氫俊鍙风嚎澶勪簬楂樼數骞崇姸鎬併€?
涓€涓?Signal 鍙互涓庝竴涓垨澶氫釜 Count 鍏宠仈銆?
### SYNAPSE

Synapse 琛ㄧず Signal 涓?Count 涔嬮棿鐨勫叧鑱斻€係ignal 鏁版嵁浼氬奖鍝嶇浉搴旂殑 Count 鏁版嵁锛岃€?Synapse 琛ㄧず杩欑鍏崇郴銆?
Synapse 鐨勫姩浣滄ā寮忥紙action mode锛夋寚瀹氳Е鍙戠浉搴?Count 鐨勮鏁板嚱鏁拌瘎浼颁互鏇存柊璁℃暟鏁版嵁鐨?Signal 鏁版嵁鏉′欢銆傞€氱敤璁℃暟鍣ㄦ帴鍙ｆ彁渚涗互涓嬪彲鐢ㄧ殑鍔ㄤ綔妯″紡锛?
- None锛歋ignal 涓嶈Е鍙戣鏁板嚱鏁般€傚湪 Pulse-Direction 璁℃暟鍑芥暟妯″紡涓嬶紝姝?Signal 琚綔涓烘柟鍚戯紙Direction锛夎瘎浼般€?
- Rising Edge锛氫綆鐢靛钩鐘舵€佽浆鎹㈠埌楂樼數骞崇姸鎬併€?
- Falling Edge锛氶珮鐢靛钩鐘舵€佽浆鎹㈠埌浣庣數骞崇姸鎬併€?
- Both Edges锛氫换浣曠姸鎬佽浆鎹€?
璁℃暟鍣ㄨ瀹氫箟涓轰竴缁勪笌璁℃暟鏁版嵁鐩稿叧鑱旂殑杈撳叆淇″彿锛岃繖浜涜鏁版暟鎹€氳繃瀵圭浉搴旇鏁板嚱鏁版墍瀹氫箟鐨勫叧鑱旇緭鍏ヤ俊鍙风姸鎬佽繘琛岃瘎浼拌€岀敓鎴愩€傚湪閫氱敤璁℃暟鍣ㄦ帴鍙ｇ殑璇涓嬶紝涓€涓鏁板櫒鐢卞涓?Count 鏋勬垚锛屾瘡涓?Count 鍏宠仈涓€缁?Signal锛屽叾鍚勮嚜鐨?Synapse 瀹炰緥琛ㄧず鐩稿簲 Count 鐨勮鏁板嚱鏁版洿鏂版潯浠躲€?
涓€涓?Synapse 灏嗕竴涓?Signal 涓庝竴涓?Count 鍏宠仈銆?
### COUNT

Count 琛ㄧず鎵€杩炴帴 Synapse 鏁堟灉鐨勭疮绉紱鍗充竴缁?Signal 鐨勮鏁版暟鎹€傞€氱敤璁℃暟鍣ㄦ帴鍙ｅ皢璁℃暟鏁版嵁琛ㄧず涓鸿嚜鐒舵暟銆?
Count 鍏锋湁涓€涓鏁板嚱鏁版ā寮忥紙count function mode锛夛紝琛ㄧず璁℃暟鏁版嵁鐨勬洿鏂拌涓恒€傞€氱敤璁℃暟鍣ㄦ帴鍙ｆ彁渚涗互涓嬪彲鐢ㄧ殑璁℃暟鍑芥暟妯″紡锛?
- Increase锛氱疮绉鏁伴€掑銆?
- Decrease锛氱疮绉鏁伴€掑噺銆?
- Pulse-Direction锛氫俊鍙?A 涓婄殑涓婂崌娌挎洿鏂扮浉搴旇鏁般€備俊鍙?B 鐨勮緭鍏ョ數骞冲喅瀹氭柟鍚戙€?
- Quadrature锛氬涓€瀵规浜ょ紪鐮佷俊鍙疯繘琛岃瘎浼颁互纭畾浣嶇疆鍜屾柟鍚戙€傚彲鐢ㄧ殑 Quadrature 妯″紡濡備笅锛?
  - x1 A锛氳嫢鏂瑰悜涓烘鍚戯紝姝ｄ氦瀵逛俊鍙?A 涓婄殑涓婂崌娌挎洿鏂扮浉搴旇鏁帮紱鑻ユ柟鍚戜负鍙嶅悜锛屼俊鍙?A 涓婄殑涓嬮檷娌挎洿鏂扮浉搴旇鏁般€傛柟鍚戠敱姝ｄ氦缂栫爜鍐冲畾銆?
  - x1 B锛氳嫢鏂瑰悜涓烘鍚戯紝姝ｄ氦瀵逛俊鍙?B 涓婄殑涓婂崌娌挎洿鏂扮浉搴旇鏁帮紱鑻ユ柟鍚戜负鍙嶅悜锛屼俊鍙?B 涓婄殑涓嬮檷娌挎洿鏂扮浉搴旇鏁般€傛柟鍚戠敱姝ｄ氦缂栫爜鍐冲畾銆?
  - x2 A锛氭浜ゅ淇″彿 A 涓婄殑浠讳綍鐘舵€佽浆鎹㈤兘浼氭洿鏂扮浉搴旇鏁般€傛柟鍚戠敱姝ｄ氦缂栫爜鍐冲畾銆?
  - x2 B锛氭浜ゅ淇″彿 B 涓婄殑浠讳綍鐘舵€佽浆鎹㈤兘浼氭洿鏂扮浉搴旇鏁般€傛柟鍚戠敱姝ｄ氦缂栫爜鍐冲畾銆?
  - x4锛氫换涓€姝ｄ氦瀵逛俊鍙蜂笂鐨勪换浣曠姸鎬佽浆鎹㈤兘浼氭洿鏂扮浉搴旇鏁般€傛柟鍚戠敱姝ｄ氦缂栫爜鍐冲畾銆?
涓€涓?Count 鍏锋湁涓€缁勬垨澶氱粍鍏宠仈 Synapse銆?
## 鑼冨紡


鏈€鍩烘湰鐨勮鏁板櫒璁惧鍙互琛ㄧず涓洪€氳繃鍗曚釜 Synapse 涓庡崟涓?Signal 鍏宠仈璧锋潵鐨勫崟涓?Count銆備互涓€涓畝鍗曞湴瀵规煇淇″彿涓婄殑涓婂崌娌胯繘琛岃鏁扮殑璁℃暟鍣ㄨ澶囦负渚嬶細

```
                Count                Synapse        Signal
                -----                -------        ------
        +---------------------+
        | Data: Count         |    Rising Edge     ________
        | Function: Increase  |  <-------------   / Source \
        |                     |                  ____________
        +---------------------+

```
鍦ㄨ绀轰緥涓紝Signal 鏄竴鏉″叿鏈夎剦鍐茬數鍘嬬殑鏉ユ簮杈撳叆绾匡紝鑰?Count 鏄竴涓鍙嶅閫掑鐨勬寔涔呰鏁板€笺€係ignal 閫氳繃 Synapse 涓庣浉搴?Count 鍏宠仈銆俰ncrease 鍑芥暟鐢?Synapse 鎸囧畾鐨?Signal 鏁版嵁鏉′欢瑙﹀彂鈥斺€斿湪鏈緥涓负鐢靛帇杈撳叆绾夸笂鐨勪笂鍗囨部鏉′欢銆傛€讳箣锛岃鏁板櫒璁惧鐨勫瓨鍦ㄤ笌琛屼负鎭板綋鍦扮敱鐩稿簲鐨?Count銆丼ignal 涓?Synapse 缁勪欢琛ㄧず锛氫笂鍗囨部鏉′欢瑙﹀彂瀵圭疮绉鏁板€肩殑 increase 鍑芥暟銆?
璁℃暟鍣ㄨ澶囧苟涓嶅眬闄愪簬鍗曚釜 Signal锛涗簨瀹炰笂锛岀悊璁轰笂璁稿 Signal 閮藉彲涓庡崟涓?Count 鍏宠仈銆備緥濡傦紝姝ｄ氦缂栫爜鍣ㄨ鏁板櫒璁惧鍙互鏍规嵁杈撳叆淇″彿璺熻釜浣嶇疆锛?
```
                   Count                 Synapse     Signal
                   -----                 -------     ------
        +-------------------------+
        | Data: Position          |    Both Edges     ___
        | Function: Quadrature x4 |  <------------   / A \
        |                         |                 _______
        |                         |
        |                         |    Both Edges     ___
        |                         |  <------------   / B \
        |                         |                 _______
        +-------------------------+

```
鍦ㄨ绀轰緥涓紝涓や釜 Signal锛堟浜ょ紪鐮佸櫒绾?A 涓?B锛変笌鍗曚釜 Count 鍏宠仈锛欰 鎴?B 涓婄殑涓婂崌娌挎垨涓嬮檷娌胯Е鍙?"Quadrature x4" 鍑芥暟锛岃鍑芥暟纭畾杩愬姩鏂瑰悜骞舵洿鏂扮浉搴旂殑浣嶇疆鏁版嵁銆?Quadrature x4" 鍑芥暟寰堝彲鑳藉疄鐜颁簬姝ｄ氦缂栫爜鍣ㄨ鏁板櫒璁惧鐨勭‖浠朵腑锛汣ount銆丼ignal 涓?Synapse 浠呬粎鏄繖绉嶇‖浠惰涓轰笌鍔熻兘鐨勮〃绀恒€?
涓庡悓涓€ Count 鍏宠仈鐨?Signal 鍙互鍏锋湁涓嶅悓鐨?Synapse 鍔ㄤ綔妯″紡鏉′欢銆備緥濡傦紝杩愯鍦ㄩ潪姝ｄ氦 Pulse-Direction 妯″紡涓嬬殑姝ｄ氦缂栫爜鍣ㄨ鏁板櫒璁惧鍙互鏈変竴鏉′笓鐢ㄤ簬杩愬姩鐨勮緭鍏ョ嚎锛屼互鍙婄浜屾潯涓撶敤浜庢柟鍚戠殑杈撳叆绾匡細

```
                   Count                   Synapse      Signal
                   -----                   -------      ------
        +---------------------------+
        | Data: Position            |    Rising Edge     ___
        | Function: Pulse-Direction |  <-------------   / A \ (Movement)
        |                           |                  _______
        |                           |
        |                           |       None         ___
        |                           |  <-------------   / B \ (Direction)
        |                           |                  _______
        +---------------------------+

```
鍙湁 Signal A 瑙﹀彂 "Pulse-Direction" 鏇存柊鍑芥暟锛屼絾浠嶉渶 Signal B 鐨勭灛鏃剁姸鎬佹墠鑳界‘瀹氭柟鍚戯紝浠庤€屾纭洿鏂颁綅缃暟鎹€傛渶缁堬紝涓や釜 Signal 閮介€氳繃鍚勮嚜鐨?Synapse 涓庡悓涓€ Count 鍏宠仈锛屼絾鍙湁涓€涓?Synapse 鍏锋湁瑙﹀彂鐩稿簲璁℃暟鍑芥暟鐨勬椿鍔ㄥ姩浣滄ā寮忔潯浠讹紝鑰屽彟涓€涓垯淇濇寔 "None" 鏉′欢鐨勫姩浣滄ā寮忥紝浠ヨ〃鏄庡叾鐩稿簲 Signal 灏界涓嶈Е鍙戯紝浣嗕粛鍙敤浜庣姸鎬佽瘎浼般€?
璇锋敞鎰忥紝Signal銆丼ynapse 涓?Count 鏄娊璞¤〃绀猴紝鏃犻渶涓庡叾鍚勮嚜鐨勭墿鐞嗘潵婧愮揣瀵嗙粦瀹氥€傝繖浣垮緱璁℃暟鍣ㄧ殑浣跨敤鑰呭彲浠ヤ粠鐗╃悊缁勪欢鐨勭粏寰樊鍒紙渚嬪杈撳叆绾挎槸宸垎杩樻槸鍗曠锛変腑瑙ｈ劚鍑烘潵锛岃浆鑰屼笓娉ㄤ簬鏁版嵁涓庤繃绋嬫墍琛ㄧず鐨勬牳蹇冩蹇碉紙渚嬪浠庢浜ょ紪鐮佹暟鎹В璇诲嚭鐨勪綅缃級銆?
## 椹卞姩 API


椹卞姩寮€鍙戣€呭彲浠ラ€氳繃鍖呭惈 include/linux/counter.h 澶存枃浠讹紝鍦ㄨ嚜宸辩殑浠ｇ爜涓娇鐢ㄩ€氱敤璁℃暟鍣ㄦ帴鍙ｃ€傝澶存枃浠舵彁渚涗簡鑻ュ共鐢ㄤ簬瀹氫箟璁℃暟鍣ㄨ澶囩殑鏍稿績鏁版嵁缁撴瀯銆佸嚱鏁板師鍨嬩笌瀹忋€?
   :internal:

   :export:

   :export:

## 椹卞姩瀹炵幇


涓烘敮鎸佷竴涓鏁板櫒璁惧锛岄┍鍔ㄥ繀椤婚鍏堥€氳繃 counter_signal 缁撴瀯鍒嗛厤鍙敤鐨?Counter Signal銆傝繖浜?Signal 搴斿瓨鍌ㄤ负鏁扮粍锛屽苟鍦?Counter 娉ㄥ唽鍒扮郴缁熶箣鍓嶏紝璁剧疆鍒板凡鍒嗛厤鐨?counter_device 缁撴瀯鐨?signals 鏁扮粍鎴愬憳涓€?
Counter Count 鍙€氳繃 counter_count 缁撴瀯鍒嗛厤锛岀浉搴旂殑 Counter Signal 鍏宠仈锛圫ynapse锛夐€氳繃 counter_synapse 缁撴瀯寤虹珛銆傚叧鑱旂殑 counter_synapse 缁撴瀯瀛樺偍涓烘暟缁勶紝骞惰缃埌鐩稿簲 counter_count 缁撴瀯鐨?synapses 鏁扮粍鎴愬憳涓€傝繖浜?counter_count 缁撴瀯鍦?Counter 娉ㄥ唽鍒扮郴缁熶箣鍓嶏紝璁剧疆鍒板凡鍒嗛厤鐨?counter_device 缁撴瀯鐨?counts 鏁扮粍鎴愬憳涓€?
蹇呴』鍚?counter_device 缁撴瀯鎻愪緵椹卞姩鍥炶皟浠ヤ究涓庤澶囬€氫俊锛氳鍐欏悇绉?Signal 鍜?Count锛屽苟鍒嗗埆璁剧疆鍜岃幏鍙栧悇绉?Synapse 涓?Count 鐨?"action mode"锛堝姩浣滄ā寮忥級涓?"function mode"锛堝嚱鏁版ā寮忥級銆?
counter_device 缁撴瀯浣跨敤 counter_alloc() 鍒嗛厤锛岀劧鍚庨€氳繃灏嗗叾浼犵粰 counter_add() 鍑芥暟娉ㄥ唽鍒扮郴缁燂紝骞堕€氳繃灏嗗叾浼犵粰 counter_unregister 鍑芥暟娉ㄩ攢銆傚瓨鍦ㄨ繖浜涜澶囩殑鎵樼鍙樹綋锛歞evm_counter_alloc() 涓?devm_counter_add()銆?
struct counter_comp 缁撴瀯鐢ㄤ簬涓?Signal銆丼ynapse 涓?Count 瀹氫箟璁℃暟鍣ㄦ墿灞曘€?
"type" 鎴愬憳鎸囧畾姝ゆ墿灞曟墍澶勭悊鐨勯珮绾ф暟鎹被鍨嬶紙渚嬪 BOOL銆丆OUNT_DIRECTION 绛夛級銆傜劧鍚庯紝璁℃暟鍣ㄨ澶囬┍鍔ㄥ彲浠ラ€氳繃鍥炶皟璁剧疆 "`*_read`" 涓?"`*_write`" 鎴愬憳锛屼互浣跨敤鍘熺敓 C 鏁版嵁绫诲瀷锛堝嵆 u8銆乽64 绛夛級澶勭悊璇ユ暟鎹€?
涓洪┍鍔ㄥ紑鍙戣€呮彁渚涗簡璇稿 `COUNTER_COMP_COUNT_U64` 涔嬬被鐨勪究鎹峰畯銆傜壒鍒槸锛屾湡鏈涢┍鍔ㄥ紑鍙戣€呭鏍囧噯 Counter 瀛愮郴缁熷睘鎬т娇鐢ㄦ墍鎻愪緵鐨勫畯锛屼互渚夸负鐢ㄦ埛绌洪棿缁存寔涓€鑷寸殑鎺ュ彛銆備緥濡傦紝涓€涓鏁板櫒鐨勬墿灞曞畾涔夊涓嬶細

```
        struct counter_comp count_ext[] = {
                COUNTER_COMP_DIRECTION(count_direction_read),
                COUNTER_COMP_ENABLE(count_enable_read, count_enable_write),
                COUNTER_COMP_CEILING(count_ceiling_read, count_ceiling_write),
        };

```
杩欎娇寰楁煡鐪嬨€佹坊鍔犲拰淇敼璇ラ┍鍔ㄦ墍鏀寔鐨勫睘鎬э紙"direction"銆?enable" 涓?"ceiling"锛夊彉寰楃畝鍗曪紝骞朵笖鍙互鍦ㄤ笉鑷充簬杩峰け浜庡眰灞?struct 澶ф嫭鍙风殑鎯呭喌涓嬬淮鎶よ繖娈典唬鐮併€?
鍥炶皟蹇呴』涓庣浉搴旂粍浠舵垨鎵╁睍鎵€鏈熸湜鐨勫嚱鏁扮被鍨嬪尮閰嶃€傝繖浜涘嚱鏁扮被鍨嬪湪 struct counter_comp 缁撴瀯涓畾涔変负 "`**_read`" 涓?"`**_write`" 鑱斿悎鎴愬憳銆?
涓婅堪鎵╁睍瀵瑰簲鐨勫洖璋冨師鍨嬪涓嬶細

```
        int count_direction_read(struct counter_device *counter,
                                 struct counter_count *count,
                                 enum counter_count_direction *direction);
        int count_enable_read(struct counter_device *counter,
                              struct counter_count *count, u8 *enable);
        int count_enable_write(struct counter_device *counter,
                               struct counter_count *count, u8 enable);
        int count_ceiling_read(struct counter_device *counter,
                               struct counter_count *count, u64 *ceiling);
        int count_ceiling_write(struct counter_device *counter,
                                struct counter_count *count, u64 ceiling);

```
纭畾瑕佸垱寤哄摢绉嶇被鍨嬬殑鎵╁睍锛屽彇鍐充簬鍏朵綔鐢ㄨ寖鍥淬€?
- Signal 鎵╁睍鏄毚闇茬壒瀹氫簬鏌愪釜 Signal 鐨勪俊鎭?鎺у埗鐨勫睘鎬с€傝繖绫诲睘鎬у皢瀛樺湪浜?sysfs 涓 Signal 鐨勭洰褰曚笅銆?
  渚嬪锛屽鏋滀綘鏈変竴涓?Signal 鐨勫弽鐩革紙invert锛夌壒鎬э紝浣犲彲浠ュ垱寤轰竴涓悕涓?"invert" 鐨?Signal 鎵╁睍鏉ュ垏鎹㈣鐗规€э細
  /sys/bus/counter/devices/counterX/signalY/invert

- Count 鎵╁睍鏄毚闇茬壒瀹氫簬鏌愪釜 Count 鐨勪俊鎭?鎺у埗鐨勫睘鎬с€傝繖绫诲睘鎬у皢瀛樺湪浜?sysfs 涓 Count 鐨勭洰褰曚笅銆?
  渚嬪锛屽鏋滀綘甯屾湜瀵规煇涓?Count 鐨勬洿鏂拌繘琛屾殏鍋?鎭㈠锛屼綘鍙互鍒涘缓涓€涓悕涓?"enable" 鐨?Count 鎵╁睍鏉ュ垏鎹細
  /sys/bus/counter/devices/counterX/countY/enable

- Device 鎵╁睍鏄毚闇蹭笉鐗瑰畾浜庢煇涓?Count 鎴?Signal 鐨勪俊鎭?鎺у埗鐨勫睘鎬с€備綘鍙互鍦ㄨ繖閲屾斁缃叏灞€鐗规€ф垨鍏朵粬鏉傞」鍔熻兘銆?
  渚嬪锛屽鏋滀綘鐨勮澶囨湁杩囨俯浼犳劅鍣紝浣犲彲浠ラ€氳繃涓€涓悕涓?"error_overtemp" 鐨?Device 鎵╁睍鎶ュ憡鑺墖杩囩儹锛?  /sys/bus/counter/devices/counterX/error_overtemp

## 瀛愮郴缁熸灦鏋?

Counter 椹卞姩浠ュ師鐢熸柟寮忎紶閫掑拰鑾峰彇鏁版嵁锛堝嵆 `u8`銆乣u64` 绛夛級锛岃€屽叡浜殑 counter 妯″潡璐熻矗 sysfs 鎺ュ彛涔嬮棿鐨勮浆鎹€傝繖淇濊瘉浜嗘墍鏈?counter 椹卞姩鐨勬爣鍑嗙敤鎴风┖闂存帴鍙ｏ紝骞堕€氳繃閫氱敤鐨勮澶囬┍鍔?ABI 瀹炵幇浜嗛€氱敤 Counter chrdev 鎺ュ彛銆?
浠ヤ笅绀轰緥璇存槑浜嗚鏁板€煎浣曚粠 counter 椹卞姩鍚戜笅浼犻€掔殑楂樺眰瑙嗗浘銆傞┍鍔ㄥ洖璋冮鍏堟敞鍐屽埌 Counter 鏍稿績缁勪欢锛屼緵鍏朵娇鐢細

```
        Driver callbacks registration:
        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        +----------------------------+
                        | Counter device driver      |
                        +----------------------------+
                        | Processes data from device |
                        +----------------------------+
                                |
                         -------------------
                        / driver callbacks /
                        -------------------
                                |
                                V
                        +----------------------+
                        | Counter core         |
                        +----------------------+
                        | Routes device driver |
                        | callbacks to the     |
                        | userspace interfaces |
                        +----------------------+
                                |
                         -------------------
                        / driver callbacks /
                        -------------------
                                |
                +---------------+---------------+
                |                               |
                V                               V
        +--------------------+          +---------------------+
        | Counter sysfs      |          | Counter chrdev      |
        +--------------------+          +---------------------+
        | Translates to the  |          | Translates to the   |
        | standard Counter   |          | standard Counter    |
        | sysfs output       |          | character device    |
        +--------------------+          +---------------------+

```
姝ゅ悗锛屾暟鎹彲浠ョ洿鎺ュ湪 Counter 璁惧涓庣敤鎴风┖闂翠箣闂翠紶杈擄紝濡備笅鎵€绀猴細

```
        Count data request:
        ~~~~~~~~~~~~~~~~~~~
                         ----------------------
                        / Counter device       \
                        +----------------------+
                        | Count register: 0x28 |
                        +----------------------+
                                |
                         -----------------
                        / raw count data /
                        -----------------
                                |
                                V
                        +----------------------------+
                        | Counter device driver      |
                        +----------------------------+
                        | Processes data from device |
                        |----------------------------|
                        | Type: u64                  |
                        | Value: 42                  |
                        +----------------------------+
                                |
                         ----------
                        / u64     /
                        ----------
                                |
                +---------------+---------------+
                |                               |
                V                               V
        +--------------------+          +---------------------+
        | Counter sysfs      |          | Counter chrdev      |
        +--------------------+          +---------------------+
        | Translates to the  |          | Translates to the   |
        | standard Counter   |          | standard Counter    |
        | sysfs output       |          | character device    |
        |--------------------|          |---------------------|
        | Type: const char * |          | Type: u64           |
        | Value: "42"        |          | Value: 42           |
        +--------------------+          +---------------------+
                |                               |
         ---------------                 -----------------------
        / const char * /                / struct counter_event /
        ---------------                 -----------------------
                |                               |
                |                               V
                |                       +-----------+
                |                       | read      |
                |                       +-----------+
                |                       \ Count: 42 /
                |                        -----------
                |
                V
        +--------------------------------------------------+
        | `/sys/bus/counter/devices/counterX/countY/count` |
        +--------------------------------------------------+
        \ Count: "42"                                      /
         --------------------------------------------------

```
娑夊強鍥涗釜涓昏缁勪欢锛?
### Counter 璁惧椹卞姩

涓庣‖浠惰澶囬€氫俊浠ヨ鍐欐暟鎹紱渚嬪鐢ㄤ簬姝ｄ氦缂栫爜鍣ㄣ€佸畾鏃跺櫒绛夌殑 counter 椹卞姩銆?
### Counter 鏍稿績

灏?counter 璁惧椹卞姩娉ㄥ唽鍒扮郴缁燂紝浠ヤ究鍦ㄧ敤鎴风┖闂翠氦浜掓湡闂磋皟鐢ㄧ浉搴旂殑鍥炶皟銆?
### Counter sysfs

灏?counter 鏁版嵁杞崲涓烘爣鍑?Counter sysfs 鎺ュ彛鏍煎紡锛屽弽涔嬩害鐒躲€?
鏈夊叧鍙敤閫氱敤璁℃暟鍣ㄦ帴鍙?sysfs 灞炴€х殑璇︾粏璇存槑锛岃鍙傞槄 Documentation/ABI/testing/sysfs-bus-counter 鏂囦欢銆?
### Counter chrdev

灏?Counter 浜嬩欢杞崲涓烘爣鍑?Counter 瀛楃璁惧锛涙暟鎹€氳繃鏍囧噯瀛楃璁惧鐨?read 璋冪敤浼犺緭锛岃€?Counter 浜嬩欢閫氳繃 ioctl 璋冪敤閰嶇疆銆?
## Sysfs 鎺ュ彛


閫氱敤璁℃暟鍣ㄦ帴鍙ｄ細鐢熸垚鑻ュ共 sysfs 灞炴€э紝瀹冧滑浣嶄簬 `/sys/bus/counter/devices/counterX` 鐩綍涓嬶紝鍏朵腑 `X` 涓虹浉搴旇鏁板櫒璁惧鐨?id銆傛湁鍏虫瘡涓€氱敤璁℃暟鍣ㄦ帴鍙?sysfs 灞炴€х殑璇︾粏淇℃伅锛岃鍙傞槄 Documentation/ABI/testing/sysfs-bus-counter銆?
閫氳繃杩欎簺 sysfs 灞炴€э紝绋嬪簭涓庤剼鏈彲浠ヤ笌鐩稿簲璁℃暟鍣ㄨ澶囩殑閫氱敤璁℃暟鍣ㄨ寖寮?Count銆丼ignal 涓?Synapse 杩涜浜や簰銆?
## Counter 瀛楃璁惧


Counter 瀛楃璁惧鑺傜偣鍦?`/dev` 鐩綍涓嬩互 `counterX` 鍒涘缓锛屽叾涓?`X` 涓虹浉搴旇鏁板櫒璁惧鐨?id銆傛爣鍑?Counter 鏁版嵁绫诲瀷鐨勫畾涔夐€氳繃鐢ㄦ埛绌洪棿 `include/uapi/linux/counter.h` 鏂囦欢鏆撮湶銆?
### Counter 浜嬩欢

Counter 璁惧椹卞姩鍙互閫氳繃浣跨敤濡備笅鍑芥暟鏀寔 Counter 浜嬩欢锛?
```
        void counter_push_event(struct counter_device *const counter, const u8 event,
                                const u8 channel);

```
浜嬩欢 id 鐢?`event` 鍙傛暟鎸囧畾锛涗簨浠堕€氶亾 id 鐢?`channel` 鍙傛暟鎸囧畾銆傝皟鐢ㄦ鍑芥暟鏃讹紝浼氭敹闆嗕笌鐩稿簲浜嬩欢鍏宠仈鐨?Counter 鏁版嵁锛屽苟涓烘瘡涓暟鎹」鐢熸垚涓€涓?`struct counter_event`锛岀劧鍚庢帹閫佸埌鐢ㄦ埛绌洪棿銆?
Counter 浜嬩欢鍙敱鐢ㄦ埛閰嶇疆锛屼互鎶ュ憡鎰熷叴瓒ｇ殑鍚勭 Counter 鏁版嵁銆傝繖鍙互琚蹇靛寲涓轰竴浠藉緟鎵ц鐨?Counter 缁勪欢 read 璋冪敤鍒楄〃銆備緥濡傦細

        +------------------------+------------------------+
        | COUNTER_EVENT_OVERFLOW | COUNTER_EVENT_INDEX    |
        +========================+========================+
        | Channel 0              | Channel 0              |
        +------------------------+------------------------+
        | ** Count 0              | ** Signal 0             |
        | ** Count 1              | ** Signal 0 Extension 0 |
        | ** Signal 3             | ** Extension 4          |
        | * Count 4 Extension 2  +------------------------+
        | * Signal 5 Extension 0 | Channel 1              |
        |                        +------------------------+
        |                        | * Signal 4             |
        |                        | * Signal 4 Extension 0 |
        |                        | * Count 7              |
        +------------------------+------------------------+

褰撲緥濡傝皟鐢?`counter_push_event(counter, COUNTER_EVENT_INDEX, 1)` 鏃讹紝瀹冧細娌跨潃 `COUNTER_EVENT_INDEX` 浜嬩欢閫氶亾 1 鐨勫垪琛ㄥ悜涓嬫墽琛?Signal 4銆丼ignal 4 Extension 0 涓?Count 7 鐨?read 鍥炶皟鈥斺€斾负姣忎釜杩斿洖鐨勬暟鎹敓鎴愪竴涓?`struct counter_event` 骞舵帹鍏?kfifo锛岀敤鎴风┖闂村彲浠ラ€氳繃瀵圭浉搴斿瓧绗﹁澶囪妭鐐规墽琛屾爣鍑?read 鎿嶄綔鏉ヨ幏鍙栥€?
### 鐢ㄦ埛绌洪棿

鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鍙互閫氳繃瀵?Counter 瀛楃璁惧鑺傜偣鎵ц ioctl 鎿嶄綔鏉ラ厤缃?Counter 浜嬩欢銆備互涓嬫槸鍙楁敮鎸佸苟鐢?`linux/counter.h` 鐢ㄦ埛绌洪棿澶存枃浠舵彁渚涚殑 ioctl 浠ｇ爜锛?
- `COUNTER_ADD_WATCH_IOCTL`

- `COUNTER_ENABLE_EVENTS_IOCTL`

- `COUNTER_DISABLE_EVENTS_IOCTL`

瑕侀厤缃簨浠朵互鏀堕泦 Counter 鏁版嵁锛岀敤鎴烽鍏堢敤鐩稿叧鐨勪簨浠?id銆佷簨浠堕€氶亾 id锛屼互鍙婅浠庝腑璇诲彇鐨勬墍闇€ Counter 缁勪欢鐨勪俊鎭～鍏呬竴涓?`struct counter_watch`锛岀劧鍚庨€氳繃 `COUNTER_ADD_WATCH_IOCTL` ioctl 鍛戒护灏嗗叾浼犲叆銆?
娉ㄦ剰锛岄€氳繃灏?`component.type` 鎴愬憳璁剧疆涓?`COUNTER_COMPONENT_NONE`锛屽彲浠ュ湪涓嶆敹闆?Counter 鏁版嵁鐨勬儏鍐典笅鐩戣涓€涓簨浠躲€傚湪姝ら厤缃笅锛孋ounter 瀛楃璁惧灏嗕粎涓洪偅浜涚浉搴旂殑 `struct counter_event` 鍏冪礌濉厖浜嬩欢鏃堕棿鎴筹紝鑰屽拷鐣ョ粍浠跺€笺€?
`COUNTER_ADD_WATCH_IOCTL` 鍛戒护浼氱紦鍐茶繖浜?Counter watch銆傚噯澶囧氨缁悗锛屽彲浠ヤ娇鐢?`COUNTER_ENABLE_EVENTS_IOCTL` ioctl 鍛戒护婵€娲昏繖浜?Counter watch銆?
鐒跺悗锛岀敤鎴风┖闂村簲鐢ㄧ▼搴忓彲浠ュ湪 Counter 瀛楃璁惧鑺傜偣涓婃墽琛?`read` 鎿嶄綔锛堝彲閫夋嫨鍏堣皟鐢?`poll`锛夛紝浠ユ绱㈠甫鏈夋墍闇€鏁版嵁鐨?`struct counter_event` 鍏冪礌銆?