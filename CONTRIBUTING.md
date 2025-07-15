# Conventions
When adding new modules, symbols or variants, please try to be consistent with
the following list of conventions. These aren't always hard rules, especially
because of how messy Unicode can be, but you should adhere to them if possible.

General conventions:
- Modifiers are entirely lowercase.
- Modifiers that correspond to symbols (by having the same name) have six established meanings:
	- The symbol is added around the base like an accent or an attachment (smaller than the base), e.g. `eq.quest`.
	- The symbol is stacked below the base, e.g. `gt.lt`.
	- The symbol is stacked to the right of the base, e.g. `colon.eq`.
	- The symbol is overlaid at the center of the base, e.g. `integral.dash`.
	- The symbol surrounds the base, e.g. `plus.square`.
	- The symbol is placed inside the base, e.g. `triangle.stroked.dot`.
Established generic modifiers:
- `.l`/`.r`/`.t`/`.b`: The four main directions (left/right/top/bottom).
	- For delimiters, `.l` means opening and `.r` means closing (see [#100](https://github.com/typst/codex/pull/100)).
- `.tl`/`.tr`/`.bl`/`.br`: The four corners
	<!-- TODO: Do we have or want to have conventions about when to choose `.tl` vs. `.t.l`? -->
- `.double`, `.triple`, `.quad`: A symbol that has 2-4 of something.
- `.stroked`/`.filled`: A symbol that has an empty/filled interior.
	(They correspond to Unicode's "white"/"black".)
- `.tiny`/`.small`/`.medium`/`.big`: A geometric shape with a certain size.
- `.light`/`.heavy`: A shape with a certain stroke weight.
- `.dotted`: A shape with a dotted line instead of a full stroke.
Established concrete modifiers:
- `.big`: A big (n-ary) version of an operator.
- `.alt`: An alternative version that is still more or less the same symbol.
- `.not`: A symbol with a (typically diagonal) line through it.
- `.o`: A symbol with a circle around it (see [#62](https://github.com/typst/codex/pull/62))
- `.sq`: A "squarified" version of a symbol (see [#110](https://github.com/typst/codex/pull/110))
- `.rev`: A horizontally mirrored version of a symbol (see [#108](https://github.com/typst/codex/issues/108))
- `.inv`: Either vertically mirrored or a 180° rotated version of a symbol (see [#108](https://github.com/typst/codex/issues/108))
- `.cw`/`.ccw`: Clockwise/Counterclockwise
Established exceptions:
- When `.eq` is used in the second sense (stacked below), it only adds a single line and not two.
	For two lines below, `.equiv` is used.
- `.not` (see above) does not correspond to the symbol `not` despite having the same name.
