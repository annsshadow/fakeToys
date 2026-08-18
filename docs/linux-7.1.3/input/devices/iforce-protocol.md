锘?# Iforce 鍗忚


:浣滆€? Johann Deneux <johann.deneux@gmail.com>

涓婚〉浣嶄簬 `<http://web.archive.org/web/*/http://www.esil.univ-mrs.fr>`_

:琛ュ厖: 鐢?Vojtech Pavlik 娣诲姞銆?

## 绠€浠?

鏈枃妗ｆ弿杩颁簡鎴戣娉曞彂鐜扮殑銆佺敤浜庡悜 I-Force 2.0 璁惧鎸囧畾鍔涘弽棣堟晥鏋滐紙force effects锛夌殑鍗忚淇℃伅銆傝繖浜涗俊鎭潎闈炴潵鑷?Immerse锛?Immersion 鍏徃锛夈€傚洜姝わ紝浣犱笉搴旇交淇℃湰鏂囨。鎵€鍐欑殑鍐呭銆傛湰鏂囨。鏃ㄥ湪甯姪鐞嗚В璇ュ崗璁紝
骞堕潪涓€浠藉弬鑰冩墜鍐屻€傛杩庢彁鍑烘剰瑙佷笌淇銆傚闇€鑱旂郴鎴戯紝璇峰彂閫侀偖浠惰嚦锛歫ohann.deneux@gmail.com


    濡傛灉浣犲皾璇曚緷鎹湰鏂囨。涓墍璇诲埌鐨勫唴瀹瑰悜浣犵殑 I-Force 璁惧鍙戦€佹暟鎹紝鐢辨閫犳垚鐨勪换浣曟崯鍧忔垨浼ゅ锛屾垜姒備笉璐熻矗銆?
## 棰勫璇存槑


鎵€鏈夋暟鍊煎潎涓哄崄鍏繘鍒讹紝閲囩敤澶х锛坆ig-endian锛夌紪鐮侊紙鏈€楂樻湁鏁堜綅鍦ㄥ乏锛夈€備絾璇锋敞鎰忥紝
鏁版嵁鍖呭唴閮ㄧ殑鍊奸噰鐢ㄥ皬绔紙little-endian锛夌紪鐮併€備綔鐢ㄦ湭鐭ュ瓧鑺傛爣璁颁负 ??? 闇€瑕佽繘涓€姝ユ繁鍏ユ鏌ョ殑淇℃伅鏍囪涓?锛?

### 鏁版嵁鍖呯殑涓€鑸舰寮?

浠ヤ笅鏄澶囦娇鐢?rs232 杩涜閫氫俊鏃舵暟鎹寘鐨勬牱瀛愩€?
== == === ==== ==
2B OP LEN DATA CS
== == === ==== ==

CS 鏄牎楠屽拰銆傚畠绛変簬鎵€鏈夊瓧鑺傜殑寮傛垨锛坋xclusive or锛夌粨鏋溿€?
浣跨敤 USB 鏃讹細

== ====
OP DATA
== ====

2B銆丩EN 鍜?CS 瀛楁宸叉秷澶憋紝澶ф鏄洜涓?USB 浼氬鐞嗗抚锛屽苟涓旀暟鎹崯鍧忔垨琚Ε鍠勫鐞嗭紝鎴栧奖鍝嶅彲蹇界暐銆?
棣栧厛锛屾垜鎻忚堪鐢辫澶囧彂閫佺粰璁＄畻鏈虹殑鏁堟灉銆?
## 璁惧杈撳叆鐘舵€?

姝ゆ暟鎹寘鐢ㄤ簬鎸囩ず姣忎釜鎸夐挳鐨勭姸鎬佷互鍙婃瘡涓酱鐨勫€硷細

```
    OP= 01 for a joystick, 03 for a wheel
    LEN= Varies from device to device
    00 X-Axis lsb
    01 X-Axis msb
    02 Y-Axis lsb, or gas pedal for a wheel
    03 Y-Axis msb, or brake pedal for a wheel
    04 Throttle
    05 Buttons
    06 Lower 4 bits: Buttons
       Upper 4 bits: Hat
    07 Rudder

```
## 璁惧鏁堟灉鐘舵€?

```
    OP= 02
    LEN= Varies
    00 ? Bit 1 (Value 2) is the value of the deadman switch
    01 Bit 8 is set if the effect is playing. Bits 0 to 7 are the effect id.
    02 ??
    03 Address of parameter block changed (lsb)
    04 Address of parameter block changed (msb)
    05 Address of second parameter block changed (lsb)
    ... depending on the number of parameter blocks updated

```
### 鍔涘弽棣堟晥鏋?

