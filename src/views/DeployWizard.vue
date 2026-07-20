<template>
  <div class="wb-wizard">
    <div class="wb-wizard-box">
      <div class="wb-wizard-header">
        <h2>🚀 部署配置</h2>
        <button class="wb-wizard-close" @click="$emit('cancel')">✕</button>
      </div>

      <!-- 步骤条 -->
      <div class="wb-steps">
        <div class="wb-step" :class="{active:step>=0,done:step>0}">
          <div class="wb-step-dot">1</div>
          <div class="wb-step-label">检测平台</div>
        </div>
        <div class="wb-step-line" :class="{done:step>0}"></div>
        <div class="wb-step" :class="{active:step>=1,done:step>1}">
          <div class="wb-step-dot">2</div>
          <div class="wb-step-label">选模型</div>
        </div>
        <div class="wb-step-line" :class="{done:step>1}"></div>
        <div class="wb-step" :class="{active:step>=2,done:step>2}">
          <div class="wb-step-dot">3</div>
          <div class="wb-step-label">推理配置</div>
        </div>
        <div class="wb-step-line" :class="{done:step>2}"></div>
        <div class="wb-step" :class="{active:step>=3,done:step>3}">
          <div class="wb-step-dot">4</div>
          <div class="wb-step-label">默认模型</div>
        </div>
        <div class="wb-step-line" :class="{done:step>3}"></div>
        <div class="wb-step" :class="{active:step>=4}">
          <div class="wb-step-dot">5</div>
          <div class="wb-step-label">确认部署</div>
        </div>
      </div>

      <!-- Step 0: 检测平台 -->
      <div v-if="step===0" class="wb-step-content">
        <button v-if="!detectDone" class="wb-detect-btn" @click="detectPlatforms" :disabled="detecting">
          {{ detecting ? '检测中...' : '🔍 检测已安装平台' }}
        </button>
        <div v-if="detectDone" class="wb-plat-grid">
          <div v-for="(p,key) in PLATFORMS" :key="key" class="wb-plat-card"
            :class="{sel:selectedPlatforms.includes(key),dim:!installed[key]?.installed}"
            @click="togglePlatform(key)">
            <div class="wb-plat-icon">{{ p.icon }}</div>
            <div class="wb-plat-name">{{ p.name }}</div>
            <div class="wb-plat-st">
              <span v-if="installed[key]?.installed" style="color:#00b42a">✅ 已安装</span>
              <span v-else style="color:#86909c">❌ 未安装</span>
            </div>
            <div v-if="selectedPlatforms.includes(key)" class="wb-plat-check">✓</div>
            <button v-if="!installed[key]?.installed" class="wb-plat-download" @click.stop="downloadPlatform(key)">
              下载
            </button>
          </div>
        </div>
        <div v-if="detectDone && installedCount===0" class="wb-warn">未检测到任何平台，请先安装</div>
      </div>

      <!-- Step 1: 选模型 -->
      <div v-if="step===1" class="wb-step-content">
        <div class="wb-model-toolbar">
          <span>{{ selectedModels.length }}/{{ ALL_MODELS.length }} 个模型</span>
          <button @click="selectAll">全选</button>
          <button @click="selectNone">取消</button>
        </div>
        <div class="wb-model-scroll">
          <div v-for="m in ALL_MODELS" :key="m.id" class="wb-model-row"
            :class="{sel:selectedModels.includes(m.id)}" @click="toggleModel(m.id)">
            <span class="mcheck">{{ selectedModels.includes(m.id) ? '✅' : '⬜' }}</span>
            <span class="mname">{{ m.name }}</span>
            <span class="mdesc">{{ m.desc }}</span>
            <span v-if="m.isAuto" class="mtag auto">Auto</span>
            <span v-else-if="m.supportsReasoning" class="mtag r">推理</span>
          </div>
        </div>
      </div>

      <!-- Step 2: 推理配置 -->
      <div v-if="step===2" class="wb-step-content">
        <div class="wb-reason-hint">选择推理等级（部署后可在平台内切换模型时使用）</div>
        <div class="wb-reasoning-grid">
          <div v-for="r in REASONING_LEVELS" :key="r.value" class="wb-reason-card"
            :class="{sel:reasoningLevel===r.value}" @click="reasoningLevel=r.value">
            <div class="wb-reason-name">{{ r.label }}</div>
            <div class="wb-reason-cost">{{ r.cost }}</div>
          </div>
        </div>
        <label class="wb-deep-toggle">
          <input type="checkbox" v-model="deepThinking" /> 深度思考
        </label>
        <div class="wb-rate-notice">
          💡 百分百1比1倍率抵扣同步 — 各推理等级积分消耗按上游实际倍率1:1同步抵扣，无任何额外加价
        </div>
      </div>

      <!-- Step 3: 选默认模型 -->
      <div v-if="step===3" class="wb-step-content">
        <div class="wb-default-hint">
          选择部署后客户端默认使用的模型<br>
          <span style="color:#86909c;font-size:12px">部署完成后可随时在各平台自定义模型中切换其他模型</span>
        </div>
        <div class="wb-model-scroll">
          <div v-for="m in selectedModelObjs" :key="m.id" class="wb-model-row"
            :class="{sel:defaultModel===m.id}" @click="defaultModel=m.id">
            <span class="mcheck">{{ defaultModel===m.id ? '🔵' : '⚪' }}</span>
            <span class="mname">{{ m.name }}</span>
            <span class="mdesc">{{ m.desc }}</span>
            <span v-if="m.isAuto" class="mtag auto">Auto</span>
            <span v-else-if="m.recommended" class="mtag r">推荐</span>
          </div>
        </div>
      </div>

      <!-- Step 4: 确认 -->
      <div v-if="step===4" class="wb-step-content">
        <div class="wb-confirm-box">
          <div class="wb-confirm-row"><span>平台</span><b>{{ selectedPlatforms.map(p=>PLATFORMS[p].name).join(', ') }}</b></div>
          <div class="wb-confirm-row"><span>模型数</span><b>{{ selectedModels.length }} 个</b></div>
          <div class="wb-confirm-row"><span>默认模型</span><b>{{ ALL_MODELS.find(m=>m.id===defaultModel)?.name || defaultModel }}</b></div>
          <div class="wb-confirm-row"><span>推理等级</span><b>{{ getLevel()?.label }} ({{ getLevel()?.cost }})</b></div>
          <div class="wb-confirm-row"><span>深度思考</span><b>{{ deepThinking?'✅':'❌' }}</b></div>
        </div>
        <button class="wb-deploy-go" @click="doDeploy" :disabled="deploying">
          {{ deploying ? '部署中...' : '确认部署' }}
        </button>
      </div>

      <!-- 结果 -->
      <div v-if="step===5" class="wb-step-content">
        <div class="wb-result-icon">{{ allSuccess ? '✅' : '⚠️' }}</div>
        <div class="wb-result-title">{{ allSuccess ? '部署成功！' : '部分完成' }}</div>
        <div v-for="r in deployResults" :key="r.platform" class="wb-result-row">
          {{ PLATFORMS[r.platform]?.icon }} {{ PLATFORMS[r.platform]?.name }}:
          <span :style="{color:r.success?'#00b42a':'#f53f3f'}">{{ r.success?'✅ '+r.message:'❌ '+r.error }}</span>
        </div>

        <div v-if="successPlatforms.length > 0" class="wb-restart-section">
          <div class="wb-restart-title">🔄 配置已写入，请重启以下软件使配置生效：</div>
          <div class="wb-restart-buttons">
            <button v-for="p in successPlatforms" :key="p" class="wb-restart-btn" @click="restartApp(p)">
              {{ PLATFORMS[p]?.icon }} 重启 {{ PLATFORMS[p]?.name }}
            </button>
          </div>
        </div>

        <div class="wb-big-warning">
          <div class="wb-big-warning-title">部署完成</div>
          <div class="wb-big-warning-content">
            默认模型 <b style="color:#00b42a">{{ ALL_MODELS.find(m=>m.id===defaultModel)?.name }}</b> 已自动配置，重启后即可使用！<br><br>
            <span style="color:#86909c">如需切换其他模型，可在各平台的「自定义模型」中随时更换</span>
          </div>
        </div>

        <div class="wb-video-section">
          <button class="wb-video-btn" @click="showVideo = !showVideo">
            📺 手动配置视频教程
          </button>
          <div v-if="showVideo" class="wb-video-player">
            <video controls style="width:100%;border-radius:8px" src="http://cloud.video.taobao.com/play/u/null/p/1/e/6/t/1/572610762040.mp4"></video>
          </div>
        </div>

        <button class="wb-deploy-go" @click="$emit('done')">进入主界面</button>
      </div>

      <!-- 导航按钮 -->
      <div v-if="step<5" class="wb-nav-btns">
        <button v-if="step>0" class="wb-nav-btn" @click="step--">上一步</button>
        <button v-if="step<4" class="wb-nav-btn primary" @click="nextStep" :disabled="!canNext">下一步</button>
      </div>
    </div>

    <!-- Toast -->
    <transition name="fade">
      <div v-if="toast.show" class="wb-toast" :class="toast.type">{{ toast.msg }}</div>
    </transition>
  </div>
