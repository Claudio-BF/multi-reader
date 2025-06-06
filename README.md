A simple program made to learn Rust.
It can read a txt file sentence by sentence and translate it into any language of your choosing.

In order to use, you first need to create a file with read and write permissions enabled:

/usr/local/share/multi-reader/progress.txt

If you want to change this location you may edit the PROGRESS_FILE in the source code.
This will allow multireader to remember your progress on books.
Now run:

multi-reader -h

And then follow the instructions.

deps:
component
regex
rust-translate
tokio

I recommend "legally obtaining" a book in another langauge and converting it to a txt file using one of the many online tools available.
