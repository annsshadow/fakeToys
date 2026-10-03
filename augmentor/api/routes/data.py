# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据管理 API 路由"""

import asyncio
import json
from pathlib import Path
from typing import Any, Dict, List, Optional

from fastapi import APIRouter, Depends, Form, HTTPException, UploadFile, File
from pydantic import BaseModel

from augmentor.converter import (
    EXCEL_FORMAT,
    INPUT_EXTENSION_FORMATS,
    INPUT_FORMAT_CHOICES,
    read_records,
)
from ..deps import (
    raise_internal_error,

    allowed_data_roots,
    assert_dataset_shape,
    get_pipeline,
    max_upload_bytes,
    read_json_file,
    resolve_within_roots,
    run_in_thread,
    to_http_error,
    verify_api_key,
    write_json_file,
)
from ..schemas import SuccessResponse

router = APIRouter(tags=["data"])


class DataFileInfo(BaseModel):
    """单个数据文件条目"""
    name: str
    path: str
    size: int


class DataFileListResponse(BaseModel):
    """数据文件列表"""
    files: List[DataFileInfo]


class DataLoadResponse(BaseModel):
    """分页读取结果

    `total` 是**过滤后**的总条数（`search` 生效时小于文件总条数），
    客户端据此翻页。
    """
    total: int
    page: int
    page_size: int
    items: List[Dict[str, Any]]


class DataUploadResponse(BaseModel):
    """上传结果：落盘路径与实际写入条数"""
    success: bool
    path: str
    count: int


class DatasetAnalysisResponse(BaseModel):
    """数据集分析结果（与 `Pipeline.analyze_dataset()` 的返回键一致）"""
    coverage_analysis: Dict[str, Any]
    statistics: Dict[str, Any]
    dedup_report: Dict[str, Any]


class VisualizeResponse(BaseModel):
    """可视化结果：图表名到文件路径的映射"""
    charts: Dict[str, str]


class DemoItem(BaseModel):
    """演示数据条目"""
    instruction: str
    input: str
    output: str


def _safe_data_path(filename: str) -> Path:
    """将文件名规范化为安全路径

    委托给 deps 中的统一白名单校验：拒绝 `..` 组件与绝对路径逃逸，
    并把结果限制在 `web.data_roots` 之内。是否要求文件存在由调用方判断。

    Args:
        filename: 客户端传入的文件名或路径

    Returns:
        已 resolve 的绝对路径

    Raises:
        HTTPException: 400 参数非法；403 路径越界
    """
    return resolve_within_roots(filename, "文件路径")


@router.get("/api/data/list", response_model=DataFileListResponse, summary="列出数据文件")
async def list_data_files():
    """列出白名单根目录下的 `train_data*.json`

    扫描范围**必须**与 `allowed_data_roots` 同源：这里曾写死 `Path(".")`，
    于是出厂默认收紧到 `["data"]` 之后，列表会把工作目录里的文件端给用户，而用户一点
    就是 403——列表能给的，路由必须能读。

    `name` 是裸文件名而 `{filename}` 只匹配单个路径段，所以它可加载的前提是
    `resolve_within_roots` 会在白名单根目录内查找相对路径（见 `deps` 的说明）。
    """
    loop = asyncio.get_event_loop()

    def scan_files():
        files = []
        seen = set()
        # 白名单里的目录可能不存在（例如 `data/` 还没建）。Python 3.13 的
        # `Path.glob` 对此返回空而不是抛错，因此无需 `is_dir()` 预检——加了
        # 反而是一条永远不会走到的分支。
        for root in allowed_data_roots():
            for f in sorted(root.glob("*.json")):
                if not f.name.startswith("train_data"):
                    continue
                resolved = f.resolve()
                if resolved in seen:  # 多个根目录互相嵌套时去重
                    continue
                seen.add(resolved)
                files.append({
                    "name": f.name,
                    "path": str(f),
                    "size": f.stat().st_size
                })
        return files

    files = await loop.run_in_executor(None, scan_files)
    return {"files": files}


