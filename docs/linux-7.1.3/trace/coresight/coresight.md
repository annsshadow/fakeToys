## Coresight - ARM 涓婄殑纭欢杈呭姪杩借釜


   :Author:   Mathieu Poirier <mathieu.poirier@linaro.org>
   :Date:     September 11th, 2014

### 绠€浠?

Coresight 鏄竴绯诲垪鎶€鏈殑缁熺О锛岀敤浜庤皟璇曞熀浜?ARM 鐨?SoC銆傚畠鍖呮嫭 JTAG 鍜岀‖浠惰緟鍔╄拷韪殑瑙ｅ喅鏂规銆傛湰鏂囨。鍏虫敞鍚庤€呫€?
鍦ㄥ鐞嗘嫢鏈夎澶?SoC 浠ュ強 GPU銆丏MA 寮曟搸绛夊叾浠栫粍浠剁殑绯荤粺鏃讹紝纭欢杈呭姪杩借釜鍙樺緱瓒婃潵瓒婃湁鐢ㄣ€侫RM 閫氳繃涓嶅悓鐨勭粍浠跺紑鍙戜簡涓€濂楃‖浠惰緟鍔╄拷韪柟妗堬紝姣忎釜缁勪欢鍦ㄧ患鍚堬紙synthesis锛夋椂鍔犲叆璁捐浠ユ弧瓒崇壒瀹氱殑杩借釜闇€姹傘€傜粍浠堕€氬父鍒嗙被涓烘簮锛坰ource锛夈€侀摼璺紙link锛夊拰姹囷紙sink锛夛紝骞讹紙閫氬父锛夐€氳繃 AMBA 鎬荤嚎鍙戠幇銆?
鈥滄簮鈥濇牴鎹敤鎴烽厤缃殑杩借釜鍦烘櫙锛岀敓鎴愯〃绀哄鐞嗗櫒鎸囦护璺緞鐨勫帇缂╂祦銆傛鍚庤娴侀€氳繃杩炴帴婧愪笌涓€涓垨澶氫釜姹囩殑閾捐矾锛屽湪 coresight 绯荤粺涓紙缁忕敱 ATB 鎬荤嚎锛夋祦鍔ㄣ€傛眹浣滀负 coresight 瀹炵幇鐨勭鐐癸紝瑕佷箞灏嗗帇缂╂祦瀛樺偍鍦ㄥ唴瀛樼紦鍐插尯涓紝瑕佷箞鍒涘缓鍒板閮ㄤ笘鐣岀殑鎺ュ彛锛屼娇鏁版嵁鍙互浼犺緭鍒颁富鏈鸿€屼笉蹇呮媴蹇冩澘杞?coresight 鍐呭瓨缂撳啿鍖鸿濉弧銆?```

  *****************************************************************
 **************************** AMBA AXI  ****************************===||
  *****************************************************************    ||
        ^                    ^                            |            ||
        |                    |                            *            **
     0000000    :::::     0000000    :::::    :::::    @@@@@@@    ||||||||||||
     0 CPU 0<-->: C :     0 CPU 0<-->: C :    : C :    @ STM @    || System ||
  |->0000000    : T :  |->0000000    : T :    : T :<--->@@@@@     || Memory ||
  |  #######<-->: I :  |  #######<-->: I :    : I :      @@@<-|   ||||||||||||
  |  # ETM #    :::::  |  # PTM #    :::::    :::::       @   |
  |   #####      ^ ^   |   #####      ^ !      ^ !        .   |   |||||||||
  | |->###       | !   | |->###       | !      | !        .   |   || DAP ||
  | |   #        | !   | |   #        | !      | !        .   |   |||||||||
  | |   .        | !   | |   .        | !      | !        .   |      |  |
  | |   .        | !   | |   .        | !      | !        .   |      |  *
  | |   .        | !   | |   .        | !      | !        .   |      | SWD/
  | |   .        | !   | |   .        | !      | !        .   |      | JTAG
  *****************************************************************<-|
 *************************** AMBA Debug APB ************************
  *****************************************************************
   |    .          !         .          !        !        .    |
   |    .          *         .          *        *        .    |
  *****************************************************************
 ******************** Cross Trigger Matrix (CTM) *******************
  *****************************************************************
   |    .     ^              .                            .    |
   |    *     !              *                            *    |
  *****************************************************************
 ****************** AMBA Advanced Trace Bus (ATB) ******************
  *****************************************************************
   |          !                        ===============         |
   |          *                         ===== F =====<---------|
   |   :::::::::                         ==== U ====
   |-->:: CTI ::<!!                       === N ===
   |   :::::::::  !                        == N ==
   |    ^         *                        == E ==
   |    !  &&&&&&&&&       IIIIIII         == L ==
   |------>&& ETB &&<......II     I        =======
   |    !  &&&&&&&&&       II     I           .
   |    !                    I     I          .
   |    !                    I REP I<..........
   |    !                    I     I
   |    !!>&&&&&&&&&       II     I           *Source: ARM ltd.
   |------>& TPIU  &<......II    I            DAP = Debug Access Port
           &&&&&&&&&       IIIIIII            ETM = Embedded Trace Macrocell
               ;                              PTM = Program Trace Macrocell
               ;                              CTI = Cross Trigger Interface
               *                              ETB = Embedded Trace Buffer
          To trace port                       TPIU= Trace Port Interface Unit
                                              SWD = Serial Wire Debug

```
铏界劧缁勪欢鍦ㄧ洰鏍囦笂鐨勯厤缃槸閫氳繃 APB 鎬荤嚎瀹屾垚鐨勶紝浣嗘墍鏈夎拷韪暟鎹兘鍦?ATB 鎬荤嚎涓婂甫澶栵紙out-of-band锛変紶杈撱€侰TM 鎻愪緵浜嗕竴绉嶅湪 CoreSight 缁勪欢涔嬮棿鑱氬悎鍜屽垎鍙戜俊鍙风殑鏂瑰紡銆?
coresight 妗嗘灦鎻愪緵浜嗕竴涓腑蹇冪偣鏉ヨ〃绀恒€侀厤缃拰绠＄悊骞冲彴涓婄殑 coresight 璁惧銆傜涓€涓疄鐜伴泦涓湪鍩烘湰鐨勮拷韪姛鑳戒笂锛屾敮鎸?ETM/PTM銆乫unnel銆乺eplicator銆乀MC銆乀PIU 鍜?ETB 绛夌粍浠躲€傛湭鏉ョ殑宸ヤ綔灏嗘敮鎸佹洿澶嶆潅鐨?IP 鍧楋紝濡?STM 鍜?CTI銆?

