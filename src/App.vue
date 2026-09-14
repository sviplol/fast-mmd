<template>
  <div class="app">
    <!-- Toast -->
    <transition name="fade">
      <div v-if="toast.show" class="toast" :class="toast.type">{{ toast.msg }}</div>
    </transition>

    <!-- 阶段1: 卡号激活（默认） -->
    <div v-if="stage==='activate'" class="screen activate-screen">
      <div class="wb-login-card">
        <div class="wb-logo">
          <span class="wb-logo-icon">⚡</span>
          <span class="wb-logo-text">AI全自动部署</span>
        </div>
        <p class="wb-slogan">你的 AI 部署超能力</p>
        <input v-model="cardInput" class="wb-input" placeholder="输入卡号（自动识别GLM/Token站）" @keydown.enter="doActivate" :disabled="loading" />
        <button class="wb-btn-primary" @click="doActivate" :disabled="loading">{{ loading ? '验证中...' : '激 活' }}</button>
        <div class="wb-links">
          <a @click="stage='login'">账号登录</a>
          <span>·</span>
          <a @click="stage='register'">注册账号</a>
          <span>·</span>
          <a @click="openShop">购买卡号</a>
        </div>
        <div class="wb-divider"></div>
        <button class="wb-btn-secondary" @click="doQueryDeploy" :disabled="queryLoading">{{ queryLoading ? '查询中...' : '🔍 部署查询' }}</button>
        <button class="wb-btn-secondary" style="margin-top:10px" @click="showDiag = true">🔧 一键自检</button>
        <button class="wb-btn-guide" @click="showGuide = true">📺 使用教程（视频版）</button>
      </div>
    </div>

    <!-- 阶段2: 账号登录 -->
    <div v-else-if="stage==='login'" class="screen activate-screen">
      <div class="wb-login-card">
        <div class="wb-logo">
          <span class="wb-logo-icon">⚡</span>
          <span class="wb-logo-text">AI全自动部署</span>
        </div>
        <p class="wb-slogan">账号登录</p>
        <input v-model="username" class="wb-input" placeholder="用户名" style="margin-bottom:12px" @keydown.enter="doLogin" />
        <input v-model="password" type="password" class="wb-input" placeholder="密码" style="margin-bottom:12px" @keydown.enter="doLogin" />
        <button class="wb-btn-primary" @click="doLogin" :disabled="loading">{{ loading ? '登录中...' : '登 录' }}</button>
        <div class="wb-links">
          <a @click="stage='register'">注册新账号</a>
          <span>·</span>
          <a @click="stage='activate'">← 卡号激活</a>
        </div>
      </div>
    </div>

    <!-- 阶段2b: 注册 -->
    <div v-else-if="stage==='register'" class="screen activate-screen">
      <div class="wb-login-card">
        <div class="wb-logo">
          <span class="wb-logo-icon">⚡</span>
          <span class="wb-logo-text">AI全自动部署</span>
        </div>
        <p class="wb-slogan">注册新账号</p>
        <input v-model="username" class="wb-input" placeholder="用户名 (3-20位)" style="margin-bottom:12px" />
        <input v-model="password" type="password" class="wb-input" placeholder="密码 (6位以上)" style="margin-bottom:12px" />
        <input v-model="password2" type="password" class="wb-input" placeholder="确认密码" style="margin-bottom:12px" @keydown.enter="doRegister" />
        <button class="wb-btn-primary" @click="doRegister" :disabled="loading">{{ loading ? '注册中...' : '注 册' }}</button>
        <div class="wb-links">
          <a @click="stage='login'">← 已有账号，去登录</a>
          <span>·</span>
          <a @click="stage='activate'">← 卡号激活</a>
        </div>
      </div>
    </div>

    <!-- 阶段3: 一键部署按钮 -->
    <div v-else-if="stage==='ready'" class="screen ready-screen">
      <div class="wb-ready-card">
        <div class="wb-ready-icon">✅</div>
        <h1 class="wb-ready-title">{{ readyTitle }}</h1>
        <p class="wb-ready-balance">余额: <b>{{ balance.toFixed(2) }}</b> {{ platform==='tk' ? 'Token' : '积分' }}</p>
        <button class="wb-btn-deploy" @click="stage='wizard'">🚀 一键部署</button>
        <button class="wb-btn-skip" @click="confirmSkip">跳过，直接进入</button>
      </div>
    </div>

    <!-- 阶段4: 部署向导 -->
    <DeployWizard v-else-if="stage==='wizard'" :api-key="apiKey" :server-platform="platform" @done="onDeployDone" @cancel="stage='ready'" />

    <!-- 阶段5: 主界面 -->
    <MainApp v-else-if="stage==='main'" :api-key="apiKey" :server-platform="platform" :user-token="userToken" :username="username" :balance="balance" @logout="logout" @deploy="stage='wizard'" />

    <!-- 阶段1b: 部署查询结果 -->
    <div v-else-if="stage==='query'" class="screen ready-screen">
      <div class="wb-query-card">
        <h2>🔍 部署查询结果</h2>
        <div v-if="queryResult.installedPlatforms && queryResult.installedPlatforms.length > 0" class="wb-query-section">
          <div class="wb-query-section-title">已安装的平台：</div>
          <div v-for="p in queryResult.installedPlatforms" :key="p.platform" class="wb-query-row">
            <span class="q-icon">{{ p.icon }}</span>
            <span class="q-name">{{ p.name }}</span>
            <span v-if="p.deployed" class="q-status ok">✅ 已部署</span>
            <span v-else class="q-status fail">❌ 未部署</span>
          </div>
        </div>
        <div v-else class="wb-query-empty">未检测到任何已安装的平台<br><a @click="openDownloadLinks" style="color:#00b42a;cursor:pointer">查看平台下载地址</a></div>

        <div v-if="queryResult.apiKey" class="wb-query-section">
          <div class="wb-query-section-title">检测到的 API Key：</div>
          <div class="wb-query-key-box">
            <code>{{ queryResult.apiKey?.slice(0, 30) }}...</code>
            <button class="wb-copy-mini" @click="copyText(queryResult.apiKey)">📋</button>
          </div>
          <div v-if="queryResult.balance !== null && queryResult.balance !== undefined" class="wb-query-balance">
            剩余: <b>{{ platform==='tk' ? Math.floor((Number(queryResult.balance)||0)*15002).toLocaleString() : Number(queryResult.balance).toFixed(2) }}</b> {{ platform==='tk' ? 'Token' : '积分' }}
          </div>
        </div>

        <div v-if="queryResult.needDeploy" class="wb-query-warn">
          ⚠️ 检测到已安装平台但未部署配置<br>请先激活卡号后部署
          <div style="margin-top:8px"><button class="wb-btn-primary" style="width:auto;padding:8px 24px" @click="stage='activate'">去激活</button></div>
        </div>

        <div class="wb-query-actions">
          <button v-if="queryResult.apiKey" class="wb-btn-primary" @click="enterWithKey">使用此 Key 进入</button>
          <button class="wb-btn-back" @click="backFromQuery">← 返回</button>
        </div>
      </div>
    </div>

    <!-- 自检弹窗 -->
    <Diagnostics v-if="showDiag" @close="showDiag = false" />

    <!-- 教程弹窗（未登录也可用） -->
    <div v-if="showGuide" class="wb-modal-overlay" @click.self="showGuide=false">
      <div class="wb-modal guide-modal" @click.stop>
        <div class="wb-modal-header">
          <h3>使用说明</h3>
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

    <!-- 自定义确认弹窗 -->
    <div v-if="confirmDialog.show" class="wb-modal-overlay" @click.self="confirmDialog.onCancel">
      <div class="wb-modal confirm-modal" @click.stop>
        <div class="confirm-title">{{ confirmDialog.title }}</div>
        <div class="confirm-msg">{{ confirmDialog.msg }}</div>
        <div class="confirm-btns">
          <button class="wb-btn-cancel" @click="confirmDialog.onCancel">取消</button>
          <button class="wb-btn-ok" @click="confirmDialog.onOk">确定</button>
        </div>
      </div>
    </div>

    <!-- 卡号不存在弹窗（带购买按钮） -->
    <div v-if="cardNotFoundDialog" class="wb-modal-overlay" @click.self="cardNotFoundDialog=false">
      <div class="wb-modal confirm-modal" @click.stop>
        <div class="confirm-title">卡号不存在</div>
        <div class="confirm-msg">该卡号在 GLM 站和 Token 站均不存在，请检查是否输入正确，或购买新卡密。</div>
        <div class="confirm-btns">
          <button class="wb-btn-cancel" @click="cardNotFoundDialog=false">取消</button>
          <button class="wb-btn-ok" @click="openShop(); cardNotFoundDialog=false">购买卡密</button>
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

    <!-- 更新内容弹窗 -->
    <div v-if="changelogShow" class="wb-modal-overlay">
      <div class="wb-modal changelog-modal">
        <div class="changelog-header">
          <span class="changelog-icon">🎉</span>
          <h2>AI全自动部署 更新到 v{{ appVersion }}</h2>
        </div>
        <div class="changelog-list">
          <div v-for="(item, i) in (CHANGELOG[appVersion] || [])" :key="i" class="changelog-item">
            <span class="changelog-dot">•</span>
            <span>{{ item }}</span>
          </div>
        </div>
        <button class="wb-btn-primary" @click="dismissChangelog">知道了，开始使用</button>
      </div>
    </div>

    <!-- 强制更新弹窗 -->
    <div v-if="updateInfo.show" class="wb-modal-overlay">
      <div class="wb-modal update-modal">
        <div class="update-icon">🔄</div>
        <h2>发现新版本 v{{ updateInfo.latest }}</h2>
        <p class="update-msg">检测到新版本已发布，请下载最新版本使用</p>
        <p class="update-version">当前版本 v{{ updateInfo.current }} → 最新版本 v{{ updateInfo.latest }}</p>
        <button class="wb-btn-primary" @click="goDownload">📥 立即下载新版本</button>
      </div>
    </div>

    <!-- 漂浮客服按钮 -->
    <button class="float-qr-btn" @click="showQR = true" title="联系客服">
      <svg class="float-qr-icon" viewBox="0 0 24 24" fill="currentColor"><path d="M8.691 2.188C3.891 2.188 0 5.476 0 9.53c0 2.212 1.17 4.203 3.002 5.55a.59.59 0 0 1 .213.665l-.39 1.48c-.019.07-.048.141-.048.213 0 .163.13.295.29.295a.326.326 0 0 0 .167-.054l1.903-1.114a.864.864 0 0 1 .717-.098 10.16 10.16 0 0 0 2.837.403c.276 0 .543-.027.811-.05-.857-2.578.157-4.972 1.932-6.446 1.703-1.415 3.882-1.98 5.853-1.838-.576-3.583-4.196-6.348-8.596-6.348zM5.785 5.991c.642 0 1.162.529 1.162 1.18a1.17 1.17 0 0 1-1.162 1.178A1.17 1.17 0 0 1 4.623 7.17c0-.651.52-1.18 1.162-1.18zm5.813 0c.642 0 1.162.529 1.162 1.18a1.17 1.17 0 0 1-1.162 1.178 1.17 1.17 0 0 1-1.162-1.178c0-.651.52-1.18 1.162-1.18zm5.34 2.867c-1.797-.052-3.746.512-5.28 1.786-1.72 1.428-2.687 3.72-1.78 6.22.942 2.453 3.666 4.229 6.884 4.229.826 0 1.622-.12 2.361-.336a.722.722 0 0 1 .598.082l1.584.918a.272.272 0 0 0 .14.047c.134 0 .24-.111.24-.247 0-.06-.023-.12-.038-.177l-.327-1.233a.582.582 0 0 1-.023-.156.49.49 0 0 1 .232-.407C22.9 18.272 24 16.484 24 14.435c0-3.312-3.809-5.457-7.062-5.577zM14.616 11.39c.536 0 .969.44.969.981a.976.976 0 0 1-.969.982.976.976 0 0 1-.969-.982c0-.541.433-.981.97-.981zm4.844 0c.536 0 .969.44.969.981a.976.976 0 0 1-.969.982.976.976 0 0 1-.969-.982c0-.541.433-.981.97-.981z"/></svg>
      <span class="float-qr-text">联系客服</span>
    </button>

    <!-- 客服二维码弹窗 -->
    <div v-if="showQR" class="wb-modal-overlay" @click.self="showQR = false">
      <div class="wb-modal qr-modal" @click.stop>
        <div class="wb-modal-header">
          <span>联系客服</span>
          <button class="wb-modal-close" @click="showQR = false">✕</button>
        </div>
        <div class="qr-modal-body">
          <img :src="qrImg" class="qr-modal-img" alt="客服二维码" />
          <p class="qr-modal-tip">微信扫码添加客服</p>
          <p class="qr-modal-wechat">微信号：zuishuai-cc</p>
        </div>
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
const showGuide = ref(false);
const showQR = ref(false);
const prevStage = ref("activate");
const toast = reactive({ show: false, msg: "", type: "info" });
const confirmDialog = reactive({ show: false, title: "确认", msg: "", onOk: null, onCancel: null });
const cardNotFoundDialog = ref(false);
const updateInfo = reactive({ show: false, current: 0, latest: 0, url: "" });
const appVersion = ref(0);
const checkingUpdate = ref(false);
const changelogShow = ref(false);

