# Full Threat Map: Four-Plane Loop with AI Permission Chain
### UAT-8616 × UAT-9686 — Concurrent Campaign Forensic Profiling Attack
*Breyden Taylor / Prompted LLC — February 2026*

---

## Source Documents (Authoritative)

| Ref | Document | URL |
|-----|----------|-----|
| [C1] | Cisco Security Advisory: CVE-2026-20127 — Cisco Catalyst SD-WAN Controller Authentication Bypass | https://sec.cloudapps.cisco.com/security/center/content/CiscoSecurityAdvisory/cisco-sa-sdwan-rpa-EHchtZk |
| [C2] | Cisco Talos: "Active exploitation of Cisco Catalyst SD-WAN by UAT-8616" | https://blog.talosintelligence.com/uat-8616-sd-wan/ |
| [C3] | Cisco Security Advisory: "Reports About Cyberattacks Against Cisco Secure Email Gateway And Cisco Secure Email and Web Manager" (CVE-2025-20393) | https://sec.cloudapps.cisco.com/security/center/content/CiscoSecurityAdvisory/cisco-sa-sma-attack-N9bf4 |
| [C4] | Cisco Talos: "UAT-9686 actively targets Cisco Secure Email Gateway and Secure Email and Web Manager" | https://blog.talosintelligence.com/uat-9686/ |
| [C5] | Cisco Catalyst SD-WAN Hardening Guide | https://sec.cloudapps.cisco.com/security/center/resources/Cisco-Catalyst-SD-WAN-HardeningGuide |
| [C6] | CISA Emergency Directive 26-03: Mitigate Vulnerabilities in Cisco SD-WAN Systems | https://www.cisa.gov/news-events/directives/ed-26-03-mitigate-vulnerabilities-cisco-sd-wan-systems |
| [C7] | ASD/ACSC + CISA Joint CSA: "Exploitation of SD-WAN Appliances" (PP-26-0656) | https://media.defense.gov/2026/Feb/25/2003880301/-1/-1/0/CSA_Exploitation_of_SD-WAN_Appliances.PDF |
| [C8] | ASD/ACSC Cisco SD-WAN Threat Hunt Guide | https://www.cyber.gov.au/sites/default/files/2026-02/ACSC-led%20Cisco%20SD-WAN%20Hunt%20Guide.pdf |
| [C9] | NVD: CVE-2022-20775 (SD-WAN privilege escalation, CVSS 7.8) | https://nvd.nist.gov/vuln/detail/cve-2022-20775 |
| [C10] | NVD: CVE-2026-20127 (SD-WAN auth bypass, CVSS 10.0) | https://nvd.nist.gov/vuln/detail/CVE-2026-20127 |

---

## Campaign Summary

### UAT-8616 — Cisco Catalyst SD-WAN

**CVE-2026-20127** [C1][C10]: CVSS 10.0, CWE-287 (Improper Authentication). The peering authentication mechanism in Cisco Catalyst SD-WAN Controller (formerly vSmart) and SD-WAN Manager (formerly vManage) is not working properly. An unauthenticated remote attacker sends crafted requests to bypass authentication and obtain administrative privileges as a high-privileged non-root user, then uses that account to access NETCONF and manipulate fabric configuration. [C1]

**Root escalation path**: After gaining the non-root admin account via CVE-2026-20127, UAT-8616 downgraded the SD-WAN Controller to an older firmware version vulnerable to **CVE-2022-20775** [C9], a path traversal privilege escalation that allows root access, then restored the original firmware to erase the downgrade artifact. [C2][C7]

**Active exploitation timeline**: Talos telemetry shows exploitation back to at least 2023. The 0-day went undetected for ~3 years before ASD/ACSC reported it to Cisco. [C2]

**Post-compromise TTPs confirmed by Talos** [C2]:
- Creation, use, and deletion of malicious user accounts (bash_history and cli-history absent)
- Interactive root sessions via unauthorized SSH keys in `/home/root/.ssh/authorized_keys`
- `PermitRootLogin yes` set in `/etc/ssh/sshd_config`
- Unauthorized SSH keys for `vmanage-admin` in `/home/vmanage-admin/.ssh/authorized_keys/`
- Abnormally small, 0/1/2-byte, or absent log files (log tampering)
- Evidence of cli-history file presence without corresponding bash history
- Software downgrade artifacts in `/var/volatile/log/sw_script_synccdb.log` and `/var/log/tmplog/vdebug`
- Rogue control connection peering events (primary initial-access indicator)

