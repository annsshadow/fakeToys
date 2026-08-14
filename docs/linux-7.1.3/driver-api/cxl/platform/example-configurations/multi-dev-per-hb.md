## 姣忎釜涓绘満妗ユ帴澶氫釜璁惧


鍦ㄦ绀轰緥绯荤粺涓紝鎴戜滑鏈変竴涓崟鎻掓Ы鍜屼竴涓?CXL 涓绘満妗ユ帴銆備富鏈烘ˉ鎺ヤ笂杩炴帴浜嗕袱涓甫鏈?4GB 鐨?CXL 鍐呭瓨鎵╁睍鍣ㄣ€?
娉ㄦ剰浜嬮」锛?
- 妗ユ帴鍐呬氦閿欙紙Intra-Bridge interleave锛夋澶勪笉浣滄弿杩般€?- 杩欎袱涓墿灞曞櫒鐢卞崟涓?CEDT/CFMWS 鎻忚堪銆?- 璇?CEDT/SRAT 涓轰袱涓澶囨弿杩颁簡涓€涓妭鐐广€?- 涓や釜璁惧鐨?HMAT 鍙湁涓€涓?proximity domain銆?
```

            Subtable Type : 00 [CXL Host Bridge Structure]
                 Reserved : 00
                   Length : 0020
   Associated host bridge : 00000007
    Specification version : 00000001
                 Reserved : 00000000
            Register base : 0000010370400000
          Register length : 0000000000010000

            Subtable Type : 01 [CXL Fixed Memory Window Structure]
                 Reserved : 00
                   Length : 002C
                 Reserved : 00000000
      Window base address : 0000001000000000
              Window size : 0000000200000000
 Interleave Members (2^n) : 00
    Interleave Arithmetic : 00
                 Reserved : 0000
              Granularity : 00000000
             Restrictions : 0006
                    QtgId : 0001
             First Target : 00000007

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
                        Entry : 0080
                        Entry : 0100

               Structure Type : 0001 [SLLBI]
                    Data Type : 03   [Bandwidth]
 Target Proximity Domain List : 00000000
 Target Proximity Domain List : 00000001
                        Entry : 1200
                        Entry : 0200

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
  }

```
