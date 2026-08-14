
## fwctl cxl driver


:Author: Dave Jiang

## Overview


CXL 瑙勮寖瀹氫箟浜嗕竴缁勫彲浠ュ彂閫佸埌 CXL 璁惧鎴栦氦鎹㈡満閭鐨勫懡浠ゃ€傚畠涔熶负鍙戦€佸埌閭鐨勫巶鍟嗙壒瀹氬懡浠ょ暀鍑轰簡绌洪棿銆俧wctl 鎻愪緵浜嗕竴鏉¤矾寰勶紝鍏佽鐢ㄦ埛绌洪棿鍙戦€佷竴缁勮鍏佽鐨勪俊绠卞懡浠ゅ埌璁惧锛岃繖浜涘懡浠ょ敱鍐呮牳椹卞姩杩涜璋冭妭銆?
灏嗕娇鐢ㄤ互涓?3 鏉″懡浠ゆ潵鏀寔 CXL 鐗规€э細
CXL spec r3.1 8.2.9.6.1 Get Supported Features (Opcode 0500h)
CXL spec r3.1 8.2.9.6.2 Get Feature (Opcode 0501h)
CXL spec r3.1 8.2.9.6.3 Set Feature (Opcode 0502h)

"Get Supported Features" 鐨勮繑鍥炴暟鎹彲鑳戒細琚唴鏍搁┍鍔ㄨ繃婊わ紝浠ヤ涪寮冧换浣曡鍐呮牳绂佹鎴栨琚唴鏍哥嫭鍗犱娇鐢ㄧ殑鐗规€с€傞┍鍔ㄤ細灏?"Get Supported Features Supported Feature Entry" 鐨?"Set Feature Size" 璁句负 0锛屼互琛ㄧず璇ョ壒鎬т笉鍙淇敼銆?Get Supported Features" 鍛戒护鍜?"Get Features" 灞炰簬 FWCTL_RPC_CONFIGURATION 鐨?fwctl 绛栫暐鑼冪暣銆?
瀵逛簬 "Set Feature" 鍛戒护锛岃闂瓥鐣ョ洰鍓嶆牴鎹澶囨姤鍛婄殑 Set Feature 褰卞搷锛坋ffects锛夊垎涓轰袱绫汇€傚鏋?Set Feature 浼氬鑷磋澶囩珛鍗冲彂鐢熷彉鏇达紝鍒?fwctl 璁块棶绛栫暐蹇呴』鏄?FWCTL_RPC_DEBUG_WRITE_FULL銆傝绾у埆鐨勫奖鍝嶆帺鐮侊紙set effects mask锛変负 "immediate config change"銆?immediate data change"銆?immediate policy change" 鎴?"immediate log change"銆傚鏋滃奖鍝嶆槸 "config change with cold reset" 鎴?"config change with conventional reset"锛屽垯 fwctl 璁块棶绛栫暐蹇呴』鏄?FWCTL_RPC_DEBUG_WRITE 鎴栨洿楂樸€?
## fwctl cxl User API


### 1. Driver info query


搴旂敤绋嬪簭鐨勭涓€姝ユ槸鍙戝嚭 ioctl(FWCTL_CMD_INFO)銆傛垚鍔熻皟鐢ㄨ ioctl 鎰忓懗鐫€ Features 鑳藉姏鍙敤锛屽苟杩斿洖涓€涓叏涓?0 鐨?32 浣嶈礋杞姐€傞渶瑕佺敤 `FWCTL_DEVICE_TYPE_CXL` 濉厖 `fwctl_info.out_device_type` 鏉ュ～鍐?`struct fwctl_info`銆傝繑鍥炵殑鏁版嵁搴斾负 `struct fwctl_info_cxl`锛屽叾涓寘鍚竴涓簲鍏ㄤ负 0 鐨勪繚鐣?32 浣嶅瓧娈点€?
### 2. Send hardware commands


