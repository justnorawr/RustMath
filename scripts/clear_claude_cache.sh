#!/bin/bash
#
# Clear Claude Desktop Cache and Logs
#
# This script clears Claude Desktop's cache, session storage, and logs.
# Useful for clearing error dialogs that persist after fixes have been applied.
#
# Usage: ./scripts/clear_claude_cache.sh
#

set -e

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "================================================"
echo "  Claude Desktop Cache & Log Cleaner"
echo "================================================"
echo ""

# Detect OS
OS=$(uname -s)
if [ "$OS" != "Darwin" ]; then
    echo -e "${RED}ERROR: This script currently only supports macOS.${NC}"
    echo "For other platforms, manually clear the following:"
    echo "  - Application cache directory"
    echo "  - Session storage directory"
    echo "  - Log files"
    exit 1
fi

# Check if Claude is running
if pgrep -f "Claude" > /dev/null; then
    echo -e "${RED}ERROR: Claude Desktop is still running.${NC}"
    echo "Please quit Claude Desktop first (⌘Q or Cmd+Q)"
    echo ""
    exit 1
fi

echo -e "${YELLOW}This will clear:${NC}"
echo "  • Cache directories"
echo "  • Session storage"
echo "  • Local storage"
echo "  • GPU cache"
echo "  • Code cache"
echo "  • IndexedDB"
echo "  • Cookies"
echo "  • Web storage"
echo "  • Service workers"
echo "  • Log files"
echo ""
echo -e "${YELLOW}Note: Your configuration (claude_desktop_config.json) will NOT be deleted.${NC}"
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Cancelled."
    exit 0
fi

# Define directories to clear
CACHE_DIR="$HOME/Library/Application Support/Claude/Cache"
SESSION_DIR="$HOME/Library/Application Support/Claude/Session Storage"
LOCAL_STORAGE_DIR="$HOME/Library/Application Support/Claude/Local Storage"
CODE_CACHE_DIR="$HOME/Library/Application Support/Claude/Code Cache"
GPU_CACHE_DIR="$HOME/Library/Application Support/Claude/GPUCache"
APP_CACHE_DIR="$HOME/Library/Caches/com.anthropic.claudefordesktop"
LOGS_DIR="$HOME/Library/Logs/Claude"
INDEXED_DB_DIR="$HOME/Library/Application Support/Claude/IndexedDB"
COOKIES_DIR="$HOME/Library/Application Support/Claude/Cookies"
SERVICE_WORKER_DIR="$HOME/Library/Application Support/Claude/Service Worker"
WEB_STORAGE_DIR="$HOME/Library/Application Support/Claude/WebStorage"

cleared_count=0

# Clear cache directories
echo ""
echo "Clearing cache directories..."

if [ -d "$CACHE_DIR" ]; then
    rm -rf "$CACHE_DIR"/*
    echo "  ✓ Cleared: Cache"
    ((cleared_count++))
fi

if [ -d "$SESSION_DIR" ]; then
    rm -rf "$SESSION_DIR"/*
    echo "  ✓ Cleared: Session Storage"
    ((cleared_count++))
fi

if [ -d "$LOCAL_STORAGE_DIR" ]; then
    rm -rf "$LOCAL_STORAGE_DIR"/*
    echo "  ✓ Cleared: Local Storage"
    ((cleared_count++))
fi

if [ -d "$CODE_CACHE_DIR" ]; then
    rm -rf "$CODE_CACHE_DIR"/*
    echo "  ✓ Cleared: Code Cache"
    ((cleared_count++))
fi

if [ -d "$GPU_CACHE_DIR" ]; then
    rm -rf "$GPU_CACHE_DIR"/*
    echo "  ✓ Cleared: GPU Cache"
    ((cleared_count++))
fi

if [ -d "$APP_CACHE_DIR" ]; then
    rm -rf "$APP_CACHE_DIR"/*
    echo "  ✓ Cleared: Application Cache"
    ((cleared_count++))
fi

if [ -d "$INDEXED_DB_DIR" ]; then
    rm -rf "$INDEXED_DB_DIR"/*
    echo "  ✓ Cleared: IndexedDB"
    ((cleared_count++))
fi

if [ -d "$COOKIES_DIR" ]; then
    rm -rf "$COOKIES_DIR"/*
    echo "  ✓ Cleared: Cookies"
    ((cleared_count++))
fi

if [ -d "$SERVICE_WORKER_DIR" ]; then
    rm -rf "$SERVICE_WORKER_DIR"/*
    echo "  ✓ Cleared: Service Worker"
    ((cleared_count++))
fi

if [ -d "$WEB_STORAGE_DIR" ]; then
    rm -rf "$WEB_STORAGE_DIR"/*
    echo "  ✓ Cleared: Web Storage"
    ((cleared_count++))
fi

# Clear logs
echo ""
echo "Clearing log files..."

if [ -d "$LOGS_DIR" ]; then
    rm -f "$LOGS_DIR"/*.log
    echo "  ✓ Cleared: Log files"
    ((cleared_count++))
fi

# Summary
echo ""
echo "================================================"
echo -e "${GREEN}✓ Successfully cleared $cleared_count locations${NC}"
echo "================================================"
echo ""
echo "Next steps:"
echo "  1. Restart Claude Desktop"
echo "  2. Error dialogs should no longer appear"
echo "  3. MCP servers will reinitialize cleanly"
echo ""
echo -e "${YELLOW}Note:${NC} If error dialogs persist after clearing cache:"
echo "  - The MCP server may be working correctly despite the error"
echo "  - Try using the tools - they should work even if error shows"
echo "  - Error dialog may show accumulated errors from ALL previous attempts"
echo "  - Consider dismissing the dialog and testing functionality"
echo ""
