
## SCSI EH锛圫CSI 閿欒澶勭悊锛?

鏈枃妗ｆ弿杩颁簡 SCSI 涓棿灞傦紙midlayer锛夌殑閿欒澶勭悊鍩虹璁炬柦銆?鏈夊叧 SCSI 涓棿灞傜殑鏇村淇℃伅锛岃鍙傞槄 Documentation/scsi/scsi_mid_low_api.rst銆?

   [^1^] SCSI 鍛戒护濡備綍绌胯繃涓棿灞傚苟杩涘叆 EH
       [1-1] struct scsi_cmnd
       [1-2] scmd 鏄浣曡瀹屾垚鐨勶紵
   	[1-2-1] 鐢?scsi_done 瀹屾垚涓€涓?scmd
   	[1-2-2] 鐢ㄨ秴鏃跺畬鎴愪竴涓?scmd
       [1-3] EH 濡備綍鎺ョ
   [^2^] SCSI EH 濡備綍宸ヤ綔
       [2-1] 閫氳繃缁嗙矑搴﹀洖璋冪殑 EH
   	[2-1-1] 姒傝堪
   	[2-1-2] scmd 娴佺粡 EH 鐨勮繃绋?   	[2-1-3] 鎺у埗娴?       [2-2] 閫氳繃 transportt->eh_strategy_handler() 鐨?EH
   	[2-2-1] transportt->eh_strategy_handler() 涔嬪墠鐨?SCSI 涓棿灞傛潯浠?   	[2-2-2] transportt->eh_strategy_handler() 涔嬪悗鐨?SCSI 涓棿灞傛潯浠?   	[2-2-3] 闇€瑕佽€冭檻鐨勪簨椤?

## 1. SCSI 鍛戒护濡備綍绌胯繃涓棿灞傚苟杩涘叆 EH


### 1.1 struct scsi_cmnd


姣忎釜 SCSI 鍛戒护閮界敤 struct scsi_cmnd锛堝嵆 scmd锛夎〃绀恒€備竴涓?scmd 鏈変袱涓?list_head 灏嗚嚜宸遍摼鎺ヨ繘閾捐〃銆傝繖涓や釜鍒嗗埆鏄?scmd->list 涓?scmd->eh_entry銆?鍓嶈€呯敤浜庣┖闂查摼琛ㄦ垨姣忚澶囧垎閰嶇殑 scmd 閾捐〃锛屽湪鏈 EH 璁ㄨ涓苟涓嶉噸瑕併€傚悗鑰?鐢ㄤ簬瀹屾垚涓?EH 閾捐〃锛岄櫎闈炲彟鏈夎鏄庯紝鏈璁轰腑 scmd 鎬绘槸閫氳繃 scmd->eh_entry
閾炬帴銆?

### 1.2 scmd 鏄浣曡瀹屾垚鐨勶紵


涓€鏃?LLDD 鍙栧緱涓€涓?scmd锛岃涔堢敱 LLDD 璋冪敤鍦ㄨ皟鐢?hostt->queuecommand() 鏃?浠庝腑闂村眰浼犲叆鐨?scsi_done 鍥炶皟鏉ュ畬鎴愬懡浠わ紝瑕佷箞鐢卞潡灞傚皢鍏惰秴鏃躲€?

##### 1.2.1 鐢?scsi_done 瀹屾垚涓€涓?scmd


