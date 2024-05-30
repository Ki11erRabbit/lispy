(import "gui-api.scm" 'gui)

(define channel (sync.mpsc-channel))
(define send (car channel))
(define recv (cdr channel))

(define gui-lib (gui.init-gui "gtk.scm"))

(display "creating button\n")
(debug-display gui.button-on-click-set!)
(define button (gui.button "Hello World!"))

(display "setting button on click\n")
(gui.button-on-click-set! button (lambda (x) (display "Hello World!")))

(display "running application\n")
(define handle-message (lambda (message) nil))

(display "running application\n")
(gui.run-application "org.test.hello-world" button handle-message)
