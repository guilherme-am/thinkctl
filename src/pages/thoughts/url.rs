#[cfg(target_arch = "wasm32")]
fn url_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let h1 = bytes[i + 1];
                let h2 = bytes[i + 2];
                let hex = |b: u8| -> Option<u8> {
                    match b {
                        b'0'..=b'9' => Some(b - b'0'),
                        b'a'..=b'f' => Some(b - b'a' + 10),
                        b'A'..=b'F' => Some(b - b'A' + 10),
                        _ => None,
                    }
                };
                if let (Some(a), Some(b)) = (hex(h1), hex(h2)) {
                    out.push((a << 4) | b);
                    i += 3;
                } else {
                    out.push(bytes[i]);
                    i += 1;
                }
            }
            _ => {
                out.push(bytes[i]);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

#[cfg(target_arch = "wasm32")]
pub fn initial_query_from_url() -> Option<String> {
    use web_sys::window;
    let search = window()?.location().search().ok()?;
    let search = search.strip_prefix('?').unwrap_or(search.as_str());
    for pair in search.split('&') {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if k == "q" {
            let decoded = url_decode(v).trim().to_string();
            if !decoded.is_empty() {
                return Some(decoded);
            }
        }
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
pub fn initial_query_from_url() -> Option<String> {
    None
}
