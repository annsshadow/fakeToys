
## NVMEM 瀛愮郴缁。

 Srinivas Kandagatla <srinivas.kandagatla@linaro.org>

本文档解NVMEM 框架及其提供API，以及如何使用它
## 1. 简
**NVMEM** Non Volatile Memory（非易失性内存）层的缩写。它用于eeprom、efuse 等非
易失性存储器中检SoC 或设备特定的配置数据
在这个框架出现之前，eeprom 这样NVMEM 驱动存放drivers/misc 中，它们都不得不
重复几乎相同的代码来注册一sysfs 文件、允许内核态用户访问它们所驱动设备的内容，等等
就其他内核态用户而言这也是一个问题，因为所使用的方案在不同驱动之间差异很大，存在相严重的抽象泄漏
该框架旨在解决这些问题。它还引入了设备树（DT）表示，供消费者设备从 NVMEM 中获取它所需的数据（MAC 地址、SoC/版本 ID、部件号等）
NVMEM Providers
+++++++++++++++

NVMEM 提供者（provider）指实现了初始化、读取和写入非易失性内存方法的实体
## 2. 娉ㄥ唽/娉ㄩ攢 NVMEM 鎻愪緵鑰。

NVMEM 提供者可以通过nvmem_register() 提供相关nvmem 配置来向 NVMEM 核心注册，成功时
核心会返回一个有效的 nvmem_device 指针
nvmem_unregister() 用于注销先前注册的提供者
```

  static int brcm_nvram_probe(struct platform_device *pdev)
  {
	struct nvmem_config config = {
		.name = "brcm-nvram",
		.reg_read = brcm_nvram_read,
	};
	...
	config.dev = &pdev->dev;
	config.priv = priv;
	config.size = resource_size(res);

	devm_nvmem_register(&config);
  }

```
设备驱动可以使用 nvmem_cell_info 定义并注册一nvmem cell
```

  static const struct nvmem_cell_info foo_nvmem_cell = {
	{
		.name		= "macaddr",
		.offset		= 0x7f00,
		.bytes		= ETH_ALEN,
	}
  };

  int nvmem_add_one_cell(nvmem, &foo_nvmem_cell);

```
此外，还可以创建 nvmem cell 查找项并注册
```

  static struct nvmem_cell_lookup foo_nvmem_lookup = {
	.nvmem_name		= "i2c-eeprom",
	.cell_name		= "macaddr",
	.dev_id			= "foo_mac.0",
	.con_id			= "mac-address",
  };

  nvmem_add_cell_lookups(&foo_nvmem_lookup, 1);

```
NVMEM Consumers
+++++++++++++++

NVMEM 消费者（consumer）是利用 NVMEM 提供者进行读取与写入的实体
## 3. 基于 NVMEM cell 的消费API


NVMEM cell NVMEM 中的数据条目/字段
```

  struct nvmem_cell *nvmem_cell_get(struct device *dev, const char *name);
  struct nvmem_cell *devm_nvmem_cell_get(struct device *dev, const char *name);

  void nvmem_cell_put(struct nvmem_cell *cell);
  void devm_nvmem_cell_put(struct device *dev, struct nvmem_cell *cell);

  void *nvmem_cell_read(struct nvmem_cell *cell, ssize_t *len);
  int nvmem_cell_write(struct nvmem_cell *cell, void *buf, ssize_t len);

```
`*nvmem_cell_get()` API 会获取给id nvmem cell 的引用，随后 nvmem_cell_read/write()
可以读取或写入该 cell。一cell 的使用结束，消费者应调用 `*nvmem_cell_put()` 来释放为cell 分配的所有内存
## 4. 基于直接 NVMEM 设备的消费API


在某些情况下，有必要直接读取/写入 NVMEM
```

  struct nvmem_device *nvmem_device_get(struct device *dev, const char *name);
  struct nvmem_device *devm_nvmem_device_get(struct device *dev,
					   const char *name);
  struct nvmem_device *nvmem_device_find(void *data,
			int (*match)(struct device *dev, const void *data));
  void nvmem_device_put(struct nvmem_device *nvmem);
  int nvmem_device_read(struct nvmem_device *nvmem, unsigned int offset,
		      size_t bytes, void *buf);
  int nvmem_device_write(struct nvmem_device *nvmem, unsigned int offset,
		       size_t bytes, void *buf);
  int nvmem_device_cell_read(struct nvmem_device *nvmem,
			   struct nvmem_cell_info *info, void *buf);
  int nvmem_device_cell_write(struct nvmem_device *nvmem,
			    struct nvmem_cell_info *info, void *buf);

```
在消费者可以直接读写入 NVMEM 之前，它应当通过某个 `*nvmem_device_get()` API 获取
nvmem_controller銆。
这些 API 与基cell API 之间的区别在于，这些 API 总是nvmem_device 作为参数
## 5. 释放NVMEM 的引

当消费者不再需NVMEM 时，它必须释放使用上述章节所API 获取NVMEM 引用
```

  void nvmem_cell_put(struct nvmem_cell *cell);
  void devm_nvmem_cell_put(struct device *dev, struct nvmem_cell *cell);
  void nvmem_device_put(struct nvmem_device *nvmem);
  void devm_nvmem_device_put(struct device *dev, struct nvmem_device *nvmem);

```
这两API 都用于释放对 NVMEM 的引用，devm_nvmem_cell_put devm_nvmem_device_put
会销毁与NVMEM 关联devres
Userspace
+++++++++

## 6. 用户空间二进制接

```

	/sys/bus/nvmem/devices/*/nvmem

```
```

  hexdump /sys/bus/nvmem/devices/qfprom0/nvmem

  0000000 0000 0000 0000 0000 0000 0000 0000 0000
  *
  00000a0 db10 2240 0000 e000 0c00 0c00 0000 0c00
  0000000 0000 0000 0000 0000 0000 0000 0000 0000
  ...
  *
  0001000

```
## 7. 设备树绑

参见 Documentation/devicetree/bindings/nvmem/nvmem.txt

## 8. NVMEM 布局


NVMEM 布局是另一种创cell 的机制。借助设备树绑定，可以通过使用偏移与长度来指定简cell。有时，cell 没有静态偏移，但内容仍然定义良好，例如 tag-length-values。在这种
情况下，必须先解NVMEM 设备的内容，并相应地添加 cell。布局让你能够读取 NVMEM 设备内容，并动态地添加 cell
布局的另一个用例是cell 进行后处理。通过布局，可以将一个自定义的后处理钩子关联到某cell。甚至可以将此钩子添加到并非由布局本身创建cell 上
## 9. 内部内核 API


   :export:
