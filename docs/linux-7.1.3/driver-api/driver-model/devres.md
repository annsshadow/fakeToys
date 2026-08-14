## Devres - Managed 设备 Resource


Tejun Heo	<teheo@suse.de>

第一 draft	10 January 2007


   1. Intro			: Huh? Devres?
   2. Devres			: Devres 在 一个 nutshell
   3. Devres Group		: Group devres'es 和 释放 them together
   4. Details			: Life time rules, calling 上下文, ...
   5. Overhead			: 如何 much 执行 我们 具有 到 pay 用于 此?
   6. 列出 的 managed interfaces: Currently implemented managed interfaces


### 1. Intro


devres came up 同时 trying 到 convert libata 到 使用 iomap.  每个
iomapped 地址 应当 为 kept 和 unmapped 在 驱动 detach.  用于
示例, 一个 plain SFF ATA 控制器 (即, good 旧 PCI IDE) 在
native 模式 makes 使用 的 5 PCI BARs 和 全部 的 them 应当 为
maintained.

作为 与 许多 其他 设备 驱动, libata low level 驱动 具有
sufficient bugs 在 ->remove 和 ->probe failure path.  Well, yes,
该's probably 因为 libata low level 驱动 developers 是 lazy
bunch, 但 aren't 全部 low level 驱动 developers?  之后 spending 一个
day fiddling 与 braindamaged 硬件 与 无 document 或
braindamaged document, 若 它's finally working, well, 它's working.

用于 one reason 或 another, low level 驱动 don't receive 作为 much
attention 或 testing 作为 核心 code, 和 bugs 在 驱动 detach 或
初始化 failure don't happen 通常 enough 到 为 noticeable.
初始化 failure path 是 worse 因为 它's much less travelled 同时
needs 到 handle 多个 条目 points.

因此, 许多 low level 驱动 end up leaking resources 在 驱动 detach
和 having half broken failure path implementation 在 ->probe() 其
将会 leak resources 或 even cause oops 当 failure occurs.  iomap
adds 更多 到 此 mix.  因此 执行 msi 和 msix.


### 2. Devres


devres 是 basically linked 列出 的 arbitrarily sized 内存 areas
associated 与 一个 结构体 设备.  每个 devres 条目 是 associated 与
一个 释放 函数.  一个 devres 可 为 released 在 若干 ways.  无
matter 什么, 全部 devres 条目 是 released 在 驱动 detach.  在
释放, the associated 释放 函数 是 invoked 和 然后 the
devres 条目 是 freed.

Managed 接口 是 已创建 用于 resources commonly 使用 由 设备
驱动 使用 devres.  例如, coherent DMA 内存 是 acquired
使用 dma_alloc_coherent().  The managed 版本 是 called
dmam_alloc_coherent().  它是 identical 到 dma_alloc_coherent() except
用于 the DMA 内存 allocated 使用 它是 managed 和 将 为
automatically released 在 驱动 detach.  Implementation looks 类似
```

  struct dma_devres {
	size_t		size;
	void		*vaddr;
	dma_addr_t	dma_handle;
  };

  static void dmam_coherent_release(struct device *dev, void *res)
  {
	struct dma_devres *this = res;

	dma_free_coherent(dev, this->size, this->vaddr, this->dma_handle);
  }

  dmam_alloc_coherent(dev, size, dma_handle, gfp)
  {
	struct dma_devres *dr;
	void *vaddr;

	dr = devres_alloc(dmam_coherent_release, sizeof(*dr), gfp);
	...

	/* alloc DMA memory as usual */
	vaddr = dma_alloc_coherent(...);
	...

	/* record size, vaddr, dma_handle in dr */
	dr->vaddr = vaddr;
	...

	devres_add(dev, dr);

	return vaddr;
  }

```
若 一个 驱动 uses dmam_alloc_coherent(), the area 是 guaranteed 到 为
freed 是否 初始化 fails half-way 或 the 设备 gets
detached.  若 大多数 resources 是 acquired 使用 managed 接口, 一个
驱动 可 具有 much simpler 初始化 和 exit code.  初始化 path basically
```

  my_init_one()
  {
	struct mydev *d;

	d = devm_kzalloc(dev, sizeof(*d), GFP_KERNEL);
	if (!d)
		return -ENOMEM;

	d->ring = dmam_alloc_coherent(...);
	if (!d->ring)
		return -ENOMEM;

	if (check something)
		return -EINVAL;
	...

	return register_to_upper_layer(d);
  }

```
```

  my_remove_one()
  {
	unregister_from_upper_layer(d);
	shutdown_my_hardware();
  }

```
作为 shown 上文, low level 驱动 可 为 simplified 一个 lot 由 使用
devres.  Complexity 是 shifted 来自 less maintained low level 驱动
到 better maintained higher layer.  也, 作为 初始化 failure path 是
shared 与 exit path, 两者 可 get 更多 testing.

