use std::path::Path;
use std::sync::Arc;
use tree_sitter::Language;

use crate::macros::{
    get_language, mk_action, mk_code, mk_emacs_mode, mk_extensions, mk_lang, mk_langs,
};
use crate::preproc::PreprocResults;
use crate::*;

mk_langs!(
    // 1) Name for enum
    // 2) Language description
    // 3) Display name
    // 4) Empty struct name to implement
    // 5) Parser name
    // 6) tree-sitter function to call to get a Language
    // 7) file extensions
    // 8) emacs modes
    (
        Mozjs,
        "The `Mozjs` language is variant of the `JavaScript` language",
        "javascript",
        MozjsCode,
        MozjsParser,
        tree_sitter_mozjs,
        [js, jsm, mjs, jsx],
        ["js", "js2"]
    ),
    (
        Javascript,
        "The `JavaScript` language",
        "javascript",
        JavascriptCode,
        JavascriptParser,
        tree_sitter_javascript,
        [],
        []
    ),
    (
        Java,
        "The `Java` language",
        "java",
        JavaCode,
        JavaParser,
        tree_sitter_java,
        [java],
        ["java"]
    ),
    (
        Kotlin,
        "The `Kotlin` language",
        "kotlin",
        KotlinCode,
        KotlinParser,
        tree_sitter_kotlin_ng,
        [kt, kts],
        ["kotlin"]
    ),
    (
        Rust,
        "The `Rust` language",
        "rust",
        RustCode,
        RustParser,
        tree_sitter_rust,
        [rs],
        ["rust"]
    ),
    (
        Cpp,
        "The `C/C++` language",
        "c/c++",
        CppCode,
        CppParser,
        tree_sitter_cpp,
        [cpp, cxx, cc, hxx, hpp, c, h, hh, inc, mm, m],
        ["c++", "c", "objc", "objc++", "objective-c++", "objective-c"]
    ),
    (
        Python,
        "The `Python` language",
        "python",
        PythonCode,
        PythonParser,
        tree_sitter_python,
        [py],
        ["python"]
    ),
    (
        Tsx,
        "The `Tsx` language incorporates the `JSX` syntax inside `TypeScript`",
        "typescript",
        TsxCode,
        TsxParser,
        tree_sitter_tsx,
        [tsx],
        []
    ),
    (
        Typescript,
        "The `TypeScript` language",
        "typescript",
        TypescriptCode,
        TypescriptParser,
        tree_sitter_typescript,
        [ts, jsw, jsmw],
        ["typescript"]
    ),
    (
        Ccomment,
        "The `Ccomment` language is a variant of the `C` language focused on comments",
        "ccomment",
        CcommentCode,
        CcommentParser,
        tree_sitter_ccomment,
        [],
        []
    ),
    (
        Preproc,
        "The `PreProc` language is a variant of the `C/C++` language focused on macros",
        "preproc",
        PreprocCode,
        PreprocParser,
        tree_sitter_preproc,
        [],
        []
    )
);

pub(crate) mod fake {
    pub(crate) fn get_true<'a>(ext: &str, mode: &str) -> Option<&'a str> {
        if ext == "m"
            || ext == "mm"
            || mode == "objc"
            || mode == "objc++"
            || mode == "objective-c++"
            || mode == "objective-c"
        {
            Some("obj-c/c++")
        } else {
            None
        }
    }
}
