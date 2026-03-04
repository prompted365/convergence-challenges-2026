# selfcheck-core

**Stealth-normalized forensic detection toolkit for Cisco appliance compromise**

> **Status: Pre-release — pending independent review.**
> This codebase is published for transparency, audit, and community review. It is **not recommended for production deployment** until it has been independently validated by qualified security researchers and endorsed by relevant investigative authorities. The author is actively requesting public audit and does not recommend direct operational use ahead of that process. If investigators or response teams choose to evaluate the toolkit, the documentation below is written to make that adoption low-lift — but the decision to deploy belongs to the teams who own the risk.

Built in response to the active exploitation of CVE-2026-20127 and CVE-2025-20393 — both CVSS 10.0 — by threat actors tracked as UAT-8616 and UAT-9686. These campaigns target Cisco Catalyst SD-WAN infrastructure and Cisco Secure Email Gateway / Secure Email and Web Manager appliances. Exploitation has been confirmed in the wild since at least 2023.

This toolkit is designed to run on a suspect appliance (or against a mounted forensic image) and produce a structured findings report. It is designed to detect compromise even when the attacker is actively monitoring the device for forensic activity.

---

## If you think your organization may be affected

Read this section first. The technical details are below. **The guidance in this section comes from the official advisories and hunt guides published by CISA, ASD/ACSC, and Cisco — not from this tool.** Follow the official guidance regardless of whether you ever run this codebase.

**What devices are at risk?**

Two product families:

- Cisco Catalyst SD-WAN Controllers (formerly vSmart) and SD-WAN Manager (formerly vManage). The vulnerability (CVE-2026-20127) allows an unauthenticated remote attacker to bypass authentication entirely and gain administrative access. The attacker then escalates to root by downgrading firmware to a version vulnerable to CVE-2022-20775, gaining root, and restoring the original firmware — leaving the device reporting a clean version.

- Cisco Secure Email Gateway and Secure Email and Web Manager (AsyncOS). The vulnerability (CVE-2025-20393) allows root command execution. The attacker installs a suite of implants: AquaShell (a Python backdoor hidden in the existing web server), AquaTunnel (a reverse SSH tunnel for persistent access), AquaPurge (a tool that surgically removes specific log entries), and Chisel (a tunneling tool for covert data exfiltration).

**How long have these been exploited?**

Cisco Talos confirmed exploitation of SD-WAN infrastructure dating back to at least 2023. The AsyncOS campaign was confirmed active since at least November 2025. Both were publicly disclosed on February 25, 2026, in a coordinated disclosure involving CISA, ASD/ACSC, the Canadian Centre for Cyber Security, and Singapore's CSA.

**What should I do right now?**

Follow the guidance in CISA Emergency Directive 26-03 and the ASD/ACSC Cisco SD-WAN Threat Hunt Guide. The essential steps are: patch to the versions specified in the Cisco Security Advisories, check for the indicators of compromise listed below, and — critically — do not assume a clean-looking device is clean. These attackers specifically erase the evidence of their access.

These are the official recommended actions from the relevant authorities. They do not depend on this tool. The indicators below are drawn directly from the published Cisco Talos reports and the ASD/ACSC Hunt Guide.

**Indicators to check manually (per official advisories — no tooling required):**

On SD-WAN devices:
- Unexpected SSH keys in `/home/root/.ssh/authorized_keys` or `/home/vmanage-admin/.ssh/authorized_keys`
- `PermitRootLogin yes` in `/etc/ssh/sshd_config`
- Abnormally small, zero-byte, or missing log files
- Artifacts in `/var/volatile/log/sw_script_synccdb.log` or `/var/log/tmplog/vdebug` indicating firmware downgrade activity

On AsyncOS devices:
- Unexpected content in `/data/web/euq_webui/htdocs/index.py` (AquaShell implant location)
- Unknown ELF binaries, particularly GoLang-compiled reverse SSH tools
- Network connections to 172.233.67.176, 172.237.29.147, or 38.54.56.95
- Log files with conspicuously clean gaps where entries were surgically removed

