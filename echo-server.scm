
(define send-addr (network.string->socket-addr-v4 "127.0.0.1:9697"))

(define send-socket (network.udp-socket send-addr))
(define recv-socket (network.udp-socket (network.string->socket-addr-v4 "127.0.0.1:9696")))

(define send-addr (network.string->socket-addr-v4 "127.0.0.1:9696"))

(define (g)
  (begin 
    (let [(y #(1 2 3))]
      (set! y #(4 5 6)))
    (sleep 5)
    (g)))


(define (loop recv-socket send-socket)
    (let [(data (network.receive recv-socket))] 
      (begin (network.connect recv-socket send-addr) 
             (network.send recv-socket data) 
             (loop recv-socket send-socket))))

(loop recv-socket send-socket)