@router.get(
    "/api/data/load/{filename}",
    response_model=DataLoadResponse,
    summary="分页加载数据",
)
async def load_data(filename: str, page: int = 1, page_size: int = 20, search: str = ""):
    """加载数据（分页）"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        items = await read_json_file(file_path)

        if search:
            search_lower = search.lower()
            items = [
                item for item in items
                if search_lower in item.get("instruction", "").lower()
                or search_lower in item.get("output", "").lower()
            ]

        total = len(items)
        start = (page - 1) * page_size
        end = start + page_size

        return {
            "total": total,
            "page": page,
            "page_size": page_size,
            "items": items[start:end]
        }
    except Exception as e:
        raise_internal_error(e)


@router.put(
    "/api/data/update/{filename}",
    response_model=SuccessResponse,
    summary="更新单条数据",
)
async def update_data_item(
    filename: str,
    index: int,
    item: dict,
    _auth: None = Depends(verify_api_key),
):
    """更新单条数据"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        items = await read_json_file(file_path)

        if index < 0 or index >= len(items):
            raise HTTPException(status_code=400, detail="索引越界")

        items[index] = item
        await write_json_file(file_path, items)

        return {"success": True}
    except HTTPException:
        raise
    except Exception as e:
        raise_internal_error(e)


@router.delete(
    "/api/data/delete/{filename}",
    response_model=SuccessResponse,
    summary="删除单条数据",
)
async def delete_data_item(
    filename: str, index: int, _auth: None = Depends(verify_api_key)
):
    """删除单条数据"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        items = await read_json_file(file_path)

        if index < 0 or index >= len(items):
            raise HTTPException(status_code=400, detail="索引越界")

        items.pop(index)
        await write_json_file(file_path, items)

        return {"success": True}
    except HTTPException:
        raise
    except Exception as e:
        raise_internal_error(e)


def _upload_source_format(filename: str, declared: str) -> Optional[str]:
    """这次上传该按哪个**源格式**解析：显式声明 > 扩展名；两者都认不出 ⇒ `None`

    `None` 不等于「json」，而是「本函数对格式没有主张」，路由随后原样走那条
    `json.loads` 的老路。为什么不能把「认不出」判成错误：无扩展名与 `.txt` 之类的
    名字上传 JSON 一直是成功的（实测改前 `noext` / `data.txt` 与 `data.json` 同路），
    判死等于把一批能用的客户端换掉。

    权威表是 `augmentor/converter.py` 的 `INPUT_EXTENSION_FORMATS`（读边）与
    `INPUT_FORMAT_CHOICES`（拼法清单），这里一个字都不另立 —— 加一种源格式仍然只改
    转换器那一行（A77「一份权威只住一处」）。显式声明 `json` 也返回 `None`：那样
    「声明了的 json」与「默认的 json」走的是同一条代码，不存在第二份 JSON 判据。
    """
    if declared:
        key = declared.strip().lower()
        if key == "json":
            return None
        if key not in INPUT_FORMAT_CHOICES:
            raise HTTPException(
                status_code=400,
                detail=(
                    f"input_format 不是读边认得的格式: {declared}"
                    f"（可选: {', '.join(INPUT_FORMAT_CHOICES)}；不传则按扩展名推断）"
                ),
            )
        return key
    inferred = INPUT_EXTENSION_FORMATS.get(Path(filename).suffix.lower())
    return None if inferred in (None, "json") else inferred


def _table_input_lies(content: bytes, source: str) -> bool:
    """交叉检查：整份字节其实是一份完整 JSON，却被按表格格式解析

    这一格不是假想敌，实测（`Temp/l91q/error_shapes.py` 第 2 档）拿一份缩进过的
    JSON 数组当 `.csv` 上传：`DictReader` 读出 **251 条**键为 `'['` 的行记录，
    顶层是数组、元素是对象 ⇒ `assert_dataset_shape` 全过 ⇒ 一份垃圾数据集静静落盘；
    同一份内容写成单行 JSON 时读出来是 **0 条**，于是上传回 200、`count: 0`。
    改前这两份输入都是 400「不是合法 JSON」⇒ 放行表格格式不能把「响亮地拒绝」换成
    「静默地接受」。

    `jsonl` 只判「整份能解析成数组」这一种：单条 JSON 对象既是一份合法的 JSONL，
    也是一份合法的 JSON，按 JSONL 读出一条记录才是它声明的语义，不该被当成撒谎。
    """
    if source == EXCEL_FORMAT:
        return False
    try:
        value = json.loads(content.decode("utf-8", "ignore"))
    except (ValueError, UnicodeDecodeError):
        return False
    if source == "jsonl":
        return isinstance(value, list)
    return isinstance(value, (list, dict))


def _upload_records(content: bytes, source: str, filename: str) -> Any:
    """表格 / JSONL 进料的解析，附两道产品面判决

    判据住在转换器（`read_records`，与 CLI 的 `convert_file` 同一套读边），这里只加
    「上传」这一侧才需要的两条：格式与内容互相矛盾、解析完一条也没有。

    Raises:
        HTTPException: 400 内容其实是一份 JSON / 解析出 0 条
        其余异常由调用方映射（`DataFormatError` 等都是 `ValueError` 子类 ⇒ 400）
    """
    if _table_input_lies(content, source):
        raise HTTPException(
            status_code=400,
            detail=(
                f"上传内容整份是一份合法 JSON，但按 {source} 解析：名字或 input_format "
                f"与内容不符。JSON 请去掉扩展名或用 input_format=json，"
                f"JSONL 请每行一条记录"
            ),
        )
    records = read_records(content, source, filename)
    if not records and content.strip():
        raise HTTPException(
            status_code=400,
            detail=(
                f"按 {source} 解析 {filename or '上传内容'} 得到 0 条记录："
                f"字节非空却一行也没读出来，通常是只有表头、整行注释或空行"
            ),
        )
    return records


def _upload_landing_path(filename: str, source: Optional[str]) -> Path:
    """上传落点：裸文件名在「按工作目录解释会越界」时才补根前缀，表格产物一律 `.json`

    **第一件事（A160，L93 起口径收窄但这一支仍活着）**：浏览器 `<input type=file>`
    的 multipart 只给裸文件名（Chromium/Firefox 会剥掉路径）。L93 之前
    `resolve_within_roots` 在「一个候选都不存在」时兜底到工作目录解释，而出厂默认
    下工作目录恰在闸外 ⇒ 从 UI 上传一份**新**文件必 403（实测 py314 旧默认配置：
    `data.json` → 403「路径超出允许的数据目录范围」，`data/data.json` → 200）。
    A163 修掉的就是那一格：现在裸名直接落进首个根目录，所以**新建**这一支已经由
    解析器本身负责。这一支仍然活着，是因为还剩一种形状：工作目录下**已有同名文件**
    而根内没有（`candidates` 取「第一个存在」⇒ 解释成闸外的那一份，仍然 403），
    这时仍按「先解释一次、越界才救」把它送回根内。

    补根前缀的时机因此始终是「**先按既有口径解释一次，只有解释成越界（403）才救**」，
    而不是无条件把裸名搬到 `roots[0]`：白名单可以有多项，而第一项未必就是这次请求
    该落的那一份 —— 直接取 `roots[0]` 会把一份本来能按工作目录解释的上传悄悄搬到
    别处去（本轮第一版就是这么写的，`tests/unit/test_upload_ceiling_l87.py` 与
    `tests/integration/test_api_data_extended.py` 三条用例当场把它判红了：文件不再
    落在断言的那个目录里；L93 的第一版也踩了同一格，判红的还是那三条）。带目录的
    显式写法（L87 挣来的那一格）与 `..`（400）、越界绝对路径（403）三支一字未动。

    **第二件事**：源是表格时把落点名换成 `.json`。转换器写边为同一件事站了
    `_reject_unwritable_output`：把 JSON 字节写进 `x.csv` 的名字就是一份假容器，
    而上传的产物**本来就是** JSON 字节（`write_json_file`）。上传面不做名字游戏 ⇒
    同一个数据集换扩展名再传一次不会覆盖它，也就不会把一份表格「转存」成同名的
    另一份数据。
    """
    name = filename.strip()
    try:
        path = resolve_within_roots(name, "文件路径")
    except HTTPException as e:
        raw = Path(name)
        if e.status_code != 403 or not name or raw.is_absolute() or raw.parent.parts:
            raise
        path = resolve_within_roots(str(allowed_data_roots()[0] / name), "文件路径")
    if source is not None and path.suffix.lower() != ".json":
        path = path.with_suffix(".json")
    return path


async def _read_upload_body(file: UploadFile, limit: int) -> bytes:
    """读上传体，但**先在 O(1) 的尺寸上判上限**；越界当场 413，且先于任何落盘动作

    为什么先判尺寸而不是分块攒：分块攒那份 `bytearray` 在**没越界**时要付实打实的
    代价（实测同一个 64.1 MiB 上传，攒块比一次 `read()` 多 64.0 MiB 峰值、慢 5.9 倍，
    因为 bytearray 扩容要留旧缓冲区）。而 Starlette 1.2.1 的 `UploadFile.write()`
    里就有 `self.size += len(data)`，解析完的 `size` 是**服务端自己按实际写入字节
    累加出来的**（不是客户端报的 Content-Length），于是「上限」可以在读之前一次判掉：
    越界的一个字节都不搬进内存，未越界的照旧一次读完。

    两道判据不是重复：`declared` 那道负责内存，事后那道负责形状（`size` 为 `None`
    的构造方 —— 不走解析器的直接构造 —— 只能读完了才知道有多大）。

    **如实记下管不到的那一半**：Starlette 的 multipart 解析在进入本函数之前就把文件
    部分写进 `SpooledTemporaryFile(max_size=1 MiB)`，超过就溢到临时目录；它那 1 MiB 的
    `max_part_size` 只判**非文件**表单字段（`_current_part.file is None` 那一支），
    文件部分直接绕开。L90 起，`api/middleware/upload_gate.py:UploadBodyGate` 在解析之前
    按 `Content-Length` 先拒一次「声明了长度且明显越界的整包」，于是这一半代价对**声明长度
    的客户端**不再发生；chunked（不声明长度）那一档闸看不见尺寸，仍然先写一次临时文件，
    再由本函数的第二道判据回话。要在全网络层封住，仍得配 ASGI/反代的 `client_max_body_size`。

    本函数是**权威判据**：闸用的是客户端自报的 `Content-Length`，这里用的是服务端自己
    累加出来的 `file.size`，两者不互换。

    Args:
        file: 上传的文件部分
        limit: 允许的最大字节数（来自 `deps.max_upload_bytes()`）

    Returns:
        不超过 `limit` 的完整字节

    Raises:
        HTTPException: 413 越界（文案含上限与实际/申报字节数，不回显内容）
    """
    declared = getattr(file, "size", None)
    if declared is not None and declared > limit:
        raise HTTPException(
            status_code=413,
            detail=(
                f"上传内容 {declared} 字节，超过 web.max_upload_bytes={limit}"
                "（未读取、未落盘）"
            ),
        )

    content = await file.read()
    if len(content) > limit:
        raise HTTPException(
            status_code=413,
            detail=(
                f"上传内容 {len(content)} 字节，超过 web.max_upload_bytes={limit}"
                "（未落盘）"
            ),
        )
    return content


@router.post(
    "/api/data/upload",
    response_model=DataUploadResponse,
    summary="上传数据文件",
)
async def upload_data(
    file: UploadFile = File(...),
    input_format: str = Form(
        "",
        description=(
            "源格式；留空则按文件名扩展名推断（" + " / ".join(INPUT_FORMAT_CHOICES) + "）。"
            "表格与 JSONL 的产物一律落成 JSON，落点名的扩展名随之换成 .json"
        ),
    ),
    _auth: None = Depends(verify_api_key),
):
    """上传数据文件

    支持的进料（L91 起）：**JSON 数组（原有）+ `.csv` / `.tsv` / `.jsonl` / `.xlsx`
    / `.xls`**，后四种由转换器读边归一成规范记录再落 JSON。改前实测（`Temp/l91q/
    upload_census.py`）：表格三种回 400「数据文件不是合法 JSON」、二进制两种回 400
    「上传内容不是合法的 UTF-8 文本」，即四种进料全无路（A132 ③）。

    落点：`file.filename` 按**白名单内的相对路径**用，不再剥成裸文件名。改前剥的
    那一刀把 `resolve_within_roots` 对外承诺的写法（「要写入请显式传 `data/xxx.json`」）
    在这条路由上变成了拿不到的出口——客户端唯一能成功的姿势是先知道目标已存在。
    `..` 仍是 400，越界的绝对路径仍是 403，白名单闸一字未松。
    **但裸文件名现在落进白名单根目录**（A160）：浏览器的文件控件只给裸名，改前从 UI
    上传新文件必 403，那条「不替调用方挑写入目录」的读侧口径在这里成了产品的死路
    ——L93 的 A163 把那条兜底本身也修了（写侧改口见 `api/deps.py` 的
    `resolve_within_roots`），这一支因此只剩「工作目录已有同名文件」那一种形状会走到。
    源是表格时落点名换成 `.json`，因为产物本来就是 JSON 字节 —— 写进 `x.csv` 的名字
    就是转换器写边专门拒收的那类假容器。

    读体上限见 `_read_upload_body`；形态判据与读侧共用 `assert_dataset_shape`，
    所以「上传成功但那份文件读端点自己拒绝」已经不可能发生。表格进料另有两道
    上传侧才需要的判决（`_upload_records`）：整份内容其实是 JSON 却按表格解析、
    以及解析完 0 条 —— 缺了它们，一份缩进的 JSON 数组改名 `.csv` 会被静静吃成
    251 条垃圾记录。

    **下面那句 `json.loads` 刻意留在事件循环里**（L87 尺子，`Temp/l87q/stall_parse_l87.py`）：
    AST 普查把它记成「async 处理器里的阻塞调用」，本仓对这一类的既有处置是
    `deps.run_in_thread`，但实测搬过去省的不到一成 —— 11.6 MiB 档最差跳距
    `inline` 比噪声地板高 11.0 ms、`executor` 高 11.8 ms（净收益 **−0.8 ms**）；
    64.1 MiB 档分别是 +88.1 ms 与 +78.1 ms（净收益 10.0 ms）。原因在 GIL：`json` 的
    C 扫描器整个 `loads()` 期间持锁，线程池治的是**让出 GIL 的等待**（磁盘 I/O、
    `sleep`），治不了解放锁的纯 CPU 工作。尺子的正负对照同批实测（`time.sleep(0.20)`：
    循环里 215.9 ms vs 线程里 20.0 ms ≈ 地板 19.0 ms），所以这两组差是真读数不是瞎读。
    **表格那一侧相反，走 `run_in_thread`**：xlsx 读边是 pandas + openpyxl 的纯 Python
    解析（zip 解压、XML 逐节点），与 `json.loads` 那种「一整段持锁的 C 扫描」不同形，
    按既有约定搬进线程池。
    真正的封顶是上面那条字节上限：停顿与 body 尺寸同阶，把尺寸交给配置就同时把
    最坏停顿交给了配置。

    Args:
        file: multipart 的文件部件
        input_format: 源格式，留空按扩展名推断；认不出的一律走既有 JSON 那条路
        _auth: 写操作鉴权

    Returns:
        `success` / `path`（实际落点名，表格输入已换成 `.json`）/ `count`
    """
    name = file.filename or ""
    source = _upload_source_format(name, input_format)
    content = await _read_upload_body(file, max_upload_bytes())
    save_path = _upload_landing_path(name, source)

    try:
        if source is None:
            items = json.loads(content.decode("utf-8"))
        else:
            items = await run_in_thread(_upload_records, content, source, name)
    except HTTPException:
        raise
    except UnicodeDecodeError as e:
        raise HTTPException(status_code=400, detail="上传内容不是合法的 UTF-8 文本") from e
    except Exception as e:
        raise to_http_error(e) from e

    assert_dataset_shape(items, "上传内容")

    try:
        await write_json_file(save_path, items)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e

    return {"success": True, "path": str(save_path), "count": len(items)}


# ============ 数据分析 ============

@router.get(
    "/api/analyze/{filename}",
    response_model=DatasetAnalysisResponse,
    summary="分析数据集",
)
async def analyze_data(filename: str):
    """分析数据集（覆盖度、统计信息、去重报告）"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        p = get_pipeline()
        return await run_in_thread(p.analyze_dataset, str(file_path))
    except HTTPException:
        raise
    except Exception as e:
        raise_internal_error(e)


