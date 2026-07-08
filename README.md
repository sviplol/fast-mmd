# GLM API 平台登录器

基于 Tauri 2 + Vue 3 + Ant Design 的桌面客户端。

## 功能

- 🎫 **卡号登录** — 输入卡号自动兑换并登录
- 👤 **账号登录/注册** — 支持用户名密码
- 🔑 **API Key 管理** — 创建/查看/复制
- 🚀 **一键部署 WorkBuddy** — 自动写入 API 配置
- 📈 **消费记录** — 查看 token 消耗/积分记录
- 🤖 **全模型支持** — GLM-5.2 / DeepSeek / Kimi 等14个模型
- 💳 **卡密充值** — 输入新卡号续费
- 📊 **概览面板** — 余额/请求/消耗一览

## 开发环境要求

1. **Node.js** 18+
2. **Rust** (rustup install)
3. **Windows Build Tools** 或 Visual Studio Build Tools (C++ 工作负载)

## 开发

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 打包 exe
npm run tauri build
```

## 打包输出

打包后的 exe 在 `src-tauri/target/release/bundle/nsis/` 目录下。

## 配置说明

- GLM 站: https://glm.2bbb.cn
- TK 站: https://tk.2bbb.cn
- API Base URL: https://glm.2bbb.cn/v1 或 https://tk.2bbb.cn/v1
- API Key 格式: fm-xxxxxxxxxxxx

## 技术栈

- **前端**: Vue 3 + Ant Design 5 + Vite
- **后端**: Rust + Tauri 2
- **打包**: NSIS (Windows exe)