### 缂╁啓涓庡垎绫?

缂╁啓锛?
PTM:
    绋嬪簭杩借釜瀹忓崟鍏冿紙Program Trace Macrocell锛?ETM:
    宓屽叆寮忚拷韪畯鍗曞厓锛圗mbedded Trace Macrocell锛?STM:
    绯荤粺杩借釜瀹忓崟鍏冿紙System trace Macrocell锛?ETB:
    宓屽叆寮忚拷韪紦鍐插尯锛圗mbedded Trace Buffer锛?ITM:
    妫€娴嬭拷韪畯鍗曞厓锛圛nstrumentation Trace Macrocell锛?TPIU:
     杩借釜绔彛鎺ュ彛鍗曞厓锛圱race Port Interface Unit锛?TMC-ETR:
        杩借釜鍐呭瓨鎺у埗鍣紝閰嶇疆涓哄祵鍏ュ紡杩借釜璺敱鍣紙Embedded Trace Router锛?TMC-ETF:
        杩借釜鍐呭瓨鎺у埗鍣紝閰嶇疆涓哄祵鍏ュ紡杩借釜 FIFO锛圗mbedded Trace FIFO锛?CTI:
    浜ゅ弶瑙﹀彂鎺ュ彛锛圕ross Trigger Interface锛?
鍒嗙被锛?
婧愶紙Source锛?
   ETMv3.x ETMv4, PTMv1.0, PTMv1.1, STM, STM500, ITM
閾捐矾锛圠ink锛?
   Funnel, replicator锛堟櫤鑳芥垨闈炴櫤鑳斤級, TMC-ETR
姹囷紙Sinks锛?
   ETBv1.0, ETB1.1, TPIU, TMC-ETF
鍏朵粬锛圡isc锛?
   CTI


### 璁惧鏍戠粦瀹?

璇﹁ `Documentation/devicetree/bindings/arm/arm,coresight-*.yaml`銆?
鎴嚦鎾板啓鏈枃鏃讹紝ITM銆丼TM 鍜?CTI 鐨勯┍鍔ㄥ皻鏈彁渚涳紝浣嗛璁′細闅忕潃鏂规鎴愮啛鑰屽姞鍏ャ€?

### 妗嗘灦涓庡疄鐜?

coresight 妗嗘灦鎻愪緵浜嗕竴涓腑蹇冪偣鏉ヨ〃绀恒€侀厤缃拰绠＄悊骞冲彴涓婄殑 coresight 璁惧銆備换浣曠鍚?coresight 瑙勮寖鐨勮澶囷紝鍙浣跨敤姝ｇ‘鐨?API锛屽氨鍙互鍚戞鏋舵敞鍐岋細


娉ㄥ唽鍑芥暟鎺ュ彈涓€涓?`struct coresight_desc *desc` 骞跺皢璁惧娉ㄥ唽鍒版牳蹇冩鏋躲€傛敞閿€鍑芥暟鎺ュ彈娉ㄥ唽鏃惰幏寰楃殑 `struct coresight_device *csdev` 寮曠敤銆?
濡傛灉娉ㄥ唽杩囩▼涓€鍒囬『鍒╋紝鏂拌澶囧皢
```

    root:~# ls /sys/bus/coresight/devices/
    replicator  20030000.tpiu    2201c000.ptm  2203c000.etm  2203e000.etm
    20010000.etb         20040000.funnel  2201d000.ptm  2203d000.etm
    root:~#

```
```

    struct coresight_desc {
            enum coresight_dev_type type;
            struct coresight_dev_subtype subtype;
            const struct coresight_ops *ops;
            struct coresight_platform_data *pdata;
            struct device *dev;
            const struct attribute_group **groups;
    };


```
鈥渃oresight_dev_type鈥?鏍囪瘑璁惧鏄粈涔堬紙鍗虫簮銆侀摼璺繕鏄眹锛夛紝鑰?鈥渃oresight_dev_subtype鈥?浼氳繘涓€姝ュ埢鐢昏绫诲瀷銆?
`struct coresight_ops` 鏄繀濉殑锛屽畠鍛婅瘔妗嗘灦濡備綍鎵ц涓庣粍浠剁浉鍏崇殑鍩烘湰鎿嶄綔锛屾瘡涓粍浠堕兘鏈変笉鍚岀殑闇€姹傞泦銆備负姝ゆ彁渚涗簡 `struct coresight_ops_sink`銆乣struct coresight_ops_link` 鍜?`struct coresight_ops_source`銆?
涓嬩竴涓瓧娈?`struct coresight_platform_data *pdata` 閫氳繃璋冪敤 `of_get_coresight_platform_data()` 鑾峰彇锛屼綔涓洪┍鍔?_probe 渚嬬▼鐨勪竴閮ㄥ垎锛屽苟涓?```

    static int etm_probe(struct amba_device *adev, const struct amba_id *id)
    {
     ...
     ...
     drvdata->dev = &adev->dev;
     ...
    }