// 客服二维码（压缩后200x200 JPEG base64，仅8KB不增加体积）
const qrImg = "https://i.imgs.ovh/2026/09/14/cc3fadd5dda09daf1d512da9eb5dc4d5.png";

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

const CHANGELOG = {
  26: [
    "客服二维码更新 + 微信号改为 zuishuai-cc",
  ],
  25: [
    "修复: 部署后无任何模型的严重问题 — 配置目录解析的引号匹配错误导致部署中断",
    "真实验证: WorkBuddy 运行中部署 → 20个模型写入+热加载稳定 → 重启后模型完整保留",
  ],
  24: [
    "支持 WorkBuddy 热加载：部署时 WorkBuddy 需在后台运行，配置写入后自动生效，无需重启",
    "通过进程命令行自动识别 models.json 存放位置（--user-data-dir 推断配置目录）",
    "修复: 偶发 models.json 变空 [] — 写入后延迟复验，被 WorkBuddy 覆盖时自动重写",
  ],
  23: [
    "修复: 部分用户部署后 models.json 为空的问题 — 写入改为原子操作+读回验证+失败自动重试",
    "修复: 部署前等待 WorkBuddy 进程完全退出(循环确认最多15秒), 防止文件占用导致写入失败",
  ],
  22: [
    "DeepSeek V4 Flash 升级为 V4.1 Flash（上游已支持，旧配置自动兼容）",
  ],
  21: [
    "部署流程极简化：选完平台一键部署（全模型自动配置，无需选模型/推理/默认模型）",
    "模型标识升级为 6b（vendor=品牌标识，20个模型统一格式），三档显示 快速/均衡/极致",
    "固定15个模型全量写入，思考强度5档默认中档，深度思考默认关闭",
  ],
  20: [
    "同步官方三档模型调度：快速/均衡/极致（取代旧Auto模式，官方倍率 x0.21/x0.65/x1.20）",
    "快速档默认选中，三档均支持工具调用/图片/思考强度切换",
  ],
  19: [
    "新增 HY4 Preview 模型（腾讯混元HY4预览版）",
    "修正 6 个模型上下文参数与官方对齐（glm-5.3-flash/minimax-m3/kimi-k2.6/deepseek-v4-flash→128K，v4-pro 输出→128K）",
  ],
  18: [
    "模型名称统一为「选我」品牌（与官方模型不重复，下拉显示 选我:模型名）",
  ],
  17: [
    "新增 GLM-5.3 Flash 模型（智谱快速版，低延迟高性价比）",
  ],
  16: [
    "思考强度5档位（低/中/高/超高/极致），与官方一致全部可选，默认选中「中」档",
    "模型名称前加【选我】标识，快速找到我们的模型",
    "深度思考默认关闭并提示（开启燃烧token，积分费的快）",
  ],
  14: [
    "修复: 首次输入卡密易提示卡密错误 — 深度清洗粘贴内容（自动剥离前后缀文字/空格/不可见字符，小写卡号自动转大写）",
    "修复: Mac版WorkBuddy部署后转圈 — 改用官方 models.json 入口，不再注入 entry 缓存（避免与云端配置冲突）",
    "优化: 网络错误时不再误报「卡号不存在」，兑换中卡号提示等待后重试",
  ],
  13: [
    "TK站Token数值显示修复（1KW卡密显示10000000 Token，不再显示667）",
    "Mac M芯片/英特尔CI runner修复（M芯片电脑不再提示已损坏）",
    "强制更新下载地址统一为 https://glm.2bbb.cn/start/deploy",
  ],
  12: [
    "新增漂浮客服按钮（未登录和主界面右下角）",
    "新增客服二维码弹窗（微信扫码联系）",
    "K3 上下文提升至 1M（全平台部署配置同步生效）",
    "教程弹窗改为视频卡片样式（与网站一致）",
    "未登录页新增教程入口按钮",
    "消费记录区域新增卡密30天倒计时显示",
  ],
  11: [
    "新增 Kimi K3 模型",
    "新增使用教程页面（各平台接入配置+视频教程）",
  ],
  10: [
    "修复双击打开没反应问题（窗口销毁后自动重建主窗口）",
    "修复 Mac 安装提示「已损坏」问题（CI编译后自动清除隔离属性）",
  ],
  9: [
    "彻底修复 WorkBuddy/CodeBuddy 自定义模型无法添加/保存后消失问题",
    "models.json 格式从数组改为对象 {\"models\":[...]} (WorkBuddy Provider只认对象格式)",
    "读取文件时自动去除 UTF-8 BOM 防止 JSON.parse 失败",
    "vendor 统一为 user, tags 为 custom (WorkBuddy getModelsInfo 只认 user)",
    "local_storage 不存在时自动创建目录和空 entry 文件 (不再报错)",
  ],
  8: [
    "修复 WorkBuddy/CodeBuddy 问号消耗倍率图标（加 descriptionZh/credits/官方 reasoning 格式）",
    "WorkBuddy 官方模型保留，不再被覆盖",
    "CodeBuddy 官方模型保留（之前被覆盖只剩 HY3）",
  ],
  7: [
    "修复 WorkBuddy/CodeBuddy 自定义模型保存后消失问题（去掉 useCustomProtocol/aliases 等不兼容字段）",
    "修复 CodeBuddy 部署后官方模型消失问题（保留官方模型，只追加自定义模型）",
    "自定义模型格式完全对齐官方（vendor图标+relatedModels+craft+temperature）",
    "Auto 模型显示名改为「自动模式（智能选择）」",
  ],
  6: [
    "推理等级永久拉满：WorkBuddy/CodeBuddy 全局 reasoningEffort=xhigh + alwaysThinkingEnabled=true",
    "部署时自动写入全局配置文件，无需手动 /config set",
    "全模型 reasoning.available 增加 xhigh 档位",
    "修复 OpenCode reasoningEffort 保持 max 不被降级",
    "修复 WorkBuddy 新版 entry 格式部署（裸JSON+gzip双格式）",
    "修复 isDefault 冲突：清除官方模型默认选中，强制选中我们的模型",
    "全平台 maxInputTokens 对齐后端真实值（GLM-5.2=1M，Auto=1M）",
  ],
  4: [
    "新增 Auto 自动模式：根据任务难度智能分配模型，节省 Token",
    "新增 Claude Code 支持：后端支持 Anthropic /v1/messages 端点",
    "新增自检代理检测：自动检测系统代理并一键修复",
    "新增更新内容弹窗：每次更新首次打开显示新功能",
    "优化免安装版文件命名，更醒目易识别",
    "Mac 版增加已损坏修复说明",
  ],
  3: [
    "新增强制更新机制",
    "新增消费记录和充值记录显示",
    "优化余额显示精度",
  ],
  2: [
    "新增6平台一键部署支持",
    "新增卡号激活和账号登录",
    "新增自检功能",
  ],
};

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

    // 检查是否需要显示更新内容弹窗
    const saved = store.get();
    const lastSeen = saved.lastSeenVersion || 0;
    if (v > lastSeen && CHANGELOG[v]) {
      changelogShow.value = true;
    }

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

