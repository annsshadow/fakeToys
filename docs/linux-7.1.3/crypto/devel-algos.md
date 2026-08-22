## 开发密码算

### 注册与注销变换


Crypto API 中有三种不同类型的注册函数。一种用于注册通用的密码学变换（cryptographic transformation），另外两种则专门用HASH 变换和压缩（COMPRESSion）变换。我们将在单独的章节讨论后两种，此处仅关注通用那一种
在讨论注册函数之前，必须先了解每个函数需要填充的数据结构 struct crypto_alg —该数据结构的描述见下文
通用注册函数可在 include/linux/crypto.h 中找到，其定义见下文前者注册单个变换，后者作用于一组变换描述符数组。后者在批量注册变换时很有用例如当一个驱动实现多个变换时
```
       int crypto_register_alg(struct crypto_alg *alg);
       int crypto_register_algs(struct crypto_alg *algs, int count);
```
这些函数的对应注销函数列举如下
```
       void crypto_unregister_alg(struct crypto_alg *alg);
       void crypto_unregister_algs(struct crypto_alg *algs, int count);
```
注册函数在成功时返回 0，失败时返回负的 errno 值。crypto_register_algs() 只有在成功注册了所有给定算法时才会成功；如果中途失败，则任何已做的更改都会被回滚
注销函数总是成功，因此它们没有返回值。不要尝试注销当前未注册的算法
### 单块对称密码 [CIPHER]


变换示例：aes、serpent 
本节描述所有变换实现中最简单的一种，即用于对称密码的 CIPHER 类型。CIPHER 类型用于每次恰好操作一个块、且块之间完全没有任何依赖关系的变换
#### 注册细节


[CIPHER] 算法的注册特殊之处在于其 struct crypto_alg 字段 .cra_type 为空。必须填.cra_u.cipher，并配以实现此变换的适当回调
参见下文struct cipher_alg
#### 使用 struct cipher_alg 定义密码


Struct cipher_alg 定义单块密码
以下是当这些函数从内核其他部分被调用时的示意。注.cia_setkey() 调用可能发生在这些示意之前或之后，但不得在这些示意进行期间发生
```
             KEY ---.    PLAINTEXT ---.
                    v                 v
              .cia_setkey() -> .cia_encrypt()
                                      |
                                      '-----> CIPHERTEXT
```
请注意，多次调用 .cia_setkey() 的模式同样是合法的：

```
      KEY1 --.    PLAINTEXT1 --.         KEY2 --.    PLAINTEXT2 --.
             v                 v                v                 v
       .cia_setkey() -> .cia_encrypt() -> .cia_setkey() -> .cia_encrypt()
                               |                                  |
                               '---> CIPHERTEXT1                  '---> CIPHERTEXT2
```
### 多块密码


变换示例：cbc(aes)、chacha20 
本节描述多块密码变换的实现。多块密码用于操作提供给变换函数的分散列表（scatterlist）数据。它们也将结果输出到数据分散列表中
#### 注册细节


多块密码算法的注册是整个 crypto API 中最标准的流程之一
注意，如果密码实现要求数据适当对齐，调用者应使用 crypto_skcipher_alignmask() 函数来识别内存对齐掩码。内crypto API 能够处理未对齐的请求。但这也意味着会带来额外的开销，因为内crypto API 需要重新对齐数据，这可能涉及数据的移动
#### 使用 struct skcipher_alg 定义密码


Struct skcipher_alg 定义一个多块密码，或更一般地说，一个保持长度的对称密码算法
#### 分散列表处理


某些驱动希望使用 Generic ScatterWalk（通用分散游走），以防硬件需要被喂入分散列表中包含明文、并将包含密文的独立块。请参Linux 内核分散/聚集（scatter / gather）列表实现提供的 ScatterWalk 接口
### 哈希 [HASH]


变换示例：crc32、md5、sha1、sha256 
#### 注册与注销变换


根据变换是同步的 [SHASH] 还是异步[AHASH]，以及我们要注册多少 HASH 变换，有多种方式注册 HASH 变换。你可以include/crypto/internal/hash.h 中找到原型定义：

```
       int crypto_register_ahash(struct ahash_alg *alg);

       int crypto_register_shash(struct shash_alg *alg);
       int crypto_register_shashes(struct shash_alg *algs, int count);
```
注销 HASH 变换的对应函数如下：

```
       void crypto_unregister_ahash(struct ahash_alg *alg);

       void crypto_unregister_shash(struct shash_alg *alg);
       void crypto_unregister_shashes(struct shash_alg *algs, int count);
```
#### 使用 struct shash_alg ahash_alg 定义密码


以下是当这些函数从内核其他部分被调用时的示意。注.setkey() 调用可能发生在这些示意之前或之后，但不得在这些示意进行期间发生。请注意，先调用 .init() 然后紧接着调用 .final() 同样是一次完全合法的变换
```
       I)   DATA -----------.
                            v
             .init() -> .update() -> .final()      ! .update() 在此场景                         ^    |         |            可能根本不会被调用                         '----'         '---> HASH

       II)  DATA -----------.-----------.
                            v           v
             .init() -> .update() -> .finup()      ! .update() 在此场景                         ^    |         |            可能根本不会被调用                         '----'         '---> HASH

       III) DATA -----------.
                            v
                        .digest()                  ! 整个过程                            |                        .digest() 调用处理                            '---------------> HASH
```
以下.export()/.import() 函数从内核其他部分被调用时的示意
```
       KEY--.                 DATA--.
            v                       v                  ! .update() 在此场景        .setkey() -> .init() -> .update() -> .export()   可能根本不会被调用                                 ^     |         |
                                 '-----'         '--> PARTIAL_HASH

       ----------- 此处发生其他变换 -----------

       PARTIAL_HASH--.   DATA1--.
                     v          v
                 .import -> .update() -> .final()     ! .update() 在此场景                             ^    |         |           可能根本不会被调用                             '----'         '--> HASH1

       PARTIAL_HASH--.   DATA2-.
                     v         v
                 .import -> .finup()
                               |
                               '---------------> HASH2
```
请注意，“放弃”一个请求对象是完全合法的：
- 调用 .init()，然后（多次）调.update()
- 在将来任何时候都***调用 .final()finup() .export() 中的任何一
换言之，实现应顾及资源分配与清理与请求对象相关的资源不应在调.init() .update() 之后仍保持分配状态，因为可能再也没有机会释放它们
#### 异步 HASH 变换的细

某些驱动希望使用 Generic ScatterWalk，以防实现需要被喂入包含输入数据的分散列表的独立块