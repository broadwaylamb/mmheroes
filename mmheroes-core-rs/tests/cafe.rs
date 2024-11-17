mod common;

use assert_matches::assert_matches;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{Action, CauseOfDeath, GameMode, GameScreen, Location, Time};

#[test]
fn cafe_punk() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Идём на факультет и убеждаемся что кафе ещё закрыто
    replay_game(game_ui, "4↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 8:00    Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (41)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты на факультете. Что делать?

Идти к преподу▁                                  АиТЧ    ПУНК  13-15    0/12
Посмотреть на баобаб                             МатАн   ----           0/10
Пойти в общагу                                   ГиТ     ----           0/3
Поехать в ПОМИ                                   Инф     ----           0/2
Пойти в мавзолей                                 ИнЯз    ПУНК  14-16    0/3
Пойти в компьютерный класс                       Физ-ра  ----           0/1
С меня хватит!
"
    );

    // Отдыхаем до 9:00 и снова идём на факультет
    replay_game(game_ui, "2↓r2↓r4↓r");

    // Кафе закрыто
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 9:00    Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (52)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты на факультете. Что делать?

Идти к преподу▁                                  АиТЧ    ПУНК  13-15    0/12
Посмотреть на баобаб                             МатАн   ----           0/10
Пойти в общагу                                   ГиТ     ----           0/3
Поехать в ПОМИ                                   Инф     ----           0/2
Пойти в мавзолей                                 ИнЯз    ПУНК  14-16    0/3
Пойти в компьютерный класс                       Физ-ра  ----           0/1
С меня хватит!
"
    );

    // Отдыхаем до 10:00 и снова идём на факультет. Заходим в кафе.
    replay_game(game_ui, "2↓r2↓r4↓r6↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 10:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (58)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Что брать будем?


Просто посижу с приятелями.▁                     АиТЧ    ПУНК  13-15    0/12
Я вообще зря сюда зашел.                         МатАн   ----           0/10
                                                 ГиТ     ----           0/3
                                                 Инф     ----           0/2
                                                 ИнЯз    ПУНК  14-16    0/3
                                                 Физ-ра  ----           0/1
"
    );

    // Получаем деньги у Паши, снова идём в кафе
    replay_game(game_ui, "↓r7↓2r6↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::CafePUNK(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert_characteristics!(
                state,
                health: 58,
                money: 50,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
        }
    );
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 10:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (58)                 Мат. Анализ           0   Плохо
Финансы: 50 руб.                            Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Что брать будем?


Чай за 2 р.▁                                     АиТЧ    ПУНК  13-15    0/12
Кекс за 4 р.                                     МатАн   ----           0/10
Чай и выпечку, 6 р.                              ГиТ     ----           0/3
Просто посижу с приятелями.                      Инф     ----           0/2
Я вообще зря сюда зашел.                         ИнЯз    ПУНК  14-16    0/3
                                                 Физ-ра  ----           0/1
"
    );

    // Заказываем чай
    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 11:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (62)                 Мат. Анализ           0   Плохо
Финансы: 48 руб.                            Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты на факультете. Что делать?

