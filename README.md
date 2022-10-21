# FuzzerK library + program

Author: HanishKVC,

Version: 20221007IST0039, Saraswathi + Ayudha (Knowledge,Work,Mechanisms,Tools,Processes,...) pooja release

License: GPL


## Overview

This is a simple yet flexible library + program, to test input parsing logic of
programs to see, that they handle all possible input cases sufficiently robustly.

The program being tested could be expecting its input from console(/stdin)
or from a file or over a tcp (client or server) or tls (server) connection.

The data generated to test any program, could be

* predefined and or deterministic in nature wrt the content and or structure.

* totally randomly generated, while maintaining certain structures and or
  predefined content(s) still, if required.

* take predefined data and manipulate in controlled yet random ways.

* or mixture of above.

One can either define parts that make up the end data to feed into the program
being tested, and inturn mix and match them to create the required end structure
in the data being fed. Or one could start with a valid data and manipulate it
in controlled yet random way.

This consits of a library of modules that can be used by other programs, as well
as a scriptable helper utility program (fuzzerk) to test other programs (without
having to modify those programs to integrate with the provided library).


### Library & Helper modules

#### Fuzzers and Chains

At the core, the library defines

* a Fuzz trait, which allows fuzzers to be created.

* and a FuzzChain container to allow fuzzers to be chained together
  to create the needed end pattern of data in a flexible manner.

In turn it also provides a set of predefined fuzzers, which generate new
binary buffer each time they are called, with either random or looped or
mixture of random and presetup data, based on how they are configured.

NOTE: The logics involved maintain and generate binary data, so that
it can contain either textual data or purely binary data or a mixture
of both.

One could define two kinds of fuzzers based on what is needed.

* fuzzers which modify / update themselves each time they are called

  * override need_mutable method of the fuzz trait, to indicate to
    fuzzchains that they need to call the mutable get version.

* fuzzers which dont change internal state, when they are called.

These fuzzers inturn can be chained any number of times within same or
different FuzzChains.

NOTE: Currently the logic is not multithread safe.

* [MayBeInFuture: MultiThreading safe] Use Arc (AutomicRC) and mutex to
  allow it to be chained more than once and inturn to be used from a
  multithreaded context.

One could use these traits and predefined entities directly, in their
programs, or through the Rtm and Cfgfiles mentioned below or through
the FuzzerK program.

#### CfgFiles

A generic config file handling module.

It allows one to parse a config file, consisting of Config groups.

Each Config group is made up of a bunch of non empty lines, with empty
lines before and after it. If the Config group is at the begin of the
config file, then empty line before such a config group is optional.
Similarly if the config group is at the end of the config file, then
empty line after such a config group is optional.

A config group's 1st line should have a alpha numeric char as the 0th
char in that line.

If a line's 0th char begins with #, then it is treated as a comment
and is skipped.

One needs to call its parse_file function, passing it the config file
to parse and the handler to call for handling the config groups found
in the file.


#### Rtm

The runtime manager, is a helper module to allow the creation of predefined
fuzzers, and its chaining into fuzzchains, based on the config info in the
config groups (from config file) passed to it.


#### IOBridge

This is a helper module for the fuzzerk util program to help work with either
Console or Tcp client or Tcp server or Tls server or File, in a generic way.
It could even be used as a module by any other program if required.

It is implemented as a enum, which inturn supports the following variants.

* console

* tcpclient:addr:port

  * addr could be ip addr or domain name

  * this can be used to simulate a tcpclient and will allow to connect
    with a tcp server program.

* tcpserver:addr:port

  * addr could be ip addr or domain name

  * this can be used to simulate a tcpserver and will allow a tcp client
    program to connect to it, so that tcp client program can be tested.

* tlsclient:addr:port

  * addr could be ip addr or domain name

* filewriter:path/to/file


#### VM

This is a helper module for the fuzzerk's operations to be controlled by the
end user using custom asm script files.

It provides a VM, with a set of useful instructions, which can be used by the
asm files. The mnemonics used in the asm script files have the simplicity of
assembly instructions, while at the same time supporting higher level language
features like mentioned in the section below.

It inturn uses Rtm and Cfgfiles module to instantiate fuzzers and fuzz chains
as defined by the end user.


### Minimal FuzzerK Util

A program which uses the modules mentioned previously to help test other
programs by generating fuzz input for them and pushing to them using either
console or file or tcp or tls session.

This allows a end user to quickly test their program using this fuzzer logic,
without needing to modify their program to handshake with the fuzzer library
provided by this package. They just need to write few simple text based control
files and inturn test their program, provided their program takes inputs over
the stdin or a file or a tcp session or a tls session.

