# Fix Docker BuildKit Error

## The Error
```
failed to solve: Internal: write /var/lib/docker/buildkit/containerd-overlayfs/metadata_v2.db: input/output error
```

This is a Docker storage issue. Easy fixes below!

## Quick Fixes (Try in Order)

### Fix 1: Restart Docker Desktop (90% success rate)
```bash
# Quit Docker Desktop completely
# Then reopen it from Applications

# Or from terminal:
killall Docker && open -a Docker

# Wait 30 seconds for it to fully restart
sleep 30

# Try again:
helix push dev
```

### Fix 2: Clean Docker Build Cache
```bash
docker builder prune -af
```

This clears Docker's build cache (safe, just rebuilds next time).

Then try:
```bash
helix push dev
```

### Fix 3: Increase Docker Resources

1. Open **Docker Desktop**
2. Go to **Settings** (gear icon)
3. **Resources** tab
4. Increase:
   - **Memory**: 4GB minimum (8GB better)
   - **Disk**: Make sure you have 10GB+ free
5. Click **Apply & Restart**

Wait for restart, then:
```bash
helix push dev
```

### Fix 4: Reset Docker (Nuclear Option)

**WARNING: This deletes all Docker data**

1. Open Docker Desktop
2. **Settings** → **Troubleshoot**
3. Click **Clean / Purge data**
4. Restart Docker Desktop

Then:
```bash
helix push dev
```

## Check Disk Space

```bash
df -h
```

Make sure you have at least **10GB free** on your main disk.

## After Fix Works

Once `helix push dev` succeeds:

```bash
# Test it worked
cargo run --example check_helix

# Run full pipeline
cargo run --example helix_integration
```

## Most Likely Cause

Docker just started and BuildKit isn't fully ready. **Just restart Docker Desktop** and it usually works!
