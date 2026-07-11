<template>
  <div class="app">
    <!-- Toast -->
    <transition name="fade">
      <div v-if="toast.show" class="toast" :class="toast.type">{{ toast.msg }}</div>
    </transition>

    <!-- 阶段1: 卡号激活（默认） -->
    <div v-if="stage==='activate'" class="screen activate-screen">
      <div class="activate-card">
        <h1>⚡ Fast MMD</h1>
        <p>输入卡号，一键激活部署</p>
        <select v-model="platform" class="big-input" style="margin-bottom:8px">
          <option value="glm">GLM 站 (glm.2bbb.cn) — 5200积分/20元</option>
          <option value="tk">TK 站 (tk.2bbb.cn) — Token计费</option>
        </select>
        <input v-model="cardInput" class="big-input" placeholder="卡号 (5200-XXXX...)" @keydown.enter="doActivate" :disabled="loading" />
        <button class="big-btn" @click="doActivate" :disabled="loading">{{ loading ? '验证中...' : '激 活' }}</button>
        <div class="links">
          <a @click="stage='login'">账号登录</a>
          <span>·</span>
          <a @click="stage='register'">注册账号</a>
          <span>·</span>
          <a @click="openShop">购买卡号</a>
        </div>
        <hr style="border:none;border-top:1px solid rgba(255,255,255,.15);margin:20px 0" />
        <button class="query-btn" @click="doQueryDeploy" :disabled="queryLoading">{{ queryLoading ? '查询中...' : '🔍 部署查询' }}</button>
        <button class="query-btn" style="margin-top:8px" @click="showDiag = true">🔧 一键自检</button>
      </div>
    </div>

    <!-- 阶段2: 账号登录 -->
    <div v-else-if="stage==='login'" class="screen login-screen">
      <div class="activate-card">
        <h1>⚡ Fast MMD</h1>
        <p>账号登录</p>
        <select v-model="platform" class="big-input" style="margin-bottom:8px">
          <option value="glm">GLM 站 (glm.2bbb.cn)</option>
          <option value="tk">TK 站 (tk.2bbb.cn)</option>
        </select>
        <input v-model="username" class="big-input" placeholder="用户名" style="margin-bottom:8px" @keydown.enter="doLogin" />
        <input v-model="password" type="password" class="big-input" placeholder="密码" style="margin-bottom:8px" @keydown.enter="doLogin" />
        <button class="big-btn" @click="doLogin" :disabled="loading">{{ loading ? '登录中...' : '登 录' }}</button>
        <div class="links">
          <a @click="stage='register'">注册新账号</a>
          <span>·</span>
          <a @click="stage='activate'">← 卡号激活</a>
        </div>
      </div>
    </div>

    <!-- 阶段2b: 注册 -->
    <div v-else-if="stage==='register'" class="screen login-screen">
      <div class="activate-card">
        <h1>⚡ Fast MMD</h1>
        <p>注册新账号</p>
        <select v-model="platform" class="big-input" style="margin-bottom:8px">
          <option value="glm">GLM 站 (glm.2bbb.cn)</option>
          <option value="tk">TK 站 (tk.2bbb.cn)</option>
        </select>
        <input v-model="username" class="big-input" placeholder="用户名 (3-20位)" style="margin-bottom:8px" />
        <input v-model="password" type="password" class="big-input" placeholder="密码 (6位以上)" style="margin-bottom:8px" />
        <input v-model="password2" type="password" class="big-input" placeholder="确认密码" style="margin-bottom:8px" @keydown.enter="doRegister" />
        <button class="big-btn" @click="doRegister" :disabled="loading">{{ loading ? '注册中...' : '注 册' }}</button>
        <div class="links">
          <a @click="stage='login'">← 已有账号，去登录</a>
          <span>·</span>
          <a @click="stage='activate'">← 卡号激活</a>
        </div>
      </div>
    </div>

    <!-- 阶段3: 一键部署按钮 -->
    <div v-else-if="stage==='ready'" class="screen ready-screen">
      <div class="ready-card">
        <div class="success-icon">✅</div>
        <h1>{{ readyTitle }}</h1>
        <p class="balance-text">余额: <b>{{ balance.toFixed(2) }}</b> {{ platform==='tk' ? 'token' : '积分' }}</p>
        <button class="deploy-btn" @click="stage='wizard'">🚀 一键部署</button>
        <button class="skip-btn" @click="confirmSkip">跳过，直接进入</button>
      </div>
    </div>

    <!-- 阶段4: 部署向导 -->
    <DeployWizard v-else-if="stage==='wizard'" :api-key="apiKey" :server-platform="platform" @done="onDeployDone" @cancel="stage='ready'" />

    <!-- 阶段5: 主界面 -->
    <MainApp v-else-if="stage==='main'" :api-key="apiKey" :server-platform="platform" :user-token="userToken" :username="username" :balance="balance" @logout="logout" @deploy="stage='wizard'" />

    <!-- 阶段1b: 部署查询结果 -->
    <div v-else-if="stage==='query'" class="screen ready-screen">
      <div class="query-card">
        <h2>🔍 部署查询结果</h2>
        <div v-if="queryResult.installedPlatforms && queryResult.installedPlatforms.length > 0" class="query-section">
          <div class="query-section-title">已安装的平台：</div>
          <div v-for="p in queryResult.installedPlatforms" :key="p.platform" class="query-row">
            <span class="q-icon">{{ p.icon }}</span>
            <span class="q-name">{{ p.name }}</span>
            <span v-if="p.deployed" class="q-status ok">✅ 已部署</span>
            <span v-else class="q-status fail">❌ 未部署</span>
          </div>
        </div>
        <div v-else class="query-empty">未检测到任何已安装的平台<br><a @click="openDownloadLinks" style="color:#2f54eb;cursor:pointer">查看平台下载地址</a></div>

        <div v-if="queryResult.apiKey" class="query-section">
          <div class="query-section-title">检测到的 API Key：</div>
          <div class="query-key-box">
            <code>{{ queryResult.apiKey?.slice(0, 30) }}...</code>
            <button class="copy-mini" @click="copyText(queryResult.apiKey)">📋</button>
          </div>
          <div v-if="queryResult.balance !== null && queryResult.balance !== undefined" class="query-balance">
            剩余: <b>{{ Number(queryResult.balance).toFixed(2) }}</b> {{ platform==='tk' ? 'token' : '积分' }}
          </div>
        </div>

        <div v-if="queryResult.needDeploy" class="query-warn">
          ⚠️ 检测到已安装平台但未部署配置<br>请先激活卡号后部署
          <div style="margin-top:8px"><button class="big-btn" style="width:auto;padding:6px 20px" @click="stage='activate'">去激活</button></div>
        </div>

        <div class="query-actions">
          <button v-if="queryResult.apiKey" class="big-btn" @click="enterWithKey">使用此 Key 进入</button>
          <button class="query-back" @click="backFromQuery">← 返回</button>
        </div>
      </div>
    </div>

    <!-- 自检弹窗 -->
    <Diagnostics v-if="showDiag" @close="showDiag = false" />

    <!-- 自定义确认弹窗 -->
    <div v-if="confirmDialog.show" class="confirm-overlay" @click.self="confirmDialog.onCancel">
      <div class="confirm-box" @click.stop>
        <div class="confirm-title">{{ confirmDialog.title }}</div>
        <div class="confirm-msg">{{ confirmDialog.msg }}</div>
        <div class="confirm-btns">
          <button class="confirm-cancel" @click="confirmDialog.onCancel">取消</button>
          <button class="confirm-ok" @click="confirmDialog.onOk">确定</button>
        </div>
      </div>
    </div>

    <!-- 右下角版本号 -->
    <div class="version-bar">
      <span class="version-text">v{{ appVersion }}</span>
      <button class="version-check-btn" @click="manualCheckUpdate" :disabled="checkingUpdate">
        {{ checkingUpdate ? '检查中...' : '检查更新' }}
      </button>
    </div>

    <!-- 强制更新弹窗 -->
    <div v-if="updateInfo.show" class="update-overlay">
      <div class="update-box">
        <div class="update-icon">🔄</div>
        <h2>发现新版本 v{{ updateInfo.latest }}</h2>
        <p class="update-msg">检测到新版本已发布，请下载最新版本使用</p>
        <p class="update-version">当前版本 v{{ updateInfo.current }} → 最新版本 v{{ updateInfo.latest }}</p>
        <button class="update-btn" @click="goDownload">📥 立即下载新版本</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed } from "vue";
