
## Flexible Presentation

璇ョ郴缁熸湁涓€涓崟鎻掓Ы锛屽甫涓や釜 CXL 涓绘満妗ャ€傛瘡涓富鏈烘ˉ鏈変袱涓?CXL 鍐呭瓨鎵╁睍鍣紝鍚勫甫 4GB 鍐呭瓨
锛堝叡 32GB锛夈€?

鍦ㄨ绯荤粺涓婏紝骞冲彴璁捐鑰呭笇鏈涗负鐢ㄦ埛鎻愪緵鐏垫椿鎬э紝浠ラ厤缃悇绉嶄氦閿欙紙interleave锛夋垨 NUMA 鑺傜偣
閰嶇疆銆傚洜姝や粬浠彁渚涗簡姣忕缁勫悎銆?

闇€瑕佹敞鎰忕殑浜嬮」锛?

- 璺ㄦˉ浜ら敊锛圕ross-Bridge interleave锛夊湪瑕嗙洊鍏ㄩ儴瀹归噺鐨勪竴涓?CFMWS 涓弿杩般€?
- 姣忎釜涓绘満妗ヤ篃鍒嗗埆鎻忚堪浜嗕竴涓?CFMWS銆?
- 姣忎釜璁惧涔熷垎鍒弿杩颁簡涓€涓?CFMWS銆?
- 璇?SRAT 涓轰笂杩版瘡涓?CFMWS 鎻忚堪浜嗕竴涓妭鐐广€?
- HMAT 鎻忚堪浜?SRAT 涓瘡涓妭鐐圭殑鎬ц兘銆?

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
              Window size : 0000000400000000
 Interleave Members (2^n) : 01
    Interleave Arithmetic : 00
                 Reserved : 0000
              Granularity : 00000000
             Restrictions : 0006
                    QtgId : 0001
             First Target : 00000007
            Second Target : 00000006

            Subtable Type : 01 [CXL Fixed Memory Window Structure]
                 Reserved : 00
                   Length : 002C
                 Reserved : 00000000
      Window base address : 0000002000000000
              Window size : 0000000200000000
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
      Window base address : 0000002200000000
              Window size : 0000000200000000
 Interleave Members (2^n) : 00
    Interleave Arithmetic : 00
                 Reserved : 0000
              Granularity : 00000000
             Restrictions : 0006
                    QtgId : 0001
             First Target : 00000006

            Subtable Type : 01 [CXL Fixed Memory Window Structure]
                 Reserved : 00
                   Length : 002C
                 Reserved : 00000000
      Window base address : 0000003000000000
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
      Window base address : 0000003100000000
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
      Window base address : 0000003200000000
              Window size : 0000000100000000
 Interleave Members (2^n) : 00
    Interleave Arithmetic : 00
                 Reserved : 0000
              Granularity : 00000000
             Restrictions : 0006
                    QtgId : 0001
             First Target : 00000006

            Subtable Type : 01 [CXL Fixed Memory Window Structure]
                 Reserved : 00
                   Length : 002C
                 Reserved : 00000000
      Window base address : 0000003300000000
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
        Address Length : 0000000400000000
             Reserved2 : 00000000
 Flags (decoded below) : 0000000B
             Enabled : 1
       Hot Pluggable : 1
        Non-Volatile : 0

         Subtable Type : 01 [Memory Affinity]
                Length : 28
      Proximity Domain : 00000002
             Reserved1 : 0000
          Base Address : 0000002000000000
        Address Length : 0000000200000000
             Reserved2 : 00000000
 Flags (decoded below) : 0000000B
             Enabled : 1
       Hot Pluggable : 1
        Non-Volatile : 0

         Subtable Type : 01 [Memory Affinity]
                Length : 28
      Proximity Domain : 00000003
             Reserved1 : 0000
          Base Address : 0000002200000000
        Address Length : 0000000200000000
             Reserved2 : 00000000
 Flags (decoded below) : 0000000B
             Enabled : 1
       Hot Pluggable : 1
        Non-Volatile : 0

         Subtable Type : 01 [Memory Affinity]
                Length : 28
      Proximity Domain : 00000004
             Reserved1 : 0000
          Base Address : 0000003000000000
        Address Length : 0000000100000000
             Reserved2 : 00000000
 Flags (decoded below) : 0000000B
             Enabled : 1
       Hot Pluggable : 1
        Non-Volatile : 0

         Subtable Type : 01 [Memory Affinity]
                Length : 28
      Proximity Domain : 00000005
             Reserved1 : 0000
          Base Address : 0000003100000000
        Address Length : 0000000100000000
             Reserved2 : 00000000
 Flags (decoded below) : 0000000B
             Enabled : 1
       Hot Pluggable : 1
        Non-Volatile : 0

         Subtable Type : 01 [Memory Affinity]
                Length : 28
      Proximity Domain : 00000006
             Reserved1 : 0000
          Base Address : 0000003200000000
        Address Length : 0000000100000000
             Reserved2 : 00000000
 Flags (decoded below) : 0000000B
             Enabled : 1
       Hot Pluggable : 1
        Non-Volatile : 0

         Subtable Type : 01 [Memory Affinity]
                Length : 28
      Proximity Domain : 00000007
             Reserved1 : 0000
          Base Address : 0000003300000000
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
 Target Proximity Domain List : 00000003
 Target Proximity Domain List : 00000004
 Target Proximity Domain List : 00000005
 Target Proximity Domain List : 00000006
 Target Proximity Domain List : 00000007
                        Entry : 0080
                        Entry : 0100
                        Entry : 0100
                        Entry : 0100
                        Entry : 0100
                        Entry : 0100
                        Entry : 0100
                        Entry : 0100

               Structure Type : 0001 [SLLBI]
                    Data Type : 03   [Bandwidth]
 Target Proximity Domain List : 00000000
 Target Proximity Domain List : 00000001
 Target Proximity Domain List : 00000002
 Target Proximity Domain List : 00000003
 Target Proximity Domain List : 00000004
 Target Proximity Domain List : 00000005
 Target Proximity Domain List : 00000006
 Target Proximity Domain List : 00000007
                        Entry : 1200
                        Entry : 0400
                        Entry : 0200
                        Entry : 0200
                        Entry : 0100
                        Entry : 0100
                        Entry : 0100
                        Entry : 0100

```
```

     Signature : "SLIT"    [System Locality Information Table]
    Localities : 0000000000000003
  Locality   0 : 10 20 20 20 20 20 20 20
  Locality   1 : FF 0A FF FF FF FF FF FF
  Locality   2 : FF FF 0A FF FF FF FF FF
  Locality   3 : FF FF FF 0A FF FF FF FF
  Locality   4 : FF FF FF FF 0A FF FF FF
  Locality   5 : FF FF FF FF FF 0A FF FF
  Locality   6 : FF FF FF FF FF FF 0A FF
  Locality   7 : FF FF FF FF FF FF FF 0A

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