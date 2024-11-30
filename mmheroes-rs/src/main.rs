use mmheroes_core::logic::{create_game, StateHolder};
use mmheroes_core::ui::recording::InputRecordingParser;
use mmheroes_core::{
    logic::GameMode,
    ui::{
        self, recording,
        renderer::{RendererRequest, RendererRequestConsumer},
        *,
    },
};
use pancurses::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::pin::pin;
use std::str::FromStr;
use std::sync::Mutex;

fn env_seed() -> Option<u64> {
    if cfg!(debug_assertions) {
        std::env::var("MMHEROES_SEED")
            .ok()
            .and_then(|s| u64::from_str(&s).ok())
    } else {
        None
    }
}

/// Удобно для тестирования.
///
/// Позволяет передать шаги до нужного экрана в переменной окружения, чтобы не тратить
/// время на прохождение руками.
fn env_steps() -> Option<String> {
    if cfg!(debug_assertions) {
        std::env::var("MMHEROES_STEPS").ok()
    } else {
        None
    }
}

mod screen {
    use super::{endwin, initscr, Window};

    /// A RAII object responsible for initializing and cleaning up the curses
    /// window.
    pub(crate) struct ScreenRAII {
        window: Window,
    }

    impl ScreenRAII {
        pub(crate) fn new() -> ScreenRAII {
            ScreenRAII { window: initscr() }
        }
    }

    impl Drop for ScreenRAII {
        fn drop(&mut self) {
            endwin();
        }
    }

    impl std::ops::Deref for ScreenRAII {
        type Target = Window;

        fn deref(&self) -> &Self::Target {
            &self.window
        }
    }
}

mod high_scores {
    use mmheroes_core::ui::high_scores::{decode, encode, HighScore, BUFFER_SIZE, SCORE_COUNT};
    use std::fs::*;
    use std::io::Read;

    use std::path::PathBuf;

