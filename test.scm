


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

(f y)
