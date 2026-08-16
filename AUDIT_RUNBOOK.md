# Audit command runbook

Use this file as the evidence checklist for the Filler audit. Run commands in the order shown and copy the terminal output into `AUDIT.md` or submit the generated `*_results.txt` files. Do not mark a five-game requirement as passed unless its log reports at least `4/5` student wins.

## 1. Windows PowerShell: prepare the supplied grader container

Run these commands from the project root (`C:\Users\amani\filler-1`). They copy the current source and tests to the grader package, build the image containing the engine/maps/robots, and open its Linux shell.

```powershell
Copy-Item .\Cargo.toml .\grader\docker_image\solution\Cargo.toml -Force
Copy-Item .\Cargo.lock .\grader\docker_image\solution\Cargo.lock -Force
Copy-Item .\src .\grader\docker_image\solution\src -Recurse -Force
Copy-Item .\tests .\grader\docker_image\solution\tests -Recurse -Force

docker build -t filler-grader .\grader\docker_image
docker run --rm -it -v "${PWD}\grader\docker_image\solution:/filler/solution" filler-grader
```

The Docker build and container start demonstrate that the image/container were created. Answer:

> Yes. The `filler-grader` image built successfully and its container started successfully.

## 2. Linux container: build and test the student player

After `docker run`, the prompt begins with `root@...:/filler#`. Run:

```sh
cd /filler/solution
cargo build --release
cargo test
cd /filler
PLAYER=./solution/target/release/solution
```

Expected test result: `7 passed; 0 failed`.

Answer the unit-test questions:

> Yes. `cargo test` completed with 7 passed and 0 failed tests.

> Yes. `tests/input_parsing.rs` checks player identification, Anfield dimensions/grid rows, and piece shape parsing.

> Yes. `tests/placement_validation.rs` checks exactly one self-overlap and rejects two self-overlaps and opponent overlap.

> Yes. `tests/placement_validation.rs` checks that pieces partially outside the grid are rejected.

## 3. Confirm the supplied engine runs

```sh
./linux_game_engine -q -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator
```

The command must print player scores and a winner. This is the Linux-package equivalent of the audit's `./game_engine ... robots/...` command.

## 4. Confirm the student player runs and placements are accepted

```sh
./linux_game_engine -q -f maps/map00 -p1 "$PLAYER" -p2 linux_robots/wall_e
```

If it finishes with scores/a winner and without an invalid-placement error, answer:

> Yes. The student player completed a game against `wall_e` and the engine accepted its moves. The placement validator requires exactly one overlap with the student's cells and rejects opponent overlaps; the automated placement tests pass.

## 5. Run and record each five-game series

Paste this function once into the Linux container. It alternates the student as P1/P2, writes a result file, and counts student wins automatically.

```sh
run_series() {
  label="$1"
  map="$2"
  opponent="$3"
  log_file="${label}_results.txt"

  for game in 1 2 3 4 5; do
    echo "=== Game $game ==="
    if [ $((game % 2)) -eq 1 ]; then
      ./linux_game_engine -q -f "$map" -p1 "$PLAYER" -p2 "$opponent"
    else
      ./linux_game_engine -q -f "$map" -p1 "$opponent" -p2 "$PLAYER"
    fi
  done | tee "$log_file"

  awk '
    /^=== Game / { game=$3 }
    /^Player1 won!/ { if (game % 2 == 1) wins++ }
    /^Player2 won!/ { if (game % 2 == 0) wins++ }
    END { printf "Student wins: %d/5\\n", wins }
  ' "$log_file"
}
```

Run the requested series:

```sh
run_series wall_e maps/map00 linux_robots/wall_e
run_series h2_d2 maps/map01 linux_robots/h2_d2
run_series bender maps/map02 linux_robots/bender
run_series terminator maps/map01 linux_robots/terminator
```

For each series, use the generated line in the audit:

> `wall_e`: The student won **[N]/5** games with the player order alternated. **[Pass if N is 4 or 5; otherwise, do not confirm.]**

> `h2_d2`: The student won **[N]/5** games with the player order alternated. **[Pass if N is 4 or 5; otherwise, do not confirm.]**

> `bender`: The student won **[N]/5** games with the player order alternated. **[Pass if N is 4 or 5; otherwise, do not confirm.]**

> `terminator`: The student won **[N]/5** games with the player order alternated. **[Pass if N is 4 or 5; otherwise, do not confirm.]**

## 6. Basic-code answers

Use the following evidence-based wording:

> Yes. The code separates parsing and placement logic into modules, uses consistent Rust formatting/naming, documents the project, includes a Docker build, and has automated tests. The design follows the applicable organization, naming, separation-of-concerns, and error-handling principles in the [01 Edu good-practices guide](https://github.com/01-edu/public/blob/master/subjects/good-practices/README.md).

> Yes. The `tests/` directory contains `input_parsing.rs` and `placement_validation.rs`.

> Yes for the required cases: valid one-cell overlap, invalid multiple/self overlap, opponent overlap, boundary overflow, player/header parsing, grid dimensions/rows, and piece shape. The suite cannot prove every theoretically possible input case, but it covers the audit's required cases.
