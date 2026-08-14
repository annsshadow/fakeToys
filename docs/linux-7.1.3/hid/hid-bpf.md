
## HID-BPF


HID 是输入设备的标准协议，但某些设备可能需要定制化调整，传统上通过内核驱动修复来完成。改用 eBPF 能力可以加速开发，并为现有 HID 接口增添新能力。

    :local:
    :depth: 2


## 何时（以及为何）使用 HID-BPF


在以下若干场景下，使用 HID-BPF 优于标准的内核驱动修复：

### 游戏手柄的死区

假设你的手柄已经比较老旧，很容易看到它在中立点附近抖动。这通常在应用层通过为该特定轴添加**死区**来过滤。

借助 HID-BPF，我们可以直接在内核中进行这类过滤，这样当输入控制器上没有其它事件发生时，就不会唤醒用户空间。

当然，由于该死区是针对具体某个设备的，我们无法为所有同型号手柄创建一个通用的修复。为此新增一个内核 API（例如新增一个 sysfs 项）并不能保证这个新的内核 API 会被广泛采用和维护。

HID-BPF 允许用户空间程序自行加载该程序，确保我们只在有用户时才加载该自定义 API。

### 报告描述符的简单修正

在 HID 代码树中，有一半的驱动仅仅是为了修正报告描述符中的一个按键或一个字节。这些修复都需要内核补丁，并随后经历进入发行版的漫长过程，对用户而言既漫长又痛苦。

我们可以改为提供一个 eBPF 程序来减轻这种负担。一旦该程序被用户验证通过，我们就可以把源代码嵌入内核树，直接随内核发布并加载该 eBPF 程序，而无需为它加载特定的内核模块。

注意：eBPF 程序的分发及其纳入内核尚未完全实现。

### 新增需要新内核 API 的特性

这类特性的一个例子是 USI（Universal Stylus Interface，通用触控笔接口）触控笔。基本上，USI 触控笔需要一个新的内核 API，因为它有我们的 HID 与输入协议栈不支持的新通信通道。与其使用 hidraw、创建新的 sysfs 项或 ioctl，我们可以依赖 eBPF，使内核 API 由使用者控制，并且不会因每次事件都唤醒用户空间而影响性能。

### 将设备变形为其他形态并从用户空间控制

内核将 HID 项到 evdev 位的映射相对静态。它无法动态地把某个给定设备转换成其他形态，因为它缺少所需的上下文，而且任何这样的转换都无法被用户空间撤销（甚至发现）。

然而，某些设备在这种静态定义方式下毫无用处。例如，Microsoft Surface Dial 是一个带触觉反馈的按钮，目前几乎不可用。

借助 eBPF，用户空间可以把该设备变形为鼠标，并将拨盘事件转换为滚轮事件。此外，用户空间程序可以根据上下文设置/取消触觉反馈。例如，如果屏幕上可见某个菜单，我们可能需要在每 15 度产生一次触觉点击；而在网页中滚动时，设备在最高分辨率下发出事件会带来更好的用户体验。

### 防火墙

如果我们想阻止其他用户访问设备的某个特定功能怎么办？（想想某个可能已损坏的固件更新入口点。）

借助 eBPF，我们可以拦截发往设备的任何 HID 命令，并决定是否放行。

这也允许在用户空间与内核/BPF 程序之间同步状态，因为我们可以拦截任何传入的命令。

### 追踪

最后的用途是追踪事件，以及我们借助 BPF 对事件进行汇总和分析所能做的种种有趣之事。

目前，追踪依赖 hidraw。它工作良好，但存在几个问题：

1. 如果驱动没有导出 hidraw 节点，我们就无法追踪任何内容（eBPF 在那里将处于“上帝模式”，这可能会引起一些人的顾虑）。
2. hidraw 无法捕获其他进程对设备的请求，这意味着我们有时需要在内核中添加 printk 才能弄清楚发生了什么。

## HID-BPF 的高层视图


HID-BPF 背后的核心思想是它在字节数组层面工作。因此，对 HID 报告和 HID 报告描述符的所有解析都必须在加载 eBPF 程序的用户空间组件中实现。

例如，在上面提到的死区手柄中，需要把数据流中的哪些字段置为 `0` 必须由用户空间计算出来。

由此推论，HID-BPF 不了解内核中其它子系统的存在。*你无法从 eBPF 中直接通过输入 API 发出输入事件*。

当某个 BPF 程序需要发出输入事件时，它需要与 HID 协议交互，并依赖 HID 内核处理将 HID 数据转换为输入事件。

## 内核树内的 HID-BPF 程序与 ``udev-hid-bpf``


官方的设备修复以内核源码的形式随内核树一起发布，位于 `drivers/hid/bpf/progs` 目录。这使得我们可以在 `tools/testing/selftests/hid` 中为它们添加自测试。

