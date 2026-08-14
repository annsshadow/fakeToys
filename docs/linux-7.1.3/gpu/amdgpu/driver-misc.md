
## AMDGPU 驱动杂项信息


## GPU 产品信息


某些显卡可通过 sysfs 获取 GPU 相关信息

### product_name


   :doc: product_name

### product_number


   :doc: product_number

### serial_number


   :doc: serial_number

### fru_id


   :doc: fru_id

### manufacturer


   :doc: manufacturer

### unique_id


   :doc: unique_id

### board_info


   :doc: board_info

## GPU 内存使用信息


可通过 sysfs 访问各种内存统计信息

### mem_info_vram_total


   :doc: mem_info_vram_total

### mem_info_vram_used


   :doc: mem_info_vram_used

### mem_info_vis_vram_total


   :doc: mem_info_vis_vram_total

### mem_info_vis_vram_used


   :doc: mem_info_vis_vram_used

### mem_info_gtt_total


   :doc: mem_info_gtt_total

### mem_info_gtt_used


   :doc: mem_info_gtt_used

## PCIe 统计信息


### pcie_bw


   :doc: pcie_bw

### pcie_replay_count


   :doc: pcie_replay_count

## GPU SmartShift 信息


可通过 sysfs 获取 GPU SmartShift 信息

### smartshift_apu_power


   :doc: smartshift_apu_power

### smartshift_dgpu_power


   :doc: smartshift_dgpu_power

### smartshift_bias


   :doc: smartshift_bias

## UMA 预留（Carveout）


某些 Atom ROM 版本会暴露 VRAM 预留（carveout）大小的可选项，并允许在受支持的 BIOS 实现上通过 ATCS 功能码 0xA 更改预留大小。

对于这些平台，用户可以使用 uma/ 下的以下文件来设置预留大小，方式与 Windows 用户在 AMD Adrenalin 的“Tuning”选项卡中所做的类似。

注意，对于不支持此功能的 BIOS 实现，这些文件根本不会被创建。

### uma/carveout_options


   :doc: uma/carveout_options

### uma/carveout


   :doc: uma/carveout
