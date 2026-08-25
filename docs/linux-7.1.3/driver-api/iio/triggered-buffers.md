## 触发缓冲区（Triggered Buffers

既然我们已经知道缓冲区和触发器是什么，让我们看看它们如何协同工作
## IIO 触发缓冲区设

- `iio_triggered_buffer_setup` 设置触发缓冲区与 pollfunc
- `iio_triggered_buffer_cleanup` 释放`iio_triggered_buffer_setup` 分配  资源
- struct iio_buffer_setup_ops 与缓冲区设置相关的回
```

    const struct iio_buffer_setup_ops sensor_buffer_setup_ops = {
      .preenable    = sensor_buffer_preenable,
      .postenable   = sensor_buffer_postenable,
      .postdisable  = sensor_buffer_postdisable,
      .predisable   = sensor_buffer_predisable,
    };

    irqreturn_t sensor_iio_pollfunc(int irq, void *p)
    {
        pf->timestamp = iio_get_time_ns((struct indio_dev *)p);
        return IRQ_WAKE_THREAD;
    }

    irqreturn_t sensor_trigger_handler(int irq, void *p)
    {
        u16 buf[8];
        int i = 0;

        /* 读取每个活动通道的数*/
        for_each_set_bit(bit, active_scan_mask, masklength)
            buf[i++] = sensor_get_data(bit)

        iio_push_to_buffers_with_timestamp(indio_dev, buf, timestamp);

        iio_trigger_notify_done(trigger);
        return IRQ_HANDLED;
    }

    /* 设置触发缓冲区，通常probe 函数*/
    iio_triggered_buffer_setup(indio_dev, sensor_iio_polfunc,
                               sensor_trigger_handler,
                               sensor_buffer_setup_ops);

```
这里需要注意的重要事项有：

- `iio_buffer_setup_ops`，缓冲区配置序列中预定义点（例如启用前、禁用后）要调用  缓冲区设置函数。如果未指定，IIO 核心使用默认iio_triggered_buffer_setup_ops- **sensor_iio_pollfunc**，将用作 poll 函数上半部的函数。它应该尽可能少地处理，因为
  它在中断上下文中运行。最常见的操作是记录当前时间戳，因此可以使用 IIO 核心定义  `iio_pollfunc_store_time` 函数- **sensor_trigger_handler**，将用作 poll 函数下半部的函数。它在内核线程的上下文中
  运行，所有处理都在这里进行。它通常从设备读取数据，并与上半部记录的时间戳一  存入内部缓冲区
## 更多细节
