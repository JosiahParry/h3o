compact_cells <- function(h3) {
  h3_strings <- h3_to_strings(h3)
  compact_cells_(h3[match(unique(h3_strings), h3_strings)])
}
