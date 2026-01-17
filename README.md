# Obsidian Quick Add

A lightweight macOS menubar app for quickly adding tasks to your Obsidian daily notes from anywhere, with natural language date parsing.

![Demo](docs/demo.gif)

## Features

- **Global hotkey** (`Cmd+Shift+K`) - Opens instantly from any app
- **Natural language dates** - Type "buy milk tomorrow" or "meeting next friday" and dates are automatically parsed
- **Inline highlighting** - See detected dates highlighted as you type
- **Multiple vault support** - Automatically discovers all your Obsidian vaults
- **Tasks plugin format** - Creates tasks compatible with the Obsidian Tasks plugin
- **Fast** - Built with Tauri for near-instant startup (~50ms)

## Installation

### Download Release

Download the latest `.dmg` from [Releases](https://github.com/Oyveloper/obsidian-quick-add/releases) and drag to Applications.

### Build from Source

Requires: [Node.js](https://nodejs.org/), [Rust](https://rustup.rs/), and [Tauri CLI](https://tauri.app/)

```bash
# Clone the repo
git clone https://github.com/Oyveloper/obsidian-quick-add.git
cd obsidian-quick-add

# Install dependencies
npm install

# Build the app
npm run tauri build

# The app will be at src-tauri/target/release/bundle/macos/Obsidian Quick Add.app
```

## Usage

1. **Launch the app** - It runs in the background with no dock icon
2. **Press `Cmd+Shift+K`** - Opens the quick add window
3. **Type your task** - Natural language dates are highlighted automatically
4. **Press Enter** - Task is added to today's daily note
5. **Press Escape** - Closes without adding

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd+Shift+K` | Open quick add (global) |
| `Enter` | Submit task |
| `Escape` | Cancel and close |
| `Ctrl+U` | Clear input |

### Changing Vault

Click the vault name in the bottom-right corner to switch between vaults.

## Obsidian Setup

### Required Plugins

**[Daily Notes](https://help.obsidian.md/Plugins/Daily+notes)** (Core plugin)
- Enable in Settings → Core plugins → Daily notes
- Configure your daily note folder and date format

### Recommended Plugins

**[Tasks](https://github.com/obsidian-tasks-group/obsidian-tasks)**
- Tasks are created in the format: `- [ ] task content ⏳ 2024-01-17`
- The ⏳ emoji indicates the scheduled date, which Tasks plugin understands

### Daily Notes Configuration

The app reads your daily notes settings from `.obsidian/daily-notes.json` in your vault:

- **Folder** - Where daily notes are stored (supports date patterns like `Daily/YYYY/MM`)
- **Date format** - How daily notes are named (e.g., `YYYY-MM-DD`)

Tasks are appended under a `## Tasks` heading in your daily note. If the heading doesn't exist, it's created automatically.

## How It Works

1. **Vault Discovery** - Reads `~/Library/Application Support/obsidian/obsidian.json` to find all registered vaults
2. **Date Parsing** - Uses [chrono-node](https://github.com/wanasit/chrono) to parse natural language dates
3. **Task Creation** - Appends tasks to the `## Tasks` section of today's daily note
4. **Window Management** - Creates a fresh window each time to work seamlessly with tiling window managers

## Tiling Window Manager Support

For **Aerospace** users, add this to your `~/.aerospace.toml`:

```toml
[[on-window-detected]]
if.app-id = 'com.oyvind.obsidian-quick-add'
run = 'layout floating'
```

## Tech Stack

- **[Tauri 2.0](https://tauri.app/)** - Rust-based app framework
- **[SvelteKit](https://kit.svelte.dev/)** - Frontend framework
- **[chrono-node](https://github.com/wanasit/chrono)** - Natural language date parsing

## License

MIT
