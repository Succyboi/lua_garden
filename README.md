# lua_garden

lua_garden is a signal processing programming playground that is scripted in Lua.

It is currently in its infancy and may be lacking features that would allow it to reach its full potential.

This project was initially kickstarted by my dissatisfaction with the reliance of signal processing on visual scripting. I wanted a tool that would allow me to quickly iterate upon my ideas in a text-based programming language. Hence the choice for lua, which is interpretable, which means no compilation downtime.

The end goal of this project is to have a tool that facilitates the development of audio plug-ins and interactive sonic performance.

## Building

lua_garden can be built same as any other [NIH-plug](https://github.com/robbert-vdh/nih-plug?tab=readme-ov-file#building) plugin.

## Roadmap

The following is a list of features that I'd like to implement.

- [ ] Parameters
  - [ ] Plugin parameter binding

- [ ] Standard library
  - [ ] Delay-line
  - [ ] SVF
  - [ ] RMS

- [ ] Workspace auto-reload
- [ ] Gapless reloading
- [ ] Midi

## Libraries

lua_garden is written in rust, using the following libraries:
TODO

lua_garden's Lua standard library was developed by this repository's contributors, derived from [andie](https://stupidplusplus.itch.io/andie)'s audio library Bloop and inspired by various snippets found on [musicdsp.org](https://www.musicdsp.org).