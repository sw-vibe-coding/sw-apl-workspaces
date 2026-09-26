# Citations

Every source this repository's workspaces and documents refer to.
None of them is copied: the workspaces here are written from scratch,
and a historical workspace is named only to say what a function is in
the spirit of. Where a formula or an approximation is taken from a
book, the workspace's comment names it and this file lists it.

bitsavers refuses scripted downloads from its main host; its mirrors
carry the same files, for example
`bitsavers.informatik.uni-stuttgart.de` and
`bitsavers.trailing-edge.com`.

## The language

- **APL\360 User's Manual**, IBM, August 1968, and its March 1970
  edition (GH20-0683-1). What (A) '70 is: the primitives, the del
  editor, the system commands, the trouble reports. Its description
  of library 1 -- APLCOURSE with `TEACH` and `EASYDRILL`, TYPEDRILL,
  PLOTFORMAT, ADVANCEDEX -- is the only thing known here about those
  workspaces, and it is used as a statement of purpose, not as a
  source of code.
  <https://www.bitsavers.org/pdf/ibm/apl/APL_360_Users_Manual_Aug68.pdf>
  <https://www.bitsavers.org/pdf/ibm/apl/GH20-0683-1_APL_360_Users_Manual_Mar70.pdf>
- **IBM 5110 APL Reference Manual**, SA21-9303-0, IBM, December 1977.
  What (B) '75 is: execute, format, the system variables and
  functions, the 64-column screen that sets the width every printed
  line here fits.
  <https://www.bitsavers.org/pdf/ibm/5110/SA21-9303-0_IBM_5110_APL_Reference_Manual_Dec1977.pdf>
- **APLSV User's Guide**, SH20-1460-1, IBM, March 1975. The APL the
  5100 family's derives from; its operations guide describes the
  instructional workspaces the 5100 inherited.
  <https://www.bitsavers.org/pdf/ibm/apl/>
- **sw-apl**, the interpreter, beside this repository: its
  `docs/language.md` (what each mode has), `docs/workspaces.md` (the
  file format, the modes line, the naming of files),
  `docs/aplcourse-how-to.md` (why converted historical material stays
  in `work/`). <https://github.com/sw-vibe-coding/sw-apl>

## The historical workspaces, by name only

- **Try MTS, APL language features**: APL\MTS running in a browser,
  with `)LOAD 1 APLCOURSE` and the list of its functions. Consulted
  for what APLCOURSE offered a learner; nothing from it is here.
  <https://try-mts.com/apl-language-features/>
- **APL History Collection**, Software Preservation Group, Computer
  History Museum: lists the APL PLUS STATPAK manual (Scientific Time
  Sharing Corporation, 1969) and the SHARP APL Utility Library
  Catalogue, whose statistical functions are said to descend from
  IBM's STATPACK. Consulted for the kind of functions a statistics
  package of the period offered. STATS here is written from the
  definitions of the statistics, not from either.
  <https://softwarepreservation.computerhistory.org/apl/>

## Mathematics

Each with the book or paper it comes from; the workspace's comment
names the function.

- **Rob J. Hyndman and Yanan Fan, "Sample Quantiles in Statistical
  Packages"**, The American Statistician 50(4), 1996, 361-365. The
  nine definitions of a sample quantile; STATS's `QUANTILE` is their
  type 7, linear interpolation at position 1+p(n-1), the default of
  most packages. <https://doi.org/10.2307/2684934>
