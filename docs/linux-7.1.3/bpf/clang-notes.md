
## Clang 瀹炵幇璇存槑


鏈枃妗ｆ彁渚涗簡鐗瑰畾浜?eBPF 鎸囦护闆嗙殑 Clang/LLVM 瀹炵幇鐨勬洿澶氳缁嗕俊鎭€?

## 鐗堟湰


Clang 瀹氫箟浜嗏€淐PU鈥濈増鏈紝鍏朵腑 CPU 鐗堟湰 3 瀵瑰簲浜庡綋鍓嶇殑 eBPF ISA銆?

Clang 鍙互浣跨敤鈥?0000鈥濋€夋嫨 eBPF ISA 鐗堟湰锛屼緥濡傞€夋嫨鐗堟湰 3銆?

## 绠楁湳鎸囦护


瀵逛簬 3 涔嬪墠鐨?CPU 鐗堟湰锛孋lang v7.0 鍙婃洿楂樼増鏈彲浠ュ惎鐢?`BPF_ALU` 鏀寔
`-Xclang -target-feature -Xclang +alu32`銆? 鍦?CPU 鐗堟湰 3 涓紝鑷姩鍖呭惈鏀寔銆?

## 璺宠浆鎸囦护


濡傛灉浣跨敤`-O0`锛孋lang灏嗙敓鎴恅BPF_CALL | BPF_X | BPF_JMP`锛?x8d锛?
鎸囦护锛孡inux 鍐呮牳楠岃瘉鍣ㄤ笉鏀寔璇ユ寚浠ゃ€?

## 鍘熷瓙鎿嶄綔


褰揱-mcpu=v3`涓烘椂锛孋lang鍙互榛樿鐢熸垚鍘熷瓙鎸囦护
宸插惎鐢ㄣ€傚鏋滆缃簡杈冧綆鐗堟湰鐨刞-mcpu`锛屽垯鍞竴鐨勫師瀛愭寚浠?
Clang 鍙互鐢熸垚鐨勬槸 `BPF_ADD` **娌℃湁** `BPF_FETCH`銆傚鏋滄偍闇€瑕佸惎鐢?
鍘熷瓙鍔熻兘锛屽悓鏃朵繚鎸佽緝浣庣殑 `-mcpu` 鐗堟湰锛屾偍鍙互浣跨敤
`-Xclang -target-feature -Xclang +alu32`銆?
