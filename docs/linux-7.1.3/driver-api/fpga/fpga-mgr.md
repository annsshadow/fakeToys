## FPGA Manager


### Overview


FPGA manager 核心导出一组用于通过镜像对 FPGA 进行编程的函数。该 API 与厂商无关。所有厂商相关的细节都隐藏在底层驱动中，该驱动向核心注册一组 ops。FPGA 镜像数据本身是与厂商强相关的，但在我们这里它只是二进制数据。FPGA manager 核心不会解析它。

待编程的 FPGA 镜像可以位于分散/聚集（scatter gather）列表、单个连续缓冲区或固件文件中。由于应当避免为缓冲区分配连续的内核内存，因此建议用户尽可能改用分散/聚集列表。

编程镜像的具体参数由一个结构体（struct fpga_image_info）给出。该结构体包含诸如指向 FPGA 镜像的指针，以及镜像特有的参数（例如该镜像是针对完整还是部分重配置而构建的）。

### How to support a new FPGA device


要新增一个 FPGA manager，需编写一个实现了某组 ops 的驱动。其 probe 函数调用 `fpga_mgr_register()` 或 `fpga_mgr_register_full()`，
```

	static const struct fpga_manager_ops socfpga_fpga_ops = {
		.write_init = socfpga_fpga_ops_configure_init,
		.write = socfpga_fpga_ops_configure_write,
		.write_complete = socfpga_fpga_ops_configure_complete,
		.state = socfpga_fpga_ops_state,
	};

	static int socfpga_fpga_probe(struct platform_device *pdev)
	{
		struct device *dev = &pdev->dev;
		struct socfpga_fpga_priv *priv;
		struct fpga_manager *mgr;
		int ret;

		priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
		if (!priv)
			return -ENOMEM;

		/*
		 * do ioremaps, get interrupts, etc. and save
		 * them in priv
		 */

		mgr = fpga_mgr_register(dev, "Altera SOCFPGA FPGA Manager",
					&socfpga_fpga_ops, priv);
		if (IS_ERR(mgr))
			return PTR_ERR(mgr);

		platform_set_drvdata(pdev, mgr);

		return 0;
	}

	static int socfpga_fpga_remove(struct platform_device *pdev)
	{
		struct fpga_manager *mgr = platform_get_drvdata(pdev);

		fpga_mgr_unregister(mgr);

		return 0;
	}

```
另外，probe 函数也可以调用某个资源托管（resource managed）的注册函数 `devm_fpga_mgr_register()` 或 `devm_fpga_mgr_register_full()`。使用这些函数时参数语法相同，但应当去掉对 `fpga_mgr_unregister()` 的调用。在上面的例子中，`socfpga_fpga_remove()` 函数就不再需要了。

ops 将实现针对该特定 FPGA 进行编程序列所需的各种设备相关的寄存器写入。这些 ops 在成功时返回 0，否则返回负的错误码。

```
 1. .parse_header (optional, may be called once or multiple times)
 2. .write_init
 3. .write or .write_sg (may be called once or multiple times)
 4. .write_complete

```
`.parse_header` 函数会把 header_size 和 data_size 设置到 struct fpga_image_info 中。在调用 parse_header 之前，header_size 由 initial_header_size 初始化。如果 fpga_manager_ops 的 skip_header 标志为真，则 `.write` 函数将获得从开头位置 header_size 偏移处开始的镜像缓冲区。如果设置了 data_size，`.write` 函数将获得 data_size 字节的镜像缓冲区，否则 `.write` 将获得直到镜像缓冲区末尾的数据。这不会影响 `.write_sg`，`.write_sg` 仍然以 sg_table 形式获得整个镜像。如果 FPGA 镜像已被映射为单个连续缓冲区，则整个缓冲区会被传入 `.parse_header`。如果镜像以分散/聚集形式存在，核心代码会在第一次调用 `.parse_header` 之前至少缓冲 `.initial_header_size` 大小，如果不够，`.parse_header` 应把期望的大小写入 info->header_size 并返回 -EAGAIN，随后会带着更大的镜像缓冲区部分再次被调用。

`.write_init` 函数用于让 FPGA 准备好接收镜像数据。传入 `.write_init` 的缓冲区至少长 info->header_size 字节；如果整个比特流不能立即可用，核心代码会在开始之前至少缓冲这么多。

`.write` 函数向 FPGA 写入一个缓冲区。该缓冲区可能包含整个 FPGA 镜像，也可能只是 FPGA 镜像的一小段。在后一种情况下，该函数会被多次调用以写入连续的片段。此接口适合使用 PIO 的驱动。

`.write_sg` 版本的行为与 `.write` 相同，只是输入是一个 sg_table 分散列表。此接口适合使用 DMA 的驱动。

`.write_complete` 函数在所有镜像写入完成后被调用，用于将 FPGA 置入工作模式。

ops 还包含一个 `.state` 函数，用于确定 FPGA 所处的状态并返回 enum fpga_mgr_states 类型的代码。它不会导致状态发生改变。

### API for implementing a new FPGA Manager driver


- `fpga_mgr_states` -  :c`fpga_manager->state` 的取值。
- struct fpga_manager -  FPGA manager 结构体
- struct fpga_manager_ops -  底层 FPGA manager 驱动 ops
- struct fpga_manager_info -  fpga_mgr_register_full() 的参数结构体
- __fpga_mgr_register_full() -  使用 fpga_mgr_info 结构体创建并注册一个 FPGA manager，以提供最大灵活度的选项
- __fpga_mgr_register() -  使用标准参数创建并注册一个 FPGA manager
- __devm_fpga_mgr_register_full() -  __fpga_mgr_register_full() 的资源托管版本
- __devm_fpga_mgr_register() -  __fpga_mgr_register() 的资源托管版本
- fpga_mgr_unregister() -  注销一个 FPGA manager

辅助宏 `fpga_mgr_register_full()`、`fpga_mgr_register()`、`devm_fpga_mgr_register_full()` 和 `devm_fpga_mgr_register()` 可用于简化注册过程。

   :functions: fpga_mgr_states

   :functions: fpga_manager

   :functions: fpga_manager_ops

   :functions: fpga_manager_info

   :functions: __fpga_mgr_register_full

   :functions: __fpga_mgr_register

   :functions: __devm_fpga_mgr_register_full

   :functions: __devm_fpga_mgr_register

   :functions: fpga_mgr_unregister
