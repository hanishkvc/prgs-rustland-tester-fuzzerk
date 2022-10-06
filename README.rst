####################
FuzzerK library+
####################
Author: HanishKVC,
Version: 20220819IST0750


Overview
##########

This can be used to test input parsing logic of programs to see, that
they handle all possible input cases sufficiently robustly.

The program could be expecting its input from console(/stdin) or from
a file or over a tcp or tls connection.


Library & Helper modules
||||||||||||||||||||||||||

Fuzzers and Chains
===================

At the core, the library defines

* a Fuzz trait, which allows fuzzers to be created.

* and FuzzChain containers to allow fuzzers to be chained together
  to create the needed end pattern of data.

In turn it also provides a set of predefined fuzzers, which generate new
binary buffer each time they are called, with either random or looped or
mixture of random and presetup data, based on how they are configured.

NOTE: The logics involved maintain and generate binary data, so that
it can contain either textual data or purely binary data or a mixture
of both.

One could define two kinds of fuzzers based on what is needed.

* fuzzers which modify / update themselves each time they are called

  * they cant be used/chained more than once, currently.

* fuzzers which dont change internal state, when they are called.

  * these can be chained any number of times within same or different
    chains.

Two kinds of FuzzChainers are provided currently

* one which allows its members to modify themselves (rather their
  internal context). Currently one cant use the same instance of such
  muttable fuzzers more than once. (Later may add a RefCount and if
  reqd mutex to allow it to be chained more than once and also inturn
  to be used from a multithreaded context).

* one which expects its members to work, without updating/modifying any
  of their internal context. Such fuzzer instances can be reused as
  required within a single or even across multiple such chains.


CfgFiles
==========

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


Rtm
=====

The runtime manager, is a helper module to allow the creation of predefined
fuzzers, and its chaining into fuzzchains, based on the config info in the
config groups (from config file) passed to it.


IOBridge
==========

This is a helper module for the util program to help work with either
Console or Tcp server or Tls server or File, in a generic way.

* console

* tcpclient:addr:port

  * addr could be ip addr or domain name

* tlsclient:addr:port

  * addr could be ip addr or domain name

* filewriter:path/to/file


VM
====

This is a helper module for the util program's operations to be controlled
by the end user using custom prg files.

It provides a VM, with a set of useful instructions, for use by these program
files.

It inturn uses Rtm and Cfgfiles module to instantiate fuzzers and fuzz chains
as defined by the end user.


Minimal FuzzerK Util
|||||||||||||||||||||

A program which uses the modules mentioned previously to help test
other programs by generating fuzz input for them and pushing to them
using either console or file or tcp or tls session.

This allows a end user to quickly test their program using this fuzzer
logic, without needing to modify their program to handshake with the
fuzzer library provided by this package. They just need to write some
simple text based control files and inturn test their program, provided
their program takes inputs over the stdin or a tcp session or a tls
session.

It also allows one to test the libraries/modules in a simple yet
flexible and potentially useful way.



Usage Flow possibilities
##########################

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
    builtin VM related program/script file. This allows more complex
    test cases to be realised.



Runtime
#########

Control files
||||||||||||||||

The below are the control files used by the minimal fuzzerk program
available in this package/crate.

FuzzerChains File
===================

Overview
-----------

This configures and instantiates one or more predefined fuzzers and the
fuzz chains created using them. End user can create these files and then
pass it to the program, so that at runtime the needed fuzz chain can be
created without having to recompile things, ie provided they can make do
with the fuzzers that is already provided.

Alert: Dont intermix tab and spaces, even thou visually both may appear
to be equivalent, the logic will not accept such intermixing, bcas it
cant be sure how many spaces a tab represents in a given context/instance.

The Template
---------------

NOTE1:RawRST: The | (and one space after that for non empty lines) is for
rst to identify the below lines has a block of data to be retained as such
by rst.

NOTE2:RawRST: The two slashes \\ below is to work with rst format,
in reality it is only a single slash \ as part of the escape sequence.

