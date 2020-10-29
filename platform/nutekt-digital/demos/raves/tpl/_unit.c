/*
    BSD 3-Clause License

    Copyright (c) 2018, KORG INC.
    All rights reserved.

    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright notice, this
      list of conditions and the following disclaimer.

    * Redistributions in binary form must reproduce the above copyright notice,
      this list of conditions and the following disclaimer in the documentation
      and/or other materials provided with the distribution.

    * Neither the name of the copyright holder nor the names of its
      contributors may be used to endorse or promote products derived from
      this software without specific prior written permission.

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
    IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
    FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
    OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
    OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//*/

/**
 * @file    _unit.c
 * @brief   Oscillator entry template.
 *
 * @addtogroup api
 * @{
 */

#include "userosc.h"

/*===========================================================================*/
/* Externs and Types.                                                        */
/*===========================================================================*/

/**
 * @name   Externs and Types.
 * @{
 */

extern uint8_t _bss_start;
extern uint8_t _bss_end;

extern void (*__init_array_start []) (void);
extern void (*__init_array_end []) (void);

typedef void (*__init_fptr)(void);

/** @} */

/*===========================================================================*/
/* Locals Constants and Vars.                                                */
/*===========================================================================*/

/**
 * @name   Local Constants and Vars.
 * @{
 */

__attribute__((used, section(".hooks")))
static const user_osc_hook_table_t s_hook_table = {
  .magic = {'U','O','S','C'},
  .api = USER_API_VERSION,
  .platform = USER_TARGET_PLATFORM>>8,
  .reserved0 = {0},
  .func_entry = _entry,
  .func_cycle = _hook_cycle,
  .func_on = _hook_on,
  .func_off = _hook_off,
  .func_mute = _hook_mute,
  .func_value = _hook_value,
  .func_param = _hook_param,
  .reserved1 = {0}
};

/** @} */

/*===========================================================================*/
/* Default Hooks.                                                             */
/*===========================================================================*/

/**
 * @name   Default Hooks.
 * @{
 */

__attribute__((used))
void _entry(uint32_t platform, uint32_t api)
{
  // Ensure zero-clear BSS segment
  uint8_t * __restrict bss_p = (uint8_t *)&_bss_start;
  const uint8_t * const bss_e = (uint8_t *)&_bss_end;

  for (; bss_p != bss_e;)
    *(bss_p++) = 0;

  // Call constructors if any.
  const size_t count = __init_array_end - __init_array_start;
  for (size_t i = 0; i<count; ++i) {
    __init_fptr init_p = (__init_fptr)__init_array_start[i];
    if (init_p != NULL)
      init_p();
  }

  // Call user initialization
  _hook_init(platform, api);
}

extern void r_osc_init(uint32_t platform, uint32_t api);
extern void r_osc_cycle(const user_osc_param_t * const params,
                        int32_t *yn, const uint32_t frames);
extern void r_osc_param(uint16_t index, uint16_t value);
extern void r_osc_noteon(const user_osc_param_t * const params);
extern void r_osc_noteoff(const user_osc_param_t * const params);

void OSC_INIT(uint32_t platform, uint32_t api)
{
  r_osc_init(platform, api);
}

void OSC_CYCLE(const user_osc_param_t * const params, int32_t *yn, const uint32_t frames)
{
  r_osc_cycle(params, yn, frames);
}

void OSC_NOTEON(const user_osc_param_t * const params)
{
  r_osc_noteon(params);
}

void OSC_NOTEOFF(const user_osc_param_t * const params)
{
  r_osc_noteoff(params);
}

void OSC_MUTE(const user_osc_param_t * const params)
{
  (void)params;
}

void OSC_VALUE(uint16_t value)
{
  (void)value;
}

void OSC_PARAM(uint16_t index, uint16_t value)
{
  r_osc_param(index, value);
}

/** @} */


/** @} */
