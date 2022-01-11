 (defun match (pattern match)
      (cond
        ((null pattern) (null match))           
        ((or (atom pattern) (atom match)) nil)   
        ((equalp (first pattern) (first match)) 
         (match (rest pattern) (rest match)))   
        ((or (eql (first pattern) '!)  (eql (first pattern) '*) nil)         
         (match (rest pattern) (rest match)))
        (t nil)))


(print(match '(color apple red) '(color apple red))) ;T
(print(match '(color apple red) '(color apple green))) ;nil
(print(match '(this table !) '(this table supports))) ;T
(print(match '(! table !) '(this table supports))) ;T
(print(match '(color apple *) '(color apple red))) ;T
