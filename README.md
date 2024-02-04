# head

This is a `head` implementation with Rust. `head` is a command line program to display the beggining of a text file or piped data.

## Usage

To show the first 10 lines of the file:
```
cchead -n 10 file.txt
```

To show the first 10 bytes of the file:
```
cchead -c 10 file.txt
```

This also supports piping:
```
cat file.txt | cchead -c 25 
```
