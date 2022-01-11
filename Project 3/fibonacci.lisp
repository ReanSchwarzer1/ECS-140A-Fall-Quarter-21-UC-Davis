

;1. calculate nth fibonacci

(defun nth-fib (n)
	
 (cond ((= n 1) 1)
 ((= n 2) 1)
 (t (+
 (nth-fib (- n 1))
 (nth-fib (- n 2))))))
 
 
;(print(nth-fib 9 ))



;2.calculate fibonacci series

(defun fib (n &optional (x 0) (y 1) (ls ()))
  (if (= n 0)
      (nreverse ls)
      (fib (1- n) y (+ x y) (cons x ls))))


;(print(fib 9 ))
;3.calculate fibonacci to a given limit

; helper function to calculate fibonacci of a given limit 
(defun fib-lt-helper (n limit &optional (x 0) (y 1) (ls ()))
  (if (or (= n 0)
       (> x limit))
      (nreverse ls)
      (fib-lt-helper (1- n) limit y (+ x y) (cons x ls) )))

(defun fib-lt (n)
(print(fib-lt-helper n n ))
)


;(fib-lt 20)
