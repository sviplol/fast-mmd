<template>
  <div class="wb-main" :data-theme="isDark ? 'dark' : 'light'">
    <!-- 左侧边栏 -->
    <div class="wb-sidebar">
      <div class="wb-sidebar-header">
        <div class="wb-sidebar-logo">
          <span class="wb-logo-icon">⚡</span>
          <span class="wb-logo-text">AI全自动部署</span>
        </div>
        <div class="wb-sidebar-version">v{{ appVersion }}</div>
      </div>
      
      <button class="wb-deploy-btn" @click="$emit('deploy')">
        <img :src="logoIcon" class="wb-deploy-logo" alt="logo" />
        <span class="wb-deploy-title">一键部署</span>
      </button>
      
      <div class="wb-sidebar-menu">
        <button v-for="t in tabs" :key="t.key" class="wb-menu-item" :class="{active:tab===t.key}" @click="tab=t.key">
          <span class="wb-menu-icon">{{t.icon}}</span>
          <span class="wb-menu-label">{{t.label}}</span>
        </button>
      </div>
      
      <div class="wb-sidebar-footer">
        <div class="wb-balance-card">
          <div class="wb-balance-label">余额({{unit}})</div>
          <div class="wb-balance-value">{{(balance||0).toFixed(2)}}</div>
        </div>
        <button class="wb-sidebar-btn" @click="showRecharge=true"><span class="wb-btn-icon">💰</span>充值</button>
        <button class="wb-sidebar-btn" @click="showDiag=true"><span class="wb-btn-icon">🔧</span>自检</button>
        <button class="wb-sidebar-btn" @click="showGuide=true"><span class="wb-btn-icon">📺</span>教程</button>
        <button class="wb-sidebar-btn" @click="toggleTheme">{{ isDark ? '☀️ 白天' : '🌙 夜晚' }}</button>
        <button class="wb-sidebar-btn danger" @click="$emit('logout')">退出</button>
      </div>
    </div>
    
    <!-- 右侧主内容区 -->
    <div class="wb-content">
      <!-- 顶部栏 -->
      <div class="wb-topbar">
        <div class="wb-topbar-title">{{tabs.find(t=>t.key===tab)?.label}}</div>
        <div class="wb-topbar-actions">
          <button class="wb-topbar-btn" @click="showClear=true">清除部署</button>
          <button class="wb-topbar-btn primary" @click="$emit('deploy')">部署</button>
        </div>
      </div>
      
      <!-- 内容区 -->
      <div class="wb-content-body">
        <!-- 概览 -->
        <div v-if="tab==='overview'" class="wb-page">
          <div class="wb-stats-grid">
            <div class="wb-stat-card">
              <div class="wb-stat-label">余额({{unit}})</div>
              <div class="wb-stat-value green">{{(balance||0).toFixed(2)}}</div>
            </div>
            <div class="wb-stat-card">
              <div class="wb-stat-label">请求次数</div>
              <div class="wb-stat-value">{{usage.total_requests||0}}</div>
            </div>
            <div class="wb-stat-card">
              <div class="wb-stat-label">消耗({{unit}})</div>
              <div class="wb-stat-value red">{{(usage.used||0).toFixed(2)}}</div>
            </div>
            <div class="wb-stat-card">
              <div class="wb-stat-label">剩余({{unit}})</div>
              <div class="wb-stat-value blue">{{remaining.toFixed(0)}}</div>
            </div>
          </div>
          
          <div class="wb-info-card">
            <div class="wb-info-row">
              <span class="wb-info-label">API Key</span>
              <code class="wb-info-value" @click="copy(apiKey)">{{apiKey?apiKey.slice(0,24)+'...':''}}</code>
            </div>
            <div class="wb-info-row">
              <span class="wb-info-label">Base URL</span>
              <code class="wb-info-value" @click="copy(baseUrl)">{{baseUrl}}</code>
            </div>
          </div>
        </div>
        
        <!-- 消费记录 -->
        <div v-if="tab==='usage'" class="wb-page">
          <!-- 卡密倒计时 -->
          <div v-if="usage.quota_records && usage.quota_records.length" class="wb-quota-section">
            <div class="wb-section-title">⏳ 卡密倒计时</div>
            <div v-for="(r,i) in usage.quota_records" :key="'q'+i" class="wb-quota-row" :class="{expired: r.status==='expired'}">
              <span v-if="r.status==='active'" class="q-active">
                {{r.amount}}{{unit}}（剩{{r.remaining}}）
                <span :style="{color: r.days_left<=3?'#f53f3f':r.days_left<=7?'#ff7d00':'#86909c'}">{{r.days_left}}天后到期</span>
                · {{r.expire_date}}
              </span>
              <span v-else class="q-expired">
                ✕ {{r.amount}}{{unit}}（已用{{r.used}}）已到期作废，扣除{{r.deducted}}{{unit}} · {{r.expire_date}}
              </span>
            </div>
          </div>
          
          <div v-if="usage.recharge_items && usage.recharge_items.length" class="wb-recharge-section">
            <div class="wb-section-title">📋 充值记录</div>
            <div v-for="(r,i) in usage.recharge_items" :key="'r'+i" class="wb-recharge-row">
              <span class="r-code" @click="copy(r.code)">{{r.code}}</span>
              <span class="r-amount">+{{Number(r.amount).toFixed(2)}}</span>
              <span class="r-time">{{(r.time||'').replace('T',' ').replace('Z','')}}</span>
            </div>
          </div>
          
          <div v-if="!usage.items||!usage.items.length" class="wb-empty">暂无消费记录</div>
          <div v-else class="wb-usage-list">
            <div class="wb-usage-notice">
              ℹ️ 仅显示最近 20 条消耗记录，历史记录已归档。如扣费异常请核对上方卡密倒计时。
            </div>
            <div v-for="(item,i) in usage.items" :key="i" class="wb-usage-row">
              <span class="u-time">{{(item.created_at||'').replace('T',' ').replace('Z','')}}</span>
              <span class="u-model">{{item.model}}</span>
              <span class="u-cost">{{(item.cost_cny||0).toFixed(4)}}</span>
              <span class="u-tok">{{item.prompt_tokens||0}}+{{item.completion_tokens||0}}</span>
            </div>
          </div>
        </div>
        
        <!-- 充值 -->
        <div v-if="tab==='recharge'" class="wb-page">
          <div class="wb-promo-banner" @click="openShop">
            <div class="wb-promo-title">限时活动：好评送300{{unit}}！</div>
            <div class="wb-promo-desc">购买后带<b>5图好评</b> + 联系客服 → <b>免费领取300{{unit}}卡密一张</b>（每月限一次）</div>
            <div class="wb-promo-btn">点击前往购买</div>
          </div>
          <div class="wb-recharge-card">
            <div class="wb-recharge-title">卡密充值</div>
            <input v-model="rechargeCard" class="wb-input" placeholder="输入新卡号 (5200-XXXX...)" />
            <button class="wb-btn-primary" @click="doRecharge" :disabled="recharging">{{recharging?'充值中...':'充 值'}}</button>
          </div>
        </div>
        
        <!-- 模型 -->
        <div v-if="tab==='models'" class="wb-page">
          <div v-if="!models.length" class="wb-empty">加载中...</div>
          <div v-else class="wb-model-grid">
            <span v-for="m in models" :key="m.id" class="wb-model-tag" @click="copy(m.id)">{{m.id}}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 充值弹窗 -->
    <div v-if="showRecharge" class="wb-modal-overlay" @click.self="showRecharge=false">
      <div class="wb-modal" @click.stop>
        <div class="wb-modal-header">
          <span>卡密充值</span>
          <button class="wb-modal-close" @click="showRecharge=false">✕</button>
        </div>
        <div class="wb-modal-body">
          <div class="wb-mini-promo" @click="openShop">好评送300{{unit}}！5图好评+联系客服→免费领卡密</div>
          <input v-model="rechargeCard" class="wb-input" placeholder="输入卡号 (5200-XXXX...)" />
          <button class="wb-btn-primary" @click="doRecharge" :disabled="recharging">{{recharging?'充值中...':'确认充值'}}</button>
          <button class="wb-btn-secondary" @click="openShop">购买卡号</button>
          <button class="wb-btn-cancel" @click="showRecharge=false">取消</button>
        </div>
      </div>
    </div>

    <!-- 自检弹窗 -->
    <Diagnostics v-if="showDiag" @close="showDiag=false" />

    <!-- 清除部署弹窗 -->
    <div v-if="showClear" class="wb-modal-overlay" @click.self="showClear=false">
      <div class="wb-modal" style="width:400px" @click.stop>
        <div class="wb-modal-header">
          <span>清除部署配置</span>
          <button class="wb-modal-close" @click="showClear=false">✕</button>
        </div>
        <div class="wb-modal-body">
          <p style="font-size:13px;color:var(--wb-text-tertiary);margin-bottom:16px">选择要清除的平台配置：</p>
          <div v-for="(p,key) in clearPlatforms" :key="key" class="wb-clear-row">
            <label>
              <input type="checkbox" v-model="clearPlatforms[key]" />
              {{ platformLabels[key] }}
            </label>
          </div>
          <div style="margin-top:16px">
            <label style="font-size:13px;color:var(--wb-text-tertiary)">推理等级:</label>
            <select v-model="clearReasoning" class="wb-input" style="height:36px;font-size:13px;margin-top:6px">
              <option value="">全部清除</option>
              <option value="max">仅 max</option>
              <option value="high">仅 high</option>
              <option value="medium">仅 medium</option>
            </select>
          </div>
          <button class="wb-btn-primary" style="background:#f53f3f;margin-top:16px" @click="doClearDeploy" :disabled="clearing">{{clearing?'清除中...':'确认清除'}}</button>
          <button class="wb-btn-cancel" @click="showClear=false">取消</button>
        </div>
      </div>
    </div>

    <!-- 教程弹窗 -->
    <div v-if="showGuide" class="wb-modal-overlay" @click.self="showGuide=false">
      <div class="wb-modal guide-modal" @click.stop>
        <div class="wb-modal-header">
          <span>使用说明</span>
          <button class="wb-modal-close" @click="showGuide=false">✕</button>
        </div>
        <div class="guide-tip">5 个教程都在本页，新手建议从"一键部署"开始。</div>
        <div class="vg-grid">
          <div v-for="(g, i) in guideVideos" :key="g.title" class="vg-card" :class="{featured: i===0}">
            <div class="vg-head">
              <span class="vg-step">{{ String(i+1).padStart(2,'0') }}</span>
              <div class="vg-copy">
                <div class="vg-title-row">
                  <h2>{{ g.title }}</h2>
                  <span v-if="g.tag" class="vg-tag">{{ g.tag }}</span>
                </div>
                <p>{{ g.desc }}</p>
              </div>
            </div>
            <video class="vg-player" :src="g.url" controls preload="metadata" playsinline controlslist="nodownload"></video>
          </div>
        </div>
      </div>
    </div>

    <!-- Toast -->
    <transition name="fade">
      <div v-if="toast.show" class="toast" :class="toast.type">{{ toast.msg }}</div>
    </transition>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from "vue";
