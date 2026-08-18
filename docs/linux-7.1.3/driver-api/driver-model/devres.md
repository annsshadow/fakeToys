锘?# Devres - Managed 璁惧 Resource


Tejun Heo	<teheo@suse.de>

绗竴 draft	10 January 2007


   1. Intro			: Huh? Devres?
   2. Devres			: Devres 鍦?涓€涓?nutshell
   3. Devres Group		: Group devres'es 鍜?閲婃斁 them together
   4. Details			: Life time rules, calling 涓婁笅鏂? ...
   5. Overhead			: 濡備綍 much 鎵ц 鎴戜滑 鍏锋湁 鍒?pay 鐢ㄤ簬 姝?
   6. 鍒楀嚭 鐨?managed interfaces: Currently implemented managed interfaces


### 1. Intro


devres came up 鍚屾椂 trying 鍒?convert libata 鍒?浣跨敤 iomap.  姣忎釜
iomapped 鍦板潃 搴斿綋 涓?kept 鍜?unmapped 鍦?椹卞姩 detach.  鐢ㄤ簬
绀轰緥, 涓€涓?plain SFF ATA 鎺у埗鍣?(鍗? good 鏃?PCI IDE) 鍦?
native 妯″紡 makes 浣跨敤 鐨?5 PCI BARs 鍜?鍏ㄩ儴 鐨?them 搴斿綋 涓?
maintained.

浣滀负 涓?璁稿 鍏朵粬 璁惧 椹卞姩, libata low level 椹卞姩 鍏锋湁
sufficient bugs 鍦?->remove 鍜?->probe failure path.  Well, yes,
璇?s probably 鍥犱负 libata low level 椹卞姩 developers 鏄?lazy
bunch, 浣?aren't 鍏ㄩ儴 low level 椹卞姩 developers?  涔嬪悗 spending 涓€涓?
day fiddling 涓?braindamaged 纭欢 涓?鏃?document 鎴?
braindamaged document, 鑻?瀹?s finally working, well, 瀹?s working.

鐢ㄤ簬 one reason 鎴?another, low level 椹卞姩 don't receive 浣滀负 much
attention 鎴?testing 浣滀负 鏍稿績 code, 鍜?bugs 鍦?椹卞姩 detach 鎴?
鍒濆鍖?failure don't happen 閫氬父 enough 鍒?涓?noticeable.
鍒濆鍖?failure path 鏄?worse 鍥犱负 瀹?s much less travelled 鍚屾椂
needs 鍒?handle 澶氫釜 鏉＄洰 points.

鍥犳, 璁稿 low level 椹卞姩 end up leaking resources 鍦?椹卞姩 detach
鍜?having half broken failure path implementation 鍦?->probe() 鍏?
灏嗕細 leak resources 鎴?even cause oops 褰?failure occurs.  iomap
adds 鏇村 鍒?姝?mix.  鍥犳 鎵ц msi 鍜?msix.


### 2. Devres


devres 鏄?basically linked 鍒楀嚭 鐨?arbitrarily sized 鍐呭瓨 areas
associated 涓?涓€涓?缁撴瀯浣?璁惧.  姣忎釜 devres 鏉＄洰 鏄?associated 涓?
涓€涓?閲婃斁 鍑芥暟.  涓€涓?devres 鍙?涓?released 鍦?鑻ュ共 ways.  鏃?
matter 浠€涔? 鍏ㄩ儴 devres 鏉＄洰 鏄?released 鍦?椹卞姩 detach.  鍦?
閲婃斁, the associated 閲婃斁 鍑芥暟 鏄?invoked 鍜?鐒跺悗 the
devres 鏉＄洰 鏄?freed.

Managed 鎺ュ彛 鏄?宸插垱寤?鐢ㄤ簬 resources commonly 浣跨敤 鐢?璁惧
椹卞姩 浣跨敤 devres.  渚嬪, coherent DMA 鍐呭瓨 鏄?acquired
浣跨敤 dma_alloc_coherent().  The managed 鐗堟湰 鏄?called
dmam_alloc_coherent().  瀹冩槸 identical 鍒?dma_alloc_coherent() except
鐢ㄤ簬 the DMA 鍐呭瓨 allocated 浣跨敤 瀹冩槸 managed 鍜?灏?涓?
automatically released 鍦?椹卞姩 detach.  Implementation looks 绫讳技
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
鑻?涓€涓?椹卞姩 uses dmam_alloc_coherent(), the area 鏄?guaranteed 鍒?涓?
freed 鏄惁 鍒濆鍖?fails half-way 鎴?the 璁惧 gets
detached.  鑻?澶у鏁?resources 鏄?acquired 浣跨敤 managed 鎺ュ彛, 涓€涓?
椹卞姩 鍙?鍏锋湁 much simpler 鍒濆鍖?鍜?exit code.  鍒濆鍖?path basically
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
浣滀负 shown 涓婃枃, low level 椹卞姩 鍙?涓?simplified 涓€涓?lot 鐢?浣跨敤
devres.  Complexity 鏄?shifted 鏉ヨ嚜 less maintained low level 椹卞姩
鍒?better maintained higher layer.  涔? 浣滀负 鍒濆鍖?failure path 鏄?
shared 涓?exit path, 涓よ€?鍙?get 鏇村 testing.

