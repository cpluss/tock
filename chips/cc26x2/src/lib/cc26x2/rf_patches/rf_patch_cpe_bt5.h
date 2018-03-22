/******************************************************************************
*  Filename:       rf_patch_cpe_bt5.h
*  Revised:        $Date: 2017-12-07 14:15:27 +0100 (to, 07 des 2017) $
*  Revision:       $Revision: 18112 $
*
*  Description: RF core patch for Bluetooth 5 support ("BLE" and "BLE5" API command sets) in CC13x2 and CC26x2
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
#ifndef _RF_PATCH_CPE_BT5_H
#define _RF_PATCH_CPE_BT5_H

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


CPE_PATCH_TYPE patchImageBt5[] = {
   0x210040e1,
   0x21004159,
   0x210041d5,
   0x2100438b,
   0x210043db,
   0x2100407d,
   0x21004089,
   0x210040a1,
   0x210044fd,
   0x21004511,
   0x2100452b,
   0x210040c5,
   0xb081b5ff,
   0x9d0a4803,
   0xb5f84700,
   0x48024684,
   0x47004613,
   0x00007f43,
   0x00005145,
   0x461db570,
   0x47204c00,
   0x00022281,
   0x0a804670,
   0x288d4a05,
   0x4710d004,
   0x32e44a03,
   0x90042001,
   0x20004902,
   0x47107008,
   0x00006f69,
   0x210001f8,
   0x88084903,
   0x46714a03,
   0xd0004290,
   0x47081c89,
   0x210001a6,
   0x00001404,
   0x4801b510,
   0x00004700,
   0x000009df,
   0x49044803,
   0x05c068c0,
   0x47880fc0,
   0x47084902,
   0x21000340,
   0x000087d1,
   0x000053cd,
   0x4614b5f8,
   0x9b06461a,
   0x46139300,
   0xf7ff4622,
   0x9000ffb1,
   0xd12d2800,
   0x6848492e,
   0xd0292800,
   0x00498809,
   0x43411b09,
   0x68c0482b,
   0x00640844,
   0x20187922,
   0xb2c61a80,
   0x04802001,
   0x0cc71808,
   0x46317965,
   0xf0004825,
   0x4824fa0f,
   0x19832201,
   0x408a1e69,
   0x21182000,
   0xe0071b8e,
   0x19090041,
   0x437988c9,
   0x40e91889,
   0x1c405419,
   0xdcf54286,
   0x4780481b,
   0xbdf89800,
   0xf7ffb510,
   0x4604ff80,
   0x20004914,
   0x60484a17,
   0x88006810,
   0x28030b00,
   0x0760d01d,
   0x6810d41b,
   0x28007bc0,
   0x2802d001,
   0x2013d115,
   0x4a0c01c0,
   0x68d08008,
   0xd00428c0,
   0xd00428d8,
   0x383938ff,
   0x480bd109,
   0x480be000,
   0x480b6048,
   0x211860d0,
   0xf0004804,
   0x4620f9cd,
   0x0000bd10,
   0x21004554,
   0x21000028,
   0x21000000,
   0x0000764d,
   0x21000108,
   0x00063b91,
   0x0003fd29,
   0x000090fd,
   0x4aa28801,
   0x29031889,
   0x48a1d806,
   0x8501217c,
   0x624149a0,
   0x47702001,
   0x4708499f,
   0xb430489f,
   0x31404601,
   0x2a027c0a,
   0x6802d116,
   0x754a79d2,
   0x68936802,
   0x32804602,
   0x7d486093,
   0xd00b2802,
   0xd0092800,
   0x2c061ec4,
   0x4991d809,
   0x18400080,
   0x6b803840,
   0x60901818,
   0xbc304891,
   0x20034700,
   0x80c802c0,
   0x72082002,
   0x2003bc30,
   0xb5704770,
   0x4604488a,
   0x7da53440,
   0xd0122d01,
   0x06497d01,
   0x21800fca,
   0x7c21540a,
   0xd10a2900,
   0x78403060,
   0x07c00880,
   0x7ce0d002,
   0xd5020700,
   0x75a02001,
   0x2000e000,
   0x487f73a0,
   0x75a54780,
   0xb570bd70,
   0x487d4c7a,
   0x35504625,
   0x28024780,
   0x3440d109,
   0x4a7a88e1,
   0xd1044291,
   0x06897ce9,
   0x1d91d401,
   0xbd7080e1,
   0x4971b570,
   0x4608890a,
   0xb2d23050,
   0xd1072a28,
   0x68d2680a,
   0x7a936142,
   0x02127ad2,
   0x8302189a,
   0x4a6e7803,
   0xd0102b00,
   0x2b017983,
   0x7c03d10d,
   0x07db095b,
   0x7d09d009,
   0x74c14d69,
   0x20207f6c,
   0x77684320,
   0x776c4790,
   0x21ffbd70,
   0x479074c1,
   0xb510bd70,
   0x4c5c4863,
   0x28024780,
   0x4621d10e,
   0x88ca3140,
   0x429a4b5c,
   0x7ccad108,
   0x07d20952,
   0x7d22d004,
   0xd4010692,
   0x80ca1d9a,
   0xb570bd10,
   0x49594851,
   0x7cc03040,
   0x07c00940,
   0x4d54d007,
   0x8b2c4856,
   0x83284320,
   0x832c4788,
   0x4788bd70,
   0xb570bd70,
   0x49524c48,
   0x36404626,
   0x00a87935,
   0x6b401840,
   0x2d0a4780,
   0xd0104621,
   0x780a3154,
   0x07db0993,
   0x73b2d004,
   0x2303780a,
   0x700a431a,
   0xb2ca8921,
   0xd3012a2b,
   0x81213928,
   0x3153bd70,
   0x4939e7ed,
   0x71083140,
   0xd01a2825,
   0x280adc08,
   0x280bd011,
   0x2818d011,
   0x281ed011,
   0xe014d106,
   0xd010282a,
   0xd006283c,
   0xd010283d,
   0x00804938,
   0x6b401840,
   0x48374770,
   0x48374770,
   0x48374770,
   0x48374770,
   0x48374770,
   0x48374770,
   0x48374770,
   0xb5f84770,
   0x4c244607,
   0x5d00204e,
   0x07ee0985,
   0x2e0025fb,
   0x7d26d017,
   0x0f240734,
   0xd0032c05,
   0x42202401,
   0xe012d002,
   0xe7fa2402,
   0xd00509c0,
   0xd5030670,
   0x0f806848,
   0xd0082801,
   0x005b085b,
   0x00520852,
   0x2800e003,
   0x402bd001,
   0x2b06402a,
   0x2010d003,
   0xd0102b02,
   0x2302e010,
   0x4638402a,
   0xfe1af7ff,
   0xda072800,
   0x1ab900c2,
   0x7e493920,
   0x42112214,
   0x2000d100,
   0x4302bdf8,
   0x46384303,
   0xfe0af7ff,
   0x0000bdf8,
   0xffffe7d5,
   0x21000108,
   0x00020619,
   0x00020625,
   0x21000160,
   0x000245a5,
   0x00022cfd,
   0x00023b49,
   0x00001404,
   0x000238fd,
   0x210000a8,
   0x000224fb,
   0x00021d4f,
   0x00002020,
   0x00024fc0,
   0x2100434b,
   0x21004327,
   0x210042fb,
   0x210042a9,
   0x21004283,
   0x21004243,
   0x210041f1,
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
   0xfddaf7ff,
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
   0x4801b403,
   0xbd019001,
   0x000089dd,
   0x00000000,
   0x00000000,
};
#define _NWORD_PATCHIMAGE_BT5 334

#define _NWORD_PATCHSYS_BT5 0

#define _IRQ_PATCH_0 0x210044b9


#ifndef _BT5_SYSRAM_START
#define _BT5_SYSRAM_START 0x20000000
#endif

#ifndef _BT5_CPERAM_START
#define _BT5_CPERAM_START 0x21000000
#endif

#define _BT5_SYS_PATCH_FIXED_ADDR 0x20000000

#define _BT5_PARSER_PATCH_TAB_OFFSET 0x0390
#define _BT5_PATCH_TAB_OFFSET 0x0398
#define _BT5_IRQPATCH_OFFSET 0x0434
#define _BT5_PATCH_VEC_OFFSET 0x4024

#ifndef _BT5_NO_PROG_STATE_VAR
static uint8_t bBt5PatchEntered = 0;
#endif

PATCH_FUN_SPEC void enterBt5CpePatch(void)
{
   uint32_t *pPatchVec = (uint32_t *) (_BT5_CPERAM_START + _BT5_PATCH_VEC_OFFSET);

#if (_NWORD_PATCHIMAGE_BT5 > 0)
   memcpy(pPatchVec, patchImageBt5, sizeof(patchImageBt5));
#endif
}

PATCH_FUN_SPEC void enterBt5SysPatch(void)
{
}

PATCH_FUN_SPEC void configureBt5Patch(void)
{
   uint8_t *pParserPatchTab = (uint8_t *) (_BT5_CPERAM_START + _BT5_PARSER_PATCH_TAB_OFFSET);
   uint8_t *pPatchTab = (uint8_t *) (_BT5_CPERAM_START + _BT5_PATCH_TAB_OFFSET);
   uint32_t *pIrqPatch = (uint32_t *) (_BT5_CPERAM_START + _BT5_IRQPATCH_OFFSET);


   pPatchTab[142] = 0;
   pPatchTab[66] = 1;
   pParserPatchTab[1] = 2;
   pPatchTab[1] = 3;
   pPatchTab[18] = 4;
   pPatchTab[112] = 5;
   pPatchTab[115] = 6;
   pPatchTab[22] = 7;
   pPatchTab[28] = 8;
   pPatchTab[105] = 9;
   pPatchTab[106] = 10;
   pPatchTab[70] = 11;

   pIrqPatch[21] = _IRQ_PATCH_0;
}

PATCH_FUN_SPEC void applyBt5Patch(void)
{
#ifdef _BT5_NO_PROG_STATE_VAR
   enterBt5SysPatch();
   enterBt5CpePatch();
#else
   if (!bBt5PatchEntered)
   {
      enterBt5SysPatch();
      enterBt5CpePatch();
      bBt5PatchEntered = 1;
   }
#endif
   configureBt5Patch();
}

PATCH_FUN_SPEC void refreshBt5Patch(void)
{
   configureBt5Patch();
}

#ifndef _BT5_NO_PROG_STATE_VAR
PATCH_FUN_SPEC void cleanBt5Patch(void)
{
   bBt5PatchEntered = 0;
}
#endif

PATCH_FUN_SPEC void rf_patch_cpe_bt5(void)
{
   applyBt5Patch();
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

#endif //  _RF_PATCH_CPE_BT5_H

