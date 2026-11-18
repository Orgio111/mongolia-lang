# Governance proposal

The project owner initially appoints maintainers for language, compiler, runtime/backends,
tooling, security and releases. Appointments, decision records and conflicts of interest should
be public before accepting external production commitments. This file does not invent a current
maintainer roster or legal organization.

RFCs govern syntax, types, ownership, standard APIs, package resolution, ABI and compiler contracts.
Use numbered documents with proposed/accepted/implemented/rejected status, discussion period,
decision rationale and dissent. Require one domain reviewer plus one independent maintainer for
semantic or unsafe changes; contributors do not approve their own changes. Security fixes may
use private review followed by a public record. Unresolved disputes go to the project owner with
written rationale and an appeal through a revised RFC.

Before 1.0, document breaking changes and migration steps in each release. At 1.0, state exactly
which language editions, APIs, platforms and ABI surfaces are stable. Deprecate with diagnostics,
replacement guidance and a documented removal window. Never equate an internal IR with stable ABI.
Release gates include passing claimed-platform CI, signed/checksummed artifacts, license/dependency
review, changelog, known limitations and current docs. Verify Mongolia/mn naming and registry
availability before public publication; no such availability has been established here.

The release reviewer checks provenance, checksums, SBOM, licenses, native platform evidence and signer fingerprint. A separate maintainer authorizes publication of the reviewed artifact digests. Workflow artifacts are candidates. A versioned tag or successful build alone does not confer release approval.
