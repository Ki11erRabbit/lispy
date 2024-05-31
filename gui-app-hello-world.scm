(import "gui-api.scm" 'gui)

(define channel (sync.mpsc-channel))
(define send (car channel))
(define recv (cdr channel))

(define gui-lib (gui.init-gui "gtk.scm"))

(define init-button (lambda (x) (let 
                                 [(button (gui.button "Hello World!"))] 
                                    (begin 
                                        (gui.button-on-click-set! button (lambda (x) (display "Hello World!\n"))) 
                                        button))))

(display "creating button\n")
;(define button (gui.button "Hello World!"))

(display "setting button on click\n")
;(gui.button-on-click-set! button (lambda (x) (display "Hello World!")))

(define (handle-message) nil)

(display "running application\n")
(gui.run-application "org.test.hello-world" init-button handle-message)
