# Myman

Myman 是一个**本地优先的个人助理桌面应用**。当前版本围绕统一的信息管理模型，支持随手记、任务、日程、知识条目、文件记录、标签、归档和本地全文搜索。

应用基于 Tauri 构建，数据默认保存在本机 SQLite 数据库中，不依赖云端服务。

## 当前状态

当前 `main` 已完成第一阶段基础功能和第二阶段第一批标签管理功能，并通过本地 Runner 的离线业务测试：

- React / Vitest：14 个测试通过；
- Rust：11 个测试通过；
- 测试覆盖实体创建与编辑、筛选、分页、归档恢复、AND / OR 搜索、标签重命名、标签合并、孤立标签清理和搜索索引刷新。

这表示项目已经可以启动桌面开发版，进行人工功能体验。

测试通过不代表所有 GUI 布局、真实历史数据和未覆盖操作都已经完全验证。体验过程中发现问题时，请记录操作步骤、预期结果和实际结果。

## 已实现功能

### 统一内容管理

当前支持以下实体类型：

- 随手记；
- 任务；
- 日程；
- 知识条目；
- 文件记录。

这些内容共享统一的标题、摘要、正文、标签、归档状态和搜索能力。

### 搜索与筛选

- 按实体类型筛选；
- 按标签筛选；
- SQLite FTS5 本地全文搜索；
- AND 模式：要求搜索词全部匹配；
- OR 模式：任意搜索词匹配即可；
- 列表分页和“加载更多”。

### 归档管理

- 将当前内容归档；
- 查看归档箱；
- 从归档箱恢复内容。

### 标签管理

- 查看标签在当前内容和归档内容中的使用数量；
- 重命名标签；
- 合并两个标签；
- 合并时避免产生重复的实体标签关联；
- 清理没有任何内容引用的孤立标签；
- 标签重命名或合并后自动刷新全文搜索索引；
- 标签名称继续执行去除首尾空格和转为小写的规范化规则。

## 技术栈

- Tauri 2：桌面应用外壳；
- React 19 + TypeScript：前端界面；
- SQLite：本地数据库；
- SQLite FTS5：全文搜索；
- Vitest + Testing Library：前端测试；
- Rust 单元测试：后端业务测试。

## 运行环境

建议准备：

- Windows 10 或 Windows 11；
- Node.js 20.x；
- npm；
- Rust 工具链，包括 `cargo` 和 `rustc`；
- Microsoft Visual Studio Build Tools 2022；
- Visual Studio Build Tools 中的“使用 C++ 的桌面开发”工作负载；
- Microsoft Edge WebView2 Runtime。

如果同一台电脑已经能够运行 GPT Local Runner 的 MyMan 离线测试，Node.js、Rust 和主要编译环境通常已经准备好。

## 快速启动桌面开发版

如果本地仓库已经位于 `D:\Projects\Myman`，在 PowerShell 中运行：

```powershell
cd D:\Projects\Myman
git pull --ff-only
npm install
npm run tauri:dev
```

依赖已经安装时，可以简化为：

```powershell
cd D:\Projects\Myman
git pull --ff-only
npm run tauri:dev
```

运行后，Tauri 会启动 Vite 开发服务器，并打开 Myman 桌面窗口。

> 要体验数据库、标签、搜索和归档等完整功能，必须运行 `npm run tauri:dev`。
>
> `npm run dev` 只会启动浏览器前端。浏览器环境中没有 Tauri 后端，调用本地数据库命令时会失败。

## 第一次克隆项目

```powershell
git clone https://github.com/FIELD9z/MyMan.git
cd MyMan
npm install
npm run tauri:dev
```

## 建议的人工体验流程

### 1. 创建内容

分别创建：

- 一条随手记；
- 一个任务；
- 一个日程；
- 一个知识条目；
- 一个文件记录。

检查标题、摘要、正文和标签是否能够正常保存并重新显示。

### 2. 验证标签规范化

创建标签时尝试输入：

```text
  Work  
```

保存后应规范化为：

```text
work
```

### 3. 验证搜索

创建几条具有不同标题、正文和标签的内容，然后分别测试：

- AND 搜索；
- OR 搜索；
- 类型筛选；
- 标签筛选；
- 搜索与筛选组合使用。

### 4. 验证归档

- 归档一条内容；
- 确认它从当前列表消失；
- 打开归档箱；
- 恢复该内容；
- 确认它重新出现在当前列表。

### 5. 验证标签重命名