|
| FuzzerType:TypeNameABC:InstanceNameABC1
|   Arg1: IntValueX
|   Arg2: StringValueM
|   Arg3: String   ValueN
|   Arg4: "   String Value with SpacesAt Ends "
|   Arg5: 0xABCDEF0102030405060708090A303132323334
|   ArgX: String\\tValueY\\n
|   ArgA:
|     Value1,
|     Value2,
|     ValueXYZ
|
|
| FuzzerType:TypeNameXYZ:InstanceNameXYZ99
|     Arg1:
|         ValueA,
|         Value   B,
|         Value\\tWhatElse\\nC\\t,
|         " Value\\tWhatElse\\nF   ",
|         0x3031203234203536,
|         ValueZ,
|     Arg2:
|         ValueX,
|         ValueM,
|         ValueN,
|
|
| FuzzChain:FuzzChain:FC100
|     InstanceNameABC1
|     InstanceNameXYZ99
|     InstanceNameXYZ99
|

NOTE: The sample template above, also shows how string (textual or binary or
a mixture of both) can be specified in different ways, based on what one needs.


Predefined Fuzzers
-------------------

There are two types of fuzzers,

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

  * Buf8RandomizeFuzzer [TODO]

    * return/append a buffer which contains the originally provided data, with
      some amount of random modifications to its contents, as noted below.

      * a predefined number of bytes randomly modified

        * if not predefined, then it is randomly decided as to how many bytes
          should be randomly modified.

      * the new random byte values are selected to be within a specified range
        of values.

        * if start value is not specified, it is assumed to be 0

        * if end value is not specified, it is assumed to be 255

      * the positions that are randomly modified are selected randomly, but
        inturn restricted to be within a specified range of positions.

        * if start position is not specified, it is assumed to be 0

        * if end position is not specified, it is assumed to be till end
          of the provided original buffer.


Custom Fuzzers
----------------

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



Prg file
==========

Overview
----------

This allows the end user to control the actions to be performed by fuzzerk, in a simple and flexible way.

The commands/operations that can be specified using prg file include


Data/Variables Related
~~~~~~~~~~~~~~~~~~~~~~~

* letstr <string_var_id> <string value>

* letint <int_var_id> <integer_value>

* inc <int_var_id>

* dec <int_var_id>

* bufnew <buf_id> <buf_size>

  Create a named buffer of a given size

* letbuf <buf_id> data_for_buffer

  Create a buffer and fill it with specified data. The data specified could be

  * a textual string till end of line. This can even include space in between.

    * if you want white space(s) at begin or end of the textual string, you need to use the hex string option mentioned next.

  * a hex string till end of line (identified by having 0x at begining of the data)

  * special data markers

    * __TIME__STAMP__

      * This puts the current time stamp into the buffer

    * __RANDOM__BYTES__TheLength

      * This puts TheLength amount of random bytes into the buffer

* bufsmerge destbuf srcbuf1 srcbuf2 ..... srcbufn

  This allows a new buffer to be created with contents of the source buffers specified merged/concatenated together.

  If only 1 source buffer is specified, it is equivalent to copying it into a new dest buffer.

  * bufsmerge destbuf srcbuf

    * destbuf = srcbuf

  If more than 1 source buffer is specified, it concats all the source buffers into a new dest buffer.

    * destbuf = srcbuf1 + srcbuf2 + ..... + srcbufn

* buf8randomize bufid randcount buf_startoffset buf_endoffset rand_startval rand_endval

  * randomize randcount values from with in a part (start and end offset) of the buf
    with values from a given range (start and end value).

  * other than bufid, other arguments are optional and if not given a suitable default value
    will be used

    * randcount - randomly generated to be less than buflen

    * buf_startoffset and buf_endoffset map to begin and end of buffer being operated on, if not specified.

    * rand_startval will be mapped to 0 and rand_endval to 255, if needed

  * inclusive ends

    * buf_endoffset is inclusive, that is value at corresponding index may be randomized, if it gets
      randomly selected during running/execution of the buf8randmoze instruction/operation.

    * rand_endval is inclusive


