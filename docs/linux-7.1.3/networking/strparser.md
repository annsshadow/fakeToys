
## 流解析器（strparser）


## 简介


流解析器（strparser）是一个用于解析运行在数据流之上的应用层协议消息的实用工具。流解析器与内核中的上层协作，为应用层消息提供内核支持。例如，内核连接多路复用器（KCM）使用流解析器借助 BPF 程序来解析消息。

strparser 在两种模式之一下工作：接收回调模式或通用模式。

在接收回调模式下，strparser 从 TCP 套接字的 data_ready 回调中被调用。消息在套接字上收到时即被解析并投递。

在通用模式下，一系列 skb 从外部来源喂给 strparser。消息在该序列被处理时被解析并投递。此模式允许 strparser 应用于任意的数据流。

## 接口


该 API 包括一个上下文结构体、一组回调、实用函数，以及用于接收回调模式的 data_ready 函数。这些回调包括一个 parse_msg 函数（在解析时被调用，例如 KCM 中的 BPF 解析）和一个 rcv_msg 函数（在一条完整消息完成时调用）。

## 函数


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
## 回调


共有七个回调：

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
## 统计


每个流解析器实例都维护着各种计数器。这些计数器位于 strp_stats 结构体中。strp_aggr_stats 是一个便于为多个流解析器实例累计统计信息的结构体。save_strp_stats 和 aggregate_strp_stats 是用于保存和聚合统计信息的辅助函数。

## 消息组装限制


流解析器提供了限制消息组装所消耗资源的机制。

当开始组装一条新消息时会设置一个定时器。在接收回调模式下，消息超时取自关联 TCP 套接字的 rcvtime。在通用模式下，超时作为 strp_process 的参数传入。如果定时器在组装完成之前触发，则流解析器被中止，并且在接收回调模式下会在 TCP 套接字上设置 ETIMEDOUT 错误。

在接收回调模式下，消息长度限制为关联 TCP 套接字的接收缓冲区大小。如果 parse_msg 返回的长度大于套接字缓冲区大小，则流解析器被中止，并在 TCP 套接字上设置 EMSGSIZE 错误。注意，这使得带有流解析器的套接字的最大接收 skbuff 大小为 TCP 套接字的 2*sk_rcvbuf。

在通用模式下，消息长度限制作为 strp_process 的参数传入。

## 作者


Tom Herbert (tom@quantonium.net)
