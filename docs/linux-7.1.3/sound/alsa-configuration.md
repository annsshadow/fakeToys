## ALSA锛圓dvanced Linux Sound Architecture锛夐┍鍔ㄩ厤缃寚鍗?



## 鍐呮牳閰嶇疆


瑕佸惎鐢?ALSA 鏀寔锛屼綘鑷冲皯闇€瑕佸湪鍐呮牳涓瀯寤轰富澹板崱鏀寔锛坄CONFIG_SOUND`锛夈€傜敱浜?ALSA 鍙互妯℃嫙 OSS锛屽洜姝や綘鏃犻渶閫夋嫨浠讳綍 OSS 妯″潡銆?

濡傛灉浣犲笇鏈涚敤 ALSA 杩愯 OSS 搴旂敤绋嬪簭锛岃鍚敤鈥淥SS API 妯℃嫙鈥濓紙`CONFIG_SND_OSSEMUL`锛変互鍙?OSS 娣烽煶鍣ㄥ拰 PCM 鏀寔銆?

濡傛灉浣犳兂鏀寔 SB Live! 绛夊０鍗′笂鐨勬尝琛紙WaveTable锛夊姛鑳斤紝鍒欓渶瑕佸惎鐢ㄢ€滈煶搴忓櫒鏀寔鈥濓紙`CONFIG_SND_SEQUENCER`锛夈€?

鑻ヨ璁?ALSA 鐨勮皟璇曚俊鎭洿璇︾粏锛岃鍚敤鈥淰erbose printk鈥濆拰鈥淒ebug鈥濋€夐」銆傝嫢瑕佹鏌ュ唴瀛樻硠婕忥紝杩樿鎵撳紑鈥淒ebug memory鈥濄€傗€淒ebug detection鈥濅細娣诲姞鐢ㄤ簬澹板崱妫€娴嬬殑妫€鏌ャ€?

璇锋敞鎰忥紝鎵€鏈?ALSA ISA 椹卞姩閮芥敮鎸?Linux 鐨?isapnp API锛堝墠鎻愭槸澹板崱鏀寔 ISA PnP锛夈€備綘鏃犻渶浣跨敤 isapnptools 鏉ラ厤缃０鍗°€?


## 妯″潡鍙傛暟


鐢ㄦ埛鍙互甯﹂€夐」鍔犺浇妯″潡銆傚鏋滄煇涓ā鍧楁敮鎸佸鍧楀０鍗★紝鑰屼綘鍙堟湁澶氬潡鍚岀被鍨嬪０鍗★紝鍒欏彲浠ョ敤閫楀彿鍒嗛殧涓洪€夐」鎸囧畾澶氫釜鍊笺€?


### 妯″潡 snd


ALSA 鏍稿績妯″潡銆傚畠琚墍鏈?ALSA 澹板崱椹卞姩鎵€浣跨敤銆?
瀹冩帴鍙椾互涓嬪叿鏈夊叏灞€褰卞搷鐨勯€夐」銆?

major
    澹板崱椹卞姩鐨?major 鍙凤紱
    榛樿鍊硷細116
cards_limit
    闄愬埗鑷姩鍔犺浇鐨勫０鍗＄储寮曪紙1-8锛夛紱
    榛樿鍊硷細1锛?
    鑻ヨ鑷姩鍔犺浇澶氬潡澹板崱锛岃灏嗘閫夐」涓?snd-card-X 鍒悕涓€璧锋寚瀹氥€?
slots
    涓虹粰瀹氶┍鍔ㄤ繚鐣欐Ы浣嶇储寮曪紱
    姝ら€夐」鎺ュ彈澶氫釜瀛楃涓层€?
    璇﹁ `Module Autoloading Support`_ 灏忚妭銆?
debug
    鎸囧畾璋冭瘯淇℃伅绾у埆锛?
    锛? = 绂佺敤璋冭瘯鎵撳嵃锛? = 鏅€氳皟璇曚俊鎭紝
    2 = 璇︾粏璋冭瘯淇℃伅锛夛紱
    姝ら€夐」浠呭湪 `CONFIG_SND_DEBUG=y` 鏃舵墠浼氬嚭鐜般€?
    姝ら€夐」鍙€氳繃 sysfs 涓殑
    /sys/module/snd/parameters/debug 鏂囦欢鍔ㄦ€佷慨鏀广€?

### 妯″潡 snd-pcm-oss


PCM OSS 妯℃嫙妯″潡銆?
璇ユā鍧楁帴鍙楃敤浜庢敼鍙樿澶囨槧灏勭殑閫夐」銆?

dsp_map
    鍒嗛厤缁欑 1 涓?OSS 璁惧鐨?PCM 璁惧鍙凤紱
    榛樿鍊硷細0
adsp_map
    鍒嗛厤缁欑 2 涓?OSS 璁惧鐨?PCM 璁惧鍙凤紱
    榛樿鍊硷細1
nonblock_open
    鎵撳紑绻佸繖鐨?PCM 璁惧鏃朵笉闃诲锛?
    榛樿鍊硷細1

渚嬪锛屽綋 `dsp_map=2` 鏃讹紝/dev/dsp 灏嗚鏄犲皠鍒?
绗?0 鍙峰０鍗＄殑绗?2 涓?PCM銆傜被浼煎湴锛屽綋 `adsp_map=0` 鏃讹紝
/dev/adsp 灏嗚鏄犲皠鍒扮 0 鍙峰０鍗＄殑绗?0 涓?PCM銆?
鑻ヨ淇敼绗簩鍧楁垨鏇村悗闈㈢殑澹板崱锛屽彲鐢ㄩ€楀彿鎸囧畾閫夐」锛?
渚嬪 `dsp_map=0,1`銆?

`nonblock_open` 閫夐」鐢ㄤ簬鏀瑰彉 PCM 鍦ㄦ墦寮€璁惧鏃剁殑琛屼负銆?
褰撹閫夐」闈為浂鏃讹紝鎵撳紑涓€涓箒蹇欑殑 OSS PCM 璁惧涓嶄細琚樆濉烇紝
鑰屾槸绔嬪嵆浠?EAGAIN 杩斿洖锛堝氨鍍?O_NONBLOCK 鏍囧織涓€鏍凤級銆?

### 妯″潡 snd-rawmidi


璇ユā鍧楁帴鍙楃敤浜庢敼鍙樿澶囨槧灏勭殑閫夐」銆?
涓?snd-pcm-oss 妯″潡绫讳技銆?

midi_map
    鍒嗛厤缁欑 1 涓?OSS 璁惧鐨?MIDI 璁惧鍙凤紱
    榛樿鍊硷細0
amidi_map
    鍒嗛厤缁欑 2 涓?OSS 璁惧鐨?MIDI 璁惧鍙凤紱
    榛樿鍊硷細1

### 妯″潡 snd-soc-core


SoC 鏍稿績妯″潡銆傚畠琚墍鏈?ALSA 澹板崱椹卞姩鎵€浣跨敤銆?
瀹冩帴鍙椾互涓嬪叿鏈夊叏灞€褰卞搷鐨勯€夐」銆?

prealloc_buffer_size_kbytes
    浠?kbytes 涓哄崟浣嶆寚瀹氶鍒嗛厤缂撳啿鍖哄ぇ灏忥紙榛樿锛?12锛夈€?

### 椤跺眰澹板崱妯″潡鐨勯€氱敤鍙傛暟


姣忎釜椤跺眰澹板崱妯″潡閮芥帴鍙椾互涓嬮€夐」銆?

index
    澹板崱鐨勭储寮曪紙妲戒綅 #锛夛紱
    鍙栧€硷細0 鍒?31 鎴栬礋鏁帮紱
    鑻ラ潪璐燂紝鍒欏垎閰嶈绱㈠紩鍙凤紱
    鑻ヤ负璐燂紝鍒欒В閲婁负鍏佽绱㈠紩鐨勪綅鎺╃爜锛?
    鍒嗛厤绗竴涓┖闂茬殑鍏佽绱㈠紩锛?
    榛樿鍊硷細-1
id
    澹板崱 ID锛堟爣璇嗙鎴栧悕绉帮級锛?
    鏈€闀?15 涓瓧绗︼紱
    榛樿鍊硷細澹板崱绫诲瀷锛?
    鍦?/proc/asound/ 涓嬩細鍒涘缓浠ユ鍚嶇О鍛藉悕鐨勭洰褰曪紝
    鍏朵腑鍖呭惈璇ュ０鍗＄殑鐩稿叧淇℃伅锛?
    鍦ㄨ瘑鍒０鍗℃椂鍙互鐢ㄦ ID 浠ｆ浛绱㈠紩鍙?
enable
    鍚敤澹板崱锛?
    榛樿鍊硷細瀵逛簬 PCI 鍜?ISA PnP 澹板崱涓哄惎鐢?

杩欎簺閫夐」鐢ㄤ簬鎸囧畾瀹炰緥鐨勯『搴忥紝鎴栧湪澶氫釜璁惧缁戝畾鍒板悓涓€椹卞姩鏃?
鎺у埗姣忎釜璁惧鐨勫惎鐢ㄤ笌绂佺敤銆備緥濡傦紝璁稿鏈哄櫒鏈変袱鍧?HD-audio
鎺у埗鍣紙涓€鍧楃敤浜?HDMI/DP 闊抽锛屽彟涓€鍧楃敤浜庢澘杞芥ā鎷熼煶棰戯級銆?
鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝绗簩鍧楁槸涓昏鐢ㄩ€旓紝鐢ㄦ埛甯屾湜灏嗗叾鍒嗛厤涓?
鏈€鍏堝嚭鐜扮殑澹板崱銆傚彲浠ラ€氳繃鎸囧畾 "index=1,0" 妯″潡鍙傛暟鏉ュ疄鐜帮紝
杩欎細浜ゆ崲鍒嗛厤妲戒綅銆?

濡備粖锛屽湪甯︽湁 PulseAudio 鍜?PipeWire 绛夋敮鎸佸姩鎬侀厤缃殑澹伴煶
鍚庣鐨勬儏鍐典笅锛岃繖绉嶇敤娉曞凡娌′粈涔堜环鍊硷紝浣嗗湪杩囧幓瀹冨闈欐€侀厤缃?
寰堟湁甯姪銆?

### 妯″潡 snd-adlib


鐢ㄤ簬 AdLib FM 澹板崱鐨勬ā鍧椼€?

port
    OPL 鑺墖鐨勭鍙ｅ彿 #

璇ユā鍧楁敮鎸佸鍧楀０鍗°€傚畠涓嶆敮鎸佽嚜鍔ㄦ帰娴嬶紝鍥犳蹇呴』鎸囧畾绔彛銆?
瀵逛簬瀹為檯鐨?AdLib FM 澹板崱锛岀鍙ｄ负 0x388銆?
娉ㄦ剰璇ュ０鍗℃病鏈?PCM 鏀寔鍜屾贩闊冲櫒锛屼粎鏈?FM 鍚堟垚銆?

璇风‘淇濅綘宸插噯澶囧ソ alsa-tools 杞欢鍖呬腑鐨?`sbiload`锛屽苟鍦?
鍔犺浇妯″潡鍚庨€氳繃 `sbiload -l` 鏌ユ槑鎵€鍒嗛厤鐨?ALSA 闊冲簭鍣ㄧ鍙ｅ彿銆?

绀轰緥杈撳嚭锛?
```

      Port     Client name                       Port name
      64:0     OPL2 FM synth                     OPL2 FM Port

```
鍔犺浇鍚屾牱鐢?`sbiload` 鎻愪緵鐨?`std.sb` 鍜?`drums.sb` 闊宠壊锛?
```

      sbiload -p 64:0 std.sb drums.sb

```
濡傛灉浣犱娇鐢ㄨ椹卞姩鏉ラ┍鍔?OPL3锛屽垯鍙互鏀圭敤 `std.o3` 鍜?`drums.o3`銆?
鑻ヨ璁╁０鍗″彂鍑哄０闊筹紝鍙娇鐢?alsa-utils 涓殑 `aplaymidi`锛?
```

      aplaymidi -p 64:0 foo.mid

```
### 妯″潡 snd-ad1816a


鐢ㄤ簬鍩轰簬 Analog Devices AD1816A/AD1815 ISA 鑺墖鐨勫０鍗＄殑妯″潡銆?

clockfreq
    AD1816A 鑺墖鐨勬椂閽熼鐜囷紙榛樿 = 0锛?3000Hz锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€佽嚜鍔ㄦ帰娴嬪拰 PnP銆?

### 妯″潡 snd-ad1848


鐢ㄤ簬鍩轰簬 AD1848/AD1847/CS4248 ISA 鑺墖鐨勫０鍗＄殑妯″潡銆?

port
    AD1848 鑺墖鐨勭鍙ｅ彿 #
irq
    AD1848 鑺墖鐨?IRQ #
dma1
    AD1848 鑺墖鐨?DMA #锛?,1,3锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€傚畠涓嶆敮鎸佽嚜鍔ㄦ帰娴嬶紝鍥犳蹇呴』鎸囧畾涓荤鍙ｏ紒锛侊紒
鍏朵粬绔彛涓哄彲閫夈€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-ad1889


鐢ㄤ簬 Analog Devices AD1889 鑺墖鐨勬ā鍧椼€?

ac97_quirk
    閽堝寮傚父纭欢鐨?AC'97 瑙勯伩鏂规锛?
    璇﹁ intel8x0 妯″潡鐨勬弿杩般€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-ali5451


鐢ㄤ簬 ALi M5451 PCI 鑺墖鐨勬ā鍧椼€?

pcm_channels
    涓?PCM 鍒嗛厤鐨勭‖浠堕€氶亾鏁?
spdif
    鏀寔 SPDIF I/O锛?
    榛樿鍊硷細绂佺敤

璇ユā鍧楁敮鎸佸崟鑺墖鍜岃嚜鍔ㄦ帰娴嬨€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-als100


鐢ㄤ簬鍩轰簬 Avance Logic ALS100/ALS120 ISA 鑺墖鐨勫０鍗＄殑妯″潡銆?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€佽嚜鍔ㄦ帰娴嬪拰 PnP銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-als300


鐢ㄤ簬 Avance Logic ALS300 鍜?ALS300+ 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-als4000


鐢ㄤ簬鍩轰簬 Avance Logic ALS4000 PCI 鑺墖鐨勫０鍗＄殑妯″潡銆?

joystick_port
    浼犵粺娓告垙鏉嗘敮鎸佺殑绔彛鍙?#锛?
    0 = 绂佺敤锛堥粯璁わ級锛? = 鑷姩鎺㈡祴

璇ユā鍧楁敮鎸佸鍧楀０鍗°€佽嚜鍔ㄦ帰娴嬪拰 PnP銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-asihpi


鐢ㄤ簬 AudioScience ASI 澹板崱鐨勬ā鍧椼€?

enable_hpi_hwdep
    涓?AudioScience 澹板崱鍚敤 HPI hwdep

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-atiixp


鐢ㄤ簬 ATI IXP 150/200/250/400 AC97 鎺у埗鍣ㄧ殑妯″潡銆?

ac97_clock
    AC'97 鏃堕挓锛堥粯璁?= 48000锛?
ac97_quirk
    閽堝寮傚父纭欢鐨?AC'97 瑙勯伩鏂规锛?
    璇﹁涓嬮潰鐨?`AC97 Quirk Option`_ 灏忚妭銆?
ac97_codec
    鐢ㄤ簬鎸囧畾浣跨敤鏌愪釜 AC'97 缂栬В鐮佸櫒鑰岄潪鎺㈡祴鐨勮閬挎柟妗堛€?
    濡傛灉杩欏浣犳湁鏁堬紝璇烽檮涓婁綘鐨?`lspci -vn` 杈撳嚭鎻愪氦涓€涓?bug銆?
    锛?2 = 寮哄埗鎺㈡祴锛?1 = 榛樿琛屼负锛?-2 = 浣跨敤鎸囧畾鐨勭紪瑙ｇ爜鍣ㄣ€傦級
spdif_aclink
    閫氳繃 AC-link 浼犺緭 S/PDIF锛堥粯璁?= 1锛?

璇ユā鍧楁敮鎸佸崟鍧楀０鍗″拰鑷姩鎺㈡祴銆?

ATI IXP 鏈変袱绉嶄笉鍚岀殑鏂规硶鏉ユ帶鍒?SPDIF 杈撳嚭銆備竴绉嶆槸閫氳繃
AC-link锛屽彟涓€绉嶆槸閫氳繃鈥渄irect鈥?SPDIF 杈撳嚭銆傚叿浣撳疄鐜板彇鍐充簬
涓绘澘锛屼綘闇€瑕侀€氳繃 spdif_aclink 妯″潡閫夐」閫夋嫨姝ｇ‘鐨勬柟寮忋€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-atiixp-modem


鐢ㄤ簬 ATI IXP 150/200/250 AC97 璋冨埗鍣ㄦ帶鍒跺櫒鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸崟鍧楀０鍗″拰鑷姩鎺㈡祴銆?

娉ㄦ剰锛氳妯″潡鐨勯粯璁?index 鍊间负 -2锛屽嵆绗竴涓Ы浣嶈鎺掗櫎銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-au8810銆乻nd-au8820銆乻nd-au8830


鐢ㄤ簬 Aureal Vortex銆乂ortex2 鍜?Advantage 璁惧鐨勬ā鍧椼€?

pcifix
    鎺у埗 PCI 瑙勯伩鏂规锛?
    0 = 绂佺敤鎵€鏈夎閬挎柟妗堬紝
    1 = 灏?Aureal 澹板崱鐨?PCI 寤惰繜寮哄埗璁句负 0xff锛?
    2 = 寮哄埗 Extend PCI#2 Internal Master锛屼互鍦?VIA KT133 AGP
    妗ヤ笂楂樻晥澶勭悊 Dummy Requests锛?
    3 = 寮哄埗涓婅堪涓ょ璁剧疆锛?
    255 = 鑷姩鎺㈡祴鎵€闇€璁剧疆锛堥粯璁わ級

璇ユā鍧楁敮鎸佹墍鏈?ADB PCM 閫氶亾銆乤c97 娣烽煶鍣ㄣ€丼PDIF銆佺‖浠?EQ銆?
mpu401銆乬ameport銆侫3D 鍜屾尝琛ㄦ敮鎸佷粛鍦ㄥ紑鍙戜腑銆?
寮€鍙戝拰閫嗗悜宸ョ▼宸ヤ綔姝ｅ湪
https://savannah.nongnu.org/projects/openvortex/ 鍗忚皟杩涜銆?
SPDIF 杈撳嚭鏄?AC97 缂栬В鐮佸櫒杈撳嚭鐨勫壇鏈紝闄ら潪浣犱娇鐢?
`spdif` pcm 璁惧锛屽畠鍏佽鍘熷鏁版嵁閫忎紶銆?
纭欢 EQ 鍜?SPDIF 浠呭瓨鍦ㄤ簬 Vortex2 鍜?Advantage 涓€?

娉ㄦ剰锛氭煇浜?ALSA 娣烽煶鍣ㄥ簲鐢ㄧ▼搴忎笉鑳芥纭鐞?SPDIF 閲囨牱鐜囨帶鍒躲€?
濡傛灉浣犲湪杩欐柟闈㈤亣鍒伴棶棰橈紝鍙互灏濊瘯鍙︿竴涓吋瀹?ALSA 鐨勬贩闊冲櫒
锛坅lsamixer 鍙敤锛夈€?

### 妯″潡 snd-azt1605


鐢ㄤ簬鍩轰簬 Aztech AZT1605 鑺墖缁勭殑 Aztech Sound Galaxy 澹板崱鐨勬ā鍧椼€?

port
    BASE 鐨勭鍙ｅ彿 #锛?x220,0x240,0x260,0x280锛?
wss_port
    WSS 鐨勭鍙ｅ彿 #锛?x530,0x604,0xe80,0xf40锛?
irq
    WSS 鐨?IRQ #锛?,9,10,11锛?
dma1
    WSS 鎾斁鐨?DMA #锛?,1,3锛?
dma2
    WSS 閲囬泦鐨?DMA #锛?,1锛夛紝-1 = 绂佺敤锛堥粯璁わ級
mpu_port
    MPU-401 UART 鐨勭鍙ｅ彿 #锛?x300,0x330锛夛紝-1 = 绂佺敤锛堥粯璁わ級
mpu_irq
    MPU-401 UART 鐨?IRQ #锛?,5,7,9锛夛紝-1 = 绂佺敤锛堥粯璁わ級
fm_port
    OPL3 鐨勭鍙ｅ彿 #锛?x388锛夛紝-1 = 绂佺敤锛堥粯璁わ級

璇ユā鍧楁敮鎸佸鍧楀０鍗°€傚畠涓嶆敮鎸佽嚜鍔ㄦ帰娴嬶細`port`銆乣wss_port`銆?
`irq` 鍜?`dma1` 蹇呴』鎸囧畾銆傚叾浠栧€间负鍙€夈€?

