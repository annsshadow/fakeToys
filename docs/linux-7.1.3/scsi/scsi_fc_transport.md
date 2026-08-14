锘?## SCSI FC 浼犺緭锛圱ransport锛?

Date:  11/18/2008


```

  rports : <<TBS>>
  vports : 2.6.22
  bsg support : 2.6.30 锛圱BD锛?

```

## 绠€浠?

鏈枃浠惰褰曚簡 SCSI FC 浼犺緭锛圱ransport锛夌殑鐗规€у拰缁勪欢銆傚畠涔熸彁渚涗簡
浼犺緭灞備笌 FC LLDD 涔嬮棿鐨?API 鏂囨。銆?

```

  drivers/scsi/scsi_transport_fc.c
  include/scsi/scsi_transport_fc.h
  include/scsi/scsi_netlink_fc.h
  include/scsi/scsi_bsg_fc.h


```

鏈枃浠朵綅浜?Documentation/scsi/scsi_fc_transport.rst


## FC 杩滅▼绔彛锛坮ports锛?

  鍦ㄥ厜绾ら€氶亾锛團ibre Channel锛孎C锛夊瓙绯荤粺涓紝杩滅▼绔彛锛坮port锛夋寚鐨勬槸鏈湴绔彛
  鑳藉涓庝箣閫氫俊鐨勮繙绋嬪厜绾ら€氶亾鑺傜偣銆傚畠浠€氬父鏄瓨鍌ㄧ洰鏍囷紙渚嬪纾佺洏闃靛垪銆佺甯︽満锛夛紝
  閫氳繃 FC 浼犺緭鍝嶅簲 SCSI 鍛戒护銆?
  鍦?Linux 涓紝rports 鐢?FC 浼犺緭绫荤鐞嗭紝骞跺湪 sysfs 涓互濡備笅璺緞琛ㄧず锛?
    /sys/class/fc_remote_ports/

  姣忎釜 rport 鐩綍鍖呭惈鎻忚堪璇ヨ繙绋嬬鍙ｇ殑灞炴€э紝渚嬪绔彛 ID銆佽妭鐐瑰悕銆?  绔彛鐘舵€佸拰閾捐矾閫熷害銆?
  rports 閫氬父鐢?FC 浼犺緭鍦?fabric 鐧诲綍鎴栨壂鎻忚繃绋嬩腑鍙戠幇鏂拌澶囨椂鍒涘缓锛?  骞朵竴鐩村瓨鍦紝鐩村埌璁惧琚Щ闄ゆ垨閾捐矾涓㈠け銆?
  甯歌灞炴€э細
  - node_name锛歐orld Wide Node Name锛圵WNN锛屽叏鐞冭妭鐐瑰悕锛夈€?  - port_name锛歐orld Wide Port Name锛圵WPN锛屽叏鐞冪鍙ｅ悕锛夈€?  - port_id锛氳繙绋嬬鍙ｇ殑 FC 鍦板潃銆?  - roles锛氭寚绀鸿绔彛鏄?initiator锛堝彂璧锋柟锛夈€乼arget锛堢洰鏍囷級锛岃繕鏄袱鑰呭吋澶囥€?  - port_state锛氭樉绀哄綋鍓嶈繍琛岀姸鎬併€?
  鍙戠幇杩滅▼绔彛鍚庯紝椹卞姩閫氬父浼氬～鍏呬竴涓?fc_rport_identifiers 缁撴瀯锛屽苟璋冪敤
  fc_remote_port_add() 鏉ラ€氳繃鍏夌氦閫氶亾锛團C锛変紶杈撶被鍒涘缓璇ヨ繙绋嬬鍙ｅ苟鍚?  SCSI 瀛愮郴缁熸敞鍐屻€?
  rports 涔熷彲浠ラ€氳繃 sysfs 浣滀负 FC 涓绘満閫傞厤鍣ㄧ殑瀛愬璞″彲瑙併€?
  瀵瑰紑鍙戣€呰€岃█锛氬湪瀹炵幇涓?FC 浼犺緭绫讳氦浜掔殑椹卞姩鏃讹紝璇蜂娇鐢?  fc_remote_port_add() 鍜?fc_remote_port_delete()銆?

## FC 铏氭嫙绔彛锛坴ports锛?

