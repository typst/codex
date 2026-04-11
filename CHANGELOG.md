# Changelog

## Unreleased

### General changes

- Trivially updated to Unicode 17.0.0

### New in `sym`

- Punctuation
  - `underscore`: _

- Mathematical symbols
  - `emptyset.zero`: ∅︀
  - `nothing.zero`: ∅︀
  - `union.serif`: ∪︀
  - `union.sq.serif`: ⊔︀
  - `inter.serif`: ∩︀
  - `inter.sq.serif`: ⊓︀
  - `gt.double.nested`: ⪢
  - `lt.double.nested`: ⪡
  - `gt.arc` ⪧
  - `gt.arc.eq` ⪩
  - `lt.arc`: ⪦
  - `lt.arc.eq`: ⪨
  - `subset.approx`: ⫉
  - `subset.closed`: ⫏
  - `subset.closed.eq`: ⫑
  - `subset.eq.dot`: ⫃
  - `subset.equiv`: ⫅
  - `subset.nequiv`: ⫋
  - `subset.plus`: ⪿
  - `subset.tilde`: ⫇
  - `subset.times`: ⫁
  - `supset.approx`: ⫊
  - `supset.closed`: ⫐
  - `supset.closed.eq`: ⫒
  - `supset.eq.dot`: ⫄
  - `supset.equiv`: ⫆
  - `supset.nequiv`: ⫌
  - `supset.plus`: ⫀
  - `supset.tilde`: ⫈
  - `supset.times`: ⫂
  - `pee`: ℘
  - `gt.closed`: ⊳
  - `gt.closed.eq`: ⊵
  - `gt.closed.eq.not`: ⋭
  - `gt.closed.not`: ⋫
  - `lt.closed`: ⊲
  - `lt.closed.eq`: ⊴
  - `lt.closed.eq.not`: ⋬
  - `lt.closed.not`: ⋪

- Miscellaneous technical
  - `bowtie.stroked`: ⋈
  - `bowtie.stroked.big`: ⨝
  - `bowtie.stroked.big.l`: ⟕
  - `bowtie.stroked.big.r`: ⟖
  - `bowtie.stroked.big.l.r`: ⟗
  - `bowtie.filled`: ⧓
  - `bowtie.filled.l`: ⧑
  - `bowtie.filled.r`: ⧒

- Currency
  - `riyal`: ⃁

- Control (Printable symbols representing non-printable characters)

  - `null` ␀
  - `heading.start` ␁
  - `text.start` ␂
  - `text.end` ␃
  - `transmission.end` ␄
  - `transmission.block.end` ␗
  - `enquiry` ␅
  - `bell` ␇
  - `backspace` ␇
  - `tab.h` ␉
  - `tab.v` ␋
  - `line.feed` ␊
  - `line.return` ␍
  - `line.feed` ␌
  - `shift.out` ␎
  - `shift.in` ␏
  - `escape` ␐
  - `device.one` ␑
  - `device.two` ␒
  - `device.three` ␓
  - `device.four` ␔
  - `acknowledge` ␆
  - `acknowledge.not` ␕
  - `synchronous` ␖
  - `cancel` ␘
  - `medium.end` ␙
  - `substitute` ␚
  - `substitute.alt` ␦
  - `escape` ␛
  - `separator.file` ␜
  - `separator.group` ␝
  - `separator.record` ␞
  - `separator.unit` ␟
  - `space` ␠
  - `delete` ␡
  - `delete.alt` ␥
  - `blank` ␢
  - `blank.box` ␣

### New in `emoji`

- bigfoot: 🫈
- dancing.ballet: 🧑‍🩰
- face.distorted: 🫪
- fightcloud: 🫯
- landslide: 🛘
- orca: 🫍
- treasure: 🪎
- trombone: 🪊

### Deprecations in `sym`

- `gt.tri` and variants in favor of `gt.closed`
- `lt.tri` and variants in favor of `lt.closed`
- `join` and its variants in favor of `bowtie.big` with the same variants

### Removals **(Breaking change)**