`port` 闇€瑕佸尮閰嶅０鍗′笂 BASE ADDRESS 璺崇嚎锛?x220 鎴?0x240锛?
鎴栧０鍗?EEPROM 涓瓨鍌ㄧ殑鍊硷紙閫傜敤浜庡甫 EEPROM 涓斿皢鈥淐ONFIG MODE鈥?
璺崇嚎璁句负鈥淓EPROM SETTING鈥濈殑澹板崱锛夈€傚叾浠栧€煎彲浠ヤ粠涓婇潰鍒椾妇鐨?
閫夐」涓嚜鐢遍€夋嫨銆?

濡傛灉 `dma2` 琚寚瀹氫笖涓?`dma1` 涓嶅悓锛屽０鍗″皢浠ュ叏鍙屽伐妯″紡宸ヤ綔銆?
褰?`dma1=3` 鏃讹紝鍙湁 `dma2=0` 鏈夋晥锛屽苟涓旂敱浜庡彧鏈夐€氶亾 0 鍜?1
鍙敤浜庨噰闆嗭紝杩欎篃鏄惎鐢ㄩ噰闆嗙殑鍞竴鏂瑰紡銆?

閫氱敤璁剧疆涓?``port=0x220 wss_port=0x530 irq=10 dma1=1 dma2=0
mpu_port=0x330 mpu_irq=9 fm_port=0x388``銆?

鏃犺浣犻€夋嫨鍝釜 IRQ 鍜?DMA 閫氶亾锛岃鍔″繀鍦?BIOS 涓负浼犵粺 ISA
淇濈暀瀹冧滑銆?

### 妯″潡 snd-azt2316


鐢ㄤ簬鍩轰簬 Aztech AZT2316 鑺墖缁勭殑 Aztech Sound Galaxy 澹板崱鐨勬ā鍧椼€?

port
    BASE 鐨勭鍙ｅ彿 #锛?x220,0x240,0x260,0x280锛?
wss_port
    WSS 鐨勭鍙ｅ彿 #锛?x530,0x604,0xe80,0xf40锛?
irq
    WSS 鐨?IRQ #锛?,9,10,11锛?
dma1
    WSS 鎾斁鐨?DMA #锛?,1,3锛?
dma2
    WSS 閲囬泦鐨?DMA #锛?,1锛夛紝-1 = 绂佺敤锛堥粯璁わ級
mpu_port
    MPU-401 UART 鐨勭鍙ｅ彿 #锛?x300,0x330锛夛紝-1 = 绂佺敤锛堥粯璁わ級
mpu_irq
    MPU-401 UART 鐨?IRQ #锛?,7,9,10锛夛紝-1 = 绂佺敤锛堥粯璁わ級
fm_port
    OPL3 鐨勭鍙ｅ彿 #锛?x388锛夛紝-1 = 绂佺敤锛堥粯璁わ級

