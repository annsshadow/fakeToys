


## 编写客户端驱动

API 文档请参阅：

- [client-api](client-api)

## 概述

客户端驱动的搭建主要有两种方式，取决于对应的设备是如何提供给系统的。我们特别区分了通过常规方式（例如通过 ACPI 作为平台设备）呈现给系统的设备，以及不可发现、因而需要通过其他机制显式提供的设备，下文将对此进一步讨论。

## 非 SSAM 客户端驱动

与 SAM EC 的所有通信都通过代表该 EC 的 |ssam_controller| 处理，后者向内核呈现该 EC。面向非 SSAM 设备（因此并非 |ssam_device_driver|）的驱动需要显式地建立与该控制器的连接/关联。这可以通过 |ssam_client_bind| 函数完成。该函数返回对 SSAM 控制器的引用，但更重要的是，它还在客户端设备与控制器之间建立了一条设备链接（也可以通过 |ssam_client_link| 单独完成）。这样做很重要，因为首先，它保证在所返回的控制器在其驱动绑定到设备期间对该客户端驱动始终有效，即驱动会在控制器失效之前先被解绑；其次，它确保了正确的挂起/恢复顺序。这一设置应当在驱动的 probe 函数中完成，并且可用于在 SSAM 子系统尚未就绪时推迟探测，例如：

   static int client_driver_probe(struct platform_device *pdev)
   {
           struct ssam_controller *ctrl;

           ctrl = ssam_client_bind(&pdev->dev);
           if (IS_ERR(ctrl))
                   return PTR_ERR(ctrl) == -ENODEV ? -EPROBE_DEFER : PTR_ERR(ctrl);

           // ...

           return 0;
   }

控制器也可以通过 |ssam_get_controller| 单独获取，其生命周期可通过 |ssam_controller_get| 与 |ssam_controller_put| 加以保证。但请注意，这些函数都不能保证控制器不会被关闭或挂起。这些函数本质上只操作引用，即只保证最低限度的可访问性，而对实际可操作性不作任何保证。

## 添加 SSAM 设备

如果某个设备尚不存在/尚未通过常规方式提供，则应通过 SSAM 客户端设备 hub 将其作为 |ssam_device| 提供。可以通过将新设备的 UID 录入相应的注册表来将其添加到该 hub。SSAM 设备也可以手动通过 |ssam_device_alloc| 分配，随后必须通过 |ssam_device_add| 添加，并最终通过 |ssam_device_remove| 移除。默认情况下，设备的父设备被设为用于分配的控制器设备，但这可以在设备被添加之前更改。注意，在更改父设备时，必须注意确保默认设置（通过父子关系提供）中关于控制器生命周期和挂起/恢复顺序的保证得以保留；如有必要，可使用 |ssam_client_link|，正如非 SSAM 客户端驱动所做的那样，详见上文。

客户端设备必须始终由添加该设备的那一方在控制器关闭之前将其移除。这种移除可以通过把提供 SSAM 设备的驱动通过 |ssam_client_link| 链接到控制器来保证，从而使其在控制器驱动解绑之前先解绑。以控制器为父设备注册的客户端设备会在控制器关闭时自动移除，但不应依赖这一点，尤其是因为这并不适用于具有其他父设备的客户端设备。

## SSAM 客户端驱动

SSAM 客户端设备驱动本质上与其他设备驱动类型没有区别。它们通过 |ssam_device_driver| 表示，并通过其 UID（`struct ssam_device.uid <ssam_device>`）成员以及匹配表（`struct ssam_device_driver.match_table <ssam_device_driver>`）绑定到 |ssam_device|，这些应在声明驱动结构体实例时设置。关于如何定义驱动匹配表的成员，请参阅 |SSAM_DEVICE| 宏的文档。

SSAM 客户端设备的 UID 由 `domain`、`category`、`target`、`instance` 与 `function` 组成。`domain` 用于区分物理 SAM 设备（`SSAM_DOMAIN_SERIALHUB <ssam_device_domain>`，即通过 Surface Serial Hub 可访问的设备）与虚拟设备（`SSAM_DOMAIN_VIRTUAL <ssam_device_domain>`，例如客户端设备 hub，它们在 SAM EC 上没有真实表示，仅用于内核/驱动侧）。对于物理设备，`category` 表示目标类别，`target` 表示目标 ID，`instance` 表示用于访问物理 SAM 设备的实例 ID。此外，`function` 引用特定的设备功能，但对 SAM EC 没有意义。客户端设备的（默认）名称根据其 UID 生成。

驱动实例可以通过 |ssam_device_driver_register| 注册，通过 |ssam_device_driver_unregister| 注销。为方便起见，可使用 |module_ssam_device_driver| 宏来定义注册该驱动的模块 init 与 exit 函数。

