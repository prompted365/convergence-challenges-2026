"""
Hackerbot-Claw harness — deterministic GitHub Actions workflow scanner.

Static, dependency-free structural checks for the CI trust-boundary failures
documented in the hackerbot-claw campaign (Feb 21-28, 2026): privileged
triggers running attacker-controlled code, untrusted-context interpolation into
shell, remote payload execution, over-scoped tokens, and destructive or
publishing actions reachable from untrusted input.

Scope and honesty notes
-----------------------
* This is a fixture-level scanner for the harness, not a production linter.
  It reads workflow YAML as text (line + indentation aware) so the harness
  stays runnable with Python + pytest only. Mature tools (actionlint, zizmor,
  GitHub CodeQL actions queries) exist for production use.
* Known blind spots are encoded as strict-xfail tests in test_hbc_gaps.py so
  they show up in every run instead of hiding behind a pass count.
"""

from __future__ import annotations

import re
from dataclasses import dataclass, field

# ---------------------------------------------------------------------------
# Pattern tables
# ---------------------------------------------------------------------------

PRIVILEGED_TRIGGERS = ("pull_request_target", "workflow_run", "issue_comment")

# Attacker-controllable GitHub contexts (GitHub Security Lab's untrusted-input
# list, condensed). Matched inside ${{ ... }} expressions.
UNTRUSTED_CONTEXT_RE = re.compile(
    r"github\.(?:"
    r"event\.(?:"
    r"pull_request\.(?:title|body|head\.(?:ref|label|repo\.default_branch))"
    r"|issue\.(?:title|body)"
    r"|comment\.body|review\.body|review_comment\.body"
    r"|discussion\.(?:title|body)"
    r"|pages\.[^}\s]*\.page_name"
    r"|commits\.[^}\s]*\.(?:message|author\.(?:email|name))"
    r"|head_commit\.(?:message|author\.(?:email|name))"
    r"|workflow_run\.(?:head_branch|display_title|head_commit\.(?:message|author\.(?:email|name)))"
    r")"
    r"|head_ref)"
)

EXPR_RE = re.compile(r"\$\{\{(.*?)\}\}", re.DOTALL)

UNTRUSTED_REF_RE = re.compile(
    r"github\.event\.pull_request\.head\.(?:sha|ref)"
    r"|github\.head_ref"
    r"|github\.event\.workflow_run\.head_(?:sha|branch)"
    r"|refs/pull/"
)

UNTRUSTED_FETCH_CMD_RE = re.compile(
    r"\bgh\s+pr\s+checkout\b"
    r"|\bgit\s+fetch\b[^\n]*\bpull/"
    r"|\bgit\s+checkout\b[^\n]*github\.event\.pull_request\.head"
)

ARTIFACT_DOWNLOAD_RE = re.compile(
    r"uses:\s*(?:actions/download-artifact|dawidd6/action-download-artifact)@"
)

REMOTE_PIPE_EXEC_RE = re.compile(
    r"\b(?:curl|wget)\b[^\n|;]*\|\s*(?:sudo\s+)?(?:ba|z|da|k)?sh\b"
    r"|\b(?:ba)?sh\s+<\(\s*(?:curl|wget)\b"
    r"|\bbase64\s+(?:-d|--decode)\b[^\n|]*\|\s*(?:ba)?sh\b"
)

PR_CODE_EXEC_RE = re.compile(
    r"\b(?:npm|yarn|pnpm)\s+(?:install|ci|run|test|build)\b"
    r"|\bmake\b"
    r"|\bpip\s+install\b"
    r"|\bpython3?\s+setup\.py\b"
    r"|\./[\w./-]+\.sh\b"
    r"|\bgo\s+(?:generate|run|build|test)\b"
    r"|\bcargo\s+(?:build|test|run)\b"
    r"|\./gradlew\b|\bmvn\b"
    r"|\bbash\s+[\w./-]+\.sh\b"
)

DESTRUCTIVE_RE = re.compile(
    r"\bgh\s+release\s+delete\b"
    r"|\bgh\s+repo\s+(?:rename|delete|edit)\b"
    r"|\bgit\s+push\s+(?:-f|--force)\b"
    r"|-X\s+DELETE\b|--method\s+DELETE\b"
)

PUBLISH_RE = re.compile(
    r"\b(?:vsce|ovsx)\s+publish\b"
    r"|\bnpm\s+publish\b"
    r"|\btwine\s+upload\b"
    r"|\bgh\s+release\s+(?:upload|create)\b"
)

SECRET_REF_RE = re.compile(r"\$\{\{\s*secrets\.[A-Za-z0-9_]+\s*\}\}")
TOKEN_REF_RE = re.compile(
    r"\$\{\{\s*(?:secrets\.GITHUB_TOKEN|github\.token)\s*\}\}"
)

WRITE_ALL_RE = re.compile(r"^\s*permissions:\s*write-all\s*$", re.MULTILINE)
SCOPED_WRITE_RE = re.compile(
    r"^\s*(?:contents|actions|packages|pull-requests|deployments|statuses|"
    r"checks|security-events|issues|id-token):\s*write\s*$",
    re.MULTILINE,
)
PERMISSIONS_KEY_RE = re.compile(r"^\s*permissions:", re.MULTILINE)