</template>

<script setup>
import { ref, computed } from "vue";
import { PLATFORMS, REASONING_LEVELS, buildDeployConfig, executeDeploy } from "../utils/deploy.js";
import { ALL_MODELS, buildModelConfig } from "../utils/models.js";
import { openLink } from "../utils/api.js";

const props = defineProps({ apiKey: String, serverPlatform: String });
const emit = defineEmits(["done", "cancel"]);

const step = ref(0);
const detecting = ref(false);
const detectDone = ref(false);
const installed = ref({});
const selectedPlatforms = ref([]);
const selectedModels = ref(ALL_MODELS.map(m => m.id));
const defaultModel = ref("auto");
const reasoningLevel = ref("max");
const deepThinking = ref(true);
const deploying = ref(false);
const deployResults = ref([]);
const showVideo = ref(false);
const toast = ref({ show: false, msg: "", type: "info" });

function showToast(msg, type="info") { toast.value = { show:true, msg, type }; setTimeout(()=>{toast.value={show:false,msg:"",type:"info"};},3000); }

const installedCount = computed(() => Object.values(installed.value).filter(p => p?.installed).length);
const selectedModelObjs = computed(() => ALL_MODELS.filter(m => selectedModels.value.includes(m.id)));
const allSuccess = computed(() => deployResults.value.length > 0 && deployResults.value.every(r => r.success));
const successPlatforms = computed(() => deployResults.value.filter(r => r.success).map(r => r.platform));
const canNext = computed(() => {
  if (step.value === 0) return detectDone.value && selectedPlatforms.value.length > 0;
  if (step.value === 1) return selectedModels.value.length > 0;
  if (step.value === 3) return !!defaultModel.value;
  return true;
});

