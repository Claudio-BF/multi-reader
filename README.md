A simple program made to help you read books in other languages.
It can read a txt file sentence by sentence and translate it into any language of your choosing.

In order to use, you need to create a file with read and write permissions enabled, I recommend:

/usr/local/share/multi-reader/progress.txt

If you want to change this location you may edit the PROGRESS_FILE in the source code.
This will allow multi-reader to remember your progress on books.
Now run:

multi-reader -h

And then follow the instructions.

deps:
component
regex
rust-translate
tokio

I recommend downloading a book in another language and converting it to a txt file using one of the many online tools available.
