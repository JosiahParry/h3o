# DONE fn is_nb_pairwise_;
# DONE fn is_nb_sparse_;
# DONE fn h3_edges_pairwise_;
# DONE fn h3_edges_sparse_;
# DONE fn is_valid_edge_;
# fn get_directed_origin_;
# fn get_directed_destination_;
# fn get_directed_cells_;
# DONE fn h3_edges_;
# DONE fn edge_boundary_;

#' H3 index neighbors
#'
#' @export
#' @rdname is_nb
is_nb_pairwise <- function(x, y) {
  stopifnot(is_h3(x), is_h3(y))
  is_nb_pairwise_(x, y)
}

#' @export
#' @rdname is_nb
is_nb_sparse <- function(x, y) {
  is_nb_sparse_(x, y)
}


#' H3 Edges
#'
#' Functions to create or work with `H3Edge` vectors. See `Details` for further details.
#'
#' @param x an H3 vector
#' @param y an H3 vector
#'
#' @details
#'
#' - `h3_edges()`: returns a list of `H3Edge` vectors for each H3 index.
#' When `flat = TRUE`, returns a single `H3Edge` vector.
#' - `h3_shared_edge_pairwise()`: returns an `H3Edge` vector of shared edges. If
#' there is no shared edge `NA` is returned.
#' - `h3_shared_edge_sparse()`: returns a list of `H3Edge` vectors. Each element
#' iterates through each element of `y` checking for a shared edge.
#' - `is_edge()`: returns `TRUE` if the element inherits the `H3Edge` class.
#' - `is_valid_edge()`: checks each element of a character vector to determine if it is
#' a valid edge ID.
#' - `h3_edges_from_strings()`: create an `H3Edge` vector from a character vector.
#' - `flatten_edges()`: flattens a list of `H3Edge` vectors into a single `H3Edge` vector.
#' - `h3_edge_cells()`: returns a list of length 2 named `H3Edge` vectors of `origin` and `destination` cells
#' - `h3_edge_origin()`: returns a vector of `H3Edge` origin cells
#' - `h3_edge_destination()`: returns a vector of `H3Edge` destination cells
#' @rdname edges
#' @export
h3_edges <- function(x, flat = FALSE) {
  stopifnot(is_h3(x))
  res <- h3_edges_(x)

  if (flat) {
    res <- structure(
      unlist(res),
      class = edge_vctrs()
    )
  }

  res
}

#' @export
#' @rdname edges
h3_shared_edge_sparse <- function(x, y) {
  stopifnot(is_h3(x), is_h3(y))
  h3_edges_sparse_(x, y)
}

#' @export
#' @rdname edges
h3_shared_edge_pairwise <- function(x, y) {
  stopifnot(is_h3(x), is_h3(y))
  h3_edges_pairwise_(x, y)
}

#' @export
`[[.H3Edge` <- function(x, i, ...) {
  if (length(i) > 1) stop("subscript out of bounds", call. = FALSE)
  structure(
    .subset(x, i),
    class = edge_vctrs()
  )
}

#' @export
format.H3Edge <- function(x, ...) formatC(edges_to_strings(x), ...)

#' @export
#' @rdname edges
is_edge <- function(x) inherits(x, "H3Edge")


#' @export
#' @rdname edges
is_valid_edge <- function(x) {
  is_valid_edge_(x)
}

#' @export
#' @rdname edges
h3_edges_from_strings <- function(x) {
  h3_edge_from_strings_(x)
}


#' @export
#' @rdname edges
flatten_edges <- function(x) {
  all_classes <- vapply(x, function(x) class(x)[1], character(1))
  if (!identical(unique(all_classes), "H3Edge")) {
    stop("All list elements must be an H3Edge vector")
  }

  x <- unlist(x)
  structure(x, class = edge_vctrs())
}

#' @export
#' @rdname edges
h3_edge_cells <- function(x) {
  stopifnot(is_edge(x))
  get_directed_cells_(x)
}


#' @export
#' @rdname edges
h3_edge_origin <- function(x) {
  stopifnot(is_edge(x))
  get_directed_origin_(x)
}

#' @export
#' @rdname edges
h3_edge_destination <- function(x) {
  stopifnot(is_edge(x))
  get_directed_destination_(x)
}

st_as_sfc.H3Edge <- function(x) {
  sf::st_sfc(edge_boundary_(x), crs = 4326)
}


#' @export
#' @rdname edges
as.character.H3Edge <- function(x) {
  edges_to_strings(x)
}
