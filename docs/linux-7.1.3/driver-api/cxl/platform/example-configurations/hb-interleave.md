
## 璺ㄤ富鏈烘ˉ浜ょ粐


璇ョ郴缁熶负鍗曡矾鎻掓Ы锛屽甫鏈変袱搴?CXL 涓绘満妗ャ€傛瘡搴т富鏈烘ˉ鍚勮繛鎺ヤ竴涓?4GB 鍐呭瓨鐨?CXL 鍐呭瓨鎵╁睍鍣ㄣ€?
闇€瑕佸叧娉ㄧ殑浜嬮」锛?
- 鎻忚堪浜嗚法妗ヤ氦缁囷紙Cross-Bridge interleave锛夈€?- 涓や釜鎵╁睍鍣ㄧ敱鍗曚釜 CFMWS 鎻忚堪銆?- 璇?SRAT 涓轰袱搴т富鏈烘ˉ鎻忚堪浜嗗悓涓€涓妭鐐广€?- HMAT 鎻忚堪浜嗗崟涓妭鐐圭殑鎬ц兘銆?
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
