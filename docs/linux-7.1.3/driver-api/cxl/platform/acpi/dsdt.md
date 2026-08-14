## DSDT - 宸紓鍖栫郴缁熸弿杩拌〃锛圖ifferentiated System Description Table锛?


璇ヨ〃鎻忚堪涓€鍙版満鍣ㄦ嫢鏈夊摢浜涘璁俱€?

璇ヨ〃涓?CXL 璁惧锛堢壒鍒槸涓绘満妗ワ級鐨?UID 蹇呴』涓?CEDT 鐨勫唴瀹逛繚鎸佷竴鑷达紝鍚﹀垯 CXL 椹卞姩灏嗘棤娉曟纭帰娴嬨€?

```

    Scope (_SB)
    {
        Device (S0D0)
        {
            Name (_HID, "ACPI0016" /* Compute Express Link Host Bridge */)  // _HID: Hardware ID
            Name (_CID, Package (0x02)  // _CID: Compatible ID
            {
                EisaId ("PNP0A08") /* PCI Express Bus */,
                EisaId ("PNP0A03") /* PCI Bus */
            })
            ...
            Name (_UID, 0x05)  // _UID: Unique ID
            ...
      }

```
