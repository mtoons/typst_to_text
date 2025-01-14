# typst_to_text
Simple [typst math](https://typst.app/docs/reference/math/) to unicode text converter.

## Usage
Simply execute the command `typst_to_text` followed by your typst math.

Exemple :
```shell
$ typst_to_text (u_n) n in NN
(uₙ) n ∈ ℕ
```
## Bugs
Chars that are not alphabetical, part of a known symbol or a known shorthand are ignored.

## Info
As this is my first rust project I'm very open to feedback and contibutions on what i could improve.
