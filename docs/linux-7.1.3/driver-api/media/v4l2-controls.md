
## V4L2 控件

### 简
V4L2 控件（control）API 看起来足够简单，但要在驱动中正确实现却很快变得非常困难不过处理控件所需的代码大部分其实并不特定于某个驱动，可以移到 V4L 核心框架中
毕竟，驱动开发者唯一感兴趣的部分是：

1) 如何添加一个控件？
2) 如何设置控件的值？（即 s_ctrl
偶尔还会用到
3) 如何获取控件的值？（即 g_volatile_ctrl4) 如何校验用户提议的控件值？（即 try_ctrl
其余一切都可以在集中处完成
控制框架（control framework）的创建，是为了V4L2 规范中关于控件的所有规则在
一个集中的地方实现，并且尽可能让驱动开发者的工作变得轻松
注意，控制框架依V4L2 驱动`v4l2_device` 结构体，以及子设备（sub-device驱动v4l2_subdev 结构体
### 框架中的对象

有两个主要对象：

`v4l2_ctrl` 对象描述控件的属性，并跟踪控件的值（包括当前值和提议的新值）
`v4l2_ctrl_handler` 是跟踪控件的对象。它维护一个它所拥有v4l2_ctrl 对象列表以及另一个指向控件的引用列表，这些控件可能由其它处理器（handler）拥有
### V4L2 和子设备驱动的基本用
1) 准备驱动
	#include <media/v4l2-ctrls.h>

1.1) 将处理器（handler）添加到驱动的顶层结构体
对于 V4L2 驱动
	struct foo_dev {
		...
		struct v4l2_device v4l2_dev;
		...
		struct v4l2_ctrl_handler ctrl_handler;
		...
	};

对于子设备驱动：

	struct foo_dev {
		...
		struct v4l2_subdev sd;
		...
		struct v4l2_ctrl_handler ctrl_handler;
		...
	};

1.2) 初始化处理器（handler）：

	v4l2_ctrl_handler_init(&foo->ctrl_handler, nr_of_controls);

第二个参数是一个提示，告诉该函数该处理器预期要处理多少个控件。它将基于该信息
分配一个哈希表。这仅仅是一个提示
1.3) 将控制处理器（control handler）挂接到驱动
对于 V4L2 驱动
	foo->v4l2_dev.ctrl_handler = &foo->ctrl_handler;

对于子设备驱动：

	foo->sd.ctrl_handler = &foo->ctrl_handler;

1.4) 在最后清理处理器（handler）：

	v4l2_ctrl_handler_free(&foo->ctrl_handler);

`v4l2_ctrl_handler_free` 不会触碰处理器的 `error` 字段
2) 添加控件
通过调用 `v4l2_ctrl_new_std` 添加非菜单（non-menu）控件：

	struct v4l2_ctrl **v4l2_ctrl_new_std(struct v4l2_ctrl_handler **hdl,
			const struct v4l2_ctrl_ops *ops,
			u32 id, s32 min, s32 max, u32 step, s32 def);

菜单（menu）和整数菜单（integer menu）控件通过调用 `v4l2_ctrl_new_std_menu`
添加
	struct v4l2_ctrl **v4l2_ctrl_new_std_menu(struct v4l2_ctrl_handler **hdl,
			const struct v4l2_ctrl_ops *ops,
			u32 id, s32 max, s32 skip_mask, s32 def);

带有驱动特定菜单的菜单控件通过调用 `v4l2_ctrl_new_std_menu_items` 添加
       struct v4l2_ctrl *v4l2_ctrl_new_std_menu_items(
                       struct v4l2_ctrl_handler *hdl,
                       const struct v4l2_ctrl_ops *ops, u32 id, s32 max,
                       s32 skip_mask, s32 def, const char ** const **qmenu);

标准复合（compound）控件可以通过调用 `v4l2_ctrl_new_std_compound` 添加
       struct v4l2_ctrl **v4l2_ctrl_new_std_compound(struct v4l2_ctrl_handler **hdl,
                       const struct v4l2_ctrl_ops *ops, u32 id,
                       const union v4l2_ctrl_ptr p_def);