瀵逛簬鎵€鏈夐潪 EH 鍛戒护锛宻csi_done() 鏄畬鎴愬洖璋冦€傚畠鍙槸璋冪敤
blk_mq_complete_request() 鏉ュ垹闄ゅ潡灞傚畾鏃跺櫒骞惰Е鍙?BLOCK_SOFTIRQ銆?
BLOCK_SOFTIRQ 闂存帴璋冪敤 scsi_complete()锛屽悗鑰呰皟鐢?scsi_decide_disposition()
鏉ュ喅瀹氬浣曞鐞嗚鍛戒护銆俿csi_decide_disposition() 鏌ョ湅 scmd->result 鍊间笌
sense 鏁版嵁鏉ュ喅瀹氬浣曞鐞嗚鍛戒护銆?
 - SUCCESS锛堟垚鍔燂級

	涓鸿鍛戒护璋冪敤 scsi_finish_command()銆傝鍑芥暟鍋氫竴浜涚淮鎶ゅ伐浣滐紝鐒跺悗璋冪敤
	scsi_io_completion() 鏉ュ畬鎴?I/O銆俿csi_io_completion() 閫氳繃璋冪敤
	blk_end_request 鍙婂叾鐩稿叧鍑芥暟鏉ラ€氱煡鍧楀眰璇ヨ姹傚凡瀹屾垚锛屾垨鑰呭湪鍑洪敊鏃?	寮勬竻妤氬浣曞鐞嗗墿浣欑殑鏁版嵁銆?
 - NEEDS_RETRY锛堥渶瑕侀噸璇曪級

 - ADD_TO_MLQUEUE锛堝姞鍏ヤ腑闂村眰闃熷垪锛?
	scmd 琚噸鏂板叆闃熷埌 blk 闃熷垪銆?
 - otherwise锛堝叾浠栨儏鍐碉級

	涓鸿鍛戒护璋冪敤 scsi_eh_scmd_add(scmd)銆傝鍑芥暟鐨勭粏鑺傚弬瑙?[1-3]銆?

##### 1.2.2 鐢ㄨ秴鏃跺畬鎴愪竴涓?scmd


瓒呮椂澶勭悊鍑芥暟鏄?scsi_timeout()銆傚綋鍙戠敓瓒呮椂鏃讹紝璇ュ嚱鏁?
 1. 璋冪敤鍙€夌殑 hostt->eh_timed_out() 鍥炶皟銆傝繑鍥炲€煎彲浠ユ槸涓嬪垪涔嬩竴

    - SCSI_EH_RESET_TIMER锛堥噸缃畾鏃跺櫒锛?	琛ㄧず闇€瑕佹洿澶氭椂闂存潵瀹屾垚鍛戒护銆傚畾鏃跺櫒琚噸鏂板惎鍔ㄣ€?
    - SCSI_EH_NOT_HANDLED锛堟湭澶勭悊锛?        eh_timed_out() 鍥炶皟娌℃湁澶勭悊璇ュ懡浠ゃ€傞噰鍙栫 2 姝ャ€?
    - SCSI_EH_DONE锛堝凡瀹屾垚锛?        eh_timed_out() 瀹屾垚浜嗚鍛戒护銆?
 2. 璋冪敤 scsi_abort_command() 鏉ヨ皟搴︿竴涓紓姝ヤ腑姝紝瀹冨彲鑳戒細閲嶈瘯
    scmd->allowed + 1 娆°€傚浜庡凡缁忚缃簡 SCSI_EH_ABORT_SCHEDULED 鏍囧織鐨勫懡浠?    锛堣繖琛ㄦ槑璇ュ懡浠ゅ凡缁忚涓杩囦竴娆★紝鑰岃繖鏄竴娆″け璐ョ殑閲嶈瘯锛夈€佸綋閲嶈瘯娆℃暟
    鐢ㄥ敖鏃躲€佹垨褰?EH 鎴鏃堕棿宸茶繃鏈熸椂锛屼笉浼氳皟鐢ㄥ紓姝ヤ腑姝€傚湪杩欎簺鎯呭喌涓嬮噰鍙?    绗?3 姝ャ€?
 3. 涓鸿鍛戒护璋冪敤 scsi_eh_scmd_add(scmd)銆傛洿澶氫俊鎭弬瑙?[1-4]銆?
### 1.3 寮傛鍛戒护涓


 瓒呮椂鍙戠敓鍚庯紝浼氫粠 scsi_abort_command() 璋冨害涓€娆″懡浠や腑姝€傚鏋滀腑姝㈡垚鍔燂紝
 璇ュ懡浠よ涔堣閲嶈瘯锛堝鏋滈噸璇曟鏁板皻鏈敤灏斤級锛岃涔堜互 DID_TIME_OUT 缁堟銆?
 鍚﹀垯涓鸿鍛戒护璋冪敤 scsi_eh_scmd_add()銆傛洿澶氫俊鎭弬瑙?[1-4]銆?
### 1.4 EH 濡備綍鎺ョ


