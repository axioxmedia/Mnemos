use pyo3::prelude::*;

const KC_DIM: usize = 512;
const KC_TOP_K: usize = 32;

fn tokens(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    let mut parts: Vec<String> = lower.split_whitespace().map(|s| s.to_string()).collect();
    let hans: String = lower
        .chars()
        .filter(|c| ('\u{4e00}'..='\u{9fff}').contains(c))
        .collect();
    let chars: Vec<char> = hans.chars().collect();
    for (i, ch) in chars.iter().enumerate() {
        parts.push(ch.to_string());
        if i + 1 < chars.len() {
            parts.push(format!("{}{}", ch, chars[i + 1]));
        }
    }
    if parts.is_empty() {
        parts.push("empty".into());
    }
    parts
}

fn kenyon_project(text: &str, dim: usize) -> Vec<f64> {
    let mut scores = vec![0.0_f64; dim];
    for tok in tokens(text) {
        let h = blake3::hash(tok.as_bytes());
        let bytes = h.as_bytes();
        for j in (0..32).step_by(4) {
            let idx = u16::from_le_bytes([bytes[j], bytes[j + 1]]) as usize % dim;
            let sign = if bytes[j + 2] % 2 == 0 { 1.0 } else { -1.0 };
            let w = 0.5 + (bytes[j + 3] as f64 / 255.0);
            scores[idx] += sign * w;
        }
    }
    scores
}

#[pyfunction]
#[pyo3(signature = (text, k=None, dim=None))]
fn flyhash_sparse_py(text: &str, k: Option<usize>, dim: Option<usize>) -> PyResult<Vec<usize>> {
    let dim = dim.unwrap_or(KC_DIM);
    let k = k.unwrap_or(KC_TOP_K).min(dim);
    let scores = kenyon_project(text, dim);
    let mut idxs: Vec<usize> = (0..dim).collect();
    idxs.sort_by(|&a, &b| {
        scores[b]
            .abs()
            .partial_cmp(&scores[a].abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut top: Vec<usize> = idxs.into_iter().take(k).collect();
    top.sort_unstable();
    Ok(top)
}

#[pyfunction]
fn pack_vec_int8_py(vec: Vec<f64>) -> PyResult<Vec<u8>> {
    if vec.is_empty() {
        let mut out = Vec::new();
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&1.0f32.to_le_bytes());
        return Ok(out);
    }
    let m = vec.iter().map(|x| x.abs()).fold(0.0_f64, f64::max).max(1e-12);
    let scale = (m / 127.0) as f32;
    let n = vec.len() as u32;
    let mut out = Vec::with_capacity(8 + vec.len());
    out.extend_from_slice(&n.to_le_bytes());
    out.extend_from_slice(&scale.to_le_bytes());
    for x in vec {
        let q = ((x as f32) / scale).round().clamp(-127.0, 127.0) as i8;
        out.push(q as u8);
    }
    Ok(out)
}

#[pyfunction]
fn backend_name() -> PyResult<&'static str> {
    Ok("pyo3")
}

#[pymodule]
fn mnemos_flyhash(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(flyhash_sparse_py, m)?)?;
    m.add_function(wrap_pyfunction!(pack_vec_int8_py, m)?)?;
    m.add_function(wrap_pyfunction!(backend_name, m)?)?;
    m.add("__version__", "0.1.0")?;
    Ok(())
}
