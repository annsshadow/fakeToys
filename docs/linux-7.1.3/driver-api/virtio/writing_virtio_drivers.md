

缂栧啓 Virtio 椹卞姩


绠€浠?


鏈枃妗ｄ綔涓洪┍鍔ㄥ紑鍙戜汉鍛橀渶瑕佺紪鍐欐柊鐨?virtio 椹卞姩鎴栫悊瑙ｇ幇鏈夐┍鍔ㄨ鐐规椂鐨勫熀鏈寚鍗椼€傛湁鍏?virtio 鐨勬€讳綋姒傝堪锛岃鍙傞槄 Virtio on Linux <virtio>銆?


椹卞姩鏍锋澘浠ｇ爜


浣滀负鏈€浣庤姹傦紝virtio 椹卞姩闇€瑕佸湪 virtio 鎬荤嚎涓婃敞鍐岋紝骞舵牴鎹澶囪鑼冧负鍏堕厤缃?virtqueue锛涢┍鍔ㄤ晶鐨?virtqueue 閰嶇疆蹇呴』涓庤澶囦腑鐨?virtqueue 瀹氫箟鐩稿尮閰嶃€備竴涓熀鏈殑椹卞姩妗嗘灦鍙兘濡備笅鎵€绀?
```

	#include <linux/virtio.h>
	#include <linux/virtio_ids.h>
	#include <linux/virtio_config.h>
	#include <linux/module.h>

	/* device private data (one per device) */
	struct virtio_dummy_dev {
		struct virtqueue *vq;
	};

	static void virtio_dummy_recv_cb(struct virtqueue *vq)
	{
		struct virtio_dummy_dev *dev = vq->vdev->priv;
		char *buf;
		unsigned int len;

		while ((buf = virtqueue_get_buf(dev->vq, &len)) != NULL) {
			/* process the received data */
		}
	}

	static int virtio_dummy_probe(struct virtio_device *vdev)
	{
		struct virtio_dummy_dev *dev = NULL;

		/* initialize device data */
		dev = kzalloc(sizeof(struct virtio_dummy_dev), GFP_KERNEL);
		if (!dev)
			return -ENOMEM;

		/* the device has a single virtqueue */
		dev->vq = virtio_find_single_vq(vdev, virtio_dummy_recv_cb, "input");
		if (IS_ERR(dev->vq)) {
			kfree(dev);
			return PTR_ERR(dev->vq);

		}
		vdev->priv = dev;

		/* from this point on, the device can notify and get callbacks */
		virtio_device_ready(vdev);

		return 0;
	}

	static void virtio_dummy_remove(struct virtio_device *vdev)
	{
		struct virtio_dummy_dev *dev = vdev->priv;

		/*
		 * disable vq interrupts: equivalent to
		 * vdev->config->reset(vdev)
		 */
		virtio_reset_device(vdev);

		/* detach unused buffers */
		while ((buf = virtqueue_detach_unused_buf(dev->vq)) != NULL) {
			kfree(buf);
		}

		/* remove virtqueues */
		vdev->config->del_vqs(vdev);

		kfree(dev);
	}

	static const struct virtio_device_id id_table[] = {
		{ VIRTIO_ID_DUMMY, VIRTIO_DEV_ANY_ID },
		{ 0 },
	};

	static struct virtio_driver virtio_dummy_driver = {
		.driver.name =  KBUILD_MODNAME,
		.id_table =     id_table,
		.probe =        virtio_dummy_probe,
		.remove =       virtio_dummy_remove,
	};

	module_virtio_driver(virtio_dummy_driver);
	MODULE_DEVICE_TABLE(virtio, id_table);
	MODULE_DESCRIPTION("Dummy virtio driver");
	MODULE_LICENSE("GPL");

```
姝ゅ鐨勮澶?id `VIRTIO_ID_DUMMY` 鏄竴涓崰浣嶇锛寁irtio 椹卞姩鍙簲涓鸿鑼冧腑瀹氫箟鐨勮澶囨坊鍔狅紝璇峰弬闃?include/uapi/linux/virtio_ids.h銆傚湪灏嗚鏂囦欢娣诲姞璁惧 id 涔嬪墠锛岃嚦灏戦渶瑕佸湪 virtio 瑙勮寖涓鐣欒 id銆?

濡傛灉鎮ㄧ殑椹卞姩鍦ㄥ叾 `init` 涓?`exit` 鏂规硶涓棤闇€鍋氫换浣曠壒娈婂鐞嗭紝鍙互浣跨敤 module_virtio_driver() 杈呭姪瀹忔潵鍑忓皯鏍锋澘浠ｇ爜鐨勬暟閲忋€?