带有驱动特定菜单的整数菜单控件可以通过调用 `v4l2_ctrl_new_int_menu` 添加
	struct v4l2_ctrl **v4l2_ctrl_new_int_menu(struct v4l2_ctrl_handler **hdl,
			const struct v4l2_ctrl_ops *ops,
			u32 id, s32 max, s32 def, const s64 *qmenu_int);

这些函数通常`v4l2_ctrl_handler_init` 之后立即调用
	static const s64 exp_bias_qmenu[] = {
	       -2, -1, 0, 1, 2
	};
	static const char * const test_pattern[] = {
		"Disabled",
		"Vertical Bars",
		"Solid Black",
		"Solid White",
	};

	v4l2_ctrl_handler_init(&foo->ctrl_handler, nr_of_controls);
	v4l2_ctrl_new_std(&foo->ctrl_handler, &foo_ctrl_ops,
			V4L2_CID_BRIGHTNESS, 0, 255, 1, 128);
	v4l2_ctrl_new_std(&foo->ctrl_handler, &foo_ctrl_ops,
			V4L2_CID_CONTRAST, 0, 255, 1, 128);
	v4l2_ctrl_new_std_menu(&foo->ctrl_handler, &foo_ctrl_ops,
			V4L2_CID_POWER_LINE_FREQUENCY,
			V4L2_CID_POWER_LINE_FREQUENCY_60HZ, 0,
			V4L2_CID_POWER_LINE_FREQUENCY_DISABLED);
	v4l2_ctrl_new_int_menu(&foo->ctrl_handler, &foo_ctrl_ops,
			V4L2_CID_EXPOSURE_BIAS,
			ARRAY_SIZE(exp_bias_qmenu) - 1,
			ARRAY_SIZE(exp_bias_qmenu) / 2 - 1,
			exp_bias_qmenu);
	v4l2_ctrl_new_std_menu_items(&foo->ctrl_handler, &foo_ctrl_ops,
			V4L2_CID_TEST_PATTERN, ARRAY_SIZE(test_pattern) - 1, 0,
			0, test_pattern);
	...
	if (foo->ctrl_handler.error)
		return v4l2_ctrl_handler_free(&foo->ctrl_handler);

`v4l2_ctrl_new_std` 函数返回指向新控件的 v4l2_ctrl 指针，但如果你不需要在控件
操作（control ops）之外访问该指针，则无需保存它
`v4l2_ctrl_new_std` 函数会基于控ID 填充大部分字段，除了最小值、最大值、步和默认值。这些通过最后四个参数传入。这些值是驱动特定的，而类型、名称、标志等
控件属性都是全局的。控件的当前值会被设为默认值
`v4l2_ctrl_new_std_menu` 函数非常相似，但它用于菜单控件。没min 参数，因对于菜单控件它始终为 0，取而代之的step 之外有一skip_mask 参数：如果位 X
1，则菜单X 被跳过
`v4l2_ctrl_new_int_menu` 函数创建一个带有驱动特定菜单项的新标准整数菜单控件它与 v4l2_ctrl_new_std_menu 的不同之处在于它没有 mask 参数，并且以最后一个参接受一个有符号 64 位整数数组，构成精确的菜单项列表
`v4l2_ctrl_new_std_menu_items` 函数v4l2_ctrl_new_std_menu 非常相似，但多了一参数 qmenu，它是一个原本标准菜单控件的驱动特定菜单。这类控件的一个好例子具有生成测试图案能力的捕显示/传感器设备的测试图案控件。这些测试图案是硬件
特定的，因此菜单的内容会因设备而异
注意，如果某处失败，函数将返NULL 或错误，并将 ctrl_handler->error 设置为错误码如果 ctrl_handler->error 已经设置，则它只会返回而不做任何事情。对于无法分配内数据结构v4l2_ctrl_handler_init 也是如此
这使得初始化处理器（handler）并直接添加所有控件、只在最后检查错误码变得很容易省去了大量重复的错误检查
建议按控ID 升序添加控件：这样会快一点
3) 可选地强制初始控件设置
	v4l2_ctrl_handler_setup(&foo->ctrl_handler);