It also parallely allows one to test the libraries/modules provided here, in a
simple yet flexible and potentially useful way.

It includes a basic application specific vm and scripting language to easily
control its operation in a flexible way. It does a basic quasi ahead of time
compilation of the asm script file and inturn runs the generated program.

One could either just define the fuzzers and the fuzz chains and let the vm
run it directly using a default builtin fallback script. Or if one wants more
control and flexibility, then one could create a asm script file.

The vm and the scripting language inturn

* support basic programming constructs like variables, arithmatic operation,
  condition checks, labels and conditional and unconditional gotos and calls,
  io operations and so.

* The scripting language follows a simple syntax like assembly mnemonics, while
  at the same time providing access to modern conviniance features like

  * global and local variables,

  * variant data type,

  * automatic variable type inference,

  * functions with arguments and recursion support,

  * conditional function calls,

  * io abstraction,

  * ...

* To help with easy debugging and fixing of any issues in the asm script code

  * source code line number is tracked and printed wrt compile and run
    phases, if any error is detected.

  * on error during execution, a back trace is shown of the call stack
    in reverse order, with following info

    * CallStackDepth, IPtr,

    * CallOp(FuncName and args), SrcLineNum,

    * Mapping of Func Args to passed vars and or literal values


### Dependencies

* my helper libraries from github.com/hanishkvc

  * loggerk, argsclsk

* 3rd party rust libraries

  * boring, rand


## Usage Flow possibilities

### Whether to use Library or Program

One could use the logics of this system, in few different possible ways

* instantiate and use the Fuzzers and FuzzChains provided by the core
  library directly in your program.

* use the Rtm and Cfgfiles along with core library, to allow a end user
  to dynamically create the required fuzzchains by defining config files.
  THe end user will be able to use the provided fuzzers (existings ones
  provided by the library and or additional fuzzers created by you).

  In turn your program uses the fuzzchains as needed.

* use the fuzzerk utility program to exercise your console or network
  based program. Here the program being tested/exercised doesnt require
  to be modified. Rather one could

  * create a config file (containing fuzzers and fuzzchains), and then
    specify the specific fuzzchain and iobridge mechanism to use as
    cmdline args. This is good enough for many simple test cases.

  * create a config file (containing fuzzers and fuzzchains) and the
    builtin VM related asm script file. This allows more complex
    test cases to be realised.


### Wrt fuzzing

One could either build a fuzz chain made up of parts of the data that
is needed. Or one could specify the ideal data and then let the logic
randomly change it in a controlled manner. Or use a combination of both.


## Runtime

### Control files

The below are the control files used by the minimal fuzzerk program
available in this package/crate.

#### FuzzerChains File

##### Overview

This configures and instantiates one or more predefined fuzzers and the
fuzz chains created using them. End user can create these files and then
pass it to the program, so that at runtime the needed fuzz chain can be
created without having to recompile things, ie provided they can make do
with the fuzzers that is already provided.

Alert: Dont intermix tab and spaces, even thou visually both may appear
to be equivalent, the logic will not accept such intermixing, bcas it
cant be sure how many spaces a tab represents in a given context/instance.

##### The Template

NOTE1:RawMD: The 4 spaces at beging of below set of lines is for markdown
to identify the below lines has a block of data to be retained as such
by markdown.

   
    FuzzerType:TypeNameABC:InstanceNameABC1
      Arg1: IntValueX
      Arg2: StringValueM
      Arg3: String   ValueN
      Arg4: "   String Value with SpacesAt Ends "
      Arg5: $0xABCDEF0102030405060708090A303132323334
      ArgX: String\tValueY\n
      ArgA:
        Value1,
        Value2,
        ValueXYZ
   
   
    FuzzerType:TypeNameXYZ:InstanceNameXYZ99
        Arg1:
            ValueA,
            Value   B,
            Value\tWhatElse\nC\t,
            " Value\tWhatElse\nF   ",
            $0x3031203234203536,
            ValueZ,
        Arg2:
            ValueX,
            ValueM,
            ValueN,
   
   
    FuzzChain:FuzzChain:FC100
        InstanceNameABC1
        InstanceNameXYZ99
        InstanceNameXYZ99
   

NOTE: The sample template above, also shows how string (textual or binary or
a mixture of both) can be specified in different ways, based on what one needs.

