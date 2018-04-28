#lang racket/base

(provide g)

(require fractalide/modules/rkt/rkt-fbp/graph)

(define g (make-graph
           (node "nand" "math/nand")
           (node "clone" "clone")
           (edge "clone" "out" "1" "nand" "x" #f)
           (edge "clone" "out" "2" "nand" "y" #f)
           (virtual-in "in" "clone" "in")
           (virtual-out "res" "nand" "res")))