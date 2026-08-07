@echo off
chcp 65001 >nul
echo ========================================
echo   OA 项目编译 (Node.js + Gulp)
echo ========================================

node --version >nul 2>&1
if errorlevel 1 (
    echo [错误] 未检测到 Node.js，请先安装 Node.js 14+
    pause
    exit /b 1
)

echo [1/4] 安装依赖 ...
call npm install
if errorlevel 1 (
    echo [错误] npm install 失败
    pause
    exit /b 1
)

echo.
echo [2/4] 清理旧构建 ...
call npm run clear
if errorlevel 1 (
    echo [警告] 清理失败，继续执行...
)

echo.
echo [3/4] 准备环境 (Windows Java11) ...
call npm run preperation:win
if errorlevel 1 (
    echo [警告] 环境准备失败，继续执行...
)

echo.
echo [4/4] 构建服务端 + 前端 + API ...
call npm run build_parallel
if errorlevel 1 (
    echo [错误] 构建失败
    pause
    exit /b 1
)

echo.
echo [5/4] 部署到 target ...
call npm run deploy:win
if errorlevel 1 (
    echo [警告] 部署步骤失败
)

echo.
echo ========================================
echo   编译完成，输出目录: target\o2server\
echo ========================================
pause
