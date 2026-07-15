use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

/// 读取JSON文件并去除UTF-8 BOM, 防止JSON.parse失败
fn read_json_file(path: &Path) -> Option<serde_json::Value> {
    let content = fs::read_to_string(path).ok()?;
    let trimmed = if content.starts_with('\u{FEFF}') {
        &content[3..]
    } else {
        &content
    };
    serde_json::from_str(trimmed).ok()
}

// 推理等级排序（从低到高）
const REASONING_ORDER: [&str; 7] = ["none", "minimal", "low", "medium", "high", "xhigh", "max"];

/// 将内部推理等级映射到 WorkBuddy/CodeBuddy 支持的等级
/// WorkBuddy/CodeBuddy 只支持 low/medium/high，不支持 xhigh/max
fn to_wb_effort(level: &str) -> &str {
    match level {
        "none" | "minimal" => "low",
        "low" => "low",
        "medium" => "medium",
        "high" | "xhigh" | "max" => "high",
        _ => "high",
    }
}

/// 将内部推理等级映射到 Claude Code/Anthropic 支持的等级
fn to_anthropic_effort(level: &str) -> &str {
    to_wb_effort(level) // 同样的映射
}

/// 根据模型id返回WorkBuddy官方vendor标识(用于显示对应图标)
/// e=智谱GLM, f=通用(DeepSeek/Kimi/MiniMax/auto), j=腾讯混元, tencent=腾讯
fn to_wb_vendor(model_id: &str) -> &str {
    if model_id.starts_with("glm-") || model_id.starts_with("glm5") {
        "e"
    } else if model_id == "auto" {
        "f"
    } else if model_id.starts_with("deepseek-") || model_id.starts_with("kimi-") || model_id.starts_with("minimax-") {
        "f"
    } else if model_id.starts_with("hy3") || model_id.starts_with("hunyuan") {
        "j"
    } else {
        "f"
    }
}

/// 根据模型id返回显示名称(跟官方一致)
fn to_wb_display_name(model_id: &str) -> String {
    match model_id {
        "auto" => "自动模式（智能选择）".to_string(),
        "glm-5.2" => "GLM-5.2".to_string(),
        "glm-5.1" => "GLM-5.1".to_string(),
        "glm-5.0-turbo" => "GLM-5.0 Turbo".to_string(),
        "glm-5v-turbo" => "GLM-5V Turbo".to_string(),
        "deepseek-v3" => "DeepSeek V3".to_string(),
        "deepseek-r1" => "DeepSeek R1".to_string(),
        "deepseek-v3.2" => "DeepSeek V3.2".to_string(),
        "deepseek-v4-flash" => "DeepSeek V4 Flash".to_string(),
        "deepseek-v4-pro" => "DeepSeek V4 Pro".to_string(),
        "kimi-k2.7" => "Kimi K2.7".to_string(),
        "kimi-k2.6" => "Kimi K2.6".to_string(),
        "minimax-m2.7" => "MiniMax M2.7".to_string(),
        "minimax-m3" => "MiniMax M3".to_string(),
        "hy3-preview" => "HY3 Preview".to_string(),
        _ => model_id.to_string(),
    }
}

/// 根据模型id返回官方descriptionEn (跟官方entry完全一致)
fn to_wb_description_en(model_id: &str) -> &'static str {
    match model_id {
        "auto" => "Balances quality and speed. Automatically selects the best model for each task, with a variable credit multiplier.",
        "glm-5.2" => "1M context, built for long-horizon tasks.",
        "glm-5.1" => "Previous generation flagship model.",
        "glm-5.0-turbo" => "Fast response version.",
        "glm-5v-turbo" => "Vision model, supports image understanding.",
        "deepseek-v3" => "DeepSeek general chat model.",
        "deepseek-r1" => "DeepSeek reasoning model, deep thinking.",
        "deepseek-v3.2" => "DeepSeek V3 upgraded version.",
        "deepseek-v4-flash" => "Fast version, low latency.",
        "deepseek-v4-pro" => "DeepSeek flagship model, supporting 1M context window",
        "kimi-k2.7" => "A multimodal model, good for daily use.",
        "kimi-k2.6" => "Kimi previous version.",
        "minimax-m2.7" => "MiniMax chat model.",
        "minimax-m3" => "MiniMax latest version.",
        "hy3-preview" => "HY3 preview version.",
        _ => "",
    }
}

/// 根据模型id返回官方credits字符串
fn to_wb_credits(model_id: &str) -> &'static str {
    match model_id {
        "glm-5.2" => "x0.79",
        "glm-5.1" => "x0.40",
        "glm-5.0-turbo" => "x0.20",
        "deepseek-v3" => "x0.16",
        "deepseek-r1" => "x0.96",
        "deepseek-v3.2" => "x0.32",
        "deepseek-v4-flash" => "x0.32",
        "deepseek-v4-pro" => "x0.96",
        "kimi-k2.7" => "x0.57",
        "kimi-k2.6" => "x0.29",
        "minimax-m2.7" => "x0.07",
        "minimax-m3" => "x0.14",
        "hy3-preview" => "x0.10",
        _ => "",
    }
}

/// 根据模型id返回中文描述 (用于WorkBuddy问号图标点击显示)
fn to_wb_description_zh(model_id: &str) -> &'static str {
    match model_id {
        "auto" => "自动模式，根据任务难度智能分配模型，节省Token",
        "glm-5.2" => "智谱最新旗舰，1M上下文，深度推理+视觉+工具调用",
        "glm-5.1" => "智谱上一代旗舰模型",
        "glm-5.0-turbo" => "快速响应版，适合日常任务",
        "glm-5v-turbo" => "视觉模型，支持图片理解",
        "deepseek-v3" => "DeepSeek 通用对话模型",
        "deepseek-r1" => "DeepSeek 推理模型，深度思考",
        "deepseek-v3.2" => "DeepSeek V3 升级版",
        "deepseek-v4-flash" => "快速版，低延迟",
        "deepseek-v4-pro" => "DeepSeek 专业版，支持1M上下文窗口",
        "kimi-k2.7" => "月之暗面 Kimi 最新版，多模态",
        "kimi-k2.6" => "Kimi 上一版本",
        "minimax-m2.7" => "MiniMax 对话模型",
        "minimax-m3" => "MiniMax 最新版",
        "hy3-preview" => "腾讯混元HY3预览版，深度推理",
        _ => "",
    }
}

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(target_os = "windows")]
trait NoWindow {
    fn no_window(&mut self) -> &mut Self;
}

