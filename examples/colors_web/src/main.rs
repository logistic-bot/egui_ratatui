use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::Paragraph,
};
use ratframe::NewCC;
use ratframe::RataguiBackend;
use web_time::{Duration, Instant};

use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

#[cfg(not(target_arch = "wasm32"))]
use ratframe::native_setup;

#[cfg(target_arch = "wasm32")]
use ratframe::wasm_setup;

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_setup(ColorsApp::default());
}
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    native_setup(ColorsApp::default())
}

pub struct ColorsApp {
    terminal: Terminal<RataguiBackend>,
}

//l
impl Default for ColorsApp {
    fn default() -> Self {
        //  setup_custom_fonts(&cc.egui_ctx);

        //Creating the Ratatui backend/ Egui widget here
        let backend = RataguiBackend::new(100, 100);
        let mut terminal = Terminal::new(backend).unwrap();
        Self { terminal: terminal }
    }
}

impl NewCC for ColorsApp {
    /// Called once before the first frame.
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for ColorsApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.terminal.draw(ui).expect("epic fail");

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(self.terminal.backend_mut());

            if ui.input(|i| i.key_released(egui::Key::Q)) {
                panic!("HAVE A NICE WEEK");
            }
            if ui.input(|i| i.key_released(egui::Key::T)) {
                ()
            }
            //KeyCode::Char(c) => app.on_key(c),
        });
    }
}

fn ui(frame: &mut Frame) {
    let layout = Layout::vertical([
        Constraint::Length(30),
        Constraint::Length(17),
        Constraint::Length(2),
    ])
    .split(frame.size());

    render_named_colors(frame, layout[0]);
    render_indexed_colors(frame, layout[1]);
    render_indexed_grayscale(frame, layout[2]);
}

const NAMED_COLORS: [Color; 16] = [
    Color::Black,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::Gray,
    Color::DarkGray,
    Color::LightRed,
    Color::LightGreen,
    Color::LightYellow,
    Color::LightBlue,
    Color::LightMagenta,
    Color::LightCyan,
    Color::White,
];

fn render_named_colors(frame: &mut Frame, area: Rect) {
    let layout = Layout::vertical([Constraint::Length(3); 10]).split(area);

    render_fg_named_colors(frame, Color::Reset, layout[0]);
    render_fg_named_colors(frame, Color::Black, layout[1]);
    render_fg_named_colors(frame, Color::DarkGray, layout[2]);
    render_fg_named_colors(frame, Color::Gray, layout[3]);
    render_fg_named_colors(frame, Color::White, layout[4]);

    render_bg_named_colors(frame, Color::Reset, layout[5]);
    render_bg_named_colors(frame, Color::Black, layout[6]);
    render_bg_named_colors(frame, Color::DarkGray, layout[7]);
    render_bg_named_colors(frame, Color::Gray, layout[8]);
    render_bg_named_colors(frame, Color::White, layout[9]);
}

fn render_fg_named_colors(frame: &mut Frame, bg: Color, area: Rect) {
    let block = title_block(format!("Foreground colors on {bg} background"));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::vertical([Constraint::Length(1); 2])
        .split(inner)
        .iter()
        .flat_map(|area| {
            Layout::horizontal([Constraint::Ratio(1, 8); 8])
                .split(*area)
                .to_vec()
        })
        .collect_vec();
    for (i, &fg) in NAMED_COLORS.iter().enumerate() {
        let color_name = fg.to_string();
        let paragraph = Paragraph::new(color_name).fg(fg).bg(bg);
        frame.render_widget(paragraph, layout[i]);
    }
}

fn render_bg_named_colors(frame: &mut Frame, fg: Color, area: Rect) {
    let block = title_block(format!("Background colors with {fg} foreground"));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::vertical([Constraint::Length(1); 2])
        .split(inner)
        .iter()
        .flat_map(|area| {
            Layout::horizontal([Constraint::Ratio(1, 8); 8])
                .split(*area)
                .to_vec()
        })
        .collect_vec();
    for (i, &bg) in NAMED_COLORS.iter().enumerate() {
        let color_name = bg.to_string();
        let paragraph = Paragraph::new(color_name).fg(fg).bg(bg);
        frame.render_widget(paragraph, layout[i]);
    }
}