scmd 閫氳繃 scsi_eh_scmd_add() 杩涘叆 EH锛岃鍑芥暟鎵ц浠ヤ笅鎿嶄綔銆?
 1. 灏?scmd->eh_entry 閾炬帴鍒?shost->eh_cmd_q

 2. 璁剧疆 shost->shost_state 涓殑 SHOST_RECOVERY 浣?
 3. 閫掑 shost->host_failed

 4. 褰?shost->host_busy == shost->host_failed 鏃跺敜閱?SCSI EH 绾跨▼

濡備笂鎵€瑙侊紝涓€鏃︽湁浠讳綍 scmd 琚姞鍏?shost->eh_cmd_q锛孲HOST_RECOVERY
shost_state 浣嶅氨浼氳鎵撳紑銆傝繖浼氶樆姝换浣曟柊鐨?scmd 浠?blk 闃熷垪涓嬪彂鍒颁富鏈猴紱
鏈€缁堬紝涓绘満涓婄殑鎵€鏈?scmd 瑕佷箞姝ｅ父瀹屾垚锛岃涔堝け璐ュ苟琚姞鍏?eh_cmd_q锛岃涔?瓒呮椂骞惰鍔犲叆 shost->eh_cmd_q銆?
濡傛灉鎵€鏈?scmd 閮藉畬鎴愭垨澶辫触锛屽湪閫?scmd 鐨勬暟閲忓氨浼氱瓑浜庡け璐ョ殑 scmd 鏁伴噺鈥斺€?鍗?shost->host_busy == shost->host_failed銆傝繖浼氬敜閱?SCSI EH 绾跨▼銆傚洜姝わ紝涓€鏃?琚敜閱掞紝SCSI EH 绾跨▼鍙互棰勬湡鎵€鏈夊湪閫斿懡浠ら兘宸插け璐ュ苟閾炬帴鍦?shost->eh_cmd_q 涓娿€?
娉ㄦ剰锛岃繖骞朵笉琛ㄧず搴曞眰宸茬粡闈欐銆傚鏋?LLDD 浠ヤ竴涓敊璇姸鎬佸畬鎴愪簡涓€涓?scmd锛屽垯
鍋囧畾 LLDD 涓庡簳灞傚湪閭ｄ竴鍒诲凡缁忛仐蹇樹簡璇?scmd銆傜劧鑰岋紝濡傛灉涓€涓?scmd 瓒呮椂浜嗭紝闄ら潪
hostt->eh_timed_out() 璁╁簳灞傞仐蹇樹簡璇?scmd锛堢洰鍓嶆病鏈変换浣?LLDD 杩欐牱鍋氾級锛屽惁鍒?灏卞簳灞傝€岃█璇ュ懡浠や粛鐒舵槸娲昏穬鐨勶紝骞朵笖闅忔椂鍙兘瀹屾垚銆傚綋鐒讹紝鐢变簬瀹氭椂鍣ㄥ凡缁忚繃鏈燂紝
鎵€鏈夎繖浜涘畬鎴愰兘浼氳蹇界暐銆?
鎴戜滑绋嶅悗璁ㄨ SCSI EH 濡備綍閲囧彇琛屽姩鏉ヤ腑姝⑩€斺€旇 LLDD 閬楀繕鈥斺€旇秴鏃剁殑 scmd銆?

## 2. SCSI EH 濡備綍宸ヤ綔


LLDD 鍙互閫氳繃浠ヤ笅涓ょ鏂瑰紡涔嬩竴鏉ュ疄鐜?SCSI EH 鍔ㄤ綔銆?
 - Fine-grained EH callbacks锛堢粏绮掑害 EH 鍥炶皟锛?	LLDD 鍙互瀹炵幇缁嗙矑搴︾殑 EH 鍥炶皟锛屽苟璁?SCSI 涓棿灞傞┍鍔ㄩ敊璇鐞嗭紝
	璋冪敤閫傚綋鐨勫洖璋冦€傝繖灏嗗湪 [2-1] 涓繘涓€姝ヨ璁恒€?
 - eh_strategy_handler() callback锛坋h_strategy_handler() 鍥炶皟锛?	杩欐槸涓€涓ぇ鐨勫洖璋冿紝搴斿綋鎵ц鏁翠釜閿欒澶勭悊銆傚洜姝わ紝瀹冨簲褰撳畬鎴?SCSI
	涓棿灞傚湪鎭㈠鏈熼棿鎵ц鐨勬墍鏈夋潅鍔°€傝繖灏嗗湪 [2-2] 涓璁恒€?
