
## 在以非 D0 状态探测设备


## 简介


在某些情况下，如果开启这些设备会带来不良副作用（超出了仅仅开启该设备本身），可能更倾向于在整个系统启动期间让某些设备保持断电。

## 工作原理


_DSC（Device State for Configuration，用于配置的设备状态）对象会求值为一个整数，可用于告诉 Linux 在探测（probe）期间设备允许的最高 D 状态。如果总线驱动通常会将设备置于 D0 状态进行探测，那么对 _DSC 的支持需要内核总线类型的支持。

使用 _DSC 的缺点是，由于设备未被上电，即使设备有问题，驱动很可能也能正常探测，但第一个用户会发现设备不工作，而不是在探测时失败。因此应谨慎使用此特性。

### I²C


如果一个 I²C 驱动通过在 struct i2c_driver.flags 字段中设置 I2C_DRV_ACPI_WAIVE_D0_PROBE 标志来表明其对此的支持，并且 _DSC 对象求值的整数高于设备的 D 状态，则设备将不会在探测时被上电（置于 D0 状态）。

### D 状态


D 状态以及因此 _DSC 的允许值如下所示。关于设备电源状态的更多信息请参阅 [^1^]。


	Number	State	Description
	0	D0	设备完全上电
	1	D1
	2	D2
	3	D3hot
	4	D3cold	Off（关闭）

## 参考


[^1^] https://uefi.org/specifications/ACPI/6.4/02_Definition_of_Terms/Definition_of_Terms.html#device-power-state-definitions

## 示例


一个描述使用 _DSC 对象告知操作系统该设备在探测期间应保持断电的 ACPI 设备的 ASL 示例如下。从示例角度不相关的某些对象已被省略。


	Device (CAM0)
	{
		Name (_HID, "SONY319A")
		Name (_UID, Zero)
		Name (_CRS, ResourceTemplate ()
		{
			I2cSerialBus(0x0020, ControllerInitiated, 0x00061A80,
				     AddressingMode7Bit, "\\_SB.PCI0.I2C0",
				     0x00, ResourceConsumer)
		})
		Method (_DSC, 0, NotSerialized)
		{
			Return (0x4)
		}
	}
