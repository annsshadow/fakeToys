## 通用 Mailbox 框架


:Author: Jassi Brar <jaswinder.singh@linaro.org>

本文档旨在帮助开发者为API 编写客户端与控制器驱动。但在开始之前，我们要先指出客户端（尤其是）与控制器驱动很可能具有极强的平台相关性，因为远端固件很可能是专有
的，并实现非标准协议。因此，即便两个平台都采用例PL320 控制器，其客户端驱动无法在二者之间共享。即便是 PL320 驱动本身，也可能需要适配某些平台相关的特殊行为所以该 API 的主要目的在于避免为每个平台重复编写相似的代码。话虽如此，这并不妨远端 f/w 同样基于 Linux 并在那里使用相同api。然而这些对我们本地都没有帮助，因为
我们只在客户端的协议层面打交道
实现过程中所做出的一些选择，正是源于这个“通用”框架的这种特殊性


## 控制器驱动（参见 include/linux/mailbox_controller.h


分配 mbox_controller 以及 mbox_chan 数组填充 mbox_chan_ops，除 flush() peek_data() 外其余均为必选控制器驱动可能通过收到 IRQ、轮询某个硬件标志，或永远无法得知（客户端通过协议得知来获知消息已被远端消费。优先级从高到低的方法为 IRQ -> Poll -> None，控制器驱动应通过
'txdone_irq' 'txdone_poll' 或两者皆不设置来指定

## 客户端驱动（参见 include/linux/mailbox_client.h


客户端可能希望以阻塞模式运行（同步地发送消息，发送完成后再返回），或以非阻塞/异步
模式运行（向 API 提交一条消息与一个回调函数，并立即返回）
```

	struct demo_client {
		struct mbox_client cl;
		struct mbox_chan *mbox;
		struct completion c;
		bool async;
		/* ... */
	};

	/*
	* This is the handler for data received from remote. The behaviour is purely
	* dependent upon the protocol. This is just an example.
	*/
	static void message_from_remote(struct mbox_client *cl, void *mssg)
	{
		struct demo_client *dc = container_of(cl, struct demo_client, cl);
		if (dc->async) {
			if (is_an_ack(mssg)) {
				/* An ACK to our last sample sent */
				return; /* Or do something else here */
			} else { /* A new message from remote */
				queue_req(mssg);
			}
		} else {
			/* Remote f/w sends only ACK packets on this channel */
			return;
		}
	}

	static void sample_sent(struct mbox_client *cl, void *mssg, int r)
	{
		struct demo_client *dc = container_of(cl, struct demo_client, cl);
		complete(&dc->c);
	}

	static void client_demo(struct platform_device *pdev)
	{
		struct demo_client *dc_sync, *dc_async;
		/* The controller already knows async_pkt and sync_pkt */
		struct async_pkt ap;
		struct sync_pkt sp;

		dc_sync = kzalloc(sizeof(*dc_sync), GFP_KERNEL);
		dc_async = kzalloc(sizeof(*dc_async), GFP_KERNEL);

		/* Populate non-blocking mode client */
		dc_async->cl.dev = &pdev->dev;
		dc_async->cl.rx_callback = message_from_remote;
		dc_async->cl.tx_done = sample_sent;
		dc_async->cl.tx_block = false;
		dc_async->cl.tx_tout = 0; /* doesn't matter here */
		dc_async->cl.knows_txdone = false; /* depending upon protocol */
		dc_async->async = true;
		init_completion(&dc_async->c);

		/* Populate blocking mode client */
		dc_sync->cl.dev = &pdev->dev;
		dc_sync->cl.rx_callback = message_from_remote;
		dc_sync->cl.tx_done = NULL; /* operate in blocking mode */
		dc_sync->cl.tx_block = true;
		dc_sync->cl.tx_tout = 500; /* by half a second */
		dc_sync->cl.knows_txdone = false; /* depending upon protocol */
		dc_sync->async = false;

		/* ASync mailbox is listed second in 'mboxes' property */
		dc_async->mbox = mbox_request_channel(&dc_async->cl, 1);
		/* Populate data packet */
		/* ap.xxx = 123; etc */
		/* Send async message to remote */
		mbox_send_message(dc_async->mbox, &ap);

		/* Sync mailbox is listed first in 'mboxes' property */
		dc_sync->mbox = mbox_request_channel(&dc_sync->cl, 0);
		/* Populate data packet */
		/* sp.abc = 123; etc */
		/* Send message to remote in blocking mode */
		mbox_send_message(dc_sync->mbox, &sp);
		/* At this point 'sp' has been sent */

		/* Now wait for async chan to be done */
		wait_for_completion(&dc_async->c);
	}

```
