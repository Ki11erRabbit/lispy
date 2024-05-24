

(define socket (network.udp-socket (network.string->socket-addr-v4 "127.0.0.1:9696")))


(define (g)
  (begin 
    (let [(y #(1 2 3))]
      (set! y #(4 5 6)))
    (sleep 5)
    (g)))

(define (loop socket)
    (let [(data (network.receive socket))]
      (begin
       (network.send socket data)
       (loop socket))))

(loop socket)