function dismissChangelog() {
  changelogShow.value = false;
  const saved = store.get();
  store.set({ ...saved, lastSeenVersion: appVersion.value });
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
  else openLink("https://glm.2bbb.cn/start/deploy");
}

// 卡号格式校验 + 深度清洗
// 规则: 从粘贴文本中提取卡号（数字-大写HEX格式），自动剥离前后缀文字/空白/不可见字符
// 唯一验证: 如果提取出 fm- 开头的密钥，提示客户应输入卡密而非密钥
function validateCard(card) {
  // 1. 剥离不可见字符: 零宽空格/BOM/全角空格/换行/制表符等
  let cleaned = card.replace(/[\u200b\u200c\u200d\uFEFF\u3000\s\x00-\x1f]/g, '');
  // 2. 提取卡号主体: 匹配 "数字段-HEX段" 格式（如 10000-B5BA80D50E75042015281BF61C71ECDF）
  //    前后允许有任意被剥离后的杂字符（中文前缀/后缀文字已在上一步被压缩成相邻字符串）
  const m = cleaned.match(/(\d{1,8})-([A-Fa-f0-9]{16,64})/);
  if (m) {
    cleaned = m[1] + '-' + m[2].toUpperCase();
  } else {
    // 3. 兜底: 无匹配时保持原有过滤逻辑（字母数字和-）
    cleaned = cleaned.replace(/[^a-zA-Z0-9-]/g, '');
    // 混入密钥场景: "卡号 密钥：fm-xxx" 整体过滤后卡号被污染，尝试截断
    const fmIdx = cleaned.toLowerCase().indexOf('fm-');
    if (fmIdx > 0) cleaned = cleaned.substring(0, fmIdx);
  }

  // 如果输入的是 fm- 开头的密钥，提示错误
  if (/^fm-/i.test(cleaned)) {
    return { valid: false, cleaned, isKey: true };
  }

  // 其他情况都接受（不验证前缀格式）
  return { valid: cleaned.length > 0, cleaned, isKey: false };
}