Идти к преподу▁                                  АиТЧ    ПУНК  13-15    0/12
Посмотреть на баобаб                             МатАн   ----           0/10
Пойти в общагу                                   ГиТ     ----           0/3
Поехать в ПОМИ                                   Инф     ----           0/2
Пойти в мавзолей                                 ИнЯз    ПУНК  14-16    0/3
Пойти в компьютерный класс                       Физ-ра  ----           0/1
Сходить в кафе
С меня хватит!
"
    );

    // Снова идём в кафе, заказываем кекс
    replay_game(game_ui, "6↓r↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 12:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (69)                 Мат. Анализ           0   Плохо
Финансы: 44 руб.                            Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты на факультете. Что делать?

Идти к преподу▁                                  АиТЧ    ПУНК  13-15    0/12
Посмотреть на баобаб                             МатАн   ----           0/10
Пойти в общагу                                   ГиТ     ----           0/3
Поехать в ПОМИ                                   Инф     ----           0/2
Пойти в мавзолей                                 ИнЯз    ПУНК  14-16    0/3
Пойти в компьютерный класс                       Физ-ра  ----           0/1
Сходить в кафе
С меня хватит!
"
    );

    // Снова идём в кафе, заказываем чай и выпечку
    replay_game(game_ui, "6↓r2↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 13:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (79)                 Мат. Анализ           0   Плохо
Финансы: 38 руб.                            Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты на факультете. Что делать?

Идти к преподу▁                                  АиТЧ    ПУНК  13-15    0/12
Посмотреть на баобаб                             МатАн   ----           0/10
Пойти в общагу                                   ГиТ     ----           0/3
Поехать в ПОМИ                                   Инф     ----           0/2
Пойти в мавзолей                                 ИнЯз    ПУНК  14-16    0/3
Пойти в компьютерный класс                       Физ-ра  ----           0/1
Сходить в кафе
Саша
С меня хватит!
"
    );

    // Снова идём в кафе, просто отдыхаем
    replay_game(game_ui, "6↓r3↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 14:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (83)                 Мат. Анализ           0   Плохо
Финансы: 38 руб.                            Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты на факультете. Что делать?

Идти к преподу▁                                  АиТЧ    ПУНК  13-15    0/12
Посмотреть на баобаб                             МатАн   ----           0/10
Пойти в общагу                                   ГиТ     ----           0/3
Поехать в ПОМИ                                   Инф     ----           0/2
Пойти в мавзолей                                 ИнЯз    ПУНК  14-16    0/3
Пойти в компьютерный класс                       Физ-ра  ----           0/1
Сходить в кафе
С меня хватит!
"
    );

    // Снова идём в кафе, после чего сразу же выходим обратно
    replay_game(game_ui, "6↓r4↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 14:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (83)                 Мат. Анализ           0   Плохо
Финансы: 38 руб.                            Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты на факультете. Что делать?

Идти к преподу▁                                  АиТЧ    ПУНК  13-15    0/12
Посмотреть на баобаб                             МатАн   ----           0/10
Пойти в общагу                                   ГиТ     ----           0/3
Поехать в ПОМИ                                   Инф     ----           0/2
Пойти в мавзолей                                 ИнЯз    ПУНК  14-16    0/3
Пойти в компьютерный класс                       Физ-ра  ----           0/1
Сходить в кафе
С меня хватит!
"
    );

    // Отдыхаем в кафе до 18:00.
    replay_game(game_ui, "6↓r3↓r6↓r3↓r6↓r3↓r6↓r3↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 18:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (92)                 Мат. Анализ           0   Плохо
Финансы: 38 руб.                            Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты на факультете. Что делать?

Идти к преподу▁                                  АиТЧ    ПУНК  13-15    0/12
Посмотреть на баобаб                             МатАн   ----           0/10
Пойти в общагу                                   ГиТ     ----           0/3
Поехать в ПОМИ                                   Инф     ----           0/2
Пойти в мавзолей                                 ИнЯз    ПУНК  14-16    0/3
Пойти в компьютерный класс                       Физ-ра  ----           0/1
Сходить в кафе
С меня хватит!
"
    );

    // Отдыхаем в кафе до 19:00 и убеждаемся что оно закрывается
    replay_game(game_ui, "6↓r3↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 19:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (95)                 Мат. Анализ           0   Плохо
Финансы: 38 руб.                            Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты на факультете. Что делать?

Идти к преподу▁                                  АиТЧ    ПУНК  13-15    0/12
Посмотреть на баобаб                             МатАн   ----           0/10
Пойти в общагу                                   ГиТ     ----           0/3
Поехать в ПОМИ                                   Инф     ----           0/2
Пойти в мавзолей                                 ИнЯз    ПУНК  14-16    0/3
Пойти в компьютерный класс                       Физ-ра  ----           0/1
С меня хватит!
"
    );
}

