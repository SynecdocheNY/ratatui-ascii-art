# ASCII Art for [Ratatui](https://ratatui.rs/)

Dynamically rendered, colorful ASCII art using the [`image`](https://crates.io/crates/image) crate.

## How it Works

The `AsciiArt` widget is a wrapper around a Ratatui `Paragraph`. Each ASCII character gets its own `Span` that describes its color, and the spans are assembled into `Line`s before rendering.

## Performance
The algorithm is reasonably efficient, but can be slow in some cases. If you run into performance bottlenecks:
- Try building with optimizations (`cargo build --release`). They help a *lot* for this widget.
- Consider using lower-resolution images. Most of the detail gets lost anyways.

Feel free to send me an issue if the problem persists.

## Work in Progress
- [x] Refactor builder methods into shared trait
- [ ] Enforce animation FPS (Steal FPS Widget from [`ratatui-core`](https://github.com/ratatui/ratatui/blob/main/examples/apps/colors-rgb/src/main.rs)).
- [ ] Add Ratatui styling support (`Stylize` trait).
- [ ] Example renders in README.
- [ ] Code examples (module and in README).
- [ ] Minimize dependcies and their features.
- [ ] Optimize loading large GIF files (for debug builds).

## Inspiration

Much of the core luminance & color logic was adapted from the [`rascii_art`](https://crates.io/crates/rascii_art) crate.