fn render_indexed_colors(frame: &mut Frame, area: Rect) {
    let block = title_block("Indexed colors".into());
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::vertical([
        Constraint::Length(1), // 0 - 15
        Constraint::Length(1), // blank
        Constraint::Min(6),    // 16 - 123
        Constraint::Length(1), // blank
        Constraint::Min(6),    // 124 - 231
        Constraint::Length(1), // blank
    ])
    .split(inner);

    //    0   1   2   3   4   5    6   7   8   9  10  11   12  13  14  15
    let color_layout = Layout::horizontal([Constraint::Length(5); 16]).split(layout[0]);
    for i in 0..16 {
        let color = Color::Indexed(i);
        let color_index = format!("{i:0>2}");
        let bg = if i < 1 { Color::DarkGray } else { Color::Black };
        let paragraph = Paragraph::new(Line::from(vec![
            color_index.fg(color).bg(bg),
            "██".bg(color).fg(color),
        ]));
        frame.render_widget(paragraph, color_layout[i as usize]);
    }

    //   16  17  18  19  20  21   52  53  54  55  56  57   88  89  90  91  92  93
    //   22  23  24  25  26  27   58  59  60  61  62  63   94  95  96  97  98  99
    //   28  29  30  31  32  33   64  65  66  67  68  69  100 101 102 103 104 105
    //   34  35  36  37  38  39   70  71  72  73  74  75  106 107 108 109 110 111
    //   40  41  42  43  44  45   76  77  78  79  80  81  112 113 114 115 116 117
    //   46  47  48  49  50  51   82  83  84  85  86  87  118 119 120 121 122 123
    //
    //  124 125 126 127 128 129  160 161 162 163 164 165  196 197 198 199 200 201
    //  130 131 132 133 134 135  166 167 168 169 170 171  202 203 204 205 206 207
    //  136 137 138 139 140 141  172 173 174 175 176 177  208 209 210 211 212 213
    //  142 143 144 145 146 147  178 179 180 181 182 183  214 215 216 217 218 219
    //  148 149 150 151 152 153  184 185 186 187 188 189  220 221 222 223 224 225
    //  154 155 156 157 158 159  190 191 192 193 194 195  226 227 228 229 230 231

    // the above looks complex but it's so the colors are grouped into blocks that display nicely
    let index_layout = [layout[2], layout[4]]
        .iter()
        // two rows of 3 columns
        .flat_map(|area| {
            Layout::horizontal([Constraint::Length(27); 3])
                .split(*area)
                .to_vec()
        })
        // each with 6 rows
        .flat_map(|area| {
            Layout::vertical([Constraint::Length(1); 6])
                .split(area)
                .to_vec()
        })
        // each with 6 columns
        .flat_map(|area| {
            Layout::horizontal([Constraint::Min(4); 6])
                .split(area)
                .to_vec()
        })
        .collect_vec();

    for i in 16..=231 {
        let color = Color::Indexed(i);
        let color_index = format!("{i:0>3}");
        let paragraph = Paragraph::new(Line::from(vec![
            color_index.fg(color).bg(Color::Reset),
            ".".bg(color).fg(color),
            // There's a bug in VHS that seems to bleed backgrounds into the next
            // character. This is a workaround to make the bug less obvious.
            "███".reversed(),
        ]));
        frame.render_widget(paragraph, index_layout[i as usize - 16]);
    }
}

fn title_block(title: String) -> Block<'static> {
    Block::default()
        .borders(Borders::TOP)
        .border_style(Style::new().dark_gray())
        .title(title)
        .title_alignment(Alignment::Center)
        .title_style(Style::new().reset())
}

fn render_indexed_grayscale(frame: &mut Frame, area: Rect) {
    let layout = Layout::vertical([
        Constraint::Length(1), // 232 - 243
        Constraint::Length(1), // 244 - 255
    ])
    .split(area)
    .iter()
    .flat_map(|area| {
        Layout::horizontal([Constraint::Length(6); 12])
            .split(*area)
            .to_vec()
    })
    .collect_vec();

    for i in 232..=255 {
        let color = Color::Indexed(i);
        let color_index = format!("{i:0>3}");
        // make the dark colors easier to read
        let bg = if i < 244 { Color::Gray } else { Color::Black };
        let paragraph = Paragraph::new(Line::from(vec![
            color_index.fg(color).bg(bg),
            "██".bg(color).fg(color),
            // There's a bug in VHS that seems to bleed backgrounds into the next
            // character. This is a workaround to make the bug less obvious.
            "███████".reversed(),
        ]));
        frame.render_widget(paragraph, layout[i as usize - 232]);
    }
}