// 显示卡号不存在弹窗（带购买按钮）
function showCardNotFoundDialog() {
  cardNotFoundDialog.value = true;
}

// 卡号激活 — 智能识别 GLM/TK 站
async function doActivate() {
  const raw = cardInput.value.trim();
  if (!raw) { showToast("请输入卡号", "error"); return; }
  // 过滤"卡号："等前缀，验证格式
  const { valid, cleaned, isKey } = validateCard(raw);
  if (isKey) {
    showToast("请输入卡密，而不是 fm- 开头的密钥", "error");
    return;
  }
  if (!valid) {
    showToast("请输入卡号", "error");
    return;
  }
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

    // 2. 智能识别站点：先试 GLM，失败再试 TK
    let r = await redeemCard("glm", cleaned, "");
    let detectedPlatform = "glm";
    let glmError = r.msg || "";

    // GLM 失败时分类处理:
    // - 网络类错误(超时/网络错误) → 不再试TK(两站同一台服务器, 网络问题双站都失败), 直接提示
    // - 明确业务错误(封禁/删除/兑换中/已使用) → 不再试TK(卡号在GLM有记录)
    // - "卡号不存在" → 尝试 TK 站(卡可能在TK站)
    const networkErr = !r.ok && (glmError.includes("网络") || glmError.includes("超时") || glmError.includes("请求超时"));
    const bizErr = !r.ok && (glmError.includes("封禁") || glmError.includes("删除") || glmError.includes("兑换中"));
    if (!r.ok && !networkErr && !bizErr) {
      console.log("GLM站识别失败:", glmError, "，尝试TK站...");
      r = await redeemCard("tk", cleaned, "");
      detectedPlatform = "tk";
    } else if (!r.ok && (networkErr || bizErr)) {
      // 网络/业务错误直接用GLM的错误信息，避免TK"卡号不存在"误导用户
      r = { ok: false, msg: glmError || "网络错误，请稍后重试" };
    }

    if (r.ok) {
      apiKey.value = r.key;
      balance.value = r.balance || 0;
      platform.value = detectedPlatform;
      store.set({ apiKey: r.key, balance: r.balance || 0, platform: detectedPlatform, card: cleaned });
      showToast(`登录成功（${detectedPlatform === 'tk' ? 'Token站' : 'GLM站'}）`, "success");
      stage.value = "ready";
    } else if (r.msg && (r.msg.includes("封禁") || r.msg.includes("删除"))) {
      showToast(r.msg, "error");
    } else if (r.msg && r.msg.includes("已使用")) {
      showToast("此卡号已使用，请用账号登录", "error");
      setTimeout(() => { stage.value = "login"; }, 1500);
    } else if (r.msg && r.msg.includes("兑换中")) {
      // pending锁: 提示等待而非"卡号不存在"
      showToast(r.msg, "error");
    } else if (networkErr) {
      showToast("网络不稳定，请稍等10秒后重试", "error");
    } else {
      // 两个站都不存在：弹出购买卡密按钮
      console.log("GLM站错误:", glmError, "TK站错误:", r.msg);
      cardNotFoundDialog.value = true;
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

function openShop() { openLink("https://e.tb.cn/h.8cuB9YlvDf1ydN9?tk=qNpigtgYuKR"); }

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
</script>\n\n<style>
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

* { margin:0; padding:0; box-sizing:border-box; }
body { font-family:-apple-system,BlinkMacSystemFont,"Segoe UI","PingFang SC","Hiragino Sans GB","Microsoft YaHei",sans-serif; overflow:hidden; user-select:none; }
input, textarea, select { user-select:text; -webkit-user-select:text; -webkit-app-region:none; }
.app { width:100vw; height:100vh; overflow:hidden; background:var(--wb-bg); }
.screen { width:100%; height:100%; display:flex; align-items:center; justify-content:center; }

/* Toast */
.toast { position:fixed; top:20px; left:50%; transform:translateX(-50%); padding:10px 24px; border-radius:8px; color:#fff; font-size:14px; z-index:99999; box-shadow:var(--wb-shadow-lg); }
.toast.info { background:#165dff; }
.toast.success { background:var(--wb-primary); }
.toast.error { background:#f53f3f; }
.fade-enter-active, .fade-leave-active { transition:opacity .3s; }
.fade-enter-from, .fade-leave-to { opacity:0; }

/* ===== 登录/激活页 ===== */
.activate-screen { background:linear-gradient(180deg,#f0f9f1 0%,#e8f5e9 50%,#f7f8fa 100%); }
.wb-login-card { width:400px; background:var(--wb-card); border-radius:var(--wb-radius-lg); padding:40px; box-shadow:var(--wb-shadow-lg); text-align:center; }
.wb-logo { display:flex; align-items:center; justify-content:center; gap:10px; margin-bottom:8px; }
.wb-logo-icon { font-size:32px; }
.wb-logo-text { font-size:28px; font-weight:800; color:var(--wb-text); letter-spacing:-0.5px; }
.wb-slogan { color:var(--wb-text-tertiary); font-size:14px; margin-bottom:28px; }
.wb-input { width:100%; height:48px; border:1.5px solid var(--wb-border); border-radius:var(--wb-radius); padding:0 16px; font-size:15px; outline:none; background:var(--wb-card); color:var(--wb-text); transition:border-color .2s; }
.wb-input::placeholder { color:var(--wb-text-tertiary); }
.wb-input:focus { border-color:var(--wb-primary); }
.wb-input option { color:var(--wb-text); }
.wb-btn-primary { width:100%; height:48px; border:none; border-radius:var(--wb-radius); background:var(--wb-primary); color:#fff; font-size:16px; font-weight:600; cursor:pointer; margin-top:16px; transition:all .2s; }
.wb-btn-primary:hover { background:var(--wb-primary-dark); transform:translateY(-1px); box-shadow:0 4px 12px rgba(0,180,42,.3); }
.wb-btn-primary:disabled { opacity:.6; cursor:default; transform:none; box-shadow:none; }
.wb-btn-secondary { width:100%; height:44px; border:1.5px solid var(--wb-border); border-radius:var(--wb-radius); background:var(--wb-card); color:var(--wb-text-secondary); font-size:14px; cursor:pointer; transition:all .2s; }
.wb-btn-secondary:hover { border-color:var(--wb-primary); color:var(--wb-primary); }
.wb-btn-guide { width:100%; height:44px; margin-top:12px; border:none; border-radius:var(--wb-radius); background:linear-gradient(135deg,#ff7a45,#f53f3f); color:#fff; font-size:14px; font-weight:600; cursor:pointer; transition:all .2s; }
.wb-btn-guide:hover { transform:translateY(-1px); box-shadow:0 4px 12px rgba(245,63,63,.3); }
.wb-links { margin-top:20px; font-size:13px; }
.wb-links a { color:var(--wb-text-tertiary); cursor:pointer; transition:color .2s; }
.wb-links a:hover { color:var(--wb-primary); }
.wb-links span { margin:0 10px; color:var(--wb-border); }
.wb-divider { height:1px; background:var(--wb-border); margin:24px 0; }

/* ===== 就绪页 ===== */
.ready-screen { background:var(--wb-bg); }
.wb-ready-card { text-align:center; background:var(--wb-card); border-radius:var(--wb-radius-lg); padding:48px; box-shadow:var(--wb-shadow-lg); }
.wb-ready-icon { font-size:56px; margin-bottom:16px; }
.wb-ready-title { color:var(--wb-text); font-size:24px; font-weight:700; margin-bottom:12px; }
.wb-ready-balance { font-size:16px; color:var(--wb-text-secondary); margin-bottom:32px; }
.wb-ready-balance b { color:var(--wb-primary); font-size:20px; }
.wb-btn-deploy { width:320px; height:56px; border:none; border-radius:var(--wb-radius); background:var(--wb-primary); color:#fff; font-size:18px; font-weight:600; cursor:pointer; box-shadow:0 4px 16px rgba(0,180,42,.3); transition:all .2s; }
.wb-btn-deploy:hover { background:var(--wb-primary-dark); transform:translateY(-2px); box-shadow:0 8px 24px rgba(0,180,42,.4); }
.wb-btn-skip { display:block; margin:16px auto 0; background:none; border:none; color:var(--wb-text-tertiary); font-size:14px; cursor:pointer; }
.wb-btn-skip:hover { color:var(--wb-text-secondary); }

/* ===== 查询结果页 ===== */
.wb-query-card { width:520px; max-width:95vw; background:var(--wb-card); border-radius:var(--wb-radius-lg); padding:32px; box-shadow:var(--wb-shadow-lg); }
.wb-query-card h2 { color:var(--wb-text); font-size:20px; margin-bottom:20px; text-align:center; }
.wb-query-section { margin-bottom:20px; }
.wb-query-section-title { font-size:14px; font-weight:600; color:var(--wb-text-secondary); margin-bottom:10px; }
.wb-query-row { display:flex; align-items:center; gap:10px; padding:10px 14px; background:var(--wb-bg); border-radius:var(--wb-radius); margin-bottom:6px; }
.q-icon { font-size:18px; }
.q-name { flex:1; font-size:14px; font-weight:600; color:var(--wb-text); }
.q-status { font-size:13px; }
.q-status.ok { color:var(--wb-primary); }
.q-status.fail { color:#f53f3f; }
.wb-query-empty { text-align:center; color:var(--wb-text-tertiary); padding:24px; font-size:14px; }
.wb-query-key-box { display:flex; align-items:center; gap:10px; padding:10px 14px; background:var(--wb-bg); border-radius:var(--wb-radius); }
.wb-query-key-box code { font-size:12px; color:var(--wb-primary); word-break:break-all; flex:1; }
.wb-copy-mini { border:none; background:none; cursor:pointer; font-size:14px; }
.wb-query-balance { margin-top:12px; font-size:16px; text-align:center; color:var(--wb-text); }
.wb-query-balance b { color:var(--wb-primary); font-size:22px; }
.wb-query-warn { background:#fff7e6; border:1px solid #ffd591; border-radius:var(--wb-radius); padding:14px; font-size:13px; color:#d46b08; margin:16px 0; }
.wb-query-actions { margin-top:20px; }
.wb-btn-back { display:block; margin:16px auto 0; background:none; border:none; color:var(--wb-text-tertiary); cursor:pointer; font-size:14px; }
.wb-btn-back:hover { color:var(--wb-text-secondary); }

/* ===== 弹窗通用 ===== */
.wb-modal-overlay { position:fixed; top:0; left:0; right:0; bottom:0; background:rgba(0,0,0,.4); z-index:99999; display:flex; align-items:center; justify-content:center; backdrop-filter:blur(4px); }
.wb-modal { background:var(--wb-card); border-radius:var(--wb-radius-lg); box-shadow:var(--wb-shadow-lg); overflow:hidden; }
.wb-modal-header { display:flex; justify-content:space-between; align-items:center; padding:20px 24px; border-bottom:1px solid var(--wb-border); }
.wb-modal-header h3, .wb-modal-header span { margin:0; font-size:18px; font-weight:700; color:var(--wb-text); }
.wb-modal-close { background:none; border:none; font-size:20px; color:var(--wb-text-tertiary); cursor:pointer; width:32px; height:32px; border-radius:8px; display:flex; align-items:center; justify-content:center; transition:all .2s; }
.wb-modal-close:hover { background:var(--wb-bg); color:var(--wb-text); }
.wb-btn-cancel { flex:1; height:44px; border:1.5px solid var(--wb-border); border-radius:var(--wb-radius); background:var(--wb-card); color:var(--wb-text-secondary); font-size:15px; cursor:pointer; }
.wb-btn-ok { flex:1; height:44px; border:none; border-radius:var(--wb-radius); background:var(--wb-primary); color:#fff; font-size:15px; font-weight:600; cursor:pointer; }
.wb-btn-ok:hover { background:var(--wb-primary-dark); }

/* 确认弹窗 */
.confirm-modal { padding:28px; width:360px; text-align:center; }
.confirm-title { font-size:17px; font-weight:700; color:var(--wb-text); margin-bottom:12px; }
.confirm-msg { font-size:14px; color:var(--wb-text-secondary); line-height:1.6; margin-bottom:24px; }
.confirm-btns { display:flex; gap:12px; }

/* 更新弹窗 */
.update-modal { padding:40px 48px; text-align:center; max-width:420px; }
.update-icon { font-size:56px; margin-bottom:16px; }
.update-modal h2 { color:var(--wb-text); font-size:22px; margin-bottom:12px; }
.update-msg { color:var(--wb-text-secondary); font-size:15px; margin-bottom:8px; }
.update-version { color:var(--wb-text-tertiary); font-size:13px; margin-bottom:28px; }

/* 更新内容弹窗 */
.changelog-modal { padding:36px 44px; width:440px; max-width:90vw; max-height:80vh; overflow-y:auto; }
.changelog-header { text-align:center; margin-bottom:24px; }
.changelog-icon { font-size:44px; display:block; margin-bottom:12px; }
.changelog-header h2 { color:var(--wb-text); font-size:20px; }
.changelog-list { margin-bottom:28px; }
.changelog-item { display:flex; align-items:flex-start; gap:10px; padding:10px 0; font-size:14px; color:var(--wb-text-secondary); line-height:1.5; border-bottom:1px solid var(--wb-border); }
.changelog-item:last-child { border-bottom:none; }
.changelog-dot { color:var(--wb-primary); font-weight:bold; flex-shrink:0; }

/* 版本栏 */
.version-bar { position:fixed; bottom:12px; right:16px; z-index:9999; display:flex; align-items:center; gap:8px; }
.version-text { font-size:12px; color:var(--wb-text-tertiary); }
.version-check-btn { border:1px solid var(--wb-border); background:var(--wb-card); color:var(--wb-text-tertiary); font-size:12px; padding:4px 12px; border-radius:6px; cursor:pointer; transition:all .2s; }
.version-check-btn:hover { border-color:var(--wb-primary); color:var(--wb-primary); }
.version-check-btn:disabled { opacity:.5; cursor:default; }

/* ===== 教程弹窗 ===== */
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

/* ===== 漂浮客服按钮 ===== */
.float-qr-btn { position:fixed; bottom:70px; right:20px; padding:10px 18px; border-radius:24px; background:linear-gradient(135deg,#07c160,#06a050); border:none; cursor:pointer; box-shadow:0 4px 16px rgba(7,193,96,.4); z-index:99998; display:flex; align-items:center; gap:6px; transition:all .2s; color:#fff; }
.float-qr-btn:hover { transform:scale(1.05); box-shadow:0 6px 24px rgba(7,193,96,.5); }
.float-qr-icon { width:20px; height:20px; }
.float-qr-text { font-size:13px; font-weight:600; }

/* 客服二维码弹窗 */
.qr-modal { max-width:90vw; max-height:90vh; display:flex; flex-direction:column; }
.qr-modal-body { padding:20px; text-align:center; overflow:auto; }
.qr-modal-img { max-width:100%; max-height:60vh; border-radius:var(--wb-radius); }
.qr-modal-tip { margin-top:14px; font-size:13px; color:var(--wb-text-tertiary); }

.qr-modal-wechat { margin-top:8px; font-size:15px; font-weight:600; color:#00b42a; user-select:text; cursor:pointer; }

/* ===== 窗口控制按钮 hover 颜色（Tauri Overlay 模式） ===== */
/* 使用 CSS 变量控制 Tauri 窗口按钮颜色 */
:root {
  --tauri-window-button-hover: #e81123;
  --tauri-window-button-close-hover: #e81123;
  --tauri-window-button-minimize-hover: #f0f0f0;
  --tauri-window-button-maximize-hover: #f0f0f0;
}

/* 通过 attribute 选择器覆盖 Tauri 默认按钮样式 */
[data-tauri-window-button] {
  transition: background-color .2s, color .2s;
}
[data-tauri-window-button]:hover {
  background-color: var(--tauri-window-button-hover) !important;
  color: #fff !important;
}
[data-tauri-window-button="close"]:hover {
  background-color: #e81123 !important;
  color: #fff !important;
}
[data-tauri-window-button="minimize"]:hover,
[data-tauri-window-button="maximize"]:hover {
  background-color: rgba(0,0,0,.1) !important;
}

/* 深色主题下的按钮颜色 */
[data-theme="dark"] [data-tauri-window-button="minimize"]:hover,
[data-theme="dark"] [data-tauri-window-button="maximize"]:hover {
  background-color: rgba(255,255,255,.15) !important;
}
</style>