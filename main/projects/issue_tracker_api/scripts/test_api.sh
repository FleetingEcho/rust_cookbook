#!/usr/bin/env bash
# Runs a full end-to-end test of every API endpoint.
# Requires: curl, jq (brew install jq)
# Usage:
#   ./scripts/test_api.sh              # default: http://127.0.0.1:3001
#   BASE=http://example.com ./scripts/test_api.sh

set -euo pipefail

BASE="${BASE:-http://127.0.0.1:3001}"
API_KEY="${API_KEY:-dev-secret}"
HEADERS=(-H "x-api-key: $API_KEY" -H "Content-Type: application/json")
PASS=0
FAIL=0

# ── helpers ────────────────────────────────────────────────────────────────────

GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

log_section() { echo -e "\n${CYAN}${BOLD}── $1 ──${RESET}"; }

check() {
    local label="$1"
    local expected="$2"
    local actual="$3"
    if [[ "$actual" == "$expected" ]]; then
        echo -e "  ${GREEN}✓${RESET} $label"
        ((PASS++)) || true
    else
        echo -e "  ${RED}✗${RESET} $label  (expected ${BOLD}$expected${RESET}, got ${BOLD}$actual${RESET})"
        ((FAIL++)) || true
    fi
}

# Run curl and capture HTTP status + body separately.
# Usage: call <METHOD> <PATH> [extra curl args...]
call() {
    local method="$1"; shift
    local path="$1";   shift
    RESPONSE=$(curl -s -w "\n__STATUS__%{http_code}" \
        -X "$method" "${HEADERS[@]}" "$@" "${BASE}${path}")
    BODY=$(echo "$RESPONSE" | sed '$d')
    STATUS=$(echo "$RESPONSE" | tail -1 | sed 's/__STATUS__//')
}

pretty() { echo "$BODY" | jq . 2>/dev/null || echo "$BODY"; }

# ── 1. Health ──────────────────────────────────────────────────────────────────

log_section "Health"
call GET /health
check "GET /health → 200" "200" "$STATUS"
check "status field is ok" "ok" "$(echo "$BODY" | jq -r '.status')"
pretty

# ── 2. Issues ─────────────────────────────────────────────────────────────────

log_section "Issues – list"
call GET /api/issues
check "GET /api/issues → 200" "200" "$STATUS"
INITIAL_COUNT=$(echo "$BODY" | jq 'length')
echo "  existing issues: $INITIAL_COUNT"

log_section "Issues – list with filters"
call GET "/api/issues?status=open&priority=high"
check "GET /api/issues?status=open&priority=high → 200" "200" "$STATUS"

call GET "/api/issues?search=test"
check "GET /api/issues?search=test → 200" "200" "$STATUS"

log_section "Issues – create"
call POST /api/issues \
    --data '{"title":"Test issue from script","description":"Created by test_api.sh","priority":"medium","issue_type":"bug","assignee":"Teng","created_by":"test_script"}'
check "POST /api/issues → 200" "200" "$STATUS"
ISSUE_ID=$(echo "$BODY" | jq -r '.issue.id')
check "new issue has id" "true" "$( [[ "$ISSUE_ID" =~ ^[0-9]+$ ]] && echo true || echo false )"
echo "  created issue id: $ISSUE_ID"
pretty

log_section "Issues – get detail"
call GET "/api/issues/$ISSUE_ID"
check "GET /api/issues/:id → 200" "200" "$STATUS"
check "title matches" "Test issue from script" "$(echo "$BODY" | jq -r '.issue.title')"

log_section "Issues – update"
call PATCH "/api/issues/$ISSUE_ID" \
    --data '{"title":"Updated title","status":"in_progress","priority":"high"}'
check "PATCH /api/issues/:id → 200" "200" "$STATUS"
check "title updated" "Updated title" "$(echo "$BODY" | jq -r '.issue.title')"
check "status updated" "in_progress" "$(echo "$BODY" | jq -r '.issue.status')"

log_section "Issues – invalid update (bad status)"
call PATCH "/api/issues/$ISSUE_ID" --data '{"status":"unknown_status"}'
check "PATCH with bad status → 400" "400" "$STATUS"

# ── 3. Labels ─────────────────────────────────────────────────────────────────

log_section "Labels – list"
call GET /api/labels
check "GET /api/labels → 200" "200" "$STATUS"

log_section "Labels – create"
call POST /api/labels \
    --data '{"name":"test-label-script","color":"#ff9900"}'
check "POST /api/labels → 200" "200" "$STATUS"
LABEL_ID=$(echo "$BODY" | jq -r '.id')
check "new label has id" "true" "$( [[ "$LABEL_ID" =~ ^[0-9]+$ ]] && echo true || echo false )"
echo "  created label id: $LABEL_ID"

log_section "Labels – invalid create (missing color)"
call POST /api/labels --data '{"name":"no-color"}'
check "POST /api/labels missing color → 400" "400" "$STATUS"

log_section "Labels – add to issue"
call POST "/api/issues/$ISSUE_ID/labels/$LABEL_ID"
check "POST /api/issues/:id/labels/:label_id → 200" "200" "$STATUS"
check "linked is true" "true" "$(echo "$BODY" | jq -r '.linked')"

log_section "Labels – verify on issue detail"
call GET "/api/issues/$ISSUE_ID"
check "label appears in issue detail" "test-label-script" \
    "$(echo "$BODY" | jq -r '.labels[] | select(.id=='"$LABEL_ID"') | .name')"

