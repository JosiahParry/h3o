devtools::load_all()

g <- sfdep::guerry |>
  sf::st_set_crs(27572) |>
  sf::st_transform(4326) |>
  sf::st_geometry()

h3_from_points(sf::st_centroid(g), 10) |>
  h3_resolution()
