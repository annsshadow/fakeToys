## 授权（或不授权）你的 USB 设备连接到系

Copyright (C) 2007 Inaky Perez-Gonzalez <inaky@linux.intel.com> Intel Corporation

此特性允许你控制一USB 设备是否可以在系统中使用（或不被使用）。此特性将允许实现由用户空间完全控制的 USB 设备锁定（lock-down）
截至目前，当连接一USB 设备时，它会被配置，其接口会立即对用户可用。经过此修改只有root 授权该设备进行配置后，才能使用它
## 用法


```

	$ echo 1 > /sys/bus/usb/devices/DEVICE/authorized

```
```

	$ echo 0 > /sys/bus/usb/devices/DEVICE/authorized

```
默认将连接到 hostX 的新设备设为默认不授权（即：
```

	$ echo 0 > /sys/bus/usb/devices/usbX/authorized_default

```
```

	$ echo 1 > /sys/bus/usb/devices/usbX/authorized_default

```
默认情况下，所USB 设备都是被授权的。向 authorized_default 属性写"2" 会使内核
默认仅授权连接到内部 USB 端口的设备
### 系统锁定示例（简陋版

设想你想实现一种锁定，使得只有 XYZ 类型的设备可以连接（例如，这是一台带有可```

  启动
  rc.local ->

   for host in /sys/bus/usb/devices/usb*
   do
      echo 0 > $host/authorized_default
   done

```
```

  if device_is_my_type $DEV
  then
    echo 1 > $device_path/authorized
  done


```
现在，device_is_my_type() 才是锁定真正的精髓所在。仅仅检class、type protocol 是否
匹配某事物，是你所能做的最糟糕的安全验证（或者对于想突破它的人来说是最好的）。如果你需安全的方案，请使用加密与证书认证或类似手段。对于存储密钥这类简单场```

  function device_is_my_type()
  {
    echo 1 > authorized		# 临时授权                                # FIXME：确保无人能挂载    mount DEVICENODE /mntpoint
    sum=$(md5sum /mntpoint/.signature)
    if [ $sum = $(cat /etc/lockdown/keysum) ]
    then
         echo "We are good, connected"
         umount /mntpoint
         # 其他操作以便他人可以使用    else
         echo 0 > authorized
    fi
  }


```
当然，这很简陋，你会想要PKI 做真正的证书验证，这样就不依赖共享密钥等等，但思路就是这样任何能够接触到设备小工具套件的人都可以伪造描述符与设备信息。不要信任这些。不客气
### 接口授权


有一种类似的方法可以允许或拒绝特定的 USB 接口。这允许只屏蔽一USB 设备的子集
```

	$ echo 1 > /sys/bus/usb/devices/INTERFACE/authorized

```
```

	$ echo 0 > /sys/bus/usb/devices/INTERFACE/authorized

```
在特USB 总线上，新接口的默认值也可以被更改
```

	$ echo 1 > /sys/bus/usb/devices/usbX/interface_authorized_default

```
```

	$ echo 0 > /sys/bus/usb/devices/usbX/interface_authorized_default

```
默认情况interface_authorized_default 位为 1因此所有接口默认都会被授权
注意  如果一个被取消授权的接口被重新授权，则驱动探测必须通过INTERFACE 写入
  /sys/bus/usb/drivers_probe 手动触发
对于需要多个接口的驱动，应首先授权所有需要的接口。之后再进行驱动探测这样可以避免副作用