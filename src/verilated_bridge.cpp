#include "verilated.h"

extern "C" {
  void verilated_command_args(int argc, const char **argv)
  {
    Verilated::commandArgs(argc, argv);
  }

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
    Verilated::timeInc(add);
  }

  void verilated_trace_ever_on(bool flag)
  {
    Verilated::traceEverOn(flag);
  }
}
