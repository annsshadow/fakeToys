
## ATM cxacru 璁惧椹卞姩


璇ヨ澶囬渶瑕佸浐浠讹細http://accessrunner.sourceforge.net/

铏界劧璁惧鑳藉鍦ㄦ湭鍔犺浇妯″潡鐨勬儏鍐典笅绠＄悊/缁存姢 ADSL 杩炴帴锛屼絾鏈夋椂鍦ㄥ嵏杞介┍鍔ㄥ悗璁惧浼氬仠姝㈠搷搴旓紝姝ゆ椂蹇呴』鎷斾笅璁惧鐢垫簮鎴栨柇鐢典互淇璇ラ棶棰樸€?
娉ㄦ剰锛氬 cxacru-cf.bin 鐨勬敮鎸佸凡琚Щ闄ゃ€傝鏂囦欢涔嬪墠鏈姝ｇ‘鍔犺浇锛屽洜姝ゅ璁惧閰嶇疆娌℃湁鏁堟灉銆備慨澶嶅畠鍙兘鍦ㄦ彁渚涗簡鏃犳晥閰嶇疆鏃跺鑷寸幇鏈夎澶囨棤娉曞伐浣溿€?
鎻愪緵浜嗕竴涓剼鏈?cxacru-cf.py锛岀敤浜庡皢鐜版湁鏂囦欢杞崲涓?sysfs 褰㈠紡銆?
妫€娴嬪埌鐨勮澶囦細浣滀负鍚嶄负 "cxacru" 鐨?ATM 璁惧鍑虹幇銆傚湪 /sys/class/atm/ 涓嬶紝瀹冧滑鏄悕涓?cxacruN 鐨勭洰褰曪紝鍏朵腑 N 涓鸿澶囧彿銆備竴涓悕涓?device 鐨勭鍙烽摼鎺ユ寚鍚?USB 鎺ュ彛璁惧鐨勭洰褰曪紝鍏朵腑鍖呭惈浜嗗嚑涓敤浜庤幏鍙栬澶囩粺璁′俊鎭殑 sysfs 灞炴€ф枃浠讹細

- adsl_controller_version

- adsl_headend
- adsl_headend_environment

 - 鍏充簬杩滅灞€绔紙headend锛夌殑淇℃伅銆?
- adsl_config

 - 閰嶇疆鍐欏叆鎺ュ彛銆? - 浠ュ崄鍏繘鍒舵牸寮忓啓鍏ュ弬鏁?<index>=<value>锛?	  浠ョ┖鐧藉垎闅旓紝渚嬪锛?
		"1=0 a=5"

 - 涓€娆℃渶澶氬彂閫?7 涓弬鏁帮紝璁剧疆浠讳綍鍊兼椂璋冨埗瑙ｈ皟鍣ㄩ兘浼氶噸鍚?	  ADSL 杩炴帴銆傝繖浜涘弬鏁颁細琚褰曚笅鏉ヤ互澶囧皢鏉ュ弬鑰冦€?
- downstream_attenuation (dB)
- downstream_bits_per_frame
- downstream_rate (kbps)
- downstream_snr_margin (dB)

 - 涓嬭缁熻淇℃伅銆?
- upstream_attenuation (dB)
- upstream_bits_per_frame
- upstream_rate (kbps)
- upstream_snr_margin (dB)
- transmitter_power (dBm/Hz)

 - 涓婅缁熻淇℃伅銆?
- downstream_crc_errors
- downstream_fec_errors
- downstream_hec_errors
- upstream_crc_errors
- upstream_fec_errors
- upstream_hec_errors

 - 閿欒璁℃暟銆?
- line_startable

 - 琛ㄧず璁惧涓婄殑 ADSL 鏀寔
	  鏄?鍙互琚惎鐢紝鍙傝 adsl_start銆?
- line_status

  - "initialising"锛堝垵濮嬪寲涓級
  - "down"锛堟柇寮€锛?  - "attempting to activate"锛堝皾璇曟縺娲伙級
  - "training"锛堣缁冿級
  - "channel analysis"锛堜俊閬撳垎鏋愶級
  - "exchange"锛堜氦鎹級
  - "waiting"锛堢瓑寰咃級
  - "up"锛堝凡杩炴帴锛?
	濡傛灉娌℃湁淇″彿锛屼細鍦?"down" 涓?"attempting to activate"
	涔嬮棿鍒囨崲銆?
- link_status

  - "not connected"锛堟湭杩炴帴锛?  - "connected"锛堝凡杩炴帴锛?  - "lost"锛堜涪澶憋級

- mac_address

- modulation

  - ""锛堟湭杩炴帴鏃讹級
  - "ANSI T1.413"
  - "ITU-T G.992.1 (G.DMT)"
  - "ITU-T G.992.2 (G.LITE)"

- startup_attempts

 - 鍒濆鍖?ADSL 鐨勬€诲皾璇曟鏁般€?
瑕佸惎鐢?绂佺敤 ADSL锛屽彲浠ュ悜 adsl_state 鏂囦欢鍐欏叆浠ヤ笅鍐呭锛?
  - "start"锛堝惎鍔級
  - "stop"锛堝仠姝級
  - "restart"锛堝仠姝紝绛夊緟 1.5s锛岀劧鍚庡惎鍔級
  - "poll"锛堝湪鍥犲け璐ヨ€岃绂佺敤鍚庯紝鐢ㄤ簬鎭㈠鐘舵€佽疆璇級

```

	[4942145.150704] ATM dev 0: ADSL state: running
	[4942243.663766] ATM dev 0: ADSL line: down
	[4942249.665075] ATM dev 0: ADSL line: attempting to activate
	[4942253.654954] ATM dev 0: ADSL line: training
	[4942255.666387] ATM dev 0: ADSL line: channel analysis
	[4942259.656262] ATM dev 0: ADSL line: exchange
	[2635357.696901] ATM dev 0: ADSL line: up (8128 kb/s down | 832 kb/s up)

```
