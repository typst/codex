# Proposals
Proposals used to be written in the [Proposals document](https://typst.app/project/riXtMSim5zLCo7DWngIFbT),
although it is now preferred to have a GitHub issue for each one.
Nonetheless, the document still contains a lot of useful information.

# Conventions
When adding new modules, symbols or variants, please try to be consistent with
existing ones. Below are some guidelines based on existing symbols. These aren't
always hard rules, especially because of how messy Unicode can be, but you should
adhere to them if possible.

General conventions:
- Modifier and module names are entirely lowercase.
	Symbol names are lowercase unless the symbol is an uppercase letter.
- When a symbol is added to a base, the symbol name is used as a modifier on the base.
	[^1](Though a modifier can also just coincidentally be a symbol name, e.g. `.not`.)
	This can have the following meanings:
	- The symbol is added around or inside the base as a subordinate (smaller than the base),
		e.g. `eq.quest`, `triangle.stroked.dot`.
	- The symbol is stacked below the base, e.g. `gt.lt`.
	- The symbol is stacked to the right of the base, e.g. `colon.eq`.
	- The symbol is overlaid at the center of the base, e.g. `integral.dash`.
	- The symbol surrounds the base, e.g. `plus.square`.
- Notable exceptions to the previous convention:
	- When `.eq` is used in the second sense (stacked below), it only adds a single line and not two.
		For two lines below, `.equiv` is used.
Established generic modifiers:
- `.l`/`.r`/`.t`/`.b`: The four main directions (left/right/top/bottom), e.g. `arrow.{l,r,t,b}`.
	- For delimiters, `.l` means opening and `.r` means closing (see [#100](https://github.com/typst/codex/pull/100)).
- `.tl`/`.tr`/`.bl`/`.br`: The four corners, e.g. `arrow.{tl,tr,bl,br}`.
	<!-- TODO: Do we have or want to have conventions about when to choose `.tl` vs. `.t.l`? -->
- `.double`, `.triple`, `.quad`: A symbol that has 2-4 of something.
- `.stroked`/`.filled`: A symbol that has an empty/filled interior.
	(They correspond to Unicode's "white"/"black".)
- `.tiny`/`.small`/`.medium`/`.big`: A geometric shape with a certain size.
- `.light`/`.heavy`: A shape with a certain stroke weight.
- `.dotted`: A shape with a dotted line instead of a full stroke.
- `.alt`: An alternate glyph for the symbol.
Established concrete modifiers:
- `.big`: A [large](https://www.unicode.org/Public/math/latest/MathClassEx-15.html) (n-ary) version of an operator.
- `.not`: A negation of the symbol.
- `.o`: A symbol with a circle around it (see [#62](https://github.com/typst/codex/pull/62))
- `.sq`: A "squarified" version of a symbol (see [#110](https://github.com/typst/codex/pull/110))
- `.rev`: A horizontally mirrored version of a symbol (see [#108](https://github.com/typst/codex/issues/108))
- `.inv`: Either vertically mirrored or a 180° rotated version of a symbol (see [#108](https://github.com/typst/codex/issues/108))
- `.cw`/`.ccw`: Clockwise/Counterclockwise
