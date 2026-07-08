<template>
  <div class="wizard">
    <div class="wizard-box">
      <div class="wheader">
        <h2>🚀 部署配置</h2>
        <button class="xbtn" @click="$emit('cancel')">✕</button>
      </div>

      <div class="steps">
        <div class="step-dot" :class="{active:step>=0,done:step>0}">1</div>
        <div class="step-line" :class="{done:step>0}"></div>
        <div class="step-dot" :class="{active:step>=1,done:step>1}">2</div>
        <div class="step-line" :class="{done:step>1}"></div>
        <div class="step-dot" :class="{active:step>=2,done:step>2}">3</div>
        <div class="step-line" :class="{done:step>2}"></div>
        <div class="step-dot" :class="{active:step>=3}">4</div>
      </div>
      <div class="step-labels">
        <span :class="{active:step===0}">检测平台</span>
        <span :class="{active:step===1}">选模型</span>
        <span :class="{active:step===2}">推理配置</span>
        <span :class="{active:step===3}">确认部署</span>
      </div>

      <!-- Step 0: 检测平台 -->
      <div v-if="step===0" class="step-content">
        <button v-if="!detectDone" class="detect-btn" @click="detectPlatforms" :disabled="detecting">
          {{ detecting ? '检测中...' : '🔍 检测已安装平台' }}
        </button>
        <div v-if="detectDone" class="plat-grid">
          <div v-for="(p,key) in PLATFORMS" :key="key" class="plat-card"
            :class="{sel:selectedPlatforms.includes(key),dim:!installed[key]?.installed}"
            @click="togglePlatform(key)">
            <div class="plat-icon">{{ p.icon }}</div>
            <div class="plat-name">{{ p.name }}</div>
            <div class="plat-st">
              <span v-if="installed[key]?.installed" style="color:#52c41a">✅ 已安装</span>
              <span v-else style="color:#999">❌ 未安装</span>
            </div>
            <!-- 选中标记 -->
            <div v-if="selectedPlatforms.includes(key)" class="plat-check">✓</div>
            <!-- 未安装显示下载按钮 -->
            <button v-if="!installed[key]?.installed" class="plat-download" @click.stop="downloadPlatform(key)">
              下载
            </button>
          </div>
        </div>
        <div v-if="detectDone && installedCount===0" class="warn">未检测到任何平台，请先安装</div>
      </div>

      <!-- Step 1: 选模型 -->
      <div v-if="step===1" class="step-content">
        <div class="model-toolbar">
          <span>{{ selectedModels.length }}/{{ ALL_MODELS.length }} 个模型</span>
          <button @click="selectAll">全选</button>
          <button @click="selectNone">取消</button>
        </div>
        <div class="model-scroll">
          <div v-for="m in ALL_MODELS" :key="m.id" class="model-row"
            :class="{sel:selectedModels.includes(m.id)}" @click="toggleModel(m.id)">
            <span class="mcheck">{{ selectedModels.includes(m.id) ? '✅' : '⬜' }}</span>
            <span class="mname">{{ m.name }}</span>
            <span class="mdesc">{{ m.desc }}</span>
            <span v-if="m.supportsReasoning" class="mtag r">推理</span>
          </div>
        </div>
      </div>

      <!-- Step 2: 推理配置 -->
      <div v-if="step===2" class="step-content">
        <div class="reason-hint">💡 推理等级支持多选，选中的等级将全部写入平台配置，用户可在软件中自由切换</div>
        <div class="reasoning-grid">
          <div v-for="r in REASONING_LEVELS" :key="r.value" class="reason-card"
            :class="{sel:reasoningLevels.includes(r.value)}" @click="toggleReasoning(r.value)">
            <div class="reason-name">{{ r.label }}</div>
            <div class="reason-cost">{{ r.cost }}</div>
          </div>
        </div>
        <label class="deep-toggle">
          <input type="checkbox" v-model="deepThinking" /> 🧠 深度思考
        </label>
        <div v-if="highestLevel" class="warn-box">
          ⚠️ 最高选中等级: {{ highestLevel.label }}，消耗约 {{ highestLevel.cost }} 倍积分
          <span v-if="reasoningLevels.includes('max')">，这是最高等级！</span>
        </div>
      </div>

      <!-- Step 3: 确认 -->
      <div v-if="step===3" class="step-content">
        <div class="confirm-box">
          <div class="confirm-row"><span>平台</span><b>{{ selectedPlatforms.map(p=>PLATFORMS[p].name).join(', ') }}</b></div>
          <div class="confirm-row"><span>模型数</span><b>{{ selectedModels.length }} 个</b></div>
          <div class="confirm-row"><span>推理等级</span><b>{{ reasoningLevels.map(v=>getLevelName(v)).join(', ') }}</b></div>
          <div class="confirm-row"><span>深度思考</span><b>{{ deepThinking?'✅':'❌' }}</b></div>
        </div>
        <div v-if="reasoningLevels.includes('max')" class="warn-box" style="margin-top:8px">
          🔴 max 等级消耗 12 倍积分！
        </div>
        <button class="deploy-go" @click="doDeploy" :disabled="deploying">
          {{ deploying ? '部署中...' : '确认部署' }}
        </button>
      </div>

      <!-- 结果 -->
      <div v-if="step===4" class="step-content">
        <div class="result-icon">{{ allSuccess ? '✅' : '⚠️' }}</div>
        <div class="result-title">{{ allSuccess ? '部署成功！' : '部分完成' }}</div>
        <div v-for="r in deployResults" :key="r.platform" class="result-row">
          {{ PLATFORMS[r.platform]?.icon }} {{ PLATFORMS[r.platform]?.name }}:
          <span :style="{color:r.success?'#52c41a':'#ff4d4f'}">{{ r.success?'✅ '+r.message:'❌ '+r.error }}</span>
        </div>

        <!-- 重启已部署的软件 -->
        <div v-if="successPlatforms.length > 0" class="restart-section">
          <div class="restart-title">🔄 配置已写入，请重启以下软件使配置生效：</div>
          <div class="restart-buttons">
            <button v-for="p in successPlatforms" :key="p" class="restart-btn" @click="restartApp(p)">
              {{ PLATFORMS[p]?.icon }} 重启 {{ PLATFORMS[p]?.name }}
            </button>
          </div>
        </div>

        <!-- 大提示：手动选择自定义模型 -->
        <div class="big-warning">
          <div class="big-warning-title">⚠️ 重要提示</div>
          <div class="big-warning-content">
            部署完成后，请在软件中 <b>手动选择模型列表中的「自定义模型」</b>！<br>
            如果不手动选择自定义模型，配置将<b>不会生效</b>！
          </div>
        </div>

        <!-- 视频教程 -->
        <div class="video-section">
          <button class="video-btn" @click="showVideo = !showVideo">
            📺 手动配置视频教程
          </button>
          <div v-if="showVideo" class="video-player">
            <video controls style="width:100%;border-radius:8px" src="http://cloud.video.taobao.com/play/u/null/p/1/e/6/t/1/572610762040.mp4"></video>
          </div>
        </div>

        <button class="deploy-go" @click="$emit('done')">进入主界面</button>
      </div>

      <!-- 导航按钮 -->
      <div v-if="step<4" class="nav-btns">
        <button v-if="step>0" @click="step--">上一步</button>
        <button v-if="step<3" class="primary" @click="nextStep" :disabled="!canNext">下一步</button>
      </div>
    </div>

    <!-- Toast -->
    <transition name="fade">
      <div v-if="toast.show" class="diag-toast" :class="toast.type">{{ toast.msg }}</div>
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
const reasoningLevels = ref(["high", "max"]);
const deepThinking = ref(false);
const deploying = ref(false);
const deployResults = ref([]);
const showVideo = ref(false);
const toast = ref({ show: false, msg: "", type: "info" });

