# mnemos_flyhash (PyO3)

第三阶段：FlyHash 热点路径 Rust(PyO3) 扩展。

```bash
# Windows / Linux（需 Rust + maturin）
cd native/mnemos_flyhash
pip install maturin
maturin develop --release
# 或 maturin build --release 产出 wheel，装到 data_dir/wheels
```

Python `retrieval/flyhash.py` 优先 `import mnemos_flyhash`，失败回退纯 Python（仍含 int8 pack + Kenyon 稀疏）。

导出：
- `flyhash_sparse_py(text, k=None, dim=None)`
- `pack_vec_int8_py(vec)`
- `backend_name()` → `"pyo3"`
