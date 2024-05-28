
(import "mod.scm" 'mod)

(mod.test)

(module 'test 
        (define (test) (display "test\n")))
   
(test.test)

(thread.spawn (lambda () (display "thread\n")))

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

(try (error 'test "error") 
     [(catch 'test msg) (display "caught error\n")])

(display "making macro\n")
(define-syntax-rule (swap x y)
                    (let [(tmp x)]
                      (begin
                        (set! x y)
                        (set! y tmp))))

(display "done\n")
(define x 1)
(define y 2)
(define tmp 0)

(display "swapping\n")
(swap y x)

(call 'fib 10)

(cond
  [(= 1 2) (display "1=2\n")]
  [(= 1 1) (display "1=1\n")]
  [else (display "else\n")])

(struct point (x y))

(point-x (point 1 2))

(point-x-set!(point 1 2) 4)

(match (point 1 2)
        [(point x y) (display "point\n")])


(enum color 
    (rgb red green blue))


(color-rgb 1 2 3)

(color-rgb-red (color-rgb 1 2 3))

(color-rgb-red-set! (color-rgb 1 2 3) 2)

(match (color-rgb 1 2 3)
        [(color rgb r g b) (display "color\n")]
        [else (display "else\n")])

(network.string->ipv4 "8.8.8.8")

(match "hello world" 
        ["hello world" (display "match\n")])


;(import "gtk.so" 'ffi)

;(ffi.hello-gtk)

(import "data.scm" 'data)

(define d (data.deque-empty))

(display "pushing\n")
(data.deque-push-back d 1)
;(data.deque-for-each d (lambda (x) (begin (debug-display x) (display " "))))
(display "pushing\n")
(data.deque-push-back d 2)
;(data.deque-for-each d (lambda (x) (begin (debug-display x) (display " "))))
(display "pushing\n")
(data.deque-push-back d 3)

(if (data.deque-empty? d)
    (display "empty\n")
    (display "not empty\n"))

(data.deque-for-each d (lambda (x) (begin (display (integer->string x)) (display " "))))


(/ 1 0)