import { lookup, getModels, redeemCard, openLink } from "../utils/api.js";
import Diagnostics from "./Diagnostics.vue";

const props = defineProps({ apiKey:String, serverPlatform:String, userToken:String, username:String, balance:Number });
const emit = defineEmits(["logout","deploy"]);

const tab = ref("overview");
const balance = ref(props.balance || null);
const usage = ref({});
const models = ref([]);
const showRecharge = ref(false);
const showDiag = ref(false);
const showGuide = ref(false);
const appVersion = ref(12);
const isDark = ref(false);

// 根据中国时间自动切换白天/夜晚（6:00-18:00白天，18:00-6:00夜晚）
function updateThemeByTime() {
  const hour = new Date().getHours();
  const shouldDark = hour >= 18 || hour < 6;
  if (shouldDark !== isDark.value) {
    isDark.value = shouldDark;
    document.documentElement.setAttribute('data-theme', isDark.value ? 'dark' : 'light');
  }
}

function toggleTheme() {
  isDark.value = !isDark.value;
  document.documentElement.setAttribute('data-theme', isDark.value ? 'dark' : 'light');
}

// 启动时自动设置主题
updateThemeByTime();
// 每分钟检查一次时间
setInterval(updateThemeByTime, 60000);

const guideVideos = [
  {
    title: "一键部署",
    tag: "新手必看 · 约 1 分钟",
    desc: "第一次使用先看这里，从部署到可用约 1 分钟。",
    url: "https://cloud.video.taobao.com/vod/NpXS-BJjCgHlZTDafPUrLCsm0TT7Fmn6CwDdzD5Luoc.mp4",
  },
  {
    title: "卡密兑换",
    tag: "",
    desc: "学会把卡号兑换成 fm 开头的密钥，或给已有密钥充值。",
    url: "https://cloud.video.taobao.com/vod/qSaVAc8UI4yNr3eN7-hw1I8IjV5I4uokXW8-Gspuw5E.mp4",
  },
  {
    title: "添加更多模型",
    tag: "",
    desc: "需要使用更多 AI 模型时，看这里完成添加与配置。",
    url: "https://cloud.video.taobao.com/vod/TekZcYGevT5C9Nv48r7KuYTu17WvIZ3PnLJYvTJ0Iek.mp4",
  },
  {
    title: "查询余额、密钥与用量",
    tag: "",
    desc: "学会查询卡号、余额、fm 密钥和每次调用的用量。",
    url: "https://cloud.video.taobao.com/vod/3UKa965CJzhoL-4qwTdMjSO0fEbxbHGvz_qyp8o1_90.mp4",
  },
  {
    title: "余额充值",
    tag: "",
    desc: "余额不足时，按视频步骤快速完成充值。",
    url: "https://cloud.video.taobao.com/vod/p64SNGt42czhEb2sPat_r29-DwMbBcXdB2O8x3_7qAg.mp4",
  },
];
const showQR = ref(false);
const showClear = ref(false);
const rechargeCard = ref("");
const recharging = ref(false);
const clearing = ref(false);
const clearPlatforms = reactive({ opencode:false, claudecode:false, codebuddy:false, workbuddy:false, clawcode:false, trae:false });
const clearReasoning = ref("");
const platformLabels = { opencode:"OpenCode", claudecode:"Claude Code", codebuddy:"CodeBuddy CN", workbuddy:"WorkBuddy", clawcode:"Claw Code", trae:"Trae" };
const toast = reactive({ show:false, msg:"", type:"info" });

