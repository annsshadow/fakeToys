## 鎺у埗鍙伴┍鍔。

Linux 内核有两类通用的控制台驱动。第一类由内核在启动过程中分配给所有虚拟控制台。这类驱动被称为“系统驱动（system driver）”，并且只允许存在一个系统驱动。系统驱动是常驻的，它永远不会被卸载，尽管它可能变为非活动状态

第二类必须被显式地加载和卸载。本文将称其为“模块化驱动（modular driver）”。任意时刻都可以有多个模块化驱动共存，每个驱动都与其他驱动（包括系统驱动）共享控制台。不过，模块化驱动无法接管当前正被另一个模块化驱动占用的控制台。（例外：调用了 do_take_over_console() 的驱动，无论占用控制台的是哪类驱动，都能成功完成接管。）它们只能接管被系统驱动占用的控制台。同理，如果模块化驱动被控制台释放，系统驱动就会接管回来

```

	 do_take_over_console() - load and bind driver to console layer
	 give_up_console() - unload driver; it will only work if driver
			     is fully unbound

```
```

	 do_register_con_driver()
	 do_unregister_con_driver()

```
如果启用sysfs，可以检/sys/class/vtconsole 的内容。它展示了系统当前注册的控制台后端，其命名为 vtcon<n>，其<n> 是从 0 15 的整数
```

       ls /sys/class/vtconsole
       .  ..  vtcon0  vtcon1

```
```

     ls /sys/class/vtconsole/vtcon0
     .  ..  bind  name  uevent

```
这些文件代表什么？

     1. bind - 这是一个可写文件。读取时它显示驱动的状态；写入时它用于将该驱动绑定或解除绑定到虚拟控制台。可能的值为

	0
   - 表示驱动未绑定，如果向其 echo，则命令驱动解除绑定

        1
   - 表示驱动已绑定，如果向其 echo，则命令驱动绑定

```

	  cat /sys/class/vtconsole/vtcon0/name
	  (S) VGA+

	      '(S)' stands for a (S)ystem driver, i.e., it cannot be directly
	      commanded to bind or unbind

	      'VGA+' is the name of the driver

	  cat /sys/class/vtconsole/vtcon1/name
	  (M) frame buffer device

	      In this case, '(M)' stands for a (M)odular driver, one that can be
	      directly commanded to bind or unbind.

     3. uevent - ignore this file

```
解除绑定时，模块化驱动首先被分离，然后系统驱动接管该驱动腾出的控制台。另一方面，绑定时，会把该驱动绑定到当前由系统驱动占用的控制台

注意 1
```

    Device Drivers ->
	Character devices ->
		Support for binding and unbinding console drivers

```
注意 2：如果任意虚拟控制台处于 KD_GRAPHICS 模式，那么绑定或解除绑定都不会成功。一个会把控制台设为 KD_GRAPHICS 的例子是 X

这个功能有多大用处？它对控制台驱动开发者非常有用。通过把驱动从控制台层解除绑定，可以卸载驱动、做出修改、重新编译、重新加载并重新绑定驱动，而无需重启内核。对于想要在帧缓冲控制台VGA 控制台之间（或反过来）切换的普通用户，这个功能也使之成为可能。（注意 注意 注意：更多细节请阅读 Documentation/fb 下的 fbcon.txt。）

## 给开发者的说明

```

     do_register_con_driver()
     do_bind_con_driver() - private function

```
give_up_console() do_unregister_con_driver() 的封装，并且只有当驱动被完全解除绑定时这个调用才会成功。con_is_bound() 会检查驱动是否已绑定

## 控制台驱动编写者指

为了让绑定和解除绑定到控制台能够正常工作，控制台驱动必须遵循以下准则

1. 除系统驱动外，所有驱动都必须调用 do_register_con_driver() do_take_over_console()。do_register_con_driver() 只是把驱动加入控制台的内部列表，它不会接管控制台。而顾名思义，do_take_over_console() 还会接管（或绑定到）控制台

2. con->con_init() 期间分配的所有资源必须在 con->con_deinit() 中释放

3. con->con_startup() 中分配的所有资源，必须在之前已绑定的驱动被解除绑定时释放。控制台层没有与 con->con_startup() 相对应的调用，因此何时可以合法释放这些资源由驱动自己判断。在 con->con_deinit() 中调con_is_bound() 会有所帮助。如果该调用返回 false()，那么释放这些资源是安全的。必须确保这种平衡，因为当一个重新绑定该驱动到控制台的请求到来时，con->con_startup() 可能会再次被调用

4. 在驱动退出时，确保该驱动已被完全解除绑定。如果条件满足，那么驱动必须调用 do_unregister_con_driver() give_up_console()

5. do_unregister_con_driver() 也可以在驱动无法再为控制台请求提供服务的情况下被调用。使用帧缓冲控制台时可能发生这种情况——它突然失去了所有的驱动

当前的这批控制台驱动应当仍能正常工作，但绑定和解除绑定它们可能会有问题。只需做最小的修复，就能让这些驱动正常工作

Antonino Daplas <adaplas@pol.net>
