# Federal Register API Integration

**Source:** https://www.federalregister.gov/reader-aids/developer-resources

## API Overview

The Federal Register API provides unauthenticated, CORS-enabled access to all federal regulatory documents published since 1994.

- **Base URL:** `https://www.federalregister.gov/api/v1/`
- **Authentication:** None required
- **Format:** JSON (JSONP via `callback=` parameter)
- **Rate limits:** Reasonable use (no published hard limits)
- **Source code:** https://github.com/usnationalarchives/federalregister-api-core

## Endpoints

### Single Document
```
GET /api/v1/documents/{document_number}.json
```

### Multi-Get
```
GET /api/v1/documents/{doc1},{doc2},{doc3}.json
```

### Search
```
GET /api/v1/documents.json?conditions[term]=artificial+intelligence&conditions[agencies][]=defense-department
```

## Relevant Search Patterns for AI Governance

### AI Safety and Autonomous Systems
```bash
# Executive orders mentioning AI
curl "https://www.federalregister.gov/api/v1/documents.json?conditions[term]=artificial+intelligence&conditions[type][]=PRESDOCU&per_page=20&order=newest"

# NIST AI framework documents
curl "https://www.federalregister.gov/api/v1/documents.json?conditions[term]=AI+safety&conditions[agencies][]=national-institute-of-standards-and-technology&per_page=20"

# Defense Department AI policy
curl "https://www.federalregister.gov/api/v1/documents.json?conditions[term]=autonomous+systems+artificial+intelligence&conditions[agencies][]=defense-department&per_page=20"

# CISA critical infrastructure + AI
curl "https://www.federalregister.gov/api/v1/documents.json?conditions[term]=artificial+intelligence+critical+infrastructure&conditions[agencies][]=cybersecurity-and-infrastructure-security-agency&per_page=20"
```

### Financial Regulation + AI
```bash
# SEC algorithmic trading
curl "https://www.federalregister.gov/api/v1/documents.json?conditions[term]=algorithmic+trading+artificial+intelligence&conditions[agencies][]=securities-and-exchange-commission&per_page=20"

# CFTC automated trading
curl "https://www.federalregister.gov/api/v1/documents.json?conditions[term]=automated+trading&conditions[agencies][]=commodity-futures-trading-commission&per_page=20"
```

### Nuclear and Strategic Stability
```bash
# NNSA / DOE nuclear policy
curl "https://www.federalregister.gov/api/v1/documents.json?conditions[term]=nuclear+security+autonomous&conditions[agencies][]=energy-department&per_page=20"

# Arms control
curl "https://www.federalregister.gov/api/v1/documents.json?conditions[term]=arms+control+artificial+intelligence&per_page=20"
```

## Key Agency Slugs

| Agency | API Slug |
|--------|----------|
| Department of Defense | `defense-department` |
| NIST | `national-institute-of-standards-and-technology` |
| Department of Commerce | `commerce-department` |
| SEC | `securities-and-exchange-commission` |
| CFTC | `commodity-futures-trading-commission` |
| Department of Energy | `energy-department` |
| CISA | `cybersecurity-and-infrastructure-security-agency` |
| FTC | `federal-trade-commission` |
| Department of State | `state-department` |
| Office of Science and Technology Policy | `science-and-technology-policy-office` |

## Document Types

| Type | API Code | Description |
|------|----------|-------------|
| Presidential Documents | `PRESDOCU` | Executive orders, memoranda, proclamations |
| Rules | `RULE` | Final rules with regulatory effect |
| Proposed Rules | `PRORULE` | Notice of proposed rulemaking (NPRM) |
| Notices | `NOTICE` | General notices, meetings, guidance |

## eCFR (Electronic Code of Federal Regulations)

Companion API for the codified regulations:
- **Base URL:** `https://www.ecfr.gov/api/`
- **Docs:** https://www.ecfr.gov/reader-aids/ecfr-developer-resources/rest-api-interactive-documentation

## Integration Notes

- Monitor for new executive orders on AI via `PRESDOCU` type filter
- Track NIST AISI publications via `notices` from NIST
- Watch for NPRM (proposed rules) from SEC/CFTC on algorithmic trading
- All results paginated; follow `next_page_url` in response
- `publication_date` field enables date-range filtering for monitoring