璇ユā鍧楁敮鎸佸鍧楀０鍗°€傚畠涓嶆敮鎸佽嚜鍔ㄦ帰娴嬶細`port`銆乣wss_port`銆?
`irq` 鍜?`dma1` 蹇呴』鎸囧畾銆傚叾浠栧€间负鍙€夈€?

`port` 闇€瑕佸尮閰嶅０鍗′笂 BASE ADDRESS 璺崇嚎锛?x220 鎴?0x240锛?
鎴栧０鍗?EEPROM 涓瓨鍌ㄧ殑鍊硷紙閫傜敤浜庡甫 EEPROM 涓斿皢鈥淐ONFIG MODE鈥?
璺崇嚎璁句负鈥淓EPROM SETTING鈥濈殑澹板崱锛夈€傚叾浠栧€煎彲浠ヤ粠涓婇潰鍒椾妇鐨?
閫夐」涓嚜鐢遍€夋嫨銆?

濡傛灉 `dma2` 琚寚瀹氫笖涓?`dma1` 涓嶅悓锛屽０鍗″皢浠ュ叏鍙屽伐妯″紡宸ヤ綔銆?
褰?`dma1=3` 鏃讹紝鍙湁 `dma2=0` 鏈夋晥锛屽苟涓旂敱浜庡彧鏈夐€氶亾 0 鍜?1
鍙敤浜庨噰闆嗭紝杩欎篃鏄惎鐢ㄩ噰闆嗙殑鍞竴鏂瑰紡銆?

閫氱敤璁剧疆涓?``port=0x220 wss_port=0x530 irq=10 dma1=1 dma2=0
mpu_port=0x330 mpu_irq=9 fm_port=0x388``銆?

鏃犺浣犻€夋嫨鍝釜 IRQ 鍜?DMA 閫氶亾锛岃鍔″繀鍦?BIOS 涓负浼犵粺 ISA
淇濈暀瀹冧滑銆?

### 妯″潡 snd-aw2


鐢ㄤ簬 Audiowerk2 澹板崱鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-azt2320


鐢ㄤ簬鍩轰簬 Aztech System AZT2320 ISA 鑺墖锛堜粎 PnP锛夌殑澹板崱鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€丳nP 鍜岃嚜鍔ㄦ帰娴嬨€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-azt3328


鐢ㄤ簬鍩轰簬 Aztech AZF3328 PCI 鑺墖鐨勫０鍗＄殑妯″潡銆?

joystick
    鍚敤娓告垙鏉嗭紙榛樿鍏抽棴锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-bt87x


鐢ㄤ簬鍩轰簬 Bt87x 鑺墖鐨勮棰戝崱鐨勬ā鍧椼€?

digital_rate
    瑕嗙洊榛樿鐨勬暟瀛楅€熺巼锛圚z锛?
load_all
    鍗充娇涓嶇煡閬撳０鍗″瀷鍙蜂篃鍔犺浇椹卞姩

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

娉ㄦ剰锛氳妯″潡鐨勯粯璁?index 鍊间负 -2锛屽嵆绗竴涓Ы浣嶈鎺掗櫎銆?

### 妯″潡 snd-ca0106


鐢ㄤ簬 Creative Audigy LS 鍜?SB Live 24bit 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?


### 妯″潡 snd-cmi8330


鐢ㄤ簬鍩轰簬 C-Media CMI8330 ISA 鑺墖鐨勫０鍗＄殑妯″潡銆?

isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

鍦?`isapnp=0` 鏃讹紝鍙娇鐢ㄤ互涓嬮€夐」锛?

wssport
    CMI8330 鑺墖锛圵SS锛夌殑绔彛鍙?#
wssirq
    CMI8330 鑺墖锛圵SS锛夌殑 IRQ #
wssdma
    CMI8330 鑺墖锛圵SS锛夌殑绗竴涓?DMA #
sbport
    CMI8330 鑺墖锛圫B16锛夌殑绔彛鍙?#
sbirq
    CMI8330 鑺墖锛圫B16锛夌殑 IRQ #
sbdma8
    CMI8330 鑺墖锛圫B16锛夌殑 8 浣?DMA #
sbdma16
    CMI8330 鑺墖锛圫B16锛夌殑 16 浣?DMA #
fmport
    锛堝彲閫夛級OPL3 I/O 绔彛
mpuport
    锛堝彲閫夛級MPU401 I/O 绔彛
mpuirq
    锛堝彲閫夛級MPU401 irq #

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-cmipci


鐢ㄤ簬 C-Media CMI8338/8738/8768/8770 PCI 澹板崱鐨勬ā鍧椼€?

mpu_port
    MIDI 鎺ュ彛鐨勭鍙ｅ湴鍧€锛堜粎 8338锛夛細
    0x300,0x310,0x320,0x330 = 浼犵粺绔彛锛?
    1 = 闆嗘垚 PCI 绔彛锛?738 涓婄殑榛樿锛夛紝
    0 = 绂佺敤
fm_port
    OPL-3 FM 鍚堟垚鍣ㄧ殑绔彛鍦板潃锛堜粎 8x38锛夛細
    0x388 = 浼犵粺绔彛锛?
    1 = 闆嗘垚 PCI 绔彛锛?738 涓婄殑榛樿锛夛紝
    0 = 绂佺敤
soft_ac3
    杞欢杞崲鍘熷 SPDIF 鏁版嵁鍖咃紙浠?model 033锛夛紙榛樿 = 1锛?
joystick_port
    娓告垙鏉嗙鍙ｅ湴鍧€锛? = 绂佺敤锛? = 鑷姩鎺㈡祴锛?

璇ユā鍧楁敮鎸佽嚜鍔ㄦ帰娴嬪拰澶氬潡澹板崱銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-cs4231


鐢ㄤ簬鍩轰簬 CS4231 ISA 鑺墖鐨勫０鍗＄殑妯″潡銆?

port
    CS4231 鑺墖鐨勭鍙ｅ彿 #
mpu_port
    MPU-401 UART 鐨勭鍙ｅ彿 #锛堝彲閫夛級锛?1 = 绂佺敤
irq
    CS4231 鑺墖鐨?IRQ #
mpu_irq
    MPU-401 UART 鐨?IRQ #
dma1
    CS4231 鑺墖鐨勭涓€涓?DMA #
dma2
    CS4231 鑺墖鐨勭浜屼釜 DMA #

璇ユā鍧楁敮鎸佸鍧楀０鍗°€傝妯″潡涓嶆敮鎸佽嚜鍔ㄦ帰娴嬶紝鍥犳蹇呴』鎸囧畾涓荤鍙ｏ紒锛侊紒
鍏朵粬绔彛涓哄彲閫夈€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-cs4236


鐢ㄤ簬鍩轰簬 CS4232/CS4232A銆丆S4235/CS4236/CS4236B/CS4237B/CS4238B/
CS4239 ISA 鑺墖鐨勫０鍗＄殑妯″潡銆?

isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

鍦?`isapnp=0` 鏃讹紝鍙娇鐢ㄤ互涓嬮€夐」锛?

port
    CS4236 鑺墖鐨勭鍙ｅ彿 #锛圥nP 璁剧疆 - 0x534锛?
cport
    CS4236 鑺墖鐨勬帶鍒剁鍙ｅ彿 #锛圥nP 璁剧疆 - 0x120,0x210,0xf00锛?
mpu_port
    MPU-401 UART 鐨勭鍙ｅ彿 #锛圥nP 璁剧疆 - 0x300锛夛紝-1 = 绂佺敤
fm_port
    CS4236 鑺墖鐨?FM 绔彛鍙?#锛圥nP 璁剧疆 - 0x388锛夛紝-1 = 绂佺敤
irq
    CS4236 鑺墖鐨?IRQ #锛?,7,9,11,12,15锛?
mpu_irq
    MPU-401 UART 鐨?IRQ #锛?,11,12,15锛?
dma1
    CS4236 鑺墖鐨勭涓€涓?DMA #锛?,1,3锛?
dma2
    CS4236 鑺墖鐨勭浜屼釜 DMA #锛?,1,3锛夛紝-1 = 绂佺敤

璇ユā鍧楁敮鎸佸鍧楀０鍗°€傝妯″潡涓嶆敮鎸佽嚜鍔ㄦ帰娴嬶紙鑻ユ湭浣跨敤 ISA PnP锛夛紝
鍥犳蹇呴』鎸囧畾涓荤鍙ｅ拰鎺у埗绔彛锛侊紒锛佸叾浠栫鍙ｄ负鍙€夈€?

鏀寔鐢垫簮绠＄悊銆?

姝ゆā鍧椾篃琚埆鍚嶄负 snd-cs4232锛屽洜涓哄畠鍚屾椂鎻愪緵浜嗘棫鐨?
snd-cs4232 鍔熻兘銆?

### 妯″潡 snd-cs4281


鐢ㄤ簬 Cirrus Logic CS4281 澹拌姱鐗囩殑妯″潡銆?

dual_codec
    绗簩缂栬В鐮佸櫒 ID锛? = 绂佺敤锛岄粯璁わ級

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-cs46xx


鐢ㄤ簬鍩轰簬 CS4610/CS4612/CS4614/CS4615/CS4622/CS4624/CS4630/
CS4280 PCI 鑺墖鐨?PCI 澹板崱鐨勬ā鍧椼€?

external_amp
    寮哄埗鍚敤澶栭儴鏀惧ぇ鍣ㄣ€?
thinkpad
    寮哄埗鍚敤 Thinkpad 鐨?CLKRUN 鎺у埗銆?
mmap_valid
    鏀寔 OSS mmap 妯″紡锛堥粯璁?= 0锛夈€?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?
閫氬父澶栭儴鏀惧ぇ鍣ㄥ拰 CLKRUN 鎺у埗浼氭牴鎹?PCI 瀛愮郴缁熷巶鍟?璁惧 ID 鑷姩
鎺㈡祴銆傚鏋滃畠浠笉宸ヤ綔锛岃鏄惧紡缁欏嚭涓婅堪閫夐」銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-cs5530


鐢ㄤ簬 Cyrix/NatSemi Geode 5530 鑺墖鐨勬ā鍧椼€?

### 妯″潡 snd-cs5535audio


鐢ㄤ簬澶氬姛鑳?CS5535 閰嶅 PCI 璁惧鐨勬ā鍧椼€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-ctxfi


鐢ㄤ簬 Creative Sound Blaster X-Fi 鏉垮崱锛?0k1 / 20k2 鑺墖锛夌殑妯″潡銆?

- Creative Sound Blaster X-Fi Titanium Fatal1ty Champion Series
- Creative Sound Blaster X-Fi Titanium Fatal1ty Professional Series
- Creative Sound Blaster X-Fi Titanium Professional Audio
- Creative Sound Blaster X-Fi Titanium
- Creative Sound Blaster X-Fi Elite Pro
- Creative Sound Blaster X-Fi Platinum
- Creative Sound Blaster X-Fi Fatal1ty
- Creative Sound Blaster X-Fi XtremeGamer
- Creative Sound Blaster X-Fi XtremeMusic


reference_rate
    鍙傝€冮噰鏍风巼锛?4100 鎴?48000锛堥粯璁わ級
multiple
    鍙傝€冮噰鏍风巼鐨勫€嶆暟锛? 鎴?2锛堥粯璁わ級
subsystem
    瑕嗙洊鐢ㄤ簬鎺㈡祴鐨?PCI SSID锛?
    璇ュ€肩敱 SSVID << 16 | SSDID 缁勬垚銆?
    榛樿鍊间负闆讹紝琛ㄧず涓嶈鐩栥€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?


### 妯″潡 snd-darla20


鐢ㄤ簬 Echoaudio Darla20 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-darla24


鐢ㄤ簬 Echoaudio Darla24 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-dt019x


鐢ㄤ簬 Diamond Technologies DT-019X / Avance Logic ALS-007锛堜粎 PnP锛夌殑妯″潡銆?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€傝妯″潡浠呭湪鍚敤 ISA PnP 鏀寔鏃舵墠鍙敤銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-dummy


鐢ㄤ簬铏氭嫙澹板崱鐨勬ā鍧椼€傝繖涓€滃０鍗♀€濅笉杩涜浠讳綍杈撳嚭鎴栬緭鍏ワ紝浣嗕綘鍙互
灏嗗畠鐢ㄤ簬浠讳綍闇€瑕佸０鍗＄殑搴旂敤绋嬪簭锛堝 RealPlayer锛夈€?

pcm_devs
    鍒嗛厤缁欐瘡鍧楀０鍗＄殑 PCM 璁惧鏁帮紙榛樿 = 1锛屾渶澶?4锛?
pcm_substreams
    鍒嗛厤缁欐瘡涓?PCM 鐨?PCM 瀛愭祦鏁帮紙榛樿 = 8锛屾渶澶?128锛?
hrtimer
    浣跨敤 hrtimer锛?1锛岄粯璁わ級鎴栫郴缁熷畾鏃跺櫒锛?0锛?
fake_buffer
    铏氬亣缂撳啿鍖哄垎閰嶏紙榛樿 = 1锛?

褰撳垱寤哄涓?PCM 璁惧鏃讹紝snd-dummy 瀵规瘡涓?PCM 璁惧缁欏嚭涓嶅悓鐨勮涓猴細
- 0 = 甯?mmap 鏀寔鐨勪氦閿欐ā寮?
- 1 = 甯?mmap 鏀寔鐨勯潪浜ら敊妯″紡
- 2 = 涓嶅甫 mmap 鐨勪氦閿欐ā寮?
- 3 = 涓嶅甫 mmap 鐨勯潪浜ら敊妯″紡

榛樿鎯呭喌涓嬶紝snd-dummy 椹卞姩涓嶅垎閰嶇湡瀹炵殑缂撳啿鍖猴紝鑰屾槸蹇界暐
璇?鍐欐垨灏嗗崟涓櫄鎷熼〉 mmap 鍒版墍鏈夌紦鍐插尯椤碉紝浠ヨ妭鐪佽祫婧愩€?
濡傛灉浣犵殑搴旂敤绋嬪簭闇€瑕佽/鍐欑紦鍐插尯鏁版嵁淇濇寔涓€鑷达紝璇蜂紶鍏?
fake_buffer=0 閫夐」銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-echo3g


鐢ㄤ簬 Echoaudio 3G 澹板崱锛圙ina3G/Layla3G锛夌殑妯″潡銆?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-emu10k1


鐢ㄤ簬鍩轰簬 EMU10K1/EMU10k2 鐨?PCI 澹板崱鐨勬ā鍧椼€?

- Sound Blaster Live!
- Sound Blaster PCI 512
- Sound Blaster Audigy
- E-MU APS锛堥儴鍒嗘敮鎸侊級
- E-MU DAS

extin
    鐢ㄤ簬 FX8010 鐨勫彲鐢ㄥ閮ㄨ緭鍏ヤ綅鍥撅紙瑙佷笅锛?
extout
    鐢ㄤ簬 FX8010 鐨勫彲鐢ㄥ閮ㄨ緭鍑轰綅鍥撅紙瑙佷笅锛?
seq_ports
    鍒嗛厤鐨勯煶搴忓櫒绔彛锛堥粯璁?4锛?
max_synth_voices
    鐢ㄤ簬娉㈣〃鐨勮闊虫暟涓婇檺锛堥粯璁?64锛?
max_buffer_size
    浠?MB 涓哄崟浣嶆寚瀹氭尝琛?PCM 缂撳啿鍖虹殑鏈€澶уぇ灏忋€?
    榛樿鍊间负 128銆?
enable_ir
    鍚敤 IR

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

杈撳叆涓庤緭鍑洪厤缃?		[extin/extout]
- Creative Card wo/Digital out			[0x0003/0x1f03]
- Creative Card w/Digital out			[0x0003/0x1f0f]
- Creative Card w/Digital CD in			[0x000f/0x1f0f]
- Creative Card wo/Digital out + LiveDrive	[0x3fc3/0x1fc3]
- Creative Card w/Digital out + LiveDrive	[0x3fc3/0x1fcf]
- Creative Card w/Digital CD in + LiveDrive	[0x3fcf/0x1fcf]
- Creative Card wo/Digital out + Digital I/O 2  [0x0fc3/0x1f0f]
- Creative Card w/Digital out + Digital I/O 2	[0x0fc3/0x1f0f]
- Creative Card w/Digital CD in + Digital I/O 2	[0x0fcf/0x1f0f]
- Creative Card 5.1/w Digital out + LiveDrive	[0x3fc3/0x1fff]
- Creative Card 5.1 (c) 2003			[0x3fc3/0x7cff]
- Creative Card all ins and outs		[0x3fff/0x7fff]

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-emu10k1x


鐢ㄤ簬 Creative Emu10k1X锛圫B Live Dell OEM 鐗堟湰锛夌殑妯″潡銆?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-ens1370


鐢ㄤ簬 Ensoniq AudioPCI ES1370 PCI 澹板崱鐨勬ā鍧椼€?

- SoundBlaster PCI 64
- SoundBlaster PCI 128

joystick
    鍚敤娓告垙鏉嗭紙榛樿鍏抽棴锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-ens1371


鐢ㄤ簬 Ensoniq AudioPCI ES1371 PCI 澹板崱鐨勬ā鍧椼€?

- SoundBlaster PCI 64
- SoundBlaster PCI 128
- SoundBlaster Vibra PCI

joystick_port
    娓告垙鏉嗙殑绔彛鍙?#锛?x200,0x208,0x210,0x218锛夛紝0 = 绂佺敤
    锛堥粯璁わ級锛? = 鑷姩鎺㈡祴

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-es1688


鐢ㄤ簬 ESS AudioDrive ES-1688 鍜?ES-688 澹板崱鐨勬ā鍧椼€?

isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級
mpu_port
    MPU-401 绔彛鐨勭鍙ｅ彿 #锛?x300,0x310,0x320,0x330锛夛紝-1 = 绂佺敤锛堥粯璁わ級
mpu_irq
    MPU-401 绔彛鐨?IRQ #锛?,7,9,10锛?
fm_port
    OPL3 鐨勭鍙ｅ彿 #锛堝彲閫夛紱榛樿涓?MPU 绔彛鍏辩敤锛?

鍦?`isapnp=0` 鏃讹紝鍙娇鐢ㄤ互涓嬮檮鍔犻€夐」锛?

port
    ES-1688 鑺墖鐨勭鍙ｅ彿 #锛?x220,0x240,0x260锛?
irq
    ES-1688 鑺墖鐨?IRQ #锛?,7,9,10锛?
dma8
    ES-1688 鑺墖鐨?DMA #锛?,1,3锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴锛堜笉鍚?MPU-401 绔彛锛?
浠ュ強甯?ES968 鑺墖鐨?PnP銆?

### 妯″潡 snd-es18xx


鐢ㄤ簬 ESS AudioDrive ES-18xx 澹板崱鐨勬ā鍧椼€?

isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

鍦?`isapnp=0` 鏃讹紝鍙娇鐢ㄤ互涓嬮€夐」锛?

port
    ES-18xx 鑺墖鐨勭鍙ｅ彿 #锛?x220,0x240,0x260锛?
mpu_port
    MPU-401 绔彛鐨勭鍙ｅ彿 #锛?x300,0x310,0x320,0x330锛夛紝-1 = 绂佺敤锛堥粯璁わ級
fm_port
    FM 鐨勭鍙ｅ彿 #锛堝彲閫夛紝鏈娇鐢級
irq
    ES-18xx 鑺墖鐨?IRQ #锛?,7,9,10锛?
dma1
    ES-18xx 鑺墖鐨勭涓€涓?DMA #锛?,1,3锛?
dma2
    ES-18xx 鑺墖鐨勭涓€涓?DMA #锛?,1,3锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€両SA PnP 鍜岃嚜鍔ㄦ帰娴嬶紙鑻ユ湭浣跨敤鍘熺敓 ISA PnP
渚嬬▼鍒欎笉鍚?MPU-401 绔彛锛夈€傚綋 `dma2` 涓?`dma1` 鐩哥瓑鏃讹紝椹卞姩浠?
鍗婂弻宸ユ柟寮忓伐浣溿€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-es1938


鐢ㄤ簬鍩轰簬 ESS Solo-1锛圗S1938,ES1946锛夎姱鐗囩殑澹板崱鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-es1968


鐢ㄤ簬鍩轰簬 ESS Maestro-1/2/2E锛圗S1968/ES1978锛夎姱鐗囩殑澹板崱鐨勬ā鍧椼€?

total_bufsize
    浠?kB 涓哄崟浣嶇殑鎬荤紦鍐插尯澶у皬锛?-4096kB锛?
pcm_substreams_p
    鎾斁閫氶亾鏁帮紙1-8锛岄粯璁?2锛?
pcm_substreams_c
    閲囬泦閫氶亾鏁帮紙1-8锛岄粯璁?0锛?
clock
    鏃堕挓锛? = 鑷姩鎺㈡祴锛?
use_pm
    鏀寔鐢垫簮绠＄悊锛? = 鍏抽棴锛? = 寮€鍚紝2 = 鑷姩锛堥粯璁わ級锛?
enable_mpu
    鍚敤 MPU401锛? = 鍏抽棴锛? = 寮€鍚紝2 = 鑷姩锛堥粯璁わ級锛?
joystick
    鍚敤娓告垙鏉嗭紙榛樿鍏抽棴锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-fm801


鐢ㄤ簬鍩轰簬 ForteMedia FM801 鐨?PCI 澹板崱鐨勬ā鍧椼€?

tea575x_tuner
    鍚敤 TEA575x 璋冭皭鍣紱
    1 = MediaForte 256-PCS锛?
    2 = MediaForte 256-PCPR锛?
    3 = MediaForte 64-PCR
    楂?16 浣嶄负瑙嗛锛堟敹闊虫満锛夎澶囧彿 + 1锛?
    渚嬪锛?x10002锛圡ediaForte 256-PCPR锛岃澶?1锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-gina20


鐢ㄤ簬 Echoaudio Gina20 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-gina24


鐢ㄤ簬 Echoaudio Gina24 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-gusclassic


鐢ㄤ簬 Gravis UltraSound Classic 澹板崱鐨勬ā鍧椼€?

port
    GF1 鑺墖鐨勭鍙ｅ彿 #锛?x220,0x230,0x240,0x250,0x260锛?
irq
    GF1 鑺墖鐨?IRQ #锛?,5,9,11,12,15锛?
dma1
    GF1 鑺墖鐨?DMA #锛?,3,5,6,7锛?
dma2
    GF1 鑺墖鐨?DMA #锛?,3,5,6,7,-1=绂佺敤锛?
joystick_dac
    0 鍒?31锛岋紙0.59V-4.52V 鎴?0.389V-2.98V锛?
voices
    GF1 璇煶鏁颁笂闄愶紙14-32锛?
pcm_voices
    淇濈暀鐨?PCM 璇煶鏁?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

### 妯″潡 snd-gusextreme


鐢ㄤ簬 Gravis UltraSound Extreme锛圫ynergy ViperMax锛夊０鍗＄殑妯″潡銆?

port
    ES-1688 鑺墖鐨勭鍙ｅ彿 #锛?x220,0x230,0x240,0x250,0x260锛?
gf1_port
    GF1 鑺墖鐨勭鍙ｅ彿 #锛?x210,0x220,0x230,0x240,0x250,0x260,0x270锛?
mpu_port
    MPU-401 绔彛鐨勭鍙ｅ彿 #锛?x300,0x310,0x320,0x330锛夛紝-1 = 绂佺敤
irq
    ES-1688 鑺墖鐨?IRQ #锛?,7,9,10锛?
gf1_irq
    GF1 鑺墖鐨?IRQ #锛?,5,9,11,12,15锛?
mpu_irq
    MPU-401 绔彛鐨?IRQ #锛?,7,9,10锛?
dma8
    ES-1688 鑺墖鐨?DMA #锛?,1,3锛?
dma1
    GF1 鑺墖鐨?DMA #锛?,3,5,6,7锛?
joystick_dac
    0 鍒?31锛岋紙0.59V-4.52V 鎴?0.389V-2.98V锛?
voices
    GF1 璇煶鏁颁笂闄愶紙14-32锛?
pcm_voices
    淇濈暀鐨?PCM 璇煶鏁?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴锛堜笉鍚?MPU-401 绔彛锛夈€?

### 妯″潡 snd-gusmax


鐢ㄤ簬 Gravis UltraSound MAX 澹板崱鐨勬ā鍧椼€?

port
    GF1 鑺墖鐨勭鍙ｅ彿 #锛?x220,0x230,0x240,0x250,0x260锛?
irq
    GF1 鑺墖鐨?IRQ #锛?,5,9,11,12,15锛?
dma1
    GF1 鑺墖鐨?DMA #锛?,3,5,6,7锛?
dma2
    GF1 鑺墖鐨?DMA #锛?,3,5,6,7,-1=绂佺敤锛?
joystick_dac
    0 鍒?31锛岋紙0.59V-4.52V 鎴?0.389V-2.98V锛?
voices
    GF1 璇煶鏁颁笂闄愶紙14-32锛?
pcm_voices
    淇濈暀鐨?PCM 璇煶鏁?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

### 妯″潡 snd-hda-intel


鐢ㄤ簬 Intel HD Audio锛圛CH6, ICH6M, ESB2, ICH7, ICH8, ICH9, ICH10,
PCH, SCH锛夈€丄TI SB450, SB600, R600, RS600, RS690, RS780, RV610, RV620,
RV630, RV635, RV670, RV770, VIA VT8251/VT8237A, SIS966, ULI M5461 鐨勬ā鍧椼€?

[姣忎釜澹板崱瀹炰緥鐨勫涓€夐」]

model
    寮哄埗鎸囧畾鍨嬪彿鍚嶇О
position_fix
    淇 DMA 鎸囬拡锛?
    -1 = 绯荤粺榛樿锛氭牴鎹帶鍒跺櫒纭欢閫夋嫨鍚堥€傜殑鏂规锛?
    0 = 鑷姩锛氬綋 POSBUF 涓嶅伐浣滄椂鍥為€€鍒?LPIB锛?
    1 = 浣跨敤 LPIB锛?
    2 = POSBUF锛氫娇鐢ㄤ綅缃紦鍐插尯锛?
    3 = VIACOMBO锛氶拡瀵归噰闆嗙殑 VIA 鐗瑰畾瑙勯伩鏂规锛?
    4 = COMBO锛氭挱鏀句娇鐢?LPIB锛岄噰闆嗘祦鑷姩
    5 = SKL+锛氬簲鐢ㄨ繎鏈?Intel 鑺墖涓婂彲鐢ㄧ殑寤惰繜璁＄畻
    6 = FIFO锛氱敤鍥哄畾鐨?FIFO 澶у皬淇浣嶇疆锛岀敤浜庤繎鏈?AMD 鑺墖
probe_mask
    鐢ㄤ簬鎺㈡祴缂栬В鐮佸櫒鐨勪綅鎺╃爜锛堥粯璁?= -1锛屽嵆鎵€鏈夋Ы浣嶏級锛?
    褰撶 8 浣嶏紙0x100锛夌疆浣嶆椂锛屼綆 8 浣嶇敤浣溾€滃浐瀹氣€濈殑缂栬В鐮佸櫒
    妲戒綅锛涘嵆鏃犺纭欢鎶ュ憡浠€涔堬紝椹卞姩閮戒細鎺㈡祴杩欎簺妲戒綅
probe_only
    浠呮帰娴嬭€屼笉鍒濆鍖栫紪瑙ｇ爜鍣紙榛樿=off锛夛紱
    鐢ㄤ簬妫€鏌ョ紪瑙ｇ爜鍣ㄧ殑鍒濆鐘舵€佷互璋冭瘯
bdl_pos_adj
    浠ラ噰鏍蜂负鍗曚綅鎸囧畾 DMA IRQ 瀹氭椂寤惰繜銆?
    浼犲叆 -1 灏嗚椹卞姩鏍规嵁鎺у埗鍣ㄨ姱鐗囬€夋嫨鍚堥€傜殑鍙栧€笺€?
patch
    鎸囧畾鍦ㄥ垵濮嬪寲缂栬В鐮佸櫒涔嬪墠鐢ㄤ簬淇敼 HD-audio 璁剧疆鐨勬棭鏈?
    鈥減atch鈥濇枃浠躲€?
    姝ら€夐」浠呭湪璁剧疆浜?`CONFIG_SND_HDA_PATCH_LOADER=y` 鏃跺彲鐢ㄣ€?
    璇﹁ hd-audio/notes.rst銆?
beep_mode
    閫夋嫨铚傞福娉ㄥ唽妯″紡锛?=鍏抽棴锛?=寮€鍚級锛?
    榛樿鍊奸€氳繃 `CONFIG_SND_HDA_INPUT_BEEP_MODE` kconfig 璁剧疆銆?

[鍗曚竴锛堝叏灞€锛夐€夐」]

single_cmd
    浣跨敤鍗曚竴绔嬪嵆鍛戒护涓庣紪瑙ｇ爜鍣ㄩ€氫俊
    锛堜粎鐢ㄤ簬璋冭瘯锛?
enable_msi
    鍚敤娑堟伅淇″彿涓柇锛圡SI锛夛紙榛樿 = 鍏抽棴锛?
power_save
    鑷姩鐪佺數瓒呮椂锛堜互绉掍负鍗曚綅锛? = 绂佺敤锛?
power_save_controller
    鍦ㄧ渷鐢垫ā寮忎笅澶嶄綅 HD-audio 鎺у埗鍣紙榛樿 = 寮€鍚級
pm_blacklist
    鍚敤 / 绂佺敤鐢垫簮绠＄悊鎷掔粷鍒楄〃锛堥粯璁?= 鏌ヨ PM
    鎷掔粷鍒楄〃锛? = 璺宠繃 PM 鎷掔粷鍒楄〃锛? = 寮哄埗鍏抽棴杩愯鏃?PM锛?
align_buffer_size
    寮哄埗灏嗙紦鍐插尯/鍛ㄦ湡澶у皬鍥涜垗浜斿叆鍒?128 瀛楄妭鐨勫€嶆暟銆?
    杩欏湪鍐呭瓨璁块棶鏂归潰鏇撮珮鏁堬紝浣?HDA 瑙勮寖骞朵笉瑕佹眰锛?
    涓斾細闃绘鐢ㄦ埛鎸囧畾绮剧‘鐨勫懆鏈?缂撳啿鍖哄ぇ灏忋€傦紙榛樿 = 寮€鍚級
snoop
    鍚敤/绂佺敤绐ユ帰锛堥粯璁?= 寮€鍚級

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

鏈夊叧 HD-audio 椹卞姩鐨勬洿澶氱粏鑺傦紝璇峰弬瑙?hd-audio/notes.rst銆?

姣忎釜缂栬В鐮佸櫒閮藉彲鑳芥湁閽堝涓嶅悓閰嶇疆鐨勫瀷鍙疯〃銆?
濡傛灉浣犵殑鏈哄櫒鏈垪鍦ㄥ叾涓紝鍒欎細璁剧疆榛樿锛堥€氬父鏄渶绮剧畝鐨勶級
閰嶇疆銆傚湪杩欑鎯呭喌涓嬩綘鍙互浼犲叆 `model=<name>` 閫夐」鏉ユ寚瀹?
鏌愪釜鍨嬪彿銆傛牴鎹紪瑙ｇ爜鍣ㄨ姱鐗囩殑涓嶅悓鏈変笉鍚屽瀷鍙枫€傚彲鐢ㄥ瀷鍙峰垪琛?
瑙?hd-audio/models.rst銆?

鍨嬪彿鍚?`generic` 琚涓轰竴绉嶇壒娈婃儏鍐点€傚綋缁欏畾姝ゅ瀷鍙锋椂锛岄┍鍔?
浣跨敤閫氱敤鐨勭紪瑙ｇ爜鍣ㄨВ鏋愬櫒锛岃€屼笉浣跨敤鈥渃odec-patch鈥濄€傝繖鏈夋椂
瀵规祴璇曞拰璋冭瘯寰堟湁鐢ㄣ€?

model 閫夐」涔熷彲鐢ㄤ簬鍒悕鍒板彟涓€涓?PCI 鎴栫紪瑙ｇ爜鍣?SSID銆?
褰撲互 `model=XXXX:YYYY` 鐨勫舰寮忎紶鍏ユ椂锛屽叾涓?XXXX 鍜?YYYY
鍒嗗埆鏄崄鍏繘鍒剁殑瀛愮郴缁熷巶鍟嗗拰瀛愮郴缁熻澶?ID锛岄┍鍔ㄥ皢鎶婅
SSID 浣滀负寮傚父琛ㄧ殑鍙傝€冦€?

濡傛灉榛樿閰嶇疆涓嶅伐浣滐紝鑰屼笂杩版煇涓€椤逛笌浣犵殑璁惧鍖归厤锛岃灏嗗畠
杩炲悓 alsa-info.sh 鐨勮緭鍑猴紙浣跨敤 `--no-upload` 閫夐」锛変竴璧?
鎶ュ憡缁?kernel bugzilla 鎴?alsa-devel 閭欢鍒楄〃
锛堣 `Links and Addresses`_ 灏忚妭锛夈€?

`power_save` 鍜?`power_save_controller` 閫夐」鐢ㄤ簬鐪佺數妯″紡銆?
璇﹁ powersave.rst銆?

娉ㄦ剰 2锛氬鏋滀綘鐨勮緭鍑烘湁鍜斿棐澹帮紝璇峰皾璇曟ā鍧楅€夐」
`position_fix=1` 鎴?`2`銆俙position_fix=1` 灏嗕娇鐢ㄦ湭鍋?FIFO
澶у皬淇鐨?SD_LPIB 瀵勫瓨鍣ㄥ€间綔涓哄綋鍓?DMA 鎸囬拡銆俙position_fix=2`
灏嗕娇椹卞姩浣跨敤浣嶇疆缂撳啿鍖鸿€屼笉鏄鍙?SD_LPIB 瀵勫瓨鍣ㄣ€?
锛堥€氬父 SD_LPIB 瀵勫瓨鍣ㄦ瘮浣嶇疆缂撳啿鍖烘洿绮剧‘銆傦級

`position_fix=3` 涓撶敤浜?VIA 璁惧銆傞噰闆嗘祦鐨勪綅缃粠 LPIB 鍜?
POSBUF 涓や釜鍊间腑妫€鏌ャ€俙position_fix=4` 鏄粍鍚堟ā寮忥紝鎾斁浣跨敤
LPIB锛岄噰闆嗕娇鐢?POSBUF銆?

