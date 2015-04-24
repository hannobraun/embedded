/* ----------------------------------------------------------------------------
 *         SAM Software Package License
 * ----------------------------------------------------------------------------
 * Copyright (c) 2012, Atmel Corporation
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following condition is met:
 *
 * - Redistributions of source code must retain the above copyright notice,
 * this list of conditions and the disclaimer below.
 *
 * Atmel's name may not be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * DISCLAIMER: THIS SOFTWARE IS PROVIDED BY ATMEL "AS IS" AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT ARE
 * DISCLAIMED. IN NO EVENT SHALL ATMEL BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA,
 * OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 * EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 * ----------------------------------------------------------------------------
 */

#ifndef _SAM3X8E_
#define _SAM3X8E_

/** \addtogroup SAM3X8E_definitions SAM3X8E definitions
  This file defines all structures and symbols for SAM3X8E:
    - registers and bitfields
    - peripheral base address
    - peripheral ID
    - PIO definitions
*/
/*@{*/

#if !(defined(__ASSEMBLY__) || defined(__IAR_SYSTEMS_ASM__))
#include <stdint.h>
#endif

/* ************************************************************************** */
/*   CMSIS DEFINITIONS FOR SAM3X8E */
/* ************************************************************************** */
/** \addtogroup SAM3X8E_cmsis CMSIS Definitions */
/*@{*/

typedef struct _DeviceVectors
{
  /* Stack pointer */
  void* pvStack;

  /* Cortex-M handlers */
  void* pfnReset_Handler;
  void* pfnNMI_Handler;
  void* pfnHardFault_Handler;
  void* pfnMemManage_Handler;
  void* pfnBusFault_Handler;
  void* pfnUsageFault_Handler;
  void* pfnReserved1_Handler;
  void* pfnReserved2_Handler;
  void* pfnReserved3_Handler;
  void* pfnReserved4_Handler;
  void* pfnSVC_Handler;
  void* pfnDebugMon_Handler;
  void* pfnReserved5_Handler;
  void* pfnPendSV_Handler;
  void* pfnSysTick_Handler;

  /* Peripheral handlers */
  void* pfnSUPC_Handler;   /*  0 Supply Controller */
  void* pfnRSTC_Handler;   /*  1 Reset Controller */
  void* pfnRTC_Handler;    /*  2 Real Time Clock */
  void* pfnRTT_Handler;    /*  3 Real Time Timer */
  void* pfnWDT_Handler;    /*  4 Watchdog Timer */
  void* pfnPMC_Handler;    /*  5 Power Management Controller */
  void* pfnEFC0_Handler;   /*  6 Enhanced Flash Controller 0 */
  void* pfnEFC1_Handler;   /*  7 Enhanced Flash Controller 1 */
  void* pfnUART_Handler;   /*  8 Universal Asynchronous Receiver Transceiver */
  void* pfnSMC_Handler;    /*  9 Static Memory Controller */
  void* pvReserved10;
  void* pfnPIOA_Handler;   /* 11 Parallel I/O Controller A, */
  void* pfnPIOB_Handler;   /* 12 Parallel I/O Controller B */
  void* pfnPIOC_Handler;   /* 13 Parallel I/O Controller C */
  void* pfnPIOD_Handler;   /* 14 Parallel I/O Controller D */
  void* pvReserved15;
  void* pvReserved16;
  void* pfnUSART0_Handler; /* 17 USART 0 */
  void* pfnUSART1_Handler; /* 18 USART 1 */
  void* pfnUSART2_Handler; /* 19 USART 2 */
  void* pfnUSART3_Handler; /* 20 USART 3 */
  void* pfnHSMCI_Handler;  /* 21 Multimedia Card Interface */
  void* pfnTWI0_Handler;   /* 22 Two-Wire Interface 0 */
  void* pfnTWI1_Handler;   /* 23 Two-Wire Interface 1 */
  void* pfnSPI0_Handler;   /* 24 Serial Peripheral Interface */
  void* pvReserved25;
  void* pfnSSC_Handler;    /* 26 Synchronous Serial Controller */
  void* pfnTC0_Handler;    /* 27 Timer Counter 0 */
  void* pfnTC1_Handler;    /* 28 Timer Counter 1 */
  void* pfnTC2_Handler;    /* 29 Timer Counter 2 */
  void* pfnTC3_Handler;    /* 30 Timer Counter 3 */
  void* pfnTC4_Handler;    /* 31 Timer Counter 4 */
  void* pfnTC5_Handler;    /* 32 Timer Counter 5 */
  void* pfnTC6_Handler;    /* 33 Timer Counter 6 */
  void* pfnTC7_Handler;    /* 34 Timer Counter 7 */
  void* pfnTC8_Handler;    /* 35 Timer Counter 8 */
  void* pfnPWM_Handler;    /* 36 Pulse Width Modulation Controller */
  void* pfnADC_Handler;    /* 37 ADC Controller */
  void* pfnDACC_Handler;   /* 38 DAC Controller */
  void* pfnDMAC_Handler;   /* 39 DMA Controller */
  void* pfnUOTGHS_Handler; /* 40 USB OTG High Speed */
  void* pfnTRNG_Handler;   /* 41 True Random Number Generator */
  void* pfnEMAC_Handler;   /* 42 Ethernet MAC */
  void* pfnCAN0_Handler;   /* 43 CAN Controller 0 */
  void* pfnCAN1_Handler;   /* 44 CAN Controller 1 */
} DeviceVectors;

/* Cortex-M3 core handlers */
void Reset_Handler      ( void );
void NMI_Handler        ( void );
void HardFault_Handler  ( void );
void MemManage_Handler  ( void );
void BusFault_Handler   ( void );
void UsageFault_Handler ( void );
void SVC_Handler        ( void );
void DebugMon_Handler   ( void );
void PendSV_Handler     ( void );
void SysTick_Handler    ( void );

/* Peripherals handlers */
void ADC_Handler        ( void );
void CAN0_Handler       ( void );
void CAN1_Handler       ( void );
void DACC_Handler       ( void );
void DMAC_Handler       ( void );
void EFC0_Handler       ( void );
void EFC1_Handler       ( void );
void EMAC_Handler       ( void );
void HSMCI_Handler      ( void );
void PIOA_Handler       ( void );
void PIOB_Handler       ( void );
void PIOC_Handler       ( void );
void PIOD_Handler       ( void );
void PMC_Handler        ( void );
void PWM_Handler        ( void );
void RSTC_Handler       ( void );
void RTC_Handler        ( void );
void RTT_Handler        ( void );
void SMC_Handler        ( void );
void SPI0_Handler       ( void );
void SSC_Handler        ( void );
void SUPC_Handler       ( void );
void TC0_Handler        ( void );
void TC1_Handler        ( void );
void TC2_Handler        ( void );
void TC3_Handler        ( void );
void TC4_Handler        ( void );
void TC5_Handler        ( void );
void TC6_Handler        ( void );
void TC7_Handler        ( void );
void TC8_Handler        ( void );
void TRNG_Handler       ( void );
void TWI0_Handler       ( void );
void TWI1_Handler       ( void );
void UART_Handler       ( void );
void UOTGHS_Handler     ( void );
void USART0_Handler     ( void );
void USART1_Handler     ( void );
void USART2_Handler     ( void );
void USART3_Handler     ( void );
void WDT_Handler        ( void );

/*@}*/

#endif /* _SAM3X8E_ */
