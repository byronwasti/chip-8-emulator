use std::fmt;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, border};
use tui::widgets::canvas::{Canvas, Points};
use tui::layout::{Group, Direction, Size};
use tui::style::Color;

use peripherals::{Chip8Disp, PixelData};

pub struct TuiDisplay {
    terminal: Terminal<TermionBackend>,
    data: [[bool; 64]; 32],
}

impl fmt::Display for TuiDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.data.iter() {
            for x in y.iter() {
                if *x {
                    write!(f, "X")?;
                } else {
                    write!(f, "_")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "End")
    }
}

impl TuiDisplay {
    pub fn new() -> TuiDisplay {
        let backend = TermionBackend::new().unwrap();
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();

        TuiDisplay {
            terminal,
            data: [[false; 64]; 32],
        }
    }
}

impl Chip8Disp for TuiDisplay {
    fn draw(&mut self) {
        debug!("Display:\n{}", &self);

        let mut points = Vec::new();
        for (y, line) in self.data.iter().enumerate() {
            let y = self.data.len() - y - 1;
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
                            color: Color::White,
                        });
                    })
                    .x_bounds([0.0, 64.0])
                    .y_bounds([0.0, 32.0])
                    .render(t, &chunks[0]);
            });

        self.terminal.draw().unwrap();
    }

    fn set_pixel_data(&mut self, data: &[PixelData]) -> bool {
        let mut return_val = false;
        for (_, pixel) in data.iter().enumerate() {
            if pixel.x >= 64 || pixel.y >= 32 {
                warn!("Invalid pixel value: {:?}", pixel);
                continue;
            }

            if pixel.val && self.data[pixel.y][pixel.x] {
                self.data[pixel.y][pixel.x] = false;
                return_val = true;
            } else if !self.data[pixel.y][pixel.x] {
                self.data[pixel.y][pixel.x] = pixel.val;
            }
        }

        return_val
    }

    fn clear(&mut self) {
        self.data = [[false; 64]; 32];
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