### 姒傝堪


  鏂扮殑 FC 鏍囧噯瀹氫箟浜嗗厑璁稿崟涓墿鐞嗙鍙ｈ〃鐜颁负澶氫釜閫氫俊绔彛鐨勬満鍒躲€備娇鐢?  N_Port Id 铏氭嫙鍖栵紙NPIV锛夋満鍒讹紝涓?Fabric 鐨勭偣瀵圭偣杩炴帴鍙互琚垎閰嶅浜?1 涓?  N_Port_ID銆傛瘡涓?N_Port_ID 瀵?fabric 涓婄殑鍏朵粬绔偣鑰岃█琛ㄧ幇涓轰竴涓嫭绔嬬殑绔彛锛?  灏界瀹冧笌浜ゆ崲鏈哄叡浜竴鏉＄墿鐞嗛摼璺繘琛岄€氫俊銆傛瘡涓?N_Port_ID 鍙互鍩轰簬 fabric
  鍒嗗尯锛坺oning锛夊拰闃靛垪 LUN 鎺╃爜鎷ユ湁瀵?fabric 鐨勭嫭鐗硅鍥撅紙灏卞儚鏅€氱殑闈?NPIV
  閫傞厤鍣ㄤ竴鏍凤級銆備娇鐢ㄨ櫄鎷?Fabric锛圴F锛夋満鍒讹紝涓烘瘡涓抚娣诲姞 fabric 澶撮儴浣跨鍙?  鑳藉涓?Fabric Port 浜や簰浠ュ姞鍏ュ涓?fabric銆傜鍙ｅ皢鍦ㄥ叾鍔犲叆鐨勬瘡涓?fabric 涓?  鑾峰緱涓€涓?N_Port_ID銆傛瘡涓?fabric 閮藉皢鎷ユ湁鑷繁瀵圭鐐瑰拰閰嶇疆鍙傛暟鐨勭嫭鐗硅鍥俱€?  NPIV 鍙笌 VF 涓€璧蜂娇鐢紝浠ヤ究绔彛鑳藉湪姣忎釜铏氭嫙 fabric 涓婅幏寰楀涓?N_Port_ID銆?
  FC 浼犺緭鐜板湪寮曞叆浜嗕竴涓柊鐨勫璞♀€斺€攙port銆倂port 鏄竴涓嫢鏈夊叏鐞冨敮涓€鐨?  World Wide Port Name锛坵wpn锛夊拰 World Wide Node Name锛坵wnn锛夌殑瀹炰綋銆備紶杈撳眰
  杩樺厑璁镐负 vport 鎸囧畾 FC4 瑙掕壊锛屽叾涓?FCP_Initiator 鏄鏈熺殑涓昏瑙掕壊銆備竴鏃?  閫氳繃涓婅堪鏌愮鏂规硶瀹炰緥鍖栵紝瀹冨皢鎷ユ湁涓€涓嫭鐗圭殑 N_Port_ID 浠ュ強瀵?fabric 绔偣鍜?  瀛樺偍瀹炰綋鐨勮鍥俱€備笌鐗╃悊閫傞厤鍣ㄥ叧鑱旂殑 fc_host 灏嗗鍑哄垱寤?vport 鐨勮兘鍔涖€備紶杈撳眰
  灏嗗湪 Linux 璁惧鏍戜腑鍒涘缓 vport 瀵硅薄锛屽苟鎸囩ず fc_host 鐨勯┍鍔ㄥ疄渚嬪寲璇ヨ櫄鎷熺鍙ｃ€?  閫氬父锛岄┍鍔ㄤ細鍦?vport 涓婂垱寤轰竴涓柊鐨?scsi_host 瀹炰緥锛屼粠鑰屼负 vport 浜х敓涓€涓?  鐙壒鐨?<H,C,T,L> 鍛藉悕绌洪棿銆傚洜姝わ紝鏃犺 FC 绔彛鏄熀浜庣墿鐞嗙鍙ｈ繕鏄櫄鎷熺鍙ｏ紝
  姣忎釜閮藉皢琛ㄧ幇涓轰竴涓叿鏈夎嚜宸?target 鍜?LUN 绌洪棿鐨勭嫭鐗?scsi_host銆?