These previously deprecated items were removed:
- `paren.double.*`
- `brace.double.*`
- `bracket.double.*`
- `shell.double.*`
- `bar.circle`
- `ast.small`
- `ast.circle`
- `backslash.circle`
- `dash.circle`
- `dot.circle`, `dot.circle.big`
- `quote.angle.*`
- `plus.circle`, `plus.circle.*`
- `plus.small`
- `minus.circle`
- `div.circle`
- `times.circle`, `times.circle.big`
- `eq.circle`
- `eq.small`
- `gt.circle`
- `gt.small`
- `lt.circle`
- `lt.small`
- `sect`, `sect.*`
- `diff`
- `integral.sect`
- `angle.l`, `angle.l.*`
- `angle.r`, `angle.r.*`
- `angle.oblique`
- `angle.right.sq`
- `angle.spheric.top`
- `parallel.circle`
- `perp.circle`
- `franc`
- `circle.nested`
- `kai`, `Kai`
- `alef`
- `bet`
- `gimmel`
- `dalet`
- `shin`
- `planck.reduce`

## Version 0.2.0 (October 7, 2025)

### General changes

- Codepoints that have a symbol and emoji presentation now have the correct variation selector attached depending on whether they appear in `sym` or `emoji`
- Added support for multi-character symbols **(Breaking change)**
- Added support for deprecated symbol variants **(Breaking change)**
- Added `ModifierSet` type and made use of it in public API **(Breaking change)**
- Added `Symbol::get`, `Symbol::variants`, and `Symbol::modifiers` functions
- Added Rust module for styling mathematical codepoints (behind `styling` feature flag, enabled by default)

### Changed codepoint **(Breaking change)**

- `sym.planck` from ℎ to ħ
- `sym.peso` from ₱ to $
- `emoji.dancing.bunny` from women to gender neutral
- `emoji.handholding` from men to gender neutral

### New in `sym`

- Arrows
  - `arrow.r.double.struck`: ⤃
  - `arrow.r.struck`: ⇸
  - `arrow.r.dstruck`: ⇻
  - `arrow.r.tail.struck`: ⤔
  - `arrow.r.tail.dstruck`: ⤕
  - `arrow.r.twohead.struck`: ⤀
  - `arrow.r.twohead.dstruck`: ⤁
  - `arrow.r.twohead.tail`: ⤖
  - `arrow.r.twohead.tail.struck`: ⤗
  - `arrow.r.twohead.tail.dstruck`: ⤘
  - `arrow.r.open`: ⇾
  - `arrow.l.double.struck`: ⤂
  - `arrow.l.struck`: ⇷
  - `arrow.l.dstruck`: ⇺
  - `arrow.l.tail.struck`: ⬹
  - `arrow.l.tail.dstruck`: ⬺
  - `arrow.l.twohead.struck`: ⬴
  - `arrow.l.twohead.dstruck`: ⬵
  - `arrow.l.twohead.tail`: ⬻
  - `arrow.l.twohead.tail.struck`: ⬼
  - `arrow.l.twohead.tail.dstruck`: ⬽
  - `arrow.t.struck`: ⤉
  - `arrow.t.dstruck`: ⇞
  - `arrow.b.struck`: ⤈
  - `arrow.b.dstruck`: ⇟
  - `arrow.l.r.double.struck`: ⤄
  - `arrow.l.r.struck`: ⇹
  - `arrow.l.r.dstruck`: ⇼
  - `arrow.l.open`: ⇽
  - `arrow.l.r.open`: ⇿

- Delimiters
  - `bracket.l.tick.t`: ⦍
  - `bracket.l.tick.b`: ⦏
  - `bracket.r.tick.t`: ⦐
  - `bracket.r.tick.b`: ⦎
  - `paren.l.flat`: ⟮
  - `paren.r.flat`: ⟯
  - `paren.l.closed`: ⦇
  - `paren.r.closed`: ⦈
  - `shell.l.filled`: ⦗
  - `shell.r.filled`: ⦘
  - `chevron.l.closed`: ⦉
  - `chevron.r.closed`: ⦊
  - `corner.l.t`: ⌜
  - `corner.l.b`: ⌞
  - `corner.r.t`: ⌝
  - `corner.r.b`: ⌟
  - `bag.l`: ⟅
  - `bag.r`: ⟆
  - `mustache.l`: ⎰
  - `mustache.r`: ⎱

- Punctuation
  - `comma.inv`: ⸲
  - `comma.rev`: ⹁
  - `interrobang.inv`: ⸘
  - `semi.inv`: ⸵
  - `slash.o`: ⊘
  - `ast.op.o`: ⊛
  - `dot.o`: ⊙
  - `dot.o.big`: ⨀
  - `colon.currency`: ₡
  - `permyriad`: ‱

