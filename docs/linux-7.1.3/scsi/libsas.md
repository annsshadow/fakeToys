
## SAS 灞?

SAS 灞傛槸涓€涓鐞嗗熀纭€璁炬柦锛岀敤浜庣鐞?SAS LLDD銆傚畠浣嶄簬 SCSI Core 涓?SAS LLDD 涔嬮棿銆傚叾甯冨眬濡備笅锛歋CSI Core 鍏虫敞 SAM/SPC 鐩稿叧浜嬮」锛岃€?SAS LLDD+瀹氬簭鍣ㄥ叧娉?phy/OOB/閾捐矾绠＄悊锛孲AS 灞傚垯鍏虫敞锛?
      - SAS Phy/Port/HA 浜嬩欢绠＄悊锛堢敱 LLDD 浜х敓锛岀敱 SAS 灞傚鐞嗭級锛?      - SAS 绔彛绠＄悊锛堝垱寤?閿€姣侊級锛?      - SAS 鍩熷彂鐜颁笌閲嶆柊楠岃瘉锛?      - SAS 鍩熻澶囩鐞嗭紝
      - SCSI 涓绘満娉ㄥ唽/娉ㄩ攢锛?      - 鍚?SCSI Core锛圫AS锛夋垨 libata锛圫ATA锛夋敞鍐岃澶囷紝浠ュ強
      - 鎵╁睍鍣ㄧ鐞嗗苟鍚戠敤鎴风┖闂村鍑烘墿灞曞櫒鎺у埗銆?
SAS LLDD 鏄竴涓?PCI 璁惧椹卞姩銆傚畠鍏虫敞 phy/OOB 绠＄悊銆佸巶鍟嗙浉鍏充换鍔★紝骞跺悜 SAS 灞備骇鐢熶簨浠躲€?
SAS 灞傚畬鎴愪簡 SAS 1.1 瑙勮寖涓杩扮殑澶ч儴鍒?SAS 浠诲姟銆?
sas_ha_struct 鍚?SAS 灞傛弿杩?SAS LLDD銆傚畠鐨勫ぇ閮ㄥ垎鐢?SAS 灞備娇鐢紝浣嗗皯鏁板瓧娈甸渶瑕佺敱 LLDD 鍒濆鍖栥€?
鍦ㄥ垵濮嬪寲瀹岀‖浠朵箣鍚庯紝浣犱粠 probe() 鍑芥暟涓皟鐢?sas_register_ha()銆傚畠浼氬皢浣犵殑 LLDD 娉ㄥ唽鍒?SCSI 瀛愮郴缁燂紝鍒涘缓涓€涓?SCSI 涓绘満锛屽苟灏嗕綘鐨?SAS 椹卞姩娉ㄥ唽鍒板畠鍒涘缓鐨?sysfs SAS 鏍戜腑銆傜劧鍚庡畠杩斿洖銆傛帴鐫€浣犲惎鐢ㄤ綘鐨?phys 浠ュ疄闄呭紑濮?OOB锛堟鏃朵綘鐨勯┍鍔ㄥ皢寮€濮嬭皟鐢?notify_* 浜嬩欢鍥炶皟锛夈€?
## 缁撴瀯浣撹鏄?

### ``struct sas_phy``


閫氬父鎯呭喌涓嬪畠闈欐€佸唴宓屽湪浣犵殑椹卞姩鐨?
```
    struct my_phy {
	    blah;
	    struct sas_phy sas_phy;
	    bleh;
    };
```

涔嬩腑锛岄殢鍚庢墍鏈?phys 閮芥槸浣?HA struct 涓?my_phy 鐨勬暟缁勶紙濡備笅鎵€绀猴級銆?
鐒跺悗闅忕潃浣犻€愭鍒濆鍖栦綘鐨?phys锛屼綘涔熷悓鏃跺垵濮嬪寲 sas_phy struct锛屼互鍙婁綘鑷繁鐨?phy 缁撴瀯銆?
涓€鑸€岃█锛宲hys 鐢?LLDD 绠＄悊锛岀鍙ｇ敱 SAS 灞傜鐞嗐€傚洜姝?phys 鐢?LLDD 鍒濆鍖栧拰鏇存柊锛岀鍙ｇ敱 SAS 灞傚垵濮嬪寲鍜屾洿鏂般€?
瀛樺湪涓€绉嶆満鍒讹細LLDD 鍙互璇诲啓鏌愪簺瀛楁锛岃€?SAS 灞傚彧鑳借鍙栬繖浜涘瓧娈碉紝鍙嶄箣浜︾劧銆傚叾鐩殑鍦ㄤ簬閬垮厤涓嶅繀瑕佺殑鍔犻攣銆?
enabled
    - 蹇呴』璁剧疆锛?/1锛?
