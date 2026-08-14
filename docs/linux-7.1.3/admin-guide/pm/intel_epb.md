
## Intel 鎬ц兘涓庤兘鑰楀亸缃彁绀?


:Copyright: |copy| 2019 Intel Corporation

:Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>


   :doc: overview

## Intel 鎬ц兘涓庤兘鑰楀亸缃紙EPB锛夊湪 ``sysfs`` 涓殑灞炴€?


缁欏畾锛堥€昏緫锛塁PU 鐨?Intel 鎬ц兘涓庤兘鑰楀亸缃彁绀猴紙EPB锛夊€煎彲浠ラ€氳繃 ``sysfs`` 涓嬬殑涓€涓睘鎬э紙鏂囦欢锛夋潵鏌ョ湅鎴栨洿鏂帮紝璇ュ睘鎬т綅浜?`/sys/devices/system/cpu/cpu<N>/power/`锛屽叾涓?CPU 缂栧彿 `<N>` 鍦ㄧ郴缁熷垵濮嬪寲鏃跺垎閰嶏細

`energy_perf_bias`
	浠?0 - 15 鐨勬粦鍔ㄥ埢搴︽樉绀鸿 CPU 褰撳墠鐨?EPB 鍊硷紝鍏朵腑
	鍊?0 瀵瑰簲鏈€楂樻€ц兘鐨勫亸濂斤紝鍊?15 瀵瑰簲鏈€澶ц妭鑳姐€?

	涓轰簡鏇存柊璇?CPU 鐨?EPB 鍊硷紝鍙互鍚戣灞炴€у啓鍏ワ紝鏃㈠彲浠ュ啓鍏ヤ笂杩?0 - 15 婊戝姩鍒诲害涓殑涓€涓暟瀛楋紝涔熷彲浠ュ啓鍏ヤ互涓嬩唬琛ㄥ叾鍚箟鐨勫瓧绗︿覆涔嬩竴锛?performance"銆?balance-performance"銆?normal"銆?balance-power"銆?power"銆?

	璇ュ睘鎬у瓨鍦ㄤ簬鎵€鏈夋敮鎸?EPB 鐗规€х殑鍦ㄧ嚎 CPU 涓娿€?

娉ㄦ剰锛岃櫧鐒跺埌澶勭悊鍣ㄧ殑 EPB 鎺ュ彛瀹氫箟鍦ㄩ€昏緫 CPU 绾у埆锛屼絾鏀寔瀹冪殑鐗╃悊瀵勫瓨鍣ㄥ彲鑳借澶氫釜 CPU 鍏变韩锛堜緥濡傦紝鍚屼竴灏佽涓殑 SMT 鍏勫紵鏍稿績鎴栨牳蹇冿級銆傚洜姝わ紝鏇存柊涓€涓?CPU 鐨?EPB 鍊煎彲鑳藉鑷村叾瀹?CPU 鐨?EPB 鍊煎彂鐢熷彉鍖栥€?
