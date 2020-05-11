use crate::ui::recording::InputRecorder;
use crate::ui::Milliseconds;
use crate::ui::*;

use crate::logic::{Game, GameMode, Time};

use core::ffi::c_void;
use core::mem::{align_of, size_of};

pub type AllocatorContext = *mut c_void;

/// Функция, принимающая в качестве первого аргумента некоторый контекст,
/// в качестве второго аргумента размер выделяемого блока памяти,
/// а в качестве третьего — выравнивание.
pub type Allocator = unsafe fn(AllocatorContext, usize, usize) -> *mut c_void;

/// Функция, принимающая в качестве первого аргумента некоторый контекст,
/// в качестве второго — указатель на освобождаемый блок памяти,
/// а в качестве третьего — размер освобождаемого блока.
pub type Deallocator = unsafe fn(AllocatorContext, *mut c_void, usize);

// Unwinding through FFI boundaries is undefined behavior, so we stop any
// unwinding and abort.
#[cfg(feature = "std")]
fn ffi_safely_run<R, F: FnOnce() -> R>(f: F) -> R {
    use std::panic::*;

    // AssertUnwindSafe is okay here, since we'll abort anyway.
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(r) => r,
        Err(_) => std::process::abort(),
    }
}

// However, if this crate is compiled as no_std, there is no unwinding.
// The client will define the panic behavior themselves using #[panic_handler]
#[cfg(not(feature = "std"))]
fn ffi_safely_run<R, F: FnOnce() -> R>(f: F) -> R {
    f()
}

macro_rules! ffi_constructor {
    ($name:tt, $(<$($lifetime:lifetime),*>)? ($($arg_name:ident: $args:ty),*) -> $retty:ty) => {
        /// Выделяет память для объекта, используя переданный аллокатор,
        /// а затем инициализирует объект и возвращает на него указатель.
        ///
        /// Аллокатор должен возвращать корректно выровненный указатель на блок памяти
        /// достаточного размера. Нарушение любого из этих условий — неопределённое поведение.
        ///
        /// Размер и выравнивание передаются в качестве аргументов аллокатору.
        #[no_mangle]
        pub unsafe extern "C" fn $name $($(<$lifetime>),*)?(
            $($arg_name: $args,)*
            allocator_context: AllocatorContext,
            allocator: Allocator
        ) -> *mut $retty {
            use core::ptr::{null_mut, write};

            let memory = allocator(
                allocator_context,
                size_of::<$retty>(),
                align_of::<$retty>()
            ) as *mut $retty;
            if memory.is_null() {
                return null_mut();
            }

            ffi_safely_run(move || {
                write(memory, <$retty>::new($($arg_name),*));
            });

            memory
        }
    };
}

macro_rules! ffi_destructor {
    ($name:ident, ($arg:ident: $ty:ty)) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            $arg: *mut $ty,
            deallocator_context: AllocatorContext,
            deallocator: Deallocator,
        ) {
            if $arg.is_null() {
                return;
            }
            ffi_safely_run(|| $arg.drop_in_place());
            deallocator(deallocator_context, $arg as *mut c_void, size_of::<$ty>())
        }
    };
}

ffi_constructor!(mmheroes_game_create, (mode: GameMode, seed: u64) -> Game);
ffi_destructor!(mmheroes_game_destroy, (game: Game));

/// Число возможных вариантов для выбора.
///
/// Аргумент `game` не должен быть нулевым указателем, иначе UB.
#[no_mangle]
pub extern "C" fn mmheroes_game_get_available_actions(game: &mut Game) -> usize {
    ffi_safely_run(|| game.available_actions())
}