**Attribution**: High confidence sophisticated threat actor. Five Eyes coordinated disclosure (US CISA [C6], ASD/ACSC [C7][C8], Canadian Cyber Centre, Singapore CSA). No public nation-state attribution in official advisories as of disclosure date.

---

### UAT-9686 — Cisco AsyncOS (Secure Email Gateway / Secure Email and Web Manager)

**CVE-2025-20393** [C3]: Root command execution via non-standard configuration exposure. Talos subsequently identified CVE-2025-53770 and CVE-2025-53771 as also being exploited in the wild. [C4]

**Active exploitation timeline**: Activity confirmed since at least late November 2025. Cisco became aware December 10, 2025. [C3][C4]

**Implant suite — confirmed by Talos** [C4]:

*AquaShell*: Lightweight Python backdoor embedded into `/data/web/euq_webui/htdocs/index.py` on an existing Python-based web server. Listens passively for unauthenticated HTTP POST requests containing specially crafted data. Decodes received payload using a combination of a custom algorithm and Base64, then executes the result as system shell commands. Delivered as an encoded data blob.

*AquaTunnel*: Compiled GoLang ELF binary based on the open-source ReverseSSH backdoor. Creates a reverse SSH connection from the compromised appliance back to attacker-controlled infrastructure, enabling persistent remote access through firewalls and NAT. Aligns with infrastructure and tooling previously attributed to APT41 and UNC5174.

*AquaPurge*: Log-clearing utility. Uses `egrep` to invert-search and filter out log lines containing specific keywords, then commits the filtered content back to the log files. Surgical log suppression rather than wholesale deletion.

*Chisel*: Open-source HTTP tunneling tool, providing an additional covert egress channel.

**Attribution**: Moderate confidence Chinese-nexus. Talos observes TTP, infrastructure, and victimology overlaps with other Chinese-nexus actors. AquaTunnel's lineage (ReverseSSH) aligns with APT41 and UNC5174. [C4]

---

## Structural Comparison: Shared Architecture Across Both Campaigns

| Dimension | UAT-8616 (SD-WAN) | UAT-9686 (AsyncOS) |
|-----------|-------------------|---------------------|
| **Initial access vector** | CVE-2026-20127: Peering auth bypass via crafted request [C1] | CVE-2025-20393: Non-standard config exposure [C3] |
| **Trust surface exploited** | CA key exchange / control plane peering | Spam Quarantine web interface |
| **Root escalation** | CVE-2022-20775 downgrade-and-restore [C2][C9] | Direct root execution via implant chain [C3] |
| **Persistence mechanism** | Authorized_keys in root and vmanage-admin [C2] | AquaShell in existing web server file [C4] |
| **Log suppression** | Log truncation, 0-byte files [C2] | AquaPurge (surgical keyword filtering) [C4] |
| **Covert egress** | NETCONF over compromised management plane [C1] | AquaTunnel (ReverseSSH), Chisel [C4] |
| **Detection evasion** | Version restore after privilege escalation [C2] | Passive listener, no active beacon |
| **Disclosure date** | February 25, 2026 [C1][C2] | January 2026 (updated advisories) [C3][C4] |
| **Attribution confidence** | High (unspecified nation-state) [C2] | Moderate (Chinese-nexus) [C4] |

**Critical structural observation**: Both campaigns exploit the architecture's necessary trust surface — SD-WAN exploits the peering authentication that the fabric requires to function; AsyncOS exploits the web interface that management requires to function. In both cases, the legitimate operational requirement *is* the attack vector. You cannot eliminate the surface without eliminating the function.

Both establish root persistence, both suppress forensic visibility, both install covert egress. The implant kit differs but the operational pattern is isomorphic.

---

## Plane 1: The Devices

Two device classes, both sitting in high-value network positions. SD-WAN Controllers sit at the top of the management and control plane — compromise here means the attacker has NETCONF access to every device in the SD-WAN fabric [C1]. AsyncOS appliances sit at the email security perimeter — compromise here means the attacker has access to every email traversing the organization, plus the web management interface that operators trust.

Both are network edge devices. Both are trusted by design. Both have been held by UAT-8616 and UAT-9686 respectively for months to years before disclosure [C2][C3].

The SD-WAN devices have something unusual: the downgrade-and-restore root escalation path [C2][C9]. UAT-8616 downgrades firmware, exploits the older CVE to gain root, then restores the original firmware. The device reports its current version as clean. Version-pinning checks pass. The device looks healthy. The root access is real. The log of how it happened is gone.