// 软件logo图标（引用Tauri图标资源）
const logoIcon = "/icons/32x32.png";

const baseUrl = computed(() => "https://" + props.serverPlatform + ".2bbb.cn/v1");
const remaining = computed(() => usage.value.quota > 0 ? usage.value.quota - (usage.value.used||0) : (usage.value.balance||0));
const unit = computed(() => props.serverPlatform === 'tk' ? 'Token' : '积分');

const tabs = [
  { key:"overview", label:"概览", icon:"📊" },
  { key:"usage", label:"消费记录", icon:"💳" },
  { key:"recharge", label:"充值", icon:"💰" },
  { key:"models", label:"模型列表", icon:"🤖" },
];

function showToast(msg, type) {
  toast.show = true; toast.msg = msg; toast.type = type || "info";
  setTimeout(function() { toast.show = false; }, 3000);
}

function copy(text) {
  navigator.clipboard.writeText(text);
  showToast("已复制", "success");
}

function openShop() {
  openLink("https://e.tb.cn/h.8cuB9YlvDf1ydN9?tk=qNpigtgYuKR");
}

// 卡号过滤：只保留字母数字和-，去掉空格/中文/任何符号
function filterCardInput(card) {
  return card.replace(/[^a-zA-Z0-9-]/g, '');
}

