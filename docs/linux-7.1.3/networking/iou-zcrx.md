
## io_uring 闆舵嫹璐濇帴鏀讹紙Rx锛?

## 绠€浠?

io_uring 闆舵嫹璐濇帴鏀讹紙ZC Rx锛夋槸涓€椤瑰湪缃戠粶鎺ユ敹璺緞涓婃秷闄ゅ唴鏍稿埌鐢ㄦ埛鎷疯礉鐨勭壒鎬э紝鍏佽鏁版嵁鍖呮暟鎹鐩存帴鎺ユ敹鍒扮敤鎴风┖闂村唴瀛樹腑銆傝鐗规€т笌 TCP_ZEROCOPY_RECEIVE 鐨勪笉鍚屼箣澶勫湪浜庯紝娌℃湁涓ユ牸鐨勫榻愯姹傦紝涔熶笉闇€瑕?mmap()/munmap()銆備笌 DPDK 绛夊唴鏍告梺璺柟妗堢浉姣旓紝鏁版嵁鍖呭ご鐢卞唴鏍?TCP 鏍堟甯稿鐞嗐€?
## NIC 纭欢闇€姹?

io_uring ZC Rx 宸ヤ綔闇€瑕佽嫢骞?NIC 纭欢鐗规€с€傜洰鍓嶅唴鏍?API 涓嶄細閰嶇疆 NIC锛屽繀椤荤敱鐢ㄦ埛鏉ュ畬鎴愩€?
### 澶?鏁版嵁鍒嗙


闇€瑕佸湪 L4 杈圭晫灏嗘暟鎹寘鎷嗗垎涓哄ご閮ㄤ笌璐熻浇銆傚ご閮ㄥ儚寰€甯镐竴鏍疯鎺ユ敹鍒板唴鏍稿唴瀛樹腑锛屽苟鐢?TCP 鏍堟甯稿鐞嗐€傝礋杞借鐩存帴鎺ユ敹鍒扮敤鎴风┖闂村唴瀛樹腑銆?
### 娴佸鍚?

涓烘鐗规€ч厤缃簡鐗瑰畾鐨勭‖浠?Rx 闃熷垪锛屼絾鐜颁唬 NIC 閫氬父灏嗘祦鍒嗗竷鍒版墍鏈夌‖浠?Rx 闃熷垪涓娿€傞渶瑕佹祦瀵煎悜锛坒low steering锛夋潵纭繚鍙湁鏈熸湜鐨勬祦琚鍚戝埌涓?io_uring ZC Rx 閰嶇疆鐨勭‖浠堕槦鍒椼€?
### RSS


闄や簡涓婇潰鐨勬祦瀵煎悜涔嬪锛岃繕闇€瑕?RSS 鏉ュ皢鎵€鏈夊叾浠栭潪闆舵嫹璐濇祦浠庝负 io_uring ZC Rx 閰嶇疆鐨勯槦鍒椾笂寮曞紑銆?
## 鐢ㄦ硶


### 閰嶇疆 NIC


鐩墠蹇呴』鍦ㄥ甫澶栧畬鎴愩€?
```

  ethtool -L eth0 combined 2

```
```

  ethtool -G eth0 tcp-data-split on

```
```

  ethtool -X eth0 equal 1

```
```

  ethtool -N eth0 flow-type tcp6 ... action 1

```
### 閰嶇疆 io_uring


鏈妭鎻忚堪搴曞眰鐨?io_uring 鍐呮牳 API銆傚叧浜庡浣曚娇鐢ㄩ珮灞?API锛岃鍙傝€?liburing 鏂囨。銆?
```

  IORING_SETUP_SINGLE_ISSUER
  IORING_SETUP_DEFER_TASKRUN
  IORING_SETUP_CQE32 or IORING_SETUP_CQE_MIXED

```
### 鍒涘缓鍐呭瓨鍖哄煙


```

  void *area_ptr = mmap(NULL, area_size,
                        PROT_READ | PROT_WRITE,
                        MAP_ANONYMOUS | MAP_PRIVATE,
                        0, 0);

```
### 鍒涘缓琛ュ厖鐜?

```

  void *ring_ptr = mmap(NULL, ring_size,
                        PROT_READ | PROT_WRITE,
                        MAP_ANONYMOUS | MAP_PRIVATE,
                        0, 0);

```
璇ヨˉ鍏呯幆鐢卞ご閮ㄧ殑涓€浜涚┖闂达紝鍔犱笂涓€涓暟缁勭粍鎴?```

  size_t rq_entries = 4096;
  size_t ring_size = rq_entries * sizeof(struct io_uring_zcrx_rqe) + PAGE_SIZE;
  /* align to page size */
  ring_size = (ring_size + (PAGE_SIZE - 1)) & ~(PAGE_SIZE - 1);

```
### 娉ㄥ唽 ZC Rx


