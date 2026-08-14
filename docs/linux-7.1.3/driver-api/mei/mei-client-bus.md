
## Intel(R) 绠＄悊寮曟搸锛圡E锛夊鎴风鎬荤嚎 API


## 鍔ㄦ満


MEI 瀛楃璁惧瀵逛簬涓撶敤搴旂敤绋嬪簭浠庣敤鎴风┖闂村悜 Intel ME 涓殑浼楀鍥轰欢璁惧鍙戦€佸拰
鎺ユ敹鏁版嵁寰堟湁鐢ㄣ€傜劧鑰岋紝瀵逛簬 ME 鐨勬煇浜涘姛鑳借€岃█锛屽鐢ㄧ幇鏈夌殑杞欢鏍堝苟閫氳繃鐜版湁鐨?鍐呮牳瀛愮郴缁熸潵鏆撮湶瀹冧滑鏇存湁鎰忎箟銆?
涓轰簡鏃犵紳鎺ュ叆鍐呮牳璁惧椹卞姩妯″瀷锛屾垜浠湪 MEI 椹卞姩涔嬩笂娣诲姞浜嗕竴涓唴鏍歌櫄鎷熸€荤嚎
鎶借薄銆傝繖浣垮緱鍙互涓哄悇绉?MEI 鐗规€у疄鐜?Linux 鍐呮牳椹卞姩锛屼綔涓哄悇鑷瓙绯荤粺涓嫭绔嬬殑
瀹炰綋銆傜敋鑷冲彲浠ラ€氳繃鍚戠幇鏈変唬鐮佹坊鍔犱竴灞?MEI CL 鎬荤嚎灞傦紝鏉ユ綔鍦ㄥ湴澶嶇敤宸叉湁鐨?璁惧椹卞姩銆?

## MEI CL 鎬荤嚎 API


涓烘煇涓?MEI 瀹㈡埛绔疄鐜伴┍鍔ㄤ笌浠讳綍鍏跺畠鍩轰簬鎬荤嚎鐨勮澶囬┍鍔ㄩ潪甯哥浉浼笺€傞┍鍔ㄩ€氳繃
`include/linux/mei_cl_bus.c` 涓畾涔夌殑 `struct mei_cl_driver` 缁撴瀯灏嗚嚜宸?娉ㄥ唽涓?MEI CL 鎬荤嚎椹卞姩銆?

        struct mei_cl_driver {
                struct device_driver driver;
                const char *name;

                const struct mei_cl_device_id *id_table;

                int (**probe)(struct mei_cl_device **dev, const struct mei_cl_id *id);
                int (**remove)(struct mei_cl_device **dev);
        };



`include/linux/mod_devicetable.h` 涓畾涔夌殑 `struct mei_cl_device_id` 缁撴瀯鍏佽
椹卞姩灏嗚嚜宸辩粦瀹氬埌涓€涓澶囧悕銆?

        struct mei_cl_device_id {
                char name[MEI_CL_NAME_SIZE];
                uuid_le uuid;
                __u8    version;
                kernel_ulong_t driver_info;
        };

瑕佺湡姝ｅ湪 ME 瀹㈡埛绔€荤嚎涓婃敞鍐屼竴涓┍鍔紝蹇呴』璋冪敤 `mei_cl_add_driver` API銆傝繖
閫氬父鍦ㄦā鍧楀垵濮嬪寲鏃惰皟鐢ㄣ€?
涓€鏃﹂┍鍔ㄦ敞鍐屽苟缁戝畾鍒拌澶囷紝椹卞姩閫氬父浼氬皾璇曞湪璇ユ€荤嚎涓婂仛涓€浜?I/O锛岃€岃繖搴斿綋閫氳繃
`mei_cl_send` 鍜?`mei_cl_recv` 鍑芥暟瀹屾垚銆傛洿璇︾粏鐨勪俊鎭 API 涓€鑺傘€?
涓轰簡璁╅┍鍔ㄦ敹鍒版湁鍏冲緟澶勭悊娴侀噺鎴栦簨浠剁殑閫氱煡锛岄┍鍔ㄥ簲褰撳垎鍒€氳繃
`mei_cl_devev_register_rx_cb` 鍜?`mei_cldev_register_notify_cb` 鍑芥暟娉ㄥ唽鍥炶皟銆?

### API:

    :export: drivers/misc/mei/bus.c



## 绀轰緥


浣滀负涓€涓悊璁虹ず渚嬶紝鍋囪 ME 甯︽湁涓€涓?"contact" NFC IP銆傝璁惧鐨勯┍鍔ㄥ垵濮嬪寲鍜?閫€鍑轰緥绋嬪涓嬫墍绀猴細


        #define CONTACT_DRIVER_NAME "contact"

        static struct mei_cl_device_id contact_mei_cl_tbl[] = {
                { CONTACT_DRIVER_NAME, },

                /** required last entry **/
                { }
        };
        MODULE_DEVICE_TABLE(mei_cl, contact_mei_cl_tbl);

        static struct mei_cl_driver contact_driver = {
                .id_table = contact_mei_tbl,
                .name = CONTACT_DRIVER_NAME,

                .probe = contact_probe,
                .remove = contact_remove,
        };

        static int contact_init(void)
        {
                int r;

                r = mei_cl_driver_register(&contact_driver);
                if (r) {
                        pr_err(CONTACT_DRIVER_NAME ": driver registration failed\n");
                        return r;
                }

                return 0;
        }

        static void __exit contact_exit(void)
        {
                mei_cl_driver_unregister(&contact_driver);
        }

        module_init(contact_init);
        module_exit(contact_exit);

鑰岃椹卞姩绠€鍖栧悗鐨?probe 渚嬬▼濡備笅鎵€绀猴細


        int contact_probe(struct mei_cl_device **dev, struct mei_cl_device_id **id)
        {
                [...]
                mei_cldev_enable(dev);

                mei_cldev_register_rx_cb(dev, contact_rx_cb);

                return 0;
        }

鍦?probe 渚嬬▼涓紝椹卞姩棣栧厛浣胯兘 MEI 璁惧锛岀劧鍚庢敞鍐屼竴涓?rx 澶勭悊绋嬪簭锛岃繖灏藉彲鑳?鎺ヨ繎浜庢敞鍐屼竴涓嚎绋嬪寲 IRQ 澶勭悊绋嬪簭銆傝澶勭悊绋嬪簭鐨勫疄鐜伴€氬父浼氳皟鐢?`mei_cldev_recv`锛岀劧鍚庡鐞嗘帴鏀跺埌鐨勬暟鎹€?

        #define MAX_PAYLOAD 128
        #define HDR_SIZE 4
        static void conntact_rx_cb(struct mei_cl_device *cldev)
        {
                struct contact *c = mei_cldev_get_drvdata(cldev);
                unsigned char payload[MAX_PAYLOAD];
                ssize_t payload_sz;

                payload_sz = mei_cldev_recv(cldev, payload,  MAX_PAYLOAD)
                if (reply_size < HDR_SIZE) {
                        return;
                }

                c->process_rx(payload);

        }

## MEI 瀹㈡埛绔€荤嚎椹卞姩


- [hdcp](hdcp)
- [nfc](nfc)
