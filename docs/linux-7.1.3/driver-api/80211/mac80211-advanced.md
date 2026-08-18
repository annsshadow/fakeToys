## mac80211 瀛愮郴缁燂紙杩涢樁锛?

鏈功涓繖閮ㄥ垎鍐呭浠呭 mac80211 涓庨┍鍔ㄤ箣闂寸殑楂樼骇浜や簰鏈夌敤锛岀敤浠ュ彂鎸ユ洿澶?纭欢鑳藉姏骞舵彁鍗囨€ц兘銆?
## LED 鏀寔


Mac80211 鏀寔澶氱璁?LED 闂儊鐨勬柟寮忋€傚湪鍙兘鐨勬儏鍐典笅锛屽簲灏嗚澶?LED 鏆撮湶涓?LED 绫昏澶囷紝骞舵寕鎺ュ埌鐩稿簲鐨勮Е鍙戝櫒涓婏紝闅忓悗鐢?mac80211 鍦ㄩ€傚綋鏃舵満瑙﹀彂璇ヨЕ鍙戝櫒銆?
   :functions:
	ieee80211_get_tx_led_name
	ieee80211_get_rx_led_name
	ieee80211_get_assoc_led_name
	ieee80211_get_radio_led_name
	ieee80211_tpt_blink
	ieee80211_tpt_led_trigger_flags
	ieee80211_create_tpt_led_trigger

## 纭欢鍔犲瘑鍔犻€?

   :doc: 纭欢鍔犲瘑鍔犻€?
   :functions:
	set_key_cmd
	ieee80211_key_conf
	ieee80211_key_flags
	ieee80211_get_tkip_p1k
	ieee80211_get_tkip_p1k_iv
	ieee80211_get_tkip_p2k

## 鐪佺數鏀寔


   :doc: 鐪佺數鏀寔

## 淇℃爣杩囨护鏀寔


   :doc: 淇℃爣杩囨护鏀寔

   :functions: ieee80211_beacon_loss

## 澶氶槦鍒椾笌 QoS 鏀寔


寰呭畾

   :functions: ieee80211_tx_queue_params

## 鎺ュ叆鐐规ā寮忔敮鎸?

寰呭畾

鍏朵腑閮ㄥ垎 if_conf 鍐呭搴斿湪姝ゅ璁ㄨ

鍦ㄦ澶勬垨纭欢鍔犲瘑绔犺妭涓彃鍏ュ叧浜庝娇鐢ㄧ‖浠跺姞瀵嗙殑 VLAN 鎺ュ彛鐨勮鏄庛€?
### 瀵圭渷鐢靛鎴风鐨勬敮鎸?

   :doc: AP 瀵圭渷鐢靛鎴风鐨勬敮鎸?
   :functions:
	ieee80211_get_buffered_bc
	ieee80211_beacon_get
	ieee80211_sta_eosp
	ieee80211_frame_release_type
	ieee80211_sta_ps_transition
	ieee80211_sta_ps_transition_ni
	ieee80211_sta_set_buffered
	ieee80211_sta_block_awake

## 鏀寔澶氫釜铏氭嫙鎺ュ彛


寰呭畾

娉ㄦ剰锛氫娇鐢ㄧ浉鍚?MAC 鍦板潃鐨?WDS 鍑犱箮鎬绘槸鍙互鐨?
鍦ㄦ澶勬彃鍏ュ叧浜庢嫢鏈変笉鍚?MAC 鍦板潃鐨勫涓櫄鎷熸帴鍙ｇ殑璇存槑锛屾敞鏄?mac80211 鏀寔鍝簺閰嶇疆锛?骞舵坊鍔犲叧浜庨厤鍚堢‖浠跺姞瀵嗘敮鎸佺殑璇存槑銆?
   :functions:
	ieee80211_iterate_active_interfaces
	ieee80211_iterate_active_interfaces_atomic

## 绔欑偣澶勭悊


寰呭姙

   :functions:
	ieee80211_sta
	sta_notify_cmd
	ieee80211_find_sta
	ieee80211_find_sta_by_ifaddr

## 纭欢鎵弿鍗歌浇


寰呭畾

   :functions: ieee80211_scan_completed

## 鑱氬悎


### TX A-MPDU 鑱氬悎


   :doc: TX A-MPDU 鑱氬悎

### RX A-MPDU 鑱氬悎


   :doc: RX A-MPDU 鑱氬悎

   :functions: ieee80211_ampdu_mlme_action

## 绌洪棿澶嶇敤鐪佺數锛圫MPS锛?

   :doc: 绌洪棿澶嶇敤鐪佺數

   :functions:
	ieee80211_request_smps
	ieee80211_smps_mode

寰呭畾

鏈功杩欎竴閮ㄥ垎鎻忚堪閫熺巼鎺у埗绠楁硶鎺ュ彛锛屼互鍙婂畠涓?mac80211 鍜岄┍鍔ㄧ殑鍏崇郴銆?
## 閫熺巼鎺у埗 API


寰呭畾

   :functions:
	ieee80211_start_tx_ba_session
	ieee80211_start_tx_ba_cb_irqsafe
	ieee80211_stop_tx_ba_session
	ieee80211_stop_tx_ba_cb_irqsafe
	ieee80211_rate_control_changed
	ieee80211_tx_rate_control

寰呭畾

鏈功杩欎竴閮ㄥ垎鎻忚堪 mac80211 鍐呴儴瀹炵幇銆?
## 瀵嗛挜澶勭悊


### 瀵嗛挜澶勭悊鍩虹


   :doc: 瀵嗛挜澶勭悊鍩虹

### 鏇村寰呭畾


寰呭畾

## 鎺ユ敹澶勭悊


寰呭畾

## 鍙戦€佸鐞?

寰呭畾

## 绔欑偣淇℃伅澶勭悊


### 缂栫▼淇℃伅


   :functions:
	sta_info
	ieee80211_sta_info_flags

### STA 淇℃伅鐢熷懡鍛ㄦ湡瑙勫垯


   :doc: STA 淇℃伅鐢熷懡鍛ㄦ湡瑙勫垯

## 鑱氬悎鍑芥暟


   :functions:
	sta_ampdu_mlme
	tid_ampdu_tx
	tid_ampdu_rx

## 鍚屾鍑芥暟


寰呭畾

娑夊強澶ч噺鍔犻攣锛屽澶勪娇鐢?RCU