id
    - 蹇呴』璁剧疆 [0,MAX_PHYS)]

class, proto, type, role, oob_mode, linkrate
    - 蹇呴』璁剧疆

oob_mode
    - 浣犲湪 OOB 瀹屾垚鍚庤缃椤癸紝鐒跺悗閫氱煡 SAS 灞傘€?
sas_addr
    - 閫氬父鎸囧悜涓€涓暟缁勶紝璇ユ暟缁勪繚瀛樹簡 phy 鐨?sas 鍦板潃锛屽彲鑳戒綅浜庝綘鐨?my_phy struct 涓殑鏌愬銆?
attached_sas_addr
    - 褰撲綘锛圠LDD锛夋敹鍒颁竴涓?IDENTIFY 甯ф垨 FIS 甯ф椂锛屽湪閫氱煡 SAS 灞?_涔嬪墠_ 璁剧疆姝ら」銆傚叾鎬濊矾鏄紝鏈夋椂 LLDD 鍙兘甯屾湜鍦ㄩ偅涓?phy/绔彛涓婁吉閫犳垨鎻愪緵涓€涓笉鍚岀殑 SAS 鍦板潃锛岃繖鍏佽瀹冭繖鏍峰仛銆傛渶濂芥儏鍐典笅锛屼綘搴斿綋浠?IDENTIFY 甯т腑澶嶅埗 sas 鍦板潃锛屾垨鑰呭鐩存帴杩炴帴鐨?SATA 璁惧鐢熸垚涓€涓?SAS 鍦板潃銆傝鍊肩◢鍚庡彲鑳借 Discover 杩囩▼鏀瑰彉銆?
frame_rcvd
    - 杩欐槸浣犳敹鍒?IDENTIFY/FIS 甯ф椂澶嶅埗瀹冪殑浣嶇疆锛涗綘鍔犻攣銆佸鍒躲€佽缃?frame_rcvd_size 骞惰В閿佽閿侊紝鐒跺悗璋冪敤浜嬩欢銆傚畠鏄竴涓寚閽堬紝鍥犱负鏃犳硶 _绮剧‘_ 鐭ラ亾浣犵殑纭欢甯уぇ灏忥紝鎵€浠ヤ綘鍦ㄤ綘鐨?phy struct 涓畾涔夊疄闄呯殑鏁扮粍骞惰璇ユ寚閽堟寚鍚戝畠銆備綘鍦ㄨ閿佺殑淇濇姢涓嬪皢甯т粠浣犵殑鍙?DMA 鍐呭瓨澶嶅埗鍒拌鍖哄煙銆?
sas_prim
    - 杩欐槸鏀跺埌鍘熻鏃跺畠浠墍鍘诲線鐨勪綅缃€傚弬瑙?sas.h銆傝幏鍙栭攣锛岃缃師璇紝閲婃斁閿侊紝鐒跺悗閫氱煡銆?
port
    - 濡傛灉璇?phy 灞炰簬鏌愪釜绔彛锛屽垯瀹冩寚鍚?sas_port鈥斺€擫LDD 鍙姝ら」銆傚畠鎸囧悜璇?phy 鎵€灞炵殑 sas_port銆傜敱 SAS 灞傝缃€?
ha
    - 鍙互璁剧疆锛汼AS 灞傛棤璁哄浣曢兘浼氳缃畠銆?
lldd_phy
    - 浣犲簲褰撳皢姝ら」璁剧疆涓烘寚鍚戜綘鐨?phy锛岃繖鏍峰綋 SAS 灞傝皟鐢ㄤ綘鐨勬煇涓洖璋冨苟浼犵粰浣犱竴涓?phy 鏃讹紝浣犲彲浠ユ洿蹇湴鎵惧埌浣嶇疆銆傚鏋?sas_phy 鏄唴宓岀殑锛屼綘涔熷彲浠ヤ娇鐢?container_of鈥斺€旈殢浣犲枩娆€?

### ``struct sas_port``