import DeployWizard from "./views/DeployWizard.vue";
import MainApp from "./views/MainApp.vue";
import Diagnostics from "./views/Diagnostics.vue";
import { redeemCard, lookup, login, register, createKey, openLink } from "./utils/api.js";
import { store } from "./utils/store.js";

const stage = ref("activate");
const cardInput = ref("");
const username = ref("");
const password = ref("");
const password2 = ref("");
const platform = ref("glm");
const apiKey = ref("");
const userToken = ref("");
const balance = ref(0);
const loading = ref(false);
const queryLoading = ref(false);
const queryResult = ref({});
const showDiag = ref(false);
const prevStage = ref("activate");
const toast = reactive({ show: false, msg: "", type: "info" });
const confirmDialog = reactive({ show: false, title: "确认", msg: "", onOk: null, onCancel: null });
const updateInfo = reactive({ show: false, current: 0, latest: 0, url: "" });
const appVersion = ref(0);
const checkingUpdate = ref(false);

function showConfirm(title, msg, onOk) {
  confirmDialog.show = true;
  confirmDialog.title = title;
  confirmDialog.msg = msg;
  confirmDialog.onOk = () => { confirmDialog.show = false; if (onOk) onOk(); };
  confirmDialog.onCancel = () => { confirmDialog.show = false; };
}

