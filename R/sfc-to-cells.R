#' Convert sf geometry to H3 Cells
#'
#' Given a vector of sf geometries (class `sfc`) create a list of `H3` vectors.
#' Each list element contains the vector of H3 cells that cover the geometry.
#'
#' Note, use `flatten_h3()` to reduce the list to a single vector.
#'
#' @inheritParams h3_from_points
#'
#' @examples
#' if (interactive() && rlang::is_installed("sf")) {
#'   nc <- sf::st_read(system.file("shape/nc.shp", package = "sf"), quiet = TRUE)
#'   geo <- sf::st_geometry(nc)
#'   cells <- to_cells(geo, 5)
#'
#'   head(cells)
#'
#'   plot(flatten_h3(cells))
#' }
#'
#' @export
to_cells <- function(x, resolution) {
  stopifnot(
    inherits(x, "sfc"),
    resolution >= 0 && resolution <= 15
  )

  # additional check for degeres if sf is installed
  if (rlang::is_installed("sf")) {
    units <- sf::st_crs(x)$units_gdal

    if (is.na(units)) {
      rlang::warn("`x` has missing units. Cannot confirm if degrees are used.")
    } else if (units != "degree") {
      rlang::abort("`x` must have a CRS using degrees such as EPSG:4326.")
    }
  }

  sfc_to_cells(x, resolution)
}

