## VDUSE - 鈥渧DPA Device in Userspace鈥濓紙鐢ㄦ埛绌洪棿涓殑 vDPA 璁惧锛?

vDPA锛坴irtio 鏁版嵁璺緞鍔犻€燂級璁惧鏄竴绉嶄娇鐢ㄧ鍚?virtio 瑙勮寖鐨勬暟鎹矾寰勩€佸苟閰嶅悎鍘傚晢鐗瑰畾鎺у埗璺緞鐨勮澶囥€倂DPA 璁惧鍙互鐗╃悊涓婁綅浜庣‖浠朵笂锛屼篃鍙互鐢辫蒋浠舵ā鎷熴€俈DUSE 鏄竴涓鏋讹紝浣垮緱鍦ㄧ敤鎴风┖闂翠腑瀹炵幇杞欢妯℃嫙鐨?vDPA 璁惧鎴愪负鍙兘銆備负浜嗕娇璁惧妯℃嫙鏇村畨鍏紝琚ā鎷熺殑 vDPA 璁惧鐨勬帶鍒惰矾寰勫湪鍐呮牳涓鐞嗭紝鍙湁鏁版嵁璺緞鍦ㄧ敤鎴风┖闂村疄鐜般€?
娉ㄦ剰锛岀洰鍓?VDUSE 妗嗘灦浠呮敮鎸?virtio 鍧楄澶囷紝杩欏彲浠ュ湪瀹炵幇鏁版嵁璺緞鐨勭敤鎴风┖闂磋繘绋嬬敱闈炵壒鏉冪敤鎴疯繍琛屾椂闄嶄綆瀹夊叏椋庨櫓銆傚鍏朵粬璁惧绫诲瀷鐨勬敮鎸佸彲浠ュ湪鐩稿簲璁惧椹卞姩鐨勫畨鍏ㄩ棶棰樺湪鏈潵琚緞娓呮垨淇鍚庢坊鍔犮€?
### 鍒涘缓/閿€姣?VDUSE 璁惧


VDUSE 璁惧鎸夊涓嬫柟寮忓垱寤猴細

1. 鍦?/dev/vduse/control 涓婄敤 ioctl(VDUSE_CREATE_DEV) 鍒涘缓涓€涓柊鐨?VDUSE 瀹炰緥銆?
2. 鍦?/dev/vduse/$NAME 涓婄敤 ioctl(VDUSE_VQ_SETUP) 璁剧疆姣忎釜 virtqueue銆?
3. 寮€濮嬪鐞嗘潵鑷?/dev/vduse/$NAME 鐨?VDUSE 娑堟伅銆傚墠鍑犳潯娑堟伅浼氬湪灏?VDUSE 瀹炰緥鎸傛帴鍒?vDPA 鎬荤嚎鏃跺埌杈俱€?
4. 鍙戦€?VDPA_CMD_DEV_NEW netlink 娑堟伅锛屽皢 VDUSE 瀹炰緥鎸傛帴鍒?vDPA 鎬荤嚎銆?
VDUSE 璁惧鎸夊涓嬫柟寮忛攢姣侊細

