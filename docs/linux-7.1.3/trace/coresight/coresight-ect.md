
## CoreSight Embedded Cross Trigger (CTI & CTM).


    :Author:   Mike Leach <mike.leach@linaro.org>
    :Date:     November 2019

### 纭欢 Description


The CoreSight Cross Trigger 鎺ュ彛 (CTI) 鏄?涓€涓?纭欢 璁惧 璇?takes
鍚勪釜 杈撳叆 鍜?杈撳嚭 纭欢 signals known 浣滀负 triggers 鍒?鍜?鏉ヨ嚜
璁惧 鍜?interconnects them 閫氳繃 the Cross Trigger Matrix (CTM) 鍒?鍏朵粬
璁惧 閫氳繃 numbered channels, 涓轰簡 propagate 浜嬩欢 涔嬮棿 璁惧.

```

 0000000  in_trigs  :::::::
 0 C   0----------->:     :             +======>(other CTI channel IO)
 0  P  0<-----------:     :             v
 0   U 0  out_trigs :     : Channels  *****      :::::::
 0000000            : CTI :<=========>*CTM*<====>: CTI :---+
 #######  in_trigs  :     : (id 0-3)  *****      :::::::   v
 # ETM #----------->:     :                         ^   #######
 #     #<-----------:     :                         +---# ETR #
 ####### out_trigs  :::::::                             #######

```
The CTI 椹卞姩 enables the programming 鐨?the CTI 鍒?attach triggers 鍒?
channels. 褰?涓€涓?杈撳叆 trigger becomes active, the attached channel 灏?
become active. 浠讳綍 杈撳嚭 trigger attached 鍒?璇?channel 灏?涔?
become active. The active channel 鏄?propagated 鍒?鍏朵粬 CTIs 閫氳繃 the CTM,
activating connected 杈撳嚭 triggers 閭ｉ噷, 闄ら潪 filtered 鐢?the CTI
channel gate.

瀹冩槸 涔?鍙兘 鍒?activate 涓€涓?channel 浣跨敤 绯荤粺 杞欢 directly
programming 瀵勫瓨鍣?鍦?the CTI.

The CTIs 鏄?registered 鐢?the 绯荤粺 鍒?涓?associated 涓?CPUs 鍜?鎴?鍏朵粬
CoreSight 璁惧 鍦?the trace 鏁版嵁 path. 褰?杩欎簺 璁惧 鏄?宸插惎鐢?the
attached CTIs 灏?涔?涓?宸插惎鐢? 榛樿鎯呭喌涓?鍦?鐢垫簮 up the CTIs 鍏锋湁
鏃?programmed trigger/channel attachments, 鍥犳 灏?涓?affect the 绯荤粺
鐩村埌 explicitly programmed.

The 纭欢 trigger connections 涔嬮棿 CTIs 鍜?璁惧 鏄?implementation
瀹氫箟, 闄ら潪 the CPU/ETM combination 鏄?涓€涓?v8 architecture, 鍦?鍏?case
the connections 鍏锋湁 涓€涓?architecturally 瀹氫箟 鏍囧噯 layout.

The 纭欢 trigger signals 鍙?涔?涓?connected 鍒?non-CoreSight 璁惧
(e.g. UART), 鎴?涓?propagated off 鑺墖 浣滀负 纭欢 IO lines.

鍏ㄩ儴 the CTI 璁惧 鏄?associated 涓?涓€涓?CTM. 鍦?璁稿 绯荤粺 閭ｉ噷 灏?涓?涓€涓?
鍗曚釜 effective CTM (one CTM, 鎴?澶氫釜 CTMs 鍏ㄩ儴 interconnected), 浣?瀹冩槸
鍙兘 璇?绯荤粺 鍙?鍏锋湁 nets 鐨?CTIs+CTM 璇?鏄?涓?interconnected 鐢?
涓€涓?CTM 鍒?姣忎釜 鍏朵粬. 鍦?杩欎簺 绯荤粺 涓€涓?CTM 绱㈠紩 鏄?declared 鍒?associate
CTI 璁惧 璇?鏄?interconnected 閫氳繃 涓€涓?given CTM.

### Sysfs 鏂囦欢 鍜?directories


