# Load File

We need load the `.obj` file to RAM, and this type of file has not header about length of data.

So that we should use the buffer that is growable.

Different from [t1gars](https://github.com/zaiic/t1gars), we should use `Vec` here.
