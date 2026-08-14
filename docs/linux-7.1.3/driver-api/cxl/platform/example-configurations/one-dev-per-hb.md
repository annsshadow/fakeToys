## 姣忎釜涓绘満妗ヤ竴涓澶?


鏈郴缁熸湁涓€涓崟鎻掓Ы锛屽甫涓や釜 CXL 涓绘満妗ャ€傛瘡涓富鏈烘ˉ鏈変竴涓崟鐙殑 CXL 鍐呭瓨鎵╁睍鍣紝鍏锋湁 4GB 鍐呭瓨銆?

闇€瑕佹敞鎰忕殑浜嬮」锛?

- 鏈娇鐢ㄨ法妗ヤ氦閿欙紙Cross-Bridge interleave锛夈€?
- 鎵╁睍鍣ㄤ綅浜庝袱涓嫭绔嬩絾鐩搁偦鐨勫唴瀛樺尯鍩熶腑銆?
- 姝?CEDT/SRAT 涓烘瘡涓澶囨弿杩颁竴涓妭鐐?
- 鎵╁睍鍣ㄥ叿鏈夌浉鍚岀殑鎬ц兘锛屽苟涓斿皢浣嶄簬鍚屼竴鍐呭瓨灞備腑銆?

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
