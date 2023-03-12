.onLoad <- function(...) {
  vctrs::s3_register("sf::st_as_sfc", "H3")
  vctrs::s3_register("sf::st_as_sfc", "H3Edge")
}
