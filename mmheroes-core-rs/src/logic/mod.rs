pub mod timetable;
pub use timetable::*;

pub mod characteristics;
pub use characteristics::*;

pub mod game_state;
pub use game_state::*;

pub mod subjects;
pub use subjects::*;

pub mod npc;

pub use npc::*;

pub mod player;

pub use player::Player;

pub mod subject_status;

pub use subject_status::SubjectStatus;

#[macro_use]
pub mod actions;

pub use actions::*;

use npc::Classmate::*;

use crate::random;

use assert_matches::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CauseOfDeath {
    /// Умер по пути на факультет.
    OnTheWayToPUNK,

    /// Умер по пути в мавзолей.
    OnTheWayToMausoleum,

    /// Умер по пути домой. Бывает.
    OnTheWayToDorm,

    /// Упал с лестницы у главного входа.
    FellFromStairs,

    /// Сгорел на работе.
    Burnout,

    /// Заучился.
    Overstudied,

    /// Зубрежка до добра не доводит!
    StudiedTooWell,

    /// Не смог расстаться с компьютером.
    CouldntLeaveTheComputer,

    /// В электричке нашли бездыханное тело.
    CorpseFoundInTheTrain,

    /// Контролеры жизни лишили.
    KilledByInspectors,

    /// Заснул в электричке и не проснулся.
    FellAsleepInTheTrain,

    /// Раздвоение ложной личности.
    SplitPersonality,

    /// Пивной алкоголизм, батенька...
    BeerAlcoholism,

    /// Спился.
    DrankTooMuch,

    /// Губит людей не пиво, а избыток пива.
    DrankTooMuchBeer,

    /// Альтруизм не довел до добра.
    Altruism,

    /// Превратился в овощ.
    TurnedToVegetable,

    /// <препод> замучил.
    TorturedByProfessor(Subject),

    /// Бурно прогрессирующая паранойя
    Paranoia,

    /// Время вышло.
    TimeOut,

    /// Вышел сам.
    Suicide,

    /// Раздавлен безжалостной ошибкой в программе.
    SoftwareBug,
}

/// Максимальное число возможных вариантов на главном экране.
pub const MAX_OPTIONS_IN_SCENE_ROUTER: usize = 12;

#[derive(Debug)]
pub enum GameScreen {
    /// Самый первый экран, который видит пользователь.
    Intro,

    /// Экран, который видит пользователь, если запускает игру с каким-то аргументом.
    /// Предлагает выбрать стиль игры.
    InitialParameters,

    /// Экран с предысторией ("ты просыпаешься от звонка будильника...")
    Ding(Player),

    /// Экран с расписанием.
    Timetable(GameState),

    /// Главный экран.
    SceneRouter(GameState),

    /// Выбор предмета для подготовки к зачёту.
    Study(GameState),

    /// "Воспользуюсь конспектом" или "Буду учиться как умею"
    PromptUseLectureNotes(GameState),

    /// Экран с рекордами — баобаб в ПУНКе или доска объявлений в ПОМИ.
    HighScores(GameState),

    /// Отдых в мавзолее.
    RestInMausoleum(GameState),

    /// Кафе в ПУНКе
    CafePUNK(GameState),

    /// Взаимодействие с Колей.
    KolyaInteraction(GameState, npc::KolyaInteraction),

    /// Взаимодействие с Пашей.
    PashaInteraction(GameState, npc::PashaInteraction),

    /// Взаимодействие с Гришей.
    GrishaInteraction(GameState, npc::GrishaInteraction),

    /// Взаимодействие с Сашей.
    SashaInteraction(GameState, npc::SashaInteraction),

    /// Взаимодействие с Кузьменко.
    KuzmenkoInteraction(GameState, npc::KuzmenkoInteraction),

    /// Экран "Идти к преподу"
    GoToProfessor(GameState),

    /// Экран сдачи зачёта.
    Exam(GameState, Subject),

    // TODO: Добавить больше параметров. Сейчас поддерживается только "не тянет поспать"
    /// Сон.
    Sleep(GameState),

    /// Посидеть в интернете. Если второй аргумент `true`, это означает, что
    /// герой нашёл в интернете решение задачи по информатике.
    SurfInternet(GameState, bool),

    /// Экран "ты серьёзно хочешь закончить игру?"
    IAmDone(GameState),

    /// Финальный экран с описанием причины смерти/отчисления, либо поздравлением.
    GameEnd(GameState),

    /// Пользователю предлагается либо повторить игру, либо выйти.
    WannaTryAgain,

    /// Экран, который отображается пользователю, если он решил выйти из игры.
    Disclaimer,

    /// Экран помощи с описанием цели игры.
    WhatToDo(GameState),

    /// Экран помощи с описанием главного экрана.
    AboutScreen(GameState),

    /// Экран помощи с описанием локаций.
    WhereToGoAndWhy(GameState),

    /// Экран помощи с описанием преподавателей.
    AboutProfessors(GameState),

    /// Экран помощи с описанием NPC-шек.
    AboutCharacters(GameState),

    /// Экран помощи с информацией о программе.
    AboutThisProgram(GameState),

    /// Терминальное состояние. Ему тоже соответствует никакой экран.
    /// Игра завершена безвозвратно.
    Terminal,
}

/// The game mode selector.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(C)]
pub enum GameMode {
    /// Normal game mode, the character has average characteristics.
    /// This is the default.
    Normal,

    /// The player will be prompted to select initial character characteristics:
    /// - Random student, same as `Normal` mode.
    /// - Clever student: better brain, but worse stamina and charisma
    /// - Impudent student: better stamina, but worse brain and charisma
    /// - Sociable student: better charisma, but worse brain and stamina.
    SelectInitialParameters,

    /// Same as `SelectInitialParameters`, but another option is available —
    /// God mode. It enables maximum abilities.
    ///
    /// This mode is enable by passing a special flag to the executable.
    God,
}

use GameScreen::*;

pub struct Game {
    screen: GameScreen,
    rng: random::Rng,
    mode: GameMode,
    available_actions: ActionVec,
}

impl Game {
    pub fn new(mode: GameMode, seed: u64) -> Game {
        let rng = random::Rng::new(seed);
        Game {
            screen: Intro,
            rng,
            mode,
            available_actions: wait_for_any_key(),
        }
    }

    pub fn screen(&self) -> &GameScreen {
        &self.screen
    }

    pub fn mode(&self) -> GameMode {
        self.mode
    }