不过，这些目标的编译不属于常规内核编译的一部分，因为它们需要一个外部工具才能加载。该工具目前是 `udev-hid-bpf <https://libevdev.pages.freedesktop.org/udev-hid-bpf/index.html>`_。

为方便起见，该外部仓库将这里的 `drivers/hid/bpf/progs` 文件复制到自己独立的 `src/bpf/stable` 目录中。这使得发行版无需拉取整个内核源码树就能发布并打包这些 HID-BPF 修复。`udev-hid-bpf` 还具备根据用户所运行内核来处理多个目标文件的能力。

## 可用的程序类型


HID-BPF 构建在 BPF “之上”，这意味着我们使用 bpf struct_ops 方法来声明程序。

HID-BPF 提供以下可用的附加类型：

1. 事件处理/过滤，使用 libbpf 中的 `SEC("struct_ops/hid_device_event")`。
2. 来自用户空间的操作，使用 libbpf 中的 `SEC("syscall")`。
3. 报告描述符的更改，使用 libbpf 中的 `SEC("struct_ops/hid_rdesc_fixup")` 或 `SEC("struct_ops.s/hid_rdesc_fixup")`。

`hid_device_event` 在从设备收到事件时调用一个 BPF 程序。因此，我们处于 IRQ 上下文，可以对数据进行操作或通知用户空间。并且由于我们处于 IRQ 上下文，我们无法与设备回话。

`syscall` 表示用户空间调用了 `BPF_PROG_RUN` 系统调用设施。这一次我们可以执行 HID-BPF 允许的任何操作，并且允许与设备对话。

最后，`hid_rdesc_fixup` 与其它类型不同，因为该类型只能有一个 BPF 程序。它在驱动的 `probe` 时被调用，并允许通过 BPF 程序更改报告描述符。一旦 `hid_rdesc_fixup` 程序被加载，除非插入它的程序通过固定（pin）该程序并关闭所有指向它的 fd 来允许，否则无法覆盖它。

注意，`hid_rdesc_fixup` 可以声明为可休眠的（`SEC("struct_ops.s/hid_rdesc_fixup")`）。

## 开发者 API：


### HID-BPF 可用的 ``struct_ops``：


   :identifiers: hid_bpf_ops


### 程序中可用的用户 API 数据结构：


   :identifiers: hid_bpf_ctx

### 所有 HID-BPF struct_ops 程序都可使用的 API：


   :identifiers: hid_bpf_get_data

### syscall 类 HID-BPF 程序或可休眠的 HID-BPF struct_ops 程序都可使用的 API：


   :identifiers: hid_bpf_hw_request hid_bpf_hw_output_report hid_bpf_input_report hid_bpf_try_input_report hid_bpf_allocate_context hid_bpf_release_context

## HID-BPF 程序的通用概览


### 访问附加到上下文的数据


`struct hid_bpf_ctx` 不会直接导出 `data` 字段，要访问它，BPF 程序需要先调用 `hid_bpf_get_data`。

`offset` 可以是任意整数，但 `size` 必须是常量，在编译时已知。

这样就允许以下情况：

1. 对于给定设备，如果我们知道报告长度始终为某个固定值，我们可以请求 `data` 指针指向完整的报告长度。

   内核会确保我们使用正确的大小和偏移，而 eBPF 会确保：

```
     __u8 *data = hid_bpf_get_data(ctx, 0 /* offset */, 256 /* size */);

     if (!data)
         return 0; /* ensure data is correct, now the verifier knows we
                    * have 256 bytes available */

     bpf_printk("hello world: %02x %02x %02x", data[0], data[128], data[255]);
```

2. 如果报告长度可变，但我们知道 `X` 的值始终是一个 16 位值：

```
      __u16 *x = hid_bpf_get_data(ctx, offset, sizeof(*x));

      if (!x)
          return 0; /* something went wrong */

      *x += 1; /* increment X by one */
```

### HID-BPF 程序的效果


对于所有 HID-BPF 附加类型（除了 `hid_rdesc_fixup`），可以有多个 eBPF 程序附加到同一设备。如果某个 HID-BPF struct_ops 带有 `hid_rdesc_fixup`，而另一个已附加到该设备，内核在附加该 struct_ops 时会返回 `-EINVAL`。

除非在附加程序时向 flags 添加了 `BPF_F_BEFORE`，否则新程序会被追加到列表末尾。`BPF_F_BEFORE` 会把新程序插入到列表开头，这对例如追踪场景很有用——我们需要获取来自设备的未处理事件。

注意，如果有多个程序使用了 `BPF_F_BEFORE` 标志，实际上只有最近加载的那一个才在列表首位。

#### ``SEC("struct_ops/hid_device_event")``