const PLATFORM_LABELS = {
  opencode: { icon: "📦", name: "OpenCode", url: "https://opencode.ai" },
  claudecode: { icon: "🤖", name: "Claude Code", url: "https://claude.ai/code" },
  codebuddy: { icon: "💻", name: "CodeBuddy", url: "https://codebuddy.cn" },
  workbuddy: { icon: "🔧", name: "WorkBuddy", url: "" },
  trae: { icon: "🚀", name: "Trae", url: "https://trae.cn" },
};

const readyTitle = computed(() => apiKey.value ? "激活成功" : userToken.value ? "登录成功" : "就绪");

function showToast(msg, type = "info") {
  toast.show = true; toast.msg = msg; toast.type = type;
  setTimeout(() => { toast.show = false; }, 3000);
}

function copyText(text) {
  navigator.clipboard.writeText(text);
  showToast("已复制", "success");
}

// 强制更新检测
async function checkForUpdate() {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const v = await invoke("get_app_version");
    appVersion.value = v;
    const r = await invoke("check_update");
    if (r.has_update) {
      updateInfo.show = true;
      updateInfo.current = r.current;
      updateInfo.latest = r.latest;
      updateInfo.url = r.url;
    }
  } catch(e) {
    // 非Tauri环境或请求失败，静默跳过
  }
}

// 手动检查更新
async function manualCheckUpdate() {
  checkingUpdate.value = true;
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const r = await invoke("check_update");
    if (r.has_update) {
      updateInfo.show = true;
      updateInfo.current = r.current;
      updateInfo.latest = r.latest;
      updateInfo.url = r.url;
    } else {
      showToast("当前已是最新版本 v" + r.current, "success");
    }
  } catch(e) {
    showToast("检查更新失败", "error");
  } finally {
    checkingUpdate.value = false;
  }
}

function goDownload() {
  if (updateInfo.url) openLink(updateInfo.url);
}

// 卡号格式校验 + 过滤前缀
function validateCard(card) {
  // 自动过滤"卡号："、"卡号:"、"卡号 "等前缀
  let cleaned = card.replace(/^卡号[：:\s]*/i, '').replace(/^card[：:\s]*/i, '').trim();
  return { valid: /^[0-9]+-[A-Z0-9]+$/i.test(cleaned), cleaned };
}