**Why does this matter to my organization specifically?**

Both campaigns target network infrastructure that sits in high-trust positions. An SD-WAN Controller compromise gives the attacker NETCONF access to every device in the SD-WAN fabric — they can reconfigure your entire network. An email gateway compromise gives the attacker access to every email transiting the organization, plus the management interface that your administrators trust. These are not endpoints. They are infrastructure. The blast radius scales with how much of your network depends on them.

---

## For investigation teams and qualified engineers

> This section is technical reference for people evaluating the codebase for potential adoption. **This tool has not been validated by independent review and is not endorsed by any authority.** It is published so that qualified teams can read it, break it, and decide for themselves whether any of it is useful. The author welcomes audit, criticism, and correction.

### What this tool does

selfcheck-core is a Rust crate (with a Python mirror test harness) that performs layered forensic detection across 13 detection layers. The layers are designed to catch compromise even when the attacker has cleaned up after themselves — because cleanup itself creates detectable artifacts.

The tool wraps every filesystem and process-table read in a stealth normalization layer called **torque**, which disguises forensic I/O as normal system activity (ambient read patterns, jitter envelopes, logarithmic drift). This matters because these specific threat actors have demonstrated the ability to monitor devices for forensic activity.

### Detection layers

| Layer | Module | What it detects |
|-------|--------|-----------------|
| 0 | asyncos.rs | Environment baseline — torque-normalized reads of system state |
| 1 | asyncos.rs | AquaShell implant at known path, with known-good hash comparison |
| 2 | asyncos.rs | IOC hash scan across expanded directories, with file-size filtering |
| 3 | asyncos.rs | IOC IP correlation against confirmed C2 addresses |
| 4 | asyncos.rs | Manifest drift — per-path z-score deviation from known-good baselines |
| 5 | asyncos.rs | Process baseline — unexpected processes vs. known-good set |
| 5b | asyncos.rs | SSH persistence audit — authorized_keys enumeration and PermitRootLogin check (maps to ACSC Hunt Guide rule PERSISTENCE-T1098.004-RootSSHKeyAccepted-013) |
| 6 | geom.rs | Geometric drift scoring with quadrant classification |
| 7 | sonar.rs | Three-band echo detection at acoustic (ms), optical (us), and gravitational (s) timescales |
| 8 | tripwire.rs | Probabilistic tripwires with McFly lineage verification — detects timeline inconsistencies from system state rollback |
| 9 | eris.rs | Cross-domain narrative coherence — detects partial cleanup across filesystem, process, cache, log, and entropy domains |
| 10 | spinaltap.rs | Criticality inversion scoring, invariant lock violations, Shannon entropy density profiling |
| 11 | redshift.rs | Adversarial robustness validation — tests detection surface against 18 simulated evasion vectors |
| 12 | anchor.rs | Physics-anchored invariant proofs — 8 detection guarantees across thermodynamic, causal, and observational classes |

### How findings are structured

Every detection produces a `Finding` with:
- **Severity**: Critical / High / Medium / Low
- **Confidence**: 0.0–1.0 (how certain the detection is)
- **Role**: Core (primary signal) or Supplementary (corroborating)
- **Surprise score**: How far the observation deviates from expected state
- **Evidence**: The specific artifact or measurement that triggered the finding

Warnings (inaccessible paths, failed commands) are tracked separately from findings so that operational noise does not pollute the signal.

### ACSC Hunt Guide rule mapping

The tool's detection layers map to the structured rule IDs from the ASD/ACSC Cisco SD-WAN Threat Hunt Guide:

| Hunt Guide Rule ID | Detection Layer | Finding ID |
|---|---|---|
| PRIVESC-T1601.002-AppDowngrade-001 | L4 (manifest drift) | Firmware downgrade artifacts in z-score deviation |
| INITIALACCESS-T1190-RemoteColorNull-006 | L0–L3 (IOC surface) | Rogue peering, C2 IP correlation |
| PERSISTENCE-T1098.004-RootSSHKeyAccepted-013 | L5b (SSH audit) | PERSISTENCE-SSH-AUTHORIZED-KEYS, PERSISTENCE-SSHD-PERMIT-ROOT-LOGIN |

