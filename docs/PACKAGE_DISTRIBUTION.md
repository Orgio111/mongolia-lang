# Future distribution channels

| Channel | Proposed packaging | Gate before submission |
|---|---|---|
| Homebrew | A formula can build the locked workspace from a versioned source archive. Pin its digest, install the mn binary, and include a formula test executing a small .mn file. Start with a project-maintained tap. | macOS Intel and ARM native tests, source URL ownership, formula audit, license review. [Formula cookbook](https://docs.brew.sh/Formula-Cookbook) |
| Scoop | A bucket manifest can describe a Windows ZIP, architecture, SHA-256 and bin/mn.exe shim. | Stable HTTPS asset URL and a clean Windows install, upgrade and uninstall. [App manifests](https://github.com/ScoopInstaller/Scoop/wiki/App-Manifests) |
| WinGet | Prepare portable ZIP manifests for the supported architecture with a stable installer URL and InstallerSha256. | Reserve publisher/package identity, validate locally, verify the hash and pass service validation. [Microsoft validation](https://learn.microsoft.com/en-us/windows/package-manager/package/winget-validation) |
| Linux packages | Separate deb and RPM adapters should supply architecture, license notices, installed paths, dependencies and removals. Build and test on clean supported distributions. For apt distribution sign repository metadata. | Distribution maintainers, key rotation, upgrade tests, repository ownership and a rollback policy. [Debian package authentication](https://www.debian.org/doc/manuals/debian-handbook/sect.package-authentication.en.html) |

These entries are distribution proposals. No public package, namespace, tap, bucket or registry account has been created as part of Phase 10.