与 SSAM 客户端设备关联的控制器可在其 `struct ssam_device.ctrl <ssam_device>` 成员中找到。该引用保证至少在客户端驱动绑定期间有效，但理论上也应在该客户端设备存在期间一直有效。但注意，在已绑定的客户端驱动之外进行访问时，必须确保控制器设备在进行任何请求或（注销）注册事件通知器时未被挂起（因此通常应避免）。当控制器从已绑定的客户端驱动内部访问时，这一点可以得到保证。

## 发起同步请求

同步请求（目前）是主机发起的、与 EC 通信的主要形式。有多种方式来定义并执行此类请求，但大多最终都归结为与下面示例类似的形式。该示例定义了一个写-读请求，即调用者向 SAM EC 提供一个参数并收到一个响应。调用者需要知道响应载荷的（最大）长度并为其提供一个缓冲区。

必须注意确保传给 SAM EC 的任何命令载荷数据都以小端（little-endian）格式提供，同样地，从它收到的任何响应载荷数据都要从小端转换为主机字节序。

   int perform_request(struct ssam_controller **ctrl, u32 arg, u32 **ret)
   {
           struct ssam_request rqst;
           struct ssam_response resp;
           int status;

           /** Convert request argument to little-endian. **/
           __le32 arg_le = cpu_to_le32(arg);
           __le32 ret_le = cpu_to_le32(0);

           /*
            - Initialize request specification. Replace this with your values.
            - The rqst.payload field may be NULL if rqst.length is zero,
            - indicating that the request does not have any argument.
            *
            - Note: The request parameters used here are not valid, i.e.
            - they do not correspond to an actual SAM/EC request.
            */
           rqst.target_category = SSAM_SSH_TC_SAM;
           rqst.target_id = SSAM_SSH_TID_SAM;
           rqst.command_id = 0x02;
           rqst.instance_id = 0x03;
           rqst.flags = SSAM_REQUEST_HAS_RESPONSE;
           rqst.length = sizeof(arg_le);
           rqst.payload = (u8 *)&arg_le;

           /** Initialize request response. **/
           resp.capacity = sizeof(ret_le);
           resp.length = 0;
           resp.pointer = (u8 *)&ret_le;

           /*
            - Perform actual request. The response pointer may be null in case
            - the request does not have any response. This must be consistent
            - with the SSAM_REQUEST_HAS_RESPONSE flag set in the specification
            - above.
            */
           status = ssam_request_do_sync(ctrl, &rqst, &resp);

           /*
            - Alternatively use
            *
            - ssam_request_do_sync_onstack(ctrl, &rqst, &resp, sizeof(arg_le));
            *
            - to perform the request, allocating the message buffer directly
            - on the stack as opposed to allocation via kzalloc().
            */

           /*
            - Convert request response back to native format. Note that in the
            - error case, this value is not touched by the SSAM core, i.e.
            - 'ret_le' will be zero as specified in its initialization.
            */
           *ret = le32_to_cpu(ret_le);

           return status;
   }

注意，|ssam_request_do_sync| 本质上是对更低层请求原语的封装，那些原语也可用于执行请求。更多细节请参阅其实现与文档。

定义此类函数中，arguably 一种对用户更友好的方式是使用其中一个生成宏，例如：

   SSAM_DEFINE_SYNC_REQUEST_W(__ssam_tmp_perf_mode_set, __le32, {
           .target_category = SSAM_SSH_TC_TMP,
           .target_id       = SSAM_SSH_TID_SAM,
           .command_id      = 0x03,
           .instance_id     = 0x00,
   });

该示例定义了一个函数

   static int __ssam_tmp_perf_mode_set(struct ssam_controller **ctrl, const __le32 **arg);

用于执行指定的请求，调用该函数时传入控制器。在此示例中，参数通过 `arg` 指针提供。注意，生成的函数会在栈上分配消息缓冲区。因此，如果请求提供的参数较大，应当避免使用这类宏。还要注意，与前面非宏的示例不同，该函数不做任何字节序转换，这必须由调用者处理。除了这些差异之外，宏生成的函数与上面非宏示例中的函数相似。

这类函数生成宏的完整列表为：

- `SSAM_DEFINE_SYNC_REQUEST_N`：用于无返回值且无参数的请求。
- `SSAM_DEFINE_SYNC_REQUEST_R`：用于有返回值但无参数的请求。
- `SSAM_DEFINE_SYNC_REQUEST_W`：用于无返回值但有参数的请求。

更多细节请参阅它们各自的文档。对于这些宏中的每一个，都提供了一个特殊变体，针对适用于同一设备类型多个实例的请求类型：

