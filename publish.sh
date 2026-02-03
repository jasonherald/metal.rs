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
    "metal-sys"
    "metal-foundation"
    "metal"
    "quartz-core"
    "metal-fx"
)

# Step 1: Dry run all crates first
echo ""
echo -e "${YELLOW}Step 1: Running dry-run for all crates...${NC}"
echo ""

for crate in "${CRATES[@]}"; do
    echo -e "  Checking ${GREEN}$crate${NC}..."
    if ! cargo publish -p "$crate" --dry-run 2>&1 | grep -v "Uploading\|Compiling\|Finished\|Packaged"; then
        echo -e "${RED}Dry-run failed for $crate${NC}"
        exit 1
    fi
    echo -e "  ${GREEN}✓${NC} $crate passed dry-run"
done

echo ""
echo -e "${GREEN}All dry-runs passed!${NC}"
echo ""

# Step 2: Ask for confirmation
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
    if [ "$crate" != "metal-fx" ]; then
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
echo "  https://crates.io/crates/metal-sys"
echo "  https://crates.io/crates/metal-foundation"
echo "  https://crates.io/crates/metal"
echo "  https://crates.io/crates/quartz-core"
echo "  https://crates.io/crates/metal-fx"
echo ""