```

    At this time, the transport is written to create only NPIV-based
    vports. However, consideration was given to VF-based vports and it
    should be a minor change to add support if needed.  The remaining
    discussion will concentrate on NPIV.

  .. Note::
    World Wide Name assignment (and uniqueness guarantees) are left
    up to an administrative entity controlling the vport. For example,
    if vports are to be associated with virtual machines, a XEN mgmt
    utility would be responsible for creating wwpn/wwnn's for the vport,
    using its own naming authority and OUI. (Note: it already does this
    for virtual MAC addresses).


```

### 璁惧鏍戜笌 Vport 瀵硅薄锛?

  濡備粖锛岃澶囨爲閫氬父鍖呭惈 scsi_host 瀵硅薄锛屽叾涓嬫柟鏄?rports 鍜?scsi target
  瀵硅薄銆傜洰鍓?FC 浼犺緭浼氬垱寤?vport 瀵硅薄锛屽苟灏嗗叾鏀剧疆鍦ㄥ搴斾簬鐗╃悊閫傞厤鍣ㄧ殑
  scsi_host 瀵硅薄涔嬩笅銆侺LDD 浼氫负 vport 鍒嗛厤涓€涓柊鐨?scsi_host锛屽苟灏嗗叾瀵硅薄
  閾炬帴鍒?vport 涔嬩笅銆倂port 鐨?scsi_host 涔嬩笅鐨勫叾浣欐爲缁撴瀯涓庨潪 NPIV 鎯呭喌鐩稿悓銆?  浼犺緭灞傜殑褰撳墠瀹炵幇寰堝鏄撳厑璁?vport 鐨勭埗瀵硅薄涓嶆槸 scsi_host銆傛湭鏉ヨ繖鍙敤浜庡皢
  瀵硅薄閾炬帴鍒扮壒瀹氫簬铏氭嫙鏈虹殑璁惧鏍戙€傚鏋?vport 鐨勭埗瀵硅薄涓嶆槸鐗╃悊绔彛鐨?  scsi_host锛屽垯浼氬湪鐗╃悊绔彛鐨?scsi_host 涓斁缃竴涓寚鍚?vport 瀵硅薄鐨勭鍙烽摼鎺ャ€?
  浠ヤ笅鏄澶囨爲涓彲棰勬湡鐨勫唴瀹癸細


```

     /sys/devices/.../host17/

   and it has the typical descendant tree::

     /sys/devices/.../host17/rport-17:0-0/target17:0:0/17:0:0:0:

   and then the vport is created on the Physical Port::

     /sys/devices/.../host17/vport-17:0-0

   and the vport's Scsi_Host is then created::

     /sys/devices/.../host17/vport-17:0-0/host18

   and then the rest of the tree progresses, such as::

     /sys/devices/.../host17/vport-17:0-0/host18/rport-18:0-0/target18:0:0/18:0:0:0:

  Here's what to expect in the sysfs tree::

   scsi_hosts:
     /sys/class/scsi_host/host17                physical port's scsi_host
     /sys/class/scsi_host/host18                vport's scsi_host
   fc_hosts:
     /sys/class/fc_host/host17                  physical port's fc_host
     /sys/class/fc_host/host18                  vport's fc_host
   fc_vports:
     /sys/class/fc_vports/vport-17:0-0          the vport's fc_vport
   fc_rports:
     /sys/class/fc_remote_ports/rport-17:0-0    rport on the physical port
     /sys/class/fc_remote_ports/rport-18:0-0    rport on the vport


```