    /// Возвращает текущее состояние игры, если оно доступно.
    /// Оно может быть недоступно, например, если игра ещё не началась
    /// или уже закончилась.
    pub fn game_state(&self) -> Option<&GameState> {
        match &self.screen {
            Timetable(state)
            | SceneRouter(state)
            | Study(state)
            | PromptUseLectureNotes(state)
            | Sleep(state)
            | HighScores(state)
            | IAmDone(state)
            | GameEnd(state)
            | WhatToDo(state)
            | AboutScreen(state)
            | WhereToGoAndWhy(state)
            | AboutProfessors(state)
            | AboutCharacters(state)
            | AboutThisProgram(state)
            | KolyaInteraction(state, _)
            | PashaInteraction(state, _)
            | GrishaInteraction(state, _)
            | SashaInteraction(state, _)
            | KuzmenkoInteraction(state, _)
            | GoToProfessor(state)
            | Exam(state, _)
            | SurfInternet(state, _)
            | RestInMausoleum(state)
            | CafePUNK(state) => Some(state),
            Intro | InitialParameters | Ding(_) | WannaTryAgain | Disclaimer
            | Terminal => None,
        }
    }

    pub fn available_actions(&self) -> &[Action] {
        &*self.available_actions
    }

    pub fn perform_action(&mut self, action: Action) {
        self.available_actions = self._perform_action(action)
    }

    /// Accepts an action, returns the number of actions available in the updated state.
    fn _perform_action(&mut self, action: Action) -> ActionVec {
        match &self.screen {
            Terminal => panic!("Attempted to perform an action in terminal state"),
            Intro => self.start_game(),
            InitialParameters => self.ding(action),
            Ding(player) => {
                let state = GameState::new(
                    player.clone(),
                    timetable::Timetable::random(&mut self.rng),
                    Location::Dorm,
                );
                self.view_timetable(state)
            }
            Timetable(state) => {
                let state = state.clone();
                self.scene_router(state)
            }
            SceneRouter(state) => {
                let state = state.clone();
                self.handle_scene_router_action(state, action)
            }
            Study(state) => {
                let state = state.clone();
                self.choose_use_lecture_notes(state, action)
            }
            PromptUseLectureNotes(state) => {
                let state = state.clone();
                let (subject, use_lecture_notes) = match action {
                    Action::UseLectureNotes(subject) => (subject, true),
                    Action::DontUseLectureNotes(subject) => (subject, false),
                    _ => illegal_action!(action),
                };
                self.study(state, subject, use_lecture_notes)
            }
            Sleep(state) => {
                let state = state.clone();
                self.handle_sleeping(state, action)
            }
            HighScores(state) => match action {
                Action::AnyKey => {
                    let state = state.clone();
                    self.scene_router(state)
                }
                _ => illegal_action!(action),
            },
            RestInMausoleum(state) => {
                let state = state.clone();
                self.handle_rest_in_mausoleum(state, action)
            }
            CafePUNK(state) => {
                let state = state.clone();
                self.handle_cafe_punk_action(state, action)
            }
            KolyaInteraction(state, interaction) => {
                let state = state.clone();
                let interaction = *interaction;
                self.proceed_with_kolya(state, action, interaction)
            }
            PashaInteraction(state, interaction) => {
                let state = state.clone();
                let interaction = *interaction;
                self.proceed_with_pasha(state, action, interaction)
            }
            GrishaInteraction(state, interaction) => {
                let state = state.clone();
                let interaction = *interaction;
                self.proceed_with_grisha(state, action, interaction)
            }
            SashaInteraction(state, interaction) => {
                let state = state.clone();
                let interaction = *interaction;
                self.proceed_with_sasha(state, action, interaction)
            }
            KuzmenkoInteraction(state, _) => {
                assert_eq!(action, Action::AnyKey);
                let state = state.clone();
                self.scene_router(state)
            }
            GoToProfessor(state) => match action {
                Action::Exam(subject) => {
                    let state = state.clone();
                    self.enter_exam(state, subject)
                }
                Action::DontGoToProfessor => {
                    let state = state.clone();
                    self.scene_router(state)
                }
                _ => illegal_action!(action),
            },
            Exam(state, subject) => {
                todo!()
            }
            SurfInternet(state, found_program) => {
                let state = state.clone();
                let found_program = *found_program;
                self.proceed_with_internet(state, action, found_program)
            }
            IAmDone(state) => {
                let state = state.clone();
                self.handle_i_am_done(state, action)
            }
            GameEnd(_) => self.wanna_try_again(),
            WannaTryAgain => self.handle_wanna_try_again(action),
            Disclaimer => {
                self.screen = Terminal;
                ActionVec::new()
            }
            WhatToDo(state)
            | AboutScreen(state)
            | WhereToGoAndWhy(state)
            | AboutProfessors(state)
            | AboutCharacters(state)
            | AboutThisProgram(state) => {
                let state = state.clone();
                self.handle_what_to_do(state, action)
            }
        }
    }

    fn start_game(&mut self) -> ActionVec {
        match self.mode {
            GameMode::SelectInitialParameters => {
                self.screen = InitialParameters;
                // Можно выбрать 4 стиля игры:
                // - Случайный студент
                // - Шибко умный
                // - Шибко наглый
                // - Шибко общительный
                ActionVec::from([
                    Action::RandomStudent,
                    Action::CleverStudent,
                    Action::ImpudentStudent,
                    Action::SociableStudent,
                ])
            }
            GameMode::God => {
                self.screen = InitialParameters;
                // Можно выбрать 5 стилей игры:
                // - Случайный студент
                // - Шибко умный
                // - Шибко наглый
                // - Шибко общительный
                // - GOD-режим
                ActionVec::from([
                    Action::RandomStudent,
                    Action::CleverStudent,
                    Action::ImpudentStudent,
                    Action::SociableStudent,
                    Action::GodMode,
                ])
            }
            GameMode::Normal => self.ding(Action::RandomStudent),
        }
    }

    fn ding(&mut self, action: Action) -> ActionVec {
        self.screen = Ding(self.initialize_player(action));
        wait_for_any_key()
    }

