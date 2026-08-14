


## Kernel driver dell-smm-hwmon

鏈〉浠嬬粛 Dell 绗旇鏈笂 dell-smm-hwmon 鍐呮牳椹卞姩锛岃鏄庡浣曢€氳繃绯荤粺绠＄悊妯″紡锛圫MM锛塀IOS 璇诲彇椋庢墖涓庢俯搴︿紶鎰熷櫒鐘舵€侊紝骞剁粡鐢辨爣鍑?hwmon sysfs 鎺ュ彛鍚戠敤鎴风┖闂存毚闇茬浉鍏崇洃娴嬪睘鎬с€?

## 鍐呮牳椹卞姩 dell-smm-hwmon


:Copyright: |copy| 2002-2005 Massimo Dal Zotto <dz@debian.org>
:Copyright: |copy| 2019 Giovanni Mascellani <gio@debian.org>

### Description

### 鎻忚堪


On many Dell laptops the System Management Mode (SMM) BIOS can be
queried for the status of fans and temperature sensors.  Userspace
utilities like `sensors` can be used to return the readings. The
userspace suite `i8kutils`__ can also be used to read the sensors and
automatically adjust fan speed (please notice that it currently uses
the deprecated `/proc/i8k` interface).

鍦ㄨ澶?Dell 绗旇鏈笂锛屽彲浠ユ煡璇㈢郴缁熺鐞嗘ā寮忥紙SMM锛塀IOS 浠ヨ幏鍙栭鎵囧拰娓╁害浼犳劅鍣ㄧ殑鐘舵€併€?
鍙互浣跨敤璇稿 `sensors` 杩欐牱鐨勭敤鎴风┖闂村伐鍏锋潵杩斿洖璇绘暟銆傜敤鎴风┖闂寸殑 `i8kutils`__ 濂椾欢
涔熷彲鐢ㄤ簬璇诲彇浼犳劅鍣ㄥ苟鑷姩璋冭妭椋庢墖閫熷害锛堣娉ㄦ剰锛屽畠鐩墠浣跨敤宸插簾寮冪殑 `/proc/i8k` 鎺ュ彛锛夈€?

 __ https://github.com/vitorafsr/i8kutils

### ``sysfs`` interface

### ``sysfs`` 鎺ュ彛


Temperature sensors and fans can be queried and set via the standard
`hwmon` interface on `sysfs`, under the directory
`/sys/class/hwmon/hwmonX` for some value of `X` (search for the
`X` such that `/sys/class/hwmon/hwmonX/name` has content
`dell_smm`). A number of other attributes can be read or written:

娓╁害鍜岄鎵囧彲浠ラ€氳繃 `sysfs` 涓婃爣鍑?`hwmon` 鎺ュ彛杩涜鏌ヨ鍜岃缃紝浣嶄簬鐩綍
`/sys/class/hwmon/hwmonX`锛堝叾涓?`X` 涓烘煇涓€硷級涓嬶紙鏌ユ壘浣垮緱
`/sys/class/hwmon/hwmonX/name` 鍐呭涓?`dell_smm` 鐨勯偅涓?`X`锛夈€傝繕鏈夎澶氬叾浠?
灞炴€у彲浠ヨ鍙栨垨鍐欏叆锛?

=============================== ======= =======================================
Name				Perm	Description
=============================== ======= =======================================
fan[1-4]_input                  RO      Fan speed in RPM.
fan[1-4]_label                  RO      Fan label.
fan[1-4]_min                    RO      Minimal Fan speed in RPM
fan[1-4]_max                    RO      Maximal Fan speed in RPM
fan[1-4]_target                 RO      Expected Fan speed in RPM
pwm[1-4]                        RW      Control the fan PWM duty-cycle.
pwm[1-4]_enable                 RW/WO   Enable or disable automatic BIOS fan
                                        control (not supported on all laptops,
                                        see below for details).
temp[1-10]_input                RO      Temperature reading in milli-degrees
                                        Celsius.
temp[1-10]_label                RO      Temperature sensor label.
=============================== ======= =======================================

Due to the nature of the SMM interface, each pwmX attribute controls
fan number X.

