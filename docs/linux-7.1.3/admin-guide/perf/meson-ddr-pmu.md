
## Amlogic SoC DDR 甯﹀鎬ц兘鐩戞帶鍗曞厓锛圥MU锛?

Amlogic Meson G12 SoC 鍦?DRAM 鎺у埗鍣ㄥ唴閮ㄥ寘鍚竴涓甫瀹界洃瑙嗗櫒銆傝鐩戣鍣ㄥ寘鍚?4 涓€氶亾銆傛瘡涓€氶亾鍙互缁熻璁块棶 DRAM 鐨勮姹傘€傝閫氶亾鍙互鍚屾椂缁熻鏈€澶?3 涓?AXI 绔彛銆傚畠鏈夊姪浜庢樉绀烘€ц兘鐡堕鏄惁鍑虹幇鍦?DDR 甯﹀涓娿€?
鐩墠锛岃椹卞姩鏀寔浠ヤ笅 5 涓?perf 浜嬩欢锛?
| meson_ddr_bw/total_rw_bytes/ |
| --- |
| meson_ddr_bw/chan_2_rw_bytes/ |

meson_ddr_bw/chan_{1,2,3,4}_rw_bytes/ 浜嬩欢鏄笌閫氶亾鐩稿叧鐨勪簨浠躲€傛瘡涓€氶亾鏀寔杩囨护锛屽彲浠ヨ閫氶亾鐩戞帶 SoC 涓崟鐙殑 IP 妯″潡銆?
浠ヤ笅鏄?DDR 璁块棶璇锋眰浜嬩欢杩囨护鍏抽敭瀛楋細

| arm             - 鏉ヨ嚜 CPU |
| --- |
| gpu             - 鏉ヨ嚜 3D GPU |
| hdcp            - 鏉ヨ嚜 HDCP 鎺у埗鍣?|
| usb3_0          - 鏉ヨ嚜 USB3.0 鎺у埗鍣?|
| h265enc         - 鏉ヨ嚜 HEVC 缂栫爜鍣?|
| vpu_write1      - 鏉ヨ嚜 VDIN 鍐?|
| vdec            - 鏉ヨ嚜浼犵粺缂栬В鐮佸櫒瑙嗛瑙ｇ爜鍣?|
| ge2d            - 鏉ヨ嚜 ge2d |
| usb0            - 鏉ヨ嚜 USB2.0 鎺у埗鍣?0 |
| arb0            - 鏉ヨ嚜 arb0 |
| usb1            - 鏉ヨ嚜 USB2.0 鎺у埗鍣?1 |
| sd_emmc_c       - 鏉ヨ嚜 SD eMMC c 鎺у埗鍣?|

绀轰緥锛?
  - 鏄剧ず姣忕鐨勬€?DDR 甯﹀锛?
    .. code-block:: bash

       perf stat -a -e meson_ddr_bw/total_rw_bytes/ -I 1000 sleep 10

  - 鍒嗗埆鏄剧ず鏉ヨ嚜 CPU 鍜?GPU 鐨勭嫭绔?DDR 甯﹀锛屼互鍙婂畠浠殑鎬诲拰锛?
    .. code-block:: bash

       perf stat -a -e meson_ddr_bw/chan_1_rw_bytes,arm=1/ -I 1000 sleep 10
       perf stat -a -e meson_ddr_bw/chan_2_rw_bytes,gpu=1/ -I 1000 sleep 10
       perf stat -a -e meson_ddr_bw/chan_3_rw_bytes,arm=1,gpu=1/ -I 1000 sleep 10
