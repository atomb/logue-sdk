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

/*
 * File: waves.cpp
 *
 * Morphing wavetable oscillator
 *
 */

#include "userosc.h"
#include "waves.hpp"

static Waves s_waves;

void OSC_INIT(uint32_t platform, uint32_t api)
{
  r_osc_init(&s_waves, platform, api);
}

void OSC_CYCLE(const user_osc_param_t * const params,
               int32_t *yn,
               const uint32_t frames)
{
  
  Waves::State &s = s_waves.state;
  const Waves::Params &p = s_waves.params;

  // Handle events.
  {
    const uint32_t flags = s.flags;
    s.flags = Waves::k_flags_none;
    
    //s_waves.updatePitch(osc_w0f_for_note((params->pitch)>>8, params->pitch & 0xFF));
    //s_waves.updatePitch(r_osc_w0f_for_note((params->pitch)>>8, params->pitch & 0xFF));
    r_update_pitch(&s_waves, r_osc_w0f_for_note((params->pitch)>>8, params->pitch & 0xFF));
    
    //s_waves.updateWaves(flags);
    r_update_waves(&s_waves, flags);
    
    if (flags & Waves::k_flag_reset)
      s.reset();
    
    s.lfo = q31_to_f32(params->shape_lfo);

    if (flags & Waves::k_flag_bitcrush) {
      s.dither = p.bitcrush * 2e-008f;
      s.bitres = osc_bitresf(p.bitcrush);
      s.bitresrcp = 1.f / s.bitres;
    }
  }
  
  // Temporaries.
  float phi0 = s.phi0;
  float phi1 = s.phi1;
  float phisub = s.phisub;

  float lfoz = s.lfoz;
  const float lfo_inc = (s.lfo - lfoz) / frames;
  
  const float ditheramt = p.bitcrush * 2e-008f;
  
  const float bitres = osc_bitresf(p.bitcrush);
  const float bitres_recip = 1.f / bitres;

  const float submix = p.submix;
  const float ringmix = p.ringmix;
  
  dsp::BiQuad &prelpf = s_waves.prelpf;
  dsp::BiQuad &postlpf = s_waves.postlpf;
  
  q31_t * __restrict y = (q31_t *)yn;
  const q31_t * y_e = y + frames;
  
  for (; y != y_e; ) {

    const float wavemix = clipminmaxf(0.005f, p.shape+lfoz, 0.995f);
    
    float sig = (1.f - wavemix) * osc_wave_scanf(s.wave0, phi0);
    sig += wavemix * osc_wave_scanf(s.wave1, phi1);
    
    const float subsig = osc_wave_scanf(s.subwave, phisub);
    sig = (1.f - submix) * sig + submix * subsig;
    sig = (1.f - ringmix) * sig + ringmix * (subsig * sig);
    sig = clip1m1f(sig);
    
    sig = prelpf.process_fo(sig);
    sig += s.dither * r_osc_white();
    sig = r_mul_round(sig, s.bitres, s.bitresrcp);
    //sig = si_roundf(sig * s.bitres) * s.bitresrcp;
    sig = postlpf.process_fo(sig);
    sig = osc_softclipf(0.125f, sig);
    
    *(y++) = f32_to_q31(sig);
    
    phi0 += s.w00;
    phi0 -= (uint32_t)phi0;
    phi1 += s.w01;
    phi1 -= (uint32_t)phi1;
    phisub += s.w0sub;
    phisub -= (uint32_t)phisub;
    lfoz += lfo_inc;
  }
  
  s.phi0 = phi0;
  s.phi1 = phi1;
  s.phisub = phisub;
  s.lfoz = lfoz;
}

void OSC_NOTEON(const user_osc_param_t * const params)
{
  r_osc_noteon(&s_waves, params);
}

void OSC_NOTEOFF(const user_osc_param_t * const params)
{
  r_osc_noteoff(&s_waves, params);
}

void OSC_PARAM(uint16_t index, uint16_t value)
{
  r_osc_param(&s_waves, index, value);
}