function nextStep() { if (canNext.value) step.value++; }

function togglePlatform(key) {
  if (!installed.value[key]?.installed) return;
  const i = selectedPlatforms.value.indexOf(key);
  if (i >= 0) selectedPlatforms.value.splice(i, 1);
  else selectedPlatforms.value.push(key);
}

function downloadPlatform(key) {
  const url = PLATFORMS[key]?.url;
  if (url) openLink(url);
}

function toggleModel(id) {
  const i = selectedModels.value.indexOf(id);
  if (i >= 0) selectedModels.value.splice(i, 1);
  else selectedModels.value.push(id);
  if (!selectedModels.value.includes(defaultModel.value) && selectedModels.value.length > 0) {
    defaultModel.value = selectedModels.value[0];
  }
}

function selectAll() { selectedModels.value = ALL_MODELS.map(m => m.id); }
function selectNone() { selectedModels.value = []; }

async function detectPlatforms() {
  detecting.value = true;
  try {
    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import("@tauri-apps/api/core");
      installed.value = await invoke("detect_all_platforms");
    } else {
      installed.value = {};
    }
    if (installed.value["workbuddy"]?.installed) {
      selectedPlatforms.value = ["workbuddy"];
    } else {
      for (const key of Object.keys(PLATFORMS)) {
        if (installed.value[key]?.installed) { selectedPlatforms.value = [key]; break; }
      }
    }
    detectDone.value = true;
  } catch(e) { showToast("检测失败: " + e.message, "error"); }
  finally { detecting.value = false; }
}

