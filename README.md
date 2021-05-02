# Alchemists Board Game Code Algorithms

Algorithms that interpret official Alchemists companion app codes in a compatible way.

## Rationale

I have a few concerns about the official companion apps:

1. The mobile apps are built on a discontinued game engine, which uses binary libraries that are falling out of support
2. The mobile and web apps disagree on some codes, leading to game-breaking compatibility issues if players use a mixture

Additionally, if someone wanted to create a replacement to address these issues, it would be difficult to create an effective code scheme that didn't conflict with the official apps.

## Usage

If you have Rust (and `cargo`) installed, and have cloned the repo:

```shell
$ cargo run -- --help
abg-code-algorithms 0.1.0

USAGE:
    abg-code-algorithms [FLAGS] [codes]...

FLAGS:
    -h, --help       Prints help information
    -s, --sheet      Output results like the deduction sheet
    -V, --version    Prints version information
    -w, --web        Replicate any known companion web app discrepancies

ARGS:
    <codes>...    Companion app codes

$ cargo run -- golem
Code, Mushroom, Fern, Toad, Claw, Flower, Root, Scorpion, Feather, Head, Chest
GOLEM, G-, R+, B-, ++, B+, G+, R-, --, b, G

$ cargo run -- --web golem
Code, Mushroom, Fern, Toad, Claw, Flower, Root, Scorpion, Feather, Head, Chest
GOLEM, --, ++, G+, R+, B-, B+, G-, R-, b, G

$ cargo run -- --sheet demo
+------+----------+------+------+------+--------+------+----------+---------+
| DEMO | Mushroom | Fern | Toad | Claw | Flower | Root | Scorpion | Feather |
+------+----------+------+------+------+--------+------+----------+---------+
| B-   |          |      |      |  x   |        |      |          |         |
+------+----------+------+------+------+--------+------+----------+---------+
| B+   |          |      |  x   |      |        |      |          |         |
+------+----------+------+------+------+--------+------+----------+---------+
| G-   |          |      |      |      |        |  x   |          |         |
+------+----------+------+------+------+--------+------+----------+---------+
| G+   |          |      |      |      |        |      |    x     |         |
+------+----------+------+------+------+--------+------+----------+---------+
| R-   |          |  x   |      |      |        |      |          |         |
+------+----------+------+------+------+--------+------+----------+---------+
| R+   |    x     |      |      |      |        |      |          |         |
+------+----------+------+------+------+--------+------+----------+---------+
| --   |          |      |      |      |   x    |      |          |         |
+------+----------+------+------+------+--------+------+----------+---------+
| ++   |          |      |      |      |        |      |          |    x    |
+------+----------+------+------+------+--------+------+----------+---------+
```

## License

This contents of this repository are licensed under [The Unlicense](./LICENSE).

The _Alchemists_ board game is copyright © Czech Games Edition, October 2014.
_Alchemists: The King's Golem_ is copyright © Czech Games Edition, October 2016.