涓嬩竴姝ユ槸浠庣敤鎴风┖闂撮€氳繃 ioctl(FWCTL_RPC) 鍚戦┍鍔ㄥ彂閫?'Get Supported Features' 鍛戒护銆傜敱 `fwctl_rpc.in` 鎸囧悜涓€涓?`struct fwctl_rpc_cxl`銆俙struct fwctl_rpc_cxl.in_payload` 鎸囧悜鐢?CXL 瑙勮寖瀹氫箟鐨勭‖浠惰緭鍏ョ粨鏋勩€俙fwctl_rpc.out` 鎸囧悜鍖呭惈 `struct fwctl_rpc_cxl_out` 鐨勭紦鍐插尯锛屽悗鑰呭皢纭欢杈撳嚭鏁版嵁鍐呰仈涓?`fwctl_rpc_cxl_out.payload`銆傝鍛戒护浼氳璋冪敤涓ゆ銆傜涓€娆＄敤浜庤幏鍙栨墍鏀寔鐗规€х殑鏁伴噺銆傜浜屾鐢ㄤ簬鑾峰彇鍏蜂綋鐨勭壒鎬ц鎯呬綔涓鸿緭鍑烘暟鎹€?
鍦ㄨ幏寰楀叿浣撶殑鐗规€ц鎯呭悗锛屽氨鍙互閫傚綋鍦扮紪鍐欏苟鍙戦€?Get/Set Feature 鍛戒护銆傚浜?"Set Feature" 鍛戒护锛屾墍妫€绱㈠埌鐨勭壒鎬т俊鎭寘鍚竴涓?effects 瀛楁锛岃缁嗚鏄庡皢瑕佽Е鍙戠殑 "Set Feature" 鍛戒护鐨勭粨鏋溿€傝繖浼氬憡鐭ョ敤鎴风郴缁熸槸鍚﹁閰嶇疆涓哄厑璁歌 "Set Feature" 鍛戒护銆?
#### Code example of a Get Feature



        static int cxl_fwctl_rpc_get_test_feature(int fd, struct test_feature *feat_ctx,
                                                  const uint32_t expected_data)
        {
                struct cxl_mbox_get_feat_in *feat_in;
                struct fwctl_rpc_cxl_out *out;
                struct fwctl_rpc rpc = {0};
                struct fwctl_rpc_cxl *in;
                size_t out_size, in_size;
                uint32_t val;
                void *data;
                int rc;

                in_size = sizeof(**in) + sizeof(**feat_in);
                rc = posix_memalign((void **)&in, 16, in_size);
                if (rc)
                        return -ENOMEM;
                memset(in, 0, in_size);
                feat_in = &in->get_feat_in;

                uuid_copy(feat_in->uuid, feat_ctx->uuid);
                feat_in->count = feat_ctx->get_size;

                out_size = sizeof(*out) + feat_ctx->get_size;
                rc = posix_memalign((void **)&out, 16, out_size);
                if (rc)
                        goto free_in;
                memset(out, 0, out_size);

                in->opcode = CXL_MBOX_OPCODE_GET_FEATURE;
                in->op_size = sizeof(*feat_in);

                rpc.size = sizeof(rpc);
                rpc.scope = FWCTL_RPC_CONFIGURATION;
                rpc.in_len = in_size;
                rpc.out_len = out_size;
                rpc.in = (uint64_t)(uint64_t *)in;
                rpc.out = (uint64_t)(uint64_t *)out;

                rc = send_command(fd, &rpc, out);
                if (rc)
                        goto free_all;

                data = out->payload;
                val = le32toh(**(__le32 **)data);
                if (memcmp(&val, &expected_data, sizeof(val)) != 0) {
                        rc = -ENXIO;
                        goto free_all;
                }

        free_all:
                free(out);
        free_in:
                free(in);
                return rc;
        }

鏈夊叧濡備綍杩愮敤姝よ矾寰勭殑璇︾粏鐢ㄦ埛浠ｇ爜绀轰緥锛岃鍙傞槄 CXL CLI 娴嬭瘯鐩綍
<https://github.com/pmem/ndctl/tree/main/test/fwctl.c>銆?

## fwctl cxl Kernel API


   :export:
