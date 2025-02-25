use crossterm::event::{self, Event as CTEvent, KeyCode};

use ratatui::{
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{io::Result, process::Child, sync::mpsc::Receiver, thread::JoinHandle, time::Duration};

use crate::{imp::begin_serial, profile::Profile, AppArgs};

pub(crate) enum Event {
    LeftPress,
    LeftRelease,
    RightPress,
    RightRelease,
    Reset,
    Disconnected,
}

pub(crate) struct App<'a> {
    pub left_paddle: bool,
    pub right_paddle: bool,
    pub connected: bool,
    pub options: AppOptions,
    profile: Profile<'a>,
    dit_child: Option<Child>,
    dah_child: Option<Child>,
}

pub(crate) struct AppOptions {
    pub dit_side: Side,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub(crate) enum Side {
    Left,
    Right,
}

impl<'a> App<'a> {
    fn press_left(&mut self) {
        self.left_paddle = true;
        if self.options.dit_side == Side::Left {
            self.start_dit();
        } else {
            self.start_dah();
        }
    }

    fn press_right(&mut self) {
        self.right_paddle = true;
        if self.options.dit_side == Side::Right {
            self.start_dit();
        } else {
            self.start_dah();
        }
    }

    fn release_left(&mut self) {
        self.left_paddle = false;
        if self.options.dit_side == Side::Left {
            self.kill_dit();
        } else {
            self.kill_dah();
        }
    }
    fn release_right(&mut self) {
        self.right_paddle = false;
        if self.options.dit_side == Side::Right {
            self.kill_dit();
        } else {
            self.kill_dah();
        }
    }

    fn start_dit(&mut self) {
        if self.dit_child.is_some() {
            return;
        }

        self.dit_child = self.profile.dit_child_command().spawn().ok();
    }

    fn start_dah(&mut self) {
        if self.dah_child.is_some() {
            return;
        }

        self.dah_child = self.profile.dah_child_command().spawn().ok();
    }

    fn reset(&mut self) {
        self.left_paddle = false;
        self.right_paddle = false;
        self.kill_dit();
        self.kill_dah();
    }

    fn kill_dit(&mut self) {
        match self.dit_child.take() {
            Some(mut child) => _ = child.kill(),
            None => {}
        }
    }
    fn kill_dah(&mut self) {
        match self.dah_child.take() {
            Some(mut child) => _ = child.kill(),
            None => {}
        }
    }

    fn new(args: &'a AppArgs) -> std::result::Result<Self, String> {
        Ok(Self {
            left_paddle: false,
            right_paddle: false,
            connected: true,
            options: Default::default(),
            profile: Profile::load(&args.profile)?,
            dit_child: None,
            dah_child: None,
        })
    }
}

impl Default for AppOptions {
    fn default() -> Self {
        Self {
            dit_side: Side::Left,
        }
    }
}
const COLOR: Color = Color::from_u32(0xFF00CC00);

pub fn start_tui(mut term: DefaultTerminal, args: AppArgs) -> Result<()> {
    let Some((handle, rx)) = begin_serial(&args) else {
        return Ok(());
    };

    let mut app = App::new(&args).unwrap();

    loop {
        update_app(&rx, &handle, &mut app);

        _ = term.draw(|frame| render(frame, &app));
        if event::poll(Duration::from_millis(20))? {
            match event::read()? {
                CTEvent::Key(ke) => {
                    if ke.code == KeyCode::Esc {
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    let status = if app.connected {
        Paragraph::new("CONNECTED")
            .bg(COLOR)
            .fg(Color::from_u32(0xFF000000))
    } else {
        Paragraph::new("DISCONNECTED")
            .bg(Color::from_u32(0xFFCC0000))
            .fg(Color::from_u32(0xFF9c9c9c))
    };
    frame.render_widget(
        status,
        Rect::new(0, 0, if app.connected { 9 } else { 12 }, 1),
    );

    if app.left_paddle {
        frame.render_widget(Paragraph::new("L").fg(COLOR), Rect::new(0, 1, 1, 1));
        if app.options.dit_side == Side::Left {
            frame.render_widget(Dit, Rect::default());
        } else {
            frame.render_widget(Dah, Rect::default());
        }
    }

    if app.right_paddle {
        frame.render_widget(Paragraph::new("R").fg(COLOR), Rect::new(1, 1, 1, 1));
        if app.options.dit_side == Side::Right {
            frame.render_widget(Dit, Rect::default());
        } else {
            frame.render_widget(Dah, Rect::default());
        }
    }
}

fn update_app(rx: &Receiver<Event>, _handle: &JoinHandle<()>, app: &mut App) {
    match rx.recv_timeout(Duration::from_millis(10)) {
        Ok(Event::LeftPress) => app.press_left(),
        Ok(Event::RightPress) => app.press_right(),
        Ok(Event::LeftRelease) => app.release_left(),
        Ok(Event::RightRelease) => app.release_right(),
        Ok(Event::Reset) => app.reset(),
        Ok(Event::Disconnected) => {
            app.reset();
            app.connected = false;
        }
        _ => {}
    }
}

struct Dit;
struct Dah;

impl Widget for Dit {
    fn render(self, _: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::new().bg(COLOR);
        let rect = Rect::new(2, 5, 5, 2);
        block.render(rect, buf);
    }
}
impl Widget for Dah {
    fn render(self, _: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::new().bg(COLOR);
        let rect = Rect::new(8, 5, 10, 2);
        block.render(rect, buf);
    }
}
