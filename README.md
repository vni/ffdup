# fidup
FInd DUPlicates (or Find fIle DUPlicates)

A simple utility in Rust which helps to find duplicates.
Uses blake3 (currently) hashing.

It's an Initial version. Here is what I would like to add to it:
- [ ] Refactor: get rid of .unwrap()
- [ ] Refactor: use proper error propagation and handling: ? and thiserror
- [ ] Refactor: get rid of .clone()
- [ ] Use different hashing algorithms to be able to play with them (to measure theirs speed on my non-test data) (sha256, blake2x, ...)
- [ ] use clap
  - [ ] command line arguments
    - [ ] --show-all
    - [ ] --show-only-duplicates
    - [ ] --show-hashes
    - [ ] --delete
    - [ ] --delete-without-asking
    - [ ] --block-size
  - [ ] version
  - [ ] author
  - [ ] help/usage
- [ ] run in the current directory by default (no need to pass the path)
- [ ] hash a single file, if it's file or traverse the directories (currently all the args are treated as directories).
- [ ] statistics
  - [ ] speed of the hashing
  - [ ] total number of processed bytes
  - [ ] total time
  - [ ] time to hash 1Kb/1Mb/1Gb of data
