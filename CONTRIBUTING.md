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

## Design

TRI was heavily inspired by git and uses the same terminology for its CLI. The goal was to build a **t**ree changes based, **r**eproducible and **i**mmutable graphic editor for the command line using ImageMagick's functionality.

## Structure

- `./src/io.rs` is solely responsible for all IO operations. It is a trait and thus, can be mocked for tests. It is the interface for the materialization and also has the implementation. 
- `./src/meta.rs` is to work with `meta`. By `meta` we mean the low-level representation of our graphs of commits
- `./src/cli.rs` processes CLI input and invokes all other possible functionality
- `./src/hash.rs` provides API to work with hashes
- `./src/tree.rs` provides API to work with graphs. `Node` can only have one parent. Meta can be converted into `Node` which is a linear graph
- `./src/error.rs` is a super-error, that is, error covering all possible errors. We don't do exceptions here
- `./src/magick.rs` to work with imagemagick

## External Frameworks

TRI utilizes several dependencies to provide its functionality.

- sha256: To calculate the hashes for the images, we use sha256.
- csv: To write our tri-meta file with the history of the commits, we save them using csv.
- clap: In order to call imagemagick's convert, we use clap.
- colored: For coloring and formatting terminal output, we use colored.
- notify: To implement tree-watch which visualizes the tree and updates whenenver tri-meta was changed, we use notify to listen to filesystem events.
- serde_yaml: To write the path to the image of our config file, we use serde/serde_yaml.

## Workflow for `tri commit`

When `tri commit` is invoked, here are a few things happening

1. Read the meta file and get `meta`
1. Turn `meta` into a linear graph `Node`
1. Recursively reproduce commands traversing the graph up
    - If the command we're about to invoke has an associated hash, which is already present in `tri-cache` folder, use it and do not further up
    - Otherwise, invoke the command and cache it
1. Copy the output to `tri-out.{extension}` (extension is chosen based on the original extension)

Just as a side information:
Materialization is the process of making the commits reality. The graph is the precise and reproducible exact description of an image, having all the information and actions needed to get to the end result given the input image and the materialization is the process of getting there.

## Meta

Refers to file commits

Simple CSV:

```
commit,parent,command,node_status
54f85854ca6d77d50bcd5e338e78ce15,,,
54f85854ca6d77d50bcd5e338e78ce15,e330efab74317d4b98eb30b03df73fa6,crop 100x100,
54f85854ca6d77d50bcd5e338e78ce15,e330efab74317d4b98eb30b03df73fa6,monochrome,HEAD
```

For every line one of the following is true:
- It has different commit and parent hashes, non-empty command, HEAD or empty as status (normal commit)
- It only has commit but empty parent, no command and no node_status (-> Root)
- It only has commit but empty parent, no command but node_status is HEAD (-> Root is head, e.g. just initialized)