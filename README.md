# Zed MSBuild LSP

MSBuild language server for [Zed](https://github.com/zed-industries/zed).

IntelliSense (completions, diagnostics, hover) is powered by
[msbuild-project-tools-server](https://github.com/tintoy/msbuild-project-tools-server). The
extension downloads it automatically on first use.

It requires the **.NET SDK (8.0+)** to be installed and available as `dotnet` on `PATH`.

## Languages

The language server attaches to languages defined by other extensions:

| Language          | Files                         | Extension                                          |
| ----------------- | ----------------------------- | -------------------------------------------------- |
| `C# Project File` | `.csproj`                     | [C#](https://github.com/zed-extensions/csharp)     |
| `MSBuild File`    | `.proj`, `.props`, `.targets` | [C#](https://github.com/zed-extensions/csharp)     |
| `F# Project File` | `.fsproj`                     | [F#](https://github.com/nathanjcollins/zed-fsharp) |

To use it with other MSBuild files, map their extensions to `MSBuild File` in your Zed settings:

```json
{
  "file_types": {
    "MSBuild File": ["vbproj", "vcxproj", "sqlproj", "wixproj", "shproj", "njsproj", "pyproj", "tasks", "pubxml"]
  }
}
```

## Developing locally

1. Open Zed.
2. `zed: install dev extension` (from the command palette) and select this directory.
3. Install the C# and F# extensions.
4. Open a `.csproj`/`.props`/`.targets` file to see it in action.

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

## Language Server

- https://github.com/tintoy/msbuild-project-tools-server
