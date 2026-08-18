## Freescale i.MX8 DDR 鎬ц兘鐩戞帶鍗曞厓锛圥MU锛?

DRAM 鎺у埗鍣ㄥ唴閮ㄦ病鏈夋€ц兘璁℃暟鍣紝鍥犳鎬ц兘淇″彿琚紩鍑哄埌鎺у埗鍣ㄧ殑杈圭紭锛屽湪閭ｉ噷
瀹炵幇浜嗕竴缁?4 x 32 浣嶇殑璁℃暟鍣ㄣ€傝繖缁勮鏁板櫒鐢卞湪璁℃暟鍣ㄦ帶鍒跺瘎瀛樺櫒涓紪绋嬬殑 CSV
妯″紡鎺у埗锛屼粠鑰屼細浜х敓澶ч噺鐨?PERF 淇″彿銆?
姣忎釜璁℃暟鍣ㄧ殑鍊奸€氳繃 config 瀵勫瓨鍣ㄨ繘琛岄€夋嫨銆傛瘡涓鏁板櫒瀵瑰簲涓€涓瘎瀛樺櫒銆傝鏁板櫒 0
姣旇緝鐗规畩锛屽畠鎬绘槸璁℃暟鈥滄椂闂粹€濓紝骞跺湪鍒版湡鏃跺鑷韩浠ュ強鍏跺畠璁℃暟鍣ㄥ姞閿侊紝骞惰Е鍙戜竴涓?涓柇銆傚鏋滀换浣曞叾瀹冭鏁板櫒婧㈠嚭锛屽畠浼氱户缁鏁帮紝涓斾笉浼氳Е鍙戜腑鏂€?
"format" 鐩綍鎻忚堪浜?perf_event_attr 缁撴瀯鐨?config锛堜簨浠?ID锛変互鍙?config1/2
锛圓XI 杩囨护鍣ㄨ缃級瀛楁鐨勬牸寮忥紝鍙傝 /sys/bus/event_source/devices/imx8_ddr0/format/銆?"events" 鐩綍鎻忚堪浜嗗彲涓?perf 宸ュ叿閰嶅悎浣跨敤鐨勩€佺‖浠舵敮鎸佺殑浜嬩欢绫诲瀷锛屽弬瑙?/sys/bus/event_source/devices/imx8_ddr0/events/銆?caps" 鐩綍鎻忚堪浜?DDR PMU
涓疄鐜扮殑杩囨护鐗规€э紝鍙傝 /sys/bus/events_source/devices/imx8_ddr0/caps/銆?
    .. code-block:: bash

        perf stat -a -e imx8_ddr0/cycles/ cmd
        perf stat -a -e imx8_ddr0/read/,imx8_ddr0/write/ cmd

