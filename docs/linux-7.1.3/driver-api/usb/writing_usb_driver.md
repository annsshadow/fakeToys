
## 编写 USB 设备驱动


:Author: Greg Kroah-Hartman

## 简介


Linux USB 子系统已经从 2.2.7 内核中仅支持两种不同类型的设备（鼠标和键盘），发展到 2.4 内核中支持 20 多种不同类型的设备。Linux 目前支持几乎所有的 USB 类设备（键盘、鼠标、调制解调器、打印机和音箱等标准类型设备），以及数量不断增长的厂商特定设备（如 USB 转串口转换器、数码相机、以太网设备和 MP3 播放器）。有关当前支持的各类 USB 设备的完整列表，请参阅资源（Resources）。

剩余那些在 Linux 上没有支持的 USB 设备，几乎都是厂商特定的设备。每个厂商都决定实现自定义的协议来与它们的设备通信，因此通常需要创建一个自定义驱动。一些厂商对其 USB 协议持开放态度，并协助创建 Linux 驱动，而另一些厂商则不公布它们，开发者被迫进行逆向工程。有关一些方便的逆向工程工具的链接，请参阅资源（Resources）。

由于每一种不同的协议都会导致创建一个新驱动，我编写了一个通用的 USB 驱动框架（skeleton），它仿照内核源码树中的 pci-skeleton.c 文件，许多 PCI 网络驱动都基于该文件。这个 USB 框架可以在内核源码树的 drivers/usb/usb-skeleton.c 中找到。在本文中，我将逐步讲解该框架驱动的基本结构，解释其中的各个部分，以及需要做什么来针对你的特定设备进行定制。


## Linux USB 基础


如果你要编写一个 Linux USB 驱动，请先熟悉 USB 协议规范。它，连同许多其它有用的文档，可以在 USB 主页（参见资源）找到。一篇介绍 Linux USB 子系统的优秀文章可以在 USB 工作设备列表（参见资源）找到。它解释了 Linux USB 子系统是如何组织的，并向读者介绍了 USB urb（USB Request Block，USB 请求块）的概念，这对 USB 驱动至关重要。

Linux USB 驱动需要做的第一件事是向 Linux USB 子系统注册自己，提供一些关于该驱动支持哪些设备、以及在系统插入或移除该驱动所支持的设备时调用哪些函数的信息。所有这些信息都通过 `usb_driver` 结构传递给 USB 子系统
```

    static struct usb_driver skel_driver = {
	    .name        = "skeleton",
	    .probe       = skel_probe,
	    .disconnect  = skel_disconnect,
	    .suspend     = skel_suspend,
	    .resume      = skel_resume,
	    .pre_reset   = skel_pre_reset,
	    .post_reset  = skel_post_reset,
	    .id_table    = skel_table,
	    .supports_autosuspend = 1,
    };


```
变量名（name）是一个描述该驱动字符串。它用于打印到系统日志中的信息性消息。probe 和 disconnect 函数指针在该 `id_table` 变量所提供的信息匹配到的设备被看到或被移除时被调用。

fops 和 minor 变量是可选的。大多数 USB 驱动会挂接到另一个内核子系统，例如 SCSI、网络或 TTY 子系统。这类驱动向另一个内核子系统注册自己，任何用户空间的的交互都通过该接口提供。但对于没有匹配内核子系统的驱动，例如 MP3 播放器或扫描仪，就需要一种与用户空间交互的方法。USB 子系统提供了一种注册次设备号（minor device number）和一组 `file_operations` 函数指针的方式来实现这种用户空间交互。框架驱动需要这类接口，因此它提供了一个次设备起始号以及指向其 `file_operations` 函数的指针。