这将无条件地对所有控件调s_ctrl。实际上这会把硬件初始化为默认控件值。建议你
这样做，因为这能确保内部数据结构和硬件保持一致
4) 最后：实现 `v4l2_ctrl_ops`

	static const struct v4l2_ctrl_ops foo_ctrl_ops = {
		.s_ctrl = foo_s_ctrl,
	};

通常你只需s_ctrl
	static int foo_s_ctrl(struct v4l2_ctrl *ctrl)
	{
		struct foo *state = container_of(ctrl->handler, struct foo, ctrl_handler);

		switch (ctrl->id) {
		case V4L2_CID_BRIGHTNESS:
			write_reg(0x123, ctrl->val);
			break;
		case V4L2_CID_CONTRAST:
			write_reg(0x456, ctrl->val);
			break;
		}
		return 0;
	}

控制操作（control ops）以 v4l2_ctrl 指针作为参数被调用。新的控件值已经被校验过，
所以你只需实际去更新硬件寄存器即可
你完成了！这对于我们的大多数驱动来说已经足够。无需对控件值做任何校验，也无需
实现 QUERYCTRL、QUERY_EXT_CTRL QUERYMENU。G/S_CTRL 以及 G/TRY/S_EXT_CTRLS
会被自动支持
   其余小节涉及更高级的控件主题和场景。实际上，如上所述的基本用法对大多数驱动
   来说已经足够
### 继承子设备控
当通过调用 v4l2_device_register_subdev() 将一个子设备注册V4L2 驱动，并v4l2_subdev v4l2_device ctrl_handler 字段都已设置时，该子设备的控件将
自动V4L2 驱动中也可用。如果子设备驱动包含的控件在 V4L2 驱动中已经存在，那些控件会被跳过（因V4L2 驱动始终可以覆盖子设备控件）
这里发生的是，v4l2_device_register_subdev() 调用 v4l2_ctrl_add_handler()，将
子设备的控件添加v4l2_device 的控件中
### 访问控件
控制框架内部使用以下联合体（union）来访问控件值：

	union v4l2_ctrl_ptr {
		s32 *p_s32;
		s64 *p_s64;
		char *p_char;
		void *p;
	};

v4l2_ctrl 结构体包含以下可用于访问当前值和新值的字段
	s32 val;
	struct {
		s32 val;
	} cur;


	union v4l2_ctrl_ptr p_new;
	union v4l2_ctrl_ptr p_cur;

如果控件是简单的 s32 类型，则
	&ctrl->val == ctrl->p_new.p_s32
	&ctrl->cur.val == ctrl->p_cur.p_s32

对于所有其它类型，使用 ctrl->p_cur.p<something>。基本上 val cur.val 字段可以
视为别名，因为它们被使用得如此频繁
在控制操作（control ops）内部你可以自由使用这些字段。val cur.val 不言自明p_char 指针指向长度ctrl->maximum + 1 的字符缓冲区，并且总是0 结尾
除非控件被标记为 volatile（易变），否p_cur 字段指向当前缓存的控件值。当你创一个新控件时，该值会被设为与默认值相同。调v4l2_ctrl_handler_setup() 之后，该
值会被传递给硬件。通常调用此函数是个好主意
每当设置了一个新值，该新值会被自动缓存。这意味着大多数驱动不需要实g_volatile_ctrl()
操作（op）。例外情况是返回易变寄存器（例如持续变化的信号强度读数）的控件。在这种
情况下，你需要像下面这样实现 g_volatile_ctrl
	static int foo_g_volatile_ctrl(struct v4l2_ctrl *ctrl)
	{
		switch (ctrl->id) {
		case V4L2_CID_BRIGHTNESS:
			ctrl->val = read_reg(0x123);
			break;
		}
	}

注意你在 g_volatile_ctrl 中也使用了“新值”联合体。一般来说，需要实g_volatile_ctrl
的控件是只读控件。如果不是，则当控件改变时不会生V4L2_EVENT_CTRL_CH_VALUE 事件
要将一个控件标记为 volatile，你必须设置 V4L2_CTRL_FLAG_VOLATILE
	ctrl = v4l2_ctrl_new_std(&sd->ctrl_handler, ...);
	if (ctrl)
		ctrl->flags |= V4L2_CTRL_FLAG_VOLATILE;