```
鐗瑰畾绫诲埆鐨勮澶囷紙婧愩€侀摼璺垨姹囷級鍏锋湁鍙鍏舵墽琛岀殑閫氱敤鎿嶄綔锛堣 `struct coresight_ops`锛夈€俙**groups` 鏄笌浠呰缁勪欢鐗规湁鐨勬搷浣滅浉鍏崇殑 sysfs 鏉＄洰鍒楄〃銆傗€滃疄鐜板畾涔夆€濓紙Implementation defined锛夌殑瀹氬埗棰勬湡閫氳繃浣跨敤杩欎簺鏉＄洰鏉ヨ闂拰鎺у埗銆?
### 璁惧鍛藉悕鏂规


鍑虹幇鍦?鈥渃oresight鈥?鎬荤嚎涓婄殑璁惧琚懡鍚嶄负涓庡叾鐖惰澶囷紙鍗冲嚭鐜板湪 AMBA 鎬荤嚎鎴栧钩鍙版€荤嚎涓婄殑鐪熷疄璁惧锛夌浉鍚岀殑鍚嶇О銆傚洜姝ゅ悕绉板熀浜?Linux Open Firmware 灞傜殑鍛藉悕绾﹀畾锛屽嵆鍏堣窡闅忔椂閽熷悗闈㈣窡鐫€璁惧
```

    root:~# ls /sys/bus/coresight/devices/
     20010000.etf  20040000.funnel      20100000.stm     22040000.etm
     22140000.etm  230c0000.funnel      23240000.etm     20030000.tpiu
     20070000.etr  20120000.replicator  220c0000.funnel
     23040000.etm  23140000.etm         23340000.etm

```
鐒惰€岋紝闅忕潃 ACPI 鏀寔鐨勫紩鍏ワ紝鐪熷疄璁惧鐨勫悕绉版湁浜涙櫐娑╀笖涓嶇洿瑙傘€傚洜姝わ紝寮曞叆浜嗕竴绉嶆柊鐨勫懡鍚嶆柟妗堬紝鏍规嵁璁惧绫诲瀷浣跨敤鏇撮€氱敤鐨勫悕绉般€傝
```

  1) 缁戝畾鍒?CPU 鐨勮澶囷紝鏍规嵁 CPU 鐨勯€昏緫缂栧彿鍛藉悕銆?
     e.g, ETM bound to CPU0 is named "etm0"

  2) 鎵€鏈夊叾浠栬澶囬伒寰竴绉嶆ā寮忥紝"<device_type_prefix>N"锛屽叾涓細

	<device_type_prefix> 	- 鐗瑰畾浜庤澶囩被鍨嬬殑鍓嶇紑
	N			- 鏍规嵁鎺㈡祴椤哄簭鍒嗛厤鐨勫簭鍙枫€?
	e.g, tmc_etf0, tmc_etr0, funnel0, funnel1

```
```

    root:~# ls /sys/bus/coresight/devices/
     etm0     etm1     etm2         etm3  etm4      etm5      funnel0
     funnel1  funnel2  replicator0  stm0  tmc_etf0  tmc_etr0  tpiu0

```
涓嬮潰鐨勪竴浜涚ず渚嬪彲鑳藉紩鐢ㄦ棫鐨勫懡鍚嶆柟妗堬紝涓€浜涘紩鐢ㄦ柊鐨勬柟妗堬紝浠ョ‘璁や綘鍦ㄧ郴缁熶笂鐪嬪埌鐨勫苟闈炲紓甯搞€傚繀椤讳娇鐢ㄧ郴缁熶笂鎸囧畾浣嶇疆鍑虹幇鏃剁殑鈥滃悕绉扳€濄€?
### 鎷撴墤琛ㄧず