每当有匹配的事件被触发，eBPF 程序会依次被调用，并且它们操作的是同一份数据缓冲区。

如果某个程序更改了与上下文关联的数据，下一个程序将看到修改后的数据，但它将**无从知晓**原始数据是什么。

一旦所有程序都运行完毕并返回 `0` 或正值，HID 协议栈的其余部分将对修改后的数据进行处理，最后一个 hid_bpf_ctx 的 `size` 字段即为输入数据流的新大小。

返回负错误的 BPF 程序会丢弃该事件，即该事件不会被 HID 协议栈处理。客户端（hidraw、input、LED）将**不会**看到该事件。

#### ``SEC("syscall")``


`syscall` 并不附加到某个特定设备。为了指明我们正在处理的是哪个设备，用户空间需要通过设备的唯一系统 ID（sysfs 路径中的最后 4 个数字：`/sys/bus/hid/devices/xxxx:yyyy:zzzz:0000`）来引用它。

为了获取与该设备关联的上下文，程序必须调用 hid_bpf_allocate_context()，并在返回前用 hid_bpf_release_context() 释放它。一旦获取了上下文，也可以用 hid_bpf_get_data() 请求一个指向内核内存的指针。这块内存足够大，可以支持该给定设备的所有输入/输出/特性报告。

#### ``SEC("struct_ops/hid_rdesc_fixup")``


`hid_rdesc_fixup` 程序的工作方式与 `struct hid_driver` 的 `.report_fixup` 类似。

当设备被探测时，内核会用报告描述符的内容填充上下文的数据缓冲区。与该缓冲区关联的内存为 `HID_MAX_DESCRIPTOR_SIZE`（当前为 4kB）。

eBPF 程序可以随意修改数据缓冲区，内核会把修改后的内容与大小作为报告描述符使用。

每当一个包含 `SEC("struct_ops/hid_rdesc_fixup")` 程序的 struct_ops 被附加（如果之前没有程序被附加），内核会立即断开该 HID 设备并重新探测。

同样地，当该 struct_ops 被分离时，内核会对设备发出断开连接。

HID-BPF 中没有 `detach` 设施。分离一个程序发生在所有指向某个 HID-BPF struct_ops 链接的用户空间文件描述符都被关闭时。因此，如果我们需要替换某个报告描述符修正程序，需要原始报告描述符修正程序的所有者配合。先前的所有者很可能会把该 struct_ops 链接固定到 bpffs 中，之后我们就可以通过普通的 bpf 操作来替换它。

## 将 bpf 程序附加到设备


我们现在使用通过 `bpf_map__attach_struct_ops()` 的标准 struct_ops 附加方式。但由于我们需要将 struct_ops 附加到一个专用的 HID 设备，调用者必须在将程序加载进内核之前，在 struct_ops map 中设置 `hid_id`。

`hid_id` 是 HID 设备的唯一系统 ID（sysfs 路径中的最后 4 个数字：`/sys/bus/hid/devices/xxxx:yyyy:zzzz:0000`）。

也可以设置 `flags`，其类型为 `enum hid_bpf_attach_flags`。

我们无法依赖 hidraw 来把 BPF 程序绑定到 HID 设备。hidraw 是 HID 设备处理过程的产物，并不稳定。某些驱动甚至会禁用它，从而在这些设备上失去了追踪能力（而获取非 hidraw 的追踪信息恰恰很有意义）。

另一方面，`hid_id` 在 HID 设备的整个生命周期内都是稳定的，即便我们更改了它的报告描述符。

鉴于 hidraw 在设备断开/重连时并不稳定，我们建议通过 sysfs 访问设备当前的报告描述符。它在 `/sys/bus/hid/devices/BUS:VID:PID.000N/report_descriptor` 处作为一个二进制流提供。

解析报告描述符是 BPF 编程者或加载 eBPF 程序的用户空间组件的责任。

## 一个（几乎）完整的 BPF 增强 HID 设备示例


**前言：在大多数情况下，这也可以用内核驱动来实现**

设想我们有一个新的平板设备，具有一些触觉能力，可以模拟用户正在其上书写的表面。该设备还有一个特定的 3 档开关，用于在**铅笔在纸上**、**墙上的蜡笔**和**画笔在画布上**之间切换。为了锦上添花，我们还可以通过一个特性报告来控制该开关的物理位置。

当然，该开关依赖某个用户空间组件来控制设备自身的触觉特性。

### 过滤事件


第一步是对来自设备的事件进行过滤。由于开关位置实际上是在触控笔事件流中报告的，使用 hidraw 来实现这种过滤意味着每个事件都会唤醒用户空间。

这对 libinput 来说没问题，但让一个只关心报告中一个字节的外部库去承担这种唤醒，就不太理想了。