##### Sample file

    # A http test fuzzchain config file
    
    # Mostly Ok req type, except for that last NOTME
    FuzzerType:RandomFixedStringsFuzzer:OKOK_REQ_TYPE
      list:
        "GET ",
        "PUT ",
        "NOTME",
    
    FuzzerType:RandomRandomFuzzer:RANDOM_DATA
            minlen: 3
            maxlen: 8
    
    FuzzerType:RandomFixedFuzzerPrintables:RANDOM_PATH
            minlen: 3
            maxlen: 64
    
    FuzzerType:RandomFixedFuzzer:MAYBE_Space
            minlen: 1
            maxlen: 3
            charset: $0x02090A30313233342F
    
    FuzzerType:RandomFixedFuzzer:OK_SPACE
            minlen: 1
            maxlen: 1
            charset: $0x20
    
    FuzzChain:FuzzChain:FC100
            OKOK_REQ_TYPE
            RANDOM_PATH
            MAYBE_Space
            RANDOM_DATA
    
    # Valid http req inbetween
    
    FuzzerType:RandomFixedStringsFuzzer:OKOK_HTTP_PATH
            list:
                    /index.html
    
    FuzzerType:RandomFixedStringsFuzzer:OKOK_HTTP_VER
            list:
                    http/v1
    
    FuzzChain:FuzzChain:FC_OkReqInBtw
            OKOK_REQ_TYPE
            OKOK_HTTP_PATH
            OK_SPACE
            OKOK_HTTP_VER
    
    # use Buf8sRandomize to generate lot invalid and few valid requests
    
    FuzzerType:Buf8sRandomizeFuzzer:REQS_LIST
            buf8s:
                    GET /index.html http/v1
                    PUT /index.new.html http/v1
            randcount: -1
            startoffset: -1
            endoffset: -1
            startval: -1
            endval: -1
    
    FuzzChain:FuzzChain:FC_B8sR
            REQS_LIST
    


##### Types of data

As part of the key-value(s) pairs specified in fuzz chains config file, currently
the value(s) specified could be

* single int

  * key:value | key: value

* single string

  * key: value | key: " value with spaces at ends   "

  * key: $0xABCDEF010203523092 | key: value\\n with \\t newline in it

* list of int or string data

The int data needs to be decimal literal.

The string data could be

* a bunch of textual words/chars with literal single line white spaces
  (ie normal space and tab space) between them.

* string data could have white spaces at the begin or end by

  * having the string enclosed within double quotes

  * having the white spaces specified has escape sequences (\\t, \\n, \\r)

    * this also allows newline or carriage return to be embedded anywhere
      within the string data.

* binary or a mixture of textual and binary data by having the string data
  specified has a hex string which begins with $0x

The list can be specified in one of the following ways

* if the list has only a single value then

  * key: value OR

  * key:
      value a single value

* if the list has multiple values then

  * key:

      value 1

      value 2

      value 3 comma at the end is optional,

      more values

      ...

   * key: NumOfValues

        Value 1 out of NumOfValues(NOV)

        Value 2 out of NOV

        ...

        Value NOV of NOV

NOTE: The empty lines between values of the list and two adjacent slashes wrt
escape sequences are things done to satisfy rst format requirements.

##### Predefined Fuzzers

The following type of predefined fuzzers is provided by default

* ones that work with mainly provided data, without changing them

  * LoopFixedStringsFuzzer

    * each time it is called, it returns/appends the next string from the
      list of strings.

    * once the end of list is reached, it moves back to begining of the list

  * RandomFixedStringsFuzzer

    * each time it is called, it returns/appends a randomly selected string
      from the lsit of provided strings.

  * DONE: Currently the list of provided strings is treated as textual strings
    Rather convert it to a list of binary buffers, so that it can either store
    binary data or textual data (in its binary form) or a mixture of both.

* those that use random generation to a great extent

  * RandomRandomFuzzer

    * return/append a randomly generated buffer of random binary values

      * whose length is randomly decided from a given min and max length limit.

  * RandomFixedFuzzer

    * return/append a buffer, whose values are randomly selected from a given
      list of binary values.

      * whose length is randomly decided from a given min and max length limit.

      * the list of binary values to be used for selection, can be specified
        has a textual string or a hex string or so

* ones that take predefined / provided data and inturn change it

  * Buf8sRandomizeFuzzer

    * return/append a buffer which contains one of the originally provided data,
      with some amount of random modifications to its contents, as noted below.

    * one needs to provide the following info/data

      * a list of strings (textual or binary or mixture of both)

      * the number of bytes to randomly modified

        * if not explicitly predefined (ie if set to -1), then it is randomly
          decided as to how many bytes should be randomly modified.

      * the new random byte values are selected to be within a specified range
        of values.

        * if start value is not specified, it is assumed to be 0

        * if end value is not specified, it is assumed to be 255
          The end value is inclusive in the logic and will be used as part of
          the possible range wrt new values to use when changing existing value
          with new values.

      * the positions that are randomly modified are selected randomly, but
        inturn restricted to be within a specified range of positions.

        * if start position is not specified, it is assumed to be 0

        * if end position is not specified, it is assumed to be till end
          of the provided original buffer.
          The end position specified is inclusive in the logic and will be used
          as part of the possible range of positions that may be randomly selected
          for changing the byte value wrt that position.