姣忎釜 CoreSight 缁勪欢閮芥湁涓€涓?`connections` 鐩綍锛屽叾涓寘鍚寚鍚戝叾浠?CoreSight 缁勪欢鐨勯摼鎺ャ€傝繖鍏佽鐢ㄦ埛鎺㈢储杩借釜鎷撴墤锛屽浜庤緝澶х殑绯荤粺锛屽彲浠ョ‘瀹氱粰瀹氭簮鏈€鍚堥€傜殑姹囥€傝繛鎺ヤ俊鎭繕鍙敤浜庣‘瀹氬摢浜?CTI 璁惧杩炴帴鍒扮粰瀹氱粍浠躲€傝鐩綍鍖呭惈涓€涓?`nr_links` 灞炴€э紝璇︾粏璇存槑鐩綍涓殑閾炬帴鏁伴噺銆?
瀵逛簬涓€涓?ETM 婧愶紝鏈緥涓负 Juno 骞冲彴涓婄殑 `etm0`锛屼竴涓吀鍨?```

  linaro-developer:~# ls - l /sys/bus/coresight/devices/etm0/connections
  <file details>  cti_cpu0 -> ../../../23020000.cti/cti_cpu0
  <file details>  nr_links
  <file details>  out:0 -> ../../../230c0000.funnel/funnel2

```
```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/funnel2/connections
  <file details> in:0 -> ../../../23040000.etm/etm0
  <file details> in:1 -> ../../../23140000.etm/etm3
  <file details> in:2 -> ../../../23240000.etm/etm4
  <file details> in:3 -> ../../../23340000.etm/etm5
  <file details> nr_links
  <file details> out:0 -> ../../../20040000.funnel/funnel0

```
```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/funnel0/connections
  <file details> in:0 -> ../../../220c0000.funnel/funnel1
  <file details> in:1 -> ../../../230c0000.funnel/funnel2
  <file details> nr_links
  <file details> out:0 -> ../../../20010000.etf/tmc_etf0

```
鎵惧埌绗竴涓眹 `tmc_etf0`銆傝繖鍙敤浜庢敹闆嗘暟鎹?```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/tmc_etf0/connections
  <file details> cti_sys0 -> ../../../20020000.cti/cti_sys0
  <file details> in:0 -> ../../../20040000.funnel/funnel0
  <file details> nr_links
  <file details> out:0 -> ../../../20150000.funnel/funnel4

```
```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/funnel4/connections
  <file details> in:0 -> ../../../20010000.etf/tmc_etf0
  <file details> in:1 -> ../../../20140000.etf/tmc_etf1
  <file details> nr_links
  <file details> out:0 -> ../../../20120000.replicator/replicator0

```
```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/replicator0/connections
  <file details> in:0 -> ../../../20150000.funnel/funnel4
  <file details> nr_links
  <file details> out:0 -> ../../../20030000.tpiu/tpiu0
  <file details> out:1 -> ../../../20070000.etr/tmc_etr0

```
```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/tmc_etr0/connections
  <file details> cti_sys0 -> ../../../20020000.cti/cti_sys0
  <file details> in:0 -> ../../../20120000.replicator/replicator0
  <file details> nr_links

```
濡備笅鎵€杩帮紝浣跨敤 sysfs 鏃讹紝鍙渶浣胯兘涓€涓眹鍜屼竴涓簮鍗冲彲鎴愬姛杩借釜銆傛鏋朵細鎸夐渶姝ｇ‘浣胯兘鎵€鏈変腑闂撮摼璺€?
娉ㄦ剰锛歚cti_sys0` 鍑虹幇鍦ㄤ笂闈袱涓繛鎺ュ垪琛ㄤ腑銆侰TI 鍙互杩炴帴鍒板涓澶囷紝骞堕€氳繃 CTM 浠ユ槦鍨嬫嫇鎵戞帓鍒椼€傝瑙?(Documentation/trace/coresight/coresight-ect.rst) [#fourth]_銆?```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/cti_sys0/connections
  <file details> nr_links
  <file details> stm0 -> ../../../20100000.stm/stm0
  <file details> tmc_etf0 -> ../../../20010000.etf/tmc_etf0
  <file details> tmc_etr0 -> ../../../20070000.etr/tmc_etr0
  <file details> tpiu0 -> ../../../20030000.tpiu/tpiu0


```
### 濡備綍浣跨敤杩借釜鍣ㄦā鍧?

浣跨敤 Coresight 妗嗘灦鏈変袱绉嶆柟寮忥細

1. 浣跨敤 perf 鍛戒护琛屽伐鍏枫€?2. 浣跨敤 sysFS 鎺ュ彛鐩存帴涓?Coresight 璁惧浜や簰銆?
浼樺厛浣跨敤鍓嶈€咃紝鍥犱负浣跨敤 sysFS 鎺ュ彛闇€瑕佸 Coresight 纭欢鏈夋繁鍏ョ悊瑙ｃ€備互涓嬪悇鑺傛彁渚涗袱绉嶆柟娉曠殑璇︾粏淇℃伅銆?
#### 浣跨敤 sysFS 鎺ュ彛


鍦ㄥ紑濮嬭拷韪敹闆嗕箣鍓嶏紝闇€瑕佺‘瀹氫竴涓?coresight 姹囥€傚湪浠讳綍缁欏畾鏃跺埢鍙互鍚敤鐨勬眹锛堜互鍙婃簮锛夋暟閲忔病鏈夐檺鍒躲€備綔涓洪€氱敤鎿嶄綔锛屾墍鏈夊睘浜庤姹囩殑璁惧
```

    root:/sys/bus/coresight/devices# ls
    replicator  20030000.tpiu    2201c000.ptm  2203c000.etm  2203e000.etm
    20010000.etb         20040000.funnel  2201d000.ptm  2203d000.etm
    root:/sys/bus/coresight/devices# ls 20010000.etb
    enable_sink  status  trigger_cntr
    root:/sys/bus/coresight/devices# echo 1 > 20010000.etb/enable_sink
    root:/sys/bus/coresight/devices# cat 20010000.etb/enable_sink
    1
    root:/sys/bus/coresight/devices#

