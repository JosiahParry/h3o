#' Compact H3 Cells
#'
#' Reduce a set of H3 indices of the same resolution to the minimum number of H3 indices of
#' varrying resolution that entirely covers the input area.
#'
#' @param x a vector of H3 indexes
#' @export
compact_cells <- function(x) {
  compact_cells_(unique(x))
}
