mod common;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::GameMode;

#[test]
fn overstudy_to_zero_health() {
    initialize_game!((1641333345581, GameMode::Normal) => state, game_ui);
    replay_game(game_ui, "13r");
    assert_ui!(
        game_ui,
        "
Легче лбом колоть орехи,
чем учиться на МАТ-МЕХе.
Заучился.




















Нажми любую клавишу ...▁
    "
    );
    replay_game(game_ui, "2r");
    assert_ui!(
        game_ui,
        "
ДЗИНЬ!
ДДДЗЗЗЗЗИИИИИИННННННЬ !!!!
ДДДДДДЗЗЗЗЗЗЗЗЗЗЗЗЗИИИИИИИИИИННННННННННННЬ !!!!!!!!!!
Ты просыпаешься от звонка будильника 22-го мая в 8:00.
Неожиданно ты осознаешь, что началась зачетная неделя,
а твоя готовность к этому моменту практически равна нулю.
Натягивая на себя скромное одеяние студента,
ты всматриваешься в заботливо оставленное соседом на стене
расписание: когда и где можно найти искомого препода ?














Нажми любую клавишу ...▁
    "
    );
    replay_game(game_ui, "2r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 8:00    Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (49)                 Мат. Анализ           4   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           4   Плохо
Нас ждут великие дела (6)                   English               4   Плохо
У тебя много друзей (5)                     Физ-ра                2   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ПУНК  9-13     0/12
Посмотреть расписание                            МатАн   ----           0/10
Отдыхать                                         ГиТ     ПОМИ  11-14    0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ПУНК  11-13    0/3
Поехать в ПОМИ                                   Физ-ра  ПУНК  9-10     0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
    "
    );
}

/// Проверяем, что в случае отрицательного brain level попытка подготовиться к зачёту
/// ни к чему не приводит: знание предмета не увеличивается, время не тратится.
#[test]
fn study_with_negative_brain_level() {
    initialize_game!((1641336778475, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);
    replay_game(game_ui, "2↓r2↓r4↓r7↓2r4↓r3↑r↓2r3↑r↓2r4↓r↓2r3↑r↑2r3↑r↑2r3↑r↑2r3↑2r3↑2r2↑2r3↑2r3↑2r2↑r↓3r2↓2r");

    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 15:00   Версия gamma3.14   Алгебра и Т.Ч.        5   Плохо
Самочувствие: отличное (63)                 Мат. Анализ           0   Плохо
Финансы: 50 руб.                            Геометрия и Топология 5   Плохо
Клиническая смерть мозга (-1)               Информатика           4   Плохо
Нас ждут великие дела (6)                   English               4   Плохо
У тебя очень много друзей (6)               Физ-ра                2   Плохо

К чему готовиться?

Алгебра и Т.Ч.▁                                  АиТЧ    ПУНК  13-16   10/12
Мат. Анализ                                      МатАн   ----           0/10
Геометрия и Топология                            ГиТ     ПОМИ  12-15    0/3
Информатика                                      Инф     ----           0/2
English                                          ИнЯз    ----           0/3
Физ-ра                                           Физ-ра  ПУНК  9-10     0/1
Ни к чему
    "
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 15:00   Версия gamma3.14   Алгебра и Т.Ч.        5   Плохо
Самочувствие: отличное (63)                 Мат. Анализ           0   Плохо
Финансы: 50 руб.                            Геометрия и Топология 5   Плохо
Клиническая смерть мозга (-1)               Информатика           4   Плохо
Нас ждут великие дела (6)                   English               4   Плохо
У тебя очень много друзей (6)               Физ-ра                2   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ПУНК  13-16   10/12
Посмотреть расписание                            МатАн   ----           0/10
Отдыхать                                         ГиТ     ПОМИ  12-15    0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ----           0/3
Поехать в ПОМИ                                   Физ-ра  ПУНК  9-10     0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
    "
    );
}

#[test]
fn died_of_studying_to_well() {
    initialize_game!((0, GameMode::SelectInitialParameters) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::SociableStudent);

    // Ждём когда на факультет приходит Саша
    replay_game(game_ui, "2↓r2↓r");

    // Идём на факультет, обращаемся к Саше
    replay_game(game_ui, "4↓r2↑r");

    // С трёх попыток Саша соглашается дать нам конспект по геометрии
    replay_game(game_ui, "2r2↑r↓2r2↑r2↓2r");

    // Идём в общагу
    replay_game(game_ui, "2↓r");

    // Готовимся к геометрии 9 раз
    for _ in 0..9 {
        replay_game(game_ui, "r2↓2r");
    }

    assert_ui!(
        game_ui,
        "
Легче лбом колоть орехи,
чем учиться на МАТ-МЕХе.
Зубрежка до добра не доводит!




















Нажми любую клавишу ...▁
    "
    );
}