    fn initialize_player(&mut self, parameters: Action) -> Player {
        let (god_mode, brain, stamina, charisma) = match parameters {
            Action::RandomStudent => (
                false,
                BrainLevel(self.rng.random_in_range(4..7)),
                StaminaLevel(self.rng.random_in_range(4..7)),
                CharismaLevel(self.rng.random_in_range(4..7)),
            ),
            Action::CleverStudent => (
                false,
                BrainLevel(self.rng.random_in_range(5..10)),
                StaminaLevel(self.rng.random_in_range(2..5)),
                CharismaLevel(self.rng.random_in_range(2..5)),
            ),
            Action::ImpudentStudent => (
                false,
                BrainLevel(self.rng.random_in_range(2..5)),
                StaminaLevel(self.rng.random_in_range(5..10)),
                CharismaLevel(self.rng.random_in_range(2..5)),
            ),
            Action::SociableStudent => (
                false,
                BrainLevel(self.rng.random_in_range(2..5)),
                StaminaLevel(self.rng.random_in_range(2..5)),
                CharismaLevel(self.rng.random_in_range(5..10)),
            ),
            Action::GodMode => {
                (true, BrainLevel(30), StaminaLevel(30), CharismaLevel(30))
            }
            _ => illegal_action!(parameters),
        };

        let health = HealthLevel(self.rng.random(stamina.0 * 2) + 40);

        Player::new(god_mode, health, brain, stamina, charisma, |_| {
            self.rng.random(brain)
        })
    }

    fn scene_router(&mut self, state: GameState) -> ActionVec {
        // TODO: assert that no exam is in progress
        let location = state.location;
        let available_actions = match location {
            Location::PDMI => todo!(),
            Location::PUNK => {
                let mut available_actions = ActionVec::from([
                    Action::GoToProfessor,
                    Action::LookAtBaobab,
                    Action::GoFromPunkToDorm,
                    Action::GoToPDMI,
                    Action::GoToMausoleum,
                ]);
                if state.current_time < Time::computer_class_closing() {
                    available_actions.push(Action::GoToComputerClass);
                }
                if state.current_time.is_cafe_open() {
                    available_actions.push(Action::GoToCafePUNK);
                }
                for classmate_info in state.classmates.filter_by_location(location) {
                    available_actions
                        .push(Action::InteractWithClassmate(classmate_info.classmate()));
                }
                if state.player.is_employed_at_terkom() {
                    available_actions.push(Action::GoToWork);
                }
                available_actions.push(Action::IAmDone);
                available_actions
            }
            Location::Mausoleum => {
                let mut available_actions = ActionVec::from([
                    Action::GoFromMausoleumToPunk,
                    Action::GoToPDMI,
                    Action::GoFromMausoleumToDorm,
                    Action::Rest,
                ]);
                for classmate_info in state.classmates.filter_by_location(location) {
                    available_actions
                        .push(Action::InteractWithClassmate(classmate_info.classmate()));
                }
                available_actions.push(Action::IAmDone);
                available_actions
            }
            Location::Dorm => ActionVec::from([
                Action::Study,
                Action::ViewTimetable,
                Action::Rest,
                Action::GoToBed,
                Action::GoFromDormToPunk,
                Action::GoToPDMI,
                Action::GoToMausoleum,
                Action::IAmDone,
                Action::WhatToDo,
            ]),
            Location::ComputerClass => {
                if state.current_time > Time::computer_class_closing() {
                    todo!("Класс закрывается. Пошли домой!")
                }
                let mut available_actions = ActionVec::new();
                if location.is_exam_here_now(
                    Subject::ComputerScience,
                    state.current_day(),
                    state.current_time,
                ) {
                    available_actions.push(Action::Exam(Subject::ComputerScience));
                }
                available_actions.push(Action::GoFromPunkToDorm);
                available_actions.push(Action::LeaveComputerClass);
                available_actions.push(Action::GoToPDMI);
                available_actions.push(Action::GoToMausoleum);
                if state.player.has_internet() {
                    available_actions.push(Action::SurfInternet);
                }
                if state.player.has_mmheroes_floppy() {
                    available_actions.push(Action::PlayMMHEROES);
                }
                for classmate_info in state.classmates.filter_by_location(location) {
                    available_actions
                        .push(Action::InteractWithClassmate(classmate_info.classmate()));
                }
                available_actions.push(Action::IAmDone);
                available_actions
            }
        };
        self.screen = GameScreen::SceneRouter(state);
        available_actions
    }

    fn handle_scene_router_action(
        &mut self,
        state: GameState,
        action: Action,
    ) -> ActionVec {
        match state.location() {
            Location::PUNK => self.handle_punk_action(state, action),
            Location::PDMI => todo!(),
            Location::ComputerClass => self.handle_computer_class_action(state, action),
            Location::Dorm => self.handle_dorm_action(state, action),
            Location::Mausoleum => self.handle_mausoleum_action(state, action),
        }
    }

    fn enter_exam(&mut self, state: GameState, subject: Subject) -> ActionVec {
        todo!()
    }

    fn handle_sleeping(&mut self, state: GameState, action: Action) -> ActionVec {
        // TODO: Реализовать что-то помимо неудавшегося сна
        assert_matches!(self.screen, GameScreen::Sleep(_));
        assert_eq!(action, Action::AnyKey);
        self.scene_router(state)
    }

