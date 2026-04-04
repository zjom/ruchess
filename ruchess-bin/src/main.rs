use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use ruchess_core::{Color as ChessColor, Game, Move, Piece, Role, Square};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug)]
pub struct App {
    game: Game,
    /// (file 0–7, chess_rank 0–7) where rank 0 corresponds to rank 1 on the board
    cursor: (u8, u8),
    selected: Option<(u8, u8)>,
    status: String,
    move_history: Vec<String>,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::new(),
            cursor: (4, 1),
            selected: None,
            status: String::from("Navigate with arrows/ hjkl, Space/ Enter to select/move."),
            move_history: Vec::new(),
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Up | KeyCode::Char('k') => {
                if self.cursor.1 < 7 {
                    self.cursor.1 += 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.cursor.1 > 0 {
                    self.cursor.1 -= 1;
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.cursor.0 > 0 {
                    self.cursor.0 -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.cursor.0 < 7 {
                    self.cursor.0 += 1;
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => self.handle_select(),
            KeyCode::Esc => {
                self.selected = None;
                self.status = String::from("Deselected.");
            }
            _ => {}
        }
    }

    fn handle_select(&mut self) {
        let (file, rank) = self.cursor;
        let to_sq = Square::from(rank * 8 + file);

        match self.selected {
            None => {
                self.selected = Some((file, rank));
                self.status = format!("Selected {to_sq}. Choose destination.");
            }
            Some((ff, fr)) if ff == file && fr == rank => {
                self.selected = None;
                self.status = String::from("Deselected.");
            }
            Some((ff, fr)) => {
                let from_sq = Square::from(fr * 8 + ff);
                let mv = Move {
                    from: from_sq,
                    to: to_sq,
                };
                match self.game.make_move(mv) {
                    Ok(()) => {
                        let s = format!("{from_sq}{to_sq}");
                        self.move_history.push(s.clone());
                        self.selected = None;
                        self.status = format!("Played {s}.");
                    }
                    Err(e) => {
                        self.selected = None;
                        self.status = format!("Invalid move: {e}");
                    }
                }
            }
        }
    }
}

// Board cell dimensions
const CELL_W: u16 = 10;
const CELL_H: u16 = 5;
const RANK_LABEL_W: u16 = 2; // "8 " on the left
const FILE_LABEL_H: u16 = 1; // "a b c …" row

const BOARD_W: u16 = RANK_LABEL_W + 8 * CELL_W + 1;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer = Block::new()
            .title("  ♟  RuChess  ♟  ")
            .borders(Borders::ALL);
        let inner = outer.inner(area);
        outer.render(area, buf);

        let chunks =
            Layout::horizontal([Constraint::Length(BOARD_W), Constraint::Min(22)]).split(inner);

        render_board(self, chunks[0], buf);
        render_info(self, chunks[1], buf);
    }
}

fn cell_bg(is_selected: bool, is_cursor: bool, is_light: bool) -> Color {
    match (is_selected, is_cursor, is_light) {
        (true, _, _) => Color::Rgb(247, 247, 105), // bright yellow
        (_, true, _) => Color::Rgb(186, 202, 68),  // yellow-green
        (_, _, true) => Color::Rgb(238, 238, 210), // light tan
        _ => Color::Rgb(118, 150, 86),             // dark green
    }
}

fn piece_sprite(role: Role) -> String {
    match role {
        Role::Rook => {
            r#"▗▄ ▃ ▄▖
▐█▄█▄█▌
▝▜███▛▘
 ▟███▙ 
▝▀▀▀▀▀▘
"#
        }
        Role::Queen => {
            r#"▗  ▂  ▖
▐▙▟█▙▟▌
 ▜███▛ 
 ▗███▖ 
▝▀▀▀▀▀▘
"#
        }
        Role::Pawn => {
            r#"
 ▄▇▄
 ▜█▛
▄███▄
▔▔▔▔▔
"#
        }
        Role::Knight => {
            r#"  ▅ ▅
 ▟▛███▖
▝▀▜███▊
 ▗███▛ 
 ▀▀▀▀▀ 
"#
        }
        Role::King => {
            r#"  ▂▃╋▃▂  
 ▐█████▋ 
  ▜███▛  
   ▟█▙   
  ▀▀▀▀▀  
"#
        }
        Role::Bishop => {
            r#"▗▅  ▖
██▍ █
███▍█
▝███▘
▀▀▀▀▀
"#
        }
    }
    .to_string()
}

fn render_board(app: &App, area: Rect, buf: &mut Buffer) {
    let grid = app.game.grid();

    let board_x = area.x + RANK_LABEL_W;
    let board_y = area.y + FILE_LABEL_H;

    // File labels (a–h) at top and bottom
    for file in 0..8u16 {
        let ch = (b'a' + file as u8) as char;
        let x = board_x + file * CELL_W + 1;
        if let Some(c) = buf.cell_mut((x, area.y)) {
            c.set_char(ch).set_fg(Color::DarkGray);
        }
        if let Some(c) = buf.cell_mut((x, board_y + 8 * CELL_H)) {
            c.set_char(ch).set_fg(Color::DarkGray);
        }
    }

    for display_row in 0..8u16 {
        // display_row 0 = top = rank 8; display_row 7 = bottom = rank 1
        let chess_rank = (7 - display_row) as usize;
        let cell_y = board_y + display_row * CELL_H;

        // Rank label on the left
        if let Some(c) = buf.cell_mut((area.x, cell_y)) {
            c.set_char(char::from_digit(chess_rank as u32 + 1, 10).unwrap())
                .set_fg(Color::DarkGray);
        }

        for file in 0..8u16 {
            let chess_file = file as usize;
            let cell_x = board_x + file * CELL_W;

            let (cur_file, cur_rank) = app.cursor;
            let is_cursor = cur_file == file as u8 && cur_rank == chess_rank as u8;
            let is_selected = app.selected == Some((file as u8, chess_rank as u8));
            let is_light = (chess_rank + chess_file) % 2 == 1;

            let bg = cell_bg(is_selected, is_cursor, is_light);

            // Fill cell background
            for dy in 0..CELL_H {
                for dx in 0..CELL_W {
                    if let Some(c) = buf.cell_mut((cell_x + dx, cell_y + dy)) {
                        c.set_char(' ').set_bg(bg);
                    }
                }
            }

            // Draw piece sprite (5 rows, centered horizontally)
            if let Some(piece) = grid[chess_rank][chess_file] {
                let Piece(role, color) = piece;
                let fg = match color {
                    ChessColor::White => Color::Rgb(255, 255, 255),
                    ChessColor::Black => Color::Rgb(15, 15, 15),
                };
                let sprite_str = piece_sprite(role);
                let lines: Vec<&str> = sprite_str.lines().collect();
                let max_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
                let x_offset = ((CELL_W as usize).saturating_sub(max_width)) / 2;
                for (dy, row) in lines.iter().enumerate() {
                    if dy >= CELL_H as usize {
                        break;
                    }
                    for (dx, ch) in row.chars().enumerate() {
                        if let Some(c) =
                            buf.cell_mut((cell_x + x_offset as u16 + dx as u16, cell_y + dy as u16))
                        {
                            c.set_char(ch).set_style(Style::new().fg(fg).bg(bg).bold());
                        }
                    }
                }
            }

            // Small dot in bottom-right corner to mark the cursor square
            if is_cursor {
                if let Some(c) = buf.cell_mut((cell_x + CELL_W - 1, cell_y + CELL_H - 1)) {
                    c.set_char('·').set_fg(Color::White).set_bg(bg);
                }
            }
        }
    }
}

fn render_info(app: &App, area: Rect, buf: &mut Buffer) {
    let turn = app.game.turn();
    let (turn_label, turn_fg) = match turn {
        ChessColor::White => ("● White to move", Color::Rgb(255, 255, 255)),
        ChessColor::Black => ("● Black to move", Color::Rgb(150, 150, 150)),
    };

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(turn_label, Style::new().fg(turn_fg).bold())),
        Line::from(""),
        Line::from(Span::styled(
            "Controls",
            Style::new().add_modifier(Modifier::UNDERLINED),
        )),
        Line::from(" ↑↓←→   navigate"),
        Line::from(" Enter  select / move"),
        Line::from(" Esc    deselect"),
        Line::from(" q      quit"),
        Line::from(""),
        Line::from(Span::styled(
            "Moves",
            Style::new().add_modifier(Modifier::UNDERLINED),
        )),
    ];

    // Show last 8 half-moves
    let history = &app.move_history;
    let start = history.len().saturating_sub(8);
    for (i, mv) in history[start..].iter().enumerate() {
        let half_move = start + i;
        let move_num = half_move / 2 + 1;
        let label = if half_move % 2 == 0 {
            format!(" {move_num}. {mv}")
        } else {
            format!("    … {mv}")
        };
        lines.push(Line::from(label));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Status",
        Style::new().add_modifier(Modifier::UNDERLINED),
    )));
    lines.push(Line::from(format!(" {}", app.status)));

    Paragraph::new(lines)
        .block(Block::new().borders(Borders::LEFT))
        .render(area, buf);
}
