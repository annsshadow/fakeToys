

编写 Virtio 驱动


简介


本文档作为驱动开发人员需要编写新的 virtio 驱动或理解现有驱动要点时的基本指南。有关 virtio 的总体概述，请参阅 Virtio on Linux <virtio>。


驱动样板代码


作为最低要求，virtio 驱动需要在 virtio 总线上注册，并根据设备规范为其配置 virtqueue；驱动侧的 virtqueue 配置必须与设备中的 virtqueue 定义相匹配。一个基本的驱动框架可能如下所示
```

	#include <linux/virtio.h>
	#include <linux/virtio_ids.h>
	#include <linux/virtio_config.h>
	#include <linux/module.h>

	/* device private data (one per device) */
	struct virtio_dummy_dev {
		struct virtqueue *vq;
	};

	static void virtio_dummy_recv_cb(struct virtqueue *vq)
	{
		struct virtio_dummy_dev *dev = vq->vdev->priv;
		char *buf;
		unsigned int len;

		while ((buf = virtqueue_get_buf(dev->vq, &len)) != NULL) {
			/* process the received data */
		}
	}

	static int virtio_dummy_probe(struct virtio_device *vdev)
	{
		struct virtio_dummy_dev *dev = NULL;

		/* initialize device data */
		dev = kzalloc(sizeof(struct virtio_dummy_dev), GFP_KERNEL);
		if (!dev)
			return -ENOMEM;

		/* the device has a single virtqueue */
		dev->vq = virtio_find_single_vq(vdev, virtio_dummy_recv_cb, "input");
		if (IS_ERR(dev->vq)) {
			kfree(dev);
			return PTR_ERR(dev->vq);

		}
		vdev->priv = dev;

		/* from this point on, the device can notify and get callbacks */
		virtio_device_ready(vdev);

		return 0;
	}

	static void virtio_dummy_remove(struct virtio_device *vdev)
	{
		struct virtio_dummy_dev *dev = vdev->priv;

		/*
		 * disable vq interrupts: equivalent to
		 * vdev->config->reset(vdev)
		 */
		virtio_reset_device(vdev);

		/* detach unused buffers */
		while ((buf = virtqueue_detach_unused_buf(dev->vq)) != NULL) {
			kfree(buf);
		}

		/* remove virtqueues */
		vdev->config->del_vqs(vdev);

		kfree(dev);
	}

	static const struct virtio_device_id id_table[] = {
		{ VIRTIO_ID_DUMMY, VIRTIO_DEV_ANY_ID },
		{ 0 },
	};

	static struct virtio_driver virtio_dummy_driver = {
		.driver.name =  KBUILD_MODNAME,
		.id_table =     id_table,
		.probe =        virtio_dummy_probe,
		.remove =       virtio_dummy_remove,
	};

	module_virtio_driver(virtio_dummy_driver);
	MODULE_DEVICE_TABLE(virtio, id_table);
	MODULE_DESCRIPTION("Dummy virtio driver");
	MODULE_LICENSE("GPL");

```
此处的设备 id `VIRTIO_ID_DUMMY` 是一个占位符，virtio 驱动只应为规范中定义的设备添加，请参阅 include/uapi/linux/virtio_ids.h。在将该文件添加设备 id 之前，至少需要在 virtio 规范中预留该 id。

如果您的驱动在其 `init` 与 `exit` 方法中无需做任何特殊处理，可以使用 module_virtio_driver() 辅助宏来减少样板代码的数量。

`probe` 方法在此情况下完成最少的驱动设置（为设备数据分配内存）并初始化 virtqueue。virtio_device_ready() 用于启用 virtqueue，并通知设备驱动已准备好管理该设备（“DRIVER_OK”）。无论如何，virtqueue 都会在 `probe` 返回后由核心自动启用。

   :identifiers: virtio_device_ready

无论如何，在向其添加缓冲区之前必须先启用 virtqueue。

发送与接收数据


上述代码中的 virtio_dummy_recv_cb() 回调会在设备完成处理某个描述符或描述符链（用于读取或写入）并通知驱动时被触发。然而，这只是 virtio 设备-驱动通信过程的后半部分，因为无论数据传输方向如何，通信总是由驱动发起。

要将缓冲区从驱动传输到设备，首先必须根据需要使用 virtqueue_add_inbuf()、virtqueue_add_outbuf() 或 virtqueue_add_sgs() 中的任意一个，将缓冲区——打包为 `scatterlists`——添加到相应的 virtqueue，具体取决于您需要添加一个输入 `scatterlist`（供设备填入）、一个输出 `scatterlist`（供设备消费）还是多个 `scatterlists`。然后，一旦 virtqueue 设置完成，调用 virtqueue_kick() 会发送一个由以下代码处理
```

	struct scatterlist sg[1];
	sg_init_one(sg, buffer, BUFLEN);
	virtqueue_add_inbuf(dev->vq, sg, 1, buffer, GFP_ATOMIC);
	virtqueue_kick(dev->vq);

```
   :identifiers: virtqueue_add_inbuf

   :identifiers: virtqueue_add_outbuf

   :identifiers: virtqueue_add_sgs

随后，在设备读取或写入驱动准备好的缓冲区并回通知后，驱动可以调用 virtqueue_get_buf() 来读取设备产生的数据（如果 virtqueue 是用输入缓冲区设置的），或者仅仅是回收这些已被设备消费的缓冲区：

   :identifiers: virtqueue_get_buf_ctx

virtqueue 回调可以使用 virtqueue_disable_cb() 与一系列的 virtqueue_enable_cb() 函数分别禁用和重新启用。更多细节请参阅 drivers/virtio/virtio_ring.c：

   :identifiers: virtqueue_disable_cb

   :identifiers: virtqueue_enable_cb

但请注意，在某些场景下仍可能触发一些虚假回调。可靠禁用回调的方法是重置设备或 virtqueue（virtio_reset_device()）。


参考文档


_`[^1^]` Virtio 规范 v1.2：
https://docs.oasis-open.org/virtio/virtio/v1.2/virtio-v1.2.html

同时请查看该规范的更高版本。
