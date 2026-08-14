
## 用户空间驱动的定时器


:Author: Ivan Orlov <ivan.orlov0322@gmail.com>

## 前言


本文档介绍用户空间驱动的定时器：即可以由用户空间应用程序通过 IOCTL 调用创建并控制的虚拟 ALSA 定时器。
当我们想要将音频流与尚未导出 ALSA 定时器的定时器源（例如 PTP 时钟）同步，或者想要使用 `snd-aloop`
将两个虚拟声设备之间的音频流同步时（例如，有一个网络应用程序向某个 snd-aloop 设备发送帧，而另一个声应用程序监听
snd-aloop 的另一端），这类定时器会很有用。

## 启用用户空间驱动的定时器


用户空间驱动的定时器可以在内核中通过 `CONFIG_SND_UTIMER` 配置选项启用。它依赖于 `CONFIG_SND_TIMER`
选项，因此该选项也应被启用。

## 用户空间驱动的定时器 API


用户空间应用程序可以通过在 `/dev/snd/timer` 设备文件描述符上执行 `SNDRV_TIMER_IOCTL_CREATE` ioctl 调用来创建一个用户空间驱动的 ALSA 定时器。
应传递 `snd_timer_uinfo` 结构体作为 ioctl 参数：

```

    struct snd_timer_uinfo {
        __u64 resolution;
        int fd;
        unsigned int id;
        unsigned char reserved[16];
    }

```
`resolution` 字段以纳秒为单位设置虚拟定时器期望的分辨率。`resolution` 字段只是提供关于虚拟定时器的信息，
并不影响计时本身。`id` 字段会被 ioctl 覆盖，调用后该字段中得到的标识符可以在将定时器传递给 `snd-aloop`
内核模块或其他用户空间应用程序时用作定时器子设备编号。系统中某一时刻最多可存在 128 个用户空间驱动的定时器，
因此 id 的取值范围为 0 到 127。

除了覆盖 `snd_timer_uinfo` 结构体之外，ioctl 还会将一个可用于触发该定时器的定时器文件描述符存储在
`snd_timer_uinfo` 结构体的 `fd` 字段中。为定时器分配一个文件描述符，可以保证该定时器只能由其创建进程触发。
随后可以通过对该定时器文件描述符执行 `SNDRV_TIMER_IOCTL_TRIGGER` ioctl 调用来触发定时器。

因此，创建并触发定时器的示例代码为：

```

    static struct snd_timer_uinfo utimer_info = {
        /* 定时器将（大概）每 1000000 ns 触发一次 */
        .resolution = 1000000ULL,
        .id = -1,
    };

    int timer_device_fd = open("/dev/snd/timer",  O_RDWR | O_CLOEXEC);

    if (ioctl(timer_device_fd, SNDRV_TIMER_IOCTL_CREATE, &utimer_info)) {
        perror("Failed to create the timer");
        return -1;
    }

    ...

    /*
     * 现在我们想要触发定时器。绑定到该定时器的所有
     * 定时器实例的回调将在本次调用之后被执行。
     */
    ioctl(utimer_info.fd, SNDRV_TIMER_IOCTL_TRIGGER, NULL);

    ...

    /* 现在销毁定时器 */
    close(timer_info.fd);


```
关于创建并驱动定时器的更详细示例，可在 utimer ALSA 自测中找到。

### 用户空间驱动的定时器与 snd-aloop


在同步虚拟声回环两端的两个声应用程序时，用户空间驱动的定时器可以很容易地与 `snd-aloop` 模块配合使用。
例如，如果其中一个应用程序从网络接收声帧并将其发送到 snd-aloop 的 pcm 设备，而另一个应用程序在另一个 snd-aloop 的
pcm 设备上监听帧，那么合理的做法是：ALSA 中间层应在通过网络接收到新一-period 数据时发起一次数据传输，
而不是在某个 jiffies 数量耗尽时发起。用户空间驱动的 ALSA 定时器可用于实现这一点。

要将用户空间驱动的 ALSA 定时器用作 snd-aloop 的定时器源，请将以下字符串作为 snd-aloop 的 `timer_source` 参数传递：

```

  # modprobe snd-aloop timer_source="-1.4.<utimer_id>"

```
其中 `utimer_id` 是你用 `SNDRV_TIMER_IOCTL_CREATE` 创建的定时器 id，而 `4` 是
用户空间驱动定时器设备的编号（`SNDRV_TIMER_GLOBAL_UDRIVEN`）。

用于 snd-aloop 的用户空间驱动 ALSA 定时器的 `resolution` 应计算为 `1000000000ULL / frame_rate * period_size`，
因为定时器将在每准备好一个新-period 的帧时触发一次。

之后，每当你用 `SNDRV_TIMER_IOCTL_TRIGGER` 触发定时器时，新一-period 的数据就会从一个 snd-aloop 设备传输到另一个。
