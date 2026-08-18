
## 璁惧鍐呭瓨 TCP锛圖evice Memory TCP锛?

## 绠€浠?

璁惧鍐呭瓨 TCP锛坉evmem TCP锛夋敮鎸佸皢鏁版嵁鐩存帴鎺ユ敹鍒拌澶囧唴瀛橈紙dmabuf锛変腑銆傝鐗规€у綋鍓嶉拡瀵?TCP 濂楁帴瀛楀疄鐜般€?

### 鏈轰細


澶ч噺鐨勬暟鎹紶杈撲互璁惧鍐呭瓨浣滀负婧愬拰/鎴栫洰鐨勫湴銆傚姞閫熷櫒鏋佸ぇ鍦板鍔犱簡姝ょ被浼犺緭鐨勬櫘閬嶆€с€備竴浜涗緥瀛愬寘鎷細

- 鍒嗗竷寮忚缁冿紝鍏朵腑浣嶄簬涓嶅悓涓绘満涓婄殑 ML 鍔犻€熷櫒锛堝 GPU锛変箣闂翠氦鎹㈡暟鎹€?
- 鍒嗗竷寮忚８鍧楀瓨鍌ㄥ簲鐢ㄤ笌杩滅▼ SSD 涔嬮棿浼犺緭澶ч噺鏁版嵁銆傚叾涓ぇ閮ㄥ垎鏁版嵁涓嶉渶瑕佷富鏈鸿繘琛屽鐞嗐€?
閫氬父锛岀綉缁滀腑鐨勮澶囧埌璁惧鏁版嵁浼犺緭琚疄鐜颁负浠ヤ笅浣庣骇鎿嶄綔锛氳澶囧埌涓绘満鎷疯礉銆佷富鏈哄埌涓绘満缃戠粶浼犺緭锛屼互鍙婁富鏈哄埌璁惧鎷疯礉銆?
娑夊強涓绘満鎷疯礉鐨勬暟鎹祦骞堕潪鏈€浼橈紝鐗瑰埆鏄浜庢壒閲忔暟鎹紶杈擄紝骞朵笖浼氱粰绯荤粺璧勬簮锛堝涓绘満鍐呭瓨甯﹀鍜?PCIe 甯﹀锛夊甫鏉ユ樉钁楀帇鍔涖€?
Devmem TCP 閫氳繃瀹炵幇濂楁帴瀛?API 鏉ヤ紭鍖栨鐢ㄤ緥锛屼娇鐢ㄦ埛鑳藉灏嗘帴鏀跺埌鐨勭綉缁滄暟鎹寘鐩存帴鏀惧叆璁惧鍐呭瓨銆?
鏁版嵁鍖呰浇鑽风洿鎺ヤ粠 NIC 杩涘叆璁惧鍐呭瓨銆?
鏁版嵁鍖呭ご閮ㄨ繘鍏ヤ富鏈哄唴瀛橈紝骞剁敱 TCP/IP 鍗忚鏍堟甯稿鐞嗐€侼IC 蹇呴』鏀寔澶撮儴鍒嗙锛坔eader split锛夋墠鑳藉疄鐜拌繖涓€鐐广€?
浼樼偣锛?
- 涓庣幇鏈夌殑"缃戠粶浼犺緭 + 璁惧鎷疯礉"璇箟鐩告瘮锛岀紦瑙ｄ富鏈哄唴瀛樺甫瀹藉帇鍔涖€?
- 閫氳繃灏嗘暟鎹紶杈撻檺鍒跺湪 PCIe 鏍戠殑鏈€浣庡眰绾э紝缂撹В PCIe 甯﹀鍘嬪姏锛岃€屼紶缁熻矾寰勪細灏嗘暟鎹粡鐢辨牴澶嶆潅浣擄紙root complex锛夊彂閫併€?

