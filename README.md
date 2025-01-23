# Yanidsmas

This program takes a set of points as inputs and calculates the subset of points
that are on the border of the convex hull. The user will be able to choose
between four algorithms, but for now we only use graham scan. The program
outputs a png image, that shows a plot of the points and marks the points
that are part of the convex hull.

## Run the program
First make sure you make the directory where the images will be stored.

```shell
mkdir -p plotters-doc-data
```

Then you can run the program

```shell
cargo run -- [algorithm] [point file]
```

This will use [algorithm] calculate the convex hull of [point file] and stores
the image in the plotters-doc-data directory.
For example:

```shell
cargo run -- graham_scan tests/test2
```