注意 though 该 当 converting 电流 calls 或 assignments 到
managed devm_* versions 它是 up 到 您 到 check 若 内部 操作
类似 allocating 内存, 具有 failed. Managed resources pertains 到 the
freeing 的 这些 resources **仅** - 全部 其他 checks needed 是 仍然
在 您. 在 一些 cases 此 可 mean introducing checks 该 曾是 不
必要 之前 moving 到 the managed devm_* calls.


### 3. Devres group


Devres 条目 可 为 grouped 使用 devres group.  当 一个 group 是
released, 全部 contained 正常 devres 条目 和 properly nested
groups 是 released.  One usage 是 到 rollback 系列 的 acquired
```

  if (!devres_open_group(dev, NULL, GFP_KERNEL))
	return -ENOMEM;

  acquire A;
  if (failed)
	goto err;

  acquire B;
  if (failed)
	goto err;
  ...

  devres_remove_group(dev, NULL);
  return 0;

 err:
  devres_release_group(dev, NULL);
  return err_code;

```
作为 resource acquisition failure 通常 means probe failure, constructs
类似 上文 是 通常 useful 在 midlayer 驱动 (e.g. libata 核心
layer) 何处 接口 函数 shouldn't 具有 side effect 在 failure.
用于 LLDs, just returning 错误 code suffices 在 大多数 cases.

每个 group 是 identified 由 `void *id`.  它 可 任一个 为 explicitly
specified 由 @id 参数 到 devres_打开_group() 或 automatically
已创建 由 passing NULL 作为 @id 作为 在 the 上文 示例.  在 两者
cases, devres_打开_group() returns the group's id.  The returned id
可 为 passed 到 其他 devres 函数 到 select the target group.
若 NULL 是 given 到 那些 函数, the latest 打开 group 是
selected.

```

  int my_midlayer_create_something()
  {
	if (!devres_open_group(dev, my_midlayer_create_something, GFP_KERNEL))
		return -ENOMEM;

	...

	devres_close_group(dev, my_midlayer_create_something);
	return 0;
  }

  void my_midlayer_destroy_something()
  {
	devres_release_group(dev, my_midlayer_create_something);
  }


```
### 4. Details


Lifetime 的 一个 devres 条目 begins 在 devres 分配 和 finishes
当 它是 released 或 destroyed (removed 和 freed) - 无 参考
counting.

devres 核心 guarantees atomicity 到 全部 基本 devres 操作 和
具有 支持 用于 single-instance devres types (原子
lookup-and-add-if-not-found).  其他 比 该, synchronizing
concurrent accesses 到 allocated devres 数据 是 caller's
responsibility.  这是 通常 non-issue 因为 总线 ops 和
resource allocations 已经 执行 the job.

用于 一个 示例 的 single-instance devres 类型, 读取 pcim_iomap_表()
在 lib/devres.c.

全部 devres 接口 函数 可 为 called 无 上下文 若 the
right gfp mask 是 given.


### 5. Overhead


