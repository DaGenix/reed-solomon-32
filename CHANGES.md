2.0.2
=====

* Fix panic!() that would occur when handling a message with a large
  number of ECC symbols and also a large number of errors.

2.0.1
=====

* Update README.md - the version from 2.0.0 still included an example from
  1.0.0 which would no longer work and would be very confusing.

2.0.0
=====

* Rework the API to create new primary APIs that are just free functions.
  Behind the scenes, they use pre-computed tables that used to be computed
  at runtime.
* Rework the Error API to better differentiate between input errors and usage
  errors.
