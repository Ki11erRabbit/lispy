


; A component contains it's raw datatype, an update function, and a view function
(struct component [raw update view])

; Updates the component on a message event
(define (update component message sender)
  ...)

; Modifies widgets to b the data to render
(define (view component widgets sender)
  ...)

; An application contains the the root component which should be the window
; run should be a function that takes a renderer and the application itself
; handle-message should be a function that takes a reciever channel and route messages to the correct component
(struct application [run root-component handle-message])


; This should caputure the receiving end of the channel and route messages to the correct component
; Maybe the component should also be capured to make updating easier
(define (handle-message)
  ...)

(define (application-new root-component handle-message)
  (application default-runner root-component handle-message))

(define (default-runner renderer app)
  (while #t 
       (match app
	 [(application run root-component handle-message) (begin
							    (hande-message)
							    (match root-component
							      [(component raw update view) (begin
											     (renderer (view root-component)))]
							      [else (error 'run "Invalid root component")]))]
	 [else (error 'run "Invalid application")])))

(define renderer nil)


(define (run-application app)
  (match app
    [(application run root-component handle-message) (run renderer app)]
    [else (error 'run-application "Invalid application")]))




; Built in components
(define window nil)
(define virtical-box nil)
(define virtical-box-add nil)
(define horizontal-box nil)
(define horizontal-box-add nil)
(define button nil)
(define button-on-click nil)
(define label nil)
(define label-set-text nil)



(define (init-gui path)
  (begin
    (import path 'core)
    (set! renderer core.renderer)
    (if (bound? 'core.runner)
	(set! default-runner core.runner)
	nil)
    (set! virtical-box core.virtical-box)
    (set! horizontal-box core.horizontal-box)
    (set! button core.button)
    (set! label core.label)
    nil
    ))
