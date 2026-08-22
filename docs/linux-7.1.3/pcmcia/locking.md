## 锁机

本文件解释了 PCCARD PCMCIA 子系统中使用的加锁与互斥方案

## A) 概述，加锁层次：


pcmcia_socket_list_rwsem
 - 仅保护套接字（socket）列
- skt_mutex
 - 串行化卡的插/ 弹出

  - ops_mutex
 - 串行化套接字操作


## B) 互斥


以下函数以及struct pcmcia_socket 的回调必```

	socket_detect_change()
	send_event()
	socket_reset()
	socket_shutdown()
	socket_setup()
	socket_remove()
	socket_insert()
	socket_early_resume()
	socket_late_resume()
	socket_resume()
	socket_suspend()

	struct pcmcia_callback	*callback

```
以下函数以及struct pcmcia_socket 的回调必```

	socket_reset()
	socket_setup()

	struct pccard_operations	*ops
	struct pccard_resource_ops	*resource_ops;

```
注意，不得持“ops_mutex时调send_event() `struct pcmcia_callback *callback`

## C) 保护


### 1. 全局数据
struct list_head	pcmcia_socket_list;

pcmcia_socket_list_rwsem 保护

### 2. 每套接字数据
resource_ops 及其数据ops_mutex 保护
“主struct pcmcia_socket 的保护方式如下（未提及的只读字段单次使用字段）：

```

	struct list_head	socket_list;

```
```

	unsigned int		thread_events;

```
```

	u_int			suspended_state;
	void			(*tune_bridge);
	struct pcmcia_callback	*callback;
	int			resume_status;

```
```

	socket_state_t		socket;
	u_int			state;
	u_short			lock_count;
	pccard_mem_map		cis_mem;
	void __iomem 		*cis_virt;
	struct { }		irq;
	io_window_t		io[];
	pccard_mem_map		win[];
	struct list_head	cis_cache;
	size_t			fake_cis_len;
	u8			*fake_cis;
	u_int			irq_mask;
	void 			(*zoom_video);
	int 			(*power_hook);
	u8			resource...;
	struct list_head	devices_list;
	u8			device_count;
	struct 			pcmcia_state;


```
### 3. PCMCIA 设备数据

“主struct pcmcia_device 的保护方式如下（未提及的只读字段单次使用字段）：


```

	struct list_head	socket_device_list;
	struct config_t		*function_config;
	u16			_irq:1;
	u16			_io:1;
	u16			_win:4;
	u16			_locked:1;
	u16			allow_func_id_match:1;
	u16			suspended:1;
	u16			_removed:1;

```
```

	io_req_t		io;
	irq_req_t		irq;
	config_req_t		conf;
	window_handle_t		win;

```