#[test]
fn cafe_punk_no_tea_with_cake() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем до 10:00, идём на факультет и получаем стипендию
    replay_game(game_ui, "2↓r2↓r4↓r7↓2r");

    // Идём в кафе, заказываем пока не останется 4 рубля
    for _ in 0..7 {
        replay_game(game_ui, "6↓r2↓r");
    }
    replay_game(game_ui, "6↓r↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 18:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (133)                Мат. Анализ           0   Плохо
Финансы: 4 руб.                             Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты на факультете. Что делать?

Идти к преподу▁                                  АиТЧ    ПУНК  13-15    0/12
Посмотреть на баобаб                             МатАн   ----           0/10
Пойти в общагу                                   ГиТ     ----           0/3
Поехать в ПОМИ                                   Инф     ----           0/2
Пойти в мавзолей                                 ИнЯз    ПУНК  14-16    0/3
Пойти в компьютерный класс                       Физ-ра  ----           0/1
Сходить в кафе
Саша
С меня хватит!
"
    );

    // Снова идём в кафе. Меню должно быть меньше, т.к. у нас мало денег.
    replay_game(game_ui, "6↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 18:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (133)                Мат. Анализ           0   Плохо
Финансы: 4 руб.                             Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Что брать будем?


Чай за 2 р.▁                                     АиТЧ    ПУНК  13-15    0/12
Кекс за 4 р.                                     МатАн   ----           0/10
Просто посижу с приятелями.                      ГиТ     ----           0/3
Я вообще зря сюда зашел.                         Инф     ----           0/2
                                                 ИнЯз    ПУНК  14-16    0/3
                                                 Физ-ра  ----           0/1
"
    );
}

#[test]
fn cafe_punk_only_tea_left() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем до 10:00, идём на факультет и получаем стипендию
    replay_game(game_ui, "2↓r2↓r4↓r7↓2r");

    // Идём в кафе, заказываем пока не останется 2 рубля
    for _ in 0..8 {
        replay_game(game_ui, "6↓r2↓r");
    }
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 18:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (136)                Мат. Анализ           0   Плохо
Финансы: 2 руб.                             Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты на факультете. Что делать?

Идти к преподу▁                                  АиТЧ    ПУНК  13-15    0/12
Посмотреть на баобаб                             МатАн   ----           0/10
Пойти в общагу                                   ГиТ     ----           0/3
Поехать в ПОМИ                                   Инф     ----           0/2
Пойти в мавзолей                                 ИнЯз    ПУНК  14-16    0/3
Пойти в компьютерный класс                       Физ-ра  ----           0/1
Сходить в кафе
Саша
С меня хватит!
"
    );

    // Снова идём в кафе. Меню должно быть меньше, т.к. у нас мало денег.
    replay_game(game_ui, "6↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 18:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (136)                Мат. Анализ           0   Плохо
Финансы: 2 руб.                             Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Что брать будем?


Чай за 2 р.▁                                     АиТЧ    ПУНК  13-15    0/12
Просто посижу с приятелями.                      МатАн   ----           0/10
Я вообще зря сюда зашел.                         ГиТ     ----           0/3
                                                 Инф     ----           0/2
                                                 ИнЯз    ПУНК  14-16    0/3
                                                 Физ-ра  ----           0/1
"
    );
}

#[test]
fn mausoleum_rest_without_money() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Идём в мавзолей без денег
    replay_game(game_ui, "6↓r3↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::RestInMausoleum(state) => {
            assert_eq!(state.location(), Location::Mausoleum);
            assert_eq!(state.current_time(), Time(8));
            assert_characteristics!(
                state,
                health: 41,
                money: 0,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
        }
    );
    assert_eq!(
        state.observable_state().available_actions(),
        [Action::RestByOurselvesInMausoleum, Action::NoRestIsNoGood]
    );

    // Отдыхаем, немного улучшая здоровье
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::Mausoleum);
            assert_eq!(state.current_time(), Time(9));
            assert_characteristics!(
                state,
                health: 42,
                money: 0,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
        }
    );
}

