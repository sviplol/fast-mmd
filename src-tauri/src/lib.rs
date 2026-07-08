use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

// 推理等级排序（从低到高）
const REASONING_ORDER: [&str; 7] = ["none", "minimal", "low", "medium", "high", "xhigh", "max"];

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
    r.insert("trae".into(), detect_trae());
    r
}

/// OpenCode: AppData/Roaming/ai.opencode.desktop/
fn detect_opencode() -> DetectResult {
    let p = dirs::config_dir().map(|d| d.join("ai.opencode.desktop"));
    if let Some(p) = p {
        if p.exists() {
            return DetectResult { installed: true, path: Some(p.to_string_lossy().into()) };
        }
    }
    DetectResult { installed: false, path: None }
}

/// Claude Code: ~/.claude/
fn detect_claude_code() -> DetectResult {
    let p = dirs::home_dir().map(|d| d.join(".claude"));
    if let Some(p) = p {
        if p.exists() {
            return DetectResult { installed: true, path: Some(p.to_string_lossy().into()) };
        }
    }
    DetectResult { installed: false, path: None }
}

/// CodeBuddy: 检测多种安装方式
fn detect_codebuddy() -> DetectResult {
    // 1. VS Code 扩展目录
    let ext_dirs: Vec<PathBuf> = vec![
        dirs::home_dir().map(|d| d.join(".vscode/extensions")),
        dirs::home_dir().map(|d| d.join(".cursor/extensions")),
        dirs::home_dir().map(|d| d.join(".vscode-insiders/extensions")),
        dirs::home_dir().map(|d| d.join(".vscode-oss/extensions")),
    ].into_iter().flatten().collect();

    for ext_dir in &ext_dirs {
        if ext_dir.exists() {
            if let Ok(entries) = fs::read_dir(ext_dir) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.contains("tencent-cloud") || name.contains("coding-copilot") || name.contains("codebuddy") {
                        return DetectResult { installed: true, path: Some(entry.path().to_string_lossy().into()) };
                    }
                }
            }
        }
    }

    // 2. globalStorage 目录
    let gs_dirs: Vec<PathBuf> = vec![
        dirs::config_dir().map(|d| d.join("Code/User/globalStorage")),
        dirs::config_dir().map(|d| d.join("Code - Insiders/User/globalStorage")),
        dirs::config_dir().map(|d| d.join("Cursor/User/globalStorage")),
        dirs::config_dir().map(|d| d.join("Trae/User/globalStorage")),
    ].into_iter().flatten().collect();

    for gs in &gs_dirs {
        if gs.exists() {
            for sub in &["tencent-cloud.coding-copilot", "codebuddy"] {
                let cb_dir = gs.join(sub);
                if cb_dir.exists() {
                    return DetectResult { installed: true, path: Some(cb_dir.to_string_lossy().into()) };
                }
            }
        }
    }

    // 3. 开始菜单快捷方式
    let start_menu_dirs: Vec<PathBuf> = vec![
        dirs::config_dir().map(|d| d.join("Microsoft/Windows/Start Menu/Programs")),
        dirs::data_dir().map(|d| d.join("Microsoft/Windows/Start Menu/Programs")),
    ].into_iter().flatten().collect();

    for sm in &start_menu_dirs {
        if sm.exists() {
            // 递归搜索 CodeBuddy
            if let Ok(entries) = fs::read_dir(sm) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_lowercase();
                    if name.contains("codebuddy") {
                        return DetectResult { installed: true, path: Some(entry.path().to_string_lossy().into()) };
                    }
                    // 递归搜索子目录
                    if entry.path().is_dir() {
                        if let Ok(sub_entries) = fs::read_dir(entry.path()) {
                            for sub in sub_entries.flatten() {
                                let sub_name = sub.file_name().to_string_lossy().to_lowercase();
                                if sub_name.contains("codebuddy") {
                                    return DetectResult { installed: true, path: Some(sub.path().to_string_lossy().into()) };
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 4. Windows 注册表卸载列表
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        for root in &[hkcu, hklm] {
            if let Ok(uninstall) = root.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall") {
                for sub in uninstall.enum_keys().flatten() {
                    let name_lower = sub.to_lowercase();
                    if name_lower.contains("codebuddy") || name_lower.contains("coding-copilot") {
                        return DetectResult { installed: true, path: None };
                    }
                    // 也检查 DisplayName
                    if let Ok(key) = uninstall.open_subkey(&sub) {
                        if let Ok(display_name) = key.get_value::<String, _>("DisplayName") {
                            if display_name.to_lowercase().contains("codebuddy") {
                                return DetectResult { installed: true, path: None };
                            }
                        }
                    }
                }
            }
        }
    }

    // 5. 桌面快捷方式
    if let Some(desktop) = dirs::desktop_dir() {
        if let Ok(entries) = fs::read_dir(&desktop) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if name.contains("codebuddy") {
                    return DetectResult { installed: true, path: Some(entry.path().to_string_lossy().into()) };
                }
            }
        }
    }

    DetectResult { installed: false, path: None }
}

/// WorkBuddy: ~/.workbuddy/
fn detect_workbuddy() -> DetectResult {
    let paths: Vec<PathBuf> = vec![
        dirs::home_dir().map(|d| d.join(".workbuddy")),
        dirs::config_dir().map(|d| d.join("WorkBuddy")),
        dirs::data_dir().map(|d| d.join("WorkBuddy")),
    ].into_iter().flatten().collect();

    for p in &paths {
        if p.exists() && p.is_dir() {
            return DetectResult { installed: true, path: Some(p.to_string_lossy().into()) };
        }
    }

    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(uninstall) = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall") {
            for sub in uninstall.enum_keys().flatten() {
                if sub.to_lowercase().contains("workbuddy") {
                    return DetectResult { installed: true, path: None };
                }
            }
        }
    }
    DetectResult { installed: false, path: None }
}