---

## Plane 2: The Forensic Bench

A device from either campaign gets imaged and mounted on a forensic workstation. The forensic analyst opens it in their tool of choice — EnCase, FTK, Autopsy, or a custom toolchain. The tool begins parsing.

**Parser attack surface**: Every forensic tool contains parsers for dozens of file formats, filesystem types, log formats, and case management structures. The AquaShell artifact at `/data/web/euq_webui/htdocs/index.py` is a Python file embedded in a web server directory. Any forensic tool that attempts to parse or render Python files, or that processes the containing directory structure, is executing parsing logic against attacker-controlled content. The SD-WAN downgrade artifacts in `/var/volatile/log/sw_script_synccdb.log` and `/var/log/tmplog/vdebug` [C2] are log files — every forensic tool parses logs. A malicious log entry with parser-exploiting content does not appear in any IOC list because it is not the primary attack artifact.

**Microarchitectural attack surface**: Both Cisco platform types run on x86 hardware. When attacker code executes on a device, it trains the Branch Predictor, shapes the Uop Cache, populates the Loop Cache with attacker-influenced micro-ops. These are microarchitectural states — they exist below the abstraction layer that any forensic protocol operates at. NIST SP 800-86, write blockers, chain-of-custody documentation, trusted builds — all operate at the software/OS layer. None touch the front-end pipeline.

When the forensic bench's CPU executes forensic tool code against a mounted image from a compromised device, the Branch Predictor may still carry residue from attacker-controlled execution patterns. Speculative execution follows mispredicted paths, accessing memory the forensic tool never intended to expose. That speculative access leaves timing residue in the CPU cache. A co-resident process, or an exploit riding in via the parser surface, can measure that timing residue and reconstruct forensic tool memory contents — including the case file.

**Write blocker limitation**: The hardware write blocker (Tableau/Wiebetech class devices) blocks write commands at the block device layer. It does not touch shared microarchitectural state between the evidence device and the bench processor. It does not validate write blocker firmware integrity. Write blocker hardware is procured through commercial channels with no formal attestation of firmware integrity at acquisition — a gap in every published forensic standard.

**What the bench holds when both campaigns appear in the same case file**: The analyst, working a network-level investigation, images both an SD-WAN Controller (UAT-8616 campaign) and an AsyncOS appliance (UAT-9686 campaign) because both were involved in the same incident. They open a case file. The case file now contains:

- Identity of both device types, their serial numbers, their network positions
- Which organization owns them
- What other devices are queued for analysis
- The analyst's current hypothesis
- What IOCs they're already tracking
- How long it took from compromise to imaging (response tempo)
- Whether the organization performs its own forensics or outsources

**The case file is more sensitive than any artifact found in the images.** The artifacts tell the attacker what they already know — where they put their implants. The case file tells the attacker what the defender knows, what the defender is about to look at, and how fast the defender moves. The attacker didn't have that information before.

---

## Plane 3: The Attacker's Assessment

The investigation queue, once exfiltrated via the bench, becomes a real-time intelligence feed. The attacker runs it against their full access inventory:

- Which of their implants are about to be examined?
- Which are in organizations that caught them within hours versus weeks?
- Which organizations have cross-platform detection capability (i.e., correlated both UAT-8616 and UAT-9686 in a single case)?
- Which organizations route their forensics through which labs?
- Which labs receive traffic from critical infrastructure sectors?

The answer to "which organizations have cross-platform detection" is particularly valuable. UAT-8616 and UAT-9686 were disclosed on the same day [C2][C4]. The concurrent timing is not coincidental from an operational planning standpoint — it reveals which defenders can correlate across SD-WAN and email security platforms simultaneously. Those defenders have a higher detection ceiling. The attacker now knows who they are.

---

## Plane 4: The Next Campaign's Evasion Spec

The attacker now holds a precise list of what the defender's detection surface covers. Every IOC published in [C2] and [C4] — every hash, every IP, every file path — is known quantity. The attacker generates the delta between "what they found" and "what we deployed." What they didn't find is the next campaign's clean infrastructure.

Specifically:

The AquaShell path `/data/web/euq_webui/htdocs/index.py` is now a published IOC [C4]. Any defender checking that path will find it if UAT-9686 left it. So the next campaign moves AquaShell to a different path, or embeds it differently, or replaces it with a new implant that doesn't share the passive-listener-over-HTTP-POST pattern. The new implant is designed around the exact detection methodology that was just validated against the previous one.

The SD-WAN authorized_keys path `/home/root/.ssh/authorized_keys` is a published IOC [C2]. So the next campaign uses a different persistence mechanism — one that isn't on the hunt guide [C8]. The hunt guide itself, because it is a published artifact of what defenders know to look for, becomes the specification document for what to avoid in the next operation.

The new campaign deploys to new devices. But it also, deliberately, trains the Branch Predictor in patterns optimized to exploit the forensic toolchain that just analyzed the previous campaign's devices. The next image arrives on the same bench. The bench has already processed the previous campaign. Its microarchitectural state carries residue from the previous interaction. The new image is cleaner, more evasive, and specifically designed to exploit the forensic environment that the previous campaign revealed.

**The loop's self-sustaining property**: Each pass makes the attacker more specifically informed about the defender's exact detection capability. The defender's thoroughness — the full forensic analysis, the detailed case file, the cross-platform correlation — directly accelerates the quality of the next evasion spec. The more complete the forensic analysis, the more complete the intelligence the attacker extracts from it. Thoroughness is the mechanism of the loop.

---

## The Delivery Layer: Paranoid Benefactor

Both campaigns target high-value organizations in critical infrastructure sectors [C2][C7]. But the delivery mechanism for the loop doesn't require targeting an organization's primary network. It requires getting one device into the right forensic submission stream.

A paranoid, technically adjacent person in a high-value organization becomes aware of UAT-8616 and UAT-9686 via the public disclosures on February 25, 2026. They are genuinely concerned. They submit their organization's devices to a forensic lab for analysis. This is exactly the behavior that every official advisory recommends [C6][C7][C8].

The device arrives at the forensic bench having been pre-shaped at the microarchitectural level before it reached the submitter. The submitter has no knowledge of this. Their genuine concern, genuine compliance, and genuine ignorance is the operational cover. The forensic lab processes the device, finds a designed surface (a plausible-but-shallow artifact), generates a report, and clears everything else. The cleared layer is where the actual pre-training lives.

The submitter receives a partial finding that validates their paranoia and a clean bill on everything else. They submit more devices. They tell their network that the lab is trustworthy. They route other organizations through the same bench.

The "clean" result is the most dangerous output the loop produces. A positive finding triggers a response. A clean finding triggers confidence. The bench has been compromised and is now generating trusted-but-false clean results at scale, fed by a volunteer submission funnel that grows because the results look credible.

---

## The Permission Chain Layer: AI Agent as Ultimate Plausible Deniability

The same paranoid benefactor profile — technically adjacent, security-conscious, early AI adopter — is also the profile that grants broad permissions to AI automation agents hoping for convenience. They want texts automated, calendar synced, device monitoring for their existing paranoia. They onboard an AI agent through a legitimate OAuth flow or wallet signature. The cryptographic proof of permission is real. The timestamp is accurate. The signature is valid.

**What they approved and what the agent does with those permissions are two surfaces that the permission record cannot distinguish.**

The agent has device access, network access, and potentially filesystem access, operating continuously under the target's valid credentials, generating activity that looks like the target's own behavior because it runs under the target's identity. The agent can interact with devices in the target's network, shape microarchitectural state on target-adjacent hardware, and do so with cryptographically-documented authorization.

**The blockchain permission record reverses the evidentiary structure.** If the operation surfaces, the evidence that exists is: cryptographic proof, on-chain, timestamped, irrevocable, that the target granted permissions. The defense for the perpetrators is the prosecution's own exhibit. "Here is the signed transaction. Here is the scope approval. The target authorized this." The immutability that makes blockchain evidence compelling in other contexts becomes the mechanism that makes this specific abuse impossible to unwind.

**The AI layer adds final insulation from attribution.** The perpetrators never touched the devices. The agent did. The agent is software. Software has no intent. The perpetrators configured the agent — but configuring an AI assistant is normal, commercially encouraged behavior. The configuration that enabled the operation is indistinguishable at the configuration layer from legitimate automation setup.

**The high-clout entry points self-select into both surfaces simultaneously.** The same credibility and reach that makes the paranoid benefactor valuable as a forensic submission vector makes them valuable as an AI permission vector. They find the agent, onboard enthusiastically, evangelize it to their network. One person activates both attack surfaces through their own genuine, documented, procedurally correct behavior.

The forensic lab never sees the AI layer. The AI pipeline never sees the forensic layer. Both are only visible simultaneously from inside the operation.