#[test]
fn mausoleum_with_money() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Ждём 10:00, идём на факультет и получаем деньги у Паши
    replay_game(game_ui, "2↓r2↓r4↓r7↓2r");

    // Идём в мавзолей с деньгами
    replay_game(game_ui, "4↓r3↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::RestInMausoleum(state) => {
            assert_eq!(state.location(), Location::Mausoleum);
            assert_eq!(state.current_time(), Time(10));
            assert_characteristics!(
                state,
                health: 61,
                money: 50,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
        }
    );
    assert_eq!(
        state.observable_state().available_actions(),
        [
            Action::OrderCola,
            Action::OrderSoup,
            Action::OrderBeer,
            Action::RestByOurselvesInMausoleum,
            Action::NoRestIsNoGood
        ]
    );

    // Заказываем колу
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(11));
            assert_characteristics!(
                state,
                health: 64,
                money: 46,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
        }
    );

    // Ещё колу
    replay_game(game_ui, "3↓2r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(12));
            assert_characteristics!(
                state,
                health: 68,
                money: 42,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
        }
    );

    // Заказываем суп
    replay_game(game_ui, "3↓r↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(13));
            assert_characteristics!(
                state,
                health: 74,
                money: 36,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
        }
    );

    // Ещё суп
    replay_game(game_ui, "3↓r↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(14));
            assert_characteristics!(
                state,
                health: 80,
                money: 30,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
        }
    );

    // Заказываем пиво, немного увеличиваем здоровье и выносливость, но тупеем
    replay_game(game_ui, "3↓r2↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(15));
            assert_characteristics!(
                state,
                health: 81,
                money: 22,
                brain: 4,
                stamina: 5,
                charisma: 5,
            );
        }
    );

    // Заказываем ещё пиво
    replay_game(game_ui, "3↓r2↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(16));
            assert_characteristics!(
                state,
                health: 81,
                money: 14,
                brain: 4,
                stamina: 6,
                charisma: 5,
            );
        }
    );

    // Заказываем ещё пиво
    replay_game(game_ui, "3↓r2↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 84,
                money: 6,
                brain: 4,
                stamina: 6,
                charisma: 5,
            );
        }
    );

    // Проверяем, что больше не можем купить пиво
    replay_game(game_ui, "3↓r");
    assert_eq!(
        state.observable_state().available_actions(),
        [
            Action::OrderCola,
            Action::OrderSoup,
            Action::RestByOurselvesInMausoleum,
            Action::NoRestIsNoGood,
        ],
    );

    // Покупаем суп
    replay_game(game_ui, "↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(18));
            assert_characteristics!(
                state,
                health: 92,
                money: 0,
                brain: 4,
                stamina: 6,
                charisma: 5,
            );
        }
    );

    // Возвращаемся отдыхать и сразу идём назад, проверяем что ничего не изменилось
    replay_game(game_ui, "3↓r↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(18));
            assert_characteristics!(
                state,
                health: 92,
                money: 0,
                brain: 4,
                stamina: 6,
                charisma: 5,
            );
        }
    );
}

#[test]
fn mausoleum_death_of_alcoholism() {
    initialize_game!((7, GameMode::SelectInitialParameters) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::ImpudentStudent);

    // Ждём 10:00, идём на факультет и получаем деньги у Паши
    replay_game(game_ui, "2↓r2↓r4↓r7↓2r");

    // Идём в мавзолей с деньгами
    replay_game(game_ui, "4↓r3↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::RestInMausoleum(state) => {
            assert_eq!(state.location(), Location::Mausoleum);
            assert_eq!(state.current_time(), Time(10));
            assert_characteristics!(
                state,
                health: 66,
                money: 50,
                brain: 2,
                stamina: 9,
                charisma: 2,
            );
        }
    );

    // Заказываем пиво
    replay_game(game_ui, "2↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(11));
            assert_characteristics!(
                state,
                health: 66,
                money: 42,
                brain: 2,
                stamina: 9,
                charisma: 3,
            );
        }
    );

    // Ещё пиво
    replay_game(game_ui, "3↓r2↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(12));
            assert_characteristics!(
                state,
                health: 68,
                money: 34,
                brain: 2,
                stamina: 9,
                charisma: 3,
            );
        }
    );

    // Ещё пиво
    replay_game(game_ui, "3↓r2↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(13));
            assert_characteristics!(
                state,
                health: 69,
                money: 26,
                brain: 1,
                stamina: 9,
                charisma: 3,
            );
        }
    );

    // Ещё пиво
    replay_game(game_ui, "3↓r2↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GameEnd(state) => {
            assert_eq!(state.current_time(), Time(14));
            assert_eq!(
                state.player().cause_of_death(),
                Some(CauseOfDeath::BeerAlcoholism)
            );
            assert_characteristics!(
                state,
                health: 0,
                money: 18,
                brain: 0,
                stamina: 10,
                charisma: 4,
            );
        }
    );
}