娉ㄦ剰锛氬鏋滃湪鍔犺浇鏃跺嚭鐜板ぇ閲?`azx_get_response timeout` 娑堟伅锛?
寰堝彲鑳芥槸涓柇鐨勯棶棰橈紙渚嬪 ACPI irq 璺敱锛夈€傚皾璇曠敤
`pci=noacpi` 涔嬬被鐨勯€夐」鍚姩銆傛澶栵紝浣犲彲浠ュ皾璇?`single_cmd=1`
妯″潡閫夐」銆傝繖浼氬皢 HDA 鎺у埗鍣ㄤ笌缂栬В鐮佸櫒涔嬮棿鐨勯€氫俊鏂瑰紡鍒囨崲涓?
鍗曚竴绔嬪嵆鍛戒护锛岃€屼笉鏄?CORB/RIRB銆傚熀鏈笂锛屽崟涓€鍛戒护妯″紡浠呯敤浜?
BIOS锛屼綘涔熶笉浼氭敹鍒版湭 solicited 浜嬩欢銆備絾鑷冲皯锛屽畠鐙珛浜?irq
宸ヤ綔銆傝璁颁綇杩欐槸鏈€鍚庢墜娈碉紝搴斿敖鍙兘閬垮厤鈥︹€?

鍏充簬 `azx_get_response timeout` 闂鐨勬洿澶氳鏄庯細
鍦ㄦ煇浜涚‖浠朵笂锛屼綘鍙兘闇€瑕佹坊鍔犲悎閫傜殑 probe_mask 閫夐」鏉ラ伩鍏?
涓婅堪 `azx_get_response timeout` 闂銆傚綋璁块棶涓嶅瓨鍦ㄦ垨涓嶅伐浣?
鐨勭紪瑙ｇ爜鍣ㄦЫ浣嶏紙寰堝彲鑳芥槸璋冨埗鍣ㄦЫ浣嶏級瀵艰嚧 HD-audio 鎬荤嚎涓婄殑
閫氫俊鍋滄粸鏃跺氨浼氬彂鐢熻繖绉嶆儏鍐点€備綘鍙互閫氳繃鍚敤 `CONFIG_SND_DEBUG_VERBOSE`
鏉ユ煡鐪嬫帰娴嬩簡鍝簺缂栬В鐮佸櫒妲戒綅锛屾垨鑰呯洿鎺ヤ粠缂栬В鐮佸櫒 proc 鏂囦欢鐨?
鏂囦欢鍚嶇湅鍑恒€傜劧鍚庨€氳繃 probe_mask 閫夐」闄愬埗瑕佹帰娴嬬殑妲戒綅銆?
渚嬪锛宍probe_mask=1` 琛ㄧず鍙帰娴嬬涓€涓Ы浣嶏紝`probe_mask=4`
琛ㄧず鍙帰娴嬬涓変釜妲戒綅銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-hdsp


鐢ㄤ簬 RME Hammerfall DSP 闊抽鎺ュ彛鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

娉ㄦ剰锛氬綋璁剧疆浜?`CONFIG_FW_LOADER` 鏃讹紝鍥轰欢鏁版嵁鍙互閫氳繃 hotplug
鑷姩鍔犺浇銆傚惁鍒欙紝浣犻渶瑕侀€氳繃 alsa-tools 杞欢鍖呬腑鍖呭惈鐨?hdsploader
宸ュ叿鍔犺浇鍥轰欢銆傚浐浠舵暟鎹綅浜?alsa-firmware 杞欢鍖呬腑銆?

娉ㄦ剰锛歴nd-page-alloc 妯″潡鎵挎媴浜嗕互鍓?snd-hammerfall-mem 妯″潡鐨?
宸ヤ綔銆傚畠浼氬湪鍙戠幇浠讳綍 HDSP 澹板崱鏃堕鍏堝垎閰嶇紦鍐插尯銆備负浜嗙‘淇?
缂撳啿鍖哄垎閰嶆垚鍔燂紝璇峰湪鍚姩搴忓垪鐨勬棭鏈熼樁娈靛姞杞?snd-page-alloc
妯″潡銆傝瑙?`Early Buffer Allocation`_ 灏忚妭銆?

### 妯″潡 snd-hdspm


鐢ㄤ簬 RME HDSP MADI 鏉垮崱鐨勬ā鍧椼€?

precise_ptr
    鍚敤绮剧‘鎸囬拡锛屾垨绂佺敤銆?
line_outs_monitor
    榛樿灏嗘挱鏀炬祦鍙戦€佸埌妯℃嫙杈撳嚭銆?
enable_monitor
    榛樿鍦ㄩ€氶亾 63/64 涓婂惎鐢ㄦā鎷熻緭鍑恒€?

璇﹁ hdspm.rst銆?

### 妯″潡 snd-ice1712


鐢ㄤ簬鍩轰簬 Envy24锛圛CE1712锛夌殑 PCI 澹板崱鐨勬ā鍧椼€?

- MidiMan M Audio Delta 1010
- MidiMan M Audio Delta 1010LT
- MidiMan M Audio Delta DiO 2496
- MidiMan M Audio Delta 66
- MidiMan M Audio Delta 44
- MidiMan M Audio Delta 410
- MidiMan M Audio Audiophile 2496
- TerraTec EWS 88MT
- TerraTec EWS 88D
- TerraTec EWX 24/96
- TerraTec DMX 6Fire
- TerraTec Phase 88
- Hoontech SoundTrack DSP 24
- Hoontech SoundTrack DSP 24 Value
- Hoontech SoundTrack DSP 24 Media 7.1
- Event Electronics, EZ8
- Digigram VX442
- Lionstracs, Mediastaton
- Terrasoniq TS 88

model
    浣跨敤缁欏畾鐨勬澘鍗″瀷鍙凤紝浠ヤ笅涔嬩竴锛?
    delta1010, dio2496, delta66, delta44, audiophile, delta410,
    delta1010lt, vx442, ewx2496, ews88mt, ews88mt_new, ews88d,
    dmx6fire, dsp24, dsp24_value, dsp24_71, ez8,
    phase88, mediastation
omni
    MidiMan M-Audio Delta44/66 鐨?Omni I/O 鏀寔
cs8427_timeout
    CS8427 鑺墖锛圫/PDIF 鏀跺彂鍣級鐨勫浣嶈秴鏃讹紝浠?msec
    涓哄崟浣嶏紝榛樿鍊间负 500锛?.5 绉掞級

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?
娉ㄦ剰锛氭秷璐归儴鍒嗗苟闈炲湪鎵€鏈夊熀浜?Envy24 鐨勫０鍗′笂閮戒娇鐢?
锛堜緥濡傚湪 MidiMan Delta 绯诲垪涓級銆?

娉ㄦ剰锛氭敮鎸佺殑鏉垮崱閫氳繃璇诲彇 EEPROM 鎴?PCI SSID锛堣嫢 EEPROM
涓嶅彲鐢級鏉ユ娴嬨€傚鏋滈┍鍔ㄩ厤缃笉姝ｇ‘锛屾垨浣犳兂灏濊瘯鍙︿竴绉?
绫诲瀷杩涜娴嬭瘯锛屽彲浠ラ€氳繃浼犲叆 `model` 妯″潡閫夐」鏉ヨ鐩栧瀷鍙枫€?

### 妯″潡 snd-ice1724


鐢ㄤ簬鍩轰簬 Envy24HT锛圴T/ICE1724锛夈€丒nvy24PT锛圴T1720锛夌殑 PCI 澹板崱鐨勬ā鍧椼€?

- MidiMan M Audio Revolution 5.1
- MidiMan M Audio Revolution 7.1
- MidiMan M Audio Audiophile 192
- AMP Ltd AUDIO2000
- TerraTec Aureon 5.1 Sky
- TerraTec Aureon 7.1 Space
- TerraTec Aureon 7.1 Universe
- TerraTec Phase 22
- TerraTec Phase 28
- AudioTrak Prodigy 7.1
- AudioTrak Prodigy 7.1 LT
- AudioTrak Prodigy 7.1 XT
- AudioTrak Prodigy 7.1 HIFI
- AudioTrak Prodigy 7.1 HD2
- AudioTrak Prodigy 192
- Pontis MS300
- Albatron K8X800 Pro II
- Chaintech ZNF3-150
- Chaintech ZNF3-250
- Chaintech 9CJS
- Chaintech AV-710
- Shuttle SN25P
- Onkyo SE-90PCI
- Onkyo SE-200PCI
- ESI Juli@
- ESI Maya44
- Hercules Fortissimo IV
- EGO-SYS WaveTerminal 192M

model
    浣跨敤缁欏畾鐨勬澘鍗″瀷鍙凤紝浠ヤ笅涔嬩竴锛?
    revo51, revo71, amp2000, prodigy71, prodigy71lt,
    prodigy71xt, prodigy71hifi, prodigyhd2, prodigy192,
    juli, aureon51, aureon71, universe, ap192, k8x800,
    phase22, phase28, ms300, av710, se200pci, se90pci,
    fortissimo4, sn25p, WT192M, maya44

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

娉ㄦ剰锛氭敮鎸佺殑鏉垮崱閫氳繃璇诲彇 EEPROM 鎴?PCI SSID锛堣嫢 EEPROM
涓嶅彲鐢級鏉ユ娴嬨€傚鏋滈┍鍔ㄩ厤缃笉姝ｇ‘锛屾垨浣犳兂灏濊瘯鍙︿竴绉?
绫诲瀷杩涜娴嬭瘯锛屽彲浠ラ€氳繃浼犲叆 `model` 妯″潡閫夐」鏉ヨ鐩栧瀷鍙枫€?

### 妯″潡 snd-indigo


鐢ㄤ簬 Echoaudio Indigo 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-indigodj


鐢ㄤ簬 Echoaudio Indigo DJ 鐨勬ā鍧椼€?


璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?


### 妯″潡 snd-indigoio


鐢ㄤ簬 Echoaudio Indigo IO 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-intel8x0


鐢ㄤ簬鏉ヨ嚜 Intel 鍙婂吋瀹瑰巶鍟嗙殑 AC'97 涓绘澘鐨勬ā鍧椼€?

- Intel i810/810E, i815, i820, i830, i84x, MX440 ICH5, ICH6, ICH7,
  6300ESB, ESB2
- SiS 7012 (SiS 735)
- NVidia NForce, NForce2, NForce3, MCP04, CK804 CK8, CK8S, MCP501
- AMD AMD768, AMD8111
- ALi m5455

ac97_clock
    AC'97 缂栬В鐮佸櫒鏃堕挓鍩哄噯锛? = 鑷姩鎺㈡祴锛?
ac97_quirk
    閽堝寮傚父纭欢鐨?AC'97 瑙勯伩鏂规锛?
    瑙佷笅闈㈢殑 `AC97 Quirk Option`_ 灏忚妭銆?
buggy_irq
    鍚敤鏌愪簺涓绘澘涓婂紓甯镐腑鏂殑瑙勯伩鏂规
    锛堝湪 nForce 鑺墖涓婇粯璁や负寮€鍚紝鍏朵粬涓哄叧闂級
buggy_semaphore
    鍚敤閽堝甯︽湁寮傚父淇″彿閲忕殑纭欢鐨勮閬挎柟妗堬紙渚嬪鏌愪簺
    ASUS 绗旇鏈級锛堥粯璁ゅ叧闂級
spdif_aclink
    浣跨敤閫氳繃 AC-link 鐨?S/PDIF锛岃€屼笉鏄潵鑷帶鍒跺櫒鑺墖鐨?
    鐩存帴杩炴帴锛? = 鍏抽棴锛? = 寮€鍚紝-1 = 榛樿锛?

璇ユā鍧楁敮鎸佸崟鑺墖鍜岃嚜鍔ㄦ帰娴嬨€?

娉ㄦ剰锛氭渶鏂扮殑椹卞姩鏀寔鑺墖鏃堕挓鐨勮嚜鍔ㄦ帰娴嬨€傚鏋滀綘浠嶇劧閬囧埌
鎾斁杩囧揩鐨勯棶棰橈紝璇烽€氳繃妯″潡閫夐」 `ac97_clock=41194` 鏄惧紡
鎸囧畾鏃堕挓銆?

鏈┍鍔ㄤ笉鏀寔娓告垙鏉?MIDI 绔彛銆傚鏋滀綘鐨勪富鏉挎湁杩欎簺璁惧锛岃
鍒嗗埆浣跨敤 ns558 鎴?snd-mpu401 妯″潡銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-intel8x0m


鐢ㄤ簬 Intel ICH锛坕8x0锛夎姱鐗囩粍 MC97 璋冨埗鍣ㄧ殑妯″潡銆?

- Intel i810/810E, i815, i820, i830, i84x, MX440 ICH5, ICH6, ICH7
- SiS 7013 (SiS 735)
- NVidia NForce, NForce2, NForce2s, NForce3
- AMD AMD8111
- ALi m5455

ac97_clock
    AC'97 缂栬В鐮佸櫒鏃堕挓鍩哄噯锛? = 鑷姩鎺㈡祴锛?

璇ユā鍧楁敮鎸佸崟鍧楀０鍗″拰鑷姩鎺㈡祴銆?

娉ㄦ剰锛氳妯″潡鐨勯粯璁?index 鍊间负 -2锛屽嵆绗竴涓Ы浣嶈鎺掗櫎銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-interwave


鐢ㄤ簬 Gravis UltraSound PnP銆丏ynasonic 3-D/Pro銆丼TB Sound Rage 32
浠ュ強鍩轰簬 AMD InterWave (tm) 鑺墖鐨勫叾浠栧０鍗＄殑妯″潡銆?

joystick_dac
    0 鍒?31锛岋紙0.59V-4.52V 鎴?0.389V-2.98V锛?
midi
    1 = 鍚敤 MIDI UART锛? = 绂佺敤 MIDI UART锛堥粯璁わ級
pcm_voices
    涓哄悎鎴愬櫒淇濈暀鐨?PCM 璇煶鏁帮紙榛樿 2锛?
effect
    1 = 鍚敤 InterWave 鏁堟灉锛堥粯璁?0锛夛紱闇€瑕?8 涓闊?
isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

鍦?`isapnp=0` 鏃讹紝鍙娇鐢ㄤ互涓嬮€夐」锛?

port
    InterWave 鑺墖鐨勭鍙ｅ彿 #锛?x210,0x220,0x230,0x240,0x250,0x260锛?
irq
    InterWave 鑺墖鐨?IRQ #锛?,5,9,11,12,15锛?
dma1
    InterWave 鑺墖鐨?DMA #锛?,1,3,5,6,7锛?
dma2
    InterWave 鑺墖鐨?DMA #锛?,1,3,5,6,7,-1=绂佺敤锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€佽嚜鍔ㄦ帰娴嬪拰 ISA PnP銆?

### 妯″潡 snd-interwave-stb


鐢ㄤ簬 UltraSound 32-Pro锛圕ompaq 浣跨敤鐨?STB 澹板崱锛変互鍙婂熀浜?AMD
InterWave (tm) 鑺墖銆佸苟甯︽湁 TEA6330T 鐢佃矾浠ユ墿灞曟帶鍒朵綆闊炽€?
楂橀煶鍜屼富闊抽噺鐨勫叾浠栧０鍗＄殑妯″潡銆?

joystick_dac
    0 鍒?31锛岋紙0.59V-4.52V 鎴?0.389V-2.98V锛?
midi
    1 = 鍚敤 MIDI UART锛? = 绂佺敤 MIDI UART锛堥粯璁わ級
pcm_voices
    涓哄悎鎴愬櫒淇濈暀鐨?PCM 璇煶鏁帮紙榛樿 2锛?
effect
    1 = 鍚敤 InterWave 鏁堟灉锛堥粯璁?0锛夛紱闇€瑕?8 涓闊?
isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

鍦?`isapnp=0` 鏃讹紝鍙娇鐢ㄤ互涓嬮€夐」锛?

port
    InterWave 鑺墖鐨勭鍙ｅ彿 #锛?x210,0x220,0x230,0x240,0x250,0x260锛?
port_tc
    TEA6330T 鑺墖锛坕2c 鎬荤嚎锛夌殑闊宠皟鎺у埗绔彛鍙?#锛?x350,0x360,0x370,0x380锛?
irq
    InterWave 鑺墖鐨?IRQ #锛?,5,9,11,12,15锛?
dma1
    InterWave 鑺墖鐨?DMA #锛?,1,3,5,6,7锛?
dma2
    InterWave 鑺墖鐨?DMA #锛?,1,3,5,6,7,-1=绂佺敤锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€佽嚜鍔ㄦ帰娴嬪拰 ISA PnP銆?

### 妯″潡 snd-jazz16


鐢ㄤ簬 Media Vision Jazz16 鑺墖缁勭殑妯″潡銆傝鑺墖缁勭敱 3 涓姱鐗囩粍鎴愶細
MVD1216 + MVA416 + MVA514銆?

port
    SB DSP 鑺墖鐨勭鍙ｅ彿 #锛?x210,0x220,0x230,0x240,0x250,0x260锛?
irq
    SB DSP 鑺墖鐨?IRQ #锛?,5,7,9,10,15锛?
dma8
    SB DSP 鑺墖鐨?DMA #锛?,3锛?
dma16
    SB DSP 鑺墖鐨?DMA #锛?,7锛?
mpu_port
    MPU-401 绔彛鍙?#锛?x300,0x310,0x320,0x330锛?
mpu_irq
    MPU-401 鐨?irq #锛?,3,5,7锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-korg1212


鐢ㄤ簬 Korg 1212 IO PCI 澹板崱鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-layla20


鐢ㄤ簬 Echoaudio Layla20 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-layla24


鐢ㄤ簬 Echoaudio Layla24 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-lola


鐢ㄤ簬 Digigram Lola PCI-e 鏉垮崱鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-lx6464es


鐢ㄤ簬 Digigram LX6464ES 鏉垮崱鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-maestro3


鐢ㄤ簬 Allegro/Maestro3 鑺墖鐨勬ā鍧椼€?

external_amp
    鍚敤澶栭儴鏀惧ぇ鍣紙榛樿鍚敤锛?
amp_gpio
    澶栭儴鏀惧ぇ鍣ㄧ殑 GPIO 寮曡剼鍙凤紙0-15锛夋垨 -1 琛ㄧず榛樿寮曡剼
    锛坅llegro 涓?8锛屽叾浠栦负 1锛?

璇ユā鍧楁敮鎸佽嚜鍔ㄦ帰娴嬪拰澶氳姱鐗囥€?

娉ㄦ剰锛氭斁澶у櫒鐨勭粦瀹氬彇鍐充簬纭欢銆傚鏋滄墍鏈夐€氶亾閮藉凡瑙ｉ櫎闈欓煶鍗?
浠嶇劧娌℃湁澹伴煶锛岃灏濊瘯閫氳繃 amp_gpio 閫夐」鎸囧畾鍏朵粬 gpio 杩炴帴銆?
渚嬪锛屾煇浜涙澗涓嬬瑪璁版湰鍙兘闇€瑕?`amp_gpio=0x0d` 閫夐」銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-mia


鐢ㄤ簬 Echoaudio Mia 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-miro


鐢ㄤ簬 Miro 澹板崱锛歮iroSOUND PCM 1 pro銆乵iroSOUND PCM 12銆?
miroSOUND PCM 20 Radio銆?

port
    绔彛鍙?#锛?x530,0x604,0xe80,0xf40锛?
irq
    IRQ #锛?,7,9,10,11锛?
dma1
    绗竴涓?dma #锛?,1,3锛?
dma2
    绗簩涓?dma #锛?,1锛?
mpu_port
    MPU-401 绔彛鍙?#锛?x300,0x310,0x320,0x330锛?
mpu_irq
    MPU-401 鐨?irq #锛?,7,9,10锛?
fm_port
    FM 绔彛鍙?#锛?x388锛?
wss
    鍚敤 WSS 妯″紡
ide
    鍚敤鏉胯浇 ide 鏀寔

### 妯″潡 snd-mixart


鐢ㄤ簬 Digigram miXart8 澹板崱鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
娉ㄦ剰锛氫竴鍧?miXart8 鏉垮崱浼氳琛ㄧず涓?4 涓?alsa 澹板崱銆?
璇﹁ Documentation/sound/cards/mixart.rst銆?

褰撻┍鍔ㄧ紪璇戜负妯″潡涓旀敮鎸?hotplug 鍥轰欢鏃讹紝鍥轰欢鏁版嵁浼氶€氳繃
hotplug 鑷姩鍔犺浇銆傝鍦?alsa-firmware 杞欢鍖呬腑瀹夎鎵€闇€鐨?
鍥轰欢鏂囦欢銆傚綋娌℃湁鍙敤鐨?hotplug 鍥轰欢鍔犺浇鍣ㄦ椂锛屼綘闇€瑕侀€氳繃
alsa-tools 杞欢鍖呬腑鐨?mixartloader 宸ュ叿鍔犺浇鍥轰欢銆?

### 妯″潡 snd-mona


鐢ㄤ簬 Echoaudio Mona 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-mpu401


鐢ㄤ簬 MPU-401 UART 璁惧鐨勬ā鍧椼€?

port
    绔彛鍙锋垨 -1锛堢鐢級
irq
    IRQ 鍙锋垨 -1锛堢鐢級
pnp
    PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

璇ユā鍧楁敮鎸佸璁惧鍜?PnP銆?

### 妯″潡 snd-msnd-classic


鐢ㄤ簬 Turtle Beach MultiSound Classic銆乀ahiti 鎴?Monterey 澹板崱鐨勬ā鍧椼€?

io
    msnd-classic 澹板崱鐨勭鍙ｅ彿 #
irq
    msnd-classic 澹板崱鐨?IRQ #
mem
    鍐呭瓨鍦板潃锛?xb0000, 0xc8000, 0xd0000, 0xd8000, 0xe0000 鎴?0xe8000锛?
write_ndelay
    鍚敤鍐?ndelay锛堥粯璁?= 1锛?
calibrate_signal
    鏍″噯淇″彿锛堥粯璁?= 0锛?
isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級
digital
    瀛樺湪鏁板瓧瀛愭澘锛堥粯璁?= 0锛?
cfg
    閰嶇疆绔彛锛?x250, 0x260 鎴?0x270锛夐粯璁?= PnP
reset
    澶嶄綅鎵€鏈夎澶?
mpu_io
    MPU401 I/O 绔彛
mpu_irq
    MPU401 irq#
ide_io0
    IDE 绔彛 #0
ide_io1
    IDE 绔彛 #1
ide_irq
    IDE irq#
joystick_io
    娓告垙鏉?I/O 绔彛

璇ラ┍鍔ㄩ渶瑕佸浐浠舵枃浠?`turtlebeach/msndinit.bin` 鍜?
`turtlebeach/msndperm.bin` 浣嶄簬姝ｇ‘鐨勫浐浠剁洰褰曚腑銆?

鍏充簬璇ラ┍鍔ㄧ殑閲嶈淇℃伅锛岃鍙傝 Documentation/sound/cards/multisound.sh銆?
娉ㄦ剰瀹冨凡琚仠姝㈢淮鎶わ紝浣?Voyetra Turtle Beach 鍏充簬瀹冪殑鐭ヨ瘑搴?
鏉＄洰浠嶅彲鍦ㄤ互涓嬪湴鍧€鑾峰彇锛?
https://www.turtlebeach.com

### 妯″潡 snd-msnd-pinnacle


鐢ㄤ簬 Turtle Beach MultiSound Pinnacle/Fiji 澹板崱鐨勬ā鍧椼€?

io
    pinnacle/fiji 澹板崱鐨勭鍙ｅ彿 #
irq
    pinnalce/fiji 澹板崱鐨?IRQ #
mem
    鍐呭瓨鍦板潃锛?xb0000, 0xc8000, 0xd0000, 0xd8000, 0xe0000 鎴?0xe8000锛?
write_ndelay
    鍚敤鍐?ndelay锛堥粯璁?= 1锛?
calibrate_signal
    鏍″噯淇″彿锛堥粯璁?= 0锛?
isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

璇ラ┍鍔ㄩ渶瑕佸浐浠舵枃浠?`turtlebeach/pndspini.bin` 鍜?
`turtlebeach/pndsperm.bin` 浣嶄簬姝ｇ‘鐨勫浐浠剁洰褰曚腑銆?

### 妯″潡 snd-mtpav


鐢ㄤ簬 MOTU MidiTimePiece AV 澶氱鍙?MIDI锛堝苟鍙ｏ級鐨勬ā鍧椼€?

port
    MTPAV 鐨?I/O 绔彛鍙?#锛?x378,0x278锛岄粯璁?0x378锛?
irq
    MTPAV 鐨?IRQ #锛?,5锛岄粯璁?7锛?
hwports
    鍙楁敮鎸佺殑纭欢绔彛鏁帮紝榛樿=8銆?

妯″潡浠呮敮鎸?1 鍧楀０鍗°€傝妯″潡娌℃湁 enable 閫夐」銆?

### 妯″潡 snd-mts64


鐢ㄤ簬 Ego Systems锛圗SI锛塎iditerminal 4140 鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸璁惧銆?
闇€瑕?parport锛坄CONFIG_PARPORT`锛夈€?

