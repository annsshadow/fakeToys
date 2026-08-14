## i2c-nforce2 内核驱动


支持的适配器：
  - nForce2 MCP                10de:0064
  - nForce2 Ultra 400 MCP      10de:0084
  - nForce3 Pro150 MCP         10de:00D4
  - nForce3 250Gb MCP          10de:00E4
  - nForce4 MCP                10de:0052
  - nForce4 MCP-04             10de:0034
  - nForce MCP51               10de:0264
  - nForce MCP55               10de:0368
  - nForce MCP61               10de:03EB
  - nForce MCP65               10de:0446
  - nForce MCP67               10de:0542
  - nForce MCP73               10de:07D8
  - nForce MCP78S              10de:0752
  - nForce MCP79               10de:0AA2

数据手册：
           未公开提供，但似乎与 AMD-8111 SMBus 2.0 适配器相似。

作者：
 - Hans-Frieder Vogt <hfvogt@gmx.net>,
 - Thomas Leibold <thomas@plx.com>,
        - Patrick Dreker <patrick@dreker.de>

### 描述


i2c-nforce2 是 nVidia nForce2 MCP 内置 SMBus 的驱动。

```

  00:01.1 SMBus: nVidia Corporation: Unknown device 0064 (rev a2)
          Subsystem: Asustek Computer, Inc.: Unknown device 0c11
          Flags: 66Mhz, fast devsel, IRQ 5
          I/O ports at c000 [size=32]
          Capabilities: <available only to root>

```
那么此驱动应支持你主板的 SMBus。


### 说明


nForce2 芯片组中的 SMBus 适配器似乎与 AMD-8111 南桥中的 SMBus 2.0 适配器
非常相似。然而，我只能让驱动通过直接 I/O 访问工作，这与 AMD-8111 的 EC
接口不同。在 Asus A7N8X 上测试过。Asus A7N8X 的 ACPI DSDT 表列出了两个
SMBus，两者均受此驱动支持。