涓€鏃︽仮澶嶅畬鎴愶紝SCSI EH 閫氳繃璋冪敤 scsi_restart_operations() 鎭㈠姝ｅ父杩愯锛岃鍑芥暟

 1. 妫€鏌ユ槸鍚﹂渶瑕侀攣闂ㄥ苟閿侀棬銆?
 2. 娓呴櫎 SHOST_RECOVERY shost_state 浣?
 3. 鍞ら啋鍦?shost->host_wait 涓婄瓑寰呯殑杩涚▼銆傝繖鍙戠敓鍦ㄦ湁浜哄涓绘満璋冪敤
    scsi_block_when_processing_errors() 鏃躲€傦紙**鐤戦棶** 涓轰粈涔堥渶瑕佸畠锛熷湪鍒拌揪
    blk 闃熷垪涔嬪悗锛屾墍鏈夋搷浣滄棤璁哄浣曢兘浼氳闃诲銆傦級

 4. 韪㈠姩涓绘満涓婃墍鏈夎澶囦腑鐨勯槦鍒?

### 2.1 EH through fine-grained callbacks锛堥€氳繃缁嗙矑搴﹀洖璋冪殑 EH锛?

##### 2.1.1 姒傝堪


濡傛灉涓嶅瓨鍦?eh_strategy_handler()锛孲CSI 涓棿灞傝礋璐ｉ┍鍔ㄩ敊璇鐞嗐€侲H 鏈変袱涓洰鏍団€斺€?璁?LLDD銆佷富鏈轰笌璁惧閬楀繕瓒呮椂鐨?scmd锛屽苟璁╁畠浠噯澶囧ソ鎺ュ彈鏂板懡浠ゃ€傚綋涓€涓?scmd 琚?搴曞眰閬楀繕銆佷笖搴曞眰鍑嗗濂藉啀娆″鐞嗘垨澶辫触璇?scmd 鏃讹紝绉拌 scmd 宸茶鎭㈠銆?
涓轰簡瀹炵幇杩欎簺鐩爣锛孍H 浠ラ€掑鐨勪弗閲嶆€ф墽琛屾仮澶嶅姩浣溿€傛湁浜涘姩浣滈€氳繃鍙戝嚭 SCSI 鍛戒护
鏉ユ墽琛岋紝鍙︿竴浜涘垯閫氳繃璋冪敤涓嬪垪缁嗙矑搴?hostt EH 鍥炶皟涔嬩竴鏉ユ墽琛屻€傚洖璋冨彲浠ヨ鐪佺暐锛?琚渷鐣ョ殑鍥炶皟琚涓烘€绘槸澶辫触銆?
```
    int (* eh_abort_handler)(struct scsi_cmnd *);
    int (* eh_device_reset_handler)(struct scsi_cmnd *);
    int (* eh_bus_reset_handler)(struct scsi_cmnd *);
    int (* eh_host_reset_handler)(struct scsi_cmnd *);

```
涓ラ噸鎬ф洿楂樼殑鍔ㄤ綔鍙湁鍦ㄤ弗閲嶆€ф洿浣庣殑鍔ㄤ綔鏃犳硶鎭㈠閮ㄥ垎澶辫触鐨?scmd 鏃舵墠浼氶噰鍙栥€?鍙﹁娉ㄦ剰锛屾渶楂樹弗閲嶆€у姩浣滅殑澶辫触鎰忓懗鐫€ EH 澶辫触锛屽苟瀵艰嚧鎵€鏈夋湭鎭㈠鐨勮澶囪涓嬬嚎銆?
鍦ㄦ仮澶嶆湡闂达紝閬靛惊浠ヤ笅瑙勫垯

 - 鎭㈠鍔ㄤ綔鍦ㄥ緟鍔炲垪琛?eh_work_q 涓婂け璐ョ殑 scmd 涓婃墽琛屻€傚鏋滄煇涓仮澶嶅姩浣滃
   涓€涓?scmd 鎴愬姛锛屽凡鎭㈠鐨?scmd 浼氫粠 eh_work_q 涓Щ闄ゃ€?
   娉ㄦ剰锛屽鍗曚釜 scmd 鐨勪竴涓仮澶嶅姩浣滃彲浠ユ仮澶嶅涓?scmd銆備緥濡傦紝閲嶇疆涓€涓澶?   浼氭仮澶嶈璁惧涓婃墍鏈夊け璐ョ殑 scmd銆?
 - 鍙湁褰撲綆涓ラ噸鎬у姩浣滃畬鎴愬悗 eh_work_q 闈炵┖鏃讹紝鎵嶉噰鍙栨洿楂樹弗閲嶆€х殑鍔ㄤ綔銆?
 - EH 澶嶇敤澶辫触鐨?scmd 鏉ュ彂鍑虹敤浜庢仮澶嶇殑鍛戒护銆傚浜庤秴鏃剁殑 scmd锛孲CSI EH 纭繚鍦?   澶嶇敤鍏惰繘琛?EH 鍛戒护涔嬪墠锛孡LDD 宸茬粡閬楀繕浜嗚 scmd銆?
