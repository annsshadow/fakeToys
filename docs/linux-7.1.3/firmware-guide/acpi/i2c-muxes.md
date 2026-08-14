## ACPI I2C 多路复用器


描述包含 I2C 多路复用器的 I2C 设备层次结构，需要为每个多路复用器通道提供
一个 ACPI Device () 作用域。

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