// 卡号激活
async function doActivate() {
  const raw = cardInput.value.trim();
  if (!raw) { showToast("请输入卡号", "error"); return; }
  // 过滤"卡号："等前缀
  const { valid, cleaned } = validateCard(raw);
  if (!valid) { showToast("卡号格式不正确，应为 5200-XXXX 格式", "error"); return; }
  loading.value = true;
  try {
    // 1. 先查本地记录，避免重复兑换
    const saved = store.get();
    if (saved.card === cleaned && saved.apiKey) {
      // 验证 key 是否还有效
      const verify = await lookup(saved.platform || platform.value, saved.apiKey);
      if (verify.ok) {
        apiKey.value = saved.apiKey;
        balance.value = verify.balance || saved.balance || 0;
        platform.value = saved.platform || "glm";
        showToast("卡号已激活，直接进入", "success");
        stage.value = "ready";
        loading.value = false;
        return;
      }
    }

    // 2. 兑换卡号（服务器端支持已使用卡号重新登录）
    const r = await redeemCard(platform.value, cleaned, "");
    if (r.ok) {
      apiKey.value = r.key;
      balance.value = r.balance || 0;
      store.set({ apiKey: r.key, balance: r.balance || 0, platform: platform.value, card: cleaned });
      showToast("登录成功", "success");
      stage.value = "ready";
    } else if (r.msg && (r.msg.includes("不存在") || r.msg.includes("封禁") || r.msg.includes("删除"))) {
      // 卡号被封禁/删除
      showToast(r.msg, "error");
    } else if (r.msg && r.msg.includes("已使用")) {
      // 不应该走到这里了（服务器已支持重新登录），但兜底
      showToast("此卡号已使用，请用账号登录", "error");
      setTimeout(() => { stage.value = "login"; }, 1500);
    } else {
      showToast(r.msg || "卡号无效", "error");
    }
  } catch(e) { showToast("网络错误: " + e.message, "error"); }
  finally { loading.value = false; }
}

// 账号登录 — 登录后自动创建/获取 API Key
async function doLogin() {
  if (!username.value || !password.value) { showToast("请输入用户名和密码", "error"); return; }
  loading.value = true;
  try {
    const r = await login(platform.value, username.value.trim(), password.value);
    if (r.ok) {
      userToken.value = r.token;
      balance.value = r.balance || 0;
      // 尝试创建/获取 API Key
      let key = "";
      if (r.token) {
        const kr = await createKey(platform.value, r.token, 1);
        if (kr.ok && kr.key) key = kr.key;
        else if (kr.keys && kr.keys.length) key = kr.keys[0].key_text;
        if (!key) showToast("API Key 创建失败，请联系管理员", "error");
      }
      apiKey.value = key;
      store.set({ token: r.token, username: r.username, balance: r.balance || 0, platform: platform.value, apiKey: key });
      showToast("登录成功", "success");
      stage.value = "ready";
    } else { showToast(r.msg || "登录失败", "error"); }
  } catch(e) { showToast("网络错误: " + e.message, "error"); }
  finally { loading.value = false; }
}

// 注册
async function doRegister() {
  if (!username.value || !password.value) { showToast("请填写用户名和密码", "error"); return; }
  if (username.value.length < 3) { showToast("用户名至少3位", "error"); return; }
  if (password.value.length < 6) { showToast("密码至少6位", "error"); return; }
  if (password.value !== password2.value) { showToast("两次密码不一致", "error"); return; }
  loading.value = true;
  try {
    const r = await register(platform.value, username.value.trim(), password.value);
    if (r.ok || r.token) {
      userToken.value = r.token || "";
      username.value = r.username || username.value;
      balance.value = 0;
      let key = "";
      if (r.token) {
        const kr = await createKey(platform.value, r.token, 1);
        if (kr.ok && kr.key) key = kr.key;
        else if (kr.keys && kr.keys.length) key = kr.keys[0].key_text;
      }
      apiKey.value = key;
      store.set({ token: r.token || "", username: r.username || username.value, balance: 0, platform: platform.value, apiKey: key });
      showToast("注册成功！欢迎 " + (r.username || username.value), "success");
      stage.value = "ready";
    } else { showToast(r.msg || r.detail || "注册失败", "error"); }
  } catch(e) { showToast("网络错误: " + e.message, "error"); }
  finally { loading.value = false; }
}