褰撲竴涓?scmd 琚仮澶嶆椂锛屼娇鐢?scsi_eh_finish_cmd() 灏嗗叾浠?eh_work_q 绉诲姩鍒?EH
鏈湴鐨?eh_done_q銆傚湪鎵€鏈?scmd 閮借鎭㈠锛坋h_work_q 涓虹┖锛夊悗锛岃皟鐢?scsi_eh_flush_done_q() 鏉ラ噸璇曟垨閿欒瀹屾垚锛堝悜涓婂眰閫氱煡澶辫触锛夊凡鎭㈠鐨?scmd銆?
褰撲笖浠呭綋鍏?sdev 浠嶇劧鍦ㄧ嚎锛堟湭鍦?EH 鏈熼棿琚笅绾匡級銆佹湭璁剧疆 REQ_FAILFAST銆佷笖
++scmd->retries 灏忎簬 scmd->allowed 鏃讹紝scmd 鎵嶄細琚噸璇曘€?

##### 2.1.2 Flow of scmds through EH锛坰cmd 娴佺粡 EH 鐨勮繃绋嬶級


 1. 閿欒瀹屾垚 / 瓒呮椂

    :ACTION: 涓鸿 scmd 璋冪敤 scsi_eh_scmd_add()

 - 灏?scmd 鍔犲叆 shost->eh_cmd_q
 - 璁剧疆 SHOST_RECOVERY
 - shost->host_failed++

    :LOCKING: shost->host_lock

 2. EH 鍚姩

    :ACTION: 灏嗘墍鏈?scmd 绉诲姩鍒?EH 鏈湴鐨?eh_work_q銆俿host->eh_cmd_q 琚竻绌恒€?
    :LOCKING: shost->host_lock锛堝苟闈炰弗鏍煎繀瑕侊紝浠呬负涓€鑷存€э級

 3. scmd 宸叉仮澶?
    :ACTION: 璋冪敤 scsi_eh_finish_cmd() 鏉?EH-瀹屾垚璇?scmd

 - 浠庢湰鍦?eh_work_q 绉诲姩鍒版湰鍦?eh_done_q

    :LOCKING: none锛堟棤锛?
    :CONCURRENCY: 姣忎釜鐙珛鐨?eh_work_q 鏈€澶氫竴涓嚎绋嬶紝浠ヤ繚鎸侀槦鍒楁搷浣滅殑
		  鏃犻攣鎬?
 4. EH 瀹屾垚

    :ACTION: scsi_eh_flush_done_q() 閲嶈瘯 scmd 鎴栧悜涓婂眰閫氱煡澶辫触銆傚彲浠ュ苟鍙?	    璋冪敤锛屼絾姣忎釜鐙珛鐨?eh_work_q 蹇呴』鏈€澶氬彧鏈変竴涓嚎绋嬶紝浠ユ棤閿佹柟寮?	    鎿嶄綔闃熷垪

      - scmd 浠?eh_done_q 涓Щ闄わ紝骞舵竻闄?scmd->eh_entry
      - 濡傛灉闇€瑕侀噸璇曪紝浣跨敤 scsi_queue_insert() 閲嶆柊鍏ラ槦璇?scmd
      - 鍚﹀垯锛屼负璇?scmd 璋冪敤 scsi_finish_command()
      - 灏?shost->host_failed 娓呴浂

    :LOCKING: 闃熷垪鎴栧畬鎴愬嚱鏁版墽琛岄€傚綋鐨勫姞閿?

