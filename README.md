# GeoAlgBonus-Thoidsann

This program takes a set of points as inputs and calculates the subset of points
that are on the border of the convex hull. The user will be able to choose
between four algorithms, but for now we only use graham scan.

## Run the program

```shell
cargo run -- [algorithm] [point files]
```

for example

```shell
cargo run -- graham_scan tests/test2
```