### Vport 灞炴€?

  鏂扮殑 fc_vport 绫诲璞″叿鏈変互涓嬪睘鎬э細

     node_name:                                                 Read_Only
       vport 鐨?WWNN銆?
     port_name:                                                 Read_Only
       vport 鐨?WWPN銆?
     roles:                                                     Read_Only
       鎸囩ず鍦?vport 涓婂惎鐢ㄧ殑 FC4 瑙掕壊銆?
     symbolic_name:                                             Read_Write
       涓€涓瓧绗︿覆锛岄檮鍔犲埌椹卞姩鐨?symbolic port name 瀛楃涓蹭箣鍚庯紝璇ュ瓧绗︿覆
       浼氳娉ㄥ唽鍒颁氦鎹㈡満浠ユ爣璇?vport銆備緥濡傦紝hypervisor 鍙互灏嗘瀛楃涓茶缃负
       "Xen Domain 2 VM 5 Vport 2"锛岃繖缁勬爣璇嗙鍙湪浜ゆ崲鏈虹鐞嗙晫闈笂鐪嬪埌锛?       鐢ㄤ互鏍囪瘑璇ョ鍙ｃ€?
     vport_delete:                                              Write_Only
       鍐欏叆 "1" 鏃讹紝灏嗘媶闄よ vport銆?
     vport_disable:                                            Write_Only
       鍐欏叆 "1" 鏃讹紝灏嗘妸 vport 杞崲涓?disabled锛堢鐢級鐘舵€併€?       璇?vport 浠嶄細鍦?Linux 鍐呮牳涓疄渚嬪寲锛屼絾涓嶄細鍦?FC 閾捐矾涓婂浜庢椿鍔ㄧ姸鎬併€?       鍐欏叆 "0" 鏃讹紝灏嗗惎鐢ㄨ vport銆?
     vport_last_state:                                         Read_Only
       鎸囩ず vport 鐨勫墠涓€涓姸鎬併€傚弬瑙佷笅鏂団€淰port 鐘舵€佲€濅竴鑺傘€?
     vport_state:                                              Read_Only
       鎸囩ず vport 鐨勭姸鎬併€傚弬瑙佷笅鏂団€淰port 鐘舵€佲€濅竴鑺傘€?
     vport_type:                                               Read_Only
       鍙嶆槧鐢ㄤ簬鍒涘缓璇ヨ櫄鎷熺鍙ｇ殑 FC 鏈哄埗銆?       鐩墠浠呮敮鎸?NPIV銆?

  瀵逛簬 fc_host 绫诲璞★紝涓?vports 娣诲姞浜嗕互涓嬪睘鎬э細

     max_npiv_vports:                                          Read_Only
       鎸囩ず椹卞姩/閫傞厤鍣ㄥ湪璇?fc_host 涓婅兘澶熸敮鎸佺殑鍩轰簬 NPIV 鐨?vport 鐨勬渶澶ф暟閲忋€?
     npiv_vports_inuse:                                        Read_Only
       鎸囩ず宸插湪 fc_host 涓婂疄渚嬪寲鐨勫熀浜?NPIV 鐨?vport 鏁伴噺銆?
     vport_create:                                             Write_Only
       涓€涓€滅畝鍗曗€濈殑鍒涘缓鎺ュ彛锛岀敤浜庡湪 fc_host 涓婂疄渚嬪寲涓€涓?vport銆?       鍚戣灞炴€у啓鍏ヤ竴涓?"<WWPN>:<WWNN>" 瀛楃涓层€傞殢鍚庝紶杈撳眰浼氬疄渚嬪寲 vport 瀵硅薄锛?       骞惰皟鐢?LLDD 浠?FCP_Initiator 瑙掕壊鍒涘缓璇?vport銆傛瘡涓?WWN 鎸囧畾涓?16 涓?       鍗佸叚杩涘埗瀛楃锛屼笖**涓嶈兘**鍖呭惈浠讳綍鍓嶇紑锛堜緥濡?0x銆亁 绛夛級銆?
     vport_delete:                                             Write_Only
       涓€涓€滅畝鍗曗€濈殑鍒犻櫎鎺ュ彛锛岀敤浜庢媶闄や竴涓?vport銆傚悜璇ュ睘鎬у啓鍏ヤ竴涓?       "<WWPN>:<WWNN>" 瀛楃涓层€備紶杈撳眰浼氬湪 fc_host 涓婃壘鍒板叿鏈夌浉鍚?WWN 鐨?vport
       骞跺皢鍏舵媶闄ゃ€傛瘡涓?WWN 鎸囧畾涓?16 涓崄鍏繘鍒跺瓧绗︼紝涓?*涓嶈兘**鍖呭惈浠讳綍鍓嶇紑
       锛堜緥濡?0x銆亁 绛夛級銆?