/// Записывает текущий игровой день и время в аргументы `out_day` и `out_time`
/// и возвращает `true` если они доступны, иначе не трогает аргументы и возвращает
/// `false`.
///
/// Игровой день и время могут быть недоступны, например, если игра ещё не началась.
#[no_mangle]
pub extern "C" fn mmheroes_game_get_current_time(
    game: &mut Game,
    out_day: &mut u8,
    out_time: &mut Time,
) -> bool {
    ffi_safely_run(|| {
        if let Some(state) = game.game_state() {
            *out_day = state.current_day().index() as u8;
            *out_time = state.current_time();
            true
        } else {
            false
        }
    })
}

ffi_constructor!(mmheroes_game_ui_create, (game: &mut Game) -> GameUI);
ffi_destructor!(mmheroes_game_ui_destroy, (game_ui: GameUI));

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FfiRendererRequest {
    ClearScreen,
    Flush,
    WriteStr {
        buf: *const u8,
        length: usize,
    },
    MoveCursor {
        line: u8,
        column: u8,
    },
    SetColor {
        foreground: Color,
        background: Color,
    },
    Sleep {
        milliseconds: Milliseconds,
    },
}

#[repr(C)]
pub struct FfiRendererRequestIterator {
    buf: *const u8,
    len: usize,
}

/// Инициализирует итератор по запросам на рендеринг.
/// `game_ui` должен быть валидный ненулевой указатель.
#[no_mangle]
pub extern "C" fn mmheroes_renderer_request_iterator_begin(
    iterator: &mut FfiRendererRequestIterator,
    game_ui: &GameUI,
) {
    ffi_safely_run(|| {
        let rust_iterator = game_ui.requests();
        iterator.buf = rust_iterator.encoded.as_ptr();
        iterator.len = rust_iterator.encoded.len();
    })
}

/// Продвигает итератор по запросам на рендеринг.
///
/// Возвращает `true` и записывает в параметр `out` следующий запрос, если он есть.
///
/// Возвращает `false`, если запросов больше нет.
#[no_mangle]
pub unsafe extern "C" fn mmheroes_renderer_request_iterator_next(
    iterator: &mut FfiRendererRequestIterator,
    out: &mut FfiRendererRequest,
) -> bool {
    ffi_safely_run(move || {
        let mut rust_iterator = crate::ui::renderer::RendererRequestIter {
            encoded: core::slice::from_raw_parts(iterator.buf, iterator.len),
        };

        let next = rust_iterator.next();

        *iterator = FfiRendererRequestIterator {
            buf: rust_iterator.encoded.as_ptr(),
            len: rust_iterator.encoded.len(),
        };

        match next {
            None => {
                return false;
            }
            Some(RendererRequest::ClearScreen) => *out = FfiRendererRequest::ClearScreen,
            Some(RendererRequest::Flush) => *out = FfiRendererRequest::Flush,
            Some(RendererRequest::WriteStr(s)) => {
                *out = FfiRendererRequest::WriteStr {
                    buf: s.as_ptr(),
                    length: s.len(),
                }
            }
            Some(RendererRequest::MoveCursor { line, column }) => {
                *out = FfiRendererRequest::MoveCursor { line, column }
            }
            Some(RendererRequest::SetColor {
                foreground,
                background,
            }) => {
                *out = FfiRendererRequest::SetColor {
                    foreground,
                    background,
                }
            }
            Some(RendererRequest::Sleep(ms)) => {
                *out = FfiRendererRequest::Sleep { milliseconds: ms }
            }
        };
        true
    })
}

/// Воспроизводит игру с помощью входных данных, записанных ранее с помощью
/// `InputRecorder`.
///
/// В случае ошибки возвращает `false`, иначе — `true`.
#[no_mangle]
pub unsafe extern "C" fn mmheroes_replay(
    game_ui: &mut GameUI,
    recorded_input: *const u8,
    recorded_input_len: usize,
) -> bool {
    ffi_safely_run(|| {
        assert!(!recorded_input.is_null());
        let slice = core::slice::from_raw_parts(recorded_input, recorded_input_len);
        let s = match core::str::from_utf8(slice) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let mut parser = recording::InputRecordingParser::new(s);
        parser
            .parse_all(|input| game_ui.continue_game(input))
            .is_ok()
    })
}

