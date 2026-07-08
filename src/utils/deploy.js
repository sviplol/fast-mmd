/**
 * 全平台部署配置
 * 每个平台的配置方式不同：
 * - OpenCode: opencode.global.dat 的 model 字段(JSON字符串) → variant 推理等级
 * - Claude Code: ~/.claude/settings.json 的 env → ANTHROPIC_BASE_URL/AUTH_TOKEN/MODEL + effortLevel
 * - CodeBuddy: state.vscdb 的 CodeBuddy-Product-Cache (DPAPI加密，生成配置文件让用户手动操作)
 * - WorkBuddy: ~/.workbuddy/models.json 直接写入
 * - Trae: state.vscdb (同 CodeBuddy，通过扩展配置)
 */

export const PLATFORMS = {
  opencode: {
    name: "OpenCode",
    icon: "📦",
    desc: "Desktop AI 编程助手",
    color: "#722ed1",
    url: "https://opencode.ai",
  },
  claudecode: {
    name: "Claude Code",
    icon: "🤖",
    desc: "Anthropic 命令行工具",
    color: "#D97757",
    url: "https://claude.ai/code",
  },
  codebuddy: {
    name: "CodeBuddy",
    icon: "💻",
    desc: "腾讯云编程助手",
    color: "#007ACC",
    url: "https://codebuddy.cn",
  },
  workbuddy: {
    name: "WorkBuddy",
    icon: "🔧",
    desc: "独立客户端",
    color: "#2f54eb",
    url: "https://codebuddy.cn",
  },
  trae: {
    name: "Trae",
    icon: "🚀",
    desc: "字节跳动 AI IDE",
    color: "#00D9B2",
    url: "https://trae.cn",
  },
};

export const REASONING_LEVELS = [
  { value: "none", label: "关闭", desc: "不使用推理", cost: "1x" },
  { value: "minimal", label: "极低", desc: "最小推理", cost: "1.5x" },
  { value: "low", label: "低", desc: "轻度推理", cost: "2x" },
  { value: "medium", label: "中", desc: "中度推理", cost: "3x" },
  { value: "high", label: "高", desc: "高度推理", cost: "5x" },
  { value: "xhigh", label: "超高", desc: "深度推理", cost: "8x" },
  { value: "max", label: "最大", desc: "最大推理(最费积分)", cost: "12x" },
];

export function buildDeployConfig(platform, apiKey, baseUrl, model, reasoningLevels, deepThinking) {
  return {
    type: platform,
    api_key: apiKey,
    base_url: baseUrl,
    model: model,
    reasoning_level: Array.isArray(reasoningLevels) ? reasoningLevels[0] : reasoningLevels,
    reasoning_levels: Array.isArray(reasoningLevels) ? reasoningLevels : [reasoningLevels],
    deep_thinking: deepThinking,
    model_configs: [],
    selected_model_ids: [],
  };
}

export async function executeDeploy(config) {
  if (window.__TAURI_INTERNALS__) {
    const { invoke } = await import("@tauri-apps/api/core");
    return await invoke("deploy_to_platform", { config });
  }
  return JSON.stringify({
    platform: config.type,
    models: config.selected_model_ids,
    apiKey: (config.api_key || "").slice(0, 20) + "...",
    baseUrl: config.base_url,
    reasoningLevels: config.reasoning_levels,
    deepThinking: config.deep_thinking,
  }, null, 2);
}
