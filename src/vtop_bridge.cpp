#include "Vtop.h"

extern "C" {
  Vtop *vtop_new()
  {
    return new Vtop();
  }

  void vtop_delete(Vtop *vtop)
  {
    delete vtop;
    vtop = nullptr;
  }

  void vtop_eval(Vtop *vtop)
  {
    vtop->eval();
  }

  void vtop_finish(Vtop *vtop)
  {
    vtop->final();
  }

  uint8_t *vtop_port_clk(Vtop *vtop)
  {
    return &vtop->clk;
  }

  uint8_t *vtop_port_reset_l(Vtop *vtop)
  {
    return &vtop->reset_l;
  }

  uint8_t *vtop_port_out_small(Vtop *vtop)
  {
    return &vtop->out_small;
  }

  uint8_t *vtop_port_in_small(Vtop *vtop)
  {
    return &vtop->in_small;
  }

  uint64_t *vtop_port_out_quad(Vtop *vtop)
  {
    return &vtop->out_quad;
  }

  uint64_t *vtop_port_in_quad(Vtop *vtop)
  {
    return &vtop->in_quad;
  }
}