```

  struct io_uring_zcrx_area_reg area_reg = {
    .addr = (__u64)(unsigned long)area_ptr,
    .len = area_size,
    .flags = 0,
  };

  struct io_uring_region_desc region_reg = {
    .user_addr = (__u64)(unsigned long)ring_ptr,
    .size = ring_size,
    .flags = IORING_MEM_REGION_TYPE_USER,
  };

  struct io_uring_zcrx_ifq_reg reg = {
    .if_idx = if_nametoindex("eth0"),
    /* this is the HW queue with desired flow steered into it */
    .if_rxq = 1,
    .rq_entries = rq_entries,
    .area_ptr = (__u64)(unsigned long)&area_reg,
    .region_ptr = (__u64)(unsigned long)&region_reg,
  };

```
```

  io_uring_register_ifq(ring, &reg);

```
### 鏄犲皠琛ュ厖鐜?

鍐呮牳鍦ㄦ敞鍐屾椂涓鸿ˉ鍏呯幆濉厖瀛楁锛屾敞鍐?``struct
```

  struct io_uring_zcrx_rq refill_ring;

  refill_ring.khead = (unsigned *)((char *)ring_ptr + reg.offsets.head);
  refill_ring.khead = (unsigned *)((char *)ring_ptr + reg.offsets.tail);
  refill_ring.rqes =
    (struct io_uring_zcrx_rqe *)((char *)ring_ptr + reg.offsets.rqes);
  refill_ring.rq_tail = 0;
  refill_ring.ring_ptr = ring_ptr;

```
### 鎺ユ敹鏁版嵁


```

  struct io_uring_sqe *sqe;

  sqe = io_uring_get_sqe(ring);
  io_uring_prep_rw(IORING_OP_RECV_ZC, sqe, fd, NULL, 0, 0);
  sqe->ioprio |= IORING_RECV_MULTISHOT;

```
```

  io_uring_submit_and_wait(ring, 1);

```
```

  struct io_uring_cqe *cqe;
  unsigned int count = 0;
  unsigned int head;

  io_uring_for_each_cqe(ring, head, cqe) {
    struct io_uring_zcrx_cqe *rcqe = (struct io_uring_zcrx_cqe *)(cqe + 1);

    unsigned long mask = (1ULL << IORING_ZCRX_AREA_SHIFT) - 1;
    unsigned char *data = area_ptr + (rcqe->off & mask);
    /* do something with the data */

    count++;
  }
  io_uring_cq_advance(ring, count);

```
### 鍥炴敹缂撳啿鍖?

```

  struct io_uring_zcrx_rqe *rqe;
  unsigned mask = refill_ring.ring_entries - 1;
  rqe = &refill_ring.rqes[refill_ring.rq_tail & mask];

  unsigned long area_offset = rcqe->off & ~IORING_ZCRX_AREA_MASK;
  rqe->off = area_offset | area_reg.rq_area_token;
  rqe->len = cqe->res;
  IO_URING_WRITE_ONCE(*refill_ring.ktail, ++refill_ring.rq_tail);

```
### 鍖哄煙鍒嗗潡


zcrx 灏嗗唴瀛樺尯鍩熸媶鍒嗕负鍥哄畾闀垮害銆佺墿鐞嗕笂杩炵画鐨勫潡銆傝繖闄愬埗浜嗗崟涓?io_uring CQE 涓繑鍥炵殑鏈€澶х紦鍐插尯澶у皬銆傜敤鎴峰彲浠ラ€氳繃鍦ㄦ敞鍐屾湡闂村皢 `struct io_uring_zcrx_ifq_reg` 鐨?`rx_buf_len` 瀛楁璁剧疆涓烘湡鏈涚殑闀垮害锛屽悜鍐呮牳鎻愪緵浣跨敤鏇村ぇ鍧楃殑鎻愮ず銆傚鏋滆瀛楁琚缃负闆讹紝鍐呮牳榛樿浣跨敤绯荤粺椤靛ぇ灏忋€?
瑕佷娇鐢ㄦ洿澶х殑灏哄锛屽唴瀛樺尯鍩熷繀椤荤敱鐗╃悊涓婅繛缁殑銆佸ぇ灏忔槸 `rx_buf_len` 鏁存暟鍊嶇殑鑼冨洿浣滀负鍚庡銆傚畠杩橀渶瑕佸唴鏍镐笌纭欢鏀寔銆傚鏋滄敞鍐屽け璐ワ紝鐢ㄦ埛涓€鑸簲閫氳繃灏嗗叾 `rx_buf_len` 璁剧疆涓洪浂鏉ュ洖閫€鍒伴粯璁ゅ€笺€?
鏇村ぇ鐨勫潡涓嶄細瀵?CQE 涓繑鍥炵殑缂撳啿鍖哄ぇ灏忔彁渚涗换浣曢澶栦繚璇侊紝骞朵笖瀹冧滑鍙兘鍥犳祦閲忔ā寮忋€佺‖浠跺嵏杞界瓑璁稿鍥犵礌鑰屽彉鍖栥€傞櫎浜?zcrx 娉ㄥ唽涔嬪锛屽畠涓嶉渶瑕佸簲鐢ㄧ▼搴忓仛浠讳綍鏇存敼銆?
## 娴嬭瘯


鍙傝 `tools/testing/selftests/drivers/net/hw/iou-zcrx.c`
