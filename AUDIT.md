# Audit results
Functional

Try to run the command ./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator inside the container.

Can you confirm that the student was able to create the image and container correctly?
## docker build -t filler .
## docker image ls filler
## docker create --name filler-player filler
## docker container ls -a --filter "name=filler-player"
## docker rm filler-player
## docker build --no-cache -t filler .
## docker inspect filler --format '{{json .Config.Entrypoint}}'
## docker create --name filler-player filler
## docker container ls -a --filter "name=filler-player"

Try to run the student player against one of our players.
Can you confirm that the project runs correctly?

Try to run the student player against one of our players.
Can you confirm that the student player is placing the pieces correctly with the overlapping of just on cell?

Try to run ./game_engine -f maps/map00 -p1 <path to student player> -p2 robots/wall_e five times changing the position of the players each time so that the student player can be the p1 and the p2.
Can you confirm that the student player won at least 4 out of 5 times?

Try to run ./game_engine -f maps/map01 -p1 <path to student player> -p2 robots/h2_d2 five times changing the position of the players each time so that the student player can be the p1 and the p2.
Can you confirm that the student player won at least 4 out of 5 times?

Try to run ./game_engine -f maps/map02 -p1 <path to student player> -p2 robots/bender five times changing the position of the players each time so that the student player can be the p1 and the p2.
Can you confirm that the student player won at least 4 out of 5 times?

Unit Tests

Do all tests pass without errors?

Are there specific tests for Input Parsing (e.g., verifying the robot correctly reads the Anfield dimensions and the piece shape from stdin)?

Are there tests for Placement Validation (e.g., checking that a move is rejected if it overlaps two of your own cells or one of the opponent's)?

Are there tests for Boundary Detection to ensure pieces are never placed partially outside the grid?

Basic

+Does the code obey the good practices?

+Is there a test file for this code?




Copy-Item .\Cargo.toml .\grader\docker_image\solution\Cargo.toml -Force
Copy-Item .\Cargo.lock .\grader\docker_image\solution\Cargo.lock -Force

docker run --rm -it -v "${PWD}\grader\docker_image\solution:/filler/solution" filler-grader
cd /filler/solution
cargo test
cargo build --release

cd /filler

## walle
PLAYER=./solution/target/release/solution
MAP=maps/map00
OPPONENT=linux_robots/wall_e

for game in 1 2 3 4 5; do
  echo "=== Game $game ==="
  if [ $((game % 2)) -eq 1 ]; then
    ./linux_game_engine -q -f "$MAP" -p1 "$PLAYER" -p2 "$OPPONENT"
  else
    ./linux_game_engine -q -f "$MAP" -p1 "$OPPONENT" -p2 "$PLAYER"
  fi
done | tee wall_e_results.txt

## bender
PLAYER=./solution/target/release/solution
MAP=maps/map02
OPPONENT=linux_robots/bender

for game in 1 2 3 4 5; do
  echo "=== Game $game ==="
  if [ $((game % 2)) -eq 1 ]; then
    ./linux_game_engine -q -f "$MAP" -p1 "$PLAYER" -p2 "$OPPONENT"
  else
    ./linux_game_engine -q -f "$MAP" -p1 "$OPPONENT" -p2 "$PLAYER"
  fi
done | tee bender_results.txt

grep -E 'Player[12] \(|won!' bender_results.txt

## h2d2
PLAYER=./solution/target/release/solution
MAP=maps/map01
OPPONENT=linux_robots/h2_d2

for game in 1 2 3 4 5; do
  echo "=== Game $game ==="
  if [ $((game % 2)) -eq 1 ]; then
    ./linux_game_engine -q -f "$MAP" -p1 "$PLAYER" -p2 "$OPPONENT"
  else
    ./linux_game_engine -q -f "$MAP" -p1 "$OPPONENT" -p2 "$PLAYER"
  fi
done | tee h2_d2_results.txt

grep -E 'Player[12] \(|won!' h2_d2_results.txt