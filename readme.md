## state: you can print text + implementing linked list malloc (not done yet)

### notes for readers
0. **do not set optimzation level to more than 0. my guess is that optimization are meant for bins**
1. **compability with other libc is not a goal**
2. when using build.sh/run.sh (i fucking suck at bash) make sure u r on the project root
3. sometime the type may be different (returning bool as i32) and treating it as such (the difference is usually inconseqeuntial) **(correct me if im wrong)** 
4. if you are interested in reading the source code. check out `src/ctypes.rs:toascii`
5. never use llvm assembly. trust me
8. [rust_eh_personality fix](https://stackoverflow.com/questions/75310867/rust-no-std-static-lib-panic-handler) in top answer's replies from Miaao
7. if you have any idea how to switch to a linux target without a libc. tell me
9. [miaao](https://stackoverflow.com/users/20028181/miiao) i fucking thank you 

### list of problems:
0. stuff are not working (atoi) (and it is likely arcane magic causing it)
2. rust_eh_personality (causes the fucking C test part to not compile)
3. exported lib symbols are null (fuck if i know)
4. rust_eh_personality (yeah i know your personality sucks stfu)
5. dogshit llvm assembly syntax
7. rust fucking hates `.btw`.
8. any optimization turns the whole thing into shit
9. had no fucking idea alignment matters because cpus can only read their size

#### fix for `.btw` problem
```
[profile.dev]
codegen-units = 1  
```

#### miaao's workaround
```
// handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // fuck you
    loop {}
}

// dummy symbol
#[lang = "eh_personality"]
fn wawa(){
    loop {}
}
```
### balls
1. minimal dependency on C libraries


### todos:
1. impl more

### self notes:
1. wawa
2. av
3. try not to use mod.rs
4. [sysv abi](https://wiki.osdev.org/System_V_ABI)
5. [registers](https://math.hws.edu/eck/cs220/f22/registers.html)
6. [one byte mov](https://stackoverflow.com/questions/65527348/assembly-writing-a-single-byte-from-register-to-memory-overwrites-other-bytes)
7. [reg == 0](https://stackoverflow.com/questions/33721204/test-whether-a-register-is-zero-with-cmp-reg-0-vs-or-reg-reg/33724806#33724806)
9. how many times can you xor?
10. `test` is preferable over `cmp`
11.    ffff
12. what the fuck is a vdso
13. default int size is u32
