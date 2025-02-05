# Changelog

# Version 0.1.1 (February 5, 2025)
Brings back `angstrom`, but uses U+00C5 LATIN CAPITAL LETTER A WITH RING ABOVE, which is the one that should be used in place of the deprecated U+212B ANGSTROM SIGN.

# Version 0.1.0 (February 4, 2025)
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
  - `degree.c`, `degree.f`, `ohm.inv`, `angstrom`, `kelvin`
