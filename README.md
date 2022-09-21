# SDL2 Rust Hello World

There are a few gotchas when setting up Rust to use SDL2 so I thought I'd share
my setup.

Gotchas:
1) Using the "bundled" feature of the sdl2 package makes getting the correct
   SDL2 verion easier.
1) It wasn't clear to me how to make a SDL_Event, `core::mem::zeroed` seemed the
   best option.
1) For some reason the event will give you a u32 for the the type, but the
   library defines those in an enum, so I had to use `transmute`.