### Vport 鐘舵€?

  Vport 瀹炰緥鍖栫敱涓ら儴鍒嗙粍鎴愶細

    - 涓庡唴鏍稿拰 LLDD 涓€璧峰垱寤恒€傝繖鎰忓懗鐫€鎵€鏈変紶杈撳眰鍜岄┍鍔ㄧ殑鏁版嵁缁撴瀯琚缓绔嬶紝
      骞朵笖璁惧瀵硅薄琚垱寤恒€傝繖绛夋晥浜庡湪閫傞厤鍣ㄤ笂鐨勯┍鍔ㄢ€渁ttach锛堥檮鍔狅級鈥濓紝
      瀹冧笌閫傞厤鍣ㄧ殑閾捐矾鐘舵€佹棤鍏炽€?    - 閫氳繃 ELS 娴侀噺绛夊湪 FC 閾捐矾涓婂疄渚嬪寲 vport銆傝繖绛夋晥浜庘€渓ink up锛堥摼璺氨缁級鈥?      浠ュ強鎴愬姛鐨勯摼璺垵濮嬪寲銆?
  鏇村淇℃伅鍙湪涓嬫枃鐨?Vport Creation 鎺ュ彛涓€鑺備腑鎵惧埌銆?
  涓€鏃?vport 宸蹭笌鍐呮牳/LLDD 涓€璧峰疄渚嬪寲锛屽氨鍙互閫氳繃 sysfs 灞炴€ф姤鍛?vport 鐘舵€併€?  瀛樺湪浠ヤ笅鍑犵鐘舵€侊細

    FC_VPORT_UNKNOWN            - Unknown锛堟湭鐭ワ級
      涓€涓复鏃剁姸鎬侊紝閫氬父浠呭湪 vport 姝ｅ湪涓庡唴鏍稿拰 LLDD 涓€璧峰疄渚嬪寲鏃惰缃€?
    FC_VPORT_ACTIVE             - Active锛堟椿鍔級
      vport 宸叉垚鍔熷湪 FC 閾捐矾涓婂垱寤恒€傚畠鍔熻兘瀹屽銆?
    FC_VPORT_DISABLED           - Disabled锛堢鐢級
      vport 宸插疄渚嬪寲锛屼絾澶勪簬鈥渄isabled鈥濈姸鎬併€傝 vport 鏈湪 FC 閾捐矾涓婂疄渚嬪寲銆?      杩欑瓑鏁堜簬閾捐矾鈥渄own锛堟柇寮€锛夆€濈殑鐗╃悊绔彛銆?
    FC_VPORT_LINKDOWN           - Linkdown锛堥摼璺柇寮€锛?      vport 涓嶅彲杩愯锛屽洜涓虹墿鐞嗛摼璺笉鍙繍琛屻€?
    FC_VPORT_INITIALIZING       - Initializing锛堝垵濮嬪寲涓級
      vport 姝ｅ湪 FC 閾捐矾涓婂疄渚嬪寲鐨勮繃绋嬩腑銆侺LDD 灏嗗湪寮€濮嬬敤浜庡垱寤?vport 鐨?      ELS 娴侀噺涔嬪墠璁剧疆姝ょ姸鎬併€傛鐘舵€佸皢鎸佺画锛岀洿鍒?vport 鎴愬姛鍒涘缓锛堢姸鎬佸彉涓?      FC_VPORT_ACTIVE锛夋垨澶辫触锛堢姸鎬佸彉涓轰笅杩版煇涓€硷級銆傜敱浜庢鐘舵€佹槸鐬€佺殑锛?      瀹冧笉浼氳淇濈暀鍦?"vport_last_state" 涓€?
    FC_VPORT_NO_FABRIC_SUPP     - No Fabric Support锛堟棤 Fabric 鏀寔锛?      vport 涓嶅彲杩愯銆傞亣鍒颁簡浠ヤ笅鏉′欢涔嬩竴锛?
       - FC 鎷撴墤涓嶆槸鐐瑰鐐癸紙Point-to-Point锛夈€?       - FC 绔彛鏈繛鎺ュ埌 F_Port銆?       - F_Port 琛ㄧず涓嶆敮鎸?NPIV銆?
    FC_VPORT_NO_FABRIC_RSCS     - No Fabric Resources锛堟棤 Fabric 璧勬簮锛?      vport 涓嶅彲杩愯銆侳abric 鐨?FDISC 澶辫触锛屽叾鐘舵€佽〃鏄庡畠娌℃湁瓒冲鐨勮祫婧愭潵瀹屾垚
      璇ユ搷浣溿€?
    FC_VPORT_FABRIC_LOGOUT      - Fabric Logout锛團abric 娉ㄩ攢锛?      vport 涓嶅彲杩愯銆侳abric 宸插涓庤 vport 鍏宠仈鐨?N_Port_ID 鎵ц浜?LOGO銆?
    FC_VPORT_FABRIC_REJ_WWN     - Fabric Rejected WWN锛團abric 鎷掔粷 WWN锛?      vport 涓嶅彲杩愯銆侳abric 鐨?FDISC 澶辫触锛屽叾鐘舵€佽〃鏄?WWN 鏃犳晥銆?
    FC_VPORT_FAILED             - VPort Failed锛圴Port 澶辫触锛?      vport 涓嶅彲杩愯銆傝繖鏄墍鏈夊叾浠栭敊璇潯浠剁殑鍏滃簳鐘舵€併€?

  浠ヤ笅鐘舵€佽〃鍒楀嚭浜嗕笉鍚岀殑鐘舵€佽浆鎹細

   +------------------+--------------------------------+---------------------+
   | State            | Event                          | New State           |
   +==================+================================+=====================+
   | n/a              | Initialization                 | Unknown             |
   +------------------+--------------------------------+---------------------+
   | Unknown:         | Link Down                      | Linkdown            |
   |                  +--------------------------------+---------------------+
   |                  | Link Up & Loop                 | No Fabric Support   |
   |                  +--------------------------------+---------------------+
   |                  | Link Up & no Fabric            | No Fabric Support   |
   |                  +--------------------------------+---------------------+
   |                  | Link Up & FLOGI response       | No Fabric Support   |
   |                  | indicates no NPIV support      |                     |
   |                  +--------------------------------+---------------------+
   |                  | Link Up & FDISC being sent     | Initializing        |
   |                  +--------------------------------+---------------------+
   |                  | Disable request                | Disable             |
   +------------------+--------------------------------+---------------------+
   | Linkdown:        | Link Up                        | Unknown             |
   +------------------+--------------------------------+---------------------+
   | Initializing:    | FDISC ACC                      | Active              |
   |                  +--------------------------------+---------------------+
   |                  | FDISC LS_RJT w/ no resources   | No Fabric Resources |
   |                  +--------------------------------+---------------------+
   |                  | FDISC LS_RJT w/ invalid        | Fabric Rejected WWN |
   |		      | pname or invalid nport_id      |                     |
   |                  +--------------------------------+---------------------+
   |                  | FDISC LS_RJT failed for        | Vport Failed        |
   |                  | other reasons                  |                     |
   |                  +--------------------------------+---------------------+
   |                  | Link Down                      | Linkdown            |
   |                  +--------------------------------+---------------------+
   |                  | Disable request                | Disable             |
   +------------------+--------------------------------+---------------------+
   | Disable:         | Enable request                 | Unknown             |
   +------------------+--------------------------------+---------------------+
   | Active:          | LOGO received from fabric      | Fabric Logout       |
   |                  +--------------------------------+---------------------+
   |                  | Link Down                      | Linkdown            |
   |                  +--------------------------------+---------------------+
   |                  | Disable request                | Disable             |
   +------------------+--------------------------------+---------------------+
   | Fabric Logout:   | Link still up                  | Unknown             |
   +------------------+--------------------------------+---------------------+

