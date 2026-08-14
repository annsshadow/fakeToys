## 瑙﹀彂缂撳啿鍖猴紙Triggered Buffers锛?

鏃㈢劧鎴戜滑宸茬粡鐭ラ亾缂撳啿鍖哄拰瑙﹀彂鍣ㄦ槸浠€涔堬紝璁╂垜浠湅鐪嬪畠浠浣曞崗鍚屽伐浣溿€?
## IIO 瑙﹀彂缂撳啿鍖鸿缃?

- `iio_triggered_buffer_setup` 鈥?璁剧疆瑙﹀彂缂撳啿鍖轰笌 pollfunc
- `iio_triggered_buffer_cleanup` 鈥?閲婃斁鐢?`iio_triggered_buffer_setup` 鍒嗛厤鐨?  璧勬簮
- struct iio_buffer_setup_ops 鈥?涓庣紦鍐插尯璁剧疆鐩稿叧鐨勫洖璋?
```

    const struct iio_buffer_setup_ops sensor_buffer_setup_ops = {
      .preenable    = sensor_buffer_preenable,
      .postenable   = sensor_buffer_postenable,
      .postdisable  = sensor_buffer_postdisable,
      .predisable   = sensor_buffer_predisable,
    };

    irqreturn_t sensor_iio_pollfunc(int irq, void *p)
    {
        pf->timestamp = iio_get_time_ns((struct indio_dev *)p);
        return IRQ_WAKE_THREAD;
    }

    irqreturn_t sensor_trigger_handler(int irq, void *p)
    {
        u16 buf[8];
        int i = 0;

        /* 璇诲彇姣忎釜娲诲姩閫氶亾鐨勬暟鎹?*/
        for_each_set_bit(bit, active_scan_mask, masklength)
            buf[i++] = sensor_get_data(bit)

        iio_push_to_buffers_with_timestamp(indio_dev, buf, timestamp);

        iio_trigger_notify_done(trigger);
        return IRQ_HANDLED;
    }

    /* 璁剧疆瑙﹀彂缂撳啿鍖猴紝閫氬父鍦?probe 鍑芥暟涓?*/
    iio_triggered_buffer_setup(indio_dev, sensor_iio_polfunc,
                               sensor_trigger_handler,
                               sensor_buffer_setup_ops);

```
杩欓噷闇€瑕佹敞鎰忕殑閲嶈浜嬮」鏈夛細

- `iio_buffer_setup_ops`锛岀紦鍐插尯閰嶇疆搴忓垪涓瀹氫箟鐐癸紙渚嬪鍚敤鍓嶃€佺鐢ㄥ悗锛夎璋冪敤鐨?  缂撳啿鍖鸿缃嚱鏁般€傚鏋滄湭鎸囧畾锛孖IO 鏍稿績浣跨敤榛樿鐨?iio_triggered_buffer_setup_ops銆?- **sensor_iio_pollfunc**锛屽皢鐢ㄤ綔 poll 鍑芥暟涓婂崐閮ㄧ殑鍑芥暟銆傚畠搴旇灏藉彲鑳藉皯鍦板鐞嗭紝鍥犱负
  瀹冨湪涓柇涓婁笅鏂囦腑杩愯銆傛渶甯歌鐨勬搷浣滄槸璁板綍褰撳墠鏃堕棿鎴筹紝鍥犳鍙互浣跨敤 IIO 鏍稿績瀹氫箟鐨?  `iio_pollfunc_store_time` 鍑芥暟銆?- **sensor_trigger_handler**锛屽皢鐢ㄤ綔 poll 鍑芥暟涓嬪崐閮ㄧ殑鍑芥暟銆傚畠鍦ㄥ唴鏍哥嚎绋嬬殑涓婁笅鏂囦腑
  杩愯锛屾墍鏈夊鐞嗛兘鍦ㄨ繖閲岃繘琛屻€傚畠閫氬父浠庤澶囪鍙栨暟鎹紝骞朵笌涓婂崐閮ㄨ褰曠殑鏃堕棿鎴充竴璧?  瀛樺叆鍐呴儴缂撳啿鍖恒€?
## 鏇村缁嗚妭
