title = Early thoughts on the Zig programming language (0.8.0)
date = 27-04-2021


What's interesting:

- It's a somewhat simple language backed by powerful constructs. It has optionals, nullable types, error handling is done through error sets (no exceptions here), etc.
- Powerful compilation time (Comptime) constructs.
- Standard library is written in Zig and since the language is simple you'll probably understand what is being done there.
- Aims to be a modern C replacement in its principles.
- Out of the box cross compilation. The compiler is also able to compile C.
- Memory allocation is explicit, multiple allocation strategies available. Good for platforms where memory is a major concern.
- Is able to import C headers, bindings are automatically generated. Making bindings for Rust might be time intensive, altough in theory you'll have a safer interface at the end.
- Build system is built in the compiler, build scripts are written in Zig.

What's not there:

- The documentation in general is still a little bit green, this is actively being worked on. In most occasions you'll end up looking at the standard library or other projects on how to do something or asking in the official Discord.
- Package manager. There's a few unofficial versions of a package manager but there are plans to have one. Given the built in build system and how packages work, it's relatively easy to just add another project into yours but a package manager would be nice. This is one of the reasons why I'm not actively using the language, if you have your mind set on doing *everything* yourself, go ahead, but for my side projects, I don't worry too much about dependencies and like to add and remove dependencies as they are needed (Rust is currently serving this purpose).


I'm definitely looking forward for the package manager (and for more libraries to exist) and extremely excited for the next releases that promise faster compilation times (it's already pretty fast!)

You can check it at [Zig](https://ziglang.org/learn/overview/) and if you don't mind some scattered notes I took while reading the documentation (and [Zig Learn](https://ziglearn.org]) you can check this [programming-notes/zig](https://github.com/goncalopalaio/programming-notes/tree/master/zig) and the websocket server I wrote that is currently super broken [websockets-zig](https://github.com/goncalopalaio/websockets-zig)