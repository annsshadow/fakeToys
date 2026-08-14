
## CEDT - CXL 早期发现表（CXL Early Discovery Table）


CXL 早期发现表（CXL Early Discovery Table）由 BIOS 生成，用于描述 BIOS 在启动时配置的 CXL 内存区域。

## CHBS


CXL 主机桥结构（CXL Host Bridge Structure）描述 CXL 主机桥。除了描述设备寄存器信息外，它还报告此主机桥特定的主机桥 UID。这些主机桥 ID 将在其他表中被引用。

```

          Subtable Type : 00 [CXL Host Bridge Structure]
               Reserved : 00
                 Length : 0020
 Associated host bridge : 00000007    <- Host bridge _UID
  Specification version : 00000001
               Reserved : 00000000
          Register base : 0000010370400000
        Register length : 0000000000010000

```
## CFMWS


CXL 固定内存窗口结构（CXL Fixed Memory Window structure）描述与一个或多个 CXL 主机桥（如 CHBS 所述）关联的内存区域。此外，它还描述任何可能由 BIOS 编程的主机桥间交错（interleave）配置。

```

            Subtable Type : 01 [CXL Fixed Memory Window Structure]
                 Reserved : 00
                   Length : 002C
                 Reserved : 00000000
      Window base address : 000000C050000000   <- Memory Region
              Window size : 0000003CA0000000
 Interleave Members (2^n) : 01                 <- Interleave configuration
    Interleave Arithmetic : 00
                 Reserved : 0000
              Granularity : 00000000
             Restrictions : 0006
                    QtgId : 0001
             First Target : 00000007           <- Host Bridge _UID
              Next Target : 00000006           <- Host Bridge _UID

```
restriction 字段规定此 SPA 范围可用于什么（内存类型），
```

  Bit[0]: CXL Type 2 Memory
  Bit[1]: CXL Type 3 Memory
  Bit[2]: Volatile Memory
  Bit[3]: Persistent Memory
  Bit[4]: Fixed Config (HPA cannot be reused)

```
主机桥内（intra-host-bridge）交错（一个主机桥上的多个设备）不在此结构中报告，而是完全通过 CXL 设备解码器编程（主机桥与端点解码器）定义。