### 鏇村淇℃伅


  slides銆佽棰?    https://netdevconf.org/0x17/sessions/talk/device-memory-tcp.html

  patchset
    [PATCH net-next v24 00/13] Device Memory TCP
    https://lore.kernel.org/netdev/20240831004313.3713467-1-almasrymina@google.com/


## RX 鎺ュ彛


### 绀轰緥


./tools/testing/selftests/drivers/net/hw/ncdevmem:do_server 灞曠ず浜嗚缃 API 鐨?RX 璺緞鐨勭ず渚嬨€?

### NIC 璁剧疆


澶撮儴鍒嗙銆佹祦瀵煎悜锛坒low steering锛夊拰 RSS 鏄?devmem TCP 鎵€闇€鐨勫姛鑳姐€?
澶撮儴鍒嗙鐢ㄤ簬灏嗕紶鍏ョ殑鏁版嵁鍖呮媶鍒嗕负浣嶄簬涓绘満鍐呭瓨涓殑澶撮儴缂撳啿鍖猴紝浠ュ強浣嶄簬璁惧鍐呭瓨涓殑杞借嵎缂撳啿鍖恒€?
娴佸鍚戝拰 RSS 鐢ㄤ簬纭繚鍙湁浠?devmem 涓虹洰鏍囩殑鏁版嵁娴佹墠浼氳惤鍦ㄧ粦瀹氬埌 devmem 鐨?RX 闃熷垪涓娿€?
```

	# 鍚敤澶撮儴鍒嗙
	ethtool -G eth1 tcp-data-split on


	# 鍚敤娴佸鍚?	ethtool -K eth1 ntuple on

```
閰嶇疆 RSS 浠ュ皢鎵€鏈夋祦閲忎粠鐩爣 RX 闃熷垪锛坬ueue 15锛夊紩寮€锛屾柟娉曟槸
```

	ethtool --set-rxfh-indir eth1 equal 15


```
鐢ㄦ埛蹇呴』浣跨敤浠ヤ笅鏂瑰紡灏?dmabuf 缁戝畾鍒扮粰瀹?NIC 涓婄殑浠绘剰鏁伴噺鐨?RX 闃熷垪
```

	/* 灏?dmabuf 缁戝畾鍒?NIC RX 闃熷垪 15 */
	struct netdev_queue *queues;
	queues = malloc(sizeof(*queues) * 1);

	queues[0]._present.type = 1;
	queues[0]._present.idx = 1;
	queues[0].type = NETDEV_RX_QUEUE_TYPE_RX;
	queues[0].idx = 15;

	*ys = ynl_sock_create(&ynl_netdev_family, &yerr);

	req = netdev_bind_rx_req_alloc();
	netdev_bind_rx_req_set_ifindex(req, 1 /* ifindex */);
	netdev_bind_rx_req_set_dmabuf_fd(req, dmabuf_fd);
	__netdev_bind_rx_req_set_queues(req, queues, n_queue_index);

	rsp = netdev_bind_rx(*ys, req);

	dmabuf_id = rsp->dmabuf_id;


```
netlink API 杩斿洖涓€涓?dmabuf_id锛氫竴涓紩鐢ㄦ宸茬粦瀹?dmabuf 鐨勫敮涓€ ID銆?
鐢ㄦ埛鍙互閫氳繃鍏抽棴寤虹珛缁戝畾鐨?netlink 濂楁帴瀛楁潵灏?dmabuf 浠庣綉缁滆澶囪В缁戙€傛垜浠繖鏍峰仛鏄负浜嗗嵆浣?userspace 杩涚▼宕╂簝锛岀粦瀹氫篃浼氳嚜鍔ㄨВ闄ゃ€?
璇锋敞鎰忥紝浠讳綍鏉ヨ嚜浠绘剰瀵煎嚭鏂癸紙exporter锛夌殑琛屼负鑹ソ鐨?dmabuf 閮藉簲璇ヨ兘涓?devmem TCP 閰嶅悎宸ヤ綔锛屽嵆浣胯 dmabuf 瀹為檯涓婂苟闈炵敱璁惧鍐呭瓨鏀寔銆倁dmabuf 灏辨槸杩欐牱涓€涓緥瀛愶紝瀹冨皢鐢ㄦ埛鍐呭瓨锛堥潪璁惧鍐呭瓨锛夊寘瑁呭湪 dmabuf 涓€?