```
  #include "vmlinux.h"
  #include <bpf/bpf_helpers.h>
  #include <bpf/bpf_tracing.h>

  /* HID programs need to be GPL */
  char _license[] SEC("license") = "GPL";

  /* HID-BPF kfunc API definitions */
  extern __u8 *hid_bpf_get_data(struct hid_bpf_ctx *ctx,
			      unsigned int offset,
			      const size_t __sz) __ksym;

  struct {
	__uint(type, BPF_MAP_TYPE_RINGBUF);
	__uint(max_entries, 4096 * 64);
  } ringbuf SEC(".maps");

  __u8 current_value = 0;

  SEC("struct_ops/hid_device_event")
  int BPF_PROG(filter_switch, struct hid_bpf_ctx *hid_ctx)
  {
	__u8 *data = hid_bpf_get_data(hid_ctx, 0 /* offset */, 192 /* size */);
	__u8 *buf;

	if (!data)
		return 0; /* EPERM check */

	if (current_value != data[152]) {
		buf = bpf_ringbuf_reserve(&ringbuf, 1, 0);
		if (!buf)
			return 0;

		*buf = data[152];

		bpf_ringbuf_commit(buf, 0);

		current_value = data[152];
	}

	return 0;
  }

  SEC(".struct_ops.link")
  struct hid_bpf_ops haptic_tablet = {
  	.hid_device_event = (void *)filter_switch,
  };
```

```
  static int attach_filter(struct hid *hid_skel, int hid_id)
  {
  	int err, link_fd;

  	hid_skel->struct_ops.haptic_tablet->hid_id = hid_id;
  	err = hid__load(skel);
  	if (err)
  		return err;

  	link_fd = bpf_map__attach_struct_ops(hid_skel->maps.haptic_tablet);
  	if (!link_fd) {
  		fprintf(stderr, "can not attach HID-BPF program: %m\n");
  		return -1;
  	}

  	return link_fd; /* the fd of the created bpf_link */
  }
```

我们的用户空间程序现在可以监听环形缓冲区上的通知，并且仅当值发生变化时才会被唤醒。

当用户空间程序不再需要监听事件时，它可以简单地关闭 `attach_filter` 返回的 bpf 链接，这会通知内核将该程序从 HID 设备上分离。

当然，在其他使用场景中，用户空间程序也可以像任何 bpf_link 一样，通过调用 `bpf_obj_pin` 把该 fd 固定到 BPF 文件系统。

### 控制设备


为了能够更改平板的触觉反馈，用户空间程序需要向设备自身发出一个特性报告。

我们不必为此使用 hidraw，可以创建一个 `SEC("syscall")` 程序：

```
  /* some more HID-BPF kfunc API definitions */
  extern struct hid_bpf_ctx *hid_bpf_allocate_context(unsigned int hid_id) __ksym;
  extern void hid_bpf_release_context(struct hid_bpf_ctx *ctx) __ksym;
  extern int hid_bpf_hw_request(struct hid_bpf_ctx *ctx,
			      __u8* data,
			      size_t len,
			      enum hid_report_type type,
			      enum hid_class_request reqtype) __ksym;


  struct hid_send_haptics_args {
	/* data needs to come at offset 0 so we can do a memcpy into it */
	__u8 data[10];
	unsigned int hid;
  };

  SEC("syscall")
  int send_haptic(struct hid_send_haptics_args *args)
  {
	struct hid_bpf_ctx *ctx;
	int ret = 0;

	ctx = hid_bpf_allocate_context(args->hid);
	if (!ctx)
		return 0; /* EPERM check */

	ret = hid_bpf_hw_request(ctx,
				 args->data,
				 10,
				 HID_FEATURE_REPORT,
				 HID_REQ_SET_REPORT);

	hid_bpf_release_context(ctx);

	return ret;
  }
```

```
  static int set_haptic(struct hid *hid_skel, int hid_id, __u8 haptic_value)
  {
	int err, prog_fd;
	int ret = -1;
	struct hid_send_haptics_args args = {
		.hid = hid_id,
	};
	DECLARE_LIBBPF_OPTS(bpf_test_run_opts, tattrs,
		.ctx_in = &args,
		.ctx_size_in = sizeof(args),
	);

	args.data[0] = 0x02; /* report ID of the feature on our device */
	args.data[1] = haptic_value;

	prog_fd = bpf_program__fd(hid_skel->progs.set_haptic);

	err = bpf_prog_test_run_opts(prog_fd, &tattrs);
	return err;
  }
```

现在我们的用户空间程序了解了触觉状态并能够控制它。该程序可以把这个状态进一步提供给其它用户空间程序（例如通过 DBus API）。

这里有趣的一点是，我们并没有为此创建新的内核 API。这意味着如果我们实现中有 bug，我们可以随意更改与内核之间的接口，因为用户空间应用程序要对自己的使用负责。
