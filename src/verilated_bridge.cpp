#include "verilated.h"

extern "C" {
  bool verilated_got_finish()
  {
    return Verilated::gotFinish();
  }

  uint64_t verilated_time()
  {
    return Verilated::time();
  }

  void verilated_time_inc(uint64_t add)
  {
    return Verilated::timeInc(add);
  }
}
