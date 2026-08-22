
## EDAC/RAS 特

Copyright (c) 2024-2025 HiSilicon Limited.

:Author:   Shiju Jose <shiju.jose@huawei.com>
:License:  The GNU Free Documentation License, Version 1.2 without
           Invariant Sections, Front-Cover Texts nor Back-Cover Texts.
           (dual licensed under the GPL v2)

- Written for: 6.15

### 简

EDAC/RAS 组件的接入与高层设计
1. 擦除（Scrub）控
2. 错误检查擦除（ECS）控
3. ACPI RAS2 特
4. 封装后修复（PPR）控
5. 内存备用（Memory Sparing）修复控
```

        +-----------------------------------------------+
        |   Userspace - Rasdaemon                       |
        | +-------------+                               |
        | | RAS CXL mem |     +---------------+         |
        | |error handler|---->|               |         |
        | +-------------+     | RAS dynamic   |         |
        | +-------------+     | scrub, memory |         |
        | | RAS memory  |---->| repair control|         |
        | |error handler|     +----|----------+         |
        | +-------------+          |                    |
        +--------------------------|--------------------+
                                   |
                                   |
   +-------------------------------|------------------------------+
   |     Kernel EDAC extension for | controlling RAS Features     |
   |+------------------------------|----------------------------+ |
   || EDAC Core          Sysfs EDAC| Bus                        | |
   ||   +--------------------------|---------------------------+| |
   ||   |/sys/bus/edac/devices/<dev>/scrubX/ |   | EDAC device || |
   ||   |/sys/bus/edac/devices/<dev>/ecsX/   |<->| EDAC MC     || |
   ||   |/sys/bus/edac/devices/<dev>/repairX |   | EDAC sysfs  || |
   ||   +---------------------------|--------------------------+| |
   ||                           EDAC|Bus                        | |
   ||                               |                           | |
   ||   +----------+ Get feature    |      Get feature          | |
   ||   |          | desc +---------|------+ desc +----------+  | |
   ||   |EDAC scrub|<-----| EDAC device    |      |          |  | |
   ||   +----------+      | driver- RAS    |----->| EDAC mem |  | |
   ||   +----------+      | feature control|      | repair   |  | |
   ||   |          |<-----|                |      +----------+  | |
   ||   |EDAC ECS  |      +---------|------+                    | |
   ||   +----------+    Register RAS|features                   | |
   ||         ______________________|_____________              | |
   |+---------|---------------|------------------|--------------+ |
   |  +-------|----+  +-------|-------+     +----|----------+     |
   |  |            |  | CXL mem driver|     | Client driver |     |
   |  | ACPI RAS2  |  | scrub, ECS,   |     | memory repair |     |
   |  | driver     |  | sparing, PPR  |     | features      |     |
   |  +-----|------+  +-------|-------+     +------|--------+     |
   |        |                 |                    |              |
   +--------|-----------------|--------------------|--------------+
            |                 |                    |
   +--------|-----------------|--------------------|--------------+
   |    +---|-----------------|--------------------|-------+      |
   |    |                                                  |      |
   |    |            Platform HW and Firmware              |      |
   |    +--------------------------------------------------+      |
   +--------------------------------------------------------------+


```
1. EDAC 特性组- 创建特定于特性的描述符。例如：上图中的 scrub、ECS、memory repair
2. 用于控制 RAS 特性的 EDAC 设备驱动 - EDAC RAS 特性组件获取特性的属性描述符，并将设备的 RAS
   特性注册到 EDAC 总线，并通过 sysfs 暴露特性控制属性。例/sys/bus/edac/devices/<dev-name>/<feature>X/

3. RAS 动态特性控制器 - rasdaemon 中的用户空间示例模块，用于动scrub/repair 控制，以便在短时间内
   报告过量数量的已纠正内存错误时发出擦修复
### RAS 特

1. 内存擦除（Memory Scrub
内存擦除特性记录在 `Documentation/edac/scrub.rst` 中
2. 内存修复（Memory Repair
内存修复特性记录在 `Documentation/edac/memory_repair.rst` 中