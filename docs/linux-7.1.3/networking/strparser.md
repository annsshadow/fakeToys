
## 娴佽В鏋愬櫒锛坰trparser锛?

## 绠€浠?

娴佽В鏋愬櫒锛坰trparser锛夋槸涓€涓敤浜庤В鏋愯繍琛屽湪鏁版嵁娴佷箣涓婄殑搴旂敤灞傚崗璁秷鎭殑瀹炵敤宸ュ叿銆傛祦瑙ｆ瀽鍣ㄤ笌鍐呮牳涓殑涓婂眰鍗忎綔锛屼负搴旂敤灞傛秷鎭彁渚涘唴鏍告敮鎸併€備緥濡傦紝鍐呮牳杩炴帴澶氳矾澶嶇敤鍣紙KCM锛変娇鐢ㄦ祦瑙ｆ瀽鍣ㄥ€熷姪 BPF 绋嬪簭鏉ヨВ鏋愭秷鎭€?
strparser 鍦ㄤ袱绉嶆ā寮忎箣涓€涓嬪伐浣滐細鎺ユ敹鍥炶皟妯″紡鎴栭€氱敤妯″紡銆?
鍦ㄦ帴鏀跺洖璋冩ā寮忎笅锛宻trparser 浠?TCP 濂楁帴瀛楃殑 data_ready 鍥炶皟涓璋冪敤銆傛秷鎭湪濂楁帴瀛椾笂鏀跺埌鏃跺嵆琚В鏋愬苟鎶曢€掋€?
鍦ㄩ€氱敤妯″紡涓嬶紝涓€绯诲垪 skb 浠庡閮ㄦ潵婧愬杺缁?strparser銆傛秷鎭湪璇ュ簭鍒楄澶勭悊鏃惰瑙ｆ瀽骞舵姇閫掋€傛妯″紡鍏佽 strparser 搴旂敤浜庝换鎰忕殑鏁版嵁娴併€?
## 鎺ュ彛


璇?API 鍖呮嫭涓€涓笂涓嬫枃缁撴瀯浣撱€佷竴缁勫洖璋冦€佸疄鐢ㄥ嚱鏁帮紝浠ュ強鐢ㄤ簬鎺ユ敹鍥炶皟妯″紡鐨?data_ready 鍑芥暟銆傝繖浜涘洖璋冨寘鎷竴涓?parse_msg 鍑芥暟锛堝湪瑙ｆ瀽鏃惰璋冪敤锛屼緥濡?KCM 涓殑 BPF 瑙ｆ瀽锛夊拰涓€涓?rcv_msg 鍑芥暟锛堝湪涓€鏉″畬鏁存秷鎭畬鎴愭椂璋冪敤锛夈€?
## 鍑芥暟


```
	strp_init(struct strparser *strp, struct sock *sk,
		const struct strp_callbacks *cb)

     Called to initialize a stream parser. strp is a struct of type
     strparser that is allocated by the upper layer. sk is the TCP
     socket associated with the stream parser for use with receive
     callback mode; in general mode this is set to NULL. Callbacks
     are called by the stream parser (the callbacks are listed below).

     ::

	void strp_pause(struct strparser *strp)

     Temporarily pause a stream parser. Message parsing is suspended
     and no new messages are delivered to the upper layer.

     ::

	void strp_unpause(struct strparser *strp)

     Unpause a paused stream parser.

     ::

	void strp_stop(struct strparser *strp);

     strp_stop is called to completely stop stream parser operations.
     This is called internally when the stream parser encounters an
     error, and it is called from the upper layer to stop parsing
     operations.

     ::

	void strp_done(struct strparser *strp);

     strp_done is called to release any resources held by the stream
     parser instance. This must be called after the stream processor
     has been stopped.

     ::

	int strp_process(struct strparser *strp, struct sk_buff *orig_skb,
			 unsigned int orig_offset, size_t orig_len,
			 size_t max_msg_size, long timeo)

    strp_process is called in general mode for a stream parser to
    parse an sk_buff. The number of bytes processed or a negative
    error number is returned. Note that strp_process does not
    consume the sk_buff. max_msg_size is maximum size the stream
    parser will parse. timeo is timeout for completing a message.

    ::

	void strp_data_ready(struct strparser *strp);

    The upper layer calls strp_tcp_data_ready when data is ready on
    the lower socket for strparser to process. This should be called
    from a data_ready callback that is set on the socket. Note that
    maximum messages size is the limit of the receive socket
    buffer and message timeout is the receive timeout for the socket.

    ::

	void strp_check_rcv(struct strparser *strp);

    strp_check_rcv is called to check for new messages on the socket.
    This is normally called at initialization of a stream parser
    instance or after strp_unpause.

```
## 鍥炶皟