/// Продолжает игру до следующего запроса на нажатие клавиши.
///
/// При первом вызове этой функции неважно, что передаётся в параметре `input`.
#[no_mangle]
pub extern "C" fn mmheroes_continue(game_ui: &mut GameUI, input: Input) -> bool {
    ffi_safely_run(|| game_ui.continue_game(input))
}

#[repr(C)]
pub struct InputRecorderSink {
    context: *mut c_void,
    sink: fn(*mut c_void, *const u8, usize) -> bool,
}

impl core::fmt::Write for InputRecorderSink {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if (self.sink)(self.context, s.as_ptr(), s.len()) {
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }
}

ffi_constructor!(
    mmheroes_input_recorder_create,
    <'a> (sink: &'a mut InputRecorderSink) -> InputRecorder<'a, InputRecorderSink>);

ffi_destructor!(
    mmheroes_input_recorder_destroy,
    (recorder: InputRecorder<InputRecorderSink>)
);

#[no_mangle]
pub unsafe extern "C" fn mmheroes_input_recorder_record(
    recorder: &mut InputRecorder<InputRecorderSink>,
    input: Input,
) -> bool {
    ffi_safely_run(|| recorder.record_input(input).is_ok())
}

#[no_mangle]
pub unsafe extern "C" fn mmheroes_input_recorder_flush(
    recorder: &mut InputRecorder<InputRecorderSink>,
) -> bool {
    ffi_safely_run(|| recorder.flush().is_ok())
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::Layout;
    use std::ptr::{null, null_mut};

    unsafe fn allocator(
        _context: AllocatorContext,
        size: usize,
        alignment: usize,
    ) -> *mut c_void {
        std::alloc::alloc_zeroed(Layout::from_size_align(size, alignment).unwrap())
            as *mut c_void
    }

    unsafe fn deallocator(_context: AllocatorContext, memory: *mut c_void, size: usize) {
        std::alloc::dealloc(memory as *mut u8, Layout::from_size_align(size, 8).unwrap())
    }

    #[test]
    fn test_ffi() {
        unsafe {
            let game = mmheroes_game_create(GameMode::Normal, 0, null_mut(), allocator);

            let game_ui = mmheroes_game_ui_create(&mut *game, null_mut(), allocator);

            mmheroes_continue(&mut *game_ui, Input::Enter);

            let mut iterator = FfiRendererRequestIterator {
                buf: null(),
                len: 0,
            };
            mmheroes_renderer_request_iterator_begin(&mut iterator, &mut *game_ui);

            assert!(!iterator.buf.is_null());
            assert!(!iterator.len > 0);

            let mut requests = Vec::new();
            let mut request = FfiRendererRequest::ClearScreen;
            while mmheroes_renderer_request_iterator_next(&mut iterator, &mut request) {
                requests.push(request);
            }

            assert_eq!(requests.len(), 29);

            let mut day = 255u8;
            let mut time = Time(255);
            assert!(!mmheroes_game_get_current_time(
                &mut *game, &mut day, &mut time
            ));

            mmheroes_continue(&mut *game_ui, Input::Enter);
            mmheroes_continue(&mut *game_ui, Input::Enter);
            mmheroes_continue(&mut *game_ui, Input::Enter);
            mmheroes_continue(&mut *game_ui, Input::KeyDown);
            mmheroes_continue(&mut *game_ui, Input::KeyDown);
            mmheroes_continue(&mut *game_ui, Input::Enter);

            assert!(mmheroes_game_get_current_time(
                &mut *game, &mut day, &mut time
            ));
            assert_eq!(day, 0);
            assert_eq!(time, Time(9));

            mmheroes_game_ui_destroy(game_ui, null_mut(), deallocator);

            mmheroes_game_destroy(game, null_mut(), deallocator);
        }
    }
}