鐢变簬 SMM 鎺ュ彛鐨勭壒鎬э紝姣忎釜 pwmX 灞炴€ф帶鍒剁紪鍙蜂负 X 鐨勯鎵囥€?

### Enabling/Disabling automatic BIOS fan control

### 鍚敤/绂佺敤 BIOS 鑷姩椋庢墖鎺у埗


There exist two methods for enabling/disabling automatic BIOS fan control:

鏈変袱绉嶆柟娉曟潵鍚敤/绂佺敤 BIOS 鑷姩椋庢墖鎺у埗锛?

1. Separate SMM commands to enable/disable automatic BIOS fan control for all fans.

1. 浣跨敤鐙珛鐨?SMM 鍛戒护涓烘墍鏈夐鎵囧惎鐢?绂佺敤 BIOS 鑷姩椋庢墖鎺у埗銆?

2. A special fan state that enables automatic BIOS fan control for a individual fan.

2. 涓€绉嶇壒娈婄殑椋庢墖鐘舵€侊紝鍙负鍗曚釜椋庢墖鍚敤 BIOS 鑷姩椋庢墖鎺у埗銆?

The driver cannot reliably detect what method should be used on a given
device, so instead the following heuristic is used:

椹卞姩鏃犳硶鍙潬鍦版娴嬪湪鏌愪釜缁欏畾璁惧涓婂簲褰撲娇鐢ㄥ摢绉嶆柟娉曪紝鍥犳鏀圭敤浠ヤ笅鍚彂寮忚鍒欙細

- use fan state 3 for enabling BIOS fan control if the maximum fan state
  setable by the user is smaller than 3 (default setting).

- 濡傛灉鐢ㄦ埛鍙缃殑鏈€澶ч鎵囩姸鎬佸皬浜?3锛堥粯璁よ缃級锛屽垯浣跨敤椋庢墖鐘舵€?3 鏉ュ惎鐢?BIOS 椋庢墖
  鎺у埗銆?

- use separate SMM commands if device is whitelisted to support them.

- 濡傛灉璁惧鍦ㄧ櫧鍚嶅崟涓敮鎸佺嫭绔?SMM 鍛戒护锛屽垯浣跨敤鐙珛 SMM 鍛戒护銆?

When using the first method, each fan will have a standard `pwmX_enable`
sysfs attribute. Writing `1` into this attribute will disable automatic
BIOS fan control for the associated fan and set it to maximum speed. Enabling
BIOS fan control again can be achieved by writing `2` into this attribute.
Reading this sysfs attributes returns the current setting as reported by
the underlying hardware.

浣跨敤绗竴绉嶆柟娉曟椂锛屾瘡涓鎵囦細鏈変竴涓爣鍑嗙殑 `pwmX_enable` sysfs 灞炴€с€傚悜璇ュ睘鎬у啓鍏?`1`
浼氱鐢ㄥ搴旈鎵囩殑 BIOS 鑷姩椋庢墖鎺у埗锛屽苟灏嗗叾璁句负鏈€澶ч€熷害銆傚啀娆″惎鐢?BIOS 椋庢墖鎺у埗鍙?
閫氳繃鍚戣灞炴€у啓鍏?`2` 鏉ュ疄鐜般€傝鍙栨 sysfs 灞炴€т細杩斿洖搴曞眰纭欢鎶ュ憡鐨勫綋鍓嶈缃€?

When using the second method however, only the `pwm1_enable` sysfs attribute
will be available to enable/disable automatic BIOS fan control globaly for all
fans available on a given device. Additionally, this sysfs attribute is write-only
as there exists no SMM command for reading the current fan control setting.

鐒惰€岋紝浣跨敤绗簩绉嶆柟娉曟椂锛屽彧鏈?`pwm1_enable` sysfs 灞炴€у彲鐢ㄤ簬鍏ㄥ眬鍚敤/绂佺敤缁欏畾璁惧涓?
鎵€鏈夐鎵囩殑 BIOS 鑷姩椋庢墖鎺у埗銆傛澶栵紝姝?sysfs 灞炴€ф槸鍙啓鐨勶紝鍥犱负涓嶅瓨鍦ㄧ敤浜庤鍙栧綋鍓?
椋庢墖鎺у埗璁剧疆鐨?SMM 鍛戒护銆?

