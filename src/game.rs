use event::{Event, EventReceiver, EventManager};
use gfx;
use world;
use util::EncoderChannel;

pub struct Game {
    events: EventReceiver,
    //player: world::Control,
    encoder_chan: EncoderChannel,
    is_running: bool,
}

impl Game {
    pub fn new<R, F>(
        manager: &mut EventManager,
        ev_receiver: EventReceiver,
        encoder_chan: EncoderChannel,
        factory: &mut F,
    ) -> Game
    where
        R: 'static + gfx::Resources,
        F: gfx::Factory<R>,
    {

        let game_receiver = manager.add_receiver();

        Game {
            events: game_receiver,
            encoder_chan,
            is_running: true,
        }
    }

    pub fn update(&mut self) {
        match self.events.try_recv() {
            Ok(event) => {
                match event {
                    Event::Quit => self.is_running = false,
                    _ => (),
                }
            }
            Err(_) => return,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }
}
