
(define (run-application app-name root-component handle-message)
  (begin
    (thread.spawn (lambda () (gui-run root-component app-name)))
    (while #t (handle-message))))

; Built in components
(define virtical-box nil)
(define virtical-box-add nil)
(define virtical-box-get nil)
(define horizontal-box nil)
(define horizontal-box-add nil)
(define button nil)
(define button-on-click-set! nil)
(define label nil)
(define label-text-set! nil)

(define run-gui nil)

(define (init-gui path)
  (begin
    (import path 'core)
    (set! virtical-box core.virtical-box)
    (set! horizontal-box core.horizontal-box)
    (set! button core.button)
    (set! label core.label)
    nil))
