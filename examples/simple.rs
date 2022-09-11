use arche;

struct MenuState;
struct MainState;

impl arche::State for MenuState {
    fn handle_events(&mut self, _ctx: &mut arche::Context) -> arche::Trans {
        todo!()
    }

    fn update(&mut self, _ctx: &mut arche::Context) {
        todo!()
    }

    fn draw(&mut self, _ctx: &mut arche::Context) {
        todo!()
    }
}

impl arche::core::state::State for MainState {
    fn handle_events(&mut self, _ctx: &mut arche::Context) -> arche::Trans {
        todo!()
    }

    fn update(&mut self, _ctx: &mut arche::Context) {
        todo!()
    }

    fn draw(&mut self, _ctx: &mut arche::Context) {
        todo!()
    }
}

fn main() {}
