# Project Regex Engine Completed
The objective of this regex engine project is to find, implement and finish an idea that I have been wishing to start. The main concepts that I wanted and learned was the concept of building cache friendly architectures and designs.

The sources that I used as reference were:
- [ThompsonNFA](https://swtch.com/~rsc/regexp/regexp1.html)
- [Benchmark of Regex Libraries](http://lh3lh3.users.sourceforge.net/reb.shtml)

The current feature set does not implement the expected entire feature set. It only implements the '|', '?', '+', '\*', and concat operations.

I made the regex engine cache friendly, by only using vectors in rust. The Thompson NFA, that I referenced uses linked lists and malloc() operations on the State struct so it could lead to high cache miss rates when the memory becomes fragmented due to high use. 

# Future ideas to be implemented
- Benchmark with the ThompsonNFA implementation such that cache miss rates happen.
- Implement all regex features.
- Implement the extended regex features.
- Implement the perl extended regex features.