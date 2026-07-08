<template>
  <div class="diag-overlay" @click.self="$emit('close')">
    <div class="diag-box">
      <div class="diag-header">
        <h2>🔧 一键自检</h2>
        <button class="xbtn" @click="$emit('close')">✕</button>
      </div>

      <!-- Tab -->
      <div class="diag-tabs">
        <button :class="{active:tab==='auto'}" @click="tab='auto'">🔍 自动诊断</button>
        <button :class="{active:tab==='chat'}" @click="tab='chat'">💬 对话检测</button>
        <button :class="{active:tab==='error'}" @click="tab='error'">❓ 错误查询</button>
      </div>

      <!-- 自动诊断 -->
      <div v-if="tab==='auto'">
        <div v-if="running" class="diag-running"><div class="spin">🔄</div><p>诊断中...</p></div>
        <div v-else>
          <div class="diag-summary">
            <span class="ok-c">✅ {{ okCount }}</span>
            <span class="warn-c">⚠️ {{ warnCount }}</span>
            <span class="err-c">❌ {{ errCount }}</span>
            <button v-if="fixableCount>0" class="fix-all" @click="fixAll" :disabled="fixing">{{ fixing?'修复中...':`一键修复(${fixableCount})` }}</button>
          </div>
          <div class="diag-list">
            <div v-for="item in results" :key="item.id" class="diag-item" :class="item.status">
              <div class="di">
                <span class="di-icon">{{ statusIcon(item.status) }}</span>
                <span class="di-cat">[{{ item.category }}]</span>
                <span class="di-title">{{ item.title }}</span>
                <button v-if="item.fixable" class="fix-btn" @click="fixOne(item)" :disabled="fixing">🔧修复</button>
              </div>
              <div class="di-detail">{{ item.detail }}</div>
            </div>
          </div>
          <div class="diag-actions">
            <button @click="runDiagnostics" :disabled="running">🔄重新检测</button>
            <button @click="backupAll" :disabled="fixing">💾备份配置</button>
            <button @click="clearCache" :disabled="fixing">🗑清理缓存</button>
          </div>
        </div>
      </div>

      <!-- 对话检测 -->
      <div v-if="tab==='chat'" class="chat-test">
        <p class="ct-hint">用您的 API Key 发送一个测试对话，验证是否可用</p>
        <div class="ct-form">
          <label>API Key</label>
          <input v-model="testKey" class="ct-input" placeholder="fm-XXXX..." />
          <label>Base URL</label>
          <input v-model="testUrl" class="ct-input" placeholder="https://glm.2bbb.cn" />
          <label>模型</label>
          <select v-model="testModel" class="ct-input">
            <option value="glm-5.2">glm-5.2 (推荐)</option>
            <option value="deepseek-v3">deepseek-v3</option>
            <option value="deepseek-r1">deepseek-r1</option>
            <option value="kimi-k2.7">kimi-k2.7</option>
          </select>
          <button class="ct-btn" @click="runChatTest" :disabled="chatTesting">{{ chatTesting?'检测中...':'🚀 发送测试对话' }}</button>
        </div>
        <div v-if="chatResult" class="ct-result" :class="chatResult.success?'ok':'fail'">
          <div v-if="chatResult.success" class="ct-success">
            <div class="ct-label">✅ 对话成功！</div>
            <div class="ct-reply">AI 回复: <b>{{ chatResult.reply }}</b></div>
            <div class="ct-detail">{{ chatResult.detail }}</div>
          </div>
          <div v-else class="ct-fail">
            <div class="ct-label">❌ 对话失败</div>
            <div class="ct-errcode">错误码: <b>{{ chatResult.error_code }}</b></div>
            <div class="ct-errmsg">{{ chatResult.error_msg }}</div>
            <div class="ct-detail">{{ chatResult.detail }}</div>
            <div class="ct-guide">
              <div class="ct-guide-title">📋 修复步骤：</div>
              <pre class="ct-guide-text">{{ chatResult.fix_guide }}</pre>
            </div>
          </div>
        </div>
      </div>

      <!-- 错误查询 -->
      <div v-if="tab==='error'" class="err-lookup">
        <p class="el-hint">输入您遇到的错误码或错误信息，自动查询原因和解决方案</p>
        <input v-model="errorCodeInput" class="ct-input" placeholder="如: 402, 404, 11140, insufficient_balance..." @keydown.enter="lookupError" />
        <button class="ct-btn" @click="lookupError" :disabled="errorLooking">{{ errorLooking?'查询中...':'🔍 查询' }}</button>
        <div v-if="errorResult" class="el-result">
          <div class="el-title">{{ errorResult.title }}</div>
          <div class="el-cause"><b>原因:</b> {{ errorResult.cause }}</div>
          <div class="el-guide-title">📋 修复教程：</div>
          <pre class="el-guide-text">{{ errorResult.guide }}</pre>
        </div>
      </div>

      <div v-if="fixResult" class="fix-toast" :class="fixResult.type">{{ fixResult.msg }}</div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from "vue";

