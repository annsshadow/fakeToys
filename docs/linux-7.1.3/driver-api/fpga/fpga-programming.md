## 用于 FPGA 编程的内核API


### 概述


用于 FPGA 编程的内核API 是来FPGA manager、bridge（桥）与 region（区域）
API 的组合。实际用于触FPGA 编程的函数是 fpga_region_program_fpga()
fpga_region_program_fpga() 使用FPGA manager bridges 提供的功能。它会：

 - 锁定区域mutex
 - 锁定该区域的 FPGA manager mutex
 - 如果指定了相应方法，则构建一FPGA bridges 列表
 - 禁用这些 bridges
 - 使用通过 :c`fpga_region->info` 传入的信息对 FPGA 进行编程
 - 重新启用这些 bridges
 - 閲婃斁閿。
struct fpga_image_info 指定了要对哪FPGA 镜像进行编程。它fpga_image_info_alloc() 分配/释放，并fpga_image_info_free() 释放
### 如何使用一region 来编FPGA


FPGA region 驱动完成探测（probed）时，它会获得一个指FPGA manager 驱动指针，从而知道要使用哪个 manager。该 region 要么持有一个要在编程期间控制的
bridges 列表，要么持有一个指向某个函数的指针，该函数会：

```

	#include <linux/fpga/fpga-mgr.h>
	#include <linux/fpga/fpga-region.h>

	struct fpga_image_info *info;
	int ret;

	/*
	 * 首先，分配描述要编程FPGA 镜像信息的结构体
	 */
	info = fpga_image_info_alloc(dev);
	if (!info)
		return -ENOMEM;

	/* 按需设置标志，例如： */
	info->flags = FPGA_MGR_PARTIAL_RECONFIG;

	/*
	 * 指明 FPGA 镜像所在位置。下面是伪代码；你将使用这三者之一	 */
	if (image is in a scatter gather table) {

		info->sgt = [your scatter gather table]

	} else if (image is in a buffer) {

		info->buf = [your image buffer]
		info->count = [image buffer size]

	} else if (image is in a firmware file) {

		info->firmware_name = devm_kstrdup(dev, firmware_name,
						   GFP_KERNEL);

	}

	/* info 添加region 并执行编*/
	region->info = info;
	ret = fpga_region_program_fpga(region);

	/* 如果不再需要，释放镜像 info */
	region->info = NULL;
	fpga_image_info_free(info);

	if (ret)
		return ret;

	/* 现在枚举 FPGA 中出现的任何硬件*/

```
### 用于编程 FPGA API


- fpga_region_program_fpga() -  编程一FPGA
- fpga_image_info() -  指定要对哪个 FPGA 镜像进行编程
- fpga_image_info_alloc() -  分配一FPGA 镜像 info 结构- fpga_image_info_free() -  释放一FPGA 镜像 info 结构
   :functions: fpga_region_program_fpga

FPGA Manager 标志

   :doc: FPGA Manager flags

   :functions: fpga_image_info

   :functions: fpga_image_info_alloc

   :functions: fpga_image_info_free
