# ttyrec

**Disclaimer: This is a prototype.**

This is a little tool to create video or gif of a tty.

# Dependencies

- rustc (_>1.12.0_)
- ImageMagick (_convert_)
- xwd

# Usage

```bash
./ttyrec -h
ttyrec 0.1
Create gif from tty input

USAGE:
    ttyrec [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --video      Add a tty.mp4

OPTIONS:
    -o, --out-delay <out-delay>      Change delay between 2 frame for the output file
    -s, --snap-delay <snap-delay>    Change delay between 2 snapshot
```

![ttygif](tty.gif)

# License

```text
DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
        Version 2, December 2004

Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>

Everyone is permitted to copy and distribute verbatim or modified
copies of this license document, and changing it is allowed as long
as the name is changed.

DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

0\. You just DO WHAT THE FUCK YOU WANT TO.
```