### Running the tool (for evaluation purposes)

> **Do not run this on production infrastructure without internal review, risk assessment, and authorization from your incident response leadership.** The instructions below are provided to make evaluation low-friction for teams that have decided to assess the tool — not as a recommendation to deploy.

**Rust (native, on-device or against mounted image):**

```bash
cargo build --release
cargo test
```

The crate compiles to a single binary with one dependency (anyhow for error handling). No network calls, no external services, no telemetry. The source is fully auditable.

**Python test harness (validation / CI):**

```bash
python3 test_harness.py
```

The Python harness mirrors every Rust detection layer in pure Python and runs 51 tests covering clean appliances, compromised appliances, edge cases, and adversarial robustness scenarios. It exists to validate that the Rust logic is correct — the two implementations should always agree. Reviewers can start here without compiling anything.

### Test results

```
TOTAL: 51  |  PASSED: 51  |  FAILED: 0
```

Coverage spans all 16 original assessment recommendations plus 10 additional detection modules (sonar, tripwire, eris, spinaltap, redshift, anchor, swallowtail, torque, SSH persistence, and full pipeline integration).

### Crate structure

```
2/src/
  lib.rs          43 lines    Crate root — module exports
  asyncos.rs    1664 lines    Detection layers 0–5b (implant, IOC, drift, process, SSH)
  torque.rs      783 lines    Stealth I/O normalization (equilibrium, drift envelopes, executor)
  geom.rs        365 lines    Three-axis geometric drift scoring (layer 6)
  sonar.rs       710 lines    Three-band echo detection (layer 7)
  tripwire.rs    599 lines    Probabilistic tripwires + McFly lineage (layer 8)
  eris.rs       1039 lines    Discordian derailment + paradox engineering (layer 9)
  spinaltap.rs   859 lines    Criticality inversion + invariant locks (layer 10)
  redshift.rs    891 lines    Adversarial evasion vector simulation (layer 11)
  anchor.rs      782 lines    Physics-anchored invariant proofs (layer 12)
  report.rs      322 lines    Finding/Warning/Report data structures
  io.rs          250 lines    Filesystem trait abstraction
  sys.rs         146 lines    Command execution trait abstraction
```

Total: 8,453 lines of Rust. One external dependency.

### What the adversarial validation found

The redshift module simulates 18 evasion vectors — timestamp stomping, cache flushing, IOC rotation, process hiding, log entropy manipulation, coordinated cleanup, and others. Of 18 vectors tested, 10 survived (detection held) and 8 broke through. The brittleness score is 0.44. This number is reported honestly because a tool that claims 100% detection against a CVSS 10.0 nation-state campaign is lying.

The 8 anchor invariant proofs identify what we can guarantee (thermodynamic constraints on cleanup, causal ordering violations from exploitation, observation paradoxes on shared hardware) and what we cannot (cache perturbation is the weakest anchor, and several gaps are provably uncoverable with software-only detection).

### IOC sidecar updates

New indicators of compromise can be loaded at runtime via a JSON sidecar file at `/etc/selfcheck/ioc_asyncos.json` without recompiling the crate. Format:

```json
{
  "hashes": [
    {"name": "NewImplant", "sha256": "abc123..."}
  ],
  "ips": ["10.0.0.1"]
}
```

---

## For everyone else — what this is about in plain language

> **This section explains the threat. It is not a recommendation to run this software.** The tool described in this repository has not been independently reviewed or endorsed by any government agency or security authority. It is published openly so that experts can examine it. If your organization is affected by the campaigns described below, follow the official guidance from CISA and your national cybersecurity agency — that guidance does not depend on this tool existing.

In February 2026, security agencies from the United States, Australia, Canada, and Singapore jointly disclosed that sophisticated attackers had been secretly living inside Cisco networking equipment used by organizations around the world. Two separate campaigns were found, targeting two different types of Cisco devices. Both had been running undetected for months to years.

**What the attackers did:**

