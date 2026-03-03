#!/usr/bin/env bash
# Federal Register AI governance monitoring script
# Fetches recent documents related to AI safety, autonomous systems, defense AI
# Usage: ./monitor.sh [--days N] [--output DIR]
#
# Contact: breyden@prompted.community

set -euo pipefail

DAYS="${1:-30}"
OUTPUT_DIR="${2:-$(dirname "$0")/results}"
BASE_URL="https://www.federalregister.gov/api/v1/documents.json"
DATE_FROM=$(date -v-"${DAYS}"d +%Y-%m-%d 2>/dev/null || date -d "${DAYS} days ago" +%Y-%m-%d)
DATE_TO=$(date +%Y-%m-%d)

mkdir -p "$OUTPUT_DIR"

echo "Federal Register AI Governance Monitor"
echo "Date range: ${DATE_FROM} to ${DATE_TO}"
echo "Output: ${OUTPUT_DIR}"
echo "---"

# Search patterns
declare -A SEARCHES=(
    ["ai-executive-orders"]="conditions[term]=artificial+intelligence&conditions[type][]=PRESDOCU"
    ["ai-safety-nist"]="conditions[term]=AI+safety&conditions[agencies][]=national-institute-of-standards-and-technology"
    ["defense-ai"]="conditions[term]=artificial+intelligence+autonomous&conditions[agencies][]=defense-department"
    ["ai-trading-sec"]="conditions[term]=algorithmic+trading+artificial+intelligence&conditions[agencies][]=securities-and-exchange-commission"
    ["nuclear-autonomous"]="conditions[term]=nuclear+autonomous+artificial+intelligence"
    ["cisa-ai"]="conditions[term]=artificial+intelligence&conditions[agencies][]=cybersecurity-and-infrastructure-security-agency"
    ["ai-rules-proposed"]="conditions[term]=artificial+intelligence&conditions[type][]=PRORULE"
)

for name in "${!SEARCHES[@]}"; do
    query="${SEARCHES[$name]}"
    url="${BASE_URL}?${query}&conditions[publication_date][gte]=${DATE_FROM}&conditions[publication_date][lte]=${DATE_TO}&per_page=20&order=newest"
    outfile="${OUTPUT_DIR}/${name}_${DATE_TO}.json"

    echo "Fetching: ${name}..."
    if curl -s -f "$url" > "$outfile" 2>/dev/null; then
        count=$(python3 -c "import json; d=json.load(open('$outfile')); print(d.get('count', 0))" 2>/dev/null || echo "?")
        echo "  -> ${count} results -> ${outfile}"
    else
        echo "  -> FAILED"
        rm -f "$outfile"
    fi
done

echo "---"
echo "Done. Results in ${OUTPUT_DIR}/"
