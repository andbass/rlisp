(def {is-the-answer? ans}
  {if (= ans 42)
    {seq
      {print "HOORAY!"}
      {print "Seq performs all given expressions in a sequence, it returns the last one"}
    }
    {print "NO"}
  })

(is-the-answer? 42)
