# Minecraft Image/GIF Creator

I made this project after seeing a post on Reddit, about images rendered in vanilla, and found out it uses text_display entities.
So I wanted to give my own try at it.

Then I ended up making this which allows for GIFs in Minecraft too!

## Supported file types
This project uses [image-rs](https://github.com/image-rs/image), so any file format they support will be supported here.

## Usage
Download and run the exe, or build it for Linux. Then select the image or GIF, apply the filters you want to it, then click Create Datapack and select the directory of the datapacks folder.

## Building
### Make sure you have Rust installed if not go [here](https://www.rust-lang.org/learn/get-started)

Run these commands in a terminal
```
git clone https://github.com/coolguy1842/ImageCreator.git
cd ImageCreator/src-tauri
cargo install tauri-cli
cargo tauri build
```

Then the executable will be in `basedir/src-tauri/target/release`
