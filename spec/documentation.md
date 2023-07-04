# Documentation

## Design

TRI was heavily inspired by git and uses the same terminology for its CLI. The goal was to build a reproducible, tree changes based and immutable graphic editor for the command line using ImageMagick's functionality.

## Structure


## External Frameworks

TRI utilizes several dependencies to provide its functionality.

sha256: To calculate the hashes for the images, we use sha256.
csv: To write our tri-meta file with the history of the commits, we save them using csv.
clap: In order to call imagemagick's convert, we use clap.
colored: For coloring and formatting terminal output, we use colored.
notify: To implement tree-watch which updates whenenver tri-meta was changed, we use notify to listen to filesystem events and update the tree.
serde_yaml: To write the path to the image of our config file, we use serde/serde_yaml.