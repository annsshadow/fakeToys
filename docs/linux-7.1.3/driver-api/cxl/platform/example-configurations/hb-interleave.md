
## 跨主机桥交织


该系统为单路插槽，带有两座 CXL 主机桥。每座主机桥各连接一个 4GB 内存的
CXL 内存扩展器。

需要关注的事项：

- 描述了跨桥交织（Cross-Bridge interleave）。
- 两个扩展器由单个 CFMWS 描述。
- 该 SRAT 为两座主机桥描述了同一个节点。
- HMAT 描述了单个节点的性能。

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
              Window size : 0000000200000000
 Interleave Members (2^n) : 01
    Interleave Arithmetic : 00
                 Reserved : 0000
              Granularity : 00000000
             Restrictions : 0006
                    QtgId : 0001
             First Target : 00000007
            Second Target : 00000006

```
```

         Subtable Type : 01 [Memory Affinity]
                Length : 28
      Proximity Domain : 00000001
             Reserved1 : 0000
          Base Address : 0000001000000000
        Address Length : 0000000200000000
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

               Structure Type : 0001 [SLLBI]
                    Data Type : 03   [Bandwidth]
 Target Proximity Domain List : 00000000
 Target Proximity Domain List : 00000001
 Target Proximity Domain List : 00000002
                        Entry : 1200
                        Entry : 0400

```
```

     Signature : "SLIT"    [System Locality Information Table]
    Localities : 0000000000000003
  Locality   0 : 10 20
  Locality   1 : FF 0A

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
