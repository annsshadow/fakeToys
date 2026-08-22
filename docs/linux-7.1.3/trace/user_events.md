## user_events：基于用户的事件跟踪


:Author: Beau Belgrave

### 概述

基于用户的跟踪事件允许用户进程创建事件并跟踪数据，这些数据可以通过现有工具（例ftrace perf）查看
要启用此特性，请在构建内核时设CONFIG_USER_EVENTS=y

程序可以通过 /sys/kernel/tracing/user_events_status 查看事件的状态，并可以通过
/sys/kernel/tracing/user_events_data 注册和写出数据

程序也可以使/sys/kernel/tracing/dynamic_events，通过 u: 前缀来注册和删除基于用户的事件
发送给 dynamic_events 的命令格式与带有 u: 前缀ioctl 相同。由于事件会持久存在，这需CAP_PERFMON 权限
否则会返-EPERM

通常，程序会注册一组它们希望暴露给能够读取 trace_events 的工具（ftrace perf）的事件。注册过
会告诉内核：如果任何工具启用了该事件并且应当写出数据，应当反映到哪个地址和哪个位。注册会返回一
写索引（write index），用于描述/sys/kernel/tracing/user_events_data 文件上调write() writev() 
的数据

本文档中引用的结构体包含在源码树中的 /include/uapi/linux/user_events.h 文件里

**注意* *user_events_status user_events_data 都位tracefs 文件系统下，可能挂载
与上述不同的路径上

### 注册

在用户进程内部进行注册，是通过/sys/kernel/tracing/user_events_data 文件发出 ioctl() 来完成的
要发出的命令DIAG_IOCSREG

```

  struct user_reg {
        /* Input: Size of the user_reg structure being used */
        __u32 size;

        /* Input: Bit in enable address to use */
        __u8 enable_bit;

        /* Input: Enable size in bytes at address */
        __u8 enable_size;

        /* Input: Flags to use, if any */
        __u16 flags;

        /* Input: Address to update when enabled */
        __u64 enable_addr;

        /* Input: Pointer to string with event name, description and flags */
        __u64 name_args;

        /* Output: Index of the event to use when writing data */
        __u32 write_index;
  } __attribute__((__packed__));

```
struct user_reg 要求正确设置上述所有输入字段

- size：必须设置为 sizeof(struct user_reg)

- enable_bit：用于在 enable_addr 指定地址处反映事件状态的位

- enable_size：enable_addr 所指定值的大小
  必须42 位）84 位）4 位值只允许64 位内核上使用，32 位值可以在所有内核上使用

- flags：要使用的标志（如果有的话）
  调用者应当先尝试带标志调用，并在不带标志的情况下重试，以确保对较低版本内核的兼容性。如果某个标志不受支持，会返-EINVAL

- enable_addr：用于反映事件状态的值的地址。该地址必须在用户程序内自然对齐且可写

- name_args：用于描述事件的名称和参数，详见命令格式

当前支持以下标志

- USER_EVENT_REG_PERSIST：当最后一个引用关闭时，事件不会被删除。如果某个事件即使在该进程关闭或注销该事件后仍应存在，调用者可以使用此标志。需CAP_PERFMON 权限，否则会返回 -EPERM

- USER_EVENT_REG_MULTI_FORMAT：事件可以包含多种格式。这允许程序在其事件格式发生变化且希望使用相同名称时，避免自身被阻塞。使用此标志时，tracepoint 名称将采"name.unique_id" 的新格式，而非旧的 "name" 格式。将为每组唯一的（名称，格式）对创建一tracepoint。这意味着如果多个进程使用相同的名称和格式，它们将使用同一tracepoint。如果还有另一个进程使用相同的名称，但格式与其他进程不同，它将使用一个带有新唯一 id 的不tracepoint。录制程序需要扫tracefs，找出它们感兴趣的事件名称的各种不同格式。该 tracepoint 的系统名称也将使"user_events_multi" 而非 "user_events"。这可以防止单格式事件名称与 tracefs 中任何多格式事件名称冲突。unique_id 以十六进制字符串形式输出。录制程序应确保 tracepoint 名称以它们注册的事件名称开头，并且后缀. 开头且只包含十六进制字符。例如，要查找事"test" 的所有版本，可以使用正则表达"^test\.[0-9a-fA-F]+$"

注册成功后将设置以下内容

- write_index：用于此文件描述符的索引，代表写出数据时对应的这个事件。该索引对于用于注册的这个文件描述符实例是唯一的。详见“写入数据”一节

基于用户的事件会"user_events" 子系统下的任何其他事件一样出现在 tracefs 中。这意味着希望挂接这些事件的工具需要使/sys/kernel/tracing/events/user_events/[name]/enable，或在挂录制时使perf record -e user_events:[name]

