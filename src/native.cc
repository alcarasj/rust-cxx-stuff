#include "rust-cxx-stuff/src/native.h"

rust::String say_hello_native() {
  return std::string("Hello FII!");
}