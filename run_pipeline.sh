#!/bin/bash
# Full RAG Pipeline Test Script
# Run this after installing Docker Desktop

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo -e "${BLUE}   arXiv TUI - Full RAG Pipeline Test${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo ""

# Add helix to PATH
export PATH="$PATH:/Users/qtzx/.local/bin"

# Step 1: Check Docker
echo -e "${YELLOW}[1/5]${NC} Checking Docker..."
if ! command -v docker &> /dev/null; then
    echo -e "${RED}✗ Docker not found${NC}"
    echo ""
    echo "Please install Docker Desktop:"
    echo "  1. Download from: https://www.docker.com/products/docker-desktop/"
    echo "  2. Or: brew install --cask docker"
    echo "  3. Start Docker Desktop app"
    echo "  4. Run this script again"
    exit 1
fi

if ! docker info &> /dev/null; then
    echo -e "${RED}✗ Docker is not running${NC}"
    echo ""
    echo "Please start Docker Desktop and try again"
    exit 1
fi

echo -e "${GREEN}✓ Docker is running${NC}"

# Step 2: Check HelixDB CLI
echo -e "${YELLOW}[2/5]${NC} Checking HelixDB CLI..."
if ! command -v helix &> /dev/null; then
    echo -e "${RED}✗ HelixDB CLI not found${NC}"
    echo ""
    echo "Install with: curl -sSL https://install.helix-db.com | bash"
    exit 1
fi

HELIX_VERSION=$(helix --version)
echo -e "${GREEN}✓ ${HELIX_VERSION}${NC}"

# Step 3: Deploy HelixDB
echo -e "${YELLOW}[3/5]${NC} Deploying HelixDB..."
echo "Running: helix push dev"
echo ""

if helix push dev; then
    echo -e "${GREEN}✓ HelixDB deployed successfully${NC}"
else
    echo -e "${RED}✗ Deployment failed${NC}"
    echo "Check logs above for errors"
    exit 1
fi

# Wait a moment for HelixDB to fully start
echo ""
echo "Waiting for HelixDB to be ready..."
sleep 3

# Step 4: Test connection
echo -e "${YELLOW}[4/5]${NC} Testing HelixDB connection..."
if cargo run --example check_helix 2>&1 | grep -q "HelixDB is accessible"; then
    echo -e "${GREEN}✓ Connection successful${NC}"
else
    echo -e "${RED}✗ Connection failed${NC}"
    echo "HelixDB might not be fully started yet"
    exit 1
fi

# Step 5: Run full pipeline
echo ""
echo -e "${YELLOW}[5/5]${NC} Running full RAG pipeline..."
echo "This will:"
echo "  • Fetch 5 papers from arXiv about 'transformer neural networks'"
echo "  • Generate embeddings for each paper"
echo "  • Store them in HelixDB"
echo "  • Perform semantic search"
echo ""
echo "Press Enter to continue..."
read

cargo run --example helix_integration

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════${NC}"
echo -e "${GREEN}   Pipeline Complete! ✓${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════${NC}"
echo ""
echo "Next steps:"
echo "  • Run interactive search: cargo run --example semantic_search"
echo "  • Ingest more papers: cargo run --example ingest_papers"
echo "  • Start the TUI: cargo run"
echo ""
echo "To stop HelixDB:"
echo "  helix stop dev"
echo ""
