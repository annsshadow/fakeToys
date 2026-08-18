@echo off
chcp 65001 >nul
echo ========================================
echo   AI 项目编译 / 依赖安装
echo ========================================

python --version >nul 2>&1
if errorlevel 1 (
    echo [错误] 未检测到 Python，请先安装 Python 3.x
    pause
    exit /b 1
)

echo [1/2] 创建虚拟环境 ...
python -m venv .venv
if errorlevel 1 (
    echo [警告] 虚拟环境创建失败，将使用系统 Python
)

echo [2/2] 安装依赖 ...
call .venv\Scripts\activate.bat
pip install -r requirements.txt 2>nul
if not exist requirements.txt (
    echo [提示] 未找到 requirements.txt，请手动安装依赖
    echo 依赖: nlpaug, nlpcda, requests
)

echo.
echo ========================================
echo   编译完成
echo ========================================
pause
