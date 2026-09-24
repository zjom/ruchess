# ruchess

A chess rules library: representing positions, generating legal moves, playing them, and deciding when a game is over.

## Language

### Game state

**Position**:
A complete chess game state: piece placement, side to move, castling rights, en-passant possibility, clocks, and the repetition trail.
_Avoid_: state, situation

**Board**:
Piece placement only — which piece stands on which square, nothing else.
_Avoid_: position (when only placement is meant)

**History**:
The part of a Position that the Board can't show: last move, castling rights, unmoved rooks, half-move clock, repetition trail.

**Ply**:
One half-move; the count of plies played since the start of the game. Whose turn it is (the side to move) follows from the ply count.
_Avoid_: turn number, half-move (for the count)

**Side to move**:
The color whose turn it is. Derived from the Ply; never stored separately.
_Avoid_: turn, active color (except when quoting FEN)

### Moves

**Move**:
A legal move specification in a given Position: origin, destination, and kind (normal, promotion, en passant, castle).

**Uci**:
A move written as origin, destination, and optional promotion, with no knowledge of the Position.

**Make / unmake**:
Playing a Move on a Position, and exactly reversing it. The single place where the rules for what a move changes live.
_Avoid_: apply, update (for playing a move)

### Game end

**Repetition trail**:
The record of Positions that occurred before the current one in the game, used to recognise repeated positions.
_Avoid_: position hashes, hash trail

**Repetitions**:
How many times the current Position has occurred in the game, counting itself. Two Positions are the same if placement, side to move, castling rights, and en-passant possibility all match.

**Outcome**:
The result of a finished game: a win for one color, or a draw with its reason.
