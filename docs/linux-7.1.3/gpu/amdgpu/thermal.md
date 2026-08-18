##  GPU 鐢垫簮/鐑帶鍒朵笌鐩戞帶


## HWMON 鎺ュ彛


   :doc: hwmon

## GPU sysfs 鐢垫簮鐘舵€佹帴鍙?

GPU 鐢垫簮鎺у埗閫氳繃 sysfs 鏂囦欢鏆撮湶銆?
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


GFXOFF 鏄ぇ澶氭暟杩戞湡 GPU 涓叿澶囩殑涓€椤瑰姛鑳斤紝鍙湪杩愯鏃惰妭鐪佸姛鑰椼€傚綋 gfx 鎴?compute 娴佹按绾挎病鏈夊伐浣滆礋杞芥椂锛屽崱鐨?RLC锛圧unList Controller锛岃繍琛屽垪琛ㄦ帶鍒跺櫒锛夊浐浠朵細鍔ㄦ€佸湴鍏抽棴 gfx 寮曟搸鐨勭數婧愩€傚湪鍙楁敮鎸佺殑 GPU 涓婏紝GFXOFF 榛樿寮€鍚€?
鐢ㄦ埛绌洪棿鍙互閫氳繃 debugfs 鎺ュ彛涓?GFXOFF 浜や簰锛堥櫎闈炲彟鏈夎鏄庯紝鎵€鏈夊€煎潎涓?`uint32_t`锛夛細

### ``amdgpu_gfxoff``


```

  $ xxd -l1 -p /sys/kernel/debug/dri/0/amdgpu_gfxoff
  01

```

- 鍐欏叆 0 绂佺敤瀹冿紝鍐欏叆 1 鍚敤瀹冦€?- 璇诲彇 0 琛ㄧず宸茬鐢紝璇诲彇 1 琛ㄧず宸插惎鐢ㄣ€?
濡傛灉瀹冭鍚敤锛屾剰鍛崇潃 GPU 鍙互鏍规嵁闇€瑕佽嚜鐢辫繘鍏?GFXOFF 妯″紡銆傜鐢ㄦ剰鍛崇潃瀹冨皢姘歌繙涓嶄細杩涘叆 GFXOFF 妯″紡銆?
### ``amdgpu_gfxoff_status``


```

  $ xxd -l1 -p /sys/kernel/debug/dri/0/amdgpu_gfxoff_status
  02

```

- 0锛欸PU 澶勪簬 GFXOFF 鐘舵€侊紝gfx 寮曟搸宸叉柇鐢点€?- 1锛氭鍦ㄩ€€鍑?GFXOFF 鐘舵€?- 2锛氫笉澶勪簬 GFXOFF 鐘舵€?- 3锛氭鍦ㄨ繘鍏?GFXOFF 鐘舵€?
濡傛灉 GFXOFF 宸插惎鐢紝璇ュ€煎皢鍦?[0, 3] 涔嬮棿涓嶆柇璺冲彉锛屽苟灏藉彲鑳借繘鍏?0銆傚綋瀹冭绂佺敤鏃讹紝璇ュ€煎缁堜负 2銆傚鏋滀笉鏀寔鍒欒繑鍥?`-EINVAL`銆?
### ``amdgpu_gfxoff_count``


璇诲彇瀹冨彲鑾峰緱鑷郴缁熶笂鐢典互鏉ユ煡璇㈡椂鍒荤殑 GFXOFF 杩涘叆鎬绘鏁般€傝鍊间负 `uint64_t` 绫诲瀷锛屼絾鐢变簬鍥轰欢闄愬埗锛岀洰鍓嶅彲鑳戒綔涓?`uint32_t` 婧㈠嚭銆?*浠呭湪 vangogh 涓婃敮鎸?*

### ``amdgpu_gfxoff_residency``


鍚?amdgpu_gfxoff_residency 鍐欏叆 1 寮€濮嬭褰曪紝鍐欏叆 0 鍋滄璁板綍銆傝鍙栧畠鍙幏寰楀湪涓婁竴娆¤褰曢棿闅斿唴鐨勫钩鍧?GFXOFF 椹荤暀鐜囷紙鐧惧垎姣旓級涔樹互 100銆備緥濡傦紝鍊?7854 琛ㄧず鍦ㄤ笂涓€娆¤褰曢棿闅斿唴锛孏PU 鏈?78.54% 鐨勬椂闂村浜?GFXOFF 妯″紡銆?*浠呭湪 vangogh 涓婃敮鎸?*