1. 鍙戦€?VDPA_CMD_DEV_DEL netlink 娑堟伅锛屽皢 VDUSE 瀹炰緥浠?vDPA 鎬荤嚎鍒嗙銆?
2. 鍏抽棴鎸囧悜 /dev/vduse/$NAME 鐨勬枃浠舵弿杩扮銆?
3. 鍦?/dev/vduse/control 涓婄敤 ioctl(VDUSE_DESTROY_DEV) 閿€姣?VDUSE 瀹炰緥銆?
netlink 娑堟伅鍙互閫氳繃 iproute2 涓殑 vdpa 宸ュ叿鍙戦€侊紝涔熷彲浠ヤ娇鐢ㄤ互涓嬬ず渚嬩唬鐮侊細


	static int netlink_add_vduse(const char *name, enum vdpa_command cmd)
	{
		struct nl_sock *nlsock;
		struct nl_msg *msg;
		int famid;

		nlsock = nl_socket_alloc();
		if (!nlsock)
			return -ENOMEM;

		if (genl_connect(nlsock))
			goto free_sock;

		famid = genl_ctrl_resolve(nlsock, VDPA_GENL_NAME);
		if (famid < 0)
			goto close_sock;

		msg = nlmsg_alloc();
		if (!msg)
			goto close_sock;

		if (!genlmsg_put(msg, NL_AUTO_PORT, NL_AUTO_SEQ, famid, 0, 0, cmd, 0))
			goto nla_put_failure;

		NLA_PUT_STRING(msg, VDPA_ATTR_DEV_NAME, name);
		if (cmd == VDPA_CMD_DEV_NEW)
			NLA_PUT_STRING(msg, VDPA_ATTR_MGMTDEV_DEV_NAME, "vduse");

		if (nl_send_sync(nlsock, msg))
			goto close_sock;

		nl_close(nlsock);
		nl_socket_free(nlsock);

		return 0;
	nla_put_failure:
		nlmsg_free(msg);
	close_sock:
		nl_close(nlsock);
	free_sock:
		nl_socket_free(nlsock);
		return -1;
	}

### VDUSE 濡備綍宸ヤ綔


濡備笂鎵€杩帮紝VDUSE 璁惧鐢卞湪 /dev/vduse/control 涓婄殑 ioctl(VDUSE_CREATE_DEV) 鍒涘缓銆傞€氳繃璇?ioctl锛岀敤鎴风┖闂村彲浠ユ寚瀹氫竴浜涘熀鏈厤缃紝渚嬪璁惧鍚嶇О锛堝敮涓€鏍囪瘑涓€涓?VDUSE 璁惧锛夈€乿irtio 鐗规€с€乿irtio 閰嶇疆绌洪棿銆乿irtqueue 鐨勬暟閲忕瓑锛岀敤浜庤繖涓妯℃嫙鐨勮澶囥€傜劧鍚庝細鍚戠敤鎴风┖闂村鍑轰竴涓瓧绗﹁澶囨帴鍙ｏ紙/dev/vduse/$NAME锛夌敤浜庤澶囨ā鎷熴€傜敤鎴风┖闂村彲浠ヤ娇鐢?/dev/vduse/$NAME 涓婄殑 VDUSE_VQ_SETUP ioctl 鍚戣澶囨坊鍔犳瘡涓?virtqueue 鐨勯厤缃紝渚嬪 virtqueue 鐨勬渶澶уぇ灏忋€?
鍒濆鍖栦箣鍚庯紝VDUSE 璁惧鍙互閫氳繃 VDPA_CMD_DEV_NEW netlink 娑堟伅鎸傛帴鍒?vDPA 鎬荤嚎銆傜敤鎴风┖闂撮渶瑕佸湪 /dev/vduse/$NAME 涓?read()/write()锛屼互浠?VDUSE 鍐呮牳妯″潡鎺ユ敹/鍥炲涓€浜涙帶鍒舵秷鎭紝濡備笅鎵€绀猴細


	static int vduse_message_handler(int dev_fd)
	{
		int len;
		struct vduse_dev_request req;
		struct vduse_dev_response resp;

		len = read(dev_fd, &req, sizeof(req));
		if (len != sizeof(req))
			return -1;

		resp.request_id = req.request_id;

		switch (req.type) {

		/** handle different types of messages **/

		}

		len = write(dev_fd, &resp, sizeof(resp));
		if (len != sizeof(resp))
			return -1;

		return 0;
	}

