#!/usr/bin/env bash
#
# fix-flox-gvisor.sh - Fix Flox/Nix in gVisor (runsc) environments
#
# gVisor's pseudo-terminal (pty) implementation is incomplete, which causes
# Nix builds to fail with "reading a line: Input/output error" even though
# the builder process completes successfully. This script works around that
# by configuring Nix, starting the daemon, running the build (which creates
# valid outputs despite the error), and manually registering the outputs.
#
# Usage: bash .claude/fix-flox-gvisor.sh

set -euo pipefail

# Detect remote/gVisor environment
if [ "$CLAUDE_CODE_REMOTE" != "true" ] && [ "$(hostname)" != "runsc" ]; then
    echo "Not a remote/gVisor environment, skipping."
    exit 0
fi

echo "==> gVisor environment detected, applying Flox/Nix fixes..."

# 1. Configure Nix for gVisor compatibility
if ! grep -q "sandbox = false" /etc/nix/nix.conf 2>/dev/null; then
    echo "==> Configuring /etc/nix/nix.conf..."
    cat >> /etc/nix/nix.conf <<EOF
sandbox = false
filter-syscalls = false
experimental-features = nix-command flakes
EOF
fi

# 2. Start nix-daemon if not running
if ! pgrep -x nix-daemon > /dev/null 2>&1; then
    echo "==> Starting nix-daemon..."
    nix-daemon &
    sleep 2
    if [ ! -S /nix/var/nix/daemon-socket/socket ]; then
        echo "ERROR: nix-daemon failed to start"
        exit 1
    fi
fi

# 3. Attempt flox activate (will likely fail on first run due to pty bug)
echo "==> Testing flox activate..."
if flox activate -- true 2>/dev/null; then
    echo "==> Flox is already working!"
    exit 0
fi

echo "==> Flox activate failed (expected on gVisor). Building environment and registering outputs..."

# 4. Trigger the build - it will fail but create valid outputs
#    Capture the derivation path from the error output
BUILD_OUTPUT=$(flox activate -- true 2>&1 || true)

# 5. Find unregistered environment outputs in /nix/store
#    The flox environment derivation creates two outputs: environment-develop and environment-runtime
DEVELOP_PATH=$(find /nix/store -maxdepth 1 -name "*-environment-develop" -type d 2>/dev/null | head -1)
RUNTIME_PATH=$(find /nix/store -maxdepth 1 -name "*-environment-runtime" -type d 2>/dev/null | head -1)

if [ -z "$DEVELOP_PATH" ] || [ -z "$RUNTIME_PATH" ]; then
    echo "ERROR: Could not find environment outputs in /nix/store"
    echo "Build output was: $BUILD_OUTPUT"
    exit 1
fi

# Check if they're already valid
if nix-store --check-validity "$DEVELOP_PATH" 2>/dev/null && \
   nix-store --check-validity "$RUNTIME_PATH" 2>/dev/null; then
    echo "==> Outputs already registered, testing flox..."
    flox activate -- echo "Flox is working!"
    exit 0
fi

echo "==> Registering $DEVELOP_PATH and $RUNTIME_PATH..."

# 6. Find the environment derivation
ENV_DRV=$(find /nix/store -maxdepth 1 -name "*-environment.drv" 2>/dev/null | head -1)
if [ -z "$ENV_DRV" ]; then
    ENV_DRV=""
fi

# Helper: compute hex SHA256 of NAR dump
nar_hash() {
    nix-store --dump "$1" | sha256sum | cut -d' ' -f1
}

# Helper: compute NAR size
nar_size() {
    nix-store --dump "$1" | wc -c
}

# Helper: get store path references from symlinks
get_refs() {
    find "$1" -type l -exec readlink {} \; 2>/dev/null \
        | grep '^/nix/store/' \
        | sed 's|^\(/nix/store/[^/]*\).*|\1|' \
        | sort -u
}

# 7. Register both outputs using nix-store --load-db
{
    for STORE_PATH in "$DEVELOP_PATH" "$RUNTIME_PATH"; do
        HASH=$(nar_hash "$STORE_PATH")
        SIZE=$(nar_size "$STORE_PATH")
        REFS=$(get_refs "$STORE_PATH")
        REF_COUNT=$(echo "$REFS" | wc -l)

        echo "$STORE_PATH"
        echo "$HASH"
        echo "$SIZE"
        echo "$ENV_DRV"
        echo "$REF_COUNT"
        echo "$REFS"
    done
} | nix-store --load-db

# 8. Verify and test
if nix-store --check-validity "$DEVELOP_PATH" && \
   nix-store --check-validity "$RUNTIME_PATH"; then
    echo "==> Outputs registered successfully!"
else
    echo "ERROR: Failed to register outputs"
    exit 1
fi

if flox activate -- echo "Flox is working!"; then
    echo "==> Fix applied successfully. 'just' commands should now work."
else
    echo "ERROR: flox activate still failing after fix"
    exit 1
fi
