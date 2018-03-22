/******************************************************************************
*  Filename:       rf_patch_cpe_multi_protocol.h
*  Revised:        $Date: 2017-12-07 14:15:27 +0100 (to, 07 des 2017) $
*  Revision:       $Revision: 18112 $
*
*  Description: RF core patch for multi-protocol support (all available API command sets) in CC13x2 and CC26x2
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
#ifndef _RF_PATCH_CPE_MULTI_PROTOCOL_H
#define _RF_PATCH_CPE_MULTI_PROTOCOL_H

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


CPE_PATCH_TYPE patchImageMultiProtocol[] = {
   0x21004191,
   0x210041b1,
   0x21004229,
   0x210042d1,
   0x21004379,
   0x2100452f,
   0x2100457f,
   0x210040a9,
   0x210040b5,
   0x210040cd,
   0x210040e5,
   0x210046a1,
   0x21004119,
   0x2100413d,
   0x21004141,
   0x210046b5,
   0x210046cf,
   0x21004155,
   0x21004171,
   0x21004181,
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
   0x6ac34807,
   0xf808f000,
   0x009b089b,
   0x6ac14804,
   0xd10007c9,
   0xbdf862c3,
   0xb5f84902,
   0x00004708,
   0x40045040,
   0x00029dd3,
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
   0x4604fabb,
   0x47004800,
   0x0000545b,
   0xf0002004,
   0x4605fab3,
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
   0x9000ff59,
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
   0x4855fab7,
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
   0x4a46ff28,
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
   0xfa50f000,
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
   0xfecaf7ff,
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
   0xfea6f7ff,
   0x0000bdf8,
   0x21004774,
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
   0xfd5ef7ff,
   0xda072800,
   0x1ab900c2,
   0x7e493920,
   0x42112214,
   0x2000d100,
   0x4302bdf8,
   0x46384303,
   0xfd4ef7ff,
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
   0x210044ef,
   0x210044cb,
   0x2100449f,
   0x2100444d,
   0x21004427,
   0x210043e7,
   0x21004395,
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
   0xfd32f7ff,
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
#define _NWORD_PATCHIMAGE_MULTI_PROTOCOL 470

#define _NWORD_PATCHSYS_MULTI_PROTOCOL 0

#define _IRQ_PATCH_0 0x2100465d


#ifndef _MULTI_PROTOCOL_SYSRAM_START
#define _MULTI_PROTOCOL_SYSRAM_START 0x20000000
#endif

#ifndef _MULTI_PROTOCOL_CPERAM_START
#define _MULTI_PROTOCOL_CPERAM_START 0x21000000
#endif

#define _MULTI_PROTOCOL_SYS_PATCH_FIXED_ADDR 0x20000000

#define _MULTI_PROTOCOL_PARSER_PATCH_TAB_OFFSET 0x0390
#define _MULTI_PROTOCOL_PATCH_TAB_OFFSET 0x0398
#define _MULTI_PROTOCOL_IRQPATCH_OFFSET 0x0434
#define _MULTI_PROTOCOL_PATCH_VEC_OFFSET 0x4024

#ifndef _MULTI_PROTOCOL_NO_PROG_STATE_VAR
static uint8_t bMultiProtocolPatchEntered = 0;
#endif

PATCH_FUN_SPEC void enterMultiProtocolCpePatch(void)
{
   uint32_t *pPatchVec = (uint32_t *) (_MULTI_PROTOCOL_CPERAM_START + _MULTI_PROTOCOL_PATCH_VEC_OFFSET);

#if (_NWORD_PATCHIMAGE_MULTI_PROTOCOL > 0)
   memcpy(pPatchVec, patchImageMultiProtocol, sizeof(patchImageMultiProtocol));
#endif
}

PATCH_FUN_SPEC void enterMultiProtocolSysPatch(void)
{
}

PATCH_FUN_SPEC void configureMultiProtocolPatch(void)
{
   uint8_t *pParserPatchTab = (uint8_t *) (_MULTI_PROTOCOL_CPERAM_START + _MULTI_PROTOCOL_PARSER_PATCH_TAB_OFFSET);
   uint8_t *pPatchTab = (uint8_t *) (_MULTI_PROTOCOL_CPERAM_START + _MULTI_PROTOCOL_PATCH_TAB_OFFSET);
   uint32_t *pIrqPatch = (uint32_t *) (_MULTI_PROTOCOL_CPERAM_START + _MULTI_PROTOCOL_IRQPATCH_OFFSET);


   pPatchTab[84] = 0;
   pPatchTab[142] = 1;
   pPatchTab[66] = 2;
   pPatchTab[102] = 3;
   pParserPatchTab[1] = 4;
   pPatchTab[1] = 5;
   pPatchTab[18] = 6;
   pPatchTab[112] = 7;
   pPatchTab[115] = 8;
   pPatchTab[22] = 9;
   pPatchTab[53] = 10;
   pPatchTab[28] = 11;
   pPatchTab[104] = 12;
   pPatchTab[75] = 13;
   pPatchTab[73] = 14;
   pPatchTab[105] = 15;
   pPatchTab[106] = 16;
   pPatchTab[70] = 17;
   pPatchTab[71] = 18;
   pPatchTab[69] = 19;

   pIrqPatch[21] = _IRQ_PATCH_0;
}

PATCH_FUN_SPEC void applyMultiProtocolPatch(void)
{
#ifdef _MULTI_PROTOCOL_NO_PROG_STATE_VAR
   enterMultiProtocolSysPatch();
   enterMultiProtocolCpePatch();
#else
   if (!bMultiProtocolPatchEntered)
   {
      enterMultiProtocolSysPatch();
      enterMultiProtocolCpePatch();
      bMultiProtocolPatchEntered = 1;
   }
#endif
   configureMultiProtocolPatch();
}

PATCH_FUN_SPEC void refreshMultiProtocolPatch(void)
{
   configureMultiProtocolPatch();
}

#ifndef _MULTI_PROTOCOL_NO_PROG_STATE_VAR
PATCH_FUN_SPEC void cleanMultiProtocolPatch(void)
{
   bMultiProtocolPatchEntered = 0;
}
#endif

PATCH_FUN_SPEC void rf_patch_cpe_multi_protocol(void)
{
   applyMultiProtocolPatch();
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

#endif //  _RF_PATCH_CPE_MULTI_PROTOCOL_H

