
## EDAC/RAS 鐗规€?

Copyright (c) 2024-2025 HiSilicon Limited.

:Author:   Shiju Jose <shiju.jose@huawei.com>
:License:  The GNU Free Documentation License, Version 1.2 without
           Invariant Sections, Front-Cover Texts nor Back-Cover Texts.
           (dual licensed under the GPL v2)

- Written for: 6.15

### 绠€浠?

EDAC/RAS 缁勪欢鐨勬帴鍏ヤ笌楂樺眰璁捐锛?
1. 鎿﹂櫎锛圫crub锛夋帶鍒?
2. 閿欒妫€鏌ユ摝闄わ紙ECS锛夋帶鍒?
3. ACPI RAS2 鐗规€?
4. 灏佽鍚庝慨澶嶏紙PPR锛夋帶鍒?
5. 鍐呭瓨澶囩敤锛圡emory Sparing锛変慨澶嶆帶鍒?
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
1. EDAC 鐗规€х粍浠?- 鍒涘缓鐗瑰畾浜庣壒鎬х殑鎻忚堪绗︺€備緥濡傦細涓婂浘涓殑 scrub銆丒CS銆乵emory repair銆?
2. 鐢ㄤ簬鎺у埗 RAS 鐗规€х殑 EDAC 璁惧椹卞姩 - 浠?EDAC RAS 鐗规€х粍浠惰幏鍙栫壒鎬х殑灞炴€ф弿杩扮锛屽苟灏嗚澶囩殑 RAS
   鐗规€ф敞鍐屽埌 EDAC 鎬荤嚎锛屽苟閫氳繃 sysfs 鏆撮湶鐗规€ф帶鍒跺睘鎬с€備緥濡?/sys/bus/edac/devices/<dev-name>/<feature>X/

3. RAS 鍔ㄦ€佺壒鎬ф帶鍒跺櫒 - rasdaemon 涓殑鐢ㄦ埛绌洪棿绀轰緥妯″潡锛岀敤浜庡姩鎬?scrub/repair 鎺у埗锛屼互渚垮湪鐭椂闂村唴
   鎶ュ憡杩囬噺鏁伴噺鐨勫凡绾犳鍐呭瓨閿欒鏃跺彂鍑烘摝闄?淇銆?
### RAS 鐗规€?

1. 鍐呭瓨鎿﹂櫎锛圡emory Scrub锛?
鍐呭瓨鎿﹂櫎鐗规€ц褰曞湪 `Documentation/edac/scrub.rst` 涓€?
2. 鍐呭瓨淇锛圡emory Repair锛?
鍐呭瓨淇鐗规€ц褰曞湪 `Documentation/edac/memory_repair.rst` 涓€?