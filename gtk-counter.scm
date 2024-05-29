(import "gtk.so" 'gtk)


(display "creating app\n")
(define app (gtk.application-new "org.gtk.counter"))

(struct counter [count])

(define (activate app)
  (let [(window (gtk.application-window-new app))
        (inc-button (gtk.label-button-new "Increment"))
        (dec-button (gtk.label-button-new "Decrement"))
        (label (gtk.label-new "0"))
        (box (gtk.box-vertical-new))
        (counter (counter 0))]
    (begin
    (gtk.window-title-set! window "Counter")
    (gtk.window-default-size-set! window 200 100)
    (gtk.button-onclick-set! inc-button (lambda (app) (begin
                                                        (display "incrementing\n")
                                                        (counter-count-set! (+ (counter-count counter) 1))
                                                        (gtk.label-text-set! label (number->string (counter-count counter))))))
    (display "connecting button\n")
    (gtk.button-onclick-set! dec-button (lambda (app) (begin
                                                        (display "decrementing\n")
                                                        (counter-count-set! (- (counter-count counter) 1))
                                                        (gtk.label-text-set! label (number->string (counter-count counter))))))
    (display "packing\n")
    (gtk.box-append box inc-button)
    (gtk.box-append box label)
    (gtk.box-append box dec-button)
    (gtk.window-child-set! window box)
    (display "showing\n")
    (gtk.window-present window))))

(display "connecting activate\n")
(gtk.g-signal-connect app "activate" activate)
(display "running app\n")
(gtk.g-application-run app)
