# -*- mode: python ; coding: utf-8 -*-
from pathlib import Path

block_cipher = None
PACK = Path(SPECPATH).resolve()
PROJ = PACK.parent

ui_dir = PROJ / "dist"
sidecar_pkg = PROJ / "sidecar" / "mnemos_sidecar"

datas = []
if ui_dir.is_dir():
    datas.append((str(ui_dir), "ui"))
datas.append((str(sidecar_pkg), "mnemos_sidecar"))
res = PROJ / "resources"
if res.is_dir():
    datas.append((str(res), "resources"))

hidden = [
    "mnemos_sidecar",
    "mnemos_sidecar.config",
    "mnemos_sidecar.engine",
    "mnemos_sidecar.llm",
    "mnemos_sidecar.host",
    "mnemos_sidecar.host.http_server",
    "mnemos_sidecar.memory",
    "mnemos_sidecar.memory.context_builder",
    "mnemos_sidecar.memory.consolidation",
    "mnemos_sidecar.memory.crypto",
    "mnemos_sidecar.memory.db",
    "mnemos_sidecar.memory.gate",
    "mnemos_sidecar.memory.l0",
    "mnemos_sidecar.memory.l3",
    "mnemos_sidecar.retrieval",
    "mnemos_sidecar.retrieval.flyhash",
    "mnemos_sidecar.retrieval.retriever",
    "mnemos_sidecar.plugins",
    "mnemos_sidecar.plugins.base",
    "mnemos_sidecar.skills",
    "webview",
    "webview.platforms.edgechromium",
]

a = Analysis(
    [str(PACK / "launcher.py")],
    pathex=[str(PROJ / "sidecar"), str(PACK)],
    binaries=[],
    datas=datas,
    hiddenimports=hidden,
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=block_cipher,
    noarchive=False,
)
pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    [],
    exclude_binaries=True,
    name="Mnemos",
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=False,
    console=False,
    disable_windowed_traceback=False,
)

coll = COLLECT(
    exe,
    a.binaries,
    a.zipfiles,
    a.datas,
    strip=False,
    upx=False,
    upx_exclude=[],
    name="Mnemos",
)