VDUSE 妗嗘灦鐩墠寮曞叆浜嗕笁绉嶇被鍨嬬殑娑堟伅锛?
- VDUSE_GET_VQ_STATE锛氳幏鍙?virtqueue 鐨勭姸鎬侊紝鐢ㄦ埛绌洪棿搴旇繑鍥?split virtqueue 鐨?avail 绱㈠紩锛屾垨 packed virtqueue 鐨勮澶?椹卞姩鐜洖缁曡鏁颁互鍙?avail 鍜?used 绱㈠紩銆?
- VDUSE_SET_STATUS锛氳缃澶囩姸鎬侊紝鐢ㄦ埛绌洪棿搴旈伒寰?virtio 瑙勮寖锛歨ttps://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.html 鏉ュ鐞嗘娑堟伅銆備緥濡傦紝濡傛灉璁惧鏃犳硶鎺ュ彈浠?VDUSE_DEV_GET_FEATURES ioctl 鑾峰緱鐨勫凡鍗忓晢 virtio 鐗规€э紝鍒欒缃?FEATURES_OK 璁惧鐘舵€佷綅澶辫触銆?
- VDUSE_UPDATE_IOTLB锛氶€氱煡鐢ㄦ埛绌洪棿鏇存柊鎸囧畾 IOVA 鑼冨洿鐨勫唴瀛樻槧灏勶紝鐢ㄦ埛绌洪棿搴旈鍏堢Щ闄ゆ棫鏄犲皠锛岀劧鍚庨€氳繃 VDUSE_IOTLB_GET_FD ioctl 寤虹珛鏂版槧灏勩€?
鍦ㄩ€氳繃 VDUSE_SET_STATUS 娑堟伅璁剧疆 DRIVER_OK 鐘舵€佷綅涔嬪悗锛岀敤鎴风┖闂村氨鍙互寮€濮嬫暟鎹潰澶勭悊锛屽涓嬫墍绀猴細

1. 鐢?VDUSE_VQ_GET_INFO ioctl 鑾峰彇鎸囧畾 virtqueue 鐨勪俊鎭紝鍖呮嫭澶у皬銆佹弿杩扮琛?鍙敤鐜?宸茬敤鐜殑 IOVA銆佺姸鎬佷互鍙婂氨缁姸鎬併€?
2. 灏嗕笂杩?IOVA 浼犵粰 VDUSE_IOTLB_GET_FD ioctl锛屼互渚垮皢杩欎簺 IOVA 鍖哄煙鏄犲皠鍒扮敤鎴风┖闂淬€備竴浜涚ず渚嬩唬鐮佸涓嬶細


	static int perm_to_prot(uint8_t perm)
	{
		int prot = 0;

		switch (perm) {
		case VDUSE_ACCESS_WO:
			prot |= PROT_WRITE;
			break;
		case VDUSE_ACCESS_RO:
			prot |= PROT_READ;
			break;
		case VDUSE_ACCESS_RW:
			prot |= PROT_READ | PROT_WRITE;
			break;
		}

		return prot;
	}

	static void **iova_to_va(int dev_fd, uint64_t iova, uint64_t **len)
	{
		int fd;
		void *addr;
		size_t size;
		struct vduse_iotlb_entry entry;

		entry.start = iova;
		entry.last = iova;

		/*
   - Find the first IOVA region that overlaps with the specified
   - range [start, last] and return the corresponding file descriptor.
		 */
		fd = ioctl(dev_fd, VDUSE_IOTLB_GET_FD, &entry);
		if (fd < 0)
			return NULL;

		size = entry.last - entry.start + 1;
		*len = entry.last - iova + 1;
		addr = mmap(0, size, perm_to_prot(entry.perm), MAP_SHARED,
			    fd, entry.offset);
		close(fd);
		if (addr == MAP_FAILED)
			return NULL;

		/*
   - Using some data structures such as linked list to store
   - the iotlb mapping. The munmap(2) should be called for the
   - cached mapping when the corresponding VDUSE_UPDATE_IOTLB
   - message is received or the device is reset.
		 */

		return addr + iova - entry.start;
	}

