

(struct component [raw update view])
(struct message [who data])

; returns a message if needed
(define (update component raw event)
  ...)

; return data to render
(define (view component raw)
  ...)

(define component-list (vector-list-empty))
(define channels (sync.mpsc-channel))
(define send-channel (car channels))
(define recv-channel (cdr channels))

; A component contains it's raw datatype, an init function, an update function, and a view function
(struct component [raw init-root init update view])

; Creates the root component
(define (init-root)
  ...)

; Intializes the component
(define (init start root sender)
  ...)

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

(define (run renderer app)
  ...)

(define (handle-message reciever)
  ...)

(define (run-application app)
  (match app)
    [(application run root-component handle-message) 
     ]
    [else (error 'run-application "Invalid application")])

(define (run renderer)
  (let [(message-queue (deque-empy))
    (components (vector-list-empty))
    (all-components (vector-list-empty))]
        (while #t (let ([value (sync.mpsc-try-receive recv-channel)])
          (begin
            (match value
                   [(message who data) (deque-push-back message-queue value)]
                   [else nil])
            (vector-list-for-each
              all-component-list
              (lambda (component)
                (match component
                       [(component raw update view) (update component raw event)]
                       [else (error 'gui.run "Invalid component")])))
            (vector-list-for-each
              component-list
              (lambda (component)
                (match component
                       [(component raw update view) (view component raw)]
                       [else (error 'gui.run "Invalid component")])))
              )))))


(define (init path)
  (import path 'core))