    fn hi_file_path() -> PathBuf {
        let dir = directories::ProjectDirs::from("com.broadwaylamb", "", "mmheroes")
            .map(|dirs| dirs.data_local_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        dir.join("MMHEROES.HI")
    }

    pub(crate) fn load() -> Option<[HighScore; SCORE_COUNT]> {
        let mut f = match OpenOptions::new().read(true).open(hi_file_path()) {
            Ok(f) => f,
            Err(_) => return None,
        };

        let mut buffer = [0u8; BUFFER_SIZE];
        if f.read_exact(&mut buffer).is_err() {
            return None;
        }

        decode(&buffer)
    }

    pub(crate) fn save(scores: &[HighScore; SCORE_COUNT]) {
        let encoded = encode(scores);
        let _ = write(hi_file_path(), encoded.as_ref());
    }
}

use screen::ScreenRAII;

type Log = String;
type Logger = Mutex<RefCell<recording::InputRecorder<'static, Log>>>;

fn getch(window: &ScreenRAII, logger: &Logger) -> ui::Input {
    loop {
        let ui_input = match window.getch() {
            None | Some(pancurses::Input::KeyResize) => continue,
            Some(pancurses::Input::KeyUp) => ui::Input::KeyUp,
            Some(pancurses::Input::KeyDown) => ui::Input::KeyDown,
            Some(pancurses::Input::Character('\n')) => ui::Input::Enter,
            Some(_) => ui::Input::Other,
        };
        {
            let logger = logger.lock().unwrap();
            logger.borrow_mut().record_input(ui_input).unwrap();
        }
        break ui_input;
    }
}

fn pause() {
    if cfg!(windows) {
        let _ = std::process::Command::new("cmd.exe")
            .arg("/c")
            .arg("pause")
            .status();
    }
}

fn resize_terminal(height: i32, width: i32) {
    if !cfg!(windows) {
        println!("\x1B[8;{};{}t", height, width);
    }
    resize_term(height, width);
}

struct RendererRequestEvaluator<'a, 'b> {
    window: &'a ScreenRAII,
    color_pairs_map: &'b HashMap<(Color, Color), i16>,
}

impl RendererRequestConsumer for RendererRequestEvaluator<'_, '_> {
    fn consume_request(&mut self, request: RendererRequest<'_>) {
        match request {
            RendererRequest::ClearScreen => self.window.clear(),
            RendererRequest::Flush => self.window.refresh(),
            RendererRequest::WriteStr(s) => self.window.addnstr(s, s.len()),
            RendererRequest::MoveCursor { line, column } => {
                self.window.mv(line as i32, column as i32)
            }
            RendererRequest::SetColor {
                foreground,
                background,
            } => self.window.color_set(
                *self
                    .color_pairs_map
                    .get(&(foreground, background))
                    .unwrap_or_else(|| {
                        panic!("Unknown color pair: ({:?}, {:?})", foreground, background)
                    }),
            ),
            RendererRequest::Sleep(ms) => napms(ms.0),
        };
    }
}

fn main() {
    let window = ScreenRAII::new();
    start_color();
    set_blink(true);
    curs_set(1);

    cbreak();
    noecho();

    window.keypad(true);
    window.nodelay(false);

    resize_terminal(24, 80);

    window.clear();
    window.refresh();

    let color_pairs = [
        (Color::White, Color::Black),
        (Color::Gray, Color::Black),
        (Color::Red, Color::Black),
        (Color::RedBright, Color::Black),
        (Color::Green, Color::Black),
        (Color::YellowBright, Color::Black),
        (Color::Cyan, Color::Black),
        (Color::CyanBright, Color::Black),
        (Color::WhiteBright, Color::Black),
        (Color::Black, Color::White),
        (Color::Black, Color::Yellow),
        (Color::Black, Color::Gray),
        (Color::Magenta, Color::Black),
        (Color::MagentaBright, Color::Black),
        (Color::BlueBright, Color::Black),
        (Color::Blue, Color::Black),
    ];

    let mut color_pairs_map = HashMap::<(Color, Color), i16>::new();

    for (i, &(foreground, background)) in color_pairs.iter().enumerate() {
        init_pair(i as i16, foreground as i16, background as i16);
        color_pairs_map.insert((foreground, background), i as i16);
    }

    window.bkgd(COLOR_PAIR(
        *color_pairs_map.get(&(Color::White, Color::Black)).unwrap() as chtype,
    ));

    // We save each pressed key to this log, so that if a panic occurs,
    // we could print it and the player could send a useful bug report.
    let logger = {
        let log = &mut *Box::leak(Box::new(Log::new()));
        let logger = Box::new(Mutex::new(RefCell::new(recording::InputRecorder::new(log))));

        // Leak the log and the logger object so that we could obtain a reference with
        // static lifetime. This is needed for accessing it in the panic handler.
        &*Box::leak(logger)
    };

    let mode = match std::env::args().nth(1).as_deref() {
        Some("-3dec-happy-birthday-Diamond") => GameMode::God,
        Some(_) => GameMode::SelectInitialParameters,
        None => GameMode::Normal,
    };

    let seed = env_seed().unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    });

    let steps = env_steps();

    let observable_game_state = StateHolder::new(mode);
    let game = create_game(seed, &observable_game_state);
    let game = pin!(game);

    let renderer_request_evaluator = RendererRequestEvaluator {
        window: &window,
        color_pairs_map: &color_pairs_map,
    };

    let mut game_ui = GameUI::new(
        &observable_game_state,
        game,
        seed,
        high_scores::load(),
        renderer_request_evaluator,
    );

    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        endwin(); // Switch back to normal terminal I/O.
        default_hook(panic_info); // Print panic message and optionally a backtrace.
        eprintln!("Зерно игры: {}", seed);
        let logger = logger.lock().unwrap();
        logger.borrow_mut().flush().unwrap();
        eprintln!(
            "Шаги для воспроизведения бага: {:?}",
            logger.borrow_mut().output()
        );
        eprintln!("Пожалуйста, отправь зерно игры и шаги для воспроизведения бага разработчику.");
        pause();
    }));

    let mut input = if let Some(steps) = steps {
        let mut steps_parser = InputRecordingParser::new(&steps);
        match steps_parser.parse_all(|input| {
            napms(300);
            game_ui.continue_game(input)
        }) {
            Ok(()) => {}
            Err(error) => panic!("Parsing steps failed: {:?}", error),
        }
        getch(&window, logger)
    } else {
        ui::Input::Enter
    };

    while game_ui.continue_game(input) {
        input = getch(&window, logger);
    }

    high_scores::save(&game_ui.high_scores);
}
