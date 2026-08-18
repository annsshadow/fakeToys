
## 鍦ㄤ互闈?D0 鐘舵€佹帰娴嬭澶?

## 绠€浠?

鍦ㄦ煇浜涙儏鍐典笅锛屽鏋滃紑鍚繖浜涜澶囦細甯︽潵涓嶈壇鍓綔鐢紙瓒呭嚭浜嗕粎浠呭紑鍚璁惧鏈韩锛夛紝鍙兘鏇村€惧悜浜庡湪鏁翠釜绯荤粺鍚姩鏈熼棿璁╂煇浜涜澶囦繚鎸佹柇鐢点€?
## 宸ヤ綔鍘熺悊


_DSC锛圖evice State for Configuration锛岀敤浜庨厤缃殑璁惧鐘舵€侊級瀵硅薄浼氭眰鍊间负涓€涓暣鏁帮紝鍙敤浜庡憡璇?Linux 鍦ㄦ帰娴嬶紙probe锛夋湡闂磋澶囧厑璁哥殑鏈€楂?D 鐘舵€併€傚鏋滄€荤嚎椹卞姩閫氬父浼氬皢璁惧缃簬 D0 鐘舵€佽繘琛屾帰娴嬶紝閭ｄ箞瀵?_DSC 鐨勬敮鎸侀渶瑕佸唴鏍告€荤嚎绫诲瀷鐨勬敮鎸併€?
浣跨敤 _DSC 鐨勭己鐐规槸锛岀敱浜庤澶囨湭琚笂鐢碉紝鍗充娇璁惧鏈夐棶棰橈紝椹卞姩寰堝彲鑳戒篃鑳芥甯告帰娴嬶紝浣嗙涓€涓敤鎴蜂細鍙戠幇璁惧涓嶅伐浣滐紝鑰屼笉鏄湪鎺㈡祴鏃跺け璐ャ€傚洜姝ゅ簲璋ㄦ厧浣跨敤姝ょ壒鎬с€?
### I虏C


濡傛灉涓€涓?I虏C 椹卞姩閫氳繃鍦?struct i2c_driver.flags 瀛楁涓缃?I2C_DRV_ACPI_WAIVE_D0_PROBE 鏍囧織鏉ヨ〃鏄庡叾瀵规鐨勬敮鎸侊紝骞朵笖 _DSC 瀵硅薄姹傚€肩殑鏁存暟楂樹簬璁惧鐨?D 鐘舵€侊紝鍒欒澶囧皢涓嶄細鍦ㄦ帰娴嬫椂琚笂鐢碉紙缃簬 D0 鐘舵€侊級銆?
### D 鐘舵€?

D 鐘舵€佷互鍙婂洜姝?_DSC 鐨勫厑璁稿€煎涓嬫墍绀恒€傚叧浜庤澶囩數婧愮姸鎬佺殑鏇村淇℃伅璇峰弬闃?[^1^]銆?

	Number	State	Description
	0	D0	璁惧瀹屽叏涓婄數
	1	D1
	2	D2
	3	D3hot
	4	D3cold	Off锛堝叧闂級

## 鍙傝€?

[^1^] https://uefi.org/specifications/ACPI/6.4/02_Definition_of_Terms/Definition_of_Terms.html#device-power-state-definitions

## 绀轰緥


涓€涓弿杩颁娇鐢?_DSC 瀵硅薄鍛婄煡鎿嶄綔绯荤粺璇ヨ澶囧湪鎺㈡祴鏈熼棿搴斾繚鎸佹柇鐢电殑 ACPI 璁惧鐨?ASL 绀轰緥濡備笅銆備粠绀轰緥瑙掑害涓嶇浉鍏崇殑鏌愪簺瀵硅薄宸茶鐪佺暐銆?

	Device (CAM0)
	{
		Name (_HID, "SONY319A")
		Name (_UID, Zero)
		Name (_CRS, ResourceTemplate ()
		{
			I2cSerialBus(0x0020, ControllerInitiated, 0x00061A80,
				     AddressingMode7Bit, "\\_SB.PCI0.I2C0",
				     0x00, ResourceConsumer)
		})
		Method (_DSC, 0, NotSerialized)
		{
			Return (0x4)
		}
	}