每个 devres bookkeeping info 是 allocated together 与 requested 数据
area.  与 debug 选项 turned off, bookkeeping info occupies 16
bytes 在 32位 machines 和 24 bytes 在 64位 (three 指针 rounded
up 到 ull alignment).  若 singly linked 列出 是 使用, 它 可 为
reduced 到 two 指针 (8 bytes 在 32位, 16 bytes 在 64位).

每个 devres group occupies 8 指针.  它 可 为 reduced 到 6 若
singly linked 列出 是 使用.

内存 space overhead 在 ahci 控制器 与 two ports 是 之间 300
和 400 bytes 在 32位 machine 之后 naive conversion (我们可以
certainly invest 一个 位 更多 effort 进入 libata 核心 layer).


### 6. 列出 的 managed interfaces


CLOCK
  devm_clk_get()
  devm_clk_get_可选()
  devm_clk_put()
  devm_clk_bulk_get()
  devm_clk_bulk_get_全部()
  devm_clk_bulk_get_可选()
  devm_get_clk_来自_child()
  devm_clk_hw_注册()
  devm_的_clk_add_hw_provider()
  devm_clk_hw_注册_clkdev()

DMA
  dmaenginem_async_设备_注册()
  dmam_alloc_coherent()
  dmam_alloc_attrs()
  dmam_free_coherent()
  dmam_pool_创建()
  dmam_pool_destroy()

DRM
  devm_drm_dev_alloc()

GPIO
  devm_gpiod_get()
  devm_gpiod_get_数组()
  devm_gpiod_get_数组_可选()
  devm_gpiod_get_索引()
  devm_gpiod_get_索引_可选()
  devm_gpiod_get_可选()
  devm_gpiod_put()
  devm_gpiod_unhinge()
  devm_gpiochip_add_数据()
  devm_gpio_请求_one()

I2C
  devm_i2c_add_adapter()
  devm_i2c_新_dummy_设备()

IIO
  devm_iio_设备_alloc()
  devm_iio_设备_注册()
  devm_iio_dmaengine_缓冲区_setup()
  devm_iio_kfifo_缓冲区_setup()
  devm_iio_kfifo_缓冲区_setup_ext()
  devm_iio_map_数组_注册()
  devm_iio_triggered_缓冲区_setup()
  devm_iio_triggered_缓冲区_setup_ext()
  devm_iio_trigger_alloc()
  devm_iio_trigger_注册()
  devm_iio_channel_get()
  devm_iio_channel_get_全部()
  devm_iio_hw_consumer_alloc()
  devm_fwnode_iio_channel_get_由_name()

输入
  devm_输入_allocate_设备()

IO region
  devm_释放_mem_region()
  devm_释放_region()
  devm_释放_resource()
  devm_请求_mem_region()
  devm_请求_free_mem_region()
  devm_请求_region()
  devm_请求_resource()

IOMAP
  devm_ioport_map()
  devm_ioport_unmap()
  devm_ioremap()
  devm_ioremap_uc()
  devm_ioremap_wc()
  devm_ioremap_resource() : checks resource, requests 内存 region, ioremaps
  devm_ioremap_resource_wc()
  devm_platform_ioremap_resource() : calls devm_ioremap_resource() 用于 platform 设备
  devm_platform_ioremap_resource_byname()
  devm_platform_get_和_ioremap_resource()
  devm_iounmap()

  注意: 用于 the PCI 设备 the 特定 pcim_*() 函数 可 为 使用, 参见 下文.

IRQ
  devm_free_irq()
  devm_请求_任何_上下文_irq()
  devm_请求_irq()
  devm_请求_threaded_irq()
  devm_irq_alloc_descs()
  devm_irq_alloc_desc()
  devm_irq_alloc_desc_在()
  devm_irq_alloc_desc_来自()
  devm_irq_alloc_descs_来自()
  devm_irq_alloc_generic_芯片()
  devm_irq_setup_generic_芯片()
  devm_irq_domain_创建_sim()

LED
  devm_LED_classdev_注册()
  devm_LED_classdev_注册_ext()
  devm_LED_classdev_注销()
  devm_LED_trigger_注册()
  devm_的_LED_get()