```

    No Fabric Support:
    No Fabric Resources:
    Fabric Rejected WWN:
    Vport Failed:
                        Disable request                 Disable
                        Link goes down                  Linkdown


```

### 浼犺緭灞?<-> LLDD 鎺ュ彛


LLDD 瀵?vport 鐨勬敮鎸侊細

  LLDD 閫氳繃鍦ㄤ紶杈撴ā鏉夸腑鎻愪緵 vport_create() 鍑芥暟鏉ヨ〃鏄庡 vports 鐨勬敮鎸併€?  璇ュ嚱鏁扮殑瀛樺湪浼氬鑷村湪 fc_host 涓婂垱寤烘柊鐨勫睘鎬с€備綔涓虹墿鐞嗙鍙ｇ浉瀵逛簬浼犺緭灞?  瀹屾垚鍏跺垵濮嬪寲鐨勪竴閮ㄥ垎锛屽畠搴斿綋璁剧疆 max_npiv_vports 灞炴€э紝浠ユ寚绀洪┍鍔ㄥ拰/鎴?  閫傞厤鍣ㄦ墍鏀寔鐨?vport 鐨勬渶澶ф暟閲忋€?

Vport 鍒涘缓锛圴port Creation锛夛細

```

      int vport_create(struct fc_vport *vport, bool disable)

  where:

      =======   ===========================================================
      vport     Is the newly allocated vport object
      disable   If "true", the vport is to be created in a disabled stated.
                If "false", the vport is to be enabled upon creation.
      =======   ===========================================================

  When a request is made to create a new vport (via sgio/netlink, or the
  vport_create fc_host attribute), the transport will validate that the LLDD
  can support another vport (e.g. max_npiv_vports > npiv_vports_inuse).
  If not, the create request will be failed.  If space remains, the transport
  will increment the vport count, create the vport object, and then call the
  LLDD's vport_create() function with the newly allocated vport object.

  As mentioned above, vport creation is divided into two parts:

    - Creation with the kernel and LLDD. This means all transport and
      driver data structures are built up, and device objects created.
      This is equivalent to a driver "attach" on an adapter, which is
      independent of the adapter's link state.
    - Instantiation of the vport on the FC link via ELS traffic, etc.
      This is equivalent to a "link up" and successful link initialization.

  The LLDD's vport_create() function will not synchronously wait for both
  parts to be fully completed before returning. It must validate that the
  infrastructure exists to support NPIV, and complete the first part of
  vport creation (data structure build up) before returning.  We do not
  hinge vport_create() on the link-side operation mainly because:

    - The link may be down. It is not a failure if it is. It simply
      means the vport is in an inoperable state until the link comes up.
      This is consistent with the link bouncing post vport creation.
    - The vport may be created in a disabled state.
    - This is consistent with a model where:  the vport equates to a
      FC adapter. The vport_create is synonymous with driver attachment
      to the adapter, which is independent of link state.

  .. Note::

      special error codes have been defined to delineate infrastructure
      failure cases for quicker resolution.

  The expected behavior for the LLDD's vport_create() function is:

    - Validate Infrastructure:

        - If the driver or adapter cannot support another vport, whether
            due to improper firmware, (a lie about) max_npiv, or a lack of
            some other resource - return VPCERR_UNSUPPORTED.
        - If the driver validates the WWN's against those already active on
            the adapter and detects an overlap - return VPCERR_BAD_WWN.
        - If the driver detects the topology is loop, non-fabric, or the
            FLOGI did not support NPIV - return VPCERR_NO_FABRIC_SUPP.

    - Allocate data structures. If errors are encountered, such as out
        of memory conditions, return the respective negative Exxx error code.
    - If the role is FCP Initiator, the LLDD is to :

        - Call scsi_host_alloc() to allocate a scsi_host for the vport.
        - Call scsi_add_host(new_shost, &vport->dev) to start the scsi_host
          and bind it as a child of the vport device.
        - Initializes the fc_host attribute values.

    - Kick of further vport state transitions based on the disable flag and
        link state - and return success (zero).

  LLDD Implementers Notes:

  - It is suggested that there be a different fc_function_templates for
    the physical port and the virtual port.  The physical port's template
    would have the vport_create, vport_delete, and vport_disable functions,
    while the vports would not.
  - It is suggested that there be different scsi_host_templates
    for the physical port and virtual port. Likely, there are driver
    attributes, embedded into the scsi_host_template, that are applicable
    for the physical port only (link speed, topology setting, etc). This
    ensures that the attributes are applicable to the respective scsi_host.


```

