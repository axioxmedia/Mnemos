# Build portable Mnemos.exe (onedir) into D:\GrokAssets\Mnemos\release\
$ErrorActionPreference = "Stop"
$Proj = Split-Path -Parent $PSScriptRoot
if (-not $Proj) { $Proj = "D:\GrokAssets\Mnemos" }
Set-Location $Proj

$env:MNEMOS_HOME = $Proj
$env:MNEMOS_DATA_DIR = Join-Path $Proj "data"
$env:OLLAMA_MODELS = Join-Path $Proj "ollama\models"
$env:OLLAMA_HOME = Join-Path $Proj "ollama\home"
$env:MNEMOS_OLLAMA_MODEL = "deepseek-r1:32b"
New-Item -ItemType Directory -Force -Path $env:MNEMOS_DATA_DIR,$env:OLLAMA_MODELS,$env:OLLAMA_HOME,(Join-Path $Proj "release") | Out-Null

Write-Host "== frontend =="
$env:PYTHONUTF8 = "1"
chcp 65001 | Out-Null
if (-not (Test-Path (Join-Path $Proj "node_modules"))) {
  npm install --no-fund --no-audit
}
# Always rebuild so Chinese strings stay UTF-8 in bundle
npx --yes vite build
$distHtml = Join-Path $Proj "dist\index.html"
if (-not (Test-Path $distHtml)) { throw "dist/index.html missing" }

Write-Host "== venv =="
$venv = Join-Path $Proj "packaging\.venv"
if (-not (Test-Path (Join-Path $venv "Scripts\python.exe"))) {
  python -m venv $venv
}
$py = Join-Path $venv "Scripts\python.exe"
& $py -m pip install -U pip wheel
& $py -m pip install -r (Join-Path $Proj "packaging\requirements.txt")
& $py -m pip install -e (Join-Path $Proj "sidecar")

Write-Host "== PyInstaller =="
$out = Join-Path $Proj "release"
$pi = Join-Path $venv "Scripts\pyinstaller.exe"
& $pi --noconfirm --clean --distpath $out --workpath (Join-Path $Proj "packaging\build") (Join-Path $Proj "packaging\mnemos.spec")

$exe = Join-Path $out "Mnemos\Mnemos.exe"
if (-not (Test-Path $exe)) { throw "Build failed: $exe not found" }
Write-Host "OK: $exe"

$srcNote = Join-Path $Proj "packaging\README_RELEASE.txt"
if (Test-Path $srcNote) {
  Copy-Item $srcNote (Join-Path $out "README_RELEASE.txt") -Force
  $content = Get-Content -Path $srcNote -Raw -Encoding UTF8
  $utf8bom = New-Object System.Text.UTF8Encoding $true
  [System.IO.File]::WriteAllText((Join-Path $out "shuoming.txt"), $content, $utf8bom)
}
Write-Host "DONE"