function showToast(msg, type="info") { toast.value = { show:true, msg, type }; setTimeout(()=>{toast.value={show:false,msg:"",type:"info"};},3000); }

const installedCount = computed(() => Object.values(installed.value).filter(p => p?.installed).length);
const allSuccess = computed(() => deployResults.value.length > 0 && deployResults.value.every(r => r.success));
const successPlatforms = computed(() => deployResults.value.filter(r => r.success).map(r => r.platform));
const canNext = computed(() => {
  if (step.value === 0) return detectDone.value && selectedPlatforms.value.length > 0;
  if (step.value === 1) return selectedModels.value.length > 0;
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
      installed.value = { opencode:{installed:true}, claudecode:{installed:true}, codebuddy:{installed:true}, workbuddy:{installed:false}, trae:{installed:false} };
    }
    for (const key of Object.keys(PLATFORMS)) {
      if (installed.value[key]?.installed) { selectedPlatforms.value = [key]; break; }
    }
    detectDone.value = true;
  } catch(e) { showToast("检测失败: " + e.message, "error"); }
  finally { detecting.value = false; }
}

function getLevel() { return REASONING_LEVELS.find(r => r.value === reasoningLevels.value[0]); }
function getLevelName(v) { return REASONING_LEVELS.find(r => r.value === v)?.label || v; }

