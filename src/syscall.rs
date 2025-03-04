#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Syscall {
    Syscall_1(u16),
    Syscall_2(u16, u16),
    Syscall_3(u16, u16, u16),
    Syscall_4(u16, u16, u16, u16),
    Syscall_5(u16, u16, u16, u16, u16),
    Syscall_6(u16, u16, u16, u16, u16, u16),
    Syscall_7(u16, u16, u16, u16, u16, u16, u16),
    Syscall_8(u16, u16, u16, u16, u16, u16, u16, u16),
}

#[macro_export]
macro_rules! get_syscall {
    () => {};
    (SYSCALL_1 $v1:literal) => {{
        Syscall::Syscall_1($v1)
    }};
    (SYSCALL_2 $v1:literal $v2:literal) => {{
        Syscall::Syscall_2($v1, $v2)
    }};
    (SYSCALL_3 $v1:literal $v2:literal $v3:literal) => {{
        Syscall::Syscall_3($v1, $v2, $v3)
    }};
    (SYSCALL_4 $v1:literal $v2:literal $v3:literal $v4:literal) => {{
        Syscall::Syscall_4($v1, $v2, $v3, $v4)
    }};
    (SYSCALL_5 $v1:literal $v2:literal $v3:literal $v4:literal $v5:literal) => {{
        Syscall::Syscall_5($v1, $v2, $v3, $v4, $v5)
    }};
    (SYSCALL_6 $v1:literal $v2:literal $v3:literal $v4:literal $v5:literal $v6:literal) => {{
        Syscall::Syscall_6($v1, $v2, $v3, $v4, $v5, $v6)
    }};
    (SYSCALL_7 $v1:literal $v2:literal $v3:literal $v4:literal $v5:literal $v6:literal $v7:literal) => {{
        Syscall::Syscall_7($v1, $v2, $v3, $v4, $v5, $v6, $v7)
    }};
    (SYSCALL_8 $v1:literal $v2:literal $v3:literal $v4:literal $v5:literal $v6:literal $v7:literal $v8:literal) => {{
        Syscall::Syscall_8($v1, $v2, $v3, $v4, $v5, $v6, $v7, $v8)
    }};
}