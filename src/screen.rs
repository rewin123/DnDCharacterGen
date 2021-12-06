
use eframe::{egui, epi};
use eframe::egui::CtxRef;
use eframe::epi::Frame;

pub enum ScreenResult {
    Ok,
    NextScreen(Box<dyn Screen>),
    Back,
    ClearScreenStack(Box<dyn Screen>)
}


pub trait Screen {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) -> ScreenResult;
}

pub struct ScreenStack {
    screen_stack : Vec<Box<dyn Screen>>
}

impl ScreenStack {
    pub fn new(start_screen : Box<dyn Screen>) -> Self {
        Self {
            screen_stack : vec![start_screen]
        }
    }
}

impl Screen for ScreenStack {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) -> ScreenResult {

        let stack_size = self.screen_stack.len();
        let screen_res = self.screen_stack[stack_size - 1].update(ctx, frame);

        match screen_res {
            ScreenResult::Ok => {

            }
            ScreenResult::NextScreen(next_screen) => {
                self.screen_stack.push(next_screen);
                println!("New screen in stack");
            }
            ScreenResult::Back => {
                if stack_size > 1 {
                    self.screen_stack.remove(stack_size - 1);
                }
            }
            ScreenResult::ClearScreenStack(next_screen) => {
                self.screen_stack.clear();
                self.screen_stack.push(next_screen);
            }
        }

        ScreenResult::Ok
    }
}