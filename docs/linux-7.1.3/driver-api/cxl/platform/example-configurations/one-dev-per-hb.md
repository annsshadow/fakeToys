## 每个主机桥一个设


本系统有一个单插槽，带两个 CXL 主机桥。每个主机桥有一个单独的 CXL 内存扩展器，具有 4GB 内存

需要注意的事项

- 未使用跨桥交错（Cross-Bridge interleave）
- 扩展器位于两个独立但相邻的内存区域中
- CEDT/SRAT 为每个设备描述一个节
- 扩展器具有相同的性能，并且将位于同一内存层中

```

            Subtable Type : 00 [CXL Host Bridge Structure]
                 Reserved : 00
                   Length : 0020
   Associated host bridge : 00000007
    Specification version : 00000001
                 Reserved : 00000000
            Register base : 0000010370400000
          Register length : 0000000000010000

            Subtable Type : 00 [CXL Host Bridge Structure]
                 Reserved : 00
                   Length : 0020
   Associated host bridge : 00000006
    Specification version : 00000001
                 Reserved : 00000000
            Register base : 0000010380800000
          Register length : 0000000000010000

            Subtable Type : 01 [CXL Fixed Memory Window Structure]
                 Reserved : 00
                   Length : 002C
                 Reserved : 00000000
      Window base address : 0000001000000000
              Window size : 0000000100000000
 Interleave Members (2^n) : 00
    Interleave Arithmetic : 00
                 Reserved : 0000
              Granularity : 00000000
             Restrictions : 0006
                    QtgId : 0001
             First Target : 00000007

            Subtable Type : 01 [CXL Fixed Memory Window Structure]
                 Reserved : 00
                   Length : 002C
                 Reserved : 00000000
      Window base address : 0000001100000000
              Window size : 0000000100000000
 Interleave Members (2^n) : 00
    Interleave Arithmetic : 00
                 Reserved : 0000
              Granularity : 00000000
             Restrictions : 0006
                    QtgId : 0001
             First Target : 00000006

```
```

         Subtable Type : 01 [Memory Affinity]
                Length : 28
      Proximity Domain : 00000001
             Reserved1 : 0000
          Base Address : 0000001000000000
        Address Length : 0000000100000000
             Reserved2 : 00000000
 Flags (decoded below) : 0000000B
             Enabled : 1
       Hot Pluggable : 1
        Non-Volatile : 0

         Subtable Type : 01 [Memory Affinity]
                Length : 28
      Proximity Domain : 00000002
             Reserved1 : 0000
          Base Address : 0000001100000000
        Address Length : 0000000100000000
             Reserved2 : 00000000
 Flags (decoded below) : 0000000B
             Enabled : 1
       Hot Pluggable : 1
        Non-Volatile : 0

```
```

               Structure Type : 0001 [SLLBI]
                    Data Type : 00   [Latency]
 Target Proximity Domain List : 00000000
 Target Proximity Domain List : 00000001
 Target Proximity Domain List : 00000002
                        Entry : 0080
                        Entry : 0100
                        Entry : 0100

               Structure Type : 0001 [SLLBI]
                    Data Type : 03   [Bandwidth]
 Target Proximity Domain List : 00000000
 Target Proximity Domain List : 00000001
 Target Proximity Domain List : 00000002
                        Entry : 1200
                        Entry : 0200
                        Entry : 0200

```
```

     Signature : "SLIT"    [System Locality Information Table]
    Localities : 0000000000000003
  Locality   0 : 10 20 20
  Locality   1 : FF 0A FF
  Locality   2 : FF FF 0A

```
```

  Scope (_SB)
  {
    Device (S0D0)
    {
        Name (_HID, "ACPI0016" /* Compute Express Link Host Bridge */)  // _HID: Hardware ID
        ...
        Name (_UID, 0x07)  // _UID: Unique ID
    }
    ...
    Device (S0D5)
    {
        Name (_HID, "ACPI0016" /* Compute Express Link Host Bridge */)  // _HID: Hardware ID
        ...
        Name (_UID, 0x06)  // _UID: Unique ID
    }
  }

```