对于 try/s_ctrl，新值（即用户传入的值）会被填入，你可以try_ctrl 中修改它们，
或在 s_ctrl 中设置它们cur' 联合体包含当前值，你也可以（但不能修改！）使用它
如果 s_ctrl 返回 0（OK），则控制框架会把新的最终值复制到 'cur' 联合体
g_volatile/s/try_ctrl 内部，你可以访问同一个处理器（handler）拥有的所有控件的
值，因为处理器（handler）的锁（lock）被持有。如果你需要访问其它处理器（handler拥有的控件值，则必须非常小心，避免引入死锁
在控制操作（control ops）之外，你必须通过辅助函数来安全地获取或设置驱动中的单控件值：

	s32 v4l2_ctrl_g_ctrl(struct v4l2_ctrl *ctrl);
	int v4l2_ctrl_s_ctrl(struct v4l2_ctrl *ctrl, s32 val);

这些函数与控制框架的交互方式VIDIOC_G/S_CTRL ioctl 相同。不过，不要在控制操g_volatile/s/try_ctrl 内部使用它们，因为这会导致死锁，因为这些辅助函数同样会锁处理器（handler）
你也可以自己获取处理器（handler）锁
	mutex_lock(&state->ctrl_handler.lock);
	pr_info("String value is '%s'\n", ctrl1->p_cur.p_char);
	pr_info("Integer value is '%s'\n", ctrl2->cur.val);
	mutex_unlock(&state->ctrl_handler.lock);

### 菜单控件

v4l2_ctrl 结构体包含这个联合体
	union {
		u32 step;
		u32 menu_skip_mask;
	};

对于菜单控件使用 menu_skip_mask。它的作用是让你可以轻松排除某些菜单项。这VIDIOC_QUERYMENU 的实现中会用到，当某个菜单项不存在时你可以返-EINVAL。注意，
对于菜单控件，VIDIOC_QUERYCTRL 始终返回步长1
一个很好的例子MPEG Audio Layer II Bitrate 菜单控件，其中菜单是标准化可比特率的列表。但在实际中，硬件实现只会支持其中的一个子集。通过设置 skip 掩码
（mask），你可以告诉框架哪些菜单项应该被跳过。将其设置为 0 表示支持所有菜单项
你可以通过 v4l2_ctrl_config 结构体（针对自定义控件）或调v4l2_ctrl_new_std_menu()
来设置该掩码（mask）
### 自定义控
可以使用 v4l2_ctrl_new_custom() 创建驱动特定的控件：

	static const struct v4l2_ctrl_config ctrl_filter = {
		.ops = &ctrl_custom_ops,
		.id = V4L2_CID_MPEG_CX2341X_VIDEO_SPATIAL_FILTER,
		.name = "Spatial Filter",
		.type = V4L2_CTRL_TYPE_INTEGER,
		.flags = V4L2_CTRL_FLAG_SLIDER,
		.max = 15,
		.step = 1,
	};

	ctrl = v4l2_ctrl_new_custom(&foo->ctrl_handler, &ctrl_filter, NULL);

最后一个参数是 priv 指针，可设置为驱动特定的私有数据
v4l2_ctrl_config 结构体还有一个字段用于设is_private 标志
如果未设name 字段，则框架会假定这是一个标准控件，并相应地填充 name、type flags 字段
### 活动（active）与抓取（grabbed）控
如果你遇到控件之间更复杂的关系，那么你可能必须激活或停用控件。例如，如果 Chroma
AGC 控件开启，那么 Chroma Gain 控件就是非活动的。也就是说，你可以设置它，但只要
自动增益控制还开着，硬件就不会使用该值。典型的用户界面可以禁用此类输入字段
你可以使v4l2_ctrl_activate() 设置“活动”状态。默认情况下所有控件都是活动的注意框架不会检查此标志。它纯粹是为 GUI 准备的。该函数通常s_ctrl 内部调用
另一个标志是“抓取”（grabbed）标志。一个被抓取的控件意味着你无法更改它，因为它被某个资源使用。典型的例子MPEG 比特率控件，在捕获进行期间无法更改
如果使用 v4l2_ctrl_grab() 将一个控件设置为“抓取”，那么当试图设置该控件时框架将
返回 -EBUSY。v4l2_ctrl_grab() 函数通常在驱动启动或停止流传输时调用
### 控件簇（Control Clusters
默认情况下所有控件彼此独立。但在更复杂的场景中，你可能得到一个控件对另一个的
依赖关系。在这种情况下，你需要将它们“聚类”（cluster）：

	struct foo {
		struct v4l2_ctrl_handler ctrl_handler;
	#define AUDIO_CL_VOLUME (0)
	#define AUDIO_CL_MUTE   (1)
		struct v4l2_ctrl *audio_cluster[^2^];
		...
	};

	state->audio_cluster[AUDIO_CL_VOLUME] =
		v4l2_ctrl_new_std(&state->ctrl_handler, ...);
	state->audio_cluster[AUDIO_CL_MUTE] =
		v4l2_ctrl_new_std(&state->ctrl_handler, ...);
	v4l2_ctrl_cluster(ARRAY_SIZE(state->audio_cluster), state->audio_cluster);