3. 鐢?VDUSE_VQ_SETUP_KICKFD ioctl 涓烘寚瀹?virtqueue 璁剧疆 kick eventfd銆俴ick eventfd 鐢?VDUSE 鍐呮牳妯″潡鐢ㄤ簬閫氱煡鐢ㄦ埛绌洪棿娑堣垂鍙敤鐜€傝繖鏄彲閫夌殑锛屽洜涓虹敤鎴风┖闂翠篃鍙互閫夋嫨杞鍙敤鐜€?
4. 鐩戝惉 kick eventfd锛堝彲閫夛級骞舵秷璐瑰彲鐢ㄧ幆銆傛弿杩扮琛ㄤ腑鎵€鎻忚堪鐨勬弿杩扮鎵€鎸囧悜鐨勭紦鍐插尯鍦ㄨ闂箣鍓嶄篃搴旈€氳繃 VDUSE_IOTLB_GET_FD ioctl 鏄犲皠鍒扮敤鎴风┖闂淬€?
5. 鍦ㄥ凡鐢ㄧ幆琚～鍏呬箣鍚庯紝鐢?VDUSE_INJECT_VQ_IRQ ioctl 涓虹壒瀹?virtqueue 娉ㄥ叆涓€涓腑鏂€?
### 鍚敤 ASID锛圓PI 鐗堟湰 1锛?

VDUSE 浠?API 鐗堟湰 1 寮€濮嬫敮鎸佹瘡鍦板潃绌洪棿鏍囪瘑绗︼紙ASID锛夈€傚湪閫氳繃 ioctl(VDUSE_CREATE_DEV) 鍒涘缓鏂扮殑 VDUSE 瀹炰緥涔嬪墠锛屽湪 `/dev/vduse/control` 涓婄敤 ioctl(VDUSE_SET_API_VERSION) 骞惰缃?`VDUSE_API_VERSION_1` 鏉ヨ繘琛岃缃€?
涔嬪悗锛屼綘鍙互浣跨敤 ioctl(VDUSE_VQ_SETUP) 鍙傛暟鐨?asid 鎴愬憳鏉ラ€夋嫨鎵€鏌ヨ IOTLB 鐨勫湴鍧€绌洪棿銆傞┍鍔ㄥ彲浠ラ€氳繃浣跨敤 VDUSE_SET_VQ_GROUP_ASID VDUSE 娑堟伅绫诲瀷鏇存敼浠讳綍 virtqueue 缁勭殑鍦板潃绌洪棿锛屽鏋滃彲浠ユ洿鏀癸紝VDUSE 瀹炰緥闇€瑕佷互 VDUSE_REQ_RESULT_OK 鍥炲銆?
绫讳技鍦帮紝浣犲彲浠ヤ娇鐢?ioctl(VDUSE_IOTLB_GET_FD2) 鑾峰彇鎻忚堪鐗瑰畾 ASID 鐨?IOVA 鍖哄煙鐨勬枃浠舵弿杩扮銆備娇鐢ㄧず渚嬶細


	static void *iova_to_va(int dev_fd, uint32_t asid, uint64_t iova,
	                        uint64_t *len)
	{
		int fd;
		void *addr;
		size_t size;
		struct vduse_iotlb_entry_v2 entry = { 0 };

		entry.v1.start = iova;
		entry.v1.last = iova;
		entry.asid = asid;

		fd = ioctl(dev_fd, VDUSE_IOTLB_GET_FD2, &entry);
		if (fd < 0)
			return NULL;

		size = entry.v1.last - entry.v1.start + 1;
		*len = entry.v1.last - iova + 1;
		addr = mmap(0, size, perm_to_prot(entry.v1.perm), MAP_SHARED,
			    fd, entry.v1.offset);
		close(fd);
		if (addr == MAP_FAILED)
			return NULL;

		/*
   - Using some data structures such as linked list to store
   - the iotlb mapping. The munmap(2) should be called for the
   - cached mapping when the corresponding VDUSE_UPDATE_IOTLB
   - message is received or the device is reset.
		 */

		return addr + iova - entry.v1.start;
	}

鍏充簬 uAPI 鐨勬洿澶氱粏鑺傦紝璇峰弬瑙?include/uapi/linux/vduse.h銆?