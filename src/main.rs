use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
    },
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{
        canvas::{Canvas, Map, MapResolution, Rectangle},
        Block, Borders, Gauge, Paragraph,
    },
    Frame, Terminal,
};

struct App {
    pet: Pet,
    playground: Rect,
    vx: f64,
    vy: f64,
    dir_x: bool,
    dir_y: bool,
}

struct Pet {
    x: f64,
    y: f64,
    color: tui::style::Color,
    body: [String; 2],
    name: String,
}

impl Pet {
    fn new(name: String, body: [String; 2]) -> Pet {
        Pet {
            x: 50.0,
            y: 50.0,
            color: Color::Yellow,
            body,
            name,
        }
    }
}

impl App {
    fn new() -> App {
        App {
            pet: Pet {
                x: 100.0,
                y: 500.0,
                color: Color::Yellow,
                body: [
                    String::from(
                        " .::::::::..     
                :::::::::::::   
               :::::::::::' .\\    
               `::::::::::_,__o   
               ",
                    ),
                    String::from(
                        " .::::::::..     
               :::::::::::::   
              :::::::::::' ˗\\   
              `::::::::::_,__o ",
                    ),
                ],
                name: String::from("Kip"),
            },
            playground: Rect::new(10, 10, 100, 100),
            vx: 1.0,
            vy: 1.0,
            dir_x: true,
            dir_y: true,
        }
    }

    fn on_tick(&mut self) {
        self.pet.x += 1.0;
        //self.pet.y += 1.0;
        //     if self.pet.x < self.playground.left() as f64
        //         || self.pet.x + self.pet.width > self.playground.right() as f64
        //     {
        //         self.dir_x = !self.dir_x;
        //     }
        //     if self.pet.y < self.playground.top() as f64
        //         || self.pet.y + self.pet.height > self.playground.bottom() as f64
        //     {
        //         self.dir_y = !self.dir_y;
        //     }

        //     if self.dir_x {
        //         self.pet.x += self.vx;
        //     } else {
        //         self.pet.x -= self.vx;
        //     }

        //     if self.dir_y {
        //         self.pet.y += self.vy;
        //     } else {
        //         self.pet.y -= self.vy
        //     }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        SetTitle("oxidatchi - your virtual pet")
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(100);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    //Events below are placeholders
                    KeyCode::Down => {
                        //app.vy = -1.0;
                    }
                    KeyCode::Up => {
                        //app.vy = 1.0;
                    }
                    KeyCode::Right => {
                        //app.vx = 1.0;
                    }
                    KeyCode::Left => {
                        //app.vx = -1.0;
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<'a, B: Backend>(f: &mut Frame<B>, app: &'a App) {

    let size = f.size().height as f64;

    let x_bounds = 1000.0;
    let y_bounds = 1000.0;

    let game_area = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("oxidatchi"))
        .paint(|ctx| {
            ctx.print(
                app.pet.x,
                app.pet.y + (y_bounds / size),
                Span::styled(" .::::::::..", Style::default().fg(Color::LightRed)),
            );
            ctx.print(
                app.pet.x,
                app.pet.y - (y_bounds / size),
                Span::styled(" :::::::::::::", Style::default().fg(Color::LightRed)),
            );
            ctx.print(
                app.pet.x,
                app.pet.y - (y_bounds / size) * 2.0,
                Span::styled(":::::::::::' .\\", Style::default().fg(Color::LightRed)),
            );
            ctx.print(
                app.pet.x,
                app.pet.y - (y_bounds / size) * 3.0,
                Span::styled("`::::::::::_,__o", Style::default().fg(Color::LightRed)),
            );
            ctx.print(10.0, 10.0, Span::raw(size.to_string()));
        })
        .x_bounds([0.0, x_bounds])
        .y_bounds([0.0, y_bounds]);
    f.render_widget(game_area, f.size());

}