从今以后，只要属于同一个簇的一个或多个控件被设置（或“获取”，或“尝试”），只调用第一个控件（本例中为“volume”）的控制操作（control ops）。你实际上创建了一新的复合控件。类似于 C 语言中“struct”的工作方式
因此，当 s_ctrl V4L2_CID_AUDIO_VOLUME 作为参数被调用时，你应该设置属于
audio_cluster 的全部两个控件：

	static int foo_s_ctrl(struct v4l2_ctrl *ctrl)
	{
		struct foo *state = container_of(ctrl->handler, struct foo, ctrl_handler);

		switch (ctrl->id) {
		case V4L2_CID_AUDIO_VOLUME: {
			struct v4l2_ctrl *mute = ctrl->cluster[AUDIO_CL_MUTE];

			write_reg(0x123, mute->val ? 0 : ctrl->val);
			break;
		}
		case V4L2_CID_CONTRAST:
			write_reg(0x456, ctrl->val);
			break;
		}
		return 0;
	}

在上面的例子中，对于 VOLUME 情况，以下三者等价：

	ctrl == ctrl->cluster[AUDIO_CL_VOLUME] == state->audio_cluster[AUDIO_CL_VOLUME]
	ctrl->cluster[AUDIO_CL_MUTE] == state->audio_cluster[AUDIO_CL_MUTE]

在实践中，像这样使用簇数组会变得非常繁琐。因此改用以下等价的方法
	struct {
		/** audio cluster **/
		struct v4l2_ctrl *volume;
		struct v4l2_ctrl *mute;
	};

这个匿名结构体用于清晰地“聚类”这两个控件指针，但它没有其它用途。效果与创建
一个带两个控件指针的数组相同。所以你可以直接这样做：

	state->volume = v4l2_ctrl_new_std(&state->ctrl_handler, ...);
	state->mute = v4l2_ctrl_new_std(&state->ctrl_handler, ...);
	v4l2_ctrl_cluster(2, &state->volume);

foo_s_ctrl 中你可以直接使用这些指针：state->mute->val
注意，簇中的控件可能NULL。例如，如果由于某种原因 mute 从未被添加（因为硬件
不支持该特定特性），那mute 将是 NULL。所以在这种情况下我们有一个包2 个控的簇，其中只1 个实际被实例化。唯一的限制是簇的第一个控件必须始终存在，因为
它是簇的“主”（master）控件。主控件是识别该簇的控件，并提供用于该簇v4l2_ctrl_ops 结构体的指针
显然，簇数组中的所有控件必须被初始化为有效的控件或 NULL
在极少数情况下，你可能想知道簇中的哪些控件实际上是被用户显式设置的。为此你可以
检查每个控件的“is_new”标志。例如，volume/mute 簇的情况下，如果只为用户调用VIDIOC_S_CTRL 设置 mute，那mute 控件的“is_new”标志会被设置。如果用户为 mute volume 控件都调用了 VIDIOC_S_EXT_CTRLS，那么两个控件的“is_new”标志都将是 1
“is_new”标志在v4l2_ctrl_handler_setup() 调用时始终为 1
### 使用自动簇（Auto Clusters）处autogain/gain 类型控件