```
    OP=  01
    LEN= 0e
    00 Channel (when playing several effects at the same time, each must
                be assigned a channel)
    01 Wave form
	    Val 00 Constant
	    Val 20 Square
	    Val 21 Triangle
	    Val 22 Sine
	    Val 23 Sawtooth up
	    Val 24 Sawtooth down
	    Val 40 Spring (Force = f(pos))
	    Val 41 Friction (Force = f(velocity)) and Inertia
	           (Force = f(acceleration))

    02 Axes affected and trigger
	    Bits 4-7: Val 2 = effect along one axis. Byte 05 indicates direction
		    Val 4 = X axis only. Byte 05 must contain 5a
		    Val 8 = Y axis only. Byte 05 must contain b4
		    Val c = X and Y axes. Bytes 05 must contain 60
	    Bits 0-3: Val 0 = No trigger
		    Val x+1 = Button x triggers the effect
	    When the whole byte is 0, cancel the previously set trigger

    03-04 Duration of effect (little endian encoding, in ms)

    05 Direction of effect, if applicable. Else, see 02 for value to assign.

    06-07 Minimum time between triggering.

    08-09 Address of periodicity or magnitude parameters
    0a-0b Address of attack and fade parameters, or ffff if none.
    *or*
    08-09 Address of interactive parameters for X-axis,
          or ffff if not applicable
    0a-0b Address of interactive parameters for Y-axis,
	  or ffff if not applicable

    0c-0d Delay before execution of effect (little endian encoding, in ms)

```
### 鍩轰簬鏃堕棿鐨勫弬鏁?

##### 璧烽煶涓庢贰鍑?

```
    OP=  02
    LEN= 08
    00-01 Address where to store the parameters
    02-03 Duration of attack (little endian encoding, in ms)
    04 Level at end of attack. Signed byte.
    05-06 Duration of fade.
    06 Level at end of fade.

```
##### 骞呭€?

```
    OP=  03
    LEN= 03
    00-01 Address
    02 Level. Signed byte.

```
##### 鍛ㄦ湡鎬?

```
    OP=  04
    LEN= 07
    00-01 Address
    02 Magnitude. Signed byte.
    03 Offset. Signed byte.
    04 Phase. Val 00 = 0 deg, Val 40 = 90 degs.
    05-06 Period (little endian encoding, in ms)

```
### 浜や簰鍙傛暟


```
    OP=  05
    LEN= 0a
    00-01 Address
    02 Positive Coeff
    03 Negative Coeff
    04+05 Offset (center)
    06+07 Dead band (Val 01F4 = 5000 (decimal))
    08 Positive saturation (Val 0a = 1000 (decimal) Val 64 = 10000 (decimal))
    09 Negative saturation

```
姝ゅ鐨勭紪鐮佹湁浜涚壒娈婏細瀵逛簬绯绘暟锛坈oeffs锛夛紝杩欎簺鏄湁绗﹀彿鍊笺€傛渶澶у€间负 64锛堝崄杩涘埗 100锛夛紝鏈€灏忓€间负 9c銆?瀵逛簬鍋忕Щ锛坥ffset锛夛紝鏈€灏忓€间负 FE0C锛屾渶澶у€间负 01F4銆?瀵逛簬姝诲尯锛坉eadband锛夛紝鏈€灏忓€间负 0锛屾渶澶у€间负 03E8銆?
### 鎺у埗


```
    OP=  41
    LEN= 03
    00 Channel
    01 Start/Stop
	    Val 00: Stop
	    Val 01: Start and play once.
	    Val 41: Start and play n times (See byte 02 below)
    02 Number of iterations n.

```
### 鍒濆鍖?

##### 鏌ヨ鐗规€?

```
    OP=  ff
    Query command. Length varies according to the query type.
    The general format of this packet is:
    ff 01 QUERY [INDEX] CHECKSUM
    responses are of the same form:
    FF LEN QUERY VALUE_QUERIED CHECKSUM2
    where LEN = 1 + length(VALUE_QUERIED)

```
#### 鏌ヨ RAM 澶у皬


```
    QUERY = 42 ('B'uffer size)

```
璁惧搴斾互鐩稿悓鐨勬暟鎹寘鍔犱袱涓澶栧瓧鑺傦紙鍖呭惈鍐呭瓨澶у皬锛変綔涓哄洖搴旓細
ff 03 42 03 e8 CS 琛ㄧず璁惧鏈?1000 瀛楄妭鐨?RAM 鍙敤銆?
#### 鏌ヨ鏁堟灉鏁伴噺


```
    QUERY = 4e ('N'umber of effects)

```
璁惧搴旈€氳繃鍙戦€佸彲鍚屾椂鎾斁鐨勬晥鏋滄暟閲忥紙涓€涓瓧鑺傦級鏉ュ洖搴?ff 02 4e 14 CS 琛ㄧず 20 涓晥鏋溿€?
#### 鍘傚晢 ID


```
    QUERY = 4d ('M'anufacturer)

```
鏌ヨ鍘傚晢 ID锛? 瀛楄妭锛?
#### 浜у搧 ID