If no `pwmX_enable` attributes are available, then it means that the driver
cannot use the first method and the SMM codes for enabling and disabling automatic
BIOS fan control are not whitelisted for your device. It is possible that codes
that work for other laptops actually work for yours as well, or that you have to
discover new codes.

濡傛灉娌℃湁 `pwmX_enable` 灞炴€у彲鐢紝鍒欐剰鍛崇潃椹卞姩鏃犳硶浣跨敤绗竴绉嶆柟娉曪紝骞朵笖鐢ㄤ簬鍚敤鍜岀鐢?
BIOS 鑷姩椋庢墖鎺у埗鐨?SMM 浠ｇ爜鏈垪鍏ヤ綘璁惧鐨勭櫧鍚嶅崟銆傞€傜敤浜庡叾浠栫瑪璁版湰鐨勪唬鐮佸彲鑳戒篃閫傜敤
浜庝綘鐨勮澶囷紝鎴栬€呬綘鍙兘闇€瑕佸彂鐜版柊鐨勪唬鐮併€?

Check the list `i8k_whitelist_fan_control` in file
`drivers/hwmon/dell-smm-hwmon.c` in the kernel tree: as a first
attempt you can try to add your machine and use an already-known code
pair. If, after recompiling the kernel, you see that `pwm1_enable`
is present and works (i.e., you can manually control the fan speed),
then please submit your finding as a kernel patch, so that other users
can benefit from it. Please see
Documentation/process/submitting-patches.rst <submittingpatches>
for information on submitting patches.

璇锋煡鐪嬪唴鏍告爲涓枃浠?`drivers/hwmon/dell-smm-hwmon.c` 閲岀殑鍒楄〃
`i8k_whitelist_fan_control`锛氫綔涓洪娆″皾璇曪紝浣犲彲浠ヨ瘯鐫€娣诲姞浣犵殑鏈哄櫒骞朵娇鐢ㄤ竴瀵瑰凡鐭ョ殑
浠ｇ爜銆傚鏋滃湪閲嶆柊缂栬瘧鍐呮牳鍚庯紝浣犲彂鐜?`pwm1_enable` 瀛樺湪涓斿伐浣滄甯革紙鍗充綘鍙互鎵嬪姩鎺у埗
椋庢墖閫熷害锛夛紝璇峰皢浣犵殑鍙戠幇浣滀负鍐呮牳琛ヤ竵鎻愪氦锛屼互渚垮叾浠栫敤鎴蜂篃鑳藉彈鐩娿€傚叧浜庢彁浜よˉ涓佺殑淇℃伅锛?
璇峰弬闃?Documentation/process/submitting-patches.rst <submittingpatches>銆?

If no known code works on your machine, you need to resort to do some
probing, because unfortunately Dell does not publish datasheets for
its SMM. You can experiment with the code in `this repository`__ to
probe the BIOS on your machine and discover the appropriate codes.

濡傛灉娌℃湁宸茬煡浠ｇ爜鍦ㄤ綘鐨勬満鍣ㄤ笂宸ヤ綔锛屼綘闇€瑕佽繘琛屼竴浜涙帰娴嬶紝鍥犱负閬楁喚鐨勬槸 Dell 娌℃湁鍙戝竷鍏?
SMM 鐨勬暟鎹墜鍐屻€備綘鍙互鐢?`this repository`__ 涓殑浠ｇ爜鍦ㄤ綘鏈哄櫒涓婃帰娴?BIOS 骞跺彂鐜?
鐩稿簲鐨勪唬鐮併€?

 __ https://github.com/clopez/dellfan/

Again, when you find new codes, we'd be happy to have your patches!

鍚屾牱锛屽綋浣犲彂鐜版柊浠ｇ爜鏃讹紝鎴戜滑寰堜箰鎰忔敹鍒颁綘鐨勮ˉ涓侊紒

### ``thermal`` interface

### ``thermal`` 鎺ュ彛


The driver also exports the fans as thermal cooling devices with
`type` set to `dell-smm-fan[1-4]`. This allows for easy fan control
using one of the thermal governors.