const props = defineProps({});
const emit = defineEmits(["close"]);

const tab = ref("auto");
const running = ref(false);
const fixing = ref(false);
const results = ref([]);
const fixResult = ref(null);

// 对话检测
const testKey = ref("");
const testUrl = ref("https://glm.2bbb.cn");
const testModel = ref("glm-5.2");
const chatTesting = ref(false);
const chatResult = ref(null);

// 错误查询
const errorCodeInput = ref("");
const errorLooking = ref(false);
const errorResult = ref(null);

const okCount = computed(() => results.value.filter(r => r.status === "ok").length);
const warnCount = computed(() => results.value.filter(r => r.status === "warning").length);
const errCount = computed(() => results.value.filter(r => r.status === "error").length);
const fixableCount = computed(() => results.value.filter(r => r.fixable).length);

function statusIcon(s) { return s === "ok" ? "✅" : s === "warning" ? "⚠️" : s === "error" ? "❌" : "ℹ️"; }

async function runDiagnostics() {
  running.value = true;
  fixResult.value = null;
  results.value = [];
  try {
    let diag = [];

    // 1. 网络
    try {
      const r = await fetch("https://glm.2bbb.cn/api/settings", { signal: AbortSignal.timeout(5000) });
      diag.push({ id:"net", category:"网络", title: r.ok ? "服务器连接正常" : "服务器响应异常", status: r.ok?"ok":"warning", detail: r.ok?"glm.2bbb.cn 可达":`HTTP ${r.status}`, fixable:false, fix_action:"" });
    } catch(e) {
      diag.push({ id:"net", category:"网络", title:"无法连接服务器", status:"error", detail:"检查网络/VPN/防火墙", fixable:false, fix_action:"" });
    }

    // 2. Rust 诊断
    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import("@tauri-apps/api/core");
      const rustResults = await invoke("run_diagnostics");
      diag = diag.concat(rustResults.filter(r => r.id !== "network" && r.id !== "disk"));
    } else {
      diag.push({ id:"mock", category:"平台", title:"Web模式（请用桌面客户端）", status:"warning", detail:"", fixable:false, fix_action:"" });
    }

    results.value = diag;
  } catch(e) {
    results.value = [{ id:"err", category:"系统", title:"诊断失败", status:"error", detail:e.message, fixable:false, fix_action:"" }];
  } finally { running.value = false; }
}

async function fixOne(item) {
  if (!item.fixable) return;
  fixing.value = true;
  try {
    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import("@tauri-apps/api/core");
      const r = await invoke("run_fix", { fixAction: item.fix_action });
      item.status = "ok"; item.fixable = false; item.detail = "已修复: " + r;
      showFix("✅ " + r, "success");
    }
  } catch(e) { showFix("❌ " + e.message, "error"); }
  finally { fixing.value = false; }
}

async function fixAll() {
  fixing.value = true;
  const fixable = results.value.filter(r => r.fixable);
  let ok = 0, fail = 0;
  for (const item of fixable) {
    try {
      if (window.__TAURI_INTERNALS__) {
        const { invoke } = await import("@tauri-apps/api/core");
        await invoke("run_fix", { fixAction: item.fix_action });
        item.status = "ok"; item.fixable = false; ok++;
      }
    } catch(e) { fail++; }
  }
  fixing.value = false;
  showFix(`修复完成: 成功${ok} 失败${fail}`, ok > 0 ? "success" : "error");
}