function getLevel() { return REASONING_LEVELS.find(r => r.value === reasoningLevel.value); }

async function doDeploy() {
  deploying.value = true;
  deployResults.value = [];
  try {
    const baseUrl = "https://" + props.serverPlatform + ".2bbb.cn";
    const orderedIds = [defaultModel.value, ...selectedModels.value.filter(id => id !== defaultModel.value)];
    const modelObjs = orderedIds.map(id => ALL_MODELS.find(m => m.id === id)).filter(Boolean);
    for (const p of selectedPlatforms.value) {
      const configs = modelObjs.map(m => buildModelConfig(m, reasoningLevel.value, deepThinking.value));
      const config = buildDeployConfig(p, props.apiKey, baseUrl, defaultModel.value, reasoningLevel.value, deepThinking.value);
      config.model_configs = configs;
      config.selected_model_ids = orderedIds;
      try {
        const result = await executeDeploy(config);
        deployResults.value.push({ platform: p, success: true, message: typeof result === "string" ? result : "成功" });
      } catch(e) {
        deployResults.value.push({ platform: p, success: false, error: e.message });
      }
    }
    step.value = 5;
  } catch(e) { showToast("失败: " + e.message, "error"); }
  finally { deploying.value = false; }
}

async function restartApp(platformKey) {
  try {
    if (window.__TAURI_INTERNALS__) {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("restart_app", { platform: platformKey });
      showToast(PLATFORMS[platformKey]?.name + " 已尝试重启", "success");
    } else {
      showToast("请在桌面手动重启 " + PLATFORMS[platformKey]?.name, "info");
    }
  } catch(e) {
    showToast("重启失败: " + e.message + "，请手动重启", "error");
  }
}
</script>

<style scoped>
/* ===== WorkBuddy 设计系统 ===== */
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

.wb-wizard { width:100%; height:100%; background:var(--wb-bg); display:flex; align-items:center; justify-content:center; overflow:auto; }
.wb-wizard-box { background:var(--wb-card); border-radius:var(--wb-radius-lg); padding:32px; width:560px; max-width:95vw; max-height:95vh; overflow:auto; box-shadow:var(--wb-shadow-lg); }
.wb-wizard-header { display:flex; justify-content:space-between; align-items:center; margin-bottom:24px; }
.wb-wizard-header h2 { color:var(--wb-text); font-size:20px; font-weight:700; }
.wb-wizard-close { border:none; background:none; font-size:18px; cursor:pointer; color:var(--wb-text-tertiary); width:32px; height:32px; border-radius:8px; display:flex; align-items:center; justify-content:center; transition:all .2s; }
.wb-wizard-close:hover { background:var(--wb-bg); color:var(--wb-text); }

