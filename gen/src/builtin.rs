use crate::gen::ifndef;
use crate::gen::out::{Content, OutFile};

#[derive(Default, PartialEq)]
pub struct Builtins {
    pub panic: bool,
    pub rust_string: bool,
    pub rust_str: bool,
    pub rust_slice: bool,
    pub rust_box: bool,
    pub rust_vec: bool,
    pub rust_fn: bool,
    pub rust_isize: bool,
    pub unsafe_bitcopy: bool,
    pub rust_error: bool,
    pub manually_drop: bool,
    pub maybe_uninit: bool,
    pub trycatch: bool,
    pub ptr_len: bool,
    pub rust_str_new_unchecked: bool,
    pub rust_str_repr: bool,
    pub rust_slice_new: bool,
    pub rust_slice_repr: bool,
    pub content: Content,
}

impl Builtins {
    pub fn new() -> Self {
        Builtins::default()
    }
}

pub(super) fn write(out: &mut OutFile) {
    if out.builtin == Default::default() {
        return;
    }

    let include = &mut out.include;
    let builtin = &mut out.builtin;
    let out = &mut builtin.content;

    out.begin_block("namespace rust");
    out.begin_block("inline namespace cxxbridge05");
    writeln!(out, "// #include \"rust/cxx.h\"");

    ifndef::write(out, builtin.panic, "CXXBRIDGE05_PANIC");

    if builtin.rust_string {
        out.next_section();
        writeln!(out, "struct unsafe_bitcopy_t;");
    }

    if builtin.rust_error {
        out.begin_block("namespace");
        writeln!(out, "template <typename T>");
        writeln!(out, "class impl;");
        out.end_block("namespace");
    }

    ifndef::write(out, builtin.rust_string, "CXXBRIDGE05_RUST_STRING");
    ifndef::write(out, builtin.rust_str, "CXXBRIDGE05_RUST_STR");
    ifndef::write(out, builtin.rust_slice, "CXXBRIDGE05_RUST_SLICE");
    ifndef::write(out, builtin.rust_box, "CXXBRIDGE05_RUST_BOX");
    ifndef::write(out, builtin.unsafe_bitcopy, "CXXBRIDGE05_RUST_BITCOPY");
    ifndef::write(out, builtin.rust_vec, "CXXBRIDGE05_RUST_VEC");
    ifndef::write(out, builtin.rust_fn, "CXXBRIDGE05_RUST_FN");
    ifndef::write(out, builtin.rust_error, "CXXBRIDGE05_RUST_ERROR");
    ifndef::write(out, builtin.rust_isize, "CXXBRIDGE05_RUST_ISIZE");

    if builtin.manually_drop {
        out.next_section();
        include.utility = true;
        writeln!(out, "template <typename T>");
        writeln!(out, "union ManuallyDrop {{");
        writeln!(out, "  T value;");
        writeln!(
            out,
            "  ManuallyDrop(T &&value) : value(::std::move(value)) {{}}",
        );
        writeln!(out, "  ~ManuallyDrop() {{}}");
        writeln!(out, "}};");
    }

    if builtin.maybe_uninit {
        out.next_section();
        writeln!(out, "template <typename T>");
        writeln!(out, "union MaybeUninit {{");
        writeln!(out, "  T value;");
        writeln!(out, "  MaybeUninit() {{}}");
        writeln!(out, "  ~MaybeUninit() {{}}");
        writeln!(out, "}};");
    }

    out.begin_block("namespace");

    if builtin.ptr_len {
        out.begin_block("namespace repr");
        writeln!(out, "struct PtrLen final {{");
        writeln!(out, "  const void *ptr;");
        writeln!(out, "  size_t len;");
        writeln!(out, "}};");
        out.end_block("namespace repr");
    }

    if builtin.rust_str_new_unchecked || builtin.rust_str_repr {
        out.next_section();
        writeln!(out, "template <>");
        writeln!(out, "class impl<Str> final {{");
        writeln!(out, "public:");
        if builtin.rust_str_new_unchecked {
            writeln!(
                out,
                "  static Str new_unchecked(repr::PtrLen repr) noexcept {{",
            );
            writeln!(out, "    Str str;");
            writeln!(out, "    str.ptr = static_cast<const char *>(repr.ptr);");
            writeln!(out, "    str.len = repr.len;");
            writeln!(out, "    return str;");
            writeln!(out, "  }}");
        }
        if builtin.rust_str_repr {
            writeln!(out, "  static repr::PtrLen repr(Str str) noexcept {{");
            writeln!(out, "    return repr::PtrLen{{str.ptr, str.len}};");
            writeln!(out, "  }}");
        }
        writeln!(out, "}};");
    }

    if builtin.rust_slice_new || builtin.rust_slice_repr {
        out.next_section();
        writeln!(out, "template <typename T>");
        writeln!(out, "class impl<Slice<T>> final {{");
        writeln!(out, "public:");
        if builtin.rust_slice_new {
            writeln!(
                out,
                "  static Slice<T> slice(repr::PtrLen repr) noexcept {{",
            );
            writeln!(
                out,
                "    return {{static_cast<const T *>(repr.ptr), repr.len}};",
            );
            writeln!(out, "  }}");
        }
        if builtin.rust_slice_repr {
            writeln!(
                out,
                "  static repr::PtrLen repr(Slice<T> slice) noexcept {{",
            );
            writeln!(out, "    return repr::PtrLen{{slice.ptr, slice.len}};");
            writeln!(out, "  }}");
        }
        writeln!(out, "}};");
    }

    if builtin.rust_error {
        out.next_section();
        writeln!(out, "template <>");
        writeln!(out, "class impl<Error> final {{");
        writeln!(out, "public:");
        writeln!(out, "  static Error error(repr::PtrLen repr) noexcept {{");
        writeln!(out, "    Error error;");
        writeln!(out, "    error.msg = static_cast<const char *>(repr.ptr);");
        writeln!(out, "    error.len = repr.len;");
        writeln!(out, "    return error;");
        writeln!(out, "  }}");
        writeln!(out, "}};");
    }

    out.end_block("namespace");
    out.end_block("namespace cxxbridge05");

    if builtin.trycatch {
        out.begin_block("namespace behavior");
        include.exception = true;
        include.type_traits = true;
        include.utility = true;
        writeln!(out, "class missing {{}};");
        writeln!(out, "missing trycatch(...);");
        writeln!(out);
        writeln!(out, "template <typename Try, typename Fail>");
        writeln!(out, "static typename ::std::enable_if<");
        writeln!(
            out,
            "    ::std::is_same<decltype(trycatch(::std::declval<Try>(), ::std::declval<Fail>())),",
        );
        writeln!(out, "                 missing>::value>::type");
        writeln!(out, "trycatch(Try &&func, Fail &&fail) noexcept try {{");
        writeln!(out, "  func();");
        writeln!(out, "}} catch (const ::std::exception &e) {{");
        writeln!(out, "  fail(e.what());");
        writeln!(out, "}}");
        out.end_block("namespace behavior");
    }

    out.end_block("namespace rust");
}
