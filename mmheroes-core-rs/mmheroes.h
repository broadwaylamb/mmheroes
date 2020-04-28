#ifndef MMHEROES_CORE_H
#define MMHEROES_CORE_H

/* -------------------------------------------------------------------------- */
/*             Warning! This file has been autogenerated by cbindgen.         */
/*                          Don't modify it manually.                         */
/* -------------------------------------------------------------------------- */

#include <stdint.h>

typedef enum {
  MMHEROES_Color_Black = 0,
  MMHEROES_Color_White = 7,
  MMHEROES_Color_Gray = 8,
  MMHEROES_Color_Red = 9,
  MMHEROES_Color_Green = 10,
  MMHEROES_Color_Yellow = 11,
  MMHEROES_Color_Cyan = 14,
  MMHEROES_Color_WhiteBright = 15,
} MMHEROES_Color;

/**
 * The game mode selector.
 */
typedef enum {
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
   * This mode is enable by passing a special flag to the executable.
   */
  MMHEROES_GameMode_God,
} MMHEROES_GameMode;

typedef enum {
  MMHEROES_Input_KeyUp,
  MMHEROES_Input_KeyDown,
  MMHEROES_Input_Enter,
  MMHEROES_Input_Other,
  MMHEROES_Input_EOF,
} MMHEROES_Input;

typedef void *MMHEROES_RendererContext;

typedef int32_t MMHEROES_Milliseconds;

/**
 * A renderer for use in non-Rust clients.
 * Set its fields to the necessary values yourself.
 */
typedef struct {
  /**
   * An opaque object that will be passed as a first argument of
   * the renderer functions.
   * For example, if you implement the renderer using curses, this will be
   * the window object.
   */
  MMHEROES_RendererContext renderer_ctx;
  void (*clear_screen)(MMHEROES_RendererContext);
  void (*flush)(MMHEROES_RendererContext);
  void (*move_cursor_to)(MMHEROES_RendererContext, int32_t, int32_t);
  void (*get_cursor_position)(MMHEROES_RendererContext, int32_t*, int32_t*);
  void (*set_color)(MMHEROES_RendererContext, MMHEROES_Color, MMHEROES_Color);
  void (*write_str)(MMHEROES_RendererContext, const char*, uintptr_t);
  MMHEROES_Input (*getch)(MMHEROES_RendererContext);
  void (*sleep_ms)(MMHEROES_RendererContext, MMHEROES_Milliseconds);
} MMHEROES_PolymorphicRenderer;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void mmheroes_run_game(MMHEROES_PolymorphicRenderer *renderer,
                       MMHEROES_GameMode mode,
                       uint64_t seed);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* MMHEROES_CORE_H */
