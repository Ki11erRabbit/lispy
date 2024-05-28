


(struct vector-list [vec size])


(define (vector-list-empty)
  (vector-list (vector 0 nil) 0))

(define (vector-list-empty? v)
(= 0 (vector-list-size v)))

(define (resize-vector-list v capacity)
  (if (= capacity 0)
    (vector-list-vec-set! v #(nil))
    (let [(index 0)
          (new-vec (vector capacity nil))]
      (begin
        (while (< index (vector-list-size v)) (begin
          (vector-set! new-vec index (vector-ref (vector-list-vec) v index))
          (set! index (+ index 1))))
        (vector-list-vec-set! v new-vec)))))

(define (vector-list-ref v i)
    (if (or (< i 0) (>= i (vector-list-size v) ))
        (error 'vector-list-ref "index out of bounds")
        (vector-ref (vector-list-vec v) i)))

(define (vector-list-set! v i x)
    (if (or (< i 0) (>= i (vector-list-size v) ))
        (error 'vector-list-set! "index out of bounds")
        (vector-set! (vector-list-vec v) i x)))

(define (vector-list-push v x)
  (begin
    (if (= (vector-list-size v) (vector-length (vector-list-vec v)))
        (resize-vector-list v (* 2 (vector-list-size v)))
        nil)
    (vector-list-set! v (vector-list-size v) x)
    (vector-list-size-set! v (+ (vector-list-size v) 1))))

(define (vector-list-pop v)
  (if (vector-list-empty? v)
      (error 'vector-list-pop "empty vector-list")
    (let [(x (vector-list-ref v (- (vector-list-size v) 1)))]
      (begin (vector-list-size-set! v (- (vector-list-size v) 1))
      x))))

(define (vector-list-last v)
  (if (vector-list-empty? v)
      (error 'vector-list-last "empty vector-list")
      (vector-list-ref v (- (vector-list-size v) 1))))

(define (vector-list-length v)
  (vector-list-size v))

(define (vector-list-clear v)
  (vector-list-size-set! v 0)))

(define (vector-list-for-each v f)
    (let [(index 0)]
     (while (< index (vector-list-size v))
      (begin
        (f (vector-list-ref v index))
        (set! index (+ index 1))))))

(define (vector-list-map v f)
  (let [(new-v (vector-list-empty))]
    (vector-list-for-each v (lambda (x) (vector-list-push new-v (f x)))
    new-v)))
