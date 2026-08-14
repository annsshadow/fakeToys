
DPAA2 DPIO锛堟暟鎹€氳矾 I/O锛夋杩?



:鐗堟潈鎵€鏈夛細|copy| 2016-2018 NXP


鏈枃妗ｆ杩颁簡 Freescale DPAA2 DPIO 椹卞姩銆?


绠€浠?



DPAA2 DPIO锛堟暟鎹€氳矾 I/O锛夋槸涓€涓‖浠跺璞★紝鎻愪緵灏嗗抚鍏ラ槦鍜屽嚭闃熷埌缃戠粶鎺ュ彛鍙婂叾浠栧姞閫熷櫒鐨勬帴鍙ｃ€侱PIO 杩樹负缃戠粶鎺ュ彛鎻愪緵纭欢缂撳啿姹犵鐞嗐€?


鏈枃妗ｆ杩颁簡 Linux DPIO 椹卞姩銆佸叾瀛愮粍浠跺強鍏?API銆?


鏈夊叧 DPAA2 鐨勬€讳綋姒傝堪浠ュ強 Linux 涓?DPAA2 椹卞姩鎬讳綋鏋舵瀯锛岃鍙傞槄
Documentation/networking/device_drivers/ethernet/freescale/dpaa2/overview.rst銆?


椹卞姩姒傝堪



DPIO 椹卞姩缁戝畾鍒板湪 fsl-mc 鎬荤嚎涓婂彂鐜扮殑 DPIO 瀵硅薄锛屽苟鎻愪緵浠ヤ笅鏈嶅姟锛?


  A. 鍏佽鍏朵粬椹卞姩锛堜緥濡備互澶綉椹卞姩锛変负鍏跺悇鑷殑瀵硅薄鍏ラ槦鍜屽嚭闃熷抚
  B. 鍏佽椹卞姩娉ㄥ唽鏁版嵁鍙敤閫氱煡鍥炶皟锛屽綋闃熷垪鎴栭€氶亾涓婃湁鏁版嵁鍙敤鏃惰Е鍙?
  C. 鍏佽椹卞姩绠＄悊纭欢缂撳啿姹?


Linux DPIO 椹卞姩鐢?3 涓富瑕佺粍浠舵瀯鎴愨€斺€?
   DPIO 瀵硅薄椹卞姩鈥斺€旂鐞?DPIO 瀵硅薄鐨?fsl-mc 椹卞姩


   DPIO 鏈嶅姟鈥斺€斿悜鍏朵粬 Linux 椹卞姩鎻愪緵鏈嶅姟 API

```

          fsl-mc          other
           bus           drivers
            |               |
        +---+----+   +------+-----+
        |DPIO obj|   |DPIO service|
        | driver |---|  (DPIO)    |
        +--------+   +------+-----+
                            |
                     +------+-----+
                     |    QBman   |
                     | portal i/f |
                     +------------+
                            |
                         hardware


```

涓嬪浘灞曠ず浜?DPIO 椹卞姩鍚勭粍浠跺浣曚笌鍏朵粬閮ㄥ垎閰嶅悎
```

                                                   +------------+
                                                   | OS Network |
                                                   |   Stack    |
                 +------------+                    +------------+
                 | Allocator  |. . . . . . .       |  Ethernet  |
                 |(DPMCP,DPBP)|                    |   (DPNI)   |
                 +-.----------+                    +---+---+----+
                  .          .                         ^   |
                 .            .           <data avail, |   |<enqueue,
                .              .           tx confirm> |   | dequeue>
    +-------------+             .                      |   |
    | DPRC driver |              .    +--------+ +------------+
    |   (DPRC)    |               . . |DPIO obj| |DPIO service|
    +----------+--+                   | driver |-|  (DPIO)    |
               |                      +--------+ +------+-----+
               |<dev add/remove>                 +------|-----+
               |                                 |   QBman    |
          +----+--------------+                  | portal i/f |
          |   MC-bus driver   |                  +------------+
          |                   |                     |
          | /soc/fsl-mc       |                     |
          +-------------------+                     |
                                                    |
 =========================================|=========|========================
                                        +-+--DPIO---|-----------+
                                        |           |           |
                                        |        QBman Portal   |
                                        +-----------------------+

 ============================================================================


```

DPIO 瀵硅薄椹卞姩锛坉pio-driver.c锛?



   璇?dpio-driver 缁勪欢鍚?fsl-mc 鎬荤嚎娉ㄥ唽锛屼互澶勭悊绫诲瀷涓?"dpio" 鐨勫璞°€俻robe() 鐨勫疄鐜板鐞?DPIO 鐨勫熀鏈垵濮嬪寲锛屽寘鎷槧灏?DPIO 鍖哄煙锛圦Bman SW portal锛変互鍙婂垵濮嬪寲涓柇骞舵敞鍐?irq 澶勭悊鍑芥暟銆俤pio-driver 灏嗘帰娴嬪埌鐨?DPIO 娉ㄥ唽鍒?dpio-service銆?


DPIO 鏈嶅姟锛坉pio-service.c, dpaa2-io.h锛?



   璇?dpio service 缁勪欢鍚?DPAA2 椹卞姩锛堜緥濡備互澶綉椹卞姩锛夋彁渚涘叆闃熴€侀€氱煡鍜岀紦鍐茬鐞嗘柟闈㈢殑鏈嶅姟銆傜郴缁熼€氬父浼氫负姣忎釜 CPU 鍒嗛厤 1 涓?DPIO 瀵硅薄锛屼互渚垮叆闃熸搷浣滆兘澶熷湪鎵€鏈?CPU 涓婂悓鏃跺彂鐢熴€?


   閫氱煡澶勭悊
      dpaa2_io_service_register()


      dpaa2_io_service_deregister()


      dpaa2_io_service_rearm()


   鍏ラ槦
      dpaa2_io_service_pull_fq()


      dpaa2_io_service_pull_channel()


      dpaa2_io_service_enqueue_fq()


      dpaa2_io_service_enqueue_qd()


      dpaa2_io_store_create()


      dpaa2_io_store_destroy()


      dpaa2_io_store_next()


   缂撳啿姹犵鐞?
      dpaa2_io_service_release()


      dpaa2_io_service_acquire()


QBman portal 鎺ュ彛锛坬bman-portal.c锛?



   璇?qbman-portal 缁勪欢鎻愪緵鐢ㄤ簬鎵ц搴曞眰纭欢浣嶆搷浣滅殑 API锛屼緥濡傦細


      - 鍒濆鍖?Qman 杞欢 portal
      - 鏋勫缓骞跺彂閫?portal 鍛戒护
      - portal 涓柇閰嶇疆涓庡鐞?


   杩欎簺 qbman-portal API 涓嶅悜鍏朵粬椹卞姩鍏紑锛屼粎渚?dpio-service 浣跨敤銆?


鍏朵粬锛坉paa2-fd.h, dpaa2-global.h锛?



   甯ф弿杩扮浠ュ強鍒嗘暎/鑱氶泦锛坰catter-gather锛夌殑瀹氫箟锛屼互鍙婄敤浜庢搷浣滃畠浠殑 API锛屽畾涔夊湪 dpaa2-fd.h 涓€?


   鍑洪槦缁撴灉缁撴瀯浣撳強鍏惰В鏋?API 瀹氫箟鍦?dpaa2-global.h 涓€?