LLDD 涓嶈缃缁撴瀯浣撶殑浠讳綍瀛楁鈥斺€斿畠鍙鍙栧畠浠€傚畠浠簲褰撴槸涓嶈█鑷槑鐨勩€?
phy_mask 鏄?32 浣嶇殑锛岀洰鍓嶈繖搴斿綋瓒冲锛屽洜涓烘垜杩樻病鍚杩囨湁瓒呰繃 8 涓?phys 鐨?HA銆?
lldd_port
    - 鎴戣繕娌℃壘鍒板畠鐨勭敤閫斺€斺€斾篃璁稿叾浠栧笇鏈涙嫢鏈夊唴閮ㄧ鍙ｈ〃绀虹殑 LLDD 鍙互鍒╃敤瀹冦€?
### ``struct sas_ha_struct``


瀹冮€氬父鍦ㄤ綘鑷繁鐨?LLDD 涓潤鎬佸０鏄庯細

```
    struct my_sas_ha {
	blah;
	struct sas_ha_struct sas_ha;
	struct my_phy phys[MAX_PHYS];
	struct sas_port sas_ports[MAX_PHYS]; /* (1) */
	bleh;
    };

    (1) 濡傛灉浣犵殑 LLDD 娌℃湁鑷繁鐨勭鍙ｈ〃绀恒€?```

闇€瑕佸垵濮嬪寲鍝簺鍐呭锛堢ず渚嬪嚱鏁拌涓嬶級銆?
##### pcidev


sas_addr
       - 鐢变簬 SAS 灞備笉鎯冲鐞嗗唴瀛樺垎閰嶇瓑浜嬪姟锛屾椤规寚鍚戞煇澶勯潤鎬佸垎閰嶇殑鏁扮粍锛堟瘮濡傚湪浣犵殑涓绘満閫傞厤鍣ㄧ粨鏋勪腑锛夛紝骞朵繚瀛樼敱浣犳垨鍒堕€犲晢绛夌粰鍑虹殑涓绘満閫傞厤鍣ㄧ殑 SAS 鍦板潃銆?
##### sas_port


sas_phy
      - 涓€涓寚鍚戠粨鏋勪綋鐨勬寚閽堟暟缁勩€傦紙鍙傝涓婇潰鍏充簬 sas_addr 鐨勮鏄庯級銆?	杩欎簺蹇呴』璁剧疆銆傛洿澶氳鏄庤涓嬨€?
num_phys
       - sas_phy 鏁扮粍涓瓨鍦ㄧ殑 phys 鏁伴噺锛屼互鍙?sas_port 鏁扮粍涓瓨鍦ㄧ殑绔彛鏁伴噺銆傛渶澶氬彲浠ユ湁 num_phys 涓鍙ｏ紙姣忎釜绔彛涓€涓級锛屽洜姝ゆ垜浠幓鎺?num_ports锛屽彧浣跨敤 num_phys銆?
```
	/* LLDD 璋冪敤杩欎簺鏉ラ€氱煡绫诲彂鐢熶簡涓€涓簨浠躲€?*/
	void sas_notify_port_event(struct sas_phy *, enum port_event, gfp_t);
	void sas_notify_phy_event(struct sas_phy *, enum phy_event, gfp_t);
```
```
	/* 绫昏皟鐢ㄨ繖浜涙潵閫氱煡 LLDD 鍙戠敓浜嗕竴涓簨浠躲€?*/
	void (*lldd_port_formed)(struct sas_phy *);
	void (*lldd_port_deformed)(struct sas_phy *);
```
濡傛灉 LLDD 甯屾湜鍦ㄧ鍙ｅ舰鎴愭垨閿€姣佹椂鏀跺埌閫氱煡锛屽畠灏嗚繖涓ら」璁剧疆涓烘弧瓒宠绫诲瀷鐨勫嚱鏁般€?
涓€涓?SAS LLDD 杩樺簲褰撹嚦灏戝疄鐜颁笅鍒椾换鍔＄鐞嗗嚱鏁颁腑鐨勪竴涓細