### 濂楁帴瀛楄缃?

```

	ethtool -N eth1 flow-type tcp4 ... queue 15


```
### 鎺ユ敹鏁版嵁


鐢ㄦ埛搴旂敤绋嬪簭蹇呴』鍚戝唴鏍歌〃鏄庡叾鑳藉鎺ユ敹
```

	ret = recvmsg(fd, &msg, MSG_SOCK_DEVMEM);

```
鏈寚瀹?MSG_SOCK_DEVMEM 鏍囧織鐨勫簲鐢ㄧ▼搴忓湪鎺ユ敹 devmem 鏁版嵁鏃跺皢鏀跺埌 EFAULT銆?
Devmem 鏁版嵁琚洿鎺ユ帴鏀跺埌缁戝畾鍒?NIC 鐨?dmabuf 涓紝浣嶄簬"NIC
```

		for (cm = CMSG_FIRSTHDR(&msg); cm; cm = CMSG_NXTHDR(&msg, cm)) {
			if (cm->cmsg_level != SOL_SOCKET ||
				(cm->cmsg_type != SCM_DEVMEM_DMABUF &&
				 cm->cmsg_type != SCM_DEVMEM_LINEAR))
				continue;

			dmabuf_cmsg = (struct dmabuf_cmsg *)CMSG_DATA(cm);

			if (cm->cmsg_type == SCM_DEVMEM_DMABUF) {
				/* 鍒嗙墖钀藉湪 dmabuf 涓€?				 *
				 * dmabuf_cmsg->dmabuf_id 鏄鍒嗙墖
				 * 鎵€钀藉叆鐨?dmabuf銆?				 *
				 * dmabuf_cmsg->frag_offset 鏄鍒嗙墖
				 * 鍦?dmabuf 涓捣濮嬬殑鍋忕Щ銆?				 *
				 * dmabuf_cmsg->frag_size 鏄垎鐗?				 * 鐨勫ぇ灏忋€?				 *
				 * dmabuf_cmsg->frag_token 鏄竴涓护鐗岋紝
				 * 鐢ㄤ簬绋嶅悗閲婃斁姝ゅ垎鐗囨椂寮曠敤瀹冦€?				 */

				struct dmabuf_token token;
				token.token_start = dmabuf_cmsg->frag_token;
				token.token_count = 1;
				continue;
			}

			if (cm->cmsg_type == SCM_DEVMEM_LINEAR)
				/* 鍒嗙墖钀藉湪绾挎€х紦鍐插尯涓€?				 *
				 * dmabuf_cmsg->frag_size 鏄垎鐗?				 * 鐨勫ぇ灏忋€?				 */
				continue;

		}

```
搴旂敤绋嬪簭鍙兘鏀跺埌 2 涓?cmsgs锛?
- SCM_DEVMEM_DMABUF锛氳繖琛ㄧず鍒嗙墖钀藉湪鐢?dmabuf_id 鎸囩ず鐨?dmabuf 涓€?
- SCM_DEVMEM_LINEAR锛氳繖琛ㄧず鍒嗙墖钀藉湪绾挎€х紦鍐插尯涓€傚綋 NIC 鏃犳硶鍦ㄥご閮ㄨ竟鐣屽鎷嗗垎鏁版嵁鍖咃紝瀵艰嚧閮ㄥ垎锛堟垨鍏ㄩ儴锛夎浇鑽疯惤鍏ヤ富鏈哄唴瀛樻椂锛岄€氬父浼氬彂鐢熻繖绉嶆儏鍐点€?
搴旂敤绋嬪簭鍙兘鏀朵笉鍒颁换浣?SO_DEVMEM_* cmsgs銆傝繖琛ㄧず钀藉湪鏈粦瀹氬埌 dmabuf 鐨?RX 闃熷垪涓婄殑闈?devmem 甯歌 TCP 鏁版嵁銆?