然后该 USB 驱动通过调用 usb_register() 注册到 USB 子系统，
```

    static int __init usb_skel_init(void)
    {
	    int result;

	    /* register this driver with the USB subsystem */
	    result = usb_register(&skel_driver);
	    if (result < 0) {
		    pr_err("usb_register failed for the %s driver. Error number %d\n",
		           skel_driver.name, result);
		    return -1;
	    }

	    return 0;
    }
    module_init(usb_skel_init);


```
当该驱动从系统中卸载时，它需要向 USB 子系统注销自己。这通过 usb_deregister() 完成
```

    static void __exit usb_skel_exit(void)
    {
	    /* deregister this driver with the USB subsystem */
	    usb_deregister(&skel_driver);
    }
    module_exit(usb_skel_exit);


```
为了启用 linux-hotplug 系统在设备插入时自动加载该驱动，你需要创建一个 `MODULE_DEVICE_TABLE`。以下代码告诉 hotplug 脚本该模块支持
```

    /* table of devices that work with this driver */
    static struct usb_device_id skel_table [] = {
	    { USB_DEVICE(USB_SKEL_VENDOR_ID, USB_SKEL_PRODUCT_ID) },
	    { }                      /* Terminating entry */
    };
    MODULE_DEVICE_TABLE (usb, skel_table);


```
还有其它宏可用于描述支持一整个 USB 驱动类的 struct `usb_device_id`。有关此事的更多信息，请参阅 usb.h <usb_header>。


## 设备操作


当一个与你的驱动向 USB 核心注册的 ID 模式相匹配的设备被插入 USB 总线时，会调用 probe 函数。传递给它的是 `usb_device` 结构、接口号以及
```

    static int skel_probe(struct usb_interface *interface,
	const struct usb_device_id *id)


```
驱动现在需要验证该设备确实是它可以接受的。如果是，它返回 0。如果不是，或者在初始化期间发生任何错误，则 probe 函数返回一个错误码（如 `-ENOMEM` 或 `-ENODEV`）。

在框架驱动中，我们确定哪些端点被标记为批量输入（bulk-in）和批量输出（bulk-out）。我们创建缓冲区来保存将从设备发送和接收的数据，并初始化一个用于向设备写入数据的 USB urb。

相反，当设备从 USB 总线移除时，会调用 disconnect 函数，并传入设备指针。驱动需要清理此时已分配的任何私有数据，并关闭 USB 系统中任何待处理的 urb。

现在设备已插入系统且驱动已绑定到该设备，从用户程序试图与该设备通信时，传递给 USB 子系统的 `file_operations` 结构中的任何函数都将被调用。第一个被调用的函数将是 open，因为程序试图打开该设备进行 I/O。我们递增私有使用计数，并将指向我们内部结构的指针保存到 file 结构中。这样做是为了将来对文件操作的调用能够让驱动确定用户正在寻址的是哪个设备。所有
```

    /* increment our usage count for the device */
    kref_get(&dev->kref);

    /* save our object in the file's private structure */
    file->private_data = dev;


```
在 open 函数被调用之后，会调用 read 和 write 函数来接收和发送数据给设备。在 `skel_write` 函数中，我们接收到用户想要发送给设备的数据指针以及数据大小。该函数根据它已创建的写 urb 的大小（该大小取决于设备所拥有的批量输出端点的大小）来确定它能向设备发送多少数据。然后将数据从用户空间拷贝到内核空间，将 urb 指向该数据，并将 urb 提交给 USB
```

    /* we can only write as much as 1 urb will hold */
    size_t writesize = min_t(size_t, count, MAX_TRANSFER);

    /* copy the data from user space into our urb */
    copy_from_user(buf, user_buffer, writesize);

    /* set up our urb */
    usb_fill_bulk_urb(urb,
		      dev->udev,
		      usb_sndbulkpipe(dev->udev, dev->bulk_out_endpointAddr),
		      buf,
		      writesize,
		      skel_write_bulk_callback,
		      dev);

    /* send the data out the bulk port */
    retval = usb_submit_urb(urb, GFP_KERNEL);
    if (retval) {
	    dev_err(&dev->interface->dev,
                "%s - failed submitting write urb, error %d\n",
                __func__, retval);
    }


```
当写 urb 使用 `usb_fill_bulk_urb` 函数填好适当的信息后，我们将 urb 的完成回调函数指向我们自己的 `skel_write_bulk_callback` 函数。当 urb 被 USB 子系统完成时，会调用该函数。回调函数在中断上下文中被调用，因此必须小心不要在其中做过多的处理。我们的 `skel_write_bulk_callback` 实现只是报告 urb 是否成功完成，然后返回。

