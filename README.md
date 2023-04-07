# Splitty

User-friendly library to split some generic computational work between threads.

Splitty waits a computational work threshold and separate the data between threads to handle when threshold exceeded.

The threshold is a length of the input Vec<T> and defined as a constant.
