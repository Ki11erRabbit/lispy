
(import "gtk.so" 'gtk)







(define virtical-box gtk.box-vertical-new)
(define virtical-box-add gtk.box-append)
(define horizontal-box gtk.box-horizontal-new)
(define horizontal-box-add gtk.box-append)

(define button gtk.label-button-new)
(define button-on-click-set! gtk.button-onclick-set!)

(define label gtk.label-new)
(define label-text-set! gtk.label-text-set!)
;(define label-text-get gtk.label-text-get)




(define (run-gui root-init app-name) 
  (let [(root-init root-init)
        (activate (lambda (app) 
                    (let [(window (gtk.application-window-new app))]
                          (begin
                            (gtk.window-child-set! window (root-init "plank"))
                            (gtk.window-present window)))))]
    (let [(app (gtk.application-new app-name))]
      (begin
        (gtk.g-signal-connect app "activate" activate)
        (gtk.g-application-run app)))))
