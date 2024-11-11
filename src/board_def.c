#include <stdint.h>

#include "board_def.h"

/* Each pin are defined in {GROUP, PIN} pairs. Pin numberings are logical,
 * not physical. Each collection of pins is terminated with a {0xFF, 0} pair.
 */

const uint8_t pinsGPIO_Out[][2] = {
    {GRP_PINB, PIN_LED_STATUS}, {GRP_PINA, PIN_LED_PROG},
    {GRP_PINB, PIN_LED_USER0},  {GRP_PINB, PIN_LED_USER1},
    {GRP_PINA, PIN_SPI_RFM_SS}, {0xFF, 0}};

const uint8_t pinsGPIO_In[][2] = {{GRP_PULSE, PIN_PULSE1},
                                  {GRP_PULSE, PIN_PULSE2},
                                  {GRP_ONEWIRE, PIN_ONEWIRE},
                                  {GRP_RFM_INTF, PIN_RFM_IRQ},
                                  {GRP_RFM_INTF, PIN_RFM_RST},
                                  {GRP_PINA, PIN_REV0},
                                  {GRP_PINA, PIN_REV1},
                                  {GRP_PINA, PIN_REV2},
                                  {0xFF, 0}};

const uint8_t pinsUnused[][2] = {{GRP_PINA, 0}, {GRP_PINA, 1}, {0xFF, 0}};

/* ADC input pins. These must be contiguous from V1 to the final input */
const uint8_t pinsADC[][2] = {
    {GRP_ADC_VMID, PIN_ADC_VMID},     {GRP_ADC_VREF, PIN_ADC_VREF},
    {GRP_ADC_VSENS1, PIN_ADC_VSENS1}, {GRP_ADC_VSENS2, PIN_ADC_VSENS2},
    {GRP_ADC_VSENS3, PIN_ADC_VSENS3}, {GRP_ADC_CT1, PIN_ADC_CT1},
    {GRP_ADC_CT2, PIN_ADC_CT2},       {GRP_ADC_CT3, PIN_ADC_CT3},
    {GRP_ADC_CT4, PIN_ADC_CT4},       {GRP_ADC_CT5, PIN_ADC_CT5},
    {GRP_ADC_CT6, PIN_ADC_CT6},       {GRP_ADC_CT7, PIN_ADC_CT7},
    {GRP_ADC_CT8, PIN_ADC_CT8},       {GRP_ADC_CT9, PIN_ADC_CT9},
    {GRP_ADC_CT10, PIN_ADC_CT10},     {GRP_ADC_CT11, PIN_ADC_CT11},
    {GRP_ADC_AIN, PIN_ADC_AIN},       {GRP_ADC_VCAL_H, PIN_ADC_VCAL_H},
    {GRP_ADC_VCAL_L, PIN_ADC_VCAL_L}, {0xFF, 0}};

/* Remapping for analog CT inputs. */
/* Mapping for any boards <v0.2*/
const int_fast8_t ainRemap[NUM_CT] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11};

/* Mapping for boards >= v0.2
const int_fast8_t ainRemap[NUM_CT] = {3, 4, 7, 1, 2, 11, 5, 6, 8, 9, 10, 0};
*/