async function loadData() {
  if (!props.apiKey) return;
  try {
    var d = await lookup(props.serverPlatform, props.apiKey);
    if (d.ok) { usage.value = d; balance.value = d.balance; }
    var md = await getModels(props.serverPlatform, props.apiKey);
    if (md.data) models.value = md.data;
  } catch(e) {}
}

async function doRecharge() {
  const raw = rechargeCard.value.trim();
  if (!raw) { showToast("请输入卡号", "error"); return; }
  const cleaned = filterCardInput(raw);
  if (!cleaned) { showToast("卡号格式不正确", "error"); return; }
  recharging.value = true;
  try {
    var r = await redeemCard(props.serverPlatform, cleaned, props.apiKey);
    if (r.ok) {
      const added = r.added !== undefined ? r.added : r.balance;
      showToast("充值成功 +" + Number(added).toFixed(2) + " " + unit.value, "success");
      rechargeCard.value = "";
      showRecharge.value = false;
      loadData();
    } else {
      showToast(r.msg || "充值失败", "error");
    }
  } catch(e) { showToast("网络错误: " + e.message, "error"); }
  finally { recharging.value = false; }
}

onMounted(loadData);

async function doClearDeploy() {
  const selected = Object.keys(clearPlatforms).filter(k => clearPlatforms[k]);
  if (!selected.length) { showToast("请选择要清除的平台", "error"); return; }
  clearing.value = true;
  try {
    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import("@tauri-apps/api/core");
      for (const p of selected) {
        try {
          await invoke("clear_platform_deploy", { platform: p, reasoningLevel: clearReasoning.value });
        } catch(e) { console.error(p, e); }
      }
      showToast(selected.length + " 个平台配置已清除，请重启对应软件", "success");
      showClear.value = false;
      Object.keys(clearPlatforms).forEach(k => clearPlatforms[k] = false);
    } else {
      showToast("请在桌面客户端中清除", "error");
    }
  } catch(e) { showToast("清除失败: " + e.message, "error"); }
  finally { clearing.value = false; }
}
</script>

