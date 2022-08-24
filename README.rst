####################
FuzzerK library+
####################
Author: HanishKVC,
Version: 20220819IST0750


Overview
##########

Library
|||||||||

Fuzzers and Chains
===================

At the core, the library defines a Fuzz trait, which allows fuzzers to
be created. And FuzzChain containers to allow fuzzers to be chained
together to create the needed end pattern of data.

In turn it also provides a set of predefined fuzzers, which generate new
binary buffer each time they are called, with either random or looped or
mixture of random and presetup data, based on how they are configured.
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

This can be used to test input parsing logic of programs to see, that
they handle all possible input cases sufficiently robustly.

IOBridge
==========

This is a helper module for the util program to help work with either
Console or Tcp server or Tls server or File, in a generic way.

* console

* tcpclient:addr:port

  * addr could be ip addr or domain name

  * ioargs supported

    * read_timeout=millisecs

* tlsclient:addr:port

  * addr could be ip addr or domain name

  * ioargs supported

    * server_cert_check=yes/no

    * domain=the.domain.name

    * read_timeout=millisecs

* filewriter:path/to/file

  * ioargs supported

    * append=yes/no


VM
====

This is a helper module for the util program's operations to be controlled
by the end user using custom prg files, which use operations defined by
a application specific VM. This provides the VM.


MinimalFuzzerKUtil
||||||||||||||||||||

A program which uses the modules mentioned previously to help test
other programs by generating fuzz input for them and pushing to them
using either console or tcp or tls session.

This allows a end user to quickly test their program using this fuzzer
logic, without needing to modify their program to handshake with the
fuzzer library provided by this package. They just need to write some
simple text based control files and inturn test their program, provided
their program takes inputs over the stdin or a tcp session or a tls
session.

It also allows one to test the libraries/modules in a simple yet
flexible and potentially useful way.


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

This defines one or more fuzzers and the fuzz chains created using them.
End user can create these files and then pass it to the program, so that
at runtime the needed fuzz chain can be created without having to recompile
things, ie provided they can make do with the fuzzers that is already
provided.

Alert: Dont intermix tab and spaces, even thou visually both may appear
to be equivalent, the logic will not accept such intermixing, bcas it
cant be sure how many spaces a tab represents in a given context/instance.

The Template
---------------

NOTE: The | (and one space after that for non empty lines) is for rst to
identify the below lines has a block of data to be retained as such by
rst.

|
| FuzzerType:TypeNameABC:InstanceNameABC1
|   Arg1: ValueX,
|   Arg2: ValueM,
|   ArgA:
|     Value1,
|     Value2,
|     ValueXYZ
|
|
| FuzzerType:TypeNameXYZ:InstanceNameXYZ99
|     Arg1:
|         ValueA,
|         ValueB,
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


Run file
==========

Overview
----------

This gives the actions to be performed by fuzzerk

The commands possible in run file include


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
* --ioarg <ioargkeyX=valY>
* --loopcnt <number>
* --fc <fcid>


TODO Plus
############


DONE
|||||||

* end of prgfile

* the fallback predefined program in case

  * prgfile is not specified

  * instead fc, loopcnt, ioaddr, ioarg etal is passed.


TODO
||||||

* iobclose and ssl session shutdown (do I need two calls, most probably not, the doc seems bit confusing)

* In http tls single session multi request testing (with invalid data)

  * if 10msec btw requests, then server seems to get all requests.

  * if 1000msec btw requests, then server seems to only get the 1st request most of the time

  * ALERT: Need to check what happens with valid http requests instead of invalid http requests.

* A Fuzzer which allows a predefined string from a list of predefined strings to be randomly changed wrt some random positions in the string

