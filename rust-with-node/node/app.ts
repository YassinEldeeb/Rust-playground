import ffi from 'ffi-napi'

const lib = ffi.Library('../rust-addon/target/release/rust_module', {
  add_two_numbers: ['int32', ['int32', 'int32']],
})

const result = lib.add_two_numbers(10, 25)

console.log(result)
