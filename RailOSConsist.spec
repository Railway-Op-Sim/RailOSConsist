# -*- mode: python ; coding: utf-8 -*-


block_cipher = None

import sysconfig
import os.path
import pathlib

SPEC_LOC = pathlib.Path(__file__).parent


a = Analysis(
    [os.path.join(SPEC_LOC, 'railos_consist', '__init__.py')],
    pathex=[sysconfig.get_paths()["purelib"]],
    binaries=[],
    datas=[(os.path.join(SPEC_LOC, 'railos_consist', 'data'), 'railos_consist/data')],
    hiddenimports=[],
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
    a.binaries,
    a.zipfiles,
    a.datas,
    [],
    name='RailOSConsist',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=False,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
    icon=['media/icon.ico'],
)