They found ways to log into Cisco devices without needing a password (rated 10 out of 10 on the severity scale — the maximum). Once inside, they installed hidden backdoors, gave themselves permanent access, and carefully erased the evidence. On email security appliances, they planted a hidden program inside an existing web server file so it would survive reboots and look like part of the normal system. On SD-WAN controllers (the devices that manage how an organization's network traffic flows), they downgraded the software to an older vulnerable version, used that to get the highest level of access, then upgraded the software back — so the device looked like it was never touched.

**Why it matters:**

These are not personal computers or phones. These are infrastructure devices — the equipment that controls how your organization's network operates and how your email is secured. If an attacker controls your SD-WAN controller, they can redirect your network traffic. If they control your email gateway, they can read every email your organization sends and receives. The attackers targeted these devices precisely because they are trusted by design and rarely inspected.

**What this tool is trying to do:**

This tool attempts to check whether a Cisco device has been compromised by these specific campaigns. It looks for the known backdoors, checks for unauthorized access that was left behind, and — importantly — looks for the subtle traces that remain even after an attacker has tried to clean up. Cleanup is never perfect, and the tool is designed to detect the imperfections.

The tool is careful about how it checks. These attackers have shown the ability to watch for forensic activity on compromised devices. So the tool disguises its own activity as normal system operations — it reads files the way the system normally would, at normal-looking intervals, mixed in with ordinary activity. The attacker monitoring the device should not be able to distinguish the tool's forensic checks from routine system behavior.

This codebase is published for review, not for promotion. The author is requesting independent audit before recommending anyone run it. Whether it actually works as described is exactly what that review process is for.

**What to do if you run Cisco SD-WAN or Cisco Secure Email Gateway (official guidance — not dependent on this tool):**

1. Check whether your devices are running affected software versions (listed in the Cisco Security Advisories linked below).
2. Apply the patches Cisco has released.
3. Check for the indicators of compromise described in the "If you think your organization may be affected" section above.
4. If you find anything suspicious, contact your national cybersecurity agency. In the US, that is CISA. In Australia, ASD/ACSC. Forensic evidence preservation matters — do not wipe devices before they have been examined.

These steps come from the official advisories. They are the right thing to do regardless of whether this tool is ever validated.

---

## Reference documents

| Ref | Document |
|-----|----------|
| C1 | Cisco Security Advisory: CVE-2026-20127 — Cisco Catalyst SD-WAN Controller Authentication Bypass |
| C2 | Cisco Talos: Active exploitation of Cisco Catalyst SD-WAN by UAT-8616 |
| C3 | Cisco Security Advisory: CVE-2025-20393 — Cisco Secure Email Gateway and Secure Email and Web Manager |
| C4 | Cisco Talos: UAT-9686 actively targets Cisco Secure Email Gateway |
| C5 | Cisco Catalyst SD-WAN Hardening Guide |
| C6 | CISA Emergency Directive 26-03: Mitigate Vulnerabilities in Cisco SD-WAN Systems |
| C7 | ASD/ACSC + CISA Joint CSA: Exploitation of SD-WAN Appliances (PP-26-0656) |
| C8 | ASD/ACSC Cisco SD-WAN Threat Hunt Guide |
| C9 | NVD: CVE-2022-20775 (SD-WAN privilege escalation, CVSS 7.8) |
| C10 | NVD: CVE-2026-20127 (SD-WAN auth bypass, CVSS 10.0) |

---

## Review and audit

This codebase is published openly and the author is actively requesting independent security review. If you are a security researcher, incident responder, or forensic engineer and you want to audit this work, everything you need is in this repository. The Python test harness can be read and executed without compiling anything. The Rust source is fully self-contained with one dependency.

If you find issues — logic errors, false-positive risks, detection gaps, unsafe assumptions — please report them. That is the point of publishing before recommending use.

The author will not promote direct operational use of this codebase until it has been validated by independent review and, ideally, supported by investigators working the relevant campaigns.

---

## Author

Breyden Taylor / Prompted LLC — February–March 2026

---

## License

MIT
