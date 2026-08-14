## fwctl bnxt 椹卞姩


:Author: Pavan Chebbi

## 姒傝堪


BNXT 椹卞姩閫氳繃 auxiliary_device 鎻愪緵 fwctl 鏈嶅姟銆俠nxt_fwctl 椹卞姩缁戝畾鍒拌璁惧锛屽苟鍚?fwctl
瀛愮郴缁熸敞鍐岃嚜宸便€?
bnxt_fwctl 椹卞姩瀵硅澶囧浐浠跺唴閮ㄤ竴鏃犳墍鐭ャ€傚畠浣跨敤 bnxt 鎻愪緵鐨勪笂灞傚崗璁紙ULP锛夐€氶亾鏉ュ悜鍥轰欢鍙戦€?纭欢璧勬簮绠＄悊鍣紙HWRM锛夊懡浠ゃ€?
杩欎簺鍛戒护鍙互鏌ヨ鎴栨洿鏀圭敱鍥轰欢椹卞姩鐨勮澶囬厤缃紝浠ュ強璇诲啓瀵硅皟璇曟湁鐢ㄧ殑瀵勫瓨鍣ㄣ€?
## bnxt_fwctl 鐢ㄦ埛 API


姣忎釜 RPC 璇锋眰鍦?fwctl_rpc 鐨?'in' 缂撳啿鍖轰腑鍖呭惈 HWRM 杈撳叆缁撴瀯锛岃€?'out' 灏嗗寘鍚搷搴斻€?
涓€涓吀鍨嬬殑鐢ㄦ埛搴旂敤绋嬪簭鍙互浣跨敤 ioctl() 鍙戦€?FWCTL_INFO 鍛戒护鏉ュ彂鐜?bnxt_fwctl 鐨?RPC 鑳藉姏锛?濡備笅鎵€绀猴細

        ioctl(fd, FWCTL_INFO, &fwctl_info_msg);

鍏朵腑 fwctl_info_msg锛堢被鍨嬩负 struct fwctl_info锛夋弿杩颁簡 bnxt_info_msg锛堢被鍨嬩负 struct fwctl_info_bnxt锛夈€?fwctl_info_msg 璁剧疆濡備笅锛?
        size = sizeof(struct fwctl_info);
        flags = 0;
        device_data_len = sizeof(bnxt_info_msg);
        out_device_data = (__aligned_u64)&bnxt_info_msg;

bnxt_info_msg 鐨?uctx_caps 琛ㄧず include/uapi/fwctl/bnxt.h 涓?fwctl_bnxt_commands 鎵€鎻忚堪鐨?鑳藉姏銆?
FW RPC 鏈韩锛孎WCTL_RPC 鍙娇鐢?ioctl() 鍙戦€侊紝濡備笅鎵€绀猴細

        ioctl(fd, FWCTL_RPC, &fwctl_rpc_msg);

鍏朵腑 fwctl_rpc_msg锛堢被鍨嬩负 struct fwctl_rpc锛夊湪鍏?'in' 缂撳啿鍖轰腑鎼哄甫 HWRM 鍛戒护銆侶WRM 杈撳叆
缁撴瀯鍦?include/linux/bnxt/hsi.h 涓弿杩般€侶WRM_VER_GET 鐨勭ず渚嬪涓嬫墍绀猴細

        struct hwrm_ver_get_output resp;
        struct fwctl_rpc fwctl_rpc_msg;
        struct hwrm_ver_get_input req;

        req.req_type = HWRM_VER_GET;
        req.hwrm_intf_maj = HWRM_VERSION_MAJOR;
        req.hwrm_intf_min = HWRM_VERSION_MINOR;
        req.hwrm_intf_upd = HWRM_VERSION_UPDATE;
        req.cmpl_ring = -1;
        req.target_id = -1;

        fwctl_rpc_msg.size = sizeof(struct fwctl_rpc);
        fwctl_rpc_msg.scope = FWCTL_RPC_DEBUG_READ_ONLY;
        fwctl_rpc_msg.in_len = sizeof(req);
        fwctl_rpc_msg.out_len = sizeof(resp);
        fwctl_rpc_msg.in = (__aligned_u64)&req;
        fwctl_rpc_msg.out = (__aligned_u64)&resp;

鍙互缁冧範姝ゆ帴鍙ｇ殑绀轰緥 python3 绋嬪簭鍙湪浠ヤ笅 git 浠撳簱涓壘鍒帮細

https://github.com/Broadcom/fwctl-tools