读函数的工作方式与写函数略有不同：我们不使用 urb 将数据从设备传输到驱动。相反，我们调用 `usb_bulk_msg` 函数，它可用于向设备发送或接收数据，而无需创建 urb 并处理 urb 完成回调。我们调用 `usb_bulk_msg` 函数，给它一个用于放置从设备接收到的任何数据的缓冲区，以及一个超时值。如果超时期限到期而没有从设备接收到任何数据，该函数将失败并返回
```

    /* do an immediate bulk read to get data from the device */
    retval = usb_bulk_msg (skel->dev,
			   usb_rcvbulkpipe (skel->dev,
			   skel->bulk_in_endpointAddr),
			   skel->bulk_in_buffer,
			   skel->bulk_in_size,
			   &count, 5000);
    /* if the read was successful, copy the data to user space */
    if (!retval) {
	    if (copy_to_user (buffer, skel->bulk_in_buffer, count))
		    retval = -EFAULT;
	    else
		    retval = count;
    }


```
`usb_bulk_msg` 函数对于对设备进行单次读或写非常有用；但是，如果你需要持续地读或写设备，建议建立自己的 urb 并将其提交给 USB 子系统。

当用户程序释放它用于与该设备通信的文件句柄时，会调用驱动中的 release 函数。在该函数中，我们递减私有使用计数，并等待可能的
```

    /* decrement our usage count for the device */
    --skel->open_count;


```
USB 驱动必须能够平滑处理的一个较困难的问题是：USB 设备可能在任何时刻从系统中被移除，即使一个程序当前正在与它通信。它需要能够关闭任何当前的读写，并通知用户空间程序该设备已不再存在。以下代码（函数 `skel_delete`）是一个如何处理
```

    static inline void skel_delete (struct usb_skel *dev)
    {
	kfree (dev->bulk_in_buffer);
	if (dev->bulk_out_buffer != NULL)
	    usb_free_coherent (dev->udev, dev->bulk_out_size,
		dev->bulk_out_buffer,
		dev->write_urb->transfer_dma);
	usb_free_urb (dev->write_urb);
	kfree (dev);
    }


```
如果一个程序当前持有该设备的打开句柄，我们复位 `device_present` 标志。对于每一个期望设备存在的读、写、release 以及其它函数，驱动首先检查该标志以查看设备是否仍然存在。如果不存在，它报告设备已消失，并向用户空间程序返回 `-ENODEV` 错误。当最终调用 release 函数时，它判断是否没有设备，如果是，则执行 `skel_disconnect` 函数在没有打开的文件时通常会做的清理工作（见清单 5）。


## 同步（Isochronous）数据


这个 usb-skeleton 驱动没有任何发送或接收中断数据或同步数据的例子。中断数据的发送几乎与批量数据完全相同，只有一些微小的例外。同步数据的工作方式不同，有连续的数据流被发送或接收。音频和视频相机驱动是处理同步数据的驱动的好例子，如果你也需要做这件事，它们会很有用。


## 结论


如 usb-skeleton 驱动所示，编写 Linux USB 设备驱动并不是一项困难的任务。该驱动，结合当前的其它 USB 驱动，应当提供足够的例子，帮助初学者作者在最短的时间内创建一个可工作的驱动。linux-usb-devel 邮件列表的归档也包含大量有用的信息。


## 资源（Resources）


The Linux USB Project:
http://www.linux-usb.org/

Linux Hotplug Project:
http://linux-hotplug.sourceforge.net/

linux-usb Mailing List Archives:
https://lore.kernel.org/linux-usb/

Programming Guide for Linux USB Device Drivers:
https://lmu.web.psi.ch/docu/manuals/software_manuals/linux_sl/usb_linux_programming_guide.pdf

USB Home Page: https://www.usb.org
