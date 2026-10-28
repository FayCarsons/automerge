# `update_spans` reproductions

Minimal reproductions of two `update_spans` bugs. Each one runs against the latest release
(`automerge` 0.12.0 on crates.io, `@automerge/automerge` 3.5.0 on npm) and also
reproduces on `main` at `5bae7757e`.

## Deleting a grapheme leaves part of it behind

`update_spans` diffs text grapheme by grapheme, but removes each deleted grapheme with
`delete`, which removes a single code point. Any grapheme made of several code points
leaves the rest behind. Fixed for 0.12.0 in
[`6614d6e`](https://github.com/FayCarsons/automerge/commit/6614d6e11).

```sh
cd rust && cargo run --bin grapheme_delete
cd js && npm install && node grapheme_delete.mjs
```

Both print:

```
"xe\u{301}y" -> "x\u{301}y"
"x🇫🇷y" -> "x🇷y"
"x👍🏽y" -> "x🏽y"
```

Every line should end in `"xy"`.

## Edits after a block land at the wrong index under `Utf8CodeUnit`

A block marker is stored as `"\u{fffc}"`, which is 3 bytes wide under `Utf8CodeUnit`, but
`update_spans` counts each block as 1 unit. Every index after a block is off by 2.

```sh
cd rust && cargo run --bin utf8_block_width
```

```
block marker width: 3
before: "￼xay"
after:  "￼ay"
```

`after` should be `"￼xy"`: `update_spans` deletes the `x` instead of the `a`.
