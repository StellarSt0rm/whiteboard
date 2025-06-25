# <p align="center">Whiteboard</p>
So, I was watching a YouTube video, and they were drawing on the screen to highlight things, and I thought "Damn, that's so cool, let's put all my other projects to rest and work on this!"

This project is very simple, it will not have an erasing feature because of how the strokes are drawn (It's literally just an SVG, kinda), and it's not supposed to be an app to actuall draw anything, like i said, this was inspired from those in YouTube videos and i doubt you need further erasing than a simple Undo button.

> [!WARNING]
> I will never compile this for windows, so dont ask me. You can build it yourself.
> Nor will I build it for MacOS.

## Installing
To install Whiteboard you will need Nix, then just run: `nix develop --command cargo build --release` \
And finally move the newly created file at `target/build/whiteboard` to your preferred `bin` folder (`.local/bin` for example)

Or you can find a pre-built binary on the Releases tab, although for perfomance I recommend you build it yourself so it's optimized for your device.

## Features (100%)
- [X] Interface design
- [X] Shortcuts
- [X] Drawing functionality
    - [X] Basic drawing
    - [X] Colors + Color picker
    - [X] Stroke width
    - [X] Clearing the screen
