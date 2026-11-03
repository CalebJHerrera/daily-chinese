# Daily-Chinese
I'm Chinese but don't know mandarin, so I've decided to try and create a lightweight app that lets me passively learn more characters.

I've also coded it in rust because I found its speed and tendency to abstract common processes interesting.

It was a lot harder than I thought.

At the bottom is an AI-assisted attributions paragraph (haven't written things like this before, really cool!)

# What this is

It's a small pop up that shows you a "level 1" (extremely common, based on the random_zh crate's level system I suppose) Chinese simplified-traditional character pair. Shows up every x minutes and changes every y hours (haven't thought too much about the UX yet). 

## Credits & Licensing

This repository contains both software code and bundled data, which are licensed separately:

* **Source Code:** The software code of this project is licensed under the GNU General Public License v3.0 — see the [LICENSE](./LICENSE.txt) file for details.
* **Dictionary Data:** This project utilizes and bundles the CC-CEDICT Chinese-English dictionary database, maintained by MDBG (https://mdbg.net). The dictionary data is distributed under the Creative Commons Attribution-ShareAlike 4.0 International License (CC BY-SA 4.0). The full text of this data license can be found in `LICENSE.txt`.

This project relies on the following open-source libraries:
- **Slint UI Framework** - Licensed under the GNU General Public License v3.0 ([GitHub](https://github.com/slint-ui/slint)).