##### 2.1.3 Flow of control锛堟帶鍒舵祦锛?

 閫氳繃缁嗙矑搴﹀洖璋冪殑 EH 浠?scsi_unjam_host() 寮€濮嬨€?
`scsi_unjam_host`

    1. 閿佸畾 shost->host_lock锛屽皢 shost->eh_cmd_q splice_init 鍒版湰鍦?       eh_work_q锛屽苟瑙ｉ攣 host_lock銆傛敞鎰忥紝shost->eh_cmd_q 浼氳姝ゅ姩浣滄竻绌恒€?
    2. 璋冪敤 scsi_eh_get_sense銆?
    `scsi_eh_get_sense`

	瀵逛簬姣忎釜娌℃湁鏈夋晥 sense 鏁版嵁鐨勯敊璇畬鎴愬懡浠わ紝浼氶噰鍙栨鍔ㄤ綔銆傚ぇ澶氭暟
	SCSI 浼犺緭灞?LLDD 浼氬湪鍛戒护澶辫触鏃惰嚜鍔ㄨ幏鍙?sense 鏁版嵁锛坅utosense锛?	鑷姩鎰熺煡锛夈€傚嚭浜庢€ц兘鍘熷洜锛屼互鍙婂洜涓?sense 淇℃伅鍙兘鍦?CHECK CONDITION
	鍙戠敓涓庢鍔ㄤ綔涔嬮棿澶卞幓鍚屾锛屾帹鑽愪娇鐢?autosense銆?
	娉ㄦ剰锛屽鏋滀笉鏀寔 autosense锛屽綋鐢?scsi_done() 閿欒瀹屾垚璇?scmd 鏃讹紝
	scmd->sense_buffer 鍖呭惈鏃犳晥鐨?sense 鏁版嵁銆俿csi_decide_disposition()
	鍦ㄨ繖绉嶆儏鍐典笅鎬绘槸杩斿洖 FAILED锛屼粠鑰岃皟鐢?SCSI EH銆傚綋 scmd 鍒拌揪姝ゅ鏃讹紝
	浼氳幏鍙?sense 鏁版嵁骞跺啀娆¤皟鐢?scsi_decide_disposition()銆?
 1. 璋冪敤 scsi_request_sense()锛屽畠鍙戝嚭 REQUEST_SENSE 鍛戒护銆傚鏋滃け璐ワ紝鍒欎笉
           閲囧彇鍔ㄤ綔銆傛敞鎰忥紝涓嶉噰鍙栧姩浣滀細瀵艰嚧瀵硅 scmd 閲囧彇鏇撮珮涓ラ噸鎬х殑鎭㈠銆?
 2. 瀵硅 scmd 璋冪敤 scsi_decide_disposition()

    - SUCCESS锛堟垚鍔燂級
		scmd->retries 琚涓?scmd->allowed锛岄樆姝?scsi_eh_flush_done_q()
		閲嶈瘯璇?scmd锛屽苟璋冪敤 scsi_eh_finish_cmd()銆?
    - NEEDS_RETRY锛堥渶瑕侀噸璇曪級
		scsi_eh_finish_cmd() 琚皟鐢?
    - otherwise锛堝叾浠栨儏鍐碉級
		涓嶉噰鍙栧姩浣溿€?
    4. 濡傛灉 !list_empty(&eh_work_q)锛岃皟鐢?scsi_eh_ready_devs()

    `scsi_eh_ready_devs`

	璇ュ嚱鏁伴噰鍙栧洓绉嶈秺鏉ヨ秺涓ュ帀鐨勬帾鏂斤紝浣垮け璐ョ殑 sdev 鍑嗗濂芥帴鍙楁柊鍛戒护銆?
 1. 璋冪敤 scsi_eh_stu()

	`scsi_eh_stu`

	    瀵逛簬姣忎釜鏈夊け璐?scmd 涓斿甫鏈夋湁鏁?sense 鏁版嵁銆佷笖 scsi_check_sense()
	    鐨勫垽瀹氫负 FAILED 鐨?sdev锛屽彂鍑?start=1 鐨?START STOP UNIT 鍛戒护銆?	    娉ㄦ剰锛岀敱浜庢垜浠槑纭€夋嫨浜嗛敊璇畬鎴愮殑 scmd锛屽凡鐭ュ簳灞傚凡缁忛仐蹇樹簡璇?	    scmd锛屽洜姝ゆ垜浠彲浠ュ鐢ㄥ畠鏉ヨ繘琛?STU銆?
	    濡傛灉 STU 鎴愬姛涓?sdev 澶勪簬绂荤嚎鎴栧氨缁姸鎬侊紝璇?sdev 涓婃墍鏈夊け璐ョ殑
	    scmd 閮戒細閫氳繃 scsi_eh_finish_cmd() 瀹屾垚 EH銆?
	    **娉ㄦ剰** 濡傛灉鏈疄鐜?hostt->eh_abort_handler() 鎴栧畠澶辫触锛屾鏃舵垜浠?	    鍙兘浠嶆湁瓒呮椂鐨?scmd锛岃€?STU 骞朵笉鑳借搴曞眰閬楀繕閭ｄ簺 scmd銆傜劧鑰岋紝濡傛灉
	    STU 鎴愬姛锛岃鍑芥暟浼氬畬鎴愯 sdev 涓婃墍鏈?scmd 鐨?EH锛屼娇搴曞眰澶勪簬涓嶄竴鑷?	    鐨勭姸鎬併€備技涔?STU 鍔ㄤ綔鍙簲鍦ㄦ煇涓?sdev 娌℃湁瓒呮椂 scmd 鏃舵墠搴旈噰鍙栥€?
 2. 濡傛灉 !list_empty(&eh_work_q)锛岃皟鐢?scsi_eh_bus_device_reset()銆?
	`scsi_eh_bus_device_reset`

	    姝ゅ姩浣滀笌 scsi_eh_stu() 闈炲父鐩镐技锛屽彧鏄畠浣跨敤
	    hostt->eh_device_reset_handler() 鑰屼笉鏄彂鍑?STU銆傛澶栵紝鐢变簬鎴戜滑涓?	    鍙戝嚭 SCSI 鍛戒护锛屼笖閲嶇疆浼氭竻闄よ sdev 涓婄殑鎵€鏈?scmd锛屽洜姝ゆ棤闇€鎸戦€?	    閿欒瀹屾垚鐨?scmd銆?
 3. 濡傛灉 !list_empty(&eh_work_q)锛岃皟鐢?scsi_eh_bus_reset()銆?
	`scsi_eh_bus_reset`

	    hostt->eh_bus_reset_handler() 瀵规瘡涓湁澶辫触 scmd 鐨勯€氶亾璋冪敤銆傚鏋?	    鎬荤嚎閲嶇疆鎴愬姛锛岃閫氶亾涓婃墍鏈夊氨缁垨绂荤嚎鐨?sdev 涓婂け璐ョ殑 scmd 閮戒細
	    瀹屾垚 EH銆?
 4. 濡傛灉 !list_empty(&eh_work_q)锛岃皟鐢?scsi_eh_host_reset()銆?
	`scsi_eh_host_reset`

	    杩欐槸鏈€鍚庢墜娈点€傝皟鐢?hostt->eh_host_reset_handler()銆傚鏋滀富鏈洪噸缃?	    鎴愬姛锛岃涓绘満涓婃墍鏈夊氨缁垨绂荤嚎鐨?sdev 涓婂け璐ョ殑 scmd 閮戒細瀹屾垚 EH銆?
 5. 濡傛灉 !list_empty(&eh_work_q)锛岃皟鐢?scsi_eh_offline_sdevs()銆?
	`scsi_eh_offline_sdevs`

	    灏嗘墍鏈変粛鏈夋湭鎭㈠ scmd 鐨?sdev 涓嬬嚎锛屽苟瀹屾垚杩欎簺 scmd 鐨?EH銆?
    5. 璋冪敤 scsi_eh_flush_done_q()銆?
	`scsi_eh_flush_done_q`

	    姝ゆ椂鎵€鏈?scmd 閮藉凡鎭㈠锛堟垨鏀惧純锛夛紝骞剁敱 scsi_eh_finish_cmd() 鏀惧埌浜?	    eh_done_q 涓娿€傝鍑芥暟閫氳繃閲嶈瘯鎴栧悜涓婂眰閫氱煡 scmd 澶辫触鏉ュ埛鏂?	    eh_done_q銆?