璇ラ┍鍔ㄨ繕灏嗛鎵囧鍑轰负鏁ｇ儹鍐峰嵈璁惧锛屽叾 `type` 璁句负 `dell-smm-fan[1-4]`銆傝繖浣垮緱浣跨敤
鏌愪釜 thermal governor 鍙互杞绘澗鎺у埗椋庢墖銆?

### Module parameters

### 妯″潡鍙傛暟


- force:bool
                   Force loading without checking for supported
                   models. (default: 0)

- force:bool
                   寮哄埗鍔犺浇鑰屼笉妫€鏌ュ彈鏀寔鐨勫瀷鍙枫€傦紙榛樿锛?锛?

- ignore_dmi:bool
                   Continue probing hardware even if DMI data does not
                   match. (default: 0)

- ignore_dmi:bool
                   鍗充娇 DMI 鏁版嵁涓嶅尮閰嶄篃缁х画鎺㈡祴纭欢銆傦紙榛樿锛?锛?

- restricted:bool
                   Allow fan control only to processes with the
                   `CAP_SYS_ADMIN` capability set or processes run
                   as root when using the legacy `/proc/i8k`
                   interface. In this case normal users will be able
                   to read temperature and fan status but not to
                   control the fan.  If your notebook is shared with
                   other users and you don't trust them you may want
                   to use this option. (default: 1, only available
                   with `CONFIG_I8K`)

- restricted:bool
                   浠呭厑璁稿叿鏈?`CAP_SYS_ADMIN` 鑳藉姏鐨勮繘绋嬶紝鎴栧湪浣跨敤鏃х殑
                   `/proc/i8k` 鎺ュ彛鏃朵互 root 杩愯鐨勮繘绋嬫帶鍒堕鎵囥€傚湪杩欑鎯呭喌涓嬶紝鏅€氱敤鎴?
                   鑳藉璇诲彇娓╁害鍜岄鎵囩姸鎬侊紝浣嗕笉鑳芥帶鍒堕鎵囥€傚鏋滀綘鐨勭瑪璁版湰涓庡叾浠栫敤鎴峰叡浜?
                   涓斾綘涓嶄俊浠讳粬浠紝浣犲彲鑳戒細鎯充娇鐢ㄦ閫夐」銆傦紙榛樿锛?锛屼粎鍦?
                   `CONFIG_I8K` 涓嬪彲鐢級

- power_status:bool
                   Report AC status in `/proc/i8k`. (default: 0,
                   only available with `CONFIG_I8K`)

- power_status:bool
                   鍦?`/proc/i8k` 涓姤鍛婁氦娴佺數婧愮姸鎬併€傦紙榛樿锛?锛屼粎鍦?
                   `CONFIG_I8K` 涓嬪彲鐢級

- fan_mult:uint
                   Factor to multiply fan speed with. (default:
                   autodetect)

- fan_mult:uint
                   鐢ㄤ簬涔樹互椋庢墖閫熷害鐨勭郴鏁般€傦紙榛樿锛氳嚜鍔ㄦ娴嬶級

- fan_max:uint
                   Maximum configurable fan speed. (default:
                   autodetect)

- fan_max:uint
                   鍙厤缃殑鏈€澶ч鎵囬€熷害銆傦紙榛樿锛氳嚜鍔ㄦ娴嬶級

### Legacy ``/proc`` interface

### 鏃х増 ``/proc`` 鎺ュ彛


             used in new applications. This interface is only
             available when kernel is compiled with option
             `CONFIG_I8K`.

             鐢ㄤ簬鏂板簲鐢ㄤ腑銆傛鎺ュ彛浠呭湪浠ュ唴鏍搁€夐」 `CONFIG_I8K` 缂栬瘧鏃舵墠鍙敤銆?

The information provided by the kernel driver can be accessed by
```

    $ cat /proc/i8k
    1.0 A17 2J59L02 52 2 1 8040 6420 1 2

```

