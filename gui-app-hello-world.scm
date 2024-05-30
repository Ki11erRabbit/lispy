(import "gui-api.scm" 'gui)

(define channel (sync.mpsc-channel))
(define send (car channel))
(define recv (cdr channel))

(gui.init-gui "gtk.scm")

(define button (gui.button "Hello World!"))

(gui.button-on-click-set! button (lambda (x) (display "Hello World!")))

(define handle-message (lambda (message) nil))

(gui.run-application "org.test.hello-world" button handle-message)