async function backupAll() {
  fixing.value = true;
  try {
    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import("@tauri-apps/api/core");
      showFix("💾 " + await invoke("backup_configs"), "success");
    }
  } catch(e) { showFix("❌ " + e.message, "error"); }
  finally { fixing.value = false; }
}

async function clearCache() {
  fixing.value = true;
  try {
    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import("@tauri-apps/api/core");
      showFix("🗑 " + await invoke("run_fix", { fixAction: "clear_cache" }), "success");
    }
  } catch(e) { showFix("❌ " + e.message, "error"); }
  finally { fixing.value = false; }
}

async function runChatTest() {
  if (!testKey.value.trim()) { showFix("请输入 API Key", "error"); return; }
  chatTesting.value = true;
  chatResult.value = null;
  try {
    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import("@tauri-apps/api/core");
      chatResult.value = await invoke("test_api_call", {
        apiKey: testKey.value.trim(),
        baseUrl: testUrl.value.trim(),
        model: testModel.value
      });
    } else {
      // Web 模式用 fetch
      const r = await fetch(testUrl.value.trim() + "/v1/chat/completions", {
        method: "POST",
        headers: { "Content-Type": "application/json", Authorization: "Bearer " + testKey.value.trim() },
        body: JSON.stringify({ model: testModel.value, messages: [{role:"user",content:"说一个字"}], stream: false, max_tokens: 5 }),
      });
      const data = await r.json();
      if (r.ok) {
        chatResult.value = { success: true, reply: data.choices?.[0]?.message?.content || "(空)", detail: "API调用成功" };
      } else {
        chatResult.value = { success: false, error_code: data.error?.type || String(data.code||r.status), error_msg: data.error?.message||data.msg||"", detail: "请求失败", fix_guide: "请检查Key和Base URL" };
      }
    }
  } catch(e) {
    chatResult.value = { success: false, error_code: "NETWORK_ERROR", error_msg: e.message, detail: "网络错误", fix_guide: "1.检查网络\n2.关闭VPN\n3.检查URL" };
  } finally { chatTesting.value = false; }
}

async function lookupError() {
  if (!errorCodeInput.value.trim()) return;
  errorLooking.value = true;
  errorResult.value = null;
  try {
    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import("@tauri-apps/api/core");
      errorResult.value = await invoke("lookup_error", { code: errorCodeInput.value.trim() });
    } else {
      // Web 模式简单查询
      const codes = {
        "402": { title:"积分不足", cause:"积分用完", guide:"充值后重试" },
        "404": { title:"Key无效", cause:"Key错误或未选自定义模型", guide:"1.检查Key\n2.选自定义模型\n3.重新部署" },
        "11140": { title:"上游风控", cause:"账号被风控", guide:"等几分钟重试" },
      };
      errorResult.value = codes[errorCodeInput.value.trim()] || { title:"未知错误", cause:errorCodeInput.value, guide:"联系客服" };
    }
  } catch(e) { showFix("查询失败: " + e.message, "error"); }
  finally { errorLooking.value = false; }
}

function showFix(msg, type) {
  fixResult.value = { msg, type };
  setTimeout(() => { fixResult.value = null; }, 3000);
}

runDiagnostics();
</script>

