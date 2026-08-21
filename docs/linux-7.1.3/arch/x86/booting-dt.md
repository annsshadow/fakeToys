### DeviceTree 启动（DeviceTree Booting


  内核code32_start 处有一个单一32 位入口点
  解压器（实模式入口点在切换到保护模式后会进入同一32 
  入口点）。该入口点支持一种调用约定，记录
  Documentation/arch/x86/boot.rst 中
  指向 device-tree 块的物理指针通过 setup_data 传递，
  这要求启动协议版本至少为 2.09
  类型字段定义

  #define SETUP_DTB                      2

  device-tree 用作“启动页（boot page）”的扩展。因此它
  不会解析 / 考虑启动页已涵盖的数据，包括内存大小、保留范围
  命令行参数或 initrd 地址。它仅保存无法通过其他方式获取
  信息，例如中断路由或 I2C 总线后挂接的设备列表