```
    1.0 A17 2J59L02 52 2 1 8040 6420 1 2
    |   |   |       |  | | |    |    | |
    |   |   |       |  | | |    |    | +------- 10. buttons status
    |   |   |       |  | | |    |    +--------- 9.  AC status
    |   |   |       |  | | |    +-------------- 8.  fan0 RPM
    |   |   |       |  | | +------------------- 7.  fan1 RPM
    |   |   |       |  | +--------------------- 6.  fan0 status
    |   |   |       |  +----------------------- 5.  fan1 status
    |   |   |       +-------------------------- 4.  temp0 reading (Celsius)
    |   |   +---------------------------------- 3.  Dell service tag (later known as 'serial number')
    |   +-------------------------------------- 2.  BIOS version
    +------------------------------------------ 1.  /proc/i8k format version

```
A negative value, for example -22, indicates that the BIOS doesn't
return the corresponding information. This is normal on some
models/BIOSes.

璐熷€硷紝渚嬪 -22锛岃〃绀?BIOS 娌℃湁杩斿洖鐩稿簲鐨勪俊鎭€傚湪鏌愪簺鍨嬪彿/BIOS 涓婃槸姝ｅ父鐨勩€?

For performance reasons the `/proc/i8k` doesn't report by default
the AC status since this SMM call takes a long time to execute and is
not really needed.  If you want to see the ac status in `/proc/i8k`
you must explictitly enable this option by passing the
`power_status=1` parameter to insmod. If AC status is not
available -1 is printed instead.

