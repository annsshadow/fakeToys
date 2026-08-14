
### 数字电视通用函数


#### DVB 设备


这些函数负责处理 DVB 设备节点。


#### 数字电视环形缓冲区


这些例程实现了用于处理数字电视数据并在其与用户空间之间复制的环形缓冲区。


  1) 出于性能考虑，读写例程不检查缓冲区大小及/或空闲/可用字节数。
     这必须在调用这些例程之前完成。例如：

   .. code-block:: c

        /** write @buflen: bytes **/
        free = dvb_ringbuffer_free(rbuf);
        if (free >= buflen)
                count = dvb_ringbuffer_write(rbuf, buffer, buflen);
        else
                /** do something **/

        /** read min. 1000, max. @bufsize: bytes **/
        avail = dvb_ringbuffer_avail(rbuf);
        if (avail >= 1000)
                count = dvb_ringbuffer_read(rbuf, buffer, min(avail, bufsize));
        else
                /** do something **/

  2) 如果恰好有一个读取者和一个写入者，则无需对读或写操作加锁。
     两个或更多读取者之间必须互斥加锁。
     刷新缓冲区算作一次读操作。
     重置缓冲区算作一次读和写操作。
     两个或更多写入者之间必须互斥加锁。


#### 数字电视 VB2 处理器
