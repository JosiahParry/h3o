h3_handler <- function(x, handler, ...) {
  rlang::check_installed("sf")
  wk::wk_handle(sf::st_as_sfc(x), handler, ...)
}

wk_handle.H3 <- function(x, handler, ...) h3_handler(x, handler, ...)
wk_handle.H3Edge <- function(x, handler, ...) h3_handler(x, handler, ...)

h3_crs <- function(x) {
  rlang::check_installed("sf")
  sf::st_crs(4326)
}

wk_crs.H3 <- function(x) h3_crs(x)
wk_crs.H3Edge <- function(x) h3_crs(x)