<style scoped>
.diag-overlay { position:fixed; top:0; left:0; right:0; bottom:0; background:rgba(0,0,0,.5); z-index:9999; display:flex; align-items:center; justify-content:center; }
.diag-box { background:#fff; border-radius:16px; padding:24px; width:540px; max-width:95vw; max-height:88vh; overflow:auto; box-shadow:0 8px 32px rgba(0,0,0,.15); }
.diag-header { display:flex; justify-content:space-between; align-items:center; margin-bottom:12px; }
.diag-header h2 { color:#2f54eb; font-size:18px; }
.xbtn { border:none; background:none; font-size:16px; cursor:pointer; color:#999; }

.diag-tabs { display:flex; gap:0; margin-bottom:16px; border-bottom:1px solid #eee; }
.diag-tabs button { padding:8px 14px; border:none; border-bottom:2px solid transparent; background:none; cursor:pointer; font-size:13px; color:#999; }
.diag-tabs button.active { color:#2f54eb; border-bottom-color:#2f54eb; font-weight:600; }

.diag-running { text-align:center; padding:40px; }
.spin { font-size:32px; animation:spin 1s linear infinite; display:inline-block; }
@keyframes spin { to{transform:rotate(360deg);} }

.diag-summary { display:flex; gap:12px; align-items:center; margin-bottom:12px; padding:10px; background:#f5f6fa; border-radius:8px; }
.ok-c{color:#52c41a;font-weight:600;} .warn-c{color:#faad14;font-weight:600;} .err-c{color:#ff4d4f;font-weight:600;}
.fix-all { margin-left:auto; padding:6px 14px; border:none; border-radius:8px; background:#2f54eb; color:#fff; cursor:pointer; font-size:12px; }

.diag-list { max-height:300px; overflow-y:auto; }
.diag-item { padding:8px 10px; border-radius:6px; margin-bottom:4px; border-left:3px solid #ddd; }
.diag-item.ok{border-left-color:#52c41a;background:#f6ffed;}
.diag-item.warning{border-left-color:#faad14;background:#fffbe6;}
.diag-item.error{border-left-color:#ff4d4f;background:#fff2f0;}
.di{display:flex;align-items:center;gap:4px;}
.di-icon{font-size:13px;} .di-cat{font-size:10px;color:#999;} .di-title{font-size:12px;font-weight:600;flex:1;}
.fix-btn{padding:2px 8px;border:1px solid #2f54eb;border-radius:6px;background:#fff;color:#2f54eb;cursor:pointer;font-size:10px;}
.di-detail{font-size:11px;color:#666;margin-top:3px;margin-left:17px;}

.diag-actions{display:flex;gap:6px;margin-top:12px;}
.diag-actions button{padding:6px 12px;border:1px solid #ddd;border-radius:8px;background:#fff;cursor:pointer;font-size:11px;}

/* 对话检测 */
.chat-test{padding:8px 0;}
.ct-hint{font-size:13px;color:#666;margin-bottom:12px;}
.ct-form{display:flex;flex-direction:column;gap:6px;margin-bottom:16px;}
.ct-form label{font-size:12px;color:#999;font-weight:600;}
.ct-input{width:100%;height:38px;border:1px solid #ddd;border-radius:8px;padding:0 12px;font-size:14px;outline:none;}
.ct-input:focus{border-color:#2f54eb;}
.ct-btn{height:40px;border:none;border-radius:8px;background:#2f54eb;color:#fff;font-size:14px;cursor:pointer;margin-top:8px;}
.ct-btn:disabled{opacity:.5;}
.ct-result{padding:12px;border-radius:8px;margin-top:12px;}
.ct-result.ok{background:#f6ffed;border:1px solid #b7eb8f;}
.ct-result.fail{background:#fff2f0;border:1px solid #ffa39e;}
.ct-label{font-size:14px;font-weight:700;margin-bottom:6px;}
.ct-reply{font-size:14px;color:#333;}
.ct-errcode{font-size:13px;color:#ff4d4f;margin:4px 0;}
.ct-errmsg{font-size:12px;color:#666;}
.ct-detail{font-size:12px;color:#999;margin-top:4px;}
.ct-guide{margin-top:8px;padding:8px;background:#fff;border-radius:6px;}
.ct-guide-title{font-size:12px;font-weight:600;color:#2f54eb;margin-bottom:4px;}
.ct-guide-text{font-size:12px;color:#555;white-space:pre-wrap;line-height:1.6;}

/* 错误查询 */
.err-lookup{padding:8px 0;}
.el-hint{font-size:13px;color:#666;margin-bottom:12px;}
.el-result{padding:12px;background:#f5f6fa;border-radius:8px;margin-top:12px;}
.el-title{font-size:16px;font-weight:700;color:#2f54eb;margin-bottom:6px;}
.el-cause{font-size:13px;color:#555;margin-bottom:8px;}
.el-guide-title{font-size:12px;font-weight:600;color:#d46b08;margin-bottom:4px;}
.el-guide-text{font-size:12px;color:#555;white-space:pre-wrap;line-height:1.6;}

.fix-toast{position:fixed;bottom:20px;left:50%;transform:translateX(-50%);padding:8px 20px;border-radius:8px;color:#fff;font-size:13px;z-index:99999;}
.fix-toast.success{background:#52c41a;} .fix-toast.error{background:#ff4d4f;}
</style>