- Arithmetic
  - `plus.o`: ⊕
  - `plus.o.arrow`: ⟴
  - `plus.o.big`: ⨁
  - `plus.o.l`: ⨭
  - `plus.o.r`: ⨮
  - `minus.o`: ⊖
  - `div.o`: ⨸
  - `div.slanted.o`: ⦼
  - `times.o`: ⊗
  - `times.o.big`: ⨂
  - `times.o.l`: ⨴
  - `times.o.r`: ⨵
  - `times.o.hat`: ⨶

- Function and category theory
  - `compose.o`: ⊚
  - `convolve.o`: ⊛

- Geometry
  - `angle.obtuse`: ⦦
  - `angle.azimuth`: ⍼
  - `angle.right.arc.dot`: ⦝
  - `angzarr`: ⍼

- Shapes
  - `bullet.op`: ∙
  - `bullet.o`: ⦿
  - `bullet.stroked`: ◦
  - `bullet.stroked.o`: ⦾
  - `bullet.hole`: ◘
  - `bullet.hyph`: ⁃
  - `bullet.tri`: ‣
  - `bullet.l`: ⁌
  - `bullet.r`: ⁍

- Miscellaneous
  - `cc`: 🅭
  - `cc.by`: 🅯
  - `cc.nc`: 🄏
  - `cc.nd`: ⊜
  - `cc.public`: 🅮
  - `cc.sa`: 🄎
  - `cc.zero`: 🄍

- Currency
  - `afghani`: ؋
  - `baht`: ฿
  - `cedi`: ₵
  - `cent`: ¢
  - `currency`: ¤
  - `dong`: ₫
  - `dorome`: ߾
  - `dram`: ֏
  - `guarani`: ₲
  - `hryvnia`: ₴
  - `kip`: ₭
  - `lari`: ₾
  - `manat`: ₼
  - `naira`: ₦
  - `pataca`: $
  - `riel`: ៛
  - `peso.philippine`: ₱
  - `rupee.indian`: ₹
  - `rupee.generic`: ₨
  - `rupee.tamil`: ௹
  - `rupee.wancho`: 𞋿
  - `shekel`: ₪
  - `som`: ⃀
  - `taka`: ৳
  - `taman`: ߿
  - `tenge`: ₸
  - `togrog`: ₮
  - `yuan`: ¥

- Miscellaneous Technical
  - `smile`: ⌣
  - `frown`: ⌢
  - `power.standby`: ⏻
  - `power.on`: ⏽
  - `power.off`: ⭘
  - `power.on.off`: ⏼
  - `power.sleep`: ⏾

- Cyrillic
  - `sha`: ш
  - `Sha`: Ш

- Greek
  - `digamma`: ϝ
  - `epsilon.alt.rev`: ϶
  - `iota.inv`: ℩
  - `Digamma`: Ϝ
  - `Theta.alt`: ϴ

- Astronomical
  - `earth`: 🜨
  - `earth.alt`: ♁
  - `jupiter`: ♃
  - `mars`: ♂
  - `mercury`: ☿
  - `neptune`: ♆
  - `neptune.alt`: ⯉
  - `saturn`: ♄
  - `sun`: ☉
  - `uranus`: ⛢
  - `uranus.alt`: ♅
  - `venus`: ♀

- Gender
  - `gender.female`: ♀
  - `gender.female.double`: ⚢
  - `gender.female.male`: ⚤
  - `gender.intersex`: ⚥
  - `gender.male`: ♂
  - `gender.male.double`: ⚣
  - `gender.male.female`: ⚤
  - `gender.male.stroke`: ⚦
  - `gender.male.stroke.t`: ⚨
  - `gender.male.stroke.r`: ⚩
  - `gender.neuter`: ⚲
  - `gender.trans`: ⚧

### New in `emoji`

- `donkey`: 🫏
- `face.shaking`: 🫨
- `faith.khanda`: 🪯
- `flower.hyacinth`: 🪻
- `flute`: 🪈
- `ginger`: 🫚
- `goose`: 🪿
- `hairpick`: 🪮
- `hand.pushing.l`: 🫷
- `hand.pushing.r`: 🫸
- `handfan`: 🪭
- `heart.gray`: 🩶
- `heart.lightblue`: 🩵
- `heart.pink`: 🩷
- `jellyfish`: 🪼
- `maracas`: 🪇
- `moose`: 🫎
- `peapod`: 🫛
- `wing`: 🪽
- `wireless`: 🛜
- `dancing.bunny.men`: 👯‍♂
- `dancing.bunny.women`: 👯‍♀

