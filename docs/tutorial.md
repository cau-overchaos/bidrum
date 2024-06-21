# Tutorial

## Game Introduction

-   This game is a rhythm game inspired by the traditional Korean instrument, Janggu.
-   The game is designed to be played using a controller modeled after the Janggu. However, it can also be played using a keyboard.

## Note Introduction

-   There are two types of notes.
    | Gungchae Note | Yeolchae Note |
    |:-------------:|:-------------:|
    | <img height="200" src="../game/assets/img/note/left_stick.png"> | <img align="center" height="200" src="../game/assets/img/note/right_stick.png"> |
-   Each note can appear on either the gungpyeon (left side of the game) or the yeolpyeon (right side of the game).

## How to Play

-   For Gungchae Notes, if you are using the Janggu controller, strike each side with the gungchae. If you are not using the Janggu controller, press the "D" key when the note appears on the gungpyeon and the "F" key when it appears on the yeolpyeon.
-   For Yeolchae Notes, if you are using the Janggu controller, strike each side with the yeolchae. If you are not using the Janggu controller, press the "K" key when the note appears on the yeolpyeon and the "J" key when it appears on the gungpyeon.
-   The game will automatically display your score and end once the song finishes.

## Judgement System

-   A precise hit is considered when the strike timing matches the system's timing with a 0ms difference.

```rust
// ms
pub const OVERCHAOS_TIMING: i64 = 10;
pub const PERFECT_TIMING: i64 = 40;
pub const GREAT_TIMING: i64 = 70;
pub const GOOD_TIMING: i64 = 200;
pub const BAD_TIMING: i64 = 600;
pub const HAT_TIMING: i64 = 1500;
```

-   The timing for each judgement can be seen in the code above.
-   If the difference is greater than `BAD_TIMING`, it is judged as a `MISS`.

## Combo System

-   The combo system does not affect the score but it exists.

```rust
pub const OVERCHAOS_COMBO: u64 = 1;
pub const PERFECT_COMBO: u64 = 1;
pub const GREAT_COMBO: u64 = 1;
pub const GOOD_COMBO: u64 = 1;
pub const BAD_COMBO: u64 = 0;
pub const MISS_COMBO: u64 = 0;
```

-   In the code above, a value of 1 means the combo is maintained for that judgement, while a value of 0 means the combo is reset to 0.

## Health System

-   The game has a health system and if health reaches 0, the game is considered not cleared, but you can still play until the end.

```rust
pub const DEFAULT_HEALTH: u64 = 1000;
pub const OVERCHAOS_HEALTH: i64 = 50;
pub const PERFECT_HEALTH: i64 = 50;
pub const GREAT_HEALTH: i64 = 10;
pub const GOOD_HEALTH: i64 = 0;
pub const BAD_HEALTH: i64 = -20;
pub const MISS_HEALTH: i64 = -50;
```

-   Initially, 1000 health is given, and health is adjusted based on the judgements as shown in the code above.
-   Once health reaches 0, it does not recover.

## Scoring System

-   Scores are calculated based on the judgement time for each note.

```rust
self.score += ((f64::abs(BAD_TIMING as f64 -difference_abs.clamp(OVERCHAOS_TIMING, BAD_TIMING) as f64,) / (BAD_TIMING - OVERCHAOS_TIMING) as f64) * 1000.0) as u64;
```

-   Scores are calculated using the formula above.
    -   Here, `difference_abs` calculates the absolute difference in milliseconds from the perfect judgement of 0ms.
    -   The use of `clamp` ensures that if `difference_abs` exceeds `BAD_TIMING`, the score is set to 0, and if it is less than `OVERCHAOS_TIMING`, a perfect score of 1000 is given to prevent excessive competition.
