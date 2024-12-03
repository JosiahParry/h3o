#' Convert sf geometry to H3 Cells
#'
#' Given a vector of sf geometries (class `sfc`) create a list of `H3` vectors.
#' Each list element contains the vector of H3 cells that cover the geometry.
#'
#' Note, use `flatten_h3()` to reduce the list to a single vector.
#'
#' @inheritParams h3_from_points
#' @param containment default `"intersect"`. Must be one of `"intersect"`,
#'  `"centroid"`, or `"boundary"`. See details.
#'
#' @details
#'
#' The [Containment Mode](https://docs.rs/h3o/0.4.0/h3o/geom/enum.ContainmentMode.html) determines if an H3 cell should be returned.
#'
#' - `"centroid"` returns every cell whose centroid are contained inside of a polygon. This is the fastest option but may not cover the entire polygon.
#' - `"boundary"` this returns the cells which are completely contained by the polygon. Much of a polygon might not be covered using this approach.
#' - `"intersect"` ensures that a polygon is entirely covered. If an H3 cell comes in contact with the polygon it will be returned. This is the default.
#'- `"contains"` behaves the same as `"intersect"`, but also handles the case where the geometry is being covered by a cell without intersecting with its boundaries. In such cases, the covering cell is returned.
#'
#' @examples
#' if (interactive() && rlang::is_installed("sf")) {
#'   nc <- sf::st_read(system.file("shape/nc.shp", package = "sf"), quiet = TRUE)
#'   geo <- sf::st_geometry(nc)
#'   cells <- sfc_to_cells(geo, 5)
#'
#'   head(cells)
#'
#'   plot(flatten_h3(cells))
#' }
#'
#' @export
#' @returns An H3 vector.
sfc_to_cells <- function(x, resolution, containment = "intersect") {
  match.arg(containment, c("intersect", "centroid", "boundary", "covers"))
  if (!inherits(x, c("sfc_POLYGON", "sfc_MULTIPOLYGON"))) {
    rlang::abort("`x` must be of class `sfc_POLYGON` or `sfc_MULTIPOLYGON`")
  } else if (!(resolution >= 0 && resolution <= 15)) {
    rlang::abort("`resolution` must be between 0 and 15 inclusive")
  } else if (rlang::is_installed("sf")) {
    # additional check for degrees if sf is installed
    units <- sf::st_crs(x)$units_gdal

    if (is.na(units)) {
      rlang::warn("`x` has missing units. Cannot confirm if degrees are used.")
    } else if (units != "degree") {
      rlang::abort("`x` must have a CRS using degrees such as EPSG:4326.")
    }
  }
  sfc_to_cells_(x, resolution, containment)
}