##### Custom Fuzzers

If required the library can be extended to add custom fuzzers (they need to support
the fuzz trait).

If a custom fuzzer has to be created from the textual FuzzChains config file, then

* the fuzzer needs to support cfgfiles::FromVecStrings trait

  * and its from_vs method

  * inturn it can use the predefined helper functions of this trait to parse config
    file, to help create instance of the custom fuzzer, based on users configuration
    of the same.

* the RunTimeManager.handle_cfggroup needs to be updated to create the custom fuzzer

  * by calling the custom fuzzer's from_vs method



#### Asm script file

##### Overview

This allows the end user to control the actions to be performed by fuzzerk, in a simple and flexible way.

##### General

###### Data and or Variable

The VM and inturn the script file, supports what is called a variant data type. Which allows one to specify
data in one of the different supported formats (integer, string, binary buffer) and inturn it will try to
transparently convert it to the required end usage format.

Where ever var_or_value is mentioned wrt instruction operands, the text-tokens/content specified in the
corresponding location in the prgfile will be interpreted as below.

If it starts with a numeric char or + or - will be treated has a numeric/integer literal.

* if it starts with 0x, then it will be treated has a hexadecimal integer value

* else it will be treated has a decimal integer value

If it starts or ends with double quotes, it will be treated as a string literal.

* this also allows spaces to be specified at begin or end of the string literal.

