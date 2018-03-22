/******************************************************************************
*  Filename:       rf_patch_cpe_prop.h
*  Revised:        $Date: 2017-12-01 13:47:22 +0100 (fr, 01 des 2017) $
*  Revision:       $Revision: 18103 $
*
*  Description: RF core patch for proprietary radio support ("PROP" API command set) in CC13x2 and CC26x2
*
*  Copyright (c) 2015-2017, Texas Instruments Incorporated
*  All rights reserved.
*
*  Redistribution and use in source and binary forms, with or without
*  modification, are permitted provided that the following conditions are met:
*
*  1) Redistributions of source code must retain the above copyright notice,
*     this list of conditions and the following disclaimer.
*
*  2) Redistributions in binary form must reproduce the above copyright notice,
*     this list of conditions and the following disclaimer in the documentation
*     and/or other materials provided with the distribution.
*
*  3) Neither the name of the ORGANIZATION nor the names of its contributors may
*     be used to endorse or promote products derived from this software without
*     specific prior written permission.
*
*  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
*  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
*  IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
*  ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
*  LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
*  CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
*  SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
*  INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
*  CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
*  ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
*  POSSIBILITY OF SUCH DAMAGE.
*
******************************************************************************/
#ifndef _RF_PATCH_CPE_PROP_H
#define _RF_PATCH_CPE_PROP_H

