# Cool College OA 免登 Token 生成工具

用于 Cool College（coolcollege）OA 系统的免登 Token 生成，支持 AES 加密/解密、MD5 哈希、Base64 编解码，兼容 .NET 系统的 AES-CBC 解密算法。

## 依赖

- JDK 8+
- Apache Commons Codec

## 环境变量

```bash
export COOL_OA_KEY=<your_oa_key>
export COOL_OA_SECRET=<your_oa_secret>
```

Windows：

```cmd
set COOL_OA_KEY=<your_oa_key>
set COOL_OA_SECRET=<your_oa_secret>
```

密钥获取方式：向 Cool College 管理员申请 `COOL_OA_KEY` 和 `COOL_OA_SECRET`。

## 加密算法说明

| 方法 | 算法 | 用途 |
|------|------|------|
| `aesEncryp()` | AES/ECB/PKCS5Padding | 标准 AES 加密，支持 URL 安全输出 |
| `aesDecrypt()` | AES/ECB/PKCS5Padding | 标准 AES 解密 |
| `decryptData()` | AES/CBC/PKCS5Padding | 兼容 .NET 的 AES-CBC 解密 |
| `MD5()` | MD5 | 通用 MD5 哈希 |
| `oaMd5()` | MD5(key + thirdSecret) | OA 系统专用 MD5 |

### 密钥截断规则

当 key 长度超过 16 位时：
- 前 16 位作为 AES 密钥
- 后 16 位作为 CBC 模式的 IV

## 四种免登方式

运行 `main()` 方法可生成 4 种免登 Token：

| 类型 | 加密前字符串格式 | 说明 |
|------|----------------|------|
| UserId | `userId=<USER_ID>&enterpriseId=<ENTERPRISE_ID>` | 用户 ID 登录 |
| 工号 | `userId=<USER_ID>&enterpriseId=<ENTERPRISE_ID>&type=JOB_NUMBER` | 工号登录 |
| 手机号 | `userId=<MOBILE>&enterpriseId=<ENTERPRISE_ID>&type=LOGIN_MOBILE` | 手机号登录 |
| 邮箱 | `userId=<EMAIL>&enterpriseId=<ENTERPRISE_ID>&type=LOGIN_EMAIL` | 邮箱登录 |

## 使用方法

### 直接运行

```bash
javac cool.java
java net.coolcollege.login.helper.EncryptUtil
```

### 集成到其他系统

```java
import net.coolcollege.login.helper.EncryptUtil;

// 生成免登 Token
String key = EncryptUtil.oaMd5(coolKey, coolSecret);
String token = EncryptUtil.aesEncryp("userId=123&enterpriseId=456", key);
```

## 注意事项

- `COOL_OA_KEY` 和 `COOL_OA_SECRET` 属于敏感凭证，请勿硬编码到源码或提交到版本库
- AES/ECB 模式不需要 IV，AES/CBC 模式需要 IV（由密钥后 16 位派生）
- 与 .NET 系统交互时使用 `decryptData()` 方法，确保密钥截断规则一致