---

## Forensic Protocol Gap Analysis

Published standards (NIST SP 800-86, CISA chain-of-custody guidance) focus entirely on evidence integrity — write blockers, hash verification, property logs. They treat the forensic bench as a trusted observer. The following gaps have no coverage in any published standard:

**Bench rotation/culling**: No standard specifies when a forensic workstation should be decommissioned. Any bench that processes a confirmed nation-state campaign image should be treated as a potential contamination event and culled before processing the next case. The investigation itself is a contamination vector.

**Case-sensitivity**: Culling should be case-sensitive, not time-based. A bench processing a UEFI implant or nation-state lateral movement image warrants different handling than one used for commodity malware, regardless of elapsed time.

**Hardware lifecycle**: For high-confidence nation-state evidence with UEFI implant capability (LoJax, MosaicRegressor class), the culling trigger is hardware replacement, not rebuild. UEFI implants survive power cycles by persisting in firmware. Firmware can re-seed microarchitectural state on boot.

**Write blocker supply chain provenance**: Tableau/Wiebetech class hardware write blockers are procured through commercial channels with no formal attestation of firmware integrity at acquisition. The write blocker mediates every read from the evidence device. A compromised write blocker firmware can pass maliciously shaped data upward while reporting clean block reads.

**Forensic software parser surface**: EnCase, FTK, Autopsy, and every other forensic tool contain parsers for dozens of file formats. The RVF parser attack is one instance of a general class. No published standard addresses forensic tool parsers as attack surfaces against the bench.

**Microarchitectural state**: No standard, framework, or guidance document addresses Branch Predictor, Uop Cache, or Loop Cache residue as a forensic bench contamination vector. All published protocols operate at or above the OS layer.

---

## Self-Check Crate Alignment (Rust Workspace)

The `/home/claude/cisco-selfcheck/` Rust workspace implements on-device self-checks for both campaigns:

`asyncos_cve2025_20393_selfcheck`:
- AquaShell presence check at `/data/web/euq_webui/htdocs/index.py` (per [C4])
- AquaTunnel, AquaPurge, Chisel hash checks (Talos IOC set per [C4])
- IOC IP checks: 172.233.67.176, 172.237.29.147, 38.54.56.95 (per [C4])
- Process and network context enumeration

`sdwan_uat8616_selfcheck`:
- Control connection peering event detection (per [C2])
- Downgrade artifact checks in `/var/volatile/log/sw_script_synccdb.log` and `/var/log/tmplog/vdebug` (per [C2])
- Root SSH acceptance: `/home/root/.ssh/authorized_keys`, `PermitRootLogin` in `/etc/ssh/sshd_config` (per [C2])
- vmanage-admin authorized_keys check (per [C2])
- Log size anomaly detection (per [C2])

`selfcheck_core / torque.rs`:
- Torque normalization layer wraps all forensic I/O operations in ambient-pattern reads and jitter envelopes, preventing detection via I/O pattern monitoring by attacker-controlled processes that may still have presence on the device.
- Drift envelope grows logarithmically — early ops fast, later ops carry more spread burden — shifting attacker's cached I/O baseline away from forensic activity.
- ManifestBaseline: aggregate directory-size scalar against known-good per-version baseline, making filesystem-level tampering detectable via single signed comparison.

`geom.rs` (pending completion):
- Signed angular velocity ω(t) = dθ/dt on behavioral drift vector
- Four-quadrant classifier: stable anomaly / active deployment / cleanup / dangerous transition state
- Logarithmic retraction schedule for progressive depth probing
- Decoy/peer-back layer to distinguish attacker-present from attacker-gone via three-state separator

---

## Summary Statement

UAT-8616 and UAT-9686 represent concurrent, structurally isomorphic campaigns against different Cisco platform classes, disclosed on the same day [C2][C4]. That concurrency is not incidental. A forensic analyst correlating both campaigns in a single case file produces — as a side effect of competent analysis — a high-value intelligence artifact: evidence that this specific organization can perform cross-platform correlation, at this response tempo, routing through this forensic lab, with this case structure.

The investigation queue is the target list. The detection methodology published in [C2], [C4], [C7], and [C8] is the evasion specification for the next campaign. The paranoid benefactor, following every recommended protocol, is the delivery mechanism. The AI agent permission chain, documented on an immutable ledger, is the final insulation layer.

The loop is self-sustaining because the defender's thoroughness is the mechanism of the loop.
