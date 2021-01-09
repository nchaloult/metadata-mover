# Metadata Mover

A simple utility to add "Artist" metadata tags to MP3 files based on their file names.

## Example Usage

```sh
$ ls -lah
...
-rw-r--r--@  1 you  staff   3.2M Jan  9 10:19 The Japanese House - Saw You In A Dream.mp3
...

$ metadatamover "The Japanese House - Saw You In A Dream.mp3"
All finished!

$ ls -lah
...
-rw-r--r--@  1 you  staff   3.2M Jan  9 10:19 Saw You In A Dream.mp3 // With "The Japanese House" written to the "Artist" MP3 metadata tag
...
```

## Motivation

A friend of mine stores their music locally as MP3 files, and names all of them according to the template: "(artist) - (song title).mp3". They started using a new media player which takes advantage of MP3 metadata, but their files didn't have any of that metadata filled out. They wanted a program that could strip the artist from hundreds of MP3 file names and fill out their "Artist" metadata tags. I thought this was an interesting chance to learn more about Rust and how MP3 files work.
