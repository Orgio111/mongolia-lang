# Version policy

The workspace sequence is 0.1.0 (foundation and audit), 0.1.1 (hardening), then 0.1.2 (distribution changes). Both workspace crates inherit the root version. Each lockfile records the local packages in its own dependency graph. The CLI compiles CARGO_PKG_VERSION into --help and --version. The example manifest produced by init has its own project version.

Use a patch for compatible implementation, diagnostic, tooling or packaging fixes. Document compatibility-impacting security corrections explicitly. Use a minor for a new language surface or any intentional incompatible public change during the 0.x series. After 1.0 use a major for breaking changes and a minor for compatible additions. A prepared build becomes public only after its artifact digests and release gates receive approval. Never replace published bytes under the same version.

Language specification 0.1 revision 1 freezes the Phase 9 observable rules. An editorial correction increments the document revision and lists the clarified rule. A semantic change requires an RFC, explicit compatibility policy, specification version decision and executable conformance tests. Architecture v3.0 is the design edition. Historical guides, manifests for example projects and previous result logs retain their recorded versions.
