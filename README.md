# dirpulse

A CLI tool that analyzes directory contents and reports statistics about files, including size distribution, file types, and age.

## Features

- Total file and directory counts with size summaries
- Top N largest files
- File extension breakdown
- File age distribution (fresh, aging, stale)

## Installation

### From crates.io

```bash
cargo install dirpulse
```

### From source

```bash
git clone https://github.com/yourusername/dirpulse
cd dirpulse
cargo install --path .
```

## Usage

```bash
# Analyze current directory
dirpulse .

# Analyze a specific directory
dirpulse /path/to/directory

# Show top 20 largest files (default is 10)
dirpulse /path/to/directory -n 20
```

## Example Output

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│ dirpulse · /home/user/projects                                                           │
├──────────────────────────────────────────────────────────────────────────────────────────┤
│ Stale: 1.2 GB, Largest: video.mp4 (800 MB), Top 10 = 45.2%                               │
└──────────────────────────────────────────────────────────────────────────────────────────┘

1284 files · 89 directories · 4.5 GB total

── Top 10 Largest ────────────────────────────────────────────────────────────
1          800 MB    video.mp4
2          450 MB    backup.tar.gz
...

── By Extension ──────────────────────────────────────────────────────────────
.rs                       342           12.4 MB
.json                     128            2.1 MB
...

── File Age ──────────────────────────────────────────────────────────────────
Fresh (< 30 days)          234 files      1.2 GB
Aging (30 days - 6 mo)     450 files      2.1 GB
Stale (> 6 months)         600 files      1.2 GB
```

## License

MIT
