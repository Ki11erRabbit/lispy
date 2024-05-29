(import "gtk.so" 'gtk)

(display "creating app\n")
(define app (gtk.application-new "org.gtk.example"))


(define (activate app)
  (let [(window (gtk.application-window-new app))
        (button (gtk.label-button-new "Hello World"))]
    (begin
      (gtk.window-title-set! window "Hello")
      (gtk.window-default-size-set! window 200 200)
      (gtk.button-onclick-set! button (lambda (app) (display "hello world\n")))
      (gtk.window-child-set! window button)
      (gtk.window-present window))))

(display "connecting activate\n")
(gtk.g-signal-connect app "activate" activate)
(display "running app\n")
(gtk.g-application-run app)
