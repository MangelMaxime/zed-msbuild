# Zed MSBuild

MSBuild support for [Zed](https://github.com/zed-industries/zed).

Covers `.csproj`, `.fsproj`, `.vbproj`, `.vcxproj`, `.sqlproj`, `.wixproj`, `.shproj`,
`.njsproj`, `.pyproj`, `.props`, `.targets`, `.tasks` and `.pubxml` files.

MSBuild project files are well-formed XML, so this extension reuses the
[tree-sitter-xml](https://github.com/tree-sitter-grammars/tree-sitter-xml) grammar
rather than shipping a custom parser.

## Developing locally

1. Open Zed.
2. `zed: install dev extension` (from the command palette) and select this directory.
3. Open a `.csproj`/`.props`/`.targets` file to see it in action.

## Tree-Sitter

- https://github.com/tree-sitter-grammars/tree-sitter-xml
