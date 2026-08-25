
## FUSE-over-io-uring 设计文档


本文档涵fuse 通过 io-uring 进行内核/用户态通信是如何配置和运作的基本细节。关FUSE 的通用细节，请参见 fuse.rst
本文档还涵盖当前接口，该接口仍在开发中并可能发生变化
## 限制


截至目前，并非所有请求类型都通过 io-uring 支持，用户态在 io-uring 设置完成后还需要通过 /dev/fuse 处理请求。具体是通知（由守护进程侧发起）和中断
## Fuse io-uring 配置


Fuse 内核请求通过经典/dev/fuse 写接口排队——直io-uring 设置完成
为了建立 fuse-over-io-uring，fuse-server（用户态）需要向 /dev/fuse 连接文件描述符提SQE（opcode = IORING_OP_URING_CMD）。初始提交使用子命令 FUSE_URING_REQ_REGISTER，它只是注册在内核中可用的条目
一旦每个队列至少提交一个条目，内核就开始入队到 ring 队列注意，每CPU 核心都有自己fuse-io-uring 队列用户态处CQE/fuse 请求，并以子命令 FUSE_URING_REQ_COMMIT_AND_FETCH 提交结果——内核完成请求并再次将该条目标记为可用。如果有等待中的请求，该请求将立即再次提交给守护进程
初始 SQE
```

 |                                    |  FUSE 文件系统守护进程
 |                                    |
 |                                    |  >io_uring_submit()
 |                                    |   IORING_OP_URING_CMD /
 |                                    |   FUSE_URING_CMD_REGISTER
 |                                    |  [等待 cqe]
 |                                    |   >io_uring_wait_cqe() 鎴? |                                    |   >io_uring_submit_and_wait()
 |                                    |
 |  >fuse_uring_cmd()                 |
 |   >fuse_uring_register()           |

```
通过 CQE 发送请```

 |                                           |  FUSE 文件系统守护进程
 |                                           |  [等待 CQE]
 |  "rm /mnt/fuse/file"                      |
 |                                           |
 |  >sys_unlink()                            |
 |    >fuse_unlink()                         |
 |      [分配请求]                            |
 |      >fuse_send_one()                     |
 |        ...                                |
 |       >fuse_uring_queue_fuse_req          |
 |        [fg 队列上排队请求]               |
 |         >fuse_uring_add_req_to_ring_ent() |
 |         ...                               |
 |          >fuse_uring_copy_to_ring()       |
 |          >io_uring_cmd_done()             |
 |       >request_wait_answer()              |
 |         [req->waitq 上休眠]             |
 |                                           |  [接收并处CQE]
 |                                           |  [提交结果并获取下一个]
 |                                           |  >io_uring_submit()
 |                                           |   IORING_OP_URING_CMD/
 |                                           |   FUSE_URING_CMD_COMMIT_AND_FETCH
 |  >fuse_uring_cmd()                        |
 |   >fuse_uring_commit_fetch()              |
 |    >fuse_uring_commit()                   |
 |     >fuse_uring_copy_from_ring()          |
 |      [ 将结果复制到 fuse req]              |
 |     >fuse_uring_req_end()                 |
 |      >fuse_request_end()                  |
 |       [唤醒 req->waitq]                    |
 |    >fuse_uring_next_fuse_req              |
 |       [等待或处理下一个请求]                |
 |                                           |
 |       [req->waitq 被唤醒]                  |
 |    <fuse_unlink()                         |
 |  <sys_unlink()                            |




```
