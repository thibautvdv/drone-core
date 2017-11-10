#![feature(decl_macro)]

extern crate drone;

use drone::reg;
use drone::reg::prelude::*;

reg!(RW_REG 0xDEAD_BEEF 0x20 0xBEEF_CACE RReg WReg);
reg!(RO_REG 0xDEAD_BEEF 0x20 0xBEEF_CACE RReg);
reg!(WO_REG 0xDEAD_BEEF 0x20 0xBEEF_CACE WReg);

fn assert_ureg_unique<'a, T: URegUnique<'a>>() {}

fn main() {
  assert_ureg_unique::<RwReg<Sr>>();
  //~^ ERROR drone::reg::WReg<'_, drone::reg::Ur>` is not satisfied
  //~| ERROR drone::reg::RReg<'_, drone::reg::Ur>` is not satisfied
  assert_ureg_unique::<RoReg<Ur>>();
  //~^ ERROR drone::reg::WReg<'_, drone::reg::Ur>` is not satisfied
  assert_ureg_unique::<WoReg<Ur>>();
  //~^ ERROR drone::reg::RReg<'_, drone::reg::Ur>` is not satisfied
}