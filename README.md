# Zed MSBuild

MSBuild support for [Zed](https://github.com/zed-industries/zed).

Covers `.csproj`, `.fsproj`, `.vbproj`, `.vcxproj`, `.sqlproj`, `.wixproj`, `.shproj`,
`.njsproj`, `.pyproj`, `.props`, `.targets`, `.tasks` and `.pubxml` files.

MSBuild project files are well-formed XML, so this extension reuses the
[tree-sitter-xml](https://github.com/tree-sitter-grammars/tree-sitter-xml) grammar
rather than shipping a custom parser.

## Language server

IntelliSense (completions, diagnostics, hover) is powered by
[msbuild-project-tools-server](https://github.com/tintoy/msbuild-project-tools-server). The
extension downloads it automatically on first use.

It requires the **.NET SDK (8.0+)** to be installed and available as `dotnet` on `PATH`.

## Developing locally

1. Open Zed.
2. `zed: install dev extension` (from the command palette) and select this directory.
3. Open a `.csproj`/`.props`/`.targets` file to see it in action.

## Contributing

This repository uses [Conventional Commits](https://www.conventionalcommits.org/).
Commit messages are validated by a Husky commit-msg hook running
[EasyBuild.CommitLinter](https://github.com/easybuild-org/EasyBuild.CommitLinter).

After cloning, enable the hook once (requires a .NET SDK on `PATH`):

```bash
dotnet tool restore
dotnet husky install
```

Accepted commit types: `feat`, `fix`, `ci`, `chore`, `docs`, `test`, `style`,
`refactor`, `perf`, `revert`, `build`.

## Releases

Releases are automated with
[EasyBuild.ShipIt](https://github.com/easybuild-org/EasyBuild.ShipIt) via the
`EasyBuild ShipIt` GitHub Actions workflow:

1. On every push to `main`, ShipIt opens (or updates) a `chore: release X.Y.Z`
   pull request that bumps `CHANGELOG.md` and the `version` in `extension.toml`
   based on the conventional commits since the last release.
2. Merging that PR tags the commit (`vX.Y.Z`) and publishes a GitHub release.
3. Submitting/updating the extension in the
   [Zed extension registry](https://github.com/zed-industries/extensions) remains
   a manual step that references the new tag.

Repository settings required for the automation:

- **Settings → Actions → General:** enable *Allow GitHub Actions to create and
  approve pull requests*.
- **Settings → General → Pull Requests:** prefer *Squash merging* set to use the
  *pull request title*, so the release history stays clean for ShipIt.

Releases run in CI on the .NET 10 SDK (ShipIt targets `net10.0`); day-to-day
contributing only needs a .NET SDK recent enough to run the commit-msg hook.

## Tree-Sitter

- https://github.com/tree-sitter-grammars/tree-sitter-xml

## Language Server

- https://github.com/tintoy/msbuild-project-tools-server