鍑轰簬鎬ц兘鍘熷洜锛宍/proc/i8k` 榛樿涓嶆姤鍛婁氦娴佺數婧愮姸鎬侊紝鍥犱负姝?SMM 璋冪敤鎵ц鏃堕棿杈冮暱涓斿苟闈?
鐪熸闇€瑕併€傚鏋滀綘鎯冲湪 `/proc/i8k` 涓湅鍒颁氦娴佺數婧愮姸鎬侊紝蹇呴』閫氳繃鍚?insmod 浼犻€?
`power_status=1` 鍙傛暟鏉ユ樉寮忓惎鐢ㄦ閫夐」銆傚鏋滀氦娴佺數婧愮姸鎬佷笉鍙敤锛屽垯鎵撳嵃 -1銆?

The driver provides also an ioctl interface which can be used to
obtain the same information and to control the fan status. The ioctl
interface can be accessed from C programs or from shell using the
i8kctl utility. See the source file of `i8kutils` for more
information on how to use the ioctl interface.

璇ラ┍鍔ㄨ繕鎻愪緵浜嗕竴涓?ioctl 鎺ュ彛锛屽彲鐢ㄤ簬鑾峰彇鐩稿悓鐨勪俊鎭苟鎺у埗椋庢墖鐘舵€併€傝 ioctl 鎺ュ彛
鍙粠 C 绋嬪簭鎴栭€氳繃浣跨敤 i8kctl 宸ュ叿鐨?shell 璁块棶銆傚叧浜庡浣曚娇鐢?ioctl 鎺ュ彛鐨勬洿澶氫俊鎭紝
璇峰弬闃?`i8kutils` 鐨勬簮鏂囦欢銆?

### SMM Interface

### SMM 鎺ュ彛


             since Dell did not provide any Documentation,
             please keep that in mind.

             鐢变簬 Dell 娌℃湁鎻愪緵浠讳綍鏂囨。锛岃璁颁綇杩欎竴鐐广€?

The driver uses the SMM interface to send commands to the system BIOS.
This interface is normally used by Dell's 32-bit diagnostic program or
on newer notebook models by the buildin BIOS diagnostics.
The SMM may cause short hangs when the BIOS code is taking too long to
execute.

璇ラ┍鍔ㄤ娇鐢?SMM 鎺ュ彛鍚戠郴缁?BIOS 鍙戦€佸懡浠ゃ€傛鎺ュ彛閫氬父鐢?Dell 鐨?32 浣嶈瘖鏂▼搴忥紝鎴栧湪
杈冩柊鐨勭瑪璁版湰鍨嬪彿涓婄敱鍐呯疆鐨?BIOS 璇婃柇鍔熻兘浣跨敤銆傚綋 BIOS 浠ｇ爜鎵ц鏃堕棿杩囬暱鏃讹紝SMM 鍙兘
瀵艰嚧鐭殏鐨勬寕璧枫€?

The SMM handler inside the system BIOS looks at the contents of the
`eax`, `ebx`, `ecx`, `edx`, `esi` and `edi` registers.
Each register has a special purpose:

绯荤粺 BIOS 涓殑 SMM 澶勭悊绋嬪簭浼氭煡鐪?`eax`銆乣ebx`銆乣ecx`銆乣edx`銆乣esi` 鍜?`edi`
瀵勫瓨鍣ㄧ殑鍐呭銆傛瘡涓瘎瀛樺櫒閮芥湁鐗规畩鐢ㄩ€旓細

=============== ==================================
Register        Purpose
=============== ==================================
eax             Holds the command code before SMM,
                holds the first result after SMM.
ebx             Holds the arguments.
ecx             Unknown, set to 0.
edx             Holds the second result after SMM.
esi             Unknown, set to 0.
edi             Unknown, set to 0.
=============== ==================================

The SMM handler can signal a failure by either:

SMM 澶勭悊绋嬪簭鍙互閫氳繃浠ヤ笅浠讳竴鏂瑰紡鍙戝嚭澶辫触淇″彿锛?

- setting the lower sixteen bits of `eax` to `0xffff`
- not modifying `eax` at all
- setting the carry flag (legacy SMM interface only)

- 灏?`eax` 鐨勪綆 16 浣嶈涓?`0xffff`
- 瀹屽叏涓嶄慨鏀?`eax`
- 璁剧疆杩涗綅鏍囧織锛堜粎鏃х増 SMM 鎺ュ彛锛?

### Legacy SMM Interface

### 鏃х増 SMM 鎺ュ彛


When using the legacy SMM interface, a SMM is triggered by writing the least significant byte
of the command code to the special ioports `0xb2` and `0x84`. This interface is not
described inside the ACPI tables and can thus only be detected by issuing a test SMM call.

浣跨敤鏃х増 SMM 鎺ュ彛鏃讹紝鍚戠壒娈?ioport `0xb2` 鍜?`0x84` 鍐欏叆鍛戒护鐮佺殑鏈€浣庢湁鏁堝瓧鑺傛潵瑙﹀彂
SMM銆傛鎺ュ彛涓嶅湪 ACPI 琛ㄤ腑鎻忚堪锛屽洜姝ゅ彧鑳介€氳繃鍙戝嚭娴嬭瘯 SMM 璋冪敤鏉ユ娴嬨€?

### WMI SMM Interface

### WMI SMM 鎺ュ彛


On modern Dell machines, the SMM calls are done over ACPI WMI:

鍦ㄧ幇浠?Dell 鏈哄櫒涓婏紝SMM 璋冪敤閫氳繃 ACPI WMI 瀹屾垚锛?

```

 #pragma namespace("\\\\.\\root\\dcim\\sysman\\diagnostics")
 [WMI, Provider("Provider_DiagnosticsServices"), Dynamic, Locale("MS\\0x409"),
  Description("RunDellDiag"), guid("{F1DDEE52-063C-4784-A11E-8A06684B9B01}")]
  class LegacyDiags {
  [key, read] string InstanceName;
  [read] boolean Active;

  [WmiMethodId(1), Implemented, read, write, Description("Legacy Method ")]
  void Execute([in, out] uint32 EaxLen, [in, out, WmiSizeIs("EaxLen") : ToInstance] uint8 EaxVal[],
               [in, out] uint32 EbxLen, [in, out, WmiSizeIs("EbxLen") : ToInstance] uint8 EbxVal[],
               [in, out] uint32 EcxLen, [in, out, WmiSizeIs("EcxLen") : ToInstance] uint8 EcxVal[],
               [in, out] uint32 EdxLen, [in, out, WmiSizeIs("EdxLen") : ToInstance] uint8 EdxVal[]);
 };