* a small set of escape sequences (\\n, \\t, \\r, \\") are supported within these strings.
  These will be replaced with equivalent char.

* The double quote at the begin and end of string literal will be dropped.

If it starts with $0x then it will be treated has a binary buffer specified has a hex string.

If it starts with __ then it will be treated has a special data value.

* \_\_TIME\_\_STAMP\_\_

  * This puts the current time stamp in terms of milliseconds from UnixEpoch, in a suitable way.

* \_\_RANDOM\_\_BYTES\_\_TheLength

  * This puts TheLength amount of random bytes into var, in a suitable way.

If none of above, then it will be treated as a var name. However it should start with a alphabhetic char.

The VM maintains the data internally as a flexible variant type. It allows one type of data to be transparently
treated as a different type, by applying relatively sane default conversion rules.

Where ever

* int_var_or_value is mentioned wrt instructions, then one should provide a int variable or value.
  However if a different type data is provided, the following conversion will be applied

  * treat strings as textual literal representation of a integer.

  * treat buf as raw byte values corresponding to a integer.

* str_var_or_value is mentioned wrt instructions, then it should represent a string variable or value.
  However if a different type entity (value or variable) is provided, then

  * ints will be converted to their textual literal representation

  * bufs will be converted to hex string. (So that even if there are non printable characters, they are not lost)

  * Strings are generally really useful, when displaying something to the screen or when writing to a text file.

* buf_var_or_value is mentioned wrt instructions, then it should represent a binary buffer variable or value.
  However if different type entity is provided then the underlying byte values of the source variable (or literal
  after suitable interpretation) is retained.

  * int's underlying raw byte values will be passed as a binary buffer.

  * string's underlying raw byte values will be passed as a binary buffer.

  * binary buffers are useful for most cases except may be when printing to console/screen

Where ever any_var_or_value is mentioned wrt instructions, it could represent int or string or binary buffer
variable or value.



###### Clean coding (Comments, White spaces)

If required one can put extra spaces around operands to align them across lines or so, for easy reading.

If required one can put # at the beginning of a line to make it a comment line.


##### Ops/Instructions supported

The commands/operations that can be specified as part of the asm script file include

###### Data/Variables Related

Variables can be either global or local. Additionally one can specify a default value type wrt the variable,
which could be int(i) or str(s) or buf(b, a binary buffer).

####### Global variables

* letglobal.s|letstr <string_var_id> <str_var_or_value>

  create a str var and set its value

* letglobal.i|letint <int_var_id> <int_var_or_value>

  create a int var and set its value

* letglobal.b|letbuf <buf_var_id> <buf_var_or_value>

  create a binary buffer and set its value.

* letglobal <var_id> <any_var_or_value>

  This creates a global variable, whose type is infered based on the type of the source entity specified.

* bufnew <buf_var_id> <bufsize_int_var_or_value>

  Create a named buffer of a given size

* bufmerged[.b]|bufmerged.s dest_buf_var_id src1_any_var_or_value src2_any_var_or_value ..... srcn_any_var_or_value

  This allows a new buffer to be created, which contains the contents of the specified source data items.

  The source item could be either a int or str or hexstring(buf literal value) or it could be a variable of
  any supported type.

  if bufmerged or bufmerged.b is used, then the raw byte values corresponding to the specified src item will
  be used. This is useful when one needs to send underlying byte values corresponding to specified items/values
  like when sending to another program or storing into a binary file or so.

  if bufmerged.s is used, then equivalent string representation of the specified src item will be used. This is
  useful especially, when writing to console or so, where user will be interested in a human readable textual
  form of the underlying data.

####### Local variables

One can use letlocal to create a local variable

* letlocal[.i|.s|.b] <var_id> <suitable_var_or_value>

This works similar to how letglobal and its variants work, except that the variable is created in the localstack
and not in the global hashmap.

####### Special operations

* buf8randomize buf_var_id randcount buf_startoffset buf_endoffset rand_startval rand_endval

  * all the int arguments (ie other than bufid) belong to the int_var_or_value class

  * randomize randcount values from with in a part (start and end offset) of the buf
    with values from a given range (start and end value).

  * other than buf_var_id, other arguments are optional and if not given a suitable default value
    will be used

    * randcount - randomly generated to be less than buflen

    * buf_startoffset and buf_endoffset map to begin and end of buffer being operated on, if not specified.

    * rand_startval will be mapped to 0 and rand_endval to 255, if needed.
      Both these need to be u8 values, else it will be truncated to u8.

  * inclusive ends

    * buf_endoffset is inclusive, that is value at corresponding index may be randomized, if it gets
      randomly selected during running/execution of the buf8randmoze instruction/operation.

    * rand_endval is inclusive

* getsize var_or_value_ToSize WriteSizeTo_var

  returns the size (in bytes) of the data specified as a var or literal value.


###### Alu Operations

Both arithmatic and logic operations are supported.

####### Arithmatic operations

Arithmatic operations interpret their data as integer values. If one uses
a string or binary buffer related var or value, as source data, instead of
int var or value, it will be converted into a int value, using rules
specified previously, so it should contain int value in it in the reqd way.

* inc <int_var_id>

* dec <int_var_id>

* add <dest_int_var_id> <src1_any_var_or_value> <src2_any_var_or_value>

* sub <dest_int_var_id> <src1_any_var_or_value> <src2_any_var_or_value>

* mult <dest_int_var_id> <src1_any_var_or_value> <src2_any_var_or_value>

* div <dest_int_var_id> <src1_any_var_or_value> <src2_any_var_or_value>

* mod <dest_int_var_id> <src1_any_var_or_value> <src2_any_var_or_value>

####### Logical operations

Logical operations interpret their data as binary buffer values and do
unsigned logical operation on them.

* and <dest_buf_var_id> <src1_any_var_or_value> <src2_any_var_or_value>

* or <dest_buf_var_id> <src1_any_var_or_value> <src2_any_var_or_value>

* not <dest_buf_var_id> <src1_any_var_or_value>

* xor <dest_buf_var_id> <src1_any_var_or_value> <src2_any_var_or_value>


###### IOBridge related

This allows the user to work with different types of io channels in a potentially transparent and easy manner.

* iobnew <iob_id> <iobtype:typespecific_addr> <typespecific_ioarg=value> <typespecific_ioarg=value> ...

  * supported iobtypes include

    * console - for writing generated data to stdout

      * NOTE that there could be more textual info seen on the screen, but they are written to stderr,
        so that the fuzzers and fuzzchains and their generated data is not disturbed.

    * tcpclient - for connecting to a tcp server; tcpserver - for allowing a tcp client to connect

      * addr => <ipaddr|domainname><:port>

      * ioargs supported

        * read_timeout=millisecs

    * tlsclient

      * addr => <ipaddr|domainname><:port>

      * ioargs supported

        * server_cert_check=yes/no

        * domain=the.domain.name

        * read_timeout=millisecs

    * filewriter

      * addr => path/to/file

      * ioargs supported

        * append=yes/no

        * create=yes/no

* iobwrite <iob_id> <buf_any_var_or_value>

  * write the underlying raw byte contents (ie a binary buffer) of the specified var or literal value into the specified iobridge

* iobflush <iob_id>

  * request flushing of any buffering of written data by the library and or os into the underlying io device

* iobread <iob_id> <buf_var_id>

  * try to read upto specified buffer's buffer length of data from the specified iobridge

    * one can use bufnew to create buffer of a required size with no data in it.

    * remember to set the length of the buffer to be read into, to be equal to the max amount of data that one wants to read, at one go.

  * while creating a new iobridge remember to set a read_timeout, so that read wont block indefinitely, if there is no data to read.

    * all io bridge types may not support read_timeout (currently only network types ie tcpclient, tcpserver and tlsclient support it).

* iobclose <iob_id>


###### Fuzzers related

* fcget <fc_id> <buf_var_id>

  Generate a fuzzed buffer of data and store into buffer of specified id.


###### Control/System related

* sleepmsec <milliseconds_int_var_or_value>


* !label <label_id>

  a directive to mark the current location/address in the program where this directive is encountered

  This can be the destination of either if-goto|if-jump or jump|goto or checkjump

  ie act as the destination of conditional/unconditional jumps


* !func <func_id> [<func_arg1_name> <func_arg2_name> ...]

  a directive to mark the current location/address in the program where this directive is encountered
  as a function and inturn its name.

  Additional specify a list of function arguments.

  * these function arguments can inturn be only used as src operands and not as destination operands

    * ie they can be read from and not written to

  * the caller can pass variables and or literal values wrt these args.

    * if literal values are passed, then during AOT compilation, instructions to create temporary variables
      to correspond to these will be created and inserted before the call instruction, which uses/passes
      literal values wrt function arguments. These will be created as local variables of the current func
      if called from within a function, else these will be created as global variables. This should ensure
      automatic clearing of these temporary variables if called from within functions.

  Any function has access to

  * the global variables,

  * the local variables, which are defined within a funcion,

  * as well as function arguments specified directly wrt it

  Functions doesnt have access to function arguments or local variables specified wrt any of its parent
  callers, directly. Unless the same is passed to the child function, by passing down the call chain,
  has functions' arguments.

  When ever a variable is used, 1st it is checked wrt

  * the current function's arguments, and if so, then inturn
    the corresponding local variable from a parent caller or a global variable.

  * the current function's local variables list

  * only if not found in above checks, it will be checked for in global variables list.

  One needs to end the func body with a ret instruction


* if condition check

  These check values specified between themselves and inturn either call spcified function or goto specified label

  The values can be specified either has literal values of the required type, or has a variable.

  * Check involving integers

    * iflt|iflt.i|ifgt|ifgt.i|ifeq|ifeq.i|ifne|ifne.i|ifle|ifle.i|ifge|ifge.i <value1_int_var_or_value> <value2_int_var_or_value> goto <label_id>

    * iflt|iflt.i|ifgt|ifgt.i|ifeq|ifeq.i|ifne|ifne.i|ifle|ifle.i|ifge|ifge.i <value1_int_var_or_value> <value2_int_var_or_value> call <func_id> [passed1_any_var_or_value passed2_any_var_or_value ...]

  * Check involving string and buffer

    * ifeq|ifeq.s|ifne|ifne.s <val1_str_var_or_value> <val2_str_var_or_value> goto <label_id>

    * ifeq|ifeq.s|ifne|ifne.s <val1_str_var_or_value> <val2_str_var_or_value> call <func_id> [passed1_any_var_or_value passed2_any_var_or_value ...]

    * ifeq|ifeq.b|ifne|ifne.b <val1_any_var_or_value> <val2_any_var_or_value> goto <label_id>

    * ifeq|ifeq.b|ifne|ifne.b <val1_any_var_or_value> <val2_any_var_or_value> call <func_id> [passed1_any_var_or_value passed2_any_var_or_value ...]


* checkjump arg1_int_var_or_value arg2_int_var_or_value Label4LessThan Label4Equal Label4GreaterThan

  * based on whether int value corresponding to arg1 is lt or eq or gt wrt arg2,
    the logic will jump to either Label4LessThan or Label4Equal or Label4GreateThan,
    as the case may be.

  * __NEXT__ a implicit label identifying the next instuction/op in the program

    * useful if one doesnt want to jump to any specific location for a given condition,
      then the control will implicitly flow to next instruction in the program, in that case.


* jump|goto <label_id>

  * a unconditional jump


* call <func_id> [passed1_any_var_or_value passed2_any_var_or_value ...]

  * call a func

  * If the func being called requires arguments to be passed to it, then one needs to specify
    the corresponding/matching variables or values that should be passed to the called function.

    The passed variables and or values could be

    * global variables or

    * literal values (int or string or buf type)

    * if the call is being made from within a function, then

      * local variables, if any, belonging to the current function

      * any function arguments belonging to the current function


* ret

  * return from the current func


##### A sample file

           letstr <strvarid> <string value>
           letint <intvarid> <intvalue>
           iobnew <iobid> <iobtype:addr> <ioargkeyX=valY> <ioargkeyA=valC>
    !label labelid
           fcget <fcid> <bufid>
           iobwrite <iobid> <bufid>
           sleepmsec <milliseconds>
           iobread <iobid> <bufid>
           iobclose <iobid>
           inc <intvarid>
           iflt <intvarid> <chkvalue> goto labelid
    !label labelid_named_unneeded
           dec <intvarid>



### Cmdline

The key cmdline options are

* --cfgfc <path/to/fuzzers_fuzzchains.cfgfile>
* --prgfile <path/to/asm_script_file>

There are few additional options, in case one is not using a prgfile (ie asm script file)

* --ioaddr <iobtype:addr>
  * defaults to console, if not explicitly specified.
* --ioarg <ioargkeyX=valY>
  * defaults to no args, if not explicitly specified.
* --loopcnt <number>
  * defaults to 1, if not explicitly specified.
* --fc <fcid>
  * defaults to empty string, if not explicitly specified.

Additional options

* --blogdebug <yes|true>
  * enable printing of debug messages. Defaults to disabled.


## TODO Plus


### DONE

#### Previously

* end of prgfile

  * implicit end of prgfile taken care of

  * [TODO:MAYBE] Add a option for explicit !end directive or so
    Will allow functions to be defined after the normal flow is
    explicitly ended. Otherwise currently functions will have
    to put between ideally a unconditional jump at the begin
    and the start label/code.

* the fallback predefined program in case

  * prgfile is not specified

  * instead fc, loopcnt, ioaddr, ioarg etal is passed.

* iobclose and ssl session shutdown

  * keep it simple for now and just verify the 1st shutdown returns a Sent result.
    As noted in git commit logs, calling it 2nd time with or without reading of
    any left over data etal, doesnt seem to work with getting the Recieved result.
    Rather a syscall error is what is got, if there is no more data to read. So
    keep it simple for now and just ensure that 1st shutdown call leads to a
    proper Sent result.

* specify strings flexibly in cfgfc files, when defining fuzzers. As needed
  one could

  * use hex strings to intermix text and binary data,

  * use double quoted string to allow white spaces at either end of the string

* A Fuzzer which allows predefined string(s) to be randomly changed in a controlled way,
  wrt some random positions in the string and the values to use when changing.

  * This fuzzer takes a predefined list of strings, and inturn randomly changes one of
    them, when ever it is called to generated a fuzzed data.

* Allow user to use either a int variable or int literal value interchangably,
  in following instructions, where a int value is required.

  * letint, iflt, checkjump, sleepmsec, alu ops, bufnew, buf8randomize

  One needs to use $ prefix before a int literal to tell the vm compiler that
  it is a int literal value and not a int variable.

  TOTHINK: Should I add it in other places like wrt bufnew's buffer size arg, ...



#### 20221009++

* Add ALU commands add, sub, mult, div, mod

* Make letbuf more flexible by allowing either

  * literal int or textual or hex string values

  * int or str or buf variable

    * letbuf which tries to get binary data wrt vars

    * letbuf.s which tries to get string literals corresponding to these var

* switch order of value args check wrt iflt, so that it is similar to that of checkjump.

* VM: simplify and cleanup the Data var or value interpretation, through DataM mechanism.

* allow extra unneeded whitespaces in between operands of the instructions.

* Allow all VM Op int literals to use the flexible and better DataM based flow

* More flexible if condition checks using a new CondOp enum ++

  * Add support for ifeq|ne wrt int | string | buf types

  * Add support for iflt|gt|le|ge wrt int

  * Either goto a specified label or call a specified function

* DataM support some basic esc sequences wrt string literals

* BufMerged to merge different type vars and or values on a single line

* dont allow more than one variable to have same name, ie across var types.

* if-goto use Op::GoTo::run rather than duplicating goto's code in if-goto

* explicitly marked functions (!func) and Allow arguments to be passed to a function

* Use DataType to decide whether a var name is FuncArg or Global (or in future Local)
  and inturn handle appropriately.

  * initial logic was running through the func args aliases hashmap, for all vars
    however now with data type meta data, this hashmap blessing is only done for
    actual func args.

* iobwrite now works with DataM for its src operand.

#### 20221013++

Functions

* Add support for local variables by maintaining a stack of local variables wrt function
  calls. This also allows recursion, obviously limited by available memory.

  * add support for writing into a local variable, as part of operations other than setlocal

  * allow them to be used in arithmatic operations, as source data

DataType enum renamed to DataKind and inturn Variable or FuncArg rather than Global or FuncArg.

Switch to a new proper minimal Variant data type and Var's built on top of these Variants.
There is no longer seperate hashmaps wrt different basic data types. Now there is a single
hashmap for global vars and another (rather a stack of those) wrt local vars. The dest operands
are also DataM's.

* The overall flow is simplified and cleaned up as part of these and related changes.

The global and local var's can be specified to be of a specific type or else based on the src
data type, the new variable will auto infer its suitable type.

Logic for Getting and Setting of variables moved into common helper logics.

Maintain src line number with compiled ops, so that errors noticied while running the op, can
be linked back to the source for easy debugging and fixing.

Allow local variables in a function to be passed down a call chain, by passing it as func
arguments.

* func arguments and readwrite and locavariables etal

  * the function arguments mechanism is implemented has a stack of fargsmaps, wrt each func
    in the call chain. Each map entry contains a index (in addition to the real var name)
    which indicates whether the farg represents a global variable or a local variable and
    inturn in case of local variables, where in the function local variables stack it exists.

  * even thou currently writing into a farg is not supported, the logic implemented allows
    the same to be achieved easily by updating the Context::VarSet to handle the same by
    getting the location of the real variable behind the function argument, and inturn
    updating the corresponding variables-hash-map.

Allow literal values to be used as function arguments. These will be converted to temporary
variables and used across the function chain. The logic tries to keep the memory use wrt
these temp variables in control by creating local variables, if called from within a func.
Only temp variables related to literals for calls outside of any functions, will remainin
in the global variables space.

Add VarSpace Enum to identify location of variable wrt FargsMap, VarGet, etal.

Disable debug logs by default.

Add Alu logical operations.

Trap panic wrt code execution within the VM, and inturn show a backtrace of the callstack
at that time, so that it is easier for debugging and fixing the issue wrt the prg script
file.

Allow almost similar literal value representation wrt FC Config files and Prg script files.

* both use $0x as the starting prefix wrt hex strings.

* both support similar escape sequences

* However wrt cfg files and string values,

  * it is not necessary to have double quotes around string values

  * if there is double quotes only at one end of the string value, it wont be removed.

  * one can use double quotes anywhere within the string except for either end of string,
    without needing escaping.

Simplify fuzzers and their chains by removing fuzzchainimmut and making fuzzchain (mutable)
more flexible, interms of being able to add the same fuzzer multiple times as well as
explicitly setting the step to use wrt a fuzzchain.get.

Sometimes iobread in TCPServer.Prg was reading more than once, when nc sends data to it once.
* On checking if it was related to any signal coming in between, when reading, it doesnt seem
  to be the case. Read is not failing with a Interrupted error.
* On thinking further, and looking at the asm script file as well as the writen file, and also
  few more test runs, realised the obvious it was triggering when I was writing/entering more
  data than the buffer size.
* Rather has iobread logic uses the length of the buffer passed to it to decide, how much data
  to read, so if one passes data more than the given buffers length, then the 2nd time read is
  called, it will read the remaining data from the 1st send. Now If this remaining data is
  smaller than the initial buffer length specified, then subsequent iobread, will try to read
  only that much data at a time, bcas buf length after iobread is amount of data read.

  So dont forget to set the buffer length using bufnew, each time, b4 calling iobread.


#### 20221020++

A simple http fetch (tlsclient/tcpclient) prg as well as a simple benchmark prg.

Move some of the op specific parsing to compile time.

Add a 2nd compile phase to resolve jump labels, so that conditional gotos which can be used
as part of loops etal has less work at runtime.

Add ldebug macro to eject debug prints of VM from runtime, so that they dont impact the vm
performance unnecessarily, in release builds. This improves vm run speeds by 6 times.

Use common helpers wrt Jump and Call, whether direct or through if-conditional running.


### TODO

* In http tls single session multi request testing (with invalid data and with my experimental
  rust based webserver)

  * if 10msec btw requests, then server seems to get all requests.

  * if 1000msec btw requests, then server seems to only get the 1st request most of the time
    Maybe bcas of any timeout I may have set wrt keeping a session alive or so?

  * ALERT: Need to check what happens with valid http requests instead of invalid http requests.
    Also need to check wrt a standard web server, to verify as to its not a issue at fuzzerk end.

* Maybe: Merge TcpClient and TcpServer into a single entity in the IOBridge enum, and may be
  even merge Tls with Tcp entity. Obviously the new_iobtype helpers wrt each specific type, needs
  to be different, but beyond that it could be single, if things are kept simple.

* Maybe: Add support for string/buf data type wrt iflt|gt|le|ge