### Deprecated

- Hebrew
  - `alef`, use `aleph` instead
  - `bet`, use `beth` instead
  - `gimmel`, use `gimel` instead
  - `dalet`, use `daleth` instead
  - `shin`, perhaps use `sha` instead

- CJK compatibility
  - `ast.small`, use ﹡ or `\u{fe61}` instead
  - `plus.small`, use ﹢ or `\u{fe62}` instead
  - `eq.small`, use ﹦ or `\u{fe66}` instead
  - `gt.small`, use ﹥ or `\u{fe65}` instead
  - `lt.small`, use ﹤ or `\u{fe64}` instead

- `circle` -> `o` for mathematical operators
  - `bar.v.circle`, use `bar.v.o` instead
  - `ast.circle`, use `convolve.o` or `ast.op.o` instead
  - `backslash.circle`, use `backslash.o` instead
  - `dash.circle`, use `dash.o` instead
  - `dot.circle`, use `dot.o` instead
  - `dot.circle.big`, use `dot.o.big` instead
  - `plus.circle`, use `plus.o` instead
  - `plus.circle.arrow`, use `plus.o.arrow` instead
  - `plus.circle.big`, use `plus.o.big` instead
  - `minus.circle`, use `minus.o` instead
  - `div.circle`, use `div.o` instead
  - `times.circle`, use `times.o` instead
  - `times.circle.big`, use `times.o.big` instead
  - `eq.circle`, use `eq.o` instead
  - `gt.circle`, use `gt.o` instead
  - `lt.circle`, use `lt.o` instead
  - `parallel.circle`, use `parallel.o` instead
  - `perp.circle`, use `perp.o` instead
  - `circle.nested`, use `compose.o` instead

- `angle` -> `chevron`
  - `angle.l` and `angle.r` to `chevron.l` and `chevron.r`, respectively
  - `quote.angle` to `quote.chevron`

- `double` -> `stroked` for double-struck delimiters
  - `paren.double`, use `paren.stroked` instead
  - `bracket.double`, use `bracket.stroked` instead
  - `shell.double`, use `shell.stroked` instead

- Other
  - `diff`, use `partial` instead
  - `angle.spheric.top`, use `angle.spheric.t` instead
  - `angle.right.sq`, use `angle.right.square` instead
  - `planck.reduce`, use `planck` instead
  - `angle.oblique`, use `angle.obtuse` instead
  - `kai`, use ϗ or `\u{3d7}` instead
  - `Kai`, use Ϗ or `\u{3c5}` instead
  - `franc`, because the symbol was never used in practice

## Version 0.1.1 (February 5, 2025)
Brings back `angstrom`, but uses U+00C5 LATIN CAPITAL LETTER A WITH RING ABOVE, which is the one that should be used in place of the deprecated U+212B ANGSTROM SIGN.

## Version 0.1.0 (February 4, 2025)
_As this is the first release of codex, the symbol changes are relative to Typst 0.12.0._
- New
  - `inter`, `inter.and`, `inter.big`, `inter.dot`, `inter.double`, `inter.sq`, `inter.sq.big`, `inter.sq.double`, `integral.inter`
  - `asymp`, `asymp.not`
  - `mapsto`, `mapsto.long`
  - `divides.not.rev`, `divides.struck`
  - `interleave`, `interleave.big`, `interleave.struck`
  - `eq.triple.not`, `eq.dots`, `eq.dots.down`, `eq.dots.up`
  - `smt`, `smt.eq`, `lat`, `lat.eq`
  - `colon.tri`, `colon.tri.op`
  - `dagger.triple`, `dagger.l`, `dagger.r`, `dagger.inv`
  - `hourglass.stroked`, `hourglass.filled`
  - `die.six`, `die.five`, `die.four`, `die.three`, `die.two`, `die.one`
  - `errorbar.square.stroked`, `errorbar.square.filled`, `errorbar.diamond.stroked`, `errorbar.diamond.filled`, `errorbar.circle.stroked`, `errorbar.circle.filled`
  - `numero`
  - `Omega.inv`
- Renamed
  - `ohm.inv` to `Omega.inv`
- Changed codepoint
  - `angle.l.double` from `《` to `⟪`
  - `angle.r.double` from `》` to `⟫`
- Deprecated
  - `sect` and all its variants
  - `integral.sect`
- Removed
  - `degree.c`, `degree.f`, `ohm`, `ohm.inv`, `angstrom`, `kelvin`