log_section "Labels – remove from issue"
call DELETE "/api/issues/$ISSUE_ID/labels/$LABEL_ID"
check "DELETE /api/issues/:id/labels/:label_id → 200" "200" "$STATUS"
check "deleted is true" "true" "$(echo "$BODY" | jq -r '.deleted')"

log_section "Labels – list with filter"
call GET "/api/issues?label_id=$LABEL_ID"
check "GET /api/issues?label_id=:id → 200" "200" "$STATUS"

# ── 4. Comments ───────────────────────────────────────────────────────────────

log_section "Comments – list"
call GET "/api/issues/$ISSUE_ID/comments"
check "GET /api/issues/:id/comments → 200" "200" "$STATUS"

log_section "Comments – create"
call POST "/api/issues/$ISSUE_ID/comments" \
    --data '{"author":"Teng","body":"Looks good, tested locally."}'
check "POST /api/issues/:id/comments → 200" "200" "$STATUS"
COMMENT_ID=$(echo "$BODY" | jq -r '.id')
check "new comment has id" "true" "$( [[ "$COMMENT_ID" =~ ^[0-9]+$ ]] && echo true || echo false )"
echo "  created comment id: $COMMENT_ID"

log_section "Comments – invalid create (missing body)"
call POST "/api/issues/$ISSUE_ID/comments" --data '{"author":"Teng","body":""}'
check "POST comment with empty body → 400" "400" "$STATUS"

log_section "Comments – verify on issue detail"
call GET "/api/issues/$ISSUE_ID"
check "comment appears in issue detail" "Teng" \
    "$(echo "$BODY" | jq -r '.comments[] | select(.id=='"$COMMENT_ID"') | .author')"

log_section "Comments – delete"
call DELETE "/api/comments/$COMMENT_ID"
check "DELETE /api/comments/:id → 200" "200" "$STATUS"
check "deleted is true" "true" "$(echo "$BODY" | jq -r '.deleted')"

log_section "Comments – delete non-existent"
call DELETE "/api/comments/999999"
check "DELETE non-existent comment → 404" "404" "$STATUS"

# ── 5. Attachments ────────────────────────────────────────────────────────────

log_section "Attachments – list (empty)"
call GET "/api/issues/$ISSUE_ID/attachments"
check "GET /api/issues/:id/attachments → 200" "200" "$STATUS"

log_section "Attachments – upload"
TMPFILE=$(mktemp /tmp/test_upload_XXXX.txt)
echo "hello from test_api.sh $(date)" > "$TMPFILE"

# Upload does not use JSON content-type; override headers
RESPONSE=$(curl -s -w "\n__STATUS__%{http_code}" \
    -X POST \
    -H "x-api-key: $API_KEY" \
    -F "file=@$TMPFILE;type=text/plain" \
    "${BASE}/api/issues/$ISSUE_ID/attachments")
BODY=$(echo "$RESPONSE" | sed '$d')
STATUS=$(echo "$RESPONSE" | tail -1 | sed 's/__STATUS__//')

check "POST /api/issues/:id/attachments → 200" "200" "$STATUS"
ATTACHMENT_ID=$(echo "$BODY" | jq -r '.id')
check "new attachment has id" "true" "$( [[ "$ATTACHMENT_ID" =~ ^[0-9]+$ ]] && echo true || echo false )"
echo "  created attachment id: $ATTACHMENT_ID"
pretty

log_section "Attachments – list (after upload)"
call GET "/api/issues/$ISSUE_ID/attachments"
check "attachment count is 1" "1" "$(echo "$BODY" | jq 'length')"

log_section "Attachments – download"
DOWNLOAD=$(curl -s -w "\n__STATUS__%{http_code}" \
    -H "x-api-key: $API_KEY" \
    "${BASE}/api/attachments/$ATTACHMENT_ID/download")
DOWN_BODY=$(echo "$DOWNLOAD" | sed '$d')
DOWN_STATUS=$(echo "$DOWNLOAD" | tail -1 | sed 's/__STATUS__//')
check "GET /api/attachments/:id/download → 200" "200" "$DOWN_STATUS"
check "downloaded content matches" "true" "$( echo "$DOWN_BODY" | grep -q 'hello from test_api.sh' && echo true || echo false )"

log_section "Attachments – delete"
call DELETE "/api/attachments/$ATTACHMENT_ID"
check "DELETE /api/attachments/:id → 200" "200" "$STATUS"
check "deleted is true" "true" "$(echo "$BODY" | jq -r '.deleted')"

rm -f "$TMPFILE"

# ── 6. Cleanup: delete the test issue ─────────────────────────────────────────

log_section "Cleanup"
call DELETE "/api/issues/$ISSUE_ID"
check "DELETE /api/issues/:id → 200" "200" "$STATUS"
check "deleted is true" "true" "$(echo "$BODY" | jq -r '.deleted')"

log_section "Issues – get deleted issue (expect 404)"
call GET "/api/issues/$ISSUE_ID"
check "GET deleted issue → 404" "404" "$STATUS"

# ── Summary ───────────────────────────────────────────────────────────────────

echo ""
echo -e "${BOLD}Results: ${GREEN}$PASS passed${RESET}${BOLD}, ${RED}$FAIL failed${RESET}"
[[ "$FAIL" -eq 0 ]] && exit 0 || exit 1
