
#' Create H3 Index
#'
#' Create H3 indices from `sfc` objects, vectors of x and y coordinates, or H3 string IDs.
#'
#' @param x for `h3_from_points()` an object of class `sfc_POINT`. For `h3_from_strings()` a character vector of H3 index IDs. For `h3_from_xy()` a numeric vector of longitudes.
#' @param y a numeric vector of latitudes.
#' @param resolution an integer indicating the H3 cell resolution. Must be between 0 and 15 inclusive.
#' @export
#' @rdname H3
#' @examples
#' h3_from_xy(-90, 120, 5)
#'
#' h3_from_strings("85f29383fffffff")
#'
#' if (requireNamespace("sf")) {
#'   # create random points
#'   pnts <- sf::st_cast(
#'     sf::st_sfc(
#'       sf::st_multipoint(matrix(runif(10, max = 90), ncol = 2)),
#'       crs = 4326
#'     ),
#'     "POINT"
#'   )
#'
#'   # convert to H3 objects
#'   h3_from_points(pnts, 5)
#' }
h3_from_xy <- function(x, y, resolution) {
  if (resolution < 0 || resolution > 15)
    stop("`resolution` must be an integer in range [0, 15]")

  h3_from_xy_(x, y, resolution)
}

#' @export
#' @rdname H3
h3_from_points <- function(x, resolution) {

  if (resolution < 0 || resolution > 15)
    stop("`resolution` must be an integer in range [0, 15]")

  if (!inherits(x, "sfc_POINT"))
    stop("`x` must be an object of class `sfc_POINT`")

  x_crs <- attr(x, "crs")
  x_bbox <- sf::st_bbox(x)

  if (is.na(x_crs)) {
    if (any(abs(x_bbox[c("xmin", "xmax")]) > 180) || any(abs(x_bbox[c("ymin", "ymax")]) > 90)) {
      stop("Input must be in EPSG:4326")
    }  else {
      warning("CRS is missing assuming EPSG:4326")
    }
  }

  h3_from_points_(x, resolution)

}

#' @export
#' @rdname H3
h3_from_strings <- function(x) h3_from_string_(x)