### 妯″潡 snd-nm256


鐢ㄤ簬 NeoMagic NM256AV/ZX 鑺墖鐨勬ā鍧椼€?

playback_bufsize
    鏈€澶ф挱鏀惧抚澶у皬锛屼互 kB 涓哄崟浣嶏紙4-128kB锛?
capture_bufsize
    鏈€澶ч噰闆嗗抚澶у皬锛屼互 kB 涓哄崟浣嶏紙4-128kB锛?
force_ac97
    0 鎴?1锛堥粯璁ょ鐢級
buffer_top
    鎸囧畾缂撳啿鍖洪《閮ㄥ湴鍧€
use_cache
    0 鎴?1锛堥粯璁ょ鐢級
vaio_hack
    鍒悕 buffer_top=0x25a800
reset_workaround
    涓烘煇浜涚瑪璁版湰鍚敤 AC97 RESET 瑙勯伩鏂规
reset_workaround2
    涓烘煇浜涘叾浠栫瑪璁版湰鍚敤鎵╁睍鐨?AC97 RESET 瑙勯伩鏂规

璇ユā鍧楁敮鎸佸崟鑺墖鍜岃嚜鍔ㄦ帰娴嬨€?

鏀寔鐢垫簮绠＄悊銆?

娉ㄦ剰锛氬湪鏌愪簺绗旇鏈笂锛岀紦鍐插尯鍦板潃鏃犳硶鑷姩鎺㈡祴锛屾垨鍦ㄥ垵濮嬪寲
鏈熼棿瀵艰嚧鎸傝捣銆傚湪杩欑鎯呭喌涓嬶紝璇烽€氳繃 buffer_top 閫夐」鏄惧紡鎸囧畾
缂撳啿鍖洪《閮ㄥ湴鍧€銆備緥濡傦細
Sony F250锛歜uffer_top=0x25a800
Sony F270锛歜uffer_top=0x272800
璇ラ┍鍔ㄤ粎鏀寔 ac97 缂栬В鐮佸櫒銆傚嵆浣挎湭鎺㈡祴鍒帮紝涔熷彲浠ュ己鍒跺垵濮嬪寲/
浣跨敤 ac97銆傚湪杩欑鎯呭喌涓嬶紝浣跨敤 `force_ac97=1` 閫夐」鈥斺€斾絾鑳藉惁
宸ヤ綔**涓?*浣滀换浣曚繚璇侊紒

娉ㄦ剰锛歂M256 鑺墖鍙互鍦ㄥ唴閮ㄤ笌闈?AC97 缂栬В鐮佸櫒杩炴帴銆傛湰椹卞姩
浠呮敮鎸?AC97 缂栬В鐮佸櫒锛屾棤娉曚笌甯︽湁鍏朵粬锛堝緢鍙兘鏄?CS423x 鎴?
OPL3SAx锛夎姱鐗囩殑鏈哄櫒宸ヤ綔锛屽嵆浣胯璁惧鍦?lspci 涓兘琚帰娴嬪埌銆?
鍦ㄨ繖绉嶆儏鍐典笅锛岃灏濊瘯鍏朵粬椹卞姩锛屼緥濡?snd-cs4232 鎴?snd-opl3sa2銆?
鍏朵腑涓€浜涙敮鎸?ISA-PnP锛屼竴浜涗笉鏀寔銆傚湪娌℃湁 ISA PnP 鐨勬儏鍐典笅锛?
浣犻渶瑕佹寚瀹?`isapnp=0` 浠ュ強姝ｇ‘鐨勭‖浠跺弬鏁般€?

娉ㄦ剰锛氭煇浜涚瑪璁版湰闇€瑕侀拡瀵?AC97 RESET 鐨勮閬挎柟妗堛€傚浜庡凡鐭ョ殑
纭欢濡?Dell Latitude LS 鍜?Sony PCG-F305锛屾瑙勯伩鏂规浼氳嚜鍔?
鍚敤銆傚浜庡叾浠栧嚭鐜扮‖鍐荤粨鐨勭瑪璁版湰锛屼綘鍙互灏濊瘯 `reset_workaround=1`
閫夐」銆?

娉ㄦ剰锛欴ell Latitude CSx 绗旇鏈湪 AC97 RESET 鏂归潰鏈夊彟涓€涓棶棰樸€?
鍦ㄨ繖浜涚瑪璁版湰涓婏紝reset_workaround2 閫夐」榛樿寮€鍚€傚鏋滀箣鍓嶇殑
reset_workaround 閫夐」娌℃湁甯姪锛岃繖涓€夐」鍊煎緱涓€璇曘€?

娉ㄦ剰锛氳繖涓┍鍔ㄧ湡鐨勫緢绯熺硶銆傚畠绉绘鑷?OSS 椹卞姩锛岃€屽悗鑰呮槸榛戦瓟娉?
鑸€嗗悜宸ョ▼鐨勪骇鐗┿€傚鏋滈┍鍔ㄥ湪 X-server 涔嬪悗鍔犺浇锛堝涓婃墍杩帮級锛?
缂栬В鐮佸櫒鐨勬帰娴嬩細澶辫触銆備綘鍙兘鑳藉寮哄埗鍔犺浇璇ユā鍧楋紝浣嗗彲鑳藉鑷?
鎸傝捣銆傚洜姝わ紝濡傛灉閬囧埌杩欑被闂锛岃纭繚鍦?X 涔嬪墠鍔犺浇姝ゆā鍧椼€?

### 妯″潡 snd-opl3sa2


鐢ㄤ簬 Yamaha OPL3-SA2/SA3 澹板崱鐨勬ā鍧椼€?

isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

鍦?`isapnp=0` 鏃讹紝鍙娇鐢ㄤ互涓嬮€夐」锛?

port
    OPL3-SA 鑺墖鐨勬帶鍒剁鍙ｅ彿 #锛?x370锛?
sb_port
    OPL3-SA 鑺墖鐨?SB 绔彛鍙?#锛?x220,0x240锛?
wss_port
    OPL3-SA 鑺墖鐨?WSS 绔彛鍙?#锛?x530,0xe80,0xf40,0x604锛?
midi_port
    MPU-401 UART 鐨勭鍙ｅ彿 #锛?x300,0x330锛夛紝-1 = 绂佺敤
fm_port
    OPL3-SA 鑺墖鐨?FM 绔彛鍙?#锛?x388锛夛紝-1 = 绂佺敤
irq
    OPL3-SA 鑺墖鐨?IRQ #锛?,7,9,10锛?
dma1
    Yamaha OPL3-SA 鑺墖鐨勭涓€涓?DMA #锛?,1,3锛?
dma2
    Yamaha OPL3-SA 鑺墖鐨勭浜屼釜 DMA #锛?,1,3锛夛紝-1 = 绂佺敤

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰 ISA PnP銆傚畠涓嶆敮鎸佽嚜鍔ㄦ帰娴嬶紙鑻ユ湭浣跨敤
ISA PnP锛夛紝鍥犳蹇呴』鎸囧畾鎵€鏈夌鍙ｏ紒锛侊紒

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-opti92x-ad1848


鐢ㄤ簬鍩轰簬 OPTi 82c92x 鍜?Analog Devices AD1848 鑺墖鐨勫０鍗＄殑妯″潡銆?
璇ユā鍧椾篃閫傜敤浜?OAK Mozart 澹板崱銆?

isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

鍦?`isapnp=0` 鏃讹紝鍙娇鐢ㄤ互涓嬮€夐」锛?

port
    WSS 鑺墖鐨勭鍙ｅ彿 #锛?x530,0xe80,0xf40,0x604锛?
mpu_port
    MPU-401 UART 鐨勭鍙ｅ彿 #锛?x300,0x310,0x320,0x330锛?
fm_port
    OPL3 璁惧鐨勭鍙ｅ彿 #锛?x388锛?
irq
    WSS 鑺墖鐨?IRQ #锛?,7,9,10,11锛?
mpu_irq
    MPU-401 UART 鐨?IRQ #锛?,7,9,10锛?
dma1
    WSS 鑺墖鐨勭涓€涓?DMA #锛?,1,3锛?

璇ユā鍧椾粎鏀寔涓€鍧楀０鍗°€佽嚜鍔ㄦ帰娴嬪拰 PnP銆?

### 妯″潡 snd-opti92x-cs4231


鐢ㄤ簬鍩轰簬 OPTi 82c92x 鍜?Crystal CS4231 鑺墖鐨勫０鍗＄殑妯″潡銆?

isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

鍦?`isapnp=0` 鏃讹紝鍙娇鐢ㄤ互涓嬮€夐」锛?

port
    WSS 鑺墖鐨勭鍙ｅ彿 #锛?x530,0xe80,0xf40,0x604锛?
mpu_port
    MPU-401 UART 鐨勭鍙ｅ彿 #锛?x300,0x310,0x320,0x330锛?
fm_port
    OPL3 璁惧鐨勭鍙ｅ彿 #锛?x388锛?
irq
    WSS 鑺墖鐨?IRQ #锛?,7,9,10,11锛?
mpu_irq
    MPU-401 UART 鐨?IRQ #锛?,7,9,10锛?
dma1
    WSS 鑺墖鐨勭涓€涓?DMA #锛?,1,3锛?
dma2
    WSS 鑺墖鐨勭浜屼釜 DMA #锛?,1,3锛?

璇ユā鍧椾粎鏀寔涓€鍧楀０鍗°€佽嚜鍔ㄦ帰娴嬪拰 PnP銆?

### 妯″潡 snd-opti93x


鐢ㄤ簬鍩轰簬 OPTi 82c93x 鑺墖鐨勫０鍗＄殑妯″潡銆?

isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

鍦?`isapnp=0` 鏃讹紝鍙娇鐢ㄤ互涓嬮€夐」锛?

port
    WSS 鑺墖鐨勭鍙ｅ彿 #锛?x530,0xe80,0xf40,0x604锛?
mpu_port
    MPU-401 UART 鐨勭鍙ｅ彿 #锛?x300,0x310,0x320,0x330锛?
fm_port
    OPL3 璁惧鐨勭鍙ｅ彿 #锛?x388锛?
irq
    WSS 鑺墖鐨?IRQ #锛?,7,9,10,11锛?
mpu_irq
    MPU-401 UART 鐨?IRQ #锛?,7,9,10锛?
dma1
    WSS 鑺墖鐨勭涓€涓?DMA #锛?,1,3锛?
dma2
    WSS 鑺墖鐨勭浜屼釜 DMA #锛?,1,3锛?

璇ユā鍧椾粎鏀寔涓€鍧楀０鍗°€佽嚜鍔ㄦ帰娴嬪拰 PnP銆?

### 妯″潡 snd-oxygen


鐢ㄤ簬鍩轰簬 C-Media CMI8786/8787/8788 鑺墖鐨勫０鍗＄殑妯″潡锛?

- Asound A-8788
- Asus Xonar DG/DGX
- AuzenTech X-Meridian
- AuzenTech X-Meridian 2G
- Bgears b-Enspirer
- Club3D Theatron DTS
- HT-Omega Claro (plus)
- HT-Omega Claro halo (XT)
- Kuroutoshikou CMI8787-HG2PCI
- Razer Barracuda AC-1
- Sondigo Inferno
- TempoTec HiFier Fantasia
- TempoTec HiFier Serenade

璇ユā鍧楁敮鎸佽嚜鍔ㄦ帰娴嬪拰澶氬潡澹板崱銆?

### 妯″潡 snd-pcsp


鐢ㄤ簬鍐呴儴 PC 鎵０鍣紙PC-Speaker锛夌殑妯″潡銆?

nopcm
    绂佺敤 PC 鎵０鍣?PCM 澹伴煶銆備粎淇濈暀铚傞福澹般€?
nforce_wa
    鍚敤 NForce 鑺墖缁勮閬挎柟妗堛€傞鏈熷０闊宠川閲忚緝宸€?

璇ユā鍧楁敮鎸佺郴缁熻渹楦ｃ€佹煇绉?PCM 鎾斁锛岀敋鑷冲嚑涓贩闊冲櫒鎺у埗銆?

### 妯″潡 snd-pcxhr


鐢ㄤ簬 Digigram PCXHR 鏉垮崱鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-portman2x4


鐢ㄤ簬 Midiman Portman 2x4 骞跺彛 MIDI 鎺ュ彛鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-powermac锛堜粎 ppc 涓婏級


鐢ㄤ簬 PowerMac銆乮Mac 鍜?iBook 鏉胯浇澹拌姱鐗囩殑妯″潡銆?

enable_beep
    鍚敤浣跨敤 PCM 鐨勮渹楦ｅ０锛堥粯璁ゅ惎鐢級

妯″潡鏀寔鑷姩鎺㈡祴鑺墖銆?

娉ㄦ剰锛氳椹卞姩鍦ㄥ瓧鑺傚簭鏂归潰鍙兘鏈夐棶棰樸€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-pxa2xx-ac97锛堜粎 arm 涓婏級


鐢ㄤ簬 Intel PXA2xx 鑺墖鐨?AC97 椹卞姩鐨勬ā鍧椼€?

浠呯敤浜?ARM 鏋舵瀯銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-riptide


鐢ㄤ簬 Conexant Riptide 鑺墖鐨勬ā鍧椼€?

joystick_port
    娓告垙鏉嗙鍙ｅ彿 #锛堥粯璁わ細0x200锛?
mpu_port
    MPU401 绔彛鍙?#锛堥粯璁わ細0x330锛?
opl3_port
    OPL3 绔彛鍙?#锛堥粯璁わ細0x388锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?
璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?
浣犻渶瑕佸皢鍥轰欢鏂囦欢 `riptide.hex` 瀹夎鍒版爣鍑嗗浐浠惰矾寰?
锛堜緥濡?/lib/firmware锛夈€?

### 妯″潡 snd-rme32


鐢ㄤ簬 RME Digi32銆丏igi32 Pro 鍜?Digi32/8锛圫ek'd Prodif32銆?
Prodif96 鍜?Prodif Gold锛夊０鍗＄殑妯″潡銆?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-rme96