The CTI 璁惧 appear 鍦?the existing CoreSight 鎬荤嚎 alongside the 鍏朵粬
```

    >$ ls /sys/bus/coresight/devices
     cti_cpu0  cti_cpu2  cti_sys0  etm0  etm2  funnel0  replicator0  tmc_etr0
     cti_cpu1  cti_cpu3  cti_sys1  etm1  etm3  funnel1  tmc_etf0     tpiu0

```
The `cti_cpu<N>` named CTIs 鏄?associated 涓?涓€涓?CPU, 鍜?浠讳綍 ETM 浣跨敤 鐢?
璇?鏍稿績. The `cti_sys<N>` CTIs 鏄?閫氱敤 绯荤粺 infrastructure CTIs 璇?
鍙?涓?associated 涓?鍏朵粬 CoreSight 璁惧, 鎴?鍏朵粬 绯荤粺 纭欢
```

  >$ ls /sys/bus/coresight/devices/etm0/cti_cpu0
  channels  ctmid  enable  nr_trigger_cons mgmt  power powered  regs
  connections subsystem triggers0 triggers1  uevent

```
**Key 鏂囦欢 items 鏄?-**
   - `enable`: enables/disables the CTI. 璇诲彇 鍒?determine 鐢垫祦 鐘舵€?
     鑻?姝?鏄剧ず 浣滀负 宸插惎鐢?(1), 浣?`powered` 鏄剧ず unpowered (0), 鐒跺悗
     the 鍚敤 indicates 涓€涓?璇锋眰 鍒?宸插惎鐢?褰?the 璁惧 鏄?powered.
   - `ctmid` : associated CTM - 浠?relevant 鑻?绯荤粺 鍏锋湁 澶氫釜 CTI+CTM
     clusters 璇?鏄?涓?interconnected.
   - `nr_trigger_cons` : 鎬昏 connections - triggers<N> directories.
   - `powered` : 璇诲彇 鍒?determine 鑻?the CTI 鏄?currently powered.

**Sub-directories:-**
   - `triggers<N>`: 鍖呭惈 鍒楀嚭 鐨?triggers 鐢ㄤ簬 涓€涓?鍚勪釜 杩炴帴.
   - `channels`: 鍖呭惈 the channel API - CTI 涓昏 programming 鎺ュ彛.
   - `regs`: Gives access 鍒?the raw programmable CTI regs.
   - `mgmt`: the 鏍囧噯 CoreSight 绠＄悊 瀵勫瓨鍣?
   - `connections`: Links 鍒?connected **CoreSight** 璁惧. The 鏁板瓧 鐨?
     links 鍙?涓?0 鍒?`nr_trigger_cons`. Actual 鏁板瓧 given 鐢?`nr_links`
     鍦?姝?directory.


#### triggers<N> directories


鍚勪釜 trigger 杩炴帴 information. 姝?describes trigger signals 鐢ㄤ簬
CoreSight 鍜?non-CoreSight connections.

姣忎釜 triggers directory 鍏锋湁 涓€涓?set 鐨?鍙傛暟 describing the triggers 鐢ㄤ簬
the 杩炴帴.

   - `name` : name 鐨?杩炴帴
   - `in_signals` : 杈撳叆 trigger 淇″彿 indexes 浣跨敤 鍦?姝?杩炴帴.
   - `in_types` : functional types 鐢ㄤ簬 鍦?signals.
   - `out_signals` : 杈撳嚭 trigger signals 鐢ㄤ簬 姝?杩炴帴.
   - `out_types` : functional types 鐢ㄤ簬 out signals.

```

    >$ ls ./cti_cpu0/triggers0/
    in_signals  in_types  name  out_signals  out_types
    >$ cat ./cti_cpu0/triggers0/name
    cpu0
    >$ cat ./cti_cpu0/triggers0/out_signals
    0-2
    >$ cat ./cti_cpu0/triggers0/out_types
    pe_edbgreq pe_dbgrestart pe_ctiirq
    >$ cat ./cti_cpu0/triggers0/in_signals
    0-1
    >$ cat ./cti_cpu0/triggers0/in_types
    pe_dbgtrigger pe_pmuirq

```
鑻?涓€涓?杩炴帴 鍏锋湁 zero signals 鍦?浠讳竴涓?the '鍦? 鎴?'out' triggers 鐒跺悗
閭ｄ簺 鍙傛暟 灏?涓?omitted.

#### Channels API Directory


姝?鎻愪緵 涓€涓?easy way 鍒?attach triggers 鍒?channels, 鏃?needing
the 澶氫釜 娉ㄥ唽 鎿嶄綔 璇?鏄?蹇呴渶 鑻?manipulating the
'regs' sub-directory elements directly.

```

   >$ ls ./cti_sys0/channels/
   chan_clear         chan_inuse      chan_xtrigs_out     trigin_attach
   chan_free          chan_pulse      chan_xtrigs_reset   trigin_detach
   chan_gate_disable  chan_set        chan_xtrigs_sel     trigout_attach
   chan_gate_enable   chan_xtrigs_in  trig_filter_enable  trigout_detach
   trigout_filtered

```
```

  echo <chan> [<trigger>] > /<device_path>/<operation>

```
浣曞 the 鍙€?<trigger> 鏄?浠?needed 鐢ㄤ簬 trigXX_attach | detach
鎿嶄綔.

```

   >$ echo 0 1 > ./cti_sys0/channels/trigout_attach
   >$ echo 0 > ./cti_sys0/channels/chan_set

```
Attaches trigout(1) 鍒?channel(0), 鐒跺悗 activates channel(0) generating 涓€涓?
set 鐘舵€?鍦?cti_sys0.trigout(1)


