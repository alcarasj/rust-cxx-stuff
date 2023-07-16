#include "rust-cxx-stuff/src/native.h"

rust::String say_hello_native(rust::String name) {
  return std::string("Hello ") + std::string(name) + std::string("!");
}