```
	/* 浠诲姟绠＄悊鍑芥暟銆傚繀椤讳粠杩涚▼涓婁笅鏂囪皟鐢ㄣ€?*/
	int (*lldd_abort_task)(struct sas_task *);
	int (*lldd_abort_task_set)(struct domain_device *, u8 *lun);
	int (*lldd_clear_task_set)(struct domain_device *, u8 *lun);
	int (*lldd_I_T_nexus_reset)(struct domain_device *);
	int (*lldd_lu_reset)(struct domain_device *, u8 *lun);
	int (*lldd_query_task)(struct sas_task *);
```
鏇村淇℃伅璇烽槄璇?T10.org 涓婄殑 SAM銆?
```
	/* 绔彛涓庨€傞厤鍣ㄧ鐞?*/
	int (*lldd_clear_nexus_port)(struct sas_port *);
	int (*lldd_clear_nexus_ha)(struct sas_ha_struct *);
```
涓€涓?SAS LLDD 搴斿綋鑷冲皯瀹炵幇鍏朵腑涔嬩竴銆?
```
	/* Phy 绠＄悊 */
	int (*lldd_control_phy)(struct sas_phy *, enum phy_func);
```
lldd_ha
    - 灏嗘璁剧疆涓烘寚鍚戜綘鐨?HA struct銆傚鏋滀綘鍍忎笂闈㈤偅鏍峰唴宓屼簡瀹冿紝涔熷彲浠ヤ娇鐢?container_of銆?
涓€涓ず渚嬬殑鍒濆鍖栦笌娉ㄥ唽鍑芥暟鍙互鍍忚繖鏍凤紙浠?probe() 鏈€鍚庤皟鐢級锛?
```
    static int register_sas_ha(struct my_sas_ha *my_ha)
    {
	    int i;
	    static struct sas_phy   *sas_phys[MAX_PHYS];
	    static struct sas_port  *sas_ports[MAX_PHYS];

	    my_ha->sas_ha.sas_addr = &my_ha->sas_addr[0];

	    for (i = 0; i < MAX_PHYS; i++) {
		    sas_phys[i] = &my_ha->phys[i].sas_phy;
		    sas_ports[i] = &my_ha->sas_ports[i];
	    }

	    my_ha->sas_ha.sas_phy  = sas_phys;
	    my_ha->sas_ha.sas_port = sas_ports;
	    my_ha->sas_ha.num_phys = MAX_PHYS;

	    my_ha->sas_ha.lldd_port_formed = my_port_formed;

	    my_ha->sas_ha.lldd_dev_found = my_dev_found;
	    my_ha->sas_ha.lldd_dev_gone = my_dev_gone;

	    my_ha->sas_ha.lldd_execute_task = my_execute_task;

	    my_ha->sas_ha.lldd_abort_task     = my_abort_task;
	    my_ha->sas_ha.lldd_abort_task_set = my_abort_task_set;
	    my_ha->sas_ha.lldd_clear_task_set = my_clear_task_set;
	    my_ha->sas_ha.lldd_I_T_nexus_reset= NULL; (2)
	    my_ha->sas_ha.lldd_lu_reset       = my_lu_reset;
	    my_ha->sas_ha.lldd_query_task     = my_query_task;

	    my_ha->sas_ha.lldd_clear_nexus_port = my_clear_nexus_port;
	    my_ha->sas_ha.lldd_clear_nexus_ha = my_clear_nexus_ha;

	    my_ha->sas_ha.lldd_control_phy = my_control_phy;

	    return sas_register_ha(&my_ha->sas_ha);
    }
```
(2) SAS 1.1 娌℃湁瀹氫箟 I_T Nexus Reset TMF銆?
## 浜嬩欢


浜嬩欢鏄?SAS LLDD 閫氱煡 SAS 灞備换浣曚簨鎯呯殑 **鍞竴鏂瑰紡**銆侺LDD 娌℃湁鍒殑鏂规硶鎴栭€斿緞鏉ュ憡璇?SAS 灞傚叾鍐呴儴鎴?SAS 鍩熶腑鍙戠敓鐨勪换浣曚簨鎯呫€?
```
	PHYE_LOSS_OF_SIGNAL, (C)
	PHYE_OOB_DONE,
	PHYE_OOB_ERROR,      (C)
	PHYE_SPINUP_HOLD.
```
```
	PORTE_BYTES_DMAED,      (M)
	PORTE_BROADCAST_RCVD,   (E)
	PORTE_LINK_RESET_ERR,   (C)
	PORTE_TIMER_EVENT,      (C)
	PORTE_HARD_RESET.
```
涓绘満閫傞厤鍣ㄤ簨浠讹細
	HAE_RESET

涓€涓?SAS LLDD 搴斿綋鑳藉浜х敓

 - 鏉ヨ嚜 C 缁勶紙鍙€夛級鐨勮嚦灏戜竴涓簨浠讹紝
 - 鏍囪涓?M锛堝己鍒讹級鐨勪簨浠舵槸寮哄埗鐨勶紙鍙湁涓€涓級锛? - 鏍囪涓?E锛堟墿灞曞櫒锛夌殑浜嬩欢锛屽鏋滃畠甯屾湜 SAS 灞傚鐞嗗煙閲嶆柊楠岃瘉锛堝彧鏈変竴涓繖鏍风殑浜嬩欢锛夈€? - 鏈爣璁扮殑浜嬩欢鏄彲閫夌殑銆?
