# Security Review Report: nuget-scanner

**Date:** 2026-04-03
**Reviewer:** Claude Code (automated)
**Branch:** main

---

## Summary

No high-confidence security vulnerabilities were identified meeting the reporting threshold (confidence ≥ 8/10).

Two potential issues were investigated and filtered out after analysis.

---

## Investigated Findings

### Finding 1: SSRF via `catalog_entry` URL — BELOW THRESHOLD

**File:** [src/fetchers.rs](src/fetchers.rs) (lines 48–54)
**Category:** Server-Side Request Forgery (SSRF)
**Confidence:** 7/10 — does not meet reporting threshold

**Description:**
The `catalog_entry` URL returned by the NuGet registry API is used directly in a subsequent HTTP GET request without host validation. In theory, an attacker who publishes a malicious package to the NuGet registry could craft the `catalog_entry` field to point to an arbitrary host, causing the tool to make an HTTP request to an attacker-controlled or internal server.

**Why filtered:**
- Requires the attacker to publish a package to the official NuGet registry (non-trivial; needs a legitimate account)
- The user must explicitly choose to scan that specific package
- This is a local CLI tool, not a network-exposed service, significantly reducing SSRF impact
- Modern environments typically have network segmentation that limits SSRF reach

---

### Finding 2: URL injection via `package_id`/`version` CLI parameters — FALSE POSITIVE

**File:** [src/fetchers.rs](src/fetchers.rs) (lines 39–41)
**Category:** Path traversal / URL injection
**Confidence:** 2/10 — false positive

**Description:**
User-supplied CLI arguments are interpolated directly into a URL format string:

```rust
let url = format!(
    "https://api.nuget.org/v3/registration5-gz-semver2/{package_id}/{version}.json"
);
```

**Why filtered:**
- The host is hardcoded as `api.nuget.org`; only the path segment can be influenced. Path-only SSRF is excluded by policy.
- CLI arguments are trusted values by definition — a user supplying malicious input to their own local tool is not a valid threat model.
- No host redirection or code execution is possible.

---

## Conclusion

The nuget-scanner codebase contains **no reportable security vulnerabilities** at the current confidence threshold. The SSRF pattern in `fetch_catalog_dependency_group` is worth keeping in mind if the tool is ever adapted to run as a server-side or automated pipeline component, in which case host validation of the `catalog_entry` URL should be added.
