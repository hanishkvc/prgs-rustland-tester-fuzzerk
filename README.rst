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


Config files
################

FuzzerChain File
==================

Overview
-----------

This defines one or more fuzzers and the fuzz chains created using them.
End user can create these files and then pass it to the program, so that
at runtime the needed fuzz chain can be created without having to recompile
things, ie provided they can make do with the fuzzers that is already
provided.


The Template
---------------

FuzzerType:TypeNameABC:InstanceNameABC1
  Arg1: ValueX,
  Arg2: ValueM,
  ArgA:
    Value1,
    Value2,
    ValueXYZ


FuzzerType:TypeNameXYZ:InstanceNameXYZ99
    Arg1:
        ValueA,
        ValueB,
        ValueZ,
    Arg2:
        ValueX,
        ValueM,
        ValueN,


FuzzChain:FuzzChain:FC100
    InstanceNameABC1
    InstanceNameXYZ99
    InstanceNameXYZ99

