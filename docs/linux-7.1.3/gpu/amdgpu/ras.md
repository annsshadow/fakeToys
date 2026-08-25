##  AMDGPU RAS 支持


AMDGPU RAS 接口通过 sysfs（用于信息查询）debugfs（用于错误注入）暴露
## RAS debugfs/sysfs 控制与错误注入接

   :doc: AMDGPU RAS debugfs control interface

## RAS 不可恢复错误的重启行

   :doc: AMDGPU RAS Reboot Behavior for Unrecoverable Errors

## RAS 错误计数 sysfs 接口


   :doc: AMDGPU RAS sysfs Error Count Interface

## RAS EEPROM debugfs 接口


   :doc: AMDGPU RAS debugfs EEPROM table reset interface

## RAS VRAM 坏页 sysfs 接口


   :doc: AMDGPU RAS sysfs gpu_vram_bad_pages Interface

## 示例代码


用于测试错误注入的示例代码可在此处找到：
https://cgit.freedesktop.org/mesa/drm/tree/tests/amdgpu/ras_tests.c

这是 libdrm amdgpu 单元测试的一部分，覆GPU 的若干方面。共有四组测试：

RAS 基本测试

该测试验RAS 特性的启用状态，并确保必要的 sysfs debugfs 文件存在
RAS 查询测试

该测试检查每个受支持 IP 块的 RAS 可用性与启用状态，以及错误计数
RAS 注入测试

该测试为每个 IP 注入错误
RAS 禁用测试

该测试测试为每个 IP 块禁RAS 特性