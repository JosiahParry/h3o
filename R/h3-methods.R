unique.H3 <- function(x) {
  h3_strs <- h3_to_strings(x)
  x[match(unique(h3_strs), h3_strs)]
}

#' @export
format.H3 <- function(x, ...) formatC(h3_to_strings(x), ...)

#' @export
`[[.H3` <- function(x, i, ...) {
  if (length(i) > 1) stop("subscript out of bounds", call. = FALSE)
  structure(
    .subset(x, i),
    class = c("H3", "vctrs_vctr", "list")
  )
}


st_as_sfc.H3 <- function(x) {

}
