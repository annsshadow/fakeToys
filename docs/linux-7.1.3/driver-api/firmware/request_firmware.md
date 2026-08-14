## request_firmware API


你通常会先加载固件，然后以某种方式将其加载到你的设备中。
```

	 if(request_firmware(&fw_entry, $FIRMWARE, device) == 0)
                copy_fw_to_device(fw_entry->data, fw_entry->size);
	 release_firmware(fw_entry);

```
## 同步固件请求


同步固件请求会一直等待，直到找到固件或返回错误。

### request_firmware

   :functions: request_firmware

### firmware_request_nowarn

   :functions: firmware_request_nowarn

### firmware_request_platform

   :functions: firmware_request_platform

### request_firmware_direct

   :functions: request_firmware_direct

### request_firmware_into_buf

   :functions: request_firmware_into_buf

## 异步固件请求


异步固件请求允许驱动代码不必等待固件或错误返回。提供了函数回调，以便在找到固件或错误时通过回调通知驱动。request_firmware_nowait() 不能在原子上下文中调用。

### request_firmware_nowait

   :functions: request_firmware_nowait

## 重启时的特殊优化


某些设备具有一项优化，使固件在系统重启期间得以保留。使用这类优化时，驱动作者必须确保固件在从挂起恢复时仍然可用，这可以通过 firmware_request_cache() 来代替请求加载固件实现。

### firmware_request_cache()

   :functions: firmware_request_cache

## 请求固件 API 预期的驱动使用方式


一旦 API 调用返回，你就处理固件，然后释放固件。例如，如果你使用了 request_firmware() 并且它返回了，驱动就可以在 fw_entry->{data,size} 中访问固件镜像。如果出了问题，request_firmware() 返回非零值，并且 fw_entry 被设为 NULL。一旦你的驱动处理完固件，它就可以调用 release_firmware(fw_entry) 来释放固件镜像以及任何相关资源。