**API 鎿嶄綔**

   - `trigin_attach, trigout_attach`: Attach 涓€涓?channel 鍒?涓€涓?trigger 淇″彿.
   - `trigin_detach, trigout_detach`: Detach 涓€涓?channel 鏉ヨ嚜 涓€涓?trigger 淇″彿.
   - `chan_set`: Set the channel - the set 鐘舵€?灏?涓?propagated around
     the CTM 鍒?鍏朵粬 connected 璁惧.
   - `chan_clear`: Clear the channel.
   - `chan_pulse`: Set the channel 鐢ㄤ簬 涓€涓?鍗曚釜 CoreSight clock cycle.
   - `chan_gate_enable`: 鍐欏叆 鎿嶄綔 sets the CTI gate 鍒?propagate
     (鍚敤) the channel 鍒?鍏朵粬 璁惧. 姝?鎿嶄綔 takes 涓€涓?channel
     鏁板瓧. CTI gate 鏄?宸插惎鐢?鐢ㄤ簬 鍏ㄩ儴 channels 榛樿鎯呭喌涓?鍦?鐢垫簮 up. 璇诲彇
     鍒?鍒楀嚭 the currently 宸插惎鐢?channels 鍦?the gate.
   - `chan_gate_disable`: 鍐欏叆 channel 鏁板瓧 鍒?绂佺敤 gate 鐢ㄤ簬 璇?
     channel.
   - `chan_inuse`: 鏄剧ず the 鐢垫祦 channels attached 鍒?浠讳綍 淇″彿
   - `chan_free`: 鏄剧ず channels 涓?鏃?attached signals.
   - `chan_xtrigs_sel`: 鍐欏叆 涓€涓?channel 鏁板瓧 鍒?select 涓€涓?channel 鍒?view,
     璇诲彇 鍒?鏄剧ず the selected channel 鏁板瓧.
   - `chan_xtrigs_in`: 璇诲彇 鍒?鏄剧ず the 杈撳叆 triggers attached 鍒?
     the selected view channel.
   - `chan_xtrigs_out`:璇诲彇 鍒?鏄剧ず the 杈撳嚭 triggers attached 鍒?
     the selected view channel.
   - `trig_filter_enable`: Defaults 鍒?宸插惎鐢? 绂佺敤 鍒?鍏佽 potentially
     dangerous 杈撳嚭 signals 鍒?涓?set.
   - `trigout_filtered`: Trigger out signals 璇?鏄?prevented 鏉ヨ嚜 姝ｅ湪
     set 鑻?filtering `trig_filter_enable` 鏄?宸插惎鐢? One 浣跨敤 鏄?鍒?prevent
     accidental `EDBGREQ` signals stopping 涓€涓?鏍稿績.
   - `chan_xtrigs_reset`: 鍐欏叆 1 鍒?clear 鍏ㄩ儴 channel / trigger programming.
     Resets 璁惧 纭欢 鍒?榛樿 鐘舵€?


The 绀轰緥 涓嬫枃 attaches 杈撳叆 trigger 绱㈠紩 1 鍒?channel 2, 鍜?杈撳嚭
trigger 绱㈠紩 6 鍒?the 鐩稿悓 channel. 瀹?鐒跺悗 examines the 鐘舵€?鐨?the
channel / trigger connections 浣跨敤 the appropriate sysfs attributes.

The 璁剧疆 mean 璇?鑻?浠讳竴涓?杈撳叆 trigger 1, 鎴?channel 2 go active 鐒跺悗
trigger out 6 灏?go active. 鎴戜滑 鐒跺悗 鍚敤 the CTI, 鍜?浣跨敤 the 杞欢
channel control 鍒?activate channel 2. 鎴戜滑 鍙傝 the active channel 鍦?the
`choutstatus` 娉ㄥ唽 鍜?the active 淇″彿 鍦?the `trigoutstatus`
娉ㄥ唽. Finally clearing the channel removes 姝?

```

   .../cti_sys0/channels# echo 2 1 > trigin_attach
   .../cti_sys0/channels# echo 2 6 > trigout_attach
   .../cti_sys0/channels# cat chan_free
   0-1,3
   .../cti_sys0/channels# cat chan_inuse
   2
   .../cti_sys0/channels# echo 2 > chan_xtrigs_sel
   .../cti_sys0/channels# cat chan_xtrigs_trigin
   1
   .../cti_sys0/channels# cat chan_xtrigs_trigout
   6
   .../cti_sys0/# echo 1 > enable
   .../cti_sys0/channels# echo 2 > chan_set
   .../cti_sys0/channels# cat ../regs/choutstatus
   0x4
   .../cti_sys0/channels# cat ../regs/trigoutstatus
   0x40
   .../cti_sys0/channels# echo 2 > chan_clear
   .../cti_sys0/channels# cat ../regs/trigoutstatus
   0x0
   .../cti_sys0/channels# cat ../regs/choutstatus
   0x0

```