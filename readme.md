# Filler (Rust)

An algorithmic game where two virtual robots fight for territory on a grid. Built in Rust, your goal is to design a robust robot AI capable of analyzing the grid, parsing incoming game pieces, and placing them strategically to outmaneuver opponent robots.

## Table of Contents

- [About the Game](#about-the-game)
  - [The Anfield (Grid)](#the-anfield-grid)
  - [The Pieces](#the-pieces)
  - [The Robots & Game Rules](#the-robots--game-rules)
- [The Game Engine](#the-game-engine)
- [Getting Started with Docker](#getting-started-with-docker)
- [Usage & Protocol Examples](#usage--protocol-examples)
- [Unit Tests](#unit-tests)
- [Audit & Evaluation Checklist](#audit--evaluation-checklist)
- [Bonus Challenges](#bonus-challenges)

---

## About the Game

Filler is an algorithmic competition where two robots take turns placing randomly shaped pieces onto a grid of known dimensions (called the **Anfield**). Each player's objective is to cover the largest surface area possible. 

### The Anfield (Grid)

The Anfield is a two-dimensional grid with an arbitrary number of rows and columns. To launch a match, you must pass an initial Anfield grid with starting positions for both robots to the game engine.

Here is an example of an initial Anfield (30 columns by 14 rows):

```text
..............................
..............................
..$...........................
..............................
..............................
..............................
..............................
..............................
..............................
..............................
..............................
...........................@..
..............................
..............................
```

### The Pieces

Pieces are generated randomly by the game engine. Their shape and size cannot be predicted until the engine transmits them to your program during its turn. 

Examples of potential pieces:

**Piece 2 2:**
```text
.#
#.
```

**Piece 5 4:**
```text
.##..
.##..
..#..
...#.
```

**Piece 6 3:**
```text
.##...
###...
#..#..
```

### The Robots & Game Rules

1. **Turns & Placement:** Robots take turns placing the random pieces provided by the engine. Each correctly placed piece awards points. 
2. **Overlapping Rule:** To place a piece, exactly **one** cell of the new piece must overlap your existing territory. You cannot overlap your opponent's territory, and you cannot exceed the boundaries of the Anfield.
3. **Game Over Conditions:**
   - The game ends when neither player can place any more pieces, or if a player returns an unexpected error (timeout, segfault, memory issues, etc.).
   - If only one player cannot place a piece (or makes an invalid move), their turn is ignored, but the other player can continue placing pieces to maximize their score.
   - If a robot cannot place any more pieces, it must still return a result (even if invalid). The built-in robots, for example, return `0 0\n` when they have no moves left.
   - If a solution is wrong, the engine stops letting that robot play, but the game continues for the other robot.
   - Any timeout or unexpected crash results in an immediate loss.
4. **Player Identifiers:**
   - **Player 1:** Represented on the grid by `a` and `@`.
   - **Player 2:** Represented on the grid by `s` and `$`.
   - Lowercase letters (`a` or `s`) mark the most recently placed piece. On subsequent turns, these turn into uppercase letters (`@` or `$`) as they become part of the permanent territory.

---

## The Game Engine

The game engine manages the match flow and runs inside a Docker container. It supports the following command-line flags:

| Flag | Argument Type | Description |
| :--- | :--- | :--- |
| `-f`, `-file` | `string` | Path to the map |
| `-p1`, `-player1` | `string` | Path to the first AI robot |
| `-p2`, `-player2` | `string` | Path to the second AI robot |
| `-q`, `-quiet` | *None* | Quiet mode (reduces console output) |
| `-r`, `-refresh` | *None* | Throttling mode |
| `-s`, `-seed` | `int` | Use a specific random seed number |
| `-t`, `-time` | `int` | Set timeout in seconds (default: 10) |

---

## Getting Started with Docker

You must build and run this project inside the provided Docker environment. 

1. **Download the Assets:** Download and unzip the `docker_image` folder containing the `Dockerfile`, `game_engine`, maps, and opponent robots.
2. **Build the Docker Image:**
   ```bash
   cd docker_image
   docker build -t filler .
   ```
3. **Run the Container:**
   Mount your local `solution` directory to the container so you can edit code on your host machine:
   ```bash
   docker run -v "$(pwd)/solution":/filler/solution -it filler
   ```
4. **Compile and Run inside the Container:**
   Compile your Rust code within the container's interactive terminal, then launch the game engine:
   ```bash
   ./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator
   ```

---

## Usage & Protocol Examples

At the start of each turn, the game engine writes to your robot's standard input (`stdin`). 

### Input Protocol Example

```text
$$$ exec p1 : [robots/bender]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 ....................
004 ....................
005 ....................
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 ....................
012 .........$..........
013 ....................
014 ....................
Piece 4 1:
.OO.
```

### Output Protocol Response

Your robot must calculate the optimal coordinate and write it to standard output (`stdout`) in the format `X Y\n`. For example:

```text
7 2
```

---