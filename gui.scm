

(struct component [raw update view])
(struct message [type data])

; returns a message if needed
(define (update component raw event)
  ...)

; return data to render
(define (view component raw)
  ...)

(define component-list '())


(define (run renderer event)
    (begin
    (for-each
      (lambda (component)
        (match component
               [(component raw update view) (update component raw event)]
               [else (error 'gui.run "Invalid component")]))
      component-list)
    (for-each
      (lambda (component)
        (match component
               [(component raw update view) (view component raw)]
               [else (error 'gui.run "Invalid component")]))
      component-list)
      ))


(define (init path)
  (import path 'core))