UNPINNED_THIRD_PARTY_RE = re.compile(
    r"uses:\s*(?!actions/|github/|\./)([\w.-]+/[\w./-]+)@(?![0-9a-f]{40}\b)([\w.-]+)"
)

SEVERITY_ORDER = {"info": 0, "medium": 1, "high": 2, "critical": 3}


# ---------------------------------------------------------------------------
# Result types
# ---------------------------------------------------------------------------

@dataclass(frozen=True)
class Finding:
    code: str
    name: str
    severity: str
    evidence: str


@dataclass
class ScanResult:
    findings: list[Finding] = field(default_factory=list)
    verdicts: list[str] = field(default_factory=list)
    triggers: list[str] = field(default_factory=list)

    @property
    def codes(self) -> set[str]:
        return {f.code for f in self.findings}

    @property
    def max_severity(self) -> str:
        if not self.findings:
            return "none"
        return max(self.findings, key=lambda f: SEVERITY_ORDER[f.severity]).severity

    def has(self, code: str) -> bool:
        return code in self.codes


# ---------------------------------------------------------------------------
# YAML-as-text helpers
# ---------------------------------------------------------------------------

def _indent(line: str) -> int:
    return len(line) - len(line.lstrip(" "))


def extract_triggers(workflow: str) -> list[str]:
    """Return trigger event names declared under the top-level `on:` key."""
    lines = workflow.splitlines()
    triggers: list[str] = []
    for i, line in enumerate(lines):
        m = re.match(r'^(?:on|"on"|\'on\'):\s*(.*)$', line)
        if not m:
            continue
        inline = m.group(1).strip()
        if inline:
            triggers.extend(t.strip() for t in re.split(r"[\[\],\s]+", inline) if t.strip())
            break
        for nxt in lines[i + 1:]:
            if nxt.strip() == "" or nxt.lstrip().startswith("#"):
                continue
            if _indent(nxt) == 0:
                break
            km = re.match(r"^\s{1,4}(?:-\s+)?([A-Za-z_]+):?\s*", nxt)
            if km and _indent(nxt) <= 4:
                triggers.append(km.group(1))
        break
    return triggers


def extract_run_blocks(workflow: str) -> list[tuple[int, str]]:
    """Return (line_number, script_text) for every `run:` value."""
    lines = workflow.splitlines()
    blocks: list[tuple[int, str]] = []
    i = 0
    while i < len(lines):
        m = re.match(r"^(\s*)(-\s+)?run:\s*(.*)$", lines[i])
        if not m:
            i += 1
            continue
        key_indent = len(m.group(1)) + (len(m.group(2)) if m.group(2) else 0)
        value = m.group(3)
        if value.strip() and value.strip()[0] in "|>":
            body: list[str] = []
            j = i + 1
            while j < len(lines) and (lines[j].strip() == "" or _indent(lines[j]) > key_indent):
                body.append(lines[j])
                j += 1
            blocks.append((i + 1, "\n".join(body)))
            i = j
        else:
            blocks.append((i + 1, value))
            i += 1
    return blocks


def _untrusted_in(text: str) -> list[str]:
    hits = []
    for expr in EXPR_RE.findall(text):
        m = UNTRUSTED_CONTEXT_RE.search(expr)
        if m:
            hits.append(m.group(0))
    return hits


def _first_line(pattern: re.Pattern, text: str) -> int | None:
    for n, line in enumerate(text.splitlines(), start=1):
        if pattern.search(line):
            return n
    return None


# ---------------------------------------------------------------------------
# Scanner
# ---------------------------------------------------------------------------

