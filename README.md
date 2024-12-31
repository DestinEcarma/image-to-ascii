# Image to Ascii

This is a simple rust program that converts an image to ascii art. It uses the [`image`](https://crates.io/crates/image) crate to read the image.

```
                fff
              fffffff
            fffffffffff
          fffffffffffffff
         tfffffffffffffffi
         tttfffffffffffiii
         tttttfffffffiiiii
         tttttttfffiiiiiii
         ttttttttiiiiiiiii
         ttttttttiiiiiiiii
         ttttttttiiiiiiiii
         ttttttttiiiiiiiii
         ttttttttiiiiiiiii
         ttttttttiiiiiiiii
         ttttttttiiiiiiiii
         ttttttttiiiiiiiii
       CCCCttttttiiiiiii;;;;
     CCCCCCCCttttiiiii;;;;;;;;
   CCCCCCCCCCCCttiii;;;;;;;;;;;;
 CCCCCCCCCCCCCCCCi;;;;;;;;;;;;;;;;
 LLCCCCCCCCCCCCCf::;;;;;;;;;;;;;,,
 LLLLCCCCCCCCCfff::::;;;;;;;;;,,,,
 LLLLLLCCCCCfffff::::::;;;;;,,,,,,
 LLLLLLLLCfffffff::::::::;,,,,,,,,
 LLLLLLLLffffffff:::::::::,,,,,,,,
 LLLLLLLLffffffff:::::::::,,,,,,,,
 LLLLLLLLffffffff:::::::::,,,,,,,,
 LLLLLLLLffffffff:::::::::,,,,,,,,
 LLLLLLLLffffffff:::::::::,,,,,,,,
 LLLLLLLLffffffff:::::::::,,,,,,,,
 LLLLLLLLffffffff:::::::::,,,,,,,,
  LLLLLLLfffffff   :::::::,,,,,,,
    LLLLLfffff       :::::,,,,,
      LLLfff           :::,,,
        Lf               :,
```

## Usage

To run the program, use the following command:

```sh
cargo run --release -- <image_path>
```

The program will output the ascii art to the terminal.

To see more options, use the `-h` flag:

```sh
cargo run --release -- -h
```
