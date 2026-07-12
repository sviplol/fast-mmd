<template>
  <div class="main">
    <div class="mheader">
      <h2>{{ serverPlatform==='tk' ? 'TK Token' : 'Fast MMD' }}</h2>
      <div class="mhdr-right">
        <span v-if="balance!==null" class="bal">{{balance}}</span>
        <button @click="showRecharge=true">充值</button>
        <button @click="showDiag=true">自检</button>
        <button @click="showClear=true">清除部署</button>
        <button class="pri" @click="$emit('deploy')">部署</button>
        <button class="danger" @click="$emit('logout')">退出</button>
      </div>
    </div>
    <div class="mtabs">
      <button v-for="t in tabs" :key="t.key" :class="{active:tab===t.key}" @click="tab=t.key">{{ t.label }}</button>
    </div>
    <div class="mcontent">
      <div v-if="tab==='overview'">
        <div class="stats">
          <div class="stat"><span>余额</span><b style="color:#52c41a">{{(balance||0).toFixed(2)}}</b></div>
          <div class="stat"><span>请求</span><b>{{usage.total_requests||0}}</b></div>
          <div class="stat"><span>消耗</span><b style="color:#ff4d4f">{{(usage.used||0).toFixed(2)}}</b></div>
          <div class="stat"><span>剩余</span><b style="color:#2f54eb">{{remaining.toFixed(0)}}</b></div>
        </div>
        <div class="info-box">
          <div class="info-row"><span>API Key</span><code @click="copy(apiKey)" style="cursor:pointer">{{apiKey?apiKey.slice(0,24)+'...':''}}</code></div>
          <div class="info-row"><span>Base URL</span><code @click="copy(baseUrl)" style="cursor:pointer">{{baseUrl}}</code></div>
        </div>
      </div>
      <div v-if="tab==='usage'">
        <div v-if="!usage.items||!usage.items.length" class="empty">暂无消费记录</div>
        <div v-else class="usage-list">
          <div v-for="(item,i) in usage.items" :key="i" class="usage-row">
            <span class="u-time">{{(item.created_at||'').replace('T',' ').replace('Z','')}}</span>
            <span class="u-model">{{item.model}}</span>
            <span class="u-cost">{{(item.cost_cny||0).toFixed(4)}}</span>
            <span class="u-tok">{{item.prompt_tokens||0}}+{{item.completion_tokens||0}}</span>
          </div>
        </div>
      </div>
      <div v-if="tab==='recharge'">
        <div class="promo-banner" @click="openShop">
          <div class="promo-title">限时活动：好评送500积分！</div>
          <div class="promo-desc">购买后带<b>5图好评</b> + 联系客服 → <b>免费领取500积分卡密一张</b>（每月限一次）</div>
          <div class="promo-btn">点击前往购买</div>
        </div>
        <div class="recharge-section">
          <div class="recharge-title">卡密充值</div>
          <input v-model="rechargeCard" class="big-input" placeholder="输入新卡号 (5200-XXXX...)" />
          <button class="big-btn" @click="doRecharge" :disabled="recharging">{{recharging?'充值中...':'充 值'}}</button>
        </div>
      </div>
      <div v-if="tab==='models'">
        <div v-if="!models.length" class="empty">加载中...</div>
        <div v-else class="model-tags">
          <span v-for="m in models" :key="m.id" class="model-tag" @click="copy(m.id)" style="cursor:pointer">{{m.id}}</span>
        </div>
      </div>
    </div>

    <!-- 充值弹窗 -->
    <div v-if="showRecharge" class="modal-bg" style="z-index:9998" @click.self="showRecharge=false">
      <div class="modal-box" @click.stop>
        <h3>卡密充值</h3>
        <div class="mini-promo" @click="openShop">好评送500积分！5图好评+联系客服→免费领卡密</div>
        <input v-model="rechargeCard" class="big-input" placeholder="输入卡号 (5200-XXXX...)" />
        <button class="big-btn" @click="doRecharge" :disabled="recharging">{{recharging?'充值中...':'确认充值'}}</button>
        <button class="buy-btn" @click="openShop">购买卡号</button>
        <button class="cancel-btn" @click="showRecharge=false">取消</button>
      </div>
    </div>

    <!-- 自检弹窗 -->
    <Diagnostics v-if="showDiag" @close="showDiag=false" />

    <!-- 清除部署弹窗 -->
    <div v-if="showClear" class="modal-bg" style="z-index:9998" @click.self="showClear=false">
      <div class="modal-box" style="width:380px" @click.stop>
        <h3>清除部署配置</h3>
        <p style="font-size:12px;color:#888;margin-bottom:12px">选择要清除的平台配置：</p>
        <div v-for="(p,key) in clearPlatforms" :key="key" class="clear-row">
          <label>
            <input type="checkbox" v-model="clearPlatforms[key]" />
            {{ platformLabels[key] }}
          </label>
        </div>
        <div style="margin-top:12px">
          <label style="font-size:12px;color:#888">推理等级:</label>
          <select v-model="clearReasoning" class="big-input" style="height:32px;font-size:12px">
            <option value="">全部清除</option>
            <option value="max">仅 max</option>
            <option value="high">仅 high</option>
            <option value="medium">仅 medium</option>
          </select>
        </div>
        <button class="big-btn" style="background:#ff4d4f" @click="doClearDeploy" :disabled="clearing">{{clearing?'清除中...':'确认清除'}}</button>
        <button class="cancel-btn" @click="showClear=false">取消</button>
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
const showQR = ref(false);
const showClear = ref(false);
const rechargeCard = ref("");
const recharging = ref(false);
const clearing = ref(false);
const clearPlatforms = reactive({ opencode:false, claudecode:false, codebuddy:false, workbuddy:false, clawcode:false, trae:false });
const clearReasoning = ref("");
const platformLabels = { opencode:"OpenCode", claudecode:"Claude Code", codebuddy:"CodeBuddy CN", workbuddy:"WorkBuddy", clawcode:"Claw Code", trae:"Trae" };
const toast = reactive({ show:false, msg:"", type:"info" });