```
鍦ㄥ惎鍔ㄦ椂锛屽綋鍓嶇殑 etm3x 椹卞姩浼氬皢绗竴涓湴鍧€姣旇緝鍣ㄩ厤缃负 鈥淿stext鈥?鍜?鈥淿etext鈥濓紝鏈川涓婅拷韪惤鍦ㄨ鑼冨洿鍐呯殑浠讳綍鎸囦护銆傚洜姝も€滀娇鑳解€濅竴涓簮灏嗙珛鍗?```

    root:/sys/bus/coresight/devices# echo 1 > 2201c000.ptm/enable_source
    root:/sys/bus/coresight/devices# cat 2201c000.ptm/enable_source
    1
    root:/sys/bus/coresight/devices# cat 20010000.etb/status
    Depth:          0x2000
    Status:         0x1
    RAM read ptr:   0x0
    RAM wrt ptr:    0x19d3   <----- The write pointer is moving
    Trigger cnt:    0x0
    Control:        0x1
    Flush status:   0x0
    Flush ctrl:     0x2001
    root:/sys/bus/coresight/devices#

```
```

    root:/sys/bus/coresight/devices# echo 0 > 2201c000.ptm/enable_source
    root:/sys/bus/coresight/devices#

```
```

    root:/sys/bus/coresight/devices# dd if=/dev/20010000.etb \
    of=~/cstrace.bin
    64+0 records in
    64+0 records out
    32768 bytes (33 kB) copied, 0.00125258 s, 26.2 MB/s
    root:/sys/bus/coresight/devices#

