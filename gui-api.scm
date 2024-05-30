

(struct gui [vbox-new vbox-add hbox-new hbox-add button-new button-on-click-set! label-new label-text-set!])


(define (run-application app-name root-component handle-message)
  (begin
    (thread.spawn (lambda () (gui-run root-component app-name)))
    (while #t (handle-message))))

; Built in components
(define (virtical-box)
    core.virtical-box)
(define (virtical-box-add box item) (core.virtical-box-add box item))
(define (horizontal-box)
    core.horizontal-box)
(define (horizontal-box-add box item) (core.horizontal-box-add box item))
(define (button text) (core.button text))
(define (button-on-click-set! button f) (core.button-on-click-set! button f))
(define (label) (core.label))
(define (label-text-set! label text) (core.label-text-set! label text))

(define (run-gui root app-name) (core.gui-run root app-name))

(define (init-gui path)
  (begin
    (import path 'core)
    (set! virtical-box core.virtical-box)
    (set! horizontal-box core.horizontal-box)
    (set! button core.button)
    (set! button-on-click-set! core.button-on-click-set!)
    (set! label core.label)
    nil))
