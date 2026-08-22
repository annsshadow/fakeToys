## PMBus 核心驱动与内API


## 简

[from pmbus.org] 电源管理总线（PMBus，Power Management Bus）是一种开放的电源管理协议标准其定义了完整的命令语言，便于与电源系统中的电源转换器件及其他设备进行通信。该协议构建工业标准SMBus 串行接口之上，可对符合规范的电源转换产品进行编程、控制和实时监控。这一
灵活且高度通用的标准支持基于模拟与数字技术设备之间的通信，并提供真正的互操作性，从而降电源系统设计者的设计复杂度并缩短产品上市时间。该开放的电源系统标准由领先的电源与半导体
公司创立，并PMBus 实施者论坛（PMBus-IF，PMBus Implementers Forum）维护与推广，该论坛
包含 30 余家采用方，旨在为用户提供支持并促进其采用
不幸的是，虽PMBus 命令是标准化的，但并没有强制性的命令，制造商可以添加任意多的非标命令。此外，不同PMBus 设备在接收到不支持的命令时行为各异：有些设备返回错误，有些返0xff 0xffff 并设置状态错误标志，还有些设备可能直接挂起
尽管存在上述种种困难，一个通用PMBus 设备驱动仍然有用，并且自内核版本 2.6.39 起得到支持然而，除了核心 PMBus 驱动之外，还必须支持设备特定的扩展，因为 PMBus 设备开发者接下来推出何种新的设备特定功能，目前根本无从得知
为了使设备特定的扩展尽可能具备可扩展性，并避免为新型设备反复修改核心 PMBus 驱动，PMBus
驱动被拆分为核心代码、通用代码和设备特定代码。核心代码（位于 `pmbus_core.c`）提供通用功能通用代码（位`pmbus.c`）提供对通用 PMBus 设备的支持。设备特定代码负责设备特定的初始化，
并在需要时把设备特定功能映射为通用功能。这在某种程度上类似PCI 代码，其中通用代码会根需要针对各类设备以特定quirk（怪癖/兼容性处理）进行扩充
## PMBus 设备能力自动检

对于通用 PMBus 设备，`pmbus.c` 中的代码会尝试自动检测所有受支持PMBus 命令。自动检测在
一定程度上受到限制，因为需要考虑的变量实在太多。例如，几乎不可能自动检测到哪些 PMBus 命令
是分页的、哪些命令在所有页面间被复制（有关多页 PMBus 设备的细节，请参PMBus 规范）
因此，当并非所有命令都能被自动检测时，通常提供一个设备特定的驱动是更合理的做法。该驱动中的
数据结构可用于向核心驱动告知各个芯片所支持的功能
有些命令始终会被自动检测。这适用于所有限制类命令（lcrit、min、max 以及 crit 属性）以及相关报警属性。限制和报警属性被自动检测，是因为可能的组合实在太多，无法提供一个手工配置接口
## PMBus 内部 API


核心代码与设备特PMBus 代码之间API 定义`drivers/hwmon/pmbus/pmbus.h`。除内部 API 外，
`pmbus.h` 还定义了标准 PMBus 命令和虚PMBus 命令
### 标准 PMBus 命令


标准 PMBus 命令（命令0x00 0xff）在 PMBus 规范中定义
### 虚拟 PMBus 命令


提供虚拟 PMBus 命令是为了支持一些已由多个芯片厂商实现、因此值得支持的非标准功能
虚拟 PMBus 命令从命令0x100 开始，因此很容易与标准 PMBus 命令区分开来（标准命令的值不可能
大于 0xff）。虚PMBus 命令的支持是设备特定的，因此必须在设备特定代码中实现
虚拟命令命名`PMBUS_VIRT_xxx`，并`PMBUS_VIRT_BASE` 为起始。所有虚拟命令均为字（word大小
目前有两种类型的虚拟命令
- READ 命令为只读；写入操作要么被忽略，要么返回错误- RESET 命令可读可写。读取复位寄存器返回零（用于检测），写入任意值会导致相关的历史记录被复位
虚拟命令必须在设备特定驱动代码中进行处理。若某虚拟命令受支持，芯片驱动代码返回非负值；若不支持，则返回负的错误码。在这种情况下，芯片驱动可以返回 `-ENODATA` 或任何其Linux 错误码，
不过使用 `-ENODATA` 之外的错误码处理效率更高，因而更受推荐。无论哪种情况，当读取或写入虚拟
寄存器时，若芯片驱动返回错误码，调用PMBus 核心代码都会中止（换句话说，PMBus 核心代码永远
不会向芯片发送虚拟命令）
### PMBus 驱动信息


PMBus 驱动信息定义`struct pmbus_driver_info`，是设备特定驱动向核PMBus 驱动传递信息的
主要手段。具体而言，它提供以下信息
- 对于以支Direct Data Format（直接数据格式）保存其数据的设备，它提供将寄存器值转换为
  规范化数据的系数。这些数据通常由芯片制造商在器件数据手册中提供- 可将芯片所支持的功能告知核心驱动。这对于那些在执行不受支持的命令时会表现异常、和/或为  加速设备检测与初始化的芯片而言可能是必要的- 提供若干函数入口点，用于支持对通用命令执行的覆盖和/或增强。此功能可用于将非标PMBus
  命令映射为标准命令，或者用设备特定信息增强标准命令的返回值
