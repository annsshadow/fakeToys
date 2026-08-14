## ACPI I2C 澶氳矾澶嶇敤鍣?

鎻忚堪鍖呭惈 I2C 澶氳矾澶嶇敤鍣ㄧ殑 I2C 璁惧灞傛缁撴瀯锛岄渶瑕佷负姣忎釜澶氳矾澶嶇敤鍣ㄩ€氶亾鎻愪緵
涓€涓?ACPI Device () 浣滅敤鍩熴€?
```

    +------+   +------+
    | SMB1 |-->| MUX0 |--CH00--> i2c client A (0x50)
    |      |   | 0x70 |--CH01--> i2c client B (0x50)
    +------+   +------+

```
```

    Device (SMB1)
    {
        Name (_HID, ...)
        Device (MUX0)
        {
            Name (_HID, ...)
            Name (_CRS, ResourceTemplate () {
                I2cSerialBus (0x70, ControllerInitiated, I2C_SPEED,
                            AddressingMode7Bit, "\\_SB.SMB1", 0x00,
                            ResourceConsumer,,)
            }

            Device (CH00)
            {
                Name (_ADR, 0)

                Device (CLIA)
                {
                    Name (_HID, ...)
                    Name (_CRS, ResourceTemplate () {
                        I2cSerialBus (0x50, ControllerInitiated, I2C_SPEED,
                                    AddressingMode7Bit, "\\_SB.SMB1.MUX0.CH00",
                                    0x00, ResourceConsumer,,)
                    }
                }
            }

            Device (CH01)
            {
                Name (_ADR, 1)

                Device (CLIB)
                {
                    Name (_HID, ...)
                    Name (_CRS, ResourceTemplate () {
                        I2cSerialBus (0x50, ControllerInitiated, I2C_SPEED,
                                    AddressingMode7Bit, "\\_SB.SMB1.MUX0.CH01",
                                    0x00, ResourceConsumer,,)
                    }
                }
            }
        }
    }

```
