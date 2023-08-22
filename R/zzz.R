.onLoad <- function(...) {
  vctrs::s3_register("sf::st_as_sfc", "H3")
  vctrs::s3_register("sf::st_as_sfc", "H3Edge")
  vctrs::s3_register("wk::wk_handle", "H3")
  vctrs::s3_register("wk::wk_handle", "H3Edge")
  vctrs::s3_register("wk::wk_crs", "H3")
  vctrs::s3_register("wk::wk_crs", "H3Edge")
}