### 閲婃斁鍒嗙墖


閫氳繃 SCM_DEVMEM_DMABUF 鎺ユ敹鐨勫垎鐗囧湪鐢ㄦ埛澶勭悊璇ュ垎鐗囨湡闂磋鍐呮牳閿佸畾锛坧inned锛夈€傜敤鎴峰繀椤婚€氳繃浠ヤ笅鏂瑰紡灏嗗垎鐗囪繑杩樼粰鍐呮牳
```

	ret = setsockopt(client_fd, SOL_SOCKET, SO_DEVMEM_DONTNEED, &token,
			 sizeof(token));

```
鐢ㄦ埛蹇呴』纭繚鍙婃椂灏嗕护鐗岃繑杩樼粰鍐呮牳銆傚惁鍒欏皢鑰楀敖缁戝畾鍒?RX 闃熷垪鐨勬湁闄?dmabuf锛屽苟瀵艰嚧涓㈠寘銆?
鐢ㄦ埛浼犻€掔殑浠ょ墝涓嶅緱瓒呰繃 128 涓紝涓旀墍鏈変护鐗岀殑 token->token_count 鍚堣涓嶅緱瓒呰繃 1024 涓垎鐗囥€傚鏋滅敤鎴锋彁渚涚殑鍒嗙墖瓒呰繃 1024 涓紝鍐呮牳灏嗛噴鏀炬渶澶?1024 涓垎鐗囧苟鎻愬墠杩斿洖銆?
鍐呮牳杩斿洖瀹為檯閲婃斁鐨勫垎鐗囨暟閲忋€傚湪浠ヤ笅鎯呭喌涓嬶紝閲婃斁鐨勫垎鐗囨暟鍙兘灏戜簬鐢ㄦ埛鎻愪緵鐨勪护鐗屾暟閲忥細

(a) 鍐呮牳鍐呴儴娉勬紡 bug銆?(b) 鐢ㄦ埛浼犻€掍簡瓒呰繃 1024 涓垎鐗囥€?

## TX 鎺ュ彛


### 绀轰緥


./tools/testing/selftests/drivers/net/hw/ncdevmem:do_client 灞曠ず浜嗚缃 API 鐨?TX 璺緞鐨勭ず渚嬨€?

### NIC 璁剧疆


```

        struct netdev_bind_tx_req *req = NULL;
        struct netdev_bind_tx_rsp *rsp = NULL;
        struct ynl_error yerr;

        *ys = ynl_sock_create(&ynl_netdev_family, &yerr);

        req = netdev_bind_tx_req_alloc();
        netdev_bind_tx_req_set_ifindex(req, ifindex);
        netdev_bind_tx_req_set_fd(req, dmabuf_fd);

        rsp = netdev_bind_tx(*ys, req);

        tx_dmabuf_id = rsp->id;


```
netlink API 杩斿洖涓€涓?dmabuf_id锛氫竴涓紩鐢ㄦ宸茬粦瀹?dmabuf 鐨勫敮涓€ ID銆?
鐢ㄦ埛鍙互閫氳繃鍏抽棴寤虹珛缁戝畾鐨?netlink 濂楁帴瀛楁潵灏?dmabuf 浠庣綉缁滆澶囪В缁戙€傛垜浠繖鏍峰仛鏄负浜嗗嵆浣?userspace 杩涚▼宕╂簝锛岀粦瀹氫篃浼氳嚜鍔ㄨВ闄ゃ€?
璇锋敞鎰忥紝浠讳綍鏉ヨ嚜浠绘剰瀵煎嚭鏂圭殑琛屼负鑹ソ鐨?dmabuf 閮藉簲璇ヨ兘涓?devmem TCP 閰嶅悎宸ヤ綔锛屽嵆浣胯 dmabuf 瀹為檯涓婂苟闈炵敱璁惧鍐呭瓨鏀寔銆倁dmabuf 灏辨槸杩欐牱涓€涓緥瀛愶紝瀹冨皢鐢ㄦ埛鍐呭瓨锛堥潪璁惧鍐呭瓨锛夊寘瑁呭湪 dmabuf 涓€?

