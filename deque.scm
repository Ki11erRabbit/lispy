



(struct deque [front back size vec])


(define (deque-empty)
  (deque 0 0 0 #()))

(define (deque-empty? d)
  (= (deque-size d) 0))

(define (resize-deque d capacity)
  (if (= capacity 0)
    (deque-vec-set! d #(nil))
    (let [(index 0)
          (new-vec (vector capacity nil))]
        (begin
      (while (< index (deque-size d))
        (begin
        (vector-set! new-vec index (deque-ref d index))
          (set! index (+ index 1))))
    (deque-vec-set! d new-vec)
    (deque-front-set! d 0)
    (deque-back-set! d index)))))

(define (deque-ref d i)
    (if (or (< i 0) (>= i (deque-size d)))
      (error 'deque-ref "index out of bounds")
        (match d
         [(deque front back size vec) 
          (let [(offset (+ front 1))
                (real-index (modulo (+ i offset) size))]
            (vector-ref vec real-index))])))

(define (deque-set! d i val)
    (if (or (< i 0) (>= i (deque-size d)))
      (error 'deque-set! "index out of bounds")
      (match d
         [(deque front back size vec) 
          (let [(offset (+ front 1))
                (real-index (modulo (+ i offset) size))]
            (vector-set! vec real-index val))])))

(define (deque-push-front d val)
  (begin
    (if (= (deque-size d) (vector-length (deque-vec d)))
            (resize-deque d (* 2 (vector-length (deque-vec d))))
            nil)
    (match d
        [(deque front back size vec)
            (let [(new-back (if (= size 0) (+ back 1) back))
                (new-front (modulo (- front 1) (vector-length vec)))]
              (begin
                (vector-set! vec front val)
                (deque-size-set! d (+ size 1))
                (deque-back-set! d new-back)
                (deque-front-set! d new-front)))])))

(define (deque-push-back d val)
  (begin
    (if (= (deque-size d) (vector-length (deque-vec d)))
            (resize-deque d (* 2 (deque-capacity d)))
            nil)
    (match d
        [(deque front back size vec)
            (let [(new-back (modulo (+ back 1) (vector-length vec)))
                (new-front (if (= size 0) (- front 1) front))]
              (begin
                (vector-set! vec back val)
                (deque-size-set! d (+ size 1))
                (deque-back-set! d new-back)
                (deque-front-set! d new-front)))])))

(define (deque-pop-front d)
  (match d
         [(deque front back size vec)
          (if (= size 0)
              (error 'deque-pop-front "deque is empty")
              (let [(val (vector-ref vec front))
                    (new-front (modulo (+ front 1) (vector-length vec)))]
                (begin
                  (deque-size-set! d (- size 1))
                  (deque-fron-set! d new-front)
                  val)))]))

(define (deque-pop-back d)
  (match d
         [(deque front back size vec)
          (if (= size 0)
              (error 'deque-pop-back "deque is empty")
              (let [(val (vector-ref vec back))
                    (new-back (modulo (- back 1) (vector-length vec)))]
                (begin
                  (deque-size-set! d (- size 1))
                  (deque-back-set! d new-back)
                  val)))]))

(define (deque-front d)
  (match d
         [(deque front back size vec)
          (if (= size 0)
              (error 'deque-front "deque is empty")
              (vector-ref vec front))]))

(define (deque-back d)
  (match d
         [(deque front back size vec)
          (if (= size 0)
              (error 'deque-back "deque is empty")
              (vector-ref vec back))]))

(define (deque-length d)
  (deque-size d))

(define (deque-capacity d)
  (vector-length (deque-vec d)))

(define (deque-clear d)
  (begin
    (deque-size-set! d 0)
    (deque-front-set! d 0)
    (deque-back-set! d 0)
    (deque-vec-set! d (vector (vector-length (deque-vec d)) nil))))

(define (deque-for-each d f)
  (let [(index 0)]
    (begin
      (while (< index (deque-size d))
        (begin
          (f (deque-ref d index))
          (set! index (+ index 1)))))))

(define (deque-map d f)
  (let [(new-d (empty-deque))]
    (begin
      (deque-for-each d (lambda (elem) (deque-push-back new-d (f elem)))
      new-d))))
