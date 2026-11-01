@echo off
chcp 65001 >nul
set PYTHONUTF8=1
set PYTHONIOENCODING=utf-8
set MNEMOS_HOME=%~dp0..
cd /d "%MNEMOS_HOME%"
if exist "%~dp0..\release\Mnemos\Mnemos.exe" (
  start "" "%~dp0..\release\Mnemos\Mnemos.exe"
) else (
  echo Mnemos.exe not found. Run build_windows.ps1 first.
  pause
)
