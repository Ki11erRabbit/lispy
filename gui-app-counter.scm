(import "gui-api.scm" 'gui)

(define channel (sync.mpsc-channel))
(define send (car channel))
(define recv (cdr channel))

(enum message (increment) (decrement))

(define gui-lib (gui.init-gui "gtk.scm"))

(define init-button (lambda (x) (let 
                                 [(inc-button (gui.button "+"))
                                  (dec-button (gui.button "-"))
                                  (label (gui.label "0"))
                                  (box (gui.virtical-box))]
                                    (begin 
                                        (gui.button-on-click-set! inc-button (lambda (x) (sync.mpsc-send send (message-increment))))
                                        (gui.button-on-click-set! dec-button (lambda (x) (sync.mpsc-send send (message-decrement))))
                                        (gui.virtical-box-add box inc-button)
                                        (gui.virtical-box-add box label)
                                        (gui.virtical-box-add box dec-button)
                                        box))))

(display "creating button\n")
;(define button (gui.button "Hello World!"))

(display "setting button on click\n")
;(gui.button-on-click-set! button (lambda (x) (display "Hello World!")))

(define (handle-message) nil)

(display "running application\n")
(gui.run-application "org.test.hello-world" init-button handle-message)
