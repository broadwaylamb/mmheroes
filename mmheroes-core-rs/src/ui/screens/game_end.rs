use crate::logic::GameState;
use crate::ui::*;

pub(in crate::ui) fn display_i_am_done(r: &mut Renderer) -> WaitingState {
    writeln_colored!(White, r, "Ну, может не надо так резко...");
    writeln_colored!(White, r, "Ты что, серьезно хочешь закончить игру");
    writeln!(r);

    let options = tiny_vec!(capacity: 16, [
        ("Нет, не хочу!", Color::CyanBright),
        ("Я же сказал: с меня хватит!", Color::CyanBright),
    ]);

    dialog(r, options)
}

pub(in crate::ui) fn display_game_end(
    r: &mut Renderer,
    _state: &GameState,
) -> WaitingState {
    // TODO: Display proper text based on the final state
    // (cause of death/expelling, or congratulation)

    writeln_colored!(MagentaBright, r, "Уффффф! Во всяком случае, ты еще живой.");
    writeln!(r);
    write_colored!(RedBright, r, "У тебя нет целых ");
    write_colored!(
        WhiteBright,
        r,
        "{}",
        6 /* TODO: actual number of exams remaining */
    );
    writeln_colored!(RedBright, r, " зачетов!");
    writeln_colored!(MagentaBright, r, "ТЫ ОТЧИСЛЕН!");

    wait_for_any_key(r)
}

pub(in crate::ui) fn display_wanna_try_again(r: &mut Renderer) -> WaitingState {
    writeln_colored!(White, r, "Хочешь попробовать еще?");
    writeln!(r);
    writeln!(r);

    let options = tiny_vec!(capacity: 16, [
        ("ДА!!! ДА!!! ДА!!!", Color::CyanBright),
        ("Нет... Нет... Не-э-эт...", Color::CyanBright),
    ]);

    dialog(r, options)
}

pub(in crate::ui) fn display_disclaimer(r: &mut Renderer) -> WaitingState {
    writeln_colored!(Green, r, "DISCLAIMER");
    writeln!(r);
    r.set_color(Color::BlueBright, Color::Black);
    writeln!(
        r,
        "1.) Все персонажи реальны. Эта программа является лишь неким отражением"
    );
    writeln!(r, "    мнения ее автора об окружающей действительности.");
    writeln!(
        r,
        "    Автор не ставил цели оценить чью-либо линию поведения."
    );
    writeln!(r);
    writeln!(
        r,
        "2.) Почти все события реальны. Естественно, многие из них"
    );
    writeln!(r, "    представлены в несколько аллегорическом виде.");
    writeln!(r);
    writeln!(
        r,
        "3.) Все совпадения с другими реальными зачетными неделями,"
    );
    writeln!(
        r,
        "    проведенными кем-либо в каком-либо ВУЗе, лишь подчеркивают"
    );
    writeln!(r, "    реалистичность взглядов автора на реальность.");
    writeln!(r);
    writeln!(r);
    r.set_color(Color::RedBright, Color::Black);
    writeln!(
        r,
        "*.) Если вы нашли в данной программе ошибку (любую, включая опечатки),"
    );
    writeln!(r, "    Ваши комментарии будут очень полезны.");
    writeln!(r);
    r.set_color(Color::Gray, Color::Black);
    writeln!(
        r,
        "Автор не несет ответственность за психическое состояние игрока."
    );

    wait_for_any_key(r)
}