### 2.2 EH through transportt->eh_strategy_handler()锛堥€氳繃 transportt->eh_strategy_handler() 鐨?EH锛?

transportt->eh_strategy_handler() 鍦?scsi_unjam_host() 鐨勪綅缃璋冪敤锛屽畠璐熻矗
鏁翠釜鎭㈠杩囩▼銆傚湪瀹屾垚鍚庯紝璇ュ鐞嗙▼搴忓簲褰撳凡缁忚搴曞眰閬楀繕浜嗘墍鏈夊け璐ョ殑 scmd锛屽苟涓?瑕佷箞鍑嗗濂芥帴鍙楁柊鍛戒护锛岃涔堝凡涓嬬嚎銆傛澶栵紝瀹冨簲褰撴墽琛?SCSI EH 缁存姢鏉傚姟浠ョ淮鎶?SCSI 涓棿灞傜殑瀹屾暣鎬с€傛崲瑷€涔嬶紝鍦?[2-1-2] 鎻忚堪鐨勬楠や腑锛岄櫎浜嗙 1 姝ヤ箣澶栫殑鎵€鏈?姝ラ閮藉繀椤荤敱 eh_strategy_handler() 瀹炵幇銆?

##### 2.2.1 Pre transportt->eh_strategy_handler() SCSI midlayer conditions锛坱ransportt->eh_strategy_handler() 涔嬪墠鐨?SCSI 涓棿灞傛潯浠讹級


 杩涘叆澶勭悊绋嬪簭鏃讹紝浠ヤ笅鏉′欢涓虹湡銆?
 - 姣忎釜澶辫触 scmd 鐨?eh_flags 瀛楁琚€傚綋璁剧疆銆?
 - 姣忎釜澶辫触鐨?scmd 閫氳繃 scmd->eh_entry 閾炬帴鍦?scmd->eh_cmd_q 涓娿€?
 - SHOST_RECOVERY 宸茶缃€?
 - shost->host_failed == shost->host_busy