- `SSAM_DEFINE_SYNC_REQUEST_MD_N`
- `SSAM_DEFINE_SYNC_REQUEST_MD_R`
- `SSAM_DEFINE_SYNC_REQUEST_MD_W`

这些宏与上述版本的区别在于，所生成函数中设备 target 与 instance ID 并非固定，而是必须由该函数的调用者提供。

此外，还提供了可直接用于客户端设备（即 |ssam_device|）的变体。例如可以按如下方式使用：

   SSAM_DEFINE_SYNC_REQUEST_CL_R(ssam_bat_get_sta, __le32, {
           .target_category = SSAM_SSH_TC_BAT,
           .command_id      = 0x01,
   });

对该宏的这次调用定义了一个函数

   static int ssam_bat_get_sta(struct ssam_device **sdev, __le32 **ret);

用于执行指定的请求，使用客户端设备中给出的设备 ID 与控制器。这类用于客户端设备的宏的完整列表为：

- `SSAM_DEFINE_SYNC_REQUEST_CL_N`
- `SSAM_DEFINE_SYNC_REQUEST_CL_R`
- `SSAM_DEFINE_SYNC_REQUEST_CL_W`

## 处理事件

要从 SAM EC 接收事件，必须通过 |ssam_notifier_register| 为期望的事件注册一个事件通知器（notifier）。当不再需要该通知器时，必须通过 |ssam_notifier_unregister| 注销它。对于 |ssam_device| 类型的客户端，应当优先使用 |ssam_device_notifier_register| 与 |ssam_device_notifier_unregister| 这两个封装函数，因为它们能正确处理客户端设备的热移除。

注册事件通知器时，至少要提供：收到事件时调用的回调函数、指定如何启用该事件的注册表（registry）、指定应为哪个目标类别（以及根据所用注册表，可选地指定哪个实例 ID）启用事件的事件 ID，最后是描述 EC 将如何发送这些事件的标志。如果特定的注册表不按实例 ID 启用事件，则实例 ID 必须设为零。此外，可以为相应通知器指定一个优先级，它决定该通知器相对于注册到同一目标类别的任何其他通知器的顺序。

默认情况下，事件通知器会接收特定目标类别的所有事件，无论注册通知器时指定的实例 ID 为何。通过提供事件掩码（参见 |ssam_event_mask|），可以指示核心仅在事件的目标 ID 或实例 ID（或两者）与通知器 ID 所隐含的相匹配时才调用该通知器（对于目标 ID，即注册表的目标 ID）。

一般而言，注册表的目标 ID 也就是所启用事件的目标 ID（一个显著的例外是 Surface Laptop 1 和 2 上的键盘输入事件：它们通过目标 ID 为 1 的注册表启用，但提供的事件目标 ID 为 2）。

下面是一个注册事件通知器并处理收到事件的完整示例：

   u32 notifier_callback(struct ssam_event_notifier *nf,
                         const struct ssam_event *event)
   {
           int status = ...

           /** Handle the event here ... **/

           /** Convert return value and indicate that we handled the event. **/
           return ssam_notifier_from_errno(status) | SSAM_NOTIF_HANDLED;
   }

   int setup_notifier(struct ssam_device *sdev,
                      struct ssam_event_notifier *nf)
   {
           /** Set priority wrt. other handlers of same target category. **/
           nf->base.priority = 1;

           /** Set event/notifier callback. **/
           nf->base.fn = notifier_callback;

           /** Specify event registry, i.e. how events get enabled/disabled. **/
           nf->event.reg = SSAM_EVENT_REGISTRY_KIP;

           /** Specify which event to enable/disable **/
           nf->event.id.target_category = sdev->uid.category;
           nf->event.id.instance = sdev->uid.instance;

           /*
            - Specify for which events the notifier callback gets executed.
            - This essentially tells the core if it can skip notifiers that
            - don't have target or instance IDs matching those of the event.
            */
           nf->event.mask = SSAM_EVENT_MASK_STRICT;

           /** Specify event flags. **/
           nf->event.flags = SSAM_EVENT_SEQUENCED;

           return ssam_notifier_register(sdev->ctrl, nf);
   }

可以为同一事件注册多个事件通知器。事件处理核心会在通知器注册和注销时负责启用和禁用事件，其方式是跟踪当前为某个特定事件（由注册表、事件目标类别、事件实例 ID 组合而成）注册了多少个通知器。这意味着，特定事件会在其第一个通知器注册时被启用，并在其最后一个通知器注销时被禁用。因此，事件标志仅在第一个注册的通知器上生效；但应注意，针对特定事件的通知器应当始终以相同的标志注册，否则被视为一个缺陷（bug）。