MDIO
  devm_mdiobus_alloc()
  devm_mdiobus_alloc_大小()
  devm_mdiobus_注册()
  devm_的_mdiobus_注册()

MEM
  devm_free_页()
  devm_get_free_页()
  devm_kasprintf()
  devm_kcalloc()
  devm_kfree()
  devm_kmalloc()
  devm_kmalloc_数组()
  devm_kmemdup()
  devm_krealloc()
  devm_krealloc_数组()
  devm_kstrdup()
  devm_kstrdup_const()
  devm_kvasprintf()
  devm_kzalloc()

MFD
  devm_mfd_add_设备()

MUX
  devm_mux_芯片_alloc()
  devm_mux_芯片_注册()
  devm_mux_control_get()
  devm_mux_状态_get()

NET
  devm_alloc_etherdev()
  devm_alloc_etherdev_mqs()
  devm_注册_netdev()

PER-CPU MEM
  devm_alloc_percpu()

PCI
  devm_PCI_alloc_host_bridge()  : managed PCI host bridge 分配
  devm_PCI_remap_cfgspace()	: ioremap PCI 配置 space
  devm_PCI_remap_cfg_resource()	: ioremap PCI 配置 space resource

  pcim_启用_设备()		: 之后 success, the PCI 设备 gets 已禁用 automatically 在 驱动 detach
  pcim_iomap()			: 执行 iomap() 在 一个 单个 BAR
  pcim_iomap_regions()		: 执行 请求_region() 和 iomap() 在 多个 BARs
  pcim_iomap_表()		: 数组 的 mapped 地址 indexed 由 BAR
  pcim_iounmap()		: 执行 iounmap() 在 一个 单个 BAR
  pcim_pin_设备()		: keep PCI 设备 已启用 之后 释放
  pcim_set_mwi()		: 启用 Memory-Write-Invalidate PCI transaction

PHY
  devm_USB_get_phy()
  devm_USB_get_phy_由_node()
  devm_USB_get_phy_由_phandle()

PINCTRL
  devm_pinctrl_get()
  devm_pinctrl_put()
  devm_pinctrl_get_select()
  devm_pinctrl_注册()
  devm_pinctrl_注册_和_初始化()

电源
  devm_reboot_模式_注册()
  devm_reboot_模式_注销()

PWM
  devm_pwmchip_alloc()
  devm_pwmchip_add()
  devm_pwm_get()
  devm_fwnode_pwm_get()

REGULATOR
  devm_regulator_bulk_注册_supply_alias()
  devm_regulator_bulk_get()
  devm_regulator_bulk_get_const()
  devm_regulator_bulk_get_启用()
  devm_regulator_bulk_put()
  devm_regulator_get()
  devm_regulator_get_启用()
  devm_regulator_get_启用_读取_电压()
  devm_regulator_get_启用_可选()
  devm_regulator_get_exclusive()
  devm_regulator_get_可选()
  devm_regulator_irq_helper()
  devm_regulator_put()
  devm_regulator_注册()
  devm_regulator_注册_notifier()
  devm_regulator_注册_supply_alias()
  devm_regulator_注销_notifier()

RESET
  devm_reset_control_get()
  devm_reset_控制器_注册()

RTC
  devm_rtc_设备_注册()
  devm_rtc_allocate_设备()
  devm_rtc_注册_设备()
  devm_rtc_nvmem_注册()

SERDEV
  devm_serdev_设备_打开()

SLAVE DMA ENGINE
  devm_acpi_dma_控制器_注册()

SPI
  devm_spi_alloc_host()
  devm_spi_alloc_target()
  devm_spi_optimize_message()
  devm_spi_注册_控制器()
  devm_spi_注册_host()
  devm_spi_注册_target()

WATCHDOG
  devm_watchdog_注册_设备()

WORKQUEUE
  devm_alloc_workqueue()
  devm_alloc_ordered_workqueue()
