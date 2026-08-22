## efivarfs - 一(U)EFI 变量文件系统


efivarfs 文件系统被创建出来，以解决使sysfs 中的条目来维EFI 变量的不足。旧sysfs EFI 变量代码只支持最1024 字节的变量。该限制EFI 规范0.99 版本中存在，但在任何正式发布版之前就被移除了。由于变量现在可能大于单个页面，sysfs 并不是处理此问题的最佳接口

变量可以通过 efivarfs 文件系统创建、删除和修改

```
	mount -t efivarfs none /sys/firmware/efi/efivars
```
由于存在大量固件缺陷，其中移除非标准UEFI 变量会导致系统固件无法完POST（加电自检），efivarfs 中将那些非广为人知的标准化变量创建为不可变文件。这并不阻止删除—chattr -i" 仍然有效——但可以防止此类故障被意外触发

      当显/sys/firmware/efi/efivars 中某UEFI 变量的内容时（例如使"hexdump"），请注意输出的4 个字节代UEFI 变量属性，采用小端格式

      实际上，每个 efivar 的输出由以下内容组成

          +-----------------------------------+
          |4_bytes_of_attributes + efivar_data|
          +-----------------------------------+

**另请参阅*

- Documentation/admin-guide/acpi/ssdt-overlays.rst
- Documentation/ABI/removed/sysfs-firmware-efi-vars
