

######## ioctl CEC_DQEVENT


## Name


CEC_DQEVENT - 鍑洪槦锛圖equeue锛変竴涓?CEC 浜嬩欢

## Synopsis



`int ioctl(int fd, CEC_DQEVENT, struct cec_event *argp)`

## Arguments


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`

## Description


CEC 璁惧鍙互鍙戦€佸紓姝ヤ簨浠躲€傚彲閫氳繃璋冪敤 `CEC_DQEVENT` 鏉ユ绱㈣繖浜涗簨浠躲€傚鏋滄枃浠舵弿杩扮澶勪簬闈為樆濉炴ā寮忎笖娌℃湁鎸傝捣浜嬩欢锛屽垯杩斿洖 -1 骞跺皢 errno 璁剧疆涓?`EAGAIN` 閿欒鐮併€?
鍐呴儴浜嬩欢闃熷垪鏄寜鏂囦欢鍙ユ焺锛坒ilehandle锛夊拰浜嬩欢绫诲瀷鍒嗗埆缁存姢鐨勩€傚鏋滈槦鍒楀凡婊★紝鍒欐渶鍚庝竴涓簨浠朵細琚柊浜嬩欢瑕嗙洊銆傝繖鎰忓懗鐫€涓棿缁撴灉鍙兘琚涪寮冿紝浣嗘渶鏂颁簨浠跺缁堝彲鐢ㄣ€傝繖涔熸剰鍛崇潃鏈夊彲鑳借鍒颁袱涓叿鏈夌浉鍚屽€肩殑杩炵画浜嬩欢锛堜緥濡備袱涓?CEC_EVENT_STATE_CHANGE <CEC-EVENT-STATE-CHANGE> 浜嬩欢锛屽叾鐘舵€佺浉鍚岋級銆傚湪杩欑鎯呭喌涓嬶紝涓棿鐨勭姸鎬佸彉鍖栦細涓㈠け锛屼絾鍙互淇濊瘉涓ゆ浜嬩欢涔嬮棿鐨勭姸鎬佺‘瀹炲彂鐢熻繃鍙樺寲銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 8

    - - __u16
      - `phys_addr`
      - The current physical address. This is `CEC_PHYS_ADDR_INVALID` if no
        valid physical address is set.
    - - __u16
      - `log_addr_mask`
      - The current set of claimed logical addresses. This is 0 if no logical
        addresses are claimed or if `phys_addr` is `CEC_PHYS_ADDR_INVALID`.
	If bit 15 is set (`1 << CEC_LOG_ADDR_UNREGISTERED`) then this device
	has the unregistered logical address. In that case all other bits are 0.
    - - __u16
      - `have_conn_info`
      - If non-zero, then HDMI connector information is available.
        This field is only valid if `CEC_CAP_CONNECTOR_INFO` is set. If that
        capability is set and `have_conn_info` is zero, then that indicates
        that the HDMI connector device is not instantiated, either because
        the HDMI driver is still configuring the device or because the HDMI
        device was unbound.



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 16

    - - __u32
      - `lost_msgs`
      - Set to the number of lost messages since the filehandle was opened
	or since the last time this event was dequeued for this
	filehandle. The messages lost are the oldest messages. So when a
	new message arrives and there is no more room, then the oldest
	message is discarded to make room for the new one. The internal
	size of the message queue guarantees that all messages received in
	the last two seconds will be stored. Since messages should be
	replied to within a second according to the CEC specification,
	this is more than enough.



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 8

    - - __u64
      - `ts`
      - 浜嬩欢鐨勬椂闂存埑锛屽崟浣嶄负 ns銆?
	璇ユ椂闂存埑鍙栬嚜 `CLOCK_MONOTONIC` 鏃堕挓銆?
	鑻ヨ鍦ㄧ敤鎴风┖闂磋闂悓涓€鏃堕挓锛屽彲浣跨敤 `clock_gettime`銆?    - - __u32
      - `event`
      - CEC 浜嬩欢绫诲瀷锛屽弬瑙?cec-events銆?    - - __u32
      - `flags`
      - 浜嬩欢鏍囧織锛屽弬瑙?cec-event-flags銆?    - - union {
      - (anonymous)
    - - struct cec_event_state_change
      - `state_change`
      - 鐢?CEC_EVENT_STATE_CHANGE <CEC-EVENT-STATE-CHANGE> 浜嬩欢
	鍙戦€佺殑鏂扮殑閫傞厤鍣ㄧ姸鎬併€?    - - struct cec_event_lost_msgs
      - `lost_msgs`
      - 鐢?CEC_EVENT_LOST_MSGS <CEC-EVENT-LOST-MSGS> 浜嬩欢
	鍙戦€佺殑涓㈠け娑堟伅鏁伴噺銆?    - - }
      -



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-EVENT-STATE-CHANGE`:

      - `CEC_EVENT_STATE_CHANGE`
      - 1
      - 褰?CEC 閫傞厤鍣ㄧ姸鎬佸彂鐢熷彉鍖栨椂鐢熸垚銆傝皟鐢?open() 鏃朵細涓鸿鏂囦欢鍙ユ焺
	鐢熸垚涓€鏉″垵濮嬩簨浠讹紝鍙嶆槧褰撴椂 CEC 閫傞厤鍣ㄧ殑鐘舵€併€?    - .. _`CEC-EVENT-LOST-MSGS`:

      - `CEC_EVENT_LOST_MSGS`
      - 2
      - 濡傛灉鐢变簬搴旂敤绋嬪簭鏈兘鍙婃椂鍑洪槦 CEC 娑堟伅鑰屽鑷翠竴鏉℃垨澶氭潯
	CEC 娑堟伅涓㈠け锛屽垯鐢熸垚璇ヤ簨浠躲€?    - .. _`CEC-EVENT-PIN-CEC-LOW`:

      - `CEC_EVENT_PIN_CEC_LOW`
      - 3
      - 褰?CEC 寮曡剼浠庨珮鐢靛帇鍙樹负浣庣數鍘嬫椂鐢熸垚銆備粎閫傜敤浜庤缃簡
	`CEC_CAP_MONITOR_PIN` 鑳藉姏鐨勯€傞厤鍣ㄣ€?    - .. _`CEC-EVENT-PIN-CEC-HIGH`:

      - `CEC_EVENT_PIN_CEC_HIGH`
      - 4
      - 褰?CEC 寮曡剼浠庝綆鐢靛帇鍙樹负楂樼數鍘嬫椂鐢熸垚銆備粎閫傜敤浜庤缃簡
	`CEC_CAP_MONITOR_PIN` 鑳藉姏鐨勯€傞厤鍣ㄣ€?    - .. _`CEC-EVENT-PIN-HPD-LOW`:

      - `CEC_EVENT_PIN_HPD_LOW`
      - 5
      - 褰?HPD 寮曡剼浠庨珮鐢靛帇鍙樹负浣庣數鍘嬫椂鐢熸垚銆備粎閫傜敤浜庤缃簡
	`CEC_CAP_MONITOR_PIN` 鑳藉姏鐨勯€傞厤鍣ㄣ€傝皟鐢?open() 鏃跺彲璇诲彇 HPD
	寮曡剼锛岃嫢 HPD 涓轰綆鐢靛钩锛屽垯灏嗕负璇ユ枃浠跺彞鏌勭敓鎴愪竴鏉″垵濮嬩簨浠躲€?    - .. _`CEC-EVENT-PIN-HPD-HIGH`:

      - `CEC_EVENT_PIN_HPD_HIGH`
      - 6
      - 褰?HPD 寮曡剼浠庝綆鐢靛帇鍙樹负楂樼數鍘嬫椂鐢熸垚銆備粎閫傜敤浜庤缃簡
	`CEC_CAP_MONITOR_PIN` 鑳藉姏鐨勯€傞厤鍣ㄣ€傝皟鐢?open() 鏃跺彲璇诲彇 HPD
	寮曡剼锛岃嫢 HPD 涓洪珮鐢靛钩锛屽垯灏嗕负璇ユ枃浠跺彞鏌勭敓鎴愪竴鏉″垵濮嬩簨浠躲€?    - .. _`CEC-EVENT-PIN-5V-LOW`:

      - `CEC_EVENT_PIN_5V_LOW`
      - 6
      - 褰?5V 寮曡剼浠庨珮鐢靛帇鍙樹负浣庣數鍘嬫椂鐢熸垚銆備粎閫傜敤浜庤缃簡
	`CEC_CAP_MONITOR_PIN` 鑳藉姏鐨勯€傞厤鍣ㄣ€傝皟鐢?open() 鏃跺彲璇诲彇 5V
	寮曡剼锛岃嫢 5V 涓轰綆鐢靛钩锛屽垯灏嗕负璇ユ枃浠跺彞鏌勭敓鎴愪竴鏉″垵濮嬩簨浠躲€?    - .. _`CEC-EVENT-PIN-5V-HIGH`:

      - `CEC_EVENT_PIN_5V_HIGH`
      - 7
      - 褰?5V 寮曡剼浠庝綆鐢靛帇鍙樹负楂樼數鍘嬫椂鐢熸垚銆備粎閫傜敤浜庤缃簡
	`CEC_CAP_MONITOR_PIN` 鑳藉姏鐨勯€傞厤鍣ㄣ€傝皟鐢?open() 鏃跺彲璇诲彇 5V
	寮曡剼锛岃嫢 5V 涓洪珮鐢靛钩锛屽垯灏嗕负璇ユ枃浠跺彞鏌勭敓鎴愪竴鏉″垵濮嬩簨浠躲€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 8

    - .. _`CEC-EVENT-FL-INITIAL-STATE`:

      - `CEC_EVENT_FL_INITIAL_STATE`
      - 1
      - 閽堝璁惧鎵撳紑鏃剁敓鎴愮殑鍒濆浜嬩欢璁剧疆銆傚摢浜涗簨浠朵細杩欐牱鍋氾紝鍙傝涓婅〃銆?	杩欐牱搴旂敤绋嬪簭鍙互鍦?open() 鏃朵簡瑙ｅ埌 CEC 閫傞厤鍣ㄧ殑鍒濆鐘舵€併€?    - .. _`CEC-EVENT-FL-DROPPED-EVENTS`:

      - `CEC_EVENT_FL_DROPPED_EVENTS`
      - 2
      - 濡傛灉缁欏畾浜嬩欢绫诲瀷鐨勪竴涓垨澶氫釜浜嬩欢宸茶涓㈠純锛屽垯璁剧疆璇ユ爣蹇椼€?	杩欒〃鏄庡簲鐢ㄧ▼搴忔棤娉曡窡涓婂鐞嗛€熷害銆?

## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞堕€傚綋鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
ioctl CEC_DQEVENT <CEC_DQEVENT> 鍙兘杩斿洖浠ヤ笅閿欒鐮侊細

EAGAIN
    褰撴枃浠跺彞鏌勫浜庨潪闃诲妯″紡涓旀病鏈夋寕璧蜂簨浠舵椂杩斿洖銆?
ERESTARTSYS
    鍦ㄩ樆濉炴ā寮忎笅绛夊緟浜嬩欢鍒拌揪鏃讹紝鏀跺埌浜嗕竴涓腑鏂紙渚嬪 Ctrl-C锛夈€?