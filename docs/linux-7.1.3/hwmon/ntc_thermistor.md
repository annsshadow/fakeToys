## 内核驱动 ntc_thermistor


来自 Murata 的受支持热敏电阻
- Murata NTC 热敏电阻 NCP15WB473、NCP18WB473、NCP21WB473、NCP03WB473  NCP15WL333、NCP03WF104、NCP15XH103

  Prefixes: 'ncp15wb473', 'ncp18wb473', 'ncp21wb473', 'ncp03wb473',
  'ncp15wl333', 'ncp03wf104', 'ncp15xh103'

  Datasheet: 可在 Murata 公开获取

来自 EPCOS 的受支持热敏电阻
- EPCOS NTC 热敏电阻 B57330V2103

  Prefixes: b57330v2103

  Datasheet: 可在 EPCOS 公开获取

其他 NTC 热敏电阻只需通过添加补偿表即可支持；例如，NCP15WL333 的支持是通过
ncpXXwl333 表添加的
Authors:

	MyungJoo Ham <myungjoo.ham@samsung.com>

### 描述


NTC（负温度系数）热敏电阻是一种简单的热敏电阻，要求用户提供电阻值并查找相应的补偿表
以获得温度输入
NTC 驱动提供带有线性近似函数的查找表，以及四种电路模型，并可选不使用其中任何一种模型
```

   $	resistor
   [TH]	the thermistor

```
The four circuit models provided are:

```

     [pullup_uV]
	 |    |
	[TH]  $ (pullup_ohm)
	 |    |
	 +----+-----------------------[read_uV]
	 |
	 $ (pulldown_ohm)
	 |
	-+- (ground)

```
```

     [pullup_uV]
	 |
	[TH]
	 |
	 +----------------------------[read_uV]
	 |
	 $ (pulldown_ohm)
	 |
	-+- (ground)

```
```

     [pullup_uV]
	 |
	 $ (pullup_ohm)
	 |
	 +----+-----------------------[read_uV]
	 |    |
	[TH]  $ (pulldown_ohm)
	 |    |
	-+----+- (ground)

```
```

     [pullup_uV]
	 |
	 $ (pullup_ohm)
	 |
	 +----------------------------[read_uV]
	 |
	[TH]
	 |
	-+- (ground)

```
When one of the four circuit models is used, read_uV, pullup_uV, pullup_ohm,
pulldown_ohm, and connect should be provided. When none of the four models
are suitable or the user can get the resistance directly, the user should
provide read_ohm and _not_ provide the others.

### Sysfs 接口


=============== == =============================================================
name		   必填的全局属性，即热敏电阻的名称=============== == =============================================================
temp1_type	RO 始终4（热敏电阻）

temp1_input	RO 测量温度并提供测得的值		   （读取此文件会启动读取过程。）
=============== == =============================================================

注意每个 NTC 热敏电阻只有一个热敏电阻；因此只存temp1