/// Trae: AppData/Roaming/Trae/
fn detect_trae() -> DetectResult {
    let p = dirs::config_dir().map(|d| d.join("Trae"));
    if let Some(p) = p {
        if p.exists() {
            return DetectResult { installed: true, path: Some(p.to_string_lossy().into()) };
        }
    }
    // 也检查 Local
    let p2 = dirs::data_dir().map(|d| d.join("Trae"));
    if let Some(p2) = p2 {
        if p2.exists() {
            return DetectResult { installed: true, path: Some(p2.to_string_lossy().into()) };
        }
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
        "trae" => deploy_trae(&config),
        _ => Err("未知平台".into()),
    }
}

/// OpenCode 部署: 修改 opencode.global.dat 的 model 字段
/// model 是嵌套 JSON 字符串，包含 user(模型列表) + variant(推理等级 map)
/// 部署前清除旧的 antigravity 配置，避免重复堆积
fn deploy_opencode(config: &DeployConfig) -> Result<String, String> {
    let oc_dir = detect_opencode().path.ok_or("OpenCode 未安装")?;
    let dir = PathBuf::from(&oc_dir);
    let dat_path = dir.join("opencode.global.dat");

    let mut data: serde_json::Value = if dat_path.exists() {
        fs::read_to_string(&dat_path).ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    if let Some(obj) = data.as_object_mut() {
        // 解析 model 字段（嵌套 JSON 字符串）
        let model_str = obj.get("model").and_then(|v| v.as_str()).map(|s| s.to_string());
        let mut model_obj: serde_json::Value = if let Some(ref ms) = model_str {
            serde_json::from_str(ms).unwrap_or(serde_json::json!({}))
        } else {
            serde_json::json!({})
        };

        if let Some(mo) = model_obj.as_object_mut() {
            // 0. 清除旧的 antigravity 配置（避免重复堆积）
            if let Some(user_arr) = mo.get_mut("user").and_then(|v| v.as_array_mut()) {
                user_arr.retain(|m| {
                    m.get("providerID").and_then(|v| v.as_str()).map_or(true, |p| p != "antigravity")
                });
            }
            if let Some(variants) = mo.get_mut("variant").and_then(|v| v.as_object_mut()) {
                let old_keys: Vec<String> = variants.keys().filter(|k| k.starts_with("antigravity:")).cloned().collect();
                for k in old_keys { variants.remove(&k); }
            }
            if let Some(recent) = mo.get_mut("recent").and_then(|v| v.as_array_mut()) {
                recent.retain(|m| {
                    m.get("providerID").and_then(|v| v.as_str()).map_or(true, |p| p != "antigravity")
                });
            }

            // 1. 更新 user 列表 — 添加所有选中的模型
            let mut user_list: Vec<serde_json::Value> = Vec::new();
            for model_id in &config.selected_model_ids {
                user_list.push(serde_json::json!({
                    "modelID": model_id,
                    "providerID": "antigravity",
                    "visibility": "show"
                }));
            }
            mo.insert("user".to_string(), serde_json::Value::Array(user_list));

            // 2. 更新 variant — 每个模型写入所有选中的推理等级
            let mut variants = serde_json::Map::new();
            for model_id in &config.selected_model_ids {
                // 写入所有选中的等级（用户可在 OpenCode 内切换）
                let levels = if config.reasoning_levels.is_empty() {
                    vec![config.reasoning_level.clone()]
                } else {
                    config.reasoning_levels.clone()
                };
                // variant 的值是最高等级（默认），但 available 列出所有可选
                let highest = levels.iter()
                    .filter_map(|l| REASONING_ORDER.iter().position(|&r| r == l).map(|p| (l, p)))
                    .max_by_key(|(_, p)| *p)
                    .map(|(l, _)| l.clone())
                    .unwrap_or_else(|| config.reasoning_level.clone());
                variants.insert(
                    format!("antigravity:{}", model_id),
                    serde_json::json!(highest),
                );
            }
            mo.insert("variant".to_string(), serde_json::Value::Object(variants));

            // 3. 更新 recent — 默认第一个模型
            if let Some(first_model) = config.selected_model_ids.first() {
                mo.insert("recent".to_string(), serde_json::json!([{
                    "modelID": first_model,
                    "providerID": "antigravity"
                }]));
            }
        }

        // 写回 model 字段（重新序列化为 JSON 字符串）
        obj.insert("model".to_string(), serde_json::Value::String(
            serde_json::to_string(&model_obj).unwrap_or_default()
        ));
    }

    fs::write(&dat_path, serde_json::to_string_pretty(&data).unwrap())
        .map_err(|e| format!("写入失败: {}", e))?;

    Ok(format!("OpenCode: {} 个模型已配置，推理等级: {}", config.selected_model_ids.len(), config.reasoning_level))
}

/// Claude Code 部署: 修改 ~/.claude/settings.json
/// env: ANTHROPIC_BASE_URL + ANTHROPIC_AUTH_TOKEN + ANTHROPIC_MODEL
/// effortLevel: 推理等级
fn deploy_claude_code(config: &DeployConfig) -> Result<String, String> {
    let cc_dir = detect_claude_code().path.ok_or("Claude Code 未安装")?;
    let dir = PathBuf::from(&cc_dir);
    let settings_path = dir.join("settings.json");

    let mut settings: serde_json::Value = if settings_path.exists() {
        fs::read_to_string(&settings_path).ok()
            .and_then(|s| serde_json::from_str(&s).ok())
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

        // 推理等级 — 用最高的，同时写入可选列表
        obj.insert("effortLevel".to_string(), serde_json::json!(highest));
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

/// CodeBuddy 部署: state.vscdb (DPAPI加密，生成配置文件)
fn deploy_codebuddy(config: &DeployConfig) -> Result<String, String> {
    let cb_path = detect_codebuddy().path.ok_or("CodeBuddy 未安装")?;
    let base = PathBuf::from(&cb_path);

    // 构建模型配置
    let model_configs: Vec<serde_json::Value> = if !config.model_configs.is_empty() {
        config.model_configs.clone()
    } else {
        let levels = if config.reasoning_levels.is_empty() {
            vec![config.reasoning_level.clone()]
        } else {
            config.reasoning_levels.clone()
        };
        vec![serde_json::json!({
            "id": config.model,
            "supportsReasoning": true,
            "onlyReasoning": !levels.is_empty() && !levels.iter().all(|l| l == "none"),
            "reasoning": { "effort": levels.iter().filter(|l| *l != "none").next().unwrap_or(&config.reasoning_level), "summary": "auto", "available": levels },
            "supportsToolCall": true,
            "supportsImages": true,
            "maxInputTokens": 200000,
            "maxOutputTokens": 64000,
        })]
    };

    // 找 state.vscdb 路径 — 检查 Code 和 Cursor
    let mut state_db: Option<PathBuf> = None;
    if let Some(cfg) = dirs::config_dir() {
        let code_db = cfg.join("Code/User/state.vscdb");
        let cursor_db = cfg.join("Cursor/User/state.vscdb");
        if code_db.exists() {
            state_db = Some(code_db);
        } else if cursor_db.exists() {
            state_db = Some(cursor_db);
        }
    }

    // 生成配置文件
    let config_path = base.join("glm_deploy_config.json");
    let deploy_data = serde_json::json!({
        "platform": "codebuddy",
        "state_db_path": state_db.as_ref().map(|p| p.to_string_lossy().to_string()),
        "api_key": config.api_key,
        "base_url": config.base_url,
        "models": config.selected_model_ids,
        "model_configs": model_configs,
        "reasoning_level": config.reasoning_level,
        "deep_thinking": config.deep_thinking,
        "instructions": "由于 CodeBuddy-Product-Cache 使用 DPAPI 加密，请通过 CodeBuddy 扩展 UI 手动导入此配置，或使用工具解密后修改 state.vscdb",
    });

    fs::write(&config_path, serde_json::to_string_pretty(&deploy_data).unwrap())
        .map_err(|e| format!("写入失败: {}", e))?;

    Ok(format!("CodeBuddy: {} 个模型配置已生成 (需手动导入)", config.selected_model_ids.len()))
}

/// WorkBuddy 部署: ~/.workbuddy/models.json
fn deploy_workbuddy(config: &DeployConfig) -> Result<String, String> {
    let wb_dir = detect_workbuddy().path.ok_or("WorkBuddy 未安装")?;
    let dir = PathBuf::from(&wb_dir);
    let models_path = dir.join("models.json");

    let mut models: serde_json::Value = if models_path.exists() {
        fs::read_to_string(&models_path).ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(serde_json::json!([]))
    } else {
        serde_json::json!([])
    };

    let new_models: Vec<serde_json::Value> = if !config.model_configs.is_empty() {
        config.model_configs.iter().map(|mc| {
            let mut m = mc.clone();
            if let Some(obj) = m.as_object_mut() {
                obj.insert("apiKey".into(), serde_json::json!(config.api_key));
                obj.insert("baseUrl".into(), serde_json::json!(config.base_url));
                if let Some(max_in) = mc.get("maxInputTokens") {
                    obj.insert("maxAllowedSize".into(), max_in.clone());
                }
            }
            m
        }).collect()
    } else {
        vec![serde_json::json!({
            "id": config.model,
            "apiKey": config.api_key,
            "baseUrl": config.base_url,
        })]
    };

    if let Some(arr) = models.as_array_mut() {
        // 0. 清除旧的平台配置（apiKey 以 fm- 开头 或 baseUrl 含 2bbb.cn）
        arr.retain(|m| {
            let key = m.get("apiKey").and_then(|v| v.as_str()).unwrap_or("");
            let url = m.get("baseUrl").and_then(|v| v.as_str()).unwrap_or("");
            !key.starts_with("fm-") && !url.contains("2bbb.cn")
        });

        for new_m in &new_models {
            let new_id = new_m.get("id").and_then(|v| v.as_str());
            let mut found = false;
            for existing in arr.iter_mut() {
                if existing.get("id").and_then(|v| v.as_str()) == new_id {
                    if let (Some(e), Some(n)) = (existing.as_object_mut(), new_m.as_object()) {
                        for (k, v) in n { e.insert(k.clone(), v.clone()); }
                    }
                    found = true;
                    break;
                }
            }
            if !found { arr.push(new_m.clone()); }
        }
    } else {
        models = serde_json::Value::Array(new_models);
    }

    fs::write(&models_path, serde_json::to_string_pretty(&models).unwrap())
        .map_err(|e| format!("写入失败: {}", e))?;

    let default_model = config.selected_model_ids.first().unwrap_or(&config.model);
    let _ = fs::write(dir.join("config.json"), serde_json::json!({
        "api_key": config.api_key, "base_url": config.base_url, "model": default_model
    }).to_string());
    let _ = fs::write(dir.join(".env"), format!(
        "OPENAI_API_KEY={}\nOPENAI_BASE_URL={}\nMODEL={}\n",
        config.api_key, config.base_url, default_model
    ));

    Ok(format!("WorkBuddy: {} 个模型已写入", config.selected_model_ids.len()))
}

/// Trae 部署: 同 CodeBuddy (state.vscdb)
fn deploy_trae(config: &DeployConfig) -> Result<String, String> {
    let trae_dir = detect_trae().path.ok_or("Trae 未安装")?;
    let dir = PathBuf::from(&trae_dir);

    // 生成配置文件
    let config_path = dir.join("glm_deploy_config.json");
    let model_configs: Vec<serde_json::Value> = if !config.model_configs.is_empty() {
        config.model_configs.clone()
    } else {
        let levels = if config.reasoning_levels.is_empty() {
            vec![config.reasoning_level.clone()]
        } else {
            config.reasoning_levels.clone()
        };
        vec![serde_json::json!({
            "id": config.model,
            "supportsReasoning": true,
            "reasoning": { "effort": levels.iter().filter(|l| *l != "none").next().unwrap_or(&config.reasoning_level), "available": levels },
            "supportsToolCall": true,
            "maxInputTokens": 200000,
            "maxOutputTokens": 64000,
        })]
    };

    let deploy_data = serde_json::json!({
        "platform": "trae",
        "trae_dir": dir.to_string_lossy(),
        "api_key": config.api_key,
        "base_url": config.base_url,
        "models": config.selected_model_ids,
        "model_configs": model_configs,
        "instructions": "通过 Trae 扩展设置导入自定义模型配置",
    });

    fs::write(&config_path, serde_json::to_string_pretty(&deploy_data).unwrap())
        .map_err(|e| format!("写入失败: {}", e))?;

    Ok(format!("Trae: {} 个模型配置已生成", config.selected_model_ids.len()))
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
    let wb_path = detect_workbuddy().path.ok_or("WorkBuddy 未安装")?;
    let models_path = PathBuf::from(&wb_path).join("models.json");
    if !models_path.exists() { return Ok(None); }

    let content = fs::read_to_string(&models_path).map_err(|e| format!("读取失败: {}", e))?;
    let models: serde_json::Value = serde_json::from_str(&content).unwrap_or(serde_json::json!([]));

    if let Some(arr) = models.as_array() {
        for m in arr {
            let api_key = m.get("apiKey").and_then(|v| v.as_str()).unwrap_or("");
            let base_url = m.get("baseUrl").and_then(|v| v.as_str()).unwrap_or("");
            if api_key.starts_with("fm-") || base_url.contains("2bbb.cn") {
                return Ok(Some(serde_json::json!({
                    "apiKey": api_key,
                    "baseUrl": base_url,
                    "deployed": true,
                    "platform": "workbuddy",
                })));
            }
        }
    }
    Ok(None)
}

fn read_trae_config() -> Result<Option<serde_json::Value>, String> {
    let trae_path = detect_trae().path.ok_or("Trae 未安装")?;
    let config_path = PathBuf::from(&trae_path).join("glm_deploy_config.json");
    if config_path.exists() {
        let content = fs::read_to_string(&config_path).map_err(|e| format!("读取失败: {}", e))?;
        let data: serde_json::Value = serde_json::from_str(&content).unwrap_or(serde_json::json!({}));
        return Ok(Some(serde_json::json!({
            "apiKey": data.get("api_key").and_then(|v| v.as_str()).unwrap_or(""),
            "deployed": true,
            "platform": "trae",
        })));
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

fn check_opencode_config() -> DiagnosticItem {
    let oc = detect_opencode();
    if !oc.installed {
        return DiagnosticItem { id: "oc_not_installed".into(), category: "OpenCode".into(), title: "OpenCode 未安装".into(), status: "warning".into(), detail: "未检测到 OpenCode".into(), fixable: false, fix_action: "".into() };
    }
    let dir = PathBuf::from(oc.path.unwrap());
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
    let dir = PathBuf::from(cc.path.unwrap());
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
    let dir = PathBuf::from(wb.path.unwrap());
    let models = dir.join("models.json");
    if !models.exists() {
        return DiagnosticItem { id: "wb_no_config".into(), category: "WorkBuddy".into(), title: "models.json 不存在".into(), status: "warning".into(), detail: "请先部署".into(), fixable: true, fix_action: "deploy".into() };
    }
    match fs::read_to_string(&models) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(data) => {
                    if let Some(arr) = data.as_array() {
                        for m in arr {
                            let key = m.get("apiKey").and_then(|v| v.as_str()).unwrap_or("");
                            if key.starts_with("fm-") {
                                return DiagnosticItem { id: "wb_ok".into(), category: "WorkBuddy".into(), title: "配置正常".into(), status: "ok".into(), detail: format!("Key: {}...", &key[..10]), fixable: false, fix_action: "".into() };
                            }
                        }
                    }
                    DiagnosticItem { id: "wb_no_deploy".into(), category: "WorkBuddy".into(), title: "未部署我们的 API".into(), status: "warning".into(), detail: "models.json 中未找到 fm- 开头的 Key".into(), fixable: true, fix_action: "deploy".into() }
                }
                Err(e) => DiagnosticItem { id: "wb_parse_error".into(), category: "WorkBuddy".into(), title: "models.json 损坏".into(), status: "error".into(), detail: format!("{}", e), fixable: true, fix_action: "restore_backup".into() }
            }
        }
        Err(e) => DiagnosticItem { id: "wb_read_error".into(), category: "WorkBuddy".into(), title: "读取失败".into(), status: "error".into(), detail: format!("{}", e), fixable: false, fix_action: "".into() }
    }
}

fn check_codebuddy_config() -> DiagnosticItem {
    let cb = detect_codebuddy();
    if !cb.installed {
        return DiagnosticItem { id: "cb_not_installed".into(), category: "CodeBuddy".into(), title: "CodeBuddy 未安装".into(), status: "warning".into(), detail: "".into(), fixable: false, fix_action: "".into() };
    }
    DiagnosticItem { id: "cb_installed".into(), category: "CodeBuddy".into(), title: "已安装（需手动导入配置）".into(), status: "ok".into(), detail: "CodeBuddy 使用 DPAPI 加密，需手动导入 glm_deploy_config.json".into(), fixable: false, fix_action: "".into() }
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
                let _ = std::process::Command::new("cmd")
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
        std::process::Command::new("powershell")
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

/// 重启已部署的软件
#[tauri::command]
fn restart_app(platform: String) -> Result<String, String> {
    match platform.as_str() {
        "opencode" => {
            // OpenCode 是 Electron 应用，通过命令行启动
            #[cfg(target_os = "windows")]
            {
                let path = dirs::config_dir()
                    .map(|d| d.join("ai.opencode.desktop"))
                    .ok_or("找不到 OpenCode 路径")?;
                // 尝试启动 opencode.exe
                let exe = path.join("opencode.exe");
                if exe.exists() {
                    std::process::Command::new(&exe).spawn()
                        .map_err(|e| format!("启动失败: {}", e))?;
                    return Ok("OpenCode 已启动".into());
                }
                // 也检查 Local AppData
                let exe2 = dirs::data_dir()
                    .map(|d| d.join("Programs").join("opencode").join("opencode.exe"));
                if let Some(e) = exe2 {
                    if e.exists() {
                        std::process::Command::new(&e).spawn()
                            .map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("OpenCode 已启动".into());
                    }
                }
                Err("未找到 OpenCode 可执行文件，请手动启动".into())
            }
            #[cfg(not(target_os = "windows"))]
            Err("请在桌面手动启动 OpenCode".into())
        }
        "claudecode" => {
            // Claude Code 是命令行工具，不需要重启 GUI
            Ok("Claude Code 配置已生效，新开终端即可使用".into())
        }
        "codebuddy" => {
            // CodeBuddy 是 VS Code 扩展，重启 VS Code
            #[cfg(target_os = "windows")]
            {
                // 尝试重启 VS Code
                std::process::Command::new("cmd")
                    .args(["/c", "taskkill /f /im Code.exe & timeout /t 2 & start Code.exe"])
                    .spawn()
                    .map_err(|e| format!("重启失败: {}", e))?;
                Ok("VS Code 正在重启...".into())
            }
            #[cfg(not(target_os = "windows"))]
            Err("请手动重启 VS Code".into())
        }
        "workbuddy" => {
            #[cfg(target_os = "windows")]
            {
                // 尝试找 WorkBuddy exe
                let wb_dir = detect_workbuddy().path;
                if let Some(p) = wb_dir {
                    let exe = PathBuf::from(&p).join("workbuddy.exe");
                    if exe.exists() {
                        std::process::Command::new(&exe).spawn()
                            .map_err(|e| format!("启动失败: {}", e))?;
                        return Ok("WorkBuddy 已启动".into());
                    }
                }
                Err("未找到 WorkBuddy 可执行文件，请手动启动".into())
            }
            #[cfg(not(target_os = "windows"))]
            Err("请手动启动 WorkBuddy".into())
        }
        "trae" => {
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("cmd")
                    .args(["/c", "taskkill /f /im Trae.exe & timeout /t 2 & start Trae.exe"])
                    .spawn()
                    .map_err(|e| format!("重启失败: {}", e))?;
                Ok("Trae 正在重启...".into())
            }
            #[cfg(not(target_os = "windows"))]
            Err("请手动重启 Trae".into())
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
