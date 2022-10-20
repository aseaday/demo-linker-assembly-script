aslink demo
===

### 1. 编译你的wast

```
npm install
```
安装你的依赖, app是应用目录，lib是库，用户不需要lib目录也能编译app

```typescript
// @ts-ignore: decorator
@external("lib", "answerToLife")
declare function answerToLife(): i32;


if (answerToLife() != 42) {
  unreachable();
}

export function run(): i32 {
  return answerToLife();
}

```
只需要在exteral声明函数的签名, 注意到lib是外部库的名字，可以叫lib也可以叫bls250

使用
```
asb --wat
或者
npm run build
```
即可编译，在build下面有里的编译文件

### 2. 构建我们的linker + runtime

```
cargo build
```

### 3. 进行链接和编译

```
./target/debug/as-link build/release/app.wat run
```

将会打印返回结果

```
The run function result is 42
```

你可以尝试修改lib.wat的42为88，进行测试