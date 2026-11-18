# Security policy

The interpreter is not a sandbox. Run untrusted source in a separately constrained process with
OS memory/time/filesystem/network controls. The foundational CLI reads source and `init` writes
project files; future package installation and FFI expand the trust boundary significantly.

Threats include malformed UTF-8, spoofed identifiers, parser nesting, recursive calls, excessive
allocation/output, integer overflow, malicious dependencies, registry tampering, unsafe pointers,
and asynchronous device lifetime faults. Validate source before execution, check allocation
arithmetic, bound recursion and source size, and test failures as diagnostics. Safe Rust does not
make allocation exhaustion or algorithmic denial-of-service impossible.

XID and NFC are necessary but insufficient for confusable protection. The implemented mixed Latin/Cyrillic identifier rejection and limited hand-curated
confusable skeleton checks do not cover all Unicode scripts or confusable characters. Full
confusable diagnostics remain a release-review item. Do not claim UTS39 compliance without a
specified Unicode version, implementation and adversarial corpus. English/Mongolian source mixing
is allowed; security policy must distinguish mixing keywords from mixing scripts inside one name.

Before public release designate a private reporting address and security maintainers. Until then,
do not publish exploit details in a public issue; contact the repository owner through a private
channel they have actually provided. No invented reporting address is supplied. Triage privately,
reproduce, classify affected versions, coordinate patches/advisories and credit reporters with
consent. Audit dependencies and release artifacts; record checksums, build inputs and provenance.

For a public GitHub repository, the owner must enable private vulnerability reporting, verify receipt using a test message and document the monitored fallback before publication. Record acknowledgements and affected versions privately. Announce fixed versions and remediation after coordinated review. The release signing fingerprint must be distributed through a channel independent of the download.
