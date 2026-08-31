/**
 * 上游支持的全部模型
 * supportsImages 全部设为 true，避免用户发图时报错
 */

export const ALL_MODELS = [
  { id:"auto", name:"Auto", desc:"自动模式，根据任务难度智能分配模型，节省Token", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:1000000, maxOutputTokens:131072, defaultReasoning:"high", recommended:true, isAuto:true, supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:false },
  { id:"hy3-preview", name:"HY3 Preview", desc:"HY3 预览版", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:false, maxInputTokens:192000, maxOutputTokens:128000, defaultReasoning:"medium", supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:false },
  { id:"hy4-preview", name:"HY4 Preview", desc:"腾讯混元HY4预览版", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:false, maxInputTokens:128000, maxOutputTokens:8192, defaultReasoning:"medium", supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:false },
  { id:"glm-5.3", name:"GLM-5.3", desc:"智谱最新旗舰，推理+视觉+工具调用", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:1000000, maxOutputTokens:131072, defaultReasoning:"high", recommended:true, supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:true },
  { id:"glm-5.3-flash", name:"GLM-5.3 Flash", desc:"智谱快速版，低延迟高性价比", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:128000, maxOutputTokens:8192, defaultReasoning:"medium", supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:true },
  { id:"glm-5.2", name:"GLM-5.2", desc:"智谱旗舰，推理+视觉+工具调用", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:1000000, maxOutputTokens:131072, defaultReasoning:"high", supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:true },
  { id:"glm-5.1", name:"GLM-5.1", desc:"智谱上一代旗舰", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:1000000, maxOutputTokens:131072, defaultReasoning:"high", supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:true },
  { id:"glm-5v-turbo", name:"GLM-5V Turbo", desc:"视觉模型，支持图片理解", type:"vision", supportsReasoning:false, supportsImages:true, supportsToolCall:false, maxInputTokens:1000000, maxOutputTokens:131072, defaultReasoning:"none" },
  { id:"minimax-m3", name:"MiniMax M3", desc:"MiniMax 最新版", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:true, maxInputTokens:1000000, maxOutputTokens:524288, defaultReasoning:"none" },
  { id:"kimi-k3", name:"Kimi K3", desc:"月之暗面 Kimi 最新旗舰", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:1000000, maxOutputTokens:131072, defaultReasoning:"high", recommended:true, supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:true },
  { id:"kimi-k2.7", name:"Kimi K2.7", desc:"月之暗面 Kimi 最新版", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:262144, maxOutputTokens:262144, defaultReasoning:"high", supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:true },
  { id:"kimi-k2.6", name:"Kimi K2.6", desc:"Kimi 上一版本", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:262144, maxOutputTokens:262144, defaultReasoning:"medium", supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:true },
  { id:"glm-5.0-turbo", name:"GLM-5.0 Turbo", desc:"快速响应版", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:true, maxInputTokens:1000000, maxOutputTokens:131072, defaultReasoning:"none" },
  { id:"deepseek-v3", name:"DeepSeek V3", desc:"DeepSeek 通用对话", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:true, maxInputTokens:1000000, maxOutputTokens:384000, defaultReasoning:"none" },
  { id:"deepseek-r1", name:"DeepSeek R1", desc:"DeepSeek 推理模型，深度思考", type:"reasoning", supportsReasoning:true, supportsImages:true, supportsToolCall:false, maxInputTokens:1000000, maxOutputTokens:384000, defaultReasoning:"max", recommended:true, supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:true },
  { id:"deepseek-v3.2", name:"DeepSeek V3.2", desc:"DeepSeek V3 升级版", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:true, maxInputTokens:1000000, maxOutputTokens:384000, defaultReasoning:"none" },
  { id:"deepseek-v4-flash", name:"DeepSeek V4 Flash", desc:"快速版，低延迟", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:true, maxInputTokens:1000000, maxOutputTokens:384000, defaultReasoning:"none" },
  { id:"deepseek-v4-pro", name:"DeepSeek V4 Pro", desc:"DeepSeek 专业版", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:1000000, maxOutputTokens:384000, defaultReasoning:"medium", supportedEfforts:["low","medium","high","xhigh","max"], defaultEffort:"medium", canDisableThinking:true },
  { id:"minimax-m2.7", name:"MiniMax M2.7", desc:"MiniMax 对话模型", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:false, maxInputTokens:1000000, maxOutputTokens:524288, defaultReasoning:"none" },
];

export function buildModelConfig(model, reasoningLevels, deepThinking) {
  const levels = Array.isArray(reasoningLevels) ? reasoningLevels : [reasoningLevels];
  const hasReasoning = levels.filter(l => l !== "none");
  // 官方支持的5档思考强度(WorkBuddy源码白名单: low/medium/high/xhigh/max) — 全模型全开
  const supportedEfforts = model.supportsReasoning
    ? (model.supportedEfforts || ["low","medium","high","xhigh","max"])
    : undefined;
  const defaultEffort = model.supportsReasoning
    ? (model.defaultEffort || "medium")
    : undefined;
  const canDisableThinking = model.supportsReasoning ? (model.canDisableThinking !== false) : undefined;

  // Auto 模型特殊处理
  if (model.isAuto || model.id === "auto") {
    return {
      id: "auto",
      name: "Auto",
      supportsReasoning: true,
      onlyReasoning: true,
      reasoning: { effort: "high", summary: "auto", available: ["high","max"] },
      supportsToolCall: true,
      supportsImages: true,
      maxInputTokens: 1000000,
      maxOutputTokens: 128000,
      deepThinking: deepThinking,
      isAuto: true,
      desc: "自动模式，根据任务难度智能分配模型",
    };
  }

  const base = {
    id: model.id,
    name: model.name,
    supportsReasoning: model.supportsReasoning,
    onlyReasoning: model.supportsReasoning && hasReasoning.length > 0,
    reasoningLevels: model.supportsReasoning ? hasReasoning : [],
    supportedEfforts: supportedEfforts,
    defaultEffort: defaultEffort,
    canDisableThinking: canDisableThinking,
    reasoning: hasReasoning.length > 0 ? {
      effort: hasReasoning.includes("max") ? "max" : hasReasoning[0],
      summary: "auto",
      available: hasReasoning,
    } : undefined,
    supportsToolCall: model.supportsToolCall,
    supportsImages: true,
    maxInputTokens: model.maxInputTokens,
    maxOutputTokens: model.maxOutputTokens,
    deepThinking: deepThinking && model.supportsReasoning,
  };
  Object.keys(base).forEach(k => base[k] === undefined && delete base[k]);
  return base;
}