### 濂楁帴瀛楄缃?

鐢ㄦ埛鍦ㄥ彂閫?devmem TCP 鏃跺繀椤讳娇鐢?MSG_ZEROCOPY 鏍囧織銆侱evmem 鏃犳硶琚唴鏍告嫹璐濓紝鍥犳 devmem TX 鐨勮涔夌被浼间簬
```

	setsockopt(socket_fd, SOL_SOCKET, SO_ZEROCOPY, &opt, sizeof(opt));

```
杩樺缓璁敤鎴峰皢 TX 濂楁帴瀛楃粦瀹氬埌鍚屼竴鎺ュ彛
```

	setsockopt(socket_fd, SOL_SOCKET, SO_BINDTODEVICE, ifname, strlen(ifname) + 1);


```
### 鍙戦€佹暟鎹?

Devmem 鏁版嵁浣跨敤 SCM_DEVMEM_DMABUF cmsg 鍙戦€併€?
鐢ㄦ埛搴斿垱寤轰竴涓?msghdr锛屽叾涓細

- iov_base 璁剧疆涓?dmabuf 涓紑濮嬪彂閫佺殑鍋忕Щ
- iov_len 璁剧疆涓鸿浠?dmabuf 鍙戦€佺殑瀛楄妭鏁?
鐢ㄦ埛閫氳繃 dmabuf_tx_cmsg.dmabuf_id 浼犻€掕浠庝腑鍙戦€佺殑 dma-buf id銆?
涓嬮潰鐨勭ず渚嬩粠 dmabuf 鐨勫亸绉?100 澶勫彂閫?1024 瀛楄妭锛屼互鍙婁粠鍋忕Щ 2000 澶勫彂閫?2048 瀛楄妭
```

       char ctrl_data[CMSG_SPACE(sizeof(struct dmabuf_tx_cmsg))];
       struct dmabuf_tx_cmsg ddmabuf;
       struct msghdr msg = {};
       struct cmsghdr *cmsg;
       struct iovec iov[2];

       iov[0].iov_base = (void*)100;
       iov[0].iov_len = 1024;
       iov[1].iov_base = (void*)2000;
       iov[1].iov_len = 2048;

       msg.msg_iov = iov;
       msg.msg_iovlen = 2;

       msg.msg_control = ctrl_data;
       msg.msg_controllen = sizeof(ctrl_data);

       cmsg = CMSG_FIRSTHDR(&msg);
       cmsg->cmsg_level = SOL_SOCKET;
       cmsg->cmsg_type = SCM_DEVMEM_DMABUF;
       cmsg->cmsg_len = CMSG_LEN(sizeof(struct dmabuf_tx_cmsg));

       ddmabuf.dmabuf_id = tx_dmabuf_id;

       *((struct dmabuf_tx_cmsg *)CMSG_DATA(cmsg)) = ddmabuf;

       sendmsg(socket_fd, &msg, MSG_ZEROCOPY);


```
### 澶嶇敤 TX dmabuf


