
## FUSE-over-io-uring 璁捐鏂囨。


鏈枃妗ｆ兜鐩?fuse 閫氳繃 io-uring 杩涜鍐呮牳/鐢ㄦ埛鎬侀€氫俊鏄浣曢厤缃拰杩愪綔鐨勫熀鏈粏鑺傘€傚叧浜?FUSE 鐨勯€氱敤缁嗚妭锛岃鍙傝 fuse.rst銆?
鏈枃妗ｈ繕娑电洊褰撳墠鎺ュ彛锛岃鎺ュ彛浠嶅湪寮€鍙戜腑骞跺彲鑳藉彂鐢熷彉鍖栥€?
## 闄愬埗


鎴嚦鐩墠锛屽苟闈炴墍鏈夎姹傜被鍨嬮兘閫氳繃 io-uring 鏀寔锛岀敤鎴锋€佸湪 io-uring 璁剧疆瀹屾垚鍚庤繕闇€瑕侀€氳繃 /dev/fuse 澶勭悊璇锋眰銆傚叿浣撴槸閫氱煡锛堢敱瀹堟姢杩涚▼渚у彂璧凤級鍜屼腑鏂€?
## Fuse io-uring 閰嶇疆


Fuse 鍐呮牳璇锋眰閫氳繃缁忓吀鐨?/dev/fuse 璇?鍐欐帴鍙ｆ帓闃熲€斺€旂洿鍒?io-uring 璁剧疆瀹屾垚銆?
涓轰簡寤虹珛 fuse-over-io-uring锛宖use-server锛堢敤鎴锋€侊級闇€瑕佸悜 /dev/fuse 杩炴帴鏂囦欢鎻忚堪绗︽彁浜?SQE锛坥pcode = IORING_OP_URING_CMD锛夈€傚垵濮嬫彁浜や娇鐢ㄥ瓙鍛戒护 FUSE_URING_REQ_REGISTER锛屽畠鍙槸娉ㄥ唽鍦ㄥ唴鏍镐腑鍙敤鐨勬潯鐩€?
涓€鏃︽瘡涓槦鍒楄嚦灏戞彁浜や竴涓潯鐩紝鍐呮牳灏卞紑濮嬪叆闃熷埌 ring 闃熷垪銆?娉ㄦ剰锛屾瘡涓?CPU 鏍稿績閮芥湁鑷繁鐨?fuse-io-uring 闃熷垪銆?鐢ㄦ埛鎬佸鐞?CQE/fuse 璇锋眰锛屽苟浠ュ瓙鍛戒护 FUSE_URING_REQ_COMMIT_AND_FETCH 鎻愪氦缁撴灉鈥斺€斿唴鏍稿畬鎴愯姹傚苟鍐嶆灏嗚鏉＄洰鏍囪涓哄彲鐢ㄣ€傚鏋滄湁绛夊緟涓殑璇锋眰锛岃璇锋眰灏嗙珛鍗冲啀娆℃彁浜ょ粰瀹堟姢杩涚▼銆?
鍒濆 SQE
```

 |                                    |  FUSE 鏂囦欢绯荤粺瀹堟姢杩涚▼
 |                                    |
 |                                    |  >io_uring_submit()
 |                                    |   IORING_OP_URING_CMD /
 |                                    |   FUSE_URING_CMD_REGISTER
 |                                    |  [绛夊緟 cqe]
 |                                    |   >io_uring_wait_cqe() 鎴? |                                    |   >io_uring_submit_and_wait()
 |                                    |
 |  >fuse_uring_cmd()                 |
 |   >fuse_uring_register()           |

```
閫氳繃 CQE 鍙戦€佽姹?```

 |                                           |  FUSE 鏂囦欢绯荤粺瀹堟姢杩涚▼
 |                                           |  [绛夊緟 CQE]
 |  "rm /mnt/fuse/file"                      |
 |                                           |
 |  >sys_unlink()                            |
 |    >fuse_unlink()                         |
 |      [鍒嗛厤璇锋眰]                            |
 |      >fuse_send_one()                     |
 |        ...                                |
 |       >fuse_uring_queue_fuse_req          |
 |        [鍦?fg 闃熷垪涓婃帓闃熻姹俔               |
 |         >fuse_uring_add_req_to_ring_ent() |
 |         ...                               |
 |          >fuse_uring_copy_to_ring()       |
 |          >io_uring_cmd_done()             |
 |       >request_wait_answer()              |
 |         [鍦?req->waitq 涓婁紤鐪燷             |
 |                                           |  [鎺ユ敹骞跺鐞?CQE]
 |                                           |  [鎻愪氦缁撴灉骞惰幏鍙栦笅涓€涓猐
 |                                           |  >io_uring_submit()
 |                                           |   IORING_OP_URING_CMD/
 |                                           |   FUSE_URING_CMD_COMMIT_AND_FETCH
 |  >fuse_uring_cmd()                        |
 |   >fuse_uring_commit_fetch()              |
 |    >fuse_uring_commit()                   |
 |     >fuse_uring_copy_from_ring()          |
 |      [ 灏嗙粨鏋滃鍒跺埌 fuse req]              |
 |     >fuse_uring_req_end()                 |
 |      >fuse_request_end()                  |
 |       [鍞ら啋 req->waitq]                    |
 |    >fuse_uring_next_fuse_req              |
 |       [绛夊緟鎴栧鐞嗕笅涓€涓姹俔                |
 |                                           |
 |       [req->waitq 琚敜閱抅                  |
 |    <fuse_unlink()                         |
 |  <sys_unlink()                            |




```
