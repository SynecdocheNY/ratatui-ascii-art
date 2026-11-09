# ASCII Art for Ratatui

Dynamically rendered, colorful ASCII art using the `image` crate.

## How it Works

The `AsciiArt` widget is a wrapper around a Ratatui `Paragraph`. Each ASCII character gets its own `Span` that describes its color, and the spans are assembled into `Line`s before rendering.

## Performance
The algorithm is reasonably efficient, but can be slow in some cases. If you run into performance bottlenecks:
- Try building with optimizations (`cargo build --release`). They help a *lot* for this widget.
- Consider using lower-resolution images. Most of the detail gets lost anyways.

Feel free to send me an issue if the problem persists.

## Work in Progress
- Implement `AsciiArtFitting::Fit` algorithm.
- Add colorless option. Maybe a different widget altogether?

## Inspiration

Much of the core brightness & color logic was adapted from the `rascii_art` crate.