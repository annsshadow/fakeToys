## dGPU 鍥轰欢鍒峰啓锛坒irmware flashing锛?

### IFWI

鍒峰啓 dGPU 闆嗘垚鐨勫浐浠堕暅鍍忥紙IFWI锛夊彈浣跨敤 PSP 鏉ュ崗璋冩洿鏂帮紙Navi3x 鎴栨洿鏂扮殑 GPU锛夌殑 GPU 鏀寔銆傚浜庡彈鏀寔鐨?GPU锛宍amdgpu` 浼氬鍑轰竴绯诲垪鍙敤浜庡埛鍐欒繃绋嬬殑 sysfs 鏂囦欢銆?
IFWI 鍒峰啓杩囩▼濡備笅锛?
1. 纭繚 IFWI 闀滃儚閫傜敤浜庣郴缁熶笂鐨?dGPU銆?2. 灏?IFWI 闀滃儚鈥滃啓鍏ワ紙Write锛夆€濆埌 sysfs 鏂囦欢 `psp_vbflash`銆傝繖浼氬皢 IFWI 鏆傚瓨锛坰tage锛夊埌鍐呭瓨涓€?3. 浠?`psp_vbflash` sysfs 鏂囦欢鈥滆鍙栵紙Read锛夆€濅互鍚姩鍒峰啓杩囩▼銆?4. 杞锛圥oll锛塦psp_vbflash_status` sysfs 鏂囦欢浠ョ‘瀹氬埛鍐欒繃绋嬩綍鏃跺畬鎴愩€?
### USB-C PD F/W

鍦ㄦ敮鎸佸埛鍐欐洿鏂板悗鐨?USB-C PD 鍥轰欢闀滃儚鐨?GPU 涓婏紝璇ヨ繃绋嬮€氳繃 `usbc_pd_fw` sysfs 鏂囦欢瀹屾垚銆?
- 璇诲彇璇ユ枃浠跺皢鎻愪緵褰撳墠鐨勫浐浠剁増鏈€?- 灏嗗瓨鍌ㄥ湪 `/lib/firmware/amdgpu` 涓殑鍥轰欢璐熻浇锛坒irmware payload锛夌殑鍚嶇О鍐欏叆璇?sysfs 鏂囦欢锛屽皢鍚姩鍒峰啓杩囩▼銆?
瀛樺偍鍦?`/lib/firmware/amdgpu` 涓殑鍥轰欢璐熻浇鍙互浠绘剰鍛藉悕锛屽彧瑕佸畠涓?`amdgpu` 鎵€浣跨敤鐨勫叾浠栫幇鏈変簩杩涘埗鏂囦欢涓嶅啿绐佸嵆鍙€?
### sysfs 鏂囦欢
