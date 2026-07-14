/**
 * 上游支持的全部模型
 * supportsImages 全部设为 true，避免用户发图时报错
 */

export const ALL_MODELS = [
  { id:"auto", name:"Auto", desc:"自动模式，根据任务难度智能分配模型，节省Token", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:200000, maxOutputTokens:64000, defaultReasoning:"high", recommended:true, isAuto:true },
  { id:"glm-5.2", name:"GLM-5.2", desc:"智谱最新旗舰，推理+视觉+工具调用", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:200000, maxOutputTokens:64000, defaultReasoning:"high", recommended:true },
  { id:"glm-5.1", name:"GLM-5.1", desc:"智谱上一代旗舰", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:128000, maxOutputTokens:32000, defaultReasoning:"high" },
  { id:"glm-5.0-turbo", name:"GLM-5.0 Turbo", desc:"快速响应版", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:true, maxInputTokens:128000, maxOutputTokens:16000, defaultReasoning:"none" },
  { id:"glm-5v-turbo", name:"GLM-5V Turbo", desc:"视觉模型，支持图片理解", type:"vision", supportsReasoning:false, supportsImages:true, supportsToolCall:false, maxInputTokens:32000, maxOutputTokens:8000, defaultReasoning:"none" },
  { id:"deepseek-v3", name:"DeepSeek V3", desc:"DeepSeek 通用对话", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:true, maxInputTokens:128000, maxOutputTokens:16000, defaultReasoning:"none" },
  { id:"deepseek-r1", name:"DeepSeek R1", desc:"DeepSeek 推理模型，深度思考", type:"reasoning", supportsReasoning:true, supportsImages:true, supportsToolCall:false, maxInputTokens:128000, maxOutputTokens:64000, defaultReasoning:"max", recommended:true },
  { id:"deepseek-v3.2", name:"DeepSeek V3.2", desc:"DeepSeek V3 升级版", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:true, maxInputTokens:128000, maxOutputTokens:16000, defaultReasoning:"none" },
  { id:"deepseek-v4-flash", name:"DeepSeek V4 Flash", desc:"快速版，低延迟", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:true, maxInputTokens:128000, maxOutputTokens:8000, defaultReasoning:"none" },
  { id:"deepseek-v4-pro", name:"DeepSeek V4 Pro", desc:"DeepSeek 专业版", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:200000, maxOutputTokens:32000, defaultReasoning:"medium" },
  { id:"kimi-k2.7", name:"Kimi K2.7", desc:"月之暗面 Kimi 最新版", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:200000, maxOutputTokens:32000, defaultReasoning:"high" },
  { id:"kimi-k2.6", name:"Kimi K2.6", desc:"Kimi 上一版本", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:true, maxInputTokens:128000, maxOutputTokens:16000, defaultReasoning:"medium" },
  { id:"minimax-m2.7", name:"MiniMax M2.7", desc:"MiniMax 对话模型", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:false, maxInputTokens:128000, maxOutputTokens:16000, defaultReasoning:"none" },
  { id:"minimax-m3", name:"MiniMax M3", desc:"MiniMax 最新版", type:"chat", supportsReasoning:false, supportsImages:true, supportsToolCall:true, maxInputTokens:200000, maxOutputTokens:32000, defaultReasoning:"none" },
  { id:"hy3-preview", name:"HY3 Preview", desc:"HY3 预览版", type:"chat", supportsReasoning:true, supportsImages:true, supportsToolCall:false, maxInputTokens:128000, maxOutputTokens:32000, defaultReasoning:"medium" },
];

export function buildModelConfig(model, reasoningLevels, deepThinking) {
  const levels = Array.isArray(reasoningLevels) ? reasoningLevels : [reasoningLevels];
  const hasReasoning = levels.filter(l => l !== "none");

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
      maxInputTokens: 200000,
      maxOutputTokens: 64000,
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