```

Some machines support only the WMI SMM interface, while some machines support both interfaces.
The driver automatically detects which interfaces are present and will use the WMI SMM interface
if the legacy SMM interface is not present. The WMI SMM interface is usually slower than the
legacy SMM interface since ACPI methods need to be called in order to trigger a SMM.

鏈変簺鏈哄櫒鍙敮鎸?WMI SMM 鎺ュ彛锛岃€屾湁浜涙満鍣ㄤ袱绉嶆帴鍙ｉ兘鏀寔銆傞┍鍔ㄤ細鑷姩妫€娴嬪瓨鍦ㄥ摢浜涙帴鍙ｏ紝
濡傛灉鏃х増 SMM 鎺ュ彛涓嶅瓨鍦紝鍒欎娇鐢?WMI SMM 鎺ュ彛銆俉MI SMM 鎺ュ彛閫氬父姣旀棫鐗?SMM 鎺ュ彛鎱紝鍥犱负
闇€瑕佽皟鐢?ACPI 鏂规硶鏉ヨЕ鍙?SMM銆?

### SMM command codes

### SMM 鍛戒护鐮?


=============== ======================= ================================================
Command Code    Command Name            Description
=============== ======================= ================================================
`0x0025`      Get Fn key status       Returns the Fn key pressed after SMM:

                                        - 9th bit in `eax` indicates Volume up
                                        - 10th bit in `eax` indicates Volume down
                                        - both bits indicate Volume mute

`0xa069`      Get power status        Returns current power status after SMM:

                                        - 1st bit in `eax` indicates Battery connected
                                        - 3th bit in `eax` indicates AC connected

`0x00a3`      Get fan state           Returns current fan state after SMM:

                                        - 1st byte in `eax` holds the current
                                          fan state (0 - 2 or 3)

`0x01a3`      Set fan state           Sets the fan speed:

                                        - 1st byte in `ebx` holds the fan number
                                        - 2nd byte in `ebx` holds the desired
                                          fan state (0 - 2 or 3)

`0x02a3`      Get fan speed           Returns the current fan speed in RPM:

                                        - 1st byte in `ebx` holds the fan number
                                        - 1st word in `eax` holds the current
                                          fan speed in RPM (after SMM)

`0x03a3`      Get fan type            Returns the fan type:

                                        - 1st byte in `ebx` holds the fan number
                                        - 1st byte in `eax` holds the
                                          fan type (after SMM):

                                          - 5th bit indicates docking fan
                                          - 1 indicates Processor fan
                                          - 2 indicates Motherboard fan
                                          - 3 indicates Video fan
                                          - 4 indicates Power supply fan
                                          - 5 indicates Chipset fan
                                          - 6 indicates other fan type

`0x04a3`      Get nominal fan speed   Returns the nominal RPM in each fan state:

                                        - 1st byte in `ebx` holds the fan number
                                        - 2nd byte in `ebx` holds the fan state
                                          in question (0 - 2 or 3)
                                        - 1st word in `eax` holds the nominal
                                          fan speed in RPM (after SMM)

`0x05a3`      Get fan speed tolerance Returns the speed tolerance for each fan state:

                                        - 1st byte in `ebx` holds the fan number
                                        - 2nd byte in `ebx` holds the fan state
                                          in question (0 - 2 or 3)
                                        - 1st byte in `eax` returns the speed
                                          tolerance

`0x10a3`      Get sensor temperature  Returns the measured temperature:

                                        - 1st byte in `ebx` holds the sensor number
                                        - 1st byte in `eax` holds the measured
                                          temperature (after SMM)

`0x11a3`      Get sensor type         Returns the sensor type:

                                        - 1st byte in `ebx` holds the sensor number
                                        - 1st byte in `eax` holds the
                                          temperature type (after SMM):

                                          - 1 indicates CPU sensor
                                          - 2 indicates GPU sensor
                                          - 3 indicates SODIMM sensor
                                          - 4 indicates other sensor type
                                          - 5 indicates Ambient sensor
                                          - 6 indicates other sensor type

`0xfea3`      Get SMM signature       Returns Dell signature if interface
                                        is supported (after SMM):

                                        - `eax` holds 1145651527
                                          (0x44494147 or "DIAG")
                                        - `edx` holds 1145392204
                                          (0x44454c4c or "DELL")

`0xffa3`      Get SMM signature       Same as `0xfea3`, check both.
=============== ======================= ================================================

There are additional commands for enabling (`0x31a3` or `0x35a3`) and
disabling (`0x30a3` or `0x34a3`) automatic fan speed control.
The commands are however causing severe sideeffects on many machines, so
they are not used by default.

杩樻湁鐢ㄤ簬鍚敤锛坄0x31a3` 鎴?`0x35a3`锛夊拰绂佺敤锛坄0x30a3` 鎴?`0x34a3`锛夎嚜鍔ㄩ鎵囬€熷害鎺у埗鐨?
棰濆鍛戒护銆傜劧鑰岃繖浜涘懡浠ゅ湪璁稿鏈哄櫒涓婁細閫犳垚涓ラ噸鐨勫壇浣滅敤锛屽洜姝ら粯璁や笉浣跨敤銆?

On several machines (Inspiron 3505, Precision 490, Vostro 1720, ...), the
fans supports a 4th "magic" state, which signals the BIOS that automatic
fan control should be enabled for a specific fan.
However there are also some machines who do support a 4th regular fan state too,
but in case of the "magic" state, the nominal RPM reported for this state is a
placeholder value, which however is not always detectable.

鍦ㄨ嫢骞叉満鍣ㄤ笂锛圛nspiron 3505銆丳recision 490銆乂ostro 1720 绛夛級锛岄鎵囨敮鎸佺 4 涓?榄旀硶"
鐘舵€侊紝瀹冨悜 BIOS 鍙戝嚭淇″彿锛屽簲涓虹壒瀹氶鎵囧惎鐢ㄨ嚜鍔ㄩ鎵囨帶鍒躲€備笉杩囦篃鏈変竴浜涙満鍣ㄥ悓鏃舵敮鎸?
绗?4 涓父瑙勯鎵囩姸鎬侊紝浣嗗湪"榄旀硶"鐘舵€佷笅锛屼负姝ょ姸鎬佹姤鍛婄殑鏍囩О RPM 鏄竴涓崰浣嶅€硷紝鐒惰€岃繖
骞堕潪鎬绘槸鍙娴嬬殑銆?

### Firmware Bugs

### 鍥轰欢缂洪櫡


The SMM calls can behave erratic on some machines:

SMM 璋冪敤鍦ㄦ煇浜涙満鍣ㄤ笂琛ㄧ幇鍙兘涓嶇ǔ瀹氾細

======================================================= =================
Firmware Bug                                            Affected Machines
======================================================= =================
Reading of fan states return spurious errors.           Precision 490

                                                        OptiPlex 7060

Reading of fan types causes erratic fan behaviour.      Studio XPS 8000

                                                        Studio XPS 8100

                                                        Inspiron 580

                                                        Inspiron 3505

Fan-related SMM calls take too long (about 500ms).      Inspiron 7720

                                                        Vostro 3360

                                                        XPS 13 9333

                                                        XPS 15 L502X
======================================================= =================

In case you experience similar issues on your Dell machine, please
submit a bugreport on bugzilla to we can apply workarounds.

濡傛灉浣犲湪 Dell 鏈哄櫒涓婇亣鍒扮被浼奸棶棰橈紝璇峰湪 bugzilla 涓婃彁浜?bugreport锛屼互渚挎垜浠簲鐢ㄥ彉閫?
鏂规硶銆?

### Limitations

### 闄愬埗


The SMM calls can take too long to execute on some machines, causing
short hangs and/or audio glitches.
Also the fan state needs to be restored after suspend, as well as
the automatic mode settings.
When reading a temperature sensor, values above 127 degrees indicate
a BIOS read error or a deactivated sensor.

SMM 璋冪敤鍦ㄦ煇浜涙満鍣ㄤ笂鎵ц鍙兘鑰楁椂杩囬暱锛屽鑷寸煭鏆傜殑鎸傝捣鍜?鎴栭煶棰戞晠闅溿€傛澶栵紝椋庢墖鐘舵€?
闇€瑕佸湪鎸傝捣鍚庢仮澶嶏紝鑷姩妯″紡璁剧疆涔熸槸濡傛銆傝鍙栨俯搴︿紶鎰熷櫒鏃讹紝楂樹簬 127 搴︾殑鍊艰〃绀?BIOS
璇诲彇閿欒鎴栦紶鎰熷櫒琚仠鐢ㄣ€?