Vport 绂佺敤/鍚敤锛圴port Disable/Enable锛夛細

```

      int vport_disable(struct fc_vport *vport, bool disable)

  where:

      =======   =======================================
      vport     Is vport to be enabled or disabled
      disable   If "true", the vport is to be disabled.
                If "false", the vport is to be enabled.
      =======   =======================================

  When a request is made to change the disabled state on a vport, the
  transport will validate the request against the existing vport state.
  If the request is to disable and the vport is already disabled, the
  request will fail. Similarly, if the request is to enable, and the
  vport is not in a disabled state, the request will fail.  If the request
  is valid for the vport state, the transport will call the LLDD to
  change the vport's state.

  Within the LLDD, if a vport is disabled, it remains instantiated with
  the kernel and LLDD, but it is not active or visible on the FC link in
  any way. (see Vport Creation and the 2 part instantiation discussion).
  The vport will remain in this state until it is deleted or re-enabled.
  When enabling a vport, the LLDD reinstantiates the vport on the FC
  link - essentially restarting the LLDD statemachine (see Vport States
  above).


```

Vport 鍒犻櫎锛圴port Deletion锛夛細

```

      int vport_delete(struct fc_vport *vport)

  where:

      vport:    Is vport to delete

  When a request is made to delete a vport (via sgio/netlink, or via the
  fc_host or fc_vport vport_delete attributes), the transport will call
  the LLDD to terminate the vport on the FC link, and teardown all other
  datastructures and references.  If the LLDD completes successfully,
  the transport will teardown the vport objects and complete the vport
  removal.  If the LLDD delete request fails, the vport object will remain,
  but will be in an indeterminate state.

  Within the LLDD, the normal code paths for a scsi_host teardown should
  be followed. E.g. If the vport has a FCP Initiator role, the LLDD
  will call fc_remove_host() for the vports scsi_host, followed by
  scsi_remove_host() and scsi_host_put() for the vports scsi_host.


```

