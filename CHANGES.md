2.0.0
=====

* Rework the API to create new primary APIs that are just free functions.
  Behind the scenes, they use pre-computed tables that used to be computed
  at runtime.
* Rework the Error API to better differentiate between input errors and usage
  errors.
