(ns tutorial.core
  (:gen-class))

(float? 15)

(defn -main
  "I don't do a whole lot ... yet."
  [& args]

  ; (def aString "Hello")
  (def aDouble 1.234)
  ; (def aLong 15)
  ; (format "this is string %s" aString)
  ; (format "5 space and %5d" aLong)

  ; (format "Leading zeros %04d" aLong)
  ; (format "%-4d left justified" aLong)
  (format "3 decimals %.3f" aDouble))