function confirmSkip() {
  showConfirm("跳过部署", "跳过后需手动在各平台配置 API Key。确定跳过？", () => {
    stage.value = "main";
  });
}

function onDeployDone() { stage.value = "main"; }

function logout() {
  showConfirm("退出登录", "确定退出登录？退出后需重新输入卡号或登录。", () => {
    store.clear();
    stage.value = "activate";
    apiKey.value = ""; userToken.value = ""; cardInput.value = "";
    username.value = ""; password.value = ""; password2.value = ""; balance.value = 0;
  });
}

function openShop() { openLink("https://item.taobao.com/item.htm?ft=t&id=1062470106379"); }

function openDownloadLinks() {
  openLink("https://opencode.ai");
  showToast("已打开下载页，其他平台：CodeBuddy(codebuddy.cn) Trae(trae.cn)", "info");
}

// 部署查询
async function doQueryDeploy() {
  queryLoading.value = true;
  queryResult.value = {};
  prevStage.value = stage.value;
  try {
    let installedPlatforms = [];
    let foundApiKey = "";

    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import("@tauri-apps/api/core");
      const detectResult = await invoke("detect_all_platforms");
      for (const [key, info] of Object.entries(detectResult)) {
        if (info.installed) {
          const label = PLATFORM_LABELS[key] || { icon: "❓", name: key };
          let deployed = false;
          let platformKey = "";
          try {
            const readResult = await invoke("read_platform_config", { platform: key });
            if (readResult) {
              const cfg = typeof readResult === "string" ? JSON.parse(readResult) : readResult;
              if (cfg.deployed) deployed = true;
              if (cfg.apiKey && cfg.apiKey.startsWith("fm-")) {
                platformKey = cfg.apiKey;
                if (!foundApiKey) foundApiKey = platformKey;
              }
            }
          } catch(e) {}
          installedPlatforms.push({ platform: key, icon: label.icon, name: label.name, deployed, apiKey: platformKey });
        }
      }
    } else {
      installedPlatforms = [
        { platform: "opencode", icon: "📦", name: "OpenCode", deployed: false },
        { platform: "claudecode", icon: "🤖", name: "Claude Code", deployed: false },
      ];
    }

    // 也检查本地存储
    const saved = store.get();
    if (saved.apiKey && !foundApiKey) foundApiKey = saved.apiKey;

    // 查余额
    let queryBalance = null;
    if (foundApiKey) {
      const lookupResult = await lookup(platform.value, foundApiKey);
      if (lookupResult.ok) queryBalance = lookupResult.balance;
    }

    queryResult.value = {
      installedPlatforms, apiKey: foundApiKey, balance: queryBalance,
      needDeploy: installedPlatforms.length > 0 && !foundApiKey,
    };
    stage.value = "query";
  } catch(e) { showToast("查询失败: " + e.message, "error"); }
  finally { queryLoading.value = false; }
}

function backFromQuery() { stage.value = prevStage.value; }

async function enterWithKey() {
  if (!queryResult.value.apiKey) return;
  // 验证 Key
  const verify = await lookup(platform.value, queryResult.value.apiKey);
  if (!verify.ok) { showToast("Key 已失效，请重新激活", "error"); return; }
  apiKey.value = queryResult.value.apiKey;
  balance.value = verify.balance || 0;
  store.set({ apiKey: apiKey.value, balance: balance.value, platform: platform.value });
  showToast("已进入", "success");
  stage.value = "main";
}