##### 2.2.2 Post transportt->eh_strategy_handler() SCSI midlayer conditions锛坱ransportt->eh_strategy_handler() 涔嬪悗鐨?SCSI 涓棿灞傛潯浠讹級


 閫€鍑哄鐞嗙▼搴忔椂锛屼互涓嬫潯浠跺繀椤讳负鐪熴€?
 - shost->host_failed 涓洪浂銆?
 - shost->eh_cmd_q 宸叉竻绌恒€?
 - 姣忎釜 scmd->eh_entry 宸叉竻绌恒€?
 - 瀵规瘡涓?scmd 閮借皟鐢ㄤ簡 scsi_queue_insert() 鎴?scsi_finish_command()銆傛敞鎰忥紝
   澶勭悊绋嬪簭鍙嚜鐢变娇鐢?scmd->retries 涓?->allowed 鏉ラ檺鍒堕噸璇曟鏁般€?

##### 2.2.3 Things to consider锛堥渶瑕佽€冭檻鐨勪簨椤癸級


 - 瑕佺煡閬撹秴鏃剁殑 scmd 鍦ㄥ簳灞備粛鐒舵槸娲昏穬鐨勩€傚湪瀵归偅浜?scmd 鍋氫换浣曞叾浠栦簨鎯呬箣鍓嶏紝
   鍏堣搴曞眰閬楀繕瀹冧滑銆?
 - 涓轰繚鎸佷竴鑷达紝鍦ㄨ闂?淇敼 shost 鏁版嵁缁撴瀯鏃讹紝鑾峰彇 shost->host_lock銆?
 - 鍦ㄥ畬鎴愬悗锛屾瘡涓け璐ョ殑 sdev 蹇呴』宸茬粡閬楀繕浜嗘墍鏈夋椿璺冪殑 scmd銆?
 - 鍦ㄥ畬鎴愬悗锛屾瘡涓け璐ョ殑 sdev 蹇呴』鍑嗗濂芥帴鍙楁柊鍛戒护鎴栧凡涓嬬嚎銆?

Tejun Heo
htejun@gmail.com

2005 骞?9 鏈?11 鏃?