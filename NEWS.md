# h3o 0.2.0

* Adds a `wk::wk_handler` function for conversion to sfc, geos, s2, and other types that implement the handler.
* New functionality: `to_cells()` to convert an `sfc` object to H3 cells. 
* Refactors Rust code to avoid collecting into `List`s to prevent Stack Overflow bug. 
* Bumps h3o version to 0.3.5 to get speed and memory enhancements. 

# h3o 0.1.0

* Added a `NEWS.md` file to track changes to the package.
