#!/bin/bash
set -eo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "========================================"
echo "  metal.rs crates.io publish script"
echo "========================================"
echo ""

# Check if logged in to crates.io
echo -e "${YELLOW}Checking crates.io login status...${NC}"
if ! cargo login --help > /dev/null 2>&1; then
    echo -e "${RED}Error: cargo not found${NC}"
    exit 1
fi

# Check for uncommitted changes
echo -e "${YELLOW}Checking git status...${NC}"
if ! git diff --quiet || ! git diff --cached --quiet; then
    echo -e "${RED}Error: You have uncommitted changes.${NC}"
    echo "Please commit all changes before publishing:"
    echo "  git add -A && git commit -m 'Prepare for v1.0.0 release'"
    echo ""
    git status --short
    exit 1
fi
echo -e "${GREEN}✓${NC} Git working directory is clean"

# Crates in dependency order (must publish in this order)
CRATES=(
    "mtl-sys"
    "mtl-foundation"
    "mtl"
    "mtl-quartz-core"
    "mtl-fx"
)

# Step 1: Dry run first crate only (others will fail because deps aren't on crates.io yet)
echo ""
echo -e "${YELLOW}Step 1: Running dry-run for mtl-sys (base crate)...${NC}"
echo ""

if ! cargo publish -p mtl-sys --dry-run; then
    echo -e "${RED}Dry-run failed for mtl-sys${NC}"
    exit 1
fi
echo -e "${GREEN}✓${NC} mtl-sys passed dry-run"

echo ""
echo -e "${YELLOW}Note: Skipping dry-run for dependent crates (they can't verify until dependencies are published)${NC}"

# Step 2: Ask for confirmation
echo ""
echo "========================================"
echo "  Ready to publish the following crates:"
echo "========================================"
echo ""
for crate in "${CRATES[@]}"; do
    echo "  - $crate (v1.0.0)"
done
echo ""
echo -e "${YELLOW}WARNING: Once published, versions cannot be changed or deleted!${NC}"
echo ""
read -p "Type 'publish' to confirm and publish all crates: " confirmation

if [ "$confirmation" != "publish" ]; then
    echo -e "${RED}Aborted.${NC}"
    exit 1
fi

# Step 3: Publish each crate
echo ""
echo -e "${YELLOW}Step 2: Publishing crates...${NC}"
echo ""

for crate in "${CRATES[@]}"; do
    echo -e "Publishing ${GREEN}$crate${NC}..."
    if ! cargo publish -p "$crate"; then
        echo -e "${RED}Failed to publish $crate${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓${NC} $crate published successfully"

    # Wait between publishes for crates.io to index
    if [ "$crate" != "mtl-fx" ]; then
        echo "  Waiting 30 seconds for crates.io to index..."
        sleep 30
    fi
done

echo ""
echo "========================================"
echo -e "${GREEN}  All crates published successfully!${NC}"
echo "========================================"
echo ""
echo "Your crates are now available at:"
echo "  https://crates.io/crates/mtl-sys"
echo "  https://crates.io/crates/mtl-foundation"
echo "  https://crates.io/crates/mtl"
echo "  https://crates.io/crates/mtl-quartz-core"
echo "  https://crates.io/crates/mtl-fx"
echo ""