**注意* 事件子系统名称默认是 "user_events"。调用者不应假设它将永远是 "user_events"。运维方保留将来为支持事件隔离而按进程更改子系统名称的权利。此外，如果使用 USER_EVENT_REG_MULTI_FORMAT 标志，tracepoint 名称将被附加一个唯一 id，且系统名称将如上所述变"user_events_multi"

##### 命令格式

```

  name[:FLAG1[,FLAG2...]] [Field1[;Field2...]]

```
##### 支持的标

暂无

##### 字段格式

```

  type name [size]

```
支持基本类型（__data_loc、u32、u64、int、char、char[^20^] 等）
鼓励用户程序使用明确指定大小的类型，例如 u32

**注意* **不支long 类型，因为其大小在用户空间和内核之间可能不同*

大小仅对struct 前缀开头的类型有效。这允许用户在需要时向工具描述自定义struct

```

  struct mytype {
    char data[20];
  };

```
```

  struct mytype myname 20

```
### 删除

在用户进程内部删除一个事件，是通过/sys/kernel/tracing/user_events_data 文件发出 ioctl() 来完成的
要发出的命令DIAG_IOCSDEL

此命令只需要一个字符串，按名称指定要删除的事件。只有当该事件不再有任何引用（在用户空间和内核空间均如此）时，删除才会成功
因此，用户程序应当使用一个单独的文件来请求删除，而不是用于注册的那个文件

**注意* 默认情况下，当事件不再有任何引用时会自动删除。如果程序不希望自动删除，必须在注册事件时使
USER_EVENT_REG_PERSIST 标志。一旦使用了该标志，事件将一直存在，直到调用 DIAG_IOCSDEL。注册和删除一个持久化事件
都需CAP_PERFMON 权限，否则会返回 -EPERM。当同一个事件名称存在多种格式时，所有同名的事件都将被尝试删除
如果只想删除某个特定版本，则应使/sys/kernel/tracing/dynamic_events 文件来删除该特定格式的事件

### 注销

如果在注册某个事件之后不再希望它被更新，则可以通过/sys/kernel/tracing/user_events_data 文件发出 ioctl() 来禁用它
要发出的命令DIAG_IOCSUNREG。这与删除不同，删除会真正将事件从系统中移除。注销只是告诉内核你的进程
不再关心该事件的更新

```

  struct user_unreg {
        /* Input: Size of the user_unreg structure being used */
        __u32 size;

        /* Input: Bit to unregister */
        __u8 disable_bit;

        /* Input: Reserved, set to 0 */
        __u8 __reserved;

        /* Input: Reserved, set to 0 */
        __u16 __reserved2;

        /* Input: Address to unregister */
        __u64 disable_addr;
  } __attribute__((__packed__));

```
struct user_unreg 要求正确设置上述所有输入字段

- size：必须设置为 sizeof(struct user_unreg)

- disable_bit：必须设置为要禁用的位（即之前通过 enable_bit 注册的同一个位）

- disable_addr：必须设置为要禁用的地址（即之前通过 enable_addr 注册的同一个地址）

**注意* 事件在调execve() 时会自动注销。在 fork() 期间，已注册的事件会被保留，如果希望注销，必须在每个进程中手动注销

### 状

当工具挂录制基于用户的事件时，事件的状态会实时更新。这使用户程序只在有东西真正挂接到该事件时，才承write() 
writev() 调用的开销

随着工具挂接或脱离该事件，内核会更新为该事件注册的指定位。用户程序只需检查该位是否被置位，就能知道是否有东西挂接

管理员可以轻松查看所有已注册事件的状态，方法是读
```

  Name [# Comments]
  ...

  Active: ActiveCount
  Busy: BusyCount

```
```

  test

  Active: 1
  Busy: 0

```
```

  test # Used by ftrace

  Active: 1
  Busy: 1

```
### 写入数据

注册事件后，用于注册的同一fd 可以用来为该事件写入一条记录。返回的 write_index 必须位于数据的最前面
其余数据则被视为该事件的负载（payload）

例如，如果返回的 write_index 1，而我想写出一int 类型的负载，那么数据的大小必须为 8 字节 int），
其中4 个字节等1，后 4 个字节等于我想要作为负载的值

```

  int index;
  int payload;

```
用户程序可能拥有众所周知的结构体，希望将其作为负载发出。在这种情况下可以使writev()，其中第一个向量是索引
后续的向量是实际的事件负载

```

  struct payload {
        int src;
        int dst;
        int flags;
  } __attribute__((__packed__));

```
```

  struct iovec io[2];
  struct payload e;

  io[0].iov_base = &write_index;
  io[0].iov_len = sizeof(write_index);
  io[1].iov_base = &e;
  io[1].iov_len = sizeof(e);

  writev(fd, (const struct iovec*)io, 2);

```
**注意* **write_index 不会被发出到正在录制trace 中*

### 示例代码

示例代码samples/user_events

