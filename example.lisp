(def {is-the-answer? ans} ; comments work!
  {if (= ans 42)
    {seq
      {print "HOORAY!"}
      {print "Seq performs all given expressions in a sequence, it returns the last one"}
    }
    {print "NO"}
  })

; well?
(is-the-answer? 42)

; example of accessing a Rust value
(print "A rusty value: " my-rust-value)