涓庡父瑙勫唴瀛樼殑 MSG_ZEROCOPY 绫讳技锛岀敤鎴峰湪鍙戦€佹搷浣滆繘琛屾湡闂翠笉搴斾慨鏀?dma-buf 鐨勫唴瀹广€傝繖鏄洜涓哄唴鏍镐笉浼氫繚鐣?dmabuf 鍐呭鐨勫壇鏈€傜浉鍙嶏紝鍐呮牳浼氶攣瀹氾紙pin锛夊苟鍙戦€?userspace 鍙敤鐨勭紦鍐插尯涓殑鏁版嵁銆?
姝ｅ MSG_ZEROCOPY 涓€鏍凤紝鍐呮牳閫氳繃浠ヤ笅鏂瑰紡閫氱煡 userspace 鍙戦€佸畬鎴?```

        int64_t tstop = gettimeofday_ms() + waittime_ms;
        char control[CMSG_SPACE(100)] = {};
        struct sock_extended_err *serr;
        struct msghdr msg = {};
        struct cmsghdr *cm;
        int retries = 10;
        __u32 hi, lo;

        msg.msg_control = control;
        msg.msg_controllen = sizeof(control);

        while (gettimeofday_ms() < tstop) {
                if (!do_poll(fd)) continue;

                ret = recvmsg(fd, &msg, MSG_ERRQUEUE);

                for (cm = CMSG_FIRSTHDR(&msg); cm; cm = CMSG_NXTHDR(&msg, cm)) {
                        serr = (void *)CMSG_DATA(cm);

                        hi = serr->ee_data;
                        lo = serr->ee_info;

                        fprintf(stdout, "tx complete [%d,%d]\n", lo, hi);
                }
        }

```
鐩稿簲鐨?sendmsg 瀹屾垚鍚庯紝dmabuf 鍗冲彲琚?userspace 澶嶇敤銆?

## 瀹炵幇涓庢敞鎰忎簨椤?

### 涓嶅彲璇?skb


Devmem 杞借嵎瀵瑰鐞嗘暟鎹寘鐨勫唴鏍告槸涓嶅彲璁块棶鐨勩€傝繖瀵艰嚧 devmem skb 鐨勮浇鑽峰嚭鐜颁竴浜涙€紓琛屼负锛?
- 鍥炵幆锛圠oopback锛夊姛鑳戒笉鍙敤銆傚洖鐜緷璧栨嫹璐濊浇鑽凤紝鑰岃繖瀵?devmem skb 鏄笉鍙兘鐨勩€?
- 杞欢鏍￠獙鍜岃绠楀け璐ャ€?
- TCP Dump 鍜?bpf 鏃犳硶璁块棶 devmem 鏁版嵁鍖呰浇鑽枫€?

## 娴嬭瘯


鏇寸湡瀹炵殑绀轰緥浠ｇ爜鍙互鍦ㄥ唴鏍告簮鐮佷腑鐨?`tools/testing/selftests/drivers/net/hw/ncdevmem.c` 涓嬫壘鍒般€?
ncdevmem 鏄竴涓?devmem TCP 鐗堢殑 netcat銆傚畠鐨勫伐浣滄柟寮忎笌 netcat 闈炲父鐩镐技锛屼絾浼氬皢鏁版嵁鐩存帴鎺ユ敹鍒?udmabuf 涓€?
瑕佽繍琛?ncdevmem锛屼綘闇€瑕佸湪琚祴鏈哄櫒涓婄殑鏈嶅姟鍣ㄤ笂杩愯瀹冿紝骞朵笖闇€瑕佸湪瀵圭杩愯 netcat 鏉ユ彁渚?TX 鏁版嵁銆?
ncdevmem 杩樻湁涓€涓獙璇佹ā寮忥紝鏈熸湜鎺ユ敹閲嶅妯″紡鐨勬暟鎹苟鎹杩涜楠岃瘉銆備緥濡傦紝浣犲彲浠ュ惎鍔?```

	ncdevmem -s <server IP> -c <client IP> -f <ifname> -l -p 5201 -v 7

```
鍦ㄥ鎴风锛屼娇鐢ㄥ父瑙?netcat 鍚?ncdevmem 杩涚▼鍙戦€?TX 鏁版嵁
```

	yes $(echo -e \\x01\\x02\\x03\\x04\\x05\\x06) | \
		tr \\n \\0 | head -c 5G | nc <server IP> 5201 -p 5201

```
