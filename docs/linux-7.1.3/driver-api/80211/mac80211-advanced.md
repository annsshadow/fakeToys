## mac80211 子系统（进阶）


本书中这部分内容仅对 mac80211 与驱动之间的高级交互有用，用以发挥更多
硬件能力并提升性能。

## LED 支持


Mac80211 支持多种让 LED 闪烁的方式。在可能的情况下，应将设备 LED 暴露为
LED 类设备，并挂接到相应的触发器上，随后由 mac80211 在适当时机触发该触发器。

   :functions:
	ieee80211_get_tx_led_name
	ieee80211_get_rx_led_name
	ieee80211_get_assoc_led_name
	ieee80211_get_radio_led_name
	ieee80211_tpt_blink
	ieee80211_tpt_led_trigger_flags
	ieee80211_create_tpt_led_trigger

## 硬件加密加速


   :doc: 硬件加密加速

   :functions:
	set_key_cmd
	ieee80211_key_conf
	ieee80211_key_flags
	ieee80211_get_tkip_p1k
	ieee80211_get_tkip_p1k_iv
	ieee80211_get_tkip_p2k

## 省电支持


   :doc: 省电支持

## 信标过滤支持


   :doc: 信标过滤支持

   :functions: ieee80211_beacon_loss

## 多队列与 QoS 支持


待定

   :functions: ieee80211_tx_queue_params

## 接入点模式支持


待定

其中部分 if_conf 内容应在此处讨论

在此处或硬件加密章节中插入关于使用硬件加密的 VLAN 接口的说明。

### 对省电客户端的支持


   :doc: AP 对省电客户端的支持

   :functions:
	ieee80211_get_buffered_bc
	ieee80211_beacon_get
	ieee80211_sta_eosp
	ieee80211_frame_release_type
	ieee80211_sta_ps_transition
	ieee80211_sta_ps_transition_ni
	ieee80211_sta_set_buffered
	ieee80211_sta_block_awake

## 支持多个虚拟接口


待定

注意：使用相同 MAC 地址的 WDS 几乎总是可以的

在此处插入关于拥有不同 MAC 地址的多个虚拟接口的说明，注明 mac80211 支持哪些配置，
并添加关于配合硬件加密支持的说明。

   :functions:
	ieee80211_iterate_active_interfaces
	ieee80211_iterate_active_interfaces_atomic

## 站点处理


待办

   :functions:
	ieee80211_sta
	sta_notify_cmd
	ieee80211_find_sta
	ieee80211_find_sta_by_ifaddr

## 硬件扫描卸载


待定

   :functions: ieee80211_scan_completed

## 聚合


### TX A-MPDU 聚合


   :doc: TX A-MPDU 聚合

### RX A-MPDU 聚合


   :doc: RX A-MPDU 聚合

   :functions: ieee80211_ampdu_mlme_action

## 空间复用省电（SMPS）


   :doc: 空间复用省电

   :functions:
	ieee80211_request_smps
	ieee80211_smps_mode

待定

本书这一部分描述速率控制算法接口，以及它与 mac80211 和驱动的关系。

## 速率控制 API


待定

   :functions:
	ieee80211_start_tx_ba_session
	ieee80211_start_tx_ba_cb_irqsafe
	ieee80211_stop_tx_ba_session
	ieee80211_stop_tx_ba_cb_irqsafe
	ieee80211_rate_control_changed
	ieee80211_tx_rate_control

待定

本书这一部分描述 mac80211 内部实现。

## 密钥处理


### 密钥处理基础


   :doc: 密钥处理基础

### 更多待定


待定

## 接收处理


待定

## 发送处理


待定

## 站点信息处理


### 编程信息


   :functions:
	sta_info
	ieee80211_sta_info_flags

### STA 信息生命周期规则


   :doc: STA 信息生命周期规则

## 聚合函数


   :functions:
	sta_ampdu_mlme
	tid_ampdu_tx
	tid_ampdu_rx

## 同步函数


待定

涉及大量加锁，多处使用 RCU
