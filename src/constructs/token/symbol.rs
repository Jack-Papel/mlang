use lazy_static::lazy_static;

type SymbolIndex = u8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Symbol(SymbolIndex);

struct SymbolPool([*const str; 256]);

static mut SYMBOL_POOL: SymbolPool = SymbolPool([std::ptr::slice_from_raw_parts::<u8>(std::ptr::null(), 0) as *const str; 256]);
static mut POOL_SIZE: SymbolIndex = 0;

impl Drop for SymbolPool {
    fn drop(&mut self) {
        for ptr in self.0 {
            unsafe {
                let layout = std::alloc::Layout::for_value(ptr.as_ref().unwrap());
                std::alloc::dealloc(ptr.cast::<u8>() as *mut u8, layout);
            }
        }
    }
}

unsafe impl Sync for SymbolPool {}

impl Symbol {
    pub fn from(str: &str) -> Self {
        if unsafe { INITIALIZED_BUILTINS } == false {
            let _ = BUILTIN_SYMBOLS; // touch, to initialize builtins.
            unsafe { INITIALIZED_BUILTINS = true };
        }

        match (0..unsafe { POOL_SIZE }).find(|&idx| {str == Symbol(idx).get_str()}) {
            Some(idx) => Self(idx as u8),
            None => Self::intern(str)
        }
    }

    fn intern(str: &str) -> Self {
        unsafe { 
            // Copy str into a new memory address
            let ptr = {
                let layout = std::alloc::Layout::for_value(str);
                let ptr = std::alloc::alloc(layout);

                ptr.copy_from_nonoverlapping((str as *const str).cast::<u8>(), str.len());
                std::ptr::slice_from_raw_parts(ptr, str.len()) as *const str
            };
            
            let index = POOL_SIZE;
            SYMBOL_POOL.0[POOL_SIZE as usize] = ptr;
            POOL_SIZE += 1;

            Self(index)
        }
    }

    pub fn get_str(&self) -> &str {
        // Guaranteed to be safe because self.0 is guaranteed to be less than POOL_SIZE
        unsafe { &*(SYMBOL_POOL.0[self.0 as usize]) }
    }
}

macro_rules! pre_intern {
    (
        $($name:ident: $expr:expr),*
    ) => {
        #[allow(non_snake_case)]
        struct Builtins {
            $(pub $name: Symbol),*
        }

        static mut INITIALIZED_BUILTINS: bool = false;

        lazy_static! {
            static ref BUILTIN_SYMBOLS: Builtins = Builtins {
                $($name: Symbol::intern($expr)),*
            };
        }

        pub mod builtin_symbols {
            use lazy_static::lazy_static;
            
            lazy_static! {
                $(pub static ref $name: super::Symbol = super::BUILTIN_SYMBOLS.$name;)*
            }
        }
    }
}

pre_intern! {
    // Keywords
    LET: "let",
    STRUCT: "struct",
    IMPL: "impl",
    RETURN: "return",
    YIELD: "yield",
    // Boolean
    TRUE: "true",
    FALSE: "false",
    // Intrinsics
    PRINT: "print",
    ASSERT: "assert"
}