<style scoped>
/* ===== WorkBuddy 设计系统（白天） ===== */
:root {
  --wb-primary: #00b42a;
  --wb-primary-dark: #009a24;
  --wb-primary-light: #e8f7ea;
  --wb-bg: #f7f8fa;
  --wb-card: #ffffff;
  --wb-text: #1d2129;
  --wb-text-secondary: #4e5969;
  --wb-text-tertiary: #86909c;
  --wb-border: #e5e6eb;
  --wb-radius: 12px;
  --wb-radius-lg: 16px;
  --wb-shadow: 0 2px 8px rgba(0,0,0,.04);
  --wb-shadow-lg: 0 8px 24px rgba(0,0,0,.08);
}

/* ===== CodeBuddy 深色主题（夜晚） ===== */
[data-theme="dark"] {
  --wb-primary: #7b61ff;
  --wb-primary-dark: #6a51e6;
  --wb-primary-light: rgba(123,97,255,.15);
  --wb-bg: #0f1117;
  --wb-card: #1a1d29;
  --wb-text: #e8e9ed;
  --wb-text-secondary: #a0a3b0;
  --wb-text-tertiary: #6b6e7d;
  --wb-border: #2a2d3d;
  --wb-radius: 12px;
  --wb-radius-lg: 16px;
  --wb-shadow: 0 2px 8px rgba(0,0,0,.2);
  --wb-shadow-lg: 0 8px 24px rgba(0,0,0,.3);
}

.wb-main { width:100%; height:100%; display:flex; background:var(--wb-bg); overflow:hidden; }

/* 左侧边栏 */
.wb-sidebar { width:240px; background:var(--wb-card); border-right:1px solid var(--wb-border); display:flex; flex-direction:column; flex-shrink:0; }
.wb-sidebar-header { padding:20px 16px; border-bottom:1px solid var(--wb-border); display:flex; align-items:center; justify-content:space-between; }
.wb-sidebar-logo { display:flex; align-items:center; gap:8px; }
.wb-logo-icon { font-size:24px; }
.wb-logo-text { font-size:18px; font-weight:800; color:var(--wb-text); }
.wb-sidebar-version { font-size:11px; color:var(--wb-text-tertiary); background:var(--wb-bg); padding:2px 8px; border-radius:10px; }