## PEC 支持


许多 PMBus 设备支持 SMBus PEC（Packet Error Checking，包错误检查）。若 I2C 适配器与 PMBus
芯片双方都支持，则默认启用。若支持 PEC，PMBus 核心驱动会向 I2C 设备添加一个名`pec` 的属性该属性可用于控制PMBus 芯片通信时的 PEC 支持
## API 函数


### 芯片驱动提供的函

所有函数若成功则返回命令返回值（读）或零（写）。返回`-ENODATA` 表示该命令没有厂商特实现，但可能存在标准PMBus 命令。任何其他负的返回值表示该命令在此芯片上不存在，且不应再尝读取或写入该标准命令
如上所述，虚拟命令是此规则的一个例外，**必须**在驱动特定代码中处理。更多细节请参阅上文“虚PMBus 命令”
```

	if (chip_access_function) {
		status = chip_access_function();
		if (status != -ENODATA)
			return status;
	}
	if (command >= PMBUS_VIRT_BASE)	/* For word commands/registers only */
		return -EINVAL;
	return generic_access();

```
芯片驱动可在 `struct pmbus_driver_info` 中提供指向以下函数的指针。所有函数均为可选项
```

  int (*read_byte_data)(struct i2c_client *client, int page, int reg);

```
从页`<page>`、寄存器 `<reg>` 读取一个字节`<page>` 可以-1，表示“当前页面”

```

  int (*read_word_data)(struct i2c_client *client, int page, int phase,
                        int reg);

```
从页`<page>`、相`<phase>`、寄存器 `<reg>` 读取一个字。若芯片不支持多相位，则 phase 参数
可被忽略。若芯片支持多相位，相位0xff 表示所有相位
```

  int (*write_word_data)(struct i2c_client *client, int page, int reg,
			 u16 word);

```
向页`<page>`、寄存器 `<reg>` 写入一个字
```

  int (*write_byte)(struct i2c_client *client, int page, u8 value);

```
向页`<page>`、寄存器 `<reg>` 写入一个字节`<page>` 可以-1，表示“当前页面”
```

  int (*identify)(struct i2c_client *client, struct pmbus_driver_info *info);

```
确定所支持PMBus 功能。仅当芯片驱动支持多种芯片、且芯片功能无法预先确定时，此函数才是必需的目前仅由通用 pmbus 驱动（`pmbus.c`）使用
### 核心驱动导出的函

芯片驱动应使用以下函数来读取或写PMBus 寄存器。芯片驱动也可以使用直接I2C 命令。若使用
直接 I2C 命令，芯片驱动代码不得直接修改当前页面，因为所选页面已被缓存在核心驱动中，核心驱动假定该页面已被选中。必须使`pmbus_set_page()` 来选择新页面
```

  int pmbus_set_page(struct i2c_client *client, u8 page, u8 phase);

```
PMBus 页面寄存器设置为 `<page>` `<phase>`，供后续命令使用若芯片不支持多相位，phase 参数被忽略。否则，相位0xff 选择所有相位
```

  int pmbus_read_word_data(struct i2c_client *client, u8 page, u8 phase,
                           u8 reg);

```
`<page>`、`<phase>`、`<reg>` 读取字数据。类似于 `i2c_smbus_read_word_data()`，但会先选择
页面和相位。若芯片不支持多相位，则 phase 参数被忽略。否则，相位0xff 选择所有相位
```

  int pmbus_write_word_data(struct i2c_client *client, u8 page, u8 reg,
			    u16 word);

```
`<page>`、`<reg>` 写入字数据。类似于 `i2c_smbus_write_word_data()`，但会先选择页面
```

  int pmbus_read_byte_data(struct i2c_client *client, int page, u8 reg);

```
`<page>`、`<reg>` 读取字节数据。类似于 `i2c_smbus_read_byte_data()`，但会先选择页面。`<page>`
可以-1，表示“当前页面”
```

  int pmbus_write_byte(struct i2c_client *client, int page, u8 value);

```
`<page>`、`<reg>` 写入字节数据。类似于 `i2c_smbus_write_byte()`，但会先选择页面。`<page>` 可以
-1，表示“当前页面”
```

  void pmbus_clear_faults(struct i2c_client *client);

```
在所有芯片页面上执行 PMBus 的“清除故障（Clear Fault）”命令此函数会调用设备特定write_byte 函数（若已定义）。因此，绝对不能从该函数中调用它
```

  bool pmbus_check_byte_register(struct i2c_client *client, int page, int reg);

```
检查字节寄存器是否存在。若存在则返true，否则返false此函数会调用设备特定write_byte 函数（若已定义）以获取芯片状态。因此，绝对不能从该函数中调用它
```

  bool pmbus_check_word_register(struct i2c_client *client, int page, int reg);

```
检查字寄存器是否存在。若存在则返true，否则返false此函数会调用设备特定write_byte 函数（若已定义）以获取芯片状态。因此，绝对不能从该函数中调用它
```

  int pmbus_do_probe(struct i2c_client *client, struct pmbus_driver_info *info);

```
执行 probe 函数。类似于其他驱动的标probe 函数，但额外带有一个指`struct pmbus_driver_info`
的指针作为参数。若支持 identify 函数则会调用它。只能从设备probe 函数中调用
```

  const struct pmbus_driver_info
	*pmbus_get_driver_info(struct i2c_client *client);

```
返回传入 `pmbus_do_probe()` `struct pmbus_driver_info` 指针

