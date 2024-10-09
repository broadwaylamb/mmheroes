#ifndef MMHEROES_CORE_H
#define MMHEROES_CORE_H

/* ------------------------------------------------------------------------------------ */
/*                  Warning! This file has been autogenerated by cbindgen.              */
/*                               Don't modify it manually.                              */
/* ------------------------------------------------------------------------------------ */

#include <stdint.h>
#include <stdbool.h>

/**
 * Максимальное число возможных вариантов на главном экране.
 */
#define MMHEROES_MAX_OPTIONS_IN_SCENE_ROUTER 12

#define MMHEROES_NUM_DAYS 6

#define MMHEROES_NUM_SUBJECTS 6

#define MMHEROES_SCORE_COUNT 5

#define MMHEROES_RECORD_SIZE 35

#define MMHEROES_BUFFER_SIZE (MMHEROES_SCORE_COUNT * MMHEROES_RECORD_SIZE)

#define MMHEROES_MAX_NAME_LENGTH 32

typedef enum MMHEROES_Color {
  MMHEROES_Color_Black = 0,
  MMHEROES_Color_Red = 1,
  MMHEROES_Color_Yellow = 3,
  MMHEROES_Color_Blue = 4,
  MMHEROES_Color_Magenta = 5,
  MMHEROES_Color_Cyan = 6,
  MMHEROES_Color_White = 7,
  MMHEROES_Color_Gray = 8,
  MMHEROES_Color_RedBright = 9,
  MMHEROES_Color_Green = 10,
  MMHEROES_Color_YellowBright = 11,
  MMHEROES_Color_BlueBright = 12,
  MMHEROES_Color_MagentaBright = 13,
  MMHEROES_Color_CyanBright = 14,
  MMHEROES_Color_WhiteBright = 15,
} MMHEROES_Color;

/**
 * The game mode selector.
 */
typedef enum MMHEROES_GameMode {
  /**
   * Normal game mode, the character has average characteristics.
   * This is the default.
   */
  MMHEROES_GameMode_Normal,
  /**
   * The player will be prompted to select initial character characteristics:
   * - Random student, same as `Normal` mode.
   * - Clever student: better brain, but worse stamina and charisma
   * - Impudent student: better stamina, but worse brain and charisma
   * - Sociable student: better charisma, but worse brain and stamina.
   */
  MMHEROES_GameMode_SelectInitialParameters,
  /**
   * Same as `SelectInitialParameters`, but another option is available —
   * God mode. It enables maximum abilities.
   *
   * This mode is enabled by passing a special flag to the executable.
   */
  MMHEROES_GameMode_God,
} MMHEROES_GameMode;

typedef enum MMHEROES_Input {
  MMHEROES_Input_KeyUp,
  MMHEROES_Input_KeyDown,
  MMHEROES_Input_Enter,
  MMHEROES_Input_Other,
} MMHEROES_Input;

typedef struct MMHEROES_InputRecorder_InputRecorderSink MMHEROES_InputRecorder_InputRecorderSink;

/**
 * Количество часов, прошедших с полуночи.
 *
 * Имеет семантику таймстэмпа, то есть, экземпляры этого типа нельзя складывать,
 * но к ним можно прибавлять экземпляры типа `Duration` и получать новый экземпляр
 * типа `Time`.
 */
typedef uint8_t MMHEROES_Time;

typedef int16_t MMHEROES_Money;

typedef struct MMHEROES_HighScore {
  const uint8_t *name;
  uintptr_t name_len;
  MMHEROES_Money score;
} MMHEROES_HighScore;

typedef void *MMHEROES_AllocatorContext;

/**
 * Функция, принимающая в качестве первого аргумента некоторый контекст,
 * в качестве второго аргумента размер выделяемого блока памяти,
 * а в качестве третьего — выравнивание.
 */
typedef void *(*MMHEROES_Allocator)(MMHEROES_AllocatorContext, uintptr_t, uintptr_t);

typedef int32_t MMHEROES_Milliseconds;

typedef enum MMHEROES_RendererRequest_Tag {
  MMHEROES_RendererRequest_ClearScreen,
  MMHEROES_RendererRequest_Flush,
  MMHEROES_RendererRequest_WriteStr,
  MMHEROES_RendererRequest_MoveCursor,
  MMHEROES_RendererRequest_SetColor,
  MMHEROES_RendererRequest_Sleep,
} MMHEROES_RendererRequest_Tag;

typedef struct MMHEROES_RendererRequest_MMHEROES_WriteStr_Body {
  const uint8_t *buf;
  uintptr_t length;
} MMHEROES_RendererRequest_MMHEROES_WriteStr_Body;

typedef struct MMHEROES_RendererRequest_MMHEROES_MoveCursor_Body {
  uint8_t line;
  uint8_t column;
} MMHEROES_RendererRequest_MMHEROES_MoveCursor_Body;

typedef struct MMHEROES_RendererRequest_MMHEROES_SetColor_Body {
  enum MMHEROES_Color foreground;
  enum MMHEROES_Color background;
} MMHEROES_RendererRequest_MMHEROES_SetColor_Body;

