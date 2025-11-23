# Start Docker Desktop

Docker is installed but not running!

## Quick Fix

### macOS
1. **Open Spotlight** (Cmd + Space)
2. **Type**: "Docker"
3. **Press Enter** to launch Docker Desktop
4. **Wait** for the whale icon to appear in menu bar (top right)
5. **Icon should stop animating** when ready

### Or from Terminal
```bash
open -a Docker
```

## Verify It's Running

Wait about 30 seconds, then:

```bash
docker info
```

Should show Docker info (not an error).

## Then Deploy HelixDB

Once Docker Desktop is fully started:

```bash
./run_pipeline.sh
```

Or manually:

```bash
helix push dev
```

---

## What You'll See

When Docker Desktop starts:
- 🐳 Whale icon appears in menu bar
- Icon animates while starting
- Icon stops animating = ready!

Takes ~30 seconds on first start.

---

## After Docker Starts

Run the full pipeline:

```bash
./run_pipeline.sh
```

This will:
1. ✓ Check Docker is running
2. ✓ Deploy HelixDB
3. ✓ Test connection
4. ✓ Run full RAG pipeline

LET'S GO! 🚀
