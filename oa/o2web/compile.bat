@echo off
chdir /d "%~dp0"
echo Installing dependencies...
call npm install
if errorlevel 1 (
    echo [FAIL] o2web npm install failed
    pause
    exit /b 1
)
echo Building...
call npm run build
if errorlevel 1 (
    echo [FAIL] o2web build failed
    pause
    exit /b 1
)
echo [OK] o2web built successfully
