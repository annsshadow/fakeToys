## 閫氱敤 Mailbox 妗嗘灦


:Author: Jassi Brar <jaswinder.singh@linaro.org>

鏈枃妗ｆ棬鍦ㄥ府鍔╁紑鍙戣€呬负璇?API 缂栧啓瀹㈡埛绔笌鎺у埗鍣ㄩ┍鍔ㄣ€備絾鍦ㄥ紑濮嬩箣鍓嶏紝鎴戜滑瑕佸厛鎸囧嚭锛?瀹㈡埛绔紙灏ゅ叾鏄級涓庢帶鍒跺櫒椹卞姩寰堝彲鑳藉叿鏈夋瀬寮虹殑骞冲彴鐩稿叧鎬э紝鍥犱负杩滅鍥轰欢寰堝彲鑳芥槸涓撴湁
鐨勶紝骞跺疄鐜伴潪鏍囧噯鍗忚銆傚洜姝わ紝鍗充究涓や釜骞冲彴閮介噰鐢ㄤ緥濡?PL320 鎺у埗鍣紝鍏跺鎴风椹卞姩涔?鏃犳硶鍦ㄤ簩鑰呬箣闂村叡浜€傚嵆渚挎槸 PL320 椹卞姩鏈韩锛屼篃鍙兘闇€瑕侀€傞厤鏌愪簺骞冲彴鐩稿叧鐨勭壒娈婅涓恒€?鎵€浠ヨ API 鐨勪富瑕佺洰鐨勫湪浜庨伩鍏嶄负姣忎釜骞冲彴閲嶅缂栧啓鐩镐技鐨勪唬鐮併€傝瘽铏藉姝わ紝杩欏苟涓嶅Θ纰?杩滅 f/w 鍚屾牱鍩轰簬 Linux 骞跺湪閭ｉ噷浣跨敤鐩稿悓鐨?api銆傜劧鑰岃繖浜涘鎴戜滑鏈湴閮芥病鏈夊府鍔╋紝鍥犱负
鎴戜滑鍙湪瀹㈡埛绔殑鍗忚灞傞潰鎵撲氦閬撱€?
瀹炵幇杩囩▼涓墍鍋氬嚭鐨勪竴浜涢€夋嫨锛屾鏄簮浜庤繖涓€滈€氱敤鈥濇鏋剁殑杩欑鐗规畩鎬с€?


## 鎺у埗鍣ㄩ┍鍔紙鍙傝 include/linux/mailbox_controller.h锛?


鍒嗛厤 mbox_controller 浠ュ強 mbox_chan 鏁扮粍銆?濉厖 mbox_chan_ops锛岄櫎 flush() 涓?peek_data() 澶栧叾浣欏潎涓哄繀閫夈€?鎺у埗鍣ㄩ┍鍔ㄥ彲鑳介€氳繃鏀跺埌 IRQ銆佽疆璇㈡煇涓‖浠舵爣蹇楋紝鎴栨案杩滄棤娉曞緱鐭ワ紙瀹㈡埛绔€氳繃鍗忚寰楃煡锛?鏉ヨ幏鐭ユ秷鎭凡琚繙绔秷璐广€備紭鍏堢骇浠庨珮鍒颁綆鐨勬柟娉曚负 IRQ -> Poll -> None锛屾帶鍒跺櫒椹卞姩搴旈€氳繃
'txdone_irq' 鎴?'txdone_poll' 鎴栦袱鑰呯殕涓嶈缃潵鎸囧畾銆?

## 瀹㈡埛绔┍鍔紙鍙傝 include/linux/mailbox_client.h锛?


瀹㈡埛绔彲鑳藉笇鏈涗互闃诲妯″紡杩愯锛堝悓姝ュ湴鍙戦€佹秷鎭紝鍙戦€佸畬鎴愬悗鍐嶈繑鍥烇級锛屾垨浠ラ潪闃诲/寮傛
妯″紡杩愯锛堝悜 API 鎻愪氦涓€鏉℃秷鎭笌涓€涓洖璋冨嚱鏁帮紝骞剁珛鍗宠繑鍥烇級銆?
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
