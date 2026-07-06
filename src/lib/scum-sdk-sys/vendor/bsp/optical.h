#ifndef __OPTICAL_H
#define __OPTICAL_H

#include <stdint.h>

//=========================== define ==========================================

//=========================== typedef =========================================

//=========================== variables =======================================

//=========================== prototypes ======================================

//==== admin
void optical_init(void);
void optical_enable(void);
void perform_calibration(void);
void optical_sfd_isr(void);

#endif
