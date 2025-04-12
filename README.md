# Whiteboard
So, I was watching a YouTube video, and they were drawing on the screen to highlight things, and I thought "Damn, that's so cool, let's put all my other projects to rest and work on this!"

And as you can guess, I did... After like putting this one to rest for weeks after starting it. But I finally finished v1 and it mostly works, it has some weird things here and there though.

## Installing
To install Whiteboard you will need Nix, then just run: `nix-shell --run "cargo build --release"` \
And finally move the newly created file at `target/build/whiteboard` to your preferred `bin` folder (`.local/bin` for example)

Or you can find a pre-built binary on the Releases tab, although for perfomance I recommend you build it yourself so it's optimized for your device.

## Features (~50%)
- [X] Interface design
- [ ] Drawing functionality
    - [X] Basic drawing
    - [X] Colors + Color picker
    - [X] Stroke width
    - [ ] Erasing
        - [ ] Eraser tool
        - [ ] Clearing the full screen