鐢ㄤ簬 RME Digi96銆丏igi96/8 鍜?Digi96/8 PRO/PAD/PST 澹板崱鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-rme9652


鐢ㄤ簬 RME Digi9652锛圚ammerfall銆丠ammerfall-Light锛夊０鍗＄殑妯″潡銆?

precise_ptr
    鍚敤绮剧‘鎸囬拡锛堝伐浣滀笉鍙潬锛夈€傦紙榛樿 = 0锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

娉ㄦ剰锛歴nd-page-alloc 妯″潡鎵挎媴浜嗕互鍓?snd-hammerfall-mem 妯″潡
鐨勫伐浣溿€傚畠浼氬湪鍙戠幇浠讳綍 RME9652 澹板崱鏃堕鍏堝垎閰嶇紦鍐插尯銆備负浜嗙‘淇?
缂撳啿鍖哄垎閰嶆垚鍔燂紝璇峰湪鍚姩搴忓垪鐨勬棭鏈熼樁娈靛姞杞?snd-page-alloc
妯″潡銆傝瑙?`Early Buffer Allocation`_ 灏忚妭銆?

### 妯″潡 snd-sa11xx-uda1341锛堜粎 arm 涓婏級


鐢ㄤ簬 Compaq iPAQ H3600 澹板崱涓?Philips UDA1341TS 鐨勬ā鍧椼€?

妯″潡浠呮敮鎸佷竴鍧楀０鍗°€?
妯″潡娌℃湁 enable 鍜?index 閫夐」銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-sb8


鐢ㄤ簬 8 浣?SoundBlaster 澹板崱锛歋oundBlaster 1.0銆丼oundBlaster 2.0銆?
SoundBlaster Pro 鐨勬ā鍧椼€?

port
    SB DSP 鑺墖鐨勭鍙ｅ彿 #锛?x220,0x240,0x260锛?
irq
    SB DSP 鑺墖鐨?IRQ #锛?,7,9,10锛?
dma8
    SB DSP 鑺墖鐨?DMA #锛?,3锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-sb16 鍜?snd-sbawe


鐢ㄤ簬 16 浣?SoundBlaster 澹板崱锛歋oundBlaster 16锛圥nP锛夈€?
SoundBlaster AWE 32锛圥nP锛夈€丼oundBlaster AWE 64 PnP 鐨勬ā鍧椼€?

mic_agc
    楹﹀厠椋庤嚜鍔ㄥ鐩婃帶鍒?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級
csp
    ASP/CSP 鑺墖鏀寔 - 0 = 绂佺敤锛堥粯璁わ級锛? = 鍚敤
isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

鍦?isapnp=0 鏃讹紝鍙娇鐢ㄤ互涓嬮€夐」锛?

port
    SB DSP 4.x 鑺墖鐨勭鍙ｅ彿 #锛?x220,0x240,0x260锛?
mpu_port
    MPU-401 UART 鐨勭鍙ｅ彿 #锛?x300,0x330锛夛紝-1 = 绂佺敤
awe_port
    EMU8000 鍚堟垚鍣ㄧ殑鍩虹鍙ｅ彿 #锛?x620,0x640,0x660锛夛紙浠?snd-sbawe
    妯″潡锛?
irq
    SB DSP 4.x 鑺墖鐨?IRQ #锛?,7,9,10锛?
dma8
    SB DSP 4.x 鑺墖鐨?8 浣?DMA #锛?,1,3锛?
dma16
    SB DSP 4.x 鑺墖鐨?16 浣?DMA #锛?,6,7锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€佽嚜鍔ㄦ帰娴嬪拰 ISA PnP銆?


娉ㄦ剰锛氳嫢瑕佸湪 16 浣嶅崐鍙屽伐妯″紡涓嬩娇鐢?Vibra16X 澹板崱锛屽繀椤?
閫氳繃 dma16 = -1 妯″潡鍙傛暟绂佺敤 16 浣?DMA銆傛澶栵紝鎵€鏈?Sound Blaster 16
绫诲瀷澹板崱閮藉彲浠ラ€氳繃绂佺敤鍏?16 浣?DMA 閫氶亾锛屾敼鐢?8 浣?DMA 閫氶亾
浠?16 浣嶅崐鍙屽伐妯″紡宸ヤ綔銆?

鏀寔鐢垫簮绠＄悊銆?


### 妯″潡 snd-sc6000


鐢ㄤ簬 Gallant SC-6000 澹板崱鍙婂悗缁瀷鍙凤細SC-6600 鍜?SC-7000 鐨勬ā鍧椼€?

port
    绔彛鍙?#锛?x220 鎴?0x240锛?
mss_port
    MSS 绔彛鍙?#锛?x530 鎴?0xe80锛?
irq
    IRQ #锛?,7,9,10,11锛?
mpu_irq
    MPU-401 IRQ #锛?,7,9,10锛夛紝0 - 鏃?MPU-401 irq
dma
    DMA #锛?,3,0锛?
joystick
    鍚敤娓告垙绔彛 - 0 = 绂佺敤锛堥粯璁わ級锛? = 鍚敤

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

璇ュ０鍗′篃绉颁负 Audio Excel DSP 16 鎴?Zoltrix AV302銆?

### 妯″潡 snd-sscape


鐢ㄤ簬 ENSONIQ SoundScape 澹板崱鐨勬ā鍧椼€?

port
    绔彛鍙?#锛圥nP 璁剧疆锛?
wss_port
    WSS 绔彛鍙?#锛圥nP 璁剧疆锛?
irq
    IRQ #锛圥nP 璁剧疆锛?
mpu_irq
    MPU-401 IRQ #锛圥nP 璁剧疆锛?
dma
    DMA #锛圥nP 璁剧疆锛?
dma2
    绗簩涓?DMA #锛圥nP 璁剧疆锛?1 琛ㄧず绂佺敤锛?
joystick
    鍚敤娓告垙绔彛 - 0 = 绂佺敤锛堥粯璁わ級锛? = 鍚敤

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

璇ラ┍鍔ㄩ渶瑕佸唴鏍告彁渚涘浐浠跺姞杞藉櫒鏀寔銆?

### 妯″潡 snd-sun-amd7930锛堜粎 sparc 涓婏級


鐢ㄤ簬 Sparc 涓婄殑 AMD7930 澹拌姱鐗囩殑妯″潡銆?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-sun-cs4231锛堜粎 sparc 涓婏級


鐢ㄤ簬 Sparc 涓婄殑 CS4231 澹拌姱鐗囩殑妯″潡銆?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-sun-dbri锛堜粎 sparc 涓婏級


鐢ㄤ簬 Sparc 涓婄殑 DBRI 澹拌姱鐗囩殑妯″潡銆?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-wavefront


鐢ㄤ簬 Turtle Beach Maui銆乀ropez 鍜?Tropez+ 澹板崱鐨勬ā鍧椼€?

use_cs4232_midi
    浣跨敤 CS4232 MPU-401 鎺ュ彛
    锛堜綅浜庝綘璁＄畻鏈哄唴閮ㄦ棤娉曡闂殑浣嶇疆锛?
isapnp
    ISA PnP 妫€娴?- 0 = 绂佺敤锛? = 鍚敤锛堥粯璁わ級

鍦?isapnp=0 鏃讹紝鍙娇鐢ㄤ互涓嬮€夐」锛?

cs4232_pcm_port
    CS4232 PCM 鎺ュ彛鐨勭鍙ｅ彿 #
cs4232_pcm_irq
    CS4232 PCM 鎺ュ彛鐨?IRQ #锛?,7,9,11,12,15锛夈€?
cs4232_mpu_port
    CS4232 MPU-401 鎺ュ彛鐨勭鍙ｅ彿 #
cs4232_mpu_irq
    CS4232 MPU-401 鎺ュ彛鐨?IRQ #锛?,11,12,15锛夈€?
ics2115_port
    ICS2115 鐨勭鍙ｅ彿 #
ics2115_irq
    ICS2115 鐨?IRQ #
fm_port
    FM OPL-3 绔彛鍙?#
dma1
    CS4232 PCM 鎺ュ彛鐨?DMA1 #
dma2
    CS4232 PCM 鎺ュ彛鐨?DMA2 #

浠ヤ笅鏄?wavefront_synth 鍔熻兘鐨勯€夐」锛?

wf_raw
    鍋囧畾鎴戜滑闇€瑕佸紩瀵兼搷浣滅郴缁燂紙榛樿锛氬惁锛夛紱
    鑻ヤ负鏄紝鍒欏湪椹卞姩鍔犺浇鏈熼棿蹇界暐鏉垮崱鐘舵€侊紝鏃犺濡備綍
    鎴戜滑閮戒細澶嶄綅鏉垮崱骞跺姞杞藉浐浠躲€?
fx_raw
    鍋囧畾 FX 澶勭悊闇€瑕佸府鍔╋紙榛樿锛氭槸锛夛紱
    鑻ヤ负鍚︼紝鍦ㄩ┍鍔ㄥ姞杞芥椂鎴戜滑灏?FX 澶勭悊鍣ㄤ繚鐣欎负浠绘剰鐘舵€併€?
    榛樿浼氫笅杞藉井绋嬪簭鍙婄浉鍏崇殑绯绘暟锛屽皢鍏惰缃负鈥滈粯璁も€濇搷浣滐紝
    鏃犺閭ｆ剰鍛崇潃浠€涔堛€?
debug_default
    鐢ㄤ簬澹板崱鍒濆鍖栫殑璋冭瘯鍙傛暟
wait_usecs
    鍦ㄤ笉鐫＄湢鐨勬儏鍐典笅绛夊緟澶氶暱鏃堕棿锛屽崟浣嶄负寰锛堥粯璁わ細150锛夛紱
    鍩轰簬鎴戞湁闄愮殑瀹為獙锛岃繖涓瓟鏁颁技涔庤兘缁欏嚭鐩稿綋浼樺寲鐨勫悶鍚愰噺銆?
    濡傛灉浣犳兂灏濊瘯骞舵壘鍒版洿濂界殑鍊硷紝璇烽殢鎰忋€傝浣忥紝瑕佺偣鏄緱鍒颁竴涓?
    璁╂垜浠兘灏藉彲鑳藉鍦板繖绛夊緟 WaveFront 鍛戒护鐨勬暟瀛楋紝鑰屼笉浼氬ぇ鍒?
    闇稿崰鏁翠釜 CPU銆?
    鍏蜂綋鏉ヨ锛屼娇鐢ㄨ繖涓暟瀛楋紝鍦ㄧ害 134,000 娆＄姸鎬佺瓑寰呬腑锛屽彧鏈?
    绾?250 娆″鑷寸潯鐪犮€?
sleep_interval
    绛夊緟鍥炲鏃剁潯鐪犲闀挎椂闂达紙榛樿锛?00锛?
sleep_tries
    鍦ㄤ竴娆＄瓑寰呮湡闂村皾璇曠潯鐪犲灏戞锛堥粯璁わ細50锛?
ospath
    缁忓鐞嗙殑 ICS2115 OS 鍥轰欢鐨勮矾寰勫悕锛堥粯璁わ細wavefront.os锛夛紱
    ISC2115 OS 鍥轰欢鐨勮矾寰勫悕銆傚湪鏈€杩戠殑鐗堟湰涓紝瀹冮€氳繃鍥轰欢鍔犺浇鍣?
    妗嗘灦澶勭悊锛屽洜姝ゅ繀椤诲畨瑁呭湪姝ｇ‘鐨勮矾寰勪腑锛岄€氬父鏄?/lib/firmware銆?
reset_time
    绛夊緟澶嶄綅鐢熸晥澶氶暱鏃堕棿锛堥粯璁わ細2锛?
ramcheck_time
    绛夊緟 RAM 娴嬭瘯澶氬皯绉掞紙榛樿锛?0锛?
osrun_time
    绛夊緟 ICS2115 OS 澶氬皯绉掞紙榛樿锛?0锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰 ISA PnP銆?

娉ㄦ剰锛氭棭鏈熺殑鐗堟湰涓浐浠舵枃浠?`wavefront.os` 浣嶄簬 /etc銆傜幇鍦ㄥ畠閫氳繃
鍥轰欢鍔犺浇鍣ㄥ姞杞斤紝蹇呴』浣嶄簬姝ｇ‘鐨勫浐浠惰矾寰勪腑锛屼緥濡?/lib/firmware銆?
濡傛灉鍦ㄥ崌绾у唴鏍稿悗閬囧埌鏈夊叧鍥轰欢涓嬭浇鐨勯敊璇紝璇烽€傚綋鍦板鍒讹紙鎴栧缓绔?
绗﹀彿閾炬帴锛夎鏂囦欢銆?

### 妯″潡 snd-sonicvibes


鐢ㄤ簬 S3 SonicVibes PCI 澹板崱鐨勬ā鍧椼€?
- PINE Schubert 32 PCI

reverb
    娣峰搷鍚敤 - 1 = 鍚敤锛? = 绂佺敤锛堥粯璁わ級锛?
    澹板崱蹇呴』甯︽湁鏉胯浇 SRAM 鎵嶈兘浣跨敤姝ゅ姛鑳姐€?
mge
    楹﹀厠椋庡鐩婂惎鐢?- 1 = 鍚敤锛? = 绂佺敤锛堥粯璁わ級

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

### 妯″潡 snd-serial-u16550


鐢ㄤ簬 UART16550A 涓茶 MIDI 绔彛鐨勬ā鍧椼€?

port
    UART16550A 鑺墖鐨勭鍙ｅ彿 #
irq
    UART16550A 鑺墖鐨?IRQ #锛?1 = 杞妯″紡
speed
    閫熷害锛屽崟浣嶄负娉㈢壒锛?600,19200,38400,57600,115200锛?
    38400 = 榛樿
base
    娉㈢壒鐜囬櫎鏁板熀鍑嗭紙57600,115200,230400,460800锛?
    115200 = 榛樿
outs
    涓€涓覆琛岀鍙ｄ腑鐨?MIDI 绔彛鏁帮紙1-4锛?
    1 = 榛樿
adaptor
    閫傞厤鍣ㄧ被鍨嬨€?
	0 = Soundcanvas锛? = MS-124T锛? = MS-124W S/A锛?
	3 = MS-124W M/B锛? = 閫氱敤

璇ユā鍧楁敮鎸佸鍧楀０鍗°€傝妯″潡涓嶆敮鎸佽嚜鍔ㄦ帰娴嬶紝鍥犳蹇呴』鎸囧畾涓荤鍙ｏ紒锛侊紒
鍏朵粬閫夐」涓哄彲閫夈€?

### 妯″潡 snd-trident


鐢ㄤ簬 Trident 4DWave DX/NX 澹板崱鐨勬ā鍧椼€?
- Best Union  Miss Melody 4DWave PCI
- HIS  4DWave PCI
- Warpspeed  ONSpeed 4DWave PCI
- AzTech  PCI 64-Q3D
- Addonics  SV 750
- CHIC  True Sound 4Dwave
- Shark  Predator4D-PCI
- Jaton  SonicWave 4D
- SiS SI7018 PCI Audio
- Hoontech SoundTrack Digital 4DWave NX

pcm_channels
    涓?PCM 淇濈暀鐨勬渶澶ч€氶亾鏁帮紙璇煶鏁帮級
wavetable_size
    鏈€澶ф尝琛ㄥぇ灏忥紝浠?kB 涓哄崟浣嶏紙4-?kb锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗″拰鑷姩鎺㈡祴銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-ua101


鐢ㄤ簬 Edirol UA-101/UA-1000 闊抽/MIDI 鎺ュ彛鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸璁惧銆佽嚜鍔ㄦ帰娴嬪拰鐑彃鎷斻€?

### 妯″潡 snd-usb-audio


鐢ㄤ簬 USB 闊抽鍜?USB MIDI 璁惧鐨勬ā鍧椼€?

vid
    璁惧鐨勫巶鍟?ID锛堝彲閫夛級
pid
    璁惧鐨勪骇鍝?ID锛堝彲閫夛級
nrpacks
    姣忎釜 URB 鐨勬渶澶у寘鏁帮紙榛樿锛?锛?
device_setup
    璁惧鐗瑰畾鐨勯瓟鏁帮紙鍙€夛級锛?
    褰卞搷鍙栧喅浜庤澶?
    榛樿鍊硷細0x0000
ignore_ctl_error
    蹇界暐浠讳綍鏈夊叧娣烽煶鍣ㄦ帴鍙ｇ殑 USB 鎺у埗鍣ㄩ敊璇紙榛樿锛氬惁锛?
    `ignore_ctl_error=1` 鍙兘鍦ㄤ綘璁块棶娣烽煶鍣ㄥ厓绱狅紙濡?URB error -22锛?
    鏃堕亣鍒伴敊璇湁鎵€甯姪銆傝繖鍙戠敓鍦ㄦ煇浜涙湁缂洪櫡鐨?USB 璁惧鎴栨帶鍒跺櫒涓娿€?
    姝よ閬挎柟妗堜篃瀵瑰簲 `quirk_flags` 鐨勭 14 浣嶃€?
autoclock
    涓?UAC2 璁惧鍚敤鑷姩鏃堕挓閫夋嫨锛堥粯璁わ細鏄級
lowlatency
    鍚敤浣庡欢杩熸挱鏀炬ā寮忥紙榛樿锛氭槸锛夈€?
    濡傛灉閬囧埌鍥炲綊闂锛屽彲灏嗗叾鍏抽棴浠ュ垏鍥炴棫妯″紡銆?
quirk_alias
    寮傚父鍒悕鍒楄〃锛屼紶鍏ョ被浼?`0123abcd:5678beef` 鐨勫瓧绗︿覆锛屽皢璁惧
    5678:beef 涓婂凡鏈夌殑寮傚父搴旂敤鍒颁竴涓柊璁惧 0123:abcd銆?
implicit_fb
    搴旂敤閫氱敤鐨勯殣寮忓弽棣堝悓姝ユā寮忋€傚綋姝ら€夐」琚缃笖鎾斁娴佸悓姝ユā寮忎负
    ASYNC 鏃讹紝椹卞姩浼氬皾璇曞皢涓€涓浉閭荤殑 ASYNC 閲囬泦娴佺粦瀹氫负闅愬紡鍙嶉婧愩€?
    杩欑瓑浠蜂簬 quirk_flags 鐨勭 17 浣嶃€?
use_vmalloc
    浣跨敤 vmalloc() 鍒嗛厤 PCM 缂撳啿鍖猴紙榛樿锛氭槸锛夈€?
    瀵逛簬鍍?ARM 鎴?MIPS 杩欐牱鍏锋湁闈炰竴鑷存€у唴瀛樼殑鏋舵瀯锛宮map 璁块棶浣跨敤
    vmalloc 鍒嗛厤鐨勭紦鍐插尯鍙兘浜х敓涓嶄竴鑷寸殑缁撴灉銆傚鏋滃湪姝ょ被鏋舵瀯涓婁娇鐢?
    mmap锛岃鍏抽棴姝ら€夐」锛岃繖鏍蜂細鍒嗛厤骞朵娇鐢?DMA 涓€鑷存€х紦鍐插尯銆?
