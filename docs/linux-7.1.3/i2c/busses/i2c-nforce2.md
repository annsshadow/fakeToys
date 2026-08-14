## i2c-nforce2 鍐呮牳椹卞姩


鏀寔鐨勯€傞厤鍣細
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

鏁版嵁鎵嬪唽锛?           鏈叕寮€鎻愪緵锛屼絾浼间箮涓?AMD-8111 SMBus 2.0 閫傞厤鍣ㄧ浉浼笺€?
浣滆€咃細
 - Hans-Frieder Vogt <hfvogt@gmx.net>,
 - Thomas Leibold <thomas@plx.com>,
        - Patrick Dreker <patrick@dreker.de>

### 鎻忚堪


i2c-nforce2 鏄?nVidia nForce2 MCP 鍐呯疆 SMBus 鐨勯┍鍔ㄣ€?
```

  00:01.1 SMBus: nVidia Corporation: Unknown device 0064 (rev a2)
          Subsystem: Asustek Computer, Inc.: Unknown device 0c11
          Flags: 66Mhz, fast devsel, IRQ 5
          I/O ports at c000 [size=32]
          Capabilities: <available only to root>

```
閭ｄ箞姝ら┍鍔ㄥ簲鏀寔浣犱富鏉跨殑 SMBus銆?

### 璇存槑


nForce2 鑺墖缁勪腑鐨?SMBus 閫傞厤鍣ㄤ技涔庝笌 AMD-8111 鍗楁ˉ涓殑 SMBus 2.0 閫傞厤鍣?闈炲父鐩镐技銆傜劧鑰岋紝鎴戝彧鑳借椹卞姩閫氳繃鐩存帴 I/O 璁块棶宸ヤ綔锛岃繖涓?AMD-8111 鐨?EC
鎺ュ彛涓嶅悓銆傚湪 Asus A7N8X 涓婃祴璇曡繃銆侫sus A7N8X 鐨?ACPI DSDT 琛ㄥ垪鍑轰簡涓や釜
SMBus锛屼袱鑰呭潎鍙楁椹卞姩鏀寔銆?