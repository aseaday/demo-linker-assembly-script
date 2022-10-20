(module
 (type $none_=>_i32 (func (result i32)))
 (memory $0 0)
 (table $0 1 funcref)
 (export "answerToLife" (func $lib/assembly/index/answerToLife))
 (func $lib/assembly/index/answerToLife (result i32)
  i32.const 42
 )
)
