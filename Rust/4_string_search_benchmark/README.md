# Substring Search Benchmark

**Comparing the performance of different search method**

**Crate used:**
1. memchr
2. aho_corasick

This app currently does comparison between `native search`, `memchr crate` and `aho_corasick crate`, in searching for substrings in a randomly generated long text file (that has millions characters).

And the result... is not I was looking for, I might have to go back to this and change it, now it just seems like the native method is way better in most cases. Even with multiple query, aho-corasick does not out performs the rest, which seems weird...

Full output looks like this in my machine:
```console
==================== Finding all occurrance of a single string ====================
[native] Found 1 occurance of word "B14fNx8YUy" in 321 milliseconds
[memmem] Found 1 occurance of word "B14fNx8YUy" in 383 milliseconds
[aho-corasick] Found 1 occurance of words "["B14fNx8YUy"]" in 1.1338888 seconds

==================== Finding left most occurrance of three strings ====================
[native] Found 1 occurance of words "["tHGp3FEBx7", "skJCoFTN93", "fNx8YU"]" in 31 microseconds
[memmem] Found 1 occurance of words "["tHGp3FEBx7", "skJCoFTN93", "fNx8YU"]" in 53 microseconds
[aho-corasick] Found leftmost match "tHGp3FEBx7" at position 6717 in 1 milliseconds

==================== Finding all occurrance of three strings ====================
[native] Found 3 occurance of words "["tHGp3FEBx7", "skJCoFTN93", "fNx8YU"]" in 885 milliseconds
[memmem] Found 3 occurance of words "["tHGp3FEBx7", "skJCoFTN93", "fNx8YU"]" in 1.1517998 seconds
[aho-corasick] Found 3 occurance of words "["tHGp3FEBx7", "skJCoFTN93", "fNx8YU"]" in 11.73958 seconds

==================== Finding left most occurrance of ten strings ====================
[native] Found 1 occurance of words "["Hq6JYTKV", "7SC4yc4Vt", "58PjWm", "3tRoIEot", "97OUJh", "i6xiuKP", "fbkB0PB", "RUfmE4la", "H46v", "A_NON_EXISTING_WORD"]" in 56 microseconds
[memmem] Found 1 occurance of words "["Hq6JYTKV", "7SC4yc4Vt", "58PjWm", "3tRoIEot", "97OUJh", "i6xiuKP", "fbkB0PB", "RUfmE4la", "H46v", "A_NON_EXISTING_WORD"]" in 128 microseconds
[aho-corasick] Found leftmost match "3tRoIEot" at position 13 in 6 microseconds

==================== Finding all occurrance of ten strings ====================
[native] Found 15 occurance of words "["Hq6JYTKV", "7SC4yc4Vt", "58PjWm", "3tRoIEot", "97OUJh", "i6xiuKP", "fbkB0PB", "RUfmE4la", "H46v", "A_NON_EXISTING_WORD"]" in 3.0197163 seconds
[memmem] Found 15 occurance of words "["Hq6JYTKV", "7SC4yc4Vt", "58PjWm", "3tRoIEot", "97OUJh", "i6xiuKP", "fbkB0PB", "RUfmE4la", "H46v", "A_NON_EXISTING_WORD"]" in 3.681563 seconds
[aho-corasick] Found 15 occurance of words "["Hq6JYTKV", "7SC4yc4Vt", "58PjWm", "3tRoIEot", "97OUJh", "i6xiuKP", "fbkB0PB", "RUfmE4la", "H46v", "A_NON_EXISTING_WORD"]" in 12.189917 seconds
```