IOBridge related
~~~~~~~~~~~~~~~~~

* iobnew <iob_id> <iobtype:typespecific_addr> <typespecific_ioarg=value> <typespecific_ioarg=value> ...

  * supported iobtypes include

    * console - for writing generated data to stdout

      * NOTE that there could be more textual info seen on the screen, but they are written to stderr,
        so that the fuzzers and fuzzchains and their generated data is not disturbed.

    * tcpclient - for connecting to a tcp server

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

* iobwrite <iob_id> <buf_id>

  * write contents of the specified buffer into the specified iobridge

* iobflush <iob_id>

  * request flushing of any buffering of written data by the library and or os into the underlying io device

* iobread <iob_id> <buf_id>

  * try to read upto specified buffer's buffer length of data from the specified iobridge

    * one can use bufnew to create buffer of a required size with no data in it.

  * while creating a new iobridge remember to set a read_timeout, so that read wont block indefinitely, if there is no data to read.

    * all io bridge types may not support read_timeout (currently only network types ie tcpclient and tlsclient support it).

* iobclose <iob_id>


Fuzzers related
~~~~~~~~~~~~~~~~~

* fcget <fc_id> <buf_id>

  Generate a fuzzed buffer of data and store into buffer of specified id.


Control/System related
~~~~~~~~~~~~~~~~~~~~~~~

* sleepmsec <milliseconds>

* !label <label_id>

  a directive to mark the current location/address in the program where this directive is encountered

* iflt <check_value> <int_var_id> goto <label_id>

  if int value in the int_var_id is less than check_value, then goto (ie pass program flow control to) specified label.

* checkjump arg1 arg2 Label4LessThan Label4Equal Label4GreaterThan

  * __NEXT__ a implicit label identifying the next instuction/op in the program

    * useful if one doesnt want to jump to any specific location for a given condition,
      then the control will implicitly flow to next instruction in the program, in that case.

  * prefix $ to arg1 or arg2 to treat it has a literal number, else it will be treated has a int var

* jmp label

  * a unconditional jump

* call label

  * call a func

  * currently there are no function arguments support yet,
    they have to work with the global data space directly.


* ret

  * return from func


A sample file
---------------

|
|       letstr <strvarid> <string value>
|       letint <intvarid> <intvalue>
|       iobnew <iobid> <iobtype:addr> <ioargkeyX=valY> <ioargkeyA=valC>
| !label labelid
|       fcget <fcid> <bufid>
|       iobwrite <iobid> <bufid>
|       sleepmsec <milliseconds>
|       iobread <iobid> <bufid>
|       iobclose <iobid>
|       inc <intvarid>
|       iflt <chkvalue> <intvarid> goto labelid
|       dec <intvarid>
|



Cmdline
|||||||||

The key cmdline options are

* --cfgfc <path/to/fuzzers_fuzzchains.cfgfile>
* --prgfile <path/to/prgfile>

There are few additional options, in case one is not using a prgfile

* --ioaddr <iobtype:addr>
  * defaults to console, if not explicitly specified.
* --ioarg <ioargkeyX=valY>
  * defaults to no args, if not explicitly specified.
* --loopcnt <number>
  * defaults to 1, if not explicitly specified.
* --fc <fcid>
  * defaults to empty string, if not explicitly specified.


TODO Plus
############


DONE
|||||||

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


TODO
||||||


* In http tls single session multi request testing (with invalid data)

  * if 10msec btw requests, then server seems to get all requests.

  * if 1000msec btw requests, then server seems to only get the 1st request most of the time

  * ALERT: Need to check what happens with valid http requests instead of invalid http requests.

* A Fuzzer which allows a predefined string from a list of predefined strings to be randomly changed wrt some random positions in the string

* New Ops

  * use $ prefix everywhere to indicate integer values.

* allow extra unneeded whitespaces in between

  * Ok with bufsmerge