// 自动登录 — 验证 Key 有效性，封禁/删除的 Key 直接清除
try {
  const saved = store.get();
  if (saved.apiKey) {
    // 验证 key — 如果被封禁/删除 lookup 会返回错误
    lookup(saved.platform || "glm", saved.apiKey).then(r => {
      if (r.ok) {
        apiKey.value = saved.apiKey;
        balance.value = r.balance || saved.balance || 0;
        platform.value = saved.platform || "glm";
        cardInput.value = saved.card || "";
        stage.value = "main";
      } else {
        // Key 失效/封禁/删除 — 清除记录，回到激活页
        store.clear();
        if (r.msg) showToast(r.msg, "error");
        else showToast("登录已过期，请重新激活", "error");
      }
    });
  } else if (saved.token) {
    userToken.value = saved.token;
    username.value = saved.username || "";
    balance.value = saved.balance || 0;
    platform.value = saved.platform || "glm";
    apiKey.value = saved.apiKey || "";
    stage.value = "main";
  }
} catch(e) {}

// 启动时检测更新
checkForUpdate();
</script>

<style>
* { margin:0; padding:0; box-sizing:border-box; }
body { font-family:"Segoe UI","Microsoft YaHei",sans-serif; overflow:hidden; user-select:none; }
input, textarea, select { user-select:text; -webkit-user-select:text; -webkit-app-region:none; }
.app { width:100vw; height:100vh; overflow:hidden; }
.screen { width:100%; height:100%; display:flex; align-items:center; justify-content:center; }

