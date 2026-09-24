# The library

How sw-apl reaches this repository. sw-apl numbers its libraries as
APL\360 did: 0 is yours, where `)SAVE` writes; 1 is what sw-apl ships;
and this repository is 2, which `)LIBS` reports as EXTENDED.

```
      )LIBS
0 USER
1 CORE
2 EXTENDED
      )LIB 2
COURSE
DRILL
      )LOAD 2 COURSE
SAVED  12.00.00 09/24/26
      DESCRIBE
```

`)LIB 2` lists what in this library runs in the mode the session is
in, and `)LOAD 2 NAME` loads it; a (B)-only workspace is not listed
in (A), and the other way round, by its modes line.

## At the terminal and the service

sw-apl takes a library from a directory. Three ways, in the order
they exist:

- **Today, the shim.** `scripts/lib-shim.sh` makes `target/shim/`
  with this repository's `ws/` as its library 1, and
  `sw-apl --library target/shim` lists and loads these workspaces as
  `)LIB 1`. This is how the samples run until the next way exists.
- **`--lib 2=PATH/ws,EXTENDED`**, once sw-apl's library configuration
  lands: a numbered library from any directory, read-only, with the
  same file naming and modes lines as library 1. `sw-apl-server`
  takes the same flag, so the 2741 terminal and the local browser
  page see the library too.
- **`sw-apl.toml`**, the same as a file, in the directory sw-apl runs
  from or the user's configuration directory, so the flag need not
  be typed:

  ```toml
  [[library]]
  number = 2
  name = "EXTENDED"
  path = "/path/to/sw-apl-workspaces/ws"
  ```

The flag and the file are sw-apl's to settle (its `docs/plan.md`,
Phase 10, `library-config`); the forms above are what this repository
is written against, and the README's "Use it" is updated when they
land.

## In the browser

The published demo runs sw-apl in the page, with no service behind
it, so a library there is fetched from a URL. This repository
publishes `ws/` on GitHub Pages, and beside the workspaces an index,
`ws/library.json`:

```json
{"name": "EXTENDED",
 "workspaces": [
  {"name": "COURSE", "file": "COURSE.apl.ws", "modes": "(A)(B)"},
  {"name": "DRILL", "file": "DRILL.a-70.apl.ws", "modes": "(A)"},
  {"name": "DRILL", "file": "DRILL.b-75.apl.ws", "modes": "(B)"}
 ]}
```

sw-apl's page reads its list of libraries from a file beside it,
fetches each index and the files it names before the session starts,
and hands them to the session as its own library 1 is handed to it.
Pages answers every request with `Access-Control-Allow-Origin: *`,
which is what lets a page on another origin fetch them.

`scripts/gen-index.sh` writes the index from the files and their
modes lines; it is tracked, so what is served is what was committed,
and `just gates` fails when it is stale. The field names are this
repository's proposal until sw-apl's `browser-libraries` step settles
them; changing them is that script alone.

## What this repository needs from sw-apl

| Need | sw-apl step | Until then |
|---|---|---|
| `)LIB 2` and `)LOAD 2 NAME` from a configured directory | `library-config` | The shim, and `)LOAD 1` in the samples |
| `)LIBS` naming this library | `library-config` | Nothing to do here |
| Libraries by URL in the browser | `browser-libraries` | `library.json` and Pages are ready |
| `)DIALECT` and `)HELP`, which COURSE's first lesson shows | `dialect`, `help` | The lesson's text is checked against sw-apl's replies when they land |

A gap in the interpreter found while writing a workspace -- a
primitive that misbehaves, a reply that is wrong -- is reported to
sw-apl's plan with the sample that shows it, and never worked around
in a way that makes the workspace not-APL.
