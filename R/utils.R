#' Flatten a list H3 vectors
#'
#' @param x
#'
#' @export
#'
#' h3_ids <- c("831f91fffffffff", "831fb5fffffffff", "831f94fffffffff")
#'
#' flatten_h3(
#'   list(
#'     NULL,
#'     h3_from_strings(h3_ids),
#'     h3_from_strings(h3_ids[1])
#'   )
#' )
flatten_h3 <- function(x) {
  x <- unlist(x)
  types <- vapply(x, typeof, character(1))

  if (!all(types %in% c("externalptr", "NULL"))) {
    stop("All list elements must be an H3 vector")
  }

  structure(
    x,
    class = vctrs_class()
  )
}