```
鏂囦欢 cstrace.bin 鍙互浣跨敤 鈥減tm2human鈥濄€丏S-5 鎴?Trace32 瑙ｅ帇缂┿€?
浠ヤ笅鏄竴涓?DS-5 杈撳嚭锛屽睍绀轰簡涓€涓疄楠屾€у惊鐜皢鍙橀噺閫掑鍒版煇涓€肩殑杩囩▼銆傝绀轰緥寰堢畝鍗曪紝鍗磋浜轰竴绐?coresight 鎵€鎻愪緵鐨勪赴瀵屽彲鑳芥€с€?```

    Info                                    Tracing enabled
    Instruction     106378866       0x8026B53C      E52DE004        false   PUSH     {lr}
    Instruction     0       0x8026B540      E24DD00C        false   SUB      sp,sp,#0xc
    Instruction     0       0x8026B544      E3A03000        false   MOV      r3,#0
    Instruction     0       0x8026B548      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Timestamp                                       Timestamp: 17106715833
    Instruction     319     0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Instruction     9       0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Instruction     7       0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Instruction     7       0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Instruction     10      0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Instruction     6       0x8026B560      EE1D3F30        false   MRC      p15,#0x0,r3,c13,c0,#1
    Instruction     0       0x8026B564      E1A0100D        false   MOV      r1,sp
    Instruction     0       0x8026B568      E3C12D7F        false   BIC      r2,r1,#0x1fc0
    Instruction     0       0x8026B56C      E3C2203F        false   BIC      r2,r2,#0x3f
    Instruction     0       0x8026B570      E59D1004        false   LDR      r1,[sp,#4]
    Instruction     0       0x8026B574      E59F0010        false   LDR      r0,[pc,#16] ; [0x8026B58C] = 0x80550368
    Instruction     0       0x8026B578      E592200C        false   LDR      r2,[r2,#0xc]
    Instruction     0       0x8026B57C      E59221D0        false   LDR      r2,[r2,#0x1d0]
    Instruction     0       0x8026B580      EB07A4CF        true    BL       {pc}+0x1e9344 ; 0x804548c4
    Info                                    Tracing enabled
    Instruction     13570831        0x8026B584      E28DD00C        false   ADD      sp,sp,#0xc
    Instruction     0       0x8026B588      E8BD8000        true    LDM      sp!,{pc}
    Timestamp                                       Timestamp: 17107041535

```
#### 浣跨敤 perf 妗嗘灦


Coresight 杩借釜鍣ㄤ娇鐢?Perf 妗嗘灦鐨勬€ц兘鐩戞帶鍗曞厓锛圥MU锛夋娊璞℃潵琛ㄧず銆傚洜姝?perf 妗嗘灦璐熻矗鏍规嵁鎰熷叴瓒ｈ繘绋嬬殑璋冨害鏃舵満鏉ユ帶鍒惰拷韪綍鏃惰鍚敤銆傚綋鍦ㄧ郴缁熶腑閰嶇疆濂芥椂锛孋oresight PMU 浼氬湪 perf 鍛戒护琛屽伐鍏锋煡璇㈡椂鍒楀嚭锛?
	linaro@linaro-nano:~$ ./perf list pmu

		List of pre-defined events (to be used in -e):

		cs_etm//                                    [Kernel PMU event]

鏃犺绯荤粺涓彲鐢ㄧ殑杩借釜鍣ㄦ暟閲忓灏戯紙閫氬父绛変簬澶勭悊鍣ㄦ牳蹇冩暟閲忥級锛屸€渃s_etm鈥?PMU 鍙細鍒楀嚭涓€娆°€?
Coresight PMU 鐨勫伐浣滄柟寮忎笌鍏朵粬浠讳綍 PMU 鐩稿悓锛屽嵆 PMU 鐨勫悕绉颁笌閰嶇疆閫夐」涓€璧峰湪鏂滄潬 鈥?鈥?鍐呮彁渚涳紙瑙?`Config option formats`_锛夈€?
### Perf 妗嗘灦鐨勯珮绾х敤娉?

#### 姹囩殑閫夋嫨


浼氫负涓?Perf 閰嶅悎浣跨敤鑷姩閫夋嫨涓€涓悎閫傜殑姹囷紝浣嗙敱浜庨€氬父浼氭湁澶氫釜姹囷紝瑕佷娇鐢ㄧ殑姹囩殑鍚嶇О鍙互浣滀负涓€涓互 鈥楡鈥?涓哄墠缂€鐨勭壒娈婇厤缃€夐」鏉ユ寚瀹氥€?
鍙敤鐨勬眹鍦?sysFS 涓嬪垪鍑猴紝浣嶄簬
```

	root@localhost:/sys/bus/event_source/devices/cs_etm/sinks# ls
	tmc_etf0  tmc_etr0  tpiu0

	root@linaro-nano:~# perf record -e cs_etm/@tmc_etr0/u --per-thread program

```
鍏充簬涓婅堪鍙婂叾浠栧浣曚娇鐢?Coresight 涓?perf 宸ュ叿鐨勭ず渚嬶紝鏇村淇℃伅鍙湪 openCSD gitHub 浠撳簱鐨?鈥淗OWTO.md鈥?鏂囦欢涓壘鍒?[#third]_銆?
#### 浣跨敤 perf 宸ュ叿杩涜 AutoFDO 鍒嗘瀽


perf 鍙敤浜庤褰曞拰鍒嗘瀽绋嬪簭鐨勮拷韪€?
鍙互浣跨敤甯?cs_etm 浜嬩欢鐨?鈥榩erf record鈥?璁板綍鎵ц锛?```

    perf record -e cs_etm//u --per-thread

```
鈥榩erf report鈥?鍜?鈥榩erf script鈥?鍛戒护鍙敤浜庡垎鏋愭墽琛岋紝浠庢寚浠よ拷韪腑鍚堟垚鎸囦护鍜屽垎鏀簨浠躲€傗€榩erf inject鈥?鍙敤浜庣敤鍚堟垚鐨勪簨浠舵浛鎹㈣拷韪暟鎹€?-itrace 閫夐」鎺у埗鍚堟垚浜嬩欢鐨勭被鍨嬪拰棰戠巼锛堣 perf 鏂囨。锛夈€?
娉ㄦ剰鐩墠浠呮敮鎸?64 浣嶇▼搴?鈥斺€?闇€瑕佹洿澶氬伐浣滄潵鏀寔 32 浣?Arm 绋嬪簭鐨勬寚浠よВ鐮併€?
#### 杩借釜 PID


鍐呮牳鍙互鏋勫缓涓哄皢 PID 鍊煎啓鍏?PE 鐨?ContextID 瀵勫瓨鍣ㄣ€傚浜庤繍琛屽湪 EL1 鐨勫唴鏍革紝PID 瀛樺偍鍦?CONTEXTIDR_EL1 涓€侾E 鍙互瀹炵幇 Arm 铏氭嫙鍖栦富鏈烘墿灞曪紙VHE锛夛紝鍐呮牳鍙繍琛屽湪 EL2 浣滀负铏氭嫙鍖栦富鏈猴紱姝ゆ椂锛孭ID 鍊煎瓨鍌ㄥ湪 CONTEXTIDR_EL2 涓€?
perf 鎻愪緵 PMU 鏍煎紡鏉ョ紪绋?ETM锛屽皢杩欎簺鍊兼彃鍏ヨ拷韪暟鎹紱PMU 鏍煎紡瀹氫箟濡備笅锛?
  鈥渃ontextid1鈥濓細鍦?EL1 鍐呮牳鍜?EL2 鍐呮牳涓婇兘鍙敤銆傚綋鍐呮牳杩愯鍦?EL1 鏃讹紝鈥渃ontextid1鈥?鍚敤 PID 杩借釜锛涘綋鍐呮牳杩愯鍦?EL2 鏃讹紝杩欏惎鐢ㄥ瀹㈡埛鏈哄簲鐢ㄧ▼搴?PID 鐨勮拷韪€?
  鈥渃ontextid2鈥濓細浠呭湪鍐呮牳杩愯浜?EL2 鏃跺彲鐢ㄣ€傞€変腑鏃讹紝鍚敤 EL2 鍐呮牳涓婄殑 PID 杩借釜銆?
  鈥渃ontextid鈥濓細灏嗕綔涓哄惎鐢?PID 杩借釜閫夐」鐨勫埆鍚嶃€傚嵆锛屽湪 EL1 鍐呮牳涓?contextid == contextid1锛屽湪 EL2 鍐呮牳涓?contextid == contextid2銆?
perf 鎬绘槸鍦ㄧ浉鍏崇殑 EL 涓婂惎鐢?PID 杩借釜锛岃繖鏄€氳繃鑷姩鍚敤 鈥渃ontextid鈥?閰嶇疆瀹炵幇鐨?鈥斺€?浣嗗浜?EL2锛屽彲浠ヤ娇鐢?鈥渃ontextid1鈥?鍜?鈥渃ontextid2鈥?閰嶇疆杩涜鐗瑰畾璋冩暣锛屼緥濡傦紝濡傛灉鐢ㄦ埛鎯冲悓鏃惰拷韪富鏈哄拰瀹㈡埛鏈虹殑 PID锛屽彲浠ュ悓鏃惰缃?鈥渃ontextid1鈥?鍜?鈥渃ontextid2鈥?杩欎袱涓厤缃細

  perf record -e cs_etm/contextid1,contextid2/u -- vm


#### 涓哄弽棣堝鍚戜紭鍖栵紙Feedback Directed Optimization锛夌敓鎴愯鐩栫巼鏂囦欢锛欰utoFDO


鈥榩erf inject鈥?鎺ュ彈 --itrace 閫夐」锛屾鏃惰拷韪暟鎹绉婚櫎骞舵浛鎹负鍚堟垚鐨勪簨浠躲€備緥濡?```

	perf inject --itrace --strip -i perf.data -o perf.data.new

```
浠ヤ笅鏄娇鐢?ARM ETM 杩涜 autoFDO 鐨勭ず渚嬨€傚畠闇€瑕?autofdo (https://github.com/google/autofdo) 鍜?gcc 5 鐗堟湰銆俠ubble sort 绀轰緥鏉ヨ嚜 AutoFDO 鏁欑▼ (https://gcc.gnu.org/wiki/AutoFDO/Tutorial)銆?```

	$ gcc-5 -O3 sort.c -o sort
	$ taskset -c 2 ./sort
	Bubble sorting array of 30000 elements
	5910 ms

	$ perf record -e cs_etm//u --per-thread taskset -c 2 ./sort
	Bubble sorting array of 30000 elements
	12543 ms
	[ perf record: Woken up 35 times to write data ]
	[ perf record: Captured and wrote 69.640 MB perf.data ]

	$ perf inject -i perf.data -o inj.data --itrace=il64 --strip
	$ create_gcov --binary=./sort --profile=inj.data --gcov=sort.gcov -gcov_version=1
	$ gcc-5 -O3 -fauto-profile=sort.gcov sort.c -o sort_autofdo
	$ taskset -c 2 ./sort_autofdo
	Bubble sorting array of 30000 elements
	5806 ms

```
#### 閰嶇疆閫夐」鏍煎紡


浠ヤ笅瀛楃涓插彲浠ュ湪 perf 鍛戒护琛屼笂鎻愪緵浜?// 涔嬮棿锛屼互鍚敤鍚勭閫夐」銆傚畠浠篃鍒楀湪鏂囦欢澶?/sys/bus/event_source/devices/cs_etm/format/ 涓?
   :header-rows: 1

   - - 閫夐」锛圤ption锛?     - 鎻忚堪锛圖escription锛?   - - branch_broadcast
     - 绯荤粺绾ц缃殑浼氳瘽鏈湴鐗堟湰锛欵TM_MODE_BB <coresight-branch-broadcast>
   - - contextid
     - 瑙?`Tracing PID`_
   - - contextid1
     - 瑙?`Tracing PID`_
   - - contextid2
     - 瑙?`Tracing PID`_
   - - configid
     - 鐢ㄤ簬鑷畾涔夐厤缃殑閫夋嫨銆傝繖鏄竴涓疄鐜扮粏鑺傦紝涓嶇洿鎺ヤ娇鐢紝瑙?trace/coresight/coresight-config:Using Configurations in perf
   - - preset
     - 鑷畾涔夐厤缃腑鍙傛暟鐨勮鐩栵紝瑙?trace/coresight/coresight-config:Using Configurations in perf
   - - sinkid
     - 鐢ㄤ簬閫夋嫨姹囩殑瀛楃涓茬殑鍝堝笇鐗堟湰锛屼娇鐢?@ 琛ㄧず娉曟椂鑷姩璁剧疆銆傝繖鏄唴閮ㄥ疄鐜扮粏鑺傦紝涓嶇洿鎺ヤ娇鐢紝瑙?`Using perf
       framework`_銆?   - - cycacc
     - 绯荤粺绾ц缃殑浼氳瘽鏈湴鐗堟湰锛?ref:`ETMv4_MODE_CYCACC
       <coresight-cycle-accurate>`
   - - retstack
     - 绯荤粺绾ц缃殑浼氳瘽鏈湴鐗堟湰锛?ref:`ETM_MODE_RETURNSTACK
       <coresight-return-stack>`
   - - timestamp
     - 鎺у埗鏃堕棿鎴崇殑鐢熸垚鍜岄棿闅斻€?
       0 = 鍏抽棴锛? = 鏈€灏忛棿闅?.. 15 = 鏈€澶ч棿闅斻€?
       鍊?1 - 14 浣跨敤涓€涓瘡鍛ㄦ湡閫掑噺鐨勮鏁板櫒锛屽湪閫掑噺鍒伴浂鏃剁敓鎴愭椂闂存埑銆傝鏁板櫒鐨勯噸杞藉€间负 2 ^ (interval
       - 1)銆傚鏋滃€间负 1锛屽垯閲嶈浇鍊间负 1锛涘鏋滃€间负 11锛屽垯閲嶈浇鍊间负 1024锛屼緷姝ょ被鎺ㄣ€?
       璁剧疆鏈€澶ч棿闅旓紙15锛夊皢绂佺敤璁℃暟鍣ㄧ敓鎴愮殑鏃堕棿鎴筹紝閲婃斁璁℃暟鍣ㄨ祫婧愶紝鍙繚鐣欑敓鎴?SYNC 鍖呮椂鍙戝嚭鐨勬椂闂存埑銆傚悓姝ラ棿闅旂敱 TRCSYNCPR.PERIOD 鎺у埗锛岄粯璁ゆ瘡 4096 瀛楄妭鐨勮拷韪敓鎴愪竴涓€?
   - - cc_threshold
     - 鍛ㄦ湡璁℃暟闃堝€笺€傚鏋滆繖閲屾湭鎻愪緵鍊兼垨鎻愪緵鐨勫€间负 0锛屽垯浣跨敤榛樿鍊硷紙鍗?0x100锛夈€傚鏋滄彁渚涚殑鍊煎皬浜庢渶灏忓懆鏈熼槇鍊硷紙鐢?TRCIDR3.CCITMIN 鎸囩ず锛夛紝鍒欐敼鐢ㄦ渶灏忓€笺€?
### 濡備綍浣跨敤 STM 妯″潡


浣跨敤 System Trace Macrocell 妯″潡涓庝娇鐢ㄨ拷韪櫒鐩稿悓 鈥斺€?鍞竴鐨勫尯鍒槸瀹㈡埛绔┍鍔ㄨ拷韪崟鑾凤紝鑰屼笉鏄唬鐮佷腑鐨勭▼搴忔祦銆?
涓庝换浣曞叾浠?CoreSight 缁勪欢涓€鏍凤紝鍏充簬 STM 杩借釜鍣ㄧ殑鍏蜂綋淇℃伅鍙互
```

    root@genericarmv8:~# ls /sys/bus/coresight/devices/stm0
    enable_source   hwevent_select  port_enable     subsystem       uevent
    hwevent_enable  mgmt            port_select     traceid
    root@genericarmv8:~#

```
涓庝换浣曞叾浠栨簮涓€鏍凤紝闇€瑕佸厛纭畾姹囧苟浣胯兘 STM锛岀劧鍚?```

    root@genericarmv8:~# echo 1 > /sys/bus/coresight/devices/tmc_etf0/enable_sink
    root@genericarmv8:~# echo 1 > /sys/bus/coresight/devices/stm0/enable_source

```
姝ゅ悗鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鍙互閫氳繃 devfs 璇锋眰骞朵娇鐢ㄩ€氶亾
```

    root@genericarmv8:~# ls -l /dev/stm0
    crw-------    1 root     root       10,  61 Jan  3 18:11 /dev/stm0
    root@genericarmv8:~#

```
鍏充簬濡備綍浣跨敤閫氱敤 STM API 鐨勮缁嗕俊鎭彲鍦ㄦ澶勬壘鍒帮細
- Documentation/trace/stm.rst [#second]_銆?
### CTI 涓?CTM 妯″潡


CTI锛圕ross Trigger Interface锛屼氦鍙夎Е鍙戞帴鍙ｏ級鍦ㄥ崟涓?CTI 涓庣粍浠朵箣闂存彁渚涗竴缁勮Е鍙戜俊鍙凤紝骞跺彲浠ラ€氳繃 CTM锛圕ross Trigger Matrix锛屼氦鍙夎Е鍙戠煩闃碉級涓婄殑閫氶亾鍦ㄦ墍鏈?CTI 涔嬮棿浼犳挱杩欎簺淇″彿銆?
鎻愪緵浜嗕竴浠藉崟鐙殑鏂囨。鏂囦欢鏉ヨВ閲婅繖浜涜澶囩殑浣跨敤銆?(Documentation/trace/coresight/coresight-ect.rst) [#fourth]_銆?
### CoreSight 绯荤粺閰嶇疆


CoreSight 缁勪欢鍙互鏄叿鏈夎澶氱紪绋嬮€夐」鐨勫鏉傝澶囥€傛澶栵紝缁勪欢鍙互琚紪绋嬩负鍦ㄦ暣涓郴缁熶腑鐩镐簰浜や簰銆?
鎻愪緵浜?CoreSight 绯荤粺閰嶇疆绠＄悊鍣紝浠ヤ究鑳戒粠 perf 鍜?sysfs 涓交鏉鹃€夋嫨鍜屼娇鐢ㄨ繖浜涘鏉傜殑缂栫▼閰嶇疆銆?
鏇村淇℃伅璇峰弬闃呭崟鐙殑鏂囨。銆?(Documentation/trace/coresight/coresight-config.rst) [#fifth]_銆?