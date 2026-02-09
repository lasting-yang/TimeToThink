# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

**TimeToThink** 是一个仅限 macOS 的番茄钟应用，使用 Tauri（Rust 后端 + JavaScript/TypeScript 前端）构建。应用通过系统锁屏集成和全屏遮罩来强制执行休息期间，防止用户跳过休息。

**当前状态**: 规划阶段 - 完整规格说明见 TODO.md。目前尚未编写任何代码。

## 核心架构

应用基于状态机设计，包含三种状态：

- `FOCUS` (25:00) - 专注工作
- `SHORT_BREAK` (5:00) - 短休息
- `LONG_BREAK` (25:00) - 每完成 3 个番茄后的长休息

### 状态转换规则

1. `FOCUS` 结束 → 检查 `(已完成番茄数 + 1) % 3 == 0`
   - 若成立 → 进入 `LONG_BREAK`
   - 否则 → 进入 `SHORT_BREAK`
2. 进入任何 `BREAK` 状态：
   - 开始倒计时
   - 5 秒内触发锁屏（仅一次）
   - 显示 BreakGuard 全屏遮罩
3. 用户可跳过休息，但需二次确认

## 模块结构规划 (src-tauri/)

- `timer_engine.rs` - 核心状态机，1 秒 tick 循环，向前端发送事件
- `macos_lock.rs` - 执行 `CGSession -suspend` 命令实现锁屏
- `guard_control.rs` - 显示/聚焦 BreakGuard 窗口，休息期间轮询确保窗口始终在最前
- `storage.rs` - 持久化状态（当前状态、结束时间、番茄计数、锁屏调用标志）用于崩溃恢复

## 前端视图

- `MainView` - 主计时器界面，显示状态/倒计时，提供开始/暂停/重置控制
- `BreakGuardView` - 休息期间的全屏遮罩，包含"继续休息"（主要按钮）和"跳过休息"（需确认）按钮

## macOS 系统集成

### 锁屏触发
通过 Rust 的 `std::process::Command` 调用 `CGSession -suspend`。每次休息仅执行一次（前 5 秒内）。

### BreakGuard 强制显示
由于 macOS 无法在解锁前显示自定义窗口，应用采用轮询策略（每 500ms-1s），在休息期间检测桌面是否可见，立即显示并聚焦 BreakGuard 遮罩。

## 开发命令

初始化 Tauri 项目后：

```bash
# 开发模式
npm run tauri dev     # 或: cargo tauri dev

# 构建发布版本
npm run tauri build   # 或: cargo tauri build

# 运行 Rust 测试
cargo test

# 运行特定测试
cargo test test_name

# 格式化 Rust 代码
cargo fmt

# 快速编译检查
cargo check
```

## 开发里程碑

见 TODO.md 中的验收标准。开发分为四个里程碑：

1. **状态机** - 基础计时器在 FOCUS/SHORT_BREAK/LONG_BREAK 间循环，正确计数番茄数
2. **BreakGuard 界面** - 全屏遮罩及跳过确认对话框
3. **锁屏集成** - macOS 锁屏触发 + 解锁后恢复 BreakGuard
4. **状态持久化** - 崩溃恢复，确保应用重启后继续未完成的休息

## 验收清单

参考 TODO.md (第 127-134 行)：

- [ ] 专注 25:00 正常倒计时
- [ ] 专注结束自动进入休息
- [ ] 进入休息后 5 秒内自动锁屏（仅触发一次）
- [ ] 解锁后立即出现全屏休息遮罩
- [ ] 休息期间点"跳过休息"会弹二次确认；取消则继续休息
- [ ] 确认跳过后立刻回到专注
- [ ] 每完成 3 个番茄后，下一次休息为 25:00 长休
- [ ] （建议）重启应用后状态能恢复正确
