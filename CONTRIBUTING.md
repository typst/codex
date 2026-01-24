# Contributing Guidelines
Contributions are welcome! This document provides some resources and guidelines to help with the process.

Codex collects related Unicode symbols as different *variants* of the same *symbol*.[^symbol]
For example, `→ ⇒ ↑ ⇑` are four variants of the `arrow` symbol.
Each symbol has a default variant (here `→`).
To refer to a particular variant, *modifiers* can be appended to the symbol name
using dot separators.
For example `⇒` is `arrow.r.double`, `↑` is `arrow.t` and `⇑` is `arrow.t.double`.
Modifiers are order-independent, so the latter can also be referred to as `arrow.double.t`.
Additionally, not all modifiers have to be specified, in which case the best match[^match]
will be taken. For example, `⇒` can also be referred to as `arrow.double`.
Groups of related symbols are collected into *modules*. Modules can also contain other modules.
Codex exports two top-level modules: `sym` for text-style symbols and `emoji` for emoji;
Their source code is found in `src/modules/`.

If you need help with a contribution, you can also ask us [on Discord](https://discord.com/channels/1054443721975922748/1277628305142452306).

Proposals used to be written in a dedicated Proposals document,
but new proposals should now be filed as GitHub issues instead.
The [document](https://typst.app/project/riXtMSim5zLCo7DWngIFbT)
has been repurposed to serve as a collection of useful information
and naming ideas.

[^symbol]: This document also uses "symbol" in the more abstract sense of a graphical symbol.
[^match]: See the documentation of `ModifierSet::best_match_in` for the exact details.

## Pull Requests
- All PRs require two approvals by collaborators to be merged.
- PRs with breaking changes require three such approvals instead.
- PRs with changes to the public Rust API also require an approval by @laurmaedje.

To remove a symbol or variant, it is first marked as deprecated
(This is considered a breaking change).
After a Typst version that includes this deprecation notice has been released,
the deprecated symbol or variant will be removed (This is not considered a breaking change).
Instead of being removed, the name can also be repurposed for a different symbol,
which can be seen as a combination of removing the old symbol or variant
and adding a new one with the same name.

## Conventions
When adding new modules, symbols or variants, please try to be consistent with
existing ones. Below are some guidelines based on existing symbols. These aren't
always hard rules, especially because of how messy Unicode can be, but you should
adhere to them if possible.

### General Conventions
- English words use US spelling.
- Modifier and module names are entirely lowercase.
- Symbol names are lowercase unless the symbol is an uppercase letter.
- Symbol names should be at least two characters long so they can be used easily in Typst's math mode.
- When a symbol is added to a base, the symbol name is used as a modifier on the base.[^modifname]
    This can have the following meanings:
    1. The symbol is added around or inside the base as a subordinate (smaller than the base),
        e.g. `eq.quest`, `triangle.stroked.dot`.
    2. The symbol is stacked below the base, e.g. `gt.lt`.
    3. The symbol is stacked to the right of the base, e.g. `colon.eq`.
    4. The symbol is overlaid at the center of the base, e.g. `integral.dash`.
    5. The symbol surrounds the base, e.g. `plus.square`.
- Notable exceptions to the previous convention:
    - When `.eq` is used in the second sense (stacked below), it only adds a single line and not two,
        e.g. `lt.eq`. For two lines below, `.equiv` is used, e.g. `lt.equiv`.

[^modifname]: Though a modifier can also just coincidentally be a symbol name, e.g. `.not`.

### Established Generic Modifiers
These have a broad meaning and can have varying interpretations.
<!-- Geometry -->
- `.l`/`.r`/`.t`/`.b`: The four main directions (left/right/top/bottom), e.g. `arrow.l`, `times.r`.
    - For delimiters, `.l` means opening and `.r` means closing, e.g. `paren.l`, `quote.r`.
- `.tl`/`.tr`/`.bl`/`.br`: The four corners, e.g. `arrow.tl`, `triangle.stroked.tr`.
    - Generally, these are used for a single, diagonal direction,
        whereas combinations of two main directions (like `.t.l`) are used to mean both of them at once,
        e.g. `arrow.t.l`, if it existed, would be a bidirectional arrow that points both top and left,
        similarly to how `arrow.l.r` is an arrow pointing both left and right.
- `.cw`/`.ccw`: Clockwise/Counterclockwise, e.g. `arrow.cw`, `integral.ccw`.
- `.tiny`/`.small`/`.medium`/`.big`: A geometric shape with a certain size, e.g. `square.stroked.small`.
<!-- Strokes -->
- `.stroked`/`.filled`: A symbol that has an empty/filled interior, e.g. `circle.stroked`, `arrow.r.filled`.
    (They correspond to Unicode's "white"/"black".)
- `.dotted`: A shape with a dotted line instead of a full stroke, e.g. `circle.dotted`.
- `.light`/`.heavy`: A shape with a certain stroke weight, e.g. `checkmark.heavy`.
<!-- Other (in alphabetic order) -->
- `.alt`: An alternate glyph for the symbol, e.g. `phi.alt`.
- `.double`, `.triple`, `.quad`: A symbol that has 2-4 of something, e.g. `excl.double`, `eq.quad`.

### Established Concrete Modifiers
These have a specific meaning that is not open to much interpretation.
<!-- (in alphabetic order) -->
- `.big`: A [large](https://www.unicode.org/Public/math/latest/MathClassEx-15.html) (n-ary) version
    of an operator, e.g. `union.big`.
- `.inv`: Either vertically mirrored or a 180° rotated version of a symbol, e.g. `amp.inv`, `Omega.inv`.
    - See also [#108](https://github.com/typst/codex/issues/108).
- `.not`: A negation of the symbol, e.g. `eq.not`.
- `.o`: A symbol with a circle around it, e.g. `plus.circle`.
    - See also [#62](https://github.com/typst/codex/pull/62)
- `.rev`: A horizontally mirrored version of a symbol, e.g. `in.rev`.
    - See also [#108](https://github.com/typst/codex/issues/108).
- `.sq`: A "squarified" version of a symbol, e.g. `subset.sq`.
    - See also [#110](https://github.com/typst/codex/pull/110)
