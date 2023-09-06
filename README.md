
<!-- README.md is generated from README.Rmd. Please edit that file -->

# h3o

<!-- badges: start -->

[![R-CMD-check](https://github.com/JosiahParry/h3o/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/JosiahParry/h3o/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

h3o is a system-dependency free package to interact with the H3
Geospatial Indexing system by Uber. h3o utilizes the Rust library h3o
with is a pure rust implementation of H3 and does not link or use Uber’s
H3 C library. h3o R interface is powered by
[extendr](https://extendr.github.io/) and should be able to compile on
any machine.

## Installation

You can install the development version of h3o from
[GitHub](https://github.com/) with:

``` r
# install.packages("remotes")
remotes::install_github("JosiahParry/h3o")
```

## Example

To illustrate the basic usage, we can first create an sf object of
random points.

``` r
pnts <- tibble::tibble(
  x = runif(100, -5, 10),
  y = runif(100, 40, 50)
) |> 
  sf::st_as_sf(
    coords = c("x", "y"), 
    crs = 4326
  )
```

h3o utilizes vctrs to create H3 class vectors so that they can work
seemlessly within a tidyverse workflow.

h3o is intended to work with the sf package for geometric operations. H3
vectors can be created from `POINT` geometry columns (`sfc` objects).

``` r
library(h3o)

pnts |> 
  dplyr::mutate(h3 = h3_from_points(geometry, 5))
#> Simple feature collection with 100 features and 1 field
#> Geometry type: POINT
#> Dimension:     XY
#> Bounding box:  xmin: -4.80551 ymin: 40.57655 xmax: 9.958988 ymax: 49.98309
#> Geodetic CRS:  WGS 84
#> # A tibble: 100 × 2
#>                 geometry              h3
#>  *           <POINT [°]>            <H3>
#>  1 (-0.8983678 46.42658) 85186a8bfffffff
#>  2   (1.884723 41.12234) 853946a7fffffff
#>  3  (-4.056968 44.74521) 85185daffffffff
#>  4   (8.173147 49.37186) 851f85b7fffffff
#>  5   (3.206761 41.95335) 8539451bfffffff
#>  6   (4.490571 43.27424) 85396eaffffffff
#>  7  (-2.247376 43.43344) 85184b83fffffff
#>  8  (-2.228073 46.82696) 85184577fffffff
#>  9    (8.85834 47.34941) 851f8ecffffffff
#> 10   (8.744089 47.93225) 851f8177fffffff
#> # ℹ 90 more rows
```

Additionally, H3 vectors also have an `st_as_sfc()` method which lets us
convert vectors of H3 cell indexes into `POLYGON`s.

``` r
h3_cells <- pnts |> 
  dplyr::mutate(
    h3 = h3_from_points(geometry, 4),
    # replace geometry
    geometry = sf::st_as_sfc(h3)
    )

# plot the hexagons
plot(sf::st_geometry(h3_cells))
```

<img src="man/figures/README-unnamed-chunk-3-1.png" width="100%" />

H3 cell centroids can be returned using `h3_to_points()`. If `sf` is
avilable the results will be returned as an `sfc` (sf column) object.
Otherwise it will return a list of `sfg` (sf geometries).

``` r
# fetch h3 column
h3s <- h3_cells$h3

# get there centers
h3_centers <- h3_to_points(h3s) 

# plot the hexagons with the centers
plot(sf::st_geometry(h3_cells))
plot(h3_centers, pch = 16, add = TRUE, col = "black")
```

<img src="man/figures/README-unnamed-chunk-4-1.png" width="100%" />

## sf compatibility

h3o was designed with sf in mind. H3 is a geospatial indexing system so
it is important to be able to go back and from from H3 and sf objects.
H3 object can be created from sfc objects and vice versa.sfc objects can
also be created using the `sf::sf_as_sfc()` method for `H3` or `H3Edge`
vectors.

`H3Edge` vectors represent the boundaries of H3 cells. They can be
created with `h3_edges()`, `h3_shared_edge_pairwise()`, and
`h3_shared_edge_sparse()`.

``` r
cell_edges <- h3_edges(h3s[1:3])
cell_edges
#> [[1]]
#> <H3Edge[6]>
#> [1] 114186a9ffffffff 124186a9ffffffff 134186a9ffffffff 144186a9ffffffff
#> [5] 154186a9ffffffff 164186a9ffffffff
#> 
#> [[2]]
#> <H3Edge[6]>
#> [1] 1143946bffffffff 1243946bffffffff 1343946bffffffff 1443946bffffffff
#> [5] 1543946bffffffff 1643946bffffffff
#> 
#> [[3]]
#> <H3Edge[6]>
#> [1] 114185dbffffffff 124185dbffffffff 134185dbffffffff 144185dbffffffff
#> [5] 154185dbffffffff 164185dbffffffff
```

We’ve created a list of each cell’s edges. We can flatten them using
`flatten_edges()`.

``` r
cell_edges <- flatten_edges(cell_edges)
cell_edges
#> <H3Edge[18]>
#>  [1] 114186a9ffffffff 124186a9ffffffff 134186a9ffffffff 144186a9ffffffff
#>  [5] 154186a9ffffffff 164186a9ffffffff 1143946bffffffff 1243946bffffffff
#>  [9] 1343946bffffffff 1443946bffffffff 1543946bffffffff 1643946bffffffff
#> [13] 114185dbffffffff 124185dbffffffff 134185dbffffffff 144185dbffffffff
#> [17] 154185dbffffffff 164185dbffffffff
```

These can be cast to sfc objects using its `st_as_sfc()` method.

``` r
sf::st_as_sfc(cell_edges)
#> Geometry set for 18 features 
#> Geometry type: LINESTRING
#> Dimension:     XY
#> Bounding box:  xmin: -4.256158 ymin: 40.84516 xmax: 2.07735 ymax: 46.73547
#> Geodetic CRS:  WGS 84
#> First 5 geometries:
#> LINESTRING (-0.7938494 46.27618, -0.4911588 46....
#> LINESTRING (-0.9919804 46.66586, -1.044524 46.4...
#> LINESTRING (-1.044524 46.43587, -0.7938494 46.2...
#> LINESTRING (-0.4369869 46.5752, -0.6871241 46.7...
#> LINESTRING (-0.4911588 46.3459, -0.4369869 46.5...
```

Additionally, you can get the vertexes of H3 cell indexes using
`h3_to_vertexes()` which returns an `sfc_MULTIPOINT`.

``` r
h3_to_vertexes(h3s)
#> Geometry set for 100 features 
#> Geometry type: MULTIPOINT
#> Dimension:     XY
#> Bounding box:  xmin: -5.149019 ymin: 40.39257 xmax: 10.15344 ymax: 50.21131
#> Geodetic CRS:  WGS 84
#> First 5 geometries:
#> MULTIPOINT ((-0.6871241 46.73547), (-0.9919804 ...
#> MULTIPOINT ((1.563896 41.22113), (1.508041 40.9...
#> MULTIPOINT ((-3.913195 45.05064), (-4.214897 44...
#> MULTIPOINT ((7.743345 49.38758), (7.659308 49.1...
#> MULTIPOINT ((3.125022 42.13971), (3.063423 41.9...
```

## Bench marks:

Since h3o is written in Rust, it is very fast.

Creating polygons

``` r
h3_strs <- as.character(h3s)
bench::mark(
  h3o = sf::st_as_sfc(h3s),
  h3jsr = h3jsr::cell_to_polygon(h3_strs)
)
#> # A tibble: 2 × 6
#>   expression      min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr> <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 h3o        292.17µs 312.24µs     3111.   26.84KB     36.9
#> 2 h3jsr        7.51ms   7.94ms      124.    3.02MB     27.5
```

Converting points to cells

``` r
bench::mark(
  h3o = h3_from_points(pnts$geometry, 3),
  h3jsr = h3jsr::point_to_cell(pnts$geometry, 3),
  check = FALSE
)
#> # A tibble: 2 × 6
#>   expression      min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr> <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 h3o        108.44µs 127.84µs     5916.      848B     11.4
#> 2 h3jsr        2.06ms   2.55ms      373.    1.03MB     12.9
```

Retrieve edges

``` r
bench::mark(
  h3o = h3_edges(h3s),
  h3jsr = h3jsr::get_udedges(h3_strs),
  check = FALSE
)
#> # A tibble: 2 × 6
#>   expression      min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr> <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 h3o         602.5µs 682.36µs     1054.      848B     9.82
#> 2 h3jsr         1.8ms   2.12ms      425.    71.4KB    16.2
```

Get origins and destinations from edges.

``` r
# get edges for a single location
eds <- h3_edges(h3s[1])[[1]]
# strings for h3jsr
eds_str <- as.character(eds)

bench::mark(
  h3o = h3_edge_cells(eds),
  h3jsr = h3jsr::get_udends(eds_str),
  check = FALSE
)
#> # A tibble: 2 × 6
#>   expression      min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr> <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 h3o          19.6µs   22.9µs    34920.    12.7KB     10.5
#> 2 h3jsr       471.4µs  543.2µs     1787.    34.8KB     22.6
```