```
    QUERY = 50 ('P'roduct)

```
鏌ヨ浜у搧 ID锛? 瀛楄妭锛?
#### 鎵撳紑璁惧


```
    QUERY = 4f ('O'pen)

```
鏃犳暟鎹繑鍥炪€?
#### 鍏抽棴璁惧


```
    QUERY = 43 ('C')lose

```
鏃犳暟鎹繑鍥炪€?
#### 鏌ヨ鏁堟灉


```
    QUERY = 45 ('E')

```
鍙戦€佹晥鏋滅被鍨嬨€?鑻ユ敮鎸佸垯杩斿洖闈為浂鍊硷紙2 瀛楄妭锛?
#### 鍥轰欢鐗堟湰


```
    QUERY = 56 ('V'ersion)

```
杩斿洖 3 涓瓧鑺?鈥斺€?涓荤増鏈€佹鐗堟湰銆佷慨璁㈢増鏈?
##### 璁惧鐨勫垵濮嬪寲


#### 璁剧疆鎺у埗


    璁惧鐩稿叧锛屽湪涓嶅悓鍨嬪彿涓婂彲鑳戒笉鍚岋紒

```
    OP=  40 <idx> <val> [<val>]
    LEN= 2 or 3
    00 Idx
       Idx 00 Set dead zone (0..2048)
       Idx 01 Ignore Deadman sensor (0..1)
       Idx 02 Enable comm watchdog (0..1)
       Idx 03 Set the strength of the spring (0..100)
       Idx 04 Enable or disable the spring (0/1)
       Idx 05 Set axis saturation threshold (0..2048)

```
#### 璁剧疆鏁堟灉鐘舵€?

```
    OP=  42 <val>
    LEN= 1
    00 State
       Bit 3 Pause force feedback
       Bit 2 Enable force feedback
       Bit 0 Stop all effects

```
#### 璁剧疆鏁翠綋澧炵泭


```
    OP=  43 <val>
    LEN= 1
    00 Gain
       Val 00 = 0%
       Val 40 = 50%
       Val 80 = 100%

```
### 鍙傛暟鍐呭瓨


姣忎釜璁惧閮芥湁涓€瀹氭暟閲忕殑鐢ㄤ簬瀛樺偍鏁堟灉鍙傛暟鐨勫唴瀛樸€?RAM 鐨勫ぇ灏忓彲鑳戒笉鍚岋紝鎴戦亣鍒拌繃 200 鍒?1000 瀛楄妭涔嬮棿鐨勫€笺€備互涓嬫槸姣忕粍鍙傛暟鏄庢樉鎵€闇€鐨勫唴瀛橀噺锛?
 - period : 0c
 - magnitude : 02
 - attack and fade : 0e
 - interactive : 08

## 闄勫綍锛氬浣曠爺绌惰鍗忚锛?

1. 浣跨敤闅?DirectX SDK 鎻愪緵鐨勫姏鍙嶉缂栬緫鍣ㄧ敓鎴愭晥鏋滐紝鎴?   浣跨敤 Immersion Studio锛堝彲鍦ㄥ叾缃戠珯寮€鍙戣€呬笓鍖哄厤璐硅幏鍙栵細
   www.immersion.com锛?2. 鍚姩涓€涓 RS232 鎴?USB 杩涜鍡呮帰锛坰pying锛夌殑杞欢锛堝彇鍐充簬浣犲皢鎽囨潌/鏂瑰悜鐩樿繛鎺ュ埌浣曞锛夈€傛垜浣跨敤浜?fCoder 鐨?ComPortSpy锛坅lpha 鐗堟湰锛侊級
3. 鎾斁璇ユ晥鏋滐紝骞惰瀵熷梾鎺㈠睆骞曚笂鐨勫彉鍖栥€?
鍏充簬 ComPortSpy 鐨勫嚑鍙ヨ瘽锛?涔嶄竴鐪嬶紝杩欎釜杞欢浼间箮锛屽棷锛屾湁鐐光€︹€︽湁 bug銆傚疄闄呬笂锛屾暟鎹細鍑虹幇鍑犵閽熺殑寤惰繜銆傚氨鎴戜釜浜鸿€岃█锛屾瘡娆℃挱鏀炬晥鏋滄椂鎴戦兘浼氶噸鍚畠銆?璇疯浣忓畠鏄厤璐圭殑锛堝鍚屽厤璐瑰暏閰掕埇鍏嶈垂锛夎€屼笖杩樻槸 alpha 鐗堬紒

## URLS


鏌ョ湅 http://www.immerse.com 鑾峰彇 Immersion Studio锛?浠ュ強 http://www.fcoder.com 鑾峰彇 ComPortSpy銆?

I-Force 鏄?Immersion Corp. 鐨勫晢鏍囥€?