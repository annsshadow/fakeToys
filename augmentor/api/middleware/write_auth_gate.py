# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""未配密钥时的「传输边界」闸

`api/deps.py::verify_api_key` 的既有约定是：没设 ``AUGMENTOR_API_KEY`` 就不校验，
零配置可用（A347）。出厂 ``web.host`` 默认 ``0.0.0.0``，容器 CMD 也显式
``--host 0.0.0.0`` ⇒ 「改配置 / 删数据 / 清备份 / 回滚版本 / 发起增强」这一族写操作
在默认部署上是**全网卡匿名可达**，只有一条启动告警守着。

这一层不取消零配置可用，而是把它**收回到本地**：写方法只在受信传输（回环地址、
Unix 套接字、ASGI 测试夹具）上匿名放行；请求从一个非回环的本地监听地址进来就是
``403``，文案给出两条出路（设密钥，或走回环）。设了密钥时本层一个字都不管，
仍然是路由依赖的 ``401`` 契约。
"""

import ipaddress
from typing import Optional, Tuple

from starlette.responses import JSONResponse

from api import deps

WRITE_METHODS = frozenset({"POST", "PUT", "PATCH", "DELETE"})

# ASGI 测试夹具（httpx/TestClient）写进 `scope["server"]` 的固定主机名。
_TEST_TRANSPORT_HOSTS = frozenset({"", "localhost", "testserver"})

API_PREFIX = "/api/"


def is_trusted_transport(server: Optional[Tuple[str, int]]) -> bool:
    """请求是不是从「只有本机才够得着」的监听地址进来的

    ``scope["server"]`` 是**这条连接本地侧的地址**，由 ASGI 服务器写，不是请求头
    能伪造的东西：实测 uvicorn 绑 ``0.0.0.0`` 而从 ``127.0.0.1`` 访问时给出
    ``("127.0.0.1", port)``，从局域网地址访问时给出那个非回环 IP。因此「绑在 0.0.0.0
    但只有本机能用」照常工作，而跨机器的一次写请求必然落到闸外。

    Args:
        server: ``scope.get("server")``，形如 ``(host, port)``；缺省或 None 表示
            ASGI 服务器没有给出监听地址（Unix 套接字、手工构造的 scope）

    Returns:
        是否按受信传输放行
    """
    if server is None:
        return True
    host = str(server[0] if server else "")
    if not host:
        return True
    if host in _TEST_TRANSPORT_HOSTS:
        return True
    try:
        return ipaddress.ip_address(host).is_loopback
    except ValueError:
        # 主机名而不是 IP（本仓唯一的真实服务器 uvicorn 给的是 IP）：解析成 IP 再判，
        # 解析不了就按非回环拒——放行一个认不出的地址等于把这一层变成装饰。
        try:
            return ipaddress.ip_address(host.replace("[", "").replace("]", "")).is_loopback
        except ValueError:
            return False


class WriteTransportGate:
    """没配密钥时，把匿名写操作限制在受信传输上"""

    def __init__(self, app):
        """初始化中间件

        Args:
            app: 下游 ASGI 应用
        """
        self.app = app

    async def __call__(self, scope, receive, send):
        """非回环监听地址上的匿名写请求直接 403，其余原样下传

        Args:
            scope: ASGI 连接范围
            receive: 接收消息的可等待函数
            send: 发送消息的可等待函数
        """
        if (
            scope["type"] == "http"
            and scope["method"] in WRITE_METHODS
            and str(scope.get("path", "")).startswith(API_PREFIX)
            and not deps.api_key_required()
            and not is_trusted_transport(scope.get("server"))
        ):
            response = JSONResponse(
                status_code=403,
                content={
                    "detail": (
                        f"未设置 {deps.API_KEY_ENV}：写操作只允许从本机（回环）访问，"
                        f"而本请求到达的监听地址是 {scope.get('server')}。"
                        "请设置该环境变量后由客户端携带 X-API-Key，"
                        "或把服务绑到回环地址（web.host: 127.0.0.1）。"
                    )
                },
            )
            await response(scope, receive, send)
            return

        await self.app(scope, receive, send)


__all__ = ["API_PREFIX", "WRITE_METHODS", "WriteTransportGate", "is_trusted_transport"]
