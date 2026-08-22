## CA_GET_SLOT_INFO


### 名称


CA_GET_SLOT_INFO

### 摘要



`int ioctl(fd, CA_GET_SLOT_INFO, struct ca_slot_info *info)`

### 参数


`fd`
  由先`open()` 调用返回的文件描述符

`info`
  指向结构`ca_slot_info` 的指针

### 说明


返回`ca_slot_info`.slot_num 标识CA 插槽的信息

### 杩斿洖鍊。


成功时返0，并填充 `ca_slot_info`

出错时返-1，并相应地设`errno` 变量


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 16

    - -  `ENODEV`
       - 该插槽不可用

通用错误码在 Generic Error Codes <gen-errors> 章节中描述