typedef struct MMHEROES_RendererRequest_MMHEROES_Sleep_Body {
  MMHEROES_Milliseconds milliseconds;
} MMHEROES_RendererRequest_MMHEROES_Sleep_Body;

typedef struct MMHEROES_RendererRequest {
  MMHEROES_RendererRequest_Tag tag;
  union {
    MMHEROES_RendererRequest_MMHEROES_WriteStr_Body write_str;
    MMHEROES_RendererRequest_MMHEROES_MoveCursor_Body move_cursor;
    MMHEROES_RendererRequest_MMHEROES_SetColor_Body set_color;
    MMHEROES_RendererRequest_MMHEROES_Sleep_Body sleep;
  };
} MMHEROES_RendererRequest;

typedef void (*MMHEROES_RendererRequestCallback)(void*, struct MMHEROES_RendererRequest);

/**
 * Функция, принимающая в качестве первого аргумента некоторый контекст,
 * в качестве второго — указатель на освобождаемый блок памяти,
 * а в качестве третьего — размер освобождаемого блока.
 */
typedef void (*MMHEROES_Deallocator)(MMHEROES_AllocatorContext, void*, uintptr_t);

typedef struct MMHEROES_InputRecorderSink {
  void *context;
  bool (*sink)(void*, const uint8_t*, uintptr_t);
} MMHEROES_InputRecorderSink;

#define MMHEROES_WORKDAY_BEGINS 9

#define MMHEROES_WORKDAY_ENDS 18

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Записывает текущий игровой день и время в аргументы `out_day` и `out_time`
 * и возвращает `true` если они доступны, иначе не трогает аргументы и возвращает
 * `false`.
 *
 * Игровой день и время могут быть недоступны, например, если игра ещё не началась.
 */
bool mmheroes_game_get_current_time(const void *game,
                                    uint8_t *out_day,
                                    MMHEROES_Time *out_time);

/**
 * Выделяет память для объекта, используя переданный аллокатор,
 * а затем инициализирует объект и возвращает на него указатель.
 *
 * Аллокатор должен возвращать корректно выровненный указатель на блок памяти
 * достаточного размера. Нарушение любого из этих условий — неопределённое поведение.
 *
 * Размер и выравнивание передаются в качестве аргументов аллокатору.
 *
 * Параметр `high_scores` — указатель (возможно нулевой) на массив из
 * `MMHEROES_SCORE_COUNT` элементов.
 */
void *mmheroes_game_create(enum MMHEROES_GameMode mode,
                           uint64_t seed,
                           const struct MMHEROES_HighScore *high_scores,
                           MMHEROES_AllocatorContext allocator_context,
                           MMHEROES_Allocator allocator,
                           void *renderer_request_callback_context,
                           MMHEROES_RendererRequestCallback renderer_request_callback);

void mmheroes_game_destroy(void *game,
                           MMHEROES_AllocatorContext allocator_context,
                           MMHEROES_Deallocator deallocator);

/**
 * Записывает в аргумент `out` `MMHEROES_SCORE_COUNT` элементов.
 * `out` не должен быть нулевым указателем.
 * Результат, записанный в `out`, не должен жить дольше, чем экземпляр
 * соответствующего `GameUI`.
 */
void mmheroes_game_get_high_scores(const void *game,
                                   struct MMHEROES_HighScore *out);

/**
 * `new_high_scores` — ненулевой указатель на массив из `MMHEROES_SCORE_COUNT` элементов.
 */
void mmheroes_game_set_high_scores(void *game,
                                   const struct MMHEROES_HighScore *new_high_scores);

/**
 * Воспроизводит игру с помощью входных данных, записанных ранее с помощью
 * `InputRecorder`.
 *
 * В случае ошибки возвращает `false`, иначе — `true`.
 */
bool mmheroes_replay(void *game,
                     const uint8_t *recorded_input,
                     uintptr_t recorded_input_len);

/**
 * Продолжает игру до следующего запроса на нажатие клавиши.
 *
 * При первом вызове этой функции неважно, что передаётся в параметре `input`.
 */
bool mmheroes_continue(void *game,
                       enum MMHEROES_Input input);

struct MMHEROES_InputRecorder_InputRecorderSink *mmheroes_input_recorder_create(struct MMHEROES_InputRecorderSink *sink,
                                                                                MMHEROES_AllocatorContext allocator_context,
                                                                                MMHEROES_Allocator allocator);

void mmheroes_input_recorder_destroy(struct MMHEROES_InputRecorder_InputRecorderSink *recorder,
                                     MMHEROES_AllocatorContext deallocator_context,
                                     MMHEROES_Deallocator deallocator);

bool mmheroes_input_recorder_record(struct MMHEROES_InputRecorder_InputRecorderSink *recorder,
                                    enum MMHEROES_Input input);

bool mmheroes_input_recorder_flush(struct MMHEROES_InputRecorder_InputRecorderSink *recorder);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* MMHEROES_CORE_H */
