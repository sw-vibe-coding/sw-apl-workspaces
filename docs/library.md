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

sw-apl takes a library from a directory, two ways:

- **`--lib 2=PATH/ws,EXTENDED`**: a numbered library from any
  directory, read-only, with the same file naming and modes lines as
  library 1; the flag may be given again for another library.
  `sw-apl-server` takes the same flag, so the 2741 terminal and the
  local browser page see the library too. The scripts here run
  sw-apl with `--lib 2=ws,EXTENDED`, and `just apl` starts a session
  with it.
- **`sw-apl.toml`**, the same as a file: `--config FILE`, else
  `./sw-apl.toml` where sw-apl runs, else `sw-apl/config.toml` in the
  user's configuration directory. A relative path is from the file,
  and a flag wins over the file.

  ```toml
  [[library]]
  number = 2
  name = "EXTENDED"
  path = "/path/to/sw-apl-workspaces/ws"
  ```

A directory that is not there is said so on stderr and lists
nothing. Only library 0 is written to.

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

sw-apl's page reads its list of libraries from `libraries.json`
beside it (number, name, and the URL of an index), fetches each index
and every file it names before the session starts, with a timeout,
and hands them to the session as its own library 1 is handed to it. A
library that will not load is said so on the paper and the session
starts without it. The published demo's library 2 is this
repository's `ws/library.json` read through raw.githubusercontent.com,
which serves across origins; it moves to this repository's GitHub
Pages, which answers with `Access-Control-Allow-Origin: *`, once they
are enabled for the repository.

`scripts/gen-index.sh` writes the index from the files and their
modes lines; it is tracked, so what is served is what was committed,
and `just gates` fails when it is stale. The page reads the `file` of
each entry in `workspaces` and takes the modes from the file itself;
`name` and `modes` in the index are for a reader.

## What this repository needs from sw-apl

| Need | sw-apl step | Until then |
|---|---|---|
| `)LIB 2` and `)LOAD 2 NAME` from a configured directory | `library-config` | Landed; the samples use it |
| `)LIBS` naming this library | `library-config` | Landed |
| Libraries by URL in the browser | `browser-libraries` | Landed, through raw.githubusercontent.com; GitHub Pages for this repository still to be enabled |
| `)DIALECT` and `)HELP`, which COURSE's first lesson shows | `dialect`, `help` | Landed |
| A system command, or an error, in the reply to `⎕` | `quad-input-commands` | Landed; lesson 1 uses it |

A gap in the interpreter found while writing a workspace -- a
primitive that misbehaves, a reply that is wrong -- is reported to
sw-apl's plan with the sample that shows it, and never worked around
in a way that makes the workspace not-APL.