delayed_register
    璇ラ€夐」鐢ㄤ簬鍏锋湁鍦ㄥ涓?USB 鎺ュ彛涓畾涔夌殑澶氫釜娴佺殑璁惧銆傞┍鍔ㄥ彲鑳?
    浼氬娆★紙姣忎釜鎺ュ彛涓€娆★級杩涜娉ㄥ唽锛岃繖鍙兘瀵艰嚧璁惧鏋氫妇涓嶅畬鏁淬€?
    璇ラ€夐」鎺ユ敹涓€涓瓧绗︿覆鏁扮粍锛屼綘鍙互浼犲叆绫讳技 `0123abcd:4` 鐨?
    ID:INTERFACE 鏉ユ墽琛屽璇ョ粰瀹氳澶囩殑寤惰繜娉ㄥ唽銆傚湪姝や緥涓紝褰撴帰娴嬪埌
    USB 璁惧 0123:abcd 鏃讹紝椹卞姩浼氱瓑寰?USB 鎺ュ彛 4 琚帰娴嬪悗鎵嶆敞鍐屻€?
    瀵规绫昏澶囷紝椹卞姩浼氭墦鍗扮被浼?鈥淔ound post-registration device
    assignment: 1234abcd:04鈥?鐨勬秷鎭紝浠ヤ究鐢ㄦ埛娉ㄦ剰鍒拌繖涓€闇€瑕併€?
skip_validation
    璺宠繃鍗曞厓鎻忚堪绗︽牎楠岋紙榛樿锛氬惁锛夈€?
    璇ラ€夐」鐢ㄤ簬蹇界暐鍗曞厓鎻忚堪绗︾殑鏍￠獙閿欒锛堜互鍗曞厓鎻忚堪绗︾殑鍗佸叚杩涘埗杞偍
    褰㈠紡锛夛紝鑰屼笉鏄骇鐢熼┍鍔ㄦ帰娴嬮敊璇紝浠ヤ究鎴戜滑妫€鏌ュ叾缁嗚妭銆?
quirk_flags
    璇ラ€夐」鎻愪緵浜嗙敤浜庡簲鐢ㄥ紓甯告爣蹇楃殑绮剧粏涓旂伒娲荤殑鎺у埗銆傚畠鍏佽涓烘瘡涓?
    璁惧鎸囧畾寮傚父鏍囧織锛屽苟涓斿彲浠ラ€氳繃 sysfs 鍔ㄦ€佷慨鏀广€?
    鏃х殑鐢ㄦ硶鎺ュ彈涓€涓暣鏁版暟缁勶紝鍏朵腑姣忎釜鏁存暟鎸夌収鎺㈡祴椤哄簭瀵硅澶囧簲鐢?
    寮傚父鏍囧織銆備緥濡傦紝`quirk_flags=0x01,0x02` 瀵圭涓€涓澶囧簲鐢?
    get_sample_rate锛屽绗簩涓澶囧簲鐢?share_media_device銆?
    鏂扮殑鐢ㄦ硶鎺ュ彈鏍煎紡涓?`VID1:PID1:FLAGS1;VID2:PID2:FLAGS2;...` 鐨?
    瀛楃涓诧紝鍏朵腑 `VIDx` 鍜?`PIDx` 鎸囧畾璁惧锛宍FLAGSx` 鎸囧畾瑕佸簲鐢ㄧ殑
    鏍囧織銆俙VIDx` 鍜?`PIDx` 鏄?4 浣嶅崄鍏繘鍒舵暟锛屽彲浠ユ寚瀹氫负 `*` 浠ュ尮閰?
    浠绘剰鍊笺€俙FLAGSx` 鍙互鏄竴缁勪互 `|` 鍒嗛殧鐨勩€佹寜鍚嶇О缁欏嚭鐨勬爣蹇楋紝鎴?
    琛ㄧず浣嶆爣蹇楃殑鍗佸叚杩涘埗鏁般€傚彲鐢ㄧ殑鏍囧織鍚嶇О濡備笅銆傚彲浠ュ湪鏍囧織鍚嶅墠鍔?
    鍙瑰彿浠ュ璇ユ爣蹇楀彇鍙嶃€?
    渚嬪锛宍1234:abcd:mixer_playback_min_mute|!ignore_ctl_error;**:**:0x01;`
    瀵硅澶?1234:abcd 搴旂敤 `mixer_playback_min_mute` 鏍囧織骞舵竻闄?
    `ignore_ctl_error` 鏍囧織锛屽苟瀵规墍鏈夎澶囧簲鐢?`skip_sample_rate` 鏍囧織銆?

        - 绗?0 浣嶏細`get_sample_rate`
          璺宠繃璇诲彇璁惧鐨勯噰鏍风巼
        - 绗?1 浣嶏細`share_media_device`
          鍒涘缓 Media Controller API 鏉＄洰
        - 绗?2 浣嶏細`align_transfer`
          鍏佽鍦ㄤ紶杈撴椂瀵归煶棰戝瓙鏃堕殭杩涜瀵归綈
        - 绗?3 浣嶏細`tx_length`
          鍦ㄤ紶杈撲腑娣诲姞闀垮害璇存槑绗?
        - 绗?4 浣嶏細`playback_first`
          鍦ㄩ殣寮忓弽棣堟ā寮忎笅棣栧厛鍚姩鎾斁娴?
        - 绗?5 浣嶏細`skip_clock_selector`
          璺宠繃鏃堕挓閫夋嫨鍣ㄨ缃?
        - 绗?6 浣嶏細`ignore_clock_source`
          蹇界暐鏃堕挓婧愭悳绱㈢殑閿欒
        - 绗?7 浣嶏細`itf_usb_dsd_dac`
          琛ㄧず鍩轰簬 ITF-USB DSD 鐨?DAC
        - 绗?8 浣嶏細`ctl_msg_delay`
          鍦ㄦ瘡涓帶鍒舵秷鎭鐞嗘椂娣诲姞 20ms 寤惰繜
        - 绗?9 浣嶏細`ctl_msg_delay_1m`
          鍦ㄦ瘡涓帶鍒舵秷鎭鐞嗘椂娣诲姞 1-2ms 寤惰繜
        - 绗?10 浣嶏細`ctl_msg_delay_5m`
          鍦ㄦ瘡涓帶鍒舵秷鎭鐞嗘椂娣诲姞 5-6ms 寤惰繜
        - 绗?11 浣嶏細`iface_delay`
          鍦ㄦ瘡涓帴鍙ｈ缃椂娣诲姞 50ms 寤惰繜
        - 绗?12 浣嶏細`validate_rates`
          鍦ㄦ帰娴嬫椂鎵ц閲囨牱鐜囨牎楠?
        - 绗?13 浣嶏細`disable_autosuspend`
          绂佺敤杩愯鏃?PM 鑷姩鎸傝捣
        - 绗?14 浣嶏細`ignore_ctl_error`
          蹇界暐娣烽煶鍣ㄨ闂殑閿欒
        - 绗?15 浣嶏細`dsd_raw`
          鏀寔閫氱敤鐨?DSD 鍘熷 U32_BE 鏍煎紡
        - 绗?16 浣嶏細`set_iface_first`
          鍍?UAC1 涓€鏍烽鍏堣缃帴鍙?
        - 绗?17 浣嶏細`generic_implicit_fb`
          搴旂敤閫氱敤鐨勯殣寮忓弽棣堝悓姝ユā寮?
        - 绗?18 浣嶏細`skip_implicit_fb`
          涓嶅簲鐢ㄩ殣寮忓弽棣堝悓姝ユā寮?
        - 绗?19 浣嶏細`iface_skip_close`
          鍦ㄨ缃噰鏍风巼鏈熼棿涓嶅叧闂帴鍙?
        - 绗?20 浣嶏細`force_iface_reset`
          鍦ㄦ瘡娆″仠姝㈠拰閲嶅惎娴佹椂寮哄埗澶嶄綅鎺ュ彛
        - 绗?21 浣嶏細`fixed_rate`
          褰撶粰瀹氱鐐瑰彧鏈変竴涓彲鐢ㄩ€熺巼鏃讹紝涓嶈缃?PCM 閫熺巼锛堥鐜囷級
        - 绗?22 浣嶏細`mic_res_16`
          涓?Mic Capture Volume 璁剧疆鍥哄畾鍒嗚鲸鐜?16
        - 绗?23 浣嶏細`mic_res_384`
          涓?Mic Capture Volume 璁剧疆鍥哄畾鍒嗚鲸鐜?384
        - 绗?24 浣嶏細`mixer_playback_min_mute`
          灏嗘渶灏忛煶閲忔帶鍒跺€艰涓洪潤闊筹紝閫傜敤浜庢渶浣庢挱鏀惧€艰〃绀洪潤闊崇姸鎬?
          鑰岄潪鏈€灏忓彲鍚煶閲忕殑璁惧
        - 绗?25 浣嶏細`mixer_capture_min_mute`
          绫讳技浜庣 24 浣嶏紝浣嗙敤浜庨噰闆嗘祦
        - 绗?26 浣嶏細`skip_iface_setup`
          璺宠繃鎺㈡祴鏃剁殑鎺ュ彛璁剧疆锛坲sb_set_interface銆乮nit_pitch銆?
          init_sample_rate锛夛紱涓庢祦鎵撳紑鏃剁殑 snd_usb_endpoint_prepare()
          閲嶅
        - 绗?27 浣嶏細`mixer_playback_linear_vol`
          涓烘挱鏀鹃煶閲忔帶鍒跺€肩嚎鎬ф槧灏勫埌鐢靛帇锛堣€岄潪 dB锛夋按骞崇殑璁惧璁剧疆
          绾挎€ч煶閲忔槧灏勩€傜畝鑰岃█涔嬶細`x(raw) = (raw - raw_min) / (raw_max - raw_min)`锛?
          `V(x) = k ** x`锛沗dB(x) = 20 ** log10(x)`銆傝鐩栫 24 浣?
        - 绗?28 浣嶏細`mixer_capture_linear_vol`
          绫讳技浜庣 27 浣嶏紝浣嗙敤浜庨噰闆嗘祦銆傝鐩栫 25 浣?

璇ユā鍧楁敮鎸佸璁惧銆佽嚜鍔ㄦ帰娴嬪拰鐑彃鎷斻€?

娉ㄦ剰锛歚nrpacks` 鍙傛暟鍙互閫氳繃 sysfs 鍔ㄦ€佷慨鏀广€備笉瑕佸皢璇ュ€艰寰楄秴杩?20銆?
閫氳繃 sysfs 淇敼涓嶈繘琛屽仴鍏ㄦ€ф鏌ャ€?

娉ㄦ剰锛歚ignore_ctl_error=1` 鍙槸鎻愪緵浜嗕竴绉嶅揩閫熺粫杩囬棶棰樼殑鏂规硶銆傚鏋滀綘
鏈夐渶瑕佽繖浜涘紓甯哥殑鏈夌己闄疯澶囷紝璇峰悜涓婃父鎶ュ憡銆?

娉ㄦ剰锛歚quirk_alias` 閫夐」浠呯敤浜庢祴璇?寮€鍙戙€傚鏋滀綘甯屾湜鑾峰緱閫傚綋鐨勬敮鎸侊紝
璇疯仈绯讳笂娓革紝鍦ㄩ┍鍔ㄤ唬鐮佷腑闈欐€佹坊鍔犲尮閰嶇殑寮傚父銆俙quirk_flags` 鍚岀悊銆傚鏋?
鏌愯澶囧凡鐭ラ渶瑕佺壒瀹氱殑瑙勯伩鏂规锛岃鍚戜笂娓告姤鍛娿€?

### 妯″潡 snd-usb-caiaq


鐢ㄤ簬 caiaq USB 闊抽鎺ュ彛鐨勬ā鍧楋細

- Native Instruments RigKontrol2
- Native Instruments Kore Controller
- Native Instruments Audio Kontrol 1
- Native Instruments Audio 8 DJ

璇ユā鍧楁敮鎸佸璁惧銆佽嚜鍔ㄦ帰娴嬪拰鐑彃鎷斻€?

### 妯″潡 snd-usb-usx2y


鐢ㄤ簬 Tascam USB US-122銆乁S-224 鍜?US-428 璁惧鐨勬ā鍧椼€?

璇ユā鍧楁敮鎸佸璁惧銆佽嚜鍔ㄦ帰娴嬪拰鐑彃鎷斻€?

娉ㄦ剰锛氫綘闇€瑕侀€氳繃 alsa-tools 鍜?alsa-firmware 杞欢鍖呬腑鍖呭惈鐨?
`usx2yloader` 宸ュ叿鍔犺浇鍥轰欢銆?

### 妯″潡 snd-via82xx


鐢ㄤ簬鍩轰簬 VIA 82C686A/686B銆?233銆?233A銆?233C銆?235銆?237
锛堝崡妗ワ級鐨?AC'97 涓绘澘鐨勬ā鍧椼€?

mpu_port
    0x300,0x310,0x320,0x330锛屽惁鍒欎粠 BIOS 璁剧疆鑾峰彇
    [浠?VIA686A/686B]
joystick
    鍚敤娓告垙鏉嗭紙榛樿鍏抽棴锛塠浠?VIA686A/686B]
ac97_clock
    AC'97 缂栬В鐮佸櫒鏃堕挓鍩哄噯锛堥粯璁?48000Hz锛?
dxs_support
    鏀寔 DXS 閫氶亾锛? = 鑷姩锛堥粯璁わ級锛? = 鍚敤锛? = 绂佺敤锛?
    3 = 浠?48k锛? = 鏃?VRA锛? = 鍚敤浠绘剰閲囨牱鐜囦笖涓嶅悓閫氶亾浣跨敤
    涓嶅悓鐨勯噰鏍风巼 [浠?VIA8233/C銆?235銆?237]
ac97_quirk
    閽堝寮傚父纭欢鐨?AC'97 瑙勯伩鏂规锛?
    瑙佷笅闈㈢殑 `AC97 Quirk Option`_ 灏忚妭銆?

璇ユā鍧楁敮鎸佸崟鑺墖鍜岃嚜鍔ㄦ帰娴嬨€?

娉ㄦ剰锛氬湪鏌愪簺 SMP 涓绘澘锛堝 MSI 694D锛変笂锛屼腑鏂彲鑳芥棤娉曟纭敓鎴愩€?
鍦ㄨ繖绉嶆儏鍐典笅锛岃灏濊瘯灏?BIOS 涓婄殑 SMP锛堟垨 MPS锛夌増鏈涓?1.1 鑰屼笉鏄?
榛樿鍊?1.4銆傝繖鏍蜂腑鏂彿灏嗚鍒嗛厤鍦?15 浠ヤ笅銆備綘涔熷彲浠ュ崌绾т綘鐨?BIOS銆?

娉ㄦ剰锛歏IA8233/5/7锛堥潪 VIA8233A锛夊彲浠ュ皢 DXS锛坉irect sound锛夐€氶亾鏀寔涓?
绗竴涓?PCM銆傚湪杩欎簺閫氶亾涓婏紝鏈€澶氬彲鍚屾椂鎾斁 4 涓祦锛屼笖鎺у埗鍣ㄥ彲浠ュ姣忎釜
閫氶亾浠ョ嫭绔嬬殑閫熺巼鎵ц閲囨牱鐜囪浆鎹€?
榛樿鎯呭喌涓嬶紙`dxs_support = 0`锛夛紝闄ゅ凡鐭ヨ澶囧锛岄€夋嫨鍥哄畾鐨?48k 閫熺巼锛?
鍥犱负鍦ㄦ煇浜涗富鏉夸笂锛岀敱浜?BIOS 缂洪櫡锛岄櫎 48k 澶栬緭鍑哄線寰€鏈夋潅闊炽€?
璇峰厛灏濊瘯涓€娆?`dxs_support=5`锛屽鏋滃畠鍦ㄥ叾浠栭噰鏍风巼锛堜緥濡?mp3 鎾斁鐨?
44.1kHz锛変笅宸ヤ綔锛岃灏?PCI 瀛愮郴缁熷巶鍟?璁惧 ID锛堝嵆 `lspci -nv` 鐨勮緭鍑猴級
鍛婅瘔鎴戜滑銆?
濡傛灉 `dxs_support=5` 涓嶅伐浣滐紝灏濊瘯 `dxs_support=4`锛涘鏋滆繕涓嶅伐浣滐紝灏濊瘯
dxs_support=1銆傦紙dxs_support=1 閫氬父鐢ㄤ簬鏃т富鏉裤€傛纭疄鐜扮殑鏉垮崱搴旇
鑳藉湪 4 鎴?5 涓嬪伐浣溿€傦級濡傛灉浠嶇劧涓嶅伐浣滐紝鑰岄粯璁よ缃彲浠ワ紝鍒?`dxs_support=3`
鏄纭€夋嫨銆傚鏋滈粯璁よ缃牴鏈笉宸ヤ綔锛屽皾璇?`dxs_support=2` 鏉ョ鐢?DXS 閫氶亾銆?
鍦ㄤ换浣曟儏鍐典笅锛岃灏嗙粨鏋滃拰瀛愮郴缁熷巶鍟?璁惧 ID 鍛婅瘔鎴戜滑銆傝涓嬮潰鐨?
`Links and Addresses`_銆?

娉ㄦ剰锛氬浜?VIA823x 涓婄殑 MPU401锛岃鍙﹀浣跨敤 snd-mpu401 椹卞姩銆俶pu_port
閫夐」浠呯敤浜?VIA686 鑺墖銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-via82xx-modem


鐢ㄤ簬 VIA82xx AC97 璋冨埗鍣ㄧ殑妯″潡銆?

ac97_clock
    AC'97 缂栬В鐮佸櫒鏃堕挓鍩哄噯锛堥粯璁?48000Hz锛?

璇ユā鍧楁敮鎸佸崟鍧楀０鍗″拰鑷姩鎺㈡祴銆?

娉ㄦ剰锛氳妯″潡鐨勯粯璁?index 鍊间负 -2锛屽嵆绗竴涓Ы浣嶈鎺掗櫎銆?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-virmidi


鐢ㄤ簬铏氭嫙 rawmidi 璁惧鐨勬ā鍧椼€?
璇ユā鍧楀垱寤轰笌鐩稿簲 ALSA 闊冲簭鍣ㄧ鍙ｉ€氫俊鐨勮櫄鎷?rawmidi 璁惧銆?

midi_devs
    MIDI 璁惧鏁?#锛?-4锛岄粯璁?4锛?

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

### 妯″潡 snd-virtuoso


鐢ㄤ簬鍩轰簬 Asus AV66/AV100/AV200 鑺墖鐨勫０鍗＄殑妯″潡锛屽嵆 Xonar D1銆丏X銆?
D2銆丏2X銆丏S銆丏SX銆丒ssence ST锛圖eluxe锛夈€丒ssence STX锛圛I锛夈€丠DAV1.3
锛圖eluxe锛夊拰 HDAV1.3 Slim銆?

璇ユā鍧楁敮鎸佽嚜鍔ㄦ帰娴嬪拰澶氬潡澹板崱銆?

### 妯″潡 snd-vx222


鐢ㄤ簬 Digigram VX-Pocket VX222銆乂222 v2 鍜?Mic 澹板崱鐨勬ā鍧椼€?

mic
    鍦?V222 Mic 涓婂惎鐢ㄩ害鍏嬮锛圢YI锛?
ibl
    閲囬泦 IBL 澶у皬銆傦紙榛樿 = 0锛屾渶灏忓ぇ灏忥級

璇ユā鍧楁敮鎸佸鍧楀０鍗°€?

褰撻┍鍔ㄧ紪璇戜负妯″潡涓旀敮鎸?hotplug 鍥轰欢鏃讹紝鍥轰欢鏁版嵁浼氶€氳繃 hotplug 鑷姩
鍔犺浇銆傝鍦?alsa-firmware 杞欢鍖呬腑瀹夎鎵€闇€鐨勫浐浠舵枃浠躲€傚綋娌℃湁鍙敤鐨?
hotplug 鍥轰欢鍔犺浇鍣ㄦ椂锛屼綘闇€瑕侀€氳繃 alsa-tools 杞欢鍖呬腑鐨?vxloader 宸ュ叿
鍔犺浇鍥轰欢銆傝鑷姩璋冪敤 vxloader锛岃灏嗕互涓嬪唴瀹规坊鍔犲埌
/etc/modprobe.d/alsa.conf锛?

```

  install snd-vx222 /sbin/modprobe --first-time -i snd-vx222\
    && /usr/bin/vxloader