鍏朵粬锛圤ther锛夛細
  fc_host port_type 灞炴€э細
    鏈変竴涓柊鐨?fc_host port_type 鍙栧€尖€斺€擣C_PORTTYPE_NPIV銆傛鍙栧€煎繀椤诲湪鎵€鏈?    鍩轰簬 vport 鐨?fc_host 涓婅缃€傞€氬父锛屽湪鐗╃悊绔彛涓婏紝port_type 灞炴€т細鍩轰簬
    鎷撴墤绫诲瀷鍜?fabric 鐨勫瓨鍦ㄨ璁剧疆涓?NPORT銆丯LPORT 绛夈€傜敱浜庤繖涓嶉€傜敤浜?vport锛?    鍥犳鎶ュ憡鐢ㄤ簬鍒涘缓璇?vport 鐨?FC 鏈哄埗鏇翠负鍚堢悊銆?
  椹卞姩鍗歌浇锛圖river unload锛夛細
    FC 椹卞姩鍦ㄨ璋冪敤 scsi_remove_host() 涔嬪墠蹇呴』鍏堣皟鐢?fc_remove_host()銆?    杩欏厑璁?fc_host 鍦?scsi_host 琚媶闄や箣鍓嶅厛鎷嗛櫎鎵€鏈夎繙绋嬬鍙ｃ€俧c_remove_host()
    璋冪敤涔熷凡鏇存柊锛屼細鍚屾椂绉婚櫎璇?fc_host 鐨勬墍鏈?vport銆?

### 浼犺緭灞傛彁渚涚殑鍑芥暟


The following functions are supplied by the FC-transport for use by LLDs.

   ==================   =========================
   fc_vport_create      create a vport
   fc_vport_terminate   detach and remove a vport
   ==================   =========================

```

    /**
    * fc_vport_create - Admin App or LLDD requests creation of a vport
    * @shost:     scsi host the virtual port is connected to.
    * @ids:       The world wide names, FC4 port roles, etc for
    *              the virtual port.
    *
    * Notes:
    *     This routine assumes no locks are held on entry.
    */
    struct fc_vport *
    fc_vport_create(struct Scsi_Host *shost, struct fc_vport_identifiers *ids)

    /**
    * fc_vport_terminate - Admin App or LLDD requests termination of a vport
    * @vport:      fc_vport to be terminated
    *
    * Calls the LLDD vport_delete() function, then deallocates and removes
    * the vport from the shost and object tree.
    *
    * Notes:
    *      This routine assumes no locks are held on entry.
    */
    int
    fc_vport_terminate(struct fc_vport *vport)


```

## FC BSG 鏀寔锛圕T & ELS 閫忎紶锛屼互鍙婃洿澶氾級


<< To Be Supplied >>



## 鑷磋阿


The following people have contributed to this document:





James Smart
james.smart@broadcom.com