一种常见的控件簇类型处理的是“auto-foo/foo”类型的控件。典型的例子autogain/gain、autoexposure/exposure、autowhitebalance/red balance/blue balance在所有情况下，你都有一个控件决定另一个控件是由硬件自动处理，还是由用户手动控制
如果簇处于自动模式，那么手动控件应该被标记为非活动（inactive）和易变（volatile）当读取易变控件时，g_volatile_ctrl 操作应该返回由硬件自动模式自动设置的值
如果簇被切换到手动模式，那么手动控件应该重新变为活动（active），并且清除 volatile
标志（因此在手动模式下不再调g_volatile_ctrl）。此外，就在切换到手动模式之前，
由自动模式确定的当前值会被复制为新的手动值
最后，应该为自动控件设V4L2_CTRL_FLAG_UPDATE，因为更改该控件会影响手动控件的
控制标志
为了简化这一点，引入了一v4l2_ctrl_cluster 的特殊变体：

	void v4l2_ctrl_auto_cluster(unsigned ncontrols, struct v4l2_ctrl **controls,
				    u8 manual_val, bool set_volatile);

前两个参数与 v4l2_ctrl_cluster 相同。第三个参数告诉框架哪个值会将簇切换到手动模式最后一个参数可选地（optionally）为非自动控件设V4L2_CTRL_FLAG_VOLATILE。如果为
false，则手动控件永远不会是易变的。如果硬件不允许你读回由自动模式确定的值（例如
如果 autogain 开启，硬件不允许你获取当前增益值），你通常会使false
簇的第一个控件被假定为“auto”控件
使用此函数可确保你无需处理所有复杂的标志和易变（volatile）处理
### VIDIOC_LOG_STATUS 支持

这个 ioctl 允许你将驱动的当前状态转储到内核日志。v4l2_ctrl_handler_log_status
(ctrl_handler, prefix) 可用于将给定处理器（handler）所拥有的控件值转储到日志你也可以提供一个前缀（prefix）。如果前缀没有以空格结尾，则会为你添加 ”
### 不同视频节点使用不同的处理器

通常 V4L2 驱动只有一个对所有视频节点全局的控制处理器（handler）。但你也可以不同的视频节点指定不同的控制处理器。你可以通过手动设置 struct video_device ctrl_handler 字段来做到这一点
如果没有涉及子设备（subdev），这没有问题；但如果有，那么你需要阻止子设备控件
自动合并到全局控制处理器。你只需struct v4l2_device 中的 ctrl_handler 字段
设置NULL 即可。现v4l2_device_register_subdev() 将不再合并子设备控件
在每个子设备被添加之后，你将必须手动调用 v4l2_ctrl_add_handler，将子设备的控制
处理器（sd->ctrl_handler）添加到所需的处理器。这个控制处理器可能特定于某video_device，或某个 video_device 的子集。例如：radio 设备节点只有音频控件，video vbi 设备节点共享同一个用于音频和视频控件的控制处理器
如果你希望让一个处理器（例如用radio 设备节点）拥有另一个处理器（例如用video 设备节点）的子集，那么你应该首先添加控件到第一个处理器，添加其它控件到
第二个处理器，最后将第一个处理器添加到第二个处理器。例如：

	v4l2_ctrl_new_std(&radio_ctrl_handler, &radio_ops, V4L2_CID_AUDIO_VOLUME, ...);
	v4l2_ctrl_new_std(&radio_ctrl_handler, &radio_ops, V4L2_CID_AUDIO_MUTE, ...);
	v4l2_ctrl_new_std(&video_ctrl_handler, &video_ops, V4L2_CID_BRIGHTNESS, ...);
	v4l2_ctrl_new_std(&video_ctrl_handler, &video_ops, V4L2_CID_CONTRAST, ...);
	v4l2_ctrl_add_handler(&video_ctrl_handler, &radio_ctrl_handler, NULL);

v4l2_ctrl_add_handler() 的最后一个参数是一个过滤函数，允许你过滤哪些控件会被添加如果你想添加所有控件，则将其设NULL
或者你可以将特定控件添加到一个处理器
	volume = v4l2_ctrl_new_std(&video_ctrl_handler, &ops, V4L2_CID_AUDIO_VOLUME, ...);
	v4l2_ctrl_new_std(&video_ctrl_handler, &ops, V4L2_CID_BRIGHTNESS, ...);
	v4l2_ctrl_new_std(&video_ctrl_handler, &ops, V4L2_CID_CONTRAST, ...);

