
## XFRM proc - /proc/net/xfrm_* 文件


Masahide NAKAMURA <nakam@linux-ipv6.org>


### 转换统计


xfrm_proc 代码是一组统计数据，显示了被转换代码丢弃的数据包数量及其原因。
这些计数器定义为 Linux 私有 MIB 的一部分。这些计数器可以在
/proc/net/xfrm_stat 中查看。


#### 入站错误


XfrmInError:
	所有不匹配其它项的所有错误

XfrmInBufferError:
	没有剩余缓冲区

XfrmInHdrError:
	头部错误

XfrmInNoStates:
	未找到状态
	即入站的 SPI、地址或 SA 上的 IPsec 协议有误

XfrmInStateProtoError:
	转换协议相关错误
	例如 SA 密钥有误

XfrmInStateModeError:
	转换模式相关错误

XfrmInStateSeqError:
	序列号错误
	即序列号超出窗口

XfrmInStateExpired:
	状态已过期

XfrmInStateMismatch:
	状态存在不匹配的选项
	例如 UDP 封装类型不匹配

XfrmInStateInvalid:
	状态无效

XfrmInTmplMismatch:
	状态没有匹配的模板
	例如入站 SA 正确但 SP 规则有误

XfrmInNoPols:
	状态未找到策略
	例如入站 SA 正确但未找到 SP

XfrmInPolBlock:
	策略丢弃

XfrmInPolError:
	策略错误

XfrmAcquireError:
	状态在使用前尚未被完全获取

XfrmFwdHdrError:
	不允许对数据包进行转发路由

XfrmInStateDirError:
        状态方向不匹配（在入站路径上查找到了出站状态，期望为入站或无方向）

#### 出站错误

XfrmOutError:
	所有不匹配其它项的所有错误

XfrmOutBundleGenError:
	捆绑（bundle）生成错误

XfrmOutBundleCheckError:
	捆绑检查错误

XfrmOutNoStates:
	未找到状态

XfrmOutStateProtoError:
	转换协议相关错误

XfrmOutStateModeError:
	转换模式相关错误

XfrmOutStateSeqError:
	序列号错误
	即序列号溢出

XfrmOutStateExpired:
	状态已过期

XfrmOutPolBlock:
	策略丢弃

XfrmOutPolDead:
	策略已失效

XfrmOutPolError:
	策略错误

XfrmOutStateInvalid:
	状态无效，可能已过期

XfrmOutStateDirError:
        状态方向不匹配（在出站路径上查找到了入站状态，期望为出站或无方向）
