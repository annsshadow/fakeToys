## 閿佹満鍒?

鏈枃浠惰В閲婁簡 PCCARD 涓?PCMCIA 瀛愮郴缁熶腑浣跨敤鐨勫姞閿佷笌浜掓枼鏂规銆?

## A) 姒傝堪锛屽姞閿佸眰娆★細


pcmcia_socket_list_rwsem
 - 浠呬繚鎶ゅ鎺ュ瓧锛坰ocket锛夊垪琛?
- skt_mutex
 - 涓茶鍖栧崱鐨勬彃鍏?/ 寮瑰嚭

  - ops_mutex
 - 涓茶鍖栧鎺ュ瓧鎿嶄綔


## B) 浜掓枼


浠ヤ笅鍑芥暟浠ュ強瀵?struct pcmcia_socket 鐨勫洖璋冨繀椤?```

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
浠ヤ笅鍑芥暟浠ュ強瀵?struct pcmcia_socket 鐨勫洖璋冨繀椤?```

	socket_reset()
	socket_setup()

	struct pccard_operations	*ops
	struct pccard_resource_ops	*resource_ops;

```
娉ㄦ剰锛屼笉寰楁寔鏈?鈥渙ps_mutex鈥?鏃惰皟鐢?send_event() 鍜?`struct pcmcia_callback *callback`銆?

## C) 淇濇姢


### 1. 鍏ㄥ眬鏁版嵁锛?
struct list_head	pcmcia_socket_list;

鐢?pcmcia_socket_list_rwsem 淇濇姢锛?

### 2. 姣忓鎺ュ瓧鏁版嵁锛?
resource_ops 鍙婂叾鏁版嵁鐢?ops_mutex 淇濇姢銆?
鈥滀富鈥?struct pcmcia_socket 鐨勪繚鎶ゆ柟寮忓涓嬶紙鏈彁鍙婄殑鍙瀛楁鎴?鍗曟浣跨敤瀛楁锛夛細

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
### 3. 姣?PCMCIA 璁惧鏁版嵁锛?

鈥滀富鈥?struct pcmcia_device 鐨勪繚鎶ゆ柟寮忓涓嬶紙鏈彁鍙婄殑鍙瀛楁鎴?鍗曟浣跨敤瀛楁锛夛細


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
