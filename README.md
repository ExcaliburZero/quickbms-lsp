# QuickBMS-lsp [![quickbms-lsp](https://github.com/ExcaliburZero/quickbms-lsp/actions/workflows/quickbms-lsp.yml/badge.svg)](https://github.com/ExcaliburZero/quickbms-lsp/actions/workflows/quickbms-lsp.yml)
This is an experimental language server for the scripting language QuickBMS.

![An example of the hover documentation for the "print" keyword being shown in Visual Studio Code for a hello world program.](images/vscode_hover_keyword_documentation_01.png)

![An example of the goto references function for a "GreetWorld" function being shown in Visual Studio Code for a functions example script.](images/vscode_references_function_01.png)

## Language server endpoints
These are the currently supported language server endpoints.
* `initialize`
* `textDocument/definition`
* `textDocument/didOpen`
* `textDocument/didChange`
* `textDocument/documentSymbol`
* `textDocument/foldingRange`
    * Currently only for function declarations, `If/ElseIf/Else` statements, and `Do/While` loops
* `textDocument/hover`
* `textDocument/references`
    * Currently only for functions

## QuickBMS commands
The following QuickBMS commands are currently supported by the langauge server.

* [x] `QuickBMSver VERSION`
    * [ ] Support version numbers that are not specified as strings
* [x] `FindLoc VAR TYPE STRING [FILENUM] [ERR_VALUE] [END_OFF]`
* [x] `For [VAR] [OP] [VALUE] [COND] [VAR]`
    * [ ] Support all operators
* [x] `Next [VAR] [OP] [VALUE]`
* [x] `Get VAR TYPE [FILENUM] [OFFSET]`
    * [ ] Look into `OFFSET` argument. It is in list of commands, but not in the `Get` documentation.
* [x] `GetDString VAR LENGTH [FILENUM]`
* [x] `GoTo OFFSET [FILENUM] [TYPE]`
* [x] `IDString [FILENUM] STRING`
* [x] `Log NAME OFFSET SIZE [FILENUM] [XSIZE]`
    * [ ] Add support for FILENUM and XSIZE
* [x] `Clog NAME OFFSET ZSIZE SIZE [FILENUM] [XSIZE]`
* [x] `Math VAR OP VAR`
    * [ ] Support all operators
* [ ] `XMath VAR INSTR`
* [x] `Open FOLDER NAME [FILENUM] [EXISTS]`
* [x] `SavePos VAR [FILENUM]`
* [x] `Set VAR [TYPE] VAR`
* [x] `Do`
* [x] `While VAR COND VAR`
* [x] `String VAR OP VAR`
    * [ ] Support all operators
* [x] `CleanExit`
* [x] `If VAR COND VAR [...]`
    * [ ] Support all comparison operators
* [x] `[Elif VAR COND VAR]`
* [x] `[Else]`
* [x] `EndIf`
* [ ] `GetCT VAR TYPE CHAR [FILENUM]`
* [x] `ComType ALGO [DICT] [DICT_SIZE]`
    * [ ] Support optional arguments
* [x] `ReverseShort VAR [ENDIAN]`
* [x] `ReverseLong VAR [ENDIAN]`
* [x] `ReverseLongLong VAR [ENDIAN]`
* [x] `Endian TYPE [VAR]`
* [x] `FileXOR SEQ [OFFSET] [FILENUM]`
* [ ] `FileRot SEQ [OFFSET] [FILENUM]`
* [ ] `FileCrypt SEQ [OFFSET] [FILENUM]`
* [ ] `Strlen VAR VAR [SIZE]`
* [x] `GetVarChr VAR VAR OFFSET [TYPE]`
* [x] `PutVarChr VAR OFFSET VAR [TYPE]`
* [ ] `Debug [MODE]`
* [x] `Padding VAR [FILENUM] [BASE_OFF]`
* [x] `Append [DIRECTION]`
* [x] `Encryption ALGO KEY [IVEC] [MODE] [KEYLEN]`
* [x] `Print MESSAGE`
* [x] `GetArray VAR ARRAY VAR_IDX`
* [x] `PutArray ARRAY VAR_IDX VAR`
* [ ] `SortArray ARRAY [ALL]`
* [ ] `SearchArray VAR ARRAY VAR`
* [x] `CallFunction NAME [KEEP_VAR] [ARG1] [ARG2] ... [ARGn]`
    * [ ] Add support for arguments
* [x] `StartFunction NAME`
* [x] `EndFunction`
* [ ] `ScanDir PATH NAME SIZE [FILTER]`
* [ ] `CallDLL DLLNAME FUNC/OFF CONV RET [ARG1] [ARG2] ... [ARGn]`
* [ ] `Put VAR TYPE [FILENUM]`
* [ ] `PutDString VAR LENGTH [FILENUM]`
* [ ] `PutCT VAR TYPE CHAR [FILENUM]`
* [ ] `GetBits VAR BITS [FILENUM]`
* [ ] `PutBits VAR BITS [FILENUM]`
* [ ] `Include FILENAME`
* [ ] `NameCRC VAR CRC [LISTFILE] [TYPE] [POLYNOMIAL] [PARAMETERS]`
* [ ] `Codepage VAR`
* [ ] `SLog NAME OFFSET SIZE [TYPE] [FILENUM] [TAG]`
* [ ] `Reimport [MODE]`
* [ ] `Label NAME`
* [x] `Break [NAME]`
* [x] `Continue [NAME]`