## PMBus 驱动平台数据


PMBus 平台数据定义`include/linux/pmbus.h`。平台数
```

	#define PMBUS_SKIP_STATUS_CHECK			BIT(0)

	#define PMBUS_WRITE_PROTECTED			BIT(1)

	#define PMBUS_NO_CAPABILITY			BIT(2)

	#define PMBUS_READ_STATUS_AFTER_FAILED_CHECK	BIT(3)

	#define PMBUS_NO_WRITE_PROTECT			BIT(4)

	#define PMBUS_USE_COEFFICIENTS_CMD		BIT(5)

	#define PMBUS_OP_PROTECTED			BIT(6)

	#define PMBUS_VOUT_PROTECTED			BIT(7)

	struct pmbus_platform_data {
		u32 flags;              /* Device specific flags */

		/* regulator support */
		int num_regulators;
		struct regulator_init_data *reg_init_data;
	};


```
### 标志

PMBUS_SKIP_STATUS_CHECK

在寄存器检测期间，跳过对状态寄存器通信或命令错误的检查
有些 PMBus 芯片在尝试读取不受支持的寄存器时会返回有效数据。对于此类芯片，在尝试确定某个芯寄存器是否存在时，检查状态寄存器是必需的。另一PMBus 芯片不支STATUS_CML 寄存器，或会无缘无故地报告通信错误。对于此类芯片，必须禁用状态寄存器的检查
有些 i2c 控制器不支持单字节命令（即无数据的写命令 `i2c_smbus_write_byte()`）。对于此类控制器清除状态寄存器是不可能的，因此必须设置 `PMBUS_SKIP_STATUS_CHECK` 标志
PMBUS_WRITE_PROTECTED

若芯片处于写保护状态，且写保护并非由标准的 WRITE_PROTECT 命令决定，则设置此标志
PMBUS_NO_CAPABILITY

有些 PMBus 芯片在读CAPABILITY 寄存器时不会返回有效数据。对于此类芯片，应设置此标志，以PMBus 核心驱动不会使用 CAPABILITY 来判断其行为
PMBUS_READ_STATUS_AFTER_FAILED_CHECK

在每次失败的寄存器检查后读取 STATUS 寄存器
有些 PMBus 芯片在尝试读取不受支持的寄存器时会进入未定义状态。对于此类芯片，在失败的寄存器检后，有必要将芯片PMBus 控制器复位到已知状态。这可以通过读取一个已知寄存器来实现。设置此标志
后，驱动将在每次失败的寄存器检查后尝试读取 STATUS 寄存器。此读取可能失败，但它会把芯片置已知状态
PMBUS_NO_WRITE_PROTECT

有些 PMBus 芯片在读WRITE_PROTECT 寄存器时会返回无效数据。对于此类芯片，应设置此标志，以PMBus 核心驱动不会使用 WRITE_PROTECT 命令来判断其行为
PMBUS_USE_COEFFICIENTS_CMD

设置此标志后，PMBus 核心驱动将使COEFFICIENTS 寄存器来初始direct mode（直接模式）格式
的系数
PMBUS_OP_PROTECTED

若芯片的 OPERATION 命令受保护，且保护并非由标准WRITE_PROTECT 命令决定，则设置此标志
PMBUS_VOUT_PROTECTED

若芯片的 VOUT_COMMAND 命令受保护，且保护并非由标准WRITE_PROTECT 命令决定，则设置此标志
### 模块参数


pmbus_core.wp：PMBus 写保护强制模
PMBus 可能以多种写保护配置启动。`pmbus_core.wp` 可用于在需要特定写保护时使用。实际更改保的能力也可能取决于芯片，因此运行时实际的写保护配置可能不同于所请求的。pmbus_core 当前支持
以下取值：

- 0：移除写保护- 1：禁止所有写入，仅允许对 WRITE_PROTECT、OPERATION、PAGE、ON_OFF_CONFIG VOUT_COMMAND
  命令的写入- 2：禁止所有写入，仅允许对 WRITE_PROTECT、OPERATION PAGE 命令的写入- 3：禁止所有写入，仅允许对 WRITE_PROTECT 命令的写入。注意，保护应包PAGE 寄存器。对于多  芯片，若芯片严格遵循 PMBus 规范，这可能会有问题，因为它会阻止芯片切换活动页面