- 创建若干带有同一标签的内容；
- 打开“标签管理”；
- 重命名该标签；
- 返回内容列表，确认所有关联内容显示新标签；
- 使用新标签名称搜索，确认能够找到内容；
- 使用旧标签名称搜索，确认不再匹配。

### 6. 验证标签合并

- 创建分别带有 `personal` 和 `work` 标签的内容；
- 创建一条同时带有两个标签的内容；
- 将 `personal` 合并到 `work`；
- 确认所有内容只保留目标标签；
- 确认原本同时拥有两个标签的内容没有出现重复标签。

### 7. 验证孤立标签清理

正常界面操作通常不会轻易产生孤立标签。如果后续导入、迁移或异常操作产生未被内容引用的标签，可以在“标签管理”中使用“清理孤立标签”，并检查清理数量和剩余标签统计。

## 本地数据

应用数据库文件名为：

```text
myman.sqlite3
```

它保存在 Tauri 的应用数据目录中，应用标识为：

```text
com.myman.desktop
```

在 Windows 上通常位于：

```text
%APPDATA%\com.myman.desktop\myman.sqlite3
```

正式使用前，建议定期备份该数据库文件。人工测试期间如需完全重置数据，请先关闭应用并备份数据库；不要在不确认路径的情况下删除其他文件。

## 常用开发命令

### 启动桌面开发版

```powershell
npm run tauri:dev
```

### 仅启动前端页面

```powershell
npm run dev
```

仅用于前端样式和页面调试，不具备完整 Tauri 后端能力。

### 前端测试

```powershell
npm run test:run
```

### 前端代码检查

```powershell
npm run lint
```

### 前端生产构建

```powershell
npm run build
```

### Rust 测试

```powershell
cd src-tauri
cargo test --locked --offline
```

### 完整开发验证

```powershell
npm run verify
```

该命令依次执行前端 lint、前端测试、前端构建、Rust 测试和 Clippy。

### 验证桌面可执行文件但不生成安装包

```powershell
npm run tauri:build:no-bundle
```

### 构建 Windows 安装包

```powershell
npm run tauri:build
```

完整打包可能需要下载 WiX 或 NSIS。项目已在 `src-tauri\tauri.conf.json` 中启用 `bundle.useLocalToolsDir`，Windows 打包工具会缓存到项目的 target 目录，而不是用户全局缓存。

## Windows 编译问题

如果普通 PowerShell 找不到 `link.exe`，可以从 Visual Studio 开发者环境运行：

```powershell
cmd.exe /c '"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" -arch=x64 && npm run tauri:build:no-bundle'
```

如果外部依赖下载不稳定，并且本机代理位于 `127.0.0.1:7890`，可以临时设置：

```powershell
$env:HTTP_PROXY = 'http://127.0.0.1:7890'
$env:HTTPS_PROXY = 'http://127.0.0.1:7890'
$env:ALL_PROXY = 'http://127.0.0.1:7890'
npm run tauri:build
```

代理主要影响 npm、Rust crates、rustup 以及 Tauri 的 Windows 打包工具下载。

完整 Windows 构建通常输出：

```text
src-tauri\target\release\bundle\msi\Myman_0.1.0_x64_en-US.msi
src-tauri\target\release\bundle\nsis\Myman_0.1.0_x64-setup.exe
```

## 数据模型

随手记、任务、日程、知识条目和文件记录共享统一实体模型。类型专属字段放在属性或专用表中，标签、内容、关联、提醒、版本和搜索索引由各类实体共享。

初始数据库结构包括：

- `entities`；
- `entity_properties`；
- `entity_contents`；
- `tags`；
- `entity_tags`；
- `entity_links`；
- `file_index`；
- `reminders`；
- `revisions`；
- `search_index_jobs`；
- FTS5 虚拟表 `search_index`。

当前文件功能属于元数据记录，不会自动读取或索引真实文件内容。PDF、Office、OCR、语义搜索和文件内容索引属于后续阶段。

## 当前未包含的能力

以下能力尚未在当前版本中完成：

- AI 自动分类或自动打标签；
- PDF、Office 和 OCR 内容解析；
- 文件夹自动扫描和增量索引；
- 向量数据库和语义搜索；
- 云同步；
- 多设备同步；
- 完整任务提醒和系统通知；
- 面向正式发布的安装、升级和数据迁移体验。

## 学习资料

项目也作为分阶段学习项目维护。可从 `LEARNING.md` 开始，并查看 `docs/learning/` 下的路线图、周记录、概念笔记、练习和问题清单。