鍏辨湁涓冧釜鍥炶皟锛?
```
	int (*parse_msg)(struct strparser *strp, struct sk_buff *skb);

    parse_msg is called to determine the length of the next message
    in the stream. The upper layer must implement this function. It
    should parse the sk_buff as containing the headers for the
    next application layer message in the stream.

    The skb->cb in the input skb is a struct strp_msg. Only
    the offset field is relevant in parse_msg and gives the offset
    where the message starts in the skb.

    The return values of this function are:

    =========    ===========================================================
    >0           indicates length of successfully parsed message
    0            indicates more data must be received to parse the message
    -ESTRPIPE    current message should not be processed by the
		 kernel, return control of the socket to userspace which
		 can proceed to read the messages itself
    other < 0    Error in parsing, give control back to userspace
		 assuming that synchronization is lost and the stream
		 is unrecoverable (application expected to close TCP socket)
    =========    ===========================================================

    In the case that an error is returned (return value is less than
    zero) and the parser is in receive callback mode, then it will set
    the error on TCP socket and wake it up. If parse_msg returned
    -ESTRPIPE and the stream parser had previously read some bytes for
    the current message, then the error set on the attached socket is
    ENODATA since the stream is unrecoverable in that case.

    ::

	void (*lock)(struct strparser *strp)

    The lock callback is called to lock the strp structure when
    the strparser is performing an asynchronous operation (such as
    processing a timeout). In receive callback mode the default
    function is to lock_sock for the associated socket. In general
    mode the callback must be set appropriately.

    ::

	void (*unlock)(struct strparser *strp)

    The unlock callback is called to release the lock obtained
    by the lock callback. In receive callback mode the default
    function is release_sock for the associated socket. In general
    mode the callback must be set appropriately.

    ::

	void (*rcv_msg)(struct strparser *strp, struct sk_buff *skb);

    rcv_msg is called when a full message has been received and
    is queued. The callee must consume the sk_buff; it can
    call strp_pause to prevent any further messages from being
    received in rcv_msg (see strp_pause above). This callback
    must be set.

    The skb->cb in the input skb is a struct strp_msg. This
    struct contains two fields: offset and full_len. Offset is
    where the message starts in the skb, and full_len is the
    the length of the message. skb->len - offset may be greater
    than full_len since strparser does not trim the skb.

    ::

	int (*read_sock)(struct strparser *strp, read_descriptor_t *desc,
                     sk_read_actor_t recv_actor);

    The read_sock callback is used by strparser instead of
    sock->ops->read_sock, if provided.
    ::

	int (*read_sock_done)(struct strparser *strp, int err);

     read_sock_done is called when the stream parser is done reading
     the TCP socket in receive callback mode. The stream parser may
     read multiple messages in a loop and this function allows cleanup
     to occur when exiting the loop. If the callback is not set (NULL
     in strp_init) a default function is used.

     ::

	void (*abort_parser)(struct strparser *strp, int err);

     This function is called when stream parser encounters an error
     in parsing. The default function stops the stream parser and
     sets the error in the socket if the parser is in receive callback
     mode. The default function can be changed by setting the callback
     to non-NULL in strp_init.

```
## 缁熻


姣忎釜娴佽В鏋愬櫒瀹炰緥閮界淮鎶ょ潃鍚勭璁℃暟鍣ㄣ€傝繖浜涜鏁板櫒浣嶄簬 strp_stats 缁撴瀯浣撲腑銆俿trp_aggr_stats 鏄竴涓究浜庝负澶氫釜娴佽В鏋愬櫒瀹炰緥绱缁熻淇℃伅鐨勭粨鏋勪綋銆俿ave_strp_stats 鍜?aggregate_strp_stats 鏄敤浜庝繚瀛樺拰鑱氬悎缁熻淇℃伅鐨勮緟鍔╁嚱鏁般€?
## 娑堟伅缁勮闄愬埗


娴佽В鏋愬櫒鎻愪緵浜嗛檺鍒舵秷鎭粍瑁呮墍娑堣€楄祫婧愮殑鏈哄埗銆?
褰撳紑濮嬬粍瑁呬竴鏉℃柊娑堟伅鏃朵細璁剧疆涓€涓畾鏃跺櫒銆傚湪鎺ユ敹鍥炶皟妯″紡涓嬶紝娑堟伅瓒呮椂鍙栬嚜鍏宠仈 TCP 濂楁帴瀛楃殑 rcvtime銆傚湪閫氱敤妯″紡涓嬶紝瓒呮椂浣滀负 strp_process 鐨勫弬鏁颁紶鍏ャ€傚鏋滃畾鏃跺櫒鍦ㄧ粍瑁呭畬鎴愪箣鍓嶈Е鍙戯紝鍒欐祦瑙ｆ瀽鍣ㄨ涓锛屽苟涓斿湪鎺ユ敹鍥炶皟妯″紡涓嬩細鍦?TCP 濂楁帴瀛椾笂璁剧疆 ETIMEDOUT 閿欒銆?
鍦ㄦ帴鏀跺洖璋冩ā寮忎笅锛屾秷鎭暱搴﹂檺鍒朵负鍏宠仈 TCP 濂楁帴瀛楃殑鎺ユ敹缂撳啿鍖哄ぇ灏忋€傚鏋?parse_msg 杩斿洖鐨勯暱搴﹀ぇ浜庡鎺ュ瓧缂撳啿鍖哄ぇ灏忥紝鍒欐祦瑙ｆ瀽鍣ㄨ涓锛屽苟鍦?TCP 濂楁帴瀛椾笂璁剧疆 EMSGSIZE 閿欒銆傛敞鎰忥紝杩欎娇寰楀甫鏈夋祦瑙ｆ瀽鍣ㄧ殑濂楁帴瀛楃殑鏈€澶ф帴鏀?skbuff 澶у皬涓?TCP 濂楁帴瀛楃殑 2*sk_rcvbuf銆?
鍦ㄩ€氱敤妯″紡涓嬶紝娑堟伅闀垮害闄愬埗浣滀负 strp_process 鐨勫弬鏁颁紶鍏ャ€?
## 浣滆€?

Tom Herbert (tom@quantonium.net)
