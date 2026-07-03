# TODO - Rust Filler Bot

- [ ] Initialize Rust project structure (Cargo.toml + src files)
- [ ] Implement core domain models (Anfield, Piece, Cell/Player markers)
- [ ] Implement robust stdin parser for engine input stream
- [ ] Implement placement validation rules:
  - [ ] exactly one overlap with own territory
  - [ ] zero overlap with opponent territory
  - [ ] fully inside anfield bounds
- [ ] Implement move search strategy (baseline heuristic)
- [ ] Implement stdout output format `X Y`
- [ ] Add unit tests for:
  - [ ] input parsing
  - [ ] placement validation
  - [ ] boundary detection
  - [ ] coordinate output formatting
- [ ] Run tests and ensure all pass
- [ ] Document build/run instructions in README