`probe` 鏂规硶鍦ㄦ鎯呭喌涓嬪畬鎴愭渶灏戠殑椹卞姩璁剧疆锛堜负璁惧鏁版嵁鍒嗛厤鍐呭瓨锛夊苟鍒濆鍖?virtqueue銆倂irtio_device_ready() 鐢ㄤ簬鍚敤 virtqueue锛屽苟閫氱煡璁惧椹卞姩宸插噯澶囧ソ绠＄悊璇ヨ澶囷紙鈥淒RIVER_OK鈥濓級銆傛棤璁哄浣曪紝virtqueue 閮戒細鍦?`probe` 杩斿洖鍚庣敱鏍稿績鑷姩鍚敤銆?

   :identifiers: virtio_device_ready

鏃犺濡備綍锛屽湪鍚戝叾娣诲姞缂撳啿鍖轰箣鍓嶅繀椤诲厛鍚敤 virtqueue銆?

鍙戦€佷笌鎺ユ敹鏁版嵁


涓婅堪浠ｇ爜涓殑 virtio_dummy_recv_cb() 鍥炶皟浼氬湪璁惧瀹屾垚澶勭悊鏌愪釜鎻忚堪绗︽垨鎻忚堪绗﹂摼锛堢敤浜庤鍙栨垨鍐欏叆锛夊苟閫氱煡椹卞姩鏃惰瑙﹀彂銆傜劧鑰岋紝杩欏彧鏄?virtio 璁惧-椹卞姩閫氫俊杩囩▼鐨勫悗鍗婇儴鍒嗭紝鍥犱负鏃犺鏁版嵁浼犺緭鏂瑰悜濡備綍锛岄€氫俊鎬绘槸鐢遍┍鍔ㄥ彂璧枫€?

瑕佸皢缂撳啿鍖轰粠椹卞姩浼犺緭鍒拌澶囷紝棣栧厛蹇呴』鏍规嵁闇€瑕佷娇鐢?virtqueue_add_inbuf()銆乿irtqueue_add_outbuf() 鎴?virtqueue_add_sgs() 涓殑浠绘剰涓€涓紝灏嗙紦鍐插尯鈥斺€旀墦鍖呬负 `scatterlists`鈥斺€旀坊鍔犲埌鐩稿簲鐨?virtqueue锛屽叿浣撳彇鍐充簬鎮ㄩ渶瑕佹坊鍔犱竴涓緭鍏?`scatterlist`锛堜緵璁惧濉叆锛夈€佷竴涓緭鍑?`scatterlist`锛堜緵璁惧娑堣垂锛夎繕鏄涓?`scatterlists`銆傜劧鍚庯紝涓€鏃?virtqueue 璁剧疆瀹屾垚锛岃皟鐢?virtqueue_kick() 浼氬彂閫佷竴涓敱浠ヤ笅浠ｇ爜澶勭悊
```

	struct scatterlist sg[1];
	sg_init_one(sg, buffer, BUFLEN);
	virtqueue_add_inbuf(dev->vq, sg, 1, buffer, GFP_ATOMIC);
	virtqueue_kick(dev->vq);

```
   :identifiers: virtqueue_add_inbuf

   :identifiers: virtqueue_add_outbuf

   :identifiers: virtqueue_add_sgs

闅忓悗锛屽湪璁惧璇诲彇鎴栧啓鍏ラ┍鍔ㄥ噯澶囧ソ鐨勭紦鍐插尯骞跺洖閫氱煡鍚庯紝椹卞姩鍙互璋冪敤 virtqueue_get_buf() 鏉ヨ鍙栬澶囦骇鐢熺殑鏁版嵁锛堝鏋?virtqueue 鏄敤杈撳叆缂撳啿鍖鸿缃殑锛夛紝鎴栬€呬粎浠呮槸鍥炴敹杩欎簺宸茶璁惧娑堣垂鐨勭紦鍐插尯锛?

   :identifiers: virtqueue_get_buf_ctx

virtqueue 鍥炶皟鍙互浣跨敤 virtqueue_disable_cb() 涓庝竴绯诲垪鐨?virtqueue_enable_cb() 鍑芥暟鍒嗗埆绂佺敤鍜岄噸鏂板惎鐢ㄣ€傛洿澶氱粏鑺傝鍙傞槄 drivers/virtio/virtio_ring.c锛?

   :identifiers: virtqueue_disable_cb

   :identifiers: virtqueue_enable_cb

浣嗚娉ㄦ剰锛屽湪鏌愪簺鍦烘櫙涓嬩粛鍙兘瑙﹀彂涓€浜涜櫄鍋囧洖璋冦€傚彲闈犵鐢ㄥ洖璋冪殑鏂规硶鏄噸缃澶囨垨 virtqueue锛坴irtio_reset_device()锛夈€?


鍙傝€冩枃妗?


_`[^1^]` Virtio 瑙勮寖 v1.2锛?
https://docs.oasis-open.org/virtio/virtio/v1.2/virtio-v1.2.html

鍚屾椂璇锋煡鐪嬭瑙勮寖鐨勬洿楂樼増鏈€?
