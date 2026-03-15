use std::collections::HashSet;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::{Block, Clear, Paragraph};
use crate::game::Game;
use crate::piece::PieceType;

pub fn render(frame: &mut Frame, game: &Game) {
    let [board_area, sidebar_area] = Layout::horizontal([
        Constraint::Length(22),
        Constraint::Length(18),
    ]).areas(frame.area());

    render_board(frame, game, board_area);
    render_sidebar(frame, game, sidebar_area);

    if game.game_over {
        render_game_over(frame, frame.area())
    }
}

fn render_board(frame: &mut Frame, game: &Game, area: Rect) {
    let block = Block::bordered().title("TETRIS");
    let inner = block.inner(area);
    frame.render_widget(&block, area);

    let active: HashSet<(i32, i32)> = game.current.cells().iter().copied().collect();
    let piece_color = game.current.piece_type.color();

    let buf = frame.buffer_mut();
    for row in 0..20usize {
        for col in 0..10usize {
            let x = inner.x + col as u16 * 2;
            let y = inner.y + row as u16;
            let color = if active.contains(&(row as i32, col as i32)) {
                piece_color
            } else {
                game.board.get(row, col).unwrap_or(Color::Reset)
            };
            buf[(x, y)].set_char(' ').set_bg(color);
            buf[(x + 1, y)].set_char(' ').set_bg(color);
        }
    }
}

fn render_sidebar(frame: &mut Frame, game: &Game, area: Rect) {
    let block = Block::bordered().title("NEXT PIECES");
    let inner = block.inner(area);
    frame.render_widget(&block, area);

    let buf = frame.buffer_mut();

    for (i, &piece_type) in game.queue.iter().enumerate() {
        let slot_top = inner.y + 1 + i as u16 * 6;
        let cells = normalize(piece_type.cells(0));
        let color = piece_type.color();

        for (r, c) in cells {
            let x = inner.x + 1 + c as u16 * 2;
            let y = slot_top + 1 + r as u16;
            if x + 1 < inner.x + inner.width && y < inner.y + inner.height {
                buf[(x, y)].set_char(' ').set_bg(color);
                buf[(x + 1, y)].set_char(' ').set_bg(color);
            }
        }
    }

    let stats = [
        ("Score", format!("{}", game.score)),
        ("Lines", format!("{}", game.lines)),
        ("Level", format!("{}", game.level)),
    ];

    for (i, (label, value)) in stats.iter().enumerate() {
        let y_label = inner.y + 19 + i as u16 * 2;
        let y_value = y_label + 1;
        write_str(buf, inner.x + 1, y_label, inner, label);
        write_str(buf, inner.x + 1, y_value, inner, value);
    }
}

fn render_game_over(frame: &mut Frame, area: Rect) {
    let w = 24u16;
    let h = 5u16;

    let popup = Rect {
        x: area.x + area.width.saturating_sub(w) / 2,
        y: area.y + area.height.saturating_sub(h) / 2,
        width: w.min(area.width),
        height: h.min(area.height),
    };
    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new("Press Q to quit")
            .block(Block::bordered().title("GAME OVER"))
            .alignment(Alignment::Center),
        popup,
    );
}

fn normalize(cells: [(i32, i32); 4]) -> [(i32, i32); 4] {
    let min_r = cells.iter().map(|&(r, _)| r).min().unwrap();
    let min_c = cells.iter().map(|&(_, c)| c).min().unwrap();
    cells.map(|(r, c)| (r - min_r, c - min_c))
}

fn write_str(buf: &mut ratatui::buffer::Buffer, x: u16, y: u16, inner: Rect, s: &str) {
    if y >= inner.y + inner.height {
        return
    }
    for (i, ch) in s.char_indices() {
        let cx = x + i as u16;
        if cx >= inner.x + inner.width {
            break;
        }
        buf[(cx, y)].set_char(ch);
    }
}