/* 步骤条 */
.wb-steps { display:flex; align-items:center; justify-content:center; margin-bottom:24px; }
.wb-step { display:flex; flex-direction:column; align-items:center; gap:6px; }
.wb-step-dot { width:32px; height:32px; border-radius:50%; background:var(--wb-border); color:var(--wb-text-tertiary); display:flex; align-items:center; justify-content:center; font-size:14px; font-weight:600; transition:all .2s; }
.wb-step.active .wb-step-dot { background:var(--wb-primary); color:#fff; }
.wb-step.done .wb-step-dot { background:var(--wb-primary); color:#fff; }
.wb-step-label { font-size:11px; color:var(--wb-text-tertiary); }
.wb-step.active .wb-step-label { color:var(--wb-primary); font-weight:600; }
.wb-step-line { width:48px; height:2px; background:var(--wb-border); margin:0 8px; margin-bottom:20px; }
.wb-step-line.done { background:var(--wb-primary); }

.wb-step-content { min-height:220px; }
.wb-detect-btn { display:block; margin:24px auto; padding:14px 36px; border:none; border-radius:var(--wb-radius); background:var(--wb-primary); color:#fff; font-size:16px; font-weight:600; cursor:pointer; transition:all .2s; }
.wb-detect-btn:hover { background:var(--wb-primary-dark); transform:translateY(-1px); box-shadow:0 4px 12px rgba(0,180,42,.3); }
.wb-detect-btn:disabled { opacity:.6; cursor:default; }

/* 平台选择 */
.wb-plat-grid { display:grid; grid-template-columns:repeat(auto-fill,minmax(100px,1fr)); gap:10px; }
.wb-plat-card { border:2px solid var(--wb-border); border-radius:var(--wb-radius); padding:14px 8px; text-align:center; cursor:pointer; position:relative; transition:all .2s; background:var(--wb-card); }
.wb-plat-card:hover { border-color:var(--wb-primary); }
.wb-plat-card.sel { border-color:var(--wb-primary); background:var(--wb-primary-light); box-shadow:0 2px 8px rgba(0,180,42,.15); }
.wb-plat-card.dim { opacity:.5; cursor:not-allowed; }
.wb-plat-check { position:absolute; top:6px; right:8px; width:20px; height:20px; border-radius:50%; background:var(--wb-primary); color:#fff; font-size:12px; line-height:20px; font-weight:bold; }
.wb-plat-download { margin-top:6px; padding:3px 10px; border:1px solid #f53f3f; border-radius:6px; background:#fff; color:#f53f3f; font-size:11px; cursor:pointer; }
.wb-plat-download:hover { background:#fff2f0; }
.wb-plat-icon { font-size:22px; }
.wb-plat-name { font-size:12px; font-weight:600; margin-top:4px; color:var(--wb-text); }
.wb-plat-st { font-size:12px; margin-top:2px; }

/* 模型选择 */
.wb-model-toolbar { display:flex; align-items:center; gap:10px; margin-bottom:12px; font-size:13px; color:var(--wb-text-secondary); }
.wb-model-toolbar button { padding:4px 12px; border:1px solid var(--wb-border); border-radius:6px; background:var(--wb-card); cursor:pointer; font-size:12px; color:var(--wb-text-secondary); }
.wb-model-toolbar button:hover { border-color:var(--wb-primary); color:var(--wb-primary); }
.wb-model-scroll { max-height:280px; overflow-y:auto; }
.wb-model-row { display:flex; align-items:center; gap:10px; padding:10px 12px; border:1px solid var(--wb-border); border-radius:var(--wb-radius); margin-bottom:6px; cursor:pointer; transition:all .2s; background:var(--wb-card); }
.wb-model-row:hover { border-color:var(--wb-primary); }
.wb-model-row.sel { border-color:var(--wb-primary); background:var(--wb-primary-light); }
.mcheck { font-size:15px; }
.mname { font-size:14px; font-weight:600; min-width:110px; color:var(--wb-text); }
.mdesc { font-size:12px; color:var(--wb-text-tertiary); flex:1; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }
.mtag { font-size:11px; padding:2px 6px; border-radius:4px; }
.mtag.r { background:#f9f0ff; color:#722ed1; }
.mtag.auto { background:#e6fffb; color:#0f8b8d; font-weight:600; }

/* 推理配置 */
.wb-reason-hint { background:var(--wb-primary-light); border-radius:var(--wb-radius); padding:12px 16px; font-size:13px; color:var(--wb-primary); margin-bottom:14px; }
.wb-default-hint { background:var(--wb-primary-light); border-radius:var(--wb-radius); padding:14px 16px; font-size:14px; color:var(--wb-primary); margin-bottom:14px; line-height:1.5; }
.wb-reasoning-grid { display:grid; grid-template-columns:repeat(4,1fr); gap:8px; margin-bottom:14px; }
.wb-reason-card { border:2px solid var(--wb-border); border-radius:var(--wb-radius); padding:12px 8px; text-align:center; cursor:pointer; transition:all .2s; background:var(--wb-card); }
.wb-reason-card:hover { border-color:var(--wb-primary); }
.wb-reason-card.sel { border-color:var(--wb-primary); background:var(--wb-primary-light); }
.wb-reason-name { font-size:14px; font-weight:600; color:var(--wb-text); }
.wb-reason-cost { font-size:11px; color:#f53f3f; margin-top:2px; }
.wb-deep-toggle { display:flex; align-items:center; gap:8px; font-size:15px; margin-bottom:12px; cursor:pointer; color:var(--wb-text); }
.wb-deep-toggle input { width:16px; height:16px; accent-color:var(--wb-primary); }
.wb-rate-notice { background:#f6ffed; border:1px solid #b7eb8f; border-radius:var(--wb-radius); padding:14px 16px; font-size:14px; color:#389e0d; margin-top:10px; line-height:1.5; }

/* 确认 */
.wb-confirm-box { border:1px solid var(--wb-border); border-radius:var(--wb-radius); padding:16px; margin-bottom:16px; }
.wb-confirm-row { display:flex; justify-content:space-between; padding:8px 0; font-size:14px; border-bottom:1px solid var(--wb-border); }
.wb-confirm-row:last-child { border-bottom:none; }
.wb-confirm-row span { color:var(--wb-text-tertiary); }
.wb-confirm-row b { color:var(--wb-text); }
.wb-deploy-go { display:block; width:100%; padding:14px; border:none; border-radius:var(--wb-radius); background:var(--wb-primary); color:#fff; font-size:16px; font-weight:600; cursor:pointer; margin-top:14px; transition:all .2s; }
.wb-deploy-go:hover { background:var(--wb-primary-dark); transform:translateY(-1px); box-shadow:0 4px 12px rgba(0,180,42,.3); }
.wb-deploy-go:disabled { opacity:.6; cursor:default; }

/* 结果 */
.wb-result-icon { font-size:48px; text-align:center; }
.wb-result-title { text-align:center; font-size:20px; font-weight:600; margin-bottom:16px; color:var(--wb-text); }
.wb-result-row { font-size:13px; padding:6px 0; color:var(--wb-text-secondary); }

.wb-restart-section { margin-top:20px; padding:16px; background:var(--wb-primary-light); border-radius:var(--wb-radius); }
.wb-restart-title { font-size:14px; color:var(--wb-primary); margin-bottom:12px; font-weight:600; }
.wb-restart-buttons { display:flex; gap:10px; flex-wrap:wrap; justify-content:center; }
.wb-restart-btn { padding:8px 18px; border:1px solid var(--wb-primary); border-radius:var(--wb-radius); background:#fff; color:var(--wb-primary); cursor:pointer; font-size:13px; transition:all .2s; }
.wb-restart-btn:hover { background:var(--wb-primary); color:#fff; }

.wb-big-warning { margin-top:20px; padding:20px; background:#fff7e6; border:2px solid #ffd591; border-radius:var(--wb-radius); text-align:center; }
.wb-big-warning-title { font-size:17px; font-weight:700; color:#d46b08; margin-bottom:10px; }
.wb-big-warning-content { font-size:15px; color:#595959; line-height:1.6; }
.wb-big-warning-content b { color:#f53f3f; }

.wb-video-section { margin-top:20px; text-align:center; }
.wb-video-btn { padding:10px 24px; border:1px solid #722ed1; border-radius:var(--wb-radius); background:#fff; color:#722ed1; cursor:pointer; font-size:14px; transition:all .2s; }
.wb-video-btn:hover { background:#722ed1; color:#fff; }
.wb-video-player { margin-top:16px; }

/* 导航 */
.wb-nav-btns { display:flex; justify-content:space-between; margin-top:20px; }
.wb-nav-btn { padding:10px 24px; border:1.5px solid var(--wb-border); border-radius:var(--wb-radius); background:var(--wb-card); cursor:pointer; font-size:14px; color:var(--wb-text-secondary); transition:all .2s; }
.wb-nav-btn:hover { border-color:var(--wb-primary); color:var(--wb-primary); }
.wb-nav-btn.primary { background:var(--wb-primary); color:#fff; border-color:var(--wb-primary); }
.wb-nav-btn.primary:hover { background:var(--wb-primary-dark); }
.wb-nav-btn:disabled { opacity:.5; cursor:default; }

.wb-warn { text-align:center; color:#f53f3f; font-size:14px; margin-top:16px; }

/* Toast */
.wb-toast { position:fixed; top:20px; left:50%; transform:translateX(-50%); padding:10px 24px; border-radius:8px; color:#fff; font-size:14px; z-index:99999; box-shadow:var(--wb-shadow-lg); }
.wb-toast.info { background:#165dff; }
.wb-toast.success { background:var(--wb-primary); }
.wb-toast.error { background:#f53f3f; }
.fade-enter-active, .fade-leave-active { transition:opacity .3s; }
</style>