```
锛堝浜?2.2/2.4 鍐呮牳锛屾敼涓哄皢 `post-install /usr/bin/vxloader` 娣诲姞鍒?
/etc/modules.conf銆傦級
IBL 澶у皬瀹氫箟浜?PCM 鐨勪腑鏂懆鏈熴€傛洿灏忕殑澶у皬甯︽潵鏇翠綆鐨勫欢杩燂紝浣嗕篃浼氬鑷?
鏇村鐨?CPU 娑堣€椼€傝澶у皬閫氬父瀵归綈鍒?126銆傞粯璁わ紙=0锛夋椂閫夋嫨鏈€灏忕殑澶у皬銆?
鍙兘鐨?IBL 鍊煎彲浠ュ湪 /proc/asound/cardX/vx-status proc 鏂囦欢涓壘鍒般€?

鏀寔鐢垫簮绠＄悊銆?


### 妯″潡 snd-vxpocket


鐢ㄤ簬 Digigram VX-Pocket VX2 鍜?440 PCMCIA 澹板崱鐨勬ā鍧椼€?

ibl
    閲囬泦 IBL 澶у皬銆傦紙榛樿 = 0锛屾渶灏忓ぇ灏忥級

璇ユā鍧楁敮鎸佸鍧楀０鍗°€傝妯″潡浠呭湪璁剧疆浜?PCMCIA 鏀寔鐨勫唴鏍镐腑鎵嶈缂栬瘧銆?

鍦ㄨ緝鏃х殑 2.6.x 鍐呮牳涓婏紝瑕侀€氳繃鍗＄鐞嗗櫒婵€娲婚┍鍔紝浣犻渶瑕佽缃?
/etc/pcmcia/vxpocket.conf銆傚弬瑙?sound/pcmcia/vx/vxpocket.c銆?.6.13 鎴?
鏇存柊鐨勫唴鏍镐笉鍐嶉渶瑕侀厤缃枃浠躲€?

褰撻┍鍔ㄧ紪璇戜负妯″潡涓旀敮鎸?hotplug 鍥轰欢鏃讹紝鍥轰欢鏁版嵁浼氶€氳繃 hotplug 鑷姩
鍔犺浇銆傝鍦?alsa-firmware 杞欢鍖呬腑瀹夎鎵€闇€鐨勫浐浠舵枃浠躲€傚綋娌℃湁鍙敤鐨?
hotplug 鍥轰欢鍔犺浇鍣ㄦ椂锛屼綘闇€瑕侀€氳繃 alsa-tools 杞欢鍖呬腑鐨?vxloader 宸ュ叿
鍔犺浇鍥轰欢銆?

鍏充簬閲囬泦 IBL锛岃鍙傝 snd-vx222 妯″潡鐨勬弿杩般€?

娉ㄦ剰锛氳嚜 ALSA 1.0.10 璧凤紝snd-vxp440 椹卞姩宸插悎骞跺埌 snd-vxpocket 椹卞姩涓€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-ymfpci


鐢ㄤ簬 Yamaha PCI 鑺墖锛圷MF72x銆乊MF74x 鍜?YMF75x锛夌殑妯″潡銆?

mpu_port
    0x300,0x330,0x332,0x334锛岄粯璁?0锛堢鐢級锛?
    1锛堜粎 YMF744/754 鑷姩鎺㈡祴锛?
fm_port
    0x388,0x398,0x3a0,0x3a8锛岄粯璁?0锛堢鐢級
    1锛堜粎 YMF744/754 鑷姩鎺㈡祴锛?
joystick_port
    0x201,0x202,0x204,0x205锛岄粯璁?0锛堢鐢級锛?
    1锛堣嚜鍔ㄦ帰娴嬶級
rear_switch
    鍚敤鍏变韩鐨勫悗缃?绾胯矾杈撳叆寮€鍏筹紙bool锛?

璇ユā鍧楁敮鎸佽嚜鍔ㄦ帰娴嬪拰澶氳姱鐗囥€?

鏀寔鐢垫簮绠＄悊銆?

### 妯″潡 snd-pdaudiocf


鐢ㄤ簬 Sound Core PDAudioCF 澹板崱鐨勬ā鍧椼€?

鏀寔鐢垫簮绠＄悊銆?


## AC97 纭欢寮傚父瑙勯伩閫夐」


ac97_quirk 閫夐」鐢ㄤ簬涓烘澘杞?AC'97 鎺у埗鍣紙濡?snd-intel8x0锛夐┍鍔ㄤ笂鐨?
鐗瑰畾璁惧鍚敤/瑕嗙洊瑙勯伩鏂规銆傛煇浜涚‖浠舵妸 Master 鍜?Headphone 鎴?Surround
涔嬮棿鐨勮緭鍑哄紩鑴氭帴鍙嶄簡锛堣繖瑕佸綊鍔熶簬 AC'97 瑙勮寖鍦ㄥ悇涓増鏈箣闂寸殑娣蜂贡 :-锛?

椹卞姩鎻愪緵浜嗗宸茬煡闂璁惧鐨勮嚜鍔ㄦ帰娴嬶紝浣嗘湁浜涘彲鑳芥湭鐭ユ垨琚敊璇帰娴嬨€?
鍦ㄨ繖绉嶆儏鍐典笅锛岃閫氳繃姝ら€夐」浼犲叆姝ｇ‘鐨勫€笺€?

鎺ュ彈浠ヤ笅瀛楃涓诧細

default
    涓嶈鐩栭粯璁よ缃?
none
    绂佺敤寮傚父瑙勯伩
hp_only
    灏?Master 鍜?Headphone 鎺у埗缁戝畾涓哄崟涓€鎺у埗
swap_hp
    浜ゆ崲鑰虫満鍜屼富鎺у埗
swap_surround
    浜ゆ崲涓诲拰鐜粫鎺у埗
ad_sharing
    瀵逛簬 AD1985锛屽紑鍚?OMS 浣嶅苟浣跨敤鑰虫満
alc_jack
    瀵逛簬 ALC65x锛屽紑鍚彃瀛旀娴嬫ā寮?
inv_eapd
    鍙嶈浆鐨?EAPD 瀹炵幇
mute_led
    缁戝畾 EAPD 浣嶄互寮€鍚?鍏抽棴闈欓煶 LED

涓轰簡鍚戝悗鍏煎锛岀浉搴旂殑鏁存暟鍊?-1銆? 绛変篃琚帴鍙椼€?

渚嬪锛屽鏋?`Master` 闊抽噺鎺у埗瀵逛綘鐨勮澶囨棤鏁堬紝鑰屽彧鏈?`Headphone` 鏈夋晥锛?
璇蜂紶鍏?ac97_quirk=hp_only 妯″潡閫夐」銆?


## 閰嶇疆闈?ISAPNP 澹板崱


褰撳唴鏍搁厤缃簡 ISA-PnP 鏀寔鏃讹紝鏀寔 isapnp 澹板崱鐨勬ā鍧椾細鏈?`isapnp`
妯″潡閫夐」銆傚鏋滆缃簡姝ら€夐」锛屽皢**鍙?*鎺㈡祴 ISA-PnP 璁惧銆傝鎺㈡祴闈?
ISA-PnP 澹板崱锛屼綘蹇呴』浼犲叆 `isapnp=0` 閫夐」浠ュ強姝ｇ‘鐨?I/O 鍜?irq 閰嶇疆銆?

褰撳唴鏍告湭閰嶇疆 ISA-PnP 鏀寔鏃讹紝isapnp 閫夐」灏嗕笉浼氳缂栬瘧杩涘幓銆?


## 妯″潡鑷姩鍔犺浇鏀寔


ALSA 椹卞姩鍙互閫氳繃瀹氫箟妯″潡鍒悕鎸夐渶鑷姩鍔犺浇銆傚浜?ALSA 鍘熺敓璁惧锛屼細璇锋眰
瀛楃涓?`snd-card-%1`锛屽叾涓?`%i` 鏄粠 0 鍒?7 鐨勫０鍗″彿銆?

瑕佷负 OSS 鏈嶅姟鑷姩鍔犺浇 ALSA 椹卞姩锛岃瀹氫箟瀛楃涓?`sound-slot-%i`锛屽叾涓?
`%i` 琛ㄧず OSS 鐨勬Ы浣嶅彿锛屽畠瀵瑰簲 ALSA 鐨勫０鍗＄储寮曘€傞€氬父锛屽皢鍏跺畾涔変负
鍚屼竴澹板崱妯″潡銆?

鍗曞潡 emu10k1 澹板崱鐨勭ず渚嬮厤缃涓嬶細
```

    ----- /etc/modprobe.d/alsa.conf
    alias snd-card-0 snd-emu10k1
    alias sound-slot-0 snd-emu10k1
    ----- /etc/modprobe.d/alsa.conf

```
鍙嚜鍔ㄥ姞杞界殑澹板崱鏁伴噺鍙栧喅浜?snd 妯″潡鐨?`cards_limit` 妯″潡閫夐」銆傞粯璁?
璁句负 1銆傝鍚敤澶氬潡澹板崱鐨勮嚜鍔ㄥ姞杞斤紝璇峰湪璇ラ€夐」涓寚瀹氬０鍗℃暟閲忋€?

褰撴湁澶氬潡澹板崱鍙敤鏃讹紝鏈€濂戒篃閫氳繃妯″潡閫夐」涓烘瘡鍧楀０鍗℃寚瀹?index 鍙凤紝浠ヤ究
澹板崱鐨勯『搴忎繚鎸佷竴鑷淬€?

涓ゅ潡澹板崱鐨勭ず渚嬮厤缃涓嬶細
```

    ----- /etc/modprobe.d/alsa.conf
    # ALSA 閮ㄥ垎
    options snd cards_limit=2
    alias snd-card-0 snd-interwave
    alias snd-card-1 snd-ens1371
    options snd-interwave index=0
    options snd-ens1371 index=1
    # OSS/Free 閮ㄥ垎
    alias sound-slot-0 snd-interwave
    alias sound-slot-1 snd-ens1371
    ----- /etc/modprobe.d/alsa.conf

```
鍦ㄦ渚嬩腑锛宨nterwave 澹板崱濮嬬粓浣滀负绗竴鍧楀０鍗★紙index 0锛夊姞杞斤紝ens1371
浣滀负绗簩鍧楋紙index 1锛夈€?

鍙︿竴绉嶏紙杈冩柊鐨勶級鍥哄畾妲戒綅鍒嗛厤鐨勬柟娉曟槸浣跨敤 snd 妯″潡鐨?`slots` 閫夐」銆?
瀵逛簬涓婇潰鐨勪緥瀛愶紝鎸夊涓嬫柟寮忔寚瀹氾細
```

    options snd slots=snd-interwave,snd-ens1371

```
杩欐牱锛岀涓€涓Ы浣嶏紙#0锛変繚鐣欑粰 snd-interwave 椹卞姩锛岀浜屼釜锛?1锛変繚鐣欑粰
snd-ens1371銆傚鏋滀娇鐢?slots 閫夐」锛屽彲浠ョ渷鐣ユ瘡涓┍鍔ㄤ腑鐨?index 閫夐」
锛堜笉杩囧彧瑕佷笉鍐茬獊锛屼篃鍙互鍚屾椂淇濈暀锛夈€?

slots 閫夐」瀵逛簬閬垮厤鍙兘鐨勭儹鎻掓嫈鍙婂叾瀵艰嚧鐨勬Ы浣嶅啿绐佺壒鍒湁鐢ㄣ€備緥濡傦紝鍐嶆
鑰冭檻涓婇潰鐨勪緥瀛愶紝鍓嶄袱涓Ы浣嶅凡琚繚鐣欍€傚鏋滄湁浠讳綍鍏朵粬椹卞姩锛堜緥濡?
snd-usb-audio锛夊湪 snd-interwave 鎴?snd-ens1371 涔嬪墠鍔犺浇锛屽畠灏嗚鍒嗛厤鍒?
绗笁涓垨鏇村悗鐨勬Ы浣嶃€?

褰撴ā鍧楀悕浠?'!' 缁欏嚭鏃讹紝璇ユЫ浣嶅皢淇濈暀缁欓櫎璇ュ悕绉板鐨勪换浣曟ā鍧椼€備緥濡傦紝
`slots=!snd-pcsp` 灏嗘妸绗竴涓Ы浣嶄繚鐣欑粰闄?snd-pcsp 澶栫殑浠讳綍妯″潡銆?


## ALSA PCM 璁惧鍒?OSS 璁惧鐨勬槧灏?


```

    /dev/snd/pcmC0D0[c|p]  -> /dev/audio0 (/dev/audio) -> minor 4
    /dev/snd/pcmC0D0[c|p]  -> /dev/dsp0 (/dev/dsp)     -> minor 3
    /dev/snd/pcmC0D1[c|p]  -> /dev/adsp0 (/dev/adsp)   -> minor 12
    /dev/snd/pcmC1D0[c|p]  -> /dev/audio1              -> minor 4+16 = 20
    /dev/snd/pcmC1D0[c|p]  -> /dev/dsp1                -> minor 3+16 = 19
    /dev/snd/pcmC1D1[c|p]  -> /dev/adsp1               -> minor 12+16 = 28
    /dev/snd/pcmC2D0[c|p]  -> /dev/audio2              -> minor 4+32 = 36
    /dev/snd/pcmC2D0[c|p]  -> /dev/dsp2                -> minor 3+32 = 39
    /dev/snd/pcmC2D1[c|p]  -> /dev/adsp2               -> minor 12+32 = 44

```
`/dev/snd/pcmC{X}D{Y}[c|p]` 琛ㄨ揪寮忎腑鐨勭涓€涓暟瀛楄〃绀哄０鍗″彿锛岀浜屼釜
琛ㄧず璁惧鍙枫€侫LSA 璁惧甯︽湁 `c` 鎴?`p` 鍚庣紑锛屽垎鍒〃绀烘柟鍚戯細閲囬泦鍜屾挱鏀俱€?

璇锋敞鎰忥紝涓婅堪璁惧鏄犲皠鍙兘浼氶€氳繃 snd-pcm-oss 妯″潡鐨勬ā鍧楅€夐」鑰屾敼鍙樸€?


## Proc 鎺ュ彛锛?proc/asound锛?


### /proc/asound/card#/pcm#[cp]/oss


erase
    鎿﹂櫎鍏充簬 OSS 搴旂敤绋嬪簭鐨勬墍鏈夐檮鍔犱俊鎭?

<app_name> <fragments> <fragment_size> [<options>]
    <app_name>
	甯︼紙杈冮珮浼樺厛绾э級鎴栦笉甯﹁矾寰勭殑搴旂敤绋嬪簭鍚嶇О
    <fragments>
	 鍒嗙墖鏁帮紝鑷姩鍒欎负 0
    <fragment_size>
	 鍒嗙墖澶у皬锛堝瓧鑺傦級锛岃嚜鍔ㄥ垯涓?0
    <options>
	鍙€夊弬鏁?

	disable
	    搴旂敤绋嬪簭灏濊瘯涓烘閫氶亾鎵撳紑涓€涓?pcm 璁惧锛屼絾涓嶆兂浣跨敤瀹冦€?
	    锛堝洜 bug 鎴栭渶瑕?mmap锛?
	    杩欏 Quake 绛夌▼搴忓緢鏈夌敤鈥︹€?
	direct
	    涓嶄娇鐢ㄦ彃浠?
	block
	    寮哄埗鍧楁ā寮忥紙rvplayer锛?
	non-block
	    寮哄埗闈炲潡妯″紡
	whole-frag
	    鍙啓鍏ユ暣涓垎鐗囷紙浠呭奖鍝嶆挱鏀剧殑浼樺寲锛?
	no-silence
	    涓嶉鍏堝～鍏呴潤闊充互閬垮厤鍜斿棐澹?
	buggy-ptr
	    鍦?GETOPTR ioctl 涓繑鍥炵┖鐧藉潡鑰屼笉鏄凡濉厖鐨勫潡

绀轰緥锛?
```

    echo "x11amp 128 16384" > /proc/asound/card0/pcm0p/oss
    echo "squake 0 0 disable" > /proc/asound/card0/pcm0c/oss
    echo "rvplayer 0 0 block" > /proc/asound/card0/pcm0p/oss


```
## 鏃╂湡缂撳啿鍖哄垎閰?


鏌愪簺椹卞姩锛堜緥濡?hdsp锛夐渶瑕佸ぇ鐨勮繛缁紦鍐插尯锛岃€岀敱浜庡唴瀛樼鐗囷紝鏈夋椂鍦?
椹卞姩妯″潡瀹為檯鍔犺浇鏃跺啀瀵绘壘杩欐牱鐨勭┖闂村凡缁忓お杩熴€備綘鍙互閫氳繃鎻愬墠鍔犺浇
snd-page-alloc 妯″潡骞跺悜鍏?proc 鏂囦欢鍐欏叆鍛戒护鏉ラ鍒嗛厤 PCM 缂撳啿鍖猴紝
渚嬪鍦ㄦ棭鏈熷惎鍔ㄩ樁娈碉紙濡?`/etc/init.d/*.local` 鑴氭湰锛夎繘琛屻€?

璇诲彇 proc 鏂囦欢 /proc/drivers/snd-page-alloc 浼氭樉绀哄綋鍓嶉〉闈㈠垎閰嶇殑
浣跨敤鎯呭喌銆傚湪鍐欏叆鏃讹紝浣犲彲浠ュ悜 snd-page-alloc 椹卞姩鍙戦€佷互涓嬪懡浠わ細

- add VENDOR DEVICE MASK SIZE BUFFERS

VENDOR 鍜?DEVICE 鏄?PCI 鍘傚晢鍜岃澶?ID銆傚畠浠彇鏁存暟锛堝崄鍏繘鍒堕渶瑕?
0x 鍓嶇紑锛夈€侻ASK 鏄?PCI DMA 鎺╃爜銆傚鏋滀笉闄愬埗鍒欎紶 0銆係IZE 鏄鍒嗛厤鐨?
姣忎釜缂撳啿鍖虹殑澶у皬銆備綘鍙互涓?KB 鍜?MB 浣跨敤 k 鍜?m 鍚庣紑銆傛渶澶ф暟閲忎负 16MB銆?
BUFFERS 鏄鍒嗛厤鐨勭紦鍐插尯鏁伴噺銆傚畠蹇呴』澶т簬 0銆傛渶澶ф暟閲忎负 4銆?

- erase

杩欏皢鎿﹂櫎鎵€鏈夋湭鍦ㄤ娇鐢ㄤ腑鐨勯鍒嗛厤缂撳啿鍖恒€?


## 閾炬帴涓庡湴鍧€


ALSA 椤圭洰涓婚〉
    http://www.alsa-project.org
Kernel Bugzilla
    http://bugzilla.kernel.org/
ALSA 寮€鍙戣€呴偖浠跺垪琛?
    mailto:alsa-devel@alsa-project.org
alsa-info.sh 鑴氭湰
    https://www.alsa-project.org/alsa-info.sh
