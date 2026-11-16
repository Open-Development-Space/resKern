# resKern Security Policy

## Supported Versions

Unless otherwise stated in a repository's documentation, the following support policy applies:

| Version | Security Updates |
|----------|----------|
| Latest Stable Release | Supported |
| Previous Stable Release | Best Effort |
| Development Builds (`dev`) | Not Guaranteed |
| Deprecated / End-of-Life Releases | Unsupported |

**Repository-specific support policies take precedence when documented.**

---

# Reporting a Vulnerability

If you discover a security vulnerability in resKern, please report it responsibly and privately. To do this we have enabled private vulnerability reporting.

## Do Not

Please do **not**:

- Open a public issue.
- Submit a public pull request revealing the vulnerability.
- Discuss the vulnerability in public discussions, forums, chats, or social media.
- Publicly disclose exploit details before remediation has been completed.

Public disclosure before a fix is available may place users at risk.

---

## Preferred Reporting Method

Please submit vulnerability reports through:

- GitHub Security Advisories
- Private security reporting channels specified by the repository

If neither option is available, contact the project maintainers directly. Preferably utilize the built in github tools.

---

## Information to Include

To help us assess and resolve the issue quickly, please include:

- A description of the vulnerability
- Affected repository and version(s)
- Potential impact
- Reproduction steps
- Proof-of-concept code (if available)
- Screenshots or logs (if applicable)
- Suggested remediation (optional)

The more information provided, the faster the issue can be validated.

---

# Response Process

resKern, in alignment with ODS, strives to acknowledge all legitimate security reports.

Our general process is:

1. Receive and triage the report.
2. Verify the reported vulnerability.
3. Assess severity and impact.
4. Develop and test a remediation.
5. Publish fixes and security advisories as appropriate.
6. Coordinate disclosure when necessary.

Response times may vary depending on:

- Severity
- Complexity
- Maintainer availability
- Affected project scope

No specific resolution timeline is guaranteed.

---

# Responsible Disclosure

We ask security researchers and community members to:

- Act in good faith.
- Avoid privacy violations.
- Avoid service disruption.
- Avoid data destruction or modification.
- Avoid actions that may negatively impact users.

Provide maintainers a reasonable opportunity to investigate and remediate issues before public disclosure.

---

# Safe Harbor

resKern supports good-faith security research, in alignment with ODS.

We will not pursue action against researchers who:

- Follow this policy.
- Avoid intentionally harming users or systems.
- Respect privacy and data ownership.
- Report vulnerabilities responsibly.

Activities that exceed these guidelines are not authorized.

---

# Scope

Unless otherwise stated, this policy applies to:

- Official project releases.
- Official source code maintained by project maintainers.

This policy does not automatically apply to:

- Third-party forks.
- Community-maintained mirrors.
- Unofficial builds.
- External dependencies.

Security concerns involving third-party software should be reported to their respective maintainers.

---

# Development Branches and Security

Repositories that use development branches may contain:

- Experimental features
- Incomplete implementations
- Temporary debugging functionality

Development branches should not be assumed to have the same security guarantees as stable releases.

Users deploying software in production environments should use supported stable releases whenever possible.

---

# Security Updates

Security fixes may be released independently from feature releases when necessary.

Critical security updates may be prioritized and published with limited notice to reduce potential user exposure.

Where practical, security advisories and release notes will provide:

- Impacted versions
- Fixed versions
- Mitigation guidance
- Upgrade recommendations

---

# Dependency Security

Contributors are encouraged to:

- Keep dependencies updated.
- Remove unused packages.
- Monitor dependency advisories.
- Review new dependencies before introduction.
- Prefer actively maintained upstream projects.

Maintainers may reject pull requests that introduce unnecessary security risk.

---

# Contributor Expectations

Contributors should:

- Never intentionally introduce malicious code.
- Protect credentials, secrets, and signing keys.
- Avoid committing sensitive information.
- Follow repository contribution guidelines.
- Report discovered security concerns promptly.

Any intentionally malicious contribution may result in removal of access, rejection of contributions, or further action deemed necessary by project maintainers.

---

# Security-Related Pull Requests

Security fixes may occasionally require private coordination before public release.

Maintainers may:

- Request temporary confidentiality.
- Delay public discussion.
- Coordinate releases across multiple repositories.

This process exists to protect project users while remediation is underway.

---

# Contact

For questions regarding this policy, contact the maintainers of the affected repository.

Where repository-specific security documentation exists, that documentation supersedes this policy.

---

Thank you for helping keep resKern and its users secure.