function toggleReasoning(value) {
  const i = reasoningLevels.value.indexOf(value);
  if (i >= 0) {
    if (reasoningLevels.value.length > 1) reasoningLevels.value.splice(i, 1);
  } else {
    reasoningLevels.value.push(value);
  }
}

const highestLevel = computed(() => {
  if (!reasoningLevels.value.length) return null;
  let highest = null;
  let maxIdx = -1;
  for (const v of reasoningLevels.value) {
    const idx = REASONING_LEVELS.findIndex(r => r.value === v);
    if (idx > maxIdx) { maxIdx = idx; highest = REASONING_LEVELS[idx]; }
  }
  return highest;
});

async function doDeploy() {
  deploying.value = true;
  deployResults.value = [];
  try {
    const baseUrl = "https://" + props.serverPlatform + ".2bbb.cn/v1";
    const modelObjs = ALL_MODELS.filter(m => selectedModels.value.includes(m.id));
    for (const p of selectedPlatforms.value) {
      const configs = modelObjs.map(m => buildModelConfig(m, reasoningLevels.value, deepThinking.value));
      const config = buildDeployConfig(p, props.apiKey, baseUrl, modelObjs[0]?.id || "glm-5.2", reasoningLevels.value, deepThinking.value);
      config.model_configs = configs;
      config.selected_model_ids = selectedModels.value;
      try {
        const result = await executeDeploy(config);
        deployResults.value.push({ platform: p, success: true, message: typeof result === "string" ? result : "成功" });
      } catch(e) {
        deployResults.value.push({ platform: p, success: false, error: e.message });
      }
    }
    step.value = 4;
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
.wizard { width:100%; height:100%; background:#f5f6fa; display:flex; align-items:center; justify-content:center; overflow:auto; }
.wizard-box { background:#fff; border-radius:16px; padding:24px; width:520px; max-width:95vw; max-height:95vh; overflow:auto; box-shadow:0 8px 32px rgba(0,0,0,.1); }
.wheader { display:flex; justify-content:space-between; align-items:center; margin-bottom:16px; }
.wheader h2 { color:#2f54eb; font-size:18px; }
.xbtn { border:none; background:none; font-size:16px; cursor:pointer; color:#999; }

/* Steps */
.steps { display:flex; align-items:center; justify-content:center; margin-bottom:4px; }
.step-dot { width:28px; height:28px; border-radius:50%; background:#e0e0e0; color:#999; display:flex; align-items:center; justify-content:center; font-size:12px; font-weight:600; }
.step-dot.active { background:#2f54eb; color:#fff; }
.step-dot.done { background:#52c41a; color:#fff; }
.step-line { width:40px; height:2px; background:#e0e0e0; }
.step-line.done { background:#52c41a; }
.step-labels { display:flex; justify-content:space-around; font-size:11px; color:#999; margin-bottom:16px; }
.step-labels span.active { color:#2f54eb; font-weight:600; }

.step-content { min-height:200px; }
.detect-btn { display:block; margin:20px auto; padding:12px 32px; border:none; border-radius:10px; background:#2f54eb; color:#fff; font-size:15px; cursor:pointer; }

/* Platform grid */
.plat-grid { display:grid; grid-template-columns:repeat(auto-fill,minmax(90px,1fr)); gap:8px; }
.plat-card { border:2px solid #eee; border-radius:10px; padding:10px 4px; text-align:center; cursor:pointer; position:relative; transition:all .15s; }
.plat-card:hover { border-color:#2f54eb; }
.plat-card.sel { border-color:#2f54eb; background:#f0f5ff; box-shadow:0 2px 8px rgba(47,84,235,.15); }
.plat-card.dim { opacity:.6; cursor:not-allowed; }
.plat-check { position:absolute; top:4px; right:6px; width:18px; height:18px; border-radius:50%; background:#2f54eb; color:#fff; font-size:12px; line-height:18px; font-weight:bold; }
.plat-download { margin-top:4px; padding:2px 8px; border:1px solid #ff4d4f; border-radius:6px; background:#fff; color:#ff4d4f; font-size:10px; cursor:pointer; }
.plat-download:hover { background:#fff1f0; }
.plat-icon { font-size:20px; }
.plat-name { font-size:11px; font-weight:600; margin-top:2px; }
.plat-st { font-size:12px; margin-top:2px; }

/* Models */
.model-toolbar { display:flex; align-items:center; gap:8px; margin-bottom:8px; font-size:12px; }
.model-toolbar button { padding:2px 8px; border:1px solid #ddd; border-radius:6px; background:#fff; cursor:pointer; font-size:11px; }
.model-scroll { max-height:250px; overflow-y:auto; }
.model-row { display:flex; align-items:center; gap:6px; padding:6px 8px; border:1px solid #eee; border-radius:8px; margin-bottom:4px; cursor:pointer; }
.model-row:hover { border-color:#2f54eb; }
.model-row.sel { border-color:#2f54eb; background:#f0f5ff; }
.mcheck { font-size:14px; }
.mname { font-size:13px; font-weight:600; min-width:100px; }
.mdesc { font-size:11px; color:#999; flex:1; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }
.mtag { font-size:10px; padding:1px 4px; border-radius:4px; }
.mtag.r { background:#f9f0ff; color:#722ed1; }

/* Reasoning */
.reason-hint { background:#f0f5ff; border-radius:8px; padding:8px 12px; font-size:12px; color:#2f54eb; margin-bottom:10px; }
.reasoning-grid { display:grid; grid-template-columns:repeat(4,1fr); gap:6px; margin-bottom:12px; }
.reason-card { border:2px solid #eee; border-radius:8px; padding:8px 4px; text-align:center; cursor:pointer; }
.reason-card:hover { border-color:#2f54eb; }
.reason-card.sel { border-color:#2f54eb; background:#f0f5ff; }
.reason-name { font-size:13px; font-weight:600; }
.reason-cost { font-size:10px; color:#ff4d4f; }
.deep-toggle { display:flex; align-items:center; gap:6px; font-size:14px; margin-bottom:8px; cursor:pointer; }
.warn-box { background:#fff7e6; border:1px solid #ffd591; border-radius:8px; padding:8px 12px; font-size:12px; color:#d46b08; }

/* Confirm */
.confirm-box { border:1px solid #eee; border-radius:8px; padding:12px; }
.confirm-row { display:flex; justify-content:space-between; padding:4px 0; font-size:13px; }
.confirm-row span { color:#888; }

/* Result */
.result-icon { font-size:40px; text-align:center; }
.result-title { text-align:center; font-size:18px; font-weight:600; margin-bottom:12px; }
.result-row { font-size:12px; padding:2px 0; }

/* Nav */
.nav-btns { display:flex; justify-content:space-between; margin-top:16px; }
.nav-btns button { padding:8px 20px; border:1px solid #ddd; border-radius:8px; background:#fff; cursor:pointer; font-size:13px; }
.nav-btns button.primary { background:#2f54eb; color:#fff; border-color:#2f54eb; }
.nav-btns button:disabled { opacity:.5; }
.deploy-go { display:block; width:100%; padding:12px; border:none; border-radius:10px; background:#2f54eb; color:#fff; font-size:15px; cursor:pointer; margin-top:12px; }
.deploy-go:disabled { opacity:.6; }
.warn { text-align:center; color:#ff4d4f; font-size:13px; margin-top:12px; }

/* Toast */
.diag-toast { position:fixed; top:20px; left:50%; transform:translateX(-50%); padding:10px 24px; border-radius:8px; color:#fff; font-size:14px; z-index:99999; box-shadow:0 4px 12px rgba(0,0,0,.2); }
.diag-toast.info { background:#2f54eb; }
.diag-toast.success { background:#52c41a; }
.diag-toast.error { background:#ff4d4f; }
.fade-enter-active, .fade-leave-active { transition:opacity .3s; }

/* 重启按钮 */
.restart-section { margin-top:16px; padding:12px; background:#f0f5ff; border-radius:8px; }
.restart-title { font-size:13px; color:#2f54eb; margin-bottom:8px; font-weight:600; }
.restart-buttons { display:flex; gap:8px; flex-wrap:wrap; justify-content:center; }
.restart-btn { padding:6px 14px; border:1px solid #2f54eb; border-radius:8px; background:#fff; color:#2f54eb; cursor:pointer; font-size:12px; }
.restart-btn:hover { background:#2f54eb; color:#fff; }

/* 大提示 */
.big-warning { margin-top:16px; padding:16px; background:#fff7e6; border:2px solid #ffd591; border-radius:10px; text-align:center; }
.big-warning-title { font-size:16px; font-weight:700; color:#d46b08; margin-bottom:8px; }
.big-warning-content { font-size:14px; color:#595959; line-height:1.6; }
.big-warning-content b { color:#ff4d4f; }

/* 视频 */
.video-section { margin-top:16px; text-align:center; }
.video-btn { padding:8px 20px; border:1px solid #722ed1; border-radius:8px; background:#fff; color:#722ed1; cursor:pointer; font-size:13px; }
.video-btn:hover { background:#722ed1; color:#fff; }
.video-player { margin-top:12px; }
</style>