你不应该做的是为两个处理器创建两个相同的控件。例如：

	v4l2_ctrl_new_std(&radio_ctrl_handler, &radio_ops, V4L2_CID_AUDIO_MUTE, ...);
	v4l2_ctrl_new_std(&video_ctrl_handler, &video_ops, V4L2_CID_AUDIO_MUTE, ...);

这很糟糕，因为静radio 不会改变 video 静音控件。规则是：对于每个你可以拨动硬件“旋钮”，应该有一个控件
### 查找控件

通常你已经自己创建了控件，并且可以把 struct v4l2_ctrl 指针保存到自己的结构体中
但有时你需要从一个你不拥有的另一个处理器（handler）中查找控件。例如，如果你必从一个子设备（subdev）中查找 volume 控件
你可以通过调用 v4l2_ctrl_find 来做到这一点：

	struct v4l2_ctrl *volume;

	volume = v4l2_ctrl_find(sd->ctrl_handler, V4L2_CID_AUDIO_VOLUME);

由于 v4l2_ctrl_find 会锁定处理器（handler），所以你必须小心在哪里使用它。例如，
这并不是一个好主意
	struct v4l2_ctrl_handler ctrl_handler;

	v4l2_ctrl_new_std(&ctrl_handler, &video_ops, V4L2_CID_BRIGHTNESS, ...);
	v4l2_ctrl_new_std(&ctrl_handler, &video_ops, V4L2_CID_CONTRAST, ...);

……而在 video_ops.s_ctrl 中：

	case V4L2_CID_BRIGHTNESS:
		contrast = v4l2_find_ctrl(&ctrl_handler, V4L2_CID_CONTRAST);
		...

当框架调s_ctrl 时，ctrl_handler.lock 已经被获取，因此试图从同一个处理器查找
另一个控件会导致死锁
建议不要在控制操作（control ops）内部使用此函数
### 阻止控件继承

当使用一个控制处理器（handler）通过 v4l2_ctrl_add_handler 添加到另一个时，默情况下其中一个的所有控件都会被合并到另一个。但一个子设备可能拥有对某个高级嵌入式
系统有意义、但在消费级硬件中使用时毫无意义的底层控件。在这种情况下，你希望将这些
底层控件保留在子设备本地。你可以通过将控件的“is_private”标志设1 来做到这一点：

	static const struct v4l2_ctrl_config ctrl_private = {
		.ops = &ctrl_custom_ops,
		.id = V4L2_CID_...,
		.name = "Some Private Control",
		.type = V4L2_CTRL_TYPE_INTEGER,
		.max = 15,
		.step = 1,
		.is_private = 1,
	};

	ctrl = v4l2_ctrl_new_custom(&foo->ctrl_handler, &ctrl_private, NULL);

现在调用 v4l2_ctrl_add_handler 时会跳过这些控件
### V4L2_CTRL_TYPE_CTRL_CLASS 控件

GUI 可以使用此类控件来获取控件类（control class）的名称。功能完备的 GUI 可以创建
一个带多个选项卡的对话框，每个选项卡包含属于某个特定控件类的控件。每个选项卡的
名称可以通过查询一ID <control class | 1> 的特殊控件来找到
驱动无需关心这一点。每当添加属于一个新的控件类的第一个控件时，框架会自动添加此类
控件
### 添加通知回调（Notify Callbacks
有时平台或桥接（bridge）驱动需要在子设备驱动的某个控件改变时收到通知。你可以通过
调用此函数设notify 回调
	void v4l2_ctrl_notify(struct v4l2_ctrl *ctrl,
		void (**notify)(struct v4l2_ctrl **ctrl, void **priv), void **priv);

每当给定的控件值改变时，notify 回调会以指向该控件的指针以及传给 v4l2_ctrl_notify
priv 指针被调用。注意，调用 notify 函数时控制处理器（handler）的锁（lock）被持有
每个控制处理器（handler）只能有一notify 函数。任何设置另一notify 函数的尝都会导致 WARN_ON
### v4l2_ctrl 函数与数据结