AXI 杩囨护浠呰 CSV 妯″紡 0x41锛坅xid-read锛夊拰 0x42锛坅xid-write锛変娇鐢紝鐢ㄤ簬璁℃暟
涓庤繃婊よ缃浉鍖归厤鐨勮鎴栧啓鎿嶄綔銆傝繃婊よ缃洜涓嶅悓鐨?DRAM 鎺у埗鍣ㄥ疄鐜拌€屽紓锛岃繖鐢遍┍鍔?涓殑 quirks 鏉ュ尯鍒嗐€備綘涔熷彲浠ヤ粠鐢ㄦ埛绌洪棿杞偍淇℃伅锛?caps" 鐩綍浼氭樉绀?AXI 杩囨护鍣?鐨勭被鍨嬶紙filter銆乪nhanced_filter 鍜?super_filter锛夈€傚€?0 琛ㄧず涓嶆敮鎸侊紝鍊?1 琛ㄧず
鏀寔銆?
- With DDR_CAP_AXI_ID_FILTER quirk(filter: 1, enhanced_filter: 0, super_filter: 0).
  璇?quirk 閫氳繃浠ヤ笅涓ら儴鍒嗛厤缃潵瀹氫箟杩囨护鍣細
  --AXI_ID defines AxID matching value.
  --AXI_MASKING defines which bits of AxID are meaningful for the matching.

      - 0: corresponding bit is masked.
      - 1: corresponding bit is not masked, i.e. used to do the matching.

  AXI_ID 鍜?AXI_MASKING 琚槧灏勫埌鎬ц兘璁℃暟鍣ㄤ腑鐨?DPCR1 瀵勫瓨鍣ㄣ€傚綋闈炲睆钄戒綅涓?  鐩稿簲鐨?AXI_ID 浣嶅尮閰嶆椂锛岃鏁板櫒涓?```
        AxID && AXI_MASKING == AXI_ID && AXI_MASKING

  This filter doesn't support filter different AXI ID for axid-read and axid-write
  event at the same time as this filter is shared between counters.

  .. code-block:: bash

      perf stat -a -e imx8_ddr0/axid-read,axi_mask=0xMMMM,axi_id=0xDDDD/ cmd
      perf stat -a -e imx8_ddr0/axid-write,axi_mask=0xMMMM,axi_id=0xDDDD/ cmd

  .. note::

      axi_mask is inverted in userspace(i.e. set bits are bits to mask), and
      it will be reverted in driver automatically. so that the user can just specify
      axi_id to monitor a specific id, rather than having to specify axi_mask.

  .. code-block:: bash

        perf stat -a -e imx8_ddr0/axid-read,axi_id=0x12/ cmd, which will monitor ARID=0x12

```
- With DDR_CAP_AXI_ID_FILTER_ENHANCED quirk(filter: 1, enhanced_filter: 1, super_filter: 0).
  杩欐槸瀵?DDR_CAP_AXI_ID_FILTER quirk 鐨勬墿灞曪紝瀹冨厑璁稿湪涓庡彟涓€缁勬暟鎹鏁板櫒骞跺彂鐨?  鎯呭喌涓嬶紝璁℃暟鏉ヨ嚜 DDR 璇诲啓浜嬪姟鐨勫瓧鑺傛暟锛堣€岄潪绐佸彂娆℃暟锛夈€?
- With DDR_CAP_AXI_ID_PORT_CHANNEL_FILTER quirk(filter: 0, enhanced_filter: 0, super_filter: 1).
  鍏堝墠鐨?AXI 杩囨护鍣ㄥ瓨鍦ㄩ檺鍒讹紝鐢变簬杩囨护鍣ㄥ湪璁℃暟鍣ㄩ棿鍏变韩锛屽畠鏃犳硶鍚屾椂杩囨护涓嶅悓鐨?ID銆?  璇?quirk 鏄?AXI ID 杩囨护鍣ㄧ殑鎵╁睍銆備竴澶勬敼杩涙槸璁℃暟鍣?1-3 鎷ユ湁鍚勮嚜鐨勮繃婊ゅ櫒锛屾剰鍛崇潃
  瀹冩敮鎸佸苟鍙戣繃婊や笉鍚岀殑 ID銆傚彟涓€澶勬敼杩涙槸璁℃暟鍣?1-3 鏀寔 AXI PORT 鍜?CHANNEL 閫夋嫨锛?  鏀寔閫夋嫨鍦板潃閫氶亾鎴栨暟鎹€氶亾銆?
  Filter is defined with 2 configuration registers per counter 1-3.
  --Counter N MASK COMP register - including AXI_ID and AXI_MASKING.
  --Counter N MUX CNTL register - including AXI CHANNEL and AXI PORT.

      - 0: address channel
      - 1: data channel

  DDR 瀛愮郴缁熶腑鐨?PMU 浠呭瓨鍦ㄥ崟涓€鐨?port0锛屽洜姝?axi_port 琚繚鐣欙紝搴斾负 0銆?
  .. code-block:: bash

      perf stat -a -e imx8_ddr0/axid-read,axi_mask=0xMMMM,axi_id=0xDDDD,axi_channel=0xH/ cmd
      perf stat -a -e imx8_ddr0/axid-write,axi_mask=0xMMMM,axi_id=0xDDDD,axi_channel=0xH/ cmd

```
      axi_channel is inverted in userspace, and it will be reverted in driver
      automatically. So that users do not need specify axi_channel if want to
      monitor data channel from DDR transactions, since data channel is more
      meaningful.

```
