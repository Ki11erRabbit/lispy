
(import "mod.scm" 'mod)

(mod.test)

(module 'test 
        (define (test) (display "test\n")))
   
(test.test)

(thread.spawn (lambda () (display "thread\n")))
(sleep 5)

(define l '(1 2 3 4))

(define x 10)

(define (f x) (+ x 1))

(f x)

(display :str "hello world\n")

(define (fib n)
  (if (<= n 1)
      n
      (+ (fib (- n 1)) (fib (- n 2)))))

(fib 10)

(let [(y 10)]
  y)


(define (g)
  (begin 
    (let [(y #(1 2 3))]
      (set! y #(4 5 6)))
    (sleep 5)
    (g)))

(if (boolean? #t)
    (display "true\n")
    (display "false\n"))

(try (error 'test "error") ([(catch 'test)
   (display "caught error\n")]))

(/ 1 0)