娉ㄦ剰 though 璇?褰?converting 鐢垫祦 calls 鎴?assignments 鍒?
managed devm_* versions 瀹冩槸 up 鍒?鎮?鍒?check 鑻?鍐呴儴 鎿嶄綔
绫讳技 allocating 鍐呭瓨, 鍏锋湁 failed. Managed resources pertains 鍒?the
freeing 鐨?杩欎簺 resources **浠?* - 鍏ㄩ儴 鍏朵粬 checks needed 鏄?浠嶇劧
鍦?鎮? 鍦?涓€浜?cases 姝?鍙?mean introducing checks 璇?鏇炬槸 涓?
蹇呰 涔嬪墠 moving 鍒?the managed devm_* calls.


### 3. Devres group


Devres 鏉＄洰 鍙?涓?grouped 浣跨敤 devres group.  褰?涓€涓?group 鏄?
released, 鍏ㄩ儴 contained 姝ｅ父 devres 鏉＄洰 鍜?properly nested
groups 鏄?released.  One usage 鏄?鍒?rollback 绯诲垪 鐨?acquired
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
浣滀负 resource acquisition failure 閫氬父 means probe failure, constructs
绫讳技 涓婃枃 鏄?閫氬父 useful 鍦?midlayer 椹卞姩 (e.g. libata 鏍稿績
layer) 浣曞 鎺ュ彛 鍑芥暟 shouldn't 鍏锋湁 side effect 鍦?failure.
鐢ㄤ簬 LLDs, just returning 閿欒 code suffices 鍦?澶у鏁?cases.

姣忎釜 group 鏄?identified 鐢?`void *id`.  瀹?鍙?浠讳竴涓?涓?explicitly
specified 鐢?@id 鍙傛暟 鍒?devres_鎵撳紑_group() 鎴?automatically
宸插垱寤?鐢?passing NULL 浣滀负 @id 浣滀负 鍦?the 涓婃枃 绀轰緥.  鍦?涓よ€?
cases, devres_鎵撳紑_group() returns the group's id.  The returned id
鍙?涓?passed 鍒?鍏朵粬 devres 鍑芥暟 鍒?select the target group.
鑻?NULL 鏄?given 鍒?閭ｄ簺 鍑芥暟, the latest 鎵撳紑 group 鏄?
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


Lifetime 鐨?涓€涓?devres 鏉＄洰 begins 鍦?devres 鍒嗛厤 鍜?finishes
褰?瀹冩槸 released 鎴?destroyed (removed 鍜?freed) - 鏃?鍙傝€?
counting.

devres 鏍稿績 guarantees atomicity 鍒?鍏ㄩ儴 鍩烘湰 devres 鎿嶄綔 鍜?
鍏锋湁 鏀寔 鐢ㄤ簬 single-instance devres types (鍘熷瓙
lookup-and-add-if-not-found).  鍏朵粬 姣?璇? synchronizing
concurrent accesses 鍒?allocated devres 鏁版嵁 鏄?caller's
responsibility.  杩欐槸 閫氬父 non-issue 鍥犱负 鎬荤嚎 ops 鍜?
resource allocations 宸茬粡 鎵ц the job.

鐢ㄤ簬 涓€涓?绀轰緥 鐨?single-instance devres 绫诲瀷, 璇诲彇 pcim_iomap_琛?)
鍦?lib/devres.c.

鍏ㄩ儴 devres 鎺ュ彛 鍑芥暟 鍙?涓?called 鏃?涓婁笅鏂?鑻?the
right gfp mask 鏄?given.


### 5. Overhead


