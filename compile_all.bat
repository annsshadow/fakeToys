@echo off
chdir /d "%~dp0"
echo ==========================================
echo   Building all subprojects...
echo ==========================================

echo.
echo [1/6] Building cool...
cd cool
call compile.bat
if errorlevel 1 exit /b 1
cd ..

echo.
echo [2/6] Building o2server...
cd oa\o2server
call compile.bat
if errorlevel 1 exit /b 1
cd ..\..

echo.
echo [3/6] Building o2web...
cd oa\o2web
call compile.bat
if errorlevel 1 exit /b 1
cd ..\..

echo.
echo [4/6] Building csv2sql...
cd csv2sql
call compile.bat
if errorlevel 1 exit /b 1
cd ..

echo.
echo [5/6] Building watering...
cd watering
call compile.bat
if errorlevel 1 exit /b 1
cd ..

echo.
echo [6/6] train/storage/docs - no build needed

echo.
echo ==========================================
echo   All subprojects built successfully!
echo ==========================================
pause
