use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, border};
use tui::widgets::canvas::{Canvas, Line, Points};
use tui::layout::{Group, Rect, Direction, Size};
use tui::style::Color;

//use chip8_emulator::core::Screen;
use peripherals::Chip8Disp;

pub struct TuiDisplay {
    terminal: Terminal<TermionBackend>,
}

impl TuiDisplay {
    pub fn new() -> TuiDisplay {
        let backend = TermionBackend::new().unwrap();
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();

        TuiDisplay {
            terminal,
        }
    }
}

impl Chip8Disp for TuiDisplay {
    fn draw(&mut self) {
        /*
        let mut points = Vec::new();
        for (y, line) in chip8_screen.iter().enumerate() {
            for (x, value) in line.iter().enumerate() {
                if *value == true {
                    points.push( (x as f64, y as f64) );
                }
            }
        }

        let size = self.terminal.size().unwrap();
        Group::default()
            .direction(Direction::Vertical)
            .margin(1)
            .sizes(&[Size::Percent(100)])
            .render(&mut self.terminal, &size, |t, chunks| {
                Canvas::default()
                    .block(Block::default().borders(border::ALL).title("Canvas"))
                    .paint(|ctx| {
                        ctx.draw(&Points {
                            coords: &points[..],
                            color: Color::Red,
                        });
                    })
                    .x_bounds([0.0, 64.0])
                    .y_bounds([0.0, 48.0])
                    .render(t, &chunks[0]);
            });

        self.terminal.draw().unwrap();
        */
    }

    fn set_pixel_data(&mut self) {
    }
}

/*
pub fn playground() {
    let backend = TermionBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    draw(&mut terminal);

    loop {
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                if input == event::Key::Char('q') {
                    break;
                }
            }
        }
        draw(&mut terminal);
        thread::sleep(time::Duration::from_millis(500));
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<TermionBackend>) {
    let size = t.size().unwrap();

    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, &size, |t, chunks| {
            Block::default()
                .title("Block")
                .borders(border::ALL)
                .render(t, &chunks[0]);

            Canvas::default()
                .block(Block::default().borders(border::ALL).title("Canvas"))
                .paint(|ctx| {
                    ctx.draw(&Line {
                        x1: 0.0,
                        y1: 0.0,
                        x2: 64.0,
                        y2: 48.0,
                        color: Color::Yellow,
                    });

                    ctx.draw(&Points {
                        coords: &[(10.0, 11.0), (10.0, 12.0), (11.0, 12.0), (11.0, 11.0)],
                        color: Color::Red,
                    });
                })
                .x_bounds([0.0, 64.0])
                .y_bounds([0.0, 48.0])
                .render(t, &chunks[1]);
        });
    t.draw().unwrap();
}
*/