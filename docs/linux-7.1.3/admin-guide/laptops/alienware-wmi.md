
## Alienware WMI 驱动


Kurt Borja <kuurtb@gmail.com>

这是一个用"WMAX" WMI 设备的驱动，该设备存在于大多Dell 游戏本中，控制各种特殊功能
M 系列笔记本推出之前（2018 年）WMAX" 设备控制基本RGB 灯光、深度睡眠模式HDMI 模式与放大器状态
后来，该设备被完全重新定位用途。现在它主要处理散热配置文件、传感器监视与超频。该接口
名为 "AWCC"，已知被 AWCC OEM 应用程序用来控制这些特性
alienware-wmi 驱动控制这两个接口
## AWCC 接口


WMI 设备文档：Documentation/wmi/devices/alienware-wmi.rst

### 支持的设

- Alienware M 系列笔记- Alienware X 系列笔记- Alienware Aurora 台式- Dell G 系列笔记
如果你认为你的设备支AWCC 接口，但没有本文档中描述的任何特性，请尝试以alienware-wmi
模块参数
- `force_platform_profile=1`：强制探测平台配置文件支- `force_hwmon=1`：强制探HWMON 支持

如果使用这些参数模块加载成功，请考虑提交一个补丁，将你的型号加入位`drivers/platform/x86/dell/alienware-wmi-wmax.c` `awcc_dmi_table`，或联系维护获取进一步指导
### 状

当前支持以下特性：

- 平台配置文件 <platform-profile>
  - 散热配置文件控制

  - G-Mode 切换

- HWMON <hwmon>锛。
  - 传感器监
  - 手动风扇控制


### 平台配置文件


AWCC 接口暴露各种固件定义的散热配置文件。这些通过平台配置文件类接口暴露给用户空间。更信息请参sysfs-class-platform-profile
<abi_file_testing_sysfs_class_platform_profile>銆。
该驱动导出的平台配置文件类设备名称为 "alienware-wmi"，其路径可通过以下方式找到
```

 grep -l "alienware-wmi" /sys/class/platform-profile/platform-profile-*/name | sed 's|/[^/]*$||'

```
如果设备支持 G-Mode，选择 `performance` 配置文件时也会切换它
   你可以设`force_gmode` 模块参数来始终尝试切换此特性，而不检查你的型号是否支持它

### HWMON


AWCC 接口还支持传感器监视与手动风扇控制。这两个特性都通过 HWMON 接口暴露给用户空间
该驱动导出的 hwmon 类设备名称为 "alienware_wmi"，其路径可通过以下方式找到
```

 grep -l "alienware_wmi" /sys/class/hwmon/hwmon*/name | sed 's|/[^/]*$||'

```
传感器监视通过标准 HWMON 接口完成。更多信息请参sysfs-class-hwmon
<abi_file_testing_sysfs_class_hwmon>銆。
另一方面，手动风扇控制并非由 AWCC 接口直接暴露。相反，它允许我们控制风`boost` 值`boost` 值对风扇 pwm 的近似行为如下：

```

 pwm = pwm_base + (fan_boost / 255) * (pwm_max - pwm_base)

```
由于上述行为，风`boost` 控制通过以下自定义的 hwmon sysfs 属性暴露给用户空间
=============================== ======= =======================================
名称			权限	描述
=============================== ======= =======================================
fan[1-4]_boost			RW	风扇 boost 值
					介于 0 255 之间的整数=============================== ======= =======================================

   在某些设备上，手动风扇控制只有在选择`custom` 平台配置文件时才可靠工作