@router.get(
    "/api/visualize/{filename}",
    response_model=VisualizeResponse,
    summary="生成可视化图表",
)
async def visualize_data(filename: str):
    """生成数据集可视化图表"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        p = get_pipeline()
        results = await run_in_thread(p.visualize_dataset, str(file_path))
        return {"charts": results}
    except HTTPException:
        raise
    except Exception as e:
        raise_internal_error(e)


# ============ 演示数据 ============

_DEMO_DATA = [
    {
        "instruction": "什么是人工智能？",
        "input": "",
        "output": "人工智能（AI）是计算机科学的一个分支，致力于创建能够执行通常需要人类智能的任务的系统，如学习、推理、问题解决、感知和语言理解。"
    },
    {
        "instruction": "解释一下机器学习的基本概念",
        "input": "",
        "output": "机器学习是人工智能的子领域，它使计算机能够从数据中学习并做出决策或预测，而无需显式编程。主要类型包括监督学习、无监督学习和强化学习。"
    },
    {
        "instruction": "深度学习与传统机器学习有什么区别？",
        "input": "",
        "output": "深度学习使用多层神经网络自动提取特征，适合处理大量数据；传统机器学习通常需要人工特征工程，在小数据集上可能更高效。"
    },
    {
        "instruction": "什么是自然语言处理（NLP）？",
        "input": "",
        "output": "自然语言处理是AI的一个分支，专注于让计算机理解、解释和生成人类语言。应用包括机器翻译、情感分析、聊天机器人和文本摘要。"
    },
    {
        "instruction": "请介绍一下计算机视觉的基本任务",
        "input": "",
        "output": "计算机视觉的基本任务包括图像分类、目标检测、语义分割、实例分割和图像生成。这些技术应用于自动驾驶、医学影像、安防监控等领域。"
    }
]


@router.get(
    "/api/demo/data",
    response_model=List[DemoItem],
    summary="内置演示数据集",
)
async def get_demo_data():
    """获取内置演示数据集，开箱即用

    返回的是**裸数组**（不是 `{"data": [...]}` 包装），前端可直接当数据集喂给
    其它端点。这里刻意返回列表而不是 `JSONResponse`：后者会绕过
    response_model，让上面声明的契约退化成装饰。
    """
    return _DEMO_DATA
