# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""跨路由复用的响应模型

这里只放**多个路由文件都要用**的形状。各路由独有的响应模型仍就近定义在
各自模块里——读一个路由文件就能看到它对外承诺的完整契约。

注意：这些模型是响应契约，不是请求体。请求体模型（`*Request`）仍留在各路由内。
"""

from pydantic import BaseModel


class SuccessResponse(BaseModel):
    """只表示操作成败的响应

    用于「改一条 / 删一条 / 回滚」这类无附加信息的写操作。
    """
    success: bool


class MessageResponse(BaseModel):
    """带人类可读说明的操作结果

    用于异步任务入队、配置落盘这类需要回一句话给用户的场景。
    """
    success: bool
    message: str
