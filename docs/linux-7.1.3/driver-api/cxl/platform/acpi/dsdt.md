## DSDT - 差异化系统描述表（Differentiated System Description Table）


该表描述一台机器拥有哪些外设。

该表中 CXL 设备（特别是主机桥）的 UID 必须与 CEDT 的内容保持一致，否则 CXL 驱动将无法正确探测。

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