def scan_workflow(workflow: str) -> ScanResult:
    """Run every structural check over one workflow file's text."""
    result = ScanResult(triggers=extract_triggers(workflow))
    add = result.findings.append

    privileged = [t for t in result.triggers if t in PRIVILEGED_TRIGGERS]
    if privileged:
        add(Finding("HBC-T1", "privileged_trigger", "info",
                    f"privileged trigger(s): {', '.join(privileged)}"))

    # T2 — untrusted code or artifact brought into the privileged job
    untrusted_checkout_line = None
    for n, line in enumerate(workflow.splitlines(), start=1):
        if ("ref:" in line or "repository:" in line) and UNTRUSTED_REF_RE.search(line):
            untrusted_checkout_line = n
            break
        if UNTRUSTED_FETCH_CMD_RE.search(line) or ARTIFACT_DOWNLOAD_RE.search(line):
            untrusted_checkout_line = n
            break
    if untrusted_checkout_line is not None:
        add(Finding("HBC-T2", "untrusted_code_or_artifact", "high" if privileged else "medium",
                    f"untrusted ref/artifact pulled at line {untrusted_checkout_line}"))

    # T3 — untrusted context interpolated straight into shell
    run_blocks = extract_run_blocks(workflow)
    for line_no, script in run_blocks:
        hits = _untrusted_in(script)
        if hits:
            add(Finding("HBC-T3", "untrusted_interpolation_in_run",
                        "critical" if privileged else "high",
                        f"line {line_no}: {', '.join(sorted(set(hits)))}"))
            break

    # T4 — remote payload piped to a shell
    for line_no, script in run_blocks:
        if REMOTE_PIPE_EXEC_RE.search(script):
            add(Finding("HBC-T4", "remote_payload_exec", "high", f"line {line_no}"))
            break

    # T5 — token scope
    write_all = bool(WRITE_ALL_RE.search(workflow))
    scoped_write = bool(SCOPED_WRITE_RE.search(workflow))
    no_permissions = not PERMISSIONS_KEY_RE.search(workflow)
    write_capable = write_all or scoped_write
    if write_all:
        add(Finding("HBC-T5", "token_write_all", "high" if privileged else "medium",
                    "permissions: write-all"))
    elif scoped_write and privileged:
        add(Finding("HBC-T5", "token_scoped_write_on_privileged_trigger", "medium",
                    "write scope granted to a privileged-trigger workflow"))
    elif no_permissions and privileged:
        add(Finding("HBC-T5", "token_implicit_default", "medium",
                    "no permissions block; token scope falls back to repo/org default"))

    # T6 — secrets reachable in the same workflow as untrusted code
    secret_refs = SECRET_REF_RE.findall(workflow)
    if secret_refs and untrusted_checkout_line is not None:
        add(Finding("HBC-T6", "secret_reachable_from_untrusted_code", "critical" if privileged else "high",
                    f"{len(secret_refs)} secret reference(s) alongside untrusted checkout"))

    # T7 — destructive repository actions
    destructive_line = _first_line(DESTRUCTIVE_RE, workflow)
    if destructive_line is not None:
        add(Finding("HBC-T7", "destructive_repo_action", "high", f"line {destructive_line}"))

    # T8 — PR-controlled code executed after the untrusted checkout
    pr_exec = False
    if untrusted_checkout_line is not None:
        tail = "\n".join(workflow.splitlines()[untrusted_checkout_line:])
        if PR_CODE_EXEC_RE.search(tail):
            pr_exec = True
            add(Finding("HBC-T8", "pr_code_executed_after_checkout",
                        "critical" if privileged else "medium",
                        "build/test/install runs on attacker-controlled tree"))

    # T9 — publishing reachable from untrusted input
    publish_line = _first_line(PUBLISH_RE, workflow)
    if publish_line is not None and (privileged and untrusted_checkout_line is not None):
        add(Finding("HBC-T9", "publish_from_untrusted_tree", "critical",
                    f"line {publish_line}: artifact published after untrusted checkout"))

    # T10 — unpinned third-party action (mutable reference)
    m = UNPINNED_THIRD_PARTY_RE.search(workflow)
    if m:
        add(Finding("HBC-T10", "unpinned_third_party_action", "medium",
                    f"{m.group(1)}@{m.group(2)} is a mutable reference"))

    # ---- composite verdicts ------------------------------------------------
    token_or_secret = bool(secret_refs) or write_capable or no_permissions
    if privileged and untrusted_checkout_line is not None and pr_exec and token_or_secret:
        result.verdicts.append("PWN_REQUEST")
    if result.has("HBC-T3"):
        result.verdicts.append("SCRIPT_INJECTION")
    if result.has("HBC-T4"):
        result.verdicts.append("REMOTE_EXEC")
    if result.has("HBC-T7") and (write_capable or TOKEN_REF_RE.search(workflow)):
        result.verdicts.append("DESTRUCTIVE_WITH_TOKEN")
    if result.has("HBC-T9"):
        result.verdicts.append("ARTIFACT_POISONING")
    return result


# ---------------------------------------------------------------------------
# Untrusted-input payload checks (branch names, titles, filenames, PR bodies)
# ---------------------------------------------------------------------------

SHELL_METACHAR_RE = re.compile(r"\$\(|`|;\s*\w|\|\s*\w|&&|\$\{IFS\}")

REVIEWER_DIRECTED_RE = re.compile(
    r"\b(?:ai|automated|bot|llm|copilot|code)\s*(?:reviewers?|review(?:ing)?\s+agents?|assistants?)\b"
    r"[^\n]{0,160}\b(?:approve|merge|ignore|skip|do not flag|pre-?approved)\b"
    r"|\bignore\s+(?:all\s+|any\s+)?(?:previous|prior|above)\s+instructions\b",
    re.IGNORECASE,
)


def scan_untrusted_input(value: str) -> list[Finding]:
    """Checks applied to attacker-supplied strings that reach CI or reviewers."""
    out: list[Finding] = []
    if SHELL_METACHAR_RE.search(value):
        out.append(Finding("HBC-P1", "shell_metacharacters_in_input", "high",
                           "input carries command-substitution or chaining syntax"))
    if REMOTE_PIPE_EXEC_RE.search(value):
        out.append(Finding("HBC-P2", "remote_exec_in_input", "critical",
                           "input embeds a fetch-and-execute chain"))
    if REVIEWER_DIRECTED_RE.search(value):
        out.append(Finding("HBC-P3", "reviewer_directed_instruction", "high",
                           "text addresses an automated reviewer with approval/ignore instructions"))
    return out
