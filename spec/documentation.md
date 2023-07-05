# Documentation

## Design

TRI was heavily inspired by git and uses the same terminology for its CLI. The goal was to build a **t**ree changes based, **r**eproducible and **i**mmutable graphic editor for the command line using ImageMagick's functionality.

## Structure

The structure of the project is split into a lot of files, each providing its own functionality according to its name. The core functionality of the tool is provided in the tree.rs, hash.rs and io.rs files. 
Tree implements the node of the graph and function to read the meta file into a graph. Hash calculates an image given a path or a string. 
IO implements
Hash implements the hashing of the images used for the commits.
IO implements the interface and implementation of the materialization, the process we named for putting the commits into action by using ImageMagick and calling it. 

## External Frameworks

TRI utilizes several dependencies to provide its functionality.

sha256: To calculate the hashes for the images, we use sha256.
csv: To write our tri-meta file with the history of the commits, we save them using csv.
clap: In order to call imagemagick's convert, we use clap.
colored: For coloring and formatting terminal output, we use colored.
notify: To implement tree-watch which visualizes the tree and updates whenenver tri-meta was changed, we use notify to listen to filesystem events.
serde_yaml: To write the path to the image of our config file, we use serde/serde_yaml.