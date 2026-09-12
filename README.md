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

## Tree-Sitter

- https://github.com/tree-sitter-grammars/tree-sitter-xml

## Language Server

- https://github.com/tintoy/msbuild-project-tools-server
