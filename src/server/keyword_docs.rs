use std::collections::HashMap;

macro_rules! keyword {
    ($a: expr, $b: expr) => {
        ($a.to_string(), include_str!($b).to_string())
    };
}

pub fn get_keyword_docs() -> HashMap<String, String> {
    [
        keyword!("print", "keyword_docs/print.txt"),
        keyword!("set", "keyword_docs/set.txt"),
        keyword!("startfunction", "keyword_docs/functions.txt"),
        keyword!("endfunction", "keyword_docs/functions.txt"),
        keyword!("callfunction", "keyword_docs/functions.txt"),
        keyword!("endian", "keyword_docs/endian.txt"),
        keyword!("idstring", "keyword_docs/idstring.txt"),
        keyword!("if", "keyword_docs/if.txt"),
        keyword!("elif", "keyword_docs/if.txt"),
        keyword!("else", "keyword_docs/if.txt"),
        keyword!("endif", "keyword_docs/if.txt"),
        keyword!("goto", "keyword_docs/goto.txt"),
        keyword!("for", "keyword_docs/for.txt"),
        keyword!("next", "keyword_docs/for.txt"),
        keyword!("break", "keyword_docs/label.txt"),
        keyword!("continue", "keyword_docs/label.txt"),
        keyword!("cleanexit", "keyword_docs/cleanexit.txt"),
        keyword!("findloc", "keyword_docs/findloc.txt"),
        keyword!("get", "keyword_docs/get.txt"),
        keyword!("math", "keyword_docs/math.txt"),
        keyword!("log", "keyword_docs/log.txt"),
        keyword!("asize", "keyword_docs/asize.txt"),
        keyword!("long", "keyword_docs/long.txt"),
        keyword!("string", "keyword_docs/string.txt"),
        keyword!("getarray", "keyword_docs/getarray.txt"),
        keyword!("putarray", "keyword_docs/getarray.txt"),
        keyword!("encryption", "keyword_docs/encryption.txt"),
        keyword!("reverseshort", "keyword_docs/reverseshort.txt"),
        keyword!("reverselong", "keyword_docs/reverselong.txt"),
        keyword!("reverselonglong", "keyword_docs/reverselonglong.txt"),
        keyword!("filexor", "keyword_docs/filexor.txt"),
        keyword!("append", "keyword_docs/append.txt"),
        keyword!("getvarchr", "keyword_docs/getvarchr.txt"),
        keyword!("putvarchr", "keyword_docs/putvarchr.txt"),
        keyword!("byte", "keyword_docs/byte.txt"),
        keyword!("comtype", "keyword_docs/comtype.txt"),
        keyword!("clog", "keyword_docs/clog.txt"),
        keyword!("padding", "keyword_docs/padding.txt"),
        keyword!("savepos", "keyword_docs/savepos.txt"),
        keyword!("extension", "keyword_docs/extension.txt"),
        keyword!("getdstring", "keyword_docs/getdstring.txt"),
        keyword!("do", "keyword_docs/do.txt"),
        keyword!("while", "keyword_docs/do.txt"),
        keyword!("short", "keyword_docs/short.txt"),
        keyword!("open", "keyword_docs/open.txt"),
        keyword!("quickbmsver", "keyword_docs/quickbmsver.txt"),
    ]
    .iter()
    .cloned()
    .collect()
}