const baseUrl = computed(() => "https://" + props.serverPlatform + ".2bbb.cn/v1");
const remaining = computed(() => usage.value.quota > 0 ? usage.value.quota - (usage.value.used||0) : (usage.value.balance||0));

const tabs = [
  { key:"overview", label:"概览" },
  { key:"usage", label:"消费" },
  { key:"recharge", label:"充值" },
  { key:"models", label:"模型" },
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
  openLink("https://item.taobao.com/item.htm?ft=t&id=1062470106379");
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
  if (!rechargeCard.value.trim()) { showToast("请输入卡号", "error"); return; }
  recharging.value = true;
  try {
    var r = await redeemCard(props.serverPlatform, rechargeCard.value.trim(), props.apiKey);
    if (r.ok) {
      const added = r.added !== undefined ? r.added : r.balance;
      showToast("充值成功 +" + Number(added).toFixed(2) + " 积分", "success");
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
      // 重置选择
      Object.keys(clearPlatforms).forEach(k => clearPlatforms[k] = false);
    } else {
      showToast("请在桌面客户端中清除", "error");
    }
  } catch(e) { showToast("清除失败: " + e.message, "error"); }
  finally { clearing.value = false; }
}
</script>

<style scoped>
.main { width:100%; height:100%; display:flex; flex-direction:column; background:#f5f6fa; overflow:hidden; }
.mheader { background:#fff; padding:0 16px; height:48px; display:flex; justify-content:space-between; align-items:center; box-shadow:0 1px 4px rgba(0,0,0,.06); flex-shrink:0; }
.mheader h2 { color:#2f54eb; font-size:16px; }
.mhdr-right { display:flex; gap:6px; align-items:center; }
.mhdr-right button { padding:4px 10px; border:1px solid #ddd; border-radius:6px; background:#fff; cursor:pointer; font-size:12px; }
.mhdr-right button.pri { background:#2f54eb; color:#fff; border-color:#2f54eb; }
.mhdr-right button.danger { color:#ff4d4f; }
.bal { font-size:12px; color:#52c41a; font-weight:600; }

.mtabs { display:flex; background:#fff; border-bottom:1px solid #eee; padding:0 16px; flex-shrink:0; }
.mtabs button { padding:8px 16px; border:none; border-bottom:2px solid transparent; background:none; cursor:pointer; font-size:13px; color:#999; }
.mtabs button.active { color:#2f54eb; border-bottom-color:#2f54eb; font-weight:600; }

.mcontent { flex:1; padding:12px 16px; overflow:auto; }
.stats { display:grid; grid-template-columns:repeat(4,1fr); gap:8px; margin-bottom:12px; }
.stat { background:#fff; border-radius:8px; padding:10px; text-align:center; }
.stat span { display:block; font-size:11px; color:#999; margin-bottom:4px; }
.stat b { font-size:18px; }

.info-box { background:#fff; border-radius:8px; padding:12px; }
.info-row { display:flex; justify-content:space-between; align-items:center; padding:6px 0; font-size:13px; border-bottom:1px solid #f5f5f5; }
.info-row span { color:#888; }
.info-row code { font-size:12px; color:#2f54eb; }

.usage-list { }
.usage-row { display:flex; gap:8px; padding:6px 8px; background:#fff; border-radius:6px; margin-bottom:4px; font-size:12px; align-items:center; }
.u-time { color:#999; min-width:140px; }
.u-model { color:#2f54eb; min-width:100px; font-weight:600; }
.u-cost { color:#ff4d4f; min-width:60px; }
.u-tok { color:#999; }

.model-tags { display:flex; flex-wrap:wrap; gap:6px; }
.model-tag { background:#fff; padding:4px 12px; border-radius:6px; font-size:13px; color:#2f54eb; border:1px solid #ddd; }

.empty { text-align:center; color:#999; padding:40px; font-size:14px; }

.big-input { width:100%; height:40px; border:2px solid #ddd; border-radius:8px; padding:0 12px; font-size:14px; margin-bottom:8px; outline:none; user-select:text; -webkit-user-select:text; -webkit-app-region:none; background:#fff; color:#333; }
.big-input:focus { border-color:#2f54eb; }
.big-btn { width:100%; height:40px; border:none; border-radius:8px; background:#2f54eb; color:#fff; font-size:14px; cursor:pointer; }
.cancel-btn { display:block; margin:8px auto 0; background:none; border:none; color:#999; cursor:pointer; font-size:13px; }

.modal-bg { position:fixed; top:0; left:0; right:0; bottom:0; background:rgba(0,0,0,.4); display:flex; align-items:center; justify-content:center; }
.modal-box { background:#fff; border-radius:12px; padding:20px; width:340px; }
.modal-box h3 { margin-bottom:12px; text-align:center; }

.promo-banner { background:linear-gradient(135deg,#ff4d4f,#ff7a45); border-radius:12px; padding:16px; margin-bottom:16px; cursor:pointer; text-align:center; color:#fff; box-shadow:0 4px 16px rgba(255,77,79,.3); }
.promo-banner:hover { transform:translateY(-2px); }
.promo-title { font-size:18px; font-weight:700; margin-bottom:6px; }
.promo-desc { font-size:13px; opacity:.95; line-height:1.5; }
.promo-desc b { font-weight:700; text-decoration:underline; }
.promo-btn { font-size:14px; font-weight:600; margin-top:8px; }

.mini-promo { background:linear-gradient(135deg,#ff4d4f,#ff7a45); border-radius:8px; padding:8px 12px; margin-bottom:12px; text-align:center; color:#fff; font-size:12px; cursor:pointer; font-weight:600; }

.recharge-section { background:#fff; border-radius:12px; padding:16px; }
.recharge-title { font-size:15px; font-weight:600; margin-bottom:10px; color:#333; }

.buy-btn { display:block; width:100%; height:36px; border:1px solid #ff4d4f; border-radius:8px; background:#fff; color:#ff4d4f; font-size:13px; cursor:pointer; margin-top:8px; }
.buy-btn:hover { background:#fff1f0; }
.buy-btn.qr-trigger { border-color:#722ed1; color:#722ed1; }
.buy-btn.qr-trigger:hover { background:#f9f0ff; }

.qr-box { background:#fff; border-radius:16px; padding:24px; text-align:center; width:360px; }
.qr-box h3 { margin-bottom:16px; color:#333; }
.qr-img { max-width:300px; width:100%; height:auto; border-radius:8px; }
.qr-tip { font-size:13px; color:#888; margin:12px 0; }
.qr-link { font-size:13px; color:#2f54eb; cursor:pointer; margin-bottom:12px; text-decoration:underline; }

.toast { position:fixed; top:20px; left:50%; transform:translateX(-50%); padding:10px 24px; border-radius:8px; color:#fff; font-size:14px; z-index:99999; box-shadow:0 4px 12px rgba(0,0,0,.2); }
.toast.info { background:#2f54eb; }
.toast.success { background:#52c41a; }
.toast.error { background:#ff4d4f; }
.fade-enter-active, .fade-leave-active { transition:opacity .3s; }
.fade-enter-from, .fade-leave-to { opacity:0; }

pre { user-select:text; -webkit-user-select:text; }

/* 清除部署 */
.clear-row { padding:6px 0; font-size:13px; }
.clear-row label { cursor:pointer; display:flex; align-items:center; gap:6px; }
.clear-row input { width:16px; height:16px; }
</style>