/* Toast */
.toast { position:fixed; top:20px; left:50%; transform:translateX(-50%); padding:10px 24px; border-radius:8px; color:#fff; font-size:14px; z-index:99999; box-shadow:0 4px 12px rgba(0,0,0,.2); }
.toast.info { background:#2f54eb; }
.toast.success { background:#52c41a; }
.toast.error { background:#ff4d4f; }
.fade-enter-active, .fade-leave-active { transition:opacity .3s; }

.activate-card { width:360px; text-align:center; }
.activate-card h1 { color:#fff; font-size:28px; margin-bottom:4px; }
.activate-card p { color:rgba(255,255,255,.7); font-size:13px; margin-bottom:20px; }
.big-input { width:100%; height:44px; border:2px solid rgba(255,255,255,.3); border-radius:10px; padding:0 16px; font-size:15px; outline:none; background:rgba(255,255,255,.1); color:#fff; }
.big-input::placeholder { color:rgba(255,255,255,.5); }
.big-input:focus { border-color:#fff; background:rgba(255,255,255,.15); }
.big-input option { color:#333; }
.big-btn { width:100%; height:44px; border:none; border-radius:10px; background:#fff; color:#2f54eb; font-size:16px; font-weight:600; cursor:pointer; margin-top:12px; }
.big-btn:disabled { opacity:.6; }
.links { margin-top:16px; font-size:13px; }
.links a { color:rgba(255,255,255,.8); cursor:pointer; }
.links a:hover { color:#fff; }
.links span { margin:0 8px; color:rgba(255,255,255,.3); }

.query-btn { width:100%; height:38px; border:1px solid rgba(255,255,255,.4); border-radius:8px; background:transparent; color:rgba(255,255,255,.9); font-size:14px; cursor:pointer; }
.query-btn:hover { background:rgba(255,255,255,.1); }

.ready-card { text-align:center; }
.success-icon { font-size:48px; margin-bottom:8px; }
.ready-card h1 { color:#2f54eb; font-size:24px; margin-bottom:8px; }
.balance-text { font-size:16px; color:#555; margin-bottom:24px; }
.deploy-btn { width:280px; height:52px; border:none; border-radius:12px; background:linear-gradient(135deg,#2f54eb,#722ed1); color:#fff; font-size:18px; font-weight:600; cursor:pointer; box-shadow:0 4px 16px rgba(47,84,235,.3); }
.deploy-btn:hover { box-shadow:0 6px 24px rgba(47,84,235,.4); transform:translateY(-1px); }
.skip-btn { display:block; margin:12px auto 0; background:none; border:none; color:#999; font-size:13px; cursor:pointer; }

.query-card { width:480px; max-width:95vw; background:#fff; border-radius:16px; padding:24px; box-shadow:0 8px 32px rgba(0,0,0,.1); }
.query-card h2 { color:#2f54eb; font-size:20px; margin-bottom:16px; text-align:center; }
.query-section { margin-bottom:16px; }
.query-section-title { font-size:14px; font-weight:600; color:#555; margin-bottom:8px; }
.query-row { display:flex; align-items:center; gap:8px; padding:8px 12px; background:#f5f6fa; border-radius:8px; margin-bottom:4px; }
.q-icon { font-size:18px; }
.q-name { flex:1; font-size:14px; font-weight:600; }
.q-status { font-size:13px; }
.q-status.ok { color:#52c41a; }
.q-status.fail { color:#ff4d4f; }
.query-empty { text-align:center; color:#999; padding:20px; font-size:14px; }
.query-key-box { display:flex; align-items:center; gap:8px; padding:8px 12px; background:#f5f6fa; border-radius:8px; }
.query-key-box code { font-size:12px; color:#2f54eb; word-break:break-all; flex:1; }
.copy-mini { border:none; background:none; cursor:pointer; font-size:14px; }
.query-balance { margin-top:8px; font-size:16px; text-align:center; }
.query-balance b { color:#52c41a; font-size:20px; }
.query-warn { background:#fff7e6; border:1px solid #ffd591; border-radius:8px; padding:10px; font-size:13px; color:#d46b08; margin:12px 0; }
.query-actions { margin-top:16px; }
.query-back { display:block; margin:12px auto 0; background:none; border:none; color:#999; cursor:pointer; font-size:13px; }

.activate-screen { background:linear-gradient(135deg,#667eea,#764ba2); }
.ready-screen { background:#f5f6fa; }
.login-screen { background:linear-gradient(135deg,#667eea,#764ba2); }

/* 自定义确认弹窗 */
.confirm-overlay { position:fixed; top:0; left:0; right:0; bottom:0; background:rgba(0,0,0,.5); z-index:99999; display:flex; align-items:center; justify-content:center; }
.confirm-box { background:#fff; border-radius:16px; padding:24px; width:340px; text-align:center; box-shadow:0 8px 32px rgba(0,0,0,.2); }
.confirm-title { font-size:17px; font-weight:700; color:#333; margin-bottom:10px; }
.confirm-msg { font-size:14px; color:#666; line-height:1.6; margin-bottom:20px; }
.confirm-btns { display:flex; gap:12px; }
.confirm-btns button { flex:1; height:40px; border:none; border-radius:10px; font-size:15px; cursor:pointer; }
.confirm-cancel { background:#f5f5f5; color:#666; }
.confirm-ok { background:#2f54eb; color:#fff; }

/* 强制更新弹窗 */
.update-overlay { position:fixed; top:0; left:0; right:0; bottom:0; background:rgba(0,0,0,.7); z-index:999999; display:flex; align-items:center; justify-content:center; }
.update-box { background:#fff; border-radius:20px; padding:40px 48px; text-align:center; box-shadow:0 12px 48px rgba(0,0,0,.3); max-width:400px; }
.update-icon { font-size:56px; margin-bottom:16px; }
.update-box h2 { color:#333; font-size:22px; margin-bottom:12px; }
.update-msg { color:#666; font-size:15px; margin-bottom:8px; }
.update-version { color:#999; font-size:13px; margin-bottom:24px; }
.update-btn { width:100%; height:48px; border:none; border-radius:12px; background:linear-gradient(135deg,#2f54eb,#722ed1); color:#fff; font-size:17px; font-weight:600; cursor:pointer; box-shadow:0 4px 16px rgba(47,84,235,.3); }
.update-btn:hover { box-shadow:0 6px 24px rgba(47,84,235,.4); transform:translateY(-1px); }

/* 右下角版本栏 */
.version-bar { position:fixed; bottom:8px; right:12px; z-index:9999; display:flex; align-items:center; gap:8px; }
.version-text { font-size:12px; color:rgba(255,255,255,.6); }
.version-check-btn { border:1px solid rgba(255,255,255,.25); background:rgba(255,255,255,.08); color:rgba(255,255,255,.7); font-size:12px; padding:3px 10px; border-radius:6px; cursor:pointer; }
.version-check-btn:hover { background:rgba(255,255,255,.15); color:#fff; }
.version-check-btn:disabled { opacity:.5; cursor:default; }
</style>
