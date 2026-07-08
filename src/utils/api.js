const BASE_URLS = { glm: "https://glm.2bbb.cn", tk: "https://tk.2bbb.cn" };
const BASE = (p) => BASE_URLS[p] || BASE_URLS.glm;

/// 打开外部链接（Tauri 环境用 Rust，Web 环境用 window.open）
export async function openLink(url) {
  if (window.__TAURI_INTERNALS__) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("open_url", { url });
    } catch(e) {
      // 兜底：尝试 window.open
      window.open(url, "_blank");
    }
  } else {
    window.open(url, "_blank");
  }
}

async function fetchJson(url, opts = {}, timeout = 15000) {
  const ctrl = new AbortController();
  const tid = setTimeout(() => ctrl.abort(), timeout);
  try {
    const r = await fetch(url, { ...opts, signal: ctrl.signal });
    clearTimeout(tid);
    if (!r.ok) {
      let msg = `HTTP ${r.status}`;
      try { const d = await r.json(); msg = d.msg || d.detail || msg; } catch(e) {}
      return { ok: false, msg };
    }
    return await r.json();
  } catch(e) {
    clearTimeout(tid);
    if (e.name === "AbortError") return { ok: false, msg: "请求超时，请检查网络" };
    return { ok: false, msg: "网络错误: " + e.message };
  }
}

export async function login(platform, username, password) {
  return fetchJson(BASE(platform) + "/api/login", {
    method: "POST", headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password }),
  });
}

export async function register(platform, username, password) {
  // 先尝试 /api/register（需要turnstile），失败则尝试 /start/api/auth/register（不需要）
  let r = await fetchJson(BASE(platform) + "/api/register", {
    method: "POST", headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password, "cf-turnstile-response": "" }),
  });
  if (!r.ok && r.msg && (r.msg.includes("人机验证") || r.msg.includes("turnstile"))) {
    r = await fetchJson(BASE(platform) + "/start/api/auth/register", {
      method: "POST", headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ identifier: username, username, password }),
    });
    if (r.token) r.ok = true;
  }
  return r;
}

export async function redeemCard(platform, card, existingKey) {
  return fetchJson(BASE(platform) + "/api/card/redeem", {
    method: "POST", headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ card, key: existingKey || "" }),
  });
}

export async function lookup(platform, apiKey) {
  return fetchJson(BASE(platform) + "/api/lookup", {
    method: "POST", headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ key: apiKey }),
  });
}

export async function getModels(platform, apiKey) {
  const r = await fetch(BASE(platform) + "/v1/models", { headers: { Authorization: "Bearer " + apiKey } });
  return r.json();
}

export async function createKey(platform, token, groupId) {
  return fetchJson(BASE(platform) + "/api/keys", {
    method: "POST", headers: { "Content-Type": "application/json", Authorization: "Bearer " + token },
    body: JSON.stringify({ group_id: groupId || 1 }),
  });
}
