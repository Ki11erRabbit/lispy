
(import "gui.scm" 'gui)

(gui.init-gui "gtk.so")

(enum message (increment) (decrement))

(struct counter [count])

(define (counter-increment counter)
(begin
    (counter-count-set! counter (+ (counter-count counter) 1))
    nil))

(define (counter-decrement counter)
(begin
    (counter-count-set! counter (- (counter-count counter) 1))
    nil))

(define (counter-update component message sender)
  (match component
         [(gui.component counter widget update view) (match message
                                                    [(message increment) (counter-increment counter)]
                                                    [(message decrement) (counter-decrement counter)])]))

(define (counter-view component sender)
(match component
         [(gui.component counter widget update view) (begin
                        (counter-update-label counter widget)
                        (gui.virtical-box-render widget))]))

(define (counter-update-label counter widget)
(let [(label (gui.virtical-box-get widget 1))]
    (gui.label-text-set! label (number->string (counter-count counter)))))


(define channel (sync.mpsc-channel))
(define send (car channel))
(define receive (cdr channel))


(define vbox (gui.virtical-box))
(define increment-button (gui.button))
(gui.button-on-click-set! increment-button (lambda () (sync.mpsc-send send (message-increment))))
(define decrement-button (gui.button))
(gui.button-on-click-set! decrement-button (lambda () (sync.mpsc-send send (message-decrement))))
(define label (gui.label))
(gui.label-text-set! label "0")
(gui.virtical-box-add vbox increment-button)
(gui.virtical-box-add vbox label)
(gui.virtical-box-add vbox decrement-button)
(define window (gui.window))
(define counter-component (gui.component (counter 0) vbox counter-update counter-view)

(gui.window-add window counter-component)


(define app 
  (gui.application-new window
    (lambda () (begin
                 (let [(message (sync.mpsc-receive receive))]
                   (counter-update counter-component message send))))))


(gui.application-run app)
