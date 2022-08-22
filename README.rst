####################
FuzzerK library+
####################
Author: HanishKVC,
Version: 20220819IST0750


Overview
##########

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


Runtime
#########

Control files
||||||||||||||||

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
| iob open
| fc <fcid>
| iob write
| sys sleep <seconds>
| iob read
| ctl jump <index_offset>
| iob close
| loop inc
| loop iflt <number> relpos <+-number>
| loop iflt <number> abspos <cmdindex>
|



Cmdline
|||||||||

The cmdline options are