鍚箟锛?
HAE_RESET
    - 褰撲綘鐨?HA 鍙戠敓鍐呴儴閿欒骞惰閲嶇疆鏃躲€?
PORTE_BYTES_DMAED
    - 鏀跺埌 IDENTIFY/FIS 甯ф椂

PORTE_BROADCAST_RCVD
    - 鏀跺埌涓€涓師璇椂

PORTE_LINK_RESET_ERR
    - 瀹氭椂鍣ㄨ秴鏃躲€佷俊鍙蜂涪澶便€丏WS 涓㈠け绛?[^1^]_

PORTE_TIMER_EVENT
    - DWS 閲嶇疆瓒呮椂瀹氭椂鍣ㄥ埌鏈?[^1^]_

PORTE_HARD_RESET
    - 鏀跺埌纭浣嶅師璇€?
PHYE_LOSS_OF_SIGNAL
    - 璁惧娑堝け浜?[^1^]_

PHYE_OOB_DONE
    - OOB 椤哄埄瀹屾垚涓?oob_mode 鏈夋晥

PHYE_OOB_ERROR
    - 杩涜 OOB 鏃跺嚭閿欙紝璁惧鍙兘宸叉柇寮€杩炴帴銆俒^1^]_

PHYE_SPINUP_HOLD
    - 瀛樺湪 SATA锛屼絾鏈彂閫?COMWAKE銆?
       鎴栬€呬篃鍙互浠庡畠浠殑 tasklet 涓皟鐢ㄥ唴鑱旂殑 sas_phy_disconnected()锛屽畠鍙槸涓€涓緟鍔╁嚱鏁般€?
```
	int (*lldd_execute_task)(struct sas_task *, gfp_t gfp_flags);
```
鐢ㄤ簬鍚?SAS LLDD 鎺掗槦涓€涓换鍔°€侤task 鏄琚墽琛岀殑浠诲姟銆侤gfp_mask 鏄畾涔夎皟鐢ㄦ柟涓婁笅鏂囩殑 gfp_mask銆?
璇ュ嚱鏁板簲褰撳疄鐜?Execute Command SCSI RPC锛?
涔熷氨鏄锛屽綋璋冪敤 lldd_execute_task() 鏃讹紝鍛戒护 **绔嬪嵆** 鍦ㄤ紶杈撳眰涓婂彂鍑恒€傚湪 SAS LLDD 涓?**涓嶅瓨鍦?* 浠讳綍绉嶇被銆佷换浣曞眰娆＄殑鎺掗槦銆?
杩斿洖锛?
   - -SAS_QUEUE_FULL銆?ENOMEM锛屾湭鎺掗槦浠讳綍鍐呭锛?   - 0锛屼换鍔″凡鎺掗槦銆?
```
    struct sas_task {
	    dev -- 璇ヤ换鍔℃墍鍙戝線鐨勮澶?	    task_proto -- enum sas_proto 涓殑 _涓€涓猒
	    scatter -- 鎸囧悜鍒嗘暎/鑱氶泦鍒楄〃鏁扮粍鐨勬寚閽?	    num_scatter -- scatter 涓殑鍏冪礌涓暟
	    total_xfer_len -- 棰勬湡浼犺緭鐨勬€诲瓧鑺傛暟
	    data_dir -- PCI_DMA_...
	    task_done -- 浠诲姟鎵ц瀹屾垚鏃剁殑鍥炶皟
    };
```

## 鍙戠幇


sysfs 鏍戞湁浠ヤ笅鐢ㄩ€旓細

    a) 瀹冩樉绀哄綋鍓嶆椂鍒?SAS 鍩熺殑鐗╃悊甯冨眬锛屽嵆姝ゅ埢鍩熷湪鐗╃悊涓栫晫涓殑鏍峰瓙銆?    b) 鏄剧ず _鍙戠幇鏃禵 鐨勬煇浜涜澶囧弬鏁般€?
杩欐槸涓€涓寚鍚?tree(1) 绋嬪簭鐨勯摼鎺ワ紝鍦ㄦ煡鐪?SAS 鍩熸椂闈炲父鏈夌敤锛?ftp://mama.indstate.edu/linux/tree/

