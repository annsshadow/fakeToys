## ACPI 椋庢墖鎬ц兘鐘舵€?

褰撲唬琛ㄩ鎵囩殑 ACPI 璁惧锛堜緥濡?PNP0C0B 鎴?INT3404锛変笅瀛樺湪鍙€夌殑 _FPS 瀵硅薄鏃讹紝
ACPI 椋庢墖椹卞姩浼氬湪璇?ACPI 璁惧鐨?sysfs 鐩綍涓垱寤洪澶栫殑 鈥渟tate*鈥?灞炴€с€傝繖浜?灞炴€у垪鍑轰簡椋庢墖鎬ц兘鐘舵€佺殑灞炴€с€?
鏈夊叧 _FPS 鐨勬洿澶氫俊鎭紝璇峰弬鑰?ACPI 瑙勮寖锛?
http://uefi.org/specifications

渚嬪锛孖NT3404 ACPI 璁惧 sysfs 鐩綍鐨勫唴瀹?
```
 $ ls -l /sys/bus/acpi/devices/INT3404:00/
 total 0
 ...
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state0
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state1
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state10
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state11
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state2
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state3
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state4
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state5
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state6
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state7
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state8
 -r--r--r-- 1 root root 4096 Dec 13 20:38 state9
 -r--r--r-- 1 root root 4096 Dec 13 01:00 status
 ...

```
鍏朵腑姣忎釜 鈥渟tate*鈥?鏂囦欢浠ｈ〃椋庢墖鐨勪竴涓€ц兘鐘舵€侊紝骞跺寘鍚竴涓互鍐掑彿鍒嗛殧鐨?5 涓?鏁存暟锛堝瓧娈碉級鐨勫垪琛?
```
  control_percent:trip_point_index:speed_rpm:noise_level_mdb:power_mw

```
- `control_percent`锛氱敤浜庨€氳繃 _FSL 瀵硅薄锛?-100锛夊皢椋庢墖閫熷害璁剧疆涓虹壒瀹氱骇鍒殑
  鐧惧垎姣斿€笺€?
- `trip_point_index`锛氫笌姝ゆ€ц兘鐘舵€佸搴旂殑涓诲姩鍐峰嵈瑙﹀彂鐐圭紪鍙凤紙0-9锛夈€?
- `speed_rpm`锛氶鎵囪浆閫燂紝鍗曚綅涓烘瘡鍒嗛挓杞暟銆?
- `noise_level_mdb`锛氭鐘舵€佷笅椋庢墖鍙戝嚭鐨勫彲鍚櫔澹帮紝鍗曚綅涓烘鍒嗚礉锛坢illidecibel锛夈€?
- `power_mw`锛氭鐘舵€佷笅椋庢墖鐨勫姛鐜囨秷鑰楋紝鍗曚綅涓烘鐡︺€?
```
 $cat /sys/bus/acpi/devices/INT3404:00/state1
 25:0:3200:12500:1250

```
褰撶粰瀹氬瓧娈垫湭琚～鍏咃紝鎴栧叾鐢卞钩鍙板浐浠舵彁渚涚殑鍊兼棤鏁堟椂锛屼細鏄剧ず 鈥渘ot-defined鈥?瀛楃涓?鑰岄潪璇ュ€笺€?
## ACPI 椋庢墖缁嗙矑搴︽帶鍒?

褰?_FIF 瀵硅薄鎸囧畾鏀寔缁嗙矑搴︽帶鍒舵椂锛屽彲浠ラ€氳繃 _FSL 瀵硅薄灏嗛鎵囬€熷害浠?0 璁剧疆鍒?100%锛堝甫鏈夋帹鑽愮殑鏈€灏忊€滄闀库€濓級銆傜敤鎴峰彲浠ヤ娇鐢ㄧ儹 sysfs 鍐峰嵈璁惧璋冩暣椋庢墖閫熷害銆?
杩欓噷鐢ㄦ埛鍙弬鑰冮鎵囨€ц兘鐘舵€佷腑鐨勫弬鑰冮€熷害锛坰peed_rpm锛夛紝骞堕€氳繃鏇存敼鍐峰嵈璁惧鐨?cur_state 鏉ヨ缃畠銆傚鏋滄敮鎸佺粏绮掑害鎺у埗锛岀敤鎴疯繕鍙互璋冩暣鍒版€ц兘鐘舵€佷腑鏈畾涔夌殑
鍏朵粬閫熷害銆?
缁嗙矑搴︽帶鍒剁殑鏀寔閫氳繃 sysfs 灞炴€?鈥渇ine_grain_control鈥?鍛堢幇銆傚鏋滃瓨鍦ㄧ粏绮掑害
鎺у埗锛岃灞炴€ф樉绀?鈥?鈥濓紝鍚﹀垯鏄剧ず 鈥?鈥濄€?
璇?sysfs 灞炴€т笌鎬ц兘鐘舵€佷綅浜庡悓涓€鐩綍涓€?
## ACPI 椋庢墖鎬ц兘鍙嶉


鍙€夌殑 _FST 瀵硅薄鎻愪緵椋庢墖璁惧鐨勭姸鎬佷俊鎭€傝繖鍖呮嫭涓€涓瓧娈碉紝鐢ㄤ簬鎻愪緵椋庢墖褰撳墠
鏃嬭浆杞€燂紙姣忓垎閽熻浆鏁帮級銆?
璇ラ€熷害閫氳繃灞炴€?鈥渇an_speed_rpm鈥?鍦?sysfs 涓憟鐜帮紝涓庢€ц兘鐘舵€佷綅浜庡悓涓€鐩綍銆?