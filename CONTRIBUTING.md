## Get started

To start developing this project, get `nix` package manager and run
```
nix develop
```
In the root of the repo.

To try out the app, run
```
nix shell
```

If you don't want to install `nix` package manager, refer to `./flake.nix` for dependencies.

## Structure

- `./src/io.rs` is solely responsible for all IO operations. It is a trait and thus, can be mocked for tests
- `./src/meta.rs` is to work with `meta`. By `meta` we mean the low-level representation of our graphs of commits
- `./src/cli.rs` processes CLI input and invokes all other possible functionality
- `./src/hash.rs` provides API to work with hashes
- `./src/tree.rs` provides API to work with graphs. `Node` can only have one parent. Meta can be converted into `Node` which is a linear graph
- `./src/error.rs` is a super-error, that is, error covering all possible errors. We don't do exceptions here
- `./src/magick.rs` to work with imagemagick

## Workflow for `tri commit`

When `tri commit` is invoked, here are a few things happening

1. Read the meta file and get `meta`
1. Turn `meta` into a linear graph `Node`
1. Recursively reproduce commands traversing the graph up
    - If the command we're about to invoke has an associated hash, which is already present in `tri-cache` folder, use it and do not further up
    - Otherwise, invoke the command and cache it
1. Copy the output to `tri-out.{extension}` (extension is chosen based on the original extension)
