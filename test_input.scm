(define one 1)
(print one)
(print (+ 1 1))
(print (+ 1.8 (/ 12 10)))
(print (* 2 (call/cc (lambda (k) (+ one one)))))
(print (* 3 (call/cc (lambda (k) (+ one (k (/ 5 3)))))))
(define (fact n)
  (if (= n 0)
    1
    (* n (fact (- n 1)))))
(print (fact 3))
(define (map proc coll)
  (if (eqv? coll '())
    '()
    (cons (proc (car coll)) (map proc (cdr coll)))))
(print (map fact '(1 2 3 4)))