#[cfg(target_os = "windows")]
impl NoWindow for std::process::Command {
    fn no_window(&mut self) -> &mut Self {
        self.creation_flags(CREATE_NO_WINDOW);
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct DeployConfig {
    #[serde(rename = "type")]
    deploy_type: String,
    api_key: String,
    base_url: String,
    model: String,
    reasoning_level: String,
    #[serde(default)]
    reasoning_levels: Vec<String>,
    deep_thinking: bool,
    #[serde(default)]
    model_configs: Vec<serde_json::Value>,
    #[serde(default)]
    selected_model_ids: Vec<String>,
}

#[derive(Serialize)]
struct DetectResult {
    installed: bool,
    path: Option<String>,
}

/// 检测全部5个平台
#[tauri::command]
fn detect_all_platforms() -> std::collections::HashMap<String, DetectResult> {
    let mut r = std::collections::HashMap::new();
    r.insert("opencode".into(), detect_opencode());
    r.insert("claudecode".into(), detect_claude_code());
    r.insert("codebuddy".into(), detect_codebuddy());
    r.insert("workbuddy".into(), detect_workbuddy());
    r.insert("clawcode".into(), detect_claw_code());
    r.insert("trae".into(), detect_trae());
    r
}

/// 通用：检测进程名（静默，无黑框）
#[cfg(target_os = "windows")]
fn check_process(name: &str) -> bool {
    if let Ok(output) = std::process::Command::new("tasklist")
        .args(["/FI", &format!("IMAGENAME eq {}", name), "/NH"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.to_lowercase().contains(&name.to_lowercase());
    }
    false
}

/// OpenCode: ~/.config/opencode/ 或 AppData\Local\Programs\@opencode-aidesktop\
fn detect_opencode() -> DetectResult {
    // 1. 配置文件优先（check_opencode_config 需要 path 指向含 opencode.global.dat 的目录）
    let appdata = dirs::config_dir().map(|d| d.join("ai.opencode.desktop"));
    if let Some(a) = &appdata {
        if a.exists() { return DetectResult { installed: true, path: Some(a.to_string_lossy().into()) }; }
    }

    let config = dirs::home_dir().map(|d| d.join(".config/opencode/opencode.json"));
    if let Some(c) = &config {
        if c.exists() { return DetectResult { installed: true, path: Some(c.to_string_lossy().into()) }; }
    }

    // 2. 进程检测
    #[cfg(target_os = "windows")]
    if check_process("OpenCode.exe") {
        // 进程在运行但没找到配置，返回 appdata 默认路径
        let default = dirs::config_dir().map(|d| d.join("ai.opencode.desktop")).unwrap_or_default();
        return DetectResult { installed: true, path: Some(default.to_string_lossy().into()) };
    }

    // 3. 安装目录
    let install = dirs::data_dir().map(|d| d.join("Programs/@opencode-aidesktop"));
    if let Some(i) = &install {
        if i.exists() { return DetectResult { installed: true, path: Some(i.to_string_lossy().into()) }; }
    }

    DetectResult { installed: false, path: None }
}

/// Claude Code: ~/.claude/
fn detect_claude_code() -> DetectResult {
    let p = dirs::home_dir().map(|d| d.join(".claude"));
    if let Some(p) = &p {
        if p.exists() { return DetectResult { installed: true, path: Some(p.to_string_lossy().into()) }; }
    }
    #[cfg(target_os = "windows")]
    if check_process("claude.exe") {
        let default = dirs::home_dir().map(|d| d.join(".claude")).unwrap_or_default();
        return DetectResult { installed: true, path: Some(default.to_string_lossy().into()) };
    }
    DetectResult { installed: false, path: None }
}

/// CodeBuddy CN: ~/.codebuddy/ 独立客户端
fn detect_codebuddy() -> DetectResult {
    // ~/.codebuddy 目录优先
    if let Some(home) = dirs::home_dir() {
        let cb_dir = home.join(".codebuddy");
        if cb_dir.exists() { return DetectResult { installed: true, path: Some(cb_dir.to_string_lossy().into()) }; }
    }

    #[cfg(target_os = "windows")]
    if check_process("CodeBuddy CN.exe") {
        let default = dirs::home_dir().map(|d| d.join(".codebuddy")).unwrap_or_default();
        return DetectResult { installed: true, path: Some(default.to_string_lossy().into()) };
    }

    // 安装目录
    if let Some(d) = dirs::data_dir() {
        let p = d.join("Programs/CodeBuddy CN");
        if p.exists() { return DetectResult { installed: true, path: Some(p.to_string_lossy().into()) }; }
    }

    DetectResult { installed: false, path: None }
}

/// WorkBuddy: ~/.workbuddy/ 或 C:\Program Files\WorkBuddy\
fn detect_workbuddy() -> DetectResult {
    // ~/.workbuddy 优先（配置文件在这里）
    if let Some(home) = dirs::home_dir() {
        let wb = home.join(".workbuddy");
        if wb.exists() { return DetectResult { installed: true, path: Some(wb.to_string_lossy().into()) }; }
    }

    #[cfg(target_os = "windows")]
    if check_process("WorkBuddy.exe") {
        // 进程在运行但没找到 ~/.workbuddy，用默认路径
        let home = dirs::home_dir().map(|d| d.join(".workbuddy")).unwrap_or_default();
        return DetectResult { installed: true, path: Some(home.to_string_lossy().into()) };
    }

    // C:\Program Files\WorkBuddy
    for p in &[PathBuf::from("C:\\Program Files\\WorkBuddy"), PathBuf::from("C:\\Program Files (x86)\\WorkBuddy")] {
        if p.exists() {
            // 安装目录存在，但配置在 ~/.workbuddy
            let home = dirs::home_dir().map(|d| d.join(".workbuddy")).unwrap_or_default();
            return DetectResult { installed: true, path: Some(home.to_string_lossy().into()) };
        }
    }

    DetectResult { installed: false, path: None }
}

/// Claw Code (QClaw/OpenClaw): 仅检测实际安装
fn detect_claw_code() -> DetectResult {
    // ~/.openclaw 目录优先（必须有 openclaw.json 才算安装）
    if let Some(home) = dirs::home_dir() {
        let oc_dir = home.join(".openclaw");
        let cfg = oc_dir.join("openclaw.json");
        if cfg.exists() { return DetectResult { installed: true, path: Some(oc_dir.to_string_lossy().into()) }; }
    }

    #[cfg(target_os = "windows")]
    {
        if check_process("QClaw.exe") || check_process("openclaw.exe") {
            let default = dirs::home_dir().map(|d| d.join(".openclaw")).unwrap_or_default();
            return DetectResult { installed: true, path: Some(default.to_string_lossy().into()) };
        }
    }

    // %LOCALAPPDATA%\Programs\QClaw（必须有 exe 才算安装）
    if let Some(d) = dirs::data_dir() {
        let exe = d.join("Programs/QClaw/QClaw.exe");
        if exe.exists() { return DetectResult { installed: true, path: Some(exe.to_string_lossy().into()) }; }
    }

    DetectResult { installed: false, path: None }
}

/// Trae: 进程或安装目录
fn detect_trae() -> DetectResult {
    // ~/.trae 优先
    if let Some(home) = dirs::home_dir() {
        let p = home.join(".trae");
        if p.exists() { return DetectResult { installed: true, path: Some(p.to_string_lossy().into()) }; }
    }

    #[cfg(target_os = "windows")]
    if check_process("Trae.exe") {
        let default = dirs::home_dir().map(|d| d.join(".trae")).unwrap_or_default();
        return DetectResult { installed: true, path: Some(default.to_string_lossy().into()) };
    }

    // AppData
    if let Some(d) = dirs::config_dir() {
        let p = d.join("Trae");
        if p.exists() { return DetectResult { installed: true, path: Some(p.to_string_lossy().into()) }; }
    }

    // 安装目录
    if let Some(d) = dirs::data_dir() {
        let p = d.join("Programs/Trae");
        if p.exists() { return DetectResult { installed: true, path: Some(p.to_string_lossy().into()) }; }
    }

    DetectResult { installed: false, path: None }
}

/// 部署到指定平台
#[tauri::command]
fn deploy_to_platform(config: DeployConfig) -> Result<String, String> {
    match config.deploy_type.as_str() {
        "opencode" => deploy_opencode(&config),
        "claudecode" => deploy_claude_code(&config),
        "codebuddy" => deploy_codebuddy(&config),
        "workbuddy" => deploy_workbuddy(&config),
        "clawcode" => deploy_claw_code(&config),
        "trae" => deploy_trae(&config),
        _ => Err("未知平台".into()),
    }
}

/// OpenCode 部署: 修改 opencode.json + opencode.global.dat
/// 1. opencode.json: 写入 provider.antigravity 配置
/// 2. opencode.global.dat: 解析 model 字段(嵌套JSON字符串)，在 variant 对象中设置 "antigravity:模型ID": 推理等级
fn deploy_opencode(config: &DeployConfig) -> Result<String, String> {
    // 1. opencode.json — 主配置文件
    let oc_config_dir = dirs::home_dir()
        .ok_or("无法获取用户目录")?
        .join(".config")
        .join("opencode");

    if !oc_config_dir.exists() {
        fs::create_dir_all(&oc_config_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let config_path = oc_config_dir.join("opencode.json");

    let mut oc_config: serde_json::Value = if config_path.exists() {
        read_json_file(&config_path)
            .unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // 构建模型配置
    let mut models_map = serde_json::Map::new();
    for model_id in &config.selected_model_ids {
        let mc = config.model_configs.iter().find(|m| {
            m.get("id").and_then(|v| v.as_str()) == Some(model_id.as_str())
        });

        let model_name = mc.and_then(|c| c.get("name")).and_then(|v| v.as_str()).unwrap_or(model_id);

        models_map.insert(model_id.clone(), serde_json::json!({
            "name": model_name,
            "options": {
                "reasoningEffort": config.reasoning_level
            },
            "attachment": true,
            "modalities": {"input": ["text", "image"]},
            "limit": {
                "context": mc.and_then(|c| c.get("maxInputTokens")).and_then(|v| v.as_u64()).unwrap_or(1000000),
                "output": mc.and_then(|c| c.get("maxOutputTokens")).and_then(|v| v.as_u64()).unwrap_or(128000)
            }
        }));
    }

    // 更新 provider.antigravity
    if let Some(obj) = oc_config.as_object_mut() {
        if !obj.contains_key("provider") {
            obj.insert("provider".into(), serde_json::json!({}));
        }
        if let Some(provider) = obj.get_mut("provider").and_then(|v| v.as_object_mut()) {
            provider.remove("antigravity");
            provider.insert("antigravity".into(), serde_json::json!({
                "name": "Antigravity Tools",
                "npm": "@ai-sdk/openai-compatible",
                "options": {
                    "baseURL": config.base_url,
                    "apiKey": config.api_key
                },
                "models": serde_json::Value::Object(models_map)
            }));
        }
        if let Some(first_model) = config.selected_model_ids.first() {
            obj.insert("model".into(), serde_json::json!(format!("antigravity/{}", first_model)));
        }
    }

    fs::write(&config_path, serde_json::to_string_pretty(&oc_config).unwrap())
        .map_err(|e| format!("写入 opencode.json 失败: {}", e))?;

    // 2. opencode.global.dat — 解析 model 字段(嵌套JSON字符串)，设置 variant
    let dat_path = dirs::config_dir()
        .map(|d| d.join("ai.opencode.desktop/opencode.global.dat"));

    if let Some(dp) = dat_path {
        if dp.exists() {
            if let Ok(content) = fs::read_to_string(&dp) {
                if let Ok(mut data) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(model_str) = data.get("model").and_then(|v| v.as_str()).map(|s| s.to_string()) {
                        if let Ok(mut model_obj) = serde_json::from_str::<serde_json::Value>(&model_str) {
                            // 在 variant 对象中设置每个模型的推理等级
                            if let Some(variants) = model_obj.get_mut("variant").and_then(|v| v.as_object_mut()) {
                                // 清除旧的 antigravity variant（保留 auto）
                                let old_keys: Vec<String> = variants.keys()
                                    .filter(|k| k.starts_with("antigravity:") && !k.ends_with(":auto"))
                                    .cloned().collect();
                                for k in old_keys { variants.remove(&k); }

                                // 写入新的 variant
                                for model_id in &config.selected_model_ids {
                                    variants.insert(
                                        format!("antigravity:{}", model_id),
                                        serde_json::json!(config.reasoning_level),
                                    );
                                }
                            }

                            // 写回 model 字段（重新序列化为 JSON 字符串）
                            data.as_object_mut().map(|o| {
                                o.insert("model".into(), serde_json::Value::String(
                                    serde_json::to_string(&model_obj).unwrap_or_default()
                                ));
                            });

                            let _ = fs::write(&dp, serde_json::to_string_pretty(&data).unwrap());
                        }
                    }
                }
            }
        }
    }

    Ok(format!("OpenCode: {} 个模型已写入 opencode.json + variant 已设置", config.selected_model_ids.len()))
}

/// Claude Code 部署: 修改 ~/.claude/settings.json
/// env: ANTHROPIC_BASE_URL + ANTHROPIC_AUTH_TOKEN + ANTHROPIC_MODEL
/// effortLevel: 推理等级
fn deploy_claude_code(config: &DeployConfig) -> Result<String, String> {
    let cc_dir = detect_claude_code().path.ok_or("Claude Code 未安装")?;
    let dir = PathBuf::from(&cc_dir);
    let settings_path = dir.join("settings.json");

    let mut settings: serde_json::Value = if settings_path.exists() {
        read_json_file(&settings_path)
            .unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // 计算推理等级（提前到 if 块外面，供返回消息使用）
    let levels = if config.reasoning_levels.is_empty() {
        vec![config.reasoning_level.clone()]
    } else {
        config.reasoning_levels.clone()
    };
    let highest = levels.iter()
        .filter_map(|l| REASONING_ORDER.iter().position(|&r| r == l).map(|p| (l, p)))
        .max_by_key(|(_, p)| *p)
        .map(|(l, _)| l.clone())
        .unwrap_or_else(|| config.reasoning_level.clone());

    if let Some(obj) = settings.as_object_mut() {
        // 0. 清除旧配置（避免堆积）
        obj.remove("thinking");

        // Claude Code 只支持单个模型，用选中的第一个
        let model = config.selected_model_ids.first().unwrap_or(&config.model);
        let env = serde_json::json!({
            "ANTHROPIC_AUTH_TOKEN": config.api_key,
            "ANTHROPIC_BASE_URL": config.base_url,
            "ANTHROPIC_MODEL": model
        });
        obj.insert("env".to_string(), env);

        // 推理等级 — 用最高的，映射到平台支持的值，同时写入可选列表
        obj.insert("effortLevel".to_string(), serde_json::json!(to_anthropic_effort(&highest)));
        // 也写入可选等级列表，用户可手动切换
        obj.insert("availableEffortLevels".to_string(), serde_json::json!(levels));

        // 深度思考
        if config.deep_thinking {
            obj.insert("thinking".to_string(), serde_json::json!({"enabled": true, "budget": "max"}));
        }
    }

    fs::write(&settings_path, serde_json::to_string_pretty(&settings).unwrap())
        .map_err(|e| format!("写入失败: {}", e))?;

    Ok(format!("Claude Code: {} 个模型已配置 (默认: {})，推理等级: {} (可选: {})", config.selected_model_ids.len(), config.selected_model_ids.first().unwrap_or(&config.model), highest, levels.join(",")))
}

/// CodeBuddy CN 部署: 写入 ~/.codebuddy/models.json
/// 格式（实际能用的）:
/// {"models": [{id, name, vendor:"user", url, apiKey, supportsToolCall:true, supportsImages:true,
///   supportsReasoning:true, onlyReasoning:true, reasoning:{effort:"max",summary:"auto",available:["max"]},
///   maxInputTokens, maxOutputTokens, deepThinking:true}]}
fn deploy_codebuddy(config: &DeployConfig) -> Result<String, String> {
    let cb_dir = dirs::home_dir().ok_or("无法获取用户目录")?.join(".codebuddy");
    if !cb_dir.exists() {
        fs::create_dir_all(&cb_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let models_path = cb_dir.join("models.json");

    // CodeBuddy 用 url 字段，必须带 /v1 后缀
    let cb_url = if config.base_url.ends_with("/v1") {
        config.base_url.clone()
    } else {
        format!("{}/v1", config.base_url.trim_end_matches('/'))
    };

    // 所有模型统一格式 — 跟 CodeBuddy 实际能用的配置完全一致
    // 第一个模型设 isDefault=true，让客户端启动后自动选中 GLM-5.2
    let new_models: Vec<serde_json::Value> = config.selected_model_ids.iter().enumerate().map(|(i, mid)| {
        let mc = config.model_configs.iter().find(|m| {
            m.get("id").and_then(|v| v.as_str()) == Some(mid.as_str())
        });

        serde_json::json!({
            "id": mid,
            "name": to_wb_display_name(mid),
            "vendor": "user",
            "url": cb_url,
            "apiKey": config.api_key,
            "supportsToolCall": true,
            "supportsImages": true,
            "supportsReasoning": true,
            "onlyReasoning": true,
            "isDefault": i == 0,
            "reasoning": {
                "effort": to_wb_effort(&config.reasoning_level),
                "summary": "auto",
                "canDisableThinking": false,
                "defaultEffort": to_wb_effort(&config.reasoning_level),
                "supportedEfforts": ["low", "medium", "high"]
            },
            "maxInputTokens": mc.and_then(|c| c.get("maxInputTokens")).and_then(|v| v.as_u64()).unwrap_or(1000000),
            "maxOutputTokens": mc.and_then(|c| c.get("maxOutputTokens")).and_then(|v| v.as_u64()).unwrap_or(128000),
            "descriptionEn": to_wb_description_en(mid),
            "descriptionZh": to_wb_description_zh(mid),
            "credits": to_wb_credits(mid),
            "deepThinking": true
        })
    }).collect();

    // 读取现有 models.json, 保留官方模型, 只替换 vendor=user 的模型
    let existing: serde_json::Value = if models_path.exists() {
        read_json_file(&models_path)
            .unwrap_or(serde_json::json!({"models": []}))
    } else {
        serde_json::json!({"models": []})
    };
    let backup_path = cb_dir.join("models.json.launcher_bak");
    if models_path.exists() {
        let _ = fs::copy(&models_path, &backup_path);
    }

    // 分离官方模型和旧的自定义模型
    let mut official_models: Vec<serde_json::Value> = Vec::new();
    if let Some(models) = existing.get("models").and_then(|m| m.as_array()) {
        for m in models {
            let vendor = m.get("vendor").and_then(|v| v.as_str()).unwrap_or("");
            if vendor != "user" {
                official_models.push(m.clone());
            }
        }
    }

    // 合并: 官方模型 + 我们的自定义模型
    let mut all_models = official_models;
    all_models.extend(new_models);

    let out = serde_json::json!({"models": all_models});
    fs::write(&models_path, serde_json::to_string_pretty(&out).unwrap())
        .map_err(|e| format!("写入失败: {}", e))?;

    // 写入全局配置: settings.json (全局 reasoningEffort=xhigh + alwaysThinkingEnabled=true)
    let cb_settings_path = cb_dir.join("settings.json");
    let mut cb_settings: serde_json::Value = if cb_settings_path.exists() {
        read_json_file(&cb_settings_path)
            .unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };
    if let Some(obj) = cb_settings.as_object_mut() {
        obj.insert("reasoningEffort".to_string(), serde_json::json!("xhigh"));
        obj.insert("alwaysThinkingEnabled".to_string(), serde_json::json!(true));
    }
    let _ = fs::write(&cb_settings_path, serde_json::to_string_pretty(&cb_settings).unwrap_or_default());

    Ok(format!("CodeBuddy CN: {} 个模型已写入 ~/.codebuddy/models.json + 全局配置 reasoningEffort=xhigh", config.selected_model_ids.len()))
}

/// WorkBuddy 部署: 写入 ~/.workbuddy/models.json
/// 格式（用户手动配置验证可用）:
/// [{id, name, vendor:"Custom", url, apiKey, supportsToolCall, supportsImages, supportsReasoning, useCustomProtocol:false, reasoning:{supportedEfforts:["max"]}}]
fn deploy_workbuddy(config: &DeployConfig) -> Result<String, String> {
    let wb_dir = dirs::home_dir().ok_or("无法获取用户目录")?.join(".workbuddy");
    if !wb_dir.exists() {
        fs::create_dir_all(&wb_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    // 关键: 部署前必须先关闭 WorkBuddy, 否则 WorkBuddy 退出时会把内存状态覆盖回 entry 文件
    kill_workbuddy_processes();

    // 写入 models.json (WorkBuddy 自定义模型模板文件)
    // 严格按用户手动添加验证可用的格式: name=id小写, 无 reasoning 字段
    let models_path = wb_dir.join("models.json");
    if models_path.exists() {
        let _ = fs::copy(&models_path, wb_dir.join("models.json.launcher_bak"));
    }
    let wb_url = if config.base_url.ends_with("/v1") {
        config.base_url.clone()
    } else {
        format!("{}/v1", config.base_url.trim_end_matches('/'))
    };
    let models_json_entries: Vec<serde_json::Value> = config.selected_model_ids.iter().map(|mid| {
        let mc = config.model_configs.iter().find(|m| {
            m.get("id").and_then(|v| v.as_str()) == Some(mid.as_str())
        });
        let supports_reasoning = mc.and_then(|c| c.get("supportsReasoning")).and_then(|v| v.as_bool()).unwrap_or(true);
        let supports_tools = mc.and_then(|c| c.get("supportsToolCall")).and_then(|v| v.as_bool()).unwrap_or(true);
        serde_json::json!({
            "id": mid,
            "name": to_wb_display_name(mid),
            "vendor": "user",
            "url": wb_url,
            "apiKey": config.api_key,
            "supportsToolCall": supports_tools,
            "supportsImages": true,
            "supportsReasoning": supports_reasoning,
            "reasoning": {
                "effort": to_wb_effort(&config.reasoning_level),
                "summary": "auto",
                "canDisableThinking": false,
                "defaultEffort": to_wb_effort(&config.reasoning_level),
                "supportedEfforts": ["low", "medium", "high"]
            },
            "relatedModels": {
                "lite": mid,
                "reasoning": mid
            },
            "tags": ["custom"],
            "temperature": 1,
            "descriptionEn": to_wb_description_en(mid),
            "descriptionZh": to_wb_description_zh(mid),
            "credits": to_wb_credits(mid)
        })
    }).collect();
    // models.json 必须是对象格式 {"models": [...]}, 不能是数组 (WorkBuddy Provider只认对象)
    let models_json_obj = serde_json::json!({"models": models_json_entries});
    fs::write(&models_path, serde_json::to_string_pretty(&models_json_obj).unwrap())
        .map_err(|e| format!("写入 models.json 失败: {}", e))?;

    // 写入 local_storage/entry_*.info (WorkBuddy 实际运行时读取的配置文件)
    let ls_dir = wb_dir.join("local_storage");
    if !ls_dir.exists() {
        // local_storage 不存在 = 用户从未启动过 WorkBuddy, 自动创建目录和空entry
        fs::create_dir_all(&ls_dir).map_err(|e| format!("创建 local_storage 失败: {}", e))?;
        let empty_entry = serde_json::json!([{
            "userId": "launcher",
            "data": {
                "models": [],
                "config": {},
                "endpoint": {},
                "featureToggles": {},
                "agents": [],
                "builtInMarketPlugins": [],
                "builtInMarketplaces": [],
                "builtInSkillMarketplaces": [],
                "completion": {},
                "integrations": {},
                "links": [],
                "modelPromotions": [],
                "productFeatures": [],
                "productFeatureExperiment": {},
                "requestMaxStepLimit": {},
                "telemetry": {},
                "tokenUsageThresholds": {}
            },
            "ts": 0
        }]);
        let entry_name = format!("entry_{:x}.info", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
        let entry_path = ls_dir.join(&entry_name);
        fs::write(&entry_path, serde_json::to_string(&empty_entry).unwrap_or_default())
            .map_err(|e| format!("创建 entry 文件失败: {}", e))?;
    }

    // 构建要注入的 custom-local 模型列表 (跟官方模型格式完全一致以获得图标)
    let new_models: Vec<serde_json::Value> = config.selected_model_ids.iter().enumerate().map(|(i, mid)| {
        let mc = config.model_configs.iter().find(|m| {
            m.get("id").and_then(|v| v.as_str()) == Some(mid.as_str())
        });
        let supports_reasoning = mc.and_then(|c| c.get("supportsReasoning")).and_then(|v| v.as_bool()).unwrap_or(true);
        let supports_tools = mc.and_then(|c| c.get("supportsToolCall")).and_then(|v| v.as_bool()).unwrap_or(true);
        let max_input = mc.and_then(|c| c.get("maxInputTokens")).and_then(|v| v.as_u64()).unwrap_or(1000000);
        let max_output = mc.and_then(|c| c.get("maxOutputTokens")).and_then(|v| v.as_u64()).unwrap_or(128000);
        let display_name = to_wb_display_name(mid);
        let vendor = to_wb_vendor(mid);
        let desc_en = to_wb_description_en(mid);
        let desc_zh = to_wb_description_zh(mid);
        let credits = to_wb_credits(mid);
        serde_json::json!({
            "id": format!("custom-local:{}", mid),
            "name": display_name,
            "vendor": vendor,
            "url": wb_url,
            "apiKey": config.api_key,
            "supportsToolCall": supports_tools,
            "supportsImages": true,
            "supportsReasoning": supports_reasoning,
            "onlyReasoning": supports_reasoning,
            "isDefault": i == 0,
            "maxInputTokens": max_input,
            "maxOutputTokens": max_output,
            "maxAllowedSize": max_input,
            "reasoning": {
                "effort": to_wb_effort(&config.reasoning_level),
                "summary": "auto",
                "canDisableThinking": false,
                "defaultEffort": to_wb_effort(&config.reasoning_level),
                "supportedEfforts": ["low", "medium", "high"]
            },
            "relatedModels": {
                "lite": mid,
                "reasoning": mid
            },
            "tags": ["craft"],
            "temperature": 1,
            "descriptionEn": desc_en,
            "descriptionZh": desc_zh,
            "credits": credits
        })
    }).collect();

    // 处理所有 entry_*.info 文件 (支持多种格式)
    let all_entries = find_all_workbuddy_entries(&ls_dir);
    if all_entries.is_empty() {
        return Err("未找到有效的 entry_*.info 文件, 请先启动一次 WorkBuddy".into());
    }

    let mut written_count = 0;
    let mut errors: Vec<String> = Vec::new();

    for (entry_name, json_str, format_type) in &all_entries {
        let entry_path = ls_dir.join(entry_name);
        let bak = entry_path.with_extension("info.launcher_bak");
        let _ = fs::copy(&entry_path, &bak);

        let result: Result<(), String> = match *format_type {
            "gzip" => {
                // 旧版 gzip+base64 格式
                let mut data: serde_json::Value = serde_json::from_str(json_str)
                    .map_err(|e| format!("解析失败: {}", e))?;
                if let Some(models) = data.get_mut("models").and_then(|m| m.as_array_mut()) {
                    // 删除旧的 custom-local: 模型
                    models.retain(|m| {
                        let id = m.get("id").and_then(|v| v.as_str()).unwrap_or("");
                        !id.starts_with("custom-local:")
                    });
                    // 清除所有现有模型的 isDefault (防止与我们的冲突)
                    for m in models.iter_mut() {
                        if let Some(obj) = m.as_object_mut() {
                            obj.insert("isDefault".to_string(), serde_json::Value::Bool(false));
                        }
                    }
                    // 添加新的
                    for nm in &new_models {
                        models.push(nm.clone());
                    }
                }
                let new_json = serde_json::to_string(&data).map_err(|e| format!("序列化失败: {}", e))?;
                let encoded = encode_entry_info(&new_json)?;
                fs::write(&entry_path, encoded).map_err(|e| format!("写入失败: {}", e))?;
                Ok(())
            }
            "json" => {
                // 新版裸JSON格式 (数组或对象)
                let new_json = inject_models_into_json_entry(json_str, &new_models)?;
                fs::write(&entry_path, new_json.into_bytes()).map_err(|e| format!("写入失败: {}", e))?;
                Ok(())
            }
            _ => Ok(()),
        };

        match result {
            Ok(()) => { written_count += 1; }
            Err(e) => { errors.push(format!("{}: {}", entry_name, e)); }
        }
    }

    if written_count == 0 {
        return Err(format!("所有 entry 文件写入失败: {}", errors.join("; ")));
    }

    // 同时也处理 find_workbuddy_entry 返回的文件（兼容旧逻辑）
    // 但上面已经处理了所有entry，这里只是确保不会遗漏
    let _ = find_workbuddy_entry(&ls_dir);

    // 写入全局配置: config.json (全局 reasoningEffort + alwaysThinkingEnabled)
    let config_path = wb_dir.join("config.json");
    let mut wb_config: serde_json::Value = if config_path.exists() {
        read_json_file(&config_path)
            .unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };
    if let Some(obj) = wb_config.as_object_mut() {
        obj.insert("reasoningEffort".to_string(), serde_json::json!("xhigh"));
        obj.insert("alwaysThinkingEnabled".to_string(), serde_json::json!(true));
    }
    let _ = fs::write(&config_path, serde_json::to_string_pretty(&wb_config).unwrap_or_default());

    // 也写入 settings.json
    let settings_path = wb_dir.join("settings.json");
    let mut wb_settings: serde_json::Value = if settings_path.exists() {
        read_json_file(&settings_path)
            .unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };
    if let Some(obj) = wb_settings.as_object_mut() {
        obj.insert("reasoningEffort".to_string(), serde_json::json!("xhigh"));
        obj.insert("alwaysThinkingEnabled".to_string(), serde_json::json!(true));
    }
    let _ = fs::write(&settings_path, serde_json::to_string_pretty(&wb_settings).unwrap_or_default());

    Ok(format!("WorkBuddy: {} 个模型已写入 models.json + {} 个 entry 文件 + 全局配置 reasoningEffort=xhigh", config.selected_model_ids.len(), written_count))
}

/// 关闭所有 WorkBuddy 进程 (部署前必须关闭, 否则退出时会覆盖 entry 文件)
fn kill_workbuddy_processes() {
    #[cfg(target_os = "windows")]
    {
        // 尝试多种可能的进程名
        for proc_name in &["WorkBuddy.exe", "workbuddy.exe", "WorkBuddy"] {
            let _ = std::process::Command::new("taskkill")
                .args(&["/F", "/IM", proc_name])
                .no_window()
                .output();
        }
        // 也用 wmic 按命令行匹配 (防止进程名不标准)
        let _ = std::process::Command::new("wmic")
            .args(&["process", "where", "name like '%WorkBuddy%'", "call", "terminate"])
            .no_window()
            .output();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("pkill")
            .args(&["-f", "WorkBuddy"])
            .output();
        let _ = std::process::Command::new("pkill")
            .args(&["-f", "workbuddy"])
            .output();
    }
    // 等待进程完全退出和文件句柄释放 (3秒确保充分退出)
    std::thread::sleep(std::time::Duration::from_millis(3000));
}

/// 在 local_storage 目录中找到所有 entry_*.info 文件
/// 返回 (文件名, 解码后的JSON字符串, 格式类型)
/// 支持三种格式:
///   1. gzip+base64+引号 (旧版5.2.x)
///   2. 裸JSON数组 (新版5.2.5+)
///   3. 裸JSON对象
fn find_all_workbuddy_entries(ls_dir: &Path) -> Vec<(String, String, &'static str)> {
    let mut result = Vec::new();
    for entry in fs::read_dir(ls_dir).map_err(|e| format!("读取 local_storage 失败: {}", e)).into_iter().flatten() {
        if let Ok(entry) = entry {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with("entry_") || !name.ends_with(".info") {
                continue;
            }
            if let Ok(raw) = fs::read(entry.path()) {
                // 尝试格式1: gzip+base64+引号
                if let Ok(txt) = decode_entry_info(&raw) {
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&txt) {
                        let date = data.get("date").and_then(|v| v.as_str()).unwrap_or("");
                        result.push((name.clone(), txt, "gzip"));
                        let _ = date; // 不再只取date最新的
                    }
                }
                // 尝试格式2: 裸JSON (数组或对象)
                if result.iter().find(|(n,_,_)| n == &name).is_none() {
                    if let Ok(txt) = String::from_utf8(raw.to_vec()) {
                        let trimmed = txt.trim();
                        if trimmed.starts_with('[') || trimmed.starts_with('{') {
                            if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
                                result.push((name, trimmed.to_string(), "json"));
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

/// 向裸JSON格式的entry注入custom-local模型
/// 新版格式: [{userId, data:{models:[...]}, ts}, ...]
fn inject_models_into_json_entry(json_str: &str, new_models: &[serde_json::Value]) -> Result<String, String> {
    let mut data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| format!("解析JSON entry失败: {}", e))?;

    // 新格式: 数组，每个元素 {userId, data:{models:[...]}}
    if let Some(arr) = data.as_array_mut() {
        for item in arr.iter_mut() {
            if let Some(d) = item.get_mut("data").and_then(|v| v.as_object_mut()) {
                if let Some(models) = d.get_mut("models").and_then(|m| m.as_array_mut()) {
                    // 删除旧的 custom-local: 模型
                    models.retain(|m| {
                        let id = m.get("id").and_then(|v| v.as_str()).unwrap_or("");
                        !id.starts_with("custom-local:")
                    });
                    // 清除所有现有模型的 isDefault (防止与我们的冲突)
                    for m in models.iter_mut() {
                        if let Some(obj) = m.as_object_mut() {
                            obj.insert("isDefault".to_string(), serde_json::Value::Bool(false));
                        }
                    }
                    // 添加新的
                    for nm in new_models {
                        models.push(nm.clone());
                    }
                }
            }
        }
    }
    // 旧格式: 对象 {models:[...], ...}
    else if let Some(obj) = data.as_object_mut() {
        if let Some(models) = obj.get_mut("models").and_then(|m| m.as_array_mut()) {
            models.retain(|m| {
                let id = m.get("id").and_then(|v| v.as_str()).unwrap_or("");
                !id.starts_with("custom-local:")
            });
            // 清除所有现有模型的 isDefault
            for m in models.iter_mut() {
                if let Some(obj) = m.as_object_mut() {
                    obj.insert("isDefault".to_string(), serde_json::Value::Bool(false));
                }
            }
            for nm in new_models {
                models.push(nm.clone());
            }
        }
    }

    serde_json::to_string(&data).map_err(|e| format!("序列化失败: {}", e))
}

/// 在 local_storage 目录中找到当前 WorkBuddy 版本对应的 entry_*.info 文件
/// 策略: 取 date 字段最新的 (即当前版本)
fn find_workbuddy_entry(ls_dir: &Path) -> Result<String, String> {
    let mut entries: Vec<(String, String)> = Vec::new(); // (filename, date)

    for entry in fs::read_dir(ls_dir).map_err(|e| format!("读取 local_storage 失败: {}", e))? {
        let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.starts_with("entry_") || !name.ends_with(".info") {
            continue;
        }
        // 尝试解压读取日期
        if let Ok(raw) = fs::read(entry.path()) {
            if let Ok(json_str) = decode_entry_info(&raw) {
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                    let date = data.get("date").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    if !date.is_empty() {
                        entries.push((name, date));
                    }
                }
            }
        }
    }

    if entries.is_empty() {
        return Err("未找到有效的 entry_*.info 文件, 请先启动 WorkBuddy".into());
    }

    // 取日期最新的
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(entries[0].0.clone())
}

/// 解码 entry_*.info: 去引号 -> base64 解码 -> gzip 解压
fn decode_entry_info(raw: &[u8]) -> Result<String, String> {
    use base64::Engine;
    let s = String::from_utf8_lossy(raw).trim().to_string();
    // 去掉外层引号
    let s = if s.starts_with('"') && s.ends_with('"') {
        s[1..s.len()-1].to_string()
    } else {
        s
    };
    // base64 解码
    let b = base64::engine::general_purpose::STANDARD.decode(&s)
        .map_err(|e| format!("base64 解码失败: {}", e))?;
    // gzip 解压
    use flate2::read::GzDecoder;
    use std::io::Read;
    let mut gz = GzDecoder::new(&b[..]);
    let mut txt = String::new();
    gz.read_to_string(&mut txt).map_err(|e| format!("gzip 解压失败: {}", e))?;
    Ok(txt)
}

/// 编码 entry_*.info: JSON -> gzip 压缩 -> base64 编码 -> 加引号
fn encode_entry_info(json_str: &str) -> Result<Vec<u8>, String> {
    use base64::Engine;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;

    // gzip 压缩
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(json_str.as_bytes()).map_err(|e| format!("gzip 压缩失败: {}", e))?;
    let compressed = encoder.finish().map_err(|e| format!("gzip 完成失败: {}", e))?;

    // base64 编码
    let b64 = base64::engine::general_purpose::STANDARD.encode(&compressed);

    // 加引号
    let result = format!("\"{}\"", b64);
    Ok(result.into_bytes())
}

/// Trae 部署: 写入 ~/.trae/settings.json
/// 格式: {"trae.ai.enabled": true, "trae.ai.model": "xxx", "trae.ai.thinking.enabled": true, ...}
fn deploy_trae(config: &DeployConfig) -> Result<String, String> {
    // Trae 配置目录: ~/.trae/
    let trae_dir = dirs::home_dir()
        .ok_or("无法获取用户目录")?
        .join(".trae");

    if !trae_dir.exists() {
        fs::create_dir_all(&trae_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let settings_path = trae_dir.join("settings.json");

    // 读取现有配置
    let mut settings: serde_json::Value = if settings_path.exists() {
        read_json_file(&settings_path)
            .unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    let default_model = config.selected_model_ids.first().unwrap_or(&config.model);

    // 推理等级映射: Trae 用 SOLO/Builder/Chat
    let trae_mode = if config.deep_thinking || config.reasoning_levels.iter().any(|l| l == "max" || l == "xhigh") {
        "SOLO"
    } else if config.reasoning_levels.iter().any(|l| l == "high" || l == "medium") {
        "Builder"
    } else {
        "Chat"
    };

    if let Some(obj) = settings.as_object_mut() {
        obj.insert("trae.ai.enabled".into(), serde_json::json!(true));
        obj.insert("trae.ai.model".into(), serde_json::json!(default_model));
        obj.insert("trae.ai.thinking.enabled".into(), serde_json::json!(config.deep_thinking));
        obj.insert("trae.ai.thinking.budgetTokens".into(), serde_json::json!(32000));
        obj.insert("trae.ai.mode".into(), serde_json::json!(trae_mode));
        obj.insert("trae.ai.apiKey".into(), serde_json::json!(config.api_key));
        obj.insert("trae.ai.baseUrl".into(), serde_json::json!(config.base_url));
        obj.insert("trae.rules.autoLoad".into(), serde_json::json!(true));

        // 写入每个模型的最大上下文 (全平台统一支持)
        let default_mc = config.model_configs.iter().find(|m| {
            m.get("id").and_then(|v| v.as_str()) == Some(default_model.as_str())
        });
        obj.insert("trae.ai.maxInputTokens".into(), serde_json::json!(
            default_mc.and_then(|c| c.get("maxInputTokens")).and_then(|v| v.as_u64()).unwrap_or(1000000)
        ));
        obj.insert("trae.ai.maxOutputTokens".into(), serde_json::json!(
            default_mc.and_then(|c| c.get("maxOutputTokens")).and_then(|v| v.as_u64()).unwrap_or(128000)
        ));
    }

    fs::write(&settings_path, serde_json::to_string_pretty(&settings).unwrap())
        .map_err(|e| format!("写入失败: {}", e))?;

    Ok(format!("Trae: {} 个模型配置已写入 ~/.trae/settings.json (模式: {})", config.selected_model_ids.len(), trae_mode))
}

/// Claw Code 部署: 写入 ~/.openclaw/openclaw.json
/// 格式: {"models": {"providers": {"custom": {type, baseUrl, apiKey, models: [...]}}, "agents": {"defaults": {"thinking": {"level": "max"}}}}}
fn deploy_claw_code(config: &DeployConfig) -> Result<String, String> {
    let claw_dir = dirs::home_dir()
        .ok_or("无法获取用户目录")?
        .join(".openclaw");

    if !claw_dir.exists() {
        fs::create_dir_all(&claw_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let config_path = claw_dir.join("openclaw.json");

    let mut claw_config: serde_json::Value = if config_path.exists() {
        read_json_file(&config_path)
            .unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // 推理等级
    let highest = if config.reasoning_levels.is_empty() {
        config.reasoning_level.clone()
    } else {
        config.reasoning_levels.iter()
            .filter_map(|l| REASONING_ORDER.iter().position(|&r| r == l).map(|p| (l, p)))
            .max_by_key(|(_, p)| *p)
            .map(|(l, _)| l.clone())
            .unwrap_or_else(|| config.reasoning_level.clone())
    };

    // 构建模型列表
    let models_list: Vec<serde_json::Value> = config.selected_model_ids.iter().map(|mid| {
        let mc = config.model_configs.iter().find(|m| {
            m.get("id").and_then(|v| v.as_str()) == Some(mid.as_str())
        });

        let mut m = serde_json::json!({
            "id": mid,
            "name": mc.and_then(|c| c.get("name")).and_then(|v| v.as_str()).unwrap_or(mid),
            "reasoning": mc.and_then(|c| c.get("supportsReasoning")).and_then(|v| v.as_bool()).unwrap_or(true),
        });

        if let Some(obj) = m.as_object_mut() {
            if config.deep_thinking {
                obj.insert("thinking".into(), serde_json::json!({"type": "enabled", "budgetTokens": 32000}));
            }
            obj.insert("maxInputTokens".into(),
                mc.and_then(|c| c.get("maxInputTokens")).cloned()
                    .unwrap_or(serde_json::json!(1000000)));
            obj.insert("maxOutputTokens".into(),
                mc.and_then(|c| c.get("maxOutputTokens")).cloned()
                    .unwrap_or(serde_json::json!(128000)));
        }
        m
    }).collect();

    let default_model = config.selected_model_ids.first().unwrap_or(&config.model);

    if let Some(obj) = claw_config.as_object_mut() {
        // 清除旧的 custom provider
        if let Some(models) = obj.get_mut("models").and_then(|v| v.as_object_mut()) {
            if let Some(providers) = models.get_mut("providers").and_then(|v| v.as_object_mut()) {
                providers.remove("antigravity");
                providers.remove("custom-glm");
            }
        }

        // 写入新的 provider
        if !obj.contains_key("models") {
            obj.insert("models".into(), serde_json::json!({}));
        }
        if let Some(models) = obj.get_mut("models").and_then(|v| v.as_object_mut()) {
            if !models.contains_key("providers") {
                models.insert("providers".into(), serde_json::json!({}));
            }
            if let Some(providers) = models.get_mut("providers").and_then(|v| v.as_object_mut()) {
                providers.insert("antigravity".into(), serde_json::json!({
                    "type": "anthropic",
                    "baseUrl": config.base_url,
                    "apiKey": config.api_key,
                    "models": models_list
                }));
            }
            models.insert("mode".into(), serde_json::json!("merge"));
        }

        // 设置默认 agent
        if !obj.contains_key("agents") {
            obj.insert("agents".into(), serde_json::json!({}));
        }
        if let Some(agents) = obj.get_mut("agents").and_then(|v| v.as_object_mut()) {
            if !agents.contains_key("defaults") {
                agents.insert("defaults".into(), serde_json::json!({}));
            }
            if let Some(defaults) = agents.get_mut("defaults").and_then(|v| v.as_object_mut()) {
                defaults.insert("model".into(), serde_json::json!({"primary": format!("antigravity/{}", default_model)}));
                defaults.insert("thinking".into(), serde_json::json!({"level": to_wb_effort(&highest)}));
            }
        }
    }

    fs::write(&config_path, serde_json::to_string_pretty(&claw_config).unwrap())
        .map_err(|e| format!("写入失败: {}", e))?;

    Ok(format!("Claw Code: {} 个模型已写入 ~/.openclaw/openclaw.json", config.selected_model_ids.len()))
}

/// 读取平台配置 — 检测是否已部署我们的 Key
#[tauri::command]
fn read_platform_config(platform: String) -> Result<Option<serde_json::Value>, String> {
    match platform.as_str() {
        "opencode" => read_opencode_config(),
        "claudecode" => read_claude_code_config(),
        "codebuddy" => read_codebuddy_config(),
        "workbuddy" => read_workbuddy_config(),
        "trae" => read_trae_config(),
        "clawcode" => read_claw_code_config(),
        _ => Ok(None),
    }
}

fn read_opencode_config() -> Result<Option<serde_json::Value>, String> {
    let oc_dir = detect_opencode().path.ok_or("OpenCode 未安装")?;
    let dat_path = PathBuf::from(&oc_dir).join("opencode.global.dat");
    if !dat_path.exists() { return Ok(None); }

    let content = fs::read_to_string(&dat_path).map_err(|e| format!("读取失败: {}", e))?;
    let data: serde_json::Value = serde_json::from_str(&content).map_err(|e| format!("解析失败: {}", e))?;

    // 解析 model 字段（嵌套 JSON 字符串）
    if let Some(model_str) = data.get("model").and_then(|v| v.as_str()) {
        if let Ok(model_obj) = serde_json::from_str::<serde_json::Value>(model_str) {
            // 检查 variant 里有没有 antigravity: 的模型
            if let Some(variants) = model_obj.get("variant").and_then(|v| v.as_object()) {
                for (key, _val) in variants {
                    if key.starts_with("antigravity:") {
                        // 找到已部署的模型
                        return Ok(Some(serde_json::json!({
                            "apiKey": "",
                            "deployed": true,
                            "platform": "opencode",
                            "models": variants.keys().filter(|k| k.starts_with("antigravity:")).collect::<Vec<_>>(),
                        })));
                    }
                }
            }
        }
    }
    Ok(None)
}

fn read_claude_code_config() -> Result<Option<serde_json::Value>, String> {
    let cc_dir = detect_claude_code().path.ok_or("Claude Code 未安装")?;
    let settings_path = PathBuf::from(&cc_dir).join("settings.json");
    if !settings_path.exists() { return Ok(None); }

    let content = fs::read_to_string(&settings_path).map_err(|e| format!("读取失败: {}", e))?;
    let data: serde_json::Value = serde_json::from_str(&content).map_err(|e| format!("解析失败: {}", e))?;

    if let Some(env) = data.get("env") {
        let base_url = env.get("ANTHROPIC_BASE_URL").and_then(|v| v.as_str()).unwrap_or("");
        let auth_token = env.get("ANTHROPIC_AUTH_TOKEN").and_then(|v| v.as_str()).unwrap_or("");
        let model = env.get("ANTHROPIC_MODEL").and_then(|v| v.as_str()).unwrap_or("");
        let effort = data.get("effortLevel").and_then(|v| v.as_str()).unwrap_or("");

        if base_url.contains("2bbb.cn") || auth_token.starts_with("fm-") {
            return Ok(Some(serde_json::json!({
                "apiKey": auth_token,
                "baseUrl": base_url,
                "model": model,
                "effortLevel": effort,
                "deployed": true,
                "platform": "claudecode",
            })));
        }
    }
    Ok(None)
}

fn read_codebuddy_config() -> Result<Option<serde_json::Value>, String> {
    let cb_path = detect_codebuddy().path.ok_or("CodeBuddy 未安装")?;
    let base = PathBuf::from(&cb_path);
    let config_path = base.join("glm_deploy_config.json");
    if config_path.exists() {
        let content = fs::read_to_string(&config_path).map_err(|e| format!("读取失败: {}", e))?;
        let data: serde_json::Value = serde_json::from_str(&content).unwrap_or(serde_json::json!({}));
        return Ok(Some(serde_json::json!({
            "apiKey": data.get("api_key").and_then(|v| v.as_str()).unwrap_or(""),
            "deployed": true,
            "platform": "codebuddy",
        })));
    }
    Ok(None)
}

fn read_workbuddy_config() -> Result<Option<serde_json::Value>, String> {
    let wb_dir = dirs::home_dir().ok_or("无法获取用户目录")?.join(".workbuddy");

    // 1. 先检查 models.json (WorkBuddy 自定义模型模板)
    let models_path = wb_dir.join("models.json");
    if models_path.exists() {
        if let Ok(content) = fs::read_to_string(&models_path) {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(arr) = data.as_array() {
                    for m in arr {
                        let key = m.get("apiKey").and_then(|v| v.as_str()).unwrap_or("");
                        let url = m.get("url").and_then(|v| v.as_str()).unwrap_or("");
                        if key.starts_with("fm-") || url.contains("2bbb.cn") {
                            return Ok(Some(serde_json::json!({
                                "apiKey": key,
                                "baseUrl": url,
                                "deployed": true,
                                "platform": "workbuddy",
                            })));
                        }
                    }
                }
            }
        }
    }

    // 2. 再检查 entry_*.info (WorkBuddy 实际运行时读取的配置)
    let ls_dir = wb_dir.join("local_storage");
    if ls_dir.exists() {
        for entry in fs::read_dir(&ls_dir).map_err(|e| format!("读取 local_storage 失败: {}", e))? {
            let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with("entry_") || !name.ends_with(".info") {
                continue;
            }
            if let Ok(raw) = fs::read(entry.path()) {
                if let Ok(json_str) = decode_entry_info(&raw) {
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        if let Some(models) = data.get("models").and_then(|m| m.as_array()) {
                            for m in models {
                                let mid = m.get("id").and_then(|v| v.as_str()).unwrap_or("");
                                let key = m.get("apiKey").and_then(|v| v.as_str()).unwrap_or("");
                                let url = m.get("url").and_then(|v| v.as_str()).or_else(|| m.get("baseUrl").and_then(|v| v.as_str())).unwrap_or("");
                                if mid.starts_with("custom-local:") && (key.starts_with("fm-") || url.contains("2bbb.cn")) {
                                    return Ok(Some(serde_json::json!({
                                        "apiKey": key,
                                        "baseUrl": url,
                                        "deployed": true,
                                        "platform": "workbuddy",
                                    })));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

fn read_trae_config() -> Result<Option<serde_json::Value>, String> {
    let trae_dir = dirs::home_dir().map(|d| d.join(".trae"));
    if let Some(dir) = trae_dir {
        let settings = dir.join("settings.json");
        if settings.exists() {
            if let Ok(content) = fs::read_to_string(&settings) {
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                    let key = data.get("trae.ai.apiKey").and_then(|v| v.as_str()).unwrap_or("");
                    if key.starts_with("fm-") {
                        return Ok(Some(serde_json::json!({
                            "apiKey": key,
                            "deployed": true,
                            "platform": "trae",
                        })));
                    }
                }
            }
        }
    }
    Ok(None)
}

fn read_claw_code_config() -> Result<Option<serde_json::Value>, String> {
    let claw_dir = dirs::home_dir().map(|d| d.join(".openclaw"));
    if let Some(dir) = claw_dir {
        let config_path = dir.join("openclaw.json");
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(providers) = data.get("models").and_then(|m| m.get("providers")).and_then(|p| p.as_object()) {
                        for (_, provider) in providers {
                            let key = provider.get("apiKey").and_then(|v| v.as_str()).unwrap_or("");
                            if key.starts_with("fm-") {
                                return Ok(Some(serde_json::json!({
                                    "apiKey": key,
                                    "deployed": true,
                                    "platform": "clawcode",
                                })));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

/// 一键自检 — 检测所有可能的问题
#[tauri::command]
fn run_diagnostics() -> Vec<DiagnosticItem> {
    let mut items = Vec::new();

    // 1. 网络连通性
    items.push(check_network());

    // 2. API Key 有效性（从 localStorage 读取由前端处理，Rust 检查文件）

    // 3. 已安装平台检测
    let platforms = detect_all_platforms();
    let mut installed_count = 0;
    for (key, info) in &platforms {
        if info.installed {
            installed_count += 1;
            // 检查各平台配置是否正确
            match key.as_str() {
                "opencode" => items.push(check_opencode_config()),
                "claudecode" => items.push(check_claude_code_config()),
                "workbuddy" => items.push(check_workbuddy_config()),
                "codebuddy" => items.push(check_codebuddy_config()),
                "trae" => items.push(check_trae_config()),
                _ => {}
            }
        }
    }

    if installed_count == 0 {
        items.push(DiagnosticItem {
            id: "no_platform".into(),
            category: "平台".into(),
            title: "未检测到任何已安装平台".into(),
            status: "error".into(),
            detail: "请先安装 OpenCode / Claude Code / CodeBuddy / WorkBuddy / Trae 中的至少一个".into(),
            fixable: true,
            fix_action: "open_download".into(),
        });
    }

    // 4. 磁盘空间
    items.push(check_disk_space());

    // 5. 配置文件完整性
    items.push(check_config_files());

    // 6. 系统代理检测
    items.push(check_system_proxy());

    items
}

/// 诊断项
#[derive(Serialize, Clone)]
struct DiagnosticItem {
    id: String,
    category: String,
    title: String,
    status: String, // ok / warning / error
    detail: String,
    fixable: bool,
    fix_action: String, // 修复动作标识
}

fn check_network() -> DiagnosticItem {
    DiagnosticItem {
        id: "network".into(),
        category: "网络".into(),
        title: "网络连通性".into(),
        status: "ok".into(),
        detail: "网络检查由前端执行".into(),
        fixable: false,
        fix_action: "".into(),
    }
}

/// 检测系统代理设置（可能导致127.0.0.1连不上）
fn check_system_proxy() -> DiagnosticItem {
    #[cfg(target_os = "windows")]
    {
        // 检查注册表 ProxyEnable
        if let Ok(output) = std::process::Command::new("reg")
            .args(["query", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", "/v", "ProxyEnable"])
            .no_window()
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("0x1") {
                // 代理开启，检查代理服务器地址
                let proxy_server = if let Ok(o2) = std::process::Command::new("reg")
                    .args(["query", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", "/v", "ProxyServer"])
                    .no_window()
                    .output()
                {
                    let s = String::from_utf8_lossy(&o2.stdout);
                    s.lines().find(|l| l.contains("ProxyServer"))
                        .and_then(|l| l.split_whitespace().last())
                        .unwrap_or("unknown").to_string()
                } else { "unknown".to_string() };
                return DiagnosticItem {
                    id: "system_proxy".into(),
                    category: "网络".into(),
                    title: "系统代理已开启（可能导致连不上服务器）".into(),
                    status: "warning".into(),
                    detail: format!("代理服务器: {} — 如遇连接问题可一键清除", proxy_server),
                    fixable: true,
                    fix_action: "fix_proxy".into(),
                };
            }
        }
        // 检查环境变量代理
        for var in &["HTTP_PROXY", "HTTPS_PROXY", "http_proxy", "https_proxy", "ALL_PROXY"] {
            if let Ok(val) = std::env::var(var) {
                if !val.is_empty() {
                    return DiagnosticItem {
                        id: "env_proxy".into(),
                        category: "网络".into(),
                        title: format!("环境变量 {} 已设置（可能导致连不上）", var),
                        status: "warning".into(),
                        detail: format!("值: {} — 可一键清除系统代理", val),
                        fixable: true,
                        fix_action: "fix_proxy".into(),
                    };
                }
            }
        }
        DiagnosticItem {
            id: "system_proxy".into(),
            category: "网络".into(),
            title: "系统代理未开启".into(),
            status: "ok".into(),
            detail: "无代理干扰".into(),
            fixable: false,
            fix_action: "".into(),
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        for var in &["HTTP_PROXY", "HTTPS_PROXY", "http_proxy", "https_proxy"] {
            if let Ok(val) = std::env::var(var) {
                if !val.is_empty() {
                    return DiagnosticItem {
                        id: "env_proxy".into(),
                        category: "网络".into(),
                        title: format!("环境变量 {} 已设置", var),
                        status: "warning".into(),
                        detail: format!("值: {}", val),
                        fixable: true,
                        fix_action: "fix_proxy".into(),
                    };
                }
            }
        }
        DiagnosticItem { id: "system_proxy".into(), category: "网络".into(), title: "无代理干扰".into(), status: "ok".into(), detail: "".into(), fixable: false, fix_action: "".into() }
    }
}

fn check_opencode_config() -> DiagnosticItem {
    let oc = detect_opencode();
    if !oc.installed {
        return DiagnosticItem { id: "oc_not_installed".into(), category: "OpenCode".into(), title: "OpenCode 未安装".into(), status: "warning".into(), detail: "未检测到 OpenCode".into(), fixable: false, fix_action: "".into() };
    }
    let path = match oc.path {
        Some(p) => p,
        None => return DiagnosticItem { id: "oc_no_path".into(), category: "OpenCode".into(), title: "检测到进程但未找到配置路径".into(), status: "warning".into(), detail: "请先部署配置".into(), fixable: true, fix_action: "deploy".into() },
    };
    let dir = PathBuf::from(path);
    let dat = dir.join("opencode.global.dat");
    if !dat.exists() {
        return DiagnosticItem { id: "oc_no_config".into(), category: "OpenCode".into(), title: "配置文件缺失".into(), status: "warning".into(), detail: "opencode.global.dat 不存在，请先部署".into(), fixable: true, fix_action: "deploy".into() };
    }
    match fs::read_to_string(&dat) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(data) => {
                    if let Some(model_str) = data.get("model").and_then(|v| v.as_str()) {
                        if model_str.contains("antigravity:") {
                            return DiagnosticItem { id: "oc_ok".into(), category: "OpenCode".into(), title: "配置正常".into(), status: "ok".into(), detail: "已部署 antigravity 模型配置".into(), fixable: false, fix_action: "".into() };
                        }
                    }
                    DiagnosticItem { id: "oc_no_deploy".into(), category: "OpenCode".into(), title: "未部署 API 配置".into(), status: "warning".into(), detail: "OpenCode 已安装但未配置我们的 API".into(), fixable: true, fix_action: "deploy".into() }
                }
                Err(e) => DiagnosticItem { id: "oc_parse_error".into(), category: "OpenCode".into(), title: "配置文件损坏".into(), status: "error".into(), detail: format!("解析失败: {}", e), fixable: true, fix_action: "restore_backup".into() }
            }
        }
        Err(e) => DiagnosticItem { id: "oc_read_error".into(), category: "OpenCode".into(), title: "配置文件读取失败".into(), status: "error".into(), detail: format!("{}", e), fixable: false, fix_action: "".into() }
    }
}

fn check_claude_code_config() -> DiagnosticItem {
    let cc = detect_claude_code();
    if !cc.installed {
        return DiagnosticItem { id: "cc_not_installed".into(), category: "Claude Code".into(), title: "Claude Code 未安装".into(), status: "warning".into(), detail: "".into(), fixable: false, fix_action: "".into() };
    }
    let path = match cc.path {
        Some(p) => p,
        None => return DiagnosticItem { id: "cc_no_path".into(), category: "Claude Code".into(), title: "检测到进程但未找到配置路径".into(), status: "warning".into(), detail: "请先部署配置".into(), fixable: true, fix_action: "deploy".into() },
    };
    let dir = PathBuf::from(path);
    let settings = dir.join("settings.json");
    if !settings.exists() {
        return DiagnosticItem { id: "cc_no_config".into(), category: "Claude Code".into(), title: "settings.json 不存在".into(), status: "warning".into(), detail: "请先部署".into(), fixable: true, fix_action: "deploy".into() };
    }
    match fs::read_to_string(&settings) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(data) => {
                    let base_url = data.get("env").and_then(|e| e.get("ANTHROPIC_BASE_URL")).and_then(|v| v.as_str()).unwrap_or("");
                    if base_url.contains("2bbb.cn") {
                        return DiagnosticItem { id: "cc_ok".into(), category: "Claude Code".into(), title: "配置正常".into(), status: "ok".into(), detail: format!("Base URL: {}", base_url), fixable: false, fix_action: "".into() };
                    }
                    DiagnosticItem { id: "cc_no_deploy".into(), category: "Claude Code".into(), title: "未部署我们的 API".into(), status: "warning".into(), detail: "当前配置指向其他服务".into(), fixable: true, fix_action: "deploy".into() }
                }
                Err(e) => DiagnosticItem { id: "cc_parse_error".into(), category: "Claude Code".into(), title: "settings.json 损坏".into(), status: "error".into(), detail: format!("{}", e), fixable: true, fix_action: "restore_backup".into() }
            }
        }
        Err(e) => DiagnosticItem { id: "cc_read_error".into(), category: "Claude Code".into(), title: "读取失败".into(), status: "error".into(), detail: format!("{}", e), fixable: false, fix_action: "".into() }
    }
}

fn check_workbuddy_config() -> DiagnosticItem {
    let wb = detect_workbuddy();
    if !wb.installed {
        return DiagnosticItem { id: "wb_not_installed".into(), category: "WorkBuddy".into(), title: "WorkBuddy 未安装".into(), status: "warning".into(), detail: "".into(), fixable: false, fix_action: "".into() };
    }
    let wb_dir = dirs::home_dir().map(|d| d.join(".workbuddy"));
    let wb_dir = match wb_dir {
        Some(d) if d.exists() => d,
        _ => return DiagnosticItem { id: "wb_no_dir".into(), category: "WorkBuddy".into(), title: ".workbuddy 目录不存在".into(), status: "warning".into(), detail: "请先启动一次 WorkBuddy".into(), fixable: false, fix_action: "".into() },
    };

    // 1. 先检查 models.json
    let models_path = wb_dir.join("models.json");
    if models_path.exists() {
        if let Ok(content) = fs::read_to_string(&models_path) {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(arr) = data.as_array() {
                    for m in arr {
                        let key = m.get("apiKey").and_then(|v| v.as_str()).unwrap_or("");
                        if key.starts_with("fm-") {
                            let preview = if key.len() > 10 { format!("{}...", &key[..10]) } else { format!("{}...", key) };
                            return DiagnosticItem { id: "wb_ok".into(), category: "WorkBuddy".into(), title: "配置正常".into(), status: "ok".into(), detail: format!("Key: {}", preview), fixable: false, fix_action: "".into() };
                        }
                    }
                }
            }
        }
    }

    // 2. 再检查 entry_*.info
    let ls_dir = wb_dir.join("local_storage");
    if !ls_dir.exists() {
        return DiagnosticItem { id: "wb_no_ls".into(), category: "WorkBuddy".into(), title: "local_storage 不存在".into(), status: "warning".into(), detail: "请先启动一次 WorkBuddy".into(), fixable: false, fix_action: "".into() };
    }

    let mut found_ok = false;
    let mut found_key = String::new();
    let mut found_count = 0;
    let mut any_entry = false;

    for entry in fs::read_dir(&ls_dir).unwrap() {
        let entry = match entry { Ok(e) => e, Err(_) => continue };
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.starts_with("entry_") || !name.ends_with(".info") { continue }
        any_entry = true;
        if let Ok(raw) = fs::read(entry.path()) {
            if let Ok(json_str) = decode_entry_info(&raw) {
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                    if let Some(models) = data.get("models").and_then(|m| m.as_array()) {
                        for m in models {
                            let mid = m.get("id").and_then(|v| v.as_str()).unwrap_or("");
                            let key = m.get("apiKey").and_then(|v| v.as_str()).unwrap_or("");
                            let url = m.get("url").and_then(|v| v.as_str()).or_else(|| m.get("baseUrl").and_then(|v| v.as_str())).unwrap_or("");
                            if mid.starts_with("custom-local:") && (key.starts_with("fm-") || url.contains("2bbb.cn")) {
                                found_count += 1;
                                if !found_ok {
                                    found_ok = true;
                                    found_key = key.to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !any_entry {
        return DiagnosticItem { id: "wb_no_entry".into(), category: "WorkBuddy".into(), title: "未找到 entry 配置".into(), status: "warning".into(), detail: "请先启动一次 WorkBuddy".into(), fixable: false, fix_action: "".into() };
    }
    if found_ok {
        let preview = if found_key.len() > 10 { format!("{}...", &found_key[..10]) } else { format!("{}...", found_key) };
        return DiagnosticItem { id: "wb_ok".into(), category: "WorkBuddy".into(), title: "配置正常".into(), status: "ok".into(), detail: format!("Key: {} ({}个模型)", preview, found_count), fixable: false, fix_action: "".into() };
    }
    DiagnosticItem { id: "wb_no_deploy".into(), category: "WorkBuddy".into(), title: "未部署我们的 API".into(), status: "warning".into(), detail: "请先部署".into(), fixable: true, fix_action: "deploy".into() }
}

fn check_codebuddy_config() -> DiagnosticItem {
    let cb = detect_codebuddy();
    if !cb.installed {
        return DiagnosticItem { id: "cb_not_installed".into(), category: "CodeBuddy".into(), title: "CodeBuddy 未安装".into(), status: "warning".into(), detail: "".into(), fixable: false, fix_action: "".into() };
    }
    let path = match cb.path {
        Some(p) => p,
        None => return DiagnosticItem { id: "cb_no_path".into(), category: "CodeBuddy".into(), title: "未找到配置路径".into(), status: "warning".into(), detail: "请先部署".into(), fixable: true, fix_action: "deploy".into() },
    };
    let models = PathBuf::from(path).join("models.json");
    if !models.exists() {
        return DiagnosticItem { id: "cb_no_config".into(), category: "CodeBuddy".into(), title: "models.json 不存在".into(), status: "warning".into(), detail: "请先部署".into(), fixable: true, fix_action: "deploy".into() };
    }
    match fs::read_to_string(&models) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(data) => {
                    if let Some(arr) = data.get("models").and_then(|v| v.as_array()) {
                        for m in arr {
                            let key = m.get("apiKey").and_then(|v| v.as_str()).unwrap_or("");
                            if key.starts_with("fm-") {
                                let preview = if key.len() > 10 { format!("{}...", &key[..10]) } else { format!("{}...", key) };
                                return DiagnosticItem { id: "cb_ok".into(), category: "CodeBuddy".into(), title: "配置正常".into(), status: "ok".into(), detail: format!("Key: {}", preview), fixable: false, fix_action: "".into() };
                            }
                        }
                    }
                    DiagnosticItem { id: "cb_no_deploy".into(), category: "CodeBuddy".into(), title: "未部署我们的 API".into(), status: "warning".into(), detail: "models.json 中未找到 fm- 开头的 Key".into(), fixable: true, fix_action: "deploy".into() }
                }
                Err(e) => DiagnosticItem { id: "cb_parse_error".into(), category: "CodeBuddy".into(), title: "models.json 损坏".into(), status: "error".into(), detail: format!("{}", e), fixable: true, fix_action: "restore_backup".into() }
            }
        }
        Err(e) => DiagnosticItem { id: "cb_read_error".into(), category: "CodeBuddy".into(), title: "读取失败".into(), status: "error".into(), detail: format!("{}", e), fixable: false, fix_action: "".into() }
    }
}

fn check_trae_config() -> DiagnosticItem {
    let tr = detect_trae();
    if !tr.installed {
        return DiagnosticItem { id: "tr_not_installed".into(), category: "Trae".into(), title: "Trae 未安装".into(), status: "warning".into(), detail: "".into(), fixable: false, fix_action: "".into() };
    }
    DiagnosticItem { id: "tr_installed".into(), category: "Trae".into(), title: "已安装".into(), status: "ok".into(), detail: "需手动导入配置".into(), fixable: false, fix_action: "".into() }
}

fn check_disk_space() -> DiagnosticItem {
    DiagnosticItem { id: "disk".into(), category: "系统".into(), title: "磁盘空间".into(), status: "ok".into(), detail: "由前端检查".into(), fixable: false, fix_action: "".into() }
}

fn check_config_files() -> DiagnosticItem {
    // 检查 OpenCode 备份文件是否存在
    let oc_dir = dirs::config_dir().map(|d| d.join("ai.opencode.desktop"));
    if let Some(dir) = oc_dir {
        if dir.exists() {
            let bak = dir.join("opencode.global.dat.bak");
            if bak.exists() {
                return DiagnosticItem { id: "backup_ok".into(), category: "系统".into(), title: "配置备份存在".into(), status: "ok".into(), detail: "OpenCode 配置备份文件存在".into(), fixable: false, fix_action: "".into() };
            }
            return DiagnosticItem { id: "no_backup".into(), category: "系统".into(), title: "无配置备份".into(), status: "warning".into(), detail: "建议备份后再部署".into(), fixable: true, fix_action: "backup".into() };
        }
    }
    DiagnosticItem { id: "no_config_dir".into(), category: "系统".into(), title: "无配置目录".into(), status: "ok".into(), detail: "首次使用".into(), fixable: false, fix_action: "".into() }
}

/// 一键修复
#[tauri::command]
fn run_fix(fix_action: String) -> Result<String, String> {
    match fix_action.as_str() {
        "open_download" => {
            // 打开下载页面
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("cmd").no_window()
                    .args(["/c", "start https://opencode.ai"])
                    .spawn();
            }
            Ok("已打开下载页面".into())
        }
        "backup" => {
            // 备份 OpenCode 配置
            let oc_dir = dirs::config_dir().map(|d| d.join("ai.opencode.desktop"));
            if let Some(dir) = oc_dir {
                let dat = dir.join("opencode.global.dat");
                let bak = dir.join("opencode.global.dat.bak");
                if dat.exists() && !bak.exists() {
                    fs::copy(&dat, &bak).map_err(|e| format!("备份失败: {}", e))?;
                    return Ok("配置已备份".into());
                }
            }
            Ok("无需备份".into())
        }
        "restore_backup" => {
            // 从备份恢复
            let oc_dir = dirs::config_dir().map(|d| d.join("ai.opencode.desktop"));
            if let Some(dir) = oc_dir {
                let dat = dir.join("opencode.global.dat");
                let bak = dir.join("opencode.global.dat.bak");
                if bak.exists() {
                    fs::copy(&bak, &dat).map_err(|e| format!("恢复失败: {}", e))?;
                    return Ok("配置已从备份恢复".into());
                }
            }
            Err("未找到备份文件".into())
        }
        "deploy" => {
            Ok("请通过部署向导重新部署".into())
        }
        "clear_cache" => {
            // 清理 OpenCode 缓存
            let oc_dir = dirs::config_dir().map(|d| d.join("ai.opencode.desktop"));
            if let Some(dir) = oc_dir {
                let cache = dir.join("Cache");
                if cache.exists() {
                    let _ = fs::remove_dir_all(&cache);
                }
                let gpucache = dir.join("GPUCache");
                if gpucache.exists() {
                    let _ = fs::remove_dir_all(&gpucache);
                }
            }
            Ok("缓存已清理".into())
        }
        "fix_proxy" => {
            // 静默清除系统代理 + 环境变量代理 + WinHTTP代理
            #[cfg(target_os = "windows")]
            {
                // 1. 关闭注册表代理
                let _ = std::process::Command::new("reg")
                    .args(["add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
                           "/v", "ProxyEnable", "/t", "REG_DWORD", "/d", "0", "/f"])
                    .no_window().output();
                let _ = std::process::Command::new("reg")
                    .args(["add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
                           "/v", "ProxyServer", "/t", "REG_SZ", "/d", "", "/f"])
                    .no_window().output();
                // 2. 重置 WinHTTP 代理
                let _ = std::process::Command::new("netsh")
                    .args(["winhttp", "reset", "proxy"])
                    .no_window().output();
            }
            Ok("PROXY_FIXED".into())
        }
        _ => Err("未知修复操作".into()),
    }
}

/// 备份配置文件
#[tauri::command]
fn backup_configs() -> Result<String, String> {
    let mut backed_up = Vec::new();

    // OpenCode
    if let Some(dir) = dirs::config_dir().map(|d| d.join("ai.opencode.desktop")) {
        let dat = dir.join("opencode.global.dat");
        if dat.exists() {
            let bak = dir.join("opencode.global.dat.launcher_bak");
            fs::copy(&dat, &bak).map_err(|e| format!("备份失败: {}", e))?;
            backed_up.push("OpenCode");
        }
    }

    // Claude Code
    if let Some(dir) = dirs::home_dir().map(|d| d.join(".claude")) {
        let settings = dir.join("settings.json");
        if settings.exists() {
            let bak = dir.join("settings.json.launcher_bak");
            fs::copy(&settings, &bak).map_err(|e| format!("备份失败: {}", e))?;
            backed_up.push("Claude Code");
        }
    }

    // WorkBuddy
    if let Some(dir) = dirs::home_dir().map(|d| d.join(".workbuddy")) {
        let models = dir.join("models.json");
        if models.exists() {
            let bak = dir.join("models.json.launcher_bak");
            fs::copy(&models, &bak).map_err(|e| format!("备份失败: {}", e))?;
            backed_up.push("WorkBuddy");
        }
    }

    if backed_up.is_empty() {
        Ok("无需备份的配置文件".into())
    } else {
        Ok(format!("已备份: {}", backed_up.join(", ")))
    }
}

/// 打开外部 URL（用系统默认浏览器）
#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // 用 PowerShell Start-Process 打开 URL，避免 cmd start 对 & 的截断问题
        std::process::Command::new("powershell").no_window()
            .args(["-Command", &format!("Start-Process \"{}\"", url)])
            .spawn()
            .map_err(|e| format!("打开失败: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(&url).spawn().map_err(|e| format!("打开失败: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(&url).spawn().map_err(|e| format!("打开失败: {}", e))?;
    }
    Ok(())
}

/// 清除平台部署配置
#[tauri::command]
fn clear_platform_deploy(platform: String, reasoning_level: String) -> Result<String, String> {
    match platform.as_str() {
        "opencode" => {
            // 清除 opencode.json 里的 antigravity provider
            let config_path = dirs::home_dir()
                .ok_or("无法获取用户目录")?
                .join(".config/opencode/opencode.json");
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(mut data) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(provider) = data.get_mut("provider").and_then(|v| v.as_object_mut()) {
                            provider.remove("antigravity");
                        }
                        if let Some(obj) = data.as_object_mut() {
                            obj.remove("model");
                        }
                        fs::write(&config_path, serde_json::to_string_pretty(&data).unwrap()).map_err(|e| format!("写入失败: {}", e))?;
                    }
                }
            }
            // 也清除 global.dat 里的 variant
            let dat_path = dirs::config_dir().map(|d| d.join("ai.opencode.desktop/opencode.global.dat"));
            if let Some(dp) = dat_path {
                if dp.exists() {
                    if let Ok(content) = fs::read_to_string(&dp) {
                        if let Ok(mut data) = serde_json::from_str::<serde_json::Value>(&content) {
                            if let Some(model_str) = data.get("model").and_then(|v| v.as_str()).map(|s| s.to_string()) {
                                if let Ok(mut model_obj) = serde_json::from_str::<serde_json::Value>(&model_str) {
                                    if let Some(variants) = model_obj.get_mut("variant").and_then(|v| v.as_object_mut()) {
                                        let keys: Vec<String> = variants.keys().filter(|k| k.starts_with("antigravity:")).cloned().collect();
                                        for k in keys { variants.remove(&k); }
                                    }
                                    data.as_object_mut().map(|o| {
                                        o.insert("model".into(), serde_json::Value::String(serde_json::to_string(&model_obj).unwrap_or_default()));
                                    });
                                    let _ = fs::write(&dp, serde_json::to_string_pretty(&data).unwrap());
                                }
                            }
                        }
                    }
                }
            }
            Ok("OpenCode 配置已清除".into())
        }
        "claudecode" => {
            let settings_path = dirs::home_dir().ok_or("无法获取用户目录")?.join(".claude/settings.json");
            if settings_path.exists() {
                if let Ok(content) = fs::read_to_string(&settings_path) {
                    if let Ok(mut data) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(obj) = data.as_object_mut() {
                            obj.remove("env");
                            obj.remove("effortLevel");
                            obj.remove("thinking");
                            obj.remove("alwaysThinkingEnabled");
                        }
                        fs::write(&settings_path, serde_json::to_string_pretty(&data).unwrap()).map_err(|e| format!("写入失败: {}", e))?;
                    }
                }
            }
            Ok("Claude Code 配置已清除".into())
        }
        "codebuddy" => {
            let models_path = dirs::home_dir().ok_or("无法获取用户目录")?.join(".codebuddy/models.json");
            if models_path.exists() {
                fs::write(&models_path, r#"{"models":[]}"#).map_err(|e| format!("写入失败: {}", e))?;
            }
            Ok("CodeBuddy CN 配置已清除".into())
        }
        "workbuddy" => {
            // 关闭 WorkBuddy 避免覆盖
            kill_workbuddy_processes();
            // 从 entry_*.info 中移除我们的 custom-local: 模型
            let wb_dir = dirs::home_dir().ok_or("无法获取用户目录")?.join(".workbuddy");
            let ls_dir = wb_dir.join("local_storage");
            if ls_dir.exists() {
                for entry in fs::read_dir(&ls_dir).map_err(|e| format!("读取 local_storage 失败: {}", e))? {
                    let entry = match entry { Ok(e) => e, Err(_) => continue };
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !name.starts_with("entry_") || !name.ends_with(".info") { continue }
                    if let Ok(raw) = fs::read(entry.path()) {
                        if let Ok(json_str) = decode_entry_info(&raw) {
                            if let Ok(mut data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                                let mut changed = false;
                                if let Some(models) = data.get_mut("models").and_then(|m| m.as_array_mut()) {
                                    let before = models.len();
                                    models.retain(|m| {
                                        let id = m.get("id").and_then(|v| v.as_str()).unwrap_or("");
                                        !id.starts_with("custom-local:")
                                    });
                                    if models.len() != before { changed = true; }
                                }
                                if changed {
                                    let new_json = serde_json::to_string(&data).map_err(|e| format!("序列化失败: {}", e))?;
                                    let encoded = encode_entry_info(&new_json)?;
                                    fs::write(entry.path(), encoded).map_err(|e| format!("写入失败: {}", e))?;
                                }
                            }
                        }
                    }
                }
            }
            // 也清除 models.json (旧版兼容)
            let models_path = wb_dir.join("models.json");
            if models_path.exists() {
                let _ = fs::write(&models_path, "[]");
            }
            Ok("WorkBuddy 配置已清除".into())
        }
        "clawcode" => {
            let config_path = dirs::home_dir().ok_or("无法获取用户目录")?.join(".openclaw/openclaw.json");
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(mut data) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(providers) = data.get_mut("models").and_then(|m| m.get_mut("providers")).and_then(|p| p.as_object_mut()) {
                            providers.remove("antigravity");
                            providers.remove("custom-glm");
                        }
                        fs::write(&config_path, serde_json::to_string_pretty(&data).unwrap()).map_err(|e| format!("写入失败: {}", e))?;
                    }
                }
            }
            Ok("Claw Code 配置已清除".into())
        }
        "trae" => {
            let settings_path = dirs::home_dir().ok_or("无法获取用户目录")?.join(".trae/settings.json");
            if settings_path.exists() {
                if let Ok(content) = fs::read_to_string(&settings_path) {
                    if let Ok(mut data) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(obj) = data.as_object_mut() {
                            obj.remove("trae.ai.apiKey");
                            obj.remove("trae.ai.baseUrl");
                            obj.remove("trae.ai.thinking.enabled");
                            obj.remove("trae.ai.thinking.budgetTokens");
                            obj.remove("trae.ai.mode");
                        }
                        fs::write(&settings_path, serde_json::to_string_pretty(&data).unwrap()).map_err(|e| format!("写入失败: {}", e))?;
                    }
                }
            }
            Ok("Trae 配置已清除".into())
        }
        _ => Err("未知平台".into()),
    }
}

/// 重启已部署的软件
#[tauri::command]
fn restart_app(platform: String) -> Result<String, String> {
    match platform.as_str() {
        "opencode" => {
            #[cfg(target_os = "windows")]
            {
                // 先杀掉旧进程再启动
                let _ = std::process::Command::new("taskkill").no_window()
                    .args(["/f", "/im", "OpenCode.exe"])
                    .spawn();
                std::thread::sleep(std::time::Duration::from_secs(2));

                // 搜索多个可能的安装路径
                let candidates: Vec<PathBuf> = vec![
                    // 注册表发现的路径
                    dirs::data_dir().map(|d| d.join("Programs/@opencode-aidesktop/OpenCode.exe")),
                    // 备选路径
                    dirs::data_dir().map(|d| d.join("Programs/opencode/OpenCode.exe")),
                    dirs::data_dir().map(|d| d.join("Programs/OpenCode/OpenCode.exe")),
                ].into_iter().flatten().collect();

                for exe in &candidates {
                    if exe.exists() {
                        std::process::Command::new(exe).spawn()
                            .map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("OpenCode 已重启".into());
                    }
                }

                // 也尝试从开始菜单快捷方式启动
                let shortcut = dirs::config_dir()
                    .map(|d| d.join("Microsoft/Windows/Start Menu/Programs/OpenCode.lnk"));
                if let Some(sc) = shortcut {
                    if sc.exists() {
                        std::process::Command::new("cmd").no_window()
                            .args(["/c", "start", "", &sc.to_string_lossy()])
                            .spawn()
                            .map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("OpenCode 已启动".into());
                    }
                }

                // 最后尝试桌面快捷方式
                if let Some(desktop) = dirs::desktop_dir() {
                    let sc = desktop.join("OpenCode.lnk");
                    if sc.exists() {
                        std::process::Command::new("cmd").no_window()
                            .args(["/c", "start", "", &sc.to_string_lossy()])
                            .spawn()
                            .map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("OpenCode 已启动".into());
                    }
                }

                Err("未找到 OpenCode 可执行文件，请手动启动".into())
            }
            #[cfg(not(target_os = "windows"))]
            {
                // macOS: /Applications/OpenCode.app 或 /opt/homebrew/bin/opencode
                #[cfg(target_os = "macos")]
                {
                    let candidates = vec![
                        PathBuf::from("/Applications/OpenCode.app"),
                        PathBuf::from("/Applications/OpenCode.app/Contents/MacOS/OpenCode"),
                    ];
                    for exe in &candidates {
                        if exe.exists() {
                            std::process::Command::new("open").arg(exe).spawn().map_err(|e| format!("启动失败: {}", e))?;
                            return Ok("OpenCode 已启动".into());
                        }
                    }
                    Err("未找到 OpenCode，请手动启动".into())
                }
                #[cfg(not(target_os = "macos"))]
                Err("请手动启动 OpenCode".into())
            }
        }
        "claudecode" => {
            // Claude Code 是命令行工具，不需要重启 GUI
            Ok("Claude Code 配置已生效，新开终端即可使用".into())
        }
        "codebuddy" => {
            // CodeBuddy CN 独立客户端
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("taskkill").no_window()
                    .args(["/f", "/im", "CodeBuddy CN.exe"])
                    .spawn();
                std::thread::sleep(std::time::Duration::from_secs(2));

                // 搜索 exe
                let candidates: Vec<PathBuf> = vec![
                    dirs::data_dir().map(|d| d.join("Programs/CodeBuddy CN/CodeBuddy CN.exe")),
                    dirs::data_dir().map(|d| d.join("Local/Programs/CodeBuddy CN/CodeBuddy CN.exe")),
                ].into_iter().flatten().collect();

                for exe in &candidates {
                    if exe.exists() {
                        std::process::Command::new(exe).spawn()
                            .map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("CodeBuddy CN 已重启".into());
                    }
                }

                // 快捷方式
                let shortcuts = vec![
                    dirs::desktop_dir().map(|d| d.join("CodeBuddy CN.lnk")),
                    dirs::config_dir().map(|d| d.join("Microsoft/Windows/Start Menu/Programs/CodeBuddy CN/CodeBuddy CN.lnk")),
                ];
                for sc in shortcuts.iter().flatten() {
                    if sc.exists() {
                        let _ = std::process::Command::new("cmd").no_window()
                            .args(["/c", "start", "", &sc.to_string_lossy()])
                            .spawn();
                        return Ok("CodeBuddy CN 已启动".into());
                    }
                }

                Err("未找到 CodeBuddy CN 可执行文件，请手动启动".into())
            }
            #[cfg(not(target_os = "windows"))]
            {
                #[cfg(target_os = "macos")]
                {
                    let p = PathBuf::from("/Applications/CodeBuddy CN.app");
                    if p.exists() {
                        std::process::Command::new("open").arg(&p).spawn().map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("CodeBuddy CN 已启动".into());
                    }
                    Err("未找到 CodeBuddy CN，请手动启动".into())
                }
                #[cfg(not(target_os = "macos"))]
                Err("请手动启动 CodeBuddy CN".into())
            }
        }
        "workbuddy" => {
            // WorkBuddy 独立客户端
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("taskkill").no_window()
                    .args(["/f", "/im", "WorkBuddy.exe"])
                    .spawn();
                std::thread::sleep(std::time::Duration::from_secs(2));

                // 搜索 exe
                let candidates: Vec<PathBuf> = vec![
                    PathBuf::from("C:\\Program Files\\WorkBuddy\\WorkBuddy.exe"),
                    PathBuf::from("C:\\Program Files (x86)\\WorkBuddy\\WorkBuddy.exe"),
                    dirs::data_dir().map(|d| d.join("Programs/WorkBuddy/WorkBuddy.exe")).unwrap_or_default(),
                ];

                for exe in &candidates {
                    if exe.exists() {
                        std::process::Command::new(exe).spawn()
                            .map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("WorkBuddy 已重启".into());
                    }
                }

                // 快捷方式
                let shortcuts = vec![
                    dirs::desktop_dir().map(|d| d.join("WorkBuddy.lnk")),
                    dirs::config_dir().map(|d| d.join("Microsoft/Windows/Start Menu/Programs/WorkBuddy/WorkBuddy.lnk")),
                ];
                for sc in shortcuts.iter().flatten() {
                    if sc.exists() {
                        let _ = std::process::Command::new("cmd").no_window()
                            .args(["/c", "start", "", &sc.to_string_lossy()])
                            .spawn();
                        return Ok("WorkBuddy 已启动".into());
                    }
                }

                Err("未找到 WorkBuddy 可执行文件，请手动启动".into())
            }
            #[cfg(not(target_os = "windows"))]
            {
                #[cfg(target_os = "macos")]
                {
                    let p = PathBuf::from("/Applications/WorkBuddy.app");
                    if p.exists() {
                        std::process::Command::new("open").arg(&p).spawn().map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("WorkBuddy 已启动".into());
                    }
                    Err("未找到 WorkBuddy，请手动启动".into())
                }
                #[cfg(not(target_os = "macos"))]
                Err("请手动启动 WorkBuddy".into())
            }
        }
        "trae" => {
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("taskkill").no_window()
                    .args(["/f", "/im", "Trae.exe"])
                    .spawn();
                std::thread::sleep(std::time::Duration::from_secs(2));

                // 搜索 Trae exe
                let candidates: Vec<PathBuf> = vec![
                    dirs::data_dir().map(|d| d.join("Programs/Trae/Trae.exe")).unwrap_or_default(),
                    dirs::data_dir().map(|d| d.join("Local/Programs/Trae/Trae.exe")).unwrap_or_default(),
                    PathBuf::from("C:\\Program Files\\Trae\\Trae.exe"),
                ];

                for exe in &candidates {
                    if exe.exists() {
                        std::process::Command::new(exe).spawn()
                            .map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("Trae 已重启".into());
                    }
                }

                // 快捷方式
                let shortcuts = vec![
                    dirs::desktop_dir().map(|d| d.join("Trae.lnk")),
                    dirs::config_dir().map(|d| d.join("Microsoft/Windows/Start Menu/Programs/Trae.lnk")),
                ];
                for sc in shortcuts.iter().flatten() {
                    if sc.exists() {
                        let _ = std::process::Command::new("cmd").no_window()
                            .args(["/c", "start", "", &sc.to_string_lossy()])
                            .spawn();
                        return Ok("Trae 已启动".into());
                    }
                }

                Err("未找到 Trae 可执行文件，请手动启动".into())
            }
            #[cfg(not(target_os = "windows"))]
            {
                #[cfg(target_os = "macos")]
                {
                    let p = PathBuf::from("/Applications/Trae.app");
                    if p.exists() {
                        std::process::Command::new("open").arg(&p).spawn().map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("Trae 已启动".into());
                    }
                    Err("未找到 Trae，请手动启动".into())
                }
                #[cfg(not(target_os = "macos"))]
                Err("请手动重启 Trae".into())
            }
        }
        "clawcode" => {
            // QClaw/Claw Code — GUI 应用
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("taskkill").no_window()
                    .args(["/f", "/im", "QClaw.exe"])
                    .spawn();
                std::thread::sleep(std::time::Duration::from_secs(2));

                // 搜索 exe
                if let Some(d) = dirs::data_dir() {
                    let exe = d.join("Programs/QClaw/QClaw.exe");
                    if exe.exists() {
                        std::process::Command::new(&exe).spawn()
                            .map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("Claw Code 已重启".into());
                    }
                }

                // 快捷方式
                let shortcuts = vec![
                    dirs::desktop_dir().map(|d| d.join("QClaw.lnk")),
                    dirs::config_dir().map(|d| d.join("Microsoft/Windows/Start Menu/Programs/QClaw.lnk")),
                ];
                for sc in shortcuts.iter().flatten() {
                    if sc.exists() {
                        let _ = std::process::Command::new("cmd").no_window()
                            .args(["/c", "start", "", &sc.to_string_lossy()])
                            .spawn();
                        return Ok("Claw Code 已启动".into());
                    }
                }

                Err("未找到 Claw Code 可执行文件，请手动启动".into())
            }
            #[cfg(not(target_os = "windows"))]
            {
                #[cfg(target_os = "macos")]
                {
                    let candidates = vec![
                        PathBuf::from("/Applications/QClaw.app"),
                        PathBuf::from("/Applications/OpenClaw.app"),
                    ];
                    for exe in &candidates {
                        if exe.exists() {
                            std::process::Command::new("open").arg(exe).spawn().map_err(|e| format!("启动失败: {}", e))?;
                            return Ok("Claw Code 已启动".into());
                        }
                    }
                    Err("未找到 Claw Code，请手动启动".into())
                }
                #[cfg(not(target_os = "macos"))]
                Err("请手动重启 Claw Code".into())
            }
        }
        _ => Err("未知平台".into()),
    }
}

/// 对话检测 — 用用户的 API Key 发一个最小请求，验证是否可用
#[tauri::command]
fn test_api_call(api_key: String, base_url: String, model: String) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/chat/completions", base_url.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": "说一个字"}],
        "stream": false,
        "max_tokens": 5
    });

    match reqwest::blocking::Client::new()
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(30))
        .json(&body)
        .send()
    {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let body_text = resp.text().unwrap_or_default();

            if status == 200 {
                match serde_json::from_str::<serde_json::Value>(&body_text) {
                    Ok(data) => {
                        let content = data.get("choices")
                            .and_then(|c| c.get(0))
                            .and_then(|c| c.get("message"))
                            .and_then(|m| m.get("content"))
                            .and_then(|c| c.as_str())
                            .unwrap_or("(无回复内容)");
                        Ok(serde_json::json!({
                            "success": true,
                            "status": 200,
                            "reply": content,
                            "detail": "API 调用成功，Key 有效"
                        }))
                    }
                    Err(_) => {
                        let preview = if body_text.len() > 200 { body_text[..200].to_string() } else { body_text.clone() };
                        Ok(serde_json::json!({
                            "success": true,
                            "status": 200,
                            "reply": "",
                            "detail": format!("服务器返回200但响应格式异常: {}", preview)
                        }))
                    }
                }
            } else {
                // 解析错误
                let err_code = serde_json::from_str::<serde_json::Value>(&body_text)
                    .ok()
                    .and_then(|d| d.get("error").and_then(|e| e.get("type")).and_then(|t| t.as_str()).map(|s| s.to_string()))
                    .or_else(|| {
                        serde_json::from_str::<serde_json::Value>(&body_text)
                            .ok()
                            .and_then(|d| d.get("code").and_then(|c| c.as_i64()).map(|c| c.to_string()))
                    })
                    .unwrap_or_else(|| format!("HTTP_{}", status));

                let err_msg = serde_json::from_str::<serde_json::Value>(&body_text)
                    .ok()
                    .and_then(|d| {
                        d.get("error").and_then(|e| e.get("message")).and_then(|m| m.as_str()).map(|s| s.to_string())
                        .or_else(|| d.get("msg").and_then(|m| m.as_str()).map(|s| s.to_string()))
                    })
                    .unwrap_or_else(|| format!("HTTP {}", status));

                Ok(serde_json::json!({
                    "success": false,
                    "status": status,
                    "error_code": err_code,
                    "error_msg": err_msg,
                    "detail": get_error_explanation(&err_code, status),
                    "fix_guide": get_error_fix_guide(&err_code, status)
                }))
            }
        }
        Err(e) => Ok(serde_json::json!({
            "success": false,
            "status": 0,
            "error_code": "NETWORK_ERROR",
            "error_msg": e.to_string(),
            "detail": "无法连接到服务器，请检查网络",
            "fix_guide": "1. 检查网络是否正常\n2. 检查 Base URL 是否正确\n3. 关闭 VPN/代理后重试"
        }))
    }
}

/// 错误码查询 — 输入错误码返回原因+修复教程
#[tauri::command]
fn lookup_error(code: String) -> serde_json::Value {
    get_error_info(&code)
}

fn get_error_explanation(code: &str, status: u16) -> String {
    match code {
        "insufficient_balance" | "402" => "积分不足，请充值后使用".into(),
        "not_found" | "404" => "API Key 无效或模型不存在，请检查 Key 和模型名称".into(),
        "rate_limit_error" | "429" => "请求过于频繁，请稍后重试".into(),
        "server_error" | "500" | "502" | "503" => "服务器暂时不可用，请稍后重试".into(),
        "11140" => "上游账号被风控，系统会自动切换其他账号".into(),
        "14003" => "请求过多，系统会自动重试".into(),
        "14018" => "上游积分耗尽，系统会自动切换其他账号".into(),
        "11115" => "输入内容过长，请缩短后重试".into(),
        "NETWORK_ERROR" => "网络连接失败".into(),
        _ => format!("未知错误 ({}), HTTP {}", code, status),
    }
}

fn get_error_fix_guide(code: &str, status: u16) -> String {
    match code {
        "insufficient_balance" | "402" => "1. 点击「充值」按钮购买新卡号\n2. 输入新卡号充值\n3. 重新部署".into(),
        "not_found" | "404" => "1. 检查 API Key 是否以 fm- 开头\n2. 检查 Base URL 是否为 https://glm.2bbb.cn/v1\n3. 在平台中手动选择「自定义模型」\n4. 重新部署".into(),
        "rate_limit_error" | "429" => "1. 等待 30 秒后重试\n2. 减少并发请求\n3. 检查是否多个客户端同时使用同一 Key".into(),
        "11140" => "1. 系统会自动切换其他账号\n2. 如果持续报错，请等待几分钟后重试\n3. 可尝试点「自检」→「一键修复」".into(),
        "14003" => "1. 请求过多，等待 1 分钟后重试\n2. 降低使用频率".into(),
        "14018" => "1. 系统会自动切换有积分的账号\n2. 如果持续报错，联系管理员补充号池".into(),
        "11115" => "1. 缩短输入内容\n2. 减少上下文长度\n3. 清理对话历史".into(),
        "NETWORK_ERROR" => "1. 检查网络连接\n2. 关闭 VPN/代理\n3. 检查防火墙是否拦截\n4. 重启软件".into(),
        _ => format!("1. 截图保存错误信息\n2. 联系客服并提供错误码: {}\n3. 尝试重新部署", code),
    }
}

fn get_error_info(code: &str) -> serde_json::Value {
    let (title, cause, guide): (&str, &str, &str) = match code {
        "402" | "insufficient_balance" => (
            "积分不足",
            "API Key 的积分已用完",
            "1. 点击「充值」按钮\n2. 购买新卡号\n3. 输入卡号充值\n4. 重新部署"
        ),
        "404" | "not_found" => (
            "Key 无效或模型不存在",
            "API Key 错误、过期，或平台中未选择自定义模型",
            "1. 确认 API Key 以 fm- 开头\n2. 确认 Base URL 为 https://glm.2bbb.cn/v1\n3. 在平台中手动选择「自定义模型」\n4. 如 Key 过期，重新激活卡号"
        ),
        "401" | "expired" => (
            "认证失败/Key过期",
            "API Key 已过期或被吊销",
            "1. 重新激活卡号获取新 Key\n2. 重新部署到平台\n3. 在平台中重新选择自定义模型"
        ),
        "429" | "rate_limit" => (
            "请求频率限制",
            "短时间内发送过多请求",
            "1. 等待 30 秒后重试\n2. 减少并发\n3 不要同时开多个客户端用同一 Key"
        ),
        "500" | "502" | "503" | "server_error" => (
            "服务器错误",
            "服务器暂时不可用",
            "1. 等待几分钟后重试\n2. 点「自检」检查服务器状态\n3. 如持续报错联系客服"
        ),
        "11140" => (
            "上游风控(11140)",
            "上游账号被临时风控",
            "1. 系统会自动切换其他账号，稍等重试\n2. 如持续报错，等几分钟后重试\n3. 点「自检」→「一键修复」"
        ),
        "14003" => (
            "请求过多(14003)",
            "上游请求频率限制",
            "1. 等待 1 分钟后重试\n2. 降低使用频率"
        ),
        "14018" => (
            "上游积分耗尽(14018)",
            "上游号池账号积分用完",
            "1. 系统会自动切换有积分的账号\n2. 如持续报错联系管理员"
        ),
        "11115" => (
            "输入过长(11115)",
            "输入内容超过模型限制",
            "1. 缩短输入内容\n2. 清理对话历史\n3. 减少上下文"
        ),
        _ => (
            "未知错误",
            "未知错误类型",
            "1. 截图保存错误信息\n2. 联系客服并提供错误码\n3. 尝试重新部署"
        ),
    };
    serde_json::json!({
        "code": code,
        "title": title,
        "cause": cause,
        "guide": guide
    })
}

/// 软件版本号（每次发布递增，与远程 /api/fastmmd/version 的 version 字段比对）
const APP_VERSION: u32 = 9;

/// 获取当前软件版本号
#[tauri::command]
fn get_app_version() -> u32 {
    APP_VERSION
}

/// 检查更新：请求远程版本接口，返回是否有新版本
#[derive(Serialize)]
struct UpdateCheckResult {
    has_update: bool,
    current: u32,
    latest: u32,
    url: String,
}

#[tauri::command]
async fn check_update() -> Result<UpdateCheckResult, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .danger_accept_invalid_certs(true)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client
        .get("https://glm.2bbb.cn/api/fastmmd/version")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let text = resp
        .text()
        .await
        .map_err(|e| e.to_string())?;
    let body: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("{}: {}", e, &text[..text.len().min(200)]))?;
    let latest = body.get("version")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    let url = body.get("url")
        .and_then(|v| v.as_str())
        .unwrap_or("https://2bbb.lanzout.com/b04mle8za")
        .to_string();
    Ok(UpdateCheckResult {
        has_update: latest > APP_VERSION,
        current: APP_VERSION,
        latest,
        url,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // 单实例锁: 如果已有实例运行, 聚焦到已有窗口而不是启动新实例
            use tauri::Manager;
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.show();
                let _ = w.set_focus();
                let _ = w.unminimize();
            }
        }))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            detect_all_platforms,
            deploy_to_platform,
            read_platform_config,
            restart_app,
            run_diagnostics,
            run_fix,
            backup_configs,
            test_api_call,
            lookup_error,
            open_url,
            clear_platform_deploy,
            get_app_version,
            check_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
