use std::time::Duration;

use tuirealm::{Sub, SubClause, SubEventClause, terminal::TerminalAdapter};

use crate::{
    component::{Editor, GlobalListener, ScoreTable, Timer, UsernameInput},
    repository,
};

use super::{App, Id, Screen};

impl<T> App<T>
where
    T: TerminalAdapter,
{
    fn mount_home(&mut self) {
        self.mount(
            Id::UsernameInput,
            Box::new(UsernameInput::default()),
            Vec::new(),
        )
        .unwrap();

        let scores = repository::score::get_all(&self.config.database_file).unwrap();
        self.mount(
            Id::ScoreTable,
            Box::new(ScoreTable::new(scores)),
            Vec::new(),
        )
        .unwrap();

        self.active(&Id::UsernameInput).unwrap();
    }

    fn mount_game(&mut self) {
        let timer = Timer::new(
            Duration::from_secs(self.config.game_duration),
            Duration::from_secs(self.config.tick_rate),
        );
        self.mount(
            Id::Timer,
            Box::new(timer),
            vec![Sub::new(SubEventClause::Tick, SubClause::Always)],
        )
        .unwrap();

        self.mount(Id::Editor, Box::new(Editor::default()), Vec::new())
            .unwrap();

        self.active(&Id::Editor).unwrap();
    }

    pub fn mount_all(&mut self) {
        self.umount_all();

        self.mount(
            Id::GlobalListener,
            Box::new(GlobalListener::default()),
            vec![Sub::new(SubEventClause::Any, SubClause::Always)],
        )
        .unwrap();

        match self.screen {
            Screen::Home => self.mount_home(),
            Screen::Game => self.mount_game(),
        }
    }
}
