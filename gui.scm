

(struct component [raw update view])
(struct message [who data])

; returns a message if needed
(define (update component raw event)
  ...)

; return data to render
(define (view component raw)
  ...)

(define component-list (vector-list-empty))
(define channels (sync.mpsc-channel))
(define send-channel (car channels))
(define recv-channel (cdr channels))



(struct application [data handle-message])

(define (run renderer)
  (let [(message-queue (deque-empy))
    (components (vector-list-empty))
    (all-components (vector-list-empty))]
        (while #t (let ([value (sync.mpsc-try-receive recv-channel)])
          (begin
            (match value
                   [(message who data) (deque-push-back message-queue value)]
                   [else nil])
            (vector-list-for-each
              all-component-list
              (lambda (component)
                (match component
                       [(component raw update view) (update component raw event)]
                       [else (error 'gui.run "Invalid component")])))
            (vector-list-for-each
              component-list
              (lambda (component)
                (match component
                       [(component raw update view) (view component raw)]
                       [else (error 'gui.run "Invalid component")])))
              )))))


(define (init path)
  (import path 'core))
