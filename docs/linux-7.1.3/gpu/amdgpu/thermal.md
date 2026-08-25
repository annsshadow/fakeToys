##  GPU 电源/热控制与监控


## HWMON 接口


   :doc: hwmon

## GPU sysfs 电源状态接

GPU 电源控制通过 sysfs 文件暴露
### power_dpm_state


   :doc: power_dpm_state

### power_dpm_force_performance_level


   :doc: power_dpm_force_performance_level

### pp_table


   :doc: pp_table

### pp_od_clk_voltage


   :doc: pp_od_clk_voltage

### pp_dpm_*


   :doc: pp_dpm_sclk pp_dpm_mclk pp_dpm_socclk pp_dpm_fclk pp_dpm_dcefclk pp_dpm_pcie

### pp_power_profile_mode


   :doc: pp_power_profile_mode

### pm_policy


   :doc: pm_policy

### \*_busy_percent


   :doc: gpu_busy_percent

   :doc: mem_busy_percent

### gpu_metrics


   :doc: gpu_metrics

### fan_curve


   :doc: fan_curve

### acoustic_limit_rpm_threshold


   :doc: acoustic_limit_rpm_threshold

### acoustic_target_rpm_threshold


   :doc: acoustic_target_rpm_threshold

### fan_target_temperature


   :doc: fan_target_temperature

### fan_minimum_pwm


   :doc: fan_minimum_pwm

### fan_zero_rpm_enable


   :doc: fan_zero_rpm_enable

### fan_zero_rpm_stop_temperature


   :doc: fan_zero_rpm_stop_temperature

## GFXOFF


GFXOFF 是大多数近期 GPU 中具备的一项功能，可在运行时节省功耗。当 gfx compute 流水线没有工作负载时，卡RLC（RunList Controller，运行列表控制器）固件会动态地关闭 gfx 引擎的电源。在受支持的 GPU 上，GFXOFF 默认开启
用户空间可以通过 debugfs 接口GFXOFF 交互（除非另有说明，所有值均`uint32_t`）：

### ``amdgpu_gfxoff``


```

  $ xxd -l1 -p /sys/kernel/debug/dri/0/amdgpu_gfxoff
  01

```

- 写入 0 禁用它，写入 1 启用它- 读取 0 表示已禁用，读取 1 表示已启用
如果它被启用，意味着 GPU 可以根据需要自由进GFXOFF 模式。禁用意味着它将永远不会进入 GFXOFF 模式
### ``amdgpu_gfxoff_status``


```

  $ xxd -l1 -p /sys/kernel/debug/dri/0/amdgpu_gfxoff_status
  02

```

- 0：GPU 处于 GFXOFF 状态，gfx 引擎已断电- 1：正在退GFXOFF 状- 2：不处于 GFXOFF 状- 3：正在进GFXOFF 状
如果 GFXOFF 已启用，该值将[0, 3] 之间不断跳变，并尽可能进0。当它被禁用时，该值始终为 2。如果不支持则返`-EINVAL`
### ``amdgpu_gfxoff_count``


读取它可获得自系统上电以来查询时刻的 GFXOFF 进入总次数。该值为 `uint64_t` 类型，但由于固件限制，目前可能作`uint32_t` 溢出*仅在 vangogh 上支*

### ``amdgpu_gfxoff_residency``


amdgpu_gfxoff_residency 写入 1 开始记录，写入 0 停止记录。读取它可获得在上一次记录间隔内的平GFXOFF 驻留率（百分比）乘以 100。例如，7854 表示在上一次记录间隔内，GPU 78.54% 的时间处GFXOFF 模式*仅在 vangogh 上支*
