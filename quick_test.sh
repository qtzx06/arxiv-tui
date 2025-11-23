#!/bin/bash
# Quick test without Docker - just arXiv integration

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo -e "${BLUE}   Quick Test - arXiv Integration${NC}"
echo -e "${BLUE}   (No Docker/HelixDB required)${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo ""

echo "This will fetch 5 papers from arXiv about:"
echo "  • Large language models"
echo "  • Attention Is All You Need (specific paper)"
echo "  • Latest AI papers"
echo "  • Computer vision papers"
echo ""
echo "Press Enter to start..."
read

cargo run --example arxiv_search

echo ""
echo -e "${GREEN}✓ Test complete!${NC}"
echo ""
echo "To test the full RAG pipeline with HelixDB:"
echo "  1. Install Docker Desktop"
echo "  2. Run: ./run_pipeline.sh"
echo ""
