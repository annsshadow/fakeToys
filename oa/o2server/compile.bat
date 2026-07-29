@echo off
chdir /d "%~dp0"
mvn compile -q -Dmaven.compiler.release=11
if errorlevel 1 (
    echo [FAIL] o2server compile failed
    pause
    exit /b 1
)
echo [OK] o2server compiled successfully