鎴戞湡鏈涚敤鎴风┖闂村簲鐢ㄧ▼搴忓疄闄呭垱寤哄畠鐨勫浘褰㈢晫闈€?
涔熷氨鏄锛宻ysfs 鍩熸爲涓嶆樉绀轰篃涓嶄繚瀛樼姸鎬侊紝渚嬪濡傛灉浣犳敼鍙樹簡 READY LED MEANING 璁剧疆鐨勫惈涔夛紝浣嗗畠纭疄鏄剧ず鍩熻澶囩殑褰撳墠杩炴帴鐘舵€併€?
淇濆瓨鍐呴儴璁惧鐘舵€佸彉鍖栫殑璐ｄ换鍦ㄤ笂灞傦紙鍛戒护闆嗛┍鍔級鍜岀敤鎴风┖闂淬€?
褰撹澶囨垨璁惧浠粠鍩熶腑鎷斿嚭鏃讹紝杩欎細绔嬪嵆鍙嶆槧鍦?sysfs 鏍戜腑锛屽苟涓旇璁惧锛堜滑锛変細浠庣郴缁熶腑绉婚櫎銆?
domain_device 缁撴瀯鎻忚堪 SAS 鍩熶腑鐨勪换浣曡澶囥€傚畠瀹屽叏鐢?SAS 灞傜鐞嗐€備竴涓换鍔℃寚鍚戜竴涓煙璁惧锛孲AS LLDD 鐢辨鐭ラ亾灏嗕换鍔″彂寰€浣曞銆係AS LLDD 鍙鍙?domain_device 缁撴瀯鐨勫唴瀹癸紝浣嗕粠涓嶅垱寤烘垨閿€姣佸畠銆?
## 鏉ヨ嚜鐢ㄦ埛绌洪棿鐨勬墿灞曞櫒绠＄悊


鍦?sysfs 涓瘡涓墿灞曞櫒鐩綍涓嬶紝閮芥湁涓€涓悕涓?"smp_portal" 鐨勬枃浠躲€傚畠鏄竴涓簩杩涘埗 sysfs 灞炴€ф枃浠讹紝瀹炵幇浜嗕竴涓?SMP portal锛堟敞鎰忥細杩?**涓嶆槸** 涓€涓?SMP 绔彛锛夛紝鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鍙互鍚戝叾鍙戦€?SMP 璇锋眰骞舵帴鏀?SMP 鍝嶅簲銆?
鍏跺姛鑳界湅浼肩畝鍗曞疄鍒欎笉鐒讹細

1. 鏋勫缓浣犳兂鍙戦€佺殑 SMP 甯с€傛牸寮忓拰甯冨眬鍦?SAS 瑙勮寖涓弿杩般€傚皢 CRC 瀛楁鐣欎负 0銆?
open(2)

2. 浠ヨ鍐欐ā寮忔墦寮€鎵╁睍鍣ㄧ殑 SMP portal sysfs 鏂囦欢銆?
write(2)

3. 鍐欏叆浣犲湪绗?1 姝ユ瀯寤虹殑甯с€?
read(2)

4. 璇诲彇浣犳湡鏈涗负鎵€鏋勫缓甯ф帴鏀剁殑鏁版嵁閲忋€傚鏋滀綘鏀跺埌鐨勬暟鎹噺涓庢湡鏈涚殑涓嶅悓锛屽垯鍙戠敓浜嗘煇绉嶉敊璇€?
close(2)

鏁翠釜杩囩▼鍦ㄥ嚱鏁?do_smp_func() 鍙婂叾璋冪敤鑰呬腑鏈夎缁嗗睍绀猴紝浣嶄簬 "expander_conf.c" 鏂囦欢涓€?
鍐呮牳鍔熻兘瀹炵幇鍦?"sas_expander.c" 鏂囦欢涓€?
绋嬪簭 "expander_conf.c" 瀹炵幇浜嗘鍔熻兘銆傚畠鎺ュ彈涓€涓弬鏁帮紝鍗虫寚鍚戞墿灞曞櫒鐨?SMP portal 鐨?sysfs 鏂囦欢鍚嶏紝骞剁粰鍑烘墿灞曞櫒淇℃伅锛屽寘鎷矾鐢辫〃銆?
SMP portal 璁╀綘瀹屽叏鎺у埗鎵╁睍鍣紝鎵€浠ヨ灏忓績銆?