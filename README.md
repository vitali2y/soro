# soro - Simple Online RadiO cli player

## Usage
If you have *Rust* [installed](https://www.rust-lang.org/tools/install) then just run:
```
echo "https://online.melodiafm.ua/MelodiaFM https://online.kissfm.ua/KissFM https://online.hitfm.ua/HitFM" | cargo r --
```
Or:
```
cargo b && echo "https://online.melodiafm.ua/MelodiaFM https://online.kissfm.ua/KissFM https://online.hitfm.ua/HitFM" | ./target/debug/soro
```

Short list of radio stations might be found [here](https://gist.github.com/vitali2y/a8d88dfc82b823e3d15e3e433604d33f).

## App Control
Pressing *\<Enter>* simply switches to the next track in the loop, and pressing *q* exits the app.