//*****************************************************************************
//
// If building with a C++ compiler, make all of the definitions in this header
// have a C binding.
//
//*****************************************************************************
#ifdef __cplusplus
extern "C"
{
#endif

#include <stdint.h>
#include <string.h>

#ifndef CPE_PATCH_TYPE
#define CPE_PATCH_TYPE static const uint32_t
#endif

#ifndef SYS_PATCH_TYPE
#define SYS_PATCH_TYPE static const uint32_t
#endif

#ifndef PATCH_FUN_SPEC
#define PATCH_FUN_SPEC static inline
#endif

#ifndef _APPLY_PATCH_TAB
#define _APPLY_PATCH_TAB
#endif


CPE_PATCH_TYPE patchImageProp[] = {
   0x21004105,
   0x21004125,
   0x2100419d,
   0x21004245,
   0x21004331,
   0x2100408d,
   0x210040b1,
   0x210040b5,
   0x21004345,
   0x2100435f,
   0x210040c9,
   0x210040e5,
   0x210040f5,
   0xb081b5ff,
   0x9d0a4803,
   0xb5f84700,
   0x48024684,
   0x47004613,
   0x00007f43,
   0x00005145,
   0x460cb5f7,
   0x47084900,
   0x0000681d,
   0x4801b510,
   0x00004700,
   0x000009df,
   0x460db5f8,
   0x4b0b4616,
   0x290c6d59,
   0x4b03d104,
   0x78192408,
   0x70194321,
   0x47084901,
   0x21000340,
   0x0000699d,
   0xe0014a04,
   0x3ac84a03,
   0x21804801,
   0x47106041,
   0x40045000,
   0x000056cb,
   0x49044803,
   0x05c068c0,
   0x47880fc0,
   0x47084902,
   0x21000340,
   0x000087d1,
   0x000053cd,
   0xf0002000,
   0x4604f949,
   0x47004800,
   0x0000545b,
   0xf0002004,
   0x4605f941,
   0x47004800,
   0x0000533b,
   0x4905b672,
   0x22206808,
   0x600a4302,
   0x6ad24a03,
   0xb6626008,
   0x4770b250,
   0x40040000,
   0x40046040,
   0x4614b5f8,
   0x9b06461a,
   0x46139300,
   0xf7ff4622,
   0x9000ff91,
   0xd12d2800,
   0x6848495f,
   0xd0292800,
   0x00498809,
   0x43411b09,
   0x68c0485c,
   0x00640844,
   0x20187922,
   0xb2c61a80,
   0x04802001,
   0x0cc71808,
   0x46317965,
   0xf0004856,
   0x4855f945,
   0x19832201,
   0x408a1e69,
   0x21182000,
   0xe0071b8e,
   0x19090041,
   0x437988c9,
   0x40e91889,
   0x1c405419,
   0xdcf54286,
   0x4780484c,
   0xbdf89800,
   0xf7ffb570,
   0x4a46ff60,
   0x49492300,
   0x60534604,
   0x25136808,
   0x01ed8800,
   0x2e030b06,
   0x0760d00d,
   0x6808d43f,
   0x290c7bc1,
   0xdc0fd028,
   0xd0132904,
   0xd0142905,
   0xd10d290a,
   0xb2c0e01d,
   0xd0032806,
   0x8c006808,
   0xe02d8010,
   0xe02b8015,
   0xd018290f,
   0xd019291e,
   0xe0048015,
   0x01802013,
   0x4835e000,
   0x48308010,
   0x29c068c1,
   0x29d8d010,
   0x39ffd010,
   0xd1173939,
   0x20ffe00a,
   0xe7f130e7,
   0x309620ff,
   0x20ffe7ee,
   0xe7eb3045,
   0xe7e920a2,
   0xe000492a,
   0x6051492a,
   0x60c1492a,
   0x48232118,
   0xf8def000,
   0x8013e000,
   0xbd704620,
   0x4604b5f8,
   0x4e1c481d,
   0x88003040,
   0x0a80460d,
   0xd00407c0,
   0x08644821,
   0x43447900,
   0x8830e025,
   0xd0222800,
   0x19000960,
   0xff02f7ff,
   0x20014607,
   0x0240491b,
   0x46024788,
   0x1bc02005,
   0x40848831,
   0x18230848,
   0x62034817,
   0x21016241,
   0x430d61c1,
   0x60cd490a,
   0x07c969c1,
   0x6a80d1fc,
   0x60704910,
   0x39124610,
   0x46384788,
   0x2000bdf8,
   0x46206070,
   0xfedef7ff,
   0x0000bdf8,
   0x21004404,
   0x21000028,
   0x21000000,
   0x0000764d,
   0x21000108,
   0x000003cd,
   0x00063b91,
   0x0003fd29,
   0x000090fd,
   0x21000340,
   0x000040e5,
   0x40044100,
   0x480cb570,
   0x4c0c6ac0,
   0x0f800700,
   0x28012502,
   0x2803d008,
   0x6960d105,
   0x616043a8,
   0x15204907,
   0xbd706008,
   0x20014906,
   0x69604788,
   0x61604328,
   0x0000bd70,
   0x40046000,
   0x40041100,
   0xe000e180,
   0x00007d05,
   0x4c03b510,
   0xfea4f7ff,
   0x28006820,
   0xbd10d1fa,
   0x40041100,
   0x780a490b,
   0xd1042aff,
   0x7d924a0a,
   0x0f120712,
   0x4908700a,
   0x75883140,
   0x49054770,
   0x29ff7809,
   0x0900d005,
   0x43080100,
   0x31404902,
   0x47707588,
   0x210002a5,
   0x40086200,
   0x4c19b570,
   0x7ba14606,
   0xf820f000,
   0x7be14605,
   0xf0004630,
   0x4915f81b,
   0x78094604,
   0x070a2028,
   0x2d01d401,
   0x2038d100,
   0xd40106c9,
   0xd1012c01,
   0x43082140,
   0x4788490e,
   0xd0012dff,
   0x6145480d,
   0xd0012cff,
   0x61c4480c,
   0xbd704808,
   0xd0082900,
   0xd00629ff,
   0x070840c1,
   0x281c0ec0,
   0x2001d100,
   0x20ff4770,
   0x00004770,
   0x210000a8,
   0x21000340,
   0x000040e5,
   0x40045040,
   0x40046000,
   0x4801b403,
   0xbd019001,
   0x000089dd,
   0x00000000,
   0x00000000,
};
#define _NWORD_PATCHIMAGE_PROP 250

#define _NWORD_PATCHSYS_PROP 0

#define _IRQ_PATCH_0 0x210042ed


#ifndef _PROP_SYSRAM_START
#define _PROP_SYSRAM_START 0x20000000
#endif

#ifndef _PROP_CPERAM_START
#define _PROP_CPERAM_START 0x21000000
#endif

#define _PROP_SYS_PATCH_FIXED_ADDR 0x20000000

#define _PROP_PARSER_PATCH_TAB_OFFSET 0x0390
#define _PROP_PATCH_TAB_OFFSET 0x0398
#define _PROP_IRQPATCH_OFFSET 0x0434
#define _PROP_PATCH_VEC_OFFSET 0x4024

#ifndef _PROP_NO_PROG_STATE_VAR
static uint8_t bPropPatchEntered = 0;
#endif

PATCH_FUN_SPEC void enterPropCpePatch(void)
{
   uint32_t *pPatchVec = (uint32_t *) (_PROP_CPERAM_START + _PROP_PATCH_VEC_OFFSET);

#if (_NWORD_PATCHIMAGE_PROP > 0)
   memcpy(pPatchVec, patchImageProp, sizeof(patchImageProp));
#endif
}

PATCH_FUN_SPEC void enterPropSysPatch(void)
{
}

PATCH_FUN_SPEC void configurePropPatch(void)
{
   uint8_t *pPatchTab = (uint8_t *) (_PROP_CPERAM_START + _PROP_PATCH_TAB_OFFSET);
   uint32_t *pIrqPatch = (uint32_t *) (_PROP_CPERAM_START + _PROP_IRQPATCH_OFFSET);


   pPatchTab[84] = 0;
   pPatchTab[142] = 1;
   pPatchTab[66] = 2;
   pPatchTab[102] = 3;
   pPatchTab[28] = 4;
   pPatchTab[104] = 5;
   pPatchTab[75] = 6;
   pPatchTab[73] = 7;
   pPatchTab[105] = 8;
   pPatchTab[106] = 9;
   pPatchTab[70] = 10;
   pPatchTab[71] = 11;
   pPatchTab[69] = 12;

   pIrqPatch[21] = _IRQ_PATCH_0;
}

PATCH_FUN_SPEC void applyPropPatch(void)
{
#ifdef _PROP_NO_PROG_STATE_VAR
   enterPropSysPatch();
   enterPropCpePatch();
#else
   if (!bPropPatchEntered)
   {
      enterPropSysPatch();
      enterPropCpePatch();
      bPropPatchEntered = 1;
   }
#endif
   configurePropPatch();
}

PATCH_FUN_SPEC void refreshPropPatch(void)
{
   configurePropPatch();
}

#ifndef _PROP_NO_PROG_STATE_VAR
PATCH_FUN_SPEC void cleanPropPatch(void)
{
   bPropPatchEntered = 0;
}
#endif

PATCH_FUN_SPEC void rf_patch_cpe_prop(void)
{
   applyPropPatch();
}

#undef _IRQ_PATCH_0

//*****************************************************************************
//
// Mark the end of the C bindings section for C++ compilers.
//
//*****************************************************************************
#ifdef __cplusplus
}
#endif

#endif //  _RF_PATCH_CPE_PROP_H

