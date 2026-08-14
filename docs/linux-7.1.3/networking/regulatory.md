
## Linux 无线监管文档


本文档简要介绍 Linux 无线监管基础设施的工作原理。

更新的信息可在项目网页获取：

https://wireless.wiki.kernel.org/en/developers/Regulatory

### 在用户空间维护监管域


由于监管域具有动态性，我们将它们保留在用户空间，并提供一个框架，供用户空间
向内核上传一个监管域，作为所有无线设备都应遵守的中央核心监管域。

### 如何将监管域提供给内核


当监管域首次建立时，内核会请求一个包含所有监管规则的数据库文件
（regulatory.db）。随后在需要查询某个国家的规则时，内核会使用这个数据库。

### 如何将监管域提供给内核（旧 CRDA 方案）


用户空间通过由用户空间代理构建监管域并经由 nl80211 发送，从而将其提供给内核。
内核只会接受预期的监管域。

目前可用的、能完成此任务的用户空间代理是 CRDA——中央监管域代理（central
regulatory domain agent）。其文档见：

https://wireless.wiki.kernel.org/en/developers/Regulatory/CRDA

本质上，当内核知道自己需要一个新监管域时，会发送一个 udev 事件。可以放置一条
udev 规则来触发 crda，为特定的 ISO/IEC 3166 alpha2 发送相应的监管域。

下面是一个可使用的 udev 规则示例：

# Example file, should be put in /etc/udev/rules.d/regulatory.rules
KERNEL=="regulatory*", ACTION=="change", SUBSYSTEM=="platform", RUN+="/sbin/crda"

alpha2 作为环境变量 COUNTRY 传递。

### 谁请求监管域？


- 用户

用户可以使用 iw：

https://wireless.wiki.kernel.org/en/users/Documentation/iw

```
  # set regulatory domain to "Costa Rica"
  iw reg set CR

```
这会请求内核将监管域设置为指定的 alpha2。内核随后会通过发送 uevent，请求
用户空间为该用户指定的 alpha2 提供监管域。

- 用于国家信息元素（Country Information elements）的无线子系统

内核会发送 uevent 通知用户空间需要一个新监管域。随着其集成的加入，会补充更多
内容。

- 驱动

如果驱动确定需要设置特定的监管域，它们可以使用 regulatory_hint() 通知无线核心。
它们有两个选择——要么提供一个 alpha2，以便 crda 能返回该国家的监管域；要么
根据自身内部的定制知识构建自己的监管域，使无线核心能够遵守它。

**大多数**驱动依赖第一种机制，即提供带 alpha2 的监管提示。对于这些驱动，可以
使用一项额外的检查，以基于定制的 EEPROM 监管数据确保合规。驱动可以通过在其
struct wiphy 上注册一个 reg_notifier() 回调来使用这项检查。当核心的监管域发生
变更时会调用此通知函数。驱动可以利用它审查所做的更改，并审查是谁做出的更改
（驱动、用户、国家 IE），然后根据其内部 EEPROM 数据决定允许什么。希望具备全球
漫游能力的设备驱动应使用此回调。随着其支持的启用，本文档会补充更多关于全球
漫游的内容。

提供自身内置监管域的设备驱动不需要回调，因为由它们注册的信道是唯一被允许的
信道，因此**额外**的信道无法被启用。

### 示例代码——驱动提示 alpha2：


本示例来自 zd1211rw 设备驱动。您可以先建立设备 EEPROM 国家/监管域的映射

```
  static struct zd_reg_alpha2_map reg_alpha2_map[] = {
	{ ZD_REGDOMAIN_FCC, "US" },
	{ ZD_REGDOMAIN_IC, "CA" },
	{ ZD_REGDOMAIN_ETSI, "DE" }, /* Generic ETSI, use most restrictive */
	{ ZD_REGDOMAIN_JAPAN, "JP" },
	{ ZD_REGDOMAIN_JAPAN_ADD, "JP" },
	{ ZD_REGDOMAIN_SPAIN, "ES" },
	{ ZD_REGDOMAIN_FRANCE, "FR" },
  };

```
然后您可以定义一个例程，将读取到的 EEPROM 值映射为 alpha2，

```
  static int zd_reg2alpha2(u8 regdomain, char *alpha2)
  {
	unsigned int i;
	struct zd_reg_alpha2_map *reg_map;
		for (i = 0; i < ARRAY_SIZE(reg_alpha2_map); i++) {
			reg_map = &reg_alpha2_map[i];
			if (regdomain == reg_map->reg) {
			alpha2[0] = reg_map->alpha2[0];
			alpha2[1] = reg_map->alpha2[1];
			return 0;
		}
	}
	return 1;
  }

```
最后，如果找到匹配项，您可以向核心提示您发现的 alpha2。您需要在注册 wiphy
之后执行此操作。预期在初始化期间完成。

```
	r = zd_reg2alpha2(mac->regdomain, alpha2);
	if (!r)
		regulatory_hint(hw->wiphy, alpha2);

```
### 示例代码——驱动提供内置监管域：


[注意：此 API 当前不可用，需要时再添加]

如果您有可从驱动获取的监管信息，并且**需要**使用此方式，我们允许您构建一个监管
域结构并将其传递给无线核心。为此，您应 kmalloc() 一个足以容纳监管域结构的内存，
然后填入您的数据。最后只需以该监管域结构为参数调用 regulatory_hint()。

下面是一个简单示例，监管域使用栈缓存。您的实现可能不同（例如改为读取 EEPROM
缓存）。

```
  struct ieee80211_regdomain mydriver_jp_regdom = {
	.n_reg_rules = 3,
	.alpha2 =  "JP",
	//.alpha2 =  "99", /* If I have no alpha2 to map it to */
	.reg_rules = {
		/* IEEE 802.11b/g, channels 1..14 */
		REG_RULE(2412-10, 2484+10, 40, 6, 20, 0),
		/* IEEE 802.11a, channels 34..48 */
		REG_RULE(5170-10, 5240+10, 40, 6, 20,
			NL80211_RRF_NO_IR),
		/* IEEE 802.11a, channels 52..64 */
		REG_RULE(5260-10, 5320+10, 40, 6, 20,
			NL80211_RRF_NO_IR|
			NL80211_RRF_DFS),
	}
  };

```
```

	struct ieee80211_regdomain *rd;
	int size_of_regd;
	int num_rules = mydriver_jp_regdom.n_reg_rules;
	unsigned int i;

	size_of_regd = sizeof(struct ieee80211_regdomain) +
		(num_rules * sizeof(struct ieee80211_reg_rule));

	rd = kzalloc(size_of_regd, GFP_KERNEL);
	if (!rd)
		return -ENOMEM;

	memcpy(rd, &mydriver_jp_regdom, sizeof(struct ieee80211_regdomain));

	for (i=0; i < num_rules; i++)
		memcpy(&rd->reg_rules[i],
		       &mydriver_jp_regdom.reg_rules[i],
		       sizeof(struct ieee80211_reg_rule));
	regulatory_struct_hint(rd);

```
### 静态编译的监管数据库


当某个数据库需要固化进内核时，可以在构建时作为一个固件文件提供，随后被链接进
内核。
