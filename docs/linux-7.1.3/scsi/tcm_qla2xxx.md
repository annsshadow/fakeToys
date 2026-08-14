## tcm_qla2xxx 椹卞姩璇存槑


### tcm_qla2xxx jam_host 灞炴€?

鐜板湪鏂板浜嗕竴涓悕涓?jam_host 鐨勬ā鍧楃鐐瑰睘鎬?
```

	jam_host: boolean=0/1

```
璇ュ睘鎬у強閰嶅浠ｇ爜浠呭湪灏?Kconfig 鍙傛暟 TCM_QLA2XXX_DEBUG 璁句负 Y 鏃舵墠琚寘鍚€?

榛樿鎯呭喌涓嬭骞叉壈锛坖ammer锛変唬鐮佸拰鍔熻兘鏄鐢ㄧ殑銆?

浣跨敤璇ュ睘鎬у彲浠ユ帶鍒跺鍙戝線鎵€閫変富鏈虹殑 SCSI 鍛戒护鐨勪涪寮冦€?

杩欏浜庢祴璇曢敊璇鐞嗐€佹ā鎷熺紦鎱㈡帓绌猴紙slow drain锛変互鍙婂叾浠?fabrics 闂鍙兘鏈夌敤銆?

灏嗘煇涓富鏈虹殑 jam_host 灞炴€ц涓哄竷灏斿€?1锛屽皢涓㈠純鍙戝線璇ヤ富鏈虹殑鍛戒护銆?

閲嶇疆鍥?0 浠ュ仠姝㈠共鎵般€?

```
  echo 1 > /sys/kernel/config/target/qla2xxx/21:00:00:24:ff:27:8f:ae/tpgt_1/attrib/jam_host

```
```
  echo 0 > /sys/kernel/config/target/qla2xxx/21:00:00:24:ff:27:8f:ae/tpgt_1/attrib/jam_host

```