    fn handle_dorm_action(&mut self, mut state: GameState, action: Action) -> ActionVec {
        assert_eq!(state.location, Location::Dorm);
        match action {
            Action::Study => self.choose_subject_to_study(state),
            Action::ViewTimetable => self.view_timetable(state),
            Action::Rest => self.rest_in_dorm(state),
            Action::GoToBed => self.try_to_sleep(state),
            Action::GoFromDormToPunk => {
                state.location = Location::PUNK;
                self.decrease_health(
                    HealthLevel::location_change_large_penalty(),
                    state,
                    CauseOfDeath::OnTheWayToPUNK,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::GoToPDMI => todo!("Go to PDMI"),
            Action::GoToMausoleum => {
                state.location = Location::Mausoleum;
                self.decrease_health(
                    HealthLevel::location_change_large_penalty(),
                    state,
                    CauseOfDeath::OnTheWayToMausoleum,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::IAmDone => self.i_am_done(state),
            Action::WhatToDo => self.handle_what_to_do(state, Action::WhatToDoAtAll),
            _ => illegal_action!(action),
        }
    }

    fn choose_subject_to_study(&mut self, state: GameState) -> ActionVec {
        let mut available_actions = SUBJECTS
            .iter()
            .map(|(subject, _)| Action::DoStudy {
                subject: *subject,
                lecture_notes_available: state
                    .player
                    .status_for_subject(*subject)
                    .has_lecture_notes(),
            })
            .collect::<ActionVec>();
        available_actions.push(Action::DontStudy);
        self.screen = GameScreen::Study(state);
        available_actions
    }

    fn choose_use_lecture_notes(
        &mut self,
        state: GameState,
        action: Action,
    ) -> ActionVec {
        match action {
            Action::DoStudy {
                subject,
                lecture_notes_available,
            } => {
                assert_eq!(
                    state.player.status_for_subject(subject).has_lecture_notes(),
                    lecture_notes_available
                );
                if lecture_notes_available {
                    self.screen = GameScreen::PromptUseLectureNotes(state);
                    ActionVec::from([
                        Action::UseLectureNotes(subject),
                        Action::DontUseLectureNotes(subject),
                    ])
                } else {
                    self.study(state, subject, false)
                }
            }
            Action::DontStudy => self.scene_router(state),
            _ => illegal_action!(action),
        }
    }

    fn study(
        &mut self,
        mut state: GameState,
        subject: Subject,
        use_lecture_notes: bool,
    ) -> ActionVec {
        // Импликация "использовать конспект => у игрока есть конспект"
        // должна быть истинной
        assert!(
            !use_lecture_notes
                || state.player.status_for_subject(subject).has_lecture_notes(),
            "Нельзя воспользоваться конспектом, так как его на самом деле нет"
        );
        let brain_or_stamina = if subject == Subject::PhysicalEducation {
            state.player.stamina.0
        } else {
            state.player.brain.0
        };
        if brain_or_stamina <= 0 {
            return self.scene_router(state);
        }
        let health = state.player.health;
        let knowledge = &mut state.player.status_for_subject_mut(subject).knowledge;
        *knowledge += if state.current_time.is_optimal_study_time() {
            brain_or_stamina
        } else {
            brain_or_stamina * 2 / 3
        };
        *knowledge -= self.rng.random(brain_or_stamina / 2);
        *knowledge += self.rng.random(health.0 / 18);
        if use_lecture_notes {
            *knowledge += 10
        }
        assert!(*knowledge >= BrainLevel(0));
        assert!(state.player.stamina >= StaminaLevel(0));
        let mut health_penalty = 10 - self.rng.random(state.player.stamina.0);
        if health_penalty < 0 || use_lecture_notes {
            health_penalty = 0;
        }
        if state.current_time.is_suboptimal_study_time() {
            health_penalty += 12;
        }

        self.decrease_health(
            HealthLevel(health_penalty),
            state,
            CauseOfDeath::Overstudied,
            |game, state| {
                if state
                    .player
                    .status_for_subject(subject)
                    .knowledge
                    .is_lethal()
                {
                    game.decrease_health(
                        HealthLevel(10),
                        state,
                        CauseOfDeath::StudiedTooWell,
                        |game, state| game.hour_pass(state),
                    )
                } else {
                    game.hour_pass(state)
                }
            },
        )
    }

    fn interact_with_pasha(&mut self, state: GameState) -> ActionVec {
        assert_eq!(state.location, Location::PUNK);
        let interaction = if state.player.got_stipend() {
            npc::PashaInteraction::Inspiration
        } else {
            npc::PashaInteraction::Stipend
        };
        self.screen = GameScreen::PashaInteraction(state, interaction);
        wait_for_any_key()
    }

    fn proceed_with_pasha(
        &mut self,
        mut state: GameState,
        action: Action,
        interaction: npc::PashaInteraction,
    ) -> ActionVec {
        assert_eq!(action, Action::AnyKey);
        assert_eq!(state.location, Location::PUNK);
        assert_matches!(self.screen, GameScreen::PashaInteraction(_, _));
        let player = &mut state.player;
        match interaction {
            npc::PashaInteraction::Stipend => {
                assert!(!player.got_stipend());
                player.set_got_stipend();
                player.money += Money::stipend();
            }
            npc::PashaInteraction::Inspiration => {
                player.stamina += 1;
                for (subject, _) in SUBJECTS.iter() {
                    let knowledge =
                        &mut player.status_for_subject_mut(*subject).knowledge;
                    if *knowledge > BrainLevel(3) {
                        *knowledge -= self.rng.random(3);
                    }
                }
            }
        }
        self.scene_router(state)
    }

    fn interact_with_sasha(&mut self, state: GameState) -> ActionVec {
        assert_eq!(state.location, Location::PUNK);
        let mut available_actions = SUBJECTS_WITH_LECTURE_NOTES
            .into_iter()
            .filter(|subject| {
                !state
                    .player
                    .status_for_subject(*subject)
                    .has_lecture_notes()
            })
            .map(Action::RequestLectureNotesFromSasha)
            .collect::<ActionVec>();
        available_actions.push(Action::DontNeedAnythingFromSasha);
        self.screen =
            GameScreen::SashaInteraction(state, npc::SashaInteraction::ChooseSubject);
        available_actions
    }

    fn proceed_with_sasha(
        &mut self,
        mut state: GameState,
        action: Action,
        interaction: npc::SashaInteraction,
    ) -> ActionVec {
        assert_eq!(state.location, Location::PUNK);
        assert_matches!(self.screen, GameScreen::SashaInteraction(_, _));
        match action {
            Action::RequestLectureNotesFromSasha(subject) => {
                assert_eq!(interaction, npc::SashaInteraction::ChooseSubject);
                let new_interaction = if state.player.charisma
                    > CharismaLevel(self.rng.random(18))
                    && state.sasha_has_lecture_notes(subject)
                {
                    state
                        .player
                        .status_for_subject_mut(subject)
                        .set_has_lecture_notes();
                    npc::SashaInteraction::YesIHaveTheLectureNotes
                } else {
                    state.set_sasha_has_lecture_notes(subject, false);
                    npc::SashaInteraction::SorryGaveToSomeoneElse
                };
                self.screen = GameScreen::SashaInteraction(state, new_interaction);
                wait_for_any_key()
            }
            Action::DontNeedAnythingFromSasha => {
                assert_eq!(interaction, npc::SashaInteraction::ChooseSubject);
                self.screen = GameScreen::SashaInteraction(
                    state,
                    npc::SashaInteraction::SuitYourself,
                );
                wait_for_any_key()
            }
            Action::AnyKey => {
                assert_ne!(interaction, npc::SashaInteraction::ChooseSubject);
                self.scene_router(state)
            }
            _ => illegal_action!(action),
        }
    }

    fn handle_punk_action(&mut self, mut state: GameState, action: Action) -> ActionVec {
        assert_eq!(state.location, Location::PUNK);
        match action {
            Action::GoToProfessor => {
                let mut available_actions = state
                    .current_day()
                    .current_exams(state.location, state.current_time)
                    .map(|exam| Action::Exam(exam.subject()))
                    .collect::<ActionVec>();
                available_actions.push(Action::DontGoToProfessor);
                self.screen = GameScreen::GoToProfessor(state);
                available_actions
            }
            Action::LookAtBaobab => {
                self.screen = GameScreen::HighScores(state);
                wait_for_any_key()
            }
            Action::GoFromPunkToDorm => {
                state.location = Location::Dorm;
                self.scene_router(state)
            }
            Action::GoToPDMI => todo!(),
            Action::GoToMausoleum => {
                state.location = Location::Mausoleum;
                self.decrease_health(
                    HealthLevel::location_change_large_penalty(),
                    state,
                    CauseOfDeath::OnTheWayToMausoleum,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::GoToComputerClass => {
                assert!(state.current_time < Time::computer_class_closing());
                state.location = Location::ComputerClass;
                self.decrease_health(
                    HealthLevel::location_change_small_penalty(),
                    state,
                    CauseOfDeath::FellFromStairs,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::GoToCafePUNK => {
                assert!(state.current_time.is_cafe_open());
                let mut available_actions = ActionVec::new();
                let available_money = state.player.money;
                if available_money >= Money::tea_cost() {
                    available_actions.push(Action::OrderTea);
                }
                if available_money >= Money::cake_cost() {
                    available_actions.push(Action::OrderCake);
                }
                if available_money >= Money::tea_with_cake_cost() {
                    available_actions.push(Action::OrderTeaWithCake);
                }
                available_actions.push(Action::RestInCafePUNK);
                available_actions.push(Action::ShouldntHaveComeToCafePUNK);
                self.screen = GameScreen::CafePUNK(state);
                available_actions
            }
            Action::InteractWithClassmate(classmate) => {
                assert_matches!(
                    state.classmates[classmate].current_location(),
                    ClassmateLocation::Location(Location::PUNK)
                );
                self.handle_classmate_interaction(state, classmate)
            }
            Action::GoToWork => {
                assert!(state.player.is_employed_at_terkom());
                todo!()
            }
            Action::IAmDone => self.i_am_done(state),
            _ => illegal_action!(action),
        }
    }

    fn handle_cafe_punk_action(
        &mut self,
        mut state: GameState,
        action: Action,
    ) -> ActionVec {
        assert_eq!(state.location, Location::PUNK);
        assert!(state.current_time.is_cafe_open());
        assert_matches!(self.screen, GameScreen::CafePUNK(_));
        let money = &mut state.player.money;
        let health = &mut state.player.health;
        let charisma_dependent_health_gain =
            HealthLevel(self.rng.random(state.player.charisma.0));
        match action {
            Action::OrderTea => {
                *money -= Money::tea_cost();
                *health += charisma_dependent_health_gain + 2;
            }
            Action::OrderCake => {
                *money -= Money::cake_cost();
                *health += charisma_dependent_health_gain + 4;
            }
            Action::OrderTeaWithCake => {
                *money -= Money::tea_with_cake_cost();
                *health += charisma_dependent_health_gain + 7;
            }
            Action::RestInCafePUNK => {
                *health += charisma_dependent_health_gain;
            }
            Action::ShouldntHaveComeToCafePUNK => {
                return self.scene_router(state);
            }
            _ => illegal_action!(action),
        }
        self.hour_pass(state)
    }

    fn handle_classmate_interaction(
        &mut self,
        state: GameState,
        classmate: Classmate,
    ) -> ActionVec {
        match classmate {
            Kolya => self.interact_with_kolya(state),
            Pasha => self.interact_with_pasha(state),
            Diamond => todo!(),
            RAI => todo!(),
            Misha => todo!(),
            Serj => todo!(),
            Sasha => self.interact_with_sasha(state),
            NiL => todo!(),
            Kuzmenko => self.interact_with_kuzmenko(state),
            DJuG => todo!(),
            Andrew => todo!(),
            Grisha => self.interact_with_grisha(state),
        }
    }

    fn surf_internet(&mut self, state: GameState) -> ActionVec {
        let player = &state.player;
        let cs_problems_done = player
            .status_for_subject(Subject::ComputerScience)
            .problems_done();
        let cs_problems_required = SUBJECTS[Subject::ComputerScience].required_problems;
        let found_program = player.is_god_mode()
            || (self.rng.random(player.brain) > BrainLevel(6)
                && cs_problems_done < cs_problems_required);
        self.screen = GameScreen::SurfInternet(state, found_program);
        wait_for_any_key()
    }

    fn proceed_with_internet(
        &mut self,
        mut state: GameState,
        action: Action,
        found_program: bool,
    ) -> ActionVec {
        assert_eq!(action, Action::AnyKey);
        if found_program {
            state
                .player
                .status_for_subject_mut(Subject::ComputerScience)
                .more_problems_solved(1);
        } else if state.player.brain < BrainLevel(5) && self.rng.roll_dice(3) {
            state.player.brain += 1;
        }
        self.hour_pass(state)
    }

    fn interact_with_kuzmenko(&mut self, mut state: GameState) -> ActionVec {
        use npc::KuzmenkoInteraction::*;
        let tomorrow = state.current_day_index + 1;
        let saturday = 5;
        let mut additional_exam_day_idx: Option<usize> = None;
        if tomorrow <= saturday {
            for i in (tomorrow..=saturday).rev() {
                let day = &mut state.timetable.days_mut()[i];
                let has_enough_charisma =
                    state.player.charisma > self.rng.random(CharismaLevel(18));
                let can_add_exam = day.exam(Subject::ComputerScience).is_none();
                if has_enough_charisma && can_add_exam {
                    let exam_start_time = Time(10u8 + self.rng.random(5));
                    let exam_end_time =
                        exam_start_time + Duration(1i8 + self.rng.random(2));
                    let additional_exam = timetable::Exam::new(
                        Subject::ComputerScience,
                        exam_start_time,
                        exam_end_time,
                        Location::ComputerClass,
                    );
                    day.add_exam(additional_exam);
                    additional_exam_day_idx = Some(i);
                    break;
                }
            }
        }

        self.screen = match additional_exam_day_idx {
            // Проверка на `state.additional_computer_science_exams() < 2` должна быть
            // раньше — до модификации расписания.
            // Иначе получается, что при достаточной харизме Кузьменко может
            // добавить экзамены по информатике на каждый день.
            // Баг в оригинальной реализации. Возможно, стоит исправить, но
            // пока не буду.
            Some(additional_exam_day_idx)
                if state.additional_computer_science_exams() < 2 =>
            {
                state.add_additional_computer_science_exam();
                GameScreen::KuzmenkoInteraction(
                    state,
                    AdditionalComputerScienceExam {
                        day_index: additional_exam_day_idx,
                    },
                )
            }
            _ => {
                let replies = [
                    FormatFloppy,
                    FiltersInWindows,
                    ByteVisualization,
                    OlegPliss,
                    BillGatesMustDie,
                    MonitorJournal,
                    MmheroesBP7,
                    CSeminar,
                    ThirdYear,
                    STAR,
                    GetYourselvesAnEmail,
                    TerekhovSenior,
                ];
                GameScreen::KuzmenkoInteraction(state, *self.rng.random_element(&replies))
            }
        };

        wait_for_any_key()
    }

    fn handle_computer_class_action(
        &mut self,
        mut state: GameState,
        action: Action,
    ) -> ActionVec {
        assert_eq!(state.location, Location::ComputerClass);
        match action {
            Action::Exam(Subject::ComputerScience) => todo!(),
            Action::GoFromPunkToDorm => {
                state.location = Location::Dorm;
                self.scene_router(state)
            }
            Action::LeaveComputerClass => {
                state.location = Location::PUNK;
                self.decrease_health(
                    HealthLevel::location_change_small_penalty(),
                    state,
                    CauseOfDeath::CouldntLeaveTheComputer,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::GoToPDMI => todo!(),
            Action::GoToMausoleum => {
                state.location = Location::Mausoleum;
                self.decrease_health(
                    HealthLevel::location_change_small_penalty(),
                    state,
                    CauseOfDeath::OnTheWayToMausoleum,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::SurfInternet => self.surf_internet(state),
            Action::InteractWithClassmate(classmate) => {
                assert_matches!(
                    state.classmates[classmate].current_location(),
                    ClassmateLocation::Location(Location::ComputerClass)
                );
                self.handle_classmate_interaction(state, classmate)
            }
            Action::PlayMMHEROES => todo!(),
            Action::IAmDone => self.i_am_done(state),
            _ => illegal_action!(action),
        }
    }

    /// Возвращает `Some`, если Коля может помочь решить задачи по алгебре,
    /// иначе — `None`.
    fn kolya_maybe_solve_algebra_problems(
        &mut self,
        player: &mut Player,
    ) -> Option<ActionVec> {
        use Subject::AlgebraAndNumberTheory;
        let has_enough_charisma = player.charisma > self.rng.random(CharismaLevel(10));
        let algebra = player.status_for_subject(AlgebraAndNumberTheory);
        let problems_done = algebra.problems_done();
        let required_problems = SUBJECTS[AlgebraAndNumberTheory].required_problems;
        let has_at_least_2_remaining_problems = problems_done + 2 <= required_problems;
        if has_enough_charisma && has_at_least_2_remaining_problems {
            Some(wait_for_any_key())
        } else {
            None
        }
    }

    fn interact_with_kolya(&mut self, mut state: GameState) -> ActionVec {
        assert_eq!(state.location, Location::Mausoleum);
        let player = &mut state.player;
        let (available_actions, interaction) = if let Some(available_actions) =
            self.kolya_maybe_solve_algebra_problems(player)
        {
            (
                available_actions,
                npc::KolyaInteraction::SolvedAlgebraProblemsForFree,
            )
        } else if player.money < Money::oat_tincture_cost() {
            // "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
            (wait_for_any_key(), npc::KolyaInteraction::BrakeFluidNoMoney)
        } else {
            // "Знаешь, пиво, конечно, хорошо, но настойка овса - лучше!"
            // "Заказать Коле настойку овса?"
            (
                ActionVec::from([Action::Yes, Action::No]),
                npc::KolyaInteraction::PromptOatTincture,
            )
        };
        self.screen = GameScreen::KolyaInteraction(state, interaction);
        available_actions
    }

    fn proceed_with_kolya(
        &mut self,
        mut state: GameState,
        action: Action,
        interaction: npc::KolyaInteraction,
    ) -> ActionVec {
        assert_eq!(state.location, Location::Mausoleum);
        assert_matches!(self.screen, GameScreen::KolyaInteraction(_, _));
        let player = &mut state.player;
        match action {
            Action::AnyKey => {
                let algebra_status =
                    player.status_for_subject_mut(Subject::AlgebraAndNumberTheory);
                match interaction {
                    npc::KolyaInteraction::SolvedAlgebraProblemsForFree => {
                        algebra_status.more_problems_solved(2);
                        return self.hour_pass(state);
                    }
                    npc::KolyaInteraction::PromptOatTincture => unreachable!(),
                    npc::KolyaInteraction::SolvedAlgebraProblemsForOatTincture => {
                        algebra_status.more_problems_solved(2);
                        player.money -= Money::oat_tincture_cost();
                        return self.hour_pass(state);
                    }
                    npc::KolyaInteraction::BrakeFluidNoMoney => {
                        player.brain -= 1;
                        if player.brain <= BrainLevel(0) {
                            player.health = HealthLevel(0);
                            player.cause_of_death = Some(CauseOfDeath::DrankTooMuch);
                            return self.game_end(state);
                        }
                    }
                    npc::KolyaInteraction::BrakeFluidBecauseRefused => {
                        state.player.brain -= 1;
                        // Забавно, что в этой ветке можно бесконечно пить тормозную
                        // жидкость и никогда не спиться. Баг в оригинальной реализации.
                    }
                    npc::KolyaInteraction::Altruism => {
                        player.money -= Money::oat_tincture_cost();
                    }
                }
                self.scene_router(state)
            }
            Action::Yes => {
                assert_eq!(interaction, npc::KolyaInteraction::PromptOatTincture);
                if let Some(num_actions) = self.kolya_maybe_solve_algebra_problems(player)
                {
                    // "Коля решил тебе ещё 2 задачи по алгебре!"
                    self.screen = GameScreen::KolyaInteraction(
                        state,
                        npc::KolyaInteraction::SolvedAlgebraProblemsForOatTincture,
                    );
                    num_actions
                } else {
                    // "Твой альтруизм навсегда останется в памяти потомков."
                    self.screen = GameScreen::KolyaInteraction(
                        state,
                        npc::KolyaInteraction::Altruism,
                    );
                    wait_for_any_key()
                }
            }
            Action::No => {
                assert_eq!(interaction, npc::KolyaInteraction::PromptOatTincture);
                // "Зря, ой, зря ..."
                // "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
                self.screen = GameScreen::KolyaInteraction(
                    state,
                    npc::KolyaInteraction::BrakeFluidBecauseRefused,
                );
                wait_for_any_key()
            }
            _ => illegal_action!(action),
        }
    }

    fn interact_with_grisha(&mut self, state: GameState) -> ActionVec {
        use npc::GrishaInteraction::*;
        assert_eq!(state.location, Location::Mausoleum);
        let player = &state.player;
        let has_enough_charisma = player.charisma > self.rng.random(CharismaLevel(20));
        let (actions, interaction) =
            if !player.is_employed_at_terkom() && has_enough_charisma {
                (
                    ActionVec::from([
                        Action::AcceptEmploymentAtTerkom,
                        Action::DeclineEmploymentAtTerkom,
                    ]),
                    PromptEmploymentAtTerkom,
                )
            } else if !player.has_internet() && has_enough_charisma {
                (wait_for_any_key(), ProxyAddress)
            } else {
                let drink_beer = self.rng.random(3) > 0;
                let hour_pass = self.rng.roll_dice(3);
                let replies = [
                    WantFreebie {
                        drink_beer,
                        hour_pass,
                    },
                    FreebieComeToMe {
                        drink_beer,
                        hour_pass,
                    },
                    FreebieExists {
                        drink_beer,
                        hour_pass,
                    },
                    LetsOrganizeFreebieLoversClub {
                        drink_beer,
                        hour_pass,
                    },
                    NoNeedToStudyToGetDiploma {
                        drink_beer,
                        hour_pass,
                    },
                    YouStudiedDidItHelp {
                        drink_beer,
                        hour_pass,
                    },
                    ThirdYearStudentsDontAttendLectures {
                        drink_beer,
                        hour_pass,
                    },
                    TakeExampleFromKolya {
                        drink_beer,
                        hour_pass,
                    },
                    HateLevTolstoy {
                        drink_beer,
                        hour_pass,
                    },
                    DontGoToPDMI {
                        drink_beer,
                        hour_pass,
                    },
                    NamesOfFreebieLovers {
                        drink_beer,
                        hour_pass,
                    },
                    LetsHaveABreakHere {
                        drink_beer,
                        hour_pass,
                    },
                    NoNeedToTakeLectureNotes {
                        drink_beer,
                        hour_pass,
                    },
                    CantBeExpelledInFourthYear {
                        drink_beer,
                        hour_pass,
                    },
                    MechanicsHaveFreebie {
                        drink_beer,
                        hour_pass,
                    },
                ];
                (wait_for_any_key(), *self.rng.random_element(&replies[..]))
            };
        self.screen = GameScreen::GrishaInteraction(state, interaction);
        actions
    }

    fn proceed_with_grisha(
        &mut self,
        mut state: GameState,
        action: Action,
        interaction: npc::GrishaInteraction,
    ) -> ActionVec {
        use npc::GrishaInteraction::*;
        assert_matches!(self.screen, GameScreen::GrishaInteraction(_, _));
        let player = &mut state.player;
        match action {
            Action::AnyKey => match interaction {
                PromptEmploymentAtTerkom => unreachable!(),
                CongratulationsYouAreNowEmployed | AsYouWantButDontOverstudy => {
                    self.scene_router(state)
                }
                ProxyAddress => {
                    assert!(!player.has_internet());
                    player.set_has_internet();
                    self.scene_router(state)
                }
                WantFreebie {
                    drink_beer,
                    hour_pass,
                }
                | FreebieComeToMe {
                    drink_beer,
                    hour_pass,
                }
                | FreebieExists {
                    drink_beer,
                    hour_pass,
                }
                | LetsOrganizeFreebieLoversClub {
                    drink_beer,
                    hour_pass,
                }
                | NoNeedToStudyToGetDiploma {
                    drink_beer,
                    hour_pass,
                }
                | YouStudiedDidItHelp {
                    drink_beer,
                    hour_pass,
                }
                | ThirdYearStudentsDontAttendLectures {
                    drink_beer,
                    hour_pass,
                }
                | TakeExampleFromKolya {
                    drink_beer,
                    hour_pass,
                }
                | HateLevTolstoy {
                    drink_beer,
                    hour_pass,
                }
                | DontGoToPDMI {
                    drink_beer,
                    hour_pass,
                }
                | NamesOfFreebieLovers {
                    drink_beer,
                    hour_pass,
                }
                | LetsHaveABreakHere {
                    drink_beer,
                    hour_pass,
                }
                | NoNeedToTakeLectureNotes {
                    drink_beer,
                    hour_pass,
                }
                | CantBeExpelledInFourthYear {
                    drink_beer,
                    hour_pass,
                }
                | MechanicsHaveFreebie {
                    drink_beer,
                    hour_pass,
                } => {
                    if drink_beer {
                        player.brain -= self.rng.random(2);
                        if player.brain <= BrainLevel(0) {
                            player.health = HealthLevel(0);
                            player.cause_of_death = Some(CauseOfDeath::DrankTooMuchBeer);
                            return self.game_end(state);
                        }
                        player.charisma += self.rng.random(2);
                    }
                    if hour_pass {
                        return self.hour_pass(state);
                    }

                    self.scene_router(state)
                }
            },
            Action::AcceptEmploymentAtTerkom => {
                assert_eq!(interaction, PromptEmploymentAtTerkom);
                assert!(!player.is_employed_at_terkom());
                player.set_employed_at_terkom();
                self.screen = GameScreen::GrishaInteraction(
                    state,
                    CongratulationsYouAreNowEmployed,
                );
                wait_for_any_key()
            }
            Action::DeclineEmploymentAtTerkom => {
                assert_eq!(interaction, PromptEmploymentAtTerkom);
                assert!(!player.is_employed_at_terkom());
                self.screen =
                    GameScreen::GrishaInteraction(state, AsYouWantButDontOverstudy);
                wait_for_any_key()
            }
            _ => illegal_action!(action),
        }
    }

    fn handle_rest_in_mausoleum(
        &mut self,
        mut state: GameState,
        action: Action,
    ) -> ActionVec {
        let player = &mut state.player;
        match action {
            Action::OrderCola => {
                assert!(player.money >= Money::cola_cost());
                player.money -= Money::cola_cost();
                player.health += self.rng.random(player.charisma.0) + 3;
            }
            Action::OrderSoup => {
                assert!(player.money >= Money::soup_cost());
                player.money -= Money::soup_cost();
                player.health += self.rng.random(player.charisma.0) + 5;
            }
            Action::OrderBeer => {
                assert!(player.money >= Money::beer_cost());
                player.money -= Money::beer_cost();
                if self.rng.roll_dice(3) {
                    player.brain -= 1;
                }
                if self.rng.roll_dice(3) {
                    player.charisma += 1;
                }
                if self.rng.roll_dice(3) {
                    player.stamina += 2;
                }
                player.health += self.rng.random(player.charisma.0);
                if player.brain <= BrainLevel(0) {
                    player.health = HealthLevel(0);
                    player.cause_of_death = Some(CauseOfDeath::BeerAlcoholism);
                    return self.game_end(state);
                }
            }
            Action::RestByOurselvesInMausoleum => {
                player.health += self.rng.random(player.charisma.0);
            }
            Action::NoRestIsNoGood => return self.scene_router(state),
            _ => illegal_action!(action),
        }

        self.hour_pass(state)
    }

    fn handle_mausoleum_action(
        &mut self,
        mut state: GameState,
        action: Action,
    ) -> ActionVec {
        assert!(state.location == Location::Mausoleum);
        match action {
            Action::GoFromMausoleumToPunk => {
                state.location = Location::PUNK;
                self.decrease_health(
                    HealthLevel::location_change_large_penalty(),
                    state,
                    CauseOfDeath::OnTheWayToPUNK,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::GoToPDMI => todo!(),
            Action::GoFromMausoleumToDorm => {
                state.location = Location::Dorm;
                self.scene_router(state)
            }
            Action::Rest => {
                let money = state.player.money;
                self.screen = GameScreen::RestInMausoleum(state);
                let mut available_actions = ActionVec::new();
                if money >= Money::cola_cost() {
                    available_actions.push(Action::OrderCola);
                }
                if money >= Money::soup_cost() {
                    available_actions.push(Action::OrderSoup);
                }
                if money >= Money::beer_cost() {
                    available_actions.push(Action::OrderBeer);
                }
                available_actions.push(Action::RestByOurselvesInMausoleum);
                available_actions.push(Action::NoRestIsNoGood);
                available_actions
            }
            Action::InteractWithClassmate(classmate) => {
                assert_matches!(
                    state.classmates[classmate].current_location(),
                    ClassmateLocation::Location(Location::Mausoleum)
                );
                self.handle_classmate_interaction(state, classmate)
            }
            Action::IAmDone => self.i_am_done(state),
            _ => illegal_action!(action),
        }
    }

    fn view_timetable(&mut self, state: GameState) -> ActionVec {
        self.screen = Timetable(state);
        wait_for_any_key()
    }

    fn decrease_health<F: FnOnce(&mut Game, GameState) -> ActionVec>(
        &mut self,
        delta: HealthLevel,
        mut state: GameState,
        cause_of_death: CauseOfDeath,
        if_alive: F,
    ) -> ActionVec {
        if state.player.health <= delta {
            state.player.cause_of_death = Some(cause_of_death);
            self.game_end(state)
        } else {
            state.player.health -= delta;
            if_alive(self, state)
        }
    }

    fn try_to_sleep(&mut self, state: GameState) -> ActionVec {
        assert!(state.location == Location::Dorm);
        if state.current_time > Time(3) && state.current_time < Time(20) {
            self.screen = GameScreen::Sleep(state);
            wait_for_any_key()
        } else {
            self.go_to_sleep(state)
        }
    }

    fn go_to_sleep(&mut self, _state: GameState) -> ActionVec {
        todo!()
    }

    fn midnight(&mut self, state: GameState) -> ActionVec {
        match state.location {
            Location::PUNK => todo!("sub_1E907"),
            Location::PDMI => todo!("sub_1E7F8"),
            Location::ComputerClass => {
                unreachable!("Компьютерный класс уже должен быть закрыт в полночь!")
            }
            Location::Dorm => self.go_to_sleep(state),
            Location::Mausoleum => todo!("sub_1E993"),
        }
    }

    fn run_classmate_routines(&mut self, state: &mut GameState) {
        let timetable = &state.timetable;
        let day = &timetable.days()[state.current_day_index];
        let time = state.current_time;
        let classmates = &mut state.classmates;
        for classmate in classmates.iter_mut() {
            classmate.update(&mut self.rng, state.location, day, time);
        }
    }

    fn hour_pass(&mut self, mut state: GameState) -> ActionVec {
        // TODO: Lot of stuff going on here

        // TODO: Поменять эти строки местами и не забыть отредактировать метод
        // Time::is_between_9_and_19()!
        self.run_classmate_routines(&mut state);
        state.current_time += Duration(1);

        if state.current_time.is_midnight() {
            state.current_day_index += 1;
            state.current_time = Time(0);
            return self.midnight(state);
        }

        self.scene_router(state)
    }

    fn rest_in_dorm(&mut self, mut state: GameState) -> ActionVec {
        state.player.health += self.rng.random_in_range(7..15);
        self.hour_pass(state)
    }

    fn i_am_done(&mut self, state: GameState) -> ActionVec {
        self.screen = IAmDone(state);
        ActionVec::from([Action::NoIAmNotDone, Action::IAmCertainlyDone])
    }

    fn handle_i_am_done(&mut self, state: GameState, action: Action) -> ActionVec {
        match action {
            Action::NoIAmNotDone => self.scene_router(state),
            Action::IAmCertainlyDone => self.game_end(state),
            _ => illegal_action!(action),
        }
    }

    fn game_end(&mut self, state: GameState) -> ActionVec {
        self.screen = GameEnd(state);
        wait_for_any_key()
    }

    fn wanna_try_again(&mut self) -> ActionVec {
        self.screen = WannaTryAgain;
        // Хочешь попробовать снова? Да или нет.
        ActionVec::from([Action::WantToTryAgain, Action::DontWantToTryAgain])
    }

    fn handle_wanna_try_again(&mut self, action: Action) -> ActionVec {
        match action {
            Action::WantToTryAgain => self.start_game(),
            Action::DontWantToTryAgain => {
                self.screen = Disclaimer;
                wait_for_any_key()
            }
            _ => illegal_action!(action),
        }
    }

    fn handle_what_to_do(&mut self, state: GameState, action: Action) -> ActionVec {
        assert_eq!(state.location(), Location::Dorm);
        match action {
            Action::WhatToDoAtAll => self.screen = WhatToDo(state),
            Action::AboutScreen => self.screen = AboutScreen(state),
            Action::WhereToGoAndWhy => self.screen = WhereToGoAndWhy(state),
            Action::AboutProfessors => self.screen = AboutProfessors(state),
            Action::AboutCharacters => self.screen = AboutCharacters(state),
            Action::AboutThisProgram => self.screen = AboutThisProgram(state),
            Action::ThanksButNothing => {
                return self.scene_router(state);
            }
            _ => illegal_action!(action),
        };
        ActionVec::from([
            Action::WhatToDoAtAll,
            Action::AboutScreen,
            Action::WhereToGoAndWhy,
            Action::AboutProfessors,
            Action::AboutCharacters,
            Action::AboutThisProgram,
            Action::ThanksButNothing,
        ])
    }
}

#[test]
fn memory() {
    use core::mem::size_of;

    assert_eq!(size_of::<Game>(), 376);
    assert_eq!(size_of::<Player>(), 40);
    assert_eq!(size_of::<Action>(), 3);
}