姣忎釜 devres bookkeeping info 鏄?allocated together 涓?requested 鏁版嵁
area.  涓?debug 閫夐」 turned off, bookkeeping info occupies 16
bytes 鍦?32浣?machines 鍜?24 bytes 鍦?64浣?(three 鎸囬拡 rounded
up 鍒?ull alignment).  鑻?singly linked 鍒楀嚭 鏄?浣跨敤, 瀹?鍙?涓?
reduced 鍒?two 鎸囬拡 (8 bytes 鍦?32浣? 16 bytes 鍦?64浣?.

姣忎釜 devres group occupies 8 鎸囬拡.  瀹?鍙?涓?reduced 鍒?6 鑻?
singly linked 鍒楀嚭 鏄?浣跨敤.

鍐呭瓨 space overhead 鍦?ahci 鎺у埗鍣?涓?two ports 鏄?涔嬮棿 300
鍜?400 bytes 鍦?32浣?machine 涔嬪悗 naive conversion (鎴戜滑鍙互
certainly invest 涓€涓?浣?鏇村 effort 杩涘叆 libata 鏍稿績 layer).


### 6. 鍒楀嚭 鐨?managed interfaces


CLOCK
  devm_clk_get()
  devm_clk_get_鍙€?)
  devm_clk_put()
  devm_clk_bulk_get()
  devm_clk_bulk_get_鍏ㄩ儴()
  devm_clk_bulk_get_鍙€?)
  devm_get_clk_鏉ヨ嚜_child()
  devm_clk_hw_娉ㄥ唽()
  devm_鐨刜clk_add_hw_provider()
  devm_clk_hw_娉ㄥ唽_clkdev()

DMA
  dmaenginem_async_璁惧_娉ㄥ唽()
  dmam_alloc_coherent()
  dmam_alloc_attrs()
  dmam_free_coherent()
  dmam_pool_鍒涘缓()
  dmam_pool_destroy()

DRM
  devm_drm_dev_alloc()

GPIO
  devm_gpiod_get()
  devm_gpiod_get_鏁扮粍()
  devm_gpiod_get_鏁扮粍_鍙€?)
  devm_gpiod_get_绱㈠紩()
  devm_gpiod_get_绱㈠紩_鍙€?)
  devm_gpiod_get_鍙€?)
  devm_gpiod_put()
  devm_gpiod_unhinge()
  devm_gpiochip_add_鏁版嵁()
  devm_gpio_璇锋眰_one()

I2C
  devm_i2c_add_adapter()
  devm_i2c_鏂癬dummy_璁惧()

IIO
  devm_iio_璁惧_alloc()
  devm_iio_璁惧_娉ㄥ唽()
  devm_iio_dmaengine_缂撳啿鍖篲setup()
  devm_iio_kfifo_缂撳啿鍖篲setup()
  devm_iio_kfifo_缂撳啿鍖篲setup_ext()
  devm_iio_map_鏁扮粍_娉ㄥ唽()
  devm_iio_triggered_缂撳啿鍖篲setup()
  devm_iio_triggered_缂撳啿鍖篲setup_ext()
  devm_iio_trigger_alloc()
  devm_iio_trigger_娉ㄥ唽()
  devm_iio_channel_get()
  devm_iio_channel_get_鍏ㄩ儴()
  devm_iio_hw_consumer_alloc()
  devm_fwnode_iio_channel_get_鐢盻name()

杈撳叆
  devm_杈撳叆_allocate_璁惧()

IO region
  devm_閲婃斁_mem_region()
  devm_閲婃斁_region()
  devm_閲婃斁_resource()
  devm_璇锋眰_mem_region()
  devm_璇锋眰_free_mem_region()
  devm_璇锋眰_region()
  devm_璇锋眰_resource()

IOMAP
  devm_ioport_map()
  devm_ioport_unmap()
  devm_ioremap()
  devm_ioremap_uc()
  devm_ioremap_wc()
  devm_ioremap_resource() : checks resource, requests 鍐呭瓨 region, ioremaps
  devm_ioremap_resource_wc()
  devm_platform_ioremap_resource() : calls devm_ioremap_resource() 鐢ㄤ簬 platform 璁惧
  devm_platform_ioremap_resource_byname()
  devm_platform_get_鍜宊ioremap_resource()
  devm_iounmap()

  娉ㄦ剰: 鐢ㄤ簬 the PCI 璁惧 the 鐗瑰畾 pcim_*() 鍑芥暟 鍙?涓?浣跨敤, 鍙傝 涓嬫枃.

IRQ
  devm_free_irq()
  devm_璇锋眰_浠讳綍_涓婁笅鏂嘷irq()
  devm_璇锋眰_irq()
  devm_璇锋眰_threaded_irq()
  devm_irq_alloc_descs()
  devm_irq_alloc_desc()
  devm_irq_alloc_desc_鍦?)
  devm_irq_alloc_desc_鏉ヨ嚜()
  devm_irq_alloc_descs_鏉ヨ嚜()
  devm_irq_alloc_generic_鑺墖()
  devm_irq_setup_generic_鑺墖()
  devm_irq_domain_鍒涘缓_sim()

LED
  devm_LED_classdev_娉ㄥ唽()
  devm_LED_classdev_娉ㄥ唽_ext()
  devm_LED_classdev_娉ㄩ攢()
  devm_LED_trigger_娉ㄥ唽()
  devm_鐨刜LED_get()

