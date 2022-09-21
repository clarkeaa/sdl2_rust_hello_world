use sdl2::sys::*;
use std::{ffi::CString, mem::transmute};

fn main() {
    unsafe {
        if SDL_Init(SDL_INIT_VIDEO) != 0 {
            panic!("SDL_Init failed.");
        }
        let title = CString::new("Hellow").unwrap();
        let window = SDL_CreateWindow(
            title.as_ptr(),
            SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            640,
            480,
            0,
        );
        if window.is_null() {
            panic!("SDL_CreateWindow failed.")
        }
        let renderer = SDL_CreateRenderer(
            window,
            -1,
            SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32
                | SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC as u32,
        );
        if renderer.is_null() {
            panic!("SDL_CreateRenderer failed.");
        }

        let mut running = true;
        let mut event: SDL_Event = core::mem::zeroed();
        while running {
            while SDL_PollEvent(&mut event) == 1 {
                let event_type: SDL_EventType = transmute(event.type_);
                match event_type {
                    SDL_EventType::SDL_QUIT => {
                        running = false;
                    }
                    _ => (),
                };
            }
            SDL_SetRenderDrawColor(renderer, 200, 20, 20, 255);
            SDL_RenderClear(renderer);
            SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
            SDL_RenderDrawLine(renderer, 0, 0, 100, 100);
            SDL_RenderPresent(renderer);
        }
        SDL_DestroyRenderer(renderer);
        SDL_DestroyWindow(window);
        SDL_Quit();
    }
}
