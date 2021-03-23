* RegisterSetjmp
  - un-inlineable `call`
  - forces args/results to go through memory
* TLS
  - un-inlineable
  - generally just not super fast
* stack management
  -


TABLES

|   measurement    | timing |
|------------------|--------|
| baseline         | 109    |
| inlining         | 99     |
| more lazy thread | 94     |
| no dtor          | 93     |
| no register_tls  | 35     |
| relaxed          | 31     |
| !! no setjmp     | 23     |
| !! no tls        | 3      |