MDIO
  devm_mdiobus_alloc()
  devm_mdiobus_alloc_澶у皬()
  devm_mdiobus_娉ㄥ唽()
  devm_鐨刜mdiobus_娉ㄥ唽()

MEM
  devm_free_椤?)
  devm_get_free_椤?)
  devm_kasprintf()
  devm_kcalloc()
  devm_kfree()
  devm_kmalloc()
  devm_kmalloc_鏁扮粍()
  devm_kmemdup()
  devm_krealloc()
  devm_krealloc_鏁扮粍()
  devm_kstrdup()
  devm_kstrdup_const()
  devm_kvasprintf()
  devm_kzalloc()

MFD
  devm_mfd_add_璁惧()

MUX
  devm_mux_鑺墖_alloc()
  devm_mux_鑺墖_娉ㄥ唽()
  devm_mux_control_get()
  devm_mux_鐘舵€乢get()

NET
  devm_alloc_etherdev()
  devm_alloc_etherdev_mqs()
  devm_娉ㄥ唽_netdev()

PER-CPU MEM
  devm_alloc_percpu()

PCI
  devm_PCI_alloc_host_bridge()  : managed PCI host bridge 鍒嗛厤
  devm_PCI_remap_cfgspace()	: ioremap PCI 閰嶇疆 space
  devm_PCI_remap_cfg_resource()	: ioremap PCI 閰嶇疆 space resource

  pcim_鍚敤_璁惧()		: 涔嬪悗 success, the PCI 璁惧 gets 宸茬鐢?automatically 鍦?椹卞姩 detach
  pcim_iomap()			: 鎵ц iomap() 鍦?涓€涓?鍗曚釜 BAR
  pcim_iomap_regions()		: 鎵ц 璇锋眰_region() 鍜?iomap() 鍦?澶氫釜 BARs
  pcim_iomap_琛?)		: 鏁扮粍 鐨?mapped 鍦板潃 indexed 鐢?BAR
  pcim_iounmap()		: 鎵ц iounmap() 鍦?涓€涓?鍗曚釜 BAR
  pcim_pin_璁惧()		: keep PCI 璁惧 宸插惎鐢?涔嬪悗 閲婃斁
  pcim_set_mwi()		: 鍚敤 Memory-Write-Invalidate PCI transaction

PHY
  devm_USB_get_phy()
  devm_USB_get_phy_鐢盻node()
  devm_USB_get_phy_鐢盻phandle()

PINCTRL
  devm_pinctrl_get()
  devm_pinctrl_put()
  devm_pinctrl_get_select()
  devm_pinctrl_娉ㄥ唽()
  devm_pinctrl_娉ㄥ唽_鍜宊鍒濆鍖?)

鐢垫簮
  devm_reboot_妯″紡_娉ㄥ唽()
  devm_reboot_妯″紡_娉ㄩ攢()

PWM
  devm_pwmchip_alloc()
  devm_pwmchip_add()
  devm_pwm_get()
  devm_fwnode_pwm_get()

REGULATOR
  devm_regulator_bulk_娉ㄥ唽_supply_alias()
  devm_regulator_bulk_get()
  devm_regulator_bulk_get_const()
  devm_regulator_bulk_get_鍚敤()
  devm_regulator_bulk_put()
  devm_regulator_get()
  devm_regulator_get_鍚敤()
  devm_regulator_get_鍚敤_璇诲彇_鐢靛帇()
  devm_regulator_get_鍚敤_鍙€?)
  devm_regulator_get_exclusive()
  devm_regulator_get_鍙€?)
  devm_regulator_irq_helper()
  devm_regulator_put()
  devm_regulator_娉ㄥ唽()
  devm_regulator_娉ㄥ唽_notifier()
  devm_regulator_娉ㄥ唽_supply_alias()
  devm_regulator_娉ㄩ攢_notifier()

RESET
  devm_reset_control_get()
  devm_reset_鎺у埗鍣╛娉ㄥ唽()

RTC
  devm_rtc_璁惧_娉ㄥ唽()
  devm_rtc_allocate_璁惧()
  devm_rtc_娉ㄥ唽_璁惧()
  devm_rtc_nvmem_娉ㄥ唽()

SERDEV
  devm_serdev_璁惧_鎵撳紑()

SLAVE DMA ENGINE
  devm_acpi_dma_鎺у埗鍣╛娉ㄥ唽()

SPI
  devm_spi_alloc_host()
  devm_spi_alloc_target()
  devm_spi_optimize_message()
  devm_spi_娉ㄥ唽_鎺у埗鍣?)
  devm_spi_娉ㄥ唽_host()
  devm_spi_娉ㄥ唽_target()

WATCHDOG
  devm_watchdog_娉ㄥ唽_璁惧()

WORKQUEUE
  devm_alloc_workqueue()
  devm_alloc_ordered_workqueue()
