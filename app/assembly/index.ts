// @ts-ignore: decorator
@external("lib", "answerToLife")
declare function answerToLife(): i32;


if (answerToLife() != 42) {
  unreachable();
}

export function run(): i32 {
  return answerToLife();
}
