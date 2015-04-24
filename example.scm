(print "hello")

(if (= 1 1)
  '(print "I'm a fool!")
  '(print "I'm so sooooo"))

(define 'f (lambda '(x y z) '(print "You typed in: " x)))

(print f)
