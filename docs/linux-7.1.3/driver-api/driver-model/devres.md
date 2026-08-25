# Devres - Managed 设备 Resource


Tejun Heo	<teheo@suse.de>

第一 draft	10 January 2007


   1. Intro			: Huh? Devres?
   2. Devres			: Devres 一nutshell
   3. Devres Group		: Group devres'es 鍜，閲婃斁 them together
   4. Details			: Life time rules, calling 上下 ...
   5. Overhead			: 如何 much 执行 我们 具有 pay 用于 
   6. 列出 managed interfaces: Currently implemented managed interfaces


### 1. Intro


devres came up 同时 trying convert libata 使用 iomap.  每个
iomapped 地址 应当 kept unmapped 驱动 detach.  用于
示例, 一plain SFF ATA 控制( good PCI IDE) 
native 模式 makes 使用 5 PCI BARs 全部 them 应当 
maintained.

作为 许多 其他 设备 驱动, libata low level 驱动 具有
sufficient bugs 鍦?->remove 鍜?->probe failure path.  Well, yes,
s probably 因为 libata low level 驱动 developers lazy
bunch, aren't 全部 low level 驱动 developers  之后 spending 一
day fiddling braindamaged 硬件 document 
braindamaged document, 鑻，瀹?s finally working, well, 瀹?s working.

用于 one reason another, low level 驱动 don't receive 作为 much
attention testing 作为 核心 code, bugs 驱动 detach 
初始failure don't happen 通常 enough noticeable.
初始failure path worse 因为 s much less travelled 同时
needs handle 多个 条目 points.

因此, 许多 low level 驱动 end up leaking resources 驱动 detach
鍜?having half broken failure path implementation 鍦?->probe() 鍏。
将会 leak resources even cause oops failure occurs.  iomap
adds 更多 mix.  因此 执行 msi msix.


### 2. Devres


devres basically linked 列出 arbitrarily sized 内存 areas
associated 一结构设备.  每个 devres 条目 associated 
一释放 函数.  一devres released 若干 ways.  
matter 什 全部 devres 条目 released 驱动 detach.  
释放, the associated 释放 函数 invoked 然后 the
devres 条目 freed.

Managed 接口 已创用于 resources commonly 使用 设备
驱动 使用 devres.  例如, coherent DMA 内存 acquired
使用 dma_alloc_coherent().  The managed 版本 called
dmam_alloc_coherent().  它是 identical dma_alloc_coherent() except
用于 the DMA 内存 allocated 使用 它是 managed 
automatically released 驱动 detach.  Implementation looks 类似
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
一驱动 uses dmam_alloc_coherent(), the area guaranteed 
freed 是否 初始fails half-way the 设备 gets
detached.  大多resources acquired 使用 managed 接口, 一
驱动 具有 much simpler 初始exit code.  初始path basically
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
作为 shown 上文, low level 驱动 simplified 一lot 使用
devres.  Complexity shifted 来自 less maintained low level 驱动
better maintained higher layer.   作为 初始failure path 
shared exit path, 两get 更多 testing.

注意 though converting 电流 calls assignments 
managed devm_* versions 它是 up check 内部 操作
类似 allocating 内存, 具有 failed. Managed resources pertains the
freeing 这些 resources *** - 全部 其他 checks needed 仍然
 一cases mean introducing checks 曾是 
必要 之前 moving the managed devm_* calls.


### 3. Devres group


Devres 条目 grouped 使用 devres group.  一group 
released, 全部 contained 正常 devres 条目 properly nested
groups released.  One usage rollback 系列 acquired
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
类似 上文 通常 useful midlayer 驱动 (e.g. libata 核心
layer) 何处 接口 函数 shouldn't 具有 side effect failure.
用于 LLDs, just returning 错误 code suffices 大多cases.

每个 group identified `void *id`.  任一explicitly
specified @id 参数 devres_打开_group() automatically
已创passing NULL 作为 @id 作为 the 上文 示例.  两
cases, devres_打开_group() returns the group's id.  The returned id
passed 其他 devres 函数 select the target group.
NULL given 那些 函数, the latest 打开 group 
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


Lifetime 一devres 条目 begins devres 分配 finishes
它是 released destroyed (removed freed) - 参
counting.

devres 核心 guarantees atomicity 全部 基本 devres 操作 
具有 支持 用于 single-instance devres types (原子
lookup-and-add-if-not-found).  其他  synchronizing
concurrent accesses allocated devres 数据 caller's
responsibility.  这是 通常 non-issue 因为 总线 ops 
resource allocations 已经 执行 the job.

用于 一示例 single-instance devres 类型, 读取 pcim_iomap_)
鍦?lib/devres.c.

全部 devres 接口 函数 called 上下the
right gfp mask 鏄?given.


### 5. Overhead


每个 devres bookkeeping info allocated together requested 数据
area.  debug 选项 turned off, bookkeeping info occupies 16
bytes 32machines 24 bytes 64(three 指针 rounded
up ull alignment).  singly linked 列出 使用, 
reduced two 指针 (8 bytes 32 16 bytes 64.

每个 devres group occupies 8 指针.  reduced 6 
singly linked 列出 使用.

内存 space overhead ahci 控制two ports 之间 300
400 bytes 32machine 之后 naive conversion (我们可以
certainly invest 一更多 effort 进入 libata 核心 layer).


### 6. 列出 managed interfaces


CLOCK
  devm_clk_get()
  devm_clk_get_可)
  devm_clk_put()
  devm_clk_bulk_get()
  devm_clk_bulk_get_全部()
  devm_clk_bulk_get_可)
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
  devm_gpiod_get_数组_可)
  devm_gpiod_get_索引()
  devm_gpiod_get_索引_可)
  devm_gpiod_get_可)
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

  注意: 用于 the PCI 设备 the 特定 pcim_*() 函数 使用, 参见 下文.

IRQ
  devm_free_irq()
  devm_请求_任何_上下文_irq()
  devm_请求_irq()
  devm_请求_threaded_irq()
  devm_irq_alloc_descs()
  devm_irq_alloc_desc()
  devm_irq_alloc_desc_鍦?)
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
  devm_free_椤?)
  devm_get_free_椤?)
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

  pcim_启用_设备()		: 之后 success, the PCI 设备 gets 已禁automatically 驱动 detach
  pcim_iomap()			: 执行 iomap() 一单个 BAR
  pcim_iomap_regions()		: 执行 请求_region() iomap() 多个 BARs
  pcim_iomap_)		: 数组 mapped 地址 indexed BAR
  pcim_iounmap()		: 执行 iounmap() 一单个 BAR
  pcim_pin_设备()		: keep PCI 设备 已启之后 释放
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
  devm_pinctrl_注册_和_初始)

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
  devm_regulator_get_启用_可)
  devm_regulator_get_exclusive()
  devm_regulator_get_可)
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
  devm_spi_娉ㄥ唽_鎺у埗鍣?)
  devm_spi_注册_host()
  devm_spi_注册_target()

WATCHDOG
  devm_watchdog_注册_设备()

WORKQUEUE
  devm_alloc_workqueue()
  devm_alloc_ordered_workqueue()
