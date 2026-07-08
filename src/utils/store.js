const STORE_KEY = "glm_launcher_auth";

export const store = {
  get() {
    try { return JSON.parse(localStorage.getItem(STORE_KEY)) || {}; } catch { return {}; }
  },
  set(data) {
    // 合并而非覆盖，避免丢失字段
    const old = this.get();
    localStorage.setItem(STORE_KEY, JSON.stringify({ ...old, ...data }));
  },
  clear() { localStorage.removeItem(STORE_KEY); },
  getApiKey() { return this.get().apiKey || ""; },
  getPlatform() { return this.get().platform || "glm"; },
};