/* 一键部署按钮 - 单行大醒目 */
.wb-deploy-btn { margin:16px; height:56px; border:none; border-radius:var(--wb-radius-lg); background:linear-gradient(135deg,var(--wb-primary) 0%,var(--wb-primary-dark) 100%); color:#fff; cursor:pointer; display:flex; align-items:center; justify-content:center; padding:0 24px; gap:10px; transition:all .25s; box-shadow:0 4px 16px rgba(0,180,42,.3), inset 0 1px 0 rgba(255,255,255,.2); position:relative; overflow:hidden; }
.wb-deploy-btn::before { content:''; position:absolute; top:0; left:-100%; width:100%; height:100%; background:linear-gradient(90deg,transparent,rgba(255,255,255,.2),transparent); transition:left .5s; }
.wb-deploy-btn:hover::before { left:100%; }
.wb-deploy-btn:hover { transform:translateY(-2px); box-shadow:0 8px 24px rgba(0,180,42,.4), inset 0 1px 0 rgba(255,255,255,.2); }
.wb-deploy-btn:active { transform:translateY(0); box-shadow:0 2px 8px rgba(0,180,42,.3); }
.wb-deploy-logo { width:28px; height:28px; border-radius:6px; background:rgba(255,255,255,.15); padding:3px; flex-shrink:0; }
.wb-deploy-title { font-size:18px; font-weight:700; letter-spacing:1px; line-height:1; }

.wb-sidebar-menu { flex:1; padding:0 8px; }
.wb-menu-item { width:100%; height:44px; border:none; border-radius:var(--wb-radius); background:none; cursor:pointer; display:flex; align-items:center; gap:12px; padding:0 16px; margin-bottom:4px; transition:all .2s; color:var(--wb-text-secondary); }
.wb-menu-item:hover { background:var(--wb-bg); color:var(--wb-text); }
.wb-menu-item.active { background:var(--wb-primary-light); color:var(--wb-primary); font-weight:600; }
.wb-menu-icon { font-size:18px; }
.wb-menu-label { font-size:14px; }

.wb-sidebar-footer { padding:16px; border-top:1px solid var(--wb-border); }
.wb-balance-card { background:var(--wb-primary-light); border-radius:var(--wb-radius); padding:12px; margin-bottom:12px; text-align:center; }
.wb-balance-label { font-size:12px; color:var(--wb-text-secondary); margin-bottom:4px; }
.wb-balance-value { font-size:20px; font-weight:700; color:var(--wb-primary); }
.wb-sidebar-btn { width:100%; height:36px; border:1px solid var(--wb-border); border-radius:var(--wb-radius); background:var(--wb-card); color:var(--wb-text-secondary); font-size:13px; cursor:pointer; margin-bottom:6px; transition:all .2s; display:flex; align-items:center; justify-content:center; gap:6px; }
.wb-sidebar-btn:hover { border-color:var(--wb-primary); color:var(--wb-primary); }
.wb-sidebar-btn.danger { color:#f53f3f; }
.wb-sidebar-btn.danger:hover { border-color:#f53f3f; background:#fff2f0; }
[data-theme="dark"] .wb-sidebar-btn.danger:hover { background:rgba(245,63,63,.15); }
.wb-btn-icon { font-size:14px; }

/* 右侧内容区 */
.wb-content { flex:1; display:flex; flex-direction:column; overflow:hidden; }
.wb-topbar { height:56px; background:var(--wb-card); border-bottom:1px solid var(--wb-border); display:flex; align-items:center; justify-content:space-between; padding:0 24px; flex-shrink:0; }
.wb-topbar-title { font-size:18px; font-weight:700; color:var(--wb-text); }
.wb-topbar-actions { display:flex; gap:8px; }
.wb-topbar-btn { padding:6px 16px; border:1px solid var(--wb-border); border-radius:var(--wb-radius); background:var(--wb-card); color:var(--wb-text-secondary); font-size:13px; cursor:pointer; transition:all .2s; }
.wb-topbar-btn:hover { border-color:var(--wb-primary); color:var(--wb-primary); }
.wb-topbar-btn.primary { background:var(--wb-primary); color:#fff; border-color:var(--wb-primary); }
.wb-topbar-btn.primary:hover { background:var(--wb-primary-dark); }

.wb-content-body { flex:1; padding:24px; overflow:auto; }
.wb-page { max-width:800px; }

/* 统计卡片 */
.wb-stats-grid { display:grid; grid-template-columns:repeat(4,1fr); gap:16px; margin-bottom:24px; }
.wb-stat-card { background:var(--wb-card); border-radius:var(--wb-radius); padding:20px; text-align:center; box-shadow:var(--wb-shadow); }
.wb-stat-label { font-size:12px; color:var(--wb-text-tertiary); margin-bottom:8px; }
.wb-stat-value { font-size:24px; font-weight:700; color:var(--wb-text); }
.wb-stat-value.green { color:var(--wb-primary); }
.wb-stat-value.red { color:#f53f3f; }
[data-theme="dark"] .wb-stat-value.red { color:#ff7875; }
.wb-stat-value.blue { color:#165dff; }
[data-theme="dark"] .wb-stat-value.blue { color:#7b61ff; }

/* 信息卡片 */
.wb-info-card { background:var(--wb-card); border-radius:var(--wb-radius); padding:20px; box-shadow:var(--wb-shadow); }
.wb-info-row { display:flex; justify-content:space-between; align-items:center; padding:10px 0; border-bottom:1px solid var(--wb-border); }
.wb-info-row:last-child { border-bottom:none; }
.wb-info-label { font-size:14px; color:var(--wb-text-tertiary); }
.wb-info-value { font-size:13px; color:var(--wb-primary); cursor:pointer; }

/* 卡密倒计时 */
.wb-quota-section { margin-bottom:24px; }
.wb-section-title { font-size:15px; font-weight:600; color:var(--wb-text); margin-bottom:12px; padding-bottom:8px; border-bottom:1px solid var(--wb-border); }
.wb-quota-row { padding:12px 16px; background:var(--wb-card); border-radius:var(--wb-radius); margin-bottom:8px; font-size:14px; border:1px solid var(--wb-border); box-shadow:var(--wb-shadow); }
.wb-quota-row.expired { background:#fff2f0; border-color:#ffccc7; }
[data-theme="dark"] .wb-quota-row.expired { background:rgba(245,63,63,.15); border-color:rgba(245,63,63,.3); }
.q-active { color:var(--wb-text); }
.q-expired { color:#f53f3f; }
[data-theme="dark"] .q-expired { color:#ff7875; }

/* 充值记录 */
.wb-recharge-section { margin-bottom:24px; }
.wb-recharge-row { display:flex; gap:12px; padding:10px 16px; background:#f6ffed; border-radius:var(--wb-radius); margin-bottom:6px; font-size:13px; align-items:center; }
[data-theme="dark"] .wb-recharge-row { background:rgba(0,180,42,.15); }
.r-code { color:var(--wb-text); flex:1; cursor:pointer; }
.r-amount { min-width:70px; font-weight:600; color:var(--wb-primary); }
.r-time { color:var(--wb-text-tertiary); min-width:150px; }

/* 消费记录 */
.wb-usage-list { }
.wb-usage-notice { background:#fff7e6; border:1px solid #ffd591; border-radius:var(--wb-radius); padding:12px 16px; margin-bottom:12px; font-size:13px; color:#d46b08; text-align:center; }
[data-theme="dark"] .wb-usage-notice { background:rgba(255,169,64,.15); border-color:rgba(255,169,64,.3); color:#ffa940; }
.wb-usage-row { display:flex; gap:12px; padding:10px 16px; background:var(--wb-card); border-radius:var(--wb-radius); margin-bottom:6px; font-size:13px; align-items:center; box-shadow:var(--wb-shadow); }
.u-time { color:var(--wb-text-tertiary); min-width:150px; }
.u-model { color:var(--wb-primary); min-width:110px; font-weight:600; }
.u-cost { color:#f53f3f; min-width:70px; }
[data-theme="dark"] .u-cost { color:#ff7875; }
.u-tok { color:var(--wb-text-tertiary); }

/* 充值页面 */
.wb-promo-banner { background:linear-gradient(135deg,#f53f3f,#ff7a45); border-radius:var(--wb-radius-lg); padding:24px; margin-bottom:24px; cursor:pointer; text-align:center; color:#fff; box-shadow:0 4px 16px rgba(245,63,63,.3); transition:all .2s; }
.wb-promo-banner:hover { transform:translateY(-2px); box-shadow:0 8px 24px rgba(245,63,63,.4); }
.wb-promo-title { font-size:20px; font-weight:700; margin-bottom:8px; }
.wb-promo-desc { font-size:14px; opacity:.95; line-height:1.5; }
.wb-promo-desc b { font-weight:700; text-decoration:underline; }
.wb-promo-btn { font-size:15px; font-weight:600; margin-top:12px; }

.wb-recharge-card { background:var(--wb-card); border-radius:var(--wb-radius); padding:24px; box-shadow:var(--wb-shadow); }
.wb-recharge-title { font-size:16px; font-weight:600; margin-bottom:16px; color:var(--wb-text); }

/* 模型列表 */
.wb-model-grid { display:flex; flex-wrap:wrap; gap:10px; }
.wb-model-tag { background:var(--wb-card); padding:8px 16px; border-radius:var(--wb-radius); font-size:14px; color:var(--wb-primary); border:1px solid var(--wb-border); cursor:pointer; transition:all .2s; box-shadow:var(--wb-shadow); }
.wb-model-tag:hover { border-color:var(--wb-primary); transform:translateY(-1px); }

/* 空状态 */
.wb-empty { text-align:center; color:var(--wb-text-tertiary); padding:60px; font-size:15px; }

/* 弹窗 */
.wb-modal-overlay { position:fixed; top:0; left:0; right:0; bottom:0; background:rgba(0,0,0,.4); z-index:99999; display:flex; align-items:center; justify-content:center; backdrop-filter:blur(4px); }
.wb-modal { background:var(--wb-card); border-radius:var(--wb-radius-lg); box-shadow:var(--wb-shadow-lg); overflow:hidden; width:380px; }
.wb-modal-header { display:flex; justify-content:space-between; align-items:center; padding:20px 24px; border-bottom:1px solid var(--wb-border); }
.wb-modal-header span { font-size:17px; font-weight:700; color:var(--wb-text); }
.wb-modal-close { background:none; border:none; font-size:20px; color:var(--wb-text-tertiary); cursor:pointer; width:32px; height:32px; border-radius:8px; display:flex; align-items:center; justify-content:center; transition:all .2s; }
.wb-modal-close:hover { background:var(--wb-bg); color:var(--wb-text); }
.wb-modal-body { padding:24px; }

.wb-mini-promo { background:linear-gradient(135deg,#f53f3f,#ff7a45); border-radius:var(--wb-radius); padding:12px 16px; margin-bottom:16px; text-align:center; color:#fff; font-size:13px; cursor:pointer; font-weight:600; }

.wb-btn-primary { width:100%; height:44px; border:none; border-radius:var(--wb-radius); background:var(--wb-primary); color:#fff; font-size:15px; font-weight:600; cursor:pointer; margin-top:12px; transition:all .2s; }
.wb-btn-primary:hover { background:var(--wb-primary-dark); transform:translateY(-1px); box-shadow:0 4px 12px rgba(0,180,42,.3); }
.wb-btn-primary:disabled { opacity:.6; cursor:default; transform:none; box-shadow:none; }
.wb-btn-secondary { width:100%; height:40px; border:1.5px solid var(--wb-border); border-radius:var(--wb-radius); background:var(--wb-card); color:var(--wb-text-secondary); font-size:14px; cursor:pointer; margin-top:10px; transition:all .2s; }
.wb-btn-secondary:hover { border-color:var(--wb-primary); color:var(--wb-primary); }
.wb-btn-cancel { display:block; margin:12px auto 0; background:none; border:none; color:var(--wb-text-tertiary); cursor:pointer; font-size:14px; }
.wb-btn-cancel:hover { color:var(--wb-text-secondary); }

.wb-input { width:100%; height:44px; border:1.5px solid var(--wb-border); border-radius:var(--wb-radius); padding:0 16px; font-size:15px; outline:none; background:var(--wb-card); color:var(--wb-text); transition:border-color .2s; }
.wb-input::placeholder { color:var(--wb-text-tertiary); }
.wb-input:focus { border-color:var(--wb-primary); }

/* 清除部署 */
.wb-clear-row { padding:8px 0; font-size:14px; }
.wb-clear-row label { cursor:pointer; display:flex; align-items:center; gap:8px; color:var(--wb-text); }
.wb-clear-row input { width:16px; height:16px; accent-color:var(--wb-primary); }

/* 教程弹窗 */
.guide-modal { width:760px; max-width:95vw; max-height:90vh; overflow:auto; padding:0; }
.guide-tip { padding:12px 24px 16px; font-size:13px; color:var(--wb-text-tertiary); }
.vg-grid { padding:0 24px 24px; display:flex; flex-direction:column; gap:16px; }
.vg-card { background:var(--wb-card); border:1px solid var(--wb-border); border-radius:var(--wb-radius); overflow:hidden; }
.vg-card.featured { border-color:#ffccc7; background:#fffafa; }
.vg-head { display:flex; gap:14px; padding:16px 20px 12px; align-items:flex-start; }
.vg-step { font-size:28px; font-weight:800; color:var(--wb-border); line-height:1; min-width:36px; }
.vg-card.featured .vg-step { color:#cf1322; }
.vg-copy { flex:1; min-width:0; }
.vg-title-row { display:flex; align-items:center; gap:8px; flex-wrap:wrap; }
.vg-title-row h2 { margin:0; font-size:17px; font-weight:700; color:var(--wb-text); }
.vg-tag { font-size:11px; color:#cf1322; background:#fff1f0; border:1px solid #ffccc7; border-radius:4px; padding:2px 6px; font-weight:600; }
.vg-copy p { margin:4px 0 0; font-size:13px; color:var(--wb-text-tertiary); line-height:1.5; }
.vg-player { display:block; width:100%; aspect-ratio:16/9; background:#000; border:none; }
.vg-player::-webkit-media-controls-panel { background:rgba(0,0,0,.7); }

/* Toast */
.toast { position:fixed; top:20px; left:50%; transform:translateX(-50%); padding:10px 24px; border-radius:8px; color:#fff; font-size:14px; z-index:99999; box-shadow:var(--wb-shadow-lg); }
.toast.info { background:#165dff; }
.toast.success { background:var(--wb-primary); }
.toast.error { background:#f53f3f; }
.fade-enter-active, .fade-leave-active { transition:opacity .3s; }
.fade-enter-from, .fade-leave-to { opacity:0; }
</style>
