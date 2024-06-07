; Run functions with names starting with `test_`
(
  (
    (function_declaration name: (_) @run
      (#match? @run "^test_.*$"))
  ) @_
  (#set! tag v-test)
)

; Run the main function
(
  (
    (function_declaration name: (_) @run
      (#eq? @run "main"))
  ) @_
  (#set